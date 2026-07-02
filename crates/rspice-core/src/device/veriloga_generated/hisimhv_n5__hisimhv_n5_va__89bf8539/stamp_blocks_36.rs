#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_185(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51630_e78110, assign51630_e78110_d_n0, assign51630_e78110_d_n2, assign51630_e78110_d_n4, assign51630_e78110_d_n5, assign51630_e78110_d_n6, assign51630_e78110_d_n7, assign51630_e78110_d_n8, assign51630_e78110_d_n9, assign51630_e78110_d_n10, assign51630_e78110_d_n11, assign51630_e78110_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) && (locals.var_guard1312 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign51630_e78110;
        locals.var_t0_dn0 = assign51630_e78110_d_n0;
        locals.var_t0_dn2 = assign51630_e78110_d_n2;
        locals.var_t0_dn4 = assign51630_e78110_d_n4;
        locals.var_t0_dn5 = assign51630_e78110_d_n5;
        locals.var_t0_dn6 = assign51630_e78110_d_n6;
        locals.var_t0_dn7 = assign51630_e78110_d_n7;
        locals.var_t0_dn8 = assign51630_e78110_d_n8;
        locals.var_t0_dn9 = assign51630_e78110_d_n9;
        locals.var_t0_dn10 = assign51630_e78110_d_n10;
        locals.var_t0_dn11 = assign51630_e78110_d_n11;
        locals.var_t0_dn14 = assign51630_e78110_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign51640_e78126, assign51640_e78126_d_n0, assign51640_e78126_d_n2, assign51640_e78126_d_n4, assign51640_e78126_d_n5, assign51640_e78126_d_n6, assign51640_e78126_d_n7, assign51640_e78126_d_n8, assign51640_e78126_d_n9, assign51640_e78126_d_n10, assign51640_e78126_d_n11, assign51640_e78126_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51640_e78124: f64 = (locals.var_psdl - locals.var_psl);
        (assign51640_e78124, (locals.var_psdl_dn0 - locals.var_psl_dn0), (locals.var_psdl_dn2 - locals.var_psl_dn2), (locals.var_psdl_dn4 - locals.var_psl_dn4), (locals.var_psdl_dn5 - locals.var_psl_dn5), (locals.var_psdl_dn6 - locals.var_psl_dn6), (locals.var_psdl_dn7 - locals.var_psl_dn7), (locals.var_psdl_dn8 - locals.var_psl_dn8), (locals.var_psdl_dn9 - locals.var_psl_dn9), (locals.var_psdl_dn10 - locals.var_psl_dn10), (locals.var_psdl_dn11 - locals.var_psl_dn11), (locals.var_psdl_dn14 - locals.var_psl_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign51640_e78126;
        locals.var_t6_dn0 = assign51640_e78126_d_n0;
        locals.var_t6_dn2 = assign51640_e78126_d_n2;
        locals.var_t6_dn4 = assign51640_e78126_d_n4;
        locals.var_t6_dn5 = assign51640_e78126_d_n5;
        locals.var_t6_dn6 = assign51640_e78126_d_n6;
        locals.var_t6_dn7 = assign51640_e78126_d_n7;
        locals.var_t6_dn8 = assign51640_e78126_d_n8;
        locals.var_t6_dn9 = assign51640_e78126_d_n9;
        locals.var_t6_dn10 = assign51640_e78126_d_n10;
        locals.var_t6_dn11 = assign51640_e78126_d_n11;
        locals.var_t6_dn14 = assign51640_e78126_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign51650_e78142, assign51650_e78142_d_n0, assign51650_e78142_d_n2, assign51650_e78142_d_n4, assign51650_e78142_d_n5, assign51650_e78142_d_n6, assign51650_e78142_d_n7, assign51650_e78142_d_n8, assign51650_e78142_d_n9, assign51650_e78142_d_n10, assign51650_e78142_d_n11, assign51650_e78142_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51650_e78140: f64 = (locals.var_beta * locals.var_qn0);
        (assign51650_e78140, ((locals.var_beta_dn0 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn0)), ((locals.var_beta_dn2 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn2)), ((locals.var_beta_dn4 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn4)), ((locals.var_beta_dn5 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn5)), ((locals.var_beta_dn6 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn6)), ((locals.var_beta_dn7 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn7)), ((locals.var_beta_dn8 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn8)), ((locals.var_beta_dn9 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn9)), ((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)), ((locals.var_beta_dn11 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn11)), ((locals.var_beta_dn14 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign51650_e78142;
        locals.var_t3_dn0 = assign51650_e78142_d_n0;
        locals.var_t3_dn2 = assign51650_e78142_d_n2;
        locals.var_t3_dn4 = assign51650_e78142_d_n4;
        locals.var_t3_dn5 = assign51650_e78142_d_n5;
        locals.var_t3_dn6 = assign51650_e78142_d_n6;
        locals.var_t3_dn7 = assign51650_e78142_d_n7;
        locals.var_t3_dn8 = assign51650_e78142_d_n8;
        locals.var_t3_dn9 = assign51650_e78142_d_n9;
        locals.var_t3_dn10 = assign51650_e78142_d_n10;
        locals.var_t3_dn11 = assign51650_e78142_d_n11;
        locals.var_t3_dn14 = assign51650_e78142_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign51660_e78158, assign51660_e78158_d_n0, assign51660_e78158_d_n2, assign51660_e78158_d_n4, assign51660_e78158_d_n5, assign51660_e78158_d_n6, assign51660_e78158_d_n7, assign51660_e78158_d_n8, assign51660_e78158_d_n9, assign51660_e78158_d_n10, assign51660_e78158_d_n11, assign51660_e78158_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51660_e78156: f64 = (1.0 / locals.var_t3);
        (assign51660_e78156, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign51660_e78158;
        locals.var_t1_dn0 = assign51660_e78158_d_n0;
        locals.var_t1_dn2 = assign51660_e78158_d_n2;
        locals.var_t1_dn4 = assign51660_e78158_d_n4;
        locals.var_t1_dn5 = assign51660_e78158_d_n5;
        locals.var_t1_dn6 = assign51660_e78158_d_n6;
        locals.var_t1_dn7 = assign51660_e78158_d_n7;
        locals.var_t1_dn8 = assign51660_e78158_d_n8;
        locals.var_t1_dn9 = assign51660_e78158_d_n9;
        locals.var_t1_dn10 = assign51660_e78158_d_n10;
        locals.var_t1_dn11 = assign51660_e78158_d_n11;
        locals.var_t1_dn14 = assign51660_e78158_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign51670_e78174, assign51670_e78174_d_n0, assign51670_e78174_d_n2, assign51670_e78174_d_n4, assign51670_e78174_d_n5, assign51670_e78174_d_n6, assign51670_e78174_d_n7, assign51670_e78174_d_n8, assign51670_e78174_d_n9, assign51670_e78174_d_n10, assign51670_e78174_d_n11, assign51670_e78174_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51670_e78172: f64 = (locals.var_idd * locals.var_t1);
        (assign51670_e78172, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn11 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn11)), ((locals.var_idd_dn14 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign51670_e78174;
        locals.var_t5_dn0 = assign51670_e78174_d_n0;
        locals.var_t5_dn2 = assign51670_e78174_d_n2;
        locals.var_t5_dn4 = assign51670_e78174_d_n4;
        locals.var_t5_dn5 = assign51670_e78174_d_n5;
        locals.var_t5_dn6 = assign51670_e78174_d_n6;
        locals.var_t5_dn7 = assign51670_e78174_d_n7;
        locals.var_t5_dn8 = assign51670_e78174_d_n8;
        locals.var_t5_dn9 = assign51670_e78174_d_n9;
        locals.var_t5_dn10 = assign51670_e78174_d_n10;
        locals.var_t5_dn11 = assign51670_e78174_d_n11;
        locals.var_t5_dn14 = assign51670_e78174_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign51680_e78190, assign51680_e78190_d_n0, assign51680_e78190_d_n2, assign51680_e78190_d_n4, assign51680_e78190_d_n5, assign51680_e78190_d_n6, assign51680_e78190_d_n7, assign51680_e78190_d_n8, assign51680_e78190_d_n9, assign51680_e78190_d_n10, assign51680_e78190_d_n11, assign51680_e78190_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51680_e78188: f64 = (locals.var_q_ndepm__blk1135 / 1.034943e-10);
        (assign51680_e78188, (locals.var_q_ndepm__blk1135_dn0 / 1.034943e-10), (locals.var_q_ndepm__blk1135_dn2 / 1.034943e-10), (locals.var_q_ndepm__blk1135_dn4 / 1.034943e-10), (locals.var_q_ndepm__blk1135_dn5 / 1.034943e-10), (locals.var_q_ndepm__blk1135_dn6 / 1.034943e-10), (locals.var_q_ndepm__blk1135_dn7 / 1.034943e-10), (locals.var_q_ndepm__blk1135_dn8 / 1.034943e-10), (locals.var_q_ndepm__blk1135_dn9 / 1.034943e-10), (locals.var_q_ndepm__blk1135_dn10 / 1.034943e-10), (locals.var_q_ndepm__blk1135_dn11 / 1.034943e-10), (locals.var_q_ndepm__blk1135_dn14 / 1.034943e-10),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign51680_e78190;
        locals.var_t10_dn0 = assign51680_e78190_d_n0;
        locals.var_t10_dn2 = assign51680_e78190_d_n2;
        locals.var_t10_dn4 = assign51680_e78190_d_n4;
        locals.var_t10_dn5 = assign51680_e78190_d_n5;
        locals.var_t10_dn6 = assign51680_e78190_d_n6;
        locals.var_t10_dn7 = assign51680_e78190_d_n7;
        locals.var_t10_dn8 = assign51680_e78190_d_n8;
        locals.var_t10_dn9 = assign51680_e78190_d_n9;
        locals.var_t10_dn10 = assign51680_e78190_d_n10;
        locals.var_t10_dn11 = assign51680_e78190_d_n11;
        locals.var_t10_dn14 = assign51680_e78190_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign51690_e78204, assign51690_e78204_d_n0, assign51690_e78204_d_n2, assign51690_e78204_d_n4, assign51690_e78204_d_n5, assign51690_e78204_d_n6, assign51690_e78204_d_n7, assign51690_e78204_d_n8, assign51690_e78204_d_n9, assign51690_e78204_d_n10, assign51690_e78204_d_n11, assign51690_e78204_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign51690_e78204;
        locals.var_t1_dn0 = assign51690_e78204_d_n0;
        locals.var_t1_dn2 = assign51690_e78204_d_n2;
        locals.var_t1_dn4 = assign51690_e78204_d_n4;
        locals.var_t1_dn5 = assign51690_e78204_d_n5;
        locals.var_t1_dn6 = assign51690_e78204_d_n6;
        locals.var_t1_dn7 = assign51690_e78204_d_n7;
        locals.var_t1_dn8 = assign51690_e78204_d_n8;
        locals.var_t1_dn9 = assign51690_e78204_d_n9;
        locals.var_t1_dn10 = assign51690_e78204_d_n10;
        locals.var_t1_dn11 = assign51690_e78204_d_n11;
        locals.var_t1_dn14 = assign51690_e78204_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign51700_e78220, assign51700_e78220_d_n0, assign51700_e78220_d_n2, assign51700_e78220_d_n4, assign51700_e78220_d_n5, assign51700_e78220_d_n6, assign51700_e78220_d_n7, assign51700_e78220_d_n8, assign51700_e78220_d_n9, assign51700_e78220_d_n10, assign51700_e78220_d_n11, assign51700_e78220_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51700_e78218: f64 = (1.0 / locals.var_leff);
        (assign51700_e78218, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign51700_e78220;
        locals.var_t2_dn0 = assign51700_e78220_d_n0;
        locals.var_t2_dn2 = assign51700_e78220_d_n2;
        locals.var_t2_dn4 = assign51700_e78220_d_n4;
        locals.var_t2_dn5 = assign51700_e78220_d_n5;
        locals.var_t2_dn6 = assign51700_e78220_d_n6;
        locals.var_t2_dn7 = assign51700_e78220_d_n7;
        locals.var_t2_dn8 = assign51700_e78220_d_n8;
        locals.var_t2_dn9 = assign51700_e78220_d_n9;
        locals.var_t2_dn10 = assign51700_e78220_d_n10;
        locals.var_t2_dn11 = assign51700_e78220_d_n11;
        locals.var_t2_dn14 = assign51700_e78220_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign51710_e78250, assign51710_e78250_d_n0, assign51710_e78250_d_n2, assign51710_e78250_d_n4, assign51710_e78250_d_n5, assign51710_e78250_d_n6, assign51710_e78250_d_n7, assign51710_e78250_d_n8, assign51710_e78250_d_n9, assign51710_e78250_d_n10, assign51710_e78250_d_n11, assign51710_e78250_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51710_e78234: f64 = (2.0 * locals.var_t5);
        let assign51710_e78237: f64 = (2.0 * locals.var_t10);
        let assign51710_e78239: f64 = (assign51710_e78237 * locals.var_t6);
        let assign51710_e78241: f64 = (assign51710_e78239 * locals.var_t4);
        let assign51710_e78242: f64 = (assign51710_e78234 + assign51710_e78241);
        let assign51710_e78245: f64 = (locals.var_t1 * locals.var_t4);
        let assign51710_e78246: f64 = (assign51710_e78242 + assign51710_e78245);
        let assign51710_e78248: f64 = (assign51710_e78246 * locals.var_t2);
        (assign51710_e78248, (((((2.0 * locals.var_t5_dn0) + (((((2.0 * locals.var_t10_dn0) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn0)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn0))) + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn0)), (((((2.0 * locals.var_t5_dn2) + (((((2.0 * locals.var_t10_dn2) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn2)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn2))) + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn2)), (((((2.0 * locals.var_t5_dn4) + (((((2.0 * locals.var_t10_dn4) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn4)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn4))) + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn4)), (((((2.0 * locals.var_t5_dn5) + (((((2.0 * locals.var_t10_dn5) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn5)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn5))) + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn5)), (((((2.0 * locals.var_t5_dn6) + (((((2.0 * locals.var_t10_dn6) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn6)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn6))) + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn6)), (((((2.0 * locals.var_t5_dn7) + (((((2.0 * locals.var_t10_dn7) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn7)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn7))) + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn7)), (((((2.0 * locals.var_t5_dn8) + (((((2.0 * locals.var_t10_dn8) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn8)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn8))) + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn8)), (((((2.0 * locals.var_t5_dn9) + (((((2.0 * locals.var_t10_dn9) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn9)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn9))) + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn9)), (((((2.0 * locals.var_t5_dn10) + (((((2.0 * locals.var_t10_dn10) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn10)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn10))) + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn10)), (((((2.0 * locals.var_t5_dn11) + (((((2.0 * locals.var_t10_dn11) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn11)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn11))) + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn11)), (((((2.0 * locals.var_t5_dn14) + (((((2.0 * locals.var_t10_dn14) * locals.var_t6) + (assign51710_e78237 * locals.var_t6_dn14)) * locals.var_t4) + (assign51710_e78239 * locals.var_t4_dn14))) + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))) * locals.var_t2) + (assign51710_e78246 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign51710_e78250;
        locals.var_t11_dn0 = assign51710_e78250_d_n0;
        locals.var_t11_dn2 = assign51710_e78250_d_n2;
        locals.var_t11_dn4 = assign51710_e78250_d_n4;
        locals.var_t11_dn5 = assign51710_e78250_d_n5;
        locals.var_t11_dn6 = assign51710_e78250_d_n6;
        locals.var_t11_dn7 = assign51710_e78250_d_n7;
        locals.var_t11_dn8 = assign51710_e78250_d_n8;
        locals.var_t11_dn9 = assign51710_e78250_d_n9;
        locals.var_t11_dn10 = assign51710_e78250_d_n10;
        locals.var_t11_dn11 = assign51710_e78250_d_n11;
        locals.var_t11_dn14 = assign51710_e78250_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign51720_e78266, assign51720_e78266_d_n0, assign51720_e78266_d_n2, assign51720_e78266_d_n4, assign51720_e78266_d_n5, assign51720_e78266_d_n6, assign51720_e78266_d_n7, assign51720_e78266_d_n8, assign51720_e78266_d_n9, assign51720_e78266_d_n10, assign51720_e78266_d_n11, assign51720_e78266_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51720_e78264: f64 = (locals.var_t11 * locals.var_t4);
        (assign51720_e78264, ((locals.var_t11_dn0 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn0)), ((locals.var_t11_dn2 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn2)), ((locals.var_t11_dn4 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn4)), ((locals.var_t11_dn5 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn5)), ((locals.var_t11_dn6 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn6)), ((locals.var_t11_dn7 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn7)), ((locals.var_t11_dn8 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn8)), ((locals.var_t11_dn9 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn9)), ((locals.var_t11_dn10 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn10)), ((locals.var_t11_dn11 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn11)), ((locals.var_t11_dn14 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign51720_e78266;
        locals.var_t7_dn0 = assign51720_e78266_d_n0;
        locals.var_t7_dn2 = assign51720_e78266_d_n2;
        locals.var_t7_dn4 = assign51720_e78266_d_n4;
        locals.var_t7_dn5 = assign51720_e78266_d_n5;
        locals.var_t7_dn6 = assign51720_e78266_d_n6;
        locals.var_t7_dn7 = assign51720_e78266_d_n7;
        locals.var_t7_dn8 = assign51720_e78266_d_n8;
        locals.var_t7_dn9 = assign51720_e78266_d_n9;
        locals.var_t7_dn10 = assign51720_e78266_d_n10;
        locals.var_t7_dn11 = assign51720_e78266_d_n11;
        locals.var_t7_dn14 = assign51720_e78266_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign51730_e78288, assign51730_e78288_d_n0, assign51730_e78288_d_n2, assign51730_e78288_d_n4, assign51730_e78288_d_n5, assign51730_e78288_d_n6, assign51730_e78288_d_n7, assign51730_e78288_d_n8, assign51730_e78288_d_n9, assign51730_e78288_d_n10, assign51730_e78288_d_n11, assign51730_e78288_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51730_e78281: f64 = (2.0 * locals.var_t10);
        let assign51730_e78283: f64 = (assign51730_e78281 * locals.var_t6);
        let assign51730_e78285: f64 = (assign51730_e78283 + locals.var_t1);
        let assign51730_e78286: f64 = (4.0 * assign51730_e78285);
        (assign51730_e78286, (4.0 * ((((2.0 * locals.var_t10_dn0) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn0)) + locals.var_t1_dn0)), (4.0 * ((((2.0 * locals.var_t10_dn2) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn2)) + locals.var_t1_dn2)), (4.0 * ((((2.0 * locals.var_t10_dn4) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn4)) + locals.var_t1_dn4)), (4.0 * ((((2.0 * locals.var_t10_dn5) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn5)) + locals.var_t1_dn5)), (4.0 * ((((2.0 * locals.var_t10_dn6) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn6)) + locals.var_t1_dn6)), (4.0 * ((((2.0 * locals.var_t10_dn7) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn7)) + locals.var_t1_dn7)), (4.0 * ((((2.0 * locals.var_t10_dn8) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn8)) + locals.var_t1_dn8)), (4.0 * ((((2.0 * locals.var_t10_dn9) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn9)) + locals.var_t1_dn9)), (4.0 * ((((2.0 * locals.var_t10_dn10) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn10)) + locals.var_t1_dn10)), (4.0 * ((((2.0 * locals.var_t10_dn11) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn11)) + locals.var_t1_dn11)), (4.0 * ((((2.0 * locals.var_t10_dn14) * locals.var_t6) + (assign51730_e78281 * locals.var_t6_dn14)) + locals.var_t1_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign51730_e78288;
        locals.var_t11_dn0 = assign51730_e78288_d_n0;
        locals.var_t11_dn2 = assign51730_e78288_d_n2;
        locals.var_t11_dn4 = assign51730_e78288_d_n4;
        locals.var_t11_dn5 = assign51730_e78288_d_n5;
        locals.var_t11_dn6 = assign51730_e78288_d_n6;
        locals.var_t11_dn7 = assign51730_e78288_d_n7;
        locals.var_t11_dn8 = assign51730_e78288_d_n8;
        locals.var_t11_dn9 = assign51730_e78288_d_n9;
        locals.var_t11_dn10 = assign51730_e78288_d_n10;
        locals.var_t11_dn11 = assign51730_e78288_d_n11;
        locals.var_t11_dn14 = assign51730_e78288_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign51740_e78306, assign51740_e78306_d_n0, assign51740_e78306_d_n2, assign51740_e78306_d_n4, assign51740_e78306_d_n5, assign51740_e78306_d_n6, assign51740_e78306_d_n7, assign51740_e78306_d_n8, assign51740_e78306_d_n9, assign51740_e78306_d_n10, assign51740_e78306_d_n11, assign51740_e78306_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51740_e78302: f64 = (locals.var_t11 * locals.var_t4);
        let assign51740_e78304: f64 = (assign51740_e78302 * locals.var_t4);
        (assign51740_e78304, ((((locals.var_t11_dn0 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn0)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn0)), ((((locals.var_t11_dn2 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn2)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn2)), ((((locals.var_t11_dn4 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn4)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn4)), ((((locals.var_t11_dn5 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn5)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn5)), ((((locals.var_t11_dn6 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn6)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn6)), ((((locals.var_t11_dn7 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn7)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn7)), ((((locals.var_t11_dn8 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn8)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn8)), ((((locals.var_t11_dn9 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn9)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn9)), ((((locals.var_t11_dn10 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn10)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn10)), ((((locals.var_t11_dn11 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn11)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn11)), ((((locals.var_t11_dn14 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn14)) * locals.var_t4) + (assign51740_e78302 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign51740_e78306;
        locals.var_t8_dn0 = assign51740_e78306_d_n0;
        locals.var_t8_dn2 = assign51740_e78306_d_n2;
        locals.var_t8_dn4 = assign51740_e78306_d_n4;
        locals.var_t8_dn5 = assign51740_e78306_d_n5;
        locals.var_t8_dn6 = assign51740_e78306_d_n6;
        locals.var_t8_dn7 = assign51740_e78306_d_n7;
        locals.var_t8_dn8 = assign51740_e78306_d_n8;
        locals.var_t8_dn9 = assign51740_e78306_d_n9;
        locals.var_t8_dn10 = assign51740_e78306_d_n10;
        locals.var_t8_dn11 = assign51740_e78306_d_n11;
        locals.var_t8_dn14 = assign51740_e78306_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign51750_e78325, assign51750_e78325_d_n0, assign51750_e78325_d_n2, assign51750_e78325_d_n4, assign51750_e78325_d_n5, assign51750_e78325_d_n6, assign51750_e78325_d_n7, assign51750_e78325_d_n8, assign51750_e78325_d_n9, assign51750_e78325_d_n10, assign51750_e78325_d_n11, assign51750_e78325_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51750_e78320: f64 = (locals.var_t7 * locals.var_t7);
        let assign51750_e78322: f64 = (assign51750_e78320 + locals.var_t8);
        let assign51750_e78323: f64 = (assign51750_e78322).sqrt();
        (assign51750_e78323, ((((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)) + locals.var_t8_dn0) / (2.0 * assign51750_e78323)), ((((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)) + locals.var_t8_dn2) / (2.0 * assign51750_e78323)), ((((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) + locals.var_t8_dn4) / (2.0 * assign51750_e78323)), ((((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) + locals.var_t8_dn5) / (2.0 * assign51750_e78323)), ((((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) + locals.var_t8_dn6) / (2.0 * assign51750_e78323)), ((((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) + locals.var_t8_dn7) / (2.0 * assign51750_e78323)), ((((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) + locals.var_t8_dn8) / (2.0 * assign51750_e78323)), ((((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) + locals.var_t8_dn9) / (2.0 * assign51750_e78323)), ((((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) + locals.var_t8_dn10) / (2.0 * assign51750_e78323)), ((((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)) + locals.var_t8_dn11) / (2.0 * assign51750_e78323)), ((((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)) + locals.var_t8_dn14) / (2.0 * assign51750_e78323)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign51750_e78325;
        locals.var_t9_dn0 = assign51750_e78325_d_n0;
        locals.var_t9_dn2 = assign51750_e78325_d_n2;
        locals.var_t9_dn4 = assign51750_e78325_d_n4;
        locals.var_t9_dn5 = assign51750_e78325_d_n5;
        locals.var_t9_dn6 = assign51750_e78325_d_n6;
        locals.var_t9_dn7 = assign51750_e78325_d_n7;
        locals.var_t9_dn8 = assign51750_e78325_d_n8;
        locals.var_t9_dn9 = assign51750_e78325_d_n9;
        locals.var_t9_dn10 = assign51750_e78325_d_n10;
        locals.var_t9_dn11 = assign51750_e78325_d_n11;
        locals.var_t9_dn14 = assign51750_e78325_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign51760_e78344, assign51760_e78344_d_n0, assign51760_e78344_d_n2, assign51760_e78344_d_n4, assign51760_e78344_d_n5, assign51760_e78344_d_n6, assign51760_e78344_d_n7, assign51760_e78344_d_n8, assign51760_e78344_d_n9, assign51760_e78344_d_n10, assign51760_e78344_d_n11, assign51760_e78344_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51760_e78339: f64 = (-locals.var_t7);
        let assign51760_e78341: f64 = (assign51760_e78339 + locals.var_t9);
        let assign51760_e78342: f64 = (0.5 * assign51760_e78341);
        (assign51760_e78342, (0.5 * ((-locals.var_t7_dn0) + locals.var_t9_dn0)), (0.5 * ((-locals.var_t7_dn2) + locals.var_t9_dn2)), (0.5 * ((-locals.var_t7_dn4) + locals.var_t9_dn4)), (0.5 * ((-locals.var_t7_dn5) + locals.var_t9_dn5)), (0.5 * ((-locals.var_t7_dn6) + locals.var_t9_dn6)), (0.5 * ((-locals.var_t7_dn7) + locals.var_t9_dn7)), (0.5 * ((-locals.var_t7_dn8) + locals.var_t9_dn8)), (0.5 * ((-locals.var_t7_dn9) + locals.var_t9_dn9)), (0.5 * ((-locals.var_t7_dn10) + locals.var_t9_dn10)), (0.5 * ((-locals.var_t7_dn11) + locals.var_t9_dn11)), (0.5 * ((-locals.var_t7_dn14) + locals.var_t9_dn14)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign51760_e78344;
        locals.var_lred_dn0 = assign51760_e78344_d_n0;
        locals.var_lred_dn2 = assign51760_e78344_d_n2;
        locals.var_lred_dn4 = assign51760_e78344_d_n4;
        locals.var_lred_dn5 = assign51760_e78344_d_n5;
        locals.var_lred_dn6 = assign51760_e78344_d_n6;
        locals.var_lred_dn7 = assign51760_e78344_d_n7;
        locals.var_lred_dn8 = assign51760_e78344_d_n8;
        locals.var_lred_dn9 = assign51760_e78344_d_n9;
        locals.var_lred_dn10 = assign51760_e78344_d_n10;
        locals.var_lred_dn11 = assign51760_e78344_d_n11;
        locals.var_lred_dn14 = assign51760_e78344_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign51770_e78358, assign51770_e78358_d_n0, assign51770_e78358_d_n2, assign51770_e78358_d_n4, assign51770_e78358_d_n5, assign51770_e78358_d_n6, assign51770_e78358_d_n7, assign51770_e78358_d_n8, assign51770_e78358_d_n9, assign51770_e78358_d_n10, assign51770_e78358_d_n11, assign51770_e78358_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign51770_e78358;
        locals.var_t1_dn0 = assign51770_e78358_d_n0;
        locals.var_t1_dn2 = assign51770_e78358_d_n2;
        locals.var_t1_dn4 = assign51770_e78358_d_n4;
        locals.var_t1_dn5 = assign51770_e78358_d_n5;
        locals.var_t1_dn6 = assign51770_e78358_d_n6;
        locals.var_t1_dn7 = assign51770_e78358_d_n7;
        locals.var_t1_dn8 = assign51770_e78358_d_n8;
        locals.var_t1_dn9 = assign51770_e78358_d_n9;
        locals.var_t1_dn10 = assign51770_e78358_d_n10;
        locals.var_t1_dn11 = assign51770_e78358_d_n11;
        locals.var_t1_dn14 = assign51770_e78358_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign51780_e78374, assign51780_e78374_d_n0, assign51780_e78374_d_n2, assign51780_e78374_d_n4, assign51780_e78374_d_n5, assign51780_e78374_d_n6, assign51780_e78374_d_n7, assign51780_e78374_d_n8, assign51780_e78374_d_n9, assign51780_e78374_d_n10, assign51780_e78374_d_n11, assign51780_e78374_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1299 == 0.0)) {
        let assign51780_e78372: f64 = (locals.var_fmdvds * locals.var_t1);
        (assign51780_e78372, ((locals.var_fmdvds_dn0 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn0)), ((locals.var_fmdvds_dn2 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn2)), ((locals.var_fmdvds_dn4 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn4)), ((locals.var_fmdvds_dn5 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn5)), ((locals.var_fmdvds_dn6 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn6)), ((locals.var_fmdvds_dn7 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn7)), ((locals.var_fmdvds_dn8 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn8)), ((locals.var_fmdvds_dn9 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn9)), ((locals.var_fmdvds_dn10 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn10)), ((locals.var_fmdvds_dn11 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn11)), ((locals.var_fmdvds_dn14 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn14)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign51780_e78374;
        locals.var_lred_dn0 = assign51780_e78374_d_n0;
        locals.var_lred_dn2 = assign51780_e78374_d_n2;
        locals.var_lred_dn4 = assign51780_e78374_d_n4;
        locals.var_lred_dn5 = assign51780_e78374_d_n5;
        locals.var_lred_dn6 = assign51780_e78374_d_n6;
        locals.var_lred_dn7 = assign51780_e78374_d_n7;
        locals.var_lred_dn8 = assign51780_e78374_d_n8;
        locals.var_lred_dn9 = assign51780_e78374_d_n9;
        locals.var_lred_dn10 = assign51780_e78374_d_n10;
        locals.var_lred_dn11 = assign51780_e78374_d_n11;
        locals.var_lred_dn14 = assign51780_e78374_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign51790_e78387, assign51790_e78387_d_n0, assign51790_e78387_d_n2, assign51790_e78387_d_n4, assign51790_e78387_d_n5, assign51790_e78387_d_n6, assign51790_e78387_d_n7, assign51790_e78387_d_n8, assign51790_e78387_d_n9, assign51790_e78387_d_n10, assign51790_e78387_d_n11, assign51790_e78387_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51790_e78385: f64 = (locals.var_lred * locals.var_clmmod);
        (assign51790_e78385, (locals.var_lred_dn0 * locals.var_clmmod), (locals.var_lred_dn2 * locals.var_clmmod), (locals.var_lred_dn4 * locals.var_clmmod), (locals.var_lred_dn5 * locals.var_clmmod), (locals.var_lred_dn6 * locals.var_clmmod), (locals.var_lred_dn7 * locals.var_clmmod), (locals.var_lred_dn8 * locals.var_clmmod), (locals.var_lred_dn9 * locals.var_clmmod), (locals.var_lred_dn10 * locals.var_clmmod), (locals.var_lred_dn11 * locals.var_clmmod), (locals.var_lred_dn14 * locals.var_clmmod),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign51790_e78387;
        locals.var_lred_dn0 = assign51790_e78387_d_n0;
        locals.var_lred_dn2 = assign51790_e78387_d_n2;
        locals.var_lred_dn4 = assign51790_e78387_d_n4;
        locals.var_lred_dn5 = assign51790_e78387_d_n5;
        locals.var_lred_dn6 = assign51790_e78387_d_n6;
        locals.var_lred_dn7 = assign51790_e78387_d_n7;
        locals.var_lred_dn8 = assign51790_e78387_d_n8;
        locals.var_lred_dn9 = assign51790_e78387_d_n9;
        locals.var_lred_dn10 = assign51790_e78387_d_n10;
        locals.var_lred_dn11 = assign51790_e78387_d_n11;
        locals.var_lred_dn14 = assign51790_e78387_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign51800_e78400, assign51800_e78400_d_n0, assign51800_e78400_d_n2, assign51800_e78400_d_n4, assign51800_e78400_d_n5, assign51800_e78400_d_n6, assign51800_e78400_d_n7, assign51800_e78400_d_n8, assign51800_e78400_d_n9, assign51800_e78400_d_n10, assign51800_e78400_d_n11, assign51800_e78400_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51800_e78398: f64 = (locals.var_lch - locals.var_lred);
        (assign51800_e78398, (locals.var_lch_dn0 - locals.var_lred_dn0), (locals.var_lch_dn2 - locals.var_lred_dn2), (locals.var_lch_dn4 - locals.var_lred_dn4), (locals.var_lch_dn5 - locals.var_lred_dn5), (locals.var_lch_dn6 - locals.var_lred_dn6), (locals.var_lch_dn7 - locals.var_lred_dn7), (locals.var_lch_dn8 - locals.var_lred_dn8), (locals.var_lch_dn9 - locals.var_lred_dn9), (locals.var_lch_dn10 - locals.var_lred_dn10), (locals.var_lch_dn11 - locals.var_lred_dn11), (locals.var_lch_dn14 - locals.var_lred_dn14),)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    }
};
        locals.var_lch = assign51800_e78400;
        locals.var_lch_dn0 = assign51800_e78400_d_n0;
        locals.var_lch_dn2 = assign51800_e78400_d_n2;
        locals.var_lch_dn4 = assign51800_e78400_d_n4;
        locals.var_lch_dn5 = assign51800_e78400_d_n5;
        locals.var_lch_dn6 = assign51800_e78400_d_n6;
        locals.var_lch_dn7 = assign51800_e78400_d_n7;
        locals.var_lch_dn8 = assign51800_e78400_d_n8;
        locals.var_lch_dn9 = assign51800_e78400_d_n9;
        locals.var_lch_dn10 = assign51800_e78400_d_n10;
        locals.var_lch_dn11 = assign51800_e78400_d_n11;
        locals.var_lch_dn14 = assign51800_e78400_d_n14;
        locals.var_lch_rv = 0.0;

        let (assign51810_e78413, assign51810_e78413_d_n0, assign51810_e78413_d_n2, assign51810_e78413_d_n4, assign51810_e78413_d_n5, assign51810_e78413_d_n6, assign51810_e78413_d_n7, assign51810_e78413_d_n8, assign51810_e78413_d_n9, assign51810_e78413_d_n10, assign51810_e78413_d_n11, assign51810_e78413_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51810_e78411: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign51810_e78411, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign51810_e78413;
        locals.var_t2_dn0 = assign51810_e78413_d_n0;
        locals.var_t2_dn2 = assign51810_e78413_d_n2;
        locals.var_t2_dn4 = assign51810_e78413_d_n4;
        locals.var_t2_dn5 = assign51810_e78413_d_n5;
        locals.var_t2_dn6 = assign51810_e78413_d_n6;
        locals.var_t2_dn7 = assign51810_e78413_d_n7;
        locals.var_t2_dn8 = assign51810_e78413_d_n8;
        locals.var_t2_dn9 = assign51810_e78413_d_n9;
        locals.var_t2_dn10 = assign51810_e78413_d_n10;
        locals.var_t2_dn11 = assign51810_e78413_d_n11;
        locals.var_t2_dn14 = assign51810_e78413_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign51820_e78424, assign51820_e78424_d_n0, assign51820_e78424_d_n2, assign51820_e78424_d_n4, assign51820_e78424_d_n5, assign51820_e78424_d_n6, assign51820_e78424_d_n7, assign51820_e78424_d_n8, assign51820_e78424_d_n9, assign51820_e78424_d_n10, assign51820_e78424_d_n11, assign51820_e78424_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign51820_e78424;
        locals.var_t0_dn0 = assign51820_e78424_d_n0;
        locals.var_t0_dn2 = assign51820_e78424_d_n2;
        locals.var_t0_dn4 = assign51820_e78424_d_n4;
        locals.var_t0_dn5 = assign51820_e78424_d_n5;
        locals.var_t0_dn6 = assign51820_e78424_d_n6;
        locals.var_t0_dn7 = assign51820_e78424_d_n7;
        locals.var_t0_dn8 = assign51820_e78424_d_n8;
        locals.var_t0_dn9 = assign51820_e78424_d_n9;
        locals.var_t0_dn10 = assign51820_e78424_d_n10;
        locals.var_t0_dn11 = assign51820_e78424_d_n11;
        locals.var_t0_dn14 = assign51820_e78424_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign51830_e78443, assign51830_e78443_d_n0, assign51830_e78443_d_n2, assign51830_e78443_d_n4, assign51830_e78443_d_n5, assign51830_e78443_d_n6, assign51830_e78443_d_n7, assign51830_e78443_d_n8, assign51830_e78443_d_n9, assign51830_e78443_d_n10, assign51830_e78443_d_n11, assign51830_e78443_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51830_e78435: f64 = (locals.var_pds * locals.var_pds);
        let assign51830_e78437: f64 = (assign51830_e78435 + p.p262);
        let assign51830_e78438: f64 = (assign51830_e78437).sqrt();
        let assign51830_e78440: f64 = (p.p262).sqrt();
        let assign51830_e78441: f64 = (assign51830_e78438 - assign51830_e78440);
        (assign51830_e78441, (((locals.var_pds_dn0 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn0)) / (2.0 * assign51830_e78438)), (((locals.var_pds_dn2 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn2)) / (2.0 * assign51830_e78438)), (((locals.var_pds_dn4 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn4)) / (2.0 * assign51830_e78438)), (((locals.var_pds_dn5 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn5)) / (2.0 * assign51830_e78438)), (((locals.var_pds_dn6 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn6)) / (2.0 * assign51830_e78438)), (((locals.var_pds_dn7 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn7)) / (2.0 * assign51830_e78438)), (((locals.var_pds_dn8 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn8)) / (2.0 * assign51830_e78438)), (((locals.var_pds_dn9 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn9)) / (2.0 * assign51830_e78438)), (((locals.var_pds_dn10 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn10)) / (2.0 * assign51830_e78438)), (((locals.var_pds_dn11 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn11)) / (2.0 * assign51830_e78438)), (((locals.var_pds_dn14 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn14)) / (2.0 * assign51830_e78438)),)
    } else {
        (locals.var_pdsz, locals.var_pdsz_dn0, locals.var_pdsz_dn2, locals.var_pdsz_dn4, locals.var_pdsz_dn5, locals.var_pdsz_dn6, locals.var_pdsz_dn7, locals.var_pdsz_dn8, locals.var_pdsz_dn9, locals.var_pdsz_dn10, locals.var_pdsz_dn11, locals.var_pdsz_dn14,)
    }
};
        locals.var_pdsz = assign51830_e78443;
        locals.var_pdsz_dn0 = assign51830_e78443_d_n0;
        locals.var_pdsz_dn2 = assign51830_e78443_d_n2;
        locals.var_pdsz_dn4 = assign51830_e78443_d_n4;
        locals.var_pdsz_dn5 = assign51830_e78443_d_n5;
        locals.var_pdsz_dn6 = assign51830_e78443_d_n6;
        locals.var_pdsz_dn7 = assign51830_e78443_d_n7;
        locals.var_pdsz_dn8 = assign51830_e78443_d_n8;
        locals.var_pdsz_dn9 = assign51830_e78443_d_n9;
        locals.var_pdsz_dn10 = assign51830_e78443_d_n10;
        locals.var_pdsz_dn11 = assign51830_e78443_d_n11;
        locals.var_pdsz_dn14 = assign51830_e78443_d_n14;
        locals.var_pdsz_rv = 0.0;

        let (assign51840_e78458, assign51840_e78458_d_n0, assign51840_e78458_d_n2, assign51840_e78458_d_n4, assign51840_e78458_d_n5, assign51840_e78458_d_n6, assign51840_e78458_d_n7, assign51840_e78458_d_n8, assign51840_e78458_d_n9, assign51840_e78458_d_n10, assign51840_e78458_d_n11, assign51840_e78458_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51840_e78455: f64 = (locals.var_pdsz * locals.var_t0);
        let assign51840_e78456: f64 = (1.0 + assign51840_e78455);
        (assign51840_e78456, ((locals.var_pdsz_dn0 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn0)), ((locals.var_pdsz_dn2 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn2)), ((locals.var_pdsz_dn4 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn4)), ((locals.var_pdsz_dn5 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn5)), ((locals.var_pdsz_dn6 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn6)), ((locals.var_pdsz_dn7 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn7)), ((locals.var_pdsz_dn8 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn8)), ((locals.var_pdsz_dn9 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn9)), ((locals.var_pdsz_dn10 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn10)), ((locals.var_pdsz_dn11 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn11)), ((locals.var_pdsz_dn14 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign51840_e78458;
        locals.var_t4_dn0 = assign51840_e78458_d_n0;
        locals.var_t4_dn2 = assign51840_e78458_d_n2;
        locals.var_t4_dn4 = assign51840_e78458_d_n4;
        locals.var_t4_dn5 = assign51840_e78458_d_n5;
        locals.var_t4_dn6 = assign51840_e78458_d_n6;
        locals.var_t4_dn7 = assign51840_e78458_d_n7;
        locals.var_t4_dn8 = assign51840_e78458_d_n8;
        locals.var_t4_dn9 = assign51840_e78458_d_n9;
        locals.var_t4_dn10 = assign51840_e78458_d_n10;
        locals.var_t4_dn11 = assign51840_e78458_d_n11;
        locals.var_t4_dn14 = assign51840_e78458_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_186(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51850_e78471, assign51850_e78471_d_n0, assign51850_e78471_d_n2, assign51850_e78471_d_n4, assign51850_e78471_d_n5, assign51850_e78471_d_n6, assign51850_e78471_d_n7, assign51850_e78471_d_n8, assign51850_e78471_d_n9, assign51850_e78471_d_n10, assign51850_e78471_d_n11, assign51850_e78471_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51850_e78469: f64 = (locals.var_t2 * locals.var_qn0);
        (assign51850_e78469, ((locals.var_t2_dn0 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn0)), ((locals.var_t2_dn2 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn2)), ((locals.var_t2_dn4 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn4)), ((locals.var_t2_dn5 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn5)), ((locals.var_t2_dn6 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn6)), ((locals.var_t2_dn7 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn7)), ((locals.var_t2_dn8 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn8)), ((locals.var_t2_dn9 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn9)), ((locals.var_t2_dn10 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn10)), ((locals.var_t2_dn11 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn11)), ((locals.var_t2_dn14 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign51850_e78471;
        locals.var_t5_dn0 = assign51850_e78471_d_n0;
        locals.var_t5_dn2 = assign51850_e78471_d_n2;
        locals.var_t5_dn4 = assign51850_e78471_d_n4;
        locals.var_t5_dn5 = assign51850_e78471_d_n5;
        locals.var_t5_dn6 = assign51850_e78471_d_n6;
        locals.var_t5_dn7 = assign51850_e78471_d_n7;
        locals.var_t5_dn8 = assign51850_e78471_d_n8;
        locals.var_t5_dn9 = assign51850_e78471_d_n9;
        locals.var_t5_dn10 = assign51850_e78471_d_n10;
        locals.var_t5_dn11 = assign51850_e78471_d_n11;
        locals.var_t5_dn14 = assign51850_e78471_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign51860_e78484, assign51860_e78484_d_n0, assign51860_e78484_d_n2, assign51860_e78484_d_n4, assign51860_e78484_d_n5, assign51860_e78484_d_n6, assign51860_e78484_d_n7, assign51860_e78484_d_n8, assign51860_e78484_d_n9, assign51860_e78484_d_n10, assign51860_e78484_d_n11, assign51860_e78484_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51860_e78482: f64 = (locals.var_t5 / locals.var_t4);
        (assign51860_e78482, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign51860_e78484;
        locals.var_t3_dn0 = assign51860_e78484_d_n0;
        locals.var_t3_dn2 = assign51860_e78484_d_n2;
        locals.var_t3_dn4 = assign51860_e78484_d_n4;
        locals.var_t3_dn5 = assign51860_e78484_d_n5;
        locals.var_t3_dn6 = assign51860_e78484_d_n6;
        locals.var_t3_dn7 = assign51860_e78484_d_n7;
        locals.var_t3_dn8 = assign51860_e78484_d_n8;
        locals.var_t3_dn9 = assign51860_e78484_d_n9;
        locals.var_t3_dn10 = assign51860_e78484_d_n10;
        locals.var_t3_dn11 = assign51860_e78484_d_n11;
        locals.var_t3_dn14 = assign51860_e78484_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign51870_e78495, assign51870_e78495_d_n0, assign51870_e78495_d_n2, assign51870_e78495_d_n4, assign51870_e78495_d_n5, assign51870_e78495_d_n6, assign51870_e78495_d_n7, assign51870_e78495_d_n8, assign51870_e78495_d_n9, assign51870_e78495_d_n10, assign51870_e78495_d_n11, assign51870_e78495_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign51870_e78495;
        locals.var_eeff_dn0 = assign51870_e78495_d_n0;
        locals.var_eeff_dn2 = assign51870_e78495_d_n2;
        locals.var_eeff_dn4 = assign51870_e78495_d_n4;
        locals.var_eeff_dn5 = assign51870_e78495_d_n5;
        locals.var_eeff_dn6 = assign51870_e78495_d_n6;
        locals.var_eeff_dn7 = assign51870_e78495_d_n7;
        locals.var_eeff_dn8 = assign51870_e78495_d_n8;
        locals.var_eeff_dn9 = assign51870_e78495_d_n9;
        locals.var_eeff_dn10 = assign51870_e78495_d_n10;
        locals.var_eeff_dn11 = assign51870_e78495_d_n11;
        locals.var_eeff_dn14 = assign51870_e78495_d_n14;
        locals.var_eeff_rv = 0.0;

        let (assign51880_e78513, assign51880_e78513_d_n0, assign51880_e78513_d_n2, assign51880_e78513_d_n4, assign51880_e78513_d_n5, assign51880_e78513_d_n6, assign51880_e78513_d_n7, assign51880_e78513_d_n8, assign51880_e78513_d_n9, assign51880_e78513_d_n10, assign51880_e78513_d_n11, assign51880_e78513_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let (assign51880_e78511, assign51880_e78511_d_n0, assign51880_e78511_d_n2, assign51880_e78511_d_n4, assign51880_e78511_d_n5, assign51880_e78511_d_n6, assign51880_e78511_d_n7, assign51880_e78511_d_n8, assign51880_e78511_d_n9, assign51880_e78511_d_n10, assign51880_e78511_d_n11, assign51880_e78511_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign51880_e78510: f64 = (locals.var_eeff).powf(p.p160);
                (assign51880_e78510, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn0)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn2)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn4)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn5)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn6)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn7)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn8)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn9)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn10)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn11)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn14)) } } else { (assign51880_e78510 * (p.p160 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign51880_e78511, assign51880_e78511_d_n0, assign51880_e78511_d_n2, assign51880_e78511_d_n4, assign51880_e78511_d_n5, assign51880_e78511_d_n6, assign51880_e78511_d_n7, assign51880_e78511_d_n8, assign51880_e78511_d_n9, assign51880_e78511_d_n10, assign51880_e78511_d_n11, assign51880_e78511_d_n14,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign51880_e78513;
        locals.var_t8_dn0 = assign51880_e78513_d_n0;
        locals.var_t8_dn2 = assign51880_e78513_d_n2;
        locals.var_t8_dn4 = assign51880_e78513_d_n4;
        locals.var_t8_dn5 = assign51880_e78513_d_n5;
        locals.var_t8_dn6 = assign51880_e78513_d_n6;
        locals.var_t8_dn7 = assign51880_e78513_d_n7;
        locals.var_t8_dn8 = assign51880_e78513_d_n8;
        locals.var_t8_dn9 = assign51880_e78513_d_n9;
        locals.var_t8_dn10 = assign51880_e78513_d_n10;
        locals.var_t8_dn11 = assign51880_e78513_d_n11;
        locals.var_t8_dn14 = assign51880_e78513_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign51890_e78531, assign51890_e78531_d_n0, assign51890_e78531_d_n2, assign51890_e78531_d_n4, assign51890_e78531_d_n5, assign51890_e78531_d_n6, assign51890_e78531_d_n7, assign51890_e78531_d_n8, assign51890_e78531_d_n9, assign51890_e78531_d_n10, assign51890_e78531_d_n11, assign51890_e78531_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let (assign51890_e78529, assign51890_e78529_d_n0, assign51890_e78529_d_n2, assign51890_e78529_d_n4, assign51890_e78529_d_n5, assign51890_e78529_d_n6, assign51890_e78529_d_n7, assign51890_e78529_d_n8, assign51890_e78529_d_n9, assign51890_e78529_d_n10, assign51890_e78529_d_n11, assign51890_e78529_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign51890_e78528: f64 = (locals.var_eeff).powf(locals.var_muesr);
                (assign51890_e78528, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn0)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn2)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn4)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn5)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn6)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn7)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn8)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn9)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn10)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn11)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn14)) } } else { (assign51890_e78528 * (locals.var_muesr * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign51890_e78529, assign51890_e78529_d_n0, assign51890_e78529_d_n2, assign51890_e78529_d_n4, assign51890_e78529_d_n5, assign51890_e78529_d_n6, assign51890_e78529_d_n7, assign51890_e78529_d_n8, assign51890_e78529_d_n9, assign51890_e78529_d_n10, assign51890_e78529_d_n11, assign51890_e78529_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign51890_e78531;
        locals.var_t6_dn0 = assign51890_e78531_d_n0;
        locals.var_t6_dn2 = assign51890_e78531_d_n2;
        locals.var_t6_dn4 = assign51890_e78531_d_n4;
        locals.var_t6_dn5 = assign51890_e78531_d_n5;
        locals.var_t6_dn6 = assign51890_e78531_d_n6;
        locals.var_t6_dn7 = assign51890_e78531_d_n7;
        locals.var_t6_dn8 = assign51890_e78531_d_n8;
        locals.var_t6_dn9 = assign51890_e78531_d_n9;
        locals.var_t6_dn10 = assign51890_e78531_d_n10;
        locals.var_t6_dn11 = assign51890_e78531_d_n11;
        locals.var_t6_dn14 = assign51890_e78531_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign51900_e78544, assign51900_e78544_d_n0, assign51900_e78544_d_n2, assign51900_e78544_d_n4, assign51900_e78544_d_n5, assign51900_e78544_d_n6, assign51900_e78544_d_n7, assign51900_e78544_d_n8, assign51900_e78544_d_n9, assign51900_e78544_d_n10, assign51900_e78544_d_n11, assign51900_e78544_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51900_e78542: f64 = (1.6021918e-19 * 10000.0);
        (assign51900_e78542, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign51900_e78544;
        locals.var_t9_dn0 = assign51900_e78544_d_n0;
        locals.var_t9_dn2 = assign51900_e78544_d_n2;
        locals.var_t9_dn4 = assign51900_e78544_d_n4;
        locals.var_t9_dn5 = assign51900_e78544_d_n5;
        locals.var_t9_dn6 = assign51900_e78544_d_n6;
        locals.var_t9_dn7 = assign51900_e78544_d_n7;
        locals.var_t9_dn8 = assign51900_e78544_d_n8;
        locals.var_t9_dn9 = assign51900_e78544_d_n9;
        locals.var_t9_dn10 = assign51900_e78544_d_n10;
        locals.var_t9_dn11 = assign51900_e78544_d_n11;
        locals.var_t9_dn14 = assign51900_e78544_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign51910_e78557, assign51910_e78557_d_n0, assign51910_e78557_d_n2, assign51910_e78557_d_n4, assign51910_e78557_d_n5, assign51910_e78557_d_n6, assign51910_e78557_d_n7, assign51910_e78557_d_n8, assign51910_e78557_d_n9, assign51910_e78557_d_n10, assign51910_e78557_d_n11, assign51910_e78557_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51910_e78555: f64 = (locals.var_qn0 / locals.var_t9);
        (assign51910_e78555, (((locals.var_qn0_dn0 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn2 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn4 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn5 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn6 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn7 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn8 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn9 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn10 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn11 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn14 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign51910_e78557;
        locals.var_rns_dn0 = assign51910_e78557_d_n0;
        locals.var_rns_dn2 = assign51910_e78557_d_n2;
        locals.var_rns_dn4 = assign51910_e78557_d_n4;
        locals.var_rns_dn5 = assign51910_e78557_d_n5;
        locals.var_rns_dn6 = assign51910_e78557_d_n6;
        locals.var_rns_dn7 = assign51910_e78557_d_n7;
        locals.var_rns_dn8 = assign51910_e78557_d_n8;
        locals.var_rns_dn9 = assign51910_e78557_d_n9;
        locals.var_rns_dn10 = assign51910_e78557_d_n10;
        locals.var_rns_dn11 = assign51910_e78557_d_n11;
        locals.var_rns_dn14 = assign51910_e78557_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign51920_e78568, assign51920_e78568_d_n0, assign51920_e78568_d_n2, assign51920_e78568_d_n4, assign51920_e78568_d_n5, assign51920_e78568_d_n6, assign51920_e78568_d_n7, assign51920_e78568_d_n8, assign51920_e78568_d_n9, assign51920_e78568_d_n10, assign51920_e78568_d_n11, assign51920_e78568_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (locals.var_uc_muecb0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign51920_e78568;
        locals.var_t2_dn0 = assign51920_e78568_d_n0;
        locals.var_t2_dn2 = assign51920_e78568_d_n2;
        locals.var_t2_dn4 = assign51920_e78568_d_n4;
        locals.var_t2_dn5 = assign51920_e78568_d_n5;
        locals.var_t2_dn6 = assign51920_e78568_d_n6;
        locals.var_t2_dn7 = assign51920_e78568_d_n7;
        locals.var_t2_dn8 = assign51920_e78568_d_n8;
        locals.var_t2_dn9 = assign51920_e78568_d_n9;
        locals.var_t2_dn10 = assign51920_e78568_d_n10;
        locals.var_t2_dn11 = assign51920_e78568_d_n11;
        locals.var_t2_dn14 = assign51920_e78568_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign51930_e78603, assign51930_e78603_d_n0, assign51930_e78603_d_n2, assign51930_e78603_d_n4, assign51930_e78603_d_n5, assign51930_e78603_d_n6, assign51930_e78603_d_n7, assign51930_e78603_d_n8, assign51930_e78603_d_n9, assign51930_e78603_d_n10, assign51930_e78603_d_n11, assign51930_e78603_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51930_e78579: f64 = 1.0;
        let assign51930_e78583: f64 = (locals.var_uc_muecb1 * locals.var_t4);
        let assign51930_e78585: f64 = (assign51930_e78583 * locals.var_rns);
        let assign51930_e78587: f64 = (assign51930_e78585 / 100000000000.0);
        let assign51930_e78588: f64 = (locals.var_t2 + assign51930_e78587);
        let assign51930_e78589: f64 = (assign51930_e78579 / assign51930_e78588);
        let assign51930_e78592: f64 = locals.var_mphn0;
        let assign51930_e78594: f64 = (assign51930_e78592 * locals.var_t8);
        let assign51930_e78595: f64 = (assign51930_e78589 + assign51930_e78594);
        let assign51930_e78598: f64 = locals.var_t6;
        let assign51930_e78600: f64 = (assign51930_e78598 / locals.var_uc_muesr1);
        let assign51930_e78601: f64 = (assign51930_e78595 + assign51930_e78600);
        (assign51930_e78601, (((-((assign51930_e78579 * (locals.var_t2_dn0 + ((((locals.var_uc_muecb1 * locals.var_t4_dn0) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn0)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-((assign51930_e78579 * (locals.var_t2_dn2 + ((((locals.var_uc_muecb1 * locals.var_t4_dn2) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn2)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-((assign51930_e78579 * (locals.var_t2_dn4 + ((((locals.var_uc_muecb1 * locals.var_t4_dn4) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn4)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-((assign51930_e78579 * (locals.var_t2_dn5 + ((((locals.var_uc_muecb1 * locals.var_t4_dn5) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn5)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-((assign51930_e78579 * (locals.var_t2_dn6 + ((((locals.var_uc_muecb1 * locals.var_t4_dn6) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn6)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-((assign51930_e78579 * (locals.var_t2_dn7 + ((((locals.var_uc_muecb1 * locals.var_t4_dn7) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn7)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-((assign51930_e78579 * (locals.var_t2_dn8 + ((((locals.var_uc_muecb1 * locals.var_t4_dn8) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn8)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-((assign51930_e78579 * (locals.var_t2_dn9 + ((((locals.var_uc_muecb1 * locals.var_t4_dn9) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn9)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-((assign51930_e78579 * (locals.var_t2_dn10 + ((((locals.var_uc_muecb1 * locals.var_t4_dn10) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn10)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-((assign51930_e78579 * (locals.var_t2_dn11 + ((((locals.var_uc_muecb1 * locals.var_t4_dn11) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn11)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn11 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn11))) + (locals.var_t6_dn11 / locals.var_uc_muesr1)), (((-((assign51930_e78579 * (locals.var_t2_dn14 + ((((locals.var_uc_muecb1 * locals.var_t4_dn14) * locals.var_rns) + (assign51930_e78583 * locals.var_rns_dn14)) / 100000000000.0))) / (assign51930_e78588 * assign51930_e78588))) + ((locals.var_mphn0_dn14 * locals.var_t8) + (assign51930_e78592 * locals.var_t8_dn14))) + (locals.var_t6_dn14 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign51930_e78603;
        locals.var_t1_dn0 = assign51930_e78603_d_n0;
        locals.var_t1_dn2 = assign51930_e78603_d_n2;
        locals.var_t1_dn4 = assign51930_e78603_d_n4;
        locals.var_t1_dn5 = assign51930_e78603_d_n5;
        locals.var_t1_dn6 = assign51930_e78603_d_n6;
        locals.var_t1_dn7 = assign51930_e78603_d_n7;
        locals.var_t1_dn8 = assign51930_e78603_d_n8;
        locals.var_t1_dn9 = assign51930_e78603_d_n9;
        locals.var_t1_dn10 = assign51930_e78603_d_n10;
        locals.var_t1_dn11 = assign51930_e78603_d_n11;
        locals.var_t1_dn14 = assign51930_e78603_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign51940_e78616, assign51940_e78616_d_n0, assign51940_e78616_d_n2, assign51940_e78616_d_n4, assign51940_e78616_d_n5, assign51940_e78616_d_n6, assign51940_e78616_d_n7, assign51940_e78616_d_n8, assign51940_e78616_d_n9, assign51940_e78616_d_n10, assign51940_e78616_d_n11, assign51940_e78616_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51940_e78614: f64 = (1.0 / locals.var_t1);
        (assign51940_e78614, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign51940_e78616;
        locals.var_muun_dn0 = assign51940_e78616_d_n0;
        locals.var_muun_dn2 = assign51940_e78616_d_n2;
        locals.var_muun_dn4 = assign51940_e78616_d_n4;
        locals.var_muun_dn5 = assign51940_e78616_d_n5;
        locals.var_muun_dn6 = assign51940_e78616_d_n6;
        locals.var_muun_dn7 = assign51940_e78616_d_n7;
        locals.var_muun_dn8 = assign51940_e78616_d_n8;
        locals.var_muun_dn9 = assign51940_e78616_d_n9;
        locals.var_muun_dn10 = assign51940_e78616_d_n10;
        locals.var_muun_dn11 = assign51940_e78616_d_n11;
        locals.var_muun_dn14 = assign51940_e78616_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign51950_e78629, assign51950_e78629_d_n0, assign51950_e78629_d_n2, assign51950_e78629_d_n4, assign51950_e78629_d_n5, assign51950_e78629_d_n6, assign51950_e78629_d_n7, assign51950_e78629_d_n8, assign51950_e78629_d_n9, assign51950_e78629_d_n10, assign51950_e78629_d_n11, assign51950_e78629_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51950_e78627: f64 = (locals.var_muun / 10000.0);
        (assign51950_e78627, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign51950_e78629;
        locals.var_muun_dn0 = assign51950_e78629_d_n0;
        locals.var_muun_dn2 = assign51950_e78629_d_n2;
        locals.var_muun_dn4 = assign51950_e78629_d_n4;
        locals.var_muun_dn5 = assign51950_e78629_d_n5;
        locals.var_muun_dn6 = assign51950_e78629_d_n6;
        locals.var_muun_dn7 = assign51950_e78629_d_n7;
        locals.var_muun_dn8 = assign51950_e78629_d_n8;
        locals.var_muun_dn9 = assign51950_e78629_d_n9;
        locals.var_muun_dn10 = assign51950_e78629_d_n10;
        locals.var_muun_dn11 = assign51950_e78629_d_n11;
        locals.var_muun_dn14 = assign51950_e78629_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign51960_e78646, assign51960_e78646_d_n0, assign51960_e78646_d_n2, assign51960_e78646_d_n4, assign51960_e78646_d_n5, assign51960_e78646_d_n6, assign51960_e78646_d_n7, assign51960_e78646_d_n8, assign51960_e78646_d_n9, assign51960_e78646_d_n10, assign51960_e78646_d_n11, assign51960_e78646_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51960_e78641: f64 = (locals.var_qn0 + 1e-25);
        let assign51960_e78642: f64 = (locals.var_beta * assign51960_e78641);
        let assign51960_e78644: f64 = (assign51960_e78642 * locals.var_lch);
        (assign51960_e78644, ((((locals.var_beta_dn0 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn0)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn2)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn4)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn5)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn6)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn7)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn8)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn9)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn10)), ((((locals.var_beta_dn11 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn11)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn11)), ((((locals.var_beta_dn14 * assign51960_e78641) + (locals.var_beta * locals.var_qn0_dn14)) * locals.var_lch) + (assign51960_e78642 * locals.var_lch_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign51960_e78646;
        locals.var_t2_dn0 = assign51960_e78646_d_n0;
        locals.var_t2_dn2 = assign51960_e78646_d_n2;
        locals.var_t2_dn4 = assign51960_e78646_d_n4;
        locals.var_t2_dn5 = assign51960_e78646_d_n5;
        locals.var_t2_dn6 = assign51960_e78646_d_n6;
        locals.var_t2_dn7 = assign51960_e78646_d_n7;
        locals.var_t2_dn8 = assign51960_e78646_d_n8;
        locals.var_t2_dn9 = assign51960_e78646_d_n9;
        locals.var_t2_dn10 = assign51960_e78646_d_n10;
        locals.var_t2_dn11 = assign51960_e78646_d_n11;
        locals.var_t2_dn14 = assign51960_e78646_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign51970_e78659, assign51970_e78659_d_n0, assign51970_e78659_d_n2, assign51970_e78659_d_n4, assign51970_e78659_d_n5, assign51970_e78659_d_n6, assign51970_e78659_d_n7, assign51970_e78659_d_n8, assign51970_e78659_d_n9, assign51970_e78659_d_n10, assign51970_e78659_d_n11, assign51970_e78659_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51970_e78657: f64 = (1.0 / locals.var_t2);
        (assign51970_e78657, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign51970_e78659;
        locals.var_t1_dn0 = assign51970_e78659_d_n0;
        locals.var_t1_dn2 = assign51970_e78659_d_n2;
        locals.var_t1_dn4 = assign51970_e78659_d_n4;
        locals.var_t1_dn5 = assign51970_e78659_d_n5;
        locals.var_t1_dn6 = assign51970_e78659_d_n6;
        locals.var_t1_dn7 = assign51970_e78659_d_n7;
        locals.var_t1_dn8 = assign51970_e78659_d_n8;
        locals.var_t1_dn9 = assign51970_e78659_d_n9;
        locals.var_t1_dn10 = assign51970_e78659_d_n10;
        locals.var_t1_dn11 = assign51970_e78659_d_n11;
        locals.var_t1_dn14 = assign51970_e78659_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign51980_e78672, assign51980_e78672_d_n0, assign51980_e78672_d_n2, assign51980_e78672_d_n4, assign51980_e78672_d_n5, assign51980_e78672_d_n6, assign51980_e78672_d_n7, assign51980_e78672_d_n8, assign51980_e78672_d_n9, assign51980_e78672_d_n10, assign51980_e78672_d_n11, assign51980_e78672_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51980_e78670: f64 = (locals.var_idd * locals.var_t1);
        (assign51980_e78670, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn11 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn11)), ((locals.var_idd_dn14 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign51980_e78672;
        locals.var_ty_dn0 = assign51980_e78672_d_n0;
        locals.var_ty_dn2 = assign51980_e78672_d_n2;
        locals.var_ty_dn4 = assign51980_e78672_d_n4;
        locals.var_ty_dn5 = assign51980_e78672_d_n5;
        locals.var_ty_dn6 = assign51980_e78672_d_n6;
        locals.var_ty_dn7 = assign51980_e78672_d_n7;
        locals.var_ty_dn8 = assign51980_e78672_d_n8;
        locals.var_ty_dn9 = assign51980_e78672_d_n9;
        locals.var_ty_dn10 = assign51980_e78672_d_n10;
        locals.var_ty_dn11 = assign51980_e78672_d_n11;
        locals.var_ty_dn14 = assign51980_e78672_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign51990_e78687, assign51990_e78687_d_n0, assign51990_e78687_d_n2, assign51990_e78687_d_n4, assign51990_e78687_d_n5, assign51990_e78687_d_n6, assign51990_e78687_d_n7, assign51990_e78687_d_n8, assign51990_e78687_d_n9, assign51990_e78687_d_n10, assign51990_e78687_d_n11, assign51990_e78687_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign51990_e78683: f64 = (0.2 * locals.var_vmaxe);
        let assign51990_e78685: f64 = (assign51990_e78683 / locals.var_muun);
        (assign51990_e78685, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn14) * locals.var_muun) - (assign51990_e78683 * locals.var_muun_dn14)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign51990_e78687;
        locals.var_t2_dn0 = assign51990_e78687_d_n0;
        locals.var_t2_dn2 = assign51990_e78687_d_n2;
        locals.var_t2_dn4 = assign51990_e78687_d_n4;
        locals.var_t2_dn5 = assign51990_e78687_d_n5;
        locals.var_t2_dn6 = assign51990_e78687_d_n6;
        locals.var_t2_dn7 = assign51990_e78687_d_n7;
        locals.var_t2_dn8 = assign51990_e78687_d_n8;
        locals.var_t2_dn9 = assign51990_e78687_d_n9;
        locals.var_t2_dn10 = assign51990_e78687_d_n10;
        locals.var_t2_dn11 = assign51990_e78687_d_n11;
        locals.var_t2_dn14 = assign51990_e78687_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign52000_e78705, assign52000_e78705_d_n0, assign52000_e78705_d_n2, assign52000_e78705_d_n4, assign52000_e78705_d_n5, assign52000_e78705_d_n6, assign52000_e78705_d_n7, assign52000_e78705_d_n8, assign52000_e78705_d_n9, assign52000_e78705_d_n10, assign52000_e78705_d_n11, assign52000_e78705_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign52000_e78698: f64 = (locals.var_ty * locals.var_ty);
        let assign52000_e78701: f64 = (locals.var_t2 * locals.var_t2);
        let assign52000_e78702: f64 = (assign52000_e78698 + assign52000_e78701);
        let assign52000_e78703: f64 = (assign52000_e78702).sqrt();
        (assign52000_e78703, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign52000_e78703)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign52000_e78703)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign52000_e78703)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign52000_e78703)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign52000_e78703)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign52000_e78703)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign52000_e78703)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign52000_e78703)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign52000_e78703)), ((((locals.var_ty_dn11 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn11)) + ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (2.0 * assign52000_e78703)), ((((locals.var_ty_dn14 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn14)) + ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (2.0 * assign52000_e78703)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    }
};
        locals.var_ey = assign52000_e78705;
        locals.var_ey_dn0 = assign52000_e78705_d_n0;
        locals.var_ey_dn2 = assign52000_e78705_d_n2;
        locals.var_ey_dn4 = assign52000_e78705_d_n4;
        locals.var_ey_dn5 = assign52000_e78705_d_n5;
        locals.var_ey_dn6 = assign52000_e78705_d_n6;
        locals.var_ey_dn7 = assign52000_e78705_d_n7;
        locals.var_ey_dn8 = assign52000_e78705_d_n8;
        locals.var_ey_dn9 = assign52000_e78705_d_n9;
        locals.var_ey_dn10 = assign52000_e78705_d_n10;
        locals.var_ey_dn11 = assign52000_e78705_d_n11;
        locals.var_ey_dn14 = assign52000_e78705_d_n14;
        locals.var_ey_rv = 0.0;

        let (assign52010_e78718, assign52010_e78718_d_n0, assign52010_e78718_d_n2, assign52010_e78718_d_n4, assign52010_e78718_d_n5, assign52010_e78718_d_n6, assign52010_e78718_d_n7, assign52010_e78718_d_n8, assign52010_e78718_d_n9, assign52010_e78718_d_n10, assign52010_e78718_d_n11, assign52010_e78718_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign52010_e78716: f64 = (1.0 / locals.var_ey);
        (assign52010_e78716, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn11 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn14 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign52010_e78718;
        locals.var_t4_dn0 = assign52010_e78718_d_n0;
        locals.var_t4_dn2 = assign52010_e78718_d_n2;
        locals.var_t4_dn4 = assign52010_e78718_d_n4;
        locals.var_t4_dn5 = assign52010_e78718_d_n5;
        locals.var_t4_dn6 = assign52010_e78718_d_n6;
        locals.var_t4_dn7 = assign52010_e78718_d_n7;
        locals.var_t4_dn8 = assign52010_e78718_d_n8;
        locals.var_t4_dn9 = assign52010_e78718_d_n9;
        locals.var_t4_dn10 = assign52010_e78718_d_n10;
        locals.var_t4_dn11 = assign52010_e78718_d_n11;
        locals.var_t4_dn14 = assign52010_e78718_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign52020_e78731, assign52020_e78731_d_n0, assign52020_e78731_d_n2, assign52020_e78731_d_n4, assign52020_e78731_d_n5, assign52020_e78731_d_n6, assign52020_e78731_d_n7, assign52020_e78731_d_n8, assign52020_e78731_d_n9, assign52020_e78731_d_n10, assign52020_e78731_d_n11, assign52020_e78731_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign52020_e78729: f64 = (locals.var_muun * locals.var_ey);
        (assign52020_e78729, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn14 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn14)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn14,)
    }
};
        locals.var_em = assign52020_e78731;
        locals.var_em_dn0 = assign52020_e78731_d_n0;
        locals.var_em_dn2 = assign52020_e78731_d_n2;
        locals.var_em_dn4 = assign52020_e78731_d_n4;
        locals.var_em_dn5 = assign52020_e78731_d_n5;
        locals.var_em_dn6 = assign52020_e78731_d_n6;
        locals.var_em_dn7 = assign52020_e78731_d_n7;
        locals.var_em_dn8 = assign52020_e78731_d_n8;
        locals.var_em_dn9 = assign52020_e78731_d_n9;
        locals.var_em_dn10 = assign52020_e78731_d_n10;
        locals.var_em_dn11 = assign52020_e78731_d_n11;
        locals.var_em_dn14 = assign52020_e78731_d_n14;
        locals.var_em_rv = 0.0;

        let (assign52030_e78744, assign52030_e78744_d_n0, assign52030_e78744_d_n2, assign52030_e78744_d_n4, assign52030_e78744_d_n5, assign52030_e78744_d_n6, assign52030_e78744_d_n7, assign52030_e78744_d_n8, assign52030_e78744_d_n9, assign52030_e78744_d_n10, assign52030_e78744_d_n11, assign52030_e78744_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign52030_e78742: f64 = (locals.var_em / locals.var_vmaxe);
        (assign52030_e78742, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn14 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn14)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign52030_e78744;
        locals.var_t1_dn0 = assign52030_e78744_d_n0;
        locals.var_t1_dn2 = assign52030_e78744_d_n2;
        locals.var_t1_dn4 = assign52030_e78744_d_n4;
        locals.var_t1_dn5 = assign52030_e78744_d_n5;
        locals.var_t1_dn6 = assign52030_e78744_d_n6;
        locals.var_t1_dn7 = assign52030_e78744_d_n7;
        locals.var_t1_dn8 = assign52030_e78744_d_n8;
        locals.var_t1_dn9 = assign52030_e78744_d_n9;
        locals.var_t1_dn10 = assign52030_e78744_d_n10;
        locals.var_t1_dn11 = assign52030_e78744_d_n11;
        locals.var_t1_dn14 = assign52030_e78744_d_n14;
        locals.var_t1_rv = 0.0;

        let assign52040_e78748: f64 = (10.0 * 2.220446049250313e-16);
        let assign52040_e78749: f64 = (1.0 - assign52040_e78748);
        let assign52040_e78756: f64 = (10.0 * 2.220446049250313e-16);
        let assign52040_e78757: f64 = (1.0 + assign52040_e78756);
        let assign52040_e78759: f64 = if ((assign52040_e78749 <= p.p178) && (p.p178 <= assign52040_e78757)) { 1.0 } else { 0.0 };
        locals.var_guard1318 = assign52040_e78759;
        locals.var_guard1318_rv = 0.0;

        let (assign52050_e78772, assign52050_e78772_d_n0, assign52050_e78772_d_n2, assign52050_e78772_d_n4, assign52050_e78772_d_n5, assign52050_e78772_d_n6, assign52050_e78772_d_n7, assign52050_e78772_d_n8, assign52050_e78772_d_n9, assign52050_e78772_d_n10, assign52050_e78772_d_n11, assign52050_e78772_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1318 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign52050_e78772;
        locals.var_t2_dn0 = assign52050_e78772_d_n0;
        locals.var_t2_dn2 = assign52050_e78772_d_n2;
        locals.var_t2_dn4 = assign52050_e78772_d_n4;
        locals.var_t2_dn5 = assign52050_e78772_d_n5;
        locals.var_t2_dn6 = assign52050_e78772_d_n6;
        locals.var_t2_dn7 = assign52050_e78772_d_n7;
        locals.var_t2_dn8 = assign52050_e78772_d_n8;
        locals.var_t2_dn9 = assign52050_e78772_d_n9;
        locals.var_t2_dn10 = assign52050_e78772_d_n10;
        locals.var_t2_dn11 = assign52050_e78772_d_n11;
        locals.var_t2_dn14 = assign52050_e78772_d_n14;
        locals.var_t2_rv = 0.0;

        let assign52060_e78776: f64 = (10.0 * 2.220446049250313e-16);
        let assign52060_e78777: f64 = (2.0 - assign52060_e78776);
        let assign52060_e78784: f64 = (10.0 * 2.220446049250313e-16);
        let assign52060_e78785: f64 = (2.0 + assign52060_e78784);
        let assign52060_e78787: f64 = if ((assign52060_e78777 <= p.p178) && (p.p178 <= assign52060_e78785)) { 1.0 } else { 0.0 };
        locals.var_guard1319 = assign52060_e78787;
        locals.var_guard1319_rv = 0.0;

        let (assign52070_e78805, assign52070_e78805_d_n0, assign52070_e78805_d_n2, assign52070_e78805_d_n4, assign52070_e78805_d_n5, assign52070_e78805_d_n6, assign52070_e78805_d_n7, assign52070_e78805_d_n8, assign52070_e78805_d_n9, assign52070_e78805_d_n10, assign52070_e78805_d_n11, assign52070_e78805_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1318 == 0.0)) && (locals.var_guard1319 != 0.0)) {
        let assign52070_e78803: f64 = (locals.var_t1 * locals.var_t1);
        (assign52070_e78803, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign52070_e78805;
        locals.var_t2_dn0 = assign52070_e78805_d_n0;
        locals.var_t2_dn2 = assign52070_e78805_d_n2;
        locals.var_t2_dn4 = assign52070_e78805_d_n4;
        locals.var_t2_dn5 = assign52070_e78805_d_n5;
        locals.var_t2_dn6 = assign52070_e78805_d_n6;
        locals.var_t2_dn7 = assign52070_e78805_d_n7;
        locals.var_t2_dn8 = assign52070_e78805_d_n8;
        locals.var_t2_dn9 = assign52070_e78805_d_n9;
        locals.var_t2_dn10 = assign52070_e78805_d_n10;
        locals.var_t2_dn11 = assign52070_e78805_d_n11;
        locals.var_t2_dn14 = assign52070_e78805_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_187(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52080_e78829, assign52080_e78829_d_n0, assign52080_e78829_d_n2, assign52080_e78829_d_n4, assign52080_e78829_d_n5, assign52080_e78829_d_n6, assign52080_e78829_d_n7, assign52080_e78829_d_n8, assign52080_e78829_d_n9, assign52080_e78829_d_n10, assign52080_e78829_d_n11, assign52080_e78829_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1318 == 0.0)) && (locals.var_guard1319 == 0.0)) {
        let (assign52080_e78827, assign52080_e78827_d_n0, assign52080_e78827_d_n2, assign52080_e78827_d_n4, assign52080_e78827_d_n5, assign52080_e78827_d_n6, assign52080_e78827_d_n7, assign52080_e78827_d_n8, assign52080_e78827_d_n9, assign52080_e78827_d_n10, assign52080_e78827_d_n11, assign52080_e78827_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52080_e78826: f64 = (locals.var_t1).powf(p.p178);
                (assign52080_e78826, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn0)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn2)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn4)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn5)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn6)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn7)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn8)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn9)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn10)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn11)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn14)) } } else { (assign52080_e78826 * (p.p178 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign52080_e78827, assign52080_e78827_d_n0, assign52080_e78827_d_n2, assign52080_e78827_d_n4, assign52080_e78827_d_n5, assign52080_e78827_d_n6, assign52080_e78827_d_n7, assign52080_e78827_d_n8, assign52080_e78827_d_n9, assign52080_e78827_d_n10, assign52080_e78827_d_n11, assign52080_e78827_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign52080_e78829;
        locals.var_t2_dn0 = assign52080_e78829_d_n0;
        locals.var_t2_dn2 = assign52080_e78829_d_n2;
        locals.var_t2_dn4 = assign52080_e78829_d_n4;
        locals.var_t2_dn5 = assign52080_e78829_d_n5;
        locals.var_t2_dn6 = assign52080_e78829_d_n6;
        locals.var_t2_dn7 = assign52080_e78829_d_n7;
        locals.var_t2_dn8 = assign52080_e78829_d_n8;
        locals.var_t2_dn9 = assign52080_e78829_d_n9;
        locals.var_t2_dn10 = assign52080_e78829_d_n10;
        locals.var_t2_dn11 = assign52080_e78829_d_n11;
        locals.var_t2_dn14 = assign52080_e78829_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign52090_e78842, assign52090_e78842_d_n0, assign52090_e78842_d_n2, assign52090_e78842_d_n4, assign52090_e78842_d_n5, assign52090_e78842_d_n6, assign52090_e78842_d_n7, assign52090_e78842_d_n8, assign52090_e78842_d_n9, assign52090_e78842_d_n10, assign52090_e78842_d_n11, assign52090_e78842_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign52090_e78840: f64 = (1.0 + locals.var_t2);
        (assign52090_e78840, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign52090_e78842;
        locals.var_t4_dn0 = assign52090_e78842_d_n0;
        locals.var_t4_dn2 = assign52090_e78842_d_n2;
        locals.var_t4_dn4 = assign52090_e78842_d_n4;
        locals.var_t4_dn5 = assign52090_e78842_d_n5;
        locals.var_t4_dn6 = assign52090_e78842_d_n6;
        locals.var_t4_dn7 = assign52090_e78842_d_n7;
        locals.var_t4_dn8 = assign52090_e78842_d_n8;
        locals.var_t4_dn9 = assign52090_e78842_d_n9;
        locals.var_t4_dn10 = assign52090_e78842_d_n10;
        locals.var_t4_dn11 = assign52090_e78842_d_n11;
        locals.var_t4_dn14 = assign52090_e78842_d_n14;
        locals.var_t4_rv = 0.0;

        let assign52100_e78846: f64 = (10.0 * 2.220446049250313e-16);
        let assign52100_e78847: f64 = (1.0 - assign52100_e78846);
        let assign52100_e78854: f64 = (10.0 * 2.220446049250313e-16);
        let assign52100_e78855: f64 = (1.0 + assign52100_e78854);
        let assign52100_e78857: f64 = if ((assign52100_e78847 <= p.p178) && (p.p178 <= assign52100_e78855)) { 1.0 } else { 0.0 };
        locals.var_guard1320 = assign52100_e78857;
        locals.var_guard1320_rv = 0.0;

        let (assign52110_e78872, assign52110_e78872_d_n0, assign52110_e78872_d_n2, assign52110_e78872_d_n4, assign52110_e78872_d_n5, assign52110_e78872_d_n6, assign52110_e78872_d_n7, assign52110_e78872_d_n8, assign52110_e78872_d_n9, assign52110_e78872_d_n10, assign52110_e78872_d_n11, assign52110_e78872_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1320 != 0.0)) {
        let assign52110_e78870: f64 = (1.0 / locals.var_t4);
        (assign52110_e78870, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign52110_e78872;
        locals.var_t5_dn0 = assign52110_e78872_d_n0;
        locals.var_t5_dn2 = assign52110_e78872_d_n2;
        locals.var_t5_dn4 = assign52110_e78872_d_n4;
        locals.var_t5_dn5 = assign52110_e78872_d_n5;
        locals.var_t5_dn6 = assign52110_e78872_d_n6;
        locals.var_t5_dn7 = assign52110_e78872_d_n7;
        locals.var_t5_dn8 = assign52110_e78872_d_n8;
        locals.var_t5_dn9 = assign52110_e78872_d_n9;
        locals.var_t5_dn10 = assign52110_e78872_d_n10;
        locals.var_t5_dn11 = assign52110_e78872_d_n11;
        locals.var_t5_dn14 = assign52110_e78872_d_n14;
        locals.var_t5_rv = 0.0;

        let assign52120_e78876: f64 = (10.0 * 2.220446049250313e-16);
        let assign52120_e78877: f64 = (2.0 - assign52120_e78876);
        let assign52120_e78884: f64 = (10.0 * 2.220446049250313e-16);
        let assign52120_e78885: f64 = (2.0 + assign52120_e78884);
        let assign52120_e78887: f64 = if ((assign52120_e78877 <= p.p178) && (p.p178 <= assign52120_e78885)) { 1.0 } else { 0.0 };
        locals.var_guard1321 = assign52120_e78887;
        locals.var_guard1321_rv = 0.0;

        let (assign52130_e78906, assign52130_e78906_d_n0, assign52130_e78906_d_n2, assign52130_e78906_d_n4, assign52130_e78906_d_n5, assign52130_e78906_d_n6, assign52130_e78906_d_n7, assign52130_e78906_d_n8, assign52130_e78906_d_n9, assign52130_e78906_d_n10, assign52130_e78906_d_n11, assign52130_e78906_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1320 == 0.0)) && (locals.var_guard1321 != 0.0)) {
        let assign52130_e78903: f64 = (locals.var_t4).sqrt();
        let assign52130_e78904: f64 = (1.0 / assign52130_e78903);
        (assign52130_e78904, (-((locals.var_t4_dn0 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))), (-((locals.var_t4_dn2 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))), (-((locals.var_t4_dn4 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))), (-((locals.var_t4_dn5 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))), (-((locals.var_t4_dn6 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))), (-((locals.var_t4_dn7 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))), (-((locals.var_t4_dn8 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))), (-((locals.var_t4_dn9 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))), (-((locals.var_t4_dn10 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))), (-((locals.var_t4_dn11 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))), (-((locals.var_t4_dn14 / (2.0 * assign52130_e78903)) / (assign52130_e78903 * assign52130_e78903))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign52130_e78906;
        locals.var_t5_dn0 = assign52130_e78906_d_n0;
        locals.var_t5_dn2 = assign52130_e78906_d_n2;
        locals.var_t5_dn4 = assign52130_e78906_d_n4;
        locals.var_t5_dn5 = assign52130_e78906_d_n5;
        locals.var_t5_dn6 = assign52130_e78906_d_n6;
        locals.var_t5_dn7 = assign52130_e78906_d_n7;
        locals.var_t5_dn8 = assign52130_e78906_d_n8;
        locals.var_t5_dn9 = assign52130_e78906_d_n9;
        locals.var_t5_dn10 = assign52130_e78906_d_n10;
        locals.var_t5_dn11 = assign52130_e78906_d_n11;
        locals.var_t5_dn14 = assign52130_e78906_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign52140_e78933, assign52140_e78933_d_n0, assign52140_e78933_d_n2, assign52140_e78933_d_n4, assign52140_e78933_d_n5, assign52140_e78933_d_n6, assign52140_e78933_d_n7, assign52140_e78933_d_n8, assign52140_e78933_d_n9, assign52140_e78933_d_n10, assign52140_e78933_d_n11, assign52140_e78933_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1320 == 0.0)) && (locals.var_guard1321 == 0.0)) {
        let (assign52140_e78931, assign52140_e78931_d_n0, assign52140_e78931_d_n2, assign52140_e78931_d_n4, assign52140_e78931_d_n5, assign52140_e78931_d_n6, assign52140_e78931_d_n7, assign52140_e78931_d_n8, assign52140_e78931_d_n9, assign52140_e78931_d_n10, assign52140_e78931_d_n11, assign52140_e78931_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52140_e78927: f64 = (-1.0);
                let assign52140_e78929: f64 = (assign52140_e78927 / p.p178);
                let assign52140_e78930: f64 = (locals.var_t4).powf(assign52140_e78929);
                (assign52140_e78930, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn0)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn2)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn4)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn5)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn6)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn7)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn8)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn9)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn10)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn11)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52140_e78929) as f64).is_finite() && ((assign52140_e78929) as f64).fract() == 0.0 { if assign52140_e78929 == 0.0 { 0.0 } else { (assign52140_e78929 * ((locals.var_t4).powf(assign52140_e78929 - 1.0) * locals.var_t4_dn14)) } } else { (assign52140_e78930 * (assign52140_e78929 * (locals.var_t4_dn14 / locals.var_t4))) },)
            }
        };
        (assign52140_e78931, assign52140_e78931_d_n0, assign52140_e78931_d_n2, assign52140_e78931_d_n4, assign52140_e78931_d_n5, assign52140_e78931_d_n6, assign52140_e78931_d_n7, assign52140_e78931_d_n8, assign52140_e78931_d_n9, assign52140_e78931_d_n10, assign52140_e78931_d_n11, assign52140_e78931_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign52140_e78933;
        locals.var_t5_dn0 = assign52140_e78933_d_n0;
        locals.var_t5_dn2 = assign52140_e78933_d_n2;
        locals.var_t5_dn4 = assign52140_e78933_d_n4;
        locals.var_t5_dn5 = assign52140_e78933_d_n5;
        locals.var_t5_dn6 = assign52140_e78933_d_n6;
        locals.var_t5_dn7 = assign52140_e78933_d_n7;
        locals.var_t5_dn8 = assign52140_e78933_d_n8;
        locals.var_t5_dn9 = assign52140_e78933_d_n9;
        locals.var_t5_dn10 = assign52140_e78933_d_n10;
        locals.var_t5_dn11 = assign52140_e78933_d_n11;
        locals.var_t5_dn14 = assign52140_e78933_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign52150_e78946, assign52150_e78946_d_n0, assign52150_e78946_d_n2, assign52150_e78946_d_n4, assign52150_e78946_d_n5, assign52150_e78946_d_n6, assign52150_e78946_d_n7, assign52150_e78946_d_n8, assign52150_e78946_d_n9, assign52150_e78946_d_n10, assign52150_e78946_d_n11, assign52150_e78946_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign52150_e78944: f64 = (locals.var_muun * locals.var_t5);
        (assign52150_e78944, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn11 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn11)), ((locals.var_muun_dn14 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    }
};
        locals.var_mu = assign52150_e78946;
        locals.var_mu_dn0 = assign52150_e78946_d_n0;
        locals.var_mu_dn2 = assign52150_e78946_d_n2;
        locals.var_mu_dn4 = assign52150_e78946_d_n4;
        locals.var_mu_dn5 = assign52150_e78946_d_n5;
        locals.var_mu_dn6 = assign52150_e78946_d_n6;
        locals.var_mu_dn7 = assign52150_e78946_d_n7;
        locals.var_mu_dn8 = assign52150_e78946_d_n8;
        locals.var_mu_dn9 = assign52150_e78946_d_n9;
        locals.var_mu_dn10 = assign52150_e78946_d_n10;
        locals.var_mu_dn11 = assign52150_e78946_d_n11;
        locals.var_mu_dn14 = assign52150_e78946_d_n14;
        locals.var_mu_rv = 0.0;

        let (assign52160_e78957, assign52160_e78957_d_n0, assign52160_e78957_d_n2, assign52160_e78957_d_n4, assign52160_e78957_d_n5, assign52160_e78957_d_n6, assign52160_e78957_d_n7, assign52160_e78957_d_n8, assign52160_e78957_d_n9, assign52160_e78957_d_n10, assign52160_e78957_d_n11, assign52160_e78957_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    } else {
        (locals.var_mu_acc, locals.var_mu_acc_dn0, locals.var_mu_acc_dn2, locals.var_mu_acc_dn4, locals.var_mu_acc_dn5, locals.var_mu_acc_dn6, locals.var_mu_acc_dn7, locals.var_mu_acc_dn8, locals.var_mu_acc_dn9, locals.var_mu_acc_dn10, locals.var_mu_acc_dn11, locals.var_mu_acc_dn14,)
    }
};
        locals.var_mu_acc = assign52160_e78957;
        locals.var_mu_acc_dn0 = assign52160_e78957_d_n0;
        locals.var_mu_acc_dn2 = assign52160_e78957_d_n2;
        locals.var_mu_acc_dn4 = assign52160_e78957_d_n4;
        locals.var_mu_acc_dn5 = assign52160_e78957_d_n5;
        locals.var_mu_acc_dn6 = assign52160_e78957_d_n6;
        locals.var_mu_acc_dn7 = assign52160_e78957_d_n7;
        locals.var_mu_acc_dn8 = assign52160_e78957_d_n8;
        locals.var_mu_acc_dn9 = assign52160_e78957_d_n9;
        locals.var_mu_acc_dn10 = assign52160_e78957_d_n10;
        locals.var_mu_acc_dn11 = assign52160_e78957_d_n11;
        locals.var_mu_acc_dn14 = assign52160_e78957_d_n14;
        locals.var_mu_acc_rv = 0.0;

        let (assign52170_e78968, assign52170_e78968_d_n0, assign52170_e78968_d_n2, assign52170_e78968_d_n4, assign52170_e78968_d_n5, assign52170_e78968_d_n6, assign52170_e78968_d_n7, assign52170_e78968_d_n8, assign52170_e78968_d_n9, assign52170_e78968_d_n10, assign52170_e78968_d_n11, assign52170_e78968_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    } else {
        (locals.var_ey_acc__blk1118, locals.var_ey_acc__blk1118_dn0, locals.var_ey_acc__blk1118_dn2, locals.var_ey_acc__blk1118_dn4, locals.var_ey_acc__blk1118_dn5, locals.var_ey_acc__blk1118_dn6, locals.var_ey_acc__blk1118_dn7, locals.var_ey_acc__blk1118_dn8, locals.var_ey_acc__blk1118_dn9, locals.var_ey_acc__blk1118_dn10, locals.var_ey_acc__blk1118_dn11, locals.var_ey_acc__blk1118_dn14,)
    }
};
        locals.var_ey_acc__blk1118 = assign52170_e78968;
        locals.var_ey_acc__blk1118_dn0 = assign52170_e78968_d_n0;
        locals.var_ey_acc__blk1118_dn2 = assign52170_e78968_d_n2;
        locals.var_ey_acc__blk1118_dn4 = assign52170_e78968_d_n4;
        locals.var_ey_acc__blk1118_dn5 = assign52170_e78968_d_n5;
        locals.var_ey_acc__blk1118_dn6 = assign52170_e78968_d_n6;
        locals.var_ey_acc__blk1118_dn7 = assign52170_e78968_d_n7;
        locals.var_ey_acc__blk1118_dn8 = assign52170_e78968_d_n8;
        locals.var_ey_acc__blk1118_dn9 = assign52170_e78968_d_n9;
        locals.var_ey_acc__blk1118_dn10 = assign52170_e78968_d_n10;
        locals.var_ey_acc__blk1118_dn11 = assign52170_e78968_d_n11;
        locals.var_ey_acc__blk1118_dn14 = assign52170_e78968_d_n14;
        locals.var_ey_acc__blk1118_rv = 0.0;

        let (assign52180_e78979, assign52180_e78979_d_n0, assign52180_e78979_d_n2, assign52180_e78979_d_n4, assign52180_e78979_d_n5, assign52180_e78979_d_n6, assign52180_e78979_d_n7, assign52180_e78979_d_n8, assign52180_e78979_d_n9, assign52180_e78979_d_n10, assign52180_e78979_d_n11, assign52180_e78979_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn11, locals.var_vgp_ws_dn14,)
    }
};
        locals.var_vgp_ws = assign52180_e78979;
        locals.var_vgp_ws_dn0 = assign52180_e78979_d_n0;
        locals.var_vgp_ws_dn2 = assign52180_e78979_d_n2;
        locals.var_vgp_ws_dn4 = assign52180_e78979_d_n4;
        locals.var_vgp_ws_dn5 = assign52180_e78979_d_n5;
        locals.var_vgp_ws_dn6 = assign52180_e78979_d_n6;
        locals.var_vgp_ws_dn7 = assign52180_e78979_d_n7;
        locals.var_vgp_ws_dn8 = assign52180_e78979_d_n8;
        locals.var_vgp_ws_dn9 = assign52180_e78979_d_n9;
        locals.var_vgp_ws_dn10 = assign52180_e78979_d_n10;
        locals.var_vgp_ws_dn11 = assign52180_e78979_d_n11;
        locals.var_vgp_ws_dn14 = assign52180_e78979_d_n14;
        locals.var_vgp_ws_rv = 0.0;

        let (assign52190_e78990, assign52190_e78990_d_n0, assign52190_e78990_d_n2, assign52190_e78990_d_n4, assign52190_e78990_d_n5, assign52190_e78990_d_n6, assign52190_e78990_d_n7, assign52190_e78990_d_n8, assign52190_e78990_d_n9, assign52190_e78990_d_n10, assign52190_e78990_d_n11, assign52190_e78990_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_w_res_leak, locals.var_w_res_leak_dn0, locals.var_w_res_leak_dn2, locals.var_w_res_leak_dn4, locals.var_w_res_leak_dn5, locals.var_w_res_leak_dn6, locals.var_w_res_leak_dn7, locals.var_w_res_leak_dn8, locals.var_w_res_leak_dn9, locals.var_w_res_leak_dn10, locals.var_w_res_leak_dn11, locals.var_w_res_leak_dn14,)
    }
};
        locals.var_w_res_leak = assign52190_e78990;
        locals.var_w_res_leak_dn0 = assign52190_e78990_d_n0;
        locals.var_w_res_leak_dn2 = assign52190_e78990_d_n2;
        locals.var_w_res_leak_dn4 = assign52190_e78990_d_n4;
        locals.var_w_res_leak_dn5 = assign52190_e78990_d_n5;
        locals.var_w_res_leak_dn6 = assign52190_e78990_d_n6;
        locals.var_w_res_leak_dn7 = assign52190_e78990_d_n7;
        locals.var_w_res_leak_dn8 = assign52190_e78990_d_n8;
        locals.var_w_res_leak_dn9 = assign52190_e78990_d_n9;
        locals.var_w_res_leak_dn10 = assign52190_e78990_d_n10;
        locals.var_w_res_leak_dn11 = assign52190_e78990_d_n11;
        locals.var_w_res_leak_dn14 = assign52190_e78990_d_n14;
        locals.var_w_res_leak_rv = 0.0;

        let (assign52200_e79001, assign52200_e79001_d_n0, assign52200_e79001_d_n2, assign52200_e79001_d_n4, assign52200_e79001_d_n5, assign52200_e79001_d_n6, assign52200_e79001_d_n7, assign52200_e79001_d_n8, assign52200_e79001_d_n9, assign52200_e79001_d_n10, assign52200_e79001_d_n11, assign52200_e79001_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign52200_e79001;
        locals.var_w_res_dn0 = assign52200_e79001_d_n0;
        locals.var_w_res_dn2 = assign52200_e79001_d_n2;
        locals.var_w_res_dn4 = assign52200_e79001_d_n4;
        locals.var_w_res_dn5 = assign52200_e79001_d_n5;
        locals.var_w_res_dn6 = assign52200_e79001_d_n6;
        locals.var_w_res_dn7 = assign52200_e79001_d_n7;
        locals.var_w_res_dn8 = assign52200_e79001_d_n8;
        locals.var_w_res_dn9 = assign52200_e79001_d_n9;
        locals.var_w_res_dn10 = assign52200_e79001_d_n10;
        locals.var_w_res_dn11 = assign52200_e79001_d_n11;
        locals.var_w_res_dn14 = assign52200_e79001_d_n14;
        locals.var_w_res_rv = 0.0;

        let (assign52210_e79012, assign52210_e79012_d_n0, assign52210_e79012_d_n2, assign52210_e79012_d_n4, assign52210_e79012_d_n5, assign52210_e79012_d_n6, assign52210_e79012_d_n7, assign52210_e79012_d_n8, assign52210_e79012_d_n9, assign52210_e79012_d_n10, assign52210_e79012_d_n11, assign52210_e79012_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ws__blk1149, locals.var_ws__blk1149_dn0, locals.var_ws__blk1149_dn2, locals.var_ws__blk1149_dn4, locals.var_ws__blk1149_dn5, locals.var_ws__blk1149_dn6, locals.var_ws__blk1149_dn7, locals.var_ws__blk1149_dn8, locals.var_ws__blk1149_dn9, locals.var_ws__blk1149_dn10, locals.var_ws__blk1149_dn11, locals.var_ws__blk1149_dn14,)
    }
};
        locals.var_ws__blk1149 = assign52210_e79012;
        locals.var_ws__blk1149_dn0 = assign52210_e79012_d_n0;
        locals.var_ws__blk1149_dn2 = assign52210_e79012_d_n2;
        locals.var_ws__blk1149_dn4 = assign52210_e79012_d_n4;
        locals.var_ws__blk1149_dn5 = assign52210_e79012_d_n5;
        locals.var_ws__blk1149_dn6 = assign52210_e79012_d_n6;
        locals.var_ws__blk1149_dn7 = assign52210_e79012_d_n7;
        locals.var_ws__blk1149_dn8 = assign52210_e79012_d_n8;
        locals.var_ws__blk1149_dn9 = assign52210_e79012_d_n9;
        locals.var_ws__blk1149_dn10 = assign52210_e79012_d_n10;
        locals.var_ws__blk1149_dn11 = assign52210_e79012_d_n11;
        locals.var_ws__blk1149_dn14 = assign52210_e79012_d_n14;
        locals.var_ws__blk1149_rv = 0.0;

        let (assign52220_e79023, assign52220_e79023_d_n0, assign52220_e79023_d_n2, assign52220_e79023_d_n4, assign52220_e79023_d_n5, assign52220_e79023_d_n6, assign52220_e79023_d_n7, assign52220_e79023_d_n8, assign52220_e79023_d_n9, assign52220_e79023_d_n10, assign52220_e79023_d_n11, assign52220_e79023_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0__blk1324, locals.var_q_s0__blk1324_dn0, locals.var_q_s0__blk1324_dn2, locals.var_q_s0__blk1324_dn4, locals.var_q_s0__blk1324_dn5, locals.var_q_s0__blk1324_dn6, locals.var_q_s0__blk1324_dn7, locals.var_q_s0__blk1324_dn8, locals.var_q_s0__blk1324_dn9, locals.var_q_s0__blk1324_dn10, locals.var_q_s0__blk1324_dn11, locals.var_q_s0__blk1324_dn14,)
    }
};
        locals.var_q_s0__blk1324 = assign52220_e79023;
        locals.var_q_s0__blk1324_dn0 = assign52220_e79023_d_n0;
        locals.var_q_s0__blk1324_dn2 = assign52220_e79023_d_n2;
        locals.var_q_s0__blk1324_dn4 = assign52220_e79023_d_n4;
        locals.var_q_s0__blk1324_dn5 = assign52220_e79023_d_n5;
        locals.var_q_s0__blk1324_dn6 = assign52220_e79023_d_n6;
        locals.var_q_s0__blk1324_dn7 = assign52220_e79023_d_n7;
        locals.var_q_s0__blk1324_dn8 = assign52220_e79023_d_n8;
        locals.var_q_s0__blk1324_dn9 = assign52220_e79023_d_n9;
        locals.var_q_s0__blk1324_dn10 = assign52220_e79023_d_n10;
        locals.var_q_s0__blk1324_dn11 = assign52220_e79023_d_n11;
        locals.var_q_s0__blk1324_dn14 = assign52220_e79023_d_n14;
        locals.var_q_s0__blk1324_rv = 0.0;

        let (assign52230_e79040, assign52230_e79040_d_n0, assign52230_e79040_d_n2, assign52230_e79040_d_n4, assign52230_e79040_d_n5, assign52230_e79040_d_n6, assign52230_e79040_d_n7, assign52230_e79040_d_n8, assign52230_e79040_d_n9, assign52230_e79040_d_n10, assign52230_e79040_d_n11, assign52230_e79040_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign52230_e79034: f64 = (locals.var_vgsz__blk442 - locals.var_vfb);
        let assign52230_e79036: f64 = (assign52230_e79034 + locals.var_dvth);
        let assign52230_e79038: f64 = (assign52230_e79036 - locals.var_dppg);
        (assign52230_e79038, ((locals.var_vgsz__blk442_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_vgsz__blk442_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), ((locals.var_vgsz__blk442_dn4 + locals.var_dvth_dn4) - locals.var_dppg_dn4), ((locals.var_vgsz__blk442_dn5 + locals.var_dvth_dn5) - locals.var_dppg_dn5), ((locals.var_vgsz__blk442_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgsz__blk442_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgsz__blk442_dn8 + locals.var_dvth_dn8) - locals.var_dppg_dn8), ((locals.var_vgsz__blk442_dn9 + locals.var_dvth_dn9) - locals.var_dppg_dn9), ((locals.var_vgsz__blk442_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), ((locals.var_vgsz__blk442_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), ((locals.var_vgsz__blk442_dn14 + locals.var_dvth_dn14) - locals.var_dppg_dn14),)
    } else {
        (locals.var_vgpz, locals.var_vgpz_dn0, locals.var_vgpz_dn2, locals.var_vgpz_dn4, locals.var_vgpz_dn5, locals.var_vgpz_dn6, locals.var_vgpz_dn7, locals.var_vgpz_dn8, locals.var_vgpz_dn9, locals.var_vgpz_dn10, locals.var_vgpz_dn11, locals.var_vgpz_dn14,)
    }
};
        locals.var_vgpz = assign52230_e79040;
        locals.var_vgpz_dn0 = assign52230_e79040_d_n0;
        locals.var_vgpz_dn2 = assign52230_e79040_d_n2;
        locals.var_vgpz_dn4 = assign52230_e79040_d_n4;
        locals.var_vgpz_dn5 = assign52230_e79040_d_n5;
        locals.var_vgpz_dn6 = assign52230_e79040_d_n6;
        locals.var_vgpz_dn7 = assign52230_e79040_d_n7;
        locals.var_vgpz_dn8 = assign52230_e79040_d_n8;
        locals.var_vgpz_dn9 = assign52230_e79040_d_n9;
        locals.var_vgpz_dn10 = assign52230_e79040_d_n10;
        locals.var_vgpz_dn11 = assign52230_e79040_d_n11;
        locals.var_vgpz_dn14 = assign52230_e79040_d_n14;
        locals.var_vgpz_rv = 0.0;

        let assign52240_e79043: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1328 = assign52240_e79043;
        locals.var_guard1328_rv = 0.0;

        let (assign52250_e79058, assign52250_e79058_d_n0, assign52250_e79058_d_n2, assign52250_e79058_d_n4, assign52250_e79058_d_n5, assign52250_e79058_d_n6, assign52250_e79058_d_n7, assign52250_e79058_d_n8, assign52250_e79058_d_n9, assign52250_e79058_d_n10, assign52250_e79058_d_n11, assign52250_e79058_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1328 != 0.0)) {
        let assign52250_e79056: f64 = (locals.var_vgpz - p.p393);
        (assign52250_e79056, locals.var_vgpz_dn0, locals.var_vgpz_dn2, locals.var_vgpz_dn4, locals.var_vgpz_dn5, locals.var_vgpz_dn6, locals.var_vgpz_dn7, locals.var_vgpz_dn8, locals.var_vgpz_dn9, locals.var_vgpz_dn10, locals.var_vgpz_dn11, locals.var_vgpz_dn14,)
    } else {
        (locals.var_vgp_res__blk1147, locals.var_vgp_res__blk1147_dn0, locals.var_vgp_res__blk1147_dn2, locals.var_vgp_res__blk1147_dn4, locals.var_vgp_res__blk1147_dn5, locals.var_vgp_res__blk1147_dn6, locals.var_vgp_res__blk1147_dn7, locals.var_vgp_res__blk1147_dn8, locals.var_vgp_res__blk1147_dn9, locals.var_vgp_res__blk1147_dn10, locals.var_vgp_res__blk1147_dn11, locals.var_vgp_res__blk1147_dn14,)
    }
};
        locals.var_vgp_res__blk1147 = assign52250_e79058;
        locals.var_vgp_res__blk1147_dn0 = assign52250_e79058_d_n0;
        locals.var_vgp_res__blk1147_dn2 = assign52250_e79058_d_n2;
        locals.var_vgp_res__blk1147_dn4 = assign52250_e79058_d_n4;
        locals.var_vgp_res__blk1147_dn5 = assign52250_e79058_d_n5;
        locals.var_vgp_res__blk1147_dn6 = assign52250_e79058_d_n6;
        locals.var_vgp_res__blk1147_dn7 = assign52250_e79058_d_n7;
        locals.var_vgp_res__blk1147_dn8 = assign52250_e79058_d_n8;
        locals.var_vgp_res__blk1147_dn9 = assign52250_e79058_d_n9;
        locals.var_vgp_res__blk1147_dn10 = assign52250_e79058_d_n10;
        locals.var_vgp_res__blk1147_dn11 = assign52250_e79058_d_n11;
        locals.var_vgp_res__blk1147_dn14 = assign52250_e79058_d_n14;
        locals.var_vgp_res__blk1147_rv = 0.0;

        let assign52260_e79061: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1329 = assign52260_e79061;
        locals.var_guard1329_rv = 0.0;

        let (assign52270_e79081, assign52270_e79081_d_n0, assign52270_e79081_d_n2, assign52270_e79081_d_n4, assign52270_e79081_d_n5, assign52270_e79081_d_n6, assign52270_e79081_d_n7, assign52270_e79081_d_n8, assign52270_e79081_d_n9, assign52270_e79081_d_n10, assign52270_e79081_d_n11, assign52270_e79081_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1329 != 0.0)) {
        let assign52270_e79077: f64 = (locals.var_vgsz__blk442 - locals.var_vfb);
        let assign52270_e79079: f64 = (assign52270_e79077 - p.p393);
        (assign52270_e79079, locals.var_vgsz__blk442_dn0, locals.var_vgsz__blk442_dn2, locals.var_vgsz__blk442_dn4, locals.var_vgsz__blk442_dn5, locals.var_vgsz__blk442_dn6, locals.var_vgsz__blk442_dn7, locals.var_vgsz__blk442_dn8, locals.var_vgsz__blk442_dn9, locals.var_vgsz__blk442_dn10, locals.var_vgsz__blk442_dn11, locals.var_vgsz__blk442_dn14,)
    } else {
        (locals.var_vgp_res__blk1147, locals.var_vgp_res__blk1147_dn0, locals.var_vgp_res__blk1147_dn2, locals.var_vgp_res__blk1147_dn4, locals.var_vgp_res__blk1147_dn5, locals.var_vgp_res__blk1147_dn6, locals.var_vgp_res__blk1147_dn7, locals.var_vgp_res__blk1147_dn8, locals.var_vgp_res__blk1147_dn9, locals.var_vgp_res__blk1147_dn10, locals.var_vgp_res__blk1147_dn11, locals.var_vgp_res__blk1147_dn14,)
    }
};
        locals.var_vgp_res__blk1147 = assign52270_e79081;
        locals.var_vgp_res__blk1147_dn0 = assign52270_e79081_d_n0;
        locals.var_vgp_res__blk1147_dn2 = assign52270_e79081_d_n2;
        locals.var_vgp_res__blk1147_dn4 = assign52270_e79081_d_n4;
        locals.var_vgp_res__blk1147_dn5 = assign52270_e79081_d_n5;
        locals.var_vgp_res__blk1147_dn6 = assign52270_e79081_d_n6;
        locals.var_vgp_res__blk1147_dn7 = assign52270_e79081_d_n7;
        locals.var_vgp_res__blk1147_dn8 = assign52270_e79081_d_n8;
        locals.var_vgp_res__blk1147_dn9 = assign52270_e79081_d_n9;
        locals.var_vgp_res__blk1147_dn10 = assign52270_e79081_d_n10;
        locals.var_vgp_res__blk1147_dn11 = assign52270_e79081_d_n11;
        locals.var_vgp_res__blk1147_dn14 = assign52270_e79081_d_n14;
        locals.var_vgp_res__blk1147_rv = 0.0;

        let (assign52280_e79100, assign52280_e79100_d_n0, assign52280_e79100_d_n2, assign52280_e79100_d_n4, assign52280_e79100_d_n5, assign52280_e79100_d_n6, assign52280_e79100_d_n7, assign52280_e79100_d_n8, assign52280_e79100_d_n9, assign52280_e79100_d_n10, assign52280_e79100_d_n11, assign52280_e79100_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1329 == 0.0)) {
        let assign52280_e79098: f64 = (locals.var_vgp - p.p393);
        (assign52280_e79098, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn11, locals.var_vgp_dn14,)
    } else {
        (locals.var_vgp_res__blk1147, locals.var_vgp_res__blk1147_dn0, locals.var_vgp_res__blk1147_dn2, locals.var_vgp_res__blk1147_dn4, locals.var_vgp_res__blk1147_dn5, locals.var_vgp_res__blk1147_dn6, locals.var_vgp_res__blk1147_dn7, locals.var_vgp_res__blk1147_dn8, locals.var_vgp_res__blk1147_dn9, locals.var_vgp_res__blk1147_dn10, locals.var_vgp_res__blk1147_dn11, locals.var_vgp_res__blk1147_dn14,)
    }
};
        locals.var_vgp_res__blk1147 = assign52280_e79100;
        locals.var_vgp_res__blk1147_dn0 = assign52280_e79100_d_n0;
        locals.var_vgp_res__blk1147_dn2 = assign52280_e79100_d_n2;
        locals.var_vgp_res__blk1147_dn4 = assign52280_e79100_d_n4;
        locals.var_vgp_res__blk1147_dn5 = assign52280_e79100_d_n5;
        locals.var_vgp_res__blk1147_dn6 = assign52280_e79100_d_n6;
        locals.var_vgp_res__blk1147_dn7 = assign52280_e79100_d_n7;
        locals.var_vgp_res__blk1147_dn8 = assign52280_e79100_d_n8;
        locals.var_vgp_res__blk1147_dn9 = assign52280_e79100_d_n9;
        locals.var_vgp_res__blk1147_dn10 = assign52280_e79100_d_n10;
        locals.var_vgp_res__blk1147_dn11 = assign52280_e79100_d_n11;
        locals.var_vgp_res__blk1147_dn14 = assign52280_e79100_d_n14;
        locals.var_vgp_res__blk1147_rv = 0.0;

        let assign52290_e79102: f64 = (locals.var_tnp__blk1150).abs();
        let assign52290_e79104: f64 = if assign52290_e79102 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1330 = assign52290_e79104;
        locals.var_guard1330_rv = 0.0;

        let (assign52300_e79117, assign52300_e79117_d_n0, assign52300_e79117_d_n2, assign52300_e79117_d_n4, assign52300_e79117_d_n5, assign52300_e79117_d_n6, assign52300_e79117_d_n7, assign52300_e79117_d_n8, assign52300_e79117_d_n9, assign52300_e79117_d_n10, assign52300_e79117_d_n11, assign52300_e79117_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    }
};
        locals.var_ps0_res = assign52300_e79117;
        locals.var_ps0_res_dn0 = assign52300_e79117_d_n0;
        locals.var_ps0_res_dn2 = assign52300_e79117_d_n2;
        locals.var_ps0_res_dn4 = assign52300_e79117_d_n4;
        locals.var_ps0_res_dn5 = assign52300_e79117_d_n5;
        locals.var_ps0_res_dn6 = assign52300_e79117_d_n6;
        locals.var_ps0_res_dn7 = assign52300_e79117_d_n7;
        locals.var_ps0_res_dn8 = assign52300_e79117_d_n8;
        locals.var_ps0_res_dn9 = assign52300_e79117_d_n9;
        locals.var_ps0_res_dn10 = assign52300_e79117_d_n10;
        locals.var_ps0_res_dn11 = assign52300_e79117_d_n11;
        locals.var_ps0_res_dn14 = assign52300_e79117_d_n14;
        locals.var_ps0_res_rv = 0.0;

        let (assign52310_e79131, assign52310_e79131_d_n0, assign52310_e79131_d_n2, assign52310_e79131_d_n4, assign52310_e79131_d_n5, assign52310_e79131_d_n6, assign52310_e79131_d_n7, assign52310_e79131_d_n8, assign52310_e79131_d_n9, assign52310_e79131_d_n10, assign52310_e79131_d_n11, assign52310_e79131_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign52310_e79131;
        locals.var_ps0dep_dn0 = assign52310_e79131_d_n0;
        locals.var_ps0dep_dn2 = assign52310_e79131_d_n2;
        locals.var_ps0dep_dn4 = assign52310_e79131_d_n4;
        locals.var_ps0dep_dn5 = assign52310_e79131_d_n5;
        locals.var_ps0dep_dn6 = assign52310_e79131_d_n6;
        locals.var_ps0dep_dn7 = assign52310_e79131_d_n7;
        locals.var_ps0dep_dn8 = assign52310_e79131_d_n8;
        locals.var_ps0dep_dn9 = assign52310_e79131_d_n9;
        locals.var_ps0dep_dn10 = assign52310_e79131_d_n10;
        locals.var_ps0dep_dn11 = assign52310_e79131_d_n11;
        locals.var_ps0dep_dn14 = assign52310_e79131_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign52320_e79147, assign52320_e79147_d_n0, assign52320_e79147_d_n2, assign52320_e79147_d_n4, assign52320_e79147_d_n5, assign52320_e79147_d_n6, assign52320_e79147_d_n7, assign52320_e79147_d_n8, assign52320_e79147_d_n9, assign52320_e79147_d_n10, assign52320_e79147_d_n11, assign52320_e79147_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign52320_e79145: f64 = (p.p399 * locals.var_vbsc__blk1119);
        (assign52320_e79145, (p.p399 * locals.var_vbsc__blk1119_dn0), (p.p399 * locals.var_vbsc__blk1119_dn2), (p.p399 * locals.var_vbsc__blk1119_dn4), (p.p399 * locals.var_vbsc__blk1119_dn5), (p.p399 * locals.var_vbsc__blk1119_dn6), (p.p399 * locals.var_vbsc__blk1119_dn7), (p.p399 * locals.var_vbsc__blk1119_dn8), (p.p399 * locals.var_vbsc__blk1119_dn9), (p.p399 * locals.var_vbsc__blk1119_dn10), (p.p399 * locals.var_vbsc__blk1119_dn11), (p.p399 * locals.var_vbsc__blk1119_dn14),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    }
};
        locals.var_depvbs = assign52320_e79147;
        locals.var_depvbs_dn0 = assign52320_e79147_d_n0;
        locals.var_depvbs_dn2 = assign52320_e79147_d_n2;
        locals.var_depvbs_dn4 = assign52320_e79147_d_n4;
        locals.var_depvbs_dn5 = assign52320_e79147_d_n5;
        locals.var_depvbs_dn6 = assign52320_e79147_d_n6;
        locals.var_depvbs_dn7 = assign52320_e79147_d_n7;
        locals.var_depvbs_dn8 = assign52320_e79147_d_n8;
        locals.var_depvbs_dn9 = assign52320_e79147_d_n9;
        locals.var_depvbs_dn10 = assign52320_e79147_d_n10;
        locals.var_depvbs_dn11 = assign52320_e79147_d_n11;
        locals.var_depvbs_dn14 = assign52320_e79147_d_n14;
        locals.var_depvbs_rv = 0.0;

        let (assign52330_e79165,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign52330_e79161: f64 = (locals.var_vfb + p.p393);
        let assign52330_e79163: f64 = (assign52330_e79161 - 3.0);
        (assign52330_e79163,)
    } else {
        (locals.var_vgp_leak,)
    }
};
        locals.var_vgp_leak = assign52330_e79165;
        locals.var_vgp_leak_rv = 0.0;

        let assign52340_e79168: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1331 = assign52340_e79168;
        locals.var_guard1331_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_188(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52350_e79186, assign52350_e79186_d_n0, assign52350_e79186_d_n2, assign52350_e79186_d_n4, assign52350_e79186_d_n5, assign52350_e79186_d_n6, assign52350_e79186_d_n7, assign52350_e79186_d_n8, assign52350_e79186_d_n9, assign52350_e79186_d_n10, assign52350_e79186_d_n11, assign52350_e79186_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 != 0.0)) {
        let assign52350_e79184: f64 = (p.p399 * locals.var_vbsc__blk1119);
        (assign52350_e79184, (p.p399 * locals.var_vbsc__blk1119_dn0), (p.p399 * locals.var_vbsc__blk1119_dn2), (p.p399 * locals.var_vbsc__blk1119_dn4), (p.p399 * locals.var_vbsc__blk1119_dn5), (p.p399 * locals.var_vbsc__blk1119_dn6), (p.p399 * locals.var_vbsc__blk1119_dn7), (p.p399 * locals.var_vbsc__blk1119_dn8), (p.p399 * locals.var_vbsc__blk1119_dn9), (p.p399 * locals.var_vbsc__blk1119_dn10), (p.p399 * locals.var_vbsc__blk1119_dn11), (p.p399 * locals.var_vbsc__blk1119_dn14),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    }
};
        locals.var_depvbs = assign52350_e79186;
        locals.var_depvbs_dn0 = assign52350_e79186_d_n0;
        locals.var_depvbs_dn2 = assign52350_e79186_d_n2;
        locals.var_depvbs_dn4 = assign52350_e79186_d_n4;
        locals.var_depvbs_dn5 = assign52350_e79186_d_n5;
        locals.var_depvbs_dn6 = assign52350_e79186_d_n6;
        locals.var_depvbs_dn7 = assign52350_e79186_d_n7;
        locals.var_depvbs_dn8 = assign52350_e79186_d_n8;
        locals.var_depvbs_dn9 = assign52350_e79186_d_n9;
        locals.var_depvbs_dn10 = assign52350_e79186_d_n10;
        locals.var_depvbs_dn11 = assign52350_e79186_d_n11;
        locals.var_depvbs_dn14 = assign52350_e79186_d_n14;
        locals.var_depvbs_rv = 0.0;

        let (assign52360_e79204, assign52360_e79204_d_n0, assign52360_e79204_d_n2, assign52360_e79204_d_n4, assign52360_e79204_d_n5, assign52360_e79204_d_n6, assign52360_e79204_d_n7, assign52360_e79204_d_n8, assign52360_e79204_d_n9, assign52360_e79204_d_n10, assign52360_e79204_d_n11, assign52360_e79204_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 != 0.0)) {
        let assign52360_e79202: f64 = (locals.var_depvbs - 1.0);
        (assign52360_e79202, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign52360_e79204;
        locals.var_ps0dep_dn0 = assign52360_e79204_d_n0;
        locals.var_ps0dep_dn2 = assign52360_e79204_d_n2;
        locals.var_ps0dep_dn4 = assign52360_e79204_d_n4;
        locals.var_ps0dep_dn5 = assign52360_e79204_d_n5;
        locals.var_ps0dep_dn6 = assign52360_e79204_d_n6;
        locals.var_ps0dep_dn7 = assign52360_e79204_d_n7;
        locals.var_ps0dep_dn8 = assign52360_e79204_d_n8;
        locals.var_ps0dep_dn9 = assign52360_e79204_d_n9;
        locals.var_ps0dep_dn10 = assign52360_e79204_d_n10;
        locals.var_ps0dep_dn11 = assign52360_e79204_d_n11;
        locals.var_ps0dep_dn14 = assign52360_e79204_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign52370_e79220, assign52370_e79220_d_n0, assign52370_e79220_d_n2, assign52370_e79220_d_n4, assign52370_e79220_d_n5, assign52370_e79220_d_n6, assign52370_e79220_d_n7, assign52370_e79220_d_n8, assign52370_e79220_d_n9, assign52370_e79220_d_n10, assign52370_e79220_d_n11, assign52370_e79220_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn11, locals.var_vgp_ws_dn14,)
    }
};
        locals.var_vgp_ws = assign52370_e79220;
        locals.var_vgp_ws_dn0 = assign52370_e79220_d_n0;
        locals.var_vgp_ws_dn2 = assign52370_e79220_d_n2;
        locals.var_vgp_ws_dn4 = assign52370_e79220_d_n4;
        locals.var_vgp_ws_dn5 = assign52370_e79220_d_n5;
        locals.var_vgp_ws_dn6 = assign52370_e79220_d_n6;
        locals.var_vgp_ws_dn7 = assign52370_e79220_d_n7;
        locals.var_vgp_ws_dn8 = assign52370_e79220_d_n8;
        locals.var_vgp_ws_dn9 = assign52370_e79220_d_n9;
        locals.var_vgp_ws_dn10 = assign52370_e79220_d_n10;
        locals.var_vgp_ws_dn11 = assign52370_e79220_d_n11;
        locals.var_vgp_ws_dn14 = assign52370_e79220_d_n14;
        locals.var_vgp_ws_rv = 0.0;

        let (assign52380_e79236, assign52380_e79236_d_n0, assign52380_e79236_d_n2, assign52380_e79236_d_n4, assign52380_e79236_d_n5, assign52380_e79236_d_n6, assign52380_e79236_d_n7, assign52380_e79236_d_n8, assign52380_e79236_d_n9, assign52380_e79236_d_n10, assign52380_e79236_d_n11, assign52380_e79236_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn11, locals.var_vgp_res_raw_dn14,)
    }
};
        locals.var_vgp_res_raw = assign52380_e79236;
        locals.var_vgp_res_raw_dn0 = assign52380_e79236_d_n0;
        locals.var_vgp_res_raw_dn2 = assign52380_e79236_d_n2;
        locals.var_vgp_res_raw_dn4 = assign52380_e79236_d_n4;
        locals.var_vgp_res_raw_dn5 = assign52380_e79236_d_n5;
        locals.var_vgp_res_raw_dn6 = assign52380_e79236_d_n6;
        locals.var_vgp_res_raw_dn7 = assign52380_e79236_d_n7;
        locals.var_vgp_res_raw_dn8 = assign52380_e79236_d_n8;
        locals.var_vgp_res_raw_dn9 = assign52380_e79236_d_n9;
        locals.var_vgp_res_raw_dn10 = assign52380_e79236_d_n10;
        locals.var_vgp_res_raw_dn11 = assign52380_e79236_d_n11;
        locals.var_vgp_res_raw_dn14 = assign52380_e79236_d_n14;
        locals.var_vgp_res_raw_rv = 0.0;

        let (assign52390_e79257, assign52390_e79257_d_n0, assign52390_e79257_d_n2, assign52390_e79257_d_n4, assign52390_e79257_d_n5, assign52390_e79257_d_n6, assign52390_e79257_d_n7, assign52390_e79257_d_n8, assign52390_e79257_d_n9, assign52390_e79257_d_n10, assign52390_e79257_d_n11, assign52390_e79257_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 == 0.0)) {
        let assign52390_e79253: f64 = (p.p399 * locals.var_vbsc__blk1119);
        let assign52390_e79255: f64 = (assign52390_e79253 - 0.1);
        (assign52390_e79255, (p.p399 * locals.var_vbsc__blk1119_dn0), (p.p399 * locals.var_vbsc__blk1119_dn2), (p.p399 * locals.var_vbsc__blk1119_dn4), (p.p399 * locals.var_vbsc__blk1119_dn5), (p.p399 * locals.var_vbsc__blk1119_dn6), (p.p399 * locals.var_vbsc__blk1119_dn7), (p.p399 * locals.var_vbsc__blk1119_dn8), (p.p399 * locals.var_vbsc__blk1119_dn9), (p.p399 * locals.var_vbsc__blk1119_dn10), (p.p399 * locals.var_vbsc__blk1119_dn11), (p.p399 * locals.var_vbsc__blk1119_dn14),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    }
};
        locals.var_depvbs = assign52390_e79257;
        locals.var_depvbs_dn0 = assign52390_e79257_d_n0;
        locals.var_depvbs_dn2 = assign52390_e79257_d_n2;
        locals.var_depvbs_dn4 = assign52390_e79257_d_n4;
        locals.var_depvbs_dn5 = assign52390_e79257_d_n5;
        locals.var_depvbs_dn6 = assign52390_e79257_d_n6;
        locals.var_depvbs_dn7 = assign52390_e79257_d_n7;
        locals.var_depvbs_dn8 = assign52390_e79257_d_n8;
        locals.var_depvbs_dn9 = assign52390_e79257_d_n9;
        locals.var_depvbs_dn10 = assign52390_e79257_d_n10;
        locals.var_depvbs_dn11 = assign52390_e79257_d_n11;
        locals.var_depvbs_dn14 = assign52390_e79257_d_n14;
        locals.var_depvbs_rv = 0.0;

        let (assign52400_e79274, assign52400_e79274_d_n0, assign52400_e79274_d_n2, assign52400_e79274_d_n4, assign52400_e79274_d_n5, assign52400_e79274_d_n6, assign52400_e79274_d_n7, assign52400_e79274_d_n8, assign52400_e79274_d_n9, assign52400_e79274_d_n10, assign52400_e79274_d_n11, assign52400_e79274_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign52400_e79274;
        locals.var_ps0dep_dn0 = assign52400_e79274_d_n0;
        locals.var_ps0dep_dn2 = assign52400_e79274_d_n2;
        locals.var_ps0dep_dn4 = assign52400_e79274_d_n4;
        locals.var_ps0dep_dn5 = assign52400_e79274_d_n5;
        locals.var_ps0dep_dn6 = assign52400_e79274_d_n6;
        locals.var_ps0dep_dn7 = assign52400_e79274_d_n7;
        locals.var_ps0dep_dn8 = assign52400_e79274_d_n8;
        locals.var_ps0dep_dn9 = assign52400_e79274_d_n9;
        locals.var_ps0dep_dn10 = assign52400_e79274_d_n10;
        locals.var_ps0dep_dn11 = assign52400_e79274_d_n11;
        locals.var_ps0dep_dn14 = assign52400_e79274_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign52410_e79291, assign52410_e79291_d_n0, assign52410_e79291_d_n2, assign52410_e79291_d_n4, assign52410_e79291_d_n5, assign52410_e79291_d_n6, assign52410_e79291_d_n7, assign52410_e79291_d_n8, assign52410_e79291_d_n9, assign52410_e79291_d_n10, assign52410_e79291_d_n11, assign52410_e79291_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 == 0.0)) {
        (locals.var_vgp_res__blk1147, locals.var_vgp_res__blk1147_dn0, locals.var_vgp_res__blk1147_dn2, locals.var_vgp_res__blk1147_dn4, locals.var_vgp_res__blk1147_dn5, locals.var_vgp_res__blk1147_dn6, locals.var_vgp_res__blk1147_dn7, locals.var_vgp_res__blk1147_dn8, locals.var_vgp_res__blk1147_dn9, locals.var_vgp_res__blk1147_dn10, locals.var_vgp_res__blk1147_dn11, locals.var_vgp_res__blk1147_dn14,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn11, locals.var_vgp_ws_dn14,)
    }
};
        locals.var_vgp_ws = assign52410_e79291;
        locals.var_vgp_ws_dn0 = assign52410_e79291_d_n0;
        locals.var_vgp_ws_dn2 = assign52410_e79291_d_n2;
        locals.var_vgp_ws_dn4 = assign52410_e79291_d_n4;
        locals.var_vgp_ws_dn5 = assign52410_e79291_d_n5;
        locals.var_vgp_ws_dn6 = assign52410_e79291_d_n6;
        locals.var_vgp_ws_dn7 = assign52410_e79291_d_n7;
        locals.var_vgp_ws_dn8 = assign52410_e79291_d_n8;
        locals.var_vgp_ws_dn9 = assign52410_e79291_d_n9;
        locals.var_vgp_ws_dn10 = assign52410_e79291_d_n10;
        locals.var_vgp_ws_dn11 = assign52410_e79291_d_n11;
        locals.var_vgp_ws_dn14 = assign52410_e79291_d_n14;
        locals.var_vgp_ws_rv = 0.0;

        let (assign52420_e79308, assign52420_e79308_d_n0, assign52420_e79308_d_n2, assign52420_e79308_d_n4, assign52420_e79308_d_n5, assign52420_e79308_d_n6, assign52420_e79308_d_n7, assign52420_e79308_d_n8, assign52420_e79308_d_n9, assign52420_e79308_d_n10, assign52420_e79308_d_n11, assign52420_e79308_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 == 0.0)) {
        (locals.var_vgp_res__blk1147, locals.var_vgp_res__blk1147_dn0, locals.var_vgp_res__blk1147_dn2, locals.var_vgp_res__blk1147_dn4, locals.var_vgp_res__blk1147_dn5, locals.var_vgp_res__blk1147_dn6, locals.var_vgp_res__blk1147_dn7, locals.var_vgp_res__blk1147_dn8, locals.var_vgp_res__blk1147_dn9, locals.var_vgp_res__blk1147_dn10, locals.var_vgp_res__blk1147_dn11, locals.var_vgp_res__blk1147_dn14,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn11, locals.var_vgp_res_raw_dn14,)
    }
};
        locals.var_vgp_res_raw = assign52420_e79308;
        locals.var_vgp_res_raw_dn0 = assign52420_e79308_d_n0;
        locals.var_vgp_res_raw_dn2 = assign52420_e79308_d_n2;
        locals.var_vgp_res_raw_dn4 = assign52420_e79308_d_n4;
        locals.var_vgp_res_raw_dn5 = assign52420_e79308_d_n5;
        locals.var_vgp_res_raw_dn6 = assign52420_e79308_d_n6;
        locals.var_vgp_res_raw_dn7 = assign52420_e79308_d_n7;
        locals.var_vgp_res_raw_dn8 = assign52420_e79308_d_n8;
        locals.var_vgp_res_raw_dn9 = assign52420_e79308_d_n9;
        locals.var_vgp_res_raw_dn10 = assign52420_e79308_d_n10;
        locals.var_vgp_res_raw_dn11 = assign52420_e79308_d_n11;
        locals.var_vgp_res_raw_dn14 = assign52420_e79308_d_n14;
        locals.var_vgp_res_raw_rv = 0.0;

        let (assign52430_e79322,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign52430_e79322;
        locals.var_flg_conv_rv = 0.0;

        let (assign52440_e79336,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign52440_e79336;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_189(
        locals: &mut StampLocals,
    ) {
        let mut assign52450_loop_guard: usize = 0;
        while {
            let assign52450_cond_e79351: f64 = if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign52450_cond_e79351 != 0.0
        } {
            assign52450_loop_guard += 1;
            assert!(assign52450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52450_body0_e79367, assign52450_body0_e79367_d_n0, assign52450_body0_e79367_d_n2, assign52450_body0_e79367_d_n4, assign52450_body0_e79367_d_n5, assign52450_body0_e79367_d_n6, assign52450_body0_e79367_d_n7, assign52450_body0_e79367_d_n8, assign52450_body0_e79367_d_n9, assign52450_body0_e79367_d_n10, assign52450_body0_e79367_d_n11, assign52450_body0_e79367_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign52450_body0_e79365: f64 = (locals.var_beta * locals.var_ps0dep);
        (assign52450_body0_e79365, ((locals.var_beta_dn0 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn0)), ((locals.var_beta_dn2 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn2)), ((locals.var_beta_dn4 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn4)), ((locals.var_beta_dn5 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn5)), ((locals.var_beta_dn6 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn6)), ((locals.var_beta_dn7 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn7)), ((locals.var_beta_dn8 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn8)), ((locals.var_beta_dn9 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn9)), ((locals.var_beta_dn10 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn10)), ((locals.var_beta_dn11 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn11)), ((locals.var_beta_dn14 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign52450_body0_e79367;
            locals.var_t1_dn0 = assign52450_body0_e79367_d_n0;
            locals.var_t1_dn2 = assign52450_body0_e79367_d_n2;
            locals.var_t1_dn4 = assign52450_body0_e79367_d_n4;
            locals.var_t1_dn5 = assign52450_body0_e79367_d_n5;
            locals.var_t1_dn6 = assign52450_body0_e79367_d_n6;
            locals.var_t1_dn7 = assign52450_body0_e79367_d_n7;
            locals.var_t1_dn8 = assign52450_body0_e79367_d_n8;
            locals.var_t1_dn9 = assign52450_body0_e79367_d_n9;
            locals.var_t1_dn10 = assign52450_body0_e79367_d_n10;
            locals.var_t1_dn11 = assign52450_body0_e79367_d_n11;
            locals.var_t1_dn14 = assign52450_body0_e79367_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign52450_body1_e79382, assign52450_body1_e79382_d_n0, assign52450_body1_e79382_d_n2, assign52450_body1_e79382_d_n4, assign52450_body1_e79382_d_n5, assign52450_body1_e79382_d_n6, assign52450_body1_e79382_d_n7, assign52450_body1_e79382_d_n8, assign52450_body1_e79382_d_n9, assign52450_body1_e79382_d_n10, assign52450_body1_e79382_d_n11, assign52450_body1_e79382_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign52450_body1_e79380: f64 = (locals.var_t1).exp();
        (assign52450_body1_e79380, (assign52450_body1_e79380 * locals.var_t1_dn0), (assign52450_body1_e79380 * locals.var_t1_dn2), (assign52450_body1_e79380 * locals.var_t1_dn4), (assign52450_body1_e79380 * locals.var_t1_dn5), (assign52450_body1_e79380 * locals.var_t1_dn6), (assign52450_body1_e79380 * locals.var_t1_dn7), (assign52450_body1_e79380 * locals.var_t1_dn8), (assign52450_body1_e79380 * locals.var_t1_dn9), (assign52450_body1_e79380 * locals.var_t1_dn10), (assign52450_body1_e79380 * locals.var_t1_dn11), (assign52450_body1_e79380 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign52450_body1_e79382;
            locals.var_t2_dn0 = assign52450_body1_e79382_d_n0;
            locals.var_t2_dn2 = assign52450_body1_e79382_d_n2;
            locals.var_t2_dn4 = assign52450_body1_e79382_d_n4;
            locals.var_t2_dn5 = assign52450_body1_e79382_d_n5;
            locals.var_t2_dn6 = assign52450_body1_e79382_d_n6;
            locals.var_t2_dn7 = assign52450_body1_e79382_d_n7;
            locals.var_t2_dn8 = assign52450_body1_e79382_d_n8;
            locals.var_t2_dn9 = assign52450_body1_e79382_d_n9;
            locals.var_t2_dn10 = assign52450_body1_e79382_d_n10;
            locals.var_t2_dn11 = assign52450_body1_e79382_d_n11;
            locals.var_t2_dn14 = assign52450_body1_e79382_d_n14;
            locals.var_t2_rv = 0.0;
            let assign52450_body2_e79385: f64 = if locals.var_ps0dep >= 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1332 = assign52450_body2_e79385;
            locals.var_guard1332_rv = 0.0;
            let (assign52450_body3_e79411, assign52450_body3_e79411_d_n0, assign52450_body3_e79411_d_n2, assign52450_body3_e79411_d_n4, assign52450_body3_e79411_d_n5, assign52450_body3_e79411_d_n6, assign52450_body3_e79411_d_n7, assign52450_body3_e79411_d_n8, assign52450_body3_e79411_d_n9, assign52450_body3_e79411_d_n10, assign52450_body3_e79411_d_n11, assign52450_body3_e79411_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1332 != 0.0)) {
        let assign52450_body3_e79400: f64 = (-locals.var_cnst0);
        let assign52450_body3_e79403: f64 = (locals.var_t2 - 1.0);
        let assign52450_body3_e79405: f64 = (assign52450_body3_e79403 - locals.var_t1);
        let assign52450_body3_e79407: f64 = (assign52450_body3_e79405 + 1e-15);
        let assign52450_body3_e79408: f64 = (assign52450_body3_e79407).sqrt();
        let assign52450_body3_e79409: f64 = (assign52450_body3_e79400 * assign52450_body3_e79408);
        (assign52450_body3_e79409, (((-locals.var_cnst0_dn0) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign52450_body3_e79408)))), (((-locals.var_cnst0_dn2) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign52450_body3_e79408)))), (((-locals.var_cnst0_dn4) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign52450_body3_e79408)))), (((-locals.var_cnst0_dn5) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign52450_body3_e79408)))), (((-locals.var_cnst0_dn6) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign52450_body3_e79408)))), (((-locals.var_cnst0_dn7) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign52450_body3_e79408)))), (((-locals.var_cnst0_dn8) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign52450_body3_e79408)))), (((-locals.var_cnst0_dn9) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign52450_body3_e79408)))), (((-locals.var_cnst0_dn10) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign52450_body3_e79408)))), (((-locals.var_cnst0_dn11) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign52450_body3_e79408)))), (((-locals.var_cnst0_dn14) * assign52450_body3_e79408) + (assign52450_body3_e79400 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign52450_body3_e79408)))),)
    } else {
        (locals.var_q_s0__blk1324, locals.var_q_s0__blk1324_dn0, locals.var_q_s0__blk1324_dn2, locals.var_q_s0__blk1324_dn4, locals.var_q_s0__blk1324_dn5, locals.var_q_s0__blk1324_dn6, locals.var_q_s0__blk1324_dn7, locals.var_q_s0__blk1324_dn8, locals.var_q_s0__blk1324_dn9, locals.var_q_s0__blk1324_dn10, locals.var_q_s0__blk1324_dn11, locals.var_q_s0__blk1324_dn14,)
    }
};
            locals.var_q_s0__blk1324 = assign52450_body3_e79411;
            locals.var_q_s0__blk1324_dn0 = assign52450_body3_e79411_d_n0;
            locals.var_q_s0__blk1324_dn2 = assign52450_body3_e79411_d_n2;
            locals.var_q_s0__blk1324_dn4 = assign52450_body3_e79411_d_n4;
            locals.var_q_s0__blk1324_dn5 = assign52450_body3_e79411_d_n5;
            locals.var_q_s0__blk1324_dn6 = assign52450_body3_e79411_d_n6;
            locals.var_q_s0__blk1324_dn7 = assign52450_body3_e79411_d_n7;
            locals.var_q_s0__blk1324_dn8 = assign52450_body3_e79411_d_n8;
            locals.var_q_s0__blk1324_dn9 = assign52450_body3_e79411_d_n9;
            locals.var_q_s0__blk1324_dn10 = assign52450_body3_e79411_d_n10;
            locals.var_q_s0__blk1324_dn11 = assign52450_body3_e79411_d_n11;
            locals.var_q_s0__blk1324_dn14 = assign52450_body3_e79411_d_n14;
            locals.var_q_s0__blk1324_rv = 0.0;
            let (assign52450_body4_e79439, assign52450_body4_e79439_d_n0, assign52450_body4_e79439_d_n2, assign52450_body4_e79439_d_n4, assign52450_body4_e79439_d_n5, assign52450_body4_e79439_d_n6, assign52450_body4_e79439_d_n7, assign52450_body4_e79439_d_n8, assign52450_body4_e79439_d_n9, assign52450_body4_e79439_d_n10, assign52450_body4_e79439_d_n11, assign52450_body4_e79439_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1332 != 0.0)) {
        let assign52450_body4_e79427: f64 = (0.5 * locals.var_cnst0);
        let assign52450_body4_e79429: f64 = (assign52450_body4_e79427 * locals.var_cnst0);
        let assign52450_body4_e79431: f64 = (assign52450_body4_e79429 / locals.var_q_s0__blk1324);
        let assign52450_body4_e79434: f64 = (locals.var_beta * locals.var_t2);
        let assign52450_body4_e79436: f64 = (assign52450_body4_e79434 - locals.var_beta);
        let assign52450_body4_e79437: f64 = (assign52450_body4_e79431 * assign52450_body4_e79436);
        (assign52450_body4_e79437, ((((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn0)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0))), ((((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn2)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2))), ((((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn4)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4))), ((((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn5)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5))), ((((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn6)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6))), ((((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn7)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7))), ((((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn8)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8))), ((((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn9)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9))), ((((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn10)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10))), ((((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn11)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11))), ((((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign52450_body4_e79427 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1324) - (assign52450_body4_e79429 * locals.var_q_s0__blk1324_dn14)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign52450_body4_e79436) + (assign52450_body4_e79431 * (((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14))),)
    } else {
        (locals.var_q_s0_dps__blk1127, locals.var_q_s0_dps__blk1127_dn0, locals.var_q_s0_dps__blk1127_dn2, locals.var_q_s0_dps__blk1127_dn4, locals.var_q_s0_dps__blk1127_dn5, locals.var_q_s0_dps__blk1127_dn6, locals.var_q_s0_dps__blk1127_dn7, locals.var_q_s0_dps__blk1127_dn8, locals.var_q_s0_dps__blk1127_dn9, locals.var_q_s0_dps__blk1127_dn10, locals.var_q_s0_dps__blk1127_dn11, locals.var_q_s0_dps__blk1127_dn14,)
    }
};
            locals.var_q_s0_dps__blk1127 = assign52450_body4_e79439;
            locals.var_q_s0_dps__blk1127_dn0 = assign52450_body4_e79439_d_n0;
            locals.var_q_s0_dps__blk1127_dn2 = assign52450_body4_e79439_d_n2;
            locals.var_q_s0_dps__blk1127_dn4 = assign52450_body4_e79439_d_n4;
            locals.var_q_s0_dps__blk1127_dn5 = assign52450_body4_e79439_d_n5;
            locals.var_q_s0_dps__blk1127_dn6 = assign52450_body4_e79439_d_n6;
            locals.var_q_s0_dps__blk1127_dn7 = assign52450_body4_e79439_d_n7;
            locals.var_q_s0_dps__blk1127_dn8 = assign52450_body4_e79439_d_n8;
            locals.var_q_s0_dps__blk1127_dn9 = assign52450_body4_e79439_d_n9;
            locals.var_q_s0_dps__blk1127_dn10 = assign52450_body4_e79439_d_n10;
            locals.var_q_s0_dps__blk1127_dn11 = assign52450_body4_e79439_d_n11;
            locals.var_q_s0_dps__blk1127_dn14 = assign52450_body4_e79439_d_n14;
            locals.var_q_s0_dps__blk1127_rv = 0.0;
            let (assign52450_body5_e79462, assign52450_body5_e79462_d_n0, assign52450_body5_e79462_d_n2, assign52450_body5_e79462_d_n4, assign52450_body5_e79462_d_n5, assign52450_body5_e79462_d_n6, assign52450_body5_e79462_d_n7, assign52450_body5_e79462_d_n8, assign52450_body5_e79462_d_n9, assign52450_body5_e79462_d_n10, assign52450_body5_e79462_d_n11, assign52450_body5_e79462_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1332 == 0.0)) {
        let assign52450_body5_e79455: f64 = (-locals.var_beta);
        let assign52450_body5_e79458: f64 = (locals.var_ps0dep - locals.var_depvbs);
        let assign52450_body5_e79459: f64 = (assign52450_body5_e79455 * assign52450_body5_e79458);
        let assign52450_body5_e79460: f64 = (assign52450_body5_e79459).exp();
        (assign52450_body5_e79460, (assign52450_body5_e79460 * (((-locals.var_beta_dn0) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn0 - locals.var_depvbs_dn0)))), (assign52450_body5_e79460 * (((-locals.var_beta_dn2) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn2 - locals.var_depvbs_dn2)))), (assign52450_body5_e79460 * (((-locals.var_beta_dn4) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn4 - locals.var_depvbs_dn4)))), (assign52450_body5_e79460 * (((-locals.var_beta_dn5) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn5 - locals.var_depvbs_dn5)))), (assign52450_body5_e79460 * (((-locals.var_beta_dn6) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn6 - locals.var_depvbs_dn6)))), (assign52450_body5_e79460 * (((-locals.var_beta_dn7) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn7 - locals.var_depvbs_dn7)))), (assign52450_body5_e79460 * (((-locals.var_beta_dn8) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn8 - locals.var_depvbs_dn8)))), (assign52450_body5_e79460 * (((-locals.var_beta_dn9) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn9 - locals.var_depvbs_dn9)))), (assign52450_body5_e79460 * (((-locals.var_beta_dn10) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn10 - locals.var_depvbs_dn10)))), (assign52450_body5_e79460 * (((-locals.var_beta_dn11) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn11 - locals.var_depvbs_dn11)))), (assign52450_body5_e79460 * (((-locals.var_beta_dn14) * assign52450_body5_e79458) + (assign52450_body5_e79455 * (locals.var_ps0dep_dn14 - locals.var_depvbs_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign52450_body5_e79462;
            locals.var_t3_dn0 = assign52450_body5_e79462_d_n0;
            locals.var_t3_dn2 = assign52450_body5_e79462_d_n2;
            locals.var_t3_dn4 = assign52450_body5_e79462_d_n4;
            locals.var_t3_dn5 = assign52450_body5_e79462_d_n5;
            locals.var_t3_dn6 = assign52450_body5_e79462_d_n6;
            locals.var_t3_dn7 = assign52450_body5_e79462_d_n7;
            locals.var_t3_dn8 = assign52450_body5_e79462_d_n8;
            locals.var_t3_dn9 = assign52450_body5_e79462_d_n9;
            locals.var_t3_dn10 = assign52450_body5_e79462_d_n10;
            locals.var_t3_dn11 = assign52450_body5_e79462_d_n11;
            locals.var_t3_dn14 = assign52450_body5_e79462_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign52450_body6_e79482, assign52450_body6_e79482_d_n0, assign52450_body6_e79482_d_n2, assign52450_body6_e79482_d_n4, assign52450_body6_e79482_d_n5, assign52450_body6_e79482_d_n6, assign52450_body6_e79482_d_n7, assign52450_body6_e79482_d_n8, assign52450_body6_e79482_d_n9, assign52450_body6_e79482_d_n10, assign52450_body6_e79482_d_n11, assign52450_body6_e79482_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1332 == 0.0)) {
        let assign52450_body6_e79479: f64 = (locals.var_beta * locals.var_depvbs);
        let assign52450_body6_e79480: f64 = (assign52450_body6_e79479).exp();
        (assign52450_body6_e79480, (assign52450_body6_e79480 * ((locals.var_beta_dn0 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn0))), (assign52450_body6_e79480 * ((locals.var_beta_dn2 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn2))), (assign52450_body6_e79480 * ((locals.var_beta_dn4 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn4))), (assign52450_body6_e79480 * ((locals.var_beta_dn5 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn5))), (assign52450_body6_e79480 * ((locals.var_beta_dn6 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn6))), (assign52450_body6_e79480 * ((locals.var_beta_dn7 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn7))), (assign52450_body6_e79480 * ((locals.var_beta_dn8 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn8))), (assign52450_body6_e79480 * ((locals.var_beta_dn9 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn9))), (assign52450_body6_e79480 * ((locals.var_beta_dn10 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn10))), (assign52450_body6_e79480 * ((locals.var_beta_dn11 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn11))), (assign52450_body6_e79480 * ((locals.var_beta_dn14 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign52450_body6_e79482;
            locals.var_t4_dn0 = assign52450_body6_e79482_d_n0;
            locals.var_t4_dn2 = assign52450_body6_e79482_d_n2;
            locals.var_t4_dn4 = assign52450_body6_e79482_d_n4;
            locals.var_t4_dn5 = assign52450_body6_e79482_d_n5;
            locals.var_t4_dn6 = assign52450_body6_e79482_d_n6;
            locals.var_t4_dn7 = assign52450_body6_e79482_d_n7;
            locals.var_t4_dn8 = assign52450_body6_e79482_d_n8;
            locals.var_t4_dn9 = assign52450_body6_e79482_d_n9;
            locals.var_t4_dn10 = assign52450_body6_e79482_d_n10;
            locals.var_t4_dn11 = assign52450_body6_e79482_d_n11;
            locals.var_t4_dn14 = assign52450_body6_e79482_d_n14;
            locals.var_t4_rv = 0.0;
            let (assign52450_body7_e79514, assign52450_body7_e79514_d_n0, assign52450_body7_e79514_d_n2, assign52450_body7_e79514_d_n4, assign52450_body7_e79514_d_n5, assign52450_body7_e79514_d_n6, assign52450_body7_e79514_d_n7, assign52450_body7_e79514_d_n8, assign52450_body7_e79514_d_n9, assign52450_body7_e79514_d_n10, assign52450_body7_e79514_d_n11, assign52450_body7_e79514_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1332 == 0.0)) {
        let assign52450_body7_e79500: f64 = (locals.var_t2 - 1.0);
        let assign52450_body7_e79502: f64 = (assign52450_body7_e79500 - locals.var_t1);
        let assign52450_body7_e79506: f64 = (locals.var_t3 - locals.var_t4);
        let assign52450_body7_e79507: f64 = (locals.var_cnst1 * assign52450_body7_e79506);
        let assign52450_body7_e79508: f64 = (assign52450_body7_e79502 + assign52450_body7_e79507);
        let assign52450_body7_e79510: f64 = (assign52450_body7_e79508 + 1e-15);
        let assign52450_body7_e79511: f64 = (assign52450_body7_e79510).sqrt();
        let assign52450_body7_e79512: f64 = (locals.var_cnst0 * assign52450_body7_e79511);
        (assign52450_body7_e79512, ((locals.var_cnst0_dn0 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign52450_body7_e79511)))), ((locals.var_cnst0_dn2 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign52450_body7_e79511)))), ((locals.var_cnst0_dn4 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign52450_body7_e79511)))), ((locals.var_cnst0_dn5 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign52450_body7_e79511)))), ((locals.var_cnst0_dn6 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign52450_body7_e79511)))), ((locals.var_cnst0_dn7 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign52450_body7_e79511)))), ((locals.var_cnst0_dn8 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign52450_body7_e79511)))), ((locals.var_cnst0_dn9 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign52450_body7_e79511)))), ((locals.var_cnst0_dn10 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign52450_body7_e79511)))), ((locals.var_cnst0_dn11 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn11 - locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn11 - locals.var_t4_dn11)))) / (2.0 * assign52450_body7_e79511)))), ((locals.var_cnst0_dn14 * assign52450_body7_e79511) + (locals.var_cnst0 * (((locals.var_t2_dn14 - locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign52450_body7_e79506) + (locals.var_cnst1 * (locals.var_t3_dn14 - locals.var_t4_dn14)))) / (2.0 * assign52450_body7_e79511)))),)
    } else {
        (locals.var_q_s0__blk1324, locals.var_q_s0__blk1324_dn0, locals.var_q_s0__blk1324_dn2, locals.var_q_s0__blk1324_dn4, locals.var_q_s0__blk1324_dn5, locals.var_q_s0__blk1324_dn6, locals.var_q_s0__blk1324_dn7, locals.var_q_s0__blk1324_dn8, locals.var_q_s0__blk1324_dn9, locals.var_q_s0__blk1324_dn10, locals.var_q_s0__blk1324_dn11, locals.var_q_s0__blk1324_dn14,)
    }
};
            locals.var_q_s0__blk1324 = assign52450_body7_e79514;
            locals.var_q_s0__blk1324_dn0 = assign52450_body7_e79514_d_n0;
            locals.var_q_s0__blk1324_dn2 = assign52450_body7_e79514_d_n2;
            locals.var_q_s0__blk1324_dn4 = assign52450_body7_e79514_d_n4;
            locals.var_q_s0__blk1324_dn5 = assign52450_body7_e79514_d_n5;
            locals.var_q_s0__blk1324_dn6 = assign52450_body7_e79514_d_n6;
            locals.var_q_s0__blk1324_dn7 = assign52450_body7_e79514_d_n7;
            locals.var_q_s0__blk1324_dn8 = assign52450_body7_e79514_d_n8;
            locals.var_q_s0__blk1324_dn9 = assign52450_body7_e79514_d_n9;
            locals.var_q_s0__blk1324_dn10 = assign52450_body7_e79514_d_n10;
            locals.var_q_s0__blk1324_dn11 = assign52450_body7_e79514_d_n11;
            locals.var_q_s0__blk1324_dn14 = assign52450_body7_e79514_d_n14;
            locals.var_q_s0__blk1324_rv = 0.0;
            let (assign52450_body8_e79537, assign52450_body8_e79537_d_n0, assign52450_body8_e79537_d_n2, assign52450_body8_e79537_d_n4, assign52450_body8_e79537_d_n5, assign52450_body8_e79537_d_n6, assign52450_body8_e79537_d_n7, assign52450_body8_e79537_d_n8, assign52450_body8_e79537_d_n9, assign52450_body8_e79537_d_n10, assign52450_body8_e79537_d_n11, assign52450_body8_e79537_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1332 == 0.0)) {
        let assign52450_body8_e79531: f64 = (0.5 * locals.var_cnst0);
        let assign52450_body8_e79533: f64 = (assign52450_body8_e79531 * locals.var_cnst0);
        let assign52450_body8_e79535: f64 = (assign52450_body8_e79533 / locals.var_q_s0__blk1324);
        (assign52450_body8_e79535, ((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn0)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn2)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn4)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn5)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn6)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn7)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn8)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn9)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn10)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn11)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign52450_body8_e79531 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1324) - (assign52450_body8_e79533 * locals.var_q_s0__blk1324_dn14)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
            locals.var_t5 = assign52450_body8_e79537;
            locals.var_t5_dn0 = assign52450_body8_e79537_d_n0;
            locals.var_t5_dn2 = assign52450_body8_e79537_d_n2;
            locals.var_t5_dn4 = assign52450_body8_e79537_d_n4;
            locals.var_t5_dn5 = assign52450_body8_e79537_d_n5;
            locals.var_t5_dn6 = assign52450_body8_e79537_d_n6;
            locals.var_t5_dn7 = assign52450_body8_e79537_d_n7;
            locals.var_t5_dn8 = assign52450_body8_e79537_d_n8;
            locals.var_t5_dn9 = assign52450_body8_e79537_d_n9;
            locals.var_t5_dn10 = assign52450_body8_e79537_d_n10;
            locals.var_t5_dn11 = assign52450_body8_e79537_d_n11;
            locals.var_t5_dn14 = assign52450_body8_e79537_d_n14;
            locals.var_t5_rv = 0.0;
            let (assign52450_body9_e79567, assign52450_body9_e79567_d_n0, assign52450_body9_e79567_d_n2, assign52450_body9_e79567_d_n4, assign52450_body9_e79567_d_n5, assign52450_body9_e79567_d_n6, assign52450_body9_e79567_d_n7, assign52450_body9_e79567_d_n8, assign52450_body9_e79567_d_n9, assign52450_body9_e79567_d_n10, assign52450_body9_e79567_d_n11, assign52450_body9_e79567_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1332 == 0.0)) {
        let assign52450_body9_e79555: f64 = (locals.var_beta * locals.var_t2);
        let assign52450_body9_e79557: f64 = (assign52450_body9_e79555 - locals.var_beta);
        let assign52450_body9_e79560: f64 = (-locals.var_beta);
        let assign52450_body9_e79562: f64 = (assign52450_body9_e79560 * locals.var_t3);
        let assign52450_body9_e79563: f64 = (locals.var_cnst1 * assign52450_body9_e79562);
        let assign52450_body9_e79564: f64 = (assign52450_body9_e79557 + assign52450_body9_e79563);
        let assign52450_body9_e79565: f64 = (locals.var_t5 * assign52450_body9_e79564);
        (assign52450_body9_e79565, ((locals.var_t5_dn0 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0) + ((locals.var_cnst1_dn0 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn0) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn0))))))), ((locals.var_t5_dn2 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2) + ((locals.var_cnst1_dn2 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn2) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn2))))))), ((locals.var_t5_dn4 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4) + ((locals.var_cnst1_dn4 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn4) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn4))))))), ((locals.var_t5_dn5 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5) + ((locals.var_cnst1_dn5 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn5) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn5))))))), ((locals.var_t5_dn6 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6) + ((locals.var_cnst1_dn6 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn6) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn6))))))), ((locals.var_t5_dn7 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7) + ((locals.var_cnst1_dn7 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn7) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn7))))))), ((locals.var_t5_dn8 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8) + ((locals.var_cnst1_dn8 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn8) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn8))))))), ((locals.var_t5_dn9 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9) + ((locals.var_cnst1_dn9 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn9) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn9))))))), ((locals.var_t5_dn10 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10) + ((locals.var_cnst1_dn10 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn10) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn10))))))), ((locals.var_t5_dn11 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11) + ((locals.var_cnst1_dn11 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn11) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn11))))))), ((locals.var_t5_dn14 * assign52450_body9_e79564) + (locals.var_t5 * ((((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14) + ((locals.var_cnst1_dn14 * assign52450_body9_e79562) + (locals.var_cnst1 * (((-locals.var_beta_dn14) * locals.var_t3) + (assign52450_body9_e79560 * locals.var_t3_dn14))))))),)
    } else {
        (locals.var_q_s0_dps__blk1127, locals.var_q_s0_dps__blk1127_dn0, locals.var_q_s0_dps__blk1127_dn2, locals.var_q_s0_dps__blk1127_dn4, locals.var_q_s0_dps__blk1127_dn5, locals.var_q_s0_dps__blk1127_dn6, locals.var_q_s0_dps__blk1127_dn7, locals.var_q_s0_dps__blk1127_dn8, locals.var_q_s0_dps__blk1127_dn9, locals.var_q_s0_dps__blk1127_dn10, locals.var_q_s0_dps__blk1127_dn11, locals.var_q_s0_dps__blk1127_dn14,)
    }
};
            locals.var_q_s0_dps__blk1127 = assign52450_body9_e79567;
            locals.var_q_s0_dps__blk1127_dn0 = assign52450_body9_e79567_d_n0;
            locals.var_q_s0_dps__blk1127_dn2 = assign52450_body9_e79567_d_n2;
            locals.var_q_s0_dps__blk1127_dn4 = assign52450_body9_e79567_d_n4;
            locals.var_q_s0_dps__blk1127_dn5 = assign52450_body9_e79567_d_n5;
            locals.var_q_s0_dps__blk1127_dn6 = assign52450_body9_e79567_d_n6;
            locals.var_q_s0_dps__blk1127_dn7 = assign52450_body9_e79567_d_n7;
            locals.var_q_s0_dps__blk1127_dn8 = assign52450_body9_e79567_d_n8;
            locals.var_q_s0_dps__blk1127_dn9 = assign52450_body9_e79567_d_n9;
            locals.var_q_s0_dps__blk1127_dn10 = assign52450_body9_e79567_d_n10;
            locals.var_q_s0_dps__blk1127_dn11 = assign52450_body9_e79567_d_n11;
            locals.var_q_s0_dps__blk1127_dn14 = assign52450_body9_e79567_d_n14;
            locals.var_q_s0_dps__blk1127_rv = 0.0;
            let (assign52450_body10_e79585,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign52450_body10_e79583: f64 = (150.0 + 1.0);
        (assign52450_body10_e79583,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign52450_body10_e79585;
            locals.var_lp_s0_rv = 0.0;
            let (assign52450_body11_e79608, assign52450_body11_e79608_d_n0, assign52450_body11_e79608_d_n2, assign52450_body11_e79608_d_n4, assign52450_body11_e79608_d_n5, assign52450_body11_e79608_d_n6, assign52450_body11_e79608_d_n7, assign52450_body11_e79608_d_n8, assign52450_body11_e79608_d_n9, assign52450_body11_e79608_d_n10, assign52450_body11_e79608_d_n11, assign52450_body11_e79608_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52450_body11_e79603: f64 = (locals.var_vgp_ws - locals.var_ps0dep);
        let assign52450_body11_e79604: f64 = (locals.var_cox * assign52450_body11_e79603);
        let assign52450_body11_e79606: f64 = (assign52450_body11_e79604 + locals.var_q_s0__blk1324);
        (assign52450_body11_e79606, (((locals.var_cox_dn0 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn0 - locals.var_ps0dep_dn0))) + locals.var_q_s0__blk1324_dn0), (((locals.var_cox_dn2 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn2 - locals.var_ps0dep_dn2))) + locals.var_q_s0__blk1324_dn2), (((locals.var_cox_dn4 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn4 - locals.var_ps0dep_dn4))) + locals.var_q_s0__blk1324_dn4), (((locals.var_cox_dn5 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn5 - locals.var_ps0dep_dn5))) + locals.var_q_s0__blk1324_dn5), (((locals.var_cox_dn6 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn6 - locals.var_ps0dep_dn6))) + locals.var_q_s0__blk1324_dn6), (((locals.var_cox_dn7 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn7 - locals.var_ps0dep_dn7))) + locals.var_q_s0__blk1324_dn7), (((locals.var_cox_dn8 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn8 - locals.var_ps0dep_dn8))) + locals.var_q_s0__blk1324_dn8), (((locals.var_cox_dn9 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn9 - locals.var_ps0dep_dn9))) + locals.var_q_s0__blk1324_dn9), (((locals.var_cox_dn10 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn10 - locals.var_ps0dep_dn10))) + locals.var_q_s0__blk1324_dn10), (((locals.var_cox_dn11 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn11 - locals.var_ps0dep_dn11))) + locals.var_q_s0__blk1324_dn11), (((locals.var_cox_dn14 * assign52450_body11_e79603) + (locals.var_cox * (locals.var_vgp_ws_dn14 - locals.var_ps0dep_dn14))) + locals.var_q_s0__blk1324_dn14),)
    } else {
        (locals.var_pf1__blk1102, locals.var_pf1__blk1102_dn0, locals.var_pf1__blk1102_dn2, locals.var_pf1__blk1102_dn4, locals.var_pf1__blk1102_dn5, locals.var_pf1__blk1102_dn6, locals.var_pf1__blk1102_dn7, locals.var_pf1__blk1102_dn8, locals.var_pf1__blk1102_dn9, locals.var_pf1__blk1102_dn10, locals.var_pf1__blk1102_dn11, locals.var_pf1__blk1102_dn14,)
    }
};
            locals.var_pf1__blk1102 = assign52450_body11_e79608;
            locals.var_pf1__blk1102_dn0 = assign52450_body11_e79608_d_n0;
            locals.var_pf1__blk1102_dn2 = assign52450_body11_e79608_d_n2;
            locals.var_pf1__blk1102_dn4 = assign52450_body11_e79608_d_n4;
            locals.var_pf1__blk1102_dn5 = assign52450_body11_e79608_d_n5;
            locals.var_pf1__blk1102_dn6 = assign52450_body11_e79608_d_n6;
            locals.var_pf1__blk1102_dn7 = assign52450_body11_e79608_d_n7;
            locals.var_pf1__blk1102_dn8 = assign52450_body11_e79608_d_n8;
            locals.var_pf1__blk1102_dn9 = assign52450_body11_e79608_d_n9;
            locals.var_pf1__blk1102_dn10 = assign52450_body11_e79608_d_n10;
            locals.var_pf1__blk1102_dn11 = assign52450_body11_e79608_d_n11;
            locals.var_pf1__blk1102_dn14 = assign52450_body11_e79608_d_n14;
            locals.var_pf1__blk1102_rv = 0.0;
            let (assign52450_body12_e79628, assign52450_body12_e79628_d_n0, assign52450_body12_e79628_d_n2, assign52450_body12_e79628_d_n4, assign52450_body12_e79628_d_n5, assign52450_body12_e79628_d_n6, assign52450_body12_e79628_d_n7, assign52450_body12_e79628_d_n8, assign52450_body12_e79628_d_n9, assign52450_body12_e79628_d_n10, assign52450_body12_e79628_d_n11, assign52450_body12_e79628_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52450_body12_e79624: f64 = (-locals.var_cox);
        let assign52450_body12_e79626: f64 = (assign52450_body12_e79624 + locals.var_q_s0_dps__blk1127);
        (assign52450_body12_e79626, ((-locals.var_cox_dn0) + locals.var_q_s0_dps__blk1127_dn0), ((-locals.var_cox_dn2) + locals.var_q_s0_dps__blk1127_dn2), ((-locals.var_cox_dn4) + locals.var_q_s0_dps__blk1127_dn4), ((-locals.var_cox_dn5) + locals.var_q_s0_dps__blk1127_dn5), ((-locals.var_cox_dn6) + locals.var_q_s0_dps__blk1127_dn6), ((-locals.var_cox_dn7) + locals.var_q_s0_dps__blk1127_dn7), ((-locals.var_cox_dn8) + locals.var_q_s0_dps__blk1127_dn8), ((-locals.var_cox_dn9) + locals.var_q_s0_dps__blk1127_dn9), ((-locals.var_cox_dn10) + locals.var_q_s0_dps__blk1127_dn10), ((-locals.var_cox_dn11) + locals.var_q_s0_dps__blk1127_dn11), ((-locals.var_cox_dn14) + locals.var_q_s0_dps__blk1127_dn14),)
    } else {
        (locals.var_pf11__blk1103, locals.var_pf11__blk1103_dn0, locals.var_pf11__blk1103_dn2, locals.var_pf11__blk1103_dn4, locals.var_pf11__blk1103_dn5, locals.var_pf11__blk1103_dn6, locals.var_pf11__blk1103_dn7, locals.var_pf11__blk1103_dn8, locals.var_pf11__blk1103_dn9, locals.var_pf11__blk1103_dn10, locals.var_pf11__blk1103_dn11, locals.var_pf11__blk1103_dn14,)
    }
};
            locals.var_pf11__blk1103 = assign52450_body12_e79628;
            locals.var_pf11__blk1103_dn0 = assign52450_body12_e79628_d_n0;
            locals.var_pf11__blk1103_dn2 = assign52450_body12_e79628_d_n2;
            locals.var_pf11__blk1103_dn4 = assign52450_body12_e79628_d_n4;
            locals.var_pf11__blk1103_dn5 = assign52450_body12_e79628_d_n5;
            locals.var_pf11__blk1103_dn6 = assign52450_body12_e79628_d_n6;
            locals.var_pf11__blk1103_dn7 = assign52450_body12_e79628_d_n7;
            locals.var_pf11__blk1103_dn8 = assign52450_body12_e79628_d_n8;
            locals.var_pf11__blk1103_dn9 = assign52450_body12_e79628_d_n9;
            locals.var_pf11__blk1103_dn10 = assign52450_body12_e79628_d_n10;
            locals.var_pf11__blk1103_dn11 = assign52450_body12_e79628_d_n11;
            locals.var_pf11__blk1103_dn14 = assign52450_body12_e79628_d_n14;
            locals.var_pf11__blk1103_rv = 0.0;
            let (assign52450_body13_e79648, assign52450_body13_e79648_d_n0, assign52450_body13_e79648_d_n2, assign52450_body13_e79648_d_n4, assign52450_body13_e79648_d_n5, assign52450_body13_e79648_d_n6, assign52450_body13_e79648_d_n7, assign52450_body13_e79648_d_n8, assign52450_body13_e79648_d_n9, assign52450_body13_e79648_d_n10, assign52450_body13_e79648_d_n11, assign52450_body13_e79648_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52450_body13_e79644: f64 = (-locals.var_pf1__blk1102);
        let assign52450_body13_e79646: f64 = (assign52450_body13_e79644 / locals.var_pf11__blk1103);
        (assign52450_body13_e79646, ((((-locals.var_pf1__blk1102_dn0) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn0)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn2) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn2)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn4) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn4)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn5) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn5)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn6) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn6)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn7) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn7)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn8) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn8)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn9) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn9)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn10) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn10)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn11) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn11)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn14) * locals.var_pf11__blk1103) - (assign52450_body13_e79644 * locals.var_pf11__blk1103_dn14)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)),)
    } else {
        (locals.var_dps__blk1114, locals.var_dps__blk1114_dn0, locals.var_dps__blk1114_dn2, locals.var_dps__blk1114_dn4, locals.var_dps__blk1114_dn5, locals.var_dps__blk1114_dn6, locals.var_dps__blk1114_dn7, locals.var_dps__blk1114_dn8, locals.var_dps__blk1114_dn9, locals.var_dps__blk1114_dn10, locals.var_dps__blk1114_dn11, locals.var_dps__blk1114_dn14,)
    }
};
            locals.var_dps__blk1114 = assign52450_body13_e79648;
            locals.var_dps__blk1114_dn0 = assign52450_body13_e79648_d_n0;
            locals.var_dps__blk1114_dn2 = assign52450_body13_e79648_d_n2;
            locals.var_dps__blk1114_dn4 = assign52450_body13_e79648_d_n4;
            locals.var_dps__blk1114_dn5 = assign52450_body13_e79648_d_n5;
            locals.var_dps__blk1114_dn6 = assign52450_body13_e79648_d_n6;
            locals.var_dps__blk1114_dn7 = assign52450_body13_e79648_d_n7;
            locals.var_dps__blk1114_dn8 = assign52450_body13_e79648_d_n8;
            locals.var_dps__blk1114_dn9 = assign52450_body13_e79648_d_n9;
            locals.var_dps__blk1114_dn10 = assign52450_body13_e79648_d_n10;
            locals.var_dps__blk1114_dn11 = assign52450_body13_e79648_d_n11;
            locals.var_dps__blk1114_dn14 = assign52450_body13_e79648_d_n14;
            locals.var_dps__blk1114_rv = 0.0;
            let assign52450_body14_e79650: f64 = (locals.var_dps__blk1114).abs();
            let assign52450_body14_e79653: f64 = (1e-10 * 100.0);
            let assign52450_body14_e79654: f64 = if assign52450_body14_e79650 < assign52450_body14_e79653 { 1.0 } else { 0.0 };
            locals.var_guard1333 = assign52450_body14_e79654;
            locals.var_guard1333_rv = 0.0;
            let (assign52450_body15_e79673,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1333 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign52450_body15_e79673;
            locals.var_flg_conv_rv = 0.0;
            let assign52450_body16_e79676: f64 = if locals.var_dps__blk1114 > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1334 = assign52450_body16_e79676;
            locals.var_guard1334_rv = 0.0;
            let (assign52450_body17_e79698, assign52450_body17_e79698_d_n0, assign52450_body17_e79698_d_n2, assign52450_body17_e79698_d_n4, assign52450_body17_e79698_d_n5, assign52450_body17_e79698_d_n6, assign52450_body17_e79698_d_n7, assign52450_body17_e79698_d_n8, assign52450_body17_e79698_d_n9, assign52450_body17_e79698_d_n10, assign52450_body17_e79698_d_n11, assign52450_body17_e79698_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1333 == 0.0)) && (locals.var_guard1334 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1114, locals.var_dps__blk1114_dn0, locals.var_dps__blk1114_dn2, locals.var_dps__blk1114_dn4, locals.var_dps__blk1114_dn5, locals.var_dps__blk1114_dn6, locals.var_dps__blk1114_dn7, locals.var_dps__blk1114_dn8, locals.var_dps__blk1114_dn9, locals.var_dps__blk1114_dn10, locals.var_dps__blk1114_dn11, locals.var_dps__blk1114_dn14,)
    }
};
            locals.var_dps__blk1114 = assign52450_body17_e79698;
            locals.var_dps__blk1114_dn0 = assign52450_body17_e79698_d_n0;
            locals.var_dps__blk1114_dn2 = assign52450_body17_e79698_d_n2;
            locals.var_dps__blk1114_dn4 = assign52450_body17_e79698_d_n4;
            locals.var_dps__blk1114_dn5 = assign52450_body17_e79698_d_n5;
            locals.var_dps__blk1114_dn6 = assign52450_body17_e79698_d_n6;
            locals.var_dps__blk1114_dn7 = assign52450_body17_e79698_d_n7;
            locals.var_dps__blk1114_dn8 = assign52450_body17_e79698_d_n8;
            locals.var_dps__blk1114_dn9 = assign52450_body17_e79698_d_n9;
            locals.var_dps__blk1114_dn10 = assign52450_body17_e79698_d_n10;
            locals.var_dps__blk1114_dn11 = assign52450_body17_e79698_d_n11;
            locals.var_dps__blk1114_dn14 = assign52450_body17_e79698_d_n14;
            locals.var_dps__blk1114_rv = 0.0;
            let assign52450_body18_e79701: f64 = (-0.1);
            let assign52450_body18_e79702: f64 = if locals.var_dps__blk1114 < assign52450_body18_e79701 { 1.0 } else { 0.0 };
            locals.var_guard1335 = assign52450_body18_e79702;
            locals.var_guard1335_rv = 0.0;
            let (assign52450_body19_e79728, assign52450_body19_e79728_d_n0, assign52450_body19_e79728_d_n2, assign52450_body19_e79728_d_n4, assign52450_body19_e79728_d_n5, assign52450_body19_e79728_d_n6, assign52450_body19_e79728_d_n7, assign52450_body19_e79728_d_n8, assign52450_body19_e79728_d_n9, assign52450_body19_e79728_d_n10, assign52450_body19_e79728_d_n11, assign52450_body19_e79728_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1333 == 0.0)) && (locals.var_guard1334 == 0.0)) && (locals.var_guard1335 != 0.0)) {
        let assign52450_body19_e79726: f64 = (-0.1);
        (assign52450_body19_e79726, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1114, locals.var_dps__blk1114_dn0, locals.var_dps__blk1114_dn2, locals.var_dps__blk1114_dn4, locals.var_dps__blk1114_dn5, locals.var_dps__blk1114_dn6, locals.var_dps__blk1114_dn7, locals.var_dps__blk1114_dn8, locals.var_dps__blk1114_dn9, locals.var_dps__blk1114_dn10, locals.var_dps__blk1114_dn11, locals.var_dps__blk1114_dn14,)
    }
};
            locals.var_dps__blk1114 = assign52450_body19_e79728;
            locals.var_dps__blk1114_dn0 = assign52450_body19_e79728_d_n0;
            locals.var_dps__blk1114_dn2 = assign52450_body19_e79728_d_n2;
            locals.var_dps__blk1114_dn4 = assign52450_body19_e79728_d_n4;
            locals.var_dps__blk1114_dn5 = assign52450_body19_e79728_d_n5;
            locals.var_dps__blk1114_dn6 = assign52450_body19_e79728_d_n6;
            locals.var_dps__blk1114_dn7 = assign52450_body19_e79728_d_n7;
            locals.var_dps__blk1114_dn8 = assign52450_body19_e79728_d_n8;
            locals.var_dps__blk1114_dn9 = assign52450_body19_e79728_d_n9;
            locals.var_dps__blk1114_dn10 = assign52450_body19_e79728_d_n10;
            locals.var_dps__blk1114_dn11 = assign52450_body19_e79728_d_n11;
            locals.var_dps__blk1114_dn14 = assign52450_body19_e79728_d_n14;
            locals.var_dps__blk1114_rv = 0.0;
            let (assign52450_body20_e79747, assign52450_body20_e79747_d_n0, assign52450_body20_e79747_d_n2, assign52450_body20_e79747_d_n4, assign52450_body20_e79747_d_n5, assign52450_body20_e79747_d_n6, assign52450_body20_e79747_d_n7, assign52450_body20_e79747_d_n8, assign52450_body20_e79747_d_n9, assign52450_body20_e79747_d_n10, assign52450_body20_e79747_d_n11, assign52450_body20_e79747_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52450_body20_e79745: f64 = (locals.var_ps0dep + locals.var_dps__blk1114);
        (assign52450_body20_e79745, (locals.var_ps0dep_dn0 + locals.var_dps__blk1114_dn0), (locals.var_ps0dep_dn2 + locals.var_dps__blk1114_dn2), (locals.var_ps0dep_dn4 + locals.var_dps__blk1114_dn4), (locals.var_ps0dep_dn5 + locals.var_dps__blk1114_dn5), (locals.var_ps0dep_dn6 + locals.var_dps__blk1114_dn6), (locals.var_ps0dep_dn7 + locals.var_dps__blk1114_dn7), (locals.var_ps0dep_dn8 + locals.var_dps__blk1114_dn8), (locals.var_ps0dep_dn9 + locals.var_dps__blk1114_dn9), (locals.var_ps0dep_dn10 + locals.var_dps__blk1114_dn10), (locals.var_ps0dep_dn11 + locals.var_dps__blk1114_dn11), (locals.var_ps0dep_dn14 + locals.var_dps__blk1114_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
            locals.var_ps0dep = assign52450_body20_e79747;
            locals.var_ps0dep_dn0 = assign52450_body20_e79747_d_n0;
            locals.var_ps0dep_dn2 = assign52450_body20_e79747_d_n2;
            locals.var_ps0dep_dn4 = assign52450_body20_e79747_d_n4;
            locals.var_ps0dep_dn5 = assign52450_body20_e79747_d_n5;
            locals.var_ps0dep_dn6 = assign52450_body20_e79747_d_n6;
            locals.var_ps0dep_dn7 = assign52450_body20_e79747_d_n7;
            locals.var_ps0dep_dn8 = assign52450_body20_e79747_d_n8;
            locals.var_ps0dep_dn9 = assign52450_body20_e79747_d_n9;
            locals.var_ps0dep_dn10 = assign52450_body20_e79747_d_n10;
            locals.var_ps0dep_dn11 = assign52450_body20_e79747_d_n11;
            locals.var_ps0dep_dn14 = assign52450_body20_e79747_d_n14;
            locals.var_ps0dep_rv = 0.0;
            let (assign52450_body21_e79763,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign52450_body21_e79761: f64 = (locals.var_lp_s0 + 1.0);
        (assign52450_body21_e79761,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign52450_body21_e79763;
            locals.var_lp_s0_rv = 0.0;
        }

        let assign52470_e79769: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1337 = assign52470_e79769;
        locals.var_guard1337_rv = 0.0;

        let (assign52480_e79785, assign52480_e79785_d_n0, assign52480_e79785_d_n2, assign52480_e79785_d_n4, assign52480_e79785_d_n5, assign52480_e79785_d_n6, assign52480_e79785_d_n7, assign52480_e79785_d_n8, assign52480_e79785_d_n9, assign52480_e79785_d_n10, assign52480_e79785_d_n11, assign52480_e79785_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 != 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0dep0, locals.var_ps0dep0_dn0, locals.var_ps0dep0_dn2, locals.var_ps0dep0_dn4, locals.var_ps0dep0_dn5, locals.var_ps0dep0_dn6, locals.var_ps0dep0_dn7, locals.var_ps0dep0_dn8, locals.var_ps0dep0_dn9, locals.var_ps0dep0_dn10, locals.var_ps0dep0_dn11, locals.var_ps0dep0_dn14,)
    }
};
        locals.var_ps0dep0 = assign52480_e79785;
        locals.var_ps0dep0_dn0 = assign52480_e79785_d_n0;
        locals.var_ps0dep0_dn2 = assign52480_e79785_d_n2;
        locals.var_ps0dep0_dn4 = assign52480_e79785_d_n4;
        locals.var_ps0dep0_dn5 = assign52480_e79785_d_n5;
        locals.var_ps0dep0_dn6 = assign52480_e79785_d_n6;
        locals.var_ps0dep0_dn7 = assign52480_e79785_d_n7;
        locals.var_ps0dep0_dn8 = assign52480_e79785_d_n8;
        locals.var_ps0dep0_dn9 = assign52480_e79785_d_n9;
        locals.var_ps0dep0_dn10 = assign52480_e79785_d_n10;
        locals.var_ps0dep0_dn11 = assign52480_e79785_d_n11;
        locals.var_ps0dep0_dn14 = assign52480_e79785_d_n14;
        locals.var_ps0dep0_rv = 0.0;

        let assign52490_e79789: f64 = (locals.var_ps0dep0 + 0.2);
        let assign52490_e79794: f64 = if ((locals.var_ps0dep < assign52490_e79789) && (0.2 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1338 = assign52490_e79794;
        locals.var_guard1338_rv = 0.0;

        let (assign52500_e79817, assign52500_e79817_d_n0, assign52500_e79817_d_n2, assign52500_e79817_d_n4, assign52500_e79817_d_n5, assign52500_e79817_d_n6, assign52500_e79817_d_n7, assign52500_e79817_d_n8, assign52500_e79817_d_n9, assign52500_e79817_d_n10, assign52500_e79817_d_n11, assign52500_e79817_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52500_e79813: f64 = (locals.var_ps0dep0 + 0.2);
        let assign52500_e79815: f64 = (assign52500_e79813 - locals.var_ps0dep);
        (assign52500_e79815, (locals.var_ps0dep0_dn0 - locals.var_ps0dep_dn0), (locals.var_ps0dep0_dn2 - locals.var_ps0dep_dn2), (locals.var_ps0dep0_dn4 - locals.var_ps0dep_dn4), (locals.var_ps0dep0_dn5 - locals.var_ps0dep_dn5), (locals.var_ps0dep0_dn6 - locals.var_ps0dep_dn6), (locals.var_ps0dep0_dn7 - locals.var_ps0dep_dn7), (locals.var_ps0dep0_dn8 - locals.var_ps0dep_dn8), (locals.var_ps0dep0_dn9 - locals.var_ps0dep_dn9), (locals.var_ps0dep0_dn10 - locals.var_ps0dep_dn10), (locals.var_ps0dep0_dn11 - locals.var_ps0dep_dn11), (locals.var_ps0dep0_dn14 - locals.var_ps0dep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign52500_e79817;
        locals.var_tmf1_dn0 = assign52500_e79817_d_n0;
        locals.var_tmf1_dn2 = assign52500_e79817_d_n2;
        locals.var_tmf1_dn4 = assign52500_e79817_d_n4;
        locals.var_tmf1_dn5 = assign52500_e79817_d_n5;
        locals.var_tmf1_dn6 = assign52500_e79817_d_n6;
        locals.var_tmf1_dn7 = assign52500_e79817_d_n7;
        locals.var_tmf1_dn8 = assign52500_e79817_d_n8;
        locals.var_tmf1_dn9 = assign52500_e79817_d_n9;
        locals.var_tmf1_dn10 = assign52500_e79817_d_n10;
        locals.var_tmf1_dn11 = assign52500_e79817_d_n11;
        locals.var_tmf1_dn14 = assign52500_e79817_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign52510_e79838, assign52510_e79838_d_n0, assign52510_e79838_d_n2, assign52510_e79838_d_n4, assign52510_e79838_d_n5, assign52510_e79838_d_n6, assign52510_e79838_d_n7, assign52510_e79838_d_n8, assign52510_e79838_d_n9, assign52510_e79838_d_n10, assign52510_e79838_d_n11, assign52510_e79838_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52510_e79836: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign52510_e79836, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign52510_e79838;
        locals.var_x2_dn0 = assign52510_e79838_d_n0;
        locals.var_x2_dn2 = assign52510_e79838_d_n2;
        locals.var_x2_dn4 = assign52510_e79838_d_n4;
        locals.var_x2_dn5 = assign52510_e79838_d_n5;
        locals.var_x2_dn6 = assign52510_e79838_d_n6;
        locals.var_x2_dn7 = assign52510_e79838_d_n7;
        locals.var_x2_dn8 = assign52510_e79838_d_n8;
        locals.var_x2_dn9 = assign52510_e79838_d_n9;
        locals.var_x2_dn10 = assign52510_e79838_d_n10;
        locals.var_x2_dn11 = assign52510_e79838_d_n11;
        locals.var_x2_dn14 = assign52510_e79838_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign52520_e79859, assign52520_e79859_d_n0, assign52520_e79859_d_n2, assign52520_e79859_d_n4, assign52520_e79859_d_n5, assign52520_e79859_d_n6, assign52520_e79859_d_n7, assign52520_e79859_d_n8, assign52520_e79859_d_n9, assign52520_e79859_d_n10, assign52520_e79859_d_n11, assign52520_e79859_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52520_e79857: f64 = (0.2 * 0.2);
        (assign52520_e79857, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign52520_e79859;
        locals.var_xmax2_dn0 = assign52520_e79859_d_n0;
        locals.var_xmax2_dn2 = assign52520_e79859_d_n2;
        locals.var_xmax2_dn4 = assign52520_e79859_d_n4;
        locals.var_xmax2_dn5 = assign52520_e79859_d_n5;
        locals.var_xmax2_dn6 = assign52520_e79859_d_n6;
        locals.var_xmax2_dn7 = assign52520_e79859_d_n7;
        locals.var_xmax2_dn8 = assign52520_e79859_d_n8;
        locals.var_xmax2_dn9 = assign52520_e79859_d_n9;
        locals.var_xmax2_dn10 = assign52520_e79859_d_n10;
        locals.var_xmax2_dn11 = assign52520_e79859_d_n11;
        locals.var_xmax2_dn14 = assign52520_e79859_d_n14;
        locals.var_xmax2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_190(
        locals: &mut StampLocals,
    ) {
        let (assign52530_e79878, assign52530_e79878_d_n0, assign52530_e79878_d_n2, assign52530_e79878_d_n4, assign52530_e79878_d_n5, assign52530_e79878_d_n6, assign52530_e79878_d_n7, assign52530_e79878_d_n8, assign52530_e79878_d_n9, assign52530_e79878_d_n10, assign52530_e79878_d_n11, assign52530_e79878_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign52530_e79878;
        locals.var_xp_dn0 = assign52530_e79878_d_n0;
        locals.var_xp_dn2 = assign52530_e79878_d_n2;
        locals.var_xp_dn4 = assign52530_e79878_d_n4;
        locals.var_xp_dn5 = assign52530_e79878_d_n5;
        locals.var_xp_dn6 = assign52530_e79878_d_n6;
        locals.var_xp_dn7 = assign52530_e79878_d_n7;
        locals.var_xp_dn8 = assign52530_e79878_d_n8;
        locals.var_xp_dn9 = assign52530_e79878_d_n9;
        locals.var_xp_dn10 = assign52530_e79878_d_n10;
        locals.var_xp_dn11 = assign52530_e79878_d_n11;
        locals.var_xp_dn14 = assign52530_e79878_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign52540_e79897, assign52540_e79897_d_n0, assign52540_e79897_d_n2, assign52540_e79897_d_n4, assign52540_e79897_d_n5, assign52540_e79897_d_n6, assign52540_e79897_d_n7, assign52540_e79897_d_n8, assign52540_e79897_d_n9, assign52540_e79897_d_n10, assign52540_e79897_d_n11, assign52540_e79897_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign52540_e79897;
        locals.var_xmp_dn0 = assign52540_e79897_d_n0;
        locals.var_xmp_dn2 = assign52540_e79897_d_n2;
        locals.var_xmp_dn4 = assign52540_e79897_d_n4;
        locals.var_xmp_dn5 = assign52540_e79897_d_n5;
        locals.var_xmp_dn6 = assign52540_e79897_d_n6;
        locals.var_xmp_dn7 = assign52540_e79897_d_n7;
        locals.var_xmp_dn8 = assign52540_e79897_d_n8;
        locals.var_xmp_dn9 = assign52540_e79897_d_n9;
        locals.var_xmp_dn10 = assign52540_e79897_d_n10;
        locals.var_xmp_dn11 = assign52540_e79897_d_n11;
        locals.var_xmp_dn14 = assign52540_e79897_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign52550_e79916,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52550_e79916;
        locals.var_m0_rv = 0.0;

        let (assign52560_e79935,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52560_e79935;
        locals.var_mm_rv = 0.0;

        let (assign52570_e79954, assign52570_e79954_d_n0, assign52570_e79954_d_n2, assign52570_e79954_d_n4, assign52570_e79954_d_n5, assign52570_e79954_d_n6, assign52570_e79954_d_n7, assign52570_e79954_d_n8, assign52570_e79954_d_n9, assign52570_e79954_d_n10, assign52570_e79954_d_n11, assign52570_e79954_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign52570_e79954;
        locals.var_arg_dn0 = assign52570_e79954_d_n0;
        locals.var_arg_dn2 = assign52570_e79954_d_n2;
        locals.var_arg_dn4 = assign52570_e79954_d_n4;
        locals.var_arg_dn5 = assign52570_e79954_d_n5;
        locals.var_arg_dn6 = assign52570_e79954_d_n6;
        locals.var_arg_dn7 = assign52570_e79954_d_n7;
        locals.var_arg_dn8 = assign52570_e79954_d_n8;
        locals.var_arg_dn9 = assign52570_e79954_d_n9;
        locals.var_arg_dn10 = assign52570_e79954_d_n10;
        locals.var_arg_dn11 = assign52570_e79954_d_n11;
        locals.var_arg_dn14 = assign52570_e79954_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign52580_e79973, assign52580_e79973_d_n0, assign52580_e79973_d_n2, assign52580_e79973_d_n4, assign52580_e79973_d_n5, assign52580_e79973_d_n6, assign52580_e79973_d_n7, assign52580_e79973_d_n8, assign52580_e79973_d_n9, assign52580_e79973_d_n10, assign52580_e79973_d_n11, assign52580_e79973_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52580_e79973;
        locals.var_dnm_dn0 = assign52580_e79973_d_n0;
        locals.var_dnm_dn2 = assign52580_e79973_d_n2;
        locals.var_dnm_dn4 = assign52580_e79973_d_n4;
        locals.var_dnm_dn5 = assign52580_e79973_d_n5;
        locals.var_dnm_dn6 = assign52580_e79973_d_n6;
        locals.var_dnm_dn7 = assign52580_e79973_d_n7;
        locals.var_dnm_dn8 = assign52580_e79973_d_n8;
        locals.var_dnm_dn9 = assign52580_e79973_d_n9;
        locals.var_dnm_dn10 = assign52580_e79973_d_n10;
        locals.var_dnm_dn11 = assign52580_e79973_d_n11;
        locals.var_dnm_dn14 = assign52580_e79973_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign52590_e79994, assign52590_e79994_d_n0, assign52590_e79994_d_n2, assign52590_e79994_d_n4, assign52590_e79994_d_n5, assign52590_e79994_d_n6, assign52590_e79994_d_n7, assign52590_e79994_d_n8, assign52590_e79994_d_n9, assign52590_e79994_d_n10, assign52590_e79994_d_n11, assign52590_e79994_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52590_e79992: f64 = (locals.var_xp * locals.var_x2);
        (assign52590_e79992, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign52590_e79994;
        locals.var_xp_dn0 = assign52590_e79994_d_n0;
        locals.var_xp_dn2 = assign52590_e79994_d_n2;
        locals.var_xp_dn4 = assign52590_e79994_d_n4;
        locals.var_xp_dn5 = assign52590_e79994_d_n5;
        locals.var_xp_dn6 = assign52590_e79994_d_n6;
        locals.var_xp_dn7 = assign52590_e79994_d_n7;
        locals.var_xp_dn8 = assign52590_e79994_d_n8;
        locals.var_xp_dn9 = assign52590_e79994_d_n9;
        locals.var_xp_dn10 = assign52590_e79994_d_n10;
        locals.var_xp_dn11 = assign52590_e79994_d_n11;
        locals.var_xp_dn14 = assign52590_e79994_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign52600_e80015, assign52600_e80015_d_n0, assign52600_e80015_d_n2, assign52600_e80015_d_n4, assign52600_e80015_d_n5, assign52600_e80015_d_n6, assign52600_e80015_d_n7, assign52600_e80015_d_n8, assign52600_e80015_d_n9, assign52600_e80015_d_n10, assign52600_e80015_d_n11, assign52600_e80015_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52600_e80013: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52600_e80013, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign52600_e80015;
        locals.var_xmp_dn0 = assign52600_e80015_d_n0;
        locals.var_xmp_dn2 = assign52600_e80015_d_n2;
        locals.var_xmp_dn4 = assign52600_e80015_d_n4;
        locals.var_xmp_dn5 = assign52600_e80015_d_n5;
        locals.var_xmp_dn6 = assign52600_e80015_d_n6;
        locals.var_xmp_dn7 = assign52600_e80015_d_n7;
        locals.var_xmp_dn8 = assign52600_e80015_d_n8;
        locals.var_xmp_dn9 = assign52600_e80015_d_n9;
        locals.var_xmp_dn10 = assign52600_e80015_d_n10;
        locals.var_xmp_dn11 = assign52600_e80015_d_n11;
        locals.var_xmp_dn14 = assign52600_e80015_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign52610_e80036, assign52610_e80036_d_n0, assign52610_e80036_d_n2, assign52610_e80036_d_n4, assign52610_e80036_d_n5, assign52610_e80036_d_n6, assign52610_e80036_d_n7, assign52610_e80036_d_n8, assign52610_e80036_d_n9, assign52610_e80036_d_n10, assign52610_e80036_d_n11, assign52610_e80036_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52610_e80034: f64 = (locals.var_xp * locals.var_x2);
        (assign52610_e80034, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign52610_e80036;
        locals.var_xp_dn0 = assign52610_e80036_d_n0;
        locals.var_xp_dn2 = assign52610_e80036_d_n2;
        locals.var_xp_dn4 = assign52610_e80036_d_n4;
        locals.var_xp_dn5 = assign52610_e80036_d_n5;
        locals.var_xp_dn6 = assign52610_e80036_d_n6;
        locals.var_xp_dn7 = assign52610_e80036_d_n7;
        locals.var_xp_dn8 = assign52610_e80036_d_n8;
        locals.var_xp_dn9 = assign52610_e80036_d_n9;
        locals.var_xp_dn10 = assign52610_e80036_d_n10;
        locals.var_xp_dn11 = assign52610_e80036_d_n11;
        locals.var_xp_dn14 = assign52610_e80036_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign52620_e80057, assign52620_e80057_d_n0, assign52620_e80057_d_n2, assign52620_e80057_d_n4, assign52620_e80057_d_n5, assign52620_e80057_d_n6, assign52620_e80057_d_n7, assign52620_e80057_d_n8, assign52620_e80057_d_n9, assign52620_e80057_d_n10, assign52620_e80057_d_n11, assign52620_e80057_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52620_e80055: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52620_e80055, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign52620_e80057;
        locals.var_xmp_dn0 = assign52620_e80057_d_n0;
        locals.var_xmp_dn2 = assign52620_e80057_d_n2;
        locals.var_xmp_dn4 = assign52620_e80057_d_n4;
        locals.var_xmp_dn5 = assign52620_e80057_d_n5;
        locals.var_xmp_dn6 = assign52620_e80057_d_n6;
        locals.var_xmp_dn7 = assign52620_e80057_d_n7;
        locals.var_xmp_dn8 = assign52620_e80057_d_n8;
        locals.var_xmp_dn9 = assign52620_e80057_d_n9;
        locals.var_xmp_dn10 = assign52620_e80057_d_n10;
        locals.var_xmp_dn11 = assign52620_e80057_d_n11;
        locals.var_xmp_dn14 = assign52620_e80057_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign52630_e80078, assign52630_e80078_d_n0, assign52630_e80078_d_n2, assign52630_e80078_d_n4, assign52630_e80078_d_n5, assign52630_e80078_d_n6, assign52630_e80078_d_n7, assign52630_e80078_d_n8, assign52630_e80078_d_n9, assign52630_e80078_d_n10, assign52630_e80078_d_n11, assign52630_e80078_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52630_e80076: f64 = (locals.var_xp + locals.var_xmp);
        (assign52630_e80076, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign52630_e80078;
        locals.var_arg_dn0 = assign52630_e80078_d_n0;
        locals.var_arg_dn2 = assign52630_e80078_d_n2;
        locals.var_arg_dn4 = assign52630_e80078_d_n4;
        locals.var_arg_dn5 = assign52630_e80078_d_n5;
        locals.var_arg_dn6 = assign52630_e80078_d_n6;
        locals.var_arg_dn7 = assign52630_e80078_d_n7;
        locals.var_arg_dn8 = assign52630_e80078_d_n8;
        locals.var_arg_dn9 = assign52630_e80078_d_n9;
        locals.var_arg_dn10 = assign52630_e80078_d_n10;
        locals.var_arg_dn11 = assign52630_e80078_d_n11;
        locals.var_arg_dn14 = assign52630_e80078_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign52640_e80097, assign52640_e80097_d_n0, assign52640_e80097_d_n2, assign52640_e80097_d_n4, assign52640_e80097_d_n5, assign52640_e80097_d_n6, assign52640_e80097_d_n7, assign52640_e80097_d_n8, assign52640_e80097_d_n9, assign52640_e80097_d_n10, assign52640_e80097_d_n11, assign52640_e80097_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52640_e80097;
        locals.var_dnm_dn0 = assign52640_e80097_d_n0;
        locals.var_dnm_dn2 = assign52640_e80097_d_n2;
        locals.var_dnm_dn4 = assign52640_e80097_d_n4;
        locals.var_dnm_dn5 = assign52640_e80097_d_n5;
        locals.var_dnm_dn6 = assign52640_e80097_d_n6;
        locals.var_dnm_dn7 = assign52640_e80097_d_n7;
        locals.var_dnm_dn8 = assign52640_e80097_d_n8;
        locals.var_dnm_dn9 = assign52640_e80097_d_n9;
        locals.var_dnm_dn10 = assign52640_e80097_d_n10;
        locals.var_dnm_dn11 = assign52640_e80097_d_n11;
        locals.var_dnm_dn14 = assign52640_e80097_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign52650_e80112: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1339 = assign52650_e80112;
        locals.var_guard1339_rv = 0.0;

        let assign52660_e80115: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1340 = assign52660_e80115;
        locals.var_guard1340_rv = 0.0;

        let (assign52670_e80138,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) && (locals.var_guard1339 != 0.0)) && (locals.var_guard1340 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52670_e80138;
        locals.var_mm_rv = 0.0;

        let assign52680_e80141: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1341 = assign52680_e80141;
        locals.var_guard1341_rv = 0.0;

        let (assign52690_e80167,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) && (locals.var_guard1339 != 0.0)) && (locals.var_guard1340 == 0.0)) && (locals.var_guard1341 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52690_e80167;
        locals.var_mm_rv = 0.0;

        let assign52700_e80170: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1342 = assign52700_e80170;
        locals.var_guard1342_rv = 0.0;

        let (assign52710_e80199,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) && (locals.var_guard1339 != 0.0)) && (locals.var_guard1340 == 0.0)) && (locals.var_guard1341 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52710_e80199;
        locals.var_mm_rv = 0.0;

        let assign52720_e80202: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1343 = assign52720_e80202;
        locals.var_guard1343_rv = 0.0;

        let (assign52730_e80234,) = {
    if ((((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) && (locals.var_guard1339 != 0.0)) && (locals.var_guard1340 == 0.0)) && (locals.var_guard1341 == 0.0)) && (locals.var_guard1342 == 0.0)) && (locals.var_guard1343 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52730_e80234;
        locals.var_mm_rv = 0.0;

        let (assign52740_e80255,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) && (locals.var_guard1339 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52740_e80255;
        locals.var_m0_rv = 0.0;

        let mut assign52750_loop_guard: usize = 0;
        while {
            let assign52750_cond_e80277: f64 = if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) && (locals.var_guard1339 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign52750_cond_e80277 != 0.0
        } {
            assign52750_loop_guard += 1;
            assert!(assign52750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52750_body0_e80299, assign52750_body0_e80299_d_n0, assign52750_body0_e80299_d_n2, assign52750_body0_e80299_d_n4, assign52750_body0_e80299_d_n5, assign52750_body0_e80299_d_n6, assign52750_body0_e80299_d_n7, assign52750_body0_e80299_d_n8, assign52750_body0_e80299_d_n9, assign52750_body0_e80299_d_n10, assign52750_body0_e80299_d_n11, assign52750_body0_e80299_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) && (locals.var_guard1339 != 0.0)) {
        let assign52750_body0_e80297: f64 = (locals.var_dnm).sqrt();
        (assign52750_body0_e80297, (locals.var_dnm_dn0 / (2.0 * assign52750_body0_e80297)), (locals.var_dnm_dn2 / (2.0 * assign52750_body0_e80297)), (locals.var_dnm_dn4 / (2.0 * assign52750_body0_e80297)), (locals.var_dnm_dn5 / (2.0 * assign52750_body0_e80297)), (locals.var_dnm_dn6 / (2.0 * assign52750_body0_e80297)), (locals.var_dnm_dn7 / (2.0 * assign52750_body0_e80297)), (locals.var_dnm_dn8 / (2.0 * assign52750_body0_e80297)), (locals.var_dnm_dn9 / (2.0 * assign52750_body0_e80297)), (locals.var_dnm_dn10 / (2.0 * assign52750_body0_e80297)), (locals.var_dnm_dn11 / (2.0 * assign52750_body0_e80297)), (locals.var_dnm_dn14 / (2.0 * assign52750_body0_e80297)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign52750_body0_e80299;
            locals.var_dnm_dn0 = assign52750_body0_e80299_d_n0;
            locals.var_dnm_dn2 = assign52750_body0_e80299_d_n2;
            locals.var_dnm_dn4 = assign52750_body0_e80299_d_n4;
            locals.var_dnm_dn5 = assign52750_body0_e80299_d_n5;
            locals.var_dnm_dn6 = assign52750_body0_e80299_d_n6;
            locals.var_dnm_dn7 = assign52750_body0_e80299_d_n7;
            locals.var_dnm_dn8 = assign52750_body0_e80299_d_n8;
            locals.var_dnm_dn9 = assign52750_body0_e80299_d_n9;
            locals.var_dnm_dn10 = assign52750_body0_e80299_d_n10;
            locals.var_dnm_dn11 = assign52750_body0_e80299_d_n11;
            locals.var_dnm_dn14 = assign52750_body0_e80299_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign52750_body1_e80322,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) && (locals.var_guard1339 != 0.0)) {
        let assign52750_body1_e80320: f64 = (locals.var_m0 + 1.0);
        (assign52750_body1_e80320,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign52750_body1_e80322;
            locals.var_m0_rv = 0.0;
        }

        let (assign52760_e80355, assign52760_e80355_d_n0, assign52760_e80355_d_n2, assign52760_e80355_d_n4, assign52760_e80355_d_n5, assign52760_e80355_d_n6, assign52760_e80355_d_n7, assign52760_e80355_d_n8, assign52760_e80355_d_n9, assign52760_e80355_d_n10, assign52760_e80355_d_n11, assign52760_e80355_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) && (locals.var_guard1339 == 0.0)) {
        let (assign52760_e80353, assign52760_e80353_d_n0, assign52760_e80353_d_n2, assign52760_e80353_d_n4, assign52760_e80353_d_n5, assign52760_e80353_d_n6, assign52760_e80353_d_n7, assign52760_e80353_d_n8, assign52760_e80353_d_n9, assign52760_e80353_d_n10, assign52760_e80353_d_n11, assign52760_e80353_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52760_e80350: f64 = (2.0 * 2.0);
                let assign52760_e80351: f64 = (1.0 / assign52760_e80350);
                let assign52760_e80352: f64 = (locals.var_dnm).powf(assign52760_e80351);
                (assign52760_e80352, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn0)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn2)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn4)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn5)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn6)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn7)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn8)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn9)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn10)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn11)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52760_e80351) as f64).is_finite() && ((assign52760_e80351) as f64).fract() == 0.0 { if assign52760_e80351 == 0.0 { 0.0 } else { (assign52760_e80351 * ((locals.var_dnm).powf(assign52760_e80351 - 1.0) * locals.var_dnm_dn14)) } } else { (assign52760_e80352 * (assign52760_e80351 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign52760_e80353, assign52760_e80353_d_n0, assign52760_e80353_d_n2, assign52760_e80353_d_n4, assign52760_e80353_d_n5, assign52760_e80353_d_n6, assign52760_e80353_d_n7, assign52760_e80353_d_n8, assign52760_e80353_d_n9, assign52760_e80353_d_n10, assign52760_e80353_d_n11, assign52760_e80353_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52760_e80355;
        locals.var_dnm_dn0 = assign52760_e80355_d_n0;
        locals.var_dnm_dn2 = assign52760_e80355_d_n2;
        locals.var_dnm_dn4 = assign52760_e80355_d_n4;
        locals.var_dnm_dn5 = assign52760_e80355_d_n5;
        locals.var_dnm_dn6 = assign52760_e80355_d_n6;
        locals.var_dnm_dn7 = assign52760_e80355_d_n7;
        locals.var_dnm_dn8 = assign52760_e80355_d_n8;
        locals.var_dnm_dn9 = assign52760_e80355_d_n9;
        locals.var_dnm_dn10 = assign52760_e80355_d_n10;
        locals.var_dnm_dn11 = assign52760_e80355_d_n11;
        locals.var_dnm_dn14 = assign52760_e80355_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign52770_e80376, assign52770_e80376_d_n0, assign52770_e80376_d_n2, assign52770_e80376_d_n4, assign52770_e80376_d_n5, assign52770_e80376_d_n6, assign52770_e80376_d_n7, assign52770_e80376_d_n8, assign52770_e80376_d_n9, assign52770_e80376_d_n10, assign52770_e80376_d_n11, assign52770_e80376_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52770_e80374: f64 = (1.0 / locals.var_dnm);
        (assign52770_e80374, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52770_e80376;
        locals.var_dnm_dn0 = assign52770_e80376_d_n0;
        locals.var_dnm_dn2 = assign52770_e80376_d_n2;
        locals.var_dnm_dn4 = assign52770_e80376_d_n4;
        locals.var_dnm_dn5 = assign52770_e80376_d_n5;
        locals.var_dnm_dn6 = assign52770_e80376_d_n6;
        locals.var_dnm_dn7 = assign52770_e80376_d_n7;
        locals.var_dnm_dn8 = assign52770_e80376_d_n8;
        locals.var_dnm_dn9 = assign52770_e80376_d_n9;
        locals.var_dnm_dn10 = assign52770_e80376_d_n10;
        locals.var_dnm_dn11 = assign52770_e80376_d_n11;
        locals.var_dnm_dn14 = assign52770_e80376_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign52780_e80399, assign52780_e80399_d_n0, assign52780_e80399_d_n2, assign52780_e80399_d_n4, assign52780_e80399_d_n5, assign52780_e80399_d_n6, assign52780_e80399_d_n7, assign52780_e80399_d_n8, assign52780_e80399_d_n9, assign52780_e80399_d_n10, assign52780_e80399_d_n11, assign52780_e80399_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52780_e80395: f64 = (locals.var_tmf1 * 0.2);
        let assign52780_e80397: f64 = (assign52780_e80395 * locals.var_dnm);
        (assign52780_e80397, (((locals.var_tmf1_dn0 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.2) * locals.var_dnm) + (assign52780_e80395 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign52780_e80399;
        locals.var_tmf0_dn0 = assign52780_e80399_d_n0;
        locals.var_tmf0_dn2 = assign52780_e80399_d_n2;
        locals.var_tmf0_dn4 = assign52780_e80399_d_n4;
        locals.var_tmf0_dn5 = assign52780_e80399_d_n5;
        locals.var_tmf0_dn6 = assign52780_e80399_d_n6;
        locals.var_tmf0_dn7 = assign52780_e80399_d_n7;
        locals.var_tmf0_dn8 = assign52780_e80399_d_n8;
        locals.var_tmf0_dn9 = assign52780_e80399_d_n9;
        locals.var_tmf0_dn10 = assign52780_e80399_d_n10;
        locals.var_tmf0_dn11 = assign52780_e80399_d_n11;
        locals.var_tmf0_dn14 = assign52780_e80399_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign52790_e80424, assign52790_e80424_d_n0, assign52790_e80424_d_n2, assign52790_e80424_d_n4, assign52790_e80424_d_n5, assign52790_e80424_d_n6, assign52790_e80424_d_n7, assign52790_e80424_d_n8, assign52790_e80424_d_n9, assign52790_e80424_d_n10, assign52790_e80424_d_n11, assign52790_e80424_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52790_e80418: f64 = (0.2 * locals.var_xmp);
        let assign52790_e80420: f64 = (assign52790_e80418 * locals.var_dnm);
        let assign52790_e80422: f64 = (assign52790_e80420 / locals.var_arg);
        (assign52790_e80422, ((((((0.2 * locals.var_xmp_dn0) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn0)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn2) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn2)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn4) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn4)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn5) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn5)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn6) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn6)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn7) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn7)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn8) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn8)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn9) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn9)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn10) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn10)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn11) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn11)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn14) * locals.var_dnm) + (assign52790_e80418 * locals.var_dnm_dn14)) * locals.var_arg) - (assign52790_e80420 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign52790_e80424;
        locals.var_t0_dn0 = assign52790_e80424_d_n0;
        locals.var_t0_dn2 = assign52790_e80424_d_n2;
        locals.var_t0_dn4 = assign52790_e80424_d_n4;
        locals.var_t0_dn5 = assign52790_e80424_d_n5;
        locals.var_t0_dn6 = assign52790_e80424_d_n6;
        locals.var_t0_dn7 = assign52790_e80424_d_n7;
        locals.var_t0_dn8 = assign52790_e80424_d_n8;
        locals.var_t0_dn9 = assign52790_e80424_d_n9;
        locals.var_t0_dn10 = assign52790_e80424_d_n10;
        locals.var_t0_dn11 = assign52790_e80424_d_n11;
        locals.var_t0_dn14 = assign52790_e80424_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign52800_e80447, assign52800_e80447_d_n0, assign52800_e80447_d_n2, assign52800_e80447_d_n4, assign52800_e80447_d_n5, assign52800_e80447_d_n6, assign52800_e80447_d_n7, assign52800_e80447_d_n8, assign52800_e80447_d_n9, assign52800_e80447_d_n10, assign52800_e80447_d_n11, assign52800_e80447_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        let assign52800_e80443: f64 = (locals.var_ps0dep0 + 0.2);
        let assign52800_e80445: f64 = (assign52800_e80443 - locals.var_tmf0);
        (assign52800_e80445, (locals.var_ps0dep0_dn0 - locals.var_tmf0_dn0), (locals.var_ps0dep0_dn2 - locals.var_tmf0_dn2), (locals.var_ps0dep0_dn4 - locals.var_tmf0_dn4), (locals.var_ps0dep0_dn5 - locals.var_tmf0_dn5), (locals.var_ps0dep0_dn6 - locals.var_tmf0_dn6), (locals.var_ps0dep0_dn7 - locals.var_tmf0_dn7), (locals.var_ps0dep0_dn8 - locals.var_tmf0_dn8), (locals.var_ps0dep0_dn9 - locals.var_tmf0_dn9), (locals.var_ps0dep0_dn10 - locals.var_tmf0_dn10), (locals.var_ps0dep0_dn11 - locals.var_tmf0_dn11), (locals.var_ps0dep0_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign52800_e80447;
        locals.var_ps0dep_dn0 = assign52800_e80447_d_n0;
        locals.var_ps0dep_dn2 = assign52800_e80447_d_n2;
        locals.var_ps0dep_dn4 = assign52800_e80447_d_n4;
        locals.var_ps0dep_dn5 = assign52800_e80447_d_n5;
        locals.var_ps0dep_dn6 = assign52800_e80447_d_n6;
        locals.var_ps0dep_dn7 = assign52800_e80447_d_n7;
        locals.var_ps0dep_dn8 = assign52800_e80447_d_n8;
        locals.var_ps0dep_dn9 = assign52800_e80447_d_n9;
        locals.var_ps0dep_dn10 = assign52800_e80447_d_n10;
        locals.var_ps0dep_dn11 = assign52800_e80447_d_n11;
        locals.var_ps0dep_dn14 = assign52800_e80447_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign52810_e80466, assign52810_e80466_d_n0, assign52810_e80466_d_n2, assign52810_e80466_d_n4, assign52810_e80466_d_n5, assign52810_e80466_d_n6, assign52810_e80466_d_n7, assign52810_e80466_d_n8, assign52810_e80466_d_n9, assign52810_e80466_d_n10, assign52810_e80466_d_n11, assign52810_e80466_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign52810_e80466;
        locals.var_t0_dn0 = assign52810_e80466_d_n0;
        locals.var_t0_dn2 = assign52810_e80466_d_n2;
        locals.var_t0_dn4 = assign52810_e80466_d_n4;
        locals.var_t0_dn5 = assign52810_e80466_d_n5;
        locals.var_t0_dn6 = assign52810_e80466_d_n6;
        locals.var_t0_dn7 = assign52810_e80466_d_n7;
        locals.var_t0_dn8 = assign52810_e80466_d_n8;
        locals.var_t0_dn9 = assign52810_e80466_d_n9;
        locals.var_t0_dn10 = assign52810_e80466_d_n10;
        locals.var_t0_dn11 = assign52810_e80466_d_n11;
        locals.var_t0_dn14 = assign52810_e80466_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign52820_e80486, assign52820_e80486_d_n0, assign52820_e80486_d_n2, assign52820_e80486_d_n4, assign52820_e80486_d_n5, assign52820_e80486_d_n6, assign52820_e80486_d_n7, assign52820_e80486_d_n8, assign52820_e80486_d_n9, assign52820_e80486_d_n10, assign52820_e80486_d_n11, assign52820_e80486_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign52820_e80486;
        locals.var_ps0dep_dn0 = assign52820_e80486_d_n0;
        locals.var_ps0dep_dn2 = assign52820_e80486_d_n2;
        locals.var_ps0dep_dn4 = assign52820_e80486_d_n4;
        locals.var_ps0dep_dn5 = assign52820_e80486_d_n5;
        locals.var_ps0dep_dn6 = assign52820_e80486_d_n6;
        locals.var_ps0dep_dn7 = assign52820_e80486_d_n7;
        locals.var_ps0dep_dn8 = assign52820_e80486_d_n8;
        locals.var_ps0dep_dn9 = assign52820_e80486_d_n9;
        locals.var_ps0dep_dn10 = assign52820_e80486_d_n10;
        locals.var_ps0dep_dn11 = assign52820_e80486_d_n11;
        locals.var_ps0dep_dn14 = assign52820_e80486_d_n14;
        locals.var_ps0dep_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_191(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52830_e80506, assign52830_e80506_d_n0, assign52830_e80506_d_n2, assign52830_e80506_d_n4, assign52830_e80506_d_n5, assign52830_e80506_d_n6, assign52830_e80506_d_n7, assign52830_e80506_d_n8, assign52830_e80506_d_n9, assign52830_e80506_d_n10, assign52830_e80506_d_n11, assign52830_e80506_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign52830_e80506;
        locals.var_t0_dn0 = assign52830_e80506_d_n0;
        locals.var_t0_dn2 = assign52830_e80506_d_n2;
        locals.var_t0_dn4 = assign52830_e80506_d_n4;
        locals.var_t0_dn5 = assign52830_e80506_d_n5;
        locals.var_t0_dn6 = assign52830_e80506_d_n6;
        locals.var_t0_dn7 = assign52830_e80506_d_n7;
        locals.var_t0_dn8 = assign52830_e80506_d_n8;
        locals.var_t0_dn9 = assign52830_e80506_d_n9;
        locals.var_t0_dn10 = assign52830_e80506_d_n10;
        locals.var_t0_dn11 = assign52830_e80506_d_n11;
        locals.var_t0_dn14 = assign52830_e80506_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign52840_e80520, assign52840_e80520_d_n0, assign52840_e80520_d_n2, assign52840_e80520_d_n4, assign52840_e80520_d_n5, assign52840_e80520_d_n6, assign52840_e80520_d_n7, assign52840_e80520_d_n8, assign52840_e80520_d_n9, assign52840_e80520_d_n10, assign52840_e80520_d_n11, assign52840_e80520_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    }
};
        locals.var_ps0_res = assign52840_e80520;
        locals.var_ps0_res_dn0 = assign52840_e80520_d_n0;
        locals.var_ps0_res_dn2 = assign52840_e80520_d_n2;
        locals.var_ps0_res_dn4 = assign52840_e80520_d_n4;
        locals.var_ps0_res_dn5 = assign52840_e80520_d_n5;
        locals.var_ps0_res_dn6 = assign52840_e80520_d_n6;
        locals.var_ps0_res_dn7 = assign52840_e80520_d_n7;
        locals.var_ps0_res_dn8 = assign52840_e80520_d_n8;
        locals.var_ps0_res_dn9 = assign52840_e80520_d_n9;
        locals.var_ps0_res_dn10 = assign52840_e80520_d_n10;
        locals.var_ps0_res_dn11 = assign52840_e80520_d_n11;
        locals.var_ps0_res_dn14 = assign52840_e80520_d_n14;
        locals.var_ps0_res_rv = 0.0;

        let (assign52850_e80539,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let (assign52850_e80537,) = {
            if (1e-6 >= p.p407) {
                (1e-6,)
            } else {
                (p.p407,)
            }
        };
        (assign52850_e80537,)
    } else {
        (locals.var_vgpdep_dlt__blk1144,)
    }
};
        locals.var_vgpdep_dlt__blk1144 = assign52850_e80539;
        locals.var_vgpdep_dlt__blk1144_rv = 0.0;

        let assign52860_e80543: f64 = (-locals.var_vgpdep_dlt__blk1144);
        let assign52860_e80548: f64 = if ((locals.var_ps0_res > assign52860_e80543) && (locals.var_vgpdep_dlt__blk1144 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1344 = assign52860_e80548;
        locals.var_guard1344_rv = 0.0;

        let (assign52870_e80568, assign52870_e80568_d_n0, assign52870_e80568_d_n2, assign52870_e80568_d_n4, assign52870_e80568_d_n5, assign52870_e80568_d_n6, assign52870_e80568_d_n7, assign52870_e80568_d_n8, assign52870_e80568_d_n9, assign52870_e80568_d_n10, assign52870_e80568_d_n11, assign52870_e80568_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign52870_e80564: f64 = locals.var_ps0_res;
        let assign52870_e80566: f64 = (assign52870_e80564 + locals.var_vgpdep_dlt__blk1144);
        (assign52870_e80566, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign52870_e80568;
        locals.var_tmf1_dn0 = assign52870_e80568_d_n0;
        locals.var_tmf1_dn2 = assign52870_e80568_d_n2;
        locals.var_tmf1_dn4 = assign52870_e80568_d_n4;
        locals.var_tmf1_dn5 = assign52870_e80568_d_n5;
        locals.var_tmf1_dn6 = assign52870_e80568_d_n6;
        locals.var_tmf1_dn7 = assign52870_e80568_d_n7;
        locals.var_tmf1_dn8 = assign52870_e80568_d_n8;
        locals.var_tmf1_dn9 = assign52870_e80568_d_n9;
        locals.var_tmf1_dn10 = assign52870_e80568_d_n10;
        locals.var_tmf1_dn11 = assign52870_e80568_d_n11;
        locals.var_tmf1_dn14 = assign52870_e80568_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign52880_e80586, assign52880_e80586_d_n0, assign52880_e80586_d_n2, assign52880_e80586_d_n4, assign52880_e80586_d_n5, assign52880_e80586_d_n6, assign52880_e80586_d_n7, assign52880_e80586_d_n8, assign52880_e80586_d_n9, assign52880_e80586_d_n10, assign52880_e80586_d_n11, assign52880_e80586_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign52880_e80584: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign52880_e80584, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign52880_e80586;
        locals.var_x2_dn0 = assign52880_e80586_d_n0;
        locals.var_x2_dn2 = assign52880_e80586_d_n2;
        locals.var_x2_dn4 = assign52880_e80586_d_n4;
        locals.var_x2_dn5 = assign52880_e80586_d_n5;
        locals.var_x2_dn6 = assign52880_e80586_d_n6;
        locals.var_x2_dn7 = assign52880_e80586_d_n7;
        locals.var_x2_dn8 = assign52880_e80586_d_n8;
        locals.var_x2_dn9 = assign52880_e80586_d_n9;
        locals.var_x2_dn10 = assign52880_e80586_d_n10;
        locals.var_x2_dn11 = assign52880_e80586_d_n11;
        locals.var_x2_dn14 = assign52880_e80586_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign52890_e80604, assign52890_e80604_d_n0, assign52890_e80604_d_n2, assign52890_e80604_d_n4, assign52890_e80604_d_n5, assign52890_e80604_d_n6, assign52890_e80604_d_n7, assign52890_e80604_d_n8, assign52890_e80604_d_n9, assign52890_e80604_d_n10, assign52890_e80604_d_n11, assign52890_e80604_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign52890_e80602: f64 = (locals.var_vgpdep_dlt__blk1144 * locals.var_vgpdep_dlt__blk1144);
        (assign52890_e80602, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign52890_e80604;
        locals.var_xmax2_dn0 = assign52890_e80604_d_n0;
        locals.var_xmax2_dn2 = assign52890_e80604_d_n2;
        locals.var_xmax2_dn4 = assign52890_e80604_d_n4;
        locals.var_xmax2_dn5 = assign52890_e80604_d_n5;
        locals.var_xmax2_dn6 = assign52890_e80604_d_n6;
        locals.var_xmax2_dn7 = assign52890_e80604_d_n7;
        locals.var_xmax2_dn8 = assign52890_e80604_d_n8;
        locals.var_xmax2_dn9 = assign52890_e80604_d_n9;
        locals.var_xmax2_dn10 = assign52890_e80604_d_n10;
        locals.var_xmax2_dn11 = assign52890_e80604_d_n11;
        locals.var_xmax2_dn14 = assign52890_e80604_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign52900_e80620, assign52900_e80620_d_n0, assign52900_e80620_d_n2, assign52900_e80620_d_n4, assign52900_e80620_d_n5, assign52900_e80620_d_n6, assign52900_e80620_d_n7, assign52900_e80620_d_n8, assign52900_e80620_d_n9, assign52900_e80620_d_n10, assign52900_e80620_d_n11, assign52900_e80620_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign52900_e80620;
        locals.var_xp_dn0 = assign52900_e80620_d_n0;
        locals.var_xp_dn2 = assign52900_e80620_d_n2;
        locals.var_xp_dn4 = assign52900_e80620_d_n4;
        locals.var_xp_dn5 = assign52900_e80620_d_n5;
        locals.var_xp_dn6 = assign52900_e80620_d_n6;
        locals.var_xp_dn7 = assign52900_e80620_d_n7;
        locals.var_xp_dn8 = assign52900_e80620_d_n8;
        locals.var_xp_dn9 = assign52900_e80620_d_n9;
        locals.var_xp_dn10 = assign52900_e80620_d_n10;
        locals.var_xp_dn11 = assign52900_e80620_d_n11;
        locals.var_xp_dn14 = assign52900_e80620_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign52910_e80636, assign52910_e80636_d_n0, assign52910_e80636_d_n2, assign52910_e80636_d_n4, assign52910_e80636_d_n5, assign52910_e80636_d_n6, assign52910_e80636_d_n7, assign52910_e80636_d_n8, assign52910_e80636_d_n9, assign52910_e80636_d_n10, assign52910_e80636_d_n11, assign52910_e80636_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign52910_e80636;
        locals.var_xmp_dn0 = assign52910_e80636_d_n0;
        locals.var_xmp_dn2 = assign52910_e80636_d_n2;
        locals.var_xmp_dn4 = assign52910_e80636_d_n4;
        locals.var_xmp_dn5 = assign52910_e80636_d_n5;
        locals.var_xmp_dn6 = assign52910_e80636_d_n6;
        locals.var_xmp_dn7 = assign52910_e80636_d_n7;
        locals.var_xmp_dn8 = assign52910_e80636_d_n8;
        locals.var_xmp_dn9 = assign52910_e80636_d_n9;
        locals.var_xmp_dn10 = assign52910_e80636_d_n10;
        locals.var_xmp_dn11 = assign52910_e80636_d_n11;
        locals.var_xmp_dn14 = assign52910_e80636_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign52920_e80652,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52920_e80652;
        locals.var_m0_rv = 0.0;

        let (assign52930_e80668,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52930_e80668;
        locals.var_mm_rv = 0.0;

        let (assign52940_e80684, assign52940_e80684_d_n0, assign52940_e80684_d_n2, assign52940_e80684_d_n4, assign52940_e80684_d_n5, assign52940_e80684_d_n6, assign52940_e80684_d_n7, assign52940_e80684_d_n8, assign52940_e80684_d_n9, assign52940_e80684_d_n10, assign52940_e80684_d_n11, assign52940_e80684_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign52940_e80684;
        locals.var_arg_dn0 = assign52940_e80684_d_n0;
        locals.var_arg_dn2 = assign52940_e80684_d_n2;
        locals.var_arg_dn4 = assign52940_e80684_d_n4;
        locals.var_arg_dn5 = assign52940_e80684_d_n5;
        locals.var_arg_dn6 = assign52940_e80684_d_n6;
        locals.var_arg_dn7 = assign52940_e80684_d_n7;
        locals.var_arg_dn8 = assign52940_e80684_d_n8;
        locals.var_arg_dn9 = assign52940_e80684_d_n9;
        locals.var_arg_dn10 = assign52940_e80684_d_n10;
        locals.var_arg_dn11 = assign52940_e80684_d_n11;
        locals.var_arg_dn14 = assign52940_e80684_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign52950_e80700, assign52950_e80700_d_n0, assign52950_e80700_d_n2, assign52950_e80700_d_n4, assign52950_e80700_d_n5, assign52950_e80700_d_n6, assign52950_e80700_d_n7, assign52950_e80700_d_n8, assign52950_e80700_d_n9, assign52950_e80700_d_n10, assign52950_e80700_d_n11, assign52950_e80700_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52950_e80700;
        locals.var_dnm_dn0 = assign52950_e80700_d_n0;
        locals.var_dnm_dn2 = assign52950_e80700_d_n2;
        locals.var_dnm_dn4 = assign52950_e80700_d_n4;
        locals.var_dnm_dn5 = assign52950_e80700_d_n5;
        locals.var_dnm_dn6 = assign52950_e80700_d_n6;
        locals.var_dnm_dn7 = assign52950_e80700_d_n7;
        locals.var_dnm_dn8 = assign52950_e80700_d_n8;
        locals.var_dnm_dn9 = assign52950_e80700_d_n9;
        locals.var_dnm_dn10 = assign52950_e80700_d_n10;
        locals.var_dnm_dn11 = assign52950_e80700_d_n11;
        locals.var_dnm_dn14 = assign52950_e80700_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign52960_e80716,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52960_e80716;
        locals.var_m0_rv = 0.0;

        let mut assign52970_loop_guard: usize = 0;
        while {
            let assign52970_cond_e80733: f64 = if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw__blk1145)) { 1.0 } else { 0.0 };
            assign52970_cond_e80733 != 0.0
        } {
            assign52970_loop_guard += 1;
            assert!(assign52970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52970_body0_e80751, assign52970_body0_e80751_d_n0, assign52970_body0_e80751_d_n2, assign52970_body0_e80751_d_n4, assign52970_body0_e80751_d_n5, assign52970_body0_e80751_d_n6, assign52970_body0_e80751_d_n7, assign52970_body0_e80751_d_n8, assign52970_body0_e80751_d_n9, assign52970_body0_e80751_d_n10, assign52970_body0_e80751_d_n11, assign52970_body0_e80751_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign52970_body0_e80749: f64 = (locals.var_xp * locals.var_x2);
        (assign52970_body0_e80749, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign52970_body0_e80751;
            locals.var_xp_dn0 = assign52970_body0_e80751_d_n0;
            locals.var_xp_dn2 = assign52970_body0_e80751_d_n2;
            locals.var_xp_dn4 = assign52970_body0_e80751_d_n4;
            locals.var_xp_dn5 = assign52970_body0_e80751_d_n5;
            locals.var_xp_dn6 = assign52970_body0_e80751_d_n6;
            locals.var_xp_dn7 = assign52970_body0_e80751_d_n7;
            locals.var_xp_dn8 = assign52970_body0_e80751_d_n8;
            locals.var_xp_dn9 = assign52970_body0_e80751_d_n9;
            locals.var_xp_dn10 = assign52970_body0_e80751_d_n10;
            locals.var_xp_dn11 = assign52970_body0_e80751_d_n11;
            locals.var_xp_dn14 = assign52970_body0_e80751_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign52970_body1_e80769, assign52970_body1_e80769_d_n0, assign52970_body1_e80769_d_n2, assign52970_body1_e80769_d_n4, assign52970_body1_e80769_d_n5, assign52970_body1_e80769_d_n6, assign52970_body1_e80769_d_n7, assign52970_body1_e80769_d_n8, assign52970_body1_e80769_d_n9, assign52970_body1_e80769_d_n10, assign52970_body1_e80769_d_n11, assign52970_body1_e80769_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign52970_body1_e80767: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52970_body1_e80767, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign52970_body1_e80769;
            locals.var_xmp_dn0 = assign52970_body1_e80769_d_n0;
            locals.var_xmp_dn2 = assign52970_body1_e80769_d_n2;
            locals.var_xmp_dn4 = assign52970_body1_e80769_d_n4;
            locals.var_xmp_dn5 = assign52970_body1_e80769_d_n5;
            locals.var_xmp_dn6 = assign52970_body1_e80769_d_n6;
            locals.var_xmp_dn7 = assign52970_body1_e80769_d_n7;
            locals.var_xmp_dn8 = assign52970_body1_e80769_d_n8;
            locals.var_xmp_dn9 = assign52970_body1_e80769_d_n9;
            locals.var_xmp_dn10 = assign52970_body1_e80769_d_n10;
            locals.var_xmp_dn11 = assign52970_body1_e80769_d_n11;
            locals.var_xmp_dn14 = assign52970_body1_e80769_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign52970_body2_e80787,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign52970_body2_e80785: f64 = (locals.var_m0 + 1.0);
        (assign52970_body2_e80785,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign52970_body2_e80787;
            locals.var_m0_rv = 0.0;
        }

        let (assign52980_e80805, assign52980_e80805_d_n0, assign52980_e80805_d_n2, assign52980_e80805_d_n4, assign52980_e80805_d_n5, assign52980_e80805_d_n6, assign52980_e80805_d_n7, assign52980_e80805_d_n8, assign52980_e80805_d_n9, assign52980_e80805_d_n10, assign52980_e80805_d_n11, assign52980_e80805_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign52980_e80803: f64 = (locals.var_xp + locals.var_xmp);
        (assign52980_e80803, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign52980_e80805;
        locals.var_arg_dn0 = assign52980_e80805_d_n0;
        locals.var_arg_dn2 = assign52980_e80805_d_n2;
        locals.var_arg_dn4 = assign52980_e80805_d_n4;
        locals.var_arg_dn5 = assign52980_e80805_d_n5;
        locals.var_arg_dn6 = assign52980_e80805_d_n6;
        locals.var_arg_dn7 = assign52980_e80805_d_n7;
        locals.var_arg_dn8 = assign52980_e80805_d_n8;
        locals.var_arg_dn9 = assign52980_e80805_d_n9;
        locals.var_arg_dn10 = assign52980_e80805_d_n10;
        locals.var_arg_dn11 = assign52980_e80805_d_n11;
        locals.var_arg_dn14 = assign52980_e80805_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign52990_e80821, assign52990_e80821_d_n0, assign52990_e80821_d_n2, assign52990_e80821_d_n4, assign52990_e80821_d_n5, assign52990_e80821_d_n6, assign52990_e80821_d_n7, assign52990_e80821_d_n8, assign52990_e80821_d_n9, assign52990_e80821_d_n10, assign52990_e80821_d_n11, assign52990_e80821_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52990_e80821;
        locals.var_dnm_dn0 = assign52990_e80821_d_n0;
        locals.var_dnm_dn2 = assign52990_e80821_d_n2;
        locals.var_dnm_dn4 = assign52990_e80821_d_n4;
        locals.var_dnm_dn5 = assign52990_e80821_d_n5;
        locals.var_dnm_dn6 = assign52990_e80821_d_n6;
        locals.var_dnm_dn7 = assign52990_e80821_d_n7;
        locals.var_dnm_dn8 = assign52990_e80821_d_n8;
        locals.var_dnm_dn9 = assign52990_e80821_d_n9;
        locals.var_dnm_dn10 = assign52990_e80821_d_n10;
        locals.var_dnm_dn11 = assign52990_e80821_d_n11;
        locals.var_dnm_dn14 = assign52990_e80821_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign53000_e80836: f64 = if ((((locals.var_vgpdep_pw__blk1145 == 1.0) || (locals.var_vgpdep_pw__blk1145 == 2.0)) || (locals.var_vgpdep_pw__blk1145 == 4.0)) || (locals.var_vgpdep_pw__blk1145 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1345 = assign53000_e80836;
        locals.var_guard1345_rv = 0.0;

        let assign53010_e80839: f64 = if locals.var_vgpdep_pw__blk1145 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1346 = assign53010_e80839;
        locals.var_guard1346_rv = 0.0;

        let (assign53020_e80859,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) && (locals.var_guard1345 != 0.0)) && (locals.var_guard1346 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53020_e80859;
        locals.var_mm_rv = 0.0;

        let assign53030_e80862: f64 = if locals.var_vgpdep_pw__blk1145 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1347 = assign53030_e80862;
        locals.var_guard1347_rv = 0.0;

        let (assign53040_e80885,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) && (locals.var_guard1345 != 0.0)) && (locals.var_guard1346 == 0.0)) && (locals.var_guard1347 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53040_e80885;
        locals.var_mm_rv = 0.0;

        let assign53050_e80888: f64 = if locals.var_vgpdep_pw__blk1145 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1348 = assign53050_e80888;
        locals.var_guard1348_rv = 0.0;

        let (assign53060_e80914,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) && (locals.var_guard1345 != 0.0)) && (locals.var_guard1346 == 0.0)) && (locals.var_guard1347 == 0.0)) && (locals.var_guard1348 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53060_e80914;
        locals.var_mm_rv = 0.0;

        let assign53070_e80917: f64 = if locals.var_vgpdep_pw__blk1145 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1349 = assign53070_e80917;
        locals.var_guard1349_rv = 0.0;

        let (assign53080_e80946,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) && (locals.var_guard1345 != 0.0)) && (locals.var_guard1346 == 0.0)) && (locals.var_guard1347 == 0.0)) && (locals.var_guard1348 == 0.0)) && (locals.var_guard1349 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53080_e80946;
        locals.var_mm_rv = 0.0;

        let (assign53090_e80964,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) && (locals.var_guard1345 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign53090_e80964;
        locals.var_m0_rv = 0.0;

        let mut assign53100_loop_guard: usize = 0;
        while {
            let assign53100_cond_e80983: f64 = if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) && (locals.var_guard1345 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign53100_cond_e80983 != 0.0
        } {
            assign53100_loop_guard += 1;
            assert!(assign53100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign53100_body0_e81002, assign53100_body0_e81002_d_n0, assign53100_body0_e81002_d_n2, assign53100_body0_e81002_d_n4, assign53100_body0_e81002_d_n5, assign53100_body0_e81002_d_n6, assign53100_body0_e81002_d_n7, assign53100_body0_e81002_d_n8, assign53100_body0_e81002_d_n9, assign53100_body0_e81002_d_n10, assign53100_body0_e81002_d_n11, assign53100_body0_e81002_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) && (locals.var_guard1345 != 0.0)) {
        let assign53100_body0_e81000: f64 = (locals.var_dnm).sqrt();
        (assign53100_body0_e81000, (locals.var_dnm_dn0 / (2.0 * assign53100_body0_e81000)), (locals.var_dnm_dn2 / (2.0 * assign53100_body0_e81000)), (locals.var_dnm_dn4 / (2.0 * assign53100_body0_e81000)), (locals.var_dnm_dn5 / (2.0 * assign53100_body0_e81000)), (locals.var_dnm_dn6 / (2.0 * assign53100_body0_e81000)), (locals.var_dnm_dn7 / (2.0 * assign53100_body0_e81000)), (locals.var_dnm_dn8 / (2.0 * assign53100_body0_e81000)), (locals.var_dnm_dn9 / (2.0 * assign53100_body0_e81000)), (locals.var_dnm_dn10 / (2.0 * assign53100_body0_e81000)), (locals.var_dnm_dn11 / (2.0 * assign53100_body0_e81000)), (locals.var_dnm_dn14 / (2.0 * assign53100_body0_e81000)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign53100_body0_e81002;
            locals.var_dnm_dn0 = assign53100_body0_e81002_d_n0;
            locals.var_dnm_dn2 = assign53100_body0_e81002_d_n2;
            locals.var_dnm_dn4 = assign53100_body0_e81002_d_n4;
            locals.var_dnm_dn5 = assign53100_body0_e81002_d_n5;
            locals.var_dnm_dn6 = assign53100_body0_e81002_d_n6;
            locals.var_dnm_dn7 = assign53100_body0_e81002_d_n7;
            locals.var_dnm_dn8 = assign53100_body0_e81002_d_n8;
            locals.var_dnm_dn9 = assign53100_body0_e81002_d_n9;
            locals.var_dnm_dn10 = assign53100_body0_e81002_d_n10;
            locals.var_dnm_dn11 = assign53100_body0_e81002_d_n11;
            locals.var_dnm_dn14 = assign53100_body0_e81002_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign53100_body1_e81022,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) && (locals.var_guard1345 != 0.0)) {
        let assign53100_body1_e81020: f64 = (locals.var_m0 + 1.0);
        (assign53100_body1_e81020,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign53100_body1_e81022;
            locals.var_m0_rv = 0.0;
        }

        let (assign53110_e81052, assign53110_e81052_d_n0, assign53110_e81052_d_n2, assign53110_e81052_d_n4, assign53110_e81052_d_n5, assign53110_e81052_d_n6, assign53110_e81052_d_n7, assign53110_e81052_d_n8, assign53110_e81052_d_n9, assign53110_e81052_d_n10, assign53110_e81052_d_n11, assign53110_e81052_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) && (locals.var_guard1345 == 0.0)) {
        let (assign53110_e81050, assign53110_e81050_d_n0, assign53110_e81050_d_n2, assign53110_e81050_d_n4, assign53110_e81050_d_n5, assign53110_e81050_d_n6, assign53110_e81050_d_n7, assign53110_e81050_d_n8, assign53110_e81050_d_n9, assign53110_e81050_d_n10, assign53110_e81050_d_n11, assign53110_e81050_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign53110_e81047: f64 = (2.0 * locals.var_vgpdep_pw__blk1145);
                let assign53110_e81048: f64 = (1.0 / assign53110_e81047);
                let assign53110_e81049: f64 = (locals.var_dnm).powf(assign53110_e81048);
                (assign53110_e81049, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn0)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn2)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn4)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn5)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn6)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn7)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn8)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn9)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn10)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn11)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53110_e81048) as f64).is_finite() && ((assign53110_e81048) as f64).fract() == 0.0 { if assign53110_e81048 == 0.0 { 0.0 } else { (assign53110_e81048 * ((locals.var_dnm).powf(assign53110_e81048 - 1.0) * locals.var_dnm_dn14)) } } else { (assign53110_e81049 * (assign53110_e81048 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign53110_e81050, assign53110_e81050_d_n0, assign53110_e81050_d_n2, assign53110_e81050_d_n4, assign53110_e81050_d_n5, assign53110_e81050_d_n6, assign53110_e81050_d_n7, assign53110_e81050_d_n8, assign53110_e81050_d_n9, assign53110_e81050_d_n10, assign53110_e81050_d_n11, assign53110_e81050_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53110_e81052;
        locals.var_dnm_dn0 = assign53110_e81052_d_n0;
        locals.var_dnm_dn2 = assign53110_e81052_d_n2;
        locals.var_dnm_dn4 = assign53110_e81052_d_n4;
        locals.var_dnm_dn5 = assign53110_e81052_d_n5;
        locals.var_dnm_dn6 = assign53110_e81052_d_n6;
        locals.var_dnm_dn7 = assign53110_e81052_d_n7;
        locals.var_dnm_dn8 = assign53110_e81052_d_n8;
        locals.var_dnm_dn9 = assign53110_e81052_d_n9;
        locals.var_dnm_dn10 = assign53110_e81052_d_n10;
        locals.var_dnm_dn11 = assign53110_e81052_d_n11;
        locals.var_dnm_dn14 = assign53110_e81052_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign53120_e81070, assign53120_e81070_d_n0, assign53120_e81070_d_n2, assign53120_e81070_d_n4, assign53120_e81070_d_n5, assign53120_e81070_d_n6, assign53120_e81070_d_n7, assign53120_e81070_d_n8, assign53120_e81070_d_n9, assign53120_e81070_d_n10, assign53120_e81070_d_n11, assign53120_e81070_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign53120_e81068: f64 = (1.0 / locals.var_dnm);
        (assign53120_e81068, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53120_e81070;
        locals.var_dnm_dn0 = assign53120_e81070_d_n0;
        locals.var_dnm_dn2 = assign53120_e81070_d_n2;
        locals.var_dnm_dn4 = assign53120_e81070_d_n4;
        locals.var_dnm_dn5 = assign53120_e81070_d_n5;
        locals.var_dnm_dn6 = assign53120_e81070_d_n6;
        locals.var_dnm_dn7 = assign53120_e81070_d_n7;
        locals.var_dnm_dn8 = assign53120_e81070_d_n8;
        locals.var_dnm_dn9 = assign53120_e81070_d_n9;
        locals.var_dnm_dn10 = assign53120_e81070_d_n10;
        locals.var_dnm_dn11 = assign53120_e81070_d_n11;
        locals.var_dnm_dn14 = assign53120_e81070_d_n14;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_192(
        locals: &mut StampLocals,
    ) {
        let (assign53130_e81090, assign53130_e81090_d_n0, assign53130_e81090_d_n2, assign53130_e81090_d_n4, assign53130_e81090_d_n5, assign53130_e81090_d_n6, assign53130_e81090_d_n7, assign53130_e81090_d_n8, assign53130_e81090_d_n9, assign53130_e81090_d_n10, assign53130_e81090_d_n11, assign53130_e81090_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign53130_e81086: f64 = (locals.var_tmf1 * locals.var_vgpdep_dlt__blk1144);
        let assign53130_e81088: f64 = (assign53130_e81086 * locals.var_dnm);
        (assign53130_e81088, (((locals.var_tmf1_dn0 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign53130_e81086 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign53130_e81090;
        locals.var_tmf0_dn0 = assign53130_e81090_d_n0;
        locals.var_tmf0_dn2 = assign53130_e81090_d_n2;
        locals.var_tmf0_dn4 = assign53130_e81090_d_n4;
        locals.var_tmf0_dn5 = assign53130_e81090_d_n5;
        locals.var_tmf0_dn6 = assign53130_e81090_d_n6;
        locals.var_tmf0_dn7 = assign53130_e81090_d_n7;
        locals.var_tmf0_dn8 = assign53130_e81090_d_n8;
        locals.var_tmf0_dn9 = assign53130_e81090_d_n9;
        locals.var_tmf0_dn10 = assign53130_e81090_d_n10;
        locals.var_tmf0_dn11 = assign53130_e81090_d_n11;
        locals.var_tmf0_dn14 = assign53130_e81090_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign53140_e81112, assign53140_e81112_d_n0, assign53140_e81112_d_n2, assign53140_e81112_d_n4, assign53140_e81112_d_n5, assign53140_e81112_d_n6, assign53140_e81112_d_n7, assign53140_e81112_d_n8, assign53140_e81112_d_n9, assign53140_e81112_d_n10, assign53140_e81112_d_n11, assign53140_e81112_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign53140_e81106: f64 = (locals.var_vgpdep_dlt__blk1144 * locals.var_xmp);
        let assign53140_e81108: f64 = (assign53140_e81106 * locals.var_dnm);
        let assign53140_e81110: f64 = (assign53140_e81108 / locals.var_arg);
        (assign53140_e81110, ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn0) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn0)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn2) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn2)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn4) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn4)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn5) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn5)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn6) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn6)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn7) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn7)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn8) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn8)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn9) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn9)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn10) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn10)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn11) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn11)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn14) * locals.var_dnm) + (assign53140_e81106 * locals.var_dnm_dn14)) * locals.var_arg) - (assign53140_e81108 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53140_e81112;
        locals.var_t0_dn0 = assign53140_e81112_d_n0;
        locals.var_t0_dn2 = assign53140_e81112_d_n2;
        locals.var_t0_dn4 = assign53140_e81112_d_n4;
        locals.var_t0_dn5 = assign53140_e81112_d_n5;
        locals.var_t0_dn6 = assign53140_e81112_d_n6;
        locals.var_t0_dn7 = assign53140_e81112_d_n7;
        locals.var_t0_dn8 = assign53140_e81112_d_n8;
        locals.var_t0_dn9 = assign53140_e81112_d_n9;
        locals.var_t0_dn10 = assign53140_e81112_d_n10;
        locals.var_t0_dn11 = assign53140_e81112_d_n11;
        locals.var_t0_dn14 = assign53140_e81112_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign53150_e81132, assign53150_e81132_d_n0, assign53150_e81132_d_n2, assign53150_e81132_d_n4, assign53150_e81132_d_n5, assign53150_e81132_d_n6, assign53150_e81132_d_n7, assign53150_e81132_d_n8, assign53150_e81132_d_n9, assign53150_e81132_d_n10, assign53150_e81132_d_n11, assign53150_e81132_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        let assign53150_e81128: f64 = (-locals.var_vgpdep_dlt__blk1144);
        let assign53150_e81130: f64 = (assign53150_e81128 + locals.var_tmf0);
        (assign53150_e81130, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign53150_e81132;
        locals.var_ps0dep_dn0 = assign53150_e81132_d_n0;
        locals.var_ps0dep_dn2 = assign53150_e81132_d_n2;
        locals.var_ps0dep_dn4 = assign53150_e81132_d_n4;
        locals.var_ps0dep_dn5 = assign53150_e81132_d_n5;
        locals.var_ps0dep_dn6 = assign53150_e81132_d_n6;
        locals.var_ps0dep_dn7 = assign53150_e81132_d_n7;
        locals.var_ps0dep_dn8 = assign53150_e81132_d_n8;
        locals.var_ps0dep_dn9 = assign53150_e81132_d_n9;
        locals.var_ps0dep_dn10 = assign53150_e81132_d_n10;
        locals.var_ps0dep_dn11 = assign53150_e81132_d_n11;
        locals.var_ps0dep_dn14 = assign53150_e81132_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign53160_e81148, assign53160_e81148_d_n0, assign53160_e81148_d_n2, assign53160_e81148_d_n4, assign53160_e81148_d_n5, assign53160_e81148_d_n6, assign53160_e81148_d_n7, assign53160_e81148_d_n8, assign53160_e81148_d_n9, assign53160_e81148_d_n10, assign53160_e81148_d_n11, assign53160_e81148_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53160_e81148;
        locals.var_t0_dn0 = assign53160_e81148_d_n0;
        locals.var_t0_dn2 = assign53160_e81148_d_n2;
        locals.var_t0_dn4 = assign53160_e81148_d_n4;
        locals.var_t0_dn5 = assign53160_e81148_d_n5;
        locals.var_t0_dn6 = assign53160_e81148_d_n6;
        locals.var_t0_dn7 = assign53160_e81148_d_n7;
        locals.var_t0_dn8 = assign53160_e81148_d_n8;
        locals.var_t0_dn9 = assign53160_e81148_d_n9;
        locals.var_t0_dn10 = assign53160_e81148_d_n10;
        locals.var_t0_dn11 = assign53160_e81148_d_n11;
        locals.var_t0_dn14 = assign53160_e81148_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign53170_e81165, assign53170_e81165_d_n0, assign53170_e81165_d_n2, assign53170_e81165_d_n4, assign53170_e81165_d_n5, assign53170_e81165_d_n6, assign53170_e81165_d_n7, assign53170_e81165_d_n8, assign53170_e81165_d_n9, assign53170_e81165_d_n10, assign53170_e81165_d_n11, assign53170_e81165_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 == 0.0)) {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign53170_e81165;
        locals.var_ps0dep_dn0 = assign53170_e81165_d_n0;
        locals.var_ps0dep_dn2 = assign53170_e81165_d_n2;
        locals.var_ps0dep_dn4 = assign53170_e81165_d_n4;
        locals.var_ps0dep_dn5 = assign53170_e81165_d_n5;
        locals.var_ps0dep_dn6 = assign53170_e81165_d_n6;
        locals.var_ps0dep_dn7 = assign53170_e81165_d_n7;
        locals.var_ps0dep_dn8 = assign53170_e81165_d_n8;
        locals.var_ps0dep_dn9 = assign53170_e81165_d_n9;
        locals.var_ps0dep_dn10 = assign53170_e81165_d_n10;
        locals.var_ps0dep_dn11 = assign53170_e81165_d_n11;
        locals.var_ps0dep_dn14 = assign53170_e81165_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign53180_e81182, assign53180_e81182_d_n0, assign53180_e81182_d_n2, assign53180_e81182_d_n4, assign53180_e81182_d_n5, assign53180_e81182_d_n6, assign53180_e81182_d_n7, assign53180_e81182_d_n8, assign53180_e81182_d_n9, assign53180_e81182_d_n10, assign53180_e81182_d_n11, assign53180_e81182_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1344 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53180_e81182;
        locals.var_t0_dn0 = assign53180_e81182_d_n0;
        locals.var_t0_dn2 = assign53180_e81182_d_n2;
        locals.var_t0_dn4 = assign53180_e81182_d_n4;
        locals.var_t0_dn5 = assign53180_e81182_d_n5;
        locals.var_t0_dn6 = assign53180_e81182_d_n6;
        locals.var_t0_dn7 = assign53180_e81182_d_n7;
        locals.var_t0_dn8 = assign53180_e81182_d_n8;
        locals.var_t0_dn9 = assign53180_e81182_d_n9;
        locals.var_t0_dn10 = assign53180_e81182_d_n10;
        locals.var_t0_dn11 = assign53180_e81182_d_n11;
        locals.var_t0_dn14 = assign53180_e81182_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign53190_e81197, assign53190_e81197_d_n0, assign53190_e81197_d_n2, assign53190_e81197_d_n4, assign53190_e81197_d_n5, assign53190_e81197_d_n6, assign53190_e81197_d_n7, assign53190_e81197_d_n8, assign53190_e81197_d_n9, assign53190_e81197_d_n10, assign53190_e81197_d_n11, assign53190_e81197_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign53190_e81195: f64 = (-locals.var_ps0dep);
        (assign53190_e81195, (-locals.var_ps0dep_dn0), (-locals.var_ps0dep_dn2), (-locals.var_ps0dep_dn4), (-locals.var_ps0dep_dn5), (-locals.var_ps0dep_dn6), (-locals.var_ps0dep_dn7), (-locals.var_ps0dep_dn8), (-locals.var_ps0dep_dn9), (-locals.var_ps0dep_dn10), (-locals.var_ps0dep_dn11), (-locals.var_ps0dep_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign53190_e81197;
        locals.var_ps0dep_dn0 = assign53190_e81197_d_n0;
        locals.var_ps0dep_dn2 = assign53190_e81197_d_n2;
        locals.var_ps0dep_dn4 = assign53190_e81197_d_n4;
        locals.var_ps0dep_dn5 = assign53190_e81197_d_n5;
        locals.var_ps0dep_dn6 = assign53190_e81197_d_n6;
        locals.var_ps0dep_dn7 = assign53190_e81197_d_n7;
        locals.var_ps0dep_dn8 = assign53190_e81197_d_n8;
        locals.var_ps0dep_dn9 = assign53190_e81197_d_n9;
        locals.var_ps0dep_dn10 = assign53190_e81197_d_n10;
        locals.var_ps0dep_dn11 = assign53190_e81197_d_n11;
        locals.var_ps0dep_dn14 = assign53190_e81197_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign53200_e81219, assign53200_e81219_d_n0, assign53200_e81219_d_n2, assign53200_e81219_d_n4, assign53200_e81219_d_n5, assign53200_e81219_d_n6, assign53200_e81219_d_n7, assign53200_e81219_d_n8, assign53200_e81219_d_n9, assign53200_e81219_d_n10, assign53200_e81219_d_n11, assign53200_e81219_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign53200_e81211: f64 = (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150);
        let assign53200_e81213: f64 = (assign53200_e81211 * locals.var_tnp__blk1150);
        let assign53200_e81215: f64 = (assign53200_e81213 / 2.0);
        let assign53200_e81217: f64 = (assign53200_e81215 / 1.034943e-10);
        (assign53200_e81217, ((((((locals.var_q_ndepm__blk1135_dn0 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn0)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn0)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn2 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn2)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn2)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn4 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn4)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn4)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn5 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn5)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn5)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn6 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn6)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn6)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn7 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn7)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn7)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn8 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn8)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn8)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn9 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn9)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn9)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn10 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn10)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn10)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn11 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn11)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn11)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn14 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn14)) * locals.var_tnp__blk1150) + (assign53200_e81211 * locals.var_tnp__blk1150_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1322, locals.var_dphi_sb__blk1322_dn0, locals.var_dphi_sb__blk1322_dn2, locals.var_dphi_sb__blk1322_dn4, locals.var_dphi_sb__blk1322_dn5, locals.var_dphi_sb__blk1322_dn6, locals.var_dphi_sb__blk1322_dn7, locals.var_dphi_sb__blk1322_dn8, locals.var_dphi_sb__blk1322_dn9, locals.var_dphi_sb__blk1322_dn10, locals.var_dphi_sb__blk1322_dn11, locals.var_dphi_sb__blk1322_dn14,)
    }
};
        locals.var_dphi_sb__blk1322 = assign53200_e81219;
        locals.var_dphi_sb__blk1322_dn0 = assign53200_e81219_d_n0;
        locals.var_dphi_sb__blk1322_dn2 = assign53200_e81219_d_n2;
        locals.var_dphi_sb__blk1322_dn4 = assign53200_e81219_d_n4;
        locals.var_dphi_sb__blk1322_dn5 = assign53200_e81219_d_n5;
        locals.var_dphi_sb__blk1322_dn6 = assign53200_e81219_d_n6;
        locals.var_dphi_sb__blk1322_dn7 = assign53200_e81219_d_n7;
        locals.var_dphi_sb__blk1322_dn8 = assign53200_e81219_d_n8;
        locals.var_dphi_sb__blk1322_dn9 = assign53200_e81219_d_n9;
        locals.var_dphi_sb__blk1322_dn10 = assign53200_e81219_d_n10;
        locals.var_dphi_sb__blk1322_dn11 = assign53200_e81219_d_n11;
        locals.var_dphi_sb__blk1322_dn14 = assign53200_e81219_d_n14;
        locals.var_dphi_sb__blk1322_rv = 0.0;

        let (assign53210_e81240, assign53210_e81240_d_n0, assign53210_e81240_d_n2, assign53210_e81240_d_n4, assign53210_e81240_d_n5, assign53210_e81240_d_n6, assign53210_e81240_d_n7, assign53210_e81240_d_n8, assign53210_e81240_d_n9, assign53210_e81240_d_n10, assign53210_e81240_d_n11, assign53210_e81240_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign53210_e81234: f64 = (2.0 * locals.var_beta);
        let assign53210_e81236: f64 = (assign53210_e81234 * locals.var_dphi_sb__blk1322);
        let assign53210_e81237: f64 = (assign53210_e81236).sqrt();
        let assign53210_e81238: f64 = (locals.var_wdepsubsl * assign53210_e81237);
        (assign53210_e81238, ((locals.var_wdepsubsl_dn0 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn0)) / (2.0 * assign53210_e81237)))), ((locals.var_wdepsubsl_dn2 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn2)) / (2.0 * assign53210_e81237)))), ((locals.var_wdepsubsl_dn4 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn4)) / (2.0 * assign53210_e81237)))), ((locals.var_wdepsubsl_dn5 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn5)) / (2.0 * assign53210_e81237)))), ((locals.var_wdepsubsl_dn6 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn6)) / (2.0 * assign53210_e81237)))), ((locals.var_wdepsubsl_dn7 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn7)) / (2.0 * assign53210_e81237)))), ((locals.var_wdepsubsl_dn8 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn8)) / (2.0 * assign53210_e81237)))), ((locals.var_wdepsubsl_dn9 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn9)) / (2.0 * assign53210_e81237)))), ((locals.var_wdepsubsl_dn10 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn10)) / (2.0 * assign53210_e81237)))), ((locals.var_wdepsubsl_dn11 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn11)) / (2.0 * assign53210_e81237)))), ((locals.var_wdepsubsl_dn14 * assign53210_e81237) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb__blk1322) + (assign53210_e81234 * locals.var_dphi_sb__blk1322_dn14)) / (2.0 * assign53210_e81237)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53210_e81240;
        locals.var_t0_dn0 = assign53210_e81240_d_n0;
        locals.var_t0_dn2 = assign53210_e81240_d_n2;
        locals.var_t0_dn4 = assign53210_e81240_d_n4;
        locals.var_t0_dn5 = assign53210_e81240_d_n5;
        locals.var_t0_dn6 = assign53210_e81240_d_n6;
        locals.var_t0_dn7 = assign53210_e81240_d_n7;
        locals.var_t0_dn8 = assign53210_e81240_d_n8;
        locals.var_t0_dn9 = assign53210_e81240_d_n9;
        locals.var_t0_dn10 = assign53210_e81240_d_n10;
        locals.var_t0_dn11 = assign53210_e81240_d_n11;
        locals.var_t0_dn14 = assign53210_e81240_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign53220_e81261, assign53220_e81261_d_n0, assign53220_e81261_d_n2, assign53220_e81261_d_n4, assign53220_e81261_d_n5, assign53220_e81261_d_n6, assign53220_e81261_d_n7, assign53220_e81261_d_n8, assign53220_e81261_d_n9, assign53220_e81261_d_n10, assign53220_e81261_d_n11, assign53220_e81261_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign53220_e81253: f64 = (locals.var_t0).exp();
        let assign53220_e81255: f64 = (-locals.var_t0);
        let assign53220_e81256: f64 = (assign53220_e81255).exp();
        let assign53220_e81257: f64 = (assign53220_e81253 + assign53220_e81256);
        let assign53220_e81259: f64 = (assign53220_e81257 / 2.0);
        (assign53220_e81259, (((assign53220_e81253 * locals.var_t0_dn0) + (assign53220_e81256 * (-locals.var_t0_dn0))) / 2.0), (((assign53220_e81253 * locals.var_t0_dn2) + (assign53220_e81256 * (-locals.var_t0_dn2))) / 2.0), (((assign53220_e81253 * locals.var_t0_dn4) + (assign53220_e81256 * (-locals.var_t0_dn4))) / 2.0), (((assign53220_e81253 * locals.var_t0_dn5) + (assign53220_e81256 * (-locals.var_t0_dn5))) / 2.0), (((assign53220_e81253 * locals.var_t0_dn6) + (assign53220_e81256 * (-locals.var_t0_dn6))) / 2.0), (((assign53220_e81253 * locals.var_t0_dn7) + (assign53220_e81256 * (-locals.var_t0_dn7))) / 2.0), (((assign53220_e81253 * locals.var_t0_dn8) + (assign53220_e81256 * (-locals.var_t0_dn8))) / 2.0), (((assign53220_e81253 * locals.var_t0_dn9) + (assign53220_e81256 * (-locals.var_t0_dn9))) / 2.0), (((assign53220_e81253 * locals.var_t0_dn10) + (assign53220_e81256 * (-locals.var_t0_dn10))) / 2.0), (((assign53220_e81253 * locals.var_t0_dn11) + (assign53220_e81256 * (-locals.var_t0_dn11))) / 2.0), (((assign53220_e81253 * locals.var_t0_dn14) + (assign53220_e81256 * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53220_e81261;
        locals.var_t1_dn0 = assign53220_e81261_d_n0;
        locals.var_t1_dn2 = assign53220_e81261_d_n2;
        locals.var_t1_dn4 = assign53220_e81261_d_n4;
        locals.var_t1_dn5 = assign53220_e81261_d_n5;
        locals.var_t1_dn6 = assign53220_e81261_d_n6;
        locals.var_t1_dn7 = assign53220_e81261_d_n7;
        locals.var_t1_dn8 = assign53220_e81261_d_n8;
        locals.var_t1_dn9 = assign53220_e81261_d_n9;
        locals.var_t1_dn10 = assign53220_e81261_d_n10;
        locals.var_t1_dn11 = assign53220_e81261_d_n11;
        locals.var_t1_dn14 = assign53220_e81261_d_n14;
        locals.var_t1_rv = 0.0;

        let assign53230_e81263: f64 = (locals.var_t0).abs();
        let assign53230_e81265: f64 = if assign53230_e81263 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1350 = assign53230_e81265;
        locals.var_guard1350_rv = 0.0;

        let (assign53240_e81284, assign53240_e81284_d_n0, assign53240_e81284_d_n2, assign53240_e81284_d_n4, assign53240_e81284_d_n5, assign53240_e81284_d_n6, assign53240_e81284_d_n7, assign53240_e81284_d_n8, assign53240_e81284_d_n9, assign53240_e81284_d_n10, assign53240_e81284_d_n11, assign53240_e81284_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1350 != 0.0)) {
        let assign53240_e81280: f64 = (locals.var_t1).ln();
        let assign53240_e81282: f64 = (assign53240_e81280 / locals.var_dphi_sb__blk1322);
        (assign53240_e81282, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn0)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn2)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn4)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn5)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn6)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn7)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn8)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn9)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn10)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn11)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign53240_e81280 * locals.var_dphi_sb__blk1322_dn14)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)),)
    } else {
        (locals.var_c_sb__blk1323, locals.var_c_sb__blk1323_dn0, locals.var_c_sb__blk1323_dn2, locals.var_c_sb__blk1323_dn4, locals.var_c_sb__blk1323_dn5, locals.var_c_sb__blk1323_dn6, locals.var_c_sb__blk1323_dn7, locals.var_c_sb__blk1323_dn8, locals.var_c_sb__blk1323_dn9, locals.var_c_sb__blk1323_dn10, locals.var_c_sb__blk1323_dn11, locals.var_c_sb__blk1323_dn14,)
    }
};
        locals.var_c_sb__blk1323 = assign53240_e81284;
        locals.var_c_sb__blk1323_dn0 = assign53240_e81284_d_n0;
        locals.var_c_sb__blk1323_dn2 = assign53240_e81284_d_n2;
        locals.var_c_sb__blk1323_dn4 = assign53240_e81284_d_n4;
        locals.var_c_sb__blk1323_dn5 = assign53240_e81284_d_n5;
        locals.var_c_sb__blk1323_dn6 = assign53240_e81284_d_n6;
        locals.var_c_sb__blk1323_dn7 = assign53240_e81284_d_n7;
        locals.var_c_sb__blk1323_dn8 = assign53240_e81284_d_n8;
        locals.var_c_sb__blk1323_dn9 = assign53240_e81284_d_n9;
        locals.var_c_sb__blk1323_dn10 = assign53240_e81284_d_n10;
        locals.var_c_sb__blk1323_dn11 = assign53240_e81284_d_n11;
        locals.var_c_sb__blk1323_dn14 = assign53240_e81284_d_n14;
        locals.var_c_sb__blk1323_rv = 0.0;

        let (assign53250_e81313, assign53250_e81313_d_n0, assign53250_e81313_d_n2, assign53250_e81313_d_n4, assign53250_e81313_d_n5, assign53250_e81313_d_n6, assign53250_e81313_d_n7, assign53250_e81313_d_n8, assign53250_e81313_d_n9, assign53250_e81313_d_n10, assign53250_e81313_d_n11, assign53250_e81313_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1350 == 0.0)) {
        let assign53250_e81301: f64 = (locals.var_wdepsubsl * locals.var_wdepsubsl);
        let assign53250_e81303: f64 = (assign53250_e81301 * locals.var_beta);
        let assign53250_e81307: f64 = (0.1666666666666667 * locals.var_t0);
        let assign53250_e81309: f64 = (assign53250_e81307 * locals.var_t0);
        let assign53250_e81310: f64 = (1.0 - assign53250_e81309);
        let assign53250_e81311: f64 = (assign53250_e81303 * assign53250_e81310);
        (assign53250_e81311, ((((((locals.var_wdepsubsl_dn0 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn0)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn0)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn0) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn0))))), ((((((locals.var_wdepsubsl_dn2 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn2)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn2)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn2) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn2))))), ((((((locals.var_wdepsubsl_dn4 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn4)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn4)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn4) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn4))))), ((((((locals.var_wdepsubsl_dn5 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn5)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn5)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn5) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn5))))), ((((((locals.var_wdepsubsl_dn6 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn6)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn6)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn6) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn6))))), ((((((locals.var_wdepsubsl_dn7 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn7)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn7)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn7) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn7))))), ((((((locals.var_wdepsubsl_dn8 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn8)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn8)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn8) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn8))))), ((((((locals.var_wdepsubsl_dn9 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn9)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn9)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn9) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn9))))), ((((((locals.var_wdepsubsl_dn10 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn10)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn10)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn10) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn10))))), ((((((locals.var_wdepsubsl_dn11 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn11)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn11)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn11) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn11))))), ((((((locals.var_wdepsubsl_dn14 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn14)) * locals.var_beta) + (assign53250_e81301 * locals.var_beta_dn14)) * assign53250_e81310) + (assign53250_e81303 * (-(((0.1666666666666667 * locals.var_t0_dn14) * locals.var_t0) + (assign53250_e81307 * locals.var_t0_dn14))))),)
    } else {
        (locals.var_c_sb__blk1323, locals.var_c_sb__blk1323_dn0, locals.var_c_sb__blk1323_dn2, locals.var_c_sb__blk1323_dn4, locals.var_c_sb__blk1323_dn5, locals.var_c_sb__blk1323_dn6, locals.var_c_sb__blk1323_dn7, locals.var_c_sb__blk1323_dn8, locals.var_c_sb__blk1323_dn9, locals.var_c_sb__blk1323_dn10, locals.var_c_sb__blk1323_dn11, locals.var_c_sb__blk1323_dn14,)
    }
};
        locals.var_c_sb__blk1323 = assign53250_e81313;
        locals.var_c_sb__blk1323_dn0 = assign53250_e81313_d_n0;
        locals.var_c_sb__blk1323_dn2 = assign53250_e81313_d_n2;
        locals.var_c_sb__blk1323_dn4 = assign53250_e81313_d_n4;
        locals.var_c_sb__blk1323_dn5 = assign53250_e81313_d_n5;
        locals.var_c_sb__blk1323_dn6 = assign53250_e81313_d_n6;
        locals.var_c_sb__blk1323_dn7 = assign53250_e81313_d_n7;
        locals.var_c_sb__blk1323_dn8 = assign53250_e81313_d_n8;
        locals.var_c_sb__blk1323_dn9 = assign53250_e81313_d_n9;
        locals.var_c_sb__blk1323_dn10 = assign53250_e81313_d_n10;
        locals.var_c_sb__blk1323_dn11 = assign53250_e81313_d_n11;
        locals.var_c_sb__blk1323_dn14 = assign53250_e81313_d_n14;
        locals.var_c_sb__blk1323_rv = 0.0;

        let (assign53260_e81329, assign53260_e81329_d_n0, assign53260_e81329_d_n2, assign53260_e81329_d_n4, assign53260_e81329_d_n5, assign53260_e81329_d_n6, assign53260_e81329_d_n7, assign53260_e81329_d_n8, assign53260_e81329_d_n9, assign53260_e81329_d_n10, assign53260_e81329_d_n11, assign53260_e81329_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign53260_e81327: f64 = (locals.var_c_sb__blk1323 * locals.var_ps0dep);
        (assign53260_e81327, ((locals.var_c_sb__blk1323_dn0 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn0)), ((locals.var_c_sb__blk1323_dn2 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn2)), ((locals.var_c_sb__blk1323_dn4 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn4)), ((locals.var_c_sb__blk1323_dn5 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn5)), ((locals.var_c_sb__blk1323_dn6 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn6)), ((locals.var_c_sb__blk1323_dn7 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn7)), ((locals.var_c_sb__blk1323_dn8 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn8)), ((locals.var_c_sb__blk1323_dn9 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn9)), ((locals.var_c_sb__blk1323_dn10 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn10)), ((locals.var_c_sb__blk1323_dn11 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn11)), ((locals.var_c_sb__blk1323_dn14 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign53260_e81329;
        locals.var_tx_dn0 = assign53260_e81329_d_n0;
        locals.var_tx_dn2 = assign53260_e81329_d_n2;
        locals.var_tx_dn4 = assign53260_e81329_d_n4;
        locals.var_tx_dn5 = assign53260_e81329_d_n5;
        locals.var_tx_dn6 = assign53260_e81329_d_n6;
        locals.var_tx_dn7 = assign53260_e81329_d_n7;
        locals.var_tx_dn8 = assign53260_e81329_d_n8;
        locals.var_tx_dn9 = assign53260_e81329_d_n9;
        locals.var_tx_dn10 = assign53260_e81329_d_n10;
        locals.var_tx_dn11 = assign53260_e81329_d_n11;
        locals.var_tx_dn14 = assign53260_e81329_d_n14;
        locals.var_tx_rv = 0.0;

        let assign53270_e81332: f64 = if locals.var_tx > 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1351 = assign53270_e81332;
        locals.var_guard1351_rv = 0.0;

        let (assign53280_e81350, assign53280_e81350_d_n0, assign53280_e81350_d_n2, assign53280_e81350_d_n4, assign53280_e81350_d_n5, assign53280_e81350_d_n6, assign53280_e81350_d_n7, assign53280_e81350_d_n8, assign53280_e81350_d_n9, assign53280_e81350_d_n10, assign53280_e81350_d_n11, assign53280_e81350_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 != 0.0)) {
        let assign53280_e81348: f64 = (locals.var_ps0dep - locals.var_dphi_sb__blk1322);
        (assign53280_e81348, (locals.var_ps0dep_dn0 - locals.var_dphi_sb__blk1322_dn0), (locals.var_ps0dep_dn2 - locals.var_dphi_sb__blk1322_dn2), (locals.var_ps0dep_dn4 - locals.var_dphi_sb__blk1322_dn4), (locals.var_ps0dep_dn5 - locals.var_dphi_sb__blk1322_dn5), (locals.var_ps0dep_dn6 - locals.var_dphi_sb__blk1322_dn6), (locals.var_ps0dep_dn7 - locals.var_dphi_sb__blk1322_dn7), (locals.var_ps0dep_dn8 - locals.var_dphi_sb__blk1322_dn8), (locals.var_ps0dep_dn9 - locals.var_dphi_sb__blk1322_dn9), (locals.var_ps0dep_dn10 - locals.var_dphi_sb__blk1322_dn10), (locals.var_ps0dep_dn11 - locals.var_dphi_sb__blk1322_dn11), (locals.var_ps0dep_dn14 - locals.var_dphi_sb__blk1322_dn14),)
    } else {
        (locals.var_pb0dep__blk1167, locals.var_pb0dep__blk1167_dn0, locals.var_pb0dep__blk1167_dn2, locals.var_pb0dep__blk1167_dn4, locals.var_pb0dep__blk1167_dn5, locals.var_pb0dep__blk1167_dn6, locals.var_pb0dep__blk1167_dn7, locals.var_pb0dep__blk1167_dn8, locals.var_pb0dep__blk1167_dn9, locals.var_pb0dep__blk1167_dn10, locals.var_pb0dep__blk1167_dn11, locals.var_pb0dep__blk1167_dn14,)
    }
};
        locals.var_pb0dep__blk1167 = assign53280_e81350;
        locals.var_pb0dep__blk1167_dn0 = assign53280_e81350_d_n0;
        locals.var_pb0dep__blk1167_dn2 = assign53280_e81350_d_n2;
        locals.var_pb0dep__blk1167_dn4 = assign53280_e81350_d_n4;
        locals.var_pb0dep__blk1167_dn5 = assign53280_e81350_d_n5;
        locals.var_pb0dep__blk1167_dn6 = assign53280_e81350_d_n6;
        locals.var_pb0dep__blk1167_dn7 = assign53280_e81350_d_n7;
        locals.var_pb0dep__blk1167_dn8 = assign53280_e81350_d_n8;
        locals.var_pb0dep__blk1167_dn9 = assign53280_e81350_d_n9;
        locals.var_pb0dep__blk1167_dn10 = assign53280_e81350_d_n10;
        locals.var_pb0dep__blk1167_dn11 = assign53280_e81350_d_n11;
        locals.var_pb0dep__blk1167_dn14 = assign53280_e81350_d_n14;
        locals.var_pb0dep__blk1167_rv = 0.0;

        let (assign53290_e81371, assign53290_e81371_d_n0, assign53290_e81371_d_n2, assign53290_e81371_d_n4, assign53290_e81371_d_n5, assign53290_e81371_d_n6, assign53290_e81371_d_n7, assign53290_e81371_d_n8, assign53290_e81371_d_n9, assign53290_e81371_d_n10, assign53290_e81371_d_n11, assign53290_e81371_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) {
        let assign53290_e81366: f64 = (-locals.var_c_sb__blk1323);
        let assign53290_e81368: f64 = (assign53290_e81366 * locals.var_dphi_sb__blk1322);
        let assign53290_e81369: f64 = (assign53290_e81368).exp();
        (assign53290_e81369, (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn0) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn0))), (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn2) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn2))), (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn4) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn4))), (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn5) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn5))), (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn6) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn6))), (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn7) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn7))), (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn8) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn8))), (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn9) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn9))), (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn10) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn10))), (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn11) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn11))), (assign53290_e81369 * (((-locals.var_c_sb__blk1323_dn14) * locals.var_dphi_sb__blk1322) + (assign53290_e81366 * locals.var_dphi_sb__blk1322_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53290_e81371;
        locals.var_t0_dn0 = assign53290_e81371_d_n0;
        locals.var_t0_dn2 = assign53290_e81371_d_n2;
        locals.var_t0_dn4 = assign53290_e81371_d_n4;
        locals.var_t0_dn5 = assign53290_e81371_d_n5;
        locals.var_t0_dn6 = assign53290_e81371_d_n6;
        locals.var_t0_dn7 = assign53290_e81371_d_n7;
        locals.var_t0_dn8 = assign53290_e81371_d_n8;
        locals.var_t0_dn9 = assign53290_e81371_d_n9;
        locals.var_t0_dn10 = assign53290_e81371_d_n10;
        locals.var_t0_dn11 = assign53290_e81371_d_n11;
        locals.var_t0_dn14 = assign53290_e81371_d_n14;
        locals.var_t0_rv = 0.0;

        let assign53300_e81373: f64 = (locals.var_tx).abs();
        let assign53300_e81375: f64 = if assign53300_e81373 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1352 = assign53300_e81375;
        locals.var_guard1352_rv = 0.0;

        let assign53310_e81378: f64 = if locals.var_tx >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1353 = assign53310_e81378;
        locals.var_guard1353_rv = 0.0;

        let (assign53320_e81405, assign53320_e81405_d_n0, assign53320_e81405_d_n2, assign53320_e81405_d_n4, assign53320_e81405_d_n5, assign53320_e81405_d_n6, assign53320_e81405_d_n7, assign53320_e81405_d_n8, assign53320_e81405_d_n9, assign53320_e81405_d_n10, assign53320_e81405_d_n11, assign53320_e81405_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) && (locals.var_guard1353 != 0.0)) {
        let assign53320_e81400: f64 = (1.0 + locals.var_tx);
        let assign53320_e81402: f64 = (assign53320_e81400 - 500.0);
        let assign53320_e81403: f64 = (1.403592217853e217 * assign53320_e81402);
        (assign53320_e81403, (1.403592217853e217 * locals.var_tx_dn0), (1.403592217853e217 * locals.var_tx_dn2), (1.403592217853e217 * locals.var_tx_dn4), (1.403592217853e217 * locals.var_tx_dn5), (1.403592217853e217 * locals.var_tx_dn6), (1.403592217853e217 * locals.var_tx_dn7), (1.403592217853e217 * locals.var_tx_dn8), (1.403592217853e217 * locals.var_tx_dn9), (1.403592217853e217 * locals.var_tx_dn10), (1.403592217853e217 * locals.var_tx_dn11), (1.403592217853e217 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53320_e81405;
        locals.var_t1_dn0 = assign53320_e81405_d_n0;
        locals.var_t1_dn2 = assign53320_e81405_d_n2;
        locals.var_t1_dn4 = assign53320_e81405_d_n4;
        locals.var_t1_dn5 = assign53320_e81405_d_n5;
        locals.var_t1_dn6 = assign53320_e81405_d_n6;
        locals.var_t1_dn7 = assign53320_e81405_d_n7;
        locals.var_t1_dn8 = assign53320_e81405_d_n8;
        locals.var_t1_dn9 = assign53320_e81405_d_n9;
        locals.var_t1_dn10 = assign53320_e81405_d_n10;
        locals.var_t1_dn11 = assign53320_e81405_d_n11;
        locals.var_t1_dn14 = assign53320_e81405_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign53330_e81426, assign53330_e81426_d_n0, assign53330_e81426_d_n2, assign53330_e81426_d_n4, assign53330_e81426_d_n5, assign53330_e81426_d_n6, assign53330_e81426_d_n7, assign53330_e81426_d_n8, assign53330_e81426_d_n9, assign53330_e81426_d_n10, assign53330_e81426_d_n11, assign53330_e81426_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) && (locals.var_guard1353 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign53330_e81426;
        locals.var_t3_dn0 = assign53330_e81426_d_n0;
        locals.var_t3_dn2 = assign53330_e81426_d_n2;
        locals.var_t3_dn4 = assign53330_e81426_d_n4;
        locals.var_t3_dn5 = assign53330_e81426_d_n5;
        locals.var_t3_dn6 = assign53330_e81426_d_n6;
        locals.var_t3_dn7 = assign53330_e81426_d_n7;
        locals.var_t3_dn8 = assign53330_e81426_d_n8;
        locals.var_t3_dn9 = assign53330_e81426_d_n9;
        locals.var_t3_dn10 = assign53330_e81426_d_n10;
        locals.var_t3_dn11 = assign53330_e81426_d_n11;
        locals.var_t3_dn14 = assign53330_e81426_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign53340_e81448, assign53340_e81448_d_n0, assign53340_e81448_d_n2, assign53340_e81448_d_n4, assign53340_e81448_d_n5, assign53340_e81448_d_n6, assign53340_e81448_d_n7, assign53340_e81448_d_n8, assign53340_e81448_d_n9, assign53340_e81448_d_n10, assign53340_e81448_d_n11, assign53340_e81448_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) && (locals.var_guard1353 == 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign53340_e81448;
        locals.var_tmf1_dn0 = assign53340_e81448_d_n0;
        locals.var_tmf1_dn2 = assign53340_e81448_d_n2;
        locals.var_tmf1_dn4 = assign53340_e81448_d_n4;
        locals.var_tmf1_dn5 = assign53340_e81448_d_n5;
        locals.var_tmf1_dn6 = assign53340_e81448_d_n6;
        locals.var_tmf1_dn7 = assign53340_e81448_d_n7;
        locals.var_tmf1_dn8 = assign53340_e81448_d_n8;
        locals.var_tmf1_dn9 = assign53340_e81448_d_n9;
        locals.var_tmf1_dn10 = assign53340_e81448_d_n10;
        locals.var_tmf1_dn11 = assign53340_e81448_d_n11;
        locals.var_tmf1_dn14 = assign53340_e81448_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign53350_e81470, assign53350_e81470_d_n0, assign53350_e81470_d_n2, assign53350_e81470_d_n4, assign53350_e81470_d_n5, assign53350_e81470_d_n6, assign53350_e81470_d_n7, assign53350_e81470_d_n8, assign53350_e81470_d_n9, assign53350_e81470_d_n10, assign53350_e81470_d_n11, assign53350_e81470_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) && (locals.var_guard1353 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53350_e81470;
        locals.var_t1_dn0 = assign53350_e81470_d_n0;
        locals.var_t1_dn2 = assign53350_e81470_d_n2;
        locals.var_t1_dn4 = assign53350_e81470_d_n4;
        locals.var_t1_dn5 = assign53350_e81470_d_n5;
        locals.var_t1_dn6 = assign53350_e81470_d_n6;
        locals.var_t1_dn7 = assign53350_e81470_d_n7;
        locals.var_t1_dn8 = assign53350_e81470_d_n8;
        locals.var_t1_dn9 = assign53350_e81470_d_n9;
        locals.var_t1_dn10 = assign53350_e81470_d_n10;
        locals.var_t1_dn11 = assign53350_e81470_d_n11;
        locals.var_t1_dn14 = assign53350_e81470_d_n14;
        locals.var_t1_rv = 0.0;

        let mut assign53360_loop_guard: usize = 0;
        while {
            let assign53360_cond_e81493: f64 = if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign53360_cond_e81493 != 0.0
        } {
            assign53360_loop_guard += 1;
            assert!(assign53360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign53360_body0_e81517, assign53360_body0_e81517_d_n0, assign53360_body0_e81517_d_n2, assign53360_body0_e81517_d_n4, assign53360_body0_e81517_d_n5, assign53360_body0_e81517_d_n6, assign53360_body0_e81517_d_n7, assign53360_body0_e81517_d_n8, assign53360_body0_e81517_d_n9, assign53360_body0_e81517_d_n10, assign53360_body0_e81517_d_n11, assign53360_body0_e81517_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) && (locals.var_guard1353 == 0.0)) {
        let assign53360_body0_e81515: f64 = (locals.var_t1 * 1.14200738981568e26);
        (assign53360_body0_e81515, (locals.var_t1_dn0 * 1.14200738981568e26), (locals.var_t1_dn2 * 1.14200738981568e26), (locals.var_t1_dn4 * 1.14200738981568e26), (locals.var_t1_dn5 * 1.14200738981568e26), (locals.var_t1_dn6 * 1.14200738981568e26), (locals.var_t1_dn7 * 1.14200738981568e26), (locals.var_t1_dn8 * 1.14200738981568e26), (locals.var_t1_dn9 * 1.14200738981568e26), (locals.var_t1_dn10 * 1.14200738981568e26), (locals.var_t1_dn11 * 1.14200738981568e26), (locals.var_t1_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign53360_body0_e81517;
            locals.var_t1_dn0 = assign53360_body0_e81517_d_n0;
            locals.var_t1_dn2 = assign53360_body0_e81517_d_n2;
            locals.var_t1_dn4 = assign53360_body0_e81517_d_n4;
            locals.var_t1_dn5 = assign53360_body0_e81517_d_n5;
            locals.var_t1_dn6 = assign53360_body0_e81517_d_n6;
            locals.var_t1_dn7 = assign53360_body0_e81517_d_n7;
            locals.var_t1_dn8 = assign53360_body0_e81517_d_n8;
            locals.var_t1_dn9 = assign53360_body0_e81517_d_n9;
            locals.var_t1_dn10 = assign53360_body0_e81517_d_n10;
            locals.var_t1_dn11 = assign53360_body0_e81517_d_n11;
            locals.var_t1_dn14 = assign53360_body0_e81517_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign53360_body1_e81541, assign53360_body1_e81541_d_n0, assign53360_body1_e81541_d_n2, assign53360_body1_e81541_d_n4, assign53360_body1_e81541_d_n5, assign53360_body1_e81541_d_n6, assign53360_body1_e81541_d_n7, assign53360_body1_e81541_d_n8, assign53360_body1_e81541_d_n9, assign53360_body1_e81541_d_n10, assign53360_body1_e81541_d_n11, assign53360_body1_e81541_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) && (locals.var_guard1353 == 0.0)) {
        let assign53360_body1_e81539: f64 = (locals.var_tmf1 - 60.0);
        (assign53360_body1_e81539, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign53360_body1_e81541;
            locals.var_tmf1_dn0 = assign53360_body1_e81541_d_n0;
            locals.var_tmf1_dn2 = assign53360_body1_e81541_d_n2;
            locals.var_tmf1_dn4 = assign53360_body1_e81541_d_n4;
            locals.var_tmf1_dn5 = assign53360_body1_e81541_d_n5;
            locals.var_tmf1_dn6 = assign53360_body1_e81541_d_n6;
            locals.var_tmf1_dn7 = assign53360_body1_e81541_d_n7;
            locals.var_tmf1_dn8 = assign53360_body1_e81541_d_n8;
            locals.var_tmf1_dn9 = assign53360_body1_e81541_d_n9;
            locals.var_tmf1_dn10 = assign53360_body1_e81541_d_n10;
            locals.var_tmf1_dn11 = assign53360_body1_e81541_d_n11;
            locals.var_tmf1_dn14 = assign53360_body1_e81541_d_n14;
            locals.var_tmf1_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_193(
        locals: &mut StampLocals,
    ) {
        let (assign53370_e81566, assign53370_e81566_d_n0, assign53370_e81566_d_n2, assign53370_e81566_d_n4, assign53370_e81566_d_n5, assign53370_e81566_d_n6, assign53370_e81566_d_n7, assign53370_e81566_d_n8, assign53370_e81566_d_n9, assign53370_e81566_d_n10, assign53370_e81566_d_n11, assign53370_e81566_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) && (locals.var_guard1353 == 0.0)) {
        let assign53370_e81563: f64 = (locals.var_tmf1).exp();
        let assign53370_e81564: f64 = (locals.var_t1 * assign53370_e81563);
        (assign53370_e81564, ((locals.var_t1_dn0 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn0))), ((locals.var_t1_dn2 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn2))), ((locals.var_t1_dn4 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn4))), ((locals.var_t1_dn5 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn5))), ((locals.var_t1_dn6 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn6))), ((locals.var_t1_dn7 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn7))), ((locals.var_t1_dn8 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn8))), ((locals.var_t1_dn9 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn9))), ((locals.var_t1_dn10 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn10))), ((locals.var_t1_dn11 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn11))), ((locals.var_t1_dn14 * assign53370_e81563) + (locals.var_t1 * (assign53370_e81563 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53370_e81566;
        locals.var_t1_dn0 = assign53370_e81566_d_n0;
        locals.var_t1_dn2 = assign53370_e81566_d_n2;
        locals.var_t1_dn4 = assign53370_e81566_d_n4;
        locals.var_t1_dn5 = assign53370_e81566_d_n5;
        locals.var_t1_dn6 = assign53370_e81566_d_n6;
        locals.var_t1_dn7 = assign53370_e81566_d_n7;
        locals.var_t1_dn8 = assign53370_e81566_d_n8;
        locals.var_t1_dn9 = assign53370_e81566_d_n9;
        locals.var_t1_dn10 = assign53370_e81566_d_n10;
        locals.var_t1_dn11 = assign53370_e81566_d_n11;
        locals.var_t1_dn14 = assign53370_e81566_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign53380_e81588, assign53380_e81588_d_n0, assign53380_e81588_d_n2, assign53380_e81588_d_n4, assign53380_e81588_d_n5, assign53380_e81588_d_n6, assign53380_e81588_d_n7, assign53380_e81588_d_n8, assign53380_e81588_d_n9, assign53380_e81588_d_n10, assign53380_e81588_d_n11, assign53380_e81588_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) && (locals.var_guard1353 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign53380_e81588;
        locals.var_t3_dn0 = assign53380_e81588_d_n0;
        locals.var_t3_dn2 = assign53380_e81588_d_n2;
        locals.var_t3_dn4 = assign53380_e81588_d_n4;
        locals.var_t3_dn5 = assign53380_e81588_d_n5;
        locals.var_t3_dn6 = assign53380_e81588_d_n6;
        locals.var_t3_dn7 = assign53380_e81588_d_n7;
        locals.var_t3_dn8 = assign53380_e81588_d_n8;
        locals.var_t3_dn9 = assign53380_e81588_d_n9;
        locals.var_t3_dn10 = assign53380_e81588_d_n10;
        locals.var_t3_dn11 = assign53380_e81588_d_n11;
        locals.var_t3_dn14 = assign53380_e81588_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign53390_e81609, assign53390_e81609_d_n0, assign53390_e81609_d_n2, assign53390_e81609_d_n4, assign53390_e81609_d_n5, assign53390_e81609_d_n6, assign53390_e81609_d_n7, assign53390_e81609_d_n8, assign53390_e81609_d_n9, assign53390_e81609_d_n10, assign53390_e81609_d_n11, assign53390_e81609_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) {
        let assign53390_e81607: f64 = (locals.var_t1 * locals.var_t0);
        (assign53390_e81607, ((locals.var_t1_dn0 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn0)), ((locals.var_t1_dn2 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn2)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn11 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn11)), ((locals.var_t1_dn14 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53390_e81609;
        locals.var_t1_dn0 = assign53390_e81609_d_n0;
        locals.var_t1_dn2 = assign53390_e81609_d_n2;
        locals.var_t1_dn4 = assign53390_e81609_d_n4;
        locals.var_t1_dn5 = assign53390_e81609_d_n5;
        locals.var_t1_dn6 = assign53390_e81609_d_n6;
        locals.var_t1_dn7 = assign53390_e81609_d_n7;
        locals.var_t1_dn8 = assign53390_e81609_d_n8;
        locals.var_t1_dn9 = assign53390_e81609_d_n9;
        locals.var_t1_dn10 = assign53390_e81609_d_n10;
        locals.var_t1_dn11 = assign53390_e81609_d_n11;
        locals.var_t1_dn14 = assign53390_e81609_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign53400_e81630, assign53400_e81630_d_n0, assign53400_e81630_d_n2, assign53400_e81630_d_n4, assign53400_e81630_d_n5, assign53400_e81630_d_n6, assign53400_e81630_d_n7, assign53400_e81630_d_n8, assign53400_e81630_d_n9, assign53400_e81630_d_n10, assign53400_e81630_d_n11, assign53400_e81630_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 != 0.0)) {
        let assign53400_e81628: f64 = (locals.var_t1 - locals.var_t0);
        (assign53400_e81628, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign53400_e81630;
        locals.var_t2_dn0 = assign53400_e81630_d_n0;
        locals.var_t2_dn2 = assign53400_e81630_d_n2;
        locals.var_t2_dn4 = assign53400_e81630_d_n4;
        locals.var_t2_dn5 = assign53400_e81630_d_n5;
        locals.var_t2_dn6 = assign53400_e81630_d_n6;
        locals.var_t2_dn7 = assign53400_e81630_d_n7;
        locals.var_t2_dn8 = assign53400_e81630_d_n8;
        locals.var_t2_dn9 = assign53400_e81630_d_n9;
        locals.var_t2_dn10 = assign53400_e81630_d_n10;
        locals.var_t2_dn11 = assign53400_e81630_d_n11;
        locals.var_t2_dn14 = assign53400_e81630_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign53410_e81654, assign53410_e81654_d_n0, assign53410_e81654_d_n2, assign53410_e81654_d_n4, assign53410_e81654_d_n5, assign53410_e81654_d_n6, assign53410_e81654_d_n7, assign53410_e81654_d_n8, assign53410_e81654_d_n9, assign53410_e81654_d_n10, assign53410_e81654_d_n11, assign53410_e81654_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 == 0.0)) {
        let assign53410_e81650: f64 = (1.0 + locals.var_tx);
        let assign53410_e81652: f64 = (assign53410_e81650 * locals.var_t0);
        (assign53410_e81652, ((locals.var_tx_dn0 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn0)), ((locals.var_tx_dn2 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn2)), ((locals.var_tx_dn4 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn4)), ((locals.var_tx_dn5 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn5)), ((locals.var_tx_dn6 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn6)), ((locals.var_tx_dn7 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn7)), ((locals.var_tx_dn8 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn8)), ((locals.var_tx_dn9 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn9)), ((locals.var_tx_dn10 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn10)), ((locals.var_tx_dn11 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn11)), ((locals.var_tx_dn14 * locals.var_t0) + (assign53410_e81650 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53410_e81654;
        locals.var_t1_dn0 = assign53410_e81654_d_n0;
        locals.var_t1_dn2 = assign53410_e81654_d_n2;
        locals.var_t1_dn4 = assign53410_e81654_d_n4;
        locals.var_t1_dn5 = assign53410_e81654_d_n5;
        locals.var_t1_dn6 = assign53410_e81654_d_n6;
        locals.var_t1_dn7 = assign53410_e81654_d_n7;
        locals.var_t1_dn8 = assign53410_e81654_d_n8;
        locals.var_t1_dn9 = assign53410_e81654_d_n9;
        locals.var_t1_dn10 = assign53410_e81654_d_n10;
        locals.var_t1_dn11 = assign53410_e81654_d_n11;
        locals.var_t1_dn14 = assign53410_e81654_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign53420_e81682, assign53420_e81682_d_n0, assign53420_e81682_d_n2, assign53420_e81682_d_n4, assign53420_e81682_d_n5, assign53420_e81682_d_n6, assign53420_e81682_d_n7, assign53420_e81682_d_n8, assign53420_e81682_d_n9, assign53420_e81682_d_n10, assign53420_e81682_d_n11, assign53420_e81682_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1352 == 0.0)) {
        let assign53420_e81676: f64 = (locals.var_tx / 2.0);
        let assign53420_e81677: f64 = (1.0 + assign53420_e81676);
        let assign53420_e81678: f64 = (locals.var_tx * assign53420_e81677);
        let assign53420_e81680: f64 = (assign53420_e81678 * locals.var_t0);
        (assign53420_e81680, ((((locals.var_tx_dn0 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn0 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn0)), ((((locals.var_tx_dn2 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn2 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn2)), ((((locals.var_tx_dn4 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn4 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn4)), ((((locals.var_tx_dn5 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn5 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn5)), ((((locals.var_tx_dn6 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn6 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn6)), ((((locals.var_tx_dn7 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn7 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn7)), ((((locals.var_tx_dn8 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn8 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn8)), ((((locals.var_tx_dn9 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn9 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn9)), ((((locals.var_tx_dn10 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn10 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn10)), ((((locals.var_tx_dn11 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn11 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn11)), ((((locals.var_tx_dn14 * assign53420_e81677) + (locals.var_tx * (locals.var_tx_dn14 / 2.0))) * locals.var_t0) + (assign53420_e81678 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign53420_e81682;
        locals.var_t2_dn0 = assign53420_e81682_d_n0;
        locals.var_t2_dn2 = assign53420_e81682_d_n2;
        locals.var_t2_dn4 = assign53420_e81682_d_n4;
        locals.var_t2_dn5 = assign53420_e81682_d_n5;
        locals.var_t2_dn6 = assign53420_e81682_d_n6;
        locals.var_t2_dn7 = assign53420_e81682_d_n7;
        locals.var_t2_dn8 = assign53420_e81682_d_n8;
        locals.var_t2_dn9 = assign53420_e81682_d_n9;
        locals.var_t2_dn10 = assign53420_e81682_d_n10;
        locals.var_t2_dn11 = assign53420_e81682_d_n11;
        locals.var_t2_dn14 = assign53420_e81682_d_n14;
        locals.var_t2_rv = 0.0;

        let assign53430_e81684: f64 = (locals.var_t2).abs();
        let assign53430_e81686: f64 = if assign53430_e81684 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1354 = assign53430_e81686;
        locals.var_guard1354_rv = 0.0;

        let (assign53440_e81710, assign53440_e81710_d_n0, assign53440_e81710_d_n2, assign53440_e81710_d_n4, assign53440_e81710_d_n5, assign53440_e81710_d_n6, assign53440_e81710_d_n7, assign53440_e81710_d_n8, assign53440_e81710_d_n9, assign53440_e81710_d_n10, assign53440_e81710_d_n11, assign53440_e81710_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1354 != 0.0)) {
        let assign53440_e81705: f64 = (1.0 + locals.var_t2);
        let assign53440_e81706: f64 = (assign53440_e81705).ln();
        let assign53440_e81708: f64 = (assign53440_e81706 / locals.var_c_sb__blk1323);
        (assign53440_e81708, ((((locals.var_t2_dn0 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn0)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn2 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn2)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn4 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn4)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn5 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn5)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn6 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn6)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn7 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn7)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn8 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn8)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn9 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn9)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn10 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn10)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn11 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn11)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn14 / assign53440_e81705) * locals.var_c_sb__blk1323) - (assign53440_e81706 * locals.var_c_sb__blk1323_dn14)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)),)
    } else {
        (locals.var_pb0dep__blk1167, locals.var_pb0dep__blk1167_dn0, locals.var_pb0dep__blk1167_dn2, locals.var_pb0dep__blk1167_dn4, locals.var_pb0dep__blk1167_dn5, locals.var_pb0dep__blk1167_dn6, locals.var_pb0dep__blk1167_dn7, locals.var_pb0dep__blk1167_dn8, locals.var_pb0dep__blk1167_dn9, locals.var_pb0dep__blk1167_dn10, locals.var_pb0dep__blk1167_dn11, locals.var_pb0dep__blk1167_dn14,)
    }
};
        locals.var_pb0dep__blk1167 = assign53440_e81710;
        locals.var_pb0dep__blk1167_dn0 = assign53440_e81710_d_n0;
        locals.var_pb0dep__blk1167_dn2 = assign53440_e81710_d_n2;
        locals.var_pb0dep__blk1167_dn4 = assign53440_e81710_d_n4;
        locals.var_pb0dep__blk1167_dn5 = assign53440_e81710_d_n5;
        locals.var_pb0dep__blk1167_dn6 = assign53440_e81710_d_n6;
        locals.var_pb0dep__blk1167_dn7 = assign53440_e81710_d_n7;
        locals.var_pb0dep__blk1167_dn8 = assign53440_e81710_d_n8;
        locals.var_pb0dep__blk1167_dn9 = assign53440_e81710_d_n9;
        locals.var_pb0dep__blk1167_dn10 = assign53440_e81710_d_n10;
        locals.var_pb0dep__blk1167_dn11 = assign53440_e81710_d_n11;
        locals.var_pb0dep__blk1167_dn14 = assign53440_e81710_d_n14;
        locals.var_pb0dep__blk1167_rv = 0.0;

        let (assign53450_e81732, assign53450_e81732_d_n0, assign53450_e81732_d_n2, assign53450_e81732_d_n4, assign53450_e81732_d_n5, assign53450_e81732_d_n6, assign53450_e81732_d_n7, assign53450_e81732_d_n8, assign53450_e81732_d_n9, assign53450_e81732_d_n10, assign53450_e81732_d_n11, assign53450_e81732_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_guard1354 == 0.0)) {
        let assign53450_e81730: f64 = (locals.var_t2 / locals.var_c_sb__blk1323);
        (assign53450_e81730, (((locals.var_t2_dn0 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn0)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn2 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn2)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn4 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn4)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn5 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn5)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn6 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn6)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn7 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn7)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn8 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn8)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn9 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn9)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn10 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn10)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn11 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn11)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn14 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn14)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)),)
    } else {
        (locals.var_pb0dep__blk1167, locals.var_pb0dep__blk1167_dn0, locals.var_pb0dep__blk1167_dn2, locals.var_pb0dep__blk1167_dn4, locals.var_pb0dep__blk1167_dn5, locals.var_pb0dep__blk1167_dn6, locals.var_pb0dep__blk1167_dn7, locals.var_pb0dep__blk1167_dn8, locals.var_pb0dep__blk1167_dn9, locals.var_pb0dep__blk1167_dn10, locals.var_pb0dep__blk1167_dn11, locals.var_pb0dep__blk1167_dn14,)
    }
};
        locals.var_pb0dep__blk1167 = assign53450_e81732;
        locals.var_pb0dep__blk1167_dn0 = assign53450_e81732_d_n0;
        locals.var_pb0dep__blk1167_dn2 = assign53450_e81732_d_n2;
        locals.var_pb0dep__blk1167_dn4 = assign53450_e81732_d_n4;
        locals.var_pb0dep__blk1167_dn5 = assign53450_e81732_d_n5;
        locals.var_pb0dep__blk1167_dn6 = assign53450_e81732_d_n6;
        locals.var_pb0dep__blk1167_dn7 = assign53450_e81732_d_n7;
        locals.var_pb0dep__blk1167_dn8 = assign53450_e81732_d_n8;
        locals.var_pb0dep__blk1167_dn9 = assign53450_e81732_d_n9;
        locals.var_pb0dep__blk1167_dn10 = assign53450_e81732_d_n10;
        locals.var_pb0dep__blk1167_dn11 = assign53450_e81732_d_n11;
        locals.var_pb0dep__blk1167_dn14 = assign53450_e81732_d_n14;
        locals.var_pb0dep__blk1167_rv = 0.0;

        let (assign53460_e81748, assign53460_e81748_d_n0, assign53460_e81748_d_n2, assign53460_e81748_d_n4, assign53460_e81748_d_n5, assign53460_e81748_d_n6, assign53460_e81748_d_n7, assign53460_e81748_d_n8, assign53460_e81748_d_n9, assign53460_e81748_d_n10, assign53460_e81748_d_n11, assign53460_e81748_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign53460_e81746: f64 = (locals.var_ps0dep - locals.var_pb0dep__blk1167);
        (assign53460_e81746, (locals.var_ps0dep_dn0 - locals.var_pb0dep__blk1167_dn0), (locals.var_ps0dep_dn2 - locals.var_pb0dep__blk1167_dn2), (locals.var_ps0dep_dn4 - locals.var_pb0dep__blk1167_dn4), (locals.var_ps0dep_dn5 - locals.var_pb0dep__blk1167_dn5), (locals.var_ps0dep_dn6 - locals.var_pb0dep__blk1167_dn6), (locals.var_ps0dep_dn7 - locals.var_pb0dep__blk1167_dn7), (locals.var_ps0dep_dn8 - locals.var_pb0dep__blk1167_dn8), (locals.var_ps0dep_dn9 - locals.var_pb0dep__blk1167_dn9), (locals.var_ps0dep_dn10 - locals.var_pb0dep__blk1167_dn10), (locals.var_ps0dep_dn11 - locals.var_pb0dep__blk1167_dn11), (locals.var_ps0dep_dn14 - locals.var_pb0dep__blk1167_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign53460_e81748;
        locals.var_t2_dn0 = assign53460_e81748_d_n0;
        locals.var_t2_dn2 = assign53460_e81748_d_n2;
        locals.var_t2_dn4 = assign53460_e81748_d_n4;
        locals.var_t2_dn5 = assign53460_e81748_d_n5;
        locals.var_t2_dn6 = assign53460_e81748_d_n6;
        locals.var_t2_dn7 = assign53460_e81748_d_n7;
        locals.var_t2_dn8 = assign53460_e81748_d_n8;
        locals.var_t2_dn9 = assign53460_e81748_d_n9;
        locals.var_t2_dn10 = assign53460_e81748_d_n10;
        locals.var_t2_dn11 = assign53460_e81748_d_n11;
        locals.var_t2_dn14 = assign53460_e81748_d_n14;
        locals.var_t2_rv = 0.0;

        let assign53470_e81751: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1355 = assign53470_e81751;
        locals.var_guard1355_rv = 0.0;

        let (assign53480_e81780, assign53480_e81780_d_n0, assign53480_e81780_d_n2, assign53480_e81780_d_n4, assign53480_e81780_d_n5, assign53480_e81780_d_n6, assign53480_e81780_d_n7, assign53480_e81780_d_n8, assign53480_e81780_d_n9, assign53480_e81780_d_n10, assign53480_e81780_d_n11, assign53480_e81780_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let (assign53480_e81778, assign53480_e81778_d_n0, assign53480_e81778_d_n2, assign53480_e81778_d_n4, assign53480_e81778_d_n5, assign53480_e81778_d_n6, assign53480_e81778_d_n7, assign53480_e81778_d_n8, assign53480_e81778_d_n9, assign53480_e81778_d_n10, assign53480_e81778_d_n11, assign53480_e81778_d_n14,) = {
            if (locals.var_t2 < 0.0) {
                let assign53480_e81769: f64 = (-locals.var_c_2esipq_ndepm__blk1138);
                let assign53480_e81771: f64 = (assign53480_e81769 * locals.var_t2);
                let assign53480_e81772: f64 = (assign53480_e81771).sqrt();
                let assign53480_e81773: f64 = (-assign53480_e81772);
                (assign53480_e81773, (-((((-locals.var_c_2esipq_ndepm__blk1138_dn0) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn0)) / (2.0 * assign53480_e81772))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn2) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn2)) / (2.0 * assign53480_e81772))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn4) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn4)) / (2.0 * assign53480_e81772))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn5) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn5)) / (2.0 * assign53480_e81772))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn6) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn6)) / (2.0 * assign53480_e81772))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn7) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn7)) / (2.0 * assign53480_e81772))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn8) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn8)) / (2.0 * assign53480_e81772))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn9) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn9)) / (2.0 * assign53480_e81772))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn10) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn10)) / (2.0 * assign53480_e81772))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn11) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn11)) / (2.0 * assign53480_e81772))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn14) * locals.var_t2) + (assign53480_e81769 * locals.var_t2_dn14)) / (2.0 * assign53480_e81772))),)
            } else {
                let assign53480_e81776: f64 = (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2);
                let assign53480_e81777: f64 = (assign53480_e81776).sqrt();
                (assign53480_e81777, (((locals.var_c_2esipq_ndepm__blk1138_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn0)) / (2.0 * assign53480_e81777)), (((locals.var_c_2esipq_ndepm__blk1138_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn2)) / (2.0 * assign53480_e81777)), (((locals.var_c_2esipq_ndepm__blk1138_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn4)) / (2.0 * assign53480_e81777)), (((locals.var_c_2esipq_ndepm__blk1138_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn5)) / (2.0 * assign53480_e81777)), (((locals.var_c_2esipq_ndepm__blk1138_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn6)) / (2.0 * assign53480_e81777)), (((locals.var_c_2esipq_ndepm__blk1138_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn7)) / (2.0 * assign53480_e81777)), (((locals.var_c_2esipq_ndepm__blk1138_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn8)) / (2.0 * assign53480_e81777)), (((locals.var_c_2esipq_ndepm__blk1138_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn9)) / (2.0 * assign53480_e81777)), (((locals.var_c_2esipq_ndepm__blk1138_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn10)) / (2.0 * assign53480_e81777)), (((locals.var_c_2esipq_ndepm__blk1138_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn11)) / (2.0 * assign53480_e81777)), (((locals.var_c_2esipq_ndepm__blk1138_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn14)) / (2.0 * assign53480_e81777)),)
            }
        };
        (assign53480_e81778, assign53480_e81778_d_n0, assign53480_e81778_d_n2, assign53480_e81778_d_n4, assign53480_e81778_d_n5, assign53480_e81778_d_n6, assign53480_e81778_d_n7, assign53480_e81778_d_n8, assign53480_e81778_d_n9, assign53480_e81778_d_n10, assign53480_e81778_d_n11, assign53480_e81778_d_n14,)
    } else {
        (locals.var_ws__blk1149, locals.var_ws__blk1149_dn0, locals.var_ws__blk1149_dn2, locals.var_ws__blk1149_dn4, locals.var_ws__blk1149_dn5, locals.var_ws__blk1149_dn6, locals.var_ws__blk1149_dn7, locals.var_ws__blk1149_dn8, locals.var_ws__blk1149_dn9, locals.var_ws__blk1149_dn10, locals.var_ws__blk1149_dn11, locals.var_ws__blk1149_dn14,)
    }
};
        locals.var_ws__blk1149 = assign53480_e81780;
        locals.var_ws__blk1149_dn0 = assign53480_e81780_d_n0;
        locals.var_ws__blk1149_dn2 = assign53480_e81780_d_n2;
        locals.var_ws__blk1149_dn4 = assign53480_e81780_d_n4;
        locals.var_ws__blk1149_dn5 = assign53480_e81780_d_n5;
        locals.var_ws__blk1149_dn6 = assign53480_e81780_d_n6;
        locals.var_ws__blk1149_dn7 = assign53480_e81780_d_n7;
        locals.var_ws__blk1149_dn8 = assign53480_e81780_d_n8;
        locals.var_ws__blk1149_dn9 = assign53480_e81780_d_n9;
        locals.var_ws__blk1149_dn10 = assign53480_e81780_d_n10;
        locals.var_ws__blk1149_dn11 = assign53480_e81780_d_n11;
        locals.var_ws__blk1149_dn14 = assign53480_e81780_d_n14;
        locals.var_ws__blk1149_rv = 0.0;

        let assign53490_e81783: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1356 = assign53490_e81783;
        locals.var_guard1356_rv = 0.0;

        let (assign53500_e81804, assign53500_e81804_d_n0, assign53500_e81804_d_n2, assign53500_e81804_d_n4, assign53500_e81804_d_n5, assign53500_e81804_d_n6, assign53500_e81804_d_n7, assign53500_e81804_d_n8, assign53500_e81804_d_n9, assign53500_e81804_d_n10, assign53500_e81804_d_n11, assign53500_e81804_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1355 == 0.0)) && (locals.var_guard1356 != 0.0)) {
        let assign53500_e81802: f64 = (locals.var_beta * locals.var_t2);
        (assign53500_e81802, ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)), ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)), ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)), ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)), ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)), ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)), ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)), ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)), ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)), ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)), ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign53500_e81804;
        locals.var_t3_dn0 = assign53500_e81804_d_n0;
        locals.var_t3_dn2 = assign53500_e81804_d_n2;
        locals.var_t3_dn4 = assign53500_e81804_d_n4;
        locals.var_t3_dn5 = assign53500_e81804_d_n5;
        locals.var_t3_dn6 = assign53500_e81804_d_n6;
        locals.var_t3_dn7 = assign53500_e81804_d_n7;
        locals.var_t3_dn8 = assign53500_e81804_d_n8;
        locals.var_t3_dn9 = assign53500_e81804_d_n9;
        locals.var_t3_dn10 = assign53500_e81804_d_n10;
        locals.var_t3_dn11 = assign53500_e81804_d_n11;
        locals.var_t3_dn14 = assign53500_e81804_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign53510_e81834, assign53510_e81834_d_n0, assign53510_e81834_d_n2, assign53510_e81834_d_n4, assign53510_e81834_d_n5, assign53510_e81834_d_n6, assign53510_e81834_d_n7, assign53510_e81834_d_n8, assign53510_e81834_d_n9, assign53510_e81834_d_n10, assign53510_e81834_d_n11, assign53510_e81834_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1355 == 0.0)) && (locals.var_guard1356 != 0.0)) {
        let assign53510_e81823: f64 = (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv);
        let assign53510_e81825: f64 = (locals.var_t3).exp();
        let assign53510_e81827: f64 = (assign53510_e81825 - locals.var_t3);
        let assign53510_e81829: f64 = (assign53510_e81827 - 1.0);
        let assign53510_e81830: f64 = (assign53510_e81823 * assign53510_e81829);
        let assign53510_e81831: f64 = (assign53510_e81830).sqrt();
        let assign53510_e81832: f64 = (-assign53510_e81831);
        (assign53510_e81832, (-(((((locals.var_c_2esipq_ndepm__blk1138_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn0)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign53510_e81831))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn2)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign53510_e81831))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn4)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign53510_e81831))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn5)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign53510_e81831))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn6)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign53510_e81831))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn7)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign53510_e81831))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn8)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign53510_e81831))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn9)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign53510_e81831))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn10)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign53510_e81831))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn11 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn11)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn11) - locals.var_t3_dn11))) / (2.0 * assign53510_e81831))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn14 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn14)) * assign53510_e81829) + (assign53510_e81823 * ((assign53510_e81825 * locals.var_t3_dn14) - locals.var_t3_dn14))) / (2.0 * assign53510_e81831))),)
    } else {
        (locals.var_ws__blk1149, locals.var_ws__blk1149_dn0, locals.var_ws__blk1149_dn2, locals.var_ws__blk1149_dn4, locals.var_ws__blk1149_dn5, locals.var_ws__blk1149_dn6, locals.var_ws__blk1149_dn7, locals.var_ws__blk1149_dn8, locals.var_ws__blk1149_dn9, locals.var_ws__blk1149_dn10, locals.var_ws__blk1149_dn11, locals.var_ws__blk1149_dn14,)
    }
};
        locals.var_ws__blk1149 = assign53510_e81834;
        locals.var_ws__blk1149_dn0 = assign53510_e81834_d_n0;
        locals.var_ws__blk1149_dn2 = assign53510_e81834_d_n2;
        locals.var_ws__blk1149_dn4 = assign53510_e81834_d_n4;
        locals.var_ws__blk1149_dn5 = assign53510_e81834_d_n5;
        locals.var_ws__blk1149_dn6 = assign53510_e81834_d_n6;
        locals.var_ws__blk1149_dn7 = assign53510_e81834_d_n7;
        locals.var_ws__blk1149_dn8 = assign53510_e81834_d_n8;
        locals.var_ws__blk1149_dn9 = assign53510_e81834_d_n9;
        locals.var_ws__blk1149_dn10 = assign53510_e81834_d_n10;
        locals.var_ws__blk1149_dn11 = assign53510_e81834_d_n11;
        locals.var_ws__blk1149_dn14 = assign53510_e81834_d_n14;
        locals.var_ws__blk1149_rv = 0.0;

        let (assign53520_e81857, assign53520_e81857_d_n0, assign53520_e81857_d_n2, assign53520_e81857_d_n4, assign53520_e81857_d_n5, assign53520_e81857_d_n6, assign53520_e81857_d_n7, assign53520_e81857_d_n8, assign53520_e81857_d_n9, assign53520_e81857_d_n10, assign53520_e81857_d_n11, assign53520_e81857_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1355 == 0.0)) && (locals.var_guard1356 == 0.0)) {
        let assign53520_e81853: f64 = (-locals.var_beta);
        let assign53520_e81855: f64 = (assign53520_e81853 * locals.var_t2);
        (assign53520_e81855, (((-locals.var_beta_dn0) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn0)), (((-locals.var_beta_dn2) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn2)), (((-locals.var_beta_dn4) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn4)), (((-locals.var_beta_dn5) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn5)), (((-locals.var_beta_dn6) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn6)), (((-locals.var_beta_dn7) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn7)), (((-locals.var_beta_dn8) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn8)), (((-locals.var_beta_dn9) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn9)), (((-locals.var_beta_dn10) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn10)), (((-locals.var_beta_dn11) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn11)), (((-locals.var_beta_dn14) * locals.var_t2) + (assign53520_e81853 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign53520_e81857;
        locals.var_t3_dn0 = assign53520_e81857_d_n0;
        locals.var_t3_dn2 = assign53520_e81857_d_n2;
        locals.var_t3_dn4 = assign53520_e81857_d_n4;
        locals.var_t3_dn5 = assign53520_e81857_d_n5;
        locals.var_t3_dn6 = assign53520_e81857_d_n6;
        locals.var_t3_dn7 = assign53520_e81857_d_n7;
        locals.var_t3_dn8 = assign53520_e81857_d_n8;
        locals.var_t3_dn9 = assign53520_e81857_d_n9;
        locals.var_t3_dn10 = assign53520_e81857_d_n10;
        locals.var_t3_dn11 = assign53520_e81857_d_n11;
        locals.var_t3_dn14 = assign53520_e81857_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign53530_e81887, assign53530_e81887_d_n0, assign53530_e81887_d_n2, assign53530_e81887_d_n4, assign53530_e81887_d_n5, assign53530_e81887_d_n6, assign53530_e81887_d_n7, assign53530_e81887_d_n8, assign53530_e81887_d_n9, assign53530_e81887_d_n10, assign53530_e81887_d_n11, assign53530_e81887_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1355 == 0.0)) && (locals.var_guard1356 == 0.0)) {
        let assign53530_e81877: f64 = (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv);
        let assign53530_e81879: f64 = (locals.var_t3).exp();
        let assign53530_e81881: f64 = (assign53530_e81879 - locals.var_t3);
        let assign53530_e81883: f64 = (assign53530_e81881 - 1.0);
        let assign53530_e81884: f64 = (assign53530_e81877 * assign53530_e81883);
        let assign53530_e81885: f64 = (assign53530_e81884).sqrt();
        (assign53530_e81885, (((((locals.var_c_2esipq_ndepm__blk1138_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn0)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign53530_e81885)), (((((locals.var_c_2esipq_ndepm__blk1138_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn2)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign53530_e81885)), (((((locals.var_c_2esipq_ndepm__blk1138_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn4)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign53530_e81885)), (((((locals.var_c_2esipq_ndepm__blk1138_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn5)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign53530_e81885)), (((((locals.var_c_2esipq_ndepm__blk1138_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn6)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign53530_e81885)), (((((locals.var_c_2esipq_ndepm__blk1138_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn7)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign53530_e81885)), (((((locals.var_c_2esipq_ndepm__blk1138_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn8)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign53530_e81885)), (((((locals.var_c_2esipq_ndepm__blk1138_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn9)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign53530_e81885)), (((((locals.var_c_2esipq_ndepm__blk1138_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn10)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign53530_e81885)), (((((locals.var_c_2esipq_ndepm__blk1138_dn11 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn11)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn11) - locals.var_t3_dn11))) / (2.0 * assign53530_e81885)), (((((locals.var_c_2esipq_ndepm__blk1138_dn14 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn14)) * assign53530_e81883) + (assign53530_e81877 * ((assign53530_e81879 * locals.var_t3_dn14) - locals.var_t3_dn14))) / (2.0 * assign53530_e81885)),)
    } else {
        (locals.var_ws__blk1149, locals.var_ws__blk1149_dn0, locals.var_ws__blk1149_dn2, locals.var_ws__blk1149_dn4, locals.var_ws__blk1149_dn5, locals.var_ws__blk1149_dn6, locals.var_ws__blk1149_dn7, locals.var_ws__blk1149_dn8, locals.var_ws__blk1149_dn9, locals.var_ws__blk1149_dn10, locals.var_ws__blk1149_dn11, locals.var_ws__blk1149_dn14,)
    }
};
        locals.var_ws__blk1149 = assign53530_e81887;
        locals.var_ws__blk1149_dn0 = assign53530_e81887_d_n0;
        locals.var_ws__blk1149_dn2 = assign53530_e81887_d_n2;
        locals.var_ws__blk1149_dn4 = assign53530_e81887_d_n4;
        locals.var_ws__blk1149_dn5 = assign53530_e81887_d_n5;
        locals.var_ws__blk1149_dn6 = assign53530_e81887_d_n6;
        locals.var_ws__blk1149_dn7 = assign53530_e81887_d_n7;
        locals.var_ws__blk1149_dn8 = assign53530_e81887_d_n8;
        locals.var_ws__blk1149_dn9 = assign53530_e81887_d_n9;
        locals.var_ws__blk1149_dn10 = assign53530_e81887_d_n10;
        locals.var_ws__blk1149_dn11 = assign53530_e81887_d_n11;
        locals.var_ws__blk1149_dn14 = assign53530_e81887_d_n14;
        locals.var_ws__blk1149_rv = 0.0;

        let (assign53540_e81903, assign53540_e81903_d_n0, assign53540_e81903_d_n2, assign53540_e81903_d_n4, assign53540_e81903_d_n5, assign53540_e81903_d_n6, assign53540_e81903_d_n7, assign53540_e81903_d_n8, assign53540_e81903_d_n9, assign53540_e81903_d_n10, assign53540_e81903_d_n11, assign53540_e81903_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign53540_e81901: f64 = (locals.var_tnp__blk1150 - locals.var_ws__blk1149);
        (assign53540_e81901, (locals.var_tnp__blk1150_dn0 - locals.var_ws__blk1149_dn0), (locals.var_tnp__blk1150_dn2 - locals.var_ws__blk1149_dn2), (locals.var_tnp__blk1150_dn4 - locals.var_ws__blk1149_dn4), (locals.var_tnp__blk1150_dn5 - locals.var_ws__blk1149_dn5), (locals.var_tnp__blk1150_dn6 - locals.var_ws__blk1149_dn6), (locals.var_tnp__blk1150_dn7 - locals.var_ws__blk1149_dn7), (locals.var_tnp__blk1150_dn8 - locals.var_ws__blk1149_dn8), (locals.var_tnp__blk1150_dn9 - locals.var_ws__blk1149_dn9), (locals.var_tnp__blk1150_dn10 - locals.var_ws__blk1149_dn10), (locals.var_tnp__blk1150_dn11 - locals.var_ws__blk1149_dn11), (locals.var_tnp__blk1150_dn14 - locals.var_ws__blk1149_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign53540_e81903;
        locals.var_w_res_dn0 = assign53540_e81903_d_n0;
        locals.var_w_res_dn2 = assign53540_e81903_d_n2;
        locals.var_w_res_dn4 = assign53540_e81903_d_n4;
        locals.var_w_res_dn5 = assign53540_e81903_d_n5;
        locals.var_w_res_dn6 = assign53540_e81903_d_n6;
        locals.var_w_res_dn7 = assign53540_e81903_d_n7;
        locals.var_w_res_dn8 = assign53540_e81903_d_n8;
        locals.var_w_res_dn9 = assign53540_e81903_d_n9;
        locals.var_w_res_dn10 = assign53540_e81903_d_n10;
        locals.var_w_res_dn11 = assign53540_e81903_d_n11;
        locals.var_w_res_dn14 = assign53540_e81903_d_n14;
        locals.var_w_res_rv = 0.0;

        let assign53550_e81907: f64 = 1e-16;
        let assign53550_e81912: f64 = if ((locals.var_w_res < assign53550_e81907) && (1e-16 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1357 = assign53550_e81912;
        locals.var_guard1357_rv = 0.0;

        let (assign53560_e81932, assign53560_e81932_d_n0, assign53560_e81932_d_n2, assign53560_e81932_d_n4, assign53560_e81932_d_n5, assign53560_e81932_d_n6, assign53560_e81932_d_n7, assign53560_e81932_d_n8, assign53560_e81932_d_n9, assign53560_e81932_d_n10, assign53560_e81932_d_n11, assign53560_e81932_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53560_e81928: f64 = 1e-16;
        let assign53560_e81930: f64 = (assign53560_e81928 - locals.var_w_res);
        (assign53560_e81930, (-locals.var_w_res_dn0), (-locals.var_w_res_dn2), (-locals.var_w_res_dn4), (-locals.var_w_res_dn5), (-locals.var_w_res_dn6), (-locals.var_w_res_dn7), (-locals.var_w_res_dn8), (-locals.var_w_res_dn9), (-locals.var_w_res_dn10), (-locals.var_w_res_dn11), (-locals.var_w_res_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign53560_e81932;
        locals.var_tmf1_dn0 = assign53560_e81932_d_n0;
        locals.var_tmf1_dn2 = assign53560_e81932_d_n2;
        locals.var_tmf1_dn4 = assign53560_e81932_d_n4;
        locals.var_tmf1_dn5 = assign53560_e81932_d_n5;
        locals.var_tmf1_dn6 = assign53560_e81932_d_n6;
        locals.var_tmf1_dn7 = assign53560_e81932_d_n7;
        locals.var_tmf1_dn8 = assign53560_e81932_d_n8;
        locals.var_tmf1_dn9 = assign53560_e81932_d_n9;
        locals.var_tmf1_dn10 = assign53560_e81932_d_n10;
        locals.var_tmf1_dn11 = assign53560_e81932_d_n11;
        locals.var_tmf1_dn14 = assign53560_e81932_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign53570_e81950, assign53570_e81950_d_n0, assign53570_e81950_d_n2, assign53570_e81950_d_n4, assign53570_e81950_d_n5, assign53570_e81950_d_n6, assign53570_e81950_d_n7, assign53570_e81950_d_n8, assign53570_e81950_d_n9, assign53570_e81950_d_n10, assign53570_e81950_d_n11, assign53570_e81950_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53570_e81948: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign53570_e81948, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign53570_e81950;
        locals.var_x2_dn0 = assign53570_e81950_d_n0;
        locals.var_x2_dn2 = assign53570_e81950_d_n2;
        locals.var_x2_dn4 = assign53570_e81950_d_n4;
        locals.var_x2_dn5 = assign53570_e81950_d_n5;
        locals.var_x2_dn6 = assign53570_e81950_d_n6;
        locals.var_x2_dn7 = assign53570_e81950_d_n7;
        locals.var_x2_dn8 = assign53570_e81950_d_n8;
        locals.var_x2_dn9 = assign53570_e81950_d_n9;
        locals.var_x2_dn10 = assign53570_e81950_d_n10;
        locals.var_x2_dn11 = assign53570_e81950_d_n11;
        locals.var_x2_dn14 = assign53570_e81950_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign53580_e81968, assign53580_e81968_d_n0, assign53580_e81968_d_n2, assign53580_e81968_d_n4, assign53580_e81968_d_n5, assign53580_e81968_d_n6, assign53580_e81968_d_n7, assign53580_e81968_d_n8, assign53580_e81968_d_n9, assign53580_e81968_d_n10, assign53580_e81968_d_n11, assign53580_e81968_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53580_e81966: f64 = (1e-16 * 1e-16);
        (assign53580_e81966, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign53580_e81968;
        locals.var_xmax2_dn0 = assign53580_e81968_d_n0;
        locals.var_xmax2_dn2 = assign53580_e81968_d_n2;
        locals.var_xmax2_dn4 = assign53580_e81968_d_n4;
        locals.var_xmax2_dn5 = assign53580_e81968_d_n5;
        locals.var_xmax2_dn6 = assign53580_e81968_d_n6;
        locals.var_xmax2_dn7 = assign53580_e81968_d_n7;
        locals.var_xmax2_dn8 = assign53580_e81968_d_n8;
        locals.var_xmax2_dn9 = assign53580_e81968_d_n9;
        locals.var_xmax2_dn10 = assign53580_e81968_d_n10;
        locals.var_xmax2_dn11 = assign53580_e81968_d_n11;
        locals.var_xmax2_dn14 = assign53580_e81968_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign53590_e81984, assign53590_e81984_d_n0, assign53590_e81984_d_n2, assign53590_e81984_d_n4, assign53590_e81984_d_n5, assign53590_e81984_d_n6, assign53590_e81984_d_n7, assign53590_e81984_d_n8, assign53590_e81984_d_n9, assign53590_e81984_d_n10, assign53590_e81984_d_n11, assign53590_e81984_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign53590_e81984;
        locals.var_xp_dn0 = assign53590_e81984_d_n0;
        locals.var_xp_dn2 = assign53590_e81984_d_n2;
        locals.var_xp_dn4 = assign53590_e81984_d_n4;
        locals.var_xp_dn5 = assign53590_e81984_d_n5;
        locals.var_xp_dn6 = assign53590_e81984_d_n6;
        locals.var_xp_dn7 = assign53590_e81984_d_n7;
        locals.var_xp_dn8 = assign53590_e81984_d_n8;
        locals.var_xp_dn9 = assign53590_e81984_d_n9;
        locals.var_xp_dn10 = assign53590_e81984_d_n10;
        locals.var_xp_dn11 = assign53590_e81984_d_n11;
        locals.var_xp_dn14 = assign53590_e81984_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign53600_e82000, assign53600_e82000_d_n0, assign53600_e82000_d_n2, assign53600_e82000_d_n4, assign53600_e82000_d_n5, assign53600_e82000_d_n6, assign53600_e82000_d_n7, assign53600_e82000_d_n8, assign53600_e82000_d_n9, assign53600_e82000_d_n10, assign53600_e82000_d_n11, assign53600_e82000_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign53600_e82000;
        locals.var_xmp_dn0 = assign53600_e82000_d_n0;
        locals.var_xmp_dn2 = assign53600_e82000_d_n2;
        locals.var_xmp_dn4 = assign53600_e82000_d_n4;
        locals.var_xmp_dn5 = assign53600_e82000_d_n5;
        locals.var_xmp_dn6 = assign53600_e82000_d_n6;
        locals.var_xmp_dn7 = assign53600_e82000_d_n7;
        locals.var_xmp_dn8 = assign53600_e82000_d_n8;
        locals.var_xmp_dn9 = assign53600_e82000_d_n9;
        locals.var_xmp_dn10 = assign53600_e82000_d_n10;
        locals.var_xmp_dn11 = assign53600_e82000_d_n11;
        locals.var_xmp_dn14 = assign53600_e82000_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign53610_e82016,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign53610_e82016;
        locals.var_m0_rv = 0.0;

        let (assign53620_e82032,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53620_e82032;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_194(
        locals: &mut StampLocals,
    ) {
        let (assign53630_e82048, assign53630_e82048_d_n0, assign53630_e82048_d_n2, assign53630_e82048_d_n4, assign53630_e82048_d_n5, assign53630_e82048_d_n6, assign53630_e82048_d_n7, assign53630_e82048_d_n8, assign53630_e82048_d_n9, assign53630_e82048_d_n10, assign53630_e82048_d_n11, assign53630_e82048_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign53630_e82048;
        locals.var_arg_dn0 = assign53630_e82048_d_n0;
        locals.var_arg_dn2 = assign53630_e82048_d_n2;
        locals.var_arg_dn4 = assign53630_e82048_d_n4;
        locals.var_arg_dn5 = assign53630_e82048_d_n5;
        locals.var_arg_dn6 = assign53630_e82048_d_n6;
        locals.var_arg_dn7 = assign53630_e82048_d_n7;
        locals.var_arg_dn8 = assign53630_e82048_d_n8;
        locals.var_arg_dn9 = assign53630_e82048_d_n9;
        locals.var_arg_dn10 = assign53630_e82048_d_n10;
        locals.var_arg_dn11 = assign53630_e82048_d_n11;
        locals.var_arg_dn14 = assign53630_e82048_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign53640_e82064, assign53640_e82064_d_n0, assign53640_e82064_d_n2, assign53640_e82064_d_n4, assign53640_e82064_d_n5, assign53640_e82064_d_n6, assign53640_e82064_d_n7, assign53640_e82064_d_n8, assign53640_e82064_d_n9, assign53640_e82064_d_n10, assign53640_e82064_d_n11, assign53640_e82064_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53640_e82064;
        locals.var_dnm_dn0 = assign53640_e82064_d_n0;
        locals.var_dnm_dn2 = assign53640_e82064_d_n2;
        locals.var_dnm_dn4 = assign53640_e82064_d_n4;
        locals.var_dnm_dn5 = assign53640_e82064_d_n5;
        locals.var_dnm_dn6 = assign53640_e82064_d_n6;
        locals.var_dnm_dn7 = assign53640_e82064_d_n7;
        locals.var_dnm_dn8 = assign53640_e82064_d_n8;
        locals.var_dnm_dn9 = assign53640_e82064_d_n9;
        locals.var_dnm_dn10 = assign53640_e82064_d_n10;
        locals.var_dnm_dn11 = assign53640_e82064_d_n11;
        locals.var_dnm_dn14 = assign53640_e82064_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign53650_e82082, assign53650_e82082_d_n0, assign53650_e82082_d_n2, assign53650_e82082_d_n4, assign53650_e82082_d_n5, assign53650_e82082_d_n6, assign53650_e82082_d_n7, assign53650_e82082_d_n8, assign53650_e82082_d_n9, assign53650_e82082_d_n10, assign53650_e82082_d_n11, assign53650_e82082_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53650_e82080: f64 = (locals.var_xp * locals.var_x2);
        (assign53650_e82080, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign53650_e82082;
        locals.var_xp_dn0 = assign53650_e82082_d_n0;
        locals.var_xp_dn2 = assign53650_e82082_d_n2;
        locals.var_xp_dn4 = assign53650_e82082_d_n4;
        locals.var_xp_dn5 = assign53650_e82082_d_n5;
        locals.var_xp_dn6 = assign53650_e82082_d_n6;
        locals.var_xp_dn7 = assign53650_e82082_d_n7;
        locals.var_xp_dn8 = assign53650_e82082_d_n8;
        locals.var_xp_dn9 = assign53650_e82082_d_n9;
        locals.var_xp_dn10 = assign53650_e82082_d_n10;
        locals.var_xp_dn11 = assign53650_e82082_d_n11;
        locals.var_xp_dn14 = assign53650_e82082_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign53660_e82100, assign53660_e82100_d_n0, assign53660_e82100_d_n2, assign53660_e82100_d_n4, assign53660_e82100_d_n5, assign53660_e82100_d_n6, assign53660_e82100_d_n7, assign53660_e82100_d_n8, assign53660_e82100_d_n9, assign53660_e82100_d_n10, assign53660_e82100_d_n11, assign53660_e82100_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53660_e82098: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign53660_e82098, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign53660_e82100;
        locals.var_xmp_dn0 = assign53660_e82100_d_n0;
        locals.var_xmp_dn2 = assign53660_e82100_d_n2;
        locals.var_xmp_dn4 = assign53660_e82100_d_n4;
        locals.var_xmp_dn5 = assign53660_e82100_d_n5;
        locals.var_xmp_dn6 = assign53660_e82100_d_n6;
        locals.var_xmp_dn7 = assign53660_e82100_d_n7;
        locals.var_xmp_dn8 = assign53660_e82100_d_n8;
        locals.var_xmp_dn9 = assign53660_e82100_d_n9;
        locals.var_xmp_dn10 = assign53660_e82100_d_n10;
        locals.var_xmp_dn11 = assign53660_e82100_d_n11;
        locals.var_xmp_dn14 = assign53660_e82100_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign53670_e82118, assign53670_e82118_d_n0, assign53670_e82118_d_n2, assign53670_e82118_d_n4, assign53670_e82118_d_n5, assign53670_e82118_d_n6, assign53670_e82118_d_n7, assign53670_e82118_d_n8, assign53670_e82118_d_n9, assign53670_e82118_d_n10, assign53670_e82118_d_n11, assign53670_e82118_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53670_e82116: f64 = (locals.var_xp * locals.var_x2);
        (assign53670_e82116, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign53670_e82118;
        locals.var_xp_dn0 = assign53670_e82118_d_n0;
        locals.var_xp_dn2 = assign53670_e82118_d_n2;
        locals.var_xp_dn4 = assign53670_e82118_d_n4;
        locals.var_xp_dn5 = assign53670_e82118_d_n5;
        locals.var_xp_dn6 = assign53670_e82118_d_n6;
        locals.var_xp_dn7 = assign53670_e82118_d_n7;
        locals.var_xp_dn8 = assign53670_e82118_d_n8;
        locals.var_xp_dn9 = assign53670_e82118_d_n9;
        locals.var_xp_dn10 = assign53670_e82118_d_n10;
        locals.var_xp_dn11 = assign53670_e82118_d_n11;
        locals.var_xp_dn14 = assign53670_e82118_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign53680_e82136, assign53680_e82136_d_n0, assign53680_e82136_d_n2, assign53680_e82136_d_n4, assign53680_e82136_d_n5, assign53680_e82136_d_n6, assign53680_e82136_d_n7, assign53680_e82136_d_n8, assign53680_e82136_d_n9, assign53680_e82136_d_n10, assign53680_e82136_d_n11, assign53680_e82136_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53680_e82134: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign53680_e82134, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign53680_e82136;
        locals.var_xmp_dn0 = assign53680_e82136_d_n0;
        locals.var_xmp_dn2 = assign53680_e82136_d_n2;
        locals.var_xmp_dn4 = assign53680_e82136_d_n4;
        locals.var_xmp_dn5 = assign53680_e82136_d_n5;
        locals.var_xmp_dn6 = assign53680_e82136_d_n6;
        locals.var_xmp_dn7 = assign53680_e82136_d_n7;
        locals.var_xmp_dn8 = assign53680_e82136_d_n8;
        locals.var_xmp_dn9 = assign53680_e82136_d_n9;
        locals.var_xmp_dn10 = assign53680_e82136_d_n10;
        locals.var_xmp_dn11 = assign53680_e82136_d_n11;
        locals.var_xmp_dn14 = assign53680_e82136_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign53690_e82154, assign53690_e82154_d_n0, assign53690_e82154_d_n2, assign53690_e82154_d_n4, assign53690_e82154_d_n5, assign53690_e82154_d_n6, assign53690_e82154_d_n7, assign53690_e82154_d_n8, assign53690_e82154_d_n9, assign53690_e82154_d_n10, assign53690_e82154_d_n11, assign53690_e82154_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53690_e82152: f64 = (locals.var_xp + locals.var_xmp);
        (assign53690_e82152, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign53690_e82154;
        locals.var_arg_dn0 = assign53690_e82154_d_n0;
        locals.var_arg_dn2 = assign53690_e82154_d_n2;
        locals.var_arg_dn4 = assign53690_e82154_d_n4;
        locals.var_arg_dn5 = assign53690_e82154_d_n5;
        locals.var_arg_dn6 = assign53690_e82154_d_n6;
        locals.var_arg_dn7 = assign53690_e82154_d_n7;
        locals.var_arg_dn8 = assign53690_e82154_d_n8;
        locals.var_arg_dn9 = assign53690_e82154_d_n9;
        locals.var_arg_dn10 = assign53690_e82154_d_n10;
        locals.var_arg_dn11 = assign53690_e82154_d_n11;
        locals.var_arg_dn14 = assign53690_e82154_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign53700_e82170, assign53700_e82170_d_n0, assign53700_e82170_d_n2, assign53700_e82170_d_n4, assign53700_e82170_d_n5, assign53700_e82170_d_n6, assign53700_e82170_d_n7, assign53700_e82170_d_n8, assign53700_e82170_d_n9, assign53700_e82170_d_n10, assign53700_e82170_d_n11, assign53700_e82170_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53700_e82170;
        locals.var_dnm_dn0 = assign53700_e82170_d_n0;
        locals.var_dnm_dn2 = assign53700_e82170_d_n2;
        locals.var_dnm_dn4 = assign53700_e82170_d_n4;
        locals.var_dnm_dn5 = assign53700_e82170_d_n5;
        locals.var_dnm_dn6 = assign53700_e82170_d_n6;
        locals.var_dnm_dn7 = assign53700_e82170_d_n7;
        locals.var_dnm_dn8 = assign53700_e82170_d_n8;
        locals.var_dnm_dn9 = assign53700_e82170_d_n9;
        locals.var_dnm_dn10 = assign53700_e82170_d_n10;
        locals.var_dnm_dn11 = assign53700_e82170_d_n11;
        locals.var_dnm_dn14 = assign53700_e82170_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign53710_e82185: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1358 = assign53710_e82185;
        locals.var_guard1358_rv = 0.0;

        let assign53720_e82188: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1359 = assign53720_e82188;
        locals.var_guard1359_rv = 0.0;

        let (assign53730_e82208,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 != 0.0)) && (locals.var_guard1359 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53730_e82208;
        locals.var_mm_rv = 0.0;

        let assign53740_e82211: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1360 = assign53740_e82211;
        locals.var_guard1360_rv = 0.0;

        let (assign53750_e82234,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 != 0.0)) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1360 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53750_e82234;
        locals.var_mm_rv = 0.0;

        let assign53760_e82237: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1361 = assign53760_e82237;
        locals.var_guard1361_rv = 0.0;

        let (assign53770_e82263,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 != 0.0)) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1360 == 0.0)) && (locals.var_guard1361 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53770_e82263;
        locals.var_mm_rv = 0.0;

        let assign53780_e82266: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1362 = assign53780_e82266;
        locals.var_guard1362_rv = 0.0;

        let (assign53790_e82295,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 != 0.0)) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1360 == 0.0)) && (locals.var_guard1361 == 0.0)) && (locals.var_guard1362 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53790_e82295;
        locals.var_mm_rv = 0.0;

        let (assign53800_e82313,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign53800_e82313;
        locals.var_m0_rv = 0.0;

        let mut assign53810_loop_guard: usize = 0;
        while {
            let assign53810_cond_e82332: f64 = if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign53810_cond_e82332 != 0.0
        } {
            assign53810_loop_guard += 1;
            assert!(assign53810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign53810_body0_e82351, assign53810_body0_e82351_d_n0, assign53810_body0_e82351_d_n2, assign53810_body0_e82351_d_n4, assign53810_body0_e82351_d_n5, assign53810_body0_e82351_d_n6, assign53810_body0_e82351_d_n7, assign53810_body0_e82351_d_n8, assign53810_body0_e82351_d_n9, assign53810_body0_e82351_d_n10, assign53810_body0_e82351_d_n11, assign53810_body0_e82351_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 != 0.0)) {
        let assign53810_body0_e82349: f64 = (locals.var_dnm).sqrt();
        (assign53810_body0_e82349, (locals.var_dnm_dn0 / (2.0 * assign53810_body0_e82349)), (locals.var_dnm_dn2 / (2.0 * assign53810_body0_e82349)), (locals.var_dnm_dn4 / (2.0 * assign53810_body0_e82349)), (locals.var_dnm_dn5 / (2.0 * assign53810_body0_e82349)), (locals.var_dnm_dn6 / (2.0 * assign53810_body0_e82349)), (locals.var_dnm_dn7 / (2.0 * assign53810_body0_e82349)), (locals.var_dnm_dn8 / (2.0 * assign53810_body0_e82349)), (locals.var_dnm_dn9 / (2.0 * assign53810_body0_e82349)), (locals.var_dnm_dn10 / (2.0 * assign53810_body0_e82349)), (locals.var_dnm_dn11 / (2.0 * assign53810_body0_e82349)), (locals.var_dnm_dn14 / (2.0 * assign53810_body0_e82349)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign53810_body0_e82351;
            locals.var_dnm_dn0 = assign53810_body0_e82351_d_n0;
            locals.var_dnm_dn2 = assign53810_body0_e82351_d_n2;
            locals.var_dnm_dn4 = assign53810_body0_e82351_d_n4;
            locals.var_dnm_dn5 = assign53810_body0_e82351_d_n5;
            locals.var_dnm_dn6 = assign53810_body0_e82351_d_n6;
            locals.var_dnm_dn7 = assign53810_body0_e82351_d_n7;
            locals.var_dnm_dn8 = assign53810_body0_e82351_d_n8;
            locals.var_dnm_dn9 = assign53810_body0_e82351_d_n9;
            locals.var_dnm_dn10 = assign53810_body0_e82351_d_n10;
            locals.var_dnm_dn11 = assign53810_body0_e82351_d_n11;
            locals.var_dnm_dn14 = assign53810_body0_e82351_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign53810_body1_e82371,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 != 0.0)) {
        let assign53810_body1_e82369: f64 = (locals.var_m0 + 1.0);
        (assign53810_body1_e82369,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign53810_body1_e82371;
            locals.var_m0_rv = 0.0;
        }

        let (assign53820_e82401, assign53820_e82401_d_n0, assign53820_e82401_d_n2, assign53820_e82401_d_n4, assign53820_e82401_d_n5, assign53820_e82401_d_n6, assign53820_e82401_d_n7, assign53820_e82401_d_n8, assign53820_e82401_d_n9, assign53820_e82401_d_n10, assign53820_e82401_d_n11, assign53820_e82401_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 == 0.0)) {
        let (assign53820_e82399, assign53820_e82399_d_n0, assign53820_e82399_d_n2, assign53820_e82399_d_n4, assign53820_e82399_d_n5, assign53820_e82399_d_n6, assign53820_e82399_d_n7, assign53820_e82399_d_n8, assign53820_e82399_d_n9, assign53820_e82399_d_n10, assign53820_e82399_d_n11, assign53820_e82399_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign53820_e82396: f64 = (2.0 * 2.0);
                let assign53820_e82397: f64 = (1.0 / assign53820_e82396);
                let assign53820_e82398: f64 = (locals.var_dnm).powf(assign53820_e82397);
                (assign53820_e82398, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn0)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn2)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn4)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn5)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn6)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn7)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn8)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn9)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn10)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn11)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53820_e82397) as f64).is_finite() && ((assign53820_e82397) as f64).fract() == 0.0 { if assign53820_e82397 == 0.0 { 0.0 } else { (assign53820_e82397 * ((locals.var_dnm).powf(assign53820_e82397 - 1.0) * locals.var_dnm_dn14)) } } else { (assign53820_e82398 * (assign53820_e82397 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign53820_e82399, assign53820_e82399_d_n0, assign53820_e82399_d_n2, assign53820_e82399_d_n4, assign53820_e82399_d_n5, assign53820_e82399_d_n6, assign53820_e82399_d_n7, assign53820_e82399_d_n8, assign53820_e82399_d_n9, assign53820_e82399_d_n10, assign53820_e82399_d_n11, assign53820_e82399_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53820_e82401;
        locals.var_dnm_dn0 = assign53820_e82401_d_n0;
        locals.var_dnm_dn2 = assign53820_e82401_d_n2;
        locals.var_dnm_dn4 = assign53820_e82401_d_n4;
        locals.var_dnm_dn5 = assign53820_e82401_d_n5;
        locals.var_dnm_dn6 = assign53820_e82401_d_n6;
        locals.var_dnm_dn7 = assign53820_e82401_d_n7;
        locals.var_dnm_dn8 = assign53820_e82401_d_n8;
        locals.var_dnm_dn9 = assign53820_e82401_d_n9;
        locals.var_dnm_dn10 = assign53820_e82401_d_n10;
        locals.var_dnm_dn11 = assign53820_e82401_d_n11;
        locals.var_dnm_dn14 = assign53820_e82401_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign53830_e82419, assign53830_e82419_d_n0, assign53830_e82419_d_n2, assign53830_e82419_d_n4, assign53830_e82419_d_n5, assign53830_e82419_d_n6, assign53830_e82419_d_n7, assign53830_e82419_d_n8, assign53830_e82419_d_n9, assign53830_e82419_d_n10, assign53830_e82419_d_n11, assign53830_e82419_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53830_e82417: f64 = (1.0 / locals.var_dnm);
        (assign53830_e82417, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53830_e82419;
        locals.var_dnm_dn0 = assign53830_e82419_d_n0;
        locals.var_dnm_dn2 = assign53830_e82419_d_n2;
        locals.var_dnm_dn4 = assign53830_e82419_d_n4;
        locals.var_dnm_dn5 = assign53830_e82419_d_n5;
        locals.var_dnm_dn6 = assign53830_e82419_d_n6;
        locals.var_dnm_dn7 = assign53830_e82419_d_n7;
        locals.var_dnm_dn8 = assign53830_e82419_d_n8;
        locals.var_dnm_dn9 = assign53830_e82419_d_n9;
        locals.var_dnm_dn10 = assign53830_e82419_d_n10;
        locals.var_dnm_dn11 = assign53830_e82419_d_n11;
        locals.var_dnm_dn14 = assign53830_e82419_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign53840_e82439, assign53840_e82439_d_n0, assign53840_e82439_d_n2, assign53840_e82439_d_n4, assign53840_e82439_d_n5, assign53840_e82439_d_n6, assign53840_e82439_d_n7, assign53840_e82439_d_n8, assign53840_e82439_d_n9, assign53840_e82439_d_n10, assign53840_e82439_d_n11, assign53840_e82439_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53840_e82435: f64 = (locals.var_tmf1 * 1e-16);
        let assign53840_e82437: f64 = (assign53840_e82435 * locals.var_dnm);
        (assign53840_e82437, (((locals.var_tmf1_dn0 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-16) * locals.var_dnm) + (assign53840_e82435 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign53840_e82439;
        locals.var_tmf0_dn0 = assign53840_e82439_d_n0;
        locals.var_tmf0_dn2 = assign53840_e82439_d_n2;
        locals.var_tmf0_dn4 = assign53840_e82439_d_n4;
        locals.var_tmf0_dn5 = assign53840_e82439_d_n5;
        locals.var_tmf0_dn6 = assign53840_e82439_d_n6;
        locals.var_tmf0_dn7 = assign53840_e82439_d_n7;
        locals.var_tmf0_dn8 = assign53840_e82439_d_n8;
        locals.var_tmf0_dn9 = assign53840_e82439_d_n9;
        locals.var_tmf0_dn10 = assign53840_e82439_d_n10;
        locals.var_tmf0_dn11 = assign53840_e82439_d_n11;
        locals.var_tmf0_dn14 = assign53840_e82439_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign53850_e82461, assign53850_e82461_d_n0, assign53850_e82461_d_n2, assign53850_e82461_d_n4, assign53850_e82461_d_n5, assign53850_e82461_d_n6, assign53850_e82461_d_n7, assign53850_e82461_d_n8, assign53850_e82461_d_n9, assign53850_e82461_d_n10, assign53850_e82461_d_n11, assign53850_e82461_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53850_e82455: f64 = (1e-16 * locals.var_xmp);
        let assign53850_e82457: f64 = (assign53850_e82455 * locals.var_dnm);
        let assign53850_e82459: f64 = (assign53850_e82457 / locals.var_arg);
        (assign53850_e82459, ((((((1e-16 * locals.var_xmp_dn0) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn0)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn2) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn2)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn4) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn4)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn5) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn5)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn6) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn6)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn7) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn7)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn8) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn8)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn9) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn9)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn10) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn10)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn11) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn11)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn14) * locals.var_dnm) + (assign53850_e82455 * locals.var_dnm_dn14)) * locals.var_arg) - (assign53850_e82457 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53850_e82461;
        locals.var_t0_dn0 = assign53850_e82461_d_n0;
        locals.var_t0_dn2 = assign53850_e82461_d_n2;
        locals.var_t0_dn4 = assign53850_e82461_d_n4;
        locals.var_t0_dn5 = assign53850_e82461_d_n5;
        locals.var_t0_dn6 = assign53850_e82461_d_n6;
        locals.var_t0_dn7 = assign53850_e82461_d_n7;
        locals.var_t0_dn8 = assign53850_e82461_d_n8;
        locals.var_t0_dn9 = assign53850_e82461_d_n9;
        locals.var_t0_dn10 = assign53850_e82461_d_n10;
        locals.var_t0_dn11 = assign53850_e82461_d_n11;
        locals.var_t0_dn14 = assign53850_e82461_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign53860_e82481, assign53860_e82481_d_n0, assign53860_e82481_d_n2, assign53860_e82481_d_n4, assign53860_e82481_d_n5, assign53860_e82481_d_n6, assign53860_e82481_d_n7, assign53860_e82481_d_n8, assign53860_e82481_d_n9, assign53860_e82481_d_n10, assign53860_e82481_d_n11, assign53860_e82481_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let assign53860_e82477: f64 = 1e-16;
        let assign53860_e82479: f64 = (assign53860_e82477 - locals.var_tmf0);
        (assign53860_e82479, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign53860_e82481;
        locals.var_w_res_dn0 = assign53860_e82481_d_n0;
        locals.var_w_res_dn2 = assign53860_e82481_d_n2;
        locals.var_w_res_dn4 = assign53860_e82481_d_n4;
        locals.var_w_res_dn5 = assign53860_e82481_d_n5;
        locals.var_w_res_dn6 = assign53860_e82481_d_n6;
        locals.var_w_res_dn7 = assign53860_e82481_d_n7;
        locals.var_w_res_dn8 = assign53860_e82481_d_n8;
        locals.var_w_res_dn9 = assign53860_e82481_d_n9;
        locals.var_w_res_dn10 = assign53860_e82481_d_n10;
        locals.var_w_res_dn11 = assign53860_e82481_d_n11;
        locals.var_w_res_dn14 = assign53860_e82481_d_n14;
        locals.var_w_res_rv = 0.0;

        let (assign53870_e82497, assign53870_e82497_d_n0, assign53870_e82497_d_n2, assign53870_e82497_d_n4, assign53870_e82497_d_n5, assign53870_e82497_d_n6, assign53870_e82497_d_n7, assign53870_e82497_d_n8, assign53870_e82497_d_n9, assign53870_e82497_d_n10, assign53870_e82497_d_n11, assign53870_e82497_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53870_e82497;
        locals.var_t0_dn0 = assign53870_e82497_d_n0;
        locals.var_t0_dn2 = assign53870_e82497_d_n2;
        locals.var_t0_dn4 = assign53870_e82497_d_n4;
        locals.var_t0_dn5 = assign53870_e82497_d_n5;
        locals.var_t0_dn6 = assign53870_e82497_d_n6;
        locals.var_t0_dn7 = assign53870_e82497_d_n7;
        locals.var_t0_dn8 = assign53870_e82497_d_n8;
        locals.var_t0_dn9 = assign53870_e82497_d_n9;
        locals.var_t0_dn10 = assign53870_e82497_d_n10;
        locals.var_t0_dn11 = assign53870_e82497_d_n11;
        locals.var_t0_dn14 = assign53870_e82497_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign53880_e82514, assign53880_e82514_d_n0, assign53880_e82514_d_n2, assign53880_e82514_d_n4, assign53880_e82514_d_n5, assign53880_e82514_d_n6, assign53880_e82514_d_n7, assign53880_e82514_d_n8, assign53880_e82514_d_n9, assign53880_e82514_d_n10, assign53880_e82514_d_n11, assign53880_e82514_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 == 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign53880_e82514;
        locals.var_w_res_dn0 = assign53880_e82514_d_n0;
        locals.var_w_res_dn2 = assign53880_e82514_d_n2;
        locals.var_w_res_dn4 = assign53880_e82514_d_n4;
        locals.var_w_res_dn5 = assign53880_e82514_d_n5;
        locals.var_w_res_dn6 = assign53880_e82514_d_n6;
        locals.var_w_res_dn7 = assign53880_e82514_d_n7;
        locals.var_w_res_dn8 = assign53880_e82514_d_n8;
        locals.var_w_res_dn9 = assign53880_e82514_d_n9;
        locals.var_w_res_dn10 = assign53880_e82514_d_n10;
        locals.var_w_res_dn11 = assign53880_e82514_d_n11;
        locals.var_w_res_dn14 = assign53880_e82514_d_n14;
        locals.var_w_res_rv = 0.0;

        let (assign53890_e82531, assign53890_e82531_d_n0, assign53890_e82531_d_n2, assign53890_e82531_d_n4, assign53890_e82531_d_n5, assign53890_e82531_d_n6, assign53890_e82531_d_n7, assign53890_e82531_d_n8, assign53890_e82531_d_n9, assign53890_e82531_d_n10, assign53890_e82531_d_n11, assign53890_e82531_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1357 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53890_e82531;
        locals.var_t0_dn0 = assign53890_e82531_d_n0;
        locals.var_t0_dn2 = assign53890_e82531_d_n2;
        locals.var_t0_dn4 = assign53890_e82531_d_n4;
        locals.var_t0_dn5 = assign53890_e82531_d_n5;
        locals.var_t0_dn6 = assign53890_e82531_d_n6;
        locals.var_t0_dn7 = assign53890_e82531_d_n7;
        locals.var_t0_dn8 = assign53890_e82531_d_n8;
        locals.var_t0_dn9 = assign53890_e82531_d_n9;
        locals.var_t0_dn10 = assign53890_e82531_d_n10;
        locals.var_t0_dn11 = assign53890_e82531_d_n11;
        locals.var_t0_dn14 = assign53890_e82531_d_n14;
        locals.var_t0_rv = 0.0;

        let assign53900_e82534: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1363 = assign53900_e82534;
        locals.var_guard1363_rv = 0.0;

        let (assign53910_e82550, assign53910_e82550_d_n0, assign53910_e82550_d_n2, assign53910_e82550_d_n4, assign53910_e82550_d_n5, assign53910_e82550_d_n6, assign53910_e82550_d_n7, assign53910_e82550_d_n8, assign53910_e82550_d_n9, assign53910_e82550_d_n10, assign53910_e82550_d_n11, assign53910_e82550_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1363 != 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    } else {
        (locals.var_w_res_leak, locals.var_w_res_leak_dn0, locals.var_w_res_leak_dn2, locals.var_w_res_leak_dn4, locals.var_w_res_leak_dn5, locals.var_w_res_leak_dn6, locals.var_w_res_leak_dn7, locals.var_w_res_leak_dn8, locals.var_w_res_leak_dn9, locals.var_w_res_leak_dn10, locals.var_w_res_leak_dn11, locals.var_w_res_leak_dn14,)
    }
};
        locals.var_w_res_leak = assign53910_e82550;
        locals.var_w_res_leak_dn0 = assign53910_e82550_d_n0;
        locals.var_w_res_leak_dn2 = assign53910_e82550_d_n2;
        locals.var_w_res_leak_dn4 = assign53910_e82550_d_n4;
        locals.var_w_res_leak_dn5 = assign53910_e82550_d_n5;
        locals.var_w_res_leak_dn6 = assign53910_e82550_d_n6;
        locals.var_w_res_leak_dn7 = assign53910_e82550_d_n7;
        locals.var_w_res_leak_dn8 = assign53910_e82550_d_n8;
        locals.var_w_res_leak_dn9 = assign53910_e82550_d_n9;
        locals.var_w_res_leak_dn10 = assign53910_e82550_d_n10;
        locals.var_w_res_leak_dn11 = assign53910_e82550_d_n11;
        locals.var_w_res_leak_dn14 = assign53910_e82550_d_n14;
        locals.var_w_res_leak_rv = 0.0;

        let assign53920_e82553: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1364 = assign53920_e82553;
        locals.var_guard1364_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_195(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign53930_e82571, assign53930_e82571_d_n0, assign53930_e82571_d_n2, assign53930_e82571_d_n4, assign53930_e82571_d_n5, assign53930_e82571_d_n6, assign53930_e82571_d_n7, assign53930_e82571_d_n8, assign53930_e82571_d_n9, assign53930_e82571_d_n10, assign53930_e82571_d_n11, assign53930_e82571_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1364 != 0.0)) {
        let assign53930_e82569: f64 = (p.p399 * locals.var_vbsc__blk1119);
        (assign53930_e82569, (p.p399 * locals.var_vbsc__blk1119_dn0), (p.p399 * locals.var_vbsc__blk1119_dn2), (p.p399 * locals.var_vbsc__blk1119_dn4), (p.p399 * locals.var_vbsc__blk1119_dn5), (p.p399 * locals.var_vbsc__blk1119_dn6), (p.p399 * locals.var_vbsc__blk1119_dn7), (p.p399 * locals.var_vbsc__blk1119_dn8), (p.p399 * locals.var_vbsc__blk1119_dn9), (p.p399 * locals.var_vbsc__blk1119_dn10), (p.p399 * locals.var_vbsc__blk1119_dn11), (p.p399 * locals.var_vbsc__blk1119_dn14),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    }
};
        locals.var_depvbs = assign53930_e82571;
        locals.var_depvbs_dn0 = assign53930_e82571_d_n0;
        locals.var_depvbs_dn2 = assign53930_e82571_d_n2;
        locals.var_depvbs_dn4 = assign53930_e82571_d_n4;
        locals.var_depvbs_dn5 = assign53930_e82571_d_n5;
        locals.var_depvbs_dn6 = assign53930_e82571_d_n6;
        locals.var_depvbs_dn7 = assign53930_e82571_d_n7;
        locals.var_depvbs_dn8 = assign53930_e82571_d_n8;
        locals.var_depvbs_dn9 = assign53930_e82571_d_n9;
        locals.var_depvbs_dn10 = assign53930_e82571_d_n10;
        locals.var_depvbs_dn11 = assign53930_e82571_d_n11;
        locals.var_depvbs_dn14 = assign53930_e82571_d_n14;
        locals.var_depvbs_rv = 0.0;

        let (assign53940_e82589, assign53940_e82589_d_n0, assign53940_e82589_d_n2, assign53940_e82589_d_n4, assign53940_e82589_d_n5, assign53940_e82589_d_n6, assign53940_e82589_d_n7, assign53940_e82589_d_n8, assign53940_e82589_d_n9, assign53940_e82589_d_n10, assign53940_e82589_d_n11, assign53940_e82589_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1364 != 0.0)) {
        let assign53940_e82587: f64 = (locals.var_depvbs - 1.0);
        (assign53940_e82587, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign53940_e82589;
        locals.var_ps0dep_dn0 = assign53940_e82589_d_n0;
        locals.var_ps0dep_dn2 = assign53940_e82589_d_n2;
        locals.var_ps0dep_dn4 = assign53940_e82589_d_n4;
        locals.var_ps0dep_dn5 = assign53940_e82589_d_n5;
        locals.var_ps0dep_dn6 = assign53940_e82589_d_n6;
        locals.var_ps0dep_dn7 = assign53940_e82589_d_n7;
        locals.var_ps0dep_dn8 = assign53940_e82589_d_n8;
        locals.var_ps0dep_dn9 = assign53940_e82589_d_n9;
        locals.var_ps0dep_dn10 = assign53940_e82589_d_n10;
        locals.var_ps0dep_dn11 = assign53940_e82589_d_n11;
        locals.var_ps0dep_dn14 = assign53940_e82589_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign53950_e82605, assign53950_e82605_d_n0, assign53950_e82605_d_n2, assign53950_e82605_d_n4, assign53950_e82605_d_n5, assign53950_e82605_d_n6, assign53950_e82605_d_n7, assign53950_e82605_d_n8, assign53950_e82605_d_n9, assign53950_e82605_d_n10, assign53950_e82605_d_n11, assign53950_e82605_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1364 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn11, locals.var_vgp_ws_dn14,)
    }
};
        locals.var_vgp_ws = assign53950_e82605;
        locals.var_vgp_ws_dn0 = assign53950_e82605_d_n0;
        locals.var_vgp_ws_dn2 = assign53950_e82605_d_n2;
        locals.var_vgp_ws_dn4 = assign53950_e82605_d_n4;
        locals.var_vgp_ws_dn5 = assign53950_e82605_d_n5;
        locals.var_vgp_ws_dn6 = assign53950_e82605_d_n6;
        locals.var_vgp_ws_dn7 = assign53950_e82605_d_n7;
        locals.var_vgp_ws_dn8 = assign53950_e82605_d_n8;
        locals.var_vgp_ws_dn9 = assign53950_e82605_d_n9;
        locals.var_vgp_ws_dn10 = assign53950_e82605_d_n10;
        locals.var_vgp_ws_dn11 = assign53950_e82605_d_n11;
        locals.var_vgp_ws_dn14 = assign53950_e82605_d_n14;
        locals.var_vgp_ws_rv = 0.0;

        let (assign53960_e82621, assign53960_e82621_d_n0, assign53960_e82621_d_n2, assign53960_e82621_d_n4, assign53960_e82621_d_n5, assign53960_e82621_d_n6, assign53960_e82621_d_n7, assign53960_e82621_d_n8, assign53960_e82621_d_n9, assign53960_e82621_d_n10, assign53960_e82621_d_n11, assign53960_e82621_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1364 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn11, locals.var_vgp_res_raw_dn14,)
    }
};
        locals.var_vgp_res_raw = assign53960_e82621;
        locals.var_vgp_res_raw_dn0 = assign53960_e82621_d_n0;
        locals.var_vgp_res_raw_dn2 = assign53960_e82621_d_n2;
        locals.var_vgp_res_raw_dn4 = assign53960_e82621_d_n4;
        locals.var_vgp_res_raw_dn5 = assign53960_e82621_d_n5;
        locals.var_vgp_res_raw_dn6 = assign53960_e82621_d_n6;
        locals.var_vgp_res_raw_dn7 = assign53960_e82621_d_n7;
        locals.var_vgp_res_raw_dn8 = assign53960_e82621_d_n8;
        locals.var_vgp_res_raw_dn9 = assign53960_e82621_d_n9;
        locals.var_vgp_res_raw_dn10 = assign53960_e82621_d_n10;
        locals.var_vgp_res_raw_dn11 = assign53960_e82621_d_n11;
        locals.var_vgp_res_raw_dn14 = assign53960_e82621_d_n14;
        locals.var_vgp_res_raw_rv = 0.0;

        let (assign53970_e82642, assign53970_e82642_d_n0, assign53970_e82642_d_n2, assign53970_e82642_d_n4, assign53970_e82642_d_n5, assign53970_e82642_d_n6, assign53970_e82642_d_n7, assign53970_e82642_d_n8, assign53970_e82642_d_n9, assign53970_e82642_d_n10, assign53970_e82642_d_n11, assign53970_e82642_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1364 == 0.0)) {
        let assign53970_e82638: f64 = (p.p399 * locals.var_vbsc__blk1119);
        let assign53970_e82640: f64 = (assign53970_e82638 - 0.1);
        (assign53970_e82640, (p.p399 * locals.var_vbsc__blk1119_dn0), (p.p399 * locals.var_vbsc__blk1119_dn2), (p.p399 * locals.var_vbsc__blk1119_dn4), (p.p399 * locals.var_vbsc__blk1119_dn5), (p.p399 * locals.var_vbsc__blk1119_dn6), (p.p399 * locals.var_vbsc__blk1119_dn7), (p.p399 * locals.var_vbsc__blk1119_dn8), (p.p399 * locals.var_vbsc__blk1119_dn9), (p.p399 * locals.var_vbsc__blk1119_dn10), (p.p399 * locals.var_vbsc__blk1119_dn11), (p.p399 * locals.var_vbsc__blk1119_dn14),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    }
};
        locals.var_depvbs = assign53970_e82642;
        locals.var_depvbs_dn0 = assign53970_e82642_d_n0;
        locals.var_depvbs_dn2 = assign53970_e82642_d_n2;
        locals.var_depvbs_dn4 = assign53970_e82642_d_n4;
        locals.var_depvbs_dn5 = assign53970_e82642_d_n5;
        locals.var_depvbs_dn6 = assign53970_e82642_d_n6;
        locals.var_depvbs_dn7 = assign53970_e82642_d_n7;
        locals.var_depvbs_dn8 = assign53970_e82642_d_n8;
        locals.var_depvbs_dn9 = assign53970_e82642_d_n9;
        locals.var_depvbs_dn10 = assign53970_e82642_d_n10;
        locals.var_depvbs_dn11 = assign53970_e82642_d_n11;
        locals.var_depvbs_dn14 = assign53970_e82642_d_n14;
        locals.var_depvbs_rv = 0.0;

        let (assign53980_e82659, assign53980_e82659_d_n0, assign53980_e82659_d_n2, assign53980_e82659_d_n4, assign53980_e82659_d_n5, assign53980_e82659_d_n6, assign53980_e82659_d_n7, assign53980_e82659_d_n8, assign53980_e82659_d_n9, assign53980_e82659_d_n10, assign53980_e82659_d_n11, assign53980_e82659_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1364 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign53980_e82659;
        locals.var_ps0dep_dn0 = assign53980_e82659_d_n0;
        locals.var_ps0dep_dn2 = assign53980_e82659_d_n2;
        locals.var_ps0dep_dn4 = assign53980_e82659_d_n4;
        locals.var_ps0dep_dn5 = assign53980_e82659_d_n5;
        locals.var_ps0dep_dn6 = assign53980_e82659_d_n6;
        locals.var_ps0dep_dn7 = assign53980_e82659_d_n7;
        locals.var_ps0dep_dn8 = assign53980_e82659_d_n8;
        locals.var_ps0dep_dn9 = assign53980_e82659_d_n9;
        locals.var_ps0dep_dn10 = assign53980_e82659_d_n10;
        locals.var_ps0dep_dn11 = assign53980_e82659_d_n11;
        locals.var_ps0dep_dn14 = assign53980_e82659_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign53990_e82676, assign53990_e82676_d_n0, assign53990_e82676_d_n2, assign53990_e82676_d_n4, assign53990_e82676_d_n5, assign53990_e82676_d_n6, assign53990_e82676_d_n7, assign53990_e82676_d_n8, assign53990_e82676_d_n9, assign53990_e82676_d_n10, assign53990_e82676_d_n11, assign53990_e82676_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1364 == 0.0)) {
        (locals.var_vgp_res__blk1147, locals.var_vgp_res__blk1147_dn0, locals.var_vgp_res__blk1147_dn2, locals.var_vgp_res__blk1147_dn4, locals.var_vgp_res__blk1147_dn5, locals.var_vgp_res__blk1147_dn6, locals.var_vgp_res__blk1147_dn7, locals.var_vgp_res__blk1147_dn8, locals.var_vgp_res__blk1147_dn9, locals.var_vgp_res__blk1147_dn10, locals.var_vgp_res__blk1147_dn11, locals.var_vgp_res__blk1147_dn14,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn11, locals.var_vgp_ws_dn14,)
    }
};
        locals.var_vgp_ws = assign53990_e82676;
        locals.var_vgp_ws_dn0 = assign53990_e82676_d_n0;
        locals.var_vgp_ws_dn2 = assign53990_e82676_d_n2;
        locals.var_vgp_ws_dn4 = assign53990_e82676_d_n4;
        locals.var_vgp_ws_dn5 = assign53990_e82676_d_n5;
        locals.var_vgp_ws_dn6 = assign53990_e82676_d_n6;
        locals.var_vgp_ws_dn7 = assign53990_e82676_d_n7;
        locals.var_vgp_ws_dn8 = assign53990_e82676_d_n8;
        locals.var_vgp_ws_dn9 = assign53990_e82676_d_n9;
        locals.var_vgp_ws_dn10 = assign53990_e82676_d_n10;
        locals.var_vgp_ws_dn11 = assign53990_e82676_d_n11;
        locals.var_vgp_ws_dn14 = assign53990_e82676_d_n14;
        locals.var_vgp_ws_rv = 0.0;

        let (assign54000_e82693, assign54000_e82693_d_n0, assign54000_e82693_d_n2, assign54000_e82693_d_n4, assign54000_e82693_d_n5, assign54000_e82693_d_n6, assign54000_e82693_d_n7, assign54000_e82693_d_n8, assign54000_e82693_d_n9, assign54000_e82693_d_n10, assign54000_e82693_d_n11, assign54000_e82693_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1364 == 0.0)) {
        (locals.var_vgp_res__blk1147, locals.var_vgp_res__blk1147_dn0, locals.var_vgp_res__blk1147_dn2, locals.var_vgp_res__blk1147_dn4, locals.var_vgp_res__blk1147_dn5, locals.var_vgp_res__blk1147_dn6, locals.var_vgp_res__blk1147_dn7, locals.var_vgp_res__blk1147_dn8, locals.var_vgp_res__blk1147_dn9, locals.var_vgp_res__blk1147_dn10, locals.var_vgp_res__blk1147_dn11, locals.var_vgp_res__blk1147_dn14,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn11, locals.var_vgp_res_raw_dn14,)
    }
};
        locals.var_vgp_res_raw = assign54000_e82693;
        locals.var_vgp_res_raw_dn0 = assign54000_e82693_d_n0;
        locals.var_vgp_res_raw_dn2 = assign54000_e82693_d_n2;
        locals.var_vgp_res_raw_dn4 = assign54000_e82693_d_n4;
        locals.var_vgp_res_raw_dn5 = assign54000_e82693_d_n5;
        locals.var_vgp_res_raw_dn6 = assign54000_e82693_d_n6;
        locals.var_vgp_res_raw_dn7 = assign54000_e82693_d_n7;
        locals.var_vgp_res_raw_dn8 = assign54000_e82693_d_n8;
        locals.var_vgp_res_raw_dn9 = assign54000_e82693_d_n9;
        locals.var_vgp_res_raw_dn10 = assign54000_e82693_d_n10;
        locals.var_vgp_res_raw_dn11 = assign54000_e82693_d_n11;
        locals.var_vgp_res_raw_dn14 = assign54000_e82693_d_n14;
        locals.var_vgp_res_raw_rv = 0.0;

        let (assign54010_e82707,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign54010_e82707;
        locals.var_flg_conv_rv = 0.0;

        let (assign54020_e82721,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign54020_e82721;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_196(
        locals: &mut StampLocals,
    ) {
        let mut assign54030_loop_guard: usize = 0;
        while {
            let assign54030_cond_e82736: f64 = if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign54030_cond_e82736 != 0.0
        } {
            assign54030_loop_guard += 1;
            assert!(assign54030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54030_body0_e82752, assign54030_body0_e82752_d_n0, assign54030_body0_e82752_d_n2, assign54030_body0_e82752_d_n4, assign54030_body0_e82752_d_n5, assign54030_body0_e82752_d_n6, assign54030_body0_e82752_d_n7, assign54030_body0_e82752_d_n8, assign54030_body0_e82752_d_n9, assign54030_body0_e82752_d_n10, assign54030_body0_e82752_d_n11, assign54030_body0_e82752_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign54030_body0_e82750: f64 = (locals.var_beta * locals.var_ps0dep);
        (assign54030_body0_e82750, ((locals.var_beta_dn0 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn0)), ((locals.var_beta_dn2 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn2)), ((locals.var_beta_dn4 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn4)), ((locals.var_beta_dn5 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn5)), ((locals.var_beta_dn6 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn6)), ((locals.var_beta_dn7 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn7)), ((locals.var_beta_dn8 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn8)), ((locals.var_beta_dn9 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn9)), ((locals.var_beta_dn10 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn10)), ((locals.var_beta_dn11 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn11)), ((locals.var_beta_dn14 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign54030_body0_e82752;
            locals.var_t1_dn0 = assign54030_body0_e82752_d_n0;
            locals.var_t1_dn2 = assign54030_body0_e82752_d_n2;
            locals.var_t1_dn4 = assign54030_body0_e82752_d_n4;
            locals.var_t1_dn5 = assign54030_body0_e82752_d_n5;
            locals.var_t1_dn6 = assign54030_body0_e82752_d_n6;
            locals.var_t1_dn7 = assign54030_body0_e82752_d_n7;
            locals.var_t1_dn8 = assign54030_body0_e82752_d_n8;
            locals.var_t1_dn9 = assign54030_body0_e82752_d_n9;
            locals.var_t1_dn10 = assign54030_body0_e82752_d_n10;
            locals.var_t1_dn11 = assign54030_body0_e82752_d_n11;
            locals.var_t1_dn14 = assign54030_body0_e82752_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign54030_body1_e82767, assign54030_body1_e82767_d_n0, assign54030_body1_e82767_d_n2, assign54030_body1_e82767_d_n4, assign54030_body1_e82767_d_n5, assign54030_body1_e82767_d_n6, assign54030_body1_e82767_d_n7, assign54030_body1_e82767_d_n8, assign54030_body1_e82767_d_n9, assign54030_body1_e82767_d_n10, assign54030_body1_e82767_d_n11, assign54030_body1_e82767_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign54030_body1_e82765: f64 = (locals.var_t1).exp();
        (assign54030_body1_e82765, (assign54030_body1_e82765 * locals.var_t1_dn0), (assign54030_body1_e82765 * locals.var_t1_dn2), (assign54030_body1_e82765 * locals.var_t1_dn4), (assign54030_body1_e82765 * locals.var_t1_dn5), (assign54030_body1_e82765 * locals.var_t1_dn6), (assign54030_body1_e82765 * locals.var_t1_dn7), (assign54030_body1_e82765 * locals.var_t1_dn8), (assign54030_body1_e82765 * locals.var_t1_dn9), (assign54030_body1_e82765 * locals.var_t1_dn10), (assign54030_body1_e82765 * locals.var_t1_dn11), (assign54030_body1_e82765 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign54030_body1_e82767;
            locals.var_t2_dn0 = assign54030_body1_e82767_d_n0;
            locals.var_t2_dn2 = assign54030_body1_e82767_d_n2;
            locals.var_t2_dn4 = assign54030_body1_e82767_d_n4;
            locals.var_t2_dn5 = assign54030_body1_e82767_d_n5;
            locals.var_t2_dn6 = assign54030_body1_e82767_d_n6;
            locals.var_t2_dn7 = assign54030_body1_e82767_d_n7;
            locals.var_t2_dn8 = assign54030_body1_e82767_d_n8;
            locals.var_t2_dn9 = assign54030_body1_e82767_d_n9;
            locals.var_t2_dn10 = assign54030_body1_e82767_d_n10;
            locals.var_t2_dn11 = assign54030_body1_e82767_d_n11;
            locals.var_t2_dn14 = assign54030_body1_e82767_d_n14;
            locals.var_t2_rv = 0.0;
            let assign54030_body2_e82770: f64 = if locals.var_ps0dep >= 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1365 = assign54030_body2_e82770;
            locals.var_guard1365_rv = 0.0;
            let (assign54030_body3_e82796, assign54030_body3_e82796_d_n0, assign54030_body3_e82796_d_n2, assign54030_body3_e82796_d_n4, assign54030_body3_e82796_d_n5, assign54030_body3_e82796_d_n6, assign54030_body3_e82796_d_n7, assign54030_body3_e82796_d_n8, assign54030_body3_e82796_d_n9, assign54030_body3_e82796_d_n10, assign54030_body3_e82796_d_n11, assign54030_body3_e82796_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1365 != 0.0)) {
        let assign54030_body3_e82785: f64 = (-locals.var_cnst0);
        let assign54030_body3_e82788: f64 = (locals.var_t2 - 1.0);
        let assign54030_body3_e82790: f64 = (assign54030_body3_e82788 - locals.var_t1);
        let assign54030_body3_e82792: f64 = (assign54030_body3_e82790 + 1e-15);
        let assign54030_body3_e82793: f64 = (assign54030_body3_e82792).sqrt();
        let assign54030_body3_e82794: f64 = (assign54030_body3_e82785 * assign54030_body3_e82793);
        (assign54030_body3_e82794, (((-locals.var_cnst0_dn0) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign54030_body3_e82793)))), (((-locals.var_cnst0_dn2) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign54030_body3_e82793)))), (((-locals.var_cnst0_dn4) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign54030_body3_e82793)))), (((-locals.var_cnst0_dn5) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign54030_body3_e82793)))), (((-locals.var_cnst0_dn6) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign54030_body3_e82793)))), (((-locals.var_cnst0_dn7) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign54030_body3_e82793)))), (((-locals.var_cnst0_dn8) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign54030_body3_e82793)))), (((-locals.var_cnst0_dn9) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign54030_body3_e82793)))), (((-locals.var_cnst0_dn10) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign54030_body3_e82793)))), (((-locals.var_cnst0_dn11) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign54030_body3_e82793)))), (((-locals.var_cnst0_dn14) * assign54030_body3_e82793) + (assign54030_body3_e82785 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign54030_body3_e82793)))),)
    } else {
        (locals.var_q_s0__blk1324, locals.var_q_s0__blk1324_dn0, locals.var_q_s0__blk1324_dn2, locals.var_q_s0__blk1324_dn4, locals.var_q_s0__blk1324_dn5, locals.var_q_s0__blk1324_dn6, locals.var_q_s0__blk1324_dn7, locals.var_q_s0__blk1324_dn8, locals.var_q_s0__blk1324_dn9, locals.var_q_s0__blk1324_dn10, locals.var_q_s0__blk1324_dn11, locals.var_q_s0__blk1324_dn14,)
    }
};
            locals.var_q_s0__blk1324 = assign54030_body3_e82796;
            locals.var_q_s0__blk1324_dn0 = assign54030_body3_e82796_d_n0;
            locals.var_q_s0__blk1324_dn2 = assign54030_body3_e82796_d_n2;
            locals.var_q_s0__blk1324_dn4 = assign54030_body3_e82796_d_n4;
            locals.var_q_s0__blk1324_dn5 = assign54030_body3_e82796_d_n5;
            locals.var_q_s0__blk1324_dn6 = assign54030_body3_e82796_d_n6;
            locals.var_q_s0__blk1324_dn7 = assign54030_body3_e82796_d_n7;
            locals.var_q_s0__blk1324_dn8 = assign54030_body3_e82796_d_n8;
            locals.var_q_s0__blk1324_dn9 = assign54030_body3_e82796_d_n9;
            locals.var_q_s0__blk1324_dn10 = assign54030_body3_e82796_d_n10;
            locals.var_q_s0__blk1324_dn11 = assign54030_body3_e82796_d_n11;
            locals.var_q_s0__blk1324_dn14 = assign54030_body3_e82796_d_n14;
            locals.var_q_s0__blk1324_rv = 0.0;
            let (assign54030_body4_e82824, assign54030_body4_e82824_d_n0, assign54030_body4_e82824_d_n2, assign54030_body4_e82824_d_n4, assign54030_body4_e82824_d_n5, assign54030_body4_e82824_d_n6, assign54030_body4_e82824_d_n7, assign54030_body4_e82824_d_n8, assign54030_body4_e82824_d_n9, assign54030_body4_e82824_d_n10, assign54030_body4_e82824_d_n11, assign54030_body4_e82824_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1365 != 0.0)) {
        let assign54030_body4_e82812: f64 = (0.5 * locals.var_cnst0);
        let assign54030_body4_e82814: f64 = (assign54030_body4_e82812 * locals.var_cnst0);
        let assign54030_body4_e82816: f64 = (assign54030_body4_e82814 / locals.var_q_s0__blk1324);
        let assign54030_body4_e82819: f64 = (locals.var_beta * locals.var_t2);
        let assign54030_body4_e82821: f64 = (assign54030_body4_e82819 - locals.var_beta);
        let assign54030_body4_e82822: f64 = (assign54030_body4_e82816 * assign54030_body4_e82821);
        (assign54030_body4_e82822, ((((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn0)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0))), ((((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn2)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2))), ((((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn4)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4))), ((((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn5)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5))), ((((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn6)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6))), ((((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn7)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7))), ((((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn8)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8))), ((((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn9)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9))), ((((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn10)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10))), ((((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn11)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11))), ((((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign54030_body4_e82812 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1324) - (assign54030_body4_e82814 * locals.var_q_s0__blk1324_dn14)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)) * assign54030_body4_e82821) + (assign54030_body4_e82816 * (((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14))),)
    } else {
        (locals.var_q_s0_dps__blk1127, locals.var_q_s0_dps__blk1127_dn0, locals.var_q_s0_dps__blk1127_dn2, locals.var_q_s0_dps__blk1127_dn4, locals.var_q_s0_dps__blk1127_dn5, locals.var_q_s0_dps__blk1127_dn6, locals.var_q_s0_dps__blk1127_dn7, locals.var_q_s0_dps__blk1127_dn8, locals.var_q_s0_dps__blk1127_dn9, locals.var_q_s0_dps__blk1127_dn10, locals.var_q_s0_dps__blk1127_dn11, locals.var_q_s0_dps__blk1127_dn14,)
    }
};
            locals.var_q_s0_dps__blk1127 = assign54030_body4_e82824;
            locals.var_q_s0_dps__blk1127_dn0 = assign54030_body4_e82824_d_n0;
            locals.var_q_s0_dps__blk1127_dn2 = assign54030_body4_e82824_d_n2;
            locals.var_q_s0_dps__blk1127_dn4 = assign54030_body4_e82824_d_n4;
            locals.var_q_s0_dps__blk1127_dn5 = assign54030_body4_e82824_d_n5;
            locals.var_q_s0_dps__blk1127_dn6 = assign54030_body4_e82824_d_n6;
            locals.var_q_s0_dps__blk1127_dn7 = assign54030_body4_e82824_d_n7;
            locals.var_q_s0_dps__blk1127_dn8 = assign54030_body4_e82824_d_n8;
            locals.var_q_s0_dps__blk1127_dn9 = assign54030_body4_e82824_d_n9;
            locals.var_q_s0_dps__blk1127_dn10 = assign54030_body4_e82824_d_n10;
            locals.var_q_s0_dps__blk1127_dn11 = assign54030_body4_e82824_d_n11;
            locals.var_q_s0_dps__blk1127_dn14 = assign54030_body4_e82824_d_n14;
            locals.var_q_s0_dps__blk1127_rv = 0.0;
            let (assign54030_body5_e82847, assign54030_body5_e82847_d_n0, assign54030_body5_e82847_d_n2, assign54030_body5_e82847_d_n4, assign54030_body5_e82847_d_n5, assign54030_body5_e82847_d_n6, assign54030_body5_e82847_d_n7, assign54030_body5_e82847_d_n8, assign54030_body5_e82847_d_n9, assign54030_body5_e82847_d_n10, assign54030_body5_e82847_d_n11, assign54030_body5_e82847_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1365 == 0.0)) {
        let assign54030_body5_e82840: f64 = (-locals.var_beta);
        let assign54030_body5_e82843: f64 = (locals.var_ps0dep - locals.var_depvbs);
        let assign54030_body5_e82844: f64 = (assign54030_body5_e82840 * assign54030_body5_e82843);
        let assign54030_body5_e82845: f64 = (assign54030_body5_e82844).exp();
        (assign54030_body5_e82845, (assign54030_body5_e82845 * (((-locals.var_beta_dn0) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn0 - locals.var_depvbs_dn0)))), (assign54030_body5_e82845 * (((-locals.var_beta_dn2) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn2 - locals.var_depvbs_dn2)))), (assign54030_body5_e82845 * (((-locals.var_beta_dn4) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn4 - locals.var_depvbs_dn4)))), (assign54030_body5_e82845 * (((-locals.var_beta_dn5) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn5 - locals.var_depvbs_dn5)))), (assign54030_body5_e82845 * (((-locals.var_beta_dn6) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn6 - locals.var_depvbs_dn6)))), (assign54030_body5_e82845 * (((-locals.var_beta_dn7) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn7 - locals.var_depvbs_dn7)))), (assign54030_body5_e82845 * (((-locals.var_beta_dn8) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn8 - locals.var_depvbs_dn8)))), (assign54030_body5_e82845 * (((-locals.var_beta_dn9) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn9 - locals.var_depvbs_dn9)))), (assign54030_body5_e82845 * (((-locals.var_beta_dn10) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn10 - locals.var_depvbs_dn10)))), (assign54030_body5_e82845 * (((-locals.var_beta_dn11) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn11 - locals.var_depvbs_dn11)))), (assign54030_body5_e82845 * (((-locals.var_beta_dn14) * assign54030_body5_e82843) + (assign54030_body5_e82840 * (locals.var_ps0dep_dn14 - locals.var_depvbs_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign54030_body5_e82847;
            locals.var_t3_dn0 = assign54030_body5_e82847_d_n0;
            locals.var_t3_dn2 = assign54030_body5_e82847_d_n2;
            locals.var_t3_dn4 = assign54030_body5_e82847_d_n4;
            locals.var_t3_dn5 = assign54030_body5_e82847_d_n5;
            locals.var_t3_dn6 = assign54030_body5_e82847_d_n6;
            locals.var_t3_dn7 = assign54030_body5_e82847_d_n7;
            locals.var_t3_dn8 = assign54030_body5_e82847_d_n8;
            locals.var_t3_dn9 = assign54030_body5_e82847_d_n9;
            locals.var_t3_dn10 = assign54030_body5_e82847_d_n10;
            locals.var_t3_dn11 = assign54030_body5_e82847_d_n11;
            locals.var_t3_dn14 = assign54030_body5_e82847_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign54030_body6_e82867, assign54030_body6_e82867_d_n0, assign54030_body6_e82867_d_n2, assign54030_body6_e82867_d_n4, assign54030_body6_e82867_d_n5, assign54030_body6_e82867_d_n6, assign54030_body6_e82867_d_n7, assign54030_body6_e82867_d_n8, assign54030_body6_e82867_d_n9, assign54030_body6_e82867_d_n10, assign54030_body6_e82867_d_n11, assign54030_body6_e82867_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1365 == 0.0)) {
        let assign54030_body6_e82864: f64 = (locals.var_beta * locals.var_depvbs);
        let assign54030_body6_e82865: f64 = (assign54030_body6_e82864).exp();
        (assign54030_body6_e82865, (assign54030_body6_e82865 * ((locals.var_beta_dn0 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn0))), (assign54030_body6_e82865 * ((locals.var_beta_dn2 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn2))), (assign54030_body6_e82865 * ((locals.var_beta_dn4 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn4))), (assign54030_body6_e82865 * ((locals.var_beta_dn5 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn5))), (assign54030_body6_e82865 * ((locals.var_beta_dn6 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn6))), (assign54030_body6_e82865 * ((locals.var_beta_dn7 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn7))), (assign54030_body6_e82865 * ((locals.var_beta_dn8 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn8))), (assign54030_body6_e82865 * ((locals.var_beta_dn9 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn9))), (assign54030_body6_e82865 * ((locals.var_beta_dn10 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn10))), (assign54030_body6_e82865 * ((locals.var_beta_dn11 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn11))), (assign54030_body6_e82865 * ((locals.var_beta_dn14 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign54030_body6_e82867;
            locals.var_t4_dn0 = assign54030_body6_e82867_d_n0;
            locals.var_t4_dn2 = assign54030_body6_e82867_d_n2;
            locals.var_t4_dn4 = assign54030_body6_e82867_d_n4;
            locals.var_t4_dn5 = assign54030_body6_e82867_d_n5;
            locals.var_t4_dn6 = assign54030_body6_e82867_d_n6;
            locals.var_t4_dn7 = assign54030_body6_e82867_d_n7;
            locals.var_t4_dn8 = assign54030_body6_e82867_d_n8;
            locals.var_t4_dn9 = assign54030_body6_e82867_d_n9;
            locals.var_t4_dn10 = assign54030_body6_e82867_d_n10;
            locals.var_t4_dn11 = assign54030_body6_e82867_d_n11;
            locals.var_t4_dn14 = assign54030_body6_e82867_d_n14;
            locals.var_t4_rv = 0.0;
            let (assign54030_body7_e82899, assign54030_body7_e82899_d_n0, assign54030_body7_e82899_d_n2, assign54030_body7_e82899_d_n4, assign54030_body7_e82899_d_n5, assign54030_body7_e82899_d_n6, assign54030_body7_e82899_d_n7, assign54030_body7_e82899_d_n8, assign54030_body7_e82899_d_n9, assign54030_body7_e82899_d_n10, assign54030_body7_e82899_d_n11, assign54030_body7_e82899_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1365 == 0.0)) {
        let assign54030_body7_e82885: f64 = (locals.var_t2 - 1.0);
        let assign54030_body7_e82887: f64 = (assign54030_body7_e82885 - locals.var_t1);
        let assign54030_body7_e82891: f64 = (locals.var_t3 - locals.var_t4);
        let assign54030_body7_e82892: f64 = (locals.var_cnst1 * assign54030_body7_e82891);
        let assign54030_body7_e82893: f64 = (assign54030_body7_e82887 + assign54030_body7_e82892);
        let assign54030_body7_e82895: f64 = (assign54030_body7_e82893 + 1e-15);
        let assign54030_body7_e82896: f64 = (assign54030_body7_e82895).sqrt();
        let assign54030_body7_e82897: f64 = (locals.var_cnst0 * assign54030_body7_e82896);
        (assign54030_body7_e82897, ((locals.var_cnst0_dn0 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign54030_body7_e82896)))), ((locals.var_cnst0_dn2 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign54030_body7_e82896)))), ((locals.var_cnst0_dn4 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign54030_body7_e82896)))), ((locals.var_cnst0_dn5 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign54030_body7_e82896)))), ((locals.var_cnst0_dn6 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign54030_body7_e82896)))), ((locals.var_cnst0_dn7 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign54030_body7_e82896)))), ((locals.var_cnst0_dn8 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign54030_body7_e82896)))), ((locals.var_cnst0_dn9 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign54030_body7_e82896)))), ((locals.var_cnst0_dn10 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign54030_body7_e82896)))), ((locals.var_cnst0_dn11 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn11 - locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn11 - locals.var_t4_dn11)))) / (2.0 * assign54030_body7_e82896)))), ((locals.var_cnst0_dn14 * assign54030_body7_e82896) + (locals.var_cnst0 * (((locals.var_t2_dn14 - locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign54030_body7_e82891) + (locals.var_cnst1 * (locals.var_t3_dn14 - locals.var_t4_dn14)))) / (2.0 * assign54030_body7_e82896)))),)
    } else {
        (locals.var_q_s0__blk1324, locals.var_q_s0__blk1324_dn0, locals.var_q_s0__blk1324_dn2, locals.var_q_s0__blk1324_dn4, locals.var_q_s0__blk1324_dn5, locals.var_q_s0__blk1324_dn6, locals.var_q_s0__blk1324_dn7, locals.var_q_s0__blk1324_dn8, locals.var_q_s0__blk1324_dn9, locals.var_q_s0__blk1324_dn10, locals.var_q_s0__blk1324_dn11, locals.var_q_s0__blk1324_dn14,)
    }
};
            locals.var_q_s0__blk1324 = assign54030_body7_e82899;
            locals.var_q_s0__blk1324_dn0 = assign54030_body7_e82899_d_n0;
            locals.var_q_s0__blk1324_dn2 = assign54030_body7_e82899_d_n2;
            locals.var_q_s0__blk1324_dn4 = assign54030_body7_e82899_d_n4;
            locals.var_q_s0__blk1324_dn5 = assign54030_body7_e82899_d_n5;
            locals.var_q_s0__blk1324_dn6 = assign54030_body7_e82899_d_n6;
            locals.var_q_s0__blk1324_dn7 = assign54030_body7_e82899_d_n7;
            locals.var_q_s0__blk1324_dn8 = assign54030_body7_e82899_d_n8;
            locals.var_q_s0__blk1324_dn9 = assign54030_body7_e82899_d_n9;
            locals.var_q_s0__blk1324_dn10 = assign54030_body7_e82899_d_n10;
            locals.var_q_s0__blk1324_dn11 = assign54030_body7_e82899_d_n11;
            locals.var_q_s0__blk1324_dn14 = assign54030_body7_e82899_d_n14;
            locals.var_q_s0__blk1324_rv = 0.0;
            let (assign54030_body8_e82922, assign54030_body8_e82922_d_n0, assign54030_body8_e82922_d_n2, assign54030_body8_e82922_d_n4, assign54030_body8_e82922_d_n5, assign54030_body8_e82922_d_n6, assign54030_body8_e82922_d_n7, assign54030_body8_e82922_d_n8, assign54030_body8_e82922_d_n9, assign54030_body8_e82922_d_n10, assign54030_body8_e82922_d_n11, assign54030_body8_e82922_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1365 == 0.0)) {
        let assign54030_body8_e82916: f64 = (0.5 * locals.var_cnst0);
        let assign54030_body8_e82918: f64 = (assign54030_body8_e82916 * locals.var_cnst0);
        let assign54030_body8_e82920: f64 = (assign54030_body8_e82918 / locals.var_q_s0__blk1324);
        (assign54030_body8_e82920, ((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn0)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn2)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn4)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn5)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn6)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn7)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn8)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn9)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn10)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn11)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)), ((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign54030_body8_e82916 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1324) - (assign54030_body8_e82918 * locals.var_q_s0__blk1324_dn14)) / (locals.var_q_s0__blk1324 * locals.var_q_s0__blk1324)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
            locals.var_t5 = assign54030_body8_e82922;
            locals.var_t5_dn0 = assign54030_body8_e82922_d_n0;
            locals.var_t5_dn2 = assign54030_body8_e82922_d_n2;
            locals.var_t5_dn4 = assign54030_body8_e82922_d_n4;
            locals.var_t5_dn5 = assign54030_body8_e82922_d_n5;
            locals.var_t5_dn6 = assign54030_body8_e82922_d_n6;
            locals.var_t5_dn7 = assign54030_body8_e82922_d_n7;
            locals.var_t5_dn8 = assign54030_body8_e82922_d_n8;
            locals.var_t5_dn9 = assign54030_body8_e82922_d_n9;
            locals.var_t5_dn10 = assign54030_body8_e82922_d_n10;
            locals.var_t5_dn11 = assign54030_body8_e82922_d_n11;
            locals.var_t5_dn14 = assign54030_body8_e82922_d_n14;
            locals.var_t5_rv = 0.0;
            let (assign54030_body9_e82952, assign54030_body9_e82952_d_n0, assign54030_body9_e82952_d_n2, assign54030_body9_e82952_d_n4, assign54030_body9_e82952_d_n5, assign54030_body9_e82952_d_n6, assign54030_body9_e82952_d_n7, assign54030_body9_e82952_d_n8, assign54030_body9_e82952_d_n9, assign54030_body9_e82952_d_n10, assign54030_body9_e82952_d_n11, assign54030_body9_e82952_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1365 == 0.0)) {
        let assign54030_body9_e82940: f64 = (locals.var_beta * locals.var_t2);
        let assign54030_body9_e82942: f64 = (assign54030_body9_e82940 - locals.var_beta);
        let assign54030_body9_e82945: f64 = (-locals.var_beta);
        let assign54030_body9_e82947: f64 = (assign54030_body9_e82945 * locals.var_t3);
        let assign54030_body9_e82948: f64 = (locals.var_cnst1 * assign54030_body9_e82947);
        let assign54030_body9_e82949: f64 = (assign54030_body9_e82942 + assign54030_body9_e82948);
        let assign54030_body9_e82950: f64 = (locals.var_t5 * assign54030_body9_e82949);
        (assign54030_body9_e82950, ((locals.var_t5_dn0 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0) + ((locals.var_cnst1_dn0 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn0) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn0))))))), ((locals.var_t5_dn2 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2) + ((locals.var_cnst1_dn2 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn2) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn2))))))), ((locals.var_t5_dn4 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4) + ((locals.var_cnst1_dn4 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn4) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn4))))))), ((locals.var_t5_dn5 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5) + ((locals.var_cnst1_dn5 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn5) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn5))))))), ((locals.var_t5_dn6 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6) + ((locals.var_cnst1_dn6 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn6) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn6))))))), ((locals.var_t5_dn7 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7) + ((locals.var_cnst1_dn7 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn7) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn7))))))), ((locals.var_t5_dn8 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8) + ((locals.var_cnst1_dn8 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn8) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn8))))))), ((locals.var_t5_dn9 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9) + ((locals.var_cnst1_dn9 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn9) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn9))))))), ((locals.var_t5_dn10 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10) + ((locals.var_cnst1_dn10 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn10) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn10))))))), ((locals.var_t5_dn11 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11) + ((locals.var_cnst1_dn11 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn11) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn11))))))), ((locals.var_t5_dn14 * assign54030_body9_e82949) + (locals.var_t5 * ((((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14) + ((locals.var_cnst1_dn14 * assign54030_body9_e82947) + (locals.var_cnst1 * (((-locals.var_beta_dn14) * locals.var_t3) + (assign54030_body9_e82945 * locals.var_t3_dn14))))))),)
    } else {
        (locals.var_q_s0_dps__blk1127, locals.var_q_s0_dps__blk1127_dn0, locals.var_q_s0_dps__blk1127_dn2, locals.var_q_s0_dps__blk1127_dn4, locals.var_q_s0_dps__blk1127_dn5, locals.var_q_s0_dps__blk1127_dn6, locals.var_q_s0_dps__blk1127_dn7, locals.var_q_s0_dps__blk1127_dn8, locals.var_q_s0_dps__blk1127_dn9, locals.var_q_s0_dps__blk1127_dn10, locals.var_q_s0_dps__blk1127_dn11, locals.var_q_s0_dps__blk1127_dn14,)
    }
};
            locals.var_q_s0_dps__blk1127 = assign54030_body9_e82952;
            locals.var_q_s0_dps__blk1127_dn0 = assign54030_body9_e82952_d_n0;
            locals.var_q_s0_dps__blk1127_dn2 = assign54030_body9_e82952_d_n2;
            locals.var_q_s0_dps__blk1127_dn4 = assign54030_body9_e82952_d_n4;
            locals.var_q_s0_dps__blk1127_dn5 = assign54030_body9_e82952_d_n5;
            locals.var_q_s0_dps__blk1127_dn6 = assign54030_body9_e82952_d_n6;
            locals.var_q_s0_dps__blk1127_dn7 = assign54030_body9_e82952_d_n7;
            locals.var_q_s0_dps__blk1127_dn8 = assign54030_body9_e82952_d_n8;
            locals.var_q_s0_dps__blk1127_dn9 = assign54030_body9_e82952_d_n9;
            locals.var_q_s0_dps__blk1127_dn10 = assign54030_body9_e82952_d_n10;
            locals.var_q_s0_dps__blk1127_dn11 = assign54030_body9_e82952_d_n11;
            locals.var_q_s0_dps__blk1127_dn14 = assign54030_body9_e82952_d_n14;
            locals.var_q_s0_dps__blk1127_rv = 0.0;
            let (assign54030_body10_e82970,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign54030_body10_e82968: f64 = (150.0 + 1.0);
        (assign54030_body10_e82968,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign54030_body10_e82970;
            locals.var_lp_s0_rv = 0.0;
            let (assign54030_body11_e82993, assign54030_body11_e82993_d_n0, assign54030_body11_e82993_d_n2, assign54030_body11_e82993_d_n4, assign54030_body11_e82993_d_n5, assign54030_body11_e82993_d_n6, assign54030_body11_e82993_d_n7, assign54030_body11_e82993_d_n8, assign54030_body11_e82993_d_n9, assign54030_body11_e82993_d_n10, assign54030_body11_e82993_d_n11, assign54030_body11_e82993_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54030_body11_e82988: f64 = (locals.var_vgp_ws - locals.var_ps0dep);
        let assign54030_body11_e82989: f64 = (locals.var_cox * assign54030_body11_e82988);
        let assign54030_body11_e82991: f64 = (assign54030_body11_e82989 + locals.var_q_s0__blk1324);
        (assign54030_body11_e82991, (((locals.var_cox_dn0 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn0 - locals.var_ps0dep_dn0))) + locals.var_q_s0__blk1324_dn0), (((locals.var_cox_dn2 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn2 - locals.var_ps0dep_dn2))) + locals.var_q_s0__blk1324_dn2), (((locals.var_cox_dn4 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn4 - locals.var_ps0dep_dn4))) + locals.var_q_s0__blk1324_dn4), (((locals.var_cox_dn5 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn5 - locals.var_ps0dep_dn5))) + locals.var_q_s0__blk1324_dn5), (((locals.var_cox_dn6 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn6 - locals.var_ps0dep_dn6))) + locals.var_q_s0__blk1324_dn6), (((locals.var_cox_dn7 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn7 - locals.var_ps0dep_dn7))) + locals.var_q_s0__blk1324_dn7), (((locals.var_cox_dn8 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn8 - locals.var_ps0dep_dn8))) + locals.var_q_s0__blk1324_dn8), (((locals.var_cox_dn9 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn9 - locals.var_ps0dep_dn9))) + locals.var_q_s0__blk1324_dn9), (((locals.var_cox_dn10 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn10 - locals.var_ps0dep_dn10))) + locals.var_q_s0__blk1324_dn10), (((locals.var_cox_dn11 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn11 - locals.var_ps0dep_dn11))) + locals.var_q_s0__blk1324_dn11), (((locals.var_cox_dn14 * assign54030_body11_e82988) + (locals.var_cox * (locals.var_vgp_ws_dn14 - locals.var_ps0dep_dn14))) + locals.var_q_s0__blk1324_dn14),)
    } else {
        (locals.var_pf1__blk1102, locals.var_pf1__blk1102_dn0, locals.var_pf1__blk1102_dn2, locals.var_pf1__blk1102_dn4, locals.var_pf1__blk1102_dn5, locals.var_pf1__blk1102_dn6, locals.var_pf1__blk1102_dn7, locals.var_pf1__blk1102_dn8, locals.var_pf1__blk1102_dn9, locals.var_pf1__blk1102_dn10, locals.var_pf1__blk1102_dn11, locals.var_pf1__blk1102_dn14,)
    }
};
            locals.var_pf1__blk1102 = assign54030_body11_e82993;
            locals.var_pf1__blk1102_dn0 = assign54030_body11_e82993_d_n0;
            locals.var_pf1__blk1102_dn2 = assign54030_body11_e82993_d_n2;
            locals.var_pf1__blk1102_dn4 = assign54030_body11_e82993_d_n4;
            locals.var_pf1__blk1102_dn5 = assign54030_body11_e82993_d_n5;
            locals.var_pf1__blk1102_dn6 = assign54030_body11_e82993_d_n6;
            locals.var_pf1__blk1102_dn7 = assign54030_body11_e82993_d_n7;
            locals.var_pf1__blk1102_dn8 = assign54030_body11_e82993_d_n8;
            locals.var_pf1__blk1102_dn9 = assign54030_body11_e82993_d_n9;
            locals.var_pf1__blk1102_dn10 = assign54030_body11_e82993_d_n10;
            locals.var_pf1__blk1102_dn11 = assign54030_body11_e82993_d_n11;
            locals.var_pf1__blk1102_dn14 = assign54030_body11_e82993_d_n14;
            locals.var_pf1__blk1102_rv = 0.0;
            let (assign54030_body12_e83013, assign54030_body12_e83013_d_n0, assign54030_body12_e83013_d_n2, assign54030_body12_e83013_d_n4, assign54030_body12_e83013_d_n5, assign54030_body12_e83013_d_n6, assign54030_body12_e83013_d_n7, assign54030_body12_e83013_d_n8, assign54030_body12_e83013_d_n9, assign54030_body12_e83013_d_n10, assign54030_body12_e83013_d_n11, assign54030_body12_e83013_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54030_body12_e83009: f64 = (-locals.var_cox);
        let assign54030_body12_e83011: f64 = (assign54030_body12_e83009 + locals.var_q_s0_dps__blk1127);
        (assign54030_body12_e83011, ((-locals.var_cox_dn0) + locals.var_q_s0_dps__blk1127_dn0), ((-locals.var_cox_dn2) + locals.var_q_s0_dps__blk1127_dn2), ((-locals.var_cox_dn4) + locals.var_q_s0_dps__blk1127_dn4), ((-locals.var_cox_dn5) + locals.var_q_s0_dps__blk1127_dn5), ((-locals.var_cox_dn6) + locals.var_q_s0_dps__blk1127_dn6), ((-locals.var_cox_dn7) + locals.var_q_s0_dps__blk1127_dn7), ((-locals.var_cox_dn8) + locals.var_q_s0_dps__blk1127_dn8), ((-locals.var_cox_dn9) + locals.var_q_s0_dps__blk1127_dn9), ((-locals.var_cox_dn10) + locals.var_q_s0_dps__blk1127_dn10), ((-locals.var_cox_dn11) + locals.var_q_s0_dps__blk1127_dn11), ((-locals.var_cox_dn14) + locals.var_q_s0_dps__blk1127_dn14),)
    } else {
        (locals.var_pf11__blk1103, locals.var_pf11__blk1103_dn0, locals.var_pf11__blk1103_dn2, locals.var_pf11__blk1103_dn4, locals.var_pf11__blk1103_dn5, locals.var_pf11__blk1103_dn6, locals.var_pf11__blk1103_dn7, locals.var_pf11__blk1103_dn8, locals.var_pf11__blk1103_dn9, locals.var_pf11__blk1103_dn10, locals.var_pf11__blk1103_dn11, locals.var_pf11__blk1103_dn14,)
    }
};
            locals.var_pf11__blk1103 = assign54030_body12_e83013;
            locals.var_pf11__blk1103_dn0 = assign54030_body12_e83013_d_n0;
            locals.var_pf11__blk1103_dn2 = assign54030_body12_e83013_d_n2;
            locals.var_pf11__blk1103_dn4 = assign54030_body12_e83013_d_n4;
            locals.var_pf11__blk1103_dn5 = assign54030_body12_e83013_d_n5;
            locals.var_pf11__blk1103_dn6 = assign54030_body12_e83013_d_n6;
            locals.var_pf11__blk1103_dn7 = assign54030_body12_e83013_d_n7;
            locals.var_pf11__blk1103_dn8 = assign54030_body12_e83013_d_n8;
            locals.var_pf11__blk1103_dn9 = assign54030_body12_e83013_d_n9;
            locals.var_pf11__blk1103_dn10 = assign54030_body12_e83013_d_n10;
            locals.var_pf11__blk1103_dn11 = assign54030_body12_e83013_d_n11;
            locals.var_pf11__blk1103_dn14 = assign54030_body12_e83013_d_n14;
            locals.var_pf11__blk1103_rv = 0.0;
            let (assign54030_body13_e83033, assign54030_body13_e83033_d_n0, assign54030_body13_e83033_d_n2, assign54030_body13_e83033_d_n4, assign54030_body13_e83033_d_n5, assign54030_body13_e83033_d_n6, assign54030_body13_e83033_d_n7, assign54030_body13_e83033_d_n8, assign54030_body13_e83033_d_n9, assign54030_body13_e83033_d_n10, assign54030_body13_e83033_d_n11, assign54030_body13_e83033_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54030_body13_e83029: f64 = (-locals.var_pf1__blk1102);
        let assign54030_body13_e83031: f64 = (assign54030_body13_e83029 / locals.var_pf11__blk1103);
        (assign54030_body13_e83031, ((((-locals.var_pf1__blk1102_dn0) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn0)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn2) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn2)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn4) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn4)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn5) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn5)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn6) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn6)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn7) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn7)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn8) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn8)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn9) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn9)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn10) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn10)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn11) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn11)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn14) * locals.var_pf11__blk1103) - (assign54030_body13_e83029 * locals.var_pf11__blk1103_dn14)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)),)
    } else {
        (locals.var_dps__blk1114, locals.var_dps__blk1114_dn0, locals.var_dps__blk1114_dn2, locals.var_dps__blk1114_dn4, locals.var_dps__blk1114_dn5, locals.var_dps__blk1114_dn6, locals.var_dps__blk1114_dn7, locals.var_dps__blk1114_dn8, locals.var_dps__blk1114_dn9, locals.var_dps__blk1114_dn10, locals.var_dps__blk1114_dn11, locals.var_dps__blk1114_dn14,)
    }
};
            locals.var_dps__blk1114 = assign54030_body13_e83033;
            locals.var_dps__blk1114_dn0 = assign54030_body13_e83033_d_n0;
            locals.var_dps__blk1114_dn2 = assign54030_body13_e83033_d_n2;
            locals.var_dps__blk1114_dn4 = assign54030_body13_e83033_d_n4;
            locals.var_dps__blk1114_dn5 = assign54030_body13_e83033_d_n5;
            locals.var_dps__blk1114_dn6 = assign54030_body13_e83033_d_n6;
            locals.var_dps__blk1114_dn7 = assign54030_body13_e83033_d_n7;
            locals.var_dps__blk1114_dn8 = assign54030_body13_e83033_d_n8;
            locals.var_dps__blk1114_dn9 = assign54030_body13_e83033_d_n9;
            locals.var_dps__blk1114_dn10 = assign54030_body13_e83033_d_n10;
            locals.var_dps__blk1114_dn11 = assign54030_body13_e83033_d_n11;
            locals.var_dps__blk1114_dn14 = assign54030_body13_e83033_d_n14;
            locals.var_dps__blk1114_rv = 0.0;
            let assign54030_body14_e83035: f64 = (locals.var_dps__blk1114).abs();
            let assign54030_body14_e83038: f64 = (1e-10 * 100.0);
            let assign54030_body14_e83039: f64 = if assign54030_body14_e83035 < assign54030_body14_e83038 { 1.0 } else { 0.0 };
            locals.var_guard1366 = assign54030_body14_e83039;
            locals.var_guard1366_rv = 0.0;
            let (assign54030_body15_e83058,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1366 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign54030_body15_e83058;
            locals.var_flg_conv_rv = 0.0;
            let assign54030_body16_e83061: f64 = if locals.var_dps__blk1114 > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1367 = assign54030_body16_e83061;
            locals.var_guard1367_rv = 0.0;
            let (assign54030_body17_e83083, assign54030_body17_e83083_d_n0, assign54030_body17_e83083_d_n2, assign54030_body17_e83083_d_n4, assign54030_body17_e83083_d_n5, assign54030_body17_e83083_d_n6, assign54030_body17_e83083_d_n7, assign54030_body17_e83083_d_n8, assign54030_body17_e83083_d_n9, assign54030_body17_e83083_d_n10, assign54030_body17_e83083_d_n11, assign54030_body17_e83083_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1366 == 0.0)) && (locals.var_guard1367 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1114, locals.var_dps__blk1114_dn0, locals.var_dps__blk1114_dn2, locals.var_dps__blk1114_dn4, locals.var_dps__blk1114_dn5, locals.var_dps__blk1114_dn6, locals.var_dps__blk1114_dn7, locals.var_dps__blk1114_dn8, locals.var_dps__blk1114_dn9, locals.var_dps__blk1114_dn10, locals.var_dps__blk1114_dn11, locals.var_dps__blk1114_dn14,)
    }
};
            locals.var_dps__blk1114 = assign54030_body17_e83083;
            locals.var_dps__blk1114_dn0 = assign54030_body17_e83083_d_n0;
            locals.var_dps__blk1114_dn2 = assign54030_body17_e83083_d_n2;
            locals.var_dps__blk1114_dn4 = assign54030_body17_e83083_d_n4;
            locals.var_dps__blk1114_dn5 = assign54030_body17_e83083_d_n5;
            locals.var_dps__blk1114_dn6 = assign54030_body17_e83083_d_n6;
            locals.var_dps__blk1114_dn7 = assign54030_body17_e83083_d_n7;
            locals.var_dps__blk1114_dn8 = assign54030_body17_e83083_d_n8;
            locals.var_dps__blk1114_dn9 = assign54030_body17_e83083_d_n9;
            locals.var_dps__blk1114_dn10 = assign54030_body17_e83083_d_n10;
            locals.var_dps__blk1114_dn11 = assign54030_body17_e83083_d_n11;
            locals.var_dps__blk1114_dn14 = assign54030_body17_e83083_d_n14;
            locals.var_dps__blk1114_rv = 0.0;
            let assign54030_body18_e83086: f64 = (-0.1);
            let assign54030_body18_e83087: f64 = if locals.var_dps__blk1114 < assign54030_body18_e83086 { 1.0 } else { 0.0 };
            locals.var_guard1368 = assign54030_body18_e83087;
            locals.var_guard1368_rv = 0.0;
            let (assign54030_body19_e83113, assign54030_body19_e83113_d_n0, assign54030_body19_e83113_d_n2, assign54030_body19_e83113_d_n4, assign54030_body19_e83113_d_n5, assign54030_body19_e83113_d_n6, assign54030_body19_e83113_d_n7, assign54030_body19_e83113_d_n8, assign54030_body19_e83113_d_n9, assign54030_body19_e83113_d_n10, assign54030_body19_e83113_d_n11, assign54030_body19_e83113_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1366 == 0.0)) && (locals.var_guard1367 == 0.0)) && (locals.var_guard1368 != 0.0)) {
        let assign54030_body19_e83111: f64 = (-0.1);
        (assign54030_body19_e83111, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1114, locals.var_dps__blk1114_dn0, locals.var_dps__blk1114_dn2, locals.var_dps__blk1114_dn4, locals.var_dps__blk1114_dn5, locals.var_dps__blk1114_dn6, locals.var_dps__blk1114_dn7, locals.var_dps__blk1114_dn8, locals.var_dps__blk1114_dn9, locals.var_dps__blk1114_dn10, locals.var_dps__blk1114_dn11, locals.var_dps__blk1114_dn14,)
    }
};
            locals.var_dps__blk1114 = assign54030_body19_e83113;
            locals.var_dps__blk1114_dn0 = assign54030_body19_e83113_d_n0;
            locals.var_dps__blk1114_dn2 = assign54030_body19_e83113_d_n2;
            locals.var_dps__blk1114_dn4 = assign54030_body19_e83113_d_n4;
            locals.var_dps__blk1114_dn5 = assign54030_body19_e83113_d_n5;
            locals.var_dps__blk1114_dn6 = assign54030_body19_e83113_d_n6;
            locals.var_dps__blk1114_dn7 = assign54030_body19_e83113_d_n7;
            locals.var_dps__blk1114_dn8 = assign54030_body19_e83113_d_n8;
            locals.var_dps__blk1114_dn9 = assign54030_body19_e83113_d_n9;
            locals.var_dps__blk1114_dn10 = assign54030_body19_e83113_d_n10;
            locals.var_dps__blk1114_dn11 = assign54030_body19_e83113_d_n11;
            locals.var_dps__blk1114_dn14 = assign54030_body19_e83113_d_n14;
            locals.var_dps__blk1114_rv = 0.0;
            let (assign54030_body20_e83132, assign54030_body20_e83132_d_n0, assign54030_body20_e83132_d_n2, assign54030_body20_e83132_d_n4, assign54030_body20_e83132_d_n5, assign54030_body20_e83132_d_n6, assign54030_body20_e83132_d_n7, assign54030_body20_e83132_d_n8, assign54030_body20_e83132_d_n9, assign54030_body20_e83132_d_n10, assign54030_body20_e83132_d_n11, assign54030_body20_e83132_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54030_body20_e83130: f64 = (locals.var_ps0dep + locals.var_dps__blk1114);
        (assign54030_body20_e83130, (locals.var_ps0dep_dn0 + locals.var_dps__blk1114_dn0), (locals.var_ps0dep_dn2 + locals.var_dps__blk1114_dn2), (locals.var_ps0dep_dn4 + locals.var_dps__blk1114_dn4), (locals.var_ps0dep_dn5 + locals.var_dps__blk1114_dn5), (locals.var_ps0dep_dn6 + locals.var_dps__blk1114_dn6), (locals.var_ps0dep_dn7 + locals.var_dps__blk1114_dn7), (locals.var_ps0dep_dn8 + locals.var_dps__blk1114_dn8), (locals.var_ps0dep_dn9 + locals.var_dps__blk1114_dn9), (locals.var_ps0dep_dn10 + locals.var_dps__blk1114_dn10), (locals.var_ps0dep_dn11 + locals.var_dps__blk1114_dn11), (locals.var_ps0dep_dn14 + locals.var_dps__blk1114_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
            locals.var_ps0dep = assign54030_body20_e83132;
            locals.var_ps0dep_dn0 = assign54030_body20_e83132_d_n0;
            locals.var_ps0dep_dn2 = assign54030_body20_e83132_d_n2;
            locals.var_ps0dep_dn4 = assign54030_body20_e83132_d_n4;
            locals.var_ps0dep_dn5 = assign54030_body20_e83132_d_n5;
            locals.var_ps0dep_dn6 = assign54030_body20_e83132_d_n6;
            locals.var_ps0dep_dn7 = assign54030_body20_e83132_d_n7;
            locals.var_ps0dep_dn8 = assign54030_body20_e83132_d_n8;
            locals.var_ps0dep_dn9 = assign54030_body20_e83132_d_n9;
            locals.var_ps0dep_dn10 = assign54030_body20_e83132_d_n10;
            locals.var_ps0dep_dn11 = assign54030_body20_e83132_d_n11;
            locals.var_ps0dep_dn14 = assign54030_body20_e83132_d_n14;
            locals.var_ps0dep_rv = 0.0;
            let (assign54030_body21_e83148,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign54030_body21_e83146: f64 = (locals.var_lp_s0 + 1.0);
        (assign54030_body21_e83146,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign54030_body21_e83148;
            locals.var_lp_s0_rv = 0.0;
        }

        let assign54050_e83154: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1370 = assign54050_e83154;
        locals.var_guard1370_rv = 0.0;

        let (assign54060_e83170, assign54060_e83170_d_n0, assign54060_e83170_d_n2, assign54060_e83170_d_n4, assign54060_e83170_d_n5, assign54060_e83170_d_n6, assign54060_e83170_d_n7, assign54060_e83170_d_n8, assign54060_e83170_d_n9, assign54060_e83170_d_n10, assign54060_e83170_d_n11, assign54060_e83170_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 != 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0dep0, locals.var_ps0dep0_dn0, locals.var_ps0dep0_dn2, locals.var_ps0dep0_dn4, locals.var_ps0dep0_dn5, locals.var_ps0dep0_dn6, locals.var_ps0dep0_dn7, locals.var_ps0dep0_dn8, locals.var_ps0dep0_dn9, locals.var_ps0dep0_dn10, locals.var_ps0dep0_dn11, locals.var_ps0dep0_dn14,)
    }
};
        locals.var_ps0dep0 = assign54060_e83170;
        locals.var_ps0dep0_dn0 = assign54060_e83170_d_n0;
        locals.var_ps0dep0_dn2 = assign54060_e83170_d_n2;
        locals.var_ps0dep0_dn4 = assign54060_e83170_d_n4;
        locals.var_ps0dep0_dn5 = assign54060_e83170_d_n5;
        locals.var_ps0dep0_dn6 = assign54060_e83170_d_n6;
        locals.var_ps0dep0_dn7 = assign54060_e83170_d_n7;
        locals.var_ps0dep0_dn8 = assign54060_e83170_d_n8;
        locals.var_ps0dep0_dn9 = assign54060_e83170_d_n9;
        locals.var_ps0dep0_dn10 = assign54060_e83170_d_n10;
        locals.var_ps0dep0_dn11 = assign54060_e83170_d_n11;
        locals.var_ps0dep0_dn14 = assign54060_e83170_d_n14;
        locals.var_ps0dep0_rv = 0.0;

        let assign54070_e83174: f64 = (locals.var_ps0dep0 + 0.2);
        let assign54070_e83179: f64 = if ((locals.var_ps0dep < assign54070_e83174) && (0.2 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1371 = assign54070_e83179;
        locals.var_guard1371_rv = 0.0;

        let (assign54080_e83202, assign54080_e83202_d_n0, assign54080_e83202_d_n2, assign54080_e83202_d_n4, assign54080_e83202_d_n5, assign54080_e83202_d_n6, assign54080_e83202_d_n7, assign54080_e83202_d_n8, assign54080_e83202_d_n9, assign54080_e83202_d_n10, assign54080_e83202_d_n11, assign54080_e83202_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54080_e83198: f64 = (locals.var_ps0dep0 + 0.2);
        let assign54080_e83200: f64 = (assign54080_e83198 - locals.var_ps0dep);
        (assign54080_e83200, (locals.var_ps0dep0_dn0 - locals.var_ps0dep_dn0), (locals.var_ps0dep0_dn2 - locals.var_ps0dep_dn2), (locals.var_ps0dep0_dn4 - locals.var_ps0dep_dn4), (locals.var_ps0dep0_dn5 - locals.var_ps0dep_dn5), (locals.var_ps0dep0_dn6 - locals.var_ps0dep_dn6), (locals.var_ps0dep0_dn7 - locals.var_ps0dep_dn7), (locals.var_ps0dep0_dn8 - locals.var_ps0dep_dn8), (locals.var_ps0dep0_dn9 - locals.var_ps0dep_dn9), (locals.var_ps0dep0_dn10 - locals.var_ps0dep_dn10), (locals.var_ps0dep0_dn11 - locals.var_ps0dep_dn11), (locals.var_ps0dep0_dn14 - locals.var_ps0dep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign54080_e83202;
        locals.var_tmf1_dn0 = assign54080_e83202_d_n0;
        locals.var_tmf1_dn2 = assign54080_e83202_d_n2;
        locals.var_tmf1_dn4 = assign54080_e83202_d_n4;
        locals.var_tmf1_dn5 = assign54080_e83202_d_n5;
        locals.var_tmf1_dn6 = assign54080_e83202_d_n6;
        locals.var_tmf1_dn7 = assign54080_e83202_d_n7;
        locals.var_tmf1_dn8 = assign54080_e83202_d_n8;
        locals.var_tmf1_dn9 = assign54080_e83202_d_n9;
        locals.var_tmf1_dn10 = assign54080_e83202_d_n10;
        locals.var_tmf1_dn11 = assign54080_e83202_d_n11;
        locals.var_tmf1_dn14 = assign54080_e83202_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign54090_e83223, assign54090_e83223_d_n0, assign54090_e83223_d_n2, assign54090_e83223_d_n4, assign54090_e83223_d_n5, assign54090_e83223_d_n6, assign54090_e83223_d_n7, assign54090_e83223_d_n8, assign54090_e83223_d_n9, assign54090_e83223_d_n10, assign54090_e83223_d_n11, assign54090_e83223_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54090_e83221: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign54090_e83221, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign54090_e83223;
        locals.var_x2_dn0 = assign54090_e83223_d_n0;
        locals.var_x2_dn2 = assign54090_e83223_d_n2;
        locals.var_x2_dn4 = assign54090_e83223_d_n4;
        locals.var_x2_dn5 = assign54090_e83223_d_n5;
        locals.var_x2_dn6 = assign54090_e83223_d_n6;
        locals.var_x2_dn7 = assign54090_e83223_d_n7;
        locals.var_x2_dn8 = assign54090_e83223_d_n8;
        locals.var_x2_dn9 = assign54090_e83223_d_n9;
        locals.var_x2_dn10 = assign54090_e83223_d_n10;
        locals.var_x2_dn11 = assign54090_e83223_d_n11;
        locals.var_x2_dn14 = assign54090_e83223_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign54100_e83244, assign54100_e83244_d_n0, assign54100_e83244_d_n2, assign54100_e83244_d_n4, assign54100_e83244_d_n5, assign54100_e83244_d_n6, assign54100_e83244_d_n7, assign54100_e83244_d_n8, assign54100_e83244_d_n9, assign54100_e83244_d_n10, assign54100_e83244_d_n11, assign54100_e83244_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54100_e83242: f64 = (0.2 * 0.2);
        (assign54100_e83242, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign54100_e83244;
        locals.var_xmax2_dn0 = assign54100_e83244_d_n0;
        locals.var_xmax2_dn2 = assign54100_e83244_d_n2;
        locals.var_xmax2_dn4 = assign54100_e83244_d_n4;
        locals.var_xmax2_dn5 = assign54100_e83244_d_n5;
        locals.var_xmax2_dn6 = assign54100_e83244_d_n6;
        locals.var_xmax2_dn7 = assign54100_e83244_d_n7;
        locals.var_xmax2_dn8 = assign54100_e83244_d_n8;
        locals.var_xmax2_dn9 = assign54100_e83244_d_n9;
        locals.var_xmax2_dn10 = assign54100_e83244_d_n10;
        locals.var_xmax2_dn11 = assign54100_e83244_d_n11;
        locals.var_xmax2_dn14 = assign54100_e83244_d_n14;
        locals.var_xmax2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_197(
        locals: &mut StampLocals,
    ) {
        let (assign54110_e83263, assign54110_e83263_d_n0, assign54110_e83263_d_n2, assign54110_e83263_d_n4, assign54110_e83263_d_n5, assign54110_e83263_d_n6, assign54110_e83263_d_n7, assign54110_e83263_d_n8, assign54110_e83263_d_n9, assign54110_e83263_d_n10, assign54110_e83263_d_n11, assign54110_e83263_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign54110_e83263;
        locals.var_xp_dn0 = assign54110_e83263_d_n0;
        locals.var_xp_dn2 = assign54110_e83263_d_n2;
        locals.var_xp_dn4 = assign54110_e83263_d_n4;
        locals.var_xp_dn5 = assign54110_e83263_d_n5;
        locals.var_xp_dn6 = assign54110_e83263_d_n6;
        locals.var_xp_dn7 = assign54110_e83263_d_n7;
        locals.var_xp_dn8 = assign54110_e83263_d_n8;
        locals.var_xp_dn9 = assign54110_e83263_d_n9;
        locals.var_xp_dn10 = assign54110_e83263_d_n10;
        locals.var_xp_dn11 = assign54110_e83263_d_n11;
        locals.var_xp_dn14 = assign54110_e83263_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign54120_e83282, assign54120_e83282_d_n0, assign54120_e83282_d_n2, assign54120_e83282_d_n4, assign54120_e83282_d_n5, assign54120_e83282_d_n6, assign54120_e83282_d_n7, assign54120_e83282_d_n8, assign54120_e83282_d_n9, assign54120_e83282_d_n10, assign54120_e83282_d_n11, assign54120_e83282_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign54120_e83282;
        locals.var_xmp_dn0 = assign54120_e83282_d_n0;
        locals.var_xmp_dn2 = assign54120_e83282_d_n2;
        locals.var_xmp_dn4 = assign54120_e83282_d_n4;
        locals.var_xmp_dn5 = assign54120_e83282_d_n5;
        locals.var_xmp_dn6 = assign54120_e83282_d_n6;
        locals.var_xmp_dn7 = assign54120_e83282_d_n7;
        locals.var_xmp_dn8 = assign54120_e83282_d_n8;
        locals.var_xmp_dn9 = assign54120_e83282_d_n9;
        locals.var_xmp_dn10 = assign54120_e83282_d_n10;
        locals.var_xmp_dn11 = assign54120_e83282_d_n11;
        locals.var_xmp_dn14 = assign54120_e83282_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign54130_e83301,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54130_e83301;
        locals.var_m0_rv = 0.0;

        let (assign54140_e83320,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54140_e83320;
        locals.var_mm_rv = 0.0;

        let (assign54150_e83339, assign54150_e83339_d_n0, assign54150_e83339_d_n2, assign54150_e83339_d_n4, assign54150_e83339_d_n5, assign54150_e83339_d_n6, assign54150_e83339_d_n7, assign54150_e83339_d_n8, assign54150_e83339_d_n9, assign54150_e83339_d_n10, assign54150_e83339_d_n11, assign54150_e83339_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign54150_e83339;
        locals.var_arg_dn0 = assign54150_e83339_d_n0;
        locals.var_arg_dn2 = assign54150_e83339_d_n2;
        locals.var_arg_dn4 = assign54150_e83339_d_n4;
        locals.var_arg_dn5 = assign54150_e83339_d_n5;
        locals.var_arg_dn6 = assign54150_e83339_d_n6;
        locals.var_arg_dn7 = assign54150_e83339_d_n7;
        locals.var_arg_dn8 = assign54150_e83339_d_n8;
        locals.var_arg_dn9 = assign54150_e83339_d_n9;
        locals.var_arg_dn10 = assign54150_e83339_d_n10;
        locals.var_arg_dn11 = assign54150_e83339_d_n11;
        locals.var_arg_dn14 = assign54150_e83339_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign54160_e83358, assign54160_e83358_d_n0, assign54160_e83358_d_n2, assign54160_e83358_d_n4, assign54160_e83358_d_n5, assign54160_e83358_d_n6, assign54160_e83358_d_n7, assign54160_e83358_d_n8, assign54160_e83358_d_n9, assign54160_e83358_d_n10, assign54160_e83358_d_n11, assign54160_e83358_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54160_e83358;
        locals.var_dnm_dn0 = assign54160_e83358_d_n0;
        locals.var_dnm_dn2 = assign54160_e83358_d_n2;
        locals.var_dnm_dn4 = assign54160_e83358_d_n4;
        locals.var_dnm_dn5 = assign54160_e83358_d_n5;
        locals.var_dnm_dn6 = assign54160_e83358_d_n6;
        locals.var_dnm_dn7 = assign54160_e83358_d_n7;
        locals.var_dnm_dn8 = assign54160_e83358_d_n8;
        locals.var_dnm_dn9 = assign54160_e83358_d_n9;
        locals.var_dnm_dn10 = assign54160_e83358_d_n10;
        locals.var_dnm_dn11 = assign54160_e83358_d_n11;
        locals.var_dnm_dn14 = assign54160_e83358_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign54170_e83379, assign54170_e83379_d_n0, assign54170_e83379_d_n2, assign54170_e83379_d_n4, assign54170_e83379_d_n5, assign54170_e83379_d_n6, assign54170_e83379_d_n7, assign54170_e83379_d_n8, assign54170_e83379_d_n9, assign54170_e83379_d_n10, assign54170_e83379_d_n11, assign54170_e83379_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54170_e83377: f64 = (locals.var_xp * locals.var_x2);
        (assign54170_e83377, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign54170_e83379;
        locals.var_xp_dn0 = assign54170_e83379_d_n0;
        locals.var_xp_dn2 = assign54170_e83379_d_n2;
        locals.var_xp_dn4 = assign54170_e83379_d_n4;
        locals.var_xp_dn5 = assign54170_e83379_d_n5;
        locals.var_xp_dn6 = assign54170_e83379_d_n6;
        locals.var_xp_dn7 = assign54170_e83379_d_n7;
        locals.var_xp_dn8 = assign54170_e83379_d_n8;
        locals.var_xp_dn9 = assign54170_e83379_d_n9;
        locals.var_xp_dn10 = assign54170_e83379_d_n10;
        locals.var_xp_dn11 = assign54170_e83379_d_n11;
        locals.var_xp_dn14 = assign54170_e83379_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign54180_e83400, assign54180_e83400_d_n0, assign54180_e83400_d_n2, assign54180_e83400_d_n4, assign54180_e83400_d_n5, assign54180_e83400_d_n6, assign54180_e83400_d_n7, assign54180_e83400_d_n8, assign54180_e83400_d_n9, assign54180_e83400_d_n10, assign54180_e83400_d_n11, assign54180_e83400_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54180_e83398: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign54180_e83398, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign54180_e83400;
        locals.var_xmp_dn0 = assign54180_e83400_d_n0;
        locals.var_xmp_dn2 = assign54180_e83400_d_n2;
        locals.var_xmp_dn4 = assign54180_e83400_d_n4;
        locals.var_xmp_dn5 = assign54180_e83400_d_n5;
        locals.var_xmp_dn6 = assign54180_e83400_d_n6;
        locals.var_xmp_dn7 = assign54180_e83400_d_n7;
        locals.var_xmp_dn8 = assign54180_e83400_d_n8;
        locals.var_xmp_dn9 = assign54180_e83400_d_n9;
        locals.var_xmp_dn10 = assign54180_e83400_d_n10;
        locals.var_xmp_dn11 = assign54180_e83400_d_n11;
        locals.var_xmp_dn14 = assign54180_e83400_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign54190_e83421, assign54190_e83421_d_n0, assign54190_e83421_d_n2, assign54190_e83421_d_n4, assign54190_e83421_d_n5, assign54190_e83421_d_n6, assign54190_e83421_d_n7, assign54190_e83421_d_n8, assign54190_e83421_d_n9, assign54190_e83421_d_n10, assign54190_e83421_d_n11, assign54190_e83421_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54190_e83419: f64 = (locals.var_xp * locals.var_x2);
        (assign54190_e83419, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign54190_e83421;
        locals.var_xp_dn0 = assign54190_e83421_d_n0;
        locals.var_xp_dn2 = assign54190_e83421_d_n2;
        locals.var_xp_dn4 = assign54190_e83421_d_n4;
        locals.var_xp_dn5 = assign54190_e83421_d_n5;
        locals.var_xp_dn6 = assign54190_e83421_d_n6;
        locals.var_xp_dn7 = assign54190_e83421_d_n7;
        locals.var_xp_dn8 = assign54190_e83421_d_n8;
        locals.var_xp_dn9 = assign54190_e83421_d_n9;
        locals.var_xp_dn10 = assign54190_e83421_d_n10;
        locals.var_xp_dn11 = assign54190_e83421_d_n11;
        locals.var_xp_dn14 = assign54190_e83421_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign54200_e83442, assign54200_e83442_d_n0, assign54200_e83442_d_n2, assign54200_e83442_d_n4, assign54200_e83442_d_n5, assign54200_e83442_d_n6, assign54200_e83442_d_n7, assign54200_e83442_d_n8, assign54200_e83442_d_n9, assign54200_e83442_d_n10, assign54200_e83442_d_n11, assign54200_e83442_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54200_e83440: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign54200_e83440, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign54200_e83442;
        locals.var_xmp_dn0 = assign54200_e83442_d_n0;
        locals.var_xmp_dn2 = assign54200_e83442_d_n2;
        locals.var_xmp_dn4 = assign54200_e83442_d_n4;
        locals.var_xmp_dn5 = assign54200_e83442_d_n5;
        locals.var_xmp_dn6 = assign54200_e83442_d_n6;
        locals.var_xmp_dn7 = assign54200_e83442_d_n7;
        locals.var_xmp_dn8 = assign54200_e83442_d_n8;
        locals.var_xmp_dn9 = assign54200_e83442_d_n9;
        locals.var_xmp_dn10 = assign54200_e83442_d_n10;
        locals.var_xmp_dn11 = assign54200_e83442_d_n11;
        locals.var_xmp_dn14 = assign54200_e83442_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign54210_e83463, assign54210_e83463_d_n0, assign54210_e83463_d_n2, assign54210_e83463_d_n4, assign54210_e83463_d_n5, assign54210_e83463_d_n6, assign54210_e83463_d_n7, assign54210_e83463_d_n8, assign54210_e83463_d_n9, assign54210_e83463_d_n10, assign54210_e83463_d_n11, assign54210_e83463_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54210_e83461: f64 = (locals.var_xp + locals.var_xmp);
        (assign54210_e83461, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign54210_e83463;
        locals.var_arg_dn0 = assign54210_e83463_d_n0;
        locals.var_arg_dn2 = assign54210_e83463_d_n2;
        locals.var_arg_dn4 = assign54210_e83463_d_n4;
        locals.var_arg_dn5 = assign54210_e83463_d_n5;
        locals.var_arg_dn6 = assign54210_e83463_d_n6;
        locals.var_arg_dn7 = assign54210_e83463_d_n7;
        locals.var_arg_dn8 = assign54210_e83463_d_n8;
        locals.var_arg_dn9 = assign54210_e83463_d_n9;
        locals.var_arg_dn10 = assign54210_e83463_d_n10;
        locals.var_arg_dn11 = assign54210_e83463_d_n11;
        locals.var_arg_dn14 = assign54210_e83463_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign54220_e83482, assign54220_e83482_d_n0, assign54220_e83482_d_n2, assign54220_e83482_d_n4, assign54220_e83482_d_n5, assign54220_e83482_d_n6, assign54220_e83482_d_n7, assign54220_e83482_d_n8, assign54220_e83482_d_n9, assign54220_e83482_d_n10, assign54220_e83482_d_n11, assign54220_e83482_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54220_e83482;
        locals.var_dnm_dn0 = assign54220_e83482_d_n0;
        locals.var_dnm_dn2 = assign54220_e83482_d_n2;
        locals.var_dnm_dn4 = assign54220_e83482_d_n4;
        locals.var_dnm_dn5 = assign54220_e83482_d_n5;
        locals.var_dnm_dn6 = assign54220_e83482_d_n6;
        locals.var_dnm_dn7 = assign54220_e83482_d_n7;
        locals.var_dnm_dn8 = assign54220_e83482_d_n8;
        locals.var_dnm_dn9 = assign54220_e83482_d_n9;
        locals.var_dnm_dn10 = assign54220_e83482_d_n10;
        locals.var_dnm_dn11 = assign54220_e83482_d_n11;
        locals.var_dnm_dn14 = assign54220_e83482_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign54230_e83497: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1372 = assign54230_e83497;
        locals.var_guard1372_rv = 0.0;

        let assign54240_e83500: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1373 = assign54240_e83500;
        locals.var_guard1373_rv = 0.0;

        let (assign54250_e83523,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) && (locals.var_guard1372 != 0.0)) && (locals.var_guard1373 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54250_e83523;
        locals.var_mm_rv = 0.0;

        let assign54260_e83526: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1374 = assign54260_e83526;
        locals.var_guard1374_rv = 0.0;

        let (assign54270_e83552,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) && (locals.var_guard1372 != 0.0)) && (locals.var_guard1373 == 0.0)) && (locals.var_guard1374 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54270_e83552;
        locals.var_mm_rv = 0.0;

        let assign54280_e83555: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1375 = assign54280_e83555;
        locals.var_guard1375_rv = 0.0;

        let (assign54290_e83584,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) && (locals.var_guard1372 != 0.0)) && (locals.var_guard1373 == 0.0)) && (locals.var_guard1374 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54290_e83584;
        locals.var_mm_rv = 0.0;

        let assign54300_e83587: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1376 = assign54300_e83587;
        locals.var_guard1376_rv = 0.0;

        let (assign54310_e83619,) = {
    if ((((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) && (locals.var_guard1372 != 0.0)) && (locals.var_guard1373 == 0.0)) && (locals.var_guard1374 == 0.0)) && (locals.var_guard1375 == 0.0)) && (locals.var_guard1376 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54310_e83619;
        locals.var_mm_rv = 0.0;

        let (assign54320_e83640,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) && (locals.var_guard1372 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54320_e83640;
        locals.var_m0_rv = 0.0;

        let mut assign54330_loop_guard: usize = 0;
        while {
            let assign54330_cond_e83662: f64 = if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) && (locals.var_guard1372 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign54330_cond_e83662 != 0.0
        } {
            assign54330_loop_guard += 1;
            assert!(assign54330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54330_body0_e83684, assign54330_body0_e83684_d_n0, assign54330_body0_e83684_d_n2, assign54330_body0_e83684_d_n4, assign54330_body0_e83684_d_n5, assign54330_body0_e83684_d_n6, assign54330_body0_e83684_d_n7, assign54330_body0_e83684_d_n8, assign54330_body0_e83684_d_n9, assign54330_body0_e83684_d_n10, assign54330_body0_e83684_d_n11, assign54330_body0_e83684_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) && (locals.var_guard1372 != 0.0)) {
        let assign54330_body0_e83682: f64 = (locals.var_dnm).sqrt();
        (assign54330_body0_e83682, (locals.var_dnm_dn0 / (2.0 * assign54330_body0_e83682)), (locals.var_dnm_dn2 / (2.0 * assign54330_body0_e83682)), (locals.var_dnm_dn4 / (2.0 * assign54330_body0_e83682)), (locals.var_dnm_dn5 / (2.0 * assign54330_body0_e83682)), (locals.var_dnm_dn6 / (2.0 * assign54330_body0_e83682)), (locals.var_dnm_dn7 / (2.0 * assign54330_body0_e83682)), (locals.var_dnm_dn8 / (2.0 * assign54330_body0_e83682)), (locals.var_dnm_dn9 / (2.0 * assign54330_body0_e83682)), (locals.var_dnm_dn10 / (2.0 * assign54330_body0_e83682)), (locals.var_dnm_dn11 / (2.0 * assign54330_body0_e83682)), (locals.var_dnm_dn14 / (2.0 * assign54330_body0_e83682)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign54330_body0_e83684;
            locals.var_dnm_dn0 = assign54330_body0_e83684_d_n0;
            locals.var_dnm_dn2 = assign54330_body0_e83684_d_n2;
            locals.var_dnm_dn4 = assign54330_body0_e83684_d_n4;
            locals.var_dnm_dn5 = assign54330_body0_e83684_d_n5;
            locals.var_dnm_dn6 = assign54330_body0_e83684_d_n6;
            locals.var_dnm_dn7 = assign54330_body0_e83684_d_n7;
            locals.var_dnm_dn8 = assign54330_body0_e83684_d_n8;
            locals.var_dnm_dn9 = assign54330_body0_e83684_d_n9;
            locals.var_dnm_dn10 = assign54330_body0_e83684_d_n10;
            locals.var_dnm_dn11 = assign54330_body0_e83684_d_n11;
            locals.var_dnm_dn14 = assign54330_body0_e83684_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign54330_body1_e83707,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) && (locals.var_guard1372 != 0.0)) {
        let assign54330_body1_e83705: f64 = (locals.var_m0 + 1.0);
        (assign54330_body1_e83705,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign54330_body1_e83707;
            locals.var_m0_rv = 0.0;
        }

        let (assign54340_e83740, assign54340_e83740_d_n0, assign54340_e83740_d_n2, assign54340_e83740_d_n4, assign54340_e83740_d_n5, assign54340_e83740_d_n6, assign54340_e83740_d_n7, assign54340_e83740_d_n8, assign54340_e83740_d_n9, assign54340_e83740_d_n10, assign54340_e83740_d_n11, assign54340_e83740_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) && (locals.var_guard1372 == 0.0)) {
        let (assign54340_e83738, assign54340_e83738_d_n0, assign54340_e83738_d_n2, assign54340_e83738_d_n4, assign54340_e83738_d_n5, assign54340_e83738_d_n6, assign54340_e83738_d_n7, assign54340_e83738_d_n8, assign54340_e83738_d_n9, assign54340_e83738_d_n10, assign54340_e83738_d_n11, assign54340_e83738_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign54340_e83735: f64 = (2.0 * 2.0);
                let assign54340_e83736: f64 = (1.0 / assign54340_e83735);
                let assign54340_e83737: f64 = (locals.var_dnm).powf(assign54340_e83736);
                (assign54340_e83737, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn0)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn2)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn4)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn5)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn6)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn7)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn8)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn9)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn10)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn11)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54340_e83736) as f64).is_finite() && ((assign54340_e83736) as f64).fract() == 0.0 { if assign54340_e83736 == 0.0 { 0.0 } else { (assign54340_e83736 * ((locals.var_dnm).powf(assign54340_e83736 - 1.0) * locals.var_dnm_dn14)) } } else { (assign54340_e83737 * (assign54340_e83736 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign54340_e83738, assign54340_e83738_d_n0, assign54340_e83738_d_n2, assign54340_e83738_d_n4, assign54340_e83738_d_n5, assign54340_e83738_d_n6, assign54340_e83738_d_n7, assign54340_e83738_d_n8, assign54340_e83738_d_n9, assign54340_e83738_d_n10, assign54340_e83738_d_n11, assign54340_e83738_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54340_e83740;
        locals.var_dnm_dn0 = assign54340_e83740_d_n0;
        locals.var_dnm_dn2 = assign54340_e83740_d_n2;
        locals.var_dnm_dn4 = assign54340_e83740_d_n4;
        locals.var_dnm_dn5 = assign54340_e83740_d_n5;
        locals.var_dnm_dn6 = assign54340_e83740_d_n6;
        locals.var_dnm_dn7 = assign54340_e83740_d_n7;
        locals.var_dnm_dn8 = assign54340_e83740_d_n8;
        locals.var_dnm_dn9 = assign54340_e83740_d_n9;
        locals.var_dnm_dn10 = assign54340_e83740_d_n10;
        locals.var_dnm_dn11 = assign54340_e83740_d_n11;
        locals.var_dnm_dn14 = assign54340_e83740_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign54350_e83761, assign54350_e83761_d_n0, assign54350_e83761_d_n2, assign54350_e83761_d_n4, assign54350_e83761_d_n5, assign54350_e83761_d_n6, assign54350_e83761_d_n7, assign54350_e83761_d_n8, assign54350_e83761_d_n9, assign54350_e83761_d_n10, assign54350_e83761_d_n11, assign54350_e83761_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54350_e83759: f64 = (1.0 / locals.var_dnm);
        (assign54350_e83759, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54350_e83761;
        locals.var_dnm_dn0 = assign54350_e83761_d_n0;
        locals.var_dnm_dn2 = assign54350_e83761_d_n2;
        locals.var_dnm_dn4 = assign54350_e83761_d_n4;
        locals.var_dnm_dn5 = assign54350_e83761_d_n5;
        locals.var_dnm_dn6 = assign54350_e83761_d_n6;
        locals.var_dnm_dn7 = assign54350_e83761_d_n7;
        locals.var_dnm_dn8 = assign54350_e83761_d_n8;
        locals.var_dnm_dn9 = assign54350_e83761_d_n9;
        locals.var_dnm_dn10 = assign54350_e83761_d_n10;
        locals.var_dnm_dn11 = assign54350_e83761_d_n11;
        locals.var_dnm_dn14 = assign54350_e83761_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign54360_e83784, assign54360_e83784_d_n0, assign54360_e83784_d_n2, assign54360_e83784_d_n4, assign54360_e83784_d_n5, assign54360_e83784_d_n6, assign54360_e83784_d_n7, assign54360_e83784_d_n8, assign54360_e83784_d_n9, assign54360_e83784_d_n10, assign54360_e83784_d_n11, assign54360_e83784_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54360_e83780: f64 = (locals.var_tmf1 * 0.2);
        let assign54360_e83782: f64 = (assign54360_e83780 * locals.var_dnm);
        (assign54360_e83782, (((locals.var_tmf1_dn0 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.2) * locals.var_dnm) + (assign54360_e83780 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign54360_e83784;
        locals.var_tmf0_dn0 = assign54360_e83784_d_n0;
        locals.var_tmf0_dn2 = assign54360_e83784_d_n2;
        locals.var_tmf0_dn4 = assign54360_e83784_d_n4;
        locals.var_tmf0_dn5 = assign54360_e83784_d_n5;
        locals.var_tmf0_dn6 = assign54360_e83784_d_n6;
        locals.var_tmf0_dn7 = assign54360_e83784_d_n7;
        locals.var_tmf0_dn8 = assign54360_e83784_d_n8;
        locals.var_tmf0_dn9 = assign54360_e83784_d_n9;
        locals.var_tmf0_dn10 = assign54360_e83784_d_n10;
        locals.var_tmf0_dn11 = assign54360_e83784_d_n11;
        locals.var_tmf0_dn14 = assign54360_e83784_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign54370_e83809, assign54370_e83809_d_n0, assign54370_e83809_d_n2, assign54370_e83809_d_n4, assign54370_e83809_d_n5, assign54370_e83809_d_n6, assign54370_e83809_d_n7, assign54370_e83809_d_n8, assign54370_e83809_d_n9, assign54370_e83809_d_n10, assign54370_e83809_d_n11, assign54370_e83809_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54370_e83803: f64 = (0.2 * locals.var_xmp);
        let assign54370_e83805: f64 = (assign54370_e83803 * locals.var_dnm);
        let assign54370_e83807: f64 = (assign54370_e83805 / locals.var_arg);
        (assign54370_e83807, ((((((0.2 * locals.var_xmp_dn0) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn0)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn2) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn2)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn4) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn4)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn5) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn5)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn6) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn6)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn7) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn7)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn8) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn8)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn9) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn9)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn10) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn10)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn11) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn11)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn14) * locals.var_dnm) + (assign54370_e83803 * locals.var_dnm_dn14)) * locals.var_arg) - (assign54370_e83805 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54370_e83809;
        locals.var_t0_dn0 = assign54370_e83809_d_n0;
        locals.var_t0_dn2 = assign54370_e83809_d_n2;
        locals.var_t0_dn4 = assign54370_e83809_d_n4;
        locals.var_t0_dn5 = assign54370_e83809_d_n5;
        locals.var_t0_dn6 = assign54370_e83809_d_n6;
        locals.var_t0_dn7 = assign54370_e83809_d_n7;
        locals.var_t0_dn8 = assign54370_e83809_d_n8;
        locals.var_t0_dn9 = assign54370_e83809_d_n9;
        locals.var_t0_dn10 = assign54370_e83809_d_n10;
        locals.var_t0_dn11 = assign54370_e83809_d_n11;
        locals.var_t0_dn14 = assign54370_e83809_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign54380_e83832, assign54380_e83832_d_n0, assign54380_e83832_d_n2, assign54380_e83832_d_n4, assign54380_e83832_d_n5, assign54380_e83832_d_n6, assign54380_e83832_d_n7, assign54380_e83832_d_n8, assign54380_e83832_d_n9, assign54380_e83832_d_n10, assign54380_e83832_d_n11, assign54380_e83832_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign54380_e83828: f64 = (locals.var_ps0dep0 + 0.2);
        let assign54380_e83830: f64 = (assign54380_e83828 - locals.var_tmf0);
        (assign54380_e83830, (locals.var_ps0dep0_dn0 - locals.var_tmf0_dn0), (locals.var_ps0dep0_dn2 - locals.var_tmf0_dn2), (locals.var_ps0dep0_dn4 - locals.var_tmf0_dn4), (locals.var_ps0dep0_dn5 - locals.var_tmf0_dn5), (locals.var_ps0dep0_dn6 - locals.var_tmf0_dn6), (locals.var_ps0dep0_dn7 - locals.var_tmf0_dn7), (locals.var_ps0dep0_dn8 - locals.var_tmf0_dn8), (locals.var_ps0dep0_dn9 - locals.var_tmf0_dn9), (locals.var_ps0dep0_dn10 - locals.var_tmf0_dn10), (locals.var_ps0dep0_dn11 - locals.var_tmf0_dn11), (locals.var_ps0dep0_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54380_e83832;
        locals.var_ps0dep_dn0 = assign54380_e83832_d_n0;
        locals.var_ps0dep_dn2 = assign54380_e83832_d_n2;
        locals.var_ps0dep_dn4 = assign54380_e83832_d_n4;
        locals.var_ps0dep_dn5 = assign54380_e83832_d_n5;
        locals.var_ps0dep_dn6 = assign54380_e83832_d_n6;
        locals.var_ps0dep_dn7 = assign54380_e83832_d_n7;
        locals.var_ps0dep_dn8 = assign54380_e83832_d_n8;
        locals.var_ps0dep_dn9 = assign54380_e83832_d_n9;
        locals.var_ps0dep_dn10 = assign54380_e83832_d_n10;
        locals.var_ps0dep_dn11 = assign54380_e83832_d_n11;
        locals.var_ps0dep_dn14 = assign54380_e83832_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign54390_e83851, assign54390_e83851_d_n0, assign54390_e83851_d_n2, assign54390_e83851_d_n4, assign54390_e83851_d_n5, assign54390_e83851_d_n6, assign54390_e83851_d_n7, assign54390_e83851_d_n8, assign54390_e83851_d_n9, assign54390_e83851_d_n10, assign54390_e83851_d_n11, assign54390_e83851_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54390_e83851;
        locals.var_t0_dn0 = assign54390_e83851_d_n0;
        locals.var_t0_dn2 = assign54390_e83851_d_n2;
        locals.var_t0_dn4 = assign54390_e83851_d_n4;
        locals.var_t0_dn5 = assign54390_e83851_d_n5;
        locals.var_t0_dn6 = assign54390_e83851_d_n6;
        locals.var_t0_dn7 = assign54390_e83851_d_n7;
        locals.var_t0_dn8 = assign54390_e83851_d_n8;
        locals.var_t0_dn9 = assign54390_e83851_d_n9;
        locals.var_t0_dn10 = assign54390_e83851_d_n10;
        locals.var_t0_dn11 = assign54390_e83851_d_n11;
        locals.var_t0_dn14 = assign54390_e83851_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign54400_e83871, assign54400_e83871_d_n0, assign54400_e83871_d_n2, assign54400_e83871_d_n4, assign54400_e83871_d_n5, assign54400_e83871_d_n6, assign54400_e83871_d_n7, assign54400_e83871_d_n8, assign54400_e83871_d_n9, assign54400_e83871_d_n10, assign54400_e83871_d_n11, assign54400_e83871_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54400_e83871;
        locals.var_ps0dep_dn0 = assign54400_e83871_d_n0;
        locals.var_ps0dep_dn2 = assign54400_e83871_d_n2;
        locals.var_ps0dep_dn4 = assign54400_e83871_d_n4;
        locals.var_ps0dep_dn5 = assign54400_e83871_d_n5;
        locals.var_ps0dep_dn6 = assign54400_e83871_d_n6;
        locals.var_ps0dep_dn7 = assign54400_e83871_d_n7;
        locals.var_ps0dep_dn8 = assign54400_e83871_d_n8;
        locals.var_ps0dep_dn9 = assign54400_e83871_d_n9;
        locals.var_ps0dep_dn10 = assign54400_e83871_d_n10;
        locals.var_ps0dep_dn11 = assign54400_e83871_d_n11;
        locals.var_ps0dep_dn14 = assign54400_e83871_d_n14;
        locals.var_ps0dep_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_198(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign54410_e83891, assign54410_e83891_d_n0, assign54410_e83891_d_n2, assign54410_e83891_d_n4, assign54410_e83891_d_n5, assign54410_e83891_d_n6, assign54410_e83891_d_n7, assign54410_e83891_d_n8, assign54410_e83891_d_n9, assign54410_e83891_d_n10, assign54410_e83891_d_n11, assign54410_e83891_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1371 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54410_e83891;
        locals.var_t0_dn0 = assign54410_e83891_d_n0;
        locals.var_t0_dn2 = assign54410_e83891_d_n2;
        locals.var_t0_dn4 = assign54410_e83891_d_n4;
        locals.var_t0_dn5 = assign54410_e83891_d_n5;
        locals.var_t0_dn6 = assign54410_e83891_d_n6;
        locals.var_t0_dn7 = assign54410_e83891_d_n7;
        locals.var_t0_dn8 = assign54410_e83891_d_n8;
        locals.var_t0_dn9 = assign54410_e83891_d_n9;
        locals.var_t0_dn10 = assign54410_e83891_d_n10;
        locals.var_t0_dn11 = assign54410_e83891_d_n11;
        locals.var_t0_dn14 = assign54410_e83891_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign54420_e83905, assign54420_e83905_d_n0, assign54420_e83905_d_n2, assign54420_e83905_d_n4, assign54420_e83905_d_n5, assign54420_e83905_d_n6, assign54420_e83905_d_n7, assign54420_e83905_d_n8, assign54420_e83905_d_n9, assign54420_e83905_d_n10, assign54420_e83905_d_n11, assign54420_e83905_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    }
};
        locals.var_ps0_res = assign54420_e83905;
        locals.var_ps0_res_dn0 = assign54420_e83905_d_n0;
        locals.var_ps0_res_dn2 = assign54420_e83905_d_n2;
        locals.var_ps0_res_dn4 = assign54420_e83905_d_n4;
        locals.var_ps0_res_dn5 = assign54420_e83905_d_n5;
        locals.var_ps0_res_dn6 = assign54420_e83905_d_n6;
        locals.var_ps0_res_dn7 = assign54420_e83905_d_n7;
        locals.var_ps0_res_dn8 = assign54420_e83905_d_n8;
        locals.var_ps0_res_dn9 = assign54420_e83905_d_n9;
        locals.var_ps0_res_dn10 = assign54420_e83905_d_n10;
        locals.var_ps0_res_dn11 = assign54420_e83905_d_n11;
        locals.var_ps0_res_dn14 = assign54420_e83905_d_n14;
        locals.var_ps0_res_rv = 0.0;

        let (assign54430_e83924,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let (assign54430_e83922,) = {
            if (1e-6 >= p.p407) {
                (1e-6,)
            } else {
                (p.p407,)
            }
        };
        (assign54430_e83922,)
    } else {
        (locals.var_vgpdep_dlt__blk1144,)
    }
};
        locals.var_vgpdep_dlt__blk1144 = assign54430_e83924;
        locals.var_vgpdep_dlt__blk1144_rv = 0.0;

        let assign54440_e83928: f64 = (-locals.var_vgpdep_dlt__blk1144);
        let assign54440_e83933: f64 = if ((locals.var_ps0_res > assign54440_e83928) && (locals.var_vgpdep_dlt__blk1144 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1377 = assign54440_e83933;
        locals.var_guard1377_rv = 0.0;

        let (assign54450_e83953, assign54450_e83953_d_n0, assign54450_e83953_d_n2, assign54450_e83953_d_n4, assign54450_e83953_d_n5, assign54450_e83953_d_n6, assign54450_e83953_d_n7, assign54450_e83953_d_n8, assign54450_e83953_d_n9, assign54450_e83953_d_n10, assign54450_e83953_d_n11, assign54450_e83953_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54450_e83949: f64 = locals.var_ps0_res;
        let assign54450_e83951: f64 = (assign54450_e83949 + locals.var_vgpdep_dlt__blk1144);
        (assign54450_e83951, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign54450_e83953;
        locals.var_tmf1_dn0 = assign54450_e83953_d_n0;
        locals.var_tmf1_dn2 = assign54450_e83953_d_n2;
        locals.var_tmf1_dn4 = assign54450_e83953_d_n4;
        locals.var_tmf1_dn5 = assign54450_e83953_d_n5;
        locals.var_tmf1_dn6 = assign54450_e83953_d_n6;
        locals.var_tmf1_dn7 = assign54450_e83953_d_n7;
        locals.var_tmf1_dn8 = assign54450_e83953_d_n8;
        locals.var_tmf1_dn9 = assign54450_e83953_d_n9;
        locals.var_tmf1_dn10 = assign54450_e83953_d_n10;
        locals.var_tmf1_dn11 = assign54450_e83953_d_n11;
        locals.var_tmf1_dn14 = assign54450_e83953_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign54460_e83971, assign54460_e83971_d_n0, assign54460_e83971_d_n2, assign54460_e83971_d_n4, assign54460_e83971_d_n5, assign54460_e83971_d_n6, assign54460_e83971_d_n7, assign54460_e83971_d_n8, assign54460_e83971_d_n9, assign54460_e83971_d_n10, assign54460_e83971_d_n11, assign54460_e83971_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54460_e83969: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign54460_e83969, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign54460_e83971;
        locals.var_x2_dn0 = assign54460_e83971_d_n0;
        locals.var_x2_dn2 = assign54460_e83971_d_n2;
        locals.var_x2_dn4 = assign54460_e83971_d_n4;
        locals.var_x2_dn5 = assign54460_e83971_d_n5;
        locals.var_x2_dn6 = assign54460_e83971_d_n6;
        locals.var_x2_dn7 = assign54460_e83971_d_n7;
        locals.var_x2_dn8 = assign54460_e83971_d_n8;
        locals.var_x2_dn9 = assign54460_e83971_d_n9;
        locals.var_x2_dn10 = assign54460_e83971_d_n10;
        locals.var_x2_dn11 = assign54460_e83971_d_n11;
        locals.var_x2_dn14 = assign54460_e83971_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign54470_e83989, assign54470_e83989_d_n0, assign54470_e83989_d_n2, assign54470_e83989_d_n4, assign54470_e83989_d_n5, assign54470_e83989_d_n6, assign54470_e83989_d_n7, assign54470_e83989_d_n8, assign54470_e83989_d_n9, assign54470_e83989_d_n10, assign54470_e83989_d_n11, assign54470_e83989_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54470_e83987: f64 = (locals.var_vgpdep_dlt__blk1144 * locals.var_vgpdep_dlt__blk1144);
        (assign54470_e83987, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign54470_e83989;
        locals.var_xmax2_dn0 = assign54470_e83989_d_n0;
        locals.var_xmax2_dn2 = assign54470_e83989_d_n2;
        locals.var_xmax2_dn4 = assign54470_e83989_d_n4;
        locals.var_xmax2_dn5 = assign54470_e83989_d_n5;
        locals.var_xmax2_dn6 = assign54470_e83989_d_n6;
        locals.var_xmax2_dn7 = assign54470_e83989_d_n7;
        locals.var_xmax2_dn8 = assign54470_e83989_d_n8;
        locals.var_xmax2_dn9 = assign54470_e83989_d_n9;
        locals.var_xmax2_dn10 = assign54470_e83989_d_n10;
        locals.var_xmax2_dn11 = assign54470_e83989_d_n11;
        locals.var_xmax2_dn14 = assign54470_e83989_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign54480_e84005, assign54480_e84005_d_n0, assign54480_e84005_d_n2, assign54480_e84005_d_n4, assign54480_e84005_d_n5, assign54480_e84005_d_n6, assign54480_e84005_d_n7, assign54480_e84005_d_n8, assign54480_e84005_d_n9, assign54480_e84005_d_n10, assign54480_e84005_d_n11, assign54480_e84005_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign54480_e84005;
        locals.var_xp_dn0 = assign54480_e84005_d_n0;
        locals.var_xp_dn2 = assign54480_e84005_d_n2;
        locals.var_xp_dn4 = assign54480_e84005_d_n4;
        locals.var_xp_dn5 = assign54480_e84005_d_n5;
        locals.var_xp_dn6 = assign54480_e84005_d_n6;
        locals.var_xp_dn7 = assign54480_e84005_d_n7;
        locals.var_xp_dn8 = assign54480_e84005_d_n8;
        locals.var_xp_dn9 = assign54480_e84005_d_n9;
        locals.var_xp_dn10 = assign54480_e84005_d_n10;
        locals.var_xp_dn11 = assign54480_e84005_d_n11;
        locals.var_xp_dn14 = assign54480_e84005_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign54490_e84021, assign54490_e84021_d_n0, assign54490_e84021_d_n2, assign54490_e84021_d_n4, assign54490_e84021_d_n5, assign54490_e84021_d_n6, assign54490_e84021_d_n7, assign54490_e84021_d_n8, assign54490_e84021_d_n9, assign54490_e84021_d_n10, assign54490_e84021_d_n11, assign54490_e84021_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign54490_e84021;
        locals.var_xmp_dn0 = assign54490_e84021_d_n0;
        locals.var_xmp_dn2 = assign54490_e84021_d_n2;
        locals.var_xmp_dn4 = assign54490_e84021_d_n4;
        locals.var_xmp_dn5 = assign54490_e84021_d_n5;
        locals.var_xmp_dn6 = assign54490_e84021_d_n6;
        locals.var_xmp_dn7 = assign54490_e84021_d_n7;
        locals.var_xmp_dn8 = assign54490_e84021_d_n8;
        locals.var_xmp_dn9 = assign54490_e84021_d_n9;
        locals.var_xmp_dn10 = assign54490_e84021_d_n10;
        locals.var_xmp_dn11 = assign54490_e84021_d_n11;
        locals.var_xmp_dn14 = assign54490_e84021_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign54500_e84037,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54500_e84037;
        locals.var_m0_rv = 0.0;

        let (assign54510_e84053,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54510_e84053;
        locals.var_mm_rv = 0.0;

        let (assign54520_e84069, assign54520_e84069_d_n0, assign54520_e84069_d_n2, assign54520_e84069_d_n4, assign54520_e84069_d_n5, assign54520_e84069_d_n6, assign54520_e84069_d_n7, assign54520_e84069_d_n8, assign54520_e84069_d_n9, assign54520_e84069_d_n10, assign54520_e84069_d_n11, assign54520_e84069_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign54520_e84069;
        locals.var_arg_dn0 = assign54520_e84069_d_n0;
        locals.var_arg_dn2 = assign54520_e84069_d_n2;
        locals.var_arg_dn4 = assign54520_e84069_d_n4;
        locals.var_arg_dn5 = assign54520_e84069_d_n5;
        locals.var_arg_dn6 = assign54520_e84069_d_n6;
        locals.var_arg_dn7 = assign54520_e84069_d_n7;
        locals.var_arg_dn8 = assign54520_e84069_d_n8;
        locals.var_arg_dn9 = assign54520_e84069_d_n9;
        locals.var_arg_dn10 = assign54520_e84069_d_n10;
        locals.var_arg_dn11 = assign54520_e84069_d_n11;
        locals.var_arg_dn14 = assign54520_e84069_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign54530_e84085, assign54530_e84085_d_n0, assign54530_e84085_d_n2, assign54530_e84085_d_n4, assign54530_e84085_d_n5, assign54530_e84085_d_n6, assign54530_e84085_d_n7, assign54530_e84085_d_n8, assign54530_e84085_d_n9, assign54530_e84085_d_n10, assign54530_e84085_d_n11, assign54530_e84085_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54530_e84085;
        locals.var_dnm_dn0 = assign54530_e84085_d_n0;
        locals.var_dnm_dn2 = assign54530_e84085_d_n2;
        locals.var_dnm_dn4 = assign54530_e84085_d_n4;
        locals.var_dnm_dn5 = assign54530_e84085_d_n5;
        locals.var_dnm_dn6 = assign54530_e84085_d_n6;
        locals.var_dnm_dn7 = assign54530_e84085_d_n7;
        locals.var_dnm_dn8 = assign54530_e84085_d_n8;
        locals.var_dnm_dn9 = assign54530_e84085_d_n9;
        locals.var_dnm_dn10 = assign54530_e84085_d_n10;
        locals.var_dnm_dn11 = assign54530_e84085_d_n11;
        locals.var_dnm_dn14 = assign54530_e84085_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign54540_e84101,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54540_e84101;
        locals.var_m0_rv = 0.0;

        let mut assign54550_loop_guard: usize = 0;
        while {
            let assign54550_cond_e84118: f64 = if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw__blk1145)) { 1.0 } else { 0.0 };
            assign54550_cond_e84118 != 0.0
        } {
            assign54550_loop_guard += 1;
            assert!(assign54550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54550_body0_e84136, assign54550_body0_e84136_d_n0, assign54550_body0_e84136_d_n2, assign54550_body0_e84136_d_n4, assign54550_body0_e84136_d_n5, assign54550_body0_e84136_d_n6, assign54550_body0_e84136_d_n7, assign54550_body0_e84136_d_n8, assign54550_body0_e84136_d_n9, assign54550_body0_e84136_d_n10, assign54550_body0_e84136_d_n11, assign54550_body0_e84136_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54550_body0_e84134: f64 = (locals.var_xp * locals.var_x2);
        (assign54550_body0_e84134, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign54550_body0_e84136;
            locals.var_xp_dn0 = assign54550_body0_e84136_d_n0;
            locals.var_xp_dn2 = assign54550_body0_e84136_d_n2;
            locals.var_xp_dn4 = assign54550_body0_e84136_d_n4;
            locals.var_xp_dn5 = assign54550_body0_e84136_d_n5;
            locals.var_xp_dn6 = assign54550_body0_e84136_d_n6;
            locals.var_xp_dn7 = assign54550_body0_e84136_d_n7;
            locals.var_xp_dn8 = assign54550_body0_e84136_d_n8;
            locals.var_xp_dn9 = assign54550_body0_e84136_d_n9;
            locals.var_xp_dn10 = assign54550_body0_e84136_d_n10;
            locals.var_xp_dn11 = assign54550_body0_e84136_d_n11;
            locals.var_xp_dn14 = assign54550_body0_e84136_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign54550_body1_e84154, assign54550_body1_e84154_d_n0, assign54550_body1_e84154_d_n2, assign54550_body1_e84154_d_n4, assign54550_body1_e84154_d_n5, assign54550_body1_e84154_d_n6, assign54550_body1_e84154_d_n7, assign54550_body1_e84154_d_n8, assign54550_body1_e84154_d_n9, assign54550_body1_e84154_d_n10, assign54550_body1_e84154_d_n11, assign54550_body1_e84154_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54550_body1_e84152: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign54550_body1_e84152, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign54550_body1_e84154;
            locals.var_xmp_dn0 = assign54550_body1_e84154_d_n0;
            locals.var_xmp_dn2 = assign54550_body1_e84154_d_n2;
            locals.var_xmp_dn4 = assign54550_body1_e84154_d_n4;
            locals.var_xmp_dn5 = assign54550_body1_e84154_d_n5;
            locals.var_xmp_dn6 = assign54550_body1_e84154_d_n6;
            locals.var_xmp_dn7 = assign54550_body1_e84154_d_n7;
            locals.var_xmp_dn8 = assign54550_body1_e84154_d_n8;
            locals.var_xmp_dn9 = assign54550_body1_e84154_d_n9;
            locals.var_xmp_dn10 = assign54550_body1_e84154_d_n10;
            locals.var_xmp_dn11 = assign54550_body1_e84154_d_n11;
            locals.var_xmp_dn14 = assign54550_body1_e84154_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign54550_body2_e84172,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54550_body2_e84170: f64 = (locals.var_m0 + 1.0);
        (assign54550_body2_e84170,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign54550_body2_e84172;
            locals.var_m0_rv = 0.0;
        }

        let (assign54560_e84190, assign54560_e84190_d_n0, assign54560_e84190_d_n2, assign54560_e84190_d_n4, assign54560_e84190_d_n5, assign54560_e84190_d_n6, assign54560_e84190_d_n7, assign54560_e84190_d_n8, assign54560_e84190_d_n9, assign54560_e84190_d_n10, assign54560_e84190_d_n11, assign54560_e84190_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54560_e84188: f64 = (locals.var_xp + locals.var_xmp);
        (assign54560_e84188, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign54560_e84190;
        locals.var_arg_dn0 = assign54560_e84190_d_n0;
        locals.var_arg_dn2 = assign54560_e84190_d_n2;
        locals.var_arg_dn4 = assign54560_e84190_d_n4;
        locals.var_arg_dn5 = assign54560_e84190_d_n5;
        locals.var_arg_dn6 = assign54560_e84190_d_n6;
        locals.var_arg_dn7 = assign54560_e84190_d_n7;
        locals.var_arg_dn8 = assign54560_e84190_d_n8;
        locals.var_arg_dn9 = assign54560_e84190_d_n9;
        locals.var_arg_dn10 = assign54560_e84190_d_n10;
        locals.var_arg_dn11 = assign54560_e84190_d_n11;
        locals.var_arg_dn14 = assign54560_e84190_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign54570_e84206, assign54570_e84206_d_n0, assign54570_e84206_d_n2, assign54570_e84206_d_n4, assign54570_e84206_d_n5, assign54570_e84206_d_n6, assign54570_e84206_d_n7, assign54570_e84206_d_n8, assign54570_e84206_d_n9, assign54570_e84206_d_n10, assign54570_e84206_d_n11, assign54570_e84206_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54570_e84206;
        locals.var_dnm_dn0 = assign54570_e84206_d_n0;
        locals.var_dnm_dn2 = assign54570_e84206_d_n2;
        locals.var_dnm_dn4 = assign54570_e84206_d_n4;
        locals.var_dnm_dn5 = assign54570_e84206_d_n5;
        locals.var_dnm_dn6 = assign54570_e84206_d_n6;
        locals.var_dnm_dn7 = assign54570_e84206_d_n7;
        locals.var_dnm_dn8 = assign54570_e84206_d_n8;
        locals.var_dnm_dn9 = assign54570_e84206_d_n9;
        locals.var_dnm_dn10 = assign54570_e84206_d_n10;
        locals.var_dnm_dn11 = assign54570_e84206_d_n11;
        locals.var_dnm_dn14 = assign54570_e84206_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign54580_e84221: f64 = if ((((locals.var_vgpdep_pw__blk1145 == 1.0) || (locals.var_vgpdep_pw__blk1145 == 2.0)) || (locals.var_vgpdep_pw__blk1145 == 4.0)) || (locals.var_vgpdep_pw__blk1145 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1378 = assign54580_e84221;
        locals.var_guard1378_rv = 0.0;

        let assign54590_e84224: f64 = if locals.var_vgpdep_pw__blk1145 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1379 = assign54590_e84224;
        locals.var_guard1379_rv = 0.0;

        let (assign54600_e84244,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) && (locals.var_guard1378 != 0.0)) && (locals.var_guard1379 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54600_e84244;
        locals.var_mm_rv = 0.0;

        let assign54610_e84247: f64 = if locals.var_vgpdep_pw__blk1145 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1380 = assign54610_e84247;
        locals.var_guard1380_rv = 0.0;

        let (assign54620_e84270,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) && (locals.var_guard1378 != 0.0)) && (locals.var_guard1379 == 0.0)) && (locals.var_guard1380 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54620_e84270;
        locals.var_mm_rv = 0.0;

        let assign54630_e84273: f64 = if locals.var_vgpdep_pw__blk1145 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1381 = assign54630_e84273;
        locals.var_guard1381_rv = 0.0;

        let (assign54640_e84299,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) && (locals.var_guard1378 != 0.0)) && (locals.var_guard1379 == 0.0)) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1381 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54640_e84299;
        locals.var_mm_rv = 0.0;

        let assign54650_e84302: f64 = if locals.var_vgpdep_pw__blk1145 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1382 = assign54650_e84302;
        locals.var_guard1382_rv = 0.0;

        let (assign54660_e84331,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) && (locals.var_guard1378 != 0.0)) && (locals.var_guard1379 == 0.0)) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1381 == 0.0)) && (locals.var_guard1382 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54660_e84331;
        locals.var_mm_rv = 0.0;

        let (assign54670_e84349,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) && (locals.var_guard1378 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54670_e84349;
        locals.var_m0_rv = 0.0;

        let mut assign54680_loop_guard: usize = 0;
        while {
            let assign54680_cond_e84368: f64 = if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) && (locals.var_guard1378 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign54680_cond_e84368 != 0.0
        } {
            assign54680_loop_guard += 1;
            assert!(assign54680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54680_body0_e84387, assign54680_body0_e84387_d_n0, assign54680_body0_e84387_d_n2, assign54680_body0_e84387_d_n4, assign54680_body0_e84387_d_n5, assign54680_body0_e84387_d_n6, assign54680_body0_e84387_d_n7, assign54680_body0_e84387_d_n8, assign54680_body0_e84387_d_n9, assign54680_body0_e84387_d_n10, assign54680_body0_e84387_d_n11, assign54680_body0_e84387_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) && (locals.var_guard1378 != 0.0)) {
        let assign54680_body0_e84385: f64 = (locals.var_dnm).sqrt();
        (assign54680_body0_e84385, (locals.var_dnm_dn0 / (2.0 * assign54680_body0_e84385)), (locals.var_dnm_dn2 / (2.0 * assign54680_body0_e84385)), (locals.var_dnm_dn4 / (2.0 * assign54680_body0_e84385)), (locals.var_dnm_dn5 / (2.0 * assign54680_body0_e84385)), (locals.var_dnm_dn6 / (2.0 * assign54680_body0_e84385)), (locals.var_dnm_dn7 / (2.0 * assign54680_body0_e84385)), (locals.var_dnm_dn8 / (2.0 * assign54680_body0_e84385)), (locals.var_dnm_dn9 / (2.0 * assign54680_body0_e84385)), (locals.var_dnm_dn10 / (2.0 * assign54680_body0_e84385)), (locals.var_dnm_dn11 / (2.0 * assign54680_body0_e84385)), (locals.var_dnm_dn14 / (2.0 * assign54680_body0_e84385)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign54680_body0_e84387;
            locals.var_dnm_dn0 = assign54680_body0_e84387_d_n0;
            locals.var_dnm_dn2 = assign54680_body0_e84387_d_n2;
            locals.var_dnm_dn4 = assign54680_body0_e84387_d_n4;
            locals.var_dnm_dn5 = assign54680_body0_e84387_d_n5;
            locals.var_dnm_dn6 = assign54680_body0_e84387_d_n6;
            locals.var_dnm_dn7 = assign54680_body0_e84387_d_n7;
            locals.var_dnm_dn8 = assign54680_body0_e84387_d_n8;
            locals.var_dnm_dn9 = assign54680_body0_e84387_d_n9;
            locals.var_dnm_dn10 = assign54680_body0_e84387_d_n10;
            locals.var_dnm_dn11 = assign54680_body0_e84387_d_n11;
            locals.var_dnm_dn14 = assign54680_body0_e84387_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign54680_body1_e84407,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) && (locals.var_guard1378 != 0.0)) {
        let assign54680_body1_e84405: f64 = (locals.var_m0 + 1.0);
        (assign54680_body1_e84405,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign54680_body1_e84407;
            locals.var_m0_rv = 0.0;
        }

        let (assign54690_e84437, assign54690_e84437_d_n0, assign54690_e84437_d_n2, assign54690_e84437_d_n4, assign54690_e84437_d_n5, assign54690_e84437_d_n6, assign54690_e84437_d_n7, assign54690_e84437_d_n8, assign54690_e84437_d_n9, assign54690_e84437_d_n10, assign54690_e84437_d_n11, assign54690_e84437_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) && (locals.var_guard1378 == 0.0)) {
        let (assign54690_e84435, assign54690_e84435_d_n0, assign54690_e84435_d_n2, assign54690_e84435_d_n4, assign54690_e84435_d_n5, assign54690_e84435_d_n6, assign54690_e84435_d_n7, assign54690_e84435_d_n8, assign54690_e84435_d_n9, assign54690_e84435_d_n10, assign54690_e84435_d_n11, assign54690_e84435_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign54690_e84432: f64 = (2.0 * locals.var_vgpdep_pw__blk1145);
                let assign54690_e84433: f64 = (1.0 / assign54690_e84432);
                let assign54690_e84434: f64 = (locals.var_dnm).powf(assign54690_e84433);
                (assign54690_e84434, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn0)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn2)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn4)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn5)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn6)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn7)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn8)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn9)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn10)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn11)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54690_e84433) as f64).is_finite() && ((assign54690_e84433) as f64).fract() == 0.0 { if assign54690_e84433 == 0.0 { 0.0 } else { (assign54690_e84433 * ((locals.var_dnm).powf(assign54690_e84433 - 1.0) * locals.var_dnm_dn14)) } } else { (assign54690_e84434 * (assign54690_e84433 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign54690_e84435, assign54690_e84435_d_n0, assign54690_e84435_d_n2, assign54690_e84435_d_n4, assign54690_e84435_d_n5, assign54690_e84435_d_n6, assign54690_e84435_d_n7, assign54690_e84435_d_n8, assign54690_e84435_d_n9, assign54690_e84435_d_n10, assign54690_e84435_d_n11, assign54690_e84435_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54690_e84437;
        locals.var_dnm_dn0 = assign54690_e84437_d_n0;
        locals.var_dnm_dn2 = assign54690_e84437_d_n2;
        locals.var_dnm_dn4 = assign54690_e84437_d_n4;
        locals.var_dnm_dn5 = assign54690_e84437_d_n5;
        locals.var_dnm_dn6 = assign54690_e84437_d_n6;
        locals.var_dnm_dn7 = assign54690_e84437_d_n7;
        locals.var_dnm_dn8 = assign54690_e84437_d_n8;
        locals.var_dnm_dn9 = assign54690_e84437_d_n9;
        locals.var_dnm_dn10 = assign54690_e84437_d_n10;
        locals.var_dnm_dn11 = assign54690_e84437_d_n11;
        locals.var_dnm_dn14 = assign54690_e84437_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign54700_e84455, assign54700_e84455_d_n0, assign54700_e84455_d_n2, assign54700_e84455_d_n4, assign54700_e84455_d_n5, assign54700_e84455_d_n6, assign54700_e84455_d_n7, assign54700_e84455_d_n8, assign54700_e84455_d_n9, assign54700_e84455_d_n10, assign54700_e84455_d_n11, assign54700_e84455_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54700_e84453: f64 = (1.0 / locals.var_dnm);
        (assign54700_e84453, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54700_e84455;
        locals.var_dnm_dn0 = assign54700_e84455_d_n0;
        locals.var_dnm_dn2 = assign54700_e84455_d_n2;
        locals.var_dnm_dn4 = assign54700_e84455_d_n4;
        locals.var_dnm_dn5 = assign54700_e84455_d_n5;
        locals.var_dnm_dn6 = assign54700_e84455_d_n6;
        locals.var_dnm_dn7 = assign54700_e84455_d_n7;
        locals.var_dnm_dn8 = assign54700_e84455_d_n8;
        locals.var_dnm_dn9 = assign54700_e84455_d_n9;
        locals.var_dnm_dn10 = assign54700_e84455_d_n10;
        locals.var_dnm_dn11 = assign54700_e84455_d_n11;
        locals.var_dnm_dn14 = assign54700_e84455_d_n14;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_199(
        locals: &mut StampLocals,
    ) {
        let (assign54710_e84475, assign54710_e84475_d_n0, assign54710_e84475_d_n2, assign54710_e84475_d_n4, assign54710_e84475_d_n5, assign54710_e84475_d_n6, assign54710_e84475_d_n7, assign54710_e84475_d_n8, assign54710_e84475_d_n9, assign54710_e84475_d_n10, assign54710_e84475_d_n11, assign54710_e84475_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54710_e84471: f64 = (locals.var_tmf1 * locals.var_vgpdep_dlt__blk1144);
        let assign54710_e84473: f64 = (assign54710_e84471 * locals.var_dnm);
        (assign54710_e84473, (((locals.var_tmf1_dn0 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * locals.var_vgpdep_dlt__blk1144) * locals.var_dnm) + (assign54710_e84471 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign54710_e84475;
        locals.var_tmf0_dn0 = assign54710_e84475_d_n0;
        locals.var_tmf0_dn2 = assign54710_e84475_d_n2;
        locals.var_tmf0_dn4 = assign54710_e84475_d_n4;
        locals.var_tmf0_dn5 = assign54710_e84475_d_n5;
        locals.var_tmf0_dn6 = assign54710_e84475_d_n6;
        locals.var_tmf0_dn7 = assign54710_e84475_d_n7;
        locals.var_tmf0_dn8 = assign54710_e84475_d_n8;
        locals.var_tmf0_dn9 = assign54710_e84475_d_n9;
        locals.var_tmf0_dn10 = assign54710_e84475_d_n10;
        locals.var_tmf0_dn11 = assign54710_e84475_d_n11;
        locals.var_tmf0_dn14 = assign54710_e84475_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign54720_e84497, assign54720_e84497_d_n0, assign54720_e84497_d_n2, assign54720_e84497_d_n4, assign54720_e84497_d_n5, assign54720_e84497_d_n6, assign54720_e84497_d_n7, assign54720_e84497_d_n8, assign54720_e84497_d_n9, assign54720_e84497_d_n10, assign54720_e84497_d_n11, assign54720_e84497_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54720_e84491: f64 = (locals.var_vgpdep_dlt__blk1144 * locals.var_xmp);
        let assign54720_e84493: f64 = (assign54720_e84491 * locals.var_dnm);
        let assign54720_e84495: f64 = (assign54720_e84493 / locals.var_arg);
        (assign54720_e84495, ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn0) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn0)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn2) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn2)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn4) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn4)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn5) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn5)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn6) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn6)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn7) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn7)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn8) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn8)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn9) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn9)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn10) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn10)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn11) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn11)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1144 * locals.var_xmp_dn14) * locals.var_dnm) + (assign54720_e84491 * locals.var_dnm_dn14)) * locals.var_arg) - (assign54720_e84493 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54720_e84497;
        locals.var_t0_dn0 = assign54720_e84497_d_n0;
        locals.var_t0_dn2 = assign54720_e84497_d_n2;
        locals.var_t0_dn4 = assign54720_e84497_d_n4;
        locals.var_t0_dn5 = assign54720_e84497_d_n5;
        locals.var_t0_dn6 = assign54720_e84497_d_n6;
        locals.var_t0_dn7 = assign54720_e84497_d_n7;
        locals.var_t0_dn8 = assign54720_e84497_d_n8;
        locals.var_t0_dn9 = assign54720_e84497_d_n9;
        locals.var_t0_dn10 = assign54720_e84497_d_n10;
        locals.var_t0_dn11 = assign54720_e84497_d_n11;
        locals.var_t0_dn14 = assign54720_e84497_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign54730_e84517, assign54730_e84517_d_n0, assign54730_e84517_d_n2, assign54730_e84517_d_n4, assign54730_e84517_d_n5, assign54730_e84517_d_n6, assign54730_e84517_d_n7, assign54730_e84517_d_n8, assign54730_e84517_d_n9, assign54730_e84517_d_n10, assign54730_e84517_d_n11, assign54730_e84517_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        let assign54730_e84513: f64 = (-locals.var_vgpdep_dlt__blk1144);
        let assign54730_e84515: f64 = (assign54730_e84513 + locals.var_tmf0);
        (assign54730_e84515, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54730_e84517;
        locals.var_ps0dep_dn0 = assign54730_e84517_d_n0;
        locals.var_ps0dep_dn2 = assign54730_e84517_d_n2;
        locals.var_ps0dep_dn4 = assign54730_e84517_d_n4;
        locals.var_ps0dep_dn5 = assign54730_e84517_d_n5;
        locals.var_ps0dep_dn6 = assign54730_e84517_d_n6;
        locals.var_ps0dep_dn7 = assign54730_e84517_d_n7;
        locals.var_ps0dep_dn8 = assign54730_e84517_d_n8;
        locals.var_ps0dep_dn9 = assign54730_e84517_d_n9;
        locals.var_ps0dep_dn10 = assign54730_e84517_d_n10;
        locals.var_ps0dep_dn11 = assign54730_e84517_d_n11;
        locals.var_ps0dep_dn14 = assign54730_e84517_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign54740_e84533, assign54740_e84533_d_n0, assign54740_e84533_d_n2, assign54740_e84533_d_n4, assign54740_e84533_d_n5, assign54740_e84533_d_n6, assign54740_e84533_d_n7, assign54740_e84533_d_n8, assign54740_e84533_d_n9, assign54740_e84533_d_n10, assign54740_e84533_d_n11, assign54740_e84533_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54740_e84533;
        locals.var_t0_dn0 = assign54740_e84533_d_n0;
        locals.var_t0_dn2 = assign54740_e84533_d_n2;
        locals.var_t0_dn4 = assign54740_e84533_d_n4;
        locals.var_t0_dn5 = assign54740_e84533_d_n5;
        locals.var_t0_dn6 = assign54740_e84533_d_n6;
        locals.var_t0_dn7 = assign54740_e84533_d_n7;
        locals.var_t0_dn8 = assign54740_e84533_d_n8;
        locals.var_t0_dn9 = assign54740_e84533_d_n9;
        locals.var_t0_dn10 = assign54740_e84533_d_n10;
        locals.var_t0_dn11 = assign54740_e84533_d_n11;
        locals.var_t0_dn14 = assign54740_e84533_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign54750_e84550, assign54750_e84550_d_n0, assign54750_e84550_d_n2, assign54750_e84550_d_n4, assign54750_e84550_d_n5, assign54750_e84550_d_n6, assign54750_e84550_d_n7, assign54750_e84550_d_n8, assign54750_e84550_d_n9, assign54750_e84550_d_n10, assign54750_e84550_d_n11, assign54750_e84550_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 == 0.0)) {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54750_e84550;
        locals.var_ps0dep_dn0 = assign54750_e84550_d_n0;
        locals.var_ps0dep_dn2 = assign54750_e84550_d_n2;
        locals.var_ps0dep_dn4 = assign54750_e84550_d_n4;
        locals.var_ps0dep_dn5 = assign54750_e84550_d_n5;
        locals.var_ps0dep_dn6 = assign54750_e84550_d_n6;
        locals.var_ps0dep_dn7 = assign54750_e84550_d_n7;
        locals.var_ps0dep_dn8 = assign54750_e84550_d_n8;
        locals.var_ps0dep_dn9 = assign54750_e84550_d_n9;
        locals.var_ps0dep_dn10 = assign54750_e84550_d_n10;
        locals.var_ps0dep_dn11 = assign54750_e84550_d_n11;
        locals.var_ps0dep_dn14 = assign54750_e84550_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign54760_e84567, assign54760_e84567_d_n0, assign54760_e84567_d_n2, assign54760_e84567_d_n4, assign54760_e84567_d_n5, assign54760_e84567_d_n6, assign54760_e84567_d_n7, assign54760_e84567_d_n8, assign54760_e84567_d_n9, assign54760_e84567_d_n10, assign54760_e84567_d_n11, assign54760_e84567_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1377 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54760_e84567;
        locals.var_t0_dn0 = assign54760_e84567_d_n0;
        locals.var_t0_dn2 = assign54760_e84567_d_n2;
        locals.var_t0_dn4 = assign54760_e84567_d_n4;
        locals.var_t0_dn5 = assign54760_e84567_d_n5;
        locals.var_t0_dn6 = assign54760_e84567_d_n6;
        locals.var_t0_dn7 = assign54760_e84567_d_n7;
        locals.var_t0_dn8 = assign54760_e84567_d_n8;
        locals.var_t0_dn9 = assign54760_e84567_d_n9;
        locals.var_t0_dn10 = assign54760_e84567_d_n10;
        locals.var_t0_dn11 = assign54760_e84567_d_n11;
        locals.var_t0_dn14 = assign54760_e84567_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign54770_e84582, assign54770_e84582_d_n0, assign54770_e84582_d_n2, assign54770_e84582_d_n4, assign54770_e84582_d_n5, assign54770_e84582_d_n6, assign54770_e84582_d_n7, assign54770_e84582_d_n8, assign54770_e84582_d_n9, assign54770_e84582_d_n10, assign54770_e84582_d_n11, assign54770_e84582_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign54770_e84580: f64 = (-locals.var_ps0dep);
        (assign54770_e84580, (-locals.var_ps0dep_dn0), (-locals.var_ps0dep_dn2), (-locals.var_ps0dep_dn4), (-locals.var_ps0dep_dn5), (-locals.var_ps0dep_dn6), (-locals.var_ps0dep_dn7), (-locals.var_ps0dep_dn8), (-locals.var_ps0dep_dn9), (-locals.var_ps0dep_dn10), (-locals.var_ps0dep_dn11), (-locals.var_ps0dep_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54770_e84582;
        locals.var_ps0dep_dn0 = assign54770_e84582_d_n0;
        locals.var_ps0dep_dn2 = assign54770_e84582_d_n2;
        locals.var_ps0dep_dn4 = assign54770_e84582_d_n4;
        locals.var_ps0dep_dn5 = assign54770_e84582_d_n5;
        locals.var_ps0dep_dn6 = assign54770_e84582_d_n6;
        locals.var_ps0dep_dn7 = assign54770_e84582_d_n7;
        locals.var_ps0dep_dn8 = assign54770_e84582_d_n8;
        locals.var_ps0dep_dn9 = assign54770_e84582_d_n9;
        locals.var_ps0dep_dn10 = assign54770_e84582_d_n10;
        locals.var_ps0dep_dn11 = assign54770_e84582_d_n11;
        locals.var_ps0dep_dn14 = assign54770_e84582_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign54780_e84604, assign54780_e84604_d_n0, assign54780_e84604_d_n2, assign54780_e84604_d_n4, assign54780_e84604_d_n5, assign54780_e84604_d_n6, assign54780_e84604_d_n7, assign54780_e84604_d_n8, assign54780_e84604_d_n9, assign54780_e84604_d_n10, assign54780_e84604_d_n11, assign54780_e84604_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign54780_e84596: f64 = (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150);
        let assign54780_e84598: f64 = (assign54780_e84596 * locals.var_tnp__blk1150);
        let assign54780_e84600: f64 = (assign54780_e84598 / 2.0);
        let assign54780_e84602: f64 = (assign54780_e84600 / 1.034943e-10);
        (assign54780_e84602, ((((((locals.var_q_ndepm__blk1135_dn0 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn0)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn0)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn2 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn2)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn2)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn4 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn4)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn4)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn5 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn5)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn5)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn6 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn6)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn6)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn7 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn7)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn7)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn8 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn8)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn8)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn9 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn9)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn9)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn10 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn10)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn10)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn11 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn11)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn11)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1135_dn14 * locals.var_tnp__blk1150) + (locals.var_q_ndepm__blk1135 * locals.var_tnp__blk1150_dn14)) * locals.var_tnp__blk1150) + (assign54780_e84596 * locals.var_tnp__blk1150_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1322, locals.var_dphi_sb__blk1322_dn0, locals.var_dphi_sb__blk1322_dn2, locals.var_dphi_sb__blk1322_dn4, locals.var_dphi_sb__blk1322_dn5, locals.var_dphi_sb__blk1322_dn6, locals.var_dphi_sb__blk1322_dn7, locals.var_dphi_sb__blk1322_dn8, locals.var_dphi_sb__blk1322_dn9, locals.var_dphi_sb__blk1322_dn10, locals.var_dphi_sb__blk1322_dn11, locals.var_dphi_sb__blk1322_dn14,)
    }
};
        locals.var_dphi_sb__blk1322 = assign54780_e84604;
        locals.var_dphi_sb__blk1322_dn0 = assign54780_e84604_d_n0;
        locals.var_dphi_sb__blk1322_dn2 = assign54780_e84604_d_n2;
        locals.var_dphi_sb__blk1322_dn4 = assign54780_e84604_d_n4;
        locals.var_dphi_sb__blk1322_dn5 = assign54780_e84604_d_n5;
        locals.var_dphi_sb__blk1322_dn6 = assign54780_e84604_d_n6;
        locals.var_dphi_sb__blk1322_dn7 = assign54780_e84604_d_n7;
        locals.var_dphi_sb__blk1322_dn8 = assign54780_e84604_d_n8;
        locals.var_dphi_sb__blk1322_dn9 = assign54780_e84604_d_n9;
        locals.var_dphi_sb__blk1322_dn10 = assign54780_e84604_d_n10;
        locals.var_dphi_sb__blk1322_dn11 = assign54780_e84604_d_n11;
        locals.var_dphi_sb__blk1322_dn14 = assign54780_e84604_d_n14;
        locals.var_dphi_sb__blk1322_rv = 0.0;

        let (assign54790_e84625, assign54790_e84625_d_n0, assign54790_e84625_d_n2, assign54790_e84625_d_n4, assign54790_e84625_d_n5, assign54790_e84625_d_n6, assign54790_e84625_d_n7, assign54790_e84625_d_n8, assign54790_e84625_d_n9, assign54790_e84625_d_n10, assign54790_e84625_d_n11, assign54790_e84625_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign54790_e84619: f64 = (2.0 * locals.var_beta);
        let assign54790_e84621: f64 = (assign54790_e84619 * locals.var_dphi_sb__blk1322);
        let assign54790_e84622: f64 = (assign54790_e84621).sqrt();
        let assign54790_e84623: f64 = (locals.var_wdepsubsl * assign54790_e84622);
        (assign54790_e84623, ((locals.var_wdepsubsl_dn0 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn0)) / (2.0 * assign54790_e84622)))), ((locals.var_wdepsubsl_dn2 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn2)) / (2.0 * assign54790_e84622)))), ((locals.var_wdepsubsl_dn4 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn4)) / (2.0 * assign54790_e84622)))), ((locals.var_wdepsubsl_dn5 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn5)) / (2.0 * assign54790_e84622)))), ((locals.var_wdepsubsl_dn6 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn6)) / (2.0 * assign54790_e84622)))), ((locals.var_wdepsubsl_dn7 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn7)) / (2.0 * assign54790_e84622)))), ((locals.var_wdepsubsl_dn8 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn8)) / (2.0 * assign54790_e84622)))), ((locals.var_wdepsubsl_dn9 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn9)) / (2.0 * assign54790_e84622)))), ((locals.var_wdepsubsl_dn10 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn10)) / (2.0 * assign54790_e84622)))), ((locals.var_wdepsubsl_dn11 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn11)) / (2.0 * assign54790_e84622)))), ((locals.var_wdepsubsl_dn14 * assign54790_e84622) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb__blk1322) + (assign54790_e84619 * locals.var_dphi_sb__blk1322_dn14)) / (2.0 * assign54790_e84622)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54790_e84625;
        locals.var_t0_dn0 = assign54790_e84625_d_n0;
        locals.var_t0_dn2 = assign54790_e84625_d_n2;
        locals.var_t0_dn4 = assign54790_e84625_d_n4;
        locals.var_t0_dn5 = assign54790_e84625_d_n5;
        locals.var_t0_dn6 = assign54790_e84625_d_n6;
        locals.var_t0_dn7 = assign54790_e84625_d_n7;
        locals.var_t0_dn8 = assign54790_e84625_d_n8;
        locals.var_t0_dn9 = assign54790_e84625_d_n9;
        locals.var_t0_dn10 = assign54790_e84625_d_n10;
        locals.var_t0_dn11 = assign54790_e84625_d_n11;
        locals.var_t0_dn14 = assign54790_e84625_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign54800_e84646, assign54800_e84646_d_n0, assign54800_e84646_d_n2, assign54800_e84646_d_n4, assign54800_e84646_d_n5, assign54800_e84646_d_n6, assign54800_e84646_d_n7, assign54800_e84646_d_n8, assign54800_e84646_d_n9, assign54800_e84646_d_n10, assign54800_e84646_d_n11, assign54800_e84646_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign54800_e84638: f64 = (locals.var_t0).exp();
        let assign54800_e84640: f64 = (-locals.var_t0);
        let assign54800_e84641: f64 = (assign54800_e84640).exp();
        let assign54800_e84642: f64 = (assign54800_e84638 + assign54800_e84641);
        let assign54800_e84644: f64 = (assign54800_e84642 / 2.0);
        (assign54800_e84644, (((assign54800_e84638 * locals.var_t0_dn0) + (assign54800_e84641 * (-locals.var_t0_dn0))) / 2.0), (((assign54800_e84638 * locals.var_t0_dn2) + (assign54800_e84641 * (-locals.var_t0_dn2))) / 2.0), (((assign54800_e84638 * locals.var_t0_dn4) + (assign54800_e84641 * (-locals.var_t0_dn4))) / 2.0), (((assign54800_e84638 * locals.var_t0_dn5) + (assign54800_e84641 * (-locals.var_t0_dn5))) / 2.0), (((assign54800_e84638 * locals.var_t0_dn6) + (assign54800_e84641 * (-locals.var_t0_dn6))) / 2.0), (((assign54800_e84638 * locals.var_t0_dn7) + (assign54800_e84641 * (-locals.var_t0_dn7))) / 2.0), (((assign54800_e84638 * locals.var_t0_dn8) + (assign54800_e84641 * (-locals.var_t0_dn8))) / 2.0), (((assign54800_e84638 * locals.var_t0_dn9) + (assign54800_e84641 * (-locals.var_t0_dn9))) / 2.0), (((assign54800_e84638 * locals.var_t0_dn10) + (assign54800_e84641 * (-locals.var_t0_dn10))) / 2.0), (((assign54800_e84638 * locals.var_t0_dn11) + (assign54800_e84641 * (-locals.var_t0_dn11))) / 2.0), (((assign54800_e84638 * locals.var_t0_dn14) + (assign54800_e84641 * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54800_e84646;
        locals.var_t1_dn0 = assign54800_e84646_d_n0;
        locals.var_t1_dn2 = assign54800_e84646_d_n2;
        locals.var_t1_dn4 = assign54800_e84646_d_n4;
        locals.var_t1_dn5 = assign54800_e84646_d_n5;
        locals.var_t1_dn6 = assign54800_e84646_d_n6;
        locals.var_t1_dn7 = assign54800_e84646_d_n7;
        locals.var_t1_dn8 = assign54800_e84646_d_n8;
        locals.var_t1_dn9 = assign54800_e84646_d_n9;
        locals.var_t1_dn10 = assign54800_e84646_d_n10;
        locals.var_t1_dn11 = assign54800_e84646_d_n11;
        locals.var_t1_dn14 = assign54800_e84646_d_n14;
        locals.var_t1_rv = 0.0;

        let assign54810_e84648: f64 = (locals.var_t0).abs();
        let assign54810_e84650: f64 = if assign54810_e84648 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1383 = assign54810_e84650;
        locals.var_guard1383_rv = 0.0;

        let (assign54820_e84669, assign54820_e84669_d_n0, assign54820_e84669_d_n2, assign54820_e84669_d_n4, assign54820_e84669_d_n5, assign54820_e84669_d_n6, assign54820_e84669_d_n7, assign54820_e84669_d_n8, assign54820_e84669_d_n9, assign54820_e84669_d_n10, assign54820_e84669_d_n11, assign54820_e84669_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1383 != 0.0)) {
        let assign54820_e84665: f64 = (locals.var_t1).ln();
        let assign54820_e84667: f64 = (assign54820_e84665 / locals.var_dphi_sb__blk1322);
        (assign54820_e84667, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn0)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn2)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn4)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn5)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn6)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn7)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn8)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn9)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn10)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn11)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb__blk1322) - (assign54820_e84665 * locals.var_dphi_sb__blk1322_dn14)) / (locals.var_dphi_sb__blk1322 * locals.var_dphi_sb__blk1322)),)
    } else {
        (locals.var_c_sb__blk1323, locals.var_c_sb__blk1323_dn0, locals.var_c_sb__blk1323_dn2, locals.var_c_sb__blk1323_dn4, locals.var_c_sb__blk1323_dn5, locals.var_c_sb__blk1323_dn6, locals.var_c_sb__blk1323_dn7, locals.var_c_sb__blk1323_dn8, locals.var_c_sb__blk1323_dn9, locals.var_c_sb__blk1323_dn10, locals.var_c_sb__blk1323_dn11, locals.var_c_sb__blk1323_dn14,)
    }
};
        locals.var_c_sb__blk1323 = assign54820_e84669;
        locals.var_c_sb__blk1323_dn0 = assign54820_e84669_d_n0;
        locals.var_c_sb__blk1323_dn2 = assign54820_e84669_d_n2;
        locals.var_c_sb__blk1323_dn4 = assign54820_e84669_d_n4;
        locals.var_c_sb__blk1323_dn5 = assign54820_e84669_d_n5;
        locals.var_c_sb__blk1323_dn6 = assign54820_e84669_d_n6;
        locals.var_c_sb__blk1323_dn7 = assign54820_e84669_d_n7;
        locals.var_c_sb__blk1323_dn8 = assign54820_e84669_d_n8;
        locals.var_c_sb__blk1323_dn9 = assign54820_e84669_d_n9;
        locals.var_c_sb__blk1323_dn10 = assign54820_e84669_d_n10;
        locals.var_c_sb__blk1323_dn11 = assign54820_e84669_d_n11;
        locals.var_c_sb__blk1323_dn14 = assign54820_e84669_d_n14;
        locals.var_c_sb__blk1323_rv = 0.0;

        let (assign54830_e84698, assign54830_e84698_d_n0, assign54830_e84698_d_n2, assign54830_e84698_d_n4, assign54830_e84698_d_n5, assign54830_e84698_d_n6, assign54830_e84698_d_n7, assign54830_e84698_d_n8, assign54830_e84698_d_n9, assign54830_e84698_d_n10, assign54830_e84698_d_n11, assign54830_e84698_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1383 == 0.0)) {
        let assign54830_e84686: f64 = (locals.var_wdepsubsl * locals.var_wdepsubsl);
        let assign54830_e84688: f64 = (assign54830_e84686 * locals.var_beta);
        let assign54830_e84692: f64 = (0.1666666666666667 * locals.var_t0);
        let assign54830_e84694: f64 = (assign54830_e84692 * locals.var_t0);
        let assign54830_e84695: f64 = (1.0 - assign54830_e84694);
        let assign54830_e84696: f64 = (assign54830_e84688 * assign54830_e84695);
        (assign54830_e84696, ((((((locals.var_wdepsubsl_dn0 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn0)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn0)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn0) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn0))))), ((((((locals.var_wdepsubsl_dn2 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn2)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn2)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn2) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn2))))), ((((((locals.var_wdepsubsl_dn4 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn4)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn4)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn4) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn4))))), ((((((locals.var_wdepsubsl_dn5 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn5)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn5)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn5) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn5))))), ((((((locals.var_wdepsubsl_dn6 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn6)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn6)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn6) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn6))))), ((((((locals.var_wdepsubsl_dn7 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn7)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn7)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn7) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn7))))), ((((((locals.var_wdepsubsl_dn8 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn8)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn8)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn8) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn8))))), ((((((locals.var_wdepsubsl_dn9 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn9)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn9)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn9) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn9))))), ((((((locals.var_wdepsubsl_dn10 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn10)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn10)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn10) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn10))))), ((((((locals.var_wdepsubsl_dn11 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn11)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn11)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn11) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn11))))), ((((((locals.var_wdepsubsl_dn14 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn14)) * locals.var_beta) + (assign54830_e84686 * locals.var_beta_dn14)) * assign54830_e84695) + (assign54830_e84688 * (-(((0.1666666666666667 * locals.var_t0_dn14) * locals.var_t0) + (assign54830_e84692 * locals.var_t0_dn14))))),)
    } else {
        (locals.var_c_sb__blk1323, locals.var_c_sb__blk1323_dn0, locals.var_c_sb__blk1323_dn2, locals.var_c_sb__blk1323_dn4, locals.var_c_sb__blk1323_dn5, locals.var_c_sb__blk1323_dn6, locals.var_c_sb__blk1323_dn7, locals.var_c_sb__blk1323_dn8, locals.var_c_sb__blk1323_dn9, locals.var_c_sb__blk1323_dn10, locals.var_c_sb__blk1323_dn11, locals.var_c_sb__blk1323_dn14,)
    }
};
        locals.var_c_sb__blk1323 = assign54830_e84698;
        locals.var_c_sb__blk1323_dn0 = assign54830_e84698_d_n0;
        locals.var_c_sb__blk1323_dn2 = assign54830_e84698_d_n2;
        locals.var_c_sb__blk1323_dn4 = assign54830_e84698_d_n4;
        locals.var_c_sb__blk1323_dn5 = assign54830_e84698_d_n5;
        locals.var_c_sb__blk1323_dn6 = assign54830_e84698_d_n6;
        locals.var_c_sb__blk1323_dn7 = assign54830_e84698_d_n7;
        locals.var_c_sb__blk1323_dn8 = assign54830_e84698_d_n8;
        locals.var_c_sb__blk1323_dn9 = assign54830_e84698_d_n9;
        locals.var_c_sb__blk1323_dn10 = assign54830_e84698_d_n10;
        locals.var_c_sb__blk1323_dn11 = assign54830_e84698_d_n11;
        locals.var_c_sb__blk1323_dn14 = assign54830_e84698_d_n14;
        locals.var_c_sb__blk1323_rv = 0.0;

        let (assign54840_e84714, assign54840_e84714_d_n0, assign54840_e84714_d_n2, assign54840_e84714_d_n4, assign54840_e84714_d_n5, assign54840_e84714_d_n6, assign54840_e84714_d_n7, assign54840_e84714_d_n8, assign54840_e84714_d_n9, assign54840_e84714_d_n10, assign54840_e84714_d_n11, assign54840_e84714_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign54840_e84712: f64 = (locals.var_c_sb__blk1323 * locals.var_ps0dep);
        (assign54840_e84712, ((locals.var_c_sb__blk1323_dn0 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn0)), ((locals.var_c_sb__blk1323_dn2 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn2)), ((locals.var_c_sb__blk1323_dn4 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn4)), ((locals.var_c_sb__blk1323_dn5 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn5)), ((locals.var_c_sb__blk1323_dn6 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn6)), ((locals.var_c_sb__blk1323_dn7 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn7)), ((locals.var_c_sb__blk1323_dn8 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn8)), ((locals.var_c_sb__blk1323_dn9 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn9)), ((locals.var_c_sb__blk1323_dn10 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn10)), ((locals.var_c_sb__blk1323_dn11 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn11)), ((locals.var_c_sb__blk1323_dn14 * locals.var_ps0dep) + (locals.var_c_sb__blk1323 * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign54840_e84714;
        locals.var_tx_dn0 = assign54840_e84714_d_n0;
        locals.var_tx_dn2 = assign54840_e84714_d_n2;
        locals.var_tx_dn4 = assign54840_e84714_d_n4;
        locals.var_tx_dn5 = assign54840_e84714_d_n5;
        locals.var_tx_dn6 = assign54840_e84714_d_n6;
        locals.var_tx_dn7 = assign54840_e84714_d_n7;
        locals.var_tx_dn8 = assign54840_e84714_d_n8;
        locals.var_tx_dn9 = assign54840_e84714_d_n9;
        locals.var_tx_dn10 = assign54840_e84714_d_n10;
        locals.var_tx_dn11 = assign54840_e84714_d_n11;
        locals.var_tx_dn14 = assign54840_e84714_d_n14;
        locals.var_tx_rv = 0.0;

        let assign54850_e84717: f64 = if locals.var_tx > 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1384 = assign54850_e84717;
        locals.var_guard1384_rv = 0.0;

        let (assign54860_e84735, assign54860_e84735_d_n0, assign54860_e84735_d_n2, assign54860_e84735_d_n4, assign54860_e84735_d_n5, assign54860_e84735_d_n6, assign54860_e84735_d_n7, assign54860_e84735_d_n8, assign54860_e84735_d_n9, assign54860_e84735_d_n10, assign54860_e84735_d_n11, assign54860_e84735_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 != 0.0)) {
        let assign54860_e84733: f64 = (locals.var_ps0dep - locals.var_dphi_sb__blk1322);
        (assign54860_e84733, (locals.var_ps0dep_dn0 - locals.var_dphi_sb__blk1322_dn0), (locals.var_ps0dep_dn2 - locals.var_dphi_sb__blk1322_dn2), (locals.var_ps0dep_dn4 - locals.var_dphi_sb__blk1322_dn4), (locals.var_ps0dep_dn5 - locals.var_dphi_sb__blk1322_dn5), (locals.var_ps0dep_dn6 - locals.var_dphi_sb__blk1322_dn6), (locals.var_ps0dep_dn7 - locals.var_dphi_sb__blk1322_dn7), (locals.var_ps0dep_dn8 - locals.var_dphi_sb__blk1322_dn8), (locals.var_ps0dep_dn9 - locals.var_dphi_sb__blk1322_dn9), (locals.var_ps0dep_dn10 - locals.var_dphi_sb__blk1322_dn10), (locals.var_ps0dep_dn11 - locals.var_dphi_sb__blk1322_dn11), (locals.var_ps0dep_dn14 - locals.var_dphi_sb__blk1322_dn14),)
    } else {
        (locals.var_pb0dep__blk1167, locals.var_pb0dep__blk1167_dn0, locals.var_pb0dep__blk1167_dn2, locals.var_pb0dep__blk1167_dn4, locals.var_pb0dep__blk1167_dn5, locals.var_pb0dep__blk1167_dn6, locals.var_pb0dep__blk1167_dn7, locals.var_pb0dep__blk1167_dn8, locals.var_pb0dep__blk1167_dn9, locals.var_pb0dep__blk1167_dn10, locals.var_pb0dep__blk1167_dn11, locals.var_pb0dep__blk1167_dn14,)
    }
};
        locals.var_pb0dep__blk1167 = assign54860_e84735;
        locals.var_pb0dep__blk1167_dn0 = assign54860_e84735_d_n0;
        locals.var_pb0dep__blk1167_dn2 = assign54860_e84735_d_n2;
        locals.var_pb0dep__blk1167_dn4 = assign54860_e84735_d_n4;
        locals.var_pb0dep__blk1167_dn5 = assign54860_e84735_d_n5;
        locals.var_pb0dep__blk1167_dn6 = assign54860_e84735_d_n6;
        locals.var_pb0dep__blk1167_dn7 = assign54860_e84735_d_n7;
        locals.var_pb0dep__blk1167_dn8 = assign54860_e84735_d_n8;
        locals.var_pb0dep__blk1167_dn9 = assign54860_e84735_d_n9;
        locals.var_pb0dep__blk1167_dn10 = assign54860_e84735_d_n10;
        locals.var_pb0dep__blk1167_dn11 = assign54860_e84735_d_n11;
        locals.var_pb0dep__blk1167_dn14 = assign54860_e84735_d_n14;
        locals.var_pb0dep__blk1167_rv = 0.0;

        let (assign54870_e84756, assign54870_e84756_d_n0, assign54870_e84756_d_n2, assign54870_e84756_d_n4, assign54870_e84756_d_n5, assign54870_e84756_d_n6, assign54870_e84756_d_n7, assign54870_e84756_d_n8, assign54870_e84756_d_n9, assign54870_e84756_d_n10, assign54870_e84756_d_n11, assign54870_e84756_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) {
        let assign54870_e84751: f64 = (-locals.var_c_sb__blk1323);
        let assign54870_e84753: f64 = (assign54870_e84751 * locals.var_dphi_sb__blk1322);
        let assign54870_e84754: f64 = (assign54870_e84753).exp();
        (assign54870_e84754, (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn0) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn0))), (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn2) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn2))), (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn4) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn4))), (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn5) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn5))), (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn6) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn6))), (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn7) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn7))), (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn8) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn8))), (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn9) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn9))), (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn10) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn10))), (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn11) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn11))), (assign54870_e84754 * (((-locals.var_c_sb__blk1323_dn14) * locals.var_dphi_sb__blk1322) + (assign54870_e84751 * locals.var_dphi_sb__blk1322_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54870_e84756;
        locals.var_t0_dn0 = assign54870_e84756_d_n0;
        locals.var_t0_dn2 = assign54870_e84756_d_n2;
        locals.var_t0_dn4 = assign54870_e84756_d_n4;
        locals.var_t0_dn5 = assign54870_e84756_d_n5;
        locals.var_t0_dn6 = assign54870_e84756_d_n6;
        locals.var_t0_dn7 = assign54870_e84756_d_n7;
        locals.var_t0_dn8 = assign54870_e84756_d_n8;
        locals.var_t0_dn9 = assign54870_e84756_d_n9;
        locals.var_t0_dn10 = assign54870_e84756_d_n10;
        locals.var_t0_dn11 = assign54870_e84756_d_n11;
        locals.var_t0_dn14 = assign54870_e84756_d_n14;
        locals.var_t0_rv = 0.0;

        let assign54880_e84758: f64 = (locals.var_tx).abs();
        let assign54880_e84760: f64 = if assign54880_e84758 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1385 = assign54880_e84760;
        locals.var_guard1385_rv = 0.0;

        let assign54890_e84763: f64 = if locals.var_tx >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1386 = assign54890_e84763;
        locals.var_guard1386_rv = 0.0;

        let (assign54900_e84790, assign54900_e84790_d_n0, assign54900_e84790_d_n2, assign54900_e84790_d_n4, assign54900_e84790_d_n5, assign54900_e84790_d_n6, assign54900_e84790_d_n7, assign54900_e84790_d_n8, assign54900_e84790_d_n9, assign54900_e84790_d_n10, assign54900_e84790_d_n11, assign54900_e84790_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 != 0.0)) {
        let assign54900_e84785: f64 = (1.0 + locals.var_tx);
        let assign54900_e84787: f64 = (assign54900_e84785 - 500.0);
        let assign54900_e84788: f64 = (1.403592217853e217 * assign54900_e84787);
        (assign54900_e84788, (1.403592217853e217 * locals.var_tx_dn0), (1.403592217853e217 * locals.var_tx_dn2), (1.403592217853e217 * locals.var_tx_dn4), (1.403592217853e217 * locals.var_tx_dn5), (1.403592217853e217 * locals.var_tx_dn6), (1.403592217853e217 * locals.var_tx_dn7), (1.403592217853e217 * locals.var_tx_dn8), (1.403592217853e217 * locals.var_tx_dn9), (1.403592217853e217 * locals.var_tx_dn10), (1.403592217853e217 * locals.var_tx_dn11), (1.403592217853e217 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54900_e84790;
        locals.var_t1_dn0 = assign54900_e84790_d_n0;
        locals.var_t1_dn2 = assign54900_e84790_d_n2;
        locals.var_t1_dn4 = assign54900_e84790_d_n4;
        locals.var_t1_dn5 = assign54900_e84790_d_n5;
        locals.var_t1_dn6 = assign54900_e84790_d_n6;
        locals.var_t1_dn7 = assign54900_e84790_d_n7;
        locals.var_t1_dn8 = assign54900_e84790_d_n8;
        locals.var_t1_dn9 = assign54900_e84790_d_n9;
        locals.var_t1_dn10 = assign54900_e84790_d_n10;
        locals.var_t1_dn11 = assign54900_e84790_d_n11;
        locals.var_t1_dn14 = assign54900_e84790_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign54910_e84811, assign54910_e84811_d_n0, assign54910_e84811_d_n2, assign54910_e84811_d_n4, assign54910_e84811_d_n5, assign54910_e84811_d_n6, assign54910_e84811_d_n7, assign54910_e84811_d_n8, assign54910_e84811_d_n9, assign54910_e84811_d_n10, assign54910_e84811_d_n11, assign54910_e84811_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign54910_e84811;
        locals.var_t3_dn0 = assign54910_e84811_d_n0;
        locals.var_t3_dn2 = assign54910_e84811_d_n2;
        locals.var_t3_dn4 = assign54910_e84811_d_n4;
        locals.var_t3_dn5 = assign54910_e84811_d_n5;
        locals.var_t3_dn6 = assign54910_e84811_d_n6;
        locals.var_t3_dn7 = assign54910_e84811_d_n7;
        locals.var_t3_dn8 = assign54910_e84811_d_n8;
        locals.var_t3_dn9 = assign54910_e84811_d_n9;
        locals.var_t3_dn10 = assign54910_e84811_d_n10;
        locals.var_t3_dn11 = assign54910_e84811_d_n11;
        locals.var_t3_dn14 = assign54910_e84811_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign54920_e84833, assign54920_e84833_d_n0, assign54920_e84833_d_n2, assign54920_e84833_d_n4, assign54920_e84833_d_n5, assign54920_e84833_d_n6, assign54920_e84833_d_n7, assign54920_e84833_d_n8, assign54920_e84833_d_n9, assign54920_e84833_d_n10, assign54920_e84833_d_n11, assign54920_e84833_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 == 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign54920_e84833;
        locals.var_tmf1_dn0 = assign54920_e84833_d_n0;
        locals.var_tmf1_dn2 = assign54920_e84833_d_n2;
        locals.var_tmf1_dn4 = assign54920_e84833_d_n4;
        locals.var_tmf1_dn5 = assign54920_e84833_d_n5;
        locals.var_tmf1_dn6 = assign54920_e84833_d_n6;
        locals.var_tmf1_dn7 = assign54920_e84833_d_n7;
        locals.var_tmf1_dn8 = assign54920_e84833_d_n8;
        locals.var_tmf1_dn9 = assign54920_e84833_d_n9;
        locals.var_tmf1_dn10 = assign54920_e84833_d_n10;
        locals.var_tmf1_dn11 = assign54920_e84833_d_n11;
        locals.var_tmf1_dn14 = assign54920_e84833_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign54930_e84855, assign54930_e84855_d_n0, assign54930_e84855_d_n2, assign54930_e84855_d_n4, assign54930_e84855_d_n5, assign54930_e84855_d_n6, assign54930_e84855_d_n7, assign54930_e84855_d_n8, assign54930_e84855_d_n9, assign54930_e84855_d_n10, assign54930_e84855_d_n11, assign54930_e84855_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54930_e84855;
        locals.var_t1_dn0 = assign54930_e84855_d_n0;
        locals.var_t1_dn2 = assign54930_e84855_d_n2;
        locals.var_t1_dn4 = assign54930_e84855_d_n4;
        locals.var_t1_dn5 = assign54930_e84855_d_n5;
        locals.var_t1_dn6 = assign54930_e84855_d_n6;
        locals.var_t1_dn7 = assign54930_e84855_d_n7;
        locals.var_t1_dn8 = assign54930_e84855_d_n8;
        locals.var_t1_dn9 = assign54930_e84855_d_n9;
        locals.var_t1_dn10 = assign54930_e84855_d_n10;
        locals.var_t1_dn11 = assign54930_e84855_d_n11;
        locals.var_t1_dn14 = assign54930_e84855_d_n14;
        locals.var_t1_rv = 0.0;

        let mut assign54940_loop_guard: usize = 0;
        while {
            let assign54940_cond_e84878: f64 = if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign54940_cond_e84878 != 0.0
        } {
            assign54940_loop_guard += 1;
            assert!(assign54940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54940_body0_e84902, assign54940_body0_e84902_d_n0, assign54940_body0_e84902_d_n2, assign54940_body0_e84902_d_n4, assign54940_body0_e84902_d_n5, assign54940_body0_e84902_d_n6, assign54940_body0_e84902_d_n7, assign54940_body0_e84902_d_n8, assign54940_body0_e84902_d_n9, assign54940_body0_e84902_d_n10, assign54940_body0_e84902_d_n11, assign54940_body0_e84902_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 == 0.0)) {
        let assign54940_body0_e84900: f64 = (locals.var_t1 * 1.14200738981568e26);
        (assign54940_body0_e84900, (locals.var_t1_dn0 * 1.14200738981568e26), (locals.var_t1_dn2 * 1.14200738981568e26), (locals.var_t1_dn4 * 1.14200738981568e26), (locals.var_t1_dn5 * 1.14200738981568e26), (locals.var_t1_dn6 * 1.14200738981568e26), (locals.var_t1_dn7 * 1.14200738981568e26), (locals.var_t1_dn8 * 1.14200738981568e26), (locals.var_t1_dn9 * 1.14200738981568e26), (locals.var_t1_dn10 * 1.14200738981568e26), (locals.var_t1_dn11 * 1.14200738981568e26), (locals.var_t1_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign54940_body0_e84902;
            locals.var_t1_dn0 = assign54940_body0_e84902_d_n0;
            locals.var_t1_dn2 = assign54940_body0_e84902_d_n2;
            locals.var_t1_dn4 = assign54940_body0_e84902_d_n4;
            locals.var_t1_dn5 = assign54940_body0_e84902_d_n5;
            locals.var_t1_dn6 = assign54940_body0_e84902_d_n6;
            locals.var_t1_dn7 = assign54940_body0_e84902_d_n7;
            locals.var_t1_dn8 = assign54940_body0_e84902_d_n8;
            locals.var_t1_dn9 = assign54940_body0_e84902_d_n9;
            locals.var_t1_dn10 = assign54940_body0_e84902_d_n10;
            locals.var_t1_dn11 = assign54940_body0_e84902_d_n11;
            locals.var_t1_dn14 = assign54940_body0_e84902_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign54940_body1_e84926, assign54940_body1_e84926_d_n0, assign54940_body1_e84926_d_n2, assign54940_body1_e84926_d_n4, assign54940_body1_e84926_d_n5, assign54940_body1_e84926_d_n6, assign54940_body1_e84926_d_n7, assign54940_body1_e84926_d_n8, assign54940_body1_e84926_d_n9, assign54940_body1_e84926_d_n10, assign54940_body1_e84926_d_n11, assign54940_body1_e84926_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 == 0.0)) {
        let assign54940_body1_e84924: f64 = (locals.var_tmf1 - 60.0);
        (assign54940_body1_e84924, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign54940_body1_e84926;
            locals.var_tmf1_dn0 = assign54940_body1_e84926_d_n0;
            locals.var_tmf1_dn2 = assign54940_body1_e84926_d_n2;
            locals.var_tmf1_dn4 = assign54940_body1_e84926_d_n4;
            locals.var_tmf1_dn5 = assign54940_body1_e84926_d_n5;
            locals.var_tmf1_dn6 = assign54940_body1_e84926_d_n6;
            locals.var_tmf1_dn7 = assign54940_body1_e84926_d_n7;
            locals.var_tmf1_dn8 = assign54940_body1_e84926_d_n8;
            locals.var_tmf1_dn9 = assign54940_body1_e84926_d_n9;
            locals.var_tmf1_dn10 = assign54940_body1_e84926_d_n10;
            locals.var_tmf1_dn11 = assign54940_body1_e84926_d_n11;
            locals.var_tmf1_dn14 = assign54940_body1_e84926_d_n14;
            locals.var_tmf1_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_200(
        locals: &mut StampLocals,
    ) {
        let (assign54950_e84951, assign54950_e84951_d_n0, assign54950_e84951_d_n2, assign54950_e84951_d_n4, assign54950_e84951_d_n5, assign54950_e84951_d_n6, assign54950_e84951_d_n7, assign54950_e84951_d_n8, assign54950_e84951_d_n9, assign54950_e84951_d_n10, assign54950_e84951_d_n11, assign54950_e84951_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 == 0.0)) {
        let assign54950_e84948: f64 = (locals.var_tmf1).exp();
        let assign54950_e84949: f64 = (locals.var_t1 * assign54950_e84948);
        (assign54950_e84949, ((locals.var_t1_dn0 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn0))), ((locals.var_t1_dn2 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn2))), ((locals.var_t1_dn4 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn4))), ((locals.var_t1_dn5 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn5))), ((locals.var_t1_dn6 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn6))), ((locals.var_t1_dn7 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn7))), ((locals.var_t1_dn8 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn8))), ((locals.var_t1_dn9 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn9))), ((locals.var_t1_dn10 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn10))), ((locals.var_t1_dn11 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn11))), ((locals.var_t1_dn14 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54950_e84951;
        locals.var_t1_dn0 = assign54950_e84951_d_n0;
        locals.var_t1_dn2 = assign54950_e84951_d_n2;
        locals.var_t1_dn4 = assign54950_e84951_d_n4;
        locals.var_t1_dn5 = assign54950_e84951_d_n5;
        locals.var_t1_dn6 = assign54950_e84951_d_n6;
        locals.var_t1_dn7 = assign54950_e84951_d_n7;
        locals.var_t1_dn8 = assign54950_e84951_d_n8;
        locals.var_t1_dn9 = assign54950_e84951_d_n9;
        locals.var_t1_dn10 = assign54950_e84951_d_n10;
        locals.var_t1_dn11 = assign54950_e84951_d_n11;
        locals.var_t1_dn14 = assign54950_e84951_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign54960_e84973, assign54960_e84973_d_n0, assign54960_e84973_d_n2, assign54960_e84973_d_n4, assign54960_e84973_d_n5, assign54960_e84973_d_n6, assign54960_e84973_d_n7, assign54960_e84973_d_n8, assign54960_e84973_d_n9, assign54960_e84973_d_n10, assign54960_e84973_d_n11, assign54960_e84973_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign54960_e84973;
        locals.var_t3_dn0 = assign54960_e84973_d_n0;
        locals.var_t3_dn2 = assign54960_e84973_d_n2;
        locals.var_t3_dn4 = assign54960_e84973_d_n4;
        locals.var_t3_dn5 = assign54960_e84973_d_n5;
        locals.var_t3_dn6 = assign54960_e84973_d_n6;
        locals.var_t3_dn7 = assign54960_e84973_d_n7;
        locals.var_t3_dn8 = assign54960_e84973_d_n8;
        locals.var_t3_dn9 = assign54960_e84973_d_n9;
        locals.var_t3_dn10 = assign54960_e84973_d_n10;
        locals.var_t3_dn11 = assign54960_e84973_d_n11;
        locals.var_t3_dn14 = assign54960_e84973_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign54970_e84994, assign54970_e84994_d_n0, assign54970_e84994_d_n2, assign54970_e84994_d_n4, assign54970_e84994_d_n5, assign54970_e84994_d_n6, assign54970_e84994_d_n7, assign54970_e84994_d_n8, assign54970_e84994_d_n9, assign54970_e84994_d_n10, assign54970_e84994_d_n11, assign54970_e84994_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) {
        let assign54970_e84992: f64 = (locals.var_t1 * locals.var_t0);
        (assign54970_e84992, ((locals.var_t1_dn0 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn0)), ((locals.var_t1_dn2 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn2)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn11 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn11)), ((locals.var_t1_dn14 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54970_e84994;
        locals.var_t1_dn0 = assign54970_e84994_d_n0;
        locals.var_t1_dn2 = assign54970_e84994_d_n2;
        locals.var_t1_dn4 = assign54970_e84994_d_n4;
        locals.var_t1_dn5 = assign54970_e84994_d_n5;
        locals.var_t1_dn6 = assign54970_e84994_d_n6;
        locals.var_t1_dn7 = assign54970_e84994_d_n7;
        locals.var_t1_dn8 = assign54970_e84994_d_n8;
        locals.var_t1_dn9 = assign54970_e84994_d_n9;
        locals.var_t1_dn10 = assign54970_e84994_d_n10;
        locals.var_t1_dn11 = assign54970_e84994_d_n11;
        locals.var_t1_dn14 = assign54970_e84994_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign54980_e85015, assign54980_e85015_d_n0, assign54980_e85015_d_n2, assign54980_e85015_d_n4, assign54980_e85015_d_n5, assign54980_e85015_d_n6, assign54980_e85015_d_n7, assign54980_e85015_d_n8, assign54980_e85015_d_n9, assign54980_e85015_d_n10, assign54980_e85015_d_n11, assign54980_e85015_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) {
        let assign54980_e85013: f64 = (locals.var_t1 - locals.var_t0);
        (assign54980_e85013, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign54980_e85015;
        locals.var_t2_dn0 = assign54980_e85015_d_n0;
        locals.var_t2_dn2 = assign54980_e85015_d_n2;
        locals.var_t2_dn4 = assign54980_e85015_d_n4;
        locals.var_t2_dn5 = assign54980_e85015_d_n5;
        locals.var_t2_dn6 = assign54980_e85015_d_n6;
        locals.var_t2_dn7 = assign54980_e85015_d_n7;
        locals.var_t2_dn8 = assign54980_e85015_d_n8;
        locals.var_t2_dn9 = assign54980_e85015_d_n9;
        locals.var_t2_dn10 = assign54980_e85015_d_n10;
        locals.var_t2_dn11 = assign54980_e85015_d_n11;
        locals.var_t2_dn14 = assign54980_e85015_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign54990_e85039, assign54990_e85039_d_n0, assign54990_e85039_d_n2, assign54990_e85039_d_n4, assign54990_e85039_d_n5, assign54990_e85039_d_n6, assign54990_e85039_d_n7, assign54990_e85039_d_n8, assign54990_e85039_d_n9, assign54990_e85039_d_n10, assign54990_e85039_d_n11, assign54990_e85039_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 == 0.0)) {
        let assign54990_e85035: f64 = (1.0 + locals.var_tx);
        let assign54990_e85037: f64 = (assign54990_e85035 * locals.var_t0);
        (assign54990_e85037, ((locals.var_tx_dn0 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn0)), ((locals.var_tx_dn2 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn2)), ((locals.var_tx_dn4 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn4)), ((locals.var_tx_dn5 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn5)), ((locals.var_tx_dn6 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn6)), ((locals.var_tx_dn7 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn7)), ((locals.var_tx_dn8 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn8)), ((locals.var_tx_dn9 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn9)), ((locals.var_tx_dn10 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn10)), ((locals.var_tx_dn11 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn11)), ((locals.var_tx_dn14 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54990_e85039;
        locals.var_t1_dn0 = assign54990_e85039_d_n0;
        locals.var_t1_dn2 = assign54990_e85039_d_n2;
        locals.var_t1_dn4 = assign54990_e85039_d_n4;
        locals.var_t1_dn5 = assign54990_e85039_d_n5;
        locals.var_t1_dn6 = assign54990_e85039_d_n6;
        locals.var_t1_dn7 = assign54990_e85039_d_n7;
        locals.var_t1_dn8 = assign54990_e85039_d_n8;
        locals.var_t1_dn9 = assign54990_e85039_d_n9;
        locals.var_t1_dn10 = assign54990_e85039_d_n10;
        locals.var_t1_dn11 = assign54990_e85039_d_n11;
        locals.var_t1_dn14 = assign54990_e85039_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign55000_e85067, assign55000_e85067_d_n0, assign55000_e85067_d_n2, assign55000_e85067_d_n4, assign55000_e85067_d_n5, assign55000_e85067_d_n6, assign55000_e85067_d_n7, assign55000_e85067_d_n8, assign55000_e85067_d_n9, assign55000_e85067_d_n10, assign55000_e85067_d_n11, assign55000_e85067_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 == 0.0)) {
        let assign55000_e85061: f64 = (locals.var_tx / 2.0);
        let assign55000_e85062: f64 = (1.0 + assign55000_e85061);
        let assign55000_e85063: f64 = (locals.var_tx * assign55000_e85062);
        let assign55000_e85065: f64 = (assign55000_e85063 * locals.var_t0);
        (assign55000_e85065, ((((locals.var_tx_dn0 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn0 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn0)), ((((locals.var_tx_dn2 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn2 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn2)), ((((locals.var_tx_dn4 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn4 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn4)), ((((locals.var_tx_dn5 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn5 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn5)), ((((locals.var_tx_dn6 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn6 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn6)), ((((locals.var_tx_dn7 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn7 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn7)), ((((locals.var_tx_dn8 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn8 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn8)), ((((locals.var_tx_dn9 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn9 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn9)), ((((locals.var_tx_dn10 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn10 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn10)), ((((locals.var_tx_dn11 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn11 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn11)), ((((locals.var_tx_dn14 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn14 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign55000_e85067;
        locals.var_t2_dn0 = assign55000_e85067_d_n0;
        locals.var_t2_dn2 = assign55000_e85067_d_n2;
        locals.var_t2_dn4 = assign55000_e85067_d_n4;
        locals.var_t2_dn5 = assign55000_e85067_d_n5;
        locals.var_t2_dn6 = assign55000_e85067_d_n6;
        locals.var_t2_dn7 = assign55000_e85067_d_n7;
        locals.var_t2_dn8 = assign55000_e85067_d_n8;
        locals.var_t2_dn9 = assign55000_e85067_d_n9;
        locals.var_t2_dn10 = assign55000_e85067_d_n10;
        locals.var_t2_dn11 = assign55000_e85067_d_n11;
        locals.var_t2_dn14 = assign55000_e85067_d_n14;
        locals.var_t2_rv = 0.0;

        let assign55010_e85069: f64 = (locals.var_t2).abs();
        let assign55010_e85071: f64 = if assign55010_e85069 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1387 = assign55010_e85071;
        locals.var_guard1387_rv = 0.0;

        let (assign55020_e85095, assign55020_e85095_d_n0, assign55020_e85095_d_n2, assign55020_e85095_d_n4, assign55020_e85095_d_n5, assign55020_e85095_d_n6, assign55020_e85095_d_n7, assign55020_e85095_d_n8, assign55020_e85095_d_n9, assign55020_e85095_d_n10, assign55020_e85095_d_n11, assign55020_e85095_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1387 != 0.0)) {
        let assign55020_e85090: f64 = (1.0 + locals.var_t2);
        let assign55020_e85091: f64 = (assign55020_e85090).ln();
        let assign55020_e85093: f64 = (assign55020_e85091 / locals.var_c_sb__blk1323);
        (assign55020_e85093, ((((locals.var_t2_dn0 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn0)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn2 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn2)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn4 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn4)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn5 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn5)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn6 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn6)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn7 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn7)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn8 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn8)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn9 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn9)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn10 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn10)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn11 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn11)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn14 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn14)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)),)
    } else {
        (locals.var_pb0dep__blk1167, locals.var_pb0dep__blk1167_dn0, locals.var_pb0dep__blk1167_dn2, locals.var_pb0dep__blk1167_dn4, locals.var_pb0dep__blk1167_dn5, locals.var_pb0dep__blk1167_dn6, locals.var_pb0dep__blk1167_dn7, locals.var_pb0dep__blk1167_dn8, locals.var_pb0dep__blk1167_dn9, locals.var_pb0dep__blk1167_dn10, locals.var_pb0dep__blk1167_dn11, locals.var_pb0dep__blk1167_dn14,)
    }
};
        locals.var_pb0dep__blk1167 = assign55020_e85095;
        locals.var_pb0dep__blk1167_dn0 = assign55020_e85095_d_n0;
        locals.var_pb0dep__blk1167_dn2 = assign55020_e85095_d_n2;
        locals.var_pb0dep__blk1167_dn4 = assign55020_e85095_d_n4;
        locals.var_pb0dep__blk1167_dn5 = assign55020_e85095_d_n5;
        locals.var_pb0dep__blk1167_dn6 = assign55020_e85095_d_n6;
        locals.var_pb0dep__blk1167_dn7 = assign55020_e85095_d_n7;
        locals.var_pb0dep__blk1167_dn8 = assign55020_e85095_d_n8;
        locals.var_pb0dep__blk1167_dn9 = assign55020_e85095_d_n9;
        locals.var_pb0dep__blk1167_dn10 = assign55020_e85095_d_n10;
        locals.var_pb0dep__blk1167_dn11 = assign55020_e85095_d_n11;
        locals.var_pb0dep__blk1167_dn14 = assign55020_e85095_d_n14;
        locals.var_pb0dep__blk1167_rv = 0.0;

        let (assign55030_e85117, assign55030_e85117_d_n0, assign55030_e85117_d_n2, assign55030_e85117_d_n4, assign55030_e85117_d_n5, assign55030_e85117_d_n6, assign55030_e85117_d_n7, assign55030_e85117_d_n8, assign55030_e85117_d_n9, assign55030_e85117_d_n10, assign55030_e85117_d_n11, assign55030_e85117_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1387 == 0.0)) {
        let assign55030_e85115: f64 = (locals.var_t2 / locals.var_c_sb__blk1323);
        (assign55030_e85115, (((locals.var_t2_dn0 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn0)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn2 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn2)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn4 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn4)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn5 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn5)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn6 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn6)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn7 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn7)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn8 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn8)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn9 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn9)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn10 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn10)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn11 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn11)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn14 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn14)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)),)
    } else {
        (locals.var_pb0dep__blk1167, locals.var_pb0dep__blk1167_dn0, locals.var_pb0dep__blk1167_dn2, locals.var_pb0dep__blk1167_dn4, locals.var_pb0dep__blk1167_dn5, locals.var_pb0dep__blk1167_dn6, locals.var_pb0dep__blk1167_dn7, locals.var_pb0dep__blk1167_dn8, locals.var_pb0dep__blk1167_dn9, locals.var_pb0dep__blk1167_dn10, locals.var_pb0dep__blk1167_dn11, locals.var_pb0dep__blk1167_dn14,)
    }
};
        locals.var_pb0dep__blk1167 = assign55030_e85117;
        locals.var_pb0dep__blk1167_dn0 = assign55030_e85117_d_n0;
        locals.var_pb0dep__blk1167_dn2 = assign55030_e85117_d_n2;
        locals.var_pb0dep__blk1167_dn4 = assign55030_e85117_d_n4;
        locals.var_pb0dep__blk1167_dn5 = assign55030_e85117_d_n5;
        locals.var_pb0dep__blk1167_dn6 = assign55030_e85117_d_n6;
        locals.var_pb0dep__blk1167_dn7 = assign55030_e85117_d_n7;
        locals.var_pb0dep__blk1167_dn8 = assign55030_e85117_d_n8;
        locals.var_pb0dep__blk1167_dn9 = assign55030_e85117_d_n9;
        locals.var_pb0dep__blk1167_dn10 = assign55030_e85117_d_n10;
        locals.var_pb0dep__blk1167_dn11 = assign55030_e85117_d_n11;
        locals.var_pb0dep__blk1167_dn14 = assign55030_e85117_d_n14;
        locals.var_pb0dep__blk1167_rv = 0.0;

        let (assign55040_e85133, assign55040_e85133_d_n0, assign55040_e85133_d_n2, assign55040_e85133_d_n4, assign55040_e85133_d_n5, assign55040_e85133_d_n6, assign55040_e85133_d_n7, assign55040_e85133_d_n8, assign55040_e85133_d_n9, assign55040_e85133_d_n10, assign55040_e85133_d_n11, assign55040_e85133_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign55040_e85131: f64 = (locals.var_ps0dep - locals.var_pb0dep__blk1167);
        (assign55040_e85131, (locals.var_ps0dep_dn0 - locals.var_pb0dep__blk1167_dn0), (locals.var_ps0dep_dn2 - locals.var_pb0dep__blk1167_dn2), (locals.var_ps0dep_dn4 - locals.var_pb0dep__blk1167_dn4), (locals.var_ps0dep_dn5 - locals.var_pb0dep__blk1167_dn5), (locals.var_ps0dep_dn6 - locals.var_pb0dep__blk1167_dn6), (locals.var_ps0dep_dn7 - locals.var_pb0dep__blk1167_dn7), (locals.var_ps0dep_dn8 - locals.var_pb0dep__blk1167_dn8), (locals.var_ps0dep_dn9 - locals.var_pb0dep__blk1167_dn9), (locals.var_ps0dep_dn10 - locals.var_pb0dep__blk1167_dn10), (locals.var_ps0dep_dn11 - locals.var_pb0dep__blk1167_dn11), (locals.var_ps0dep_dn14 - locals.var_pb0dep__blk1167_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign55040_e85133;
        locals.var_t2_dn0 = assign55040_e85133_d_n0;
        locals.var_t2_dn2 = assign55040_e85133_d_n2;
        locals.var_t2_dn4 = assign55040_e85133_d_n4;
        locals.var_t2_dn5 = assign55040_e85133_d_n5;
        locals.var_t2_dn6 = assign55040_e85133_d_n6;
        locals.var_t2_dn7 = assign55040_e85133_d_n7;
        locals.var_t2_dn8 = assign55040_e85133_d_n8;
        locals.var_t2_dn9 = assign55040_e85133_d_n9;
        locals.var_t2_dn10 = assign55040_e85133_d_n10;
        locals.var_t2_dn11 = assign55040_e85133_d_n11;
        locals.var_t2_dn14 = assign55040_e85133_d_n14;
        locals.var_t2_rv = 0.0;

        let assign55050_e85136: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1388 = assign55050_e85136;
        locals.var_guard1388_rv = 0.0;

        let (assign55060_e85165, assign55060_e85165_d_n0, assign55060_e85165_d_n2, assign55060_e85165_d_n4, assign55060_e85165_d_n5, assign55060_e85165_d_n6, assign55060_e85165_d_n7, assign55060_e85165_d_n8, assign55060_e85165_d_n9, assign55060_e85165_d_n10, assign55060_e85165_d_n11, assign55060_e85165_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let (assign55060_e85163, assign55060_e85163_d_n0, assign55060_e85163_d_n2, assign55060_e85163_d_n4, assign55060_e85163_d_n5, assign55060_e85163_d_n6, assign55060_e85163_d_n7, assign55060_e85163_d_n8, assign55060_e85163_d_n9, assign55060_e85163_d_n10, assign55060_e85163_d_n11, assign55060_e85163_d_n14,) = {
            if (locals.var_t2 < 0.0) {
                let assign55060_e85154: f64 = (-locals.var_c_2esipq_ndepm__blk1138);
                let assign55060_e85156: f64 = (assign55060_e85154 * locals.var_t2);
                let assign55060_e85157: f64 = (assign55060_e85156).sqrt();
                let assign55060_e85158: f64 = (-assign55060_e85157);
                (assign55060_e85158, (-((((-locals.var_c_2esipq_ndepm__blk1138_dn0) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn0)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn2) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn2)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn4) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn4)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn5) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn5)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn6) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn6)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn7) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn7)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn8) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn8)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn9) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn9)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn10) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn10)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn11) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn11)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn14) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn14)) / (2.0 * assign55060_e85157))),)
            } else {
                let assign55060_e85161: f64 = (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2);
                let assign55060_e85162: f64 = (assign55060_e85161).sqrt();
                (assign55060_e85162, (((locals.var_c_2esipq_ndepm__blk1138_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn0)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn2)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn4)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn5)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn6)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn7)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn8)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn9)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn10)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn11)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn14)) / (2.0 * assign55060_e85162)),)
            }
        };
        (assign55060_e85163, assign55060_e85163_d_n0, assign55060_e85163_d_n2, assign55060_e85163_d_n4, assign55060_e85163_d_n5, assign55060_e85163_d_n6, assign55060_e85163_d_n7, assign55060_e85163_d_n8, assign55060_e85163_d_n9, assign55060_e85163_d_n10, assign55060_e85163_d_n11, assign55060_e85163_d_n14,)
    } else {
        (locals.var_ws__blk1149, locals.var_ws__blk1149_dn0, locals.var_ws__blk1149_dn2, locals.var_ws__blk1149_dn4, locals.var_ws__blk1149_dn5, locals.var_ws__blk1149_dn6, locals.var_ws__blk1149_dn7, locals.var_ws__blk1149_dn8, locals.var_ws__blk1149_dn9, locals.var_ws__blk1149_dn10, locals.var_ws__blk1149_dn11, locals.var_ws__blk1149_dn14,)
    }
};
        locals.var_ws__blk1149 = assign55060_e85165;
        locals.var_ws__blk1149_dn0 = assign55060_e85165_d_n0;
        locals.var_ws__blk1149_dn2 = assign55060_e85165_d_n2;
        locals.var_ws__blk1149_dn4 = assign55060_e85165_d_n4;
        locals.var_ws__blk1149_dn5 = assign55060_e85165_d_n5;
        locals.var_ws__blk1149_dn6 = assign55060_e85165_d_n6;
        locals.var_ws__blk1149_dn7 = assign55060_e85165_d_n7;
        locals.var_ws__blk1149_dn8 = assign55060_e85165_d_n8;
        locals.var_ws__blk1149_dn9 = assign55060_e85165_d_n9;
        locals.var_ws__blk1149_dn10 = assign55060_e85165_d_n10;
        locals.var_ws__blk1149_dn11 = assign55060_e85165_d_n11;
        locals.var_ws__blk1149_dn14 = assign55060_e85165_d_n14;
        locals.var_ws__blk1149_rv = 0.0;

        let assign55070_e85168: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1389 = assign55070_e85168;
        locals.var_guard1389_rv = 0.0;

        let (assign55080_e85189, assign55080_e85189_d_n0, assign55080_e85189_d_n2, assign55080_e85189_d_n4, assign55080_e85189_d_n5, assign55080_e85189_d_n6, assign55080_e85189_d_n7, assign55080_e85189_d_n8, assign55080_e85189_d_n9, assign55080_e85189_d_n10, assign55080_e85189_d_n11, assign55080_e85189_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1389 != 0.0)) {
        let assign55080_e85187: f64 = (locals.var_beta * locals.var_t2);
        (assign55080_e85187, ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)), ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)), ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)), ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)), ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)), ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)), ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)), ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)), ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)), ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)), ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign55080_e85189;
        locals.var_t3_dn0 = assign55080_e85189_d_n0;
        locals.var_t3_dn2 = assign55080_e85189_d_n2;
        locals.var_t3_dn4 = assign55080_e85189_d_n4;
        locals.var_t3_dn5 = assign55080_e85189_d_n5;
        locals.var_t3_dn6 = assign55080_e85189_d_n6;
        locals.var_t3_dn7 = assign55080_e85189_d_n7;
        locals.var_t3_dn8 = assign55080_e85189_d_n8;
        locals.var_t3_dn9 = assign55080_e85189_d_n9;
        locals.var_t3_dn10 = assign55080_e85189_d_n10;
        locals.var_t3_dn11 = assign55080_e85189_d_n11;
        locals.var_t3_dn14 = assign55080_e85189_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign55090_e85219, assign55090_e85219_d_n0, assign55090_e85219_d_n2, assign55090_e85219_d_n4, assign55090_e85219_d_n5, assign55090_e85219_d_n6, assign55090_e85219_d_n7, assign55090_e85219_d_n8, assign55090_e85219_d_n9, assign55090_e85219_d_n10, assign55090_e85219_d_n11, assign55090_e85219_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1389 != 0.0)) {
        let assign55090_e85208: f64 = (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv);
        let assign55090_e85210: f64 = (locals.var_t3).exp();
        let assign55090_e85212: f64 = (assign55090_e85210 - locals.var_t3);
        let assign55090_e85214: f64 = (assign55090_e85212 - 1.0);
        let assign55090_e85215: f64 = (assign55090_e85208 * assign55090_e85214);
        let assign55090_e85216: f64 = (assign55090_e85215).sqrt();
        let assign55090_e85217: f64 = (-assign55090_e85216);
        (assign55090_e85217, (-(((((locals.var_c_2esipq_ndepm__blk1138_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn0)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn2)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn4)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn5)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn6)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn7)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn8)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn9)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn10)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn11 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn11)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn11) - locals.var_t3_dn11))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn14 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn14)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn14) - locals.var_t3_dn14))) / (2.0 * assign55090_e85216))),)
    } else {
        (locals.var_ws__blk1149, locals.var_ws__blk1149_dn0, locals.var_ws__blk1149_dn2, locals.var_ws__blk1149_dn4, locals.var_ws__blk1149_dn5, locals.var_ws__blk1149_dn6, locals.var_ws__blk1149_dn7, locals.var_ws__blk1149_dn8, locals.var_ws__blk1149_dn9, locals.var_ws__blk1149_dn10, locals.var_ws__blk1149_dn11, locals.var_ws__blk1149_dn14,)
    }
};
        locals.var_ws__blk1149 = assign55090_e85219;
        locals.var_ws__blk1149_dn0 = assign55090_e85219_d_n0;
        locals.var_ws__blk1149_dn2 = assign55090_e85219_d_n2;
        locals.var_ws__blk1149_dn4 = assign55090_e85219_d_n4;
        locals.var_ws__blk1149_dn5 = assign55090_e85219_d_n5;
        locals.var_ws__blk1149_dn6 = assign55090_e85219_d_n6;
        locals.var_ws__blk1149_dn7 = assign55090_e85219_d_n7;
        locals.var_ws__blk1149_dn8 = assign55090_e85219_d_n8;
        locals.var_ws__blk1149_dn9 = assign55090_e85219_d_n9;
        locals.var_ws__blk1149_dn10 = assign55090_e85219_d_n10;
        locals.var_ws__blk1149_dn11 = assign55090_e85219_d_n11;
        locals.var_ws__blk1149_dn14 = assign55090_e85219_d_n14;
        locals.var_ws__blk1149_rv = 0.0;

        let (assign55100_e85242, assign55100_e85242_d_n0, assign55100_e85242_d_n2, assign55100_e85242_d_n4, assign55100_e85242_d_n5, assign55100_e85242_d_n6, assign55100_e85242_d_n7, assign55100_e85242_d_n8, assign55100_e85242_d_n9, assign55100_e85242_d_n10, assign55100_e85242_d_n11, assign55100_e85242_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1389 == 0.0)) {
        let assign55100_e85238: f64 = (-locals.var_beta);
        let assign55100_e85240: f64 = (assign55100_e85238 * locals.var_t2);
        (assign55100_e85240, (((-locals.var_beta_dn0) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn0)), (((-locals.var_beta_dn2) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn2)), (((-locals.var_beta_dn4) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn4)), (((-locals.var_beta_dn5) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn5)), (((-locals.var_beta_dn6) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn6)), (((-locals.var_beta_dn7) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn7)), (((-locals.var_beta_dn8) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn8)), (((-locals.var_beta_dn9) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn9)), (((-locals.var_beta_dn10) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn10)), (((-locals.var_beta_dn11) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn11)), (((-locals.var_beta_dn14) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign55100_e85242;
        locals.var_t3_dn0 = assign55100_e85242_d_n0;
        locals.var_t3_dn2 = assign55100_e85242_d_n2;
        locals.var_t3_dn4 = assign55100_e85242_d_n4;
        locals.var_t3_dn5 = assign55100_e85242_d_n5;
        locals.var_t3_dn6 = assign55100_e85242_d_n6;
        locals.var_t3_dn7 = assign55100_e85242_d_n7;
        locals.var_t3_dn8 = assign55100_e85242_d_n8;
        locals.var_t3_dn9 = assign55100_e85242_d_n9;
        locals.var_t3_dn10 = assign55100_e85242_d_n10;
        locals.var_t3_dn11 = assign55100_e85242_d_n11;
        locals.var_t3_dn14 = assign55100_e85242_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign55110_e85272, assign55110_e85272_d_n0, assign55110_e85272_d_n2, assign55110_e85272_d_n4, assign55110_e85272_d_n5, assign55110_e85272_d_n6, assign55110_e85272_d_n7, assign55110_e85272_d_n8, assign55110_e85272_d_n9, assign55110_e85272_d_n10, assign55110_e85272_d_n11, assign55110_e85272_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1389 == 0.0)) {
        let assign55110_e85262: f64 = (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv);
        let assign55110_e85264: f64 = (locals.var_t3).exp();
        let assign55110_e85266: f64 = (assign55110_e85264 - locals.var_t3);
        let assign55110_e85268: f64 = (assign55110_e85266 - 1.0);
        let assign55110_e85269: f64 = (assign55110_e85262 * assign55110_e85268);
        let assign55110_e85270: f64 = (assign55110_e85269).sqrt();
        (assign55110_e85270, (((((locals.var_c_2esipq_ndepm__blk1138_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn0)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn2)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn4)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn5)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn6)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn7)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn8)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn9)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn10)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn11 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn11)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn11) - locals.var_t3_dn11))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn14 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn14)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn14) - locals.var_t3_dn14))) / (2.0 * assign55110_e85270)),)
    } else {
        (locals.var_ws__blk1149, locals.var_ws__blk1149_dn0, locals.var_ws__blk1149_dn2, locals.var_ws__blk1149_dn4, locals.var_ws__blk1149_dn5, locals.var_ws__blk1149_dn6, locals.var_ws__blk1149_dn7, locals.var_ws__blk1149_dn8, locals.var_ws__blk1149_dn9, locals.var_ws__blk1149_dn10, locals.var_ws__blk1149_dn11, locals.var_ws__blk1149_dn14,)
    }
};
        locals.var_ws__blk1149 = assign55110_e85272;
        locals.var_ws__blk1149_dn0 = assign55110_e85272_d_n0;
        locals.var_ws__blk1149_dn2 = assign55110_e85272_d_n2;
        locals.var_ws__blk1149_dn4 = assign55110_e85272_d_n4;
        locals.var_ws__blk1149_dn5 = assign55110_e85272_d_n5;
        locals.var_ws__blk1149_dn6 = assign55110_e85272_d_n6;
        locals.var_ws__blk1149_dn7 = assign55110_e85272_d_n7;
        locals.var_ws__blk1149_dn8 = assign55110_e85272_d_n8;
        locals.var_ws__blk1149_dn9 = assign55110_e85272_d_n9;
        locals.var_ws__blk1149_dn10 = assign55110_e85272_d_n10;
        locals.var_ws__blk1149_dn11 = assign55110_e85272_d_n11;
        locals.var_ws__blk1149_dn14 = assign55110_e85272_d_n14;
        locals.var_ws__blk1149_rv = 0.0;

        let (assign55120_e85288, assign55120_e85288_d_n0, assign55120_e85288_d_n2, assign55120_e85288_d_n4, assign55120_e85288_d_n5, assign55120_e85288_d_n6, assign55120_e85288_d_n7, assign55120_e85288_d_n8, assign55120_e85288_d_n9, assign55120_e85288_d_n10, assign55120_e85288_d_n11, assign55120_e85288_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign55120_e85286: f64 = (locals.var_tnp__blk1150 - locals.var_ws__blk1149);
        (assign55120_e85286, (locals.var_tnp__blk1150_dn0 - locals.var_ws__blk1149_dn0), (locals.var_tnp__blk1150_dn2 - locals.var_ws__blk1149_dn2), (locals.var_tnp__blk1150_dn4 - locals.var_ws__blk1149_dn4), (locals.var_tnp__blk1150_dn5 - locals.var_ws__blk1149_dn5), (locals.var_tnp__blk1150_dn6 - locals.var_ws__blk1149_dn6), (locals.var_tnp__blk1150_dn7 - locals.var_ws__blk1149_dn7), (locals.var_tnp__blk1150_dn8 - locals.var_ws__blk1149_dn8), (locals.var_tnp__blk1150_dn9 - locals.var_ws__blk1149_dn9), (locals.var_tnp__blk1150_dn10 - locals.var_ws__blk1149_dn10), (locals.var_tnp__blk1150_dn11 - locals.var_ws__blk1149_dn11), (locals.var_tnp__blk1150_dn14 - locals.var_ws__blk1149_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign55120_e85288;
        locals.var_w_res_dn0 = assign55120_e85288_d_n0;
        locals.var_w_res_dn2 = assign55120_e85288_d_n2;
        locals.var_w_res_dn4 = assign55120_e85288_d_n4;
        locals.var_w_res_dn5 = assign55120_e85288_d_n5;
        locals.var_w_res_dn6 = assign55120_e85288_d_n6;
        locals.var_w_res_dn7 = assign55120_e85288_d_n7;
        locals.var_w_res_dn8 = assign55120_e85288_d_n8;
        locals.var_w_res_dn9 = assign55120_e85288_d_n9;
        locals.var_w_res_dn10 = assign55120_e85288_d_n10;
        locals.var_w_res_dn11 = assign55120_e85288_d_n11;
        locals.var_w_res_dn14 = assign55120_e85288_d_n14;
        locals.var_w_res_rv = 0.0;

        let assign55130_e85292: f64 = 1e-16;
        let assign55130_e85297: f64 = if ((locals.var_w_res < assign55130_e85292) && (1e-16 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1390 = assign55130_e85297;
        locals.var_guard1390_rv = 0.0;

        let (assign55140_e85317, assign55140_e85317_d_n0, assign55140_e85317_d_n2, assign55140_e85317_d_n4, assign55140_e85317_d_n5, assign55140_e85317_d_n6, assign55140_e85317_d_n7, assign55140_e85317_d_n8, assign55140_e85317_d_n9, assign55140_e85317_d_n10, assign55140_e85317_d_n11, assign55140_e85317_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55140_e85313: f64 = 1e-16;
        let assign55140_e85315: f64 = (assign55140_e85313 - locals.var_w_res);
        (assign55140_e85315, (-locals.var_w_res_dn0), (-locals.var_w_res_dn2), (-locals.var_w_res_dn4), (-locals.var_w_res_dn5), (-locals.var_w_res_dn6), (-locals.var_w_res_dn7), (-locals.var_w_res_dn8), (-locals.var_w_res_dn9), (-locals.var_w_res_dn10), (-locals.var_w_res_dn11), (-locals.var_w_res_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign55140_e85317;
        locals.var_tmf1_dn0 = assign55140_e85317_d_n0;
        locals.var_tmf1_dn2 = assign55140_e85317_d_n2;
        locals.var_tmf1_dn4 = assign55140_e85317_d_n4;
        locals.var_tmf1_dn5 = assign55140_e85317_d_n5;
        locals.var_tmf1_dn6 = assign55140_e85317_d_n6;
        locals.var_tmf1_dn7 = assign55140_e85317_d_n7;
        locals.var_tmf1_dn8 = assign55140_e85317_d_n8;
        locals.var_tmf1_dn9 = assign55140_e85317_d_n9;
        locals.var_tmf1_dn10 = assign55140_e85317_d_n10;
        locals.var_tmf1_dn11 = assign55140_e85317_d_n11;
        locals.var_tmf1_dn14 = assign55140_e85317_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign55150_e85335, assign55150_e85335_d_n0, assign55150_e85335_d_n2, assign55150_e85335_d_n4, assign55150_e85335_d_n5, assign55150_e85335_d_n6, assign55150_e85335_d_n7, assign55150_e85335_d_n8, assign55150_e85335_d_n9, assign55150_e85335_d_n10, assign55150_e85335_d_n11, assign55150_e85335_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55150_e85333: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign55150_e85333, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign55150_e85335;
        locals.var_x2_dn0 = assign55150_e85335_d_n0;
        locals.var_x2_dn2 = assign55150_e85335_d_n2;
        locals.var_x2_dn4 = assign55150_e85335_d_n4;
        locals.var_x2_dn5 = assign55150_e85335_d_n5;
        locals.var_x2_dn6 = assign55150_e85335_d_n6;
        locals.var_x2_dn7 = assign55150_e85335_d_n7;
        locals.var_x2_dn8 = assign55150_e85335_d_n8;
        locals.var_x2_dn9 = assign55150_e85335_d_n9;
        locals.var_x2_dn10 = assign55150_e85335_d_n10;
        locals.var_x2_dn11 = assign55150_e85335_d_n11;
        locals.var_x2_dn14 = assign55150_e85335_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign55160_e85353, assign55160_e85353_d_n0, assign55160_e85353_d_n2, assign55160_e85353_d_n4, assign55160_e85353_d_n5, assign55160_e85353_d_n6, assign55160_e85353_d_n7, assign55160_e85353_d_n8, assign55160_e85353_d_n9, assign55160_e85353_d_n10, assign55160_e85353_d_n11, assign55160_e85353_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55160_e85351: f64 = (1e-16 * 1e-16);
        (assign55160_e85351, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign55160_e85353;
        locals.var_xmax2_dn0 = assign55160_e85353_d_n0;
        locals.var_xmax2_dn2 = assign55160_e85353_d_n2;
        locals.var_xmax2_dn4 = assign55160_e85353_d_n4;
        locals.var_xmax2_dn5 = assign55160_e85353_d_n5;
        locals.var_xmax2_dn6 = assign55160_e85353_d_n6;
        locals.var_xmax2_dn7 = assign55160_e85353_d_n7;
        locals.var_xmax2_dn8 = assign55160_e85353_d_n8;
        locals.var_xmax2_dn9 = assign55160_e85353_d_n9;
        locals.var_xmax2_dn10 = assign55160_e85353_d_n10;
        locals.var_xmax2_dn11 = assign55160_e85353_d_n11;
        locals.var_xmax2_dn14 = assign55160_e85353_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign55170_e85369, assign55170_e85369_d_n0, assign55170_e85369_d_n2, assign55170_e85369_d_n4, assign55170_e85369_d_n5, assign55170_e85369_d_n6, assign55170_e85369_d_n7, assign55170_e85369_d_n8, assign55170_e85369_d_n9, assign55170_e85369_d_n10, assign55170_e85369_d_n11, assign55170_e85369_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55170_e85369;
        locals.var_xp_dn0 = assign55170_e85369_d_n0;
        locals.var_xp_dn2 = assign55170_e85369_d_n2;
        locals.var_xp_dn4 = assign55170_e85369_d_n4;
        locals.var_xp_dn5 = assign55170_e85369_d_n5;
        locals.var_xp_dn6 = assign55170_e85369_d_n6;
        locals.var_xp_dn7 = assign55170_e85369_d_n7;
        locals.var_xp_dn8 = assign55170_e85369_d_n8;
        locals.var_xp_dn9 = assign55170_e85369_d_n9;
        locals.var_xp_dn10 = assign55170_e85369_d_n10;
        locals.var_xp_dn11 = assign55170_e85369_d_n11;
        locals.var_xp_dn14 = assign55170_e85369_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign55180_e85385, assign55180_e85385_d_n0, assign55180_e85385_d_n2, assign55180_e85385_d_n4, assign55180_e85385_d_n5, assign55180_e85385_d_n6, assign55180_e85385_d_n7, assign55180_e85385_d_n8, assign55180_e85385_d_n9, assign55180_e85385_d_n10, assign55180_e85385_d_n11, assign55180_e85385_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55180_e85385;
        locals.var_xmp_dn0 = assign55180_e85385_d_n0;
        locals.var_xmp_dn2 = assign55180_e85385_d_n2;
        locals.var_xmp_dn4 = assign55180_e85385_d_n4;
        locals.var_xmp_dn5 = assign55180_e85385_d_n5;
        locals.var_xmp_dn6 = assign55180_e85385_d_n6;
        locals.var_xmp_dn7 = assign55180_e85385_d_n7;
        locals.var_xmp_dn8 = assign55180_e85385_d_n8;
        locals.var_xmp_dn9 = assign55180_e85385_d_n9;
        locals.var_xmp_dn10 = assign55180_e85385_d_n10;
        locals.var_xmp_dn11 = assign55180_e85385_d_n11;
        locals.var_xmp_dn14 = assign55180_e85385_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign55190_e85401,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55190_e85401;
        locals.var_m0_rv = 0.0;

        let (assign55200_e85417,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55200_e85417;
        locals.var_mm_rv = 0.0;

    }
}
