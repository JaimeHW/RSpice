#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_208(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign61670_e96023, assign61670_e96023_d_n0, assign61670_e96023_d_n2, assign61670_e96023_d_n4, assign61670_e96023_d_n5, assign61670_e96023_d_n6, assign61670_e96023_d_n7, assign61670_e96023_d_n8, assign61670_e96023_d_n9, assign61670_e96023_d_n10, assign61670_e96023_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1486 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    }
};
        locals.var_qdrat = assign61670_e96023;
        locals.var_qdrat_dn0 = assign61670_e96023_d_n0;
        locals.var_qdrat_dn2 = assign61670_e96023_d_n2;
        locals.var_qdrat_dn4 = assign61670_e96023_d_n4;
        locals.var_qdrat_dn5 = assign61670_e96023_d_n5;
        locals.var_qdrat_dn6 = assign61670_e96023_d_n6;
        locals.var_qdrat_dn7 = assign61670_e96023_d_n7;
        locals.var_qdrat_dn8 = assign61670_e96023_d_n8;
        locals.var_qdrat_dn9 = assign61670_e96023_d_n9;
        locals.var_qdrat_dn10 = assign61670_e96023_d_n10;
        locals.var_qdrat_dn13 = assign61670_e96023_d_n13;

        let assign61680_e96026: f64 = if locals.var_flg_zone == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1487 = assign61680_e96026;

        let (assign61690_e96037, assign61690_e96037_d_n0, assign61690_e96037_d_n2, assign61690_e96037_d_n4, assign61690_e96037_d_n5, assign61690_e96037_d_n6, assign61690_e96037_d_n7, assign61690_e96037_d_n8, assign61690_e96037_d_n9, assign61690_e96037_d_n10, assign61690_e96037_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign61690_e96037;
        locals.var_t1_dn0 = assign61690_e96037_d_n0;
        locals.var_t1_dn2 = assign61690_e96037_d_n2;
        locals.var_t1_dn4 = assign61690_e96037_d_n4;
        locals.var_t1_dn5 = assign61690_e96037_d_n5;
        locals.var_t1_dn6 = assign61690_e96037_d_n6;
        locals.var_t1_dn7 = assign61690_e96037_d_n7;
        locals.var_t1_dn8 = assign61690_e96037_d_n8;
        locals.var_t1_dn9 = assign61690_e96037_d_n9;
        locals.var_t1_dn10 = assign61690_e96037_d_n10;
        locals.var_t1_dn13 = assign61690_e96037_d_n13;

        let (assign61700_e96056, assign61700_e96056_d_n0, assign61700_e96056_d_n2, assign61700_e96056_d_n4, assign61700_e96056_d_n5, assign61700_e96056_d_n6, assign61700_e96056_d_n7, assign61700_e96056_d_n8, assign61700_e96056_d_n9, assign61700_e96056_d_n10, assign61700_e96056_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        let assign61700_e96048: f64 = (locals.var_fd2 * locals.var_qbu);
        let assign61700_e96051: f64 = (1.0 - locals.var_fd2);
        let assign61700_e96053: f64 = (assign61700_e96051 * locals.var_qb0);
        let assign61700_e96054: f64 = (assign61700_e96048 + assign61700_e96053);
        (assign61700_e96054, (((locals.var_fd2_dn0 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn0)) + (((-locals.var_fd2_dn0) * locals.var_qb0) + (assign61700_e96051 * locals.var_qb0_dn0))), (((locals.var_fd2_dn2 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn2)) + (((-locals.var_fd2_dn2) * locals.var_qb0) + (assign61700_e96051 * locals.var_qb0_dn2))), (((locals.var_fd2_dn4 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn4)) + (((-locals.var_fd2_dn4) * locals.var_qb0) + (assign61700_e96051 * locals.var_qb0_dn4))), (((locals.var_fd2_dn5 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn5)) + (((-locals.var_fd2_dn5) * locals.var_qb0) + (assign61700_e96051 * locals.var_qb0_dn5))), (((locals.var_fd2_dn6 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn6)) + (((-locals.var_fd2_dn6) * locals.var_qb0) + (assign61700_e96051 * locals.var_qb0_dn6))), (((locals.var_fd2_dn7 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn7)) + (((-locals.var_fd2_dn7) * locals.var_qb0) + (assign61700_e96051 * locals.var_qb0_dn7))), (((locals.var_fd2_dn8 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn8)) + (((-locals.var_fd2_dn8) * locals.var_qb0) + (assign61700_e96051 * locals.var_qb0_dn8))), (((locals.var_fd2_dn9 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn9)) + (((-locals.var_fd2_dn9) * locals.var_qb0) + (assign61700_e96051 * locals.var_qb0_dn9))), (((locals.var_fd2_dn10 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn10)) + (((-locals.var_fd2_dn10) * locals.var_qb0) + (assign61700_e96051 * locals.var_qb0_dn10))), (((locals.var_fd2_dn13 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn13)) + (((-locals.var_fd2_dn13) * locals.var_qb0) + (assign61700_e96051 * locals.var_qb0_dn13))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn13,)
    }
};
        locals.var_qbu = assign61700_e96056;
        locals.var_qbu_dn0 = assign61700_e96056_d_n0;
        locals.var_qbu_dn2 = assign61700_e96056_d_n2;
        locals.var_qbu_dn4 = assign61700_e96056_d_n4;
        locals.var_qbu_dn5 = assign61700_e96056_d_n5;
        locals.var_qbu_dn6 = assign61700_e96056_d_n6;
        locals.var_qbu_dn7 = assign61700_e96056_d_n7;
        locals.var_qbu_dn8 = assign61700_e96056_d_n8;
        locals.var_qbu_dn9 = assign61700_e96056_d_n9;
        locals.var_qbu_dn10 = assign61700_e96056_d_n10;
        locals.var_qbu_dn13 = assign61700_e96056_d_n13;

        let assign61710_e96059: f64 = if locals.var_qbu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1488 = assign61710_e96059;

        let (assign61720_e96072, assign61720_e96072_d_n0, assign61720_e96072_d_n2, assign61720_e96072_d_n4, assign61720_e96072_d_n5, assign61720_e96072_d_n6, assign61720_e96072_d_n7, assign61720_e96072_d_n8, assign61720_e96072_d_n9, assign61720_e96072_d_n10, assign61720_e96072_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1487 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn13,)
    }
};
        locals.var_qbu = assign61720_e96072;
        locals.var_qbu_dn0 = assign61720_e96072_d_n0;
        locals.var_qbu_dn2 = assign61720_e96072_d_n2;
        locals.var_qbu_dn4 = assign61720_e96072_d_n4;
        locals.var_qbu_dn5 = assign61720_e96072_d_n5;
        locals.var_qbu_dn6 = assign61720_e96072_d_n6;
        locals.var_qbu_dn7 = assign61720_e96072_d_n7;
        locals.var_qbu_dn8 = assign61720_e96072_d_n8;
        locals.var_qbu_dn9 = assign61720_e96072_d_n9;
        locals.var_qbu_dn10 = assign61720_e96072_d_n10;
        locals.var_qbu_dn13 = assign61720_e96072_d_n13;

        let (assign61730_e96083, assign61730_e96083_d_n0, assign61730_e96083_d_n2, assign61730_e96083_d_n4, assign61730_e96083_d_n5, assign61730_e96083_d_n6, assign61730_e96083_d_n7, assign61730_e96083_d_n8, assign61730_e96083_d_n9, assign61730_e96083_d_n10, assign61730_e96083_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign61730_e96083;
        locals.var_t1_dn0 = assign61730_e96083_d_n0;
        locals.var_t1_dn2 = assign61730_e96083_d_n2;
        locals.var_t1_dn4 = assign61730_e96083_d_n4;
        locals.var_t1_dn5 = assign61730_e96083_d_n5;
        locals.var_t1_dn6 = assign61730_e96083_d_n6;
        locals.var_t1_dn7 = assign61730_e96083_d_n7;
        locals.var_t1_dn8 = assign61730_e96083_d_n8;
        locals.var_t1_dn9 = assign61730_e96083_d_n9;
        locals.var_t1_dn10 = assign61730_e96083_d_n10;
        locals.var_t1_dn13 = assign61730_e96083_d_n13;

        let (assign61740_e96102, assign61740_e96102_d_n0, assign61740_e96102_d_n2, assign61740_e96102_d_n4, assign61740_e96102_d_n5, assign61740_e96102_d_n6, assign61740_e96102_d_n7, assign61740_e96102_d_n8, assign61740_e96102_d_n9, assign61740_e96102_d_n10, assign61740_e96102_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        let assign61740_e96094: f64 = (locals.var_fd2 * locals.var_qiu);
        let assign61740_e96097: f64 = (1.0 - locals.var_fd2);
        let assign61740_e96099: f64 = (assign61740_e96097 * locals.var_qn0);
        let assign61740_e96100: f64 = (assign61740_e96094 + assign61740_e96099);
        (assign61740_e96100, (((locals.var_fd2_dn0 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn0)) + (((-locals.var_fd2_dn0) * locals.var_qn0) + (assign61740_e96097 * locals.var_qn0_dn0))), (((locals.var_fd2_dn2 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn2)) + (((-locals.var_fd2_dn2) * locals.var_qn0) + (assign61740_e96097 * locals.var_qn0_dn2))), (((locals.var_fd2_dn4 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn4)) + (((-locals.var_fd2_dn4) * locals.var_qn0) + (assign61740_e96097 * locals.var_qn0_dn4))), (((locals.var_fd2_dn5 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn5)) + (((-locals.var_fd2_dn5) * locals.var_qn0) + (assign61740_e96097 * locals.var_qn0_dn5))), (((locals.var_fd2_dn6 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn6)) + (((-locals.var_fd2_dn6) * locals.var_qn0) + (assign61740_e96097 * locals.var_qn0_dn6))), (((locals.var_fd2_dn7 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn7)) + (((-locals.var_fd2_dn7) * locals.var_qn0) + (assign61740_e96097 * locals.var_qn0_dn7))), (((locals.var_fd2_dn8 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn8)) + (((-locals.var_fd2_dn8) * locals.var_qn0) + (assign61740_e96097 * locals.var_qn0_dn8))), (((locals.var_fd2_dn9 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn9)) + (((-locals.var_fd2_dn9) * locals.var_qn0) + (assign61740_e96097 * locals.var_qn0_dn9))), (((locals.var_fd2_dn10 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn10)) + (((-locals.var_fd2_dn10) * locals.var_qn0) + (assign61740_e96097 * locals.var_qn0_dn10))), (((locals.var_fd2_dn13 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn13)) + (((-locals.var_fd2_dn13) * locals.var_qn0) + (assign61740_e96097 * locals.var_qn0_dn13))),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    }
};
        locals.var_qiu = assign61740_e96102;
        locals.var_qiu_dn0 = assign61740_e96102_d_n0;
        locals.var_qiu_dn2 = assign61740_e96102_d_n2;
        locals.var_qiu_dn4 = assign61740_e96102_d_n4;
        locals.var_qiu_dn5 = assign61740_e96102_d_n5;
        locals.var_qiu_dn6 = assign61740_e96102_d_n6;
        locals.var_qiu_dn7 = assign61740_e96102_d_n7;
        locals.var_qiu_dn8 = assign61740_e96102_d_n8;
        locals.var_qiu_dn9 = assign61740_e96102_d_n9;
        locals.var_qiu_dn10 = assign61740_e96102_d_n10;
        locals.var_qiu_dn13 = assign61740_e96102_d_n13;

        let assign61750_e96105: f64 = if locals.var_qiu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1489 = assign61750_e96105;

        let (assign61760_e96118, assign61760_e96118_d_n0, assign61760_e96118_d_n2, assign61760_e96118_d_n4, assign61760_e96118_d_n5, assign61760_e96118_d_n6, assign61760_e96118_d_n7, assign61760_e96118_d_n8, assign61760_e96118_d_n9, assign61760_e96118_d_n10, assign61760_e96118_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1487 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    }
};
        locals.var_qiu = assign61760_e96118;
        locals.var_qiu_dn0 = assign61760_e96118_d_n0;
        locals.var_qiu_dn2 = assign61760_e96118_d_n2;
        locals.var_qiu_dn4 = assign61760_e96118_d_n4;
        locals.var_qiu_dn5 = assign61760_e96118_d_n5;
        locals.var_qiu_dn6 = assign61760_e96118_d_n6;
        locals.var_qiu_dn7 = assign61760_e96118_d_n7;
        locals.var_qiu_dn8 = assign61760_e96118_d_n8;
        locals.var_qiu_dn9 = assign61760_e96118_d_n9;
        locals.var_qiu_dn10 = assign61760_e96118_d_n10;
        locals.var_qiu_dn13 = assign61760_e96118_d_n13;

        let (assign61770_e96129, assign61770_e96129_d_n0, assign61770_e96129_d_n2, assign61770_e96129_d_n4, assign61770_e96129_d_n5, assign61770_e96129_d_n6, assign61770_e96129_d_n7, assign61770_e96129_d_n8, assign61770_e96129_d_n9, assign61770_e96129_d_n10, assign61770_e96129_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign61770_e96129;
        locals.var_t1_dn0 = assign61770_e96129_d_n0;
        locals.var_t1_dn2 = assign61770_e96129_d_n2;
        locals.var_t1_dn4 = assign61770_e96129_d_n4;
        locals.var_t1_dn5 = assign61770_e96129_d_n5;
        locals.var_t1_dn6 = assign61770_e96129_d_n6;
        locals.var_t1_dn7 = assign61770_e96129_d_n7;
        locals.var_t1_dn8 = assign61770_e96129_d_n8;
        locals.var_t1_dn9 = assign61770_e96129_d_n9;
        locals.var_t1_dn10 = assign61770_e96129_d_n10;
        locals.var_t1_dn13 = assign61770_e96129_d_n13;

        let (assign61780_e96148, assign61780_e96148_d_n0, assign61780_e96148_d_n2, assign61780_e96148_d_n4, assign61780_e96148_d_n5, assign61780_e96148_d_n6, assign61780_e96148_d_n7, assign61780_e96148_d_n8, assign61780_e96148_d_n9, assign61780_e96148_d_n10, assign61780_e96148_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        let assign61780_e96140: f64 = (locals.var_fd2 * locals.var_qdrat);
        let assign61780_e96143: f64 = (1.0 - locals.var_fd2);
        let assign61780_e96145: f64 = (assign61780_e96143 * 0.5);
        let assign61780_e96146: f64 = (assign61780_e96140 + assign61780_e96145);
        (assign61780_e96146, (((locals.var_fd2_dn0 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn0)) + ((-locals.var_fd2_dn0) * 0.5)), (((locals.var_fd2_dn2 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn2)) + ((-locals.var_fd2_dn2) * 0.5)), (((locals.var_fd2_dn4 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn4)) + ((-locals.var_fd2_dn4) * 0.5)), (((locals.var_fd2_dn5 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn5)) + ((-locals.var_fd2_dn5) * 0.5)), (((locals.var_fd2_dn6 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn6)) + ((-locals.var_fd2_dn6) * 0.5)), (((locals.var_fd2_dn7 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn7)) + ((-locals.var_fd2_dn7) * 0.5)), (((locals.var_fd2_dn8 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn8)) + ((-locals.var_fd2_dn8) * 0.5)), (((locals.var_fd2_dn9 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn9)) + ((-locals.var_fd2_dn9) * 0.5)), (((locals.var_fd2_dn10 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn10)) + ((-locals.var_fd2_dn10) * 0.5)), (((locals.var_fd2_dn13 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn13)) + ((-locals.var_fd2_dn13) * 0.5)),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    }
};
        locals.var_qdrat = assign61780_e96148;
        locals.var_qdrat_dn0 = assign61780_e96148_d_n0;
        locals.var_qdrat_dn2 = assign61780_e96148_d_n2;
        locals.var_qdrat_dn4 = assign61780_e96148_d_n4;
        locals.var_qdrat_dn5 = assign61780_e96148_d_n5;
        locals.var_qdrat_dn6 = assign61780_e96148_d_n6;
        locals.var_qdrat_dn7 = assign61780_e96148_d_n7;
        locals.var_qdrat_dn8 = assign61780_e96148_d_n8;
        locals.var_qdrat_dn9 = assign61780_e96148_d_n9;
        locals.var_qdrat_dn10 = assign61780_e96148_d_n10;
        locals.var_qdrat_dn13 = assign61780_e96148_d_n13;

        let (assign61790_e96159, assign61790_e96159_d_n0, assign61790_e96159_d_n2, assign61790_e96159_d_n4, assign61790_e96159_d_n5, assign61790_e96159_d_n6, assign61790_e96159_d_n7, assign61790_e96159_d_n8, assign61790_e96159_d_n9, assign61790_e96159_d_n10, assign61790_e96159_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign61790_e96159;
        locals.var_t1_dn0 = assign61790_e96159_d_n0;
        locals.var_t1_dn2 = assign61790_e96159_d_n2;
        locals.var_t1_dn4 = assign61790_e96159_d_n4;
        locals.var_t1_dn5 = assign61790_e96159_d_n5;
        locals.var_t1_dn6 = assign61790_e96159_d_n6;
        locals.var_t1_dn7 = assign61790_e96159_d_n7;
        locals.var_t1_dn8 = assign61790_e96159_d_n8;
        locals.var_t1_dn9 = assign61790_e96159_d_n9;
        locals.var_t1_dn10 = assign61790_e96159_d_n10;
        locals.var_t1_dn13 = assign61790_e96159_d_n13;

        let (assign61800_e96172, assign61800_e96172_d_n0, assign61800_e96172_d_n2, assign61800_e96172_d_n4, assign61800_e96172_d_n5, assign61800_e96172_d_n6, assign61800_e96172_d_n7, assign61800_e96172_d_n8, assign61800_e96172_d_n9, assign61800_e96172_d_n10, assign61800_e96172_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        let assign61800_e96170: f64 = (locals.var_fd2 * locals.var_lred);
        (assign61800_e96170, ((locals.var_fd2_dn0 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn0)), ((locals.var_fd2_dn2 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn2)), ((locals.var_fd2_dn4 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn4)), ((locals.var_fd2_dn5 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn5)), ((locals.var_fd2_dn6 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn6)), ((locals.var_fd2_dn7 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn7)), ((locals.var_fd2_dn8 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn8)), ((locals.var_fd2_dn9 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn9)), ((locals.var_fd2_dn10 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn10)), ((locals.var_fd2_dn13 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn13)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn13,)
    }
};
        locals.var_lred = assign61800_e96172;
        locals.var_lred_dn0 = assign61800_e96172_d_n0;
        locals.var_lred_dn2 = assign61800_e96172_d_n2;
        locals.var_lred_dn4 = assign61800_e96172_d_n4;
        locals.var_lred_dn5 = assign61800_e96172_d_n5;
        locals.var_lred_dn6 = assign61800_e96172_d_n6;
        locals.var_lred_dn7 = assign61800_e96172_d_n7;
        locals.var_lred_dn8 = assign61800_e96172_d_n8;
        locals.var_lred_dn9 = assign61800_e96172_d_n9;
        locals.var_lred_dn10 = assign61800_e96172_d_n10;
        locals.var_lred_dn13 = assign61800_e96172_d_n13;

        let (assign61810_e96181,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_start_of_mobility != 0.0)) {
        (0.0,)
    } else {
        (locals.var_start_of_mobility,)
    }
};
        locals.var_start_of_mobility = assign61810_e96181;

        let (assign61820_e96190, assign61820_e96190_d_n0, assign61820_e96190_d_n2, assign61820_e96190_d_n4, assign61820_e96190_d_n5, assign61820_e96190_d_n6, assign61820_e96190_d_n7, assign61820_e96190_d_n8, assign61820_e96190_d_n9, assign61820_e96190_d_n10, assign61820_e96190_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61820_e96188: f64 = (locals.var_leff - locals.var_lred);
        (assign61820_e96188, (-locals.var_lred_dn0), (-locals.var_lred_dn2), (-locals.var_lred_dn4), (-locals.var_lred_dn5), (-locals.var_lred_dn6), (-locals.var_lred_dn7), (-locals.var_lred_dn8), (-locals.var_lred_dn9), (-locals.var_lred_dn10), (-locals.var_lred_dn13),)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn13,)
    }
};
        locals.var_lch = assign61820_e96190;
        locals.var_lch_dn0 = assign61820_e96190_d_n0;
        locals.var_lch_dn2 = assign61820_e96190_d_n2;
        locals.var_lch_dn4 = assign61820_e96190_d_n4;
        locals.var_lch_dn5 = assign61820_e96190_d_n5;
        locals.var_lch_dn6 = assign61820_e96190_d_n6;
        locals.var_lch_dn7 = assign61820_e96190_d_n7;
        locals.var_lch_dn8 = assign61820_e96190_d_n8;
        locals.var_lch_dn9 = assign61820_e96190_d_n9;
        locals.var_lch_dn10 = assign61820_e96190_d_n10;
        locals.var_lch_dn13 = assign61820_e96190_d_n13;

        let assign61830_e96193: f64 = if locals.var_lch < 1e-9 { 1.0 } else { 0.0 };
        locals.var_guard1490 = assign61830_e96193;

        let (assign61840_e96202, assign61840_e96202_d_n0, assign61840_e96202_d_n2, assign61840_e96202_d_n4, assign61840_e96202_d_n5, assign61840_e96202_d_n6, assign61840_e96202_d_n7, assign61840_e96202_d_n8, assign61840_e96202_d_n9, assign61840_e96202_d_n10, assign61840_e96202_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1490 != 0.0)) {
        (1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn13,)
    }
};
        locals.var_lch = assign61840_e96202;
        locals.var_lch_dn0 = assign61840_e96202_d_n0;
        locals.var_lch_dn2 = assign61840_e96202_d_n2;
        locals.var_lch_dn4 = assign61840_e96202_d_n4;
        locals.var_lch_dn5 = assign61840_e96202_d_n5;
        locals.var_lch_dn6 = assign61840_e96202_d_n6;
        locals.var_lch_dn7 = assign61840_e96202_d_n7;
        locals.var_lch_dn8 = assign61840_e96202_d_n8;
        locals.var_lch_dn9 = assign61840_e96202_d_n9;
        locals.var_lch_dn10 = assign61840_e96202_d_n10;
        locals.var_lch_dn13 = assign61840_e96202_d_n13;

        let (assign61850_e96211, assign61850_e96211_d_n0, assign61850_e96211_d_n2, assign61850_e96211_d_n4, assign61850_e96211_d_n5, assign61850_e96211_d_n6, assign61850_e96211_d_n7, assign61850_e96211_d_n8, assign61850_e96211_d_n9, assign61850_e96211_d_n10, assign61850_e96211_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61850_e96209: f64 = (locals.var_ndep_o_esi / 100.0);
        (assign61850_e96209, (locals.var_ndep_o_esi_dn0 / 100.0), (locals.var_ndep_o_esi_dn2 / 100.0), (locals.var_ndep_o_esi_dn4 / 100.0), (locals.var_ndep_o_esi_dn5 / 100.0), (locals.var_ndep_o_esi_dn6 / 100.0), (locals.var_ndep_o_esi_dn7 / 100.0), (locals.var_ndep_o_esi_dn8 / 100.0), (locals.var_ndep_o_esi_dn9 / 100.0), (locals.var_ndep_o_esi_dn10 / 100.0), (locals.var_ndep_o_esi_dn13 / 100.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign61850_e96211;
        locals.var_t1_dn0 = assign61850_e96211_d_n0;
        locals.var_t1_dn2 = assign61850_e96211_d_n2;
        locals.var_t1_dn4 = assign61850_e96211_d_n4;
        locals.var_t1_dn5 = assign61850_e96211_d_n5;
        locals.var_t1_dn6 = assign61850_e96211_d_n6;
        locals.var_t1_dn7 = assign61850_e96211_d_n7;
        locals.var_t1_dn8 = assign61850_e96211_d_n8;
        locals.var_t1_dn9 = assign61850_e96211_d_n9;
        locals.var_t1_dn10 = assign61850_e96211_d_n10;
        locals.var_t1_dn13 = assign61850_e96211_d_n13;

        let (assign61860_e96220, assign61860_e96220_d_n0, assign61860_e96220_d_n2, assign61860_e96220_d_n4, assign61860_e96220_d_n5, assign61860_e96220_d_n6, assign61860_e96220_d_n7, assign61860_e96220_d_n8, assign61860_e96220_d_n9, assign61860_e96220_d_n10, assign61860_e96220_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61860_e96218: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign61860_e96218, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign61860_e96220;
        locals.var_t2_dn0 = assign61860_e96220_d_n0;
        locals.var_t2_dn2 = assign61860_e96220_d_n2;
        locals.var_t2_dn4 = assign61860_e96220_d_n4;
        locals.var_t2_dn5 = assign61860_e96220_d_n5;
        locals.var_t2_dn6 = assign61860_e96220_d_n6;
        locals.var_t2_dn7 = assign61860_e96220_d_n7;
        locals.var_t2_dn8 = assign61860_e96220_d_n8;
        locals.var_t2_dn9 = assign61860_e96220_d_n9;
        locals.var_t2_dn10 = assign61860_e96220_d_n10;
        locals.var_t2_dn13 = assign61860_e96220_d_n13;

        let (assign61870_e96227, assign61870_e96227_d_n0, assign61870_e96227_d_n2, assign61870_e96227_d_n4, assign61870_e96227_d_n5, assign61870_e96227_d_n6, assign61870_e96227_d_n7, assign61870_e96227_d_n8, assign61870_e96227_d_n9, assign61870_e96227_d_n10, assign61870_e96227_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign61870_e96227;
        locals.var_t0_dn0 = assign61870_e96227_d_n0;
        locals.var_t0_dn2 = assign61870_e96227_d_n2;
        locals.var_t0_dn4 = assign61870_e96227_d_n4;
        locals.var_t0_dn5 = assign61870_e96227_d_n5;
        locals.var_t0_dn6 = assign61870_e96227_d_n6;
        locals.var_t0_dn7 = assign61870_e96227_d_n7;
        locals.var_t0_dn8 = assign61870_e96227_d_n8;
        locals.var_t0_dn9 = assign61870_e96227_d_n9;
        locals.var_t0_dn10 = assign61870_e96227_d_n10;
        locals.var_t0_dn13 = assign61870_e96227_d_n13;

        let (assign61880_e96240, assign61880_e96240_d_n0, assign61880_e96240_d_n2, assign61880_e96240_d_n4, assign61880_e96240_d_n5, assign61880_e96240_d_n6, assign61880_e96240_d_n7, assign61880_e96240_d_n8, assign61880_e96240_d_n9, assign61880_e96240_d_n10, assign61880_e96240_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61880_e96235: f64 = (locals.var_psl - locals.var_ps0);
        let assign61880_e96237: f64 = (assign61880_e96235 * locals.var_t0);
        let assign61880_e96238: f64 = (1.0 + assign61880_e96237);
        (assign61880_e96238, (((locals.var_psl_dn0 - locals.var_ps0_dn0) * locals.var_t0) + (assign61880_e96235 * locals.var_t0_dn0)), (((locals.var_psl_dn2 - locals.var_ps0_dn2) * locals.var_t0) + (assign61880_e96235 * locals.var_t0_dn2)), (((locals.var_psl_dn4 - locals.var_ps0_dn4) * locals.var_t0) + (assign61880_e96235 * locals.var_t0_dn4)), (((locals.var_psl_dn5 - locals.var_ps0_dn5) * locals.var_t0) + (assign61880_e96235 * locals.var_t0_dn5)), (((locals.var_psl_dn6 - locals.var_ps0_dn6) * locals.var_t0) + (assign61880_e96235 * locals.var_t0_dn6)), (((locals.var_psl_dn7 - locals.var_ps0_dn7) * locals.var_t0) + (assign61880_e96235 * locals.var_t0_dn7)), (((locals.var_psl_dn8 - locals.var_ps0_dn8) * locals.var_t0) + (assign61880_e96235 * locals.var_t0_dn8)), (((locals.var_psl_dn9 - locals.var_ps0_dn9) * locals.var_t0) + (assign61880_e96235 * locals.var_t0_dn9)), (((locals.var_psl_dn10 - locals.var_ps0_dn10) * locals.var_t0) + (assign61880_e96235 * locals.var_t0_dn10)), (((locals.var_psl_dn13 - locals.var_ps0_dn13) * locals.var_t0) + (assign61880_e96235 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign61880_e96240;
        locals.var_t4_dn0 = assign61880_e96240_d_n0;
        locals.var_t4_dn2 = assign61880_e96240_d_n2;
        locals.var_t4_dn4 = assign61880_e96240_d_n4;
        locals.var_t4_dn5 = assign61880_e96240_d_n5;
        locals.var_t4_dn6 = assign61880_e96240_d_n6;
        locals.var_t4_dn7 = assign61880_e96240_d_n7;
        locals.var_t4_dn8 = assign61880_e96240_d_n8;
        locals.var_t4_dn9 = assign61880_e96240_d_n9;
        locals.var_t4_dn10 = assign61880_e96240_d_n10;
        locals.var_t4_dn13 = assign61880_e96240_d_n13;

        let (assign61890_e96253, assign61890_e96253_d_n0, assign61890_e96253_d_n2, assign61890_e96253_d_n4, assign61890_e96253_d_n5, assign61890_e96253_d_n6, assign61890_e96253_d_n7, assign61890_e96253_d_n8, assign61890_e96253_d_n9, assign61890_e96253_d_n10, assign61890_e96253_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61890_e96247: f64 = (locals.var_t1 * locals.var_qbu);
        let assign61890_e96250: f64 = (locals.var_t2 * locals.var_qiu);
        let assign61890_e96251: f64 = (assign61890_e96247 + assign61890_e96250);
        (assign61890_e96251, (((locals.var_t1_dn0 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn0)) + ((locals.var_t2_dn0 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn0))), (((locals.var_t1_dn2 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn2)) + ((locals.var_t2_dn2 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn2))), (((locals.var_t1_dn4 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn4)) + ((locals.var_t2_dn4 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn4))), (((locals.var_t1_dn5 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn5)) + ((locals.var_t2_dn5 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn5))), (((locals.var_t1_dn6 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn6)) + ((locals.var_t2_dn6 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn6))), (((locals.var_t1_dn7 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn7)) + ((locals.var_t2_dn7 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn7))), (((locals.var_t1_dn8 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn8)) + ((locals.var_t2_dn8 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn8))), (((locals.var_t1_dn9 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn9)) + ((locals.var_t2_dn9 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn9))), (((locals.var_t1_dn10 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn10)) + ((locals.var_t2_dn10 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn10))), (((locals.var_t1_dn13 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn13)) + ((locals.var_t2_dn13 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign61890_e96253;
        locals.var_t5_dn0 = assign61890_e96253_d_n0;
        locals.var_t5_dn2 = assign61890_e96253_d_n2;
        locals.var_t5_dn4 = assign61890_e96253_d_n4;
        locals.var_t5_dn5 = assign61890_e96253_d_n5;
        locals.var_t5_dn6 = assign61890_e96253_d_n6;
        locals.var_t5_dn7 = assign61890_e96253_d_n7;
        locals.var_t5_dn8 = assign61890_e96253_d_n8;
        locals.var_t5_dn9 = assign61890_e96253_d_n9;
        locals.var_t5_dn10 = assign61890_e96253_d_n10;
        locals.var_t5_dn13 = assign61890_e96253_d_n13;

        let (assign61900_e96262, assign61900_e96262_d_n0, assign61900_e96262_d_n2, assign61900_e96262_d_n4, assign61900_e96262_d_n5, assign61900_e96262_d_n6, assign61900_e96262_d_n7, assign61900_e96262_d_n8, assign61900_e96262_d_n9, assign61900_e96262_d_n10, assign61900_e96262_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61900_e96260: f64 = (locals.var_t5 / locals.var_t4);
        (assign61900_e96260, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn13 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign61900_e96262;
        locals.var_t3_dn0 = assign61900_e96262_d_n0;
        locals.var_t3_dn2 = assign61900_e96262_d_n2;
        locals.var_t3_dn4 = assign61900_e96262_d_n4;
        locals.var_t3_dn5 = assign61900_e96262_d_n5;
        locals.var_t3_dn6 = assign61900_e96262_d_n6;
        locals.var_t3_dn7 = assign61900_e96262_d_n7;
        locals.var_t3_dn8 = assign61900_e96262_d_n8;
        locals.var_t3_dn9 = assign61900_e96262_d_n9;
        locals.var_t3_dn10 = assign61900_e96262_d_n10;
        locals.var_t3_dn13 = assign61900_e96262_d_n13;

        let (assign61910_e96275, assign61910_e96275_d_n0, assign61910_e96275_d_n2, assign61910_e96275_d_n4, assign61910_e96275_d_n5, assign61910_e96275_d_n6, assign61910_e96275_d_n7, assign61910_e96275_d_n8, assign61910_e96275_d_n9, assign61910_e96275_d_n10, assign61910_e96275_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61910_e96271: f64 = (p.p166 * locals.var_vbsz__blk438);
        let assign61910_e96272: f64 = (1.0 + assign61910_e96271);
        let assign61910_e96273: f64 = (locals.var_t3 * assign61910_e96272);
        (assign61910_e96273, ((locals.var_t3_dn0 * assign61910_e96272) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk438_dn0))), ((locals.var_t3_dn2 * assign61910_e96272) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk438_dn2))), ((locals.var_t3_dn4 * assign61910_e96272) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk438_dn4))), ((locals.var_t3_dn5 * assign61910_e96272) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk438_dn5))), ((locals.var_t3_dn6 * assign61910_e96272) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk438_dn6))), ((locals.var_t3_dn7 * assign61910_e96272) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk438_dn7))), ((locals.var_t3_dn8 * assign61910_e96272) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk438_dn8))), ((locals.var_t3_dn9 * assign61910_e96272) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk438_dn9))), ((locals.var_t3_dn10 * assign61910_e96272) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk438_dn10))), ((locals.var_t3_dn13 * assign61910_e96272) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk438_dn13))),)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn13,)
    }
};
        locals.var_eeff = assign61910_e96275;
        locals.var_eeff_dn0 = assign61910_e96275_d_n0;
        locals.var_eeff_dn2 = assign61910_e96275_d_n2;
        locals.var_eeff_dn4 = assign61910_e96275_d_n4;
        locals.var_eeff_dn5 = assign61910_e96275_d_n5;
        locals.var_eeff_dn6 = assign61910_e96275_d_n6;
        locals.var_eeff_dn7 = assign61910_e96275_d_n7;
        locals.var_eeff_dn8 = assign61910_e96275_d_n8;
        locals.var_eeff_dn9 = assign61910_e96275_d_n9;
        locals.var_eeff_dn10 = assign61910_e96275_d_n10;
        locals.var_eeff_dn13 = assign61910_e96275_d_n13;

        let (assign61920_e96291, assign61920_e96291_d_n0, assign61920_e96291_d_n2, assign61920_e96291_d_n4, assign61920_e96291_d_n5, assign61920_e96291_d_n6, assign61920_e96291_d_n7, assign61920_e96291_d_n8, assign61920_e96291_d_n9, assign61920_e96291_d_n10, assign61920_e96291_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let (assign61920_e96289, assign61920_e96289_d_n0, assign61920_e96289_d_n2, assign61920_e96289_d_n4, assign61920_e96289_d_n5, assign61920_e96289_d_n6, assign61920_e96289_d_n7, assign61920_e96289_d_n8, assign61920_e96289_d_n9, assign61920_e96289_d_n10, assign61920_e96289_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61920_e96287: f64 = (p.p160 - 1.0);
                let assign61920_e96288: f64 = (locals.var_eeff).powf(assign61920_e96287);
                (assign61920_e96288, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((locals.var_eeff).powf(assign61920_e96287 - 1.0) * locals.var_eeff_dn0)) } } else { (assign61920_e96288 * (assign61920_e96287 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((locals.var_eeff).powf(assign61920_e96287 - 1.0) * locals.var_eeff_dn2)) } } else { (assign61920_e96288 * (assign61920_e96287 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((locals.var_eeff).powf(assign61920_e96287 - 1.0) * locals.var_eeff_dn4)) } } else { (assign61920_e96288 * (assign61920_e96287 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((locals.var_eeff).powf(assign61920_e96287 - 1.0) * locals.var_eeff_dn5)) } } else { (assign61920_e96288 * (assign61920_e96287 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((locals.var_eeff).powf(assign61920_e96287 - 1.0) * locals.var_eeff_dn6)) } } else { (assign61920_e96288 * (assign61920_e96287 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((locals.var_eeff).powf(assign61920_e96287 - 1.0) * locals.var_eeff_dn7)) } } else { (assign61920_e96288 * (assign61920_e96287 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((locals.var_eeff).powf(assign61920_e96287 - 1.0) * locals.var_eeff_dn8)) } } else { (assign61920_e96288 * (assign61920_e96287 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((locals.var_eeff).powf(assign61920_e96287 - 1.0) * locals.var_eeff_dn9)) } } else { (assign61920_e96288 * (assign61920_e96287 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((locals.var_eeff).powf(assign61920_e96287 - 1.0) * locals.var_eeff_dn10)) } } else { (assign61920_e96288 * (assign61920_e96287 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((locals.var_eeff).powf(assign61920_e96287 - 1.0) * locals.var_eeff_dn13)) } } else { (assign61920_e96288 * (assign61920_e96287 * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign61920_e96289, assign61920_e96289_d_n0, assign61920_e96289_d_n2, assign61920_e96289_d_n4, assign61920_e96289_d_n5, assign61920_e96289_d_n6, assign61920_e96289_d_n7, assign61920_e96289_d_n8, assign61920_e96289_d_n9, assign61920_e96289_d_n10, assign61920_e96289_d_n13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign61920_e96291;
        locals.var_t5_dn0 = assign61920_e96291_d_n0;
        locals.var_t5_dn2 = assign61920_e96291_d_n2;
        locals.var_t5_dn4 = assign61920_e96291_d_n4;
        locals.var_t5_dn5 = assign61920_e96291_d_n5;
        locals.var_t5_dn6 = assign61920_e96291_d_n6;
        locals.var_t5_dn7 = assign61920_e96291_d_n7;
        locals.var_t5_dn8 = assign61920_e96291_d_n8;
        locals.var_t5_dn9 = assign61920_e96291_d_n9;
        locals.var_t5_dn10 = assign61920_e96291_d_n10;
        locals.var_t5_dn13 = assign61920_e96291_d_n13;

        let (assign61930_e96300, assign61930_e96300_d_n0, assign61930_e96300_d_n2, assign61930_e96300_d_n4, assign61930_e96300_d_n5, assign61930_e96300_d_n6, assign61930_e96300_d_n7, assign61930_e96300_d_n8, assign61930_e96300_d_n9, assign61930_e96300_d_n10, assign61930_e96300_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61930_e96298: f64 = (locals.var_t5 * locals.var_eeff);
        (assign61930_e96298, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn13 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign61930_e96300;
        locals.var_t8_dn0 = assign61930_e96300_d_n0;
        locals.var_t8_dn2 = assign61930_e96300_d_n2;
        locals.var_t8_dn4 = assign61930_e96300_d_n4;
        locals.var_t8_dn5 = assign61930_e96300_d_n5;
        locals.var_t8_dn6 = assign61930_e96300_d_n6;
        locals.var_t8_dn7 = assign61930_e96300_d_n7;
        locals.var_t8_dn8 = assign61930_e96300_d_n8;
        locals.var_t8_dn9 = assign61930_e96300_d_n9;
        locals.var_t8_dn10 = assign61930_e96300_d_n10;
        locals.var_t8_dn13 = assign61930_e96300_d_n13;

        let (assign61940_e96316, assign61940_e96316_d_n0, assign61940_e96316_d_n2, assign61940_e96316_d_n4, assign61940_e96316_d_n5, assign61940_e96316_d_n6, assign61940_e96316_d_n7, assign61940_e96316_d_n8, assign61940_e96316_d_n9, assign61940_e96316_d_n10, assign61940_e96316_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let (assign61940_e96314, assign61940_e96314_d_n0, assign61940_e96314_d_n2, assign61940_e96314_d_n4, assign61940_e96314_d_n5, assign61940_e96314_d_n6, assign61940_e96314_d_n7, assign61940_e96314_d_n8, assign61940_e96314_d_n9, assign61940_e96314_d_n10, assign61940_e96314_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61940_e96312: f64 = (locals.var_muesr - 1.0);
                let assign61940_e96313: f64 = (locals.var_eeff).powf(assign61940_e96312);
                (assign61940_e96313, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((locals.var_eeff).powf(assign61940_e96312 - 1.0) * locals.var_eeff_dn0)) } } else { (assign61940_e96313 * (assign61940_e96312 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((locals.var_eeff).powf(assign61940_e96312 - 1.0) * locals.var_eeff_dn2)) } } else { (assign61940_e96313 * (assign61940_e96312 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((locals.var_eeff).powf(assign61940_e96312 - 1.0) * locals.var_eeff_dn4)) } } else { (assign61940_e96313 * (assign61940_e96312 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((locals.var_eeff).powf(assign61940_e96312 - 1.0) * locals.var_eeff_dn5)) } } else { (assign61940_e96313 * (assign61940_e96312 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((locals.var_eeff).powf(assign61940_e96312 - 1.0) * locals.var_eeff_dn6)) } } else { (assign61940_e96313 * (assign61940_e96312 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((locals.var_eeff).powf(assign61940_e96312 - 1.0) * locals.var_eeff_dn7)) } } else { (assign61940_e96313 * (assign61940_e96312 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((locals.var_eeff).powf(assign61940_e96312 - 1.0) * locals.var_eeff_dn8)) } } else { (assign61940_e96313 * (assign61940_e96312 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((locals.var_eeff).powf(assign61940_e96312 - 1.0) * locals.var_eeff_dn9)) } } else { (assign61940_e96313 * (assign61940_e96312 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((locals.var_eeff).powf(assign61940_e96312 - 1.0) * locals.var_eeff_dn10)) } } else { (assign61940_e96313 * (assign61940_e96312 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((locals.var_eeff).powf(assign61940_e96312 - 1.0) * locals.var_eeff_dn13)) } } else { (assign61940_e96313 * (assign61940_e96312 * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign61940_e96314, assign61940_e96314_d_n0, assign61940_e96314_d_n2, assign61940_e96314_d_n4, assign61940_e96314_d_n5, assign61940_e96314_d_n6, assign61940_e96314_d_n7, assign61940_e96314_d_n8, assign61940_e96314_d_n9, assign61940_e96314_d_n10, assign61940_e96314_d_n13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign61940_e96316;
        locals.var_t7_dn0 = assign61940_e96316_d_n0;
        locals.var_t7_dn2 = assign61940_e96316_d_n2;
        locals.var_t7_dn4 = assign61940_e96316_d_n4;
        locals.var_t7_dn5 = assign61940_e96316_d_n5;
        locals.var_t7_dn6 = assign61940_e96316_d_n6;
        locals.var_t7_dn7 = assign61940_e96316_d_n7;
        locals.var_t7_dn8 = assign61940_e96316_d_n8;
        locals.var_t7_dn9 = assign61940_e96316_d_n9;
        locals.var_t7_dn10 = assign61940_e96316_d_n10;
        locals.var_t7_dn13 = assign61940_e96316_d_n13;

    }

    pub(super) fn stamp_transient_block_209(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign61950_e96325, assign61950_e96325_d_n0, assign61950_e96325_d_n2, assign61950_e96325_d_n4, assign61950_e96325_d_n5, assign61950_e96325_d_n6, assign61950_e96325_d_n7, assign61950_e96325_d_n8, assign61950_e96325_d_n9, assign61950_e96325_d_n10, assign61950_e96325_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61950_e96323: f64 = (locals.var_t7 * locals.var_eeff);
        (assign61950_e96323, ((locals.var_t7_dn0 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn0)), ((locals.var_t7_dn2 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn2)), ((locals.var_t7_dn4 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn4)), ((locals.var_t7_dn5 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn5)), ((locals.var_t7_dn6 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn6)), ((locals.var_t7_dn7 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn7)), ((locals.var_t7_dn8 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn8)), ((locals.var_t7_dn9 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn9)), ((locals.var_t7_dn10 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn10)), ((locals.var_t7_dn13 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign61950_e96325;
        locals.var_t6_dn0 = assign61950_e96325_d_n0;
        locals.var_t6_dn2 = assign61950_e96325_d_n2;
        locals.var_t6_dn4 = assign61950_e96325_d_n4;
        locals.var_t6_dn5 = assign61950_e96325_d_n5;
        locals.var_t6_dn6 = assign61950_e96325_d_n6;
        locals.var_t6_dn7 = assign61950_e96325_d_n7;
        locals.var_t6_dn8 = assign61950_e96325_d_n8;
        locals.var_t6_dn9 = assign61950_e96325_d_n9;
        locals.var_t6_dn10 = assign61950_e96325_d_n10;
        locals.var_t6_dn13 = assign61950_e96325_d_n13;

        let (assign61960_e96334, assign61960_e96334_d_n0, assign61960_e96334_d_n2, assign61960_e96334_d_n4, assign61960_e96334_d_n5, assign61960_e96334_d_n6, assign61960_e96334_d_n7, assign61960_e96334_d_n8, assign61960_e96334_d_n9, assign61960_e96334_d_n10, assign61960_e96334_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61960_e96332: f64 = (1.6021918e-19 * 10000.0);
        (assign61960_e96332, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign61960_e96334;
        locals.var_t9_dn0 = assign61960_e96334_d_n0;
        locals.var_t9_dn2 = assign61960_e96334_d_n2;
        locals.var_t9_dn4 = assign61960_e96334_d_n4;
        locals.var_t9_dn5 = assign61960_e96334_d_n5;
        locals.var_t9_dn6 = assign61960_e96334_d_n6;
        locals.var_t9_dn7 = assign61960_e96334_d_n7;
        locals.var_t9_dn8 = assign61960_e96334_d_n8;
        locals.var_t9_dn9 = assign61960_e96334_d_n9;
        locals.var_t9_dn10 = assign61960_e96334_d_n10;
        locals.var_t9_dn13 = assign61960_e96334_d_n13;

        let (assign61970_e96343, assign61970_e96343_d_n0, assign61970_e96343_d_n2, assign61970_e96343_d_n4, assign61970_e96343_d_n5, assign61970_e96343_d_n6, assign61970_e96343_d_n7, assign61970_e96343_d_n8, assign61970_e96343_d_n9, assign61970_e96343_d_n10, assign61970_e96343_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61970_e96341: f64 = (locals.var_qiu / locals.var_t9);
        (assign61970_e96341, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn13 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn13)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn13,)
    }
};
        locals.var_rns = assign61970_e96343;
        locals.var_rns_dn0 = assign61970_e96343_d_n0;
        locals.var_rns_dn2 = assign61970_e96343_d_n2;
        locals.var_rns_dn4 = assign61970_e96343_d_n4;
        locals.var_rns_dn5 = assign61970_e96343_d_n5;
        locals.var_rns_dn6 = assign61970_e96343_d_n6;
        locals.var_rns_dn7 = assign61970_e96343_d_n7;
        locals.var_rns_dn8 = assign61970_e96343_d_n8;
        locals.var_rns_dn9 = assign61970_e96343_d_n9;
        locals.var_rns_dn10 = assign61970_e96343_d_n10;
        locals.var_rns_dn13 = assign61970_e96343_d_n13;

        let (assign61980_e96366, assign61980_e96366_d_n0, assign61980_e96366_d_n2, assign61980_e96366_d_n4, assign61980_e96366_d_n5, assign61980_e96366_d_n6, assign61980_e96366_d_n7, assign61980_e96366_d_n8, assign61980_e96366_d_n9, assign61980_e96366_d_n10, assign61980_e96366_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61980_e96352: f64 = (locals.var_uc_muecb1 * locals.var_rns);
        let assign61980_e96354: f64 = (assign61980_e96352 / 100000000000.0);
        let assign61980_e96355: f64 = (locals.var_uc_muecb0 + assign61980_e96354);
        let assign61980_e96356: f64 = (1.0 / assign61980_e96355);
        let assign61980_e96359: f64 = (locals.var_mphn0 * locals.var_t8);
        let assign61980_e96360: f64 = (assign61980_e96356 + assign61980_e96359);
        let assign61980_e96363: f64 = (locals.var_t6 / locals.var_uc_muesr1);
        let assign61980_e96364: f64 = (assign61980_e96360 + assign61980_e96363);
        (assign61980_e96364, (((-(((locals.var_uc_muecb1 * locals.var_rns_dn0) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn2) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn4) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn5) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn6) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn7) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn8) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn9) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn10) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn13) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((locals.var_mphn0_dn13 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn13))) + (locals.var_t6_dn13 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign61980_e96366;
        locals.var_t1_dn0 = assign61980_e96366_d_n0;
        locals.var_t1_dn2 = assign61980_e96366_d_n2;
        locals.var_t1_dn4 = assign61980_e96366_d_n4;
        locals.var_t1_dn5 = assign61980_e96366_d_n5;
        locals.var_t1_dn6 = assign61980_e96366_d_n6;
        locals.var_t1_dn7 = assign61980_e96366_d_n7;
        locals.var_t1_dn8 = assign61980_e96366_d_n8;
        locals.var_t1_dn9 = assign61980_e96366_d_n9;
        locals.var_t1_dn10 = assign61980_e96366_d_n10;
        locals.var_t1_dn13 = assign61980_e96366_d_n13;

        let (assign61990_e96375, assign61990_e96375_d_n0, assign61990_e96375_d_n2, assign61990_e96375_d_n4, assign61990_e96375_d_n5, assign61990_e96375_d_n6, assign61990_e96375_d_n7, assign61990_e96375_d_n8, assign61990_e96375_d_n9, assign61990_e96375_d_n10, assign61990_e96375_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign61990_e96373: f64 = (1.0 / locals.var_t1);
        (assign61990_e96373, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn13 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign61990_e96375;
        locals.var_muun_dn0 = assign61990_e96375_d_n0;
        locals.var_muun_dn2 = assign61990_e96375_d_n2;
        locals.var_muun_dn4 = assign61990_e96375_d_n4;
        locals.var_muun_dn5 = assign61990_e96375_d_n5;
        locals.var_muun_dn6 = assign61990_e96375_d_n6;
        locals.var_muun_dn7 = assign61990_e96375_d_n7;
        locals.var_muun_dn8 = assign61990_e96375_d_n8;
        locals.var_muun_dn9 = assign61990_e96375_d_n9;
        locals.var_muun_dn10 = assign61990_e96375_d_n10;
        locals.var_muun_dn13 = assign61990_e96375_d_n13;

        let (assign62000_e96384, assign62000_e96384_d_n0, assign62000_e96384_d_n2, assign62000_e96384_d_n4, assign62000_e96384_d_n5, assign62000_e96384_d_n6, assign62000_e96384_d_n7, assign62000_e96384_d_n8, assign62000_e96384_d_n9, assign62000_e96384_d_n10, assign62000_e96384_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62000_e96382: f64 = (locals.var_muun / 10000.0);
        (assign62000_e96382, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn13 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign62000_e96384;
        locals.var_muun_dn0 = assign62000_e96384_d_n0;
        locals.var_muun_dn2 = assign62000_e96384_d_n2;
        locals.var_muun_dn4 = assign62000_e96384_d_n4;
        locals.var_muun_dn5 = assign62000_e96384_d_n5;
        locals.var_muun_dn6 = assign62000_e96384_d_n6;
        locals.var_muun_dn7 = assign62000_e96384_d_n7;
        locals.var_muun_dn8 = assign62000_e96384_d_n8;
        locals.var_muun_dn9 = assign62000_e96384_d_n9;
        locals.var_muun_dn10 = assign62000_e96384_d_n10;
        locals.var_muun_dn13 = assign62000_e96384_d_n13;

        let (assign62010_e96397, assign62010_e96397_d_n0, assign62010_e96397_d_n2, assign62010_e96397_d_n4, assign62010_e96397_d_n5, assign62010_e96397_d_n6, assign62010_e96397_d_n7, assign62010_e96397_d_n8, assign62010_e96397_d_n9, assign62010_e96397_d_n10, assign62010_e96397_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62010_e96392: f64 = (locals.var_qn0 + 1e-25);
        let assign62010_e96393: f64 = (locals.var_beta * assign62010_e96392);
        let assign62010_e96395: f64 = (assign62010_e96393 * locals.var_lch);
        (assign62010_e96395, ((((locals.var_beta_dn0 * assign62010_e96392) + (locals.var_beta * locals.var_qn0_dn0)) * locals.var_lch) + (assign62010_e96393 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign62010_e96392) + (locals.var_beta * locals.var_qn0_dn2)) * locals.var_lch) + (assign62010_e96393 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign62010_e96392) + (locals.var_beta * locals.var_qn0_dn4)) * locals.var_lch) + (assign62010_e96393 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign62010_e96392) + (locals.var_beta * locals.var_qn0_dn5)) * locals.var_lch) + (assign62010_e96393 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign62010_e96392) + (locals.var_beta * locals.var_qn0_dn6)) * locals.var_lch) + (assign62010_e96393 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign62010_e96392) + (locals.var_beta * locals.var_qn0_dn7)) * locals.var_lch) + (assign62010_e96393 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign62010_e96392) + (locals.var_beta * locals.var_qn0_dn8)) * locals.var_lch) + (assign62010_e96393 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign62010_e96392) + (locals.var_beta * locals.var_qn0_dn9)) * locals.var_lch) + (assign62010_e96393 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign62010_e96392) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign62010_e96393 * locals.var_lch_dn10)), ((((locals.var_beta_dn13 * assign62010_e96392) + (locals.var_beta * locals.var_qn0_dn13)) * locals.var_lch) + (assign62010_e96393 * locals.var_lch_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62010_e96397;
        locals.var_t2_dn0 = assign62010_e96397_d_n0;
        locals.var_t2_dn2 = assign62010_e96397_d_n2;
        locals.var_t2_dn4 = assign62010_e96397_d_n4;
        locals.var_t2_dn5 = assign62010_e96397_d_n5;
        locals.var_t2_dn6 = assign62010_e96397_d_n6;
        locals.var_t2_dn7 = assign62010_e96397_d_n7;
        locals.var_t2_dn8 = assign62010_e96397_d_n8;
        locals.var_t2_dn9 = assign62010_e96397_d_n9;
        locals.var_t2_dn10 = assign62010_e96397_d_n10;
        locals.var_t2_dn13 = assign62010_e96397_d_n13;

        let (assign62020_e96406, assign62020_e96406_d_n0, assign62020_e96406_d_n2, assign62020_e96406_d_n4, assign62020_e96406_d_n5, assign62020_e96406_d_n6, assign62020_e96406_d_n7, assign62020_e96406_d_n8, assign62020_e96406_d_n9, assign62020_e96406_d_n10, assign62020_e96406_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62020_e96404: f64 = (1.0 / locals.var_t2);
        (assign62020_e96404, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn13 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign62020_e96406;
        locals.var_t1_dn0 = assign62020_e96406_d_n0;
        locals.var_t1_dn2 = assign62020_e96406_d_n2;
        locals.var_t1_dn4 = assign62020_e96406_d_n4;
        locals.var_t1_dn5 = assign62020_e96406_d_n5;
        locals.var_t1_dn6 = assign62020_e96406_d_n6;
        locals.var_t1_dn7 = assign62020_e96406_d_n7;
        locals.var_t1_dn8 = assign62020_e96406_d_n8;
        locals.var_t1_dn9 = assign62020_e96406_d_n9;
        locals.var_t1_dn10 = assign62020_e96406_d_n10;
        locals.var_t1_dn13 = assign62020_e96406_d_n13;

        let (assign62030_e96415, assign62030_e96415_d_n0, assign62030_e96415_d_n2, assign62030_e96415_d_n4, assign62030_e96415_d_n5, assign62030_e96415_d_n6, assign62030_e96415_d_n7, assign62030_e96415_d_n8, assign62030_e96415_d_n9, assign62030_e96415_d_n10, assign62030_e96415_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62030_e96413: f64 = (locals.var_t1 * locals.var_t1);
        (assign62030_e96413, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign62030_e96415;
        locals.var_t3_dn0 = assign62030_e96415_d_n0;
        locals.var_t3_dn2 = assign62030_e96415_d_n2;
        locals.var_t3_dn4 = assign62030_e96415_d_n4;
        locals.var_t3_dn5 = assign62030_e96415_d_n5;
        locals.var_t3_dn6 = assign62030_e96415_d_n6;
        locals.var_t3_dn7 = assign62030_e96415_d_n7;
        locals.var_t3_dn8 = assign62030_e96415_d_n8;
        locals.var_t3_dn9 = assign62030_e96415_d_n9;
        locals.var_t3_dn10 = assign62030_e96415_d_n10;
        locals.var_t3_dn13 = assign62030_e96415_d_n13;

        let (assign62040_e96425, assign62040_e96425_d_n0, assign62040_e96425_d_n2, assign62040_e96425_d_n4, assign62040_e96425_d_n5, assign62040_e96425_d_n6, assign62040_e96425_d_n7, assign62040_e96425_d_n8, assign62040_e96425_d_n9, assign62040_e96425_d_n10, assign62040_e96425_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62040_e96421: f64 = (-locals.var_beta);
        let assign62040_e96423: f64 = (assign62040_e96421 * locals.var_t3);
        (assign62040_e96423, (((-locals.var_beta_dn0) * locals.var_t3) + (assign62040_e96421 * locals.var_t3_dn0)), (((-locals.var_beta_dn2) * locals.var_t3) + (assign62040_e96421 * locals.var_t3_dn2)), (((-locals.var_beta_dn4) * locals.var_t3) + (assign62040_e96421 * locals.var_t3_dn4)), (((-locals.var_beta_dn5) * locals.var_t3) + (assign62040_e96421 * locals.var_t3_dn5)), (((-locals.var_beta_dn6) * locals.var_t3) + (assign62040_e96421 * locals.var_t3_dn6)), (((-locals.var_beta_dn7) * locals.var_t3) + (assign62040_e96421 * locals.var_t3_dn7)), (((-locals.var_beta_dn8) * locals.var_t3) + (assign62040_e96421 * locals.var_t3_dn8)), (((-locals.var_beta_dn9) * locals.var_t3) + (assign62040_e96421 * locals.var_t3_dn9)), (((-locals.var_beta_dn10) * locals.var_t3) + (assign62040_e96421 * locals.var_t3_dn10)), (((-locals.var_beta_dn13) * locals.var_t3) + (assign62040_e96421 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign62040_e96425;
        locals.var_t4_dn0 = assign62040_e96425_d_n0;
        locals.var_t4_dn2 = assign62040_e96425_d_n2;
        locals.var_t4_dn4 = assign62040_e96425_d_n4;
        locals.var_t4_dn5 = assign62040_e96425_d_n5;
        locals.var_t4_dn6 = assign62040_e96425_d_n6;
        locals.var_t4_dn7 = assign62040_e96425_d_n7;
        locals.var_t4_dn8 = assign62040_e96425_d_n8;
        locals.var_t4_dn9 = assign62040_e96425_d_n9;
        locals.var_t4_dn10 = assign62040_e96425_d_n10;
        locals.var_t4_dn13 = assign62040_e96425_d_n13;

        let (assign62050_e96434, assign62050_e96434_d_n0, assign62050_e96434_d_n2, assign62050_e96434_d_n4, assign62050_e96434_d_n5, assign62050_e96434_d_n6, assign62050_e96434_d_n7, assign62050_e96434_d_n8, assign62050_e96434_d_n9, assign62050_e96434_d_n10, assign62050_e96434_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62050_e96432: f64 = (locals.var_t4 * locals.var_lch);
        (assign62050_e96432, ((locals.var_t4_dn0 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn0)), ((locals.var_t4_dn2 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn2)), ((locals.var_t4_dn4 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn4)), ((locals.var_t4_dn5 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn5)), ((locals.var_t4_dn6 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn6)), ((locals.var_t4_dn7 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn7)), ((locals.var_t4_dn8 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn8)), ((locals.var_t4_dn9 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn9)), ((locals.var_t4_dn10 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn10)), ((locals.var_t4_dn13 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign62050_e96434;
        locals.var_t5_dn0 = assign62050_e96434_d_n0;
        locals.var_t5_dn2 = assign62050_e96434_d_n2;
        locals.var_t5_dn4 = assign62050_e96434_d_n4;
        locals.var_t5_dn5 = assign62050_e96434_d_n5;
        locals.var_t5_dn6 = assign62050_e96434_d_n6;
        locals.var_t5_dn7 = assign62050_e96434_d_n7;
        locals.var_t5_dn8 = assign62050_e96434_d_n8;
        locals.var_t5_dn9 = assign62050_e96434_d_n9;
        locals.var_t5_dn10 = assign62050_e96434_d_n10;
        locals.var_t5_dn13 = assign62050_e96434_d_n13;

        let (assign62060_e96445, assign62060_e96445_d_n0, assign62060_e96445_d_n2, assign62060_e96445_d_n4, assign62060_e96445_d_n5, assign62060_e96445_d_n6, assign62060_e96445_d_n7, assign62060_e96445_d_n8, assign62060_e96445_d_n9, assign62060_e96445_d_n10, assign62060_e96445_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62060_e96442: f64 = (locals.var_qn0 + 1e-25);
        let assign62060_e96443: f64 = (locals.var_t4 * assign62060_e96442);
        (assign62060_e96443, ((locals.var_t4_dn0 * assign62060_e96442) + (locals.var_t4 * locals.var_qn0_dn0)), ((locals.var_t4_dn2 * assign62060_e96442) + (locals.var_t4 * locals.var_qn0_dn2)), ((locals.var_t4_dn4 * assign62060_e96442) + (locals.var_t4 * locals.var_qn0_dn4)), ((locals.var_t4_dn5 * assign62060_e96442) + (locals.var_t4 * locals.var_qn0_dn5)), ((locals.var_t4_dn6 * assign62060_e96442) + (locals.var_t4 * locals.var_qn0_dn6)), ((locals.var_t4_dn7 * assign62060_e96442) + (locals.var_t4 * locals.var_qn0_dn7)), ((locals.var_t4_dn8 * assign62060_e96442) + (locals.var_t4 * locals.var_qn0_dn8)), ((locals.var_t4_dn9 * assign62060_e96442) + (locals.var_t4 * locals.var_qn0_dn9)), ((locals.var_t4_dn10 * assign62060_e96442) + (locals.var_t4 * locals.var_qn0_dn10)), ((locals.var_t4_dn13 * assign62060_e96442) + (locals.var_t4 * locals.var_qn0_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign62060_e96445;
        locals.var_t6_dn0 = assign62060_e96445_d_n0;
        locals.var_t6_dn2 = assign62060_e96445_d_n2;
        locals.var_t6_dn4 = assign62060_e96445_d_n4;
        locals.var_t6_dn5 = assign62060_e96445_d_n5;
        locals.var_t6_dn6 = assign62060_e96445_d_n6;
        locals.var_t6_dn7 = assign62060_e96445_d_n7;
        locals.var_t6_dn8 = assign62060_e96445_d_n8;
        locals.var_t6_dn9 = assign62060_e96445_d_n9;
        locals.var_t6_dn10 = assign62060_e96445_d_n10;
        locals.var_t6_dn13 = assign62060_e96445_d_n13;

        let (assign62070_e96460, assign62070_e96460_d_n0, assign62070_e96460_d_n2, assign62070_e96460_d_n4, assign62070_e96460_d_n5, assign62070_e96460_d_n6, assign62070_e96460_d_n7, assign62070_e96460_d_n8, assign62070_e96460_d_n9, assign62070_e96460_d_n10, assign62070_e96460_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62070_e96453: f64 = (10.0 * 2.220446049250313e-16);
        let assign62070_e96454: f64 = (locals.var_pds + assign62070_e96453);
        let assign62070_e96456: f64 = (assign62070_e96454 * locals.var_fdd);
        let assign62070_e96458: f64 = (assign62070_e96456 * locals.var_t1);
        (assign62070_e96458, ((((locals.var_pds_dn0 * locals.var_fdd) + (assign62070_e96454 * locals.var_fdd_dn0)) * locals.var_t1) + (assign62070_e96456 * locals.var_t1_dn0)), ((((locals.var_pds_dn2 * locals.var_fdd) + (assign62070_e96454 * locals.var_fdd_dn2)) * locals.var_t1) + (assign62070_e96456 * locals.var_t1_dn2)), ((((locals.var_pds_dn4 * locals.var_fdd) + (assign62070_e96454 * locals.var_fdd_dn4)) * locals.var_t1) + (assign62070_e96456 * locals.var_t1_dn4)), ((((locals.var_pds_dn5 * locals.var_fdd) + (assign62070_e96454 * locals.var_fdd_dn5)) * locals.var_t1) + (assign62070_e96456 * locals.var_t1_dn5)), ((((locals.var_pds_dn6 * locals.var_fdd) + (assign62070_e96454 * locals.var_fdd_dn6)) * locals.var_t1) + (assign62070_e96456 * locals.var_t1_dn6)), ((((locals.var_pds_dn7 * locals.var_fdd) + (assign62070_e96454 * locals.var_fdd_dn7)) * locals.var_t1) + (assign62070_e96456 * locals.var_t1_dn7)), ((((locals.var_pds_dn8 * locals.var_fdd) + (assign62070_e96454 * locals.var_fdd_dn8)) * locals.var_t1) + (assign62070_e96456 * locals.var_t1_dn8)), ((((locals.var_pds_dn9 * locals.var_fdd) + (assign62070_e96454 * locals.var_fdd_dn9)) * locals.var_t1) + (assign62070_e96456 * locals.var_t1_dn9)), ((((locals.var_pds_dn10 * locals.var_fdd) + (assign62070_e96454 * locals.var_fdd_dn10)) * locals.var_t1) + (assign62070_e96456 * locals.var_t1_dn10)), ((((locals.var_pds_dn13 * locals.var_fdd) + (assign62070_e96454 * locals.var_fdd_dn13)) * locals.var_t1) + (assign62070_e96456 * locals.var_t1_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign62070_e96460;
        locals.var_ty_dn0 = assign62070_e96460_d_n0;
        locals.var_ty_dn2 = assign62070_e96460_d_n2;
        locals.var_ty_dn4 = assign62070_e96460_d_n4;
        locals.var_ty_dn5 = assign62070_e96460_d_n5;
        locals.var_ty_dn6 = assign62070_e96460_d_n6;
        locals.var_ty_dn7 = assign62070_e96460_d_n7;
        locals.var_ty_dn8 = assign62070_e96460_d_n8;
        locals.var_ty_dn9 = assign62070_e96460_d_n9;
        locals.var_ty_dn10 = assign62070_e96460_d_n10;
        locals.var_ty_dn13 = assign62070_e96460_d_n13;

        let (assign62080_e96471, assign62080_e96471_d_n0, assign62080_e96471_d_n2, assign62080_e96471_d_n4, assign62080_e96471_d_n5, assign62080_e96471_d_n6, assign62080_e96471_d_n7, assign62080_e96471_d_n8, assign62080_e96471_d_n9, assign62080_e96471_d_n10, assign62080_e96471_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62080_e96467: f64 = (0.2 * locals.var_vmaxe);
        let assign62080_e96469: f64 = (assign62080_e96467 / locals.var_muun);
        (assign62080_e96469, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign62080_e96467 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign62080_e96467 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign62080_e96467 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign62080_e96467 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign62080_e96467 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign62080_e96467 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign62080_e96467 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign62080_e96467 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign62080_e96467 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn13) * locals.var_muun) - (assign62080_e96467 * locals.var_muun_dn13)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62080_e96471;
        locals.var_t2_dn0 = assign62080_e96471_d_n0;
        locals.var_t2_dn2 = assign62080_e96471_d_n2;
        locals.var_t2_dn4 = assign62080_e96471_d_n4;
        locals.var_t2_dn5 = assign62080_e96471_d_n5;
        locals.var_t2_dn6 = assign62080_e96471_d_n6;
        locals.var_t2_dn7 = assign62080_e96471_d_n7;
        locals.var_t2_dn8 = assign62080_e96471_d_n8;
        locals.var_t2_dn9 = assign62080_e96471_d_n9;
        locals.var_t2_dn10 = assign62080_e96471_d_n10;
        locals.var_t2_dn13 = assign62080_e96471_d_n13;

        let (assign62090_e96481, assign62090_e96481_d_n0, assign62090_e96481_d_n2, assign62090_e96481_d_n4, assign62090_e96481_d_n5, assign62090_e96481_d_n6, assign62090_e96481_d_n7, assign62090_e96481_d_n8, assign62090_e96481_d_n9, assign62090_e96481_d_n10, assign62090_e96481_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62090_e96477: f64 = (-locals.var_t2);
        let assign62090_e96479: f64 = (assign62090_e96477 / locals.var_muun);
        (assign62090_e96479, ((((-locals.var_t2_dn0) * locals.var_muun) - (assign62090_e96477 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn2) * locals.var_muun) - (assign62090_e96477 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn4) * locals.var_muun) - (assign62090_e96477 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn5) * locals.var_muun) - (assign62090_e96477 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn6) * locals.var_muun) - (assign62090_e96477 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn7) * locals.var_muun) - (assign62090_e96477 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn8) * locals.var_muun) - (assign62090_e96477 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn9) * locals.var_muun) - (assign62090_e96477 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn10) * locals.var_muun) - (assign62090_e96477 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn13) * locals.var_muun) - (assign62090_e96477 * locals.var_muun_dn13)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign62090_e96481;
        locals.var_t3_dn0 = assign62090_e96481_d_n0;
        locals.var_t3_dn2 = assign62090_e96481_d_n2;
        locals.var_t3_dn4 = assign62090_e96481_d_n4;
        locals.var_t3_dn5 = assign62090_e96481_d_n5;
        locals.var_t3_dn6 = assign62090_e96481_d_n6;
        locals.var_t3_dn7 = assign62090_e96481_d_n7;
        locals.var_t3_dn8 = assign62090_e96481_d_n8;
        locals.var_t3_dn9 = assign62090_e96481_d_n9;
        locals.var_t3_dn10 = assign62090_e96481_d_n10;
        locals.var_t3_dn13 = assign62090_e96481_d_n13;

        let (assign62100_e96495, assign62100_e96495_d_n0, assign62100_e96495_d_n2, assign62100_e96495_d_n4, assign62100_e96495_d_n5, assign62100_e96495_d_n6, assign62100_e96495_d_n7, assign62100_e96495_d_n8, assign62100_e96495_d_n9, assign62100_e96495_d_n10, assign62100_e96495_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62100_e96488: f64 = (locals.var_ty * locals.var_ty);
        let assign62100_e96491: f64 = (locals.var_t2 * locals.var_t2);
        let assign62100_e96492: f64 = (assign62100_e96488 + assign62100_e96491);
        let assign62100_e96493: f64 = (assign62100_e96492).sqrt();
        (assign62100_e96493, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign62100_e96493)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign62100_e96493)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign62100_e96493)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign62100_e96493)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign62100_e96493)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign62100_e96493)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign62100_e96493)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign62100_e96493)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign62100_e96493)), ((((locals.var_ty_dn13 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn13)) + ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13))) / (2.0 * assign62100_e96493)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn13,)
    }
};
        locals.var_ey = assign62100_e96495;
        locals.var_ey_dn0 = assign62100_e96495_d_n0;
        locals.var_ey_dn2 = assign62100_e96495_d_n2;
        locals.var_ey_dn4 = assign62100_e96495_d_n4;
        locals.var_ey_dn5 = assign62100_e96495_d_n5;
        locals.var_ey_dn6 = assign62100_e96495_d_n6;
        locals.var_ey_dn7 = assign62100_e96495_d_n7;
        locals.var_ey_dn8 = assign62100_e96495_d_n8;
        locals.var_ey_dn9 = assign62100_e96495_d_n9;
        locals.var_ey_dn10 = assign62100_e96495_d_n10;
        locals.var_ey_dn13 = assign62100_e96495_d_n13;

        let (assign62110_e96504, assign62110_e96504_d_n0, assign62110_e96504_d_n2, assign62110_e96504_d_n4, assign62110_e96504_d_n5, assign62110_e96504_d_n6, assign62110_e96504_d_n7, assign62110_e96504_d_n8, assign62110_e96504_d_n9, assign62110_e96504_d_n10, assign62110_e96504_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62110_e96502: f64 = (1.0 / locals.var_ey);
        (assign62110_e96502, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn13 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign62110_e96504;
        locals.var_t4_dn0 = assign62110_e96504_d_n0;
        locals.var_t4_dn2 = assign62110_e96504_d_n2;
        locals.var_t4_dn4 = assign62110_e96504_d_n4;
        locals.var_t4_dn5 = assign62110_e96504_d_n5;
        locals.var_t4_dn6 = assign62110_e96504_d_n6;
        locals.var_t4_dn7 = assign62110_e96504_d_n7;
        locals.var_t4_dn8 = assign62110_e96504_d_n8;
        locals.var_t4_dn9 = assign62110_e96504_d_n9;
        locals.var_t4_dn10 = assign62110_e96504_d_n10;
        locals.var_t4_dn13 = assign62110_e96504_d_n13;

        let (assign62120_e96513, assign62120_e96513_d_n0, assign62120_e96513_d_n2, assign62120_e96513_d_n4, assign62120_e96513_d_n5, assign62120_e96513_d_n6, assign62120_e96513_d_n7, assign62120_e96513_d_n8, assign62120_e96513_d_n9, assign62120_e96513_d_n10, assign62120_e96513_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62120_e96511: f64 = (locals.var_muun * locals.var_ey);
        (assign62120_e96511, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn13 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn13)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn13,)
    }
};
        locals.var_em = assign62120_e96513;
        locals.var_em_dn0 = assign62120_e96513_d_n0;
        locals.var_em_dn2 = assign62120_e96513_d_n2;
        locals.var_em_dn4 = assign62120_e96513_d_n4;
        locals.var_em_dn5 = assign62120_e96513_d_n5;
        locals.var_em_dn6 = assign62120_e96513_d_n6;
        locals.var_em_dn7 = assign62120_e96513_d_n7;
        locals.var_em_dn8 = assign62120_e96513_d_n8;
        locals.var_em_dn9 = assign62120_e96513_d_n9;
        locals.var_em_dn10 = assign62120_e96513_d_n10;
        locals.var_em_dn13 = assign62120_e96513_d_n13;

        let (assign62130_e96522, assign62130_e96522_d_n0, assign62130_e96522_d_n2, assign62130_e96522_d_n4, assign62130_e96522_d_n5, assign62130_e96522_d_n6, assign62130_e96522_d_n7, assign62130_e96522_d_n8, assign62130_e96522_d_n9, assign62130_e96522_d_n10, assign62130_e96522_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62130_e96520: f64 = (locals.var_em / locals.var_vmaxe);
        (assign62130_e96520, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn13 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn13)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign62130_e96522;
        locals.var_t1_dn0 = assign62130_e96522_d_n0;
        locals.var_t1_dn2 = assign62130_e96522_d_n2;
        locals.var_t1_dn4 = assign62130_e96522_d_n4;
        locals.var_t1_dn5 = assign62130_e96522_d_n5;
        locals.var_t1_dn6 = assign62130_e96522_d_n6;
        locals.var_t1_dn7 = assign62130_e96522_d_n7;
        locals.var_t1_dn8 = assign62130_e96522_d_n8;
        locals.var_t1_dn9 = assign62130_e96522_d_n9;
        locals.var_t1_dn10 = assign62130_e96522_d_n10;
        locals.var_t1_dn13 = assign62130_e96522_d_n13;

        let assign62140_e96526: f64 = (10.0 * 2.220446049250313e-16);
        let assign62140_e96527: f64 = (1.0 - assign62140_e96526);
        let assign62140_e96534: f64 = (10.0 * 2.220446049250313e-16);
        let assign62140_e96535: f64 = (1.0 + assign62140_e96534);
        let assign62140_e96537: f64 = if ((assign62140_e96527 <= p.p178) && (p.p178 <= assign62140_e96535)) { 1.0 } else { 0.0 };
        locals.var_guard1491 = assign62140_e96537;

        let (assign62150_e96546, assign62150_e96546_d_n0, assign62150_e96546_d_n2, assign62150_e96546_d_n4, assign62150_e96546_d_n5, assign62150_e96546_d_n6, assign62150_e96546_d_n7, assign62150_e96546_d_n8, assign62150_e96546_d_n9, assign62150_e96546_d_n10, assign62150_e96546_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign62150_e96546;
        locals.var_t3_dn0 = assign62150_e96546_d_n0;
        locals.var_t3_dn2 = assign62150_e96546_d_n2;
        locals.var_t3_dn4 = assign62150_e96546_d_n4;
        locals.var_t3_dn5 = assign62150_e96546_d_n5;
        locals.var_t3_dn6 = assign62150_e96546_d_n6;
        locals.var_t3_dn7 = assign62150_e96546_d_n7;
        locals.var_t3_dn8 = assign62150_e96546_d_n8;
        locals.var_t3_dn9 = assign62150_e96546_d_n9;
        locals.var_t3_dn10 = assign62150_e96546_d_n10;
        locals.var_t3_dn13 = assign62150_e96546_d_n13;

        let assign62160_e96550: f64 = (10.0 * 2.220446049250313e-16);
        let assign62160_e96551: f64 = (2.0 - assign62160_e96550);
        let assign62160_e96558: f64 = (10.0 * 2.220446049250313e-16);
        let assign62160_e96559: f64 = (2.0 + assign62160_e96558);
        let assign62160_e96561: f64 = if ((assign62160_e96551 <= p.p178) && (p.p178 <= assign62160_e96559)) { 1.0 } else { 0.0 };
        locals.var_guard1492 = assign62160_e96561;

        let (assign62170_e96573, assign62170_e96573_d_n0, assign62170_e96573_d_n2, assign62170_e96573_d_n4, assign62170_e96573_d_n5, assign62170_e96573_d_n6, assign62170_e96573_d_n7, assign62170_e96573_d_n8, assign62170_e96573_d_n9, assign62170_e96573_d_n10, assign62170_e96573_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1491 == 0.0)) && (locals.var_guard1492 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign62170_e96573;
        locals.var_t3_dn0 = assign62170_e96573_d_n0;
        locals.var_t3_dn2 = assign62170_e96573_d_n2;
        locals.var_t3_dn4 = assign62170_e96573_d_n4;
        locals.var_t3_dn5 = assign62170_e96573_d_n5;
        locals.var_t3_dn6 = assign62170_e96573_d_n6;
        locals.var_t3_dn7 = assign62170_e96573_d_n7;
        locals.var_t3_dn8 = assign62170_e96573_d_n8;
        locals.var_t3_dn9 = assign62170_e96573_d_n9;
        locals.var_t3_dn10 = assign62170_e96573_d_n10;
        locals.var_t3_dn13 = assign62170_e96573_d_n13;

        let (assign62180_e96595, assign62180_e96595_d_n0, assign62180_e96595_d_n2, assign62180_e96595_d_n4, assign62180_e96595_d_n5, assign62180_e96595_d_n6, assign62180_e96595_d_n7, assign62180_e96595_d_n8, assign62180_e96595_d_n9, assign62180_e96595_d_n10, assign62180_e96595_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1491 == 0.0)) && (locals.var_guard1492 == 0.0)) {
        let (assign62180_e96593, assign62180_e96593_d_n0, assign62180_e96593_d_n2, assign62180_e96593_d_n4, assign62180_e96593_d_n5, assign62180_e96593_d_n6, assign62180_e96593_d_n7, assign62180_e96593_d_n8, assign62180_e96593_d_n9, assign62180_e96593_d_n10, assign62180_e96593_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign62180_e96591: f64 = (p.p178 - 1.0);
                let assign62180_e96592: f64 = (locals.var_t1).powf(assign62180_e96591);
                (assign62180_e96592, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((locals.var_t1).powf(assign62180_e96591 - 1.0) * locals.var_t1_dn0)) } } else { (assign62180_e96592 * (assign62180_e96591 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((locals.var_t1).powf(assign62180_e96591 - 1.0) * locals.var_t1_dn2)) } } else { (assign62180_e96592 * (assign62180_e96591 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((locals.var_t1).powf(assign62180_e96591 - 1.0) * locals.var_t1_dn4)) } } else { (assign62180_e96592 * (assign62180_e96591 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((locals.var_t1).powf(assign62180_e96591 - 1.0) * locals.var_t1_dn5)) } } else { (assign62180_e96592 * (assign62180_e96591 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((locals.var_t1).powf(assign62180_e96591 - 1.0) * locals.var_t1_dn6)) } } else { (assign62180_e96592 * (assign62180_e96591 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((locals.var_t1).powf(assign62180_e96591 - 1.0) * locals.var_t1_dn7)) } } else { (assign62180_e96592 * (assign62180_e96591 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((locals.var_t1).powf(assign62180_e96591 - 1.0) * locals.var_t1_dn8)) } } else { (assign62180_e96592 * (assign62180_e96591 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((locals.var_t1).powf(assign62180_e96591 - 1.0) * locals.var_t1_dn9)) } } else { (assign62180_e96592 * (assign62180_e96591 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((locals.var_t1).powf(assign62180_e96591 - 1.0) * locals.var_t1_dn10)) } } else { (assign62180_e96592 * (assign62180_e96591 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((locals.var_t1).powf(assign62180_e96591 - 1.0) * locals.var_t1_dn13)) } } else { (assign62180_e96592 * (assign62180_e96591 * (locals.var_t1_dn13 / locals.var_t1))) },)
            }
        };
        (assign62180_e96593, assign62180_e96593_d_n0, assign62180_e96593_d_n2, assign62180_e96593_d_n4, assign62180_e96593_d_n5, assign62180_e96593_d_n6, assign62180_e96593_d_n7, assign62180_e96593_d_n8, assign62180_e96593_d_n9, assign62180_e96593_d_n10, assign62180_e96593_d_n13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign62180_e96595;
        locals.var_t3_dn0 = assign62180_e96595_d_n0;
        locals.var_t3_dn2 = assign62180_e96595_d_n2;
        locals.var_t3_dn4 = assign62180_e96595_d_n4;
        locals.var_t3_dn5 = assign62180_e96595_d_n5;
        locals.var_t3_dn6 = assign62180_e96595_d_n6;
        locals.var_t3_dn7 = assign62180_e96595_d_n7;
        locals.var_t3_dn8 = assign62180_e96595_d_n8;
        locals.var_t3_dn9 = assign62180_e96595_d_n9;
        locals.var_t3_dn10 = assign62180_e96595_d_n10;
        locals.var_t3_dn13 = assign62180_e96595_d_n13;

        let (assign62190_e96604, assign62190_e96604_d_n0, assign62190_e96604_d_n2, assign62190_e96604_d_n4, assign62190_e96604_d_n5, assign62190_e96604_d_n6, assign62190_e96604_d_n7, assign62190_e96604_d_n8, assign62190_e96604_d_n9, assign62190_e96604_d_n10, assign62190_e96604_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62190_e96602: f64 = (locals.var_t1 * locals.var_t3);
        (assign62190_e96602, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62190_e96604;
        locals.var_t2_dn0 = assign62190_e96604_d_n0;
        locals.var_t2_dn2 = assign62190_e96604_d_n2;
        locals.var_t2_dn4 = assign62190_e96604_d_n4;
        locals.var_t2_dn5 = assign62190_e96604_d_n5;
        locals.var_t2_dn6 = assign62190_e96604_d_n6;
        locals.var_t2_dn7 = assign62190_e96604_d_n7;
        locals.var_t2_dn8 = assign62190_e96604_d_n8;
        locals.var_t2_dn9 = assign62190_e96604_d_n9;
        locals.var_t2_dn10 = assign62190_e96604_d_n10;
        locals.var_t2_dn13 = assign62190_e96604_d_n13;

    }

    pub(super) fn stamp_transient_block_210(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62200_e96613, assign62200_e96613_d_n0, assign62200_e96613_d_n2, assign62200_e96613_d_n4, assign62200_e96613_d_n5, assign62200_e96613_d_n6, assign62200_e96613_d_n7, assign62200_e96613_d_n8, assign62200_e96613_d_n9, assign62200_e96613_d_n10, assign62200_e96613_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62200_e96611: f64 = (1.0 + locals.var_t2);
        (assign62200_e96611, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign62200_e96613;
        locals.var_t4_dn0 = assign62200_e96613_d_n0;
        locals.var_t4_dn2 = assign62200_e96613_d_n2;
        locals.var_t4_dn4 = assign62200_e96613_d_n4;
        locals.var_t4_dn5 = assign62200_e96613_d_n5;
        locals.var_t4_dn6 = assign62200_e96613_d_n6;
        locals.var_t4_dn7 = assign62200_e96613_d_n7;
        locals.var_t4_dn8 = assign62200_e96613_d_n8;
        locals.var_t4_dn9 = assign62200_e96613_d_n9;
        locals.var_t4_dn10 = assign62200_e96613_d_n10;
        locals.var_t4_dn13 = assign62200_e96613_d_n13;

        let assign62210_e96617: f64 = (10.0 * 2.220446049250313e-16);
        let assign62210_e96618: f64 = (1.0 - assign62210_e96617);
        let assign62210_e96625: f64 = (10.0 * 2.220446049250313e-16);
        let assign62210_e96626: f64 = (1.0 + assign62210_e96625);
        let assign62210_e96628: f64 = if ((assign62210_e96618 <= p.p178) && (p.p178 <= assign62210_e96626)) { 1.0 } else { 0.0 };
        locals.var_guard1493 = assign62210_e96628;

        let (assign62220_e96639, assign62220_e96639_d_n0, assign62220_e96639_d_n2, assign62220_e96639_d_n4, assign62220_e96639_d_n5, assign62220_e96639_d_n6, assign62220_e96639_d_n7, assign62220_e96639_d_n8, assign62220_e96639_d_n9, assign62220_e96639_d_n10, assign62220_e96639_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign62220_e96637: f64 = (1.0 / locals.var_t4);
        (assign62220_e96637, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign62220_e96639;
        locals.var_t5_dn0 = assign62220_e96639_d_n0;
        locals.var_t5_dn2 = assign62220_e96639_d_n2;
        locals.var_t5_dn4 = assign62220_e96639_d_n4;
        locals.var_t5_dn5 = assign62220_e96639_d_n5;
        locals.var_t5_dn6 = assign62220_e96639_d_n6;
        locals.var_t5_dn7 = assign62220_e96639_d_n7;
        locals.var_t5_dn8 = assign62220_e96639_d_n8;
        locals.var_t5_dn9 = assign62220_e96639_d_n9;
        locals.var_t5_dn10 = assign62220_e96639_d_n10;
        locals.var_t5_dn13 = assign62220_e96639_d_n13;

        let assign62230_e96643: f64 = (10.0 * 2.220446049250313e-16);
        let assign62230_e96644: f64 = (2.0 - assign62230_e96643);
        let assign62230_e96651: f64 = (10.0 * 2.220446049250313e-16);
        let assign62230_e96652: f64 = (2.0 + assign62230_e96651);
        let assign62230_e96654: f64 = if ((assign62230_e96644 <= p.p178) && (p.p178 <= assign62230_e96652)) { 1.0 } else { 0.0 };
        locals.var_guard1494 = assign62230_e96654;

        let (assign62240_e96669, assign62240_e96669_d_n0, assign62240_e96669_d_n2, assign62240_e96669_d_n4, assign62240_e96669_d_n5, assign62240_e96669_d_n6, assign62240_e96669_d_n7, assign62240_e96669_d_n8, assign62240_e96669_d_n9, assign62240_e96669_d_n10, assign62240_e96669_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign62240_e96666: f64 = (locals.var_t4).sqrt();
        let assign62240_e96667: f64 = (1.0 / assign62240_e96666);
        (assign62240_e96667, (-((locals.var_t4_dn0 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((locals.var_t4_dn2 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((locals.var_t4_dn4 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((locals.var_t4_dn5 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((locals.var_t4_dn6 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((locals.var_t4_dn7 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((locals.var_t4_dn8 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((locals.var_t4_dn9 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((locals.var_t4_dn10 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((locals.var_t4_dn13 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign62240_e96669;
        locals.var_t5_dn0 = assign62240_e96669_d_n0;
        locals.var_t5_dn2 = assign62240_e96669_d_n2;
        locals.var_t5_dn4 = assign62240_e96669_d_n4;
        locals.var_t5_dn5 = assign62240_e96669_d_n5;
        locals.var_t5_dn6 = assign62240_e96669_d_n6;
        locals.var_t5_dn7 = assign62240_e96669_d_n7;
        locals.var_t5_dn8 = assign62240_e96669_d_n8;
        locals.var_t5_dn9 = assign62240_e96669_d_n9;
        locals.var_t5_dn10 = assign62240_e96669_d_n10;
        locals.var_t5_dn13 = assign62240_e96669_d_n13;

        let (assign62250_e96694, assign62250_e96694_d_n0, assign62250_e96694_d_n2, assign62250_e96694_d_n4, assign62250_e96694_d_n5, assign62250_e96694_d_n6, assign62250_e96694_d_n7, assign62250_e96694_d_n8, assign62250_e96694_d_n9, assign62250_e96694_d_n10, assign62250_e96694_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 == 0.0)) {
        let (assign62250_e96692, assign62250_e96692_d_n0, assign62250_e96692_d_n2, assign62250_e96692_d_n4, assign62250_e96692_d_n5, assign62250_e96692_d_n6, assign62250_e96692_d_n7, assign62250_e96692_d_n8, assign62250_e96692_d_n9, assign62250_e96692_d_n10, assign62250_e96692_d_n13,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign62250_e96686: f64 = (-1.0);
                let assign62250_e96688: f64 = (assign62250_e96686 / p.p178);
                let assign62250_e96690: f64 = (assign62250_e96688 - 1.0);
                let assign62250_e96691: f64 = (locals.var_t4).powf(assign62250_e96690);
                (assign62250_e96691, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((locals.var_t4).powf(assign62250_e96690 - 1.0) * locals.var_t4_dn0)) } } else { (assign62250_e96691 * (assign62250_e96690 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((locals.var_t4).powf(assign62250_e96690 - 1.0) * locals.var_t4_dn2)) } } else { (assign62250_e96691 * (assign62250_e96690 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((locals.var_t4).powf(assign62250_e96690 - 1.0) * locals.var_t4_dn4)) } } else { (assign62250_e96691 * (assign62250_e96690 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((locals.var_t4).powf(assign62250_e96690 - 1.0) * locals.var_t4_dn5)) } } else { (assign62250_e96691 * (assign62250_e96690 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((locals.var_t4).powf(assign62250_e96690 - 1.0) * locals.var_t4_dn6)) } } else { (assign62250_e96691 * (assign62250_e96690 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((locals.var_t4).powf(assign62250_e96690 - 1.0) * locals.var_t4_dn7)) } } else { (assign62250_e96691 * (assign62250_e96690 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((locals.var_t4).powf(assign62250_e96690 - 1.0) * locals.var_t4_dn8)) } } else { (assign62250_e96691 * (assign62250_e96690 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((locals.var_t4).powf(assign62250_e96690 - 1.0) * locals.var_t4_dn9)) } } else { (assign62250_e96691 * (assign62250_e96690 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((locals.var_t4).powf(assign62250_e96690 - 1.0) * locals.var_t4_dn10)) } } else { (assign62250_e96691 * (assign62250_e96690 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((locals.var_t4).powf(assign62250_e96690 - 1.0) * locals.var_t4_dn13)) } } else { (assign62250_e96691 * (assign62250_e96690 * (locals.var_t4_dn13 / locals.var_t4))) },)
            }
        };
        (assign62250_e96692, assign62250_e96692_d_n0, assign62250_e96692_d_n2, assign62250_e96692_d_n4, assign62250_e96692_d_n5, assign62250_e96692_d_n6, assign62250_e96692_d_n7, assign62250_e96692_d_n8, assign62250_e96692_d_n9, assign62250_e96692_d_n10, assign62250_e96692_d_n13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign62250_e96694;
        locals.var_t6_dn0 = assign62250_e96694_d_n0;
        locals.var_t6_dn2 = assign62250_e96694_d_n2;
        locals.var_t6_dn4 = assign62250_e96694_d_n4;
        locals.var_t6_dn5 = assign62250_e96694_d_n5;
        locals.var_t6_dn6 = assign62250_e96694_d_n6;
        locals.var_t6_dn7 = assign62250_e96694_d_n7;
        locals.var_t6_dn8 = assign62250_e96694_d_n8;
        locals.var_t6_dn9 = assign62250_e96694_d_n9;
        locals.var_t6_dn10 = assign62250_e96694_d_n10;
        locals.var_t6_dn13 = assign62250_e96694_d_n13;

        let (assign62260_e96709, assign62260_e96709_d_n0, assign62260_e96709_d_n2, assign62260_e96709_d_n4, assign62260_e96709_d_n5, assign62260_e96709_d_n6, assign62260_e96709_d_n7, assign62260_e96709_d_n8, assign62260_e96709_d_n9, assign62260_e96709_d_n10, assign62260_e96709_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign62260_e96707: f64 = (locals.var_t4 * locals.var_t6);
        (assign62260_e96707, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn13 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign62260_e96709;
        locals.var_t5_dn0 = assign62260_e96709_d_n0;
        locals.var_t5_dn2 = assign62260_e96709_d_n2;
        locals.var_t5_dn4 = assign62260_e96709_d_n4;
        locals.var_t5_dn5 = assign62260_e96709_d_n5;
        locals.var_t5_dn6 = assign62260_e96709_d_n6;
        locals.var_t5_dn7 = assign62260_e96709_d_n7;
        locals.var_t5_dn8 = assign62260_e96709_d_n8;
        locals.var_t5_dn9 = assign62260_e96709_d_n9;
        locals.var_t5_dn10 = assign62260_e96709_d_n10;
        locals.var_t5_dn13 = assign62260_e96709_d_n13;

        let (assign62270_e96718, assign62270_e96718_d_n0, assign62270_e96718_d_n2, assign62270_e96718_d_n4, assign62270_e96718_d_n5, assign62270_e96718_d_n6, assign62270_e96718_d_n7, assign62270_e96718_d_n8, assign62270_e96718_d_n9, assign62270_e96718_d_n10, assign62270_e96718_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62270_e96716: f64 = (locals.var_muun * locals.var_t5);
        (assign62270_e96716, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn13 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn13)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn13,)
    }
};
        locals.var_mu = assign62270_e96718;
        locals.var_mu_dn0 = assign62270_e96718_d_n0;
        locals.var_mu_dn2 = assign62270_e96718_d_n2;
        locals.var_mu_dn4 = assign62270_e96718_d_n4;
        locals.var_mu_dn5 = assign62270_e96718_d_n5;
        locals.var_mu_dn6 = assign62270_e96718_d_n6;
        locals.var_mu_dn7 = assign62270_e96718_d_n7;
        locals.var_mu_dn8 = assign62270_e96718_d_n8;
        locals.var_mu_dn9 = assign62270_e96718_d_n9;
        locals.var_mu_dn10 = assign62270_e96718_d_n10;
        locals.var_mu_dn13 = assign62270_e96718_d_n13;

        let (assign62280_e96729, assign62280_e96729_d_n0, assign62280_e96729_d_n2, assign62280_e96729_d_n4, assign62280_e96729_d_n5, assign62280_e96729_d_n6, assign62280_e96729_d_n7, assign62280_e96729_d_n8, assign62280_e96729_d_n9, assign62280_e96729_d_n10, assign62280_e96729_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62280_e96725: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign62280_e96727: f64 = (assign62280_e96725 / locals.var_lch);
        (assign62280_e96727, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign62280_e96725 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign62280_e96725 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign62280_e96725 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign62280_e96725 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign62280_e96725 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign62280_e96725 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign62280_e96725 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign62280_e96725 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign62280_e96725 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn13) * locals.var_lch) - (assign62280_e96725 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn13,)
    }
};
        locals.var_betawl = assign62280_e96729;
        locals.var_betawl_dn0 = assign62280_e96729_d_n0;
        locals.var_betawl_dn2 = assign62280_e96729_d_n2;
        locals.var_betawl_dn4 = assign62280_e96729_d_n4;
        locals.var_betawl_dn5 = assign62280_e96729_d_n5;
        locals.var_betawl_dn6 = assign62280_e96729_d_n6;
        locals.var_betawl_dn7 = assign62280_e96729_d_n7;
        locals.var_betawl_dn8 = assign62280_e96729_d_n8;
        locals.var_betawl_dn9 = assign62280_e96729_d_n9;
        locals.var_betawl_dn10 = assign62280_e96729_d_n10;
        locals.var_betawl_dn13 = assign62280_e96729_d_n13;

        let (assign62290_e96739, assign62290_e96739_d_n0, assign62290_e96739_d_n2, assign62290_e96739_d_n4, assign62290_e96739_d_n5, assign62290_e96739_d_n6, assign62290_e96739_d_n7, assign62290_e96739_d_n8, assign62290_e96739_d_n9, assign62290_e96739_d_n10, assign62290_e96739_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62290_e96735: f64 = (-locals.var_betawl);
        let assign62290_e96737: f64 = (assign62290_e96735 / locals.var_lch);
        (assign62290_e96737, ((((-locals.var_betawl_dn0) * locals.var_lch) - (assign62290_e96735 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn2) * locals.var_lch) - (assign62290_e96735 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn4) * locals.var_lch) - (assign62290_e96735 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn5) * locals.var_lch) - (assign62290_e96735 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn6) * locals.var_lch) - (assign62290_e96735 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn7) * locals.var_lch) - (assign62290_e96735 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn8) * locals.var_lch) - (assign62290_e96735 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn9) * locals.var_lch) - (assign62290_e96735 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn10) * locals.var_lch) - (assign62290_e96735 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn13) * locals.var_lch) - (assign62290_e96735 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign62290_e96739;
        locals.var_t1_dn0 = assign62290_e96739_d_n0;
        locals.var_t1_dn2 = assign62290_e96739_d_n2;
        locals.var_t1_dn4 = assign62290_e96739_d_n4;
        locals.var_t1_dn5 = assign62290_e96739_d_n5;
        locals.var_t1_dn6 = assign62290_e96739_d_n6;
        locals.var_t1_dn7 = assign62290_e96739_d_n7;
        locals.var_t1_dn8 = assign62290_e96739_d_n8;
        locals.var_t1_dn9 = assign62290_e96739_d_n9;
        locals.var_t1_dn10 = assign62290_e96739_d_n10;
        locals.var_t1_dn13 = assign62290_e96739_d_n13;

        let (assign62300_e96750, assign62300_e96750_d_n0, assign62300_e96750_d_n2, assign62300_e96750_d_n4, assign62300_e96750_d_n5, assign62300_e96750_d_n6, assign62300_e96750_d_n7, assign62300_e96750_d_n8, assign62300_e96750_d_n9, assign62300_e96750_d_n10, assign62300_e96750_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign62300_e96746: f64 = (locals.var_betawl * locals.var_idd);
        let assign62300_e96748: f64 = (assign62300_e96746 * locals.var_mu);
        (assign62300_e96748, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign62300_e96746 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign62300_e96746 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu) + (assign62300_e96746 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu) + (assign62300_e96746 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign62300_e96746 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign62300_e96746 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu) + (assign62300_e96746 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu) + (assign62300_e96746 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign62300_e96746 * locals.var_mu_dn10)), ((((locals.var_betawl_dn13 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn13)) * locals.var_mu) + (assign62300_e96746 * locals.var_mu_dn13)),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    }
};
        locals.var_ids0 = assign62300_e96750;
        locals.var_ids0_dn0 = assign62300_e96750_d_n0;
        locals.var_ids0_dn2 = assign62300_e96750_d_n2;
        locals.var_ids0_dn4 = assign62300_e96750_d_n4;
        locals.var_ids0_dn5 = assign62300_e96750_d_n5;
        locals.var_ids0_dn6 = assign62300_e96750_d_n6;
        locals.var_ids0_dn7 = assign62300_e96750_d_n7;
        locals.var_ids0_dn8 = assign62300_e96750_d_n8;
        locals.var_ids0_dn9 = assign62300_e96750_d_n9;
        locals.var_ids0_dn10 = assign62300_e96750_d_n10;
        locals.var_ids0_dn13 = assign62300_e96750_d_n13;

        let assign62310_e96753: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1495 = assign62310_e96753;

        let (assign62320_e96766, assign62320_e96766_d_n0, assign62320_e96766_d_n2, assign62320_e96766_d_n4, assign62320_e96766_d_n5, assign62320_e96766_d_n6, assign62320_e96766_d_n7, assign62320_e96766_d_n8, assign62320_e96766_d_n9, assign62320_e96766_d_n10, assign62320_e96766_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62320_e96763: f64 = (locals.var_vds - locals.var_pds);
        let assign62320_e96764: f64 = (0.5 * assign62320_e96763);
        (assign62320_e96764, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn13 - locals.var_pds_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign62320_e96766;
        locals.var_t1_dn0 = assign62320_e96766_d_n0;
        locals.var_t1_dn2 = assign62320_e96766_d_n2;
        locals.var_t1_dn4 = assign62320_e96766_d_n4;
        locals.var_t1_dn5 = assign62320_e96766_d_n5;
        locals.var_t1_dn6 = assign62320_e96766_d_n6;
        locals.var_t1_dn7 = assign62320_e96766_d_n7;
        locals.var_t1_dn8 = assign62320_e96766_d_n8;
        locals.var_t1_dn9 = assign62320_e96766_d_n9;
        locals.var_t1_dn10 = assign62320_e96766_d_n10;
        locals.var_t1_dn13 = assign62320_e96766_d_n13;

        let (assign62330_e96779, assign62330_e96779_d_n0, assign62330_e96779_d_n2, assign62330_e96779_d_n4, assign62330_e96779_d_n5, assign62330_e96779_d_n6, assign62330_e96779_d_n7, assign62330_e96779_d_n8, assign62330_e96779_d_n9, assign62330_e96779_d_n10, assign62330_e96779_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62330_e96775: f64 = (2.0 * locals.var_t1);
        let assign62330_e96777: f64 = (assign62330_e96775 / 0.01);
        (assign62330_e96777, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn13) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign62330_e96779;
        locals.var_tmf1_dn0 = assign62330_e96779_d_n0;
        locals.var_tmf1_dn2 = assign62330_e96779_d_n2;
        locals.var_tmf1_dn4 = assign62330_e96779_d_n4;
        locals.var_tmf1_dn5 = assign62330_e96779_d_n5;
        locals.var_tmf1_dn6 = assign62330_e96779_d_n6;
        locals.var_tmf1_dn7 = assign62330_e96779_d_n7;
        locals.var_tmf1_dn8 = assign62330_e96779_d_n8;
        locals.var_tmf1_dn9 = assign62330_e96779_d_n9;
        locals.var_tmf1_dn10 = assign62330_e96779_d_n10;
        locals.var_tmf1_dn13 = assign62330_e96779_d_n13;

        let (assign62340_e96824, assign62340_e96824_d_n0, assign62340_e96824_d_n2, assign62340_e96824_d_n4, assign62340_e96824_d_n5, assign62340_e96824_d_n6, assign62340_e96824_d_n7, assign62340_e96824_d_n8, assign62340_e96824_d_n9, assign62340_e96824_d_n10, assign62340_e96824_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62340_e96790: f64 = (1.0 / 2.0);
        let assign62340_e96794: f64 = (1.0 / 6.0);
        let assign62340_e96798: f64 = (1.0 / 24.0);
        let assign62340_e96802: f64 = (1.0 / 120.0);
        let assign62340_e96806: f64 = (1.0 / 720.0);
        let assign62340_e96810: f64 = (1.0 / 5040.0);
        let assign62340_e96811: f64 = (locals.var_tmf1 * assign62340_e96810);
        let assign62340_e96812: f64 = (assign62340_e96806 + assign62340_e96811);
        let assign62340_e96813: f64 = (locals.var_tmf1 * assign62340_e96812);
        let assign62340_e96814: f64 = (assign62340_e96802 + assign62340_e96813);
        let assign62340_e96815: f64 = (locals.var_tmf1 * assign62340_e96814);
        let assign62340_e96816: f64 = (assign62340_e96798 + assign62340_e96815);
        let assign62340_e96817: f64 = (locals.var_tmf1 * assign62340_e96816);
        let assign62340_e96818: f64 = (assign62340_e96794 + assign62340_e96817);
        let assign62340_e96819: f64 = (locals.var_tmf1 * assign62340_e96818);
        let assign62340_e96820: f64 = (assign62340_e96790 + assign62340_e96819);
        let assign62340_e96821: f64 = (locals.var_tmf1 * assign62340_e96820);
        let assign62340_e96822: f64 = (1.0 + assign62340_e96821);
        (assign62340_e96822, ((locals.var_tmf1_dn0 * assign62340_e96820) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62340_e96818) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62340_e96816) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62340_e96814) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62340_e96812) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign62340_e96810))))))))))), ((locals.var_tmf1_dn2 * assign62340_e96820) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62340_e96818) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62340_e96816) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62340_e96814) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62340_e96812) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign62340_e96810))))))))))), ((locals.var_tmf1_dn4 * assign62340_e96820) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62340_e96818) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62340_e96816) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62340_e96814) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62340_e96812) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign62340_e96810))))))))))), ((locals.var_tmf1_dn5 * assign62340_e96820) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62340_e96818) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62340_e96816) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62340_e96814) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62340_e96812) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign62340_e96810))))))))))), ((locals.var_tmf1_dn6 * assign62340_e96820) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62340_e96818) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62340_e96816) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62340_e96814) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62340_e96812) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign62340_e96810))))))))))), ((locals.var_tmf1_dn7 * assign62340_e96820) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62340_e96818) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62340_e96816) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62340_e96814) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62340_e96812) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign62340_e96810))))))))))), ((locals.var_tmf1_dn8 * assign62340_e96820) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62340_e96818) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62340_e96816) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62340_e96814) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62340_e96812) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign62340_e96810))))))))))), ((locals.var_tmf1_dn9 * assign62340_e96820) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62340_e96818) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62340_e96816) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62340_e96814) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62340_e96812) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign62340_e96810))))))))))), ((locals.var_tmf1_dn10 * assign62340_e96820) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62340_e96818) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62340_e96816) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62340_e96814) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62340_e96812) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign62340_e96810))))))))))), ((locals.var_tmf1_dn13 * assign62340_e96820) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign62340_e96818) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign62340_e96816) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign62340_e96814) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign62340_e96812) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign62340_e96810))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign62340_e96824;
        locals.var_tmf2_dn0 = assign62340_e96824_d_n0;
        locals.var_tmf2_dn2 = assign62340_e96824_d_n2;
        locals.var_tmf2_dn4 = assign62340_e96824_d_n4;
        locals.var_tmf2_dn5 = assign62340_e96824_d_n5;
        locals.var_tmf2_dn6 = assign62340_e96824_d_n6;
        locals.var_tmf2_dn7 = assign62340_e96824_d_n7;
        locals.var_tmf2_dn8 = assign62340_e96824_d_n8;
        locals.var_tmf2_dn9 = assign62340_e96824_d_n9;
        locals.var_tmf2_dn10 = assign62340_e96824_d_n10;
        locals.var_tmf2_dn13 = assign62340_e96824_d_n13;

        let (assign62350_e96865, assign62350_e96865_d_n0, assign62350_e96865_d_n2, assign62350_e96865_d_n4, assign62350_e96865_d_n5, assign62350_e96865_d_n6, assign62350_e96865_d_n7, assign62350_e96865_d_n8, assign62350_e96865_d_n9, assign62350_e96865_d_n10, assign62350_e96865_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62350_e96833: f64 = (1.0 / 2.0);
        let assign62350_e96837: f64 = (1.0 / 3.0);
        let assign62350_e96841: f64 = (1.0 / 8.0);
        let assign62350_e96845: f64 = (1.0 / 30.0);
        let assign62350_e96849: f64 = (1.0 / 144.0);
        let assign62350_e96853: f64 = (1.0 / 840.0);
        let assign62350_e96854: f64 = (locals.var_tmf1 * assign62350_e96853);
        let assign62350_e96855: f64 = (assign62350_e96849 + assign62350_e96854);
        let assign62350_e96856: f64 = (locals.var_tmf1 * assign62350_e96855);
        let assign62350_e96857: f64 = (assign62350_e96845 + assign62350_e96856);
        let assign62350_e96858: f64 = (locals.var_tmf1 * assign62350_e96857);
        let assign62350_e96859: f64 = (assign62350_e96841 + assign62350_e96858);
        let assign62350_e96860: f64 = (locals.var_tmf1 * assign62350_e96859);
        let assign62350_e96861: f64 = (assign62350_e96837 + assign62350_e96860);
        let assign62350_e96862: f64 = (locals.var_tmf1 * assign62350_e96861);
        let assign62350_e96863: f64 = (assign62350_e96833 + assign62350_e96862);
        (assign62350_e96863, ((locals.var_tmf1_dn0 * assign62350_e96861) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62350_e96859) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62350_e96857) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62350_e96855) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign62350_e96853))))))))), ((locals.var_tmf1_dn2 * assign62350_e96861) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62350_e96859) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62350_e96857) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62350_e96855) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign62350_e96853))))))))), ((locals.var_tmf1_dn4 * assign62350_e96861) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62350_e96859) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62350_e96857) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62350_e96855) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign62350_e96853))))))))), ((locals.var_tmf1_dn5 * assign62350_e96861) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62350_e96859) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62350_e96857) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62350_e96855) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign62350_e96853))))))))), ((locals.var_tmf1_dn6 * assign62350_e96861) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62350_e96859) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62350_e96857) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62350_e96855) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign62350_e96853))))))))), ((locals.var_tmf1_dn7 * assign62350_e96861) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62350_e96859) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62350_e96857) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62350_e96855) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign62350_e96853))))))))), ((locals.var_tmf1_dn8 * assign62350_e96861) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62350_e96859) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62350_e96857) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62350_e96855) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign62350_e96853))))))))), ((locals.var_tmf1_dn9 * assign62350_e96861) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62350_e96859) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62350_e96857) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62350_e96855) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign62350_e96853))))))))), ((locals.var_tmf1_dn10 * assign62350_e96861) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62350_e96859) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62350_e96857) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62350_e96855) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign62350_e96853))))))))), ((locals.var_tmf1_dn13 * assign62350_e96861) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign62350_e96859) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign62350_e96857) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign62350_e96855) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign62350_e96853))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign62350_e96865;
        locals.var_tmf3_dn0 = assign62350_e96865_d_n0;
        locals.var_tmf3_dn2 = assign62350_e96865_d_n2;
        locals.var_tmf3_dn4 = assign62350_e96865_d_n4;
        locals.var_tmf3_dn5 = assign62350_e96865_d_n5;
        locals.var_tmf3_dn6 = assign62350_e96865_d_n6;
        locals.var_tmf3_dn7 = assign62350_e96865_d_n7;
        locals.var_tmf3_dn8 = assign62350_e96865_d_n8;
        locals.var_tmf3_dn9 = assign62350_e96865_d_n9;
        locals.var_tmf3_dn10 = assign62350_e96865_d_n10;
        locals.var_tmf3_dn13 = assign62350_e96865_d_n13;

        let (assign62360_e96876, assign62360_e96876_d_n0, assign62360_e96876_d_n2, assign62360_e96876_d_n4, assign62360_e96876_d_n5, assign62360_e96876_d_n6, assign62360_e96876_d_n7, assign62360_e96876_d_n8, assign62360_e96876_d_n9, assign62360_e96876_d_n10, assign62360_e96876_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62360_e96874: f64 = (0.01 / locals.var_tmf2);
        (assign62360_e96874, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign62360_e96876;
        locals.var_t6_dn0 = assign62360_e96876_d_n0;
        locals.var_t6_dn2 = assign62360_e96876_d_n2;
        locals.var_t6_dn4 = assign62360_e96876_d_n4;
        locals.var_t6_dn5 = assign62360_e96876_d_n5;
        locals.var_t6_dn6 = assign62360_e96876_d_n6;
        locals.var_t6_dn7 = assign62360_e96876_d_n7;
        locals.var_t6_dn8 = assign62360_e96876_d_n8;
        locals.var_t6_dn9 = assign62360_e96876_d_n9;
        locals.var_t6_dn10 = assign62360_e96876_d_n10;
        locals.var_t6_dn13 = assign62360_e96876_d_n13;

        let (assign62370_e96892, assign62370_e96892_d_n0, assign62370_e96892_d_n2, assign62370_e96892_d_n4, assign62370_e96892_d_n5, assign62370_e96892_d_n6, assign62370_e96892_d_n7, assign62370_e96892_d_n8, assign62370_e96892_d_n9, assign62370_e96892_d_n10, assign62370_e96892_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62370_e96884: f64 = (-2.0);
        let assign62370_e96886: f64 = (assign62370_e96884 * locals.var_tmf3);
        let assign62370_e96889: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign62370_e96890: f64 = (assign62370_e96886 / assign62370_e96889);
        (assign62370_e96890, ((((assign62370_e96884 * locals.var_tmf3_dn0) * assign62370_e96889) - (assign62370_e96886 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * locals.var_tmf3_dn2) * assign62370_e96889) - (assign62370_e96886 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * locals.var_tmf3_dn4) * assign62370_e96889) - (assign62370_e96886 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * locals.var_tmf3_dn5) * assign62370_e96889) - (assign62370_e96886 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * locals.var_tmf3_dn6) * assign62370_e96889) - (assign62370_e96886 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * locals.var_tmf3_dn7) * assign62370_e96889) - (assign62370_e96886 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * locals.var_tmf3_dn8) * assign62370_e96889) - (assign62370_e96886 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * locals.var_tmf3_dn9) * assign62370_e96889) - (assign62370_e96886 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * locals.var_tmf3_dn10) * assign62370_e96889) - (assign62370_e96886 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * locals.var_tmf3_dn13) * assign62370_e96889) - (assign62370_e96886 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign62370_e96889 * assign62370_e96889)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62370_e96892;
        locals.var_t2_dn0 = assign62370_e96892_d_n0;
        locals.var_t2_dn2 = assign62370_e96892_d_n2;
        locals.var_t2_dn4 = assign62370_e96892_d_n4;
        locals.var_t2_dn5 = assign62370_e96892_d_n5;
        locals.var_t2_dn6 = assign62370_e96892_d_n6;
        locals.var_t2_dn7 = assign62370_e96892_d_n7;
        locals.var_t2_dn8 = assign62370_e96892_d_n8;
        locals.var_t2_dn9 = assign62370_e96892_d_n9;
        locals.var_t2_dn10 = assign62370_e96892_d_n10;
        locals.var_t2_dn13 = assign62370_e96892_d_n13;

        let (assign62380_e96903, assign62380_e96903_d_n0, assign62380_e96903_d_n2, assign62380_e96903_d_n4, assign62380_e96903_d_n5, assign62380_e96903_d_n6, assign62380_e96903_d_n7, assign62380_e96903_d_n8, assign62380_e96903_d_n9, assign62380_e96903_d_n10, assign62380_e96903_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62380_e96901: f64 = (locals.var_t2 * 0.5);
        (assign62380_e96901, (locals.var_t2_dn0 * 0.5), (locals.var_t2_dn2 * 0.5), (locals.var_t2_dn4 * 0.5), (locals.var_t2_dn5 * 0.5), (locals.var_t2_dn6 * 0.5), (locals.var_t2_dn7 * 0.5), (locals.var_t2_dn8 * 0.5), (locals.var_t2_dn9 * 0.5), (locals.var_t2_dn10 * 0.5), (locals.var_t2_dn13 * 0.5),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62380_e96903;
        locals.var_t2_dn0 = assign62380_e96903_d_n0;
        locals.var_t2_dn2 = assign62380_e96903_d_n2;
        locals.var_t2_dn4 = assign62380_e96903_d_n4;
        locals.var_t2_dn5 = assign62380_e96903_d_n5;
        locals.var_t2_dn6 = assign62380_e96903_d_n6;
        locals.var_t2_dn7 = assign62380_e96903_d_n7;
        locals.var_t2_dn8 = assign62380_e96903_d_n8;
        locals.var_t2_dn9 = assign62380_e96903_d_n9;
        locals.var_t2_dn10 = assign62380_e96903_d_n10;
        locals.var_t2_dn13 = assign62380_e96903_d_n13;

        let (assign62390_e96916, assign62390_e96916_d_n0, assign62390_e96916_d_n2, assign62390_e96916_d_n4, assign62390_e96916_d_n5, assign62390_e96916_d_n6, assign62390_e96916_d_n7, assign62390_e96916_d_n8, assign62390_e96916_d_n9, assign62390_e96916_d_n10, assign62390_e96916_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62390_e96913: f64 = (locals.var_ps0 + locals.var_t6);
        let assign62390_e96914: f64 = (1.1 - assign62390_e96913);
        (assign62390_e96914, (-(locals.var_ps0_dn0 + locals.var_t6_dn0)), (-(locals.var_ps0_dn2 + locals.var_t6_dn2)), (-(locals.var_ps0_dn4 + locals.var_t6_dn4)), (-(locals.var_ps0_dn5 + locals.var_t6_dn5)), (-(locals.var_ps0_dn6 + locals.var_t6_dn6)), (-(locals.var_ps0_dn7 + locals.var_t6_dn7)), (-(locals.var_ps0_dn8 + locals.var_t6_dn8)), (-(locals.var_ps0_dn9 + locals.var_t6_dn9)), (-(locals.var_ps0_dn10 + locals.var_t6_dn10)), (-(locals.var_ps0_dn13 + locals.var_t6_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign62390_e96916;
        locals.var_t1_dn0 = assign62390_e96916_d_n0;
        locals.var_t1_dn2 = assign62390_e96916_d_n2;
        locals.var_t1_dn4 = assign62390_e96916_d_n4;
        locals.var_t1_dn5 = assign62390_e96916_d_n5;
        locals.var_t1_dn6 = assign62390_e96916_d_n6;
        locals.var_t1_dn7 = assign62390_e96916_d_n7;
        locals.var_t1_dn8 = assign62390_e96916_d_n8;
        locals.var_t1_dn9 = assign62390_e96916_d_n9;
        locals.var_t1_dn10 = assign62390_e96916_d_n10;
        locals.var_t1_dn13 = assign62390_e96916_d_n13;

        let (assign62400_e96934, assign62400_e96934_d_n0, assign62400_e96934_d_n2, assign62400_e96934_d_n4, assign62400_e96934_d_n5, assign62400_e96934_d_n6, assign62400_e96934_d_n7, assign62400_e96934_d_n8, assign62400_e96934_d_n9, assign62400_e96934_d_n10, assign62400_e96934_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62400_e96925: f64 = (locals.var_t1 * locals.var_t1);
        let assign62400_e96928: f64 = (4.0 * 0.05);
        let assign62400_e96930: f64 = (assign62400_e96928 * 0.05);
        let assign62400_e96931: f64 = (assign62400_e96925 + assign62400_e96930);
        let assign62400_e96932: f64 = (assign62400_e96931).sqrt();
        (assign62400_e96932, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign62400_e96932)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign62400_e96932)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign62400_e96932)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign62400_e96932)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign62400_e96932)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign62400_e96932)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign62400_e96932)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign62400_e96932)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign62400_e96932)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign62400_e96932)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign62400_e96934;
        locals.var_tmf2_dn0 = assign62400_e96934_d_n0;
        locals.var_tmf2_dn2 = assign62400_e96934_d_n2;
        locals.var_tmf2_dn4 = assign62400_e96934_d_n4;
        locals.var_tmf2_dn5 = assign62400_e96934_d_n5;
        locals.var_tmf2_dn6 = assign62400_e96934_d_n6;
        locals.var_tmf2_dn7 = assign62400_e96934_d_n7;
        locals.var_tmf2_dn8 = assign62400_e96934_d_n8;
        locals.var_tmf2_dn9 = assign62400_e96934_d_n9;
        locals.var_tmf2_dn10 = assign62400_e96934_d_n10;
        locals.var_tmf2_dn13 = assign62400_e96934_d_n13;

        let (assign62410_e96949, assign62410_e96949_d_n0, assign62410_e96949_d_n2, assign62410_e96949_d_n4, assign62410_e96949_d_n5, assign62410_e96949_d_n6, assign62410_e96949_d_n7, assign62410_e96949_d_n8, assign62410_e96949_d_n9, assign62410_e96949_d_n10, assign62410_e96949_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62410_e96945: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign62410_e96946: f64 = (1.0 + assign62410_e96945);
        let assign62410_e96947: f64 = (0.5 * assign62410_e96946);
        (assign62410_e96947, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign62410_e96949;
        locals.var_t0_dn0 = assign62410_e96949_d_n0;
        locals.var_t0_dn2 = assign62410_e96949_d_n2;
        locals.var_t0_dn4 = assign62410_e96949_d_n4;
        locals.var_t0_dn5 = assign62410_e96949_d_n5;
        locals.var_t0_dn6 = assign62410_e96949_d_n6;
        locals.var_t0_dn7 = assign62410_e96949_d_n7;
        locals.var_t0_dn8 = assign62410_e96949_d_n8;
        locals.var_t0_dn9 = assign62410_e96949_d_n9;
        locals.var_t0_dn10 = assign62410_e96949_d_n10;
        locals.var_t0_dn13 = assign62410_e96949_d_n13;

        let (assign62420_e96962, assign62420_e96962_d_n0, assign62420_e96962_d_n2, assign62420_e96962_d_n4, assign62420_e96962_d_n5, assign62420_e96962_d_n6, assign62420_e96962_d_n7, assign62420_e96962_d_n8, assign62420_e96962_d_n9, assign62420_e96962_d_n10, assign62420_e96962_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62420_e96959: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign62420_e96960: f64 = (0.5 * assign62420_e96959);
        (assign62420_e96960, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62420_e96962;
        locals.var_t2_dn0 = assign62420_e96962_d_n0;
        locals.var_t2_dn2 = assign62420_e96962_d_n2;
        locals.var_t2_dn4 = assign62420_e96962_d_n4;
        locals.var_t2_dn5 = assign62420_e96962_d_n5;
        locals.var_t2_dn6 = assign62420_e96962_d_n6;
        locals.var_t2_dn7 = assign62420_e96962_d_n7;
        locals.var_t2_dn8 = assign62420_e96962_d_n8;
        locals.var_t2_dn9 = assign62420_e96962_d_n9;
        locals.var_t2_dn10 = assign62420_e96962_d_n10;
        locals.var_t2_dn13 = assign62420_e96962_d_n13;

        let assign62430_e96965: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1496 = assign62430_e96965;

        let (assign62440_e96976, assign62440_e96976_d_n0, assign62440_e96976_d_n2, assign62440_e96976_d_n4, assign62440_e96976_d_n5, assign62440_e96976_d_n6, assign62440_e96976_d_n7, assign62440_e96976_d_n8, assign62440_e96976_d_n9, assign62440_e96976_d_n10, assign62440_e96976_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1496 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62440_e96976;
        locals.var_t2_dn0 = assign62440_e96976_d_n0;
        locals.var_t2_dn2 = assign62440_e96976_d_n2;
        locals.var_t2_dn4 = assign62440_e96976_d_n4;
        locals.var_t2_dn5 = assign62440_e96976_d_n5;
        locals.var_t2_dn6 = assign62440_e96976_d_n6;
        locals.var_t2_dn7 = assign62440_e96976_d_n7;
        locals.var_t2_dn8 = assign62440_e96976_d_n8;
        locals.var_t2_dn9 = assign62440_e96976_d_n9;
        locals.var_t2_dn10 = assign62440_e96976_d_n10;
        locals.var_t2_dn13 = assign62440_e96976_d_n13;

    }

    pub(super) fn stamp_transient_block_211(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62450_e96987, assign62450_e96987_d_n0, assign62450_e96987_d_n2, assign62450_e96987_d_n4, assign62450_e96987_d_n5, assign62450_e96987_d_n6, assign62450_e96987_d_n7, assign62450_e96987_d_n8, assign62450_e96987_d_n9, assign62450_e96987_d_n10, assign62450_e96987_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1496 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign62450_e96987;
        locals.var_t0_dn0 = assign62450_e96987_d_n0;
        locals.var_t0_dn2 = assign62450_e96987_d_n2;
        locals.var_t0_dn4 = assign62450_e96987_d_n4;
        locals.var_t0_dn5 = assign62450_e96987_d_n5;
        locals.var_t0_dn6 = assign62450_e96987_d_n6;
        locals.var_t0_dn7 = assign62450_e96987_d_n7;
        locals.var_t0_dn8 = assign62450_e96987_d_n8;
        locals.var_t0_dn9 = assign62450_e96987_d_n9;
        locals.var_t0_dn10 = assign62450_e96987_d_n10;
        locals.var_t0_dn13 = assign62450_e96987_d_n13;

        let (assign62460_e96998, assign62460_e96998_d_n0, assign62460_e96998_d_n2, assign62460_e96998_d_n4, assign62460_e96998_d_n5, assign62460_e96998_d_n6, assign62460_e96998_d_n7, assign62460_e96998_d_n8, assign62460_e96998_d_n9, assign62460_e96998_d_n10, assign62460_e96998_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62460_e96996: f64 = (locals.var_t2 + 1e-25);
        (assign62460_e96996, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62460_e96998;
        locals.var_t2_dn0 = assign62460_e96998_d_n0;
        locals.var_t2_dn2 = assign62460_e96998_d_n2;
        locals.var_t2_dn4 = assign62460_e96998_d_n4;
        locals.var_t2_dn5 = assign62460_e96998_d_n5;
        locals.var_t2_dn6 = assign62460_e96998_d_n6;
        locals.var_t2_dn7 = assign62460_e96998_d_n7;
        locals.var_t2_dn8 = assign62460_e96998_d_n8;
        locals.var_t2_dn9 = assign62460_e96998_d_n9;
        locals.var_t2_dn10 = assign62460_e96998_d_n10;
        locals.var_t2_dn13 = assign62460_e96998_d_n13;

        let (assign62470_e97009, assign62470_e97009_d_n0, assign62470_e97009_d_n2, assign62470_e97009_d_n4, assign62470_e97009_d_n5, assign62470_e97009_d_n6, assign62470_e97009_d_n7, assign62470_e97009_d_n8, assign62470_e97009_d_n9, assign62470_e97009_d_n10, assign62470_e97009_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62470_e97007: f64 = (locals.var_beta * locals.var_ptl0);
        (assign62470_e97007, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn13 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign62470_e97009;
        locals.var_t0_dn0 = assign62470_e97009_d_n0;
        locals.var_t0_dn2 = assign62470_e97009_d_n2;
        locals.var_t0_dn4 = assign62470_e97009_d_n4;
        locals.var_t0_dn5 = assign62470_e97009_d_n5;
        locals.var_t0_dn6 = assign62470_e97009_d_n6;
        locals.var_t0_dn7 = assign62470_e97009_d_n7;
        locals.var_t0_dn8 = assign62470_e97009_d_n8;
        locals.var_t0_dn9 = assign62470_e97009_d_n9;
        locals.var_t0_dn10 = assign62470_e97009_d_n10;
        locals.var_t0_dn13 = assign62470_e97009_d_n13;

        let (assign62480_e97020, assign62480_e97020_d_n0, assign62480_e97020_d_n2, assign62480_e97020_d_n4, assign62480_e97020_d_n5, assign62480_e97020_d_n6, assign62480_e97020_d_n7, assign62480_e97020_d_n8, assign62480_e97020_d_n9, assign62480_e97020_d_n10, assign62480_e97020_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62480_e97018: f64 = (locals.var_cox * locals.var_t0);
        (assign62480_e97018, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn13 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign62480_e97020;
        locals.var_t3_dn0 = assign62480_e97020_d_n0;
        locals.var_t3_dn2 = assign62480_e97020_d_n2;
        locals.var_t3_dn4 = assign62480_e97020_d_n4;
        locals.var_t3_dn5 = assign62480_e97020_d_n5;
        locals.var_t3_dn6 = assign62480_e97020_d_n6;
        locals.var_t3_dn7 = assign62480_e97020_d_n7;
        locals.var_t3_dn8 = assign62480_e97020_d_n8;
        locals.var_t3_dn9 = assign62480_e97020_d_n9;
        locals.var_t3_dn10 = assign62480_e97020_d_n10;
        locals.var_t3_dn13 = assign62480_e97020_d_n13;

        let (assign62490_e97031, assign62490_e97031_d_n0, assign62490_e97031_d_n2, assign62490_e97031_d_n4, assign62490_e97031_d_n5, assign62490_e97031_d_n6, assign62490_e97031_d_n7, assign62490_e97031_d_n8, assign62490_e97031_d_n9, assign62490_e97031_d_n10, assign62490_e97031_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62490_e97029: f64 = (locals.var_t2).powf(p.p284);
        (assign62490_e97029, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign62490_e97029 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign62490_e97029 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign62490_e97029 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign62490_e97029 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign62490_e97029 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign62490_e97029 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign62490_e97029 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign62490_e97029 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign62490_e97029 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn13)) } } else { (assign62490_e97029 * (p.p284 * (locals.var_t2_dn13 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign62490_e97031;
        locals.var_t0_dn0 = assign62490_e97031_d_n0;
        locals.var_t0_dn2 = assign62490_e97031_d_n2;
        locals.var_t0_dn4 = assign62490_e97031_d_n4;
        locals.var_t0_dn5 = assign62490_e97031_d_n5;
        locals.var_t0_dn6 = assign62490_e97031_d_n6;
        locals.var_t0_dn7 = assign62490_e97031_d_n7;
        locals.var_t0_dn8 = assign62490_e97031_d_n8;
        locals.var_t0_dn9 = assign62490_e97031_d_n9;
        locals.var_t0_dn10 = assign62490_e97031_d_n10;
        locals.var_t0_dn13 = assign62490_e97031_d_n13;

        let (assign62500_e97042, assign62500_e97042_d_n0, assign62500_e97042_d_n2, assign62500_e97042_d_n4, assign62500_e97042_d_n5, assign62500_e97042_d_n6, assign62500_e97042_d_n7, assign62500_e97042_d_n8, assign62500_e97042_d_n9, assign62500_e97042_d_n10, assign62500_e97042_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62500_e97040: f64 = (locals.var_t3 * locals.var_t0);
        (assign62500_e97040, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn13 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign62500_e97042;
        locals.var_t9_dn0 = assign62500_e97042_d_n0;
        locals.var_t9_dn2 = assign62500_e97042_d_n2;
        locals.var_t9_dn4 = assign62500_e97042_d_n4;
        locals.var_t9_dn5 = assign62500_e97042_d_n5;
        locals.var_t9_dn6 = assign62500_e97042_d_n6;
        locals.var_t9_dn7 = assign62500_e97042_d_n7;
        locals.var_t9_dn8 = assign62500_e97042_d_n8;
        locals.var_t9_dn9 = assign62500_e97042_d_n9;
        locals.var_t9_dn10 = assign62500_e97042_d_n10;
        locals.var_t9_dn13 = assign62500_e97042_d_n13;

        let (assign62510_e97055, assign62510_e97055_d_n0, assign62510_e97055_d_n2, assign62510_e97055_d_n4, assign62510_e97055_d_n5, assign62510_e97055_d_n6, assign62510_e97055_d_n7, assign62510_e97055_d_n8, assign62510_e97055_d_n9, assign62510_e97055_d_n10, assign62510_e97055_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62510_e97052: f64 = (locals.var_vdsz__blk439 * p.p285);
        let assign62510_e97053: f64 = (1.0 + assign62510_e97052);
        (assign62510_e97053, (locals.var_vdsz__blk439_dn0 * p.p285), (locals.var_vdsz__blk439_dn2 * p.p285), (locals.var_vdsz__blk439_dn4 * p.p285), (locals.var_vdsz__blk439_dn5 * p.p285), (locals.var_vdsz__blk439_dn6 * p.p285), (locals.var_vdsz__blk439_dn7 * p.p285), (locals.var_vdsz__blk439_dn8 * p.p285), (locals.var_vdsz__blk439_dn9 * p.p285), (locals.var_vdsz__blk439_dn10 * p.p285), (locals.var_vdsz__blk439_dn13 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign62510_e97055;
        locals.var_t4_dn0 = assign62510_e97055_d_n0;
        locals.var_t4_dn2 = assign62510_e97055_d_n2;
        locals.var_t4_dn4 = assign62510_e97055_d_n4;
        locals.var_t4_dn5 = assign62510_e97055_d_n5;
        locals.var_t4_dn6 = assign62510_e97055_d_n6;
        locals.var_t4_dn7 = assign62510_e97055_d_n7;
        locals.var_t4_dn8 = assign62510_e97055_d_n8;
        locals.var_t4_dn9 = assign62510_e97055_d_n9;
        locals.var_t4_dn10 = assign62510_e97055_d_n10;
        locals.var_t4_dn13 = assign62510_e97055_d_n13;

        let (assign62520_e97064, assign62520_e97064_d_n0, assign62520_e97064_d_n2, assign62520_e97064_d_n4, assign62520_e97064_d_n5, assign62520_e97064_d_n6, assign62520_e97064_d_n7, assign62520_e97064_d_n8, assign62520_e97064_d_n9, assign62520_e97064_d_n10, assign62520_e97064_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign62520_e97064;
        locals.var_t0_dn0 = assign62520_e97064_d_n0;
        locals.var_t0_dn2 = assign62520_e97064_d_n2;
        locals.var_t0_dn4 = assign62520_e97064_d_n4;
        locals.var_t0_dn5 = assign62520_e97064_d_n5;
        locals.var_t0_dn6 = assign62520_e97064_d_n6;
        locals.var_t0_dn7 = assign62520_e97064_d_n7;
        locals.var_t0_dn8 = assign62520_e97064_d_n8;
        locals.var_t0_dn9 = assign62520_e97064_d_n9;
        locals.var_t0_dn10 = assign62520_e97064_d_n10;
        locals.var_t0_dn13 = assign62520_e97064_d_n13;

        let (assign62530_e97077, assign62530_e97077_d_n0, assign62530_e97077_d_n2, assign62530_e97077_d_n4, assign62530_e97077_d_n5, assign62530_e97077_d_n6, assign62530_e97077_d_n7, assign62530_e97077_d_n8, assign62530_e97077_d_n9, assign62530_e97077_d_n10, assign62530_e97077_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62530_e97073: f64 = (locals.var_ps0 + locals.var_t6);
        let assign62530_e97075: f64 = (assign62530_e97073 - locals.var_vbsz__blk438);
        (assign62530_e97075, ((locals.var_ps0_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk438_dn0), ((locals.var_ps0_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk438_dn2), ((locals.var_ps0_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk438_dn4), ((locals.var_ps0_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk438_dn5), ((locals.var_ps0_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk438_dn6), ((locals.var_ps0_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk438_dn7), ((locals.var_ps0_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk438_dn8), ((locals.var_ps0_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk438_dn9), ((locals.var_ps0_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk438_dn10), ((locals.var_ps0_dn13 + locals.var_t6_dn13) - locals.var_vbsz__blk438_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign62530_e97077;
        locals.var_t5_dn0 = assign62530_e97077_d_n0;
        locals.var_t5_dn2 = assign62530_e97077_d_n2;
        locals.var_t5_dn4 = assign62530_e97077_d_n4;
        locals.var_t5_dn5 = assign62530_e97077_d_n5;
        locals.var_t5_dn6 = assign62530_e97077_d_n6;
        locals.var_t5_dn7 = assign62530_e97077_d_n7;
        locals.var_t5_dn8 = assign62530_e97077_d_n8;
        locals.var_t5_dn9 = assign62530_e97077_d_n9;
        locals.var_t5_dn10 = assign62530_e97077_d_n10;
        locals.var_t5_dn13 = assign62530_e97077_d_n13;

        let (assign62540_e97092, assign62540_e97092_d_n0, assign62540_e97092_d_n2, assign62540_e97092_d_n4, assign62540_e97092_d_n5, assign62540_e97092_d_n6, assign62540_e97092_d_n7, assign62540_e97092_d_n8, assign62540_e97092_d_n9, assign62540_e97092_d_n10, assign62540_e97092_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62540_e97087: f64 = (locals.var_vdsz__blk439 * locals.var_t0);
        let assign62540_e97089: f64 = (assign62540_e97087 * locals.var_t5);
        let assign62540_e97090: f64 = (locals.var_t4 + assign62540_e97089);
        (assign62540_e97090, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk439_dn0 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn0)) * locals.var_t5) + (assign62540_e97087 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk439_dn2 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn2)) * locals.var_t5) + (assign62540_e97087 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk439_dn4 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn4)) * locals.var_t5) + (assign62540_e97087 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk439_dn5 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn5)) * locals.var_t5) + (assign62540_e97087 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk439_dn6 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn6)) * locals.var_t5) + (assign62540_e97087 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk439_dn7 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn7)) * locals.var_t5) + (assign62540_e97087 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk439_dn8 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn8)) * locals.var_t5) + (assign62540_e97087 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk439_dn9 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn9)) * locals.var_t5) + (assign62540_e97087 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk439_dn10 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn10)) * locals.var_t5) + (assign62540_e97087 * locals.var_t5_dn10))), (locals.var_t4_dn13 + ((((locals.var_vdsz__blk439_dn13 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn13)) * locals.var_t5) + (assign62540_e97087 * locals.var_t5_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign62540_e97092;
        locals.var_t4_dn0 = assign62540_e97092_d_n0;
        locals.var_t4_dn2 = assign62540_e97092_d_n2;
        locals.var_t4_dn4 = assign62540_e97092_d_n4;
        locals.var_t4_dn5 = assign62540_e97092_d_n5;
        locals.var_t4_dn6 = assign62540_e97092_d_n6;
        locals.var_t4_dn7 = assign62540_e97092_d_n7;
        locals.var_t4_dn8 = assign62540_e97092_d_n8;
        locals.var_t4_dn9 = assign62540_e97092_d_n9;
        locals.var_t4_dn10 = assign62540_e97092_d_n10;
        locals.var_t4_dn13 = assign62540_e97092_d_n13;

        let (assign62550_e97103, assign62550_e97103_d_n0, assign62550_e97103_d_n2, assign62550_e97103_d_n4, assign62550_e97103_d_n5, assign62550_e97103_d_n6, assign62550_e97103_d_n7, assign62550_e97103_d_n8, assign62550_e97103_d_n9, assign62550_e97103_d_n10, assign62550_e97103_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62550_e97101: f64 = (locals.var_t9 * locals.var_t4);
        (assign62550_e97101, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn7 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn7)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn9 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn9)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn13 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign62550_e97103;
        locals.var_t6_dn0 = assign62550_e97103_d_n0;
        locals.var_t6_dn2 = assign62550_e97103_d_n2;
        locals.var_t6_dn4 = assign62550_e97103_d_n4;
        locals.var_t6_dn5 = assign62550_e97103_d_n5;
        locals.var_t6_dn6 = assign62550_e97103_d_n6;
        locals.var_t6_dn7 = assign62550_e97103_d_n7;
        locals.var_t6_dn8 = assign62550_e97103_d_n8;
        locals.var_t6_dn9 = assign62550_e97103_d_n9;
        locals.var_t6_dn10 = assign62550_e97103_d_n10;
        locals.var_t6_dn13 = assign62550_e97103_d_n13;

        let (assign62560_e97112, assign62560_e97112_d_n0, assign62560_e97112_d_n2, assign62560_e97112_d_n4, assign62560_e97112_d_n5, assign62560_e97112_d_n6, assign62560_e97112_d_n7, assign62560_e97112_d_n8, assign62560_e97112_d_n9, assign62560_e97112_d_n10, assign62560_e97112_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign62560_e97112;
        locals.var_t9_dn0 = assign62560_e97112_d_n0;
        locals.var_t9_dn2 = assign62560_e97112_d_n2;
        locals.var_t9_dn4 = assign62560_e97112_d_n4;
        locals.var_t9_dn5 = assign62560_e97112_d_n5;
        locals.var_t9_dn6 = assign62560_e97112_d_n6;
        locals.var_t9_dn7 = assign62560_e97112_d_n7;
        locals.var_t9_dn8 = assign62560_e97112_d_n8;
        locals.var_t9_dn9 = assign62560_e97112_d_n9;
        locals.var_t9_dn10 = assign62560_e97112_d_n10;
        locals.var_t9_dn13 = assign62560_e97112_d_n13;

        let (assign62570_e97122, assign62570_e97122_d_n0, assign62570_e97122_d_n2, assign62570_e97122_d_n4, assign62570_e97122_d_n5, assign62570_e97122_d_n6, assign62570_e97122_d_n7, assign62570_e97122_d_n8, assign62570_e97122_d_n9, assign62570_e97122_d_n10, assign62570_e97122_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1495 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign62570_e97122;
        locals.var_t9_dn0 = assign62570_e97122_d_n0;
        locals.var_t9_dn2 = assign62570_e97122_d_n2;
        locals.var_t9_dn4 = assign62570_e97122_d_n4;
        locals.var_t9_dn5 = assign62570_e97122_d_n5;
        locals.var_t9_dn6 = assign62570_e97122_d_n6;
        locals.var_t9_dn7 = assign62570_e97122_d_n7;
        locals.var_t9_dn8 = assign62570_e97122_d_n8;
        locals.var_t9_dn9 = assign62570_e97122_d_n9;
        locals.var_t9_dn10 = assign62570_e97122_d_n10;
        locals.var_t9_dn13 = assign62570_e97122_d_n13;

        let assign62580_e97125: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1497 = assign62580_e97125;

        let (assign62590_e97136, assign62590_e97136_d_n0, assign62590_e97136_d_n2, assign62590_e97136_d_n4, assign62590_e97136_d_n5, assign62590_e97136_d_n6, assign62590_e97136_d_n7, assign62590_e97136_d_n8, assign62590_e97136_d_n9, assign62590_e97136_d_n10, assign62590_e97136_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62590_e97134: f64 = (locals.var_beta * locals.var_gdl0);
        (assign62590_e97134, (locals.var_beta_dn0 * locals.var_gdl0), (locals.var_beta_dn2 * locals.var_gdl0), (locals.var_beta_dn4 * locals.var_gdl0), (locals.var_beta_dn5 * locals.var_gdl0), (locals.var_beta_dn6 * locals.var_gdl0), (locals.var_beta_dn7 * locals.var_gdl0), (locals.var_beta_dn8 * locals.var_gdl0), (locals.var_beta_dn9 * locals.var_gdl0), (locals.var_beta_dn10 * locals.var_gdl0), (locals.var_beta_dn13 * locals.var_gdl0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign62590_e97136;
        locals.var_t1_dn0 = assign62590_e97136_d_n0;
        locals.var_t1_dn2 = assign62590_e97136_d_n2;
        locals.var_t1_dn4 = assign62590_e97136_d_n4;
        locals.var_t1_dn5 = assign62590_e97136_d_n5;
        locals.var_t1_dn6 = assign62590_e97136_d_n6;
        locals.var_t1_dn7 = assign62590_e97136_d_n7;
        locals.var_t1_dn8 = assign62590_e97136_d_n8;
        locals.var_t1_dn9 = assign62590_e97136_d_n9;
        locals.var_t1_dn10 = assign62590_e97136_d_n10;
        locals.var_t1_dn13 = assign62590_e97136_d_n13;

        let (assign62600_e97147, assign62600_e97147_d_n0, assign62600_e97147_d_n2, assign62600_e97147_d_n4, assign62600_e97147_d_n5, assign62600_e97147_d_n6, assign62600_e97147_d_n7, assign62600_e97147_d_n8, assign62600_e97147_d_n9, assign62600_e97147_d_n10, assign62600_e97147_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62600_e97145: f64 = (locals.var_cox * locals.var_t1);
        (assign62600_e97145, ((locals.var_cox_dn0 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn0)), ((locals.var_cox_dn2 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn2)), ((locals.var_cox_dn4 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn4)), ((locals.var_cox_dn5 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn5)), ((locals.var_cox_dn6 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn6)), ((locals.var_cox_dn7 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn7)), ((locals.var_cox_dn8 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn8)), ((locals.var_cox_dn9 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn9)), ((locals.var_cox_dn10 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn10)), ((locals.var_cox_dn13 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62600_e97147;
        locals.var_t2_dn0 = assign62600_e97147_d_n0;
        locals.var_t2_dn2 = assign62600_e97147_d_n2;
        locals.var_t2_dn4 = assign62600_e97147_d_n4;
        locals.var_t2_dn5 = assign62600_e97147_d_n5;
        locals.var_t2_dn6 = assign62600_e97147_d_n6;
        locals.var_t2_dn7 = assign62600_e97147_d_n7;
        locals.var_t2_dn8 = assign62600_e97147_d_n8;
        locals.var_t2_dn9 = assign62600_e97147_d_n9;
        locals.var_t2_dn10 = assign62600_e97147_d_n10;
        locals.var_t2_dn13 = assign62600_e97147_d_n13;

        let (assign62610_e97158, assign62610_e97158_d_n0, assign62610_e97158_d_n2, assign62610_e97158_d_n4, assign62610_e97158_d_n5, assign62610_e97158_d_n6, assign62610_e97158_d_n7, assign62610_e97158_d_n8, assign62610_e97158_d_n9, assign62610_e97158_d_n10, assign62610_e97158_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62610_e97156: f64 = (locals.var_t2 * locals.var_vdsz__blk439);
        (assign62610_e97156, ((locals.var_t2_dn0 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn0)), ((locals.var_t2_dn2 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn2)), ((locals.var_t2_dn4 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn4)), ((locals.var_t2_dn5 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn5)), ((locals.var_t2_dn6 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn6)), ((locals.var_t2_dn7 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn7)), ((locals.var_t2_dn8 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn8)), ((locals.var_t2_dn9 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn9)), ((locals.var_t2_dn10 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn10)), ((locals.var_t2_dn13 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign62610_e97158;
        locals.var_t8_dn0 = assign62610_e97158_d_n0;
        locals.var_t8_dn2 = assign62610_e97158_d_n2;
        locals.var_t8_dn4 = assign62610_e97158_d_n4;
        locals.var_t8_dn5 = assign62610_e97158_d_n5;
        locals.var_t8_dn6 = assign62610_e97158_d_n6;
        locals.var_t8_dn7 = assign62610_e97158_d_n7;
        locals.var_t8_dn8 = assign62610_e97158_d_n8;
        locals.var_t8_dn9 = assign62610_e97158_d_n9;
        locals.var_t8_dn10 = assign62610_e97158_d_n10;
        locals.var_t8_dn13 = assign62610_e97158_d_n13;

        let (assign62620_e97168, assign62620_e97168_d_n0, assign62620_e97168_d_n2, assign62620_e97168_d_n4, assign62620_e97168_d_n5, assign62620_e97168_d_n6, assign62620_e97168_d_n7, assign62620_e97168_d_n8, assign62620_e97168_d_n9, assign62620_e97168_d_n10, assign62620_e97168_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign62620_e97168;
        locals.var_t8_dn0 = assign62620_e97168_d_n0;
        locals.var_t8_dn2 = assign62620_e97168_d_n2;
        locals.var_t8_dn4 = assign62620_e97168_d_n4;
        locals.var_t8_dn5 = assign62620_e97168_d_n5;
        locals.var_t8_dn6 = assign62620_e97168_d_n6;
        locals.var_t8_dn7 = assign62620_e97168_d_n7;
        locals.var_t8_dn8 = assign62620_e97168_d_n8;
        locals.var_t8_dn9 = assign62620_e97168_d_n9;
        locals.var_t8_dn10 = assign62620_e97168_d_n10;
        locals.var_t8_dn13 = assign62620_e97168_d_n13;

        let assign62630_e97171: f64 = (locals.var_t9 + locals.var_t8);
        let assign62630_e97173: f64 = if assign62630_e97171 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1498 = assign62630_e97173;

        let (assign62640_e97186, assign62640_e97186_d_n0, assign62640_e97186_d_n2, assign62640_e97186_d_n4, assign62640_e97186_d_n5, assign62640_e97186_d_n6, assign62640_e97186_d_n7, assign62640_e97186_d_n8, assign62640_e97186_d_n9, assign62640_e97186_d_n10, assign62640_e97186_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign62640_e97183: f64 = (locals.var_t9 + locals.var_t8);
        let assign62640_e97184: f64 = (locals.var_pds * assign62640_e97183);
        (assign62640_e97184, ((locals.var_pds_dn0 * assign62640_e97183) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign62640_e97183) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign62640_e97183) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign62640_e97183) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign62640_e97183) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn7 * assign62640_e97183) + (locals.var_pds * (locals.var_t9_dn7 + locals.var_t8_dn7))), ((locals.var_pds_dn8 * assign62640_e97183) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn9 * assign62640_e97183) + (locals.var_pds * (locals.var_t9_dn9 + locals.var_t8_dn9))), ((locals.var_pds_dn10 * assign62640_e97183) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn13 * assign62640_e97183) + (locals.var_pds * (locals.var_t9_dn13 + locals.var_t8_dn13))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn4, locals.var_idd1_dn5, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn8, locals.var_idd1_dn9, locals.var_idd1_dn10, locals.var_idd1_dn13,)
    }
};
        locals.var_idd1 = assign62640_e97186;
        locals.var_idd1_dn0 = assign62640_e97186_d_n0;
        locals.var_idd1_dn2 = assign62640_e97186_d_n2;
        locals.var_idd1_dn4 = assign62640_e97186_d_n4;
        locals.var_idd1_dn5 = assign62640_e97186_d_n5;
        locals.var_idd1_dn6 = assign62640_e97186_d_n6;
        locals.var_idd1_dn7 = assign62640_e97186_d_n7;
        locals.var_idd1_dn8 = assign62640_e97186_d_n8;
        locals.var_idd1_dn9 = assign62640_e97186_d_n9;
        locals.var_idd1_dn10 = assign62640_e97186_d_n10;
        locals.var_idd1_dn13 = assign62640_e97186_d_n13;

        let (assign62650_e97199, assign62650_e97199_d_n0, assign62650_e97199_d_n2, assign62650_e97199_d_n4, assign62650_e97199_d_n5, assign62650_e97199_d_n6, assign62650_e97199_d_n7, assign62650_e97199_d_n8, assign62650_e97199_d_n9, assign62650_e97199_d_n10, assign62650_e97199_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign62650_e97195: f64 = (locals.var_betawl * locals.var_idd1);
        let assign62650_e97197: f64 = (assign62650_e97195 * locals.var_mu);
        (assign62650_e97197, ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign62650_e97195 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign62650_e97195 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign62650_e97195 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign62650_e97195 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign62650_e97195 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign62650_e97195 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign62650_e97195 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn9)) * locals.var_mu) + (assign62650_e97195 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign62650_e97195 * locals.var_mu_dn10)), ((((locals.var_betawl_dn13 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn13)) * locals.var_mu) + (assign62650_e97195 * locals.var_mu_dn13)),)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn4, locals.var_idspt0_dn5, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn8, locals.var_idspt0_dn9, locals.var_idspt0_dn10, locals.var_idspt0_dn13,)
    }
};
        locals.var_idspt0 = assign62650_e97199;
        locals.var_idspt0_dn0 = assign62650_e97199_d_n0;
        locals.var_idspt0_dn2 = assign62650_e97199_d_n2;
        locals.var_idspt0_dn4 = assign62650_e97199_d_n4;
        locals.var_idspt0_dn5 = assign62650_e97199_d_n5;
        locals.var_idspt0_dn6 = assign62650_e97199_d_n6;
        locals.var_idspt0_dn7 = assign62650_e97199_d_n7;
        locals.var_idspt0_dn8 = assign62650_e97199_d_n8;
        locals.var_idspt0_dn9 = assign62650_e97199_d_n9;
        locals.var_idspt0_dn10 = assign62650_e97199_d_n10;
        locals.var_idspt0_dn13 = assign62650_e97199_d_n13;

        let (assign62660_e97210, assign62660_e97210_d_n0, assign62660_e97210_d_n2, assign62660_e97210_d_n4, assign62660_e97210_d_n5, assign62660_e97210_d_n6, assign62660_e97210_d_n7, assign62660_e97210_d_n8, assign62660_e97210_d_n9, assign62660_e97210_d_n10, assign62660_e97210_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign62660_e97208: f64 = (locals.var_ids0 + locals.var_idspt0);
        (assign62660_e97208, (locals.var_ids0_dn0 + locals.var_idspt0_dn0), (locals.var_ids0_dn2 + locals.var_idspt0_dn2), (locals.var_ids0_dn4 + locals.var_idspt0_dn4), (locals.var_ids0_dn5 + locals.var_idspt0_dn5), (locals.var_ids0_dn6 + locals.var_idspt0_dn6), (locals.var_ids0_dn7 + locals.var_idspt0_dn7), (locals.var_ids0_dn8 + locals.var_idspt0_dn8), (locals.var_ids0_dn9 + locals.var_idspt0_dn9), (locals.var_ids0_dn10 + locals.var_idspt0_dn10), (locals.var_ids0_dn13 + locals.var_idspt0_dn13),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    }
};
        locals.var_ids0 = assign62660_e97210;
        locals.var_ids0_dn0 = assign62660_e97210_d_n0;
        locals.var_ids0_dn2 = assign62660_e97210_d_n2;
        locals.var_ids0_dn4 = assign62660_e97210_d_n4;
        locals.var_ids0_dn5 = assign62660_e97210_d_n5;
        locals.var_ids0_dn6 = assign62660_e97210_d_n6;
        locals.var_ids0_dn7 = assign62660_e97210_d_n7;
        locals.var_ids0_dn8 = assign62660_e97210_d_n8;
        locals.var_ids0_dn9 = assign62660_e97210_d_n9;
        locals.var_ids0_dn10 = assign62660_e97210_d_n10;
        locals.var_ids0_dn13 = assign62660_e97210_d_n13;

        let (assign62670_e97220, assign62670_e97220_d_n0, assign62670_e97220_d_n2, assign62670_e97220_d_n4, assign62670_e97220_d_n5, assign62670_e97220_d_n6, assign62670_e97220_d_n7, assign62670_e97220_d_n8, assign62670_e97220_d_n9, assign62670_e97220_d_n10, assign62670_e97220_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1498 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn4, locals.var_idspt0_dn5, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn8, locals.var_idspt0_dn9, locals.var_idspt0_dn10, locals.var_idspt0_dn13,)
    }
};
        locals.var_idspt0 = assign62670_e97220;
        locals.var_idspt0_dn0 = assign62670_e97220_d_n0;
        locals.var_idspt0_dn2 = assign62670_e97220_d_n2;
        locals.var_idspt0_dn4 = assign62670_e97220_d_n4;
        locals.var_idspt0_dn5 = assign62670_e97220_d_n5;
        locals.var_idspt0_dn6 = assign62670_e97220_d_n6;
        locals.var_idspt0_dn7 = assign62670_e97220_d_n7;
        locals.var_idspt0_dn8 = assign62670_e97220_d_n8;
        locals.var_idspt0_dn9 = assign62670_e97220_d_n9;
        locals.var_idspt0_dn10 = assign62670_e97220_d_n10;
        locals.var_idspt0_dn13 = assign62670_e97220_d_n13;

        let assign62680_e97227: f64 = if ((locals.var_flg_rsrd == 2.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign62680_e97227;

        let assign62690_e97230: f64 = if p.p296 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign62690_e97230;

        let (assign62700_e97241, assign62700_e97241_d_n0, assign62700_e97241_d_n2, assign62700_e97241_d_n4, assign62700_e97241_d_n5, assign62700_e97241_d_n6, assign62700_e97241_d_n7, assign62700_e97241_d_n8, assign62700_e97241_d_n9, assign62700_e97241_d_n10, assign62700_e97241_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign62700_e97241;
        locals.var_t4_dn0 = assign62700_e97241_d_n0;
        locals.var_t4_dn2 = assign62700_e97241_d_n2;
        locals.var_t4_dn4 = assign62700_e97241_d_n4;
        locals.var_t4_dn5 = assign62700_e97241_d_n5;
        locals.var_t4_dn6 = assign62700_e97241_d_n6;
        locals.var_t4_dn7 = assign62700_e97241_d_n7;
        locals.var_t4_dn8 = assign62700_e97241_d_n8;
        locals.var_t4_dn9 = assign62700_e97241_d_n9;
        locals.var_t4_dn10 = assign62700_e97241_d_n10;
        locals.var_t4_dn13 = assign62700_e97241_d_n13;

        let (assign62710_e97256, assign62710_e97256_d_n0, assign62710_e97256_d_n2, assign62710_e97256_d_n4, assign62710_e97256_d_n5, assign62710_e97256_d_n6, assign62710_e97256_d_n7, assign62710_e97256_d_n8, assign62710_e97256_d_n9, assign62710_e97256_d_n10, assign62710_e97256_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62710_e97253: f64 = (locals.var_vgse - p.p300);
        let assign62710_e97254: f64 = (locals.var_uc_rd24 * assign62710_e97253);
        (assign62710_e97254, (locals.var_uc_rd24 * locals.var_vgse_dn0), (locals.var_uc_rd24 * locals.var_vgse_dn2), 0.0, 0.0, (locals.var_uc_rd24 * locals.var_vgse_dn6), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign62710_e97256;
        locals.var_t1_dn0 = assign62710_e97256_d_n0;
        locals.var_t1_dn2 = assign62710_e97256_d_n2;
        locals.var_t1_dn4 = assign62710_e97256_d_n4;
        locals.var_t1_dn5 = assign62710_e97256_d_n5;
        locals.var_t1_dn6 = assign62710_e97256_d_n6;
        locals.var_t1_dn7 = assign62710_e97256_d_n7;
        locals.var_t1_dn8 = assign62710_e97256_d_n8;
        locals.var_t1_dn9 = assign62710_e97256_d_n9;
        locals.var_t1_dn10 = assign62710_e97256_d_n10;
        locals.var_t1_dn13 = assign62710_e97256_d_n13;

        let (assign62720_e97273, assign62720_e97273_d_n0, assign62720_e97273_d_n2, assign62720_e97273_d_n4, assign62720_e97273_d_n5, assign62720_e97273_d_n6, assign62720_e97273_d_n7, assign62720_e97273_d_n8, assign62720_e97273_d_n9, assign62720_e97273_d_n10, assign62720_e97273_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62720_e97267: f64 = (locals.var_t1 - locals.var_t4);
        let assign62720_e97270: f64 = (0.01 * 0.01);
        let assign62720_e97271: f64 = (assign62720_e97267 - assign62720_e97270);
        (assign62720_e97271, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn13 - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign62720_e97273;
        locals.var_tmf1_dn0 = assign62720_e97273_d_n0;
        locals.var_tmf1_dn2 = assign62720_e97273_d_n2;
        locals.var_tmf1_dn4 = assign62720_e97273_d_n4;
        locals.var_tmf1_dn5 = assign62720_e97273_d_n5;
        locals.var_tmf1_dn6 = assign62720_e97273_d_n6;
        locals.var_tmf1_dn7 = assign62720_e97273_d_n7;
        locals.var_tmf1_dn8 = assign62720_e97273_d_n8;
        locals.var_tmf1_dn9 = assign62720_e97273_d_n9;
        locals.var_tmf1_dn10 = assign62720_e97273_d_n10;
        locals.var_tmf1_dn13 = assign62720_e97273_d_n13;

    }

    pub(super) fn stamp_transient_block_212(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62730_e97290, assign62730_e97290_d_n0, assign62730_e97290_d_n2, assign62730_e97290_d_n4, assign62730_e97290_d_n5, assign62730_e97290_d_n6, assign62730_e97290_d_n7, assign62730_e97290_d_n8, assign62730_e97290_d_n9, assign62730_e97290_d_n10, assign62730_e97290_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62730_e97284: f64 = (4.0 * locals.var_t4);
        let assign62730_e97287: f64 = (0.01 * 0.01);
        let assign62730_e97288: f64 = (assign62730_e97284 * assign62730_e97287);
        (assign62730_e97288, ((4.0 * locals.var_t4_dn0) * assign62730_e97287), ((4.0 * locals.var_t4_dn2) * assign62730_e97287), ((4.0 * locals.var_t4_dn4) * assign62730_e97287), ((4.0 * locals.var_t4_dn5) * assign62730_e97287), ((4.0 * locals.var_t4_dn6) * assign62730_e97287), ((4.0 * locals.var_t4_dn7) * assign62730_e97287), ((4.0 * locals.var_t4_dn8) * assign62730_e97287), ((4.0 * locals.var_t4_dn9) * assign62730_e97287), ((4.0 * locals.var_t4_dn10) * assign62730_e97287), ((4.0 * locals.var_t4_dn13) * assign62730_e97287),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign62730_e97290;
        locals.var_tmf2_dn0 = assign62730_e97290_d_n0;
        locals.var_tmf2_dn2 = assign62730_e97290_d_n2;
        locals.var_tmf2_dn4 = assign62730_e97290_d_n4;
        locals.var_tmf2_dn5 = assign62730_e97290_d_n5;
        locals.var_tmf2_dn6 = assign62730_e97290_d_n6;
        locals.var_tmf2_dn7 = assign62730_e97290_d_n7;
        locals.var_tmf2_dn8 = assign62730_e97290_d_n8;
        locals.var_tmf2_dn9 = assign62730_e97290_d_n9;
        locals.var_tmf2_dn10 = assign62730_e97290_d_n10;
        locals.var_tmf2_dn13 = assign62730_e97290_d_n13;

        let (assign62740_e97307, assign62740_e97307_d_n0, assign62740_e97307_d_n2, assign62740_e97307_d_n4, assign62740_e97307_d_n5, assign62740_e97307_d_n6, assign62740_e97307_d_n7, assign62740_e97307_d_n8, assign62740_e97307_d_n9, assign62740_e97307_d_n10, assign62740_e97307_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let (assign62740_e97305, assign62740_e97305_d_n0, assign62740_e97305_d_n2, assign62740_e97305_d_n4, assign62740_e97305_d_n5, assign62740_e97305_d_n6, assign62740_e97305_d_n7, assign62740_e97305_d_n8, assign62740_e97305_d_n9, assign62740_e97305_d_n10, assign62740_e97305_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign62740_e97304: f64 = (-locals.var_tmf2);
                (assign62740_e97304, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign62740_e97305, assign62740_e97305_d_n0, assign62740_e97305_d_n2, assign62740_e97305_d_n4, assign62740_e97305_d_n5, assign62740_e97305_d_n6, assign62740_e97305_d_n7, assign62740_e97305_d_n8, assign62740_e97305_d_n9, assign62740_e97305_d_n10, assign62740_e97305_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign62740_e97307;
        locals.var_tmf2_dn0 = assign62740_e97307_d_n0;
        locals.var_tmf2_dn2 = assign62740_e97307_d_n2;
        locals.var_tmf2_dn4 = assign62740_e97307_d_n4;
        locals.var_tmf2_dn5 = assign62740_e97307_d_n5;
        locals.var_tmf2_dn6 = assign62740_e97307_d_n6;
        locals.var_tmf2_dn7 = assign62740_e97307_d_n7;
        locals.var_tmf2_dn8 = assign62740_e97307_d_n8;
        locals.var_tmf2_dn9 = assign62740_e97307_d_n9;
        locals.var_tmf2_dn10 = assign62740_e97307_d_n10;
        locals.var_tmf2_dn13 = assign62740_e97307_d_n13;

        let (assign62750_e97323, assign62750_e97323_d_n0, assign62750_e97323_d_n2, assign62750_e97323_d_n4, assign62750_e97323_d_n5, assign62750_e97323_d_n6, assign62750_e97323_d_n7, assign62750_e97323_d_n8, assign62750_e97323_d_n9, assign62750_e97323_d_n10, assign62750_e97323_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62750_e97318: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign62750_e97320: f64 = (assign62750_e97318 + locals.var_tmf2);
        let assign62750_e97321: f64 = (assign62750_e97320).sqrt();
        (assign62750_e97321, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign62750_e97321)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign62750_e97321)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign62750_e97321)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign62750_e97321)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign62750_e97321)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign62750_e97321)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign62750_e97321)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign62750_e97321)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign62750_e97321)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign62750_e97321)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign62750_e97323;
        locals.var_tmf2_dn0 = assign62750_e97323_d_n0;
        locals.var_tmf2_dn2 = assign62750_e97323_d_n2;
        locals.var_tmf2_dn4 = assign62750_e97323_d_n4;
        locals.var_tmf2_dn5 = assign62750_e97323_d_n5;
        locals.var_tmf2_dn6 = assign62750_e97323_d_n6;
        locals.var_tmf2_dn7 = assign62750_e97323_d_n7;
        locals.var_tmf2_dn8 = assign62750_e97323_d_n8;
        locals.var_tmf2_dn9 = assign62750_e97323_d_n9;
        locals.var_tmf2_dn10 = assign62750_e97323_d_n10;
        locals.var_tmf2_dn13 = assign62750_e97323_d_n13;

        let (assign62760_e97340, assign62760_e97340_d_n0, assign62760_e97340_d_n2, assign62760_e97340_d_n4, assign62760_e97340_d_n5, assign62760_e97340_d_n6, assign62760_e97340_d_n7, assign62760_e97340_d_n8, assign62760_e97340_d_n9, assign62760_e97340_d_n10, assign62760_e97340_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62760_e97336: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign62760_e97337: f64 = (1.0 + assign62760_e97336);
        let assign62760_e97338: f64 = (0.5 * assign62760_e97337);
        (assign62760_e97338, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign62760_e97340;
        locals.var_t0_dn0 = assign62760_e97340_d_n0;
        locals.var_t0_dn2 = assign62760_e97340_d_n2;
        locals.var_t0_dn4 = assign62760_e97340_d_n4;
        locals.var_t0_dn5 = assign62760_e97340_d_n5;
        locals.var_t0_dn6 = assign62760_e97340_d_n6;
        locals.var_t0_dn7 = assign62760_e97340_d_n7;
        locals.var_t0_dn8 = assign62760_e97340_d_n8;
        locals.var_t0_dn9 = assign62760_e97340_d_n9;
        locals.var_t0_dn10 = assign62760_e97340_d_n10;
        locals.var_t0_dn13 = assign62760_e97340_d_n13;

        let (assign62770_e97357, assign62770_e97357_d_n0, assign62770_e97357_d_n2, assign62770_e97357_d_n4, assign62770_e97357_d_n5, assign62770_e97357_d_n6, assign62770_e97357_d_n7, assign62770_e97357_d_n8, assign62770_e97357_d_n9, assign62770_e97357_d_n10, assign62770_e97357_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62770_e97353: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign62770_e97354: f64 = (0.5 * assign62770_e97353);
        let assign62770_e97355: f64 = (locals.var_t4 + assign62770_e97354);
        (assign62770_e97355, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62770_e97357;
        locals.var_t2_dn0 = assign62770_e97357_d_n0;
        locals.var_t2_dn2 = assign62770_e97357_d_n2;
        locals.var_t2_dn4 = assign62770_e97357_d_n4;
        locals.var_t2_dn5 = assign62770_e97357_d_n5;
        locals.var_t2_dn6 = assign62770_e97357_d_n6;
        locals.var_t2_dn7 = assign62770_e97357_d_n7;
        locals.var_t2_dn8 = assign62770_e97357_d_n8;
        locals.var_t2_dn9 = assign62770_e97357_d_n9;
        locals.var_t2_dn10 = assign62770_e97357_d_n10;
        locals.var_t2_dn13 = assign62770_e97357_d_n13;

        let (assign62780_e97372, assign62780_e97372_d_n0, assign62780_e97372_d_n2, assign62780_e97372_d_n4, assign62780_e97372_d_n5, assign62780_e97372_d_n6, assign62780_e97372_d_n7, assign62780_e97372_d_n8, assign62780_e97372_d_n9, assign62780_e97372_d_n10, assign62780_e97372_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62780_e97369: f64 = (p.p296 + 1.0);
        let assign62780_e97370: f64 = (locals.var_t4 * assign62780_e97369);
        (assign62780_e97370, (locals.var_t4_dn0 * assign62780_e97369), (locals.var_t4_dn2 * assign62780_e97369), (locals.var_t4_dn4 * assign62780_e97369), (locals.var_t4_dn5 * assign62780_e97369), (locals.var_t4_dn6 * assign62780_e97369), (locals.var_t4_dn7 * assign62780_e97369), (locals.var_t4_dn8 * assign62780_e97369), (locals.var_t4_dn9 * assign62780_e97369), (locals.var_t4_dn10 * assign62780_e97369), (locals.var_t4_dn13 * assign62780_e97369),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign62780_e97372;
        locals.var_t3_dn0 = assign62780_e97372_d_n0;
        locals.var_t3_dn2 = assign62780_e97372_d_n2;
        locals.var_t3_dn4 = assign62780_e97372_d_n4;
        locals.var_t3_dn5 = assign62780_e97372_d_n5;
        locals.var_t3_dn6 = assign62780_e97372_d_n6;
        locals.var_t3_dn7 = assign62780_e97372_d_n7;
        locals.var_t3_dn8 = assign62780_e97372_d_n8;
        locals.var_t3_dn9 = assign62780_e97372_d_n9;
        locals.var_t3_dn10 = assign62780_e97372_d_n10;
        locals.var_t3_dn13 = assign62780_e97372_d_n13;

        let (assign62790_e97389, assign62790_e97389_d_n0, assign62790_e97389_d_n2, assign62790_e97389_d_n4, assign62790_e97389_d_n5, assign62790_e97389_d_n6, assign62790_e97389_d_n7, assign62790_e97389_d_n8, assign62790_e97389_d_n9, assign62790_e97389_d_n10, assign62790_e97389_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62790_e97383: f64 = (locals.var_t3 - locals.var_t2);
        let assign62790_e97386: f64 = (0.01 * 0.01);
        let assign62790_e97387: f64 = (assign62790_e97383 - assign62790_e97386);
        (assign62790_e97387, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign62790_e97389;
        locals.var_tmf1_dn0 = assign62790_e97389_d_n0;
        locals.var_tmf1_dn2 = assign62790_e97389_d_n2;
        locals.var_tmf1_dn4 = assign62790_e97389_d_n4;
        locals.var_tmf1_dn5 = assign62790_e97389_d_n5;
        locals.var_tmf1_dn6 = assign62790_e97389_d_n6;
        locals.var_tmf1_dn7 = assign62790_e97389_d_n7;
        locals.var_tmf1_dn8 = assign62790_e97389_d_n8;
        locals.var_tmf1_dn9 = assign62790_e97389_d_n9;
        locals.var_tmf1_dn10 = assign62790_e97389_d_n10;
        locals.var_tmf1_dn13 = assign62790_e97389_d_n13;

        let (assign62800_e97406, assign62800_e97406_d_n0, assign62800_e97406_d_n2, assign62800_e97406_d_n4, assign62800_e97406_d_n5, assign62800_e97406_d_n6, assign62800_e97406_d_n7, assign62800_e97406_d_n8, assign62800_e97406_d_n9, assign62800_e97406_d_n10, assign62800_e97406_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62800_e97400: f64 = (4.0 * locals.var_t3);
        let assign62800_e97403: f64 = (0.01 * 0.01);
        let assign62800_e97404: f64 = (assign62800_e97400 * assign62800_e97403);
        (assign62800_e97404, ((4.0 * locals.var_t3_dn0) * assign62800_e97403), ((4.0 * locals.var_t3_dn2) * assign62800_e97403), ((4.0 * locals.var_t3_dn4) * assign62800_e97403), ((4.0 * locals.var_t3_dn5) * assign62800_e97403), ((4.0 * locals.var_t3_dn6) * assign62800_e97403), ((4.0 * locals.var_t3_dn7) * assign62800_e97403), ((4.0 * locals.var_t3_dn8) * assign62800_e97403), ((4.0 * locals.var_t3_dn9) * assign62800_e97403), ((4.0 * locals.var_t3_dn10) * assign62800_e97403), ((4.0 * locals.var_t3_dn13) * assign62800_e97403),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign62800_e97406;
        locals.var_tmf2_dn0 = assign62800_e97406_d_n0;
        locals.var_tmf2_dn2 = assign62800_e97406_d_n2;
        locals.var_tmf2_dn4 = assign62800_e97406_d_n4;
        locals.var_tmf2_dn5 = assign62800_e97406_d_n5;
        locals.var_tmf2_dn6 = assign62800_e97406_d_n6;
        locals.var_tmf2_dn7 = assign62800_e97406_d_n7;
        locals.var_tmf2_dn8 = assign62800_e97406_d_n8;
        locals.var_tmf2_dn9 = assign62800_e97406_d_n9;
        locals.var_tmf2_dn10 = assign62800_e97406_d_n10;
        locals.var_tmf2_dn13 = assign62800_e97406_d_n13;

        let (assign62810_e97423, assign62810_e97423_d_n0, assign62810_e97423_d_n2, assign62810_e97423_d_n4, assign62810_e97423_d_n5, assign62810_e97423_d_n6, assign62810_e97423_d_n7, assign62810_e97423_d_n8, assign62810_e97423_d_n9, assign62810_e97423_d_n10, assign62810_e97423_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let (assign62810_e97421, assign62810_e97421_d_n0, assign62810_e97421_d_n2, assign62810_e97421_d_n4, assign62810_e97421_d_n5, assign62810_e97421_d_n6, assign62810_e97421_d_n7, assign62810_e97421_d_n8, assign62810_e97421_d_n9, assign62810_e97421_d_n10, assign62810_e97421_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign62810_e97420: f64 = (-locals.var_tmf2);
                (assign62810_e97420, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign62810_e97421, assign62810_e97421_d_n0, assign62810_e97421_d_n2, assign62810_e97421_d_n4, assign62810_e97421_d_n5, assign62810_e97421_d_n6, assign62810_e97421_d_n7, assign62810_e97421_d_n8, assign62810_e97421_d_n9, assign62810_e97421_d_n10, assign62810_e97421_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign62810_e97423;
        locals.var_tmf2_dn0 = assign62810_e97423_d_n0;
        locals.var_tmf2_dn2 = assign62810_e97423_d_n2;
        locals.var_tmf2_dn4 = assign62810_e97423_d_n4;
        locals.var_tmf2_dn5 = assign62810_e97423_d_n5;
        locals.var_tmf2_dn6 = assign62810_e97423_d_n6;
        locals.var_tmf2_dn7 = assign62810_e97423_d_n7;
        locals.var_tmf2_dn8 = assign62810_e97423_d_n8;
        locals.var_tmf2_dn9 = assign62810_e97423_d_n9;
        locals.var_tmf2_dn10 = assign62810_e97423_d_n10;
        locals.var_tmf2_dn13 = assign62810_e97423_d_n13;

        let (assign62820_e97439, assign62820_e97439_d_n0, assign62820_e97439_d_n2, assign62820_e97439_d_n4, assign62820_e97439_d_n5, assign62820_e97439_d_n6, assign62820_e97439_d_n7, assign62820_e97439_d_n8, assign62820_e97439_d_n9, assign62820_e97439_d_n10, assign62820_e97439_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62820_e97434: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign62820_e97436: f64 = (assign62820_e97434 + locals.var_tmf2);
        let assign62820_e97437: f64 = (assign62820_e97436).sqrt();
        (assign62820_e97437, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign62820_e97437)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign62820_e97437)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign62820_e97437)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign62820_e97437)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign62820_e97437)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign62820_e97437)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign62820_e97437)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign62820_e97437)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign62820_e97437)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign62820_e97437)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign62820_e97439;
        locals.var_tmf2_dn0 = assign62820_e97439_d_n0;
        locals.var_tmf2_dn2 = assign62820_e97439_d_n2;
        locals.var_tmf2_dn4 = assign62820_e97439_d_n4;
        locals.var_tmf2_dn5 = assign62820_e97439_d_n5;
        locals.var_tmf2_dn6 = assign62820_e97439_d_n6;
        locals.var_tmf2_dn7 = assign62820_e97439_d_n7;
        locals.var_tmf2_dn8 = assign62820_e97439_d_n8;
        locals.var_tmf2_dn9 = assign62820_e97439_d_n9;
        locals.var_tmf2_dn10 = assign62820_e97439_d_n10;
        locals.var_tmf2_dn13 = assign62820_e97439_d_n13;

        let (assign62830_e97456, assign62830_e97456_d_n0, assign62830_e97456_d_n2, assign62830_e97456_d_n4, assign62830_e97456_d_n5, assign62830_e97456_d_n6, assign62830_e97456_d_n7, assign62830_e97456_d_n8, assign62830_e97456_d_n9, assign62830_e97456_d_n10, assign62830_e97456_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62830_e97452: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign62830_e97453: f64 = (1.0 + assign62830_e97452);
        let assign62830_e97454: f64 = (0.5 * assign62830_e97453);
        (assign62830_e97454, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign62830_e97456;
        locals.var_t0_dn0 = assign62830_e97456_d_n0;
        locals.var_t0_dn2 = assign62830_e97456_d_n2;
        locals.var_t0_dn4 = assign62830_e97456_d_n4;
        locals.var_t0_dn5 = assign62830_e97456_d_n5;
        locals.var_t0_dn6 = assign62830_e97456_d_n6;
        locals.var_t0_dn7 = assign62830_e97456_d_n7;
        locals.var_t0_dn8 = assign62830_e97456_d_n8;
        locals.var_t0_dn9 = assign62830_e97456_d_n9;
        locals.var_t0_dn10 = assign62830_e97456_d_n10;
        locals.var_t0_dn13 = assign62830_e97456_d_n13;

        let (assign62840_e97473, assign62840_e97473_d_n0, assign62840_e97473_d_n2, assign62840_e97473_d_n4, assign62840_e97473_d_n5, assign62840_e97473_d_n6, assign62840_e97473_d_n7, assign62840_e97473_d_n8, assign62840_e97473_d_n9, assign62840_e97473_d_n10, assign62840_e97473_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62840_e97469: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign62840_e97470: f64 = (0.5 * assign62840_e97469);
        let assign62840_e97471: f64 = (locals.var_t3 - assign62840_e97470);
        (assign62840_e97471, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign62840_e97473;
        locals.var_t7_dn0 = assign62840_e97473_d_n0;
        locals.var_t7_dn2 = assign62840_e97473_d_n2;
        locals.var_t7_dn4 = assign62840_e97473_d_n4;
        locals.var_t7_dn5 = assign62840_e97473_d_n5;
        locals.var_t7_dn6 = assign62840_e97473_d_n6;
        locals.var_t7_dn7 = assign62840_e97473_d_n7;
        locals.var_t7_dn8 = assign62840_e97473_d_n8;
        locals.var_t7_dn9 = assign62840_e97473_d_n9;
        locals.var_t7_dn10 = assign62840_e97473_d_n10;
        locals.var_t7_dn13 = assign62840_e97473_d_n13;

        let (assign62850_e97485, assign62850_e97485_d_n0, assign62850_e97485_d_n2, assign62850_e97485_d_n4, assign62850_e97485_d_n5, assign62850_e97485_d_n6, assign62850_e97485_d_n7, assign62850_e97485_d_n8, assign62850_e97485_d_n9, assign62850_e97485_d_n10, assign62850_e97485_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign62850_e97485;
        locals.var_t7_dn0 = assign62850_e97485_d_n0;
        locals.var_t7_dn2 = assign62850_e97485_d_n2;
        locals.var_t7_dn4 = assign62850_e97485_d_n4;
        locals.var_t7_dn5 = assign62850_e97485_d_n5;
        locals.var_t7_dn6 = assign62850_e97485_d_n6;
        locals.var_t7_dn7 = assign62850_e97485_d_n7;
        locals.var_t7_dn8 = assign62850_e97485_d_n8;
        locals.var_t7_dn9 = assign62850_e97485_d_n9;
        locals.var_t7_dn10 = assign62850_e97485_d_n10;
        locals.var_t7_dn13 = assign62850_e97485_d_n13;

        let assign62860_e97488: f64 = if locals.var_vdse >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1501 = assign62860_e97488;

        let (assign62870_e97499, assign62870_e97499_d_n0, assign62870_e97499_d_n2,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign62870_e97499;
        locals.var_vdse_eff_dn0 = assign62870_e97499_d_n0;
        locals.var_vdse_eff_dn2 = assign62870_e97499_d_n2;

        let (assign62880_e97511, assign62880_e97511_d_n0, assign62880_e97511_d_n2,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1501 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign62880_e97511;
        locals.var_vdse_eff_dn0 = assign62880_e97511_d_n0;
        locals.var_vdse_eff_dn2 = assign62880_e97511_d_n2;

        let assign62890_e97515: f64 = (20.0 * 1e-12);
        let assign62890_e97516: f64 = if locals.var_vdse_eff < assign62890_e97515 { 1.0 } else { 0.0 };
        locals.var_guard1502 = assign62890_e97516;

        let (assign62900_e97547,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62900_e97527: f64 = (20.0 + 1.0);
        let assign62900_e97530: f64 = (p.p297 - 1.0);
        let assign62900_e97531: f64 = (assign62900_e97527).powf(assign62900_e97530);
        let assign62900_e97534: f64 = (20.0 + 1.0);
        let assign62900_e97537: f64 = (0.5 * p.p297);
        let assign62900_e97539: f64 = (assign62900_e97537 * 20.0);
        let assign62900_e97540: f64 = (assign62900_e97534 - assign62900_e97539);
        let assign62900_e97541: f64 = (assign62900_e97531 * assign62900_e97540);
        let assign62900_e97544: f64 = (1e-12_f64).powf(p.p297);
        let assign62900_e97545: f64 = (assign62900_e97541 * assign62900_e97544);
        (assign62900_e97545,)
    } else {
        (locals.var_ra_alpha,)
    }
};
        locals.var_ra_alpha = assign62900_e97547;

        let (assign62910_e97576,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62910_e97558: f64 = (0.5 * p.p297);
        let assign62910_e97561: f64 = (20.0 + 1.0);
        let assign62910_e97564: f64 = (p.p297 - 1.0);
        let assign62910_e97565: f64 = (assign62910_e97561).powf(assign62910_e97564);
        let assign62910_e97566: f64 = (assign62910_e97558 * assign62910_e97565);
        let assign62910_e97568: f64 = (assign62910_e97566 / 20.0);
        let assign62910_e97572: f64 = (p.p297 - 2.0);
        let assign62910_e97573: f64 = (1e-12_f64).powf(assign62910_e97572);
        let assign62910_e97574: f64 = (assign62910_e97568 * assign62910_e97573);
        (assign62910_e97574,)
    } else {
        (locals.var_ra_beta,)
    }
};
        locals.var_ra_beta = assign62910_e97576;

        let (assign62920_e97593, assign62920_e97593_d_n0, assign62920_e97593_d_n2, assign62920_e97593_d_n4, assign62920_e97593_d_n5, assign62920_e97593_d_n6, assign62920_e97593_d_n7, assign62920_e97593_d_n8, assign62920_e97593_d_n9, assign62920_e97593_d_n10, assign62920_e97593_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62920_e97588: f64 = (locals.var_ra_beta * locals.var_vdse_eff);
        let assign62920_e97590: f64 = (assign62920_e97588 * locals.var_vdse_eff);
        let assign62920_e97591: f64 = (locals.var_ra_alpha + assign62920_e97590);
        (assign62920_e97591, (((locals.var_ra_beta * locals.var_vdse_eff_dn0) * locals.var_vdse_eff) + (assign62920_e97588 * locals.var_vdse_eff_dn0)), (((locals.var_ra_beta * locals.var_vdse_eff_dn2) * locals.var_vdse_eff) + (assign62920_e97588 * locals.var_vdse_eff_dn2)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign62920_e97593;
        locals.var_t1_dn0 = assign62920_e97593_d_n0;
        locals.var_t1_dn2 = assign62920_e97593_d_n2;
        locals.var_t1_dn4 = assign62920_e97593_d_n4;
        locals.var_t1_dn5 = assign62920_e97593_d_n5;
        locals.var_t1_dn6 = assign62920_e97593_d_n6;
        locals.var_t1_dn7 = assign62920_e97593_d_n7;
        locals.var_t1_dn8 = assign62920_e97593_d_n8;
        locals.var_t1_dn9 = assign62920_e97593_d_n9;
        locals.var_t1_dn10 = assign62920_e97593_d_n10;
        locals.var_t1_dn13 = assign62920_e97593_d_n13;

        let (assign62930_e97609, assign62930_e97609_d_n0, assign62930_e97609_d_n2, assign62930_e97609_d_n4, assign62930_e97609_d_n5, assign62930_e97609_d_n6, assign62930_e97609_d_n7, assign62930_e97609_d_n8, assign62930_e97609_d_n9, assign62930_e97609_d_n10, assign62930_e97609_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1502 == 0.0)) {
        let assign62930_e97605: f64 = (locals.var_vdse_eff + 1e-12);
        let assign62930_e97607: f64 = (assign62930_e97605).powf(p.p297);
        (assign62930_e97607, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign62930_e97605).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign62930_e97607 * (p.p297 * (locals.var_vdse_eff_dn0 / assign62930_e97605))) }, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign62930_e97605).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign62930_e97607 * (p.p297 * (locals.var_vdse_eff_dn2 / assign62930_e97605))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign62930_e97609;
        locals.var_t1_dn0 = assign62930_e97609_d_n0;
        locals.var_t1_dn2 = assign62930_e97609_d_n2;
        locals.var_t1_dn4 = assign62930_e97609_d_n4;
        locals.var_t1_dn5 = assign62930_e97609_d_n5;
        locals.var_t1_dn6 = assign62930_e97609_d_n6;
        locals.var_t1_dn7 = assign62930_e97609_d_n7;
        locals.var_t1_dn8 = assign62930_e97609_d_n8;
        locals.var_t1_dn9 = assign62930_e97609_d_n9;
        locals.var_t1_dn10 = assign62930_e97609_d_n10;
        locals.var_t1_dn13 = assign62930_e97609_d_n13;

        let (assign62940_e97622, assign62940_e97622_d_n0, assign62940_e97622_d_n2, assign62940_e97622_d_n4, assign62940_e97622_d_n5, assign62940_e97622_d_n6, assign62940_e97622_d_n7, assign62940_e97622_d_n8, assign62940_e97622_d_n9, assign62940_e97622_d_n10, assign62940_e97622_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62940_e97618: f64 = (locals.var_vdse_eff + 1e-12);
        let assign62940_e97620: f64 = (assign62940_e97618).powf(p.p299);
        (assign62940_e97620, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign62940_e97618).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign62940_e97620 * (p.p299 * (locals.var_vdse_eff_dn0 / assign62940_e97618))) }, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign62940_e97618).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign62940_e97620 * (p.p299 * (locals.var_vdse_eff_dn2 / assign62940_e97618))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign62940_e97622;
        locals.var_t9_dn0 = assign62940_e97622_d_n0;
        locals.var_t9_dn2 = assign62940_e97622_d_n2;
        locals.var_t9_dn4 = assign62940_e97622_d_n4;
        locals.var_t9_dn5 = assign62940_e97622_d_n5;
        locals.var_t9_dn6 = assign62940_e97622_d_n6;
        locals.var_t9_dn7 = assign62940_e97622_d_n7;
        locals.var_t9_dn8 = assign62940_e97622_d_n8;
        locals.var_t9_dn9 = assign62940_e97622_d_n9;
        locals.var_t9_dn10 = assign62940_e97622_d_n10;
        locals.var_t9_dn13 = assign62940_e97622_d_n13;

        let (assign62950_e97641, assign62950_e97641_d_n0, assign62950_e97641_d_n2, assign62950_e97641_d_n4, assign62950_e97641_d_n5, assign62950_e97641_d_n6, assign62950_e97641_d_n7, assign62950_e97641_d_n8, assign62950_e97641_d_n9, assign62950_e97641_d_n10, assign62950_e97641_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62950_e97631: f64 = (locals.var_t7 * locals.var_t1);
        let assign62950_e97634: f64 = (locals.var_vbse * locals.var_uc_rd22);
        let assign62950_e97636: f64 = (assign62950_e97634 * locals.var_t9);
        let assign62950_e97637: f64 = (assign62950_e97631 + assign62950_e97636);
        let assign62950_e97639: f64 = (assign62950_e97637 / locals.var_weff_nf);
        (assign62950_e97639, ((((locals.var_t7_dn0 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn0)) + (((locals.var_vbse_dn0 * locals.var_uc_rd22) * locals.var_t9) + (assign62950_e97634 * locals.var_t9_dn0))) / locals.var_weff_nf), ((((locals.var_t7_dn2 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn2)) + (((locals.var_vbse_dn2 * locals.var_uc_rd22) * locals.var_t9) + (assign62950_e97634 * locals.var_t9_dn2))) / locals.var_weff_nf), ((((locals.var_t7_dn4 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn4)) + (assign62950_e97634 * locals.var_t9_dn4)) / locals.var_weff_nf), ((((locals.var_t7_dn5 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn5)) + (assign62950_e97634 * locals.var_t9_dn5)) / locals.var_weff_nf), ((((locals.var_t7_dn6 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn6)) + (assign62950_e97634 * locals.var_t9_dn6)) / locals.var_weff_nf), ((((locals.var_t7_dn7 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn7)) + (assign62950_e97634 * locals.var_t9_dn7)) / locals.var_weff_nf), ((((locals.var_t7_dn8 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn8)) + (((locals.var_vbse_dn8 * locals.var_uc_rd22) * locals.var_t9) + (assign62950_e97634 * locals.var_t9_dn8))) / locals.var_weff_nf), ((((locals.var_t7_dn9 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn9)) + (assign62950_e97634 * locals.var_t9_dn9)) / locals.var_weff_nf), ((((locals.var_t7_dn10 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn10)) + (assign62950_e97634 * locals.var_t9_dn10)) / locals.var_weff_nf), ((((locals.var_t7_dn13 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn13)) + (assign62950_e97634 * locals.var_t9_dn13)) / locals.var_weff_nf),)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn13,)
    }
};
        locals.var_ra = assign62950_e97641;
        locals.var_ra_dn0 = assign62950_e97641_d_n0;
        locals.var_ra_dn2 = assign62950_e97641_d_n2;
        locals.var_ra_dn4 = assign62950_e97641_d_n4;
        locals.var_ra_dn5 = assign62950_e97641_d_n5;
        locals.var_ra_dn6 = assign62950_e97641_d_n6;
        locals.var_ra_dn7 = assign62950_e97641_d_n7;
        locals.var_ra_dn8 = assign62950_e97641_d_n8;
        locals.var_ra_dn9 = assign62950_e97641_d_n9;
        locals.var_ra_dn10 = assign62950_e97641_d_n10;
        locals.var_ra_dn13 = assign62950_e97641_d_n13;

        let (assign62960_e97652, assign62960_e97652_d_n0, assign62960_e97652_d_n2, assign62960_e97652_d_n4, assign62960_e97652_d_n5, assign62960_e97652_d_n6, assign62960_e97652_d_n7, assign62960_e97652_d_n8, assign62960_e97652_d_n9, assign62960_e97652_d_n10, assign62960_e97652_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62960_e97650: f64 = (locals.var_ra * locals.var_ids0);
        (assign62960_e97650, ((locals.var_ra_dn0 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn0)), ((locals.var_ra_dn2 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn2)), ((locals.var_ra_dn4 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn4)), ((locals.var_ra_dn5 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn5)), ((locals.var_ra_dn6 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn6)), ((locals.var_ra_dn7 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn7)), ((locals.var_ra_dn8 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn8)), ((locals.var_ra_dn9 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn9)), ((locals.var_ra_dn10 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn10)), ((locals.var_ra_dn13 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign62960_e97652;
        locals.var_t0_dn0 = assign62960_e97652_d_n0;
        locals.var_t0_dn2 = assign62960_e97652_d_n2;
        locals.var_t0_dn4 = assign62960_e97652_d_n4;
        locals.var_t0_dn5 = assign62960_e97652_d_n5;
        locals.var_t0_dn6 = assign62960_e97652_d_n6;
        locals.var_t0_dn7 = assign62960_e97652_d_n7;
        locals.var_t0_dn8 = assign62960_e97652_d_n8;
        locals.var_t0_dn9 = assign62960_e97652_d_n9;
        locals.var_t0_dn10 = assign62960_e97652_d_n10;
        locals.var_t0_dn13 = assign62960_e97652_d_n13;

        let (assign62970_e97663, assign62970_e97663_d_n0, assign62970_e97663_d_n2, assign62970_e97663_d_n4, assign62970_e97663_d_n5, assign62970_e97663_d_n6, assign62970_e97663_d_n7, assign62970_e97663_d_n8, assign62970_e97663_d_n9, assign62970_e97663_d_n10, assign62970_e97663_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62970_e97661: f64 = (locals.var_vds + 1e-12);
        (assign62970_e97661, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign62970_e97663;
        locals.var_t1_dn0 = assign62970_e97663_d_n0;
        locals.var_t1_dn2 = assign62970_e97663_d_n2;
        locals.var_t1_dn4 = assign62970_e97663_d_n4;
        locals.var_t1_dn5 = assign62970_e97663_d_n5;
        locals.var_t1_dn6 = assign62970_e97663_d_n6;
        locals.var_t1_dn7 = assign62970_e97663_d_n7;
        locals.var_t1_dn8 = assign62970_e97663_d_n8;
        locals.var_t1_dn9 = assign62970_e97663_d_n9;
        locals.var_t1_dn10 = assign62970_e97663_d_n10;
        locals.var_t1_dn13 = assign62970_e97663_d_n13;

        let (assign62980_e97674, assign62980_e97674_d_n0, assign62980_e97674_d_n2, assign62980_e97674_d_n4, assign62980_e97674_d_n5, assign62980_e97674_d_n6, assign62980_e97674_d_n7, assign62980_e97674_d_n8, assign62980_e97674_d_n9, assign62980_e97674_d_n10, assign62980_e97674_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62980_e97672: f64 = (1.0 / locals.var_t1);
        (assign62980_e97672, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn13 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign62980_e97674;
        locals.var_t2_dn0 = assign62980_e97674_d_n0;
        locals.var_t2_dn2 = assign62980_e97674_d_n2;
        locals.var_t2_dn4 = assign62980_e97674_d_n4;
        locals.var_t2_dn5 = assign62980_e97674_d_n5;
        locals.var_t2_dn6 = assign62980_e97674_d_n6;
        locals.var_t2_dn7 = assign62980_e97674_d_n7;
        locals.var_t2_dn8 = assign62980_e97674_d_n8;
        locals.var_t2_dn9 = assign62980_e97674_d_n9;
        locals.var_t2_dn10 = assign62980_e97674_d_n10;
        locals.var_t2_dn13 = assign62980_e97674_d_n13;

    }

    pub(super) fn stamp_transient_block_213(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62990_e97687, assign62990_e97687_d_n0, assign62990_e97687_d_n2, assign62990_e97687_d_n4, assign62990_e97687_d_n5, assign62990_e97687_d_n6, assign62990_e97687_d_n7, assign62990_e97687_d_n8, assign62990_e97687_d_n9, assign62990_e97687_d_n10, assign62990_e97687_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62990_e97684: f64 = (locals.var_t0 * locals.var_t2);
        let assign62990_e97685: f64 = (1.0 + assign62990_e97684);
        (assign62990_e97685, ((locals.var_t0_dn0 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn0)), ((locals.var_t0_dn2 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn2)), ((locals.var_t0_dn4 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn4)), ((locals.var_t0_dn5 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn5)), ((locals.var_t0_dn6 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn6)), ((locals.var_t0_dn7 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn7)), ((locals.var_t0_dn8 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn8)), ((locals.var_t0_dn9 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn9)), ((locals.var_t0_dn10 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn10)), ((locals.var_t0_dn13 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign62990_e97687;
        locals.var_t3_dn0 = assign62990_e97687_d_n0;
        locals.var_t3_dn2 = assign62990_e97687_d_n2;
        locals.var_t3_dn4 = assign62990_e97687_d_n4;
        locals.var_t3_dn5 = assign62990_e97687_d_n5;
        locals.var_t3_dn6 = assign62990_e97687_d_n6;
        locals.var_t3_dn7 = assign62990_e97687_d_n7;
        locals.var_t3_dn8 = assign62990_e97687_d_n8;
        locals.var_t3_dn9 = assign62990_e97687_d_n9;
        locals.var_t3_dn10 = assign62990_e97687_d_n10;
        locals.var_t3_dn13 = assign62990_e97687_d_n13;

        let (assign63000_e97698, assign63000_e97698_d_n0, assign63000_e97698_d_n2, assign63000_e97698_d_n4, assign63000_e97698_d_n5, assign63000_e97698_d_n6, assign63000_e97698_d_n7, assign63000_e97698_d_n8, assign63000_e97698_d_n9, assign63000_e97698_d_n10, assign63000_e97698_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign63000_e97696: f64 = (1.0 / locals.var_t3);
        (assign63000_e97696, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign63000_e97698;
        locals.var_t4_dn0 = assign63000_e97698_d_n0;
        locals.var_t4_dn2 = assign63000_e97698_d_n2;
        locals.var_t4_dn4 = assign63000_e97698_d_n4;
        locals.var_t4_dn5 = assign63000_e97698_d_n5;
        locals.var_t4_dn6 = assign63000_e97698_d_n6;
        locals.var_t4_dn7 = assign63000_e97698_d_n7;
        locals.var_t4_dn8 = assign63000_e97698_d_n8;
        locals.var_t4_dn9 = assign63000_e97698_d_n9;
        locals.var_t4_dn10 = assign63000_e97698_d_n10;
        locals.var_t4_dn13 = assign63000_e97698_d_n13;

        let (assign63010_e97709, assign63010_e97709_d_n0, assign63010_e97709_d_n2, assign63010_e97709_d_n4, assign63010_e97709_d_n5, assign63010_e97709_d_n6, assign63010_e97709_d_n7, assign63010_e97709_d_n8, assign63010_e97709_d_n9, assign63010_e97709_d_n10, assign63010_e97709_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign63010_e97707: f64 = (locals.var_ids0 * locals.var_t4);
        (assign63010_e97707, ((locals.var_ids0_dn0 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn0)), ((locals.var_ids0_dn2 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn2)), ((locals.var_ids0_dn4 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn4)), ((locals.var_ids0_dn5 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn5)), ((locals.var_ids0_dn6 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn6)), ((locals.var_ids0_dn7 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn7)), ((locals.var_ids0_dn8 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn8)), ((locals.var_ids0_dn9 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn9)), ((locals.var_ids0_dn10 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn10)), ((locals.var_ids0_dn13 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn13)),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign63010_e97709;
        locals.var_ids_dn0 = assign63010_e97709_d_n0;
        locals.var_ids_dn2 = assign63010_e97709_d_n2;
        locals.var_ids_dn4 = assign63010_e97709_d_n4;
        locals.var_ids_dn5 = assign63010_e97709_d_n5;
        locals.var_ids_dn6 = assign63010_e97709_d_n6;
        locals.var_ids_dn7 = assign63010_e97709_d_n7;
        locals.var_ids_dn8 = assign63010_e97709_d_n8;
        locals.var_ids_dn9 = assign63010_e97709_d_n9;
        locals.var_ids_dn10 = assign63010_e97709_d_n10;
        locals.var_ids_dn13 = assign63010_e97709_d_n13;

        let (assign63020_e97719, assign63020_e97719_d_n0, assign63020_e97719_d_n2, assign63020_e97719_d_n4, assign63020_e97719_d_n5, assign63020_e97719_d_n6, assign63020_e97719_d_n7, assign63020_e97719_d_n8, assign63020_e97719_d_n9, assign63020_e97719_d_n10, assign63020_e97719_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 == 0.0)) {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign63020_e97719;
        locals.var_ids_dn0 = assign63020_e97719_d_n0;
        locals.var_ids_dn2 = assign63020_e97719_d_n2;
        locals.var_ids_dn4 = assign63020_e97719_d_n4;
        locals.var_ids_dn5 = assign63020_e97719_d_n5;
        locals.var_ids_dn6 = assign63020_e97719_d_n6;
        locals.var_ids_dn7 = assign63020_e97719_d_n7;
        locals.var_ids_dn8 = assign63020_e97719_d_n8;
        locals.var_ids_dn9 = assign63020_e97719_d_n9;
        locals.var_ids_dn10 = assign63020_e97719_d_n10;
        locals.var_ids_dn13 = assign63020_e97719_d_n13;

        let (assign63030_e97729, assign63030_e97729_d_n0, assign63030_e97729_d_n2, assign63030_e97729_d_n4, assign63030_e97729_d_n5, assign63030_e97729_d_n6, assign63030_e97729_d_n7, assign63030_e97729_d_n8, assign63030_e97729_d_n9, assign63030_e97729_d_n10, assign63030_e97729_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1499 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn13,)
    }
};
        locals.var_ra = assign63030_e97729;
        locals.var_ra_dn0 = assign63030_e97729_d_n0;
        locals.var_ra_dn2 = assign63030_e97729_d_n2;
        locals.var_ra_dn4 = assign63030_e97729_d_n4;
        locals.var_ra_dn5 = assign63030_e97729_d_n5;
        locals.var_ra_dn6 = assign63030_e97729_d_n6;
        locals.var_ra_dn7 = assign63030_e97729_d_n7;
        locals.var_ra_dn8 = assign63030_e97729_d_n8;
        locals.var_ra_dn9 = assign63030_e97729_d_n9;
        locals.var_ra_dn10 = assign63030_e97729_d_n10;
        locals.var_ra_dn13 = assign63030_e97729_d_n13;

        let assign63040_e97732: f64 = if p.p27 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1503 = assign63040_e97732;

        let (assign63050_e97743, assign63050_e97743_d_n0, assign63050_e97743_d_n2, assign63050_e97743_d_n4, assign63050_e97743_d_n5, assign63050_e97743_d_n6, assign63050_e97743_d_n7, assign63050_e97743_d_n8, assign63050_e97743_d_n9, assign63050_e97743_d_n10, assign63050_e97743_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63050_e97741: f64 = (1.034943e-10 * locals.var_cox_inv);
        (assign63050_e97741, (1.034943e-10 * locals.var_cox_inv_dn0), (1.034943e-10 * locals.var_cox_inv_dn2), (1.034943e-10 * locals.var_cox_inv_dn4), (1.034943e-10 * locals.var_cox_inv_dn5), (1.034943e-10 * locals.var_cox_inv_dn6), (1.034943e-10 * locals.var_cox_inv_dn7), (1.034943e-10 * locals.var_cox_inv_dn8), (1.034943e-10 * locals.var_cox_inv_dn9), (1.034943e-10 * locals.var_cox_inv_dn10), (1.034943e-10 * locals.var_cox_inv_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63050_e97743;
        locals.var_t1_dn0 = assign63050_e97743_d_n0;
        locals.var_t1_dn2 = assign63050_e97743_d_n2;
        locals.var_t1_dn4 = assign63050_e97743_d_n4;
        locals.var_t1_dn5 = assign63050_e97743_d_n5;
        locals.var_t1_dn6 = assign63050_e97743_d_n6;
        locals.var_t1_dn7 = assign63050_e97743_d_n7;
        locals.var_t1_dn8 = assign63050_e97743_d_n8;
        locals.var_t1_dn9 = assign63050_e97743_d_n9;
        locals.var_t1_dn10 = assign63050_e97743_d_n10;
        locals.var_t1_dn13 = assign63050_e97743_d_n13;

        let (assign63060_e97752, assign63060_e97752_d_n0, assign63060_e97752_d_n2, assign63060_e97752_d_n4, assign63060_e97752_d_n5, assign63060_e97752_d_n6, assign63060_e97752_d_n7, assign63060_e97752_d_n8, assign63060_e97752_d_n9, assign63060_e97752_d_n10, assign63060_e97752_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign63060_e97752;
        locals.var_t2_dn0 = assign63060_e97752_d_n0;
        locals.var_t2_dn2 = assign63060_e97752_d_n2;
        locals.var_t2_dn4 = assign63060_e97752_d_n4;
        locals.var_t2_dn5 = assign63060_e97752_d_n5;
        locals.var_t2_dn6 = assign63060_e97752_d_n6;
        locals.var_t2_dn7 = assign63060_e97752_d_n7;
        locals.var_t2_dn8 = assign63060_e97752_d_n8;
        locals.var_t2_dn9 = assign63060_e97752_d_n9;
        locals.var_t2_dn10 = assign63060_e97752_d_n10;
        locals.var_t2_dn13 = assign63060_e97752_d_n13;

        let (assign63070_e97763, assign63070_e97763_d_n0, assign63070_e97763_d_n2, assign63070_e97763_d_n4, assign63070_e97763_d_n5, assign63070_e97763_d_n6, assign63070_e97763_d_n7, assign63070_e97763_d_n8, assign63070_e97763_d_n9, assign63070_e97763_d_n10, assign63070_e97763_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63070_e97761: f64 = (locals.var_lgatesm - p.p139);
        (assign63070_e97761, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign63070_e97763;
        locals.var_t3_dn0 = assign63070_e97763_d_n0;
        locals.var_t3_dn2 = assign63070_e97763_d_n2;
        locals.var_t3_dn4 = assign63070_e97763_d_n4;
        locals.var_t3_dn5 = assign63070_e97763_d_n5;
        locals.var_t3_dn6 = assign63070_e97763_d_n6;
        locals.var_t3_dn7 = assign63070_e97763_d_n7;
        locals.var_t3_dn8 = assign63070_e97763_d_n8;
        locals.var_t3_dn9 = assign63070_e97763_d_n9;
        locals.var_t3_dn10 = assign63070_e97763_d_n10;
        locals.var_t3_dn13 = assign63070_e97763_d_n13;

        let (assign63080_e97776, assign63080_e97776_d_n0, assign63080_e97776_d_n2, assign63080_e97776_d_n4, assign63080_e97776_d_n5, assign63080_e97776_d_n6, assign63080_e97776_d_n7, assign63080_e97776_d_n8, assign63080_e97776_d_n9, assign63080_e97776_d_n10, assign63080_e97776_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63080_e97773: f64 = (locals.var_t3 * locals.var_t3);
        let assign63080_e97774: f64 = (1.0 / assign63080_e97773);
        (assign63080_e97774, (-(((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (assign63080_e97773 * assign63080_e97773))), (-(((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (assign63080_e97773 * assign63080_e97773))), (-(((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (assign63080_e97773 * assign63080_e97773))), (-(((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (assign63080_e97773 * assign63080_e97773))), (-(((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (assign63080_e97773 * assign63080_e97773))), (-(((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (assign63080_e97773 * assign63080_e97773))), (-(((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (assign63080_e97773 * assign63080_e97773))), (-(((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (assign63080_e97773 * assign63080_e97773))), (-(((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (assign63080_e97773 * assign63080_e97773))), (-(((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13)) / (assign63080_e97773 * assign63080_e97773))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign63080_e97776;
        locals.var_t4_dn0 = assign63080_e97776_d_n0;
        locals.var_t4_dn2 = assign63080_e97776_d_n2;
        locals.var_t4_dn4 = assign63080_e97776_d_n4;
        locals.var_t4_dn5 = assign63080_e97776_d_n5;
        locals.var_t4_dn6 = assign63080_e97776_d_n6;
        locals.var_t4_dn7 = assign63080_e97776_d_n7;
        locals.var_t4_dn8 = assign63080_e97776_d_n8;
        locals.var_t4_dn9 = assign63080_e97776_d_n9;
        locals.var_t4_dn10 = assign63080_e97776_d_n10;
        locals.var_t4_dn13 = assign63080_e97776_d_n13;

        let (assign63090_e97795, assign63090_e97795_d_n0, assign63090_e97795_d_n2, assign63090_e97795_d_n4, assign63090_e97795_d_n5, assign63090_e97795_d_n6, assign63090_e97795_d_n7, assign63090_e97795_d_n8, assign63090_e97795_d_n9, assign63090_e97795_d_n10, assign63090_e97795_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63090_e97786: f64 = (p.p137 - locals.var_pb20b);
        let assign63090_e97787: f64 = (2.0 * assign63090_e97786);
        let assign63090_e97789: f64 = (assign63090_e97787 * locals.var_t1);
        let assign63090_e97791: f64 = (assign63090_e97789 * locals.var_t2);
        let assign63090_e97793: f64 = (assign63090_e97791 * locals.var_t4);
        (assign63090_e97793, (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign63090_e97787 * locals.var_t1_dn0)) * locals.var_t2) + (assign63090_e97789 * locals.var_t2_dn0)) * locals.var_t4) + (assign63090_e97791 * locals.var_t4_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign63090_e97787 * locals.var_t1_dn2)) * locals.var_t2) + (assign63090_e97789 * locals.var_t2_dn2)) * locals.var_t4) + (assign63090_e97791 * locals.var_t4_dn2)), (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign63090_e97787 * locals.var_t1_dn4)) * locals.var_t2) + (assign63090_e97789 * locals.var_t2_dn4)) * locals.var_t4) + (assign63090_e97791 * locals.var_t4_dn4)), (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign63090_e97787 * locals.var_t1_dn5)) * locals.var_t2) + (assign63090_e97789 * locals.var_t2_dn5)) * locals.var_t4) + (assign63090_e97791 * locals.var_t4_dn5)), (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign63090_e97787 * locals.var_t1_dn6)) * locals.var_t2) + (assign63090_e97789 * locals.var_t2_dn6)) * locals.var_t4) + (assign63090_e97791 * locals.var_t4_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign63090_e97787 * locals.var_t1_dn7)) * locals.var_t2) + (assign63090_e97789 * locals.var_t2_dn7)) * locals.var_t4) + (assign63090_e97791 * locals.var_t4_dn7)), (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign63090_e97787 * locals.var_t1_dn8)) * locals.var_t2) + (assign63090_e97789 * locals.var_t2_dn8)) * locals.var_t4) + (assign63090_e97791 * locals.var_t4_dn8)), (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign63090_e97787 * locals.var_t1_dn9)) * locals.var_t2) + (assign63090_e97789 * locals.var_t2_dn9)) * locals.var_t4) + (assign63090_e97791 * locals.var_t4_dn9)), (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign63090_e97787 * locals.var_t1_dn10)) * locals.var_t2) + (assign63090_e97789 * locals.var_t2_dn10)) * locals.var_t4) + (assign63090_e97791 * locals.var_t4_dn10)), (((((((2.0 * (-locals.var_pb20b_dn13)) * locals.var_t1) + (assign63090_e97787 * locals.var_t1_dn13)) * locals.var_t2) + (assign63090_e97789 * locals.var_t2_dn13)) * locals.var_t4) + (assign63090_e97791 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign63090_e97795;
        locals.var_t5_dn0 = assign63090_e97795_d_n0;
        locals.var_t5_dn2 = assign63090_e97795_d_n2;
        locals.var_t5_dn4 = assign63090_e97795_d_n4;
        locals.var_t5_dn5 = assign63090_e97795_d_n5;
        locals.var_t5_dn6 = assign63090_e97795_d_n6;
        locals.var_t5_dn7 = assign63090_e97795_d_n7;
        locals.var_t5_dn8 = assign63090_e97795_d_n8;
        locals.var_t5_dn9 = assign63090_e97795_d_n9;
        locals.var_t5_dn10 = assign63090_e97795_d_n10;
        locals.var_t5_dn13 = assign63090_e97795_d_n13;

        let (assign63100_e97806, assign63100_e97806_d_n0, assign63100_e97806_d_n2, assign63100_e97806_d_n4, assign63100_e97806_d_n5, assign63100_e97806_d_n6, assign63100_e97806_d_n7, assign63100_e97806_d_n8, assign63100_e97806_d_n9, assign63100_e97806_d_n10, assign63100_e97806_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63100_e97804: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        (assign63100_e97804, ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4)), ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5)), ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8)), ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9)), ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5_dn13 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn13)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn4, locals.var_dvth0_dn5, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn8, locals.var_dvth0_dn9, locals.var_dvth0_dn10, locals.var_dvth0_dn13,)
    }
};
        locals.var_dvth0 = assign63100_e97806;
        locals.var_dvth0_dn0 = assign63100_e97806_d_n0;
        locals.var_dvth0_dn2 = assign63100_e97806_d_n2;
        locals.var_dvth0_dn4 = assign63100_e97806_d_n4;
        locals.var_dvth0_dn5 = assign63100_e97806_d_n5;
        locals.var_dvth0_dn6 = assign63100_e97806_d_n6;
        locals.var_dvth0_dn7 = assign63100_e97806_d_n7;
        locals.var_dvth0_dn8 = assign63100_e97806_d_n8;
        locals.var_dvth0_dn9 = assign63100_e97806_d_n9;
        locals.var_dvth0_dn10 = assign63100_e97806_d_n10;
        locals.var_dvth0_dn13 = assign63100_e97806_d_n13;

        let (assign63110_e97819, assign63110_e97819_d_n0, assign63110_e97819_d_n2, assign63110_e97819_d_n4, assign63110_e97819_d_n5, assign63110_e97819_d_n6, assign63110_e97819_d_n7, assign63110_e97819_d_n8, assign63110_e97819_d_n9, assign63110_e97819_d_n10, assign63110_e97819_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63110_e97815: f64 = (locals.var_t5 * 0.5);
        let assign63110_e97817: f64 = (assign63110_e97815 / locals.var_sqrt_pbsum);
        (assign63110_e97817, ((((locals.var_t5_dn0 * 0.5) * locals.var_sqrt_pbsum) - (assign63110_e97815 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn2 * 0.5) * locals.var_sqrt_pbsum) - (assign63110_e97815 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn4 * 0.5) * locals.var_sqrt_pbsum) - (assign63110_e97815 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn5 * 0.5) * locals.var_sqrt_pbsum) - (assign63110_e97815 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn6 * 0.5) * locals.var_sqrt_pbsum) - (assign63110_e97815 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn7 * 0.5) * locals.var_sqrt_pbsum) - (assign63110_e97815 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn8 * 0.5) * locals.var_sqrt_pbsum) - (assign63110_e97815 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn9 * 0.5) * locals.var_sqrt_pbsum) - (assign63110_e97815 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn10 * 0.5) * locals.var_sqrt_pbsum) - (assign63110_e97815 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn13 * 0.5) * locals.var_sqrt_pbsum) - (assign63110_e97815 * locals.var_sqrt_pbsum_dn13)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign63110_e97819;
        locals.var_t6_dn0 = assign63110_e97819_d_n0;
        locals.var_t6_dn2 = assign63110_e97819_d_n2;
        locals.var_t6_dn4 = assign63110_e97819_d_n4;
        locals.var_t6_dn5 = assign63110_e97819_d_n5;
        locals.var_t6_dn6 = assign63110_e97819_d_n6;
        locals.var_t6_dn7 = assign63110_e97819_d_n7;
        locals.var_t6_dn8 = assign63110_e97819_d_n8;
        locals.var_t6_dn9 = assign63110_e97819_d_n9;
        locals.var_t6_dn10 = assign63110_e97819_d_n10;
        locals.var_t6_dn13 = assign63110_e97819_d_n13;

        let (assign63120_e97840, assign63120_e97840_d_n0, assign63120_e97840_d_n2, assign63120_e97840_d_n4, assign63120_e97840_d_n5, assign63120_e97840_d_n6, assign63120_e97840_d_n7, assign63120_e97840_d_n8, assign63120_e97840_d_n9, assign63120_e97840_d_n10, assign63120_e97840_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63120_e97829: f64 = (p.p137 - locals.var_pb20b);
        let assign63120_e97830: f64 = (2.0 * assign63120_e97829);
        let assign63120_e97832: f64 = (assign63120_e97830 * 1.034943e-10);
        let assign63120_e97834: f64 = (assign63120_e97832 * locals.var_t2);
        let assign63120_e97836: f64 = (assign63120_e97834 * locals.var_t4);
        let assign63120_e97838: f64 = (assign63120_e97836 * locals.var_sqrt_pbsum);
        (assign63120_e97838, ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign63120_e97832 * locals.var_t2_dn0)) * locals.var_t4) + (assign63120_e97834 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign63120_e97836 * locals.var_sqrt_pbsum_dn0)), ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign63120_e97832 * locals.var_t2_dn2)) * locals.var_t4) + (assign63120_e97834 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign63120_e97836 * locals.var_sqrt_pbsum_dn2)), ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign63120_e97832 * locals.var_t2_dn4)) * locals.var_t4) + (assign63120_e97834 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign63120_e97836 * locals.var_sqrt_pbsum_dn4)), ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign63120_e97832 * locals.var_t2_dn5)) * locals.var_t4) + (assign63120_e97834 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign63120_e97836 * locals.var_sqrt_pbsum_dn5)), ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign63120_e97832 * locals.var_t2_dn6)) * locals.var_t4) + (assign63120_e97834 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign63120_e97836 * locals.var_sqrt_pbsum_dn6)), ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign63120_e97832 * locals.var_t2_dn7)) * locals.var_t4) + (assign63120_e97834 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign63120_e97836 * locals.var_sqrt_pbsum_dn7)), ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign63120_e97832 * locals.var_t2_dn8)) * locals.var_t4) + (assign63120_e97834 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign63120_e97836 * locals.var_sqrt_pbsum_dn8)), ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign63120_e97832 * locals.var_t2_dn9)) * locals.var_t4) + (assign63120_e97834 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign63120_e97836 * locals.var_sqrt_pbsum_dn9)), ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign63120_e97832 * locals.var_t2_dn10)) * locals.var_t4) + (assign63120_e97834 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign63120_e97836 * locals.var_sqrt_pbsum_dn10)), ((((((((2.0 * (-locals.var_pb20b_dn13)) * 1.034943e-10) * locals.var_t2) + (assign63120_e97832 * locals.var_t2_dn13)) * locals.var_t4) + (assign63120_e97834 * locals.var_t4_dn13)) * locals.var_sqrt_pbsum) + (assign63120_e97836 * locals.var_sqrt_pbsum_dn13)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign63120_e97840;
        locals.var_t7_dn0 = assign63120_e97840_d_n0;
        locals.var_t7_dn2 = assign63120_e97840_d_n2;
        locals.var_t7_dn4 = assign63120_e97840_d_n4;
        locals.var_t7_dn5 = assign63120_e97840_d_n5;
        locals.var_t7_dn6 = assign63120_e97840_d_n6;
        locals.var_t7_dn7 = assign63120_e97840_d_n7;
        locals.var_t7_dn8 = assign63120_e97840_d_n8;
        locals.var_t7_dn9 = assign63120_e97840_d_n9;
        locals.var_t7_dn10 = assign63120_e97840_d_n10;
        locals.var_t7_dn13 = assign63120_e97840_d_n13;

        let (assign63130_e97858, assign63130_e97858_d_n0, assign63130_e97858_d_n2, assign63130_e97858_d_n4, assign63130_e97858_d_n5, assign63130_e97858_d_n6, assign63130_e97858_d_n7, assign63130_e97858_d_n8, assign63130_e97858_d_n9, assign63130_e97858_d_n10, assign63130_e97858_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63130_e97848: f64 = (-2.0);
        let assign63130_e97850: f64 = (assign63130_e97848 * locals.var_t1);
        let assign63130_e97852: f64 = (assign63130_e97850 * locals.var_t2);
        let assign63130_e97854: f64 = (assign63130_e97852 * locals.var_t4);
        let assign63130_e97856: f64 = (assign63130_e97854 * locals.var_sqrt_pbsum);
        (assign63130_e97856, (((((((assign63130_e97848 * locals.var_t1_dn0) * locals.var_t2) + (assign63130_e97850 * locals.var_t2_dn0)) * locals.var_t4) + (assign63130_e97852 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign63130_e97854 * locals.var_sqrt_pbsum_dn0)), (((((((assign63130_e97848 * locals.var_t1_dn2) * locals.var_t2) + (assign63130_e97850 * locals.var_t2_dn2)) * locals.var_t4) + (assign63130_e97852 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign63130_e97854 * locals.var_sqrt_pbsum_dn2)), (((((((assign63130_e97848 * locals.var_t1_dn4) * locals.var_t2) + (assign63130_e97850 * locals.var_t2_dn4)) * locals.var_t4) + (assign63130_e97852 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign63130_e97854 * locals.var_sqrt_pbsum_dn4)), (((((((assign63130_e97848 * locals.var_t1_dn5) * locals.var_t2) + (assign63130_e97850 * locals.var_t2_dn5)) * locals.var_t4) + (assign63130_e97852 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign63130_e97854 * locals.var_sqrt_pbsum_dn5)), (((((((assign63130_e97848 * locals.var_t1_dn6) * locals.var_t2) + (assign63130_e97850 * locals.var_t2_dn6)) * locals.var_t4) + (assign63130_e97852 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign63130_e97854 * locals.var_sqrt_pbsum_dn6)), (((((((assign63130_e97848 * locals.var_t1_dn7) * locals.var_t2) + (assign63130_e97850 * locals.var_t2_dn7)) * locals.var_t4) + (assign63130_e97852 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign63130_e97854 * locals.var_sqrt_pbsum_dn7)), (((((((assign63130_e97848 * locals.var_t1_dn8) * locals.var_t2) + (assign63130_e97850 * locals.var_t2_dn8)) * locals.var_t4) + (assign63130_e97852 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign63130_e97854 * locals.var_sqrt_pbsum_dn8)), (((((((assign63130_e97848 * locals.var_t1_dn9) * locals.var_t2) + (assign63130_e97850 * locals.var_t2_dn9)) * locals.var_t4) + (assign63130_e97852 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign63130_e97854 * locals.var_sqrt_pbsum_dn9)), (((((((assign63130_e97848 * locals.var_t1_dn10) * locals.var_t2) + (assign63130_e97850 * locals.var_t2_dn10)) * locals.var_t4) + (assign63130_e97852 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign63130_e97854 * locals.var_sqrt_pbsum_dn10)), (((((((assign63130_e97848 * locals.var_t1_dn13) * locals.var_t2) + (assign63130_e97850 * locals.var_t2_dn13)) * locals.var_t4) + (assign63130_e97852 * locals.var_t4_dn13)) * locals.var_sqrt_pbsum) + (assign63130_e97854 * locals.var_sqrt_pbsum_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign63130_e97858;
        locals.var_t8_dn0 = assign63130_e97858_d_n0;
        locals.var_t8_dn2 = assign63130_e97858_d_n2;
        locals.var_t8_dn4 = assign63130_e97858_d_n4;
        locals.var_t8_dn5 = assign63130_e97858_d_n5;
        locals.var_t8_dn6 = assign63130_e97858_d_n6;
        locals.var_t8_dn7 = assign63130_e97858_d_n7;
        locals.var_t8_dn8 = assign63130_e97858_d_n8;
        locals.var_t8_dn9 = assign63130_e97858_d_n9;
        locals.var_t8_dn10 = assign63130_e97858_d_n10;
        locals.var_t8_dn13 = assign63130_e97858_d_n13;

        let (assign63140_e97867, assign63140_e97867_d_n0, assign63140_e97867_d_n2, assign63140_e97867_d_n4, assign63140_e97867_d_n5, assign63140_e97867_d_n6, assign63140_e97867_d_n7, assign63140_e97867_d_n8, assign63140_e97867_d_n9, assign63140_e97867_d_n10, assign63140_e97867_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (locals.var_uc_scsti1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign63140_e97867;
        locals.var_t4_dn0 = assign63140_e97867_d_n0;
        locals.var_t4_dn2 = assign63140_e97867_d_n2;
        locals.var_t4_dn4 = assign63140_e97867_d_n4;
        locals.var_t4_dn5 = assign63140_e97867_d_n5;
        locals.var_t4_dn6 = assign63140_e97867_d_n6;
        locals.var_t4_dn7 = assign63140_e97867_d_n7;
        locals.var_t4_dn8 = assign63140_e97867_d_n8;
        locals.var_t4_dn9 = assign63140_e97867_d_n9;
        locals.var_t4_dn10 = assign63140_e97867_d_n10;
        locals.var_t4_dn13 = assign63140_e97867_d_n13;

        let (assign63150_e97876, assign63150_e97876_d_n0, assign63150_e97876_d_n2, assign63150_e97876_d_n4, assign63150_e97876_d_n5, assign63150_e97876_d_n6, assign63150_e97876_d_n7, assign63150_e97876_d_n8, assign63150_e97876_d_n9, assign63150_e97876_d_n10, assign63150_e97876_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (locals.var_uc_scsti2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign63150_e97876;
        locals.var_t6_dn0 = assign63150_e97876_d_n0;
        locals.var_t6_dn2 = assign63150_e97876_d_n2;
        locals.var_t6_dn4 = assign63150_e97876_d_n4;
        locals.var_t6_dn5 = assign63150_e97876_d_n5;
        locals.var_t6_dn6 = assign63150_e97876_d_n6;
        locals.var_t6_dn7 = assign63150_e97876_d_n7;
        locals.var_t6_dn8 = assign63150_e97876_d_n8;
        locals.var_t6_dn9 = assign63150_e97876_d_n9;
        locals.var_t6_dn10 = assign63150_e97876_d_n10;
        locals.var_t6_dn13 = assign63150_e97876_d_n13;

        let (assign63160_e97889, assign63160_e97889_d_n0, assign63160_e97889_d_n2, assign63160_e97889_d_n4, assign63160_e97889_d_n5, assign63160_e97889_d_n6, assign63160_e97889_d_n7, assign63160_e97889_d_n8, assign63160_e97889_d_n9, assign63160_e97889_d_n10, assign63160_e97889_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63160_e97886: f64 = (locals.var_t6 * locals.var_vdsz__blk439);
        let assign63160_e97887: f64 = (locals.var_t4 + assign63160_e97886);
        (assign63160_e97887, (locals.var_t4_dn0 + ((locals.var_t6_dn0 * locals.var_vdsz__blk439) + (locals.var_t6 * locals.var_vdsz__blk439_dn0))), (locals.var_t4_dn2 + ((locals.var_t6_dn2 * locals.var_vdsz__blk439) + (locals.var_t6 * locals.var_vdsz__blk439_dn2))), (locals.var_t4_dn4 + ((locals.var_t6_dn4 * locals.var_vdsz__blk439) + (locals.var_t6 * locals.var_vdsz__blk439_dn4))), (locals.var_t4_dn5 + ((locals.var_t6_dn5 * locals.var_vdsz__blk439) + (locals.var_t6 * locals.var_vdsz__blk439_dn5))), (locals.var_t4_dn6 + ((locals.var_t6_dn6 * locals.var_vdsz__blk439) + (locals.var_t6 * locals.var_vdsz__blk439_dn6))), (locals.var_t4_dn7 + ((locals.var_t6_dn7 * locals.var_vdsz__blk439) + (locals.var_t6 * locals.var_vdsz__blk439_dn7))), (locals.var_t4_dn8 + ((locals.var_t6_dn8 * locals.var_vdsz__blk439) + (locals.var_t6 * locals.var_vdsz__blk439_dn8))), (locals.var_t4_dn9 + ((locals.var_t6_dn9 * locals.var_vdsz__blk439) + (locals.var_t6 * locals.var_vdsz__blk439_dn9))), (locals.var_t4_dn10 + ((locals.var_t6_dn10 * locals.var_vdsz__blk439) + (locals.var_t6 * locals.var_vdsz__blk439_dn10))), (locals.var_t4_dn13 + ((locals.var_t6_dn13 * locals.var_vdsz__blk439) + (locals.var_t6 * locals.var_vdsz__blk439_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63160_e97889;
        locals.var_t1_dn0 = assign63160_e97889_d_n0;
        locals.var_t1_dn2 = assign63160_e97889_d_n2;
        locals.var_t1_dn4 = assign63160_e97889_d_n4;
        locals.var_t1_dn5 = assign63160_e97889_d_n5;
        locals.var_t1_dn6 = assign63160_e97889_d_n6;
        locals.var_t1_dn7 = assign63160_e97889_d_n7;
        locals.var_t1_dn8 = assign63160_e97889_d_n8;
        locals.var_t1_dn9 = assign63160_e97889_d_n9;
        locals.var_t1_dn10 = assign63160_e97889_d_n10;
        locals.var_t1_dn13 = assign63160_e97889_d_n13;

        let (assign63170_e97900, assign63170_e97900_d_n0, assign63170_e97900_d_n2, assign63170_e97900_d_n4, assign63170_e97900_d_n5, assign63170_e97900_d_n6, assign63170_e97900_d_n7, assign63170_e97900_d_n8, assign63170_e97900_d_n9, assign63170_e97900_d_n10, assign63170_e97900_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63170_e97898: f64 = (locals.var_dvth0 * locals.var_t1);
        (assign63170_e97898, ((locals.var_dvth0_dn0 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn0)), ((locals.var_dvth0_dn2 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn2)), ((locals.var_dvth0_dn4 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn4)), ((locals.var_dvth0_dn5 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn5)), ((locals.var_dvth0_dn6 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn6)), ((locals.var_dvth0_dn7 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn7)), ((locals.var_dvth0_dn8 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn8)), ((locals.var_dvth0_dn9 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn9)), ((locals.var_dvth0_dn10 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn10)), ((locals.var_dvth0_dn13 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn13)),)
    } else {
        (locals.var_dvthscsti, locals.var_dvthscsti_dn0, locals.var_dvthscsti_dn2, locals.var_dvthscsti_dn4, locals.var_dvthscsti_dn5, locals.var_dvthscsti_dn6, locals.var_dvthscsti_dn7, locals.var_dvthscsti_dn8, locals.var_dvthscsti_dn9, locals.var_dvthscsti_dn10, locals.var_dvthscsti_dn13,)
    }
};
        locals.var_dvthscsti = assign63170_e97900;
        locals.var_dvthscsti_dn0 = assign63170_e97900_d_n0;
        locals.var_dvthscsti_dn2 = assign63170_e97900_d_n2;
        locals.var_dvthscsti_dn4 = assign63170_e97900_d_n4;
        locals.var_dvthscsti_dn5 = assign63170_e97900_d_n5;
        locals.var_dvthscsti_dn6 = assign63170_e97900_d_n6;
        locals.var_dvthscsti_dn7 = assign63170_e97900_d_n7;
        locals.var_dvthscsti_dn8 = assign63170_e97900_d_n8;
        locals.var_dvthscsti_dn9 = assign63170_e97900_d_n9;
        locals.var_dvthscsti_dn10 = assign63170_e97900_d_n10;
        locals.var_dvthscsti_dn13 = assign63170_e97900_d_n13;

        let (assign63180_e97913, assign63180_e97913_d_n0, assign63180_e97913_d_n2, assign63180_e97913_d_n4, assign63180_e97913_d_n5, assign63180_e97913_d_n6, assign63180_e97913_d_n7, assign63180_e97913_d_n8, assign63180_e97913_d_n9, assign63180_e97913_d_n10, assign63180_e97913_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63180_e97910: f64 = (p.p213 * locals.var_vds);
        let assign63180_e97911: f64 = (locals.var_uc_vthsti - assign63180_e97910);
        (assign63180_e97911, (-(p.p213 * locals.var_vds_dn0)), (-(p.p213 * locals.var_vds_dn2)), (-(p.p213 * locals.var_vds_dn4)), (-(p.p213 * locals.var_vds_dn5)), (-(p.p213 * locals.var_vds_dn6)), (-(p.p213 * locals.var_vds_dn7)), (-(p.p213 * locals.var_vds_dn8)), (-(p.p213 * locals.var_vds_dn9)), (-(p.p213 * locals.var_vds_dn10)), (-(p.p213 * locals.var_vds_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63180_e97913;
        locals.var_t1_dn0 = assign63180_e97913_d_n0;
        locals.var_t1_dn2 = assign63180_e97913_d_n2;
        locals.var_t1_dn4 = assign63180_e97913_d_n4;
        locals.var_t1_dn5 = assign63180_e97913_d_n5;
        locals.var_t1_dn6 = assign63180_e97913_d_n6;
        locals.var_t1_dn7 = assign63180_e97913_d_n7;
        locals.var_t1_dn8 = assign63180_e97913_d_n8;
        locals.var_t1_dn9 = assign63180_e97913_d_n9;
        locals.var_t1_dn10 = assign63180_e97913_d_n10;
        locals.var_t1_dn13 = assign63180_e97913_d_n13;

        let (assign63190_e97928, assign63190_e97928_d_n0, assign63190_e97928_d_n2, assign63190_e97928_d_n4, assign63190_e97928_d_n5, assign63190_e97928_d_n6, assign63190_e97928_d_n7, assign63190_e97928_d_n8, assign63190_e97928_d_n9, assign63190_e97928_d_n10, assign63190_e97928_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63190_e97922: f64 = (locals.var_vgsz__blk440 - locals.var_vfb);
        let assign63190_e97924: f64 = (assign63190_e97922 + locals.var_t1);
        let assign63190_e97926: f64 = (assign63190_e97924 + locals.var_dvthscsti);
        (assign63190_e97926, ((locals.var_vgsz__blk440_dn0 + locals.var_t1_dn0) + locals.var_dvthscsti_dn0), ((locals.var_vgsz__blk440_dn2 + locals.var_t1_dn2) + locals.var_dvthscsti_dn2), ((locals.var_vgsz__blk440_dn4 + locals.var_t1_dn4) + locals.var_dvthscsti_dn4), ((locals.var_vgsz__blk440_dn5 + locals.var_t1_dn5) + locals.var_dvthscsti_dn5), ((locals.var_vgsz__blk440_dn6 + locals.var_t1_dn6) + locals.var_dvthscsti_dn6), ((locals.var_vgsz__blk440_dn7 + locals.var_t1_dn7) + locals.var_dvthscsti_dn7), ((locals.var_vgsz__blk440_dn8 + locals.var_t1_dn8) + locals.var_dvthscsti_dn8), ((locals.var_vgsz__blk440_dn9 + locals.var_t1_dn9) + locals.var_dvthscsti_dn9), ((locals.var_vgsz__blk440_dn10 + locals.var_t1_dn10) + locals.var_dvthscsti_dn10), ((locals.var_vgsz__blk440_dn13 + locals.var_t1_dn13) + locals.var_dvthscsti_dn13),)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn4, locals.var_vgssti_dn5, locals.var_vgssti_dn6, locals.var_vgssti_dn7, locals.var_vgssti_dn8, locals.var_vgssti_dn9, locals.var_vgssti_dn10, locals.var_vgssti_dn13,)
    }
};
        locals.var_vgssti = assign63190_e97928;
        locals.var_vgssti_dn0 = assign63190_e97928_d_n0;
        locals.var_vgssti_dn2 = assign63190_e97928_d_n2;
        locals.var_vgssti_dn4 = assign63190_e97928_d_n4;
        locals.var_vgssti_dn5 = assign63190_e97928_d_n5;
        locals.var_vgssti_dn6 = assign63190_e97928_d_n6;
        locals.var_vgssti_dn7 = assign63190_e97928_d_n7;
        locals.var_vgssti_dn8 = assign63190_e97928_d_n8;
        locals.var_vgssti_dn9 = assign63190_e97928_d_n9;
        locals.var_vgssti_dn10 = assign63190_e97928_d_n10;
        locals.var_vgssti_dn13 = assign63190_e97928_d_n13;

        let (assign63200_e97941, assign63200_e97941_d_n0, assign63200_e97941_d_n2, assign63200_e97941_d_n4, assign63200_e97941_d_n5, assign63200_e97941_d_n6, assign63200_e97941_d_n7, assign63200_e97941_d_n8, assign63200_e97941_d_n9, assign63200_e97941_d_n10, assign63200_e97941_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63200_e97937: f64 = (locals.var_costi0_p2 * locals.var_cox_inv);
        let assign63200_e97939: f64 = (assign63200_e97937 * locals.var_cox_inv);
        (assign63200_e97939, ((((locals.var_costi0_p2_dn0 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn0)) * locals.var_cox_inv) + (assign63200_e97937 * locals.var_cox_inv_dn0)), ((((locals.var_costi0_p2_dn2 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn2)) * locals.var_cox_inv) + (assign63200_e97937 * locals.var_cox_inv_dn2)), ((((locals.var_costi0_p2_dn4 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn4)) * locals.var_cox_inv) + (assign63200_e97937 * locals.var_cox_inv_dn4)), ((((locals.var_costi0_p2_dn5 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn5)) * locals.var_cox_inv) + (assign63200_e97937 * locals.var_cox_inv_dn5)), ((((locals.var_costi0_p2_dn6 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn6)) * locals.var_cox_inv) + (assign63200_e97937 * locals.var_cox_inv_dn6)), ((((locals.var_costi0_p2_dn7 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn7)) * locals.var_cox_inv) + (assign63200_e97937 * locals.var_cox_inv_dn7)), ((((locals.var_costi0_p2_dn8 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn8)) * locals.var_cox_inv) + (assign63200_e97937 * locals.var_cox_inv_dn8)), ((((locals.var_costi0_p2_dn9 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn9)) * locals.var_cox_inv) + (assign63200_e97937 * locals.var_cox_inv_dn9)), ((((locals.var_costi0_p2_dn10 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn10)) * locals.var_cox_inv) + (assign63200_e97937 * locals.var_cox_inv_dn10)), ((((locals.var_costi0_p2_dn13 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn13)) * locals.var_cox_inv) + (assign63200_e97937 * locals.var_cox_inv_dn13)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn4, locals.var_costi3_dn5, locals.var_costi3_dn6, locals.var_costi3_dn7, locals.var_costi3_dn8, locals.var_costi3_dn9, locals.var_costi3_dn10, locals.var_costi3_dn13,)
    }
};
        locals.var_costi3 = assign63200_e97941;
        locals.var_costi3_dn0 = assign63200_e97941_d_n0;
        locals.var_costi3_dn2 = assign63200_e97941_d_n2;
        locals.var_costi3_dn4 = assign63200_e97941_d_n4;
        locals.var_costi3_dn5 = assign63200_e97941_d_n5;
        locals.var_costi3_dn6 = assign63200_e97941_d_n6;
        locals.var_costi3_dn7 = assign63200_e97941_d_n7;
        locals.var_costi3_dn8 = assign63200_e97941_d_n8;
        locals.var_costi3_dn9 = assign63200_e97941_d_n9;
        locals.var_costi3_dn10 = assign63200_e97941_d_n10;
        locals.var_costi3_dn13 = assign63200_e97941_d_n13;

        let (assign63210_e97954, assign63210_e97954_d_n0, assign63210_e97954_d_n2, assign63210_e97954_d_n4, assign63210_e97954_d_n5, assign63210_e97954_d_n6, assign63210_e97954_d_n7, assign63210_e97954_d_n8, assign63210_e97954_d_n9, assign63210_e97954_d_n10, assign63210_e97954_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63210_e97950: f64 = (locals.var_costi3 * locals.var_beta);
        let assign63210_e97952: f64 = (assign63210_e97950 * 0.5);
        (assign63210_e97952, (((locals.var_costi3_dn0 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn0)) * 0.5), (((locals.var_costi3_dn2 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn2)) * 0.5), (((locals.var_costi3_dn4 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn4)) * 0.5), (((locals.var_costi3_dn5 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn5)) * 0.5), (((locals.var_costi3_dn6 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn6)) * 0.5), (((locals.var_costi3_dn7 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn7)) * 0.5), (((locals.var_costi3_dn8 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn8)) * 0.5), (((locals.var_costi3_dn9 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn9)) * 0.5), (((locals.var_costi3_dn10 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn10)) * 0.5), (((locals.var_costi3_dn13 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn13)) * 0.5),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn4, locals.var_costi4_dn5, locals.var_costi4_dn6, locals.var_costi4_dn7, locals.var_costi4_dn8, locals.var_costi4_dn9, locals.var_costi4_dn10, locals.var_costi4_dn13,)
    }
};
        locals.var_costi4 = assign63210_e97954;
        locals.var_costi4_dn0 = assign63210_e97954_d_n0;
        locals.var_costi4_dn2 = assign63210_e97954_d_n2;
        locals.var_costi4_dn4 = assign63210_e97954_d_n4;
        locals.var_costi4_dn5 = assign63210_e97954_d_n5;
        locals.var_costi4_dn6 = assign63210_e97954_d_n6;
        locals.var_costi4_dn7 = assign63210_e97954_d_n7;
        locals.var_costi4_dn8 = assign63210_e97954_d_n8;
        locals.var_costi4_dn9 = assign63210_e97954_d_n9;
        locals.var_costi4_dn10 = assign63210_e97954_d_n10;
        locals.var_costi4_dn13 = assign63210_e97954_d_n13;

        let (assign63220_e97967, assign63220_e97967_d_n0, assign63220_e97967_d_n2, assign63220_e97967_d_n4, assign63220_e97967_d_n5, assign63220_e97967_d_n6, assign63220_e97967_d_n7, assign63220_e97967_d_n8, assign63220_e97967_d_n9, assign63220_e97967_d_n10, assign63220_e97967_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63220_e97963: f64 = (locals.var_costi4 * locals.var_beta);
        let assign63220_e97965: f64 = (assign63220_e97963 * 2.0);
        (assign63220_e97965, (((locals.var_costi4_dn0 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn0)) * 2.0), (((locals.var_costi4_dn2 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn2)) * 2.0), (((locals.var_costi4_dn4 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn4)) * 2.0), (((locals.var_costi4_dn5 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn5)) * 2.0), (((locals.var_costi4_dn6 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn6)) * 2.0), (((locals.var_costi4_dn7 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn7)) * 2.0), (((locals.var_costi4_dn8 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn8)) * 2.0), (((locals.var_costi4_dn9 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn9)) * 2.0), (((locals.var_costi4_dn10 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn10)) * 2.0), (((locals.var_costi4_dn13 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn13)) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn4, locals.var_costi5_dn5, locals.var_costi5_dn6, locals.var_costi5_dn7, locals.var_costi5_dn8, locals.var_costi5_dn9, locals.var_costi5_dn10, locals.var_costi5_dn13,)
    }
};
        locals.var_costi5 = assign63220_e97967;
        locals.var_costi5_dn0 = assign63220_e97967_d_n0;
        locals.var_costi5_dn2 = assign63220_e97967_d_n2;
        locals.var_costi5_dn4 = assign63220_e97967_d_n4;
        locals.var_costi5_dn5 = assign63220_e97967_d_n5;
        locals.var_costi5_dn6 = assign63220_e97967_d_n6;
        locals.var_costi5_dn7 = assign63220_e97967_d_n7;
        locals.var_costi5_dn8 = assign63220_e97967_d_n8;
        locals.var_costi5_dn9 = assign63220_e97967_d_n9;
        locals.var_costi5_dn10 = assign63220_e97967_d_n10;
        locals.var_costi5_dn13 = assign63220_e97967_d_n13;

        let (assign63230_e97978, assign63230_e97978_d_n0, assign63230_e97978_d_n2, assign63230_e97978_d_n4, assign63230_e97978_d_n5, assign63230_e97978_d_n6, assign63230_e97978_d_n7, assign63230_e97978_d_n8, assign63230_e97978_d_n9, assign63230_e97978_d_n10, assign63230_e97978_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63230_e97976: f64 = (locals.var_beta * 0.25);
        (assign63230_e97976, (locals.var_beta_dn0 * 0.25), (locals.var_beta_dn2 * 0.25), (locals.var_beta_dn4 * 0.25), (locals.var_beta_dn5 * 0.25), (locals.var_beta_dn6 * 0.25), (locals.var_beta_dn7 * 0.25), (locals.var_beta_dn8 * 0.25), (locals.var_beta_dn9 * 0.25), (locals.var_beta_dn10 * 0.25), (locals.var_beta_dn13 * 0.25),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign63230_e97978;
        locals.var_t11_dn0 = assign63230_e97978_d_n0;
        locals.var_t11_dn2 = assign63230_e97978_d_n2;
        locals.var_t11_dn4 = assign63230_e97978_d_n4;
        locals.var_t11_dn5 = assign63230_e97978_d_n5;
        locals.var_t11_dn6 = assign63230_e97978_d_n6;
        locals.var_t11_dn7 = assign63230_e97978_d_n7;
        locals.var_t11_dn8 = assign63230_e97978_d_n8;
        locals.var_t11_dn9 = assign63230_e97978_d_n9;
        locals.var_t11_dn10 = assign63230_e97978_d_n10;
        locals.var_t11_dn13 = assign63230_e97978_d_n13;

    }

    pub(super) fn stamp_transient_block_214(
        locals: &mut StampLocals,
    ) {
        let (assign63240_e97999, assign63240_e97999_d_n0, assign63240_e97999_d_n2, assign63240_e97999_d_n4, assign63240_e97999_d_n5, assign63240_e97999_d_n6, assign63240_e97999_d_n7, assign63240_e97999_d_n8, assign63240_e97999_d_n9, assign63240_e97999_d_n10, assign63240_e97999_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63240_e97988: f64 = (locals.var_costi3 * locals.var_t11);
        let assign63240_e97989: f64 = (locals.var_beta_inv - assign63240_e97988);
        let assign63240_e97991: f64 = (assign63240_e97989 + locals.var_vfb);
        let assign63240_e97993: f64 = (assign63240_e97991 - locals.var_uc_vthsti);
        let assign63240_e97995: f64 = (assign63240_e97993 - locals.var_dvthscsti);
        let assign63240_e97997: f64 = (assign63240_e97995 + 1e-25);
        (assign63240_e97997, ((locals.var_beta_inv_dn0 - ((locals.var_costi3_dn0 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn0))) - locals.var_dvthscsti_dn0), ((locals.var_beta_inv_dn2 - ((locals.var_costi3_dn2 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn2))) - locals.var_dvthscsti_dn2), ((locals.var_beta_inv_dn4 - ((locals.var_costi3_dn4 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn4))) - locals.var_dvthscsti_dn4), ((locals.var_beta_inv_dn5 - ((locals.var_costi3_dn5 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn5))) - locals.var_dvthscsti_dn5), ((locals.var_beta_inv_dn6 - ((locals.var_costi3_dn6 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn6))) - locals.var_dvthscsti_dn6), ((locals.var_beta_inv_dn7 - ((locals.var_costi3_dn7 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn7))) - locals.var_dvthscsti_dn7), ((locals.var_beta_inv_dn8 - ((locals.var_costi3_dn8 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn8))) - locals.var_dvthscsti_dn8), ((locals.var_beta_inv_dn9 - ((locals.var_costi3_dn9 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn9))) - locals.var_dvthscsti_dn9), ((locals.var_beta_inv_dn10 - ((locals.var_costi3_dn10 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn10))) - locals.var_dvthscsti_dn10), ((locals.var_beta_inv_dn13 - ((locals.var_costi3_dn13 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn13))) - locals.var_dvthscsti_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign63240_e97999;
        locals.var_t10_dn0 = assign63240_e97999_d_n0;
        locals.var_t10_dn2 = assign63240_e97999_d_n2;
        locals.var_t10_dn4 = assign63240_e97999_d_n4;
        locals.var_t10_dn5 = assign63240_e97999_d_n5;
        locals.var_t10_dn6 = assign63240_e97999_d_n6;
        locals.var_t10_dn7 = assign63240_e97999_d_n7;
        locals.var_t10_dn8 = assign63240_e97999_d_n8;
        locals.var_t10_dn9 = assign63240_e97999_d_n9;
        locals.var_t10_dn10 = assign63240_e97999_d_n10;
        locals.var_t10_dn13 = assign63240_e97999_d_n13;

        let (assign63250_e98012, assign63250_e98012_d_n0, assign63250_e98012_d_n2, assign63250_e98012_d_n4, assign63250_e98012_d_n5, assign63250_e98012_d_n6, assign63250_e98012_d_n7, assign63250_e98012_d_n8, assign63250_e98012_d_n9, assign63250_e98012_d_n10, assign63250_e98012_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63250_e98008: f64 = (locals.var_vgsz__blk440 - locals.var_t10);
        let assign63250_e98010: f64 = (assign63250_e98008 - 0.005);
        (assign63250_e98010, (locals.var_vgsz__blk440_dn0 - locals.var_t10_dn0), (locals.var_vgsz__blk440_dn2 - locals.var_t10_dn2), (locals.var_vgsz__blk440_dn4 - locals.var_t10_dn4), (locals.var_vgsz__blk440_dn5 - locals.var_t10_dn5), (locals.var_vgsz__blk440_dn6 - locals.var_t10_dn6), (locals.var_vgsz__blk440_dn7 - locals.var_t10_dn7), (locals.var_vgsz__blk440_dn8 - locals.var_t10_dn8), (locals.var_vgsz__blk440_dn9 - locals.var_t10_dn9), (locals.var_vgsz__blk440_dn10 - locals.var_t10_dn10), (locals.var_vgsz__blk440_dn13 - locals.var_t10_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63250_e98012;
        locals.var_t1_dn0 = assign63250_e98012_d_n0;
        locals.var_t1_dn2 = assign63250_e98012_d_n2;
        locals.var_t1_dn4 = assign63250_e98012_d_n4;
        locals.var_t1_dn5 = assign63250_e98012_d_n5;
        locals.var_t1_dn6 = assign63250_e98012_d_n6;
        locals.var_t1_dn7 = assign63250_e98012_d_n7;
        locals.var_t1_dn8 = assign63250_e98012_d_n8;
        locals.var_t1_dn9 = assign63250_e98012_d_n9;
        locals.var_t1_dn10 = assign63250_e98012_d_n10;
        locals.var_t1_dn13 = assign63250_e98012_d_n13;

        let (assign63260_e98027, assign63260_e98027_d_n0, assign63260_e98027_d_n2, assign63260_e98027_d_n4, assign63260_e98027_d_n5, assign63260_e98027_d_n6, assign63260_e98027_d_n7, assign63260_e98027_d_n8, assign63260_e98027_d_n9, assign63260_e98027_d_n10, assign63260_e98027_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let (assign63260_e98025,) = {
            if (locals.var_t10 >= 0.0) {
                (1.0,)
            } else {
                let assign63260_e98024: f64 = (-1.0);
                (assign63260_e98024,)
            }
        };
        (assign63260_e98025, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign63260_e98027;
        locals.var_t0_dn0 = assign63260_e98027_d_n0;
        locals.var_t0_dn2 = assign63260_e98027_d_n2;
        locals.var_t0_dn4 = assign63260_e98027_d_n4;
        locals.var_t0_dn5 = assign63260_e98027_d_n5;
        locals.var_t0_dn6 = assign63260_e98027_d_n6;
        locals.var_t0_dn7 = assign63260_e98027_d_n7;
        locals.var_t0_dn8 = assign63260_e98027_d_n8;
        locals.var_t0_dn9 = assign63260_e98027_d_n9;
        locals.var_t0_dn10 = assign63260_e98027_d_n10;
        locals.var_t0_dn13 = assign63260_e98027_d_n13;

        let (assign63270_e98047, assign63270_e98047_d_n0, assign63270_e98047_d_n2, assign63270_e98047_d_n4, assign63270_e98047_d_n5, assign63270_e98047_d_n6, assign63270_e98047_d_n7, assign63270_e98047_d_n8, assign63270_e98047_d_n9, assign63270_e98047_d_n10, assign63270_e98047_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63270_e98036: f64 = (locals.var_t1 * locals.var_t1);
        let assign63270_e98039: f64 = (locals.var_t0 * 4.0);
        let assign63270_e98041: f64 = (assign63270_e98039 * locals.var_t10);
        let assign63270_e98043: f64 = (assign63270_e98041 * 0.005);
        let assign63270_e98044: f64 = (assign63270_e98036 + assign63270_e98043);
        let assign63270_e98045: f64 = (assign63270_e98044).sqrt();
        (assign63270_e98045, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((locals.var_t0_dn0 * 4.0) * locals.var_t10) + (assign63270_e98039 * locals.var_t10_dn0)) * 0.005)) / (2.0 * assign63270_e98045)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((locals.var_t0_dn2 * 4.0) * locals.var_t10) + (assign63270_e98039 * locals.var_t10_dn2)) * 0.005)) / (2.0 * assign63270_e98045)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((locals.var_t0_dn4 * 4.0) * locals.var_t10) + (assign63270_e98039 * locals.var_t10_dn4)) * 0.005)) / (2.0 * assign63270_e98045)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((locals.var_t0_dn5 * 4.0) * locals.var_t10) + (assign63270_e98039 * locals.var_t10_dn5)) * 0.005)) / (2.0 * assign63270_e98045)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((locals.var_t0_dn6 * 4.0) * locals.var_t10) + (assign63270_e98039 * locals.var_t10_dn6)) * 0.005)) / (2.0 * assign63270_e98045)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + ((((locals.var_t0_dn7 * 4.0) * locals.var_t10) + (assign63270_e98039 * locals.var_t10_dn7)) * 0.005)) / (2.0 * assign63270_e98045)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((locals.var_t0_dn8 * 4.0) * locals.var_t10) + (assign63270_e98039 * locals.var_t10_dn8)) * 0.005)) / (2.0 * assign63270_e98045)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + ((((locals.var_t0_dn9 * 4.0) * locals.var_t10) + (assign63270_e98039 * locals.var_t10_dn9)) * 0.005)) / (2.0 * assign63270_e98045)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((locals.var_t0_dn10 * 4.0) * locals.var_t10) + (assign63270_e98039 * locals.var_t10_dn10)) * 0.005)) / (2.0 * assign63270_e98045)), ((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) + ((((locals.var_t0_dn13 * 4.0) * locals.var_t10) + (assign63270_e98039 * locals.var_t10_dn13)) * 0.005)) / (2.0 * assign63270_e98045)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign63270_e98047;
        locals.var_t2_dn0 = assign63270_e98047_d_n0;
        locals.var_t2_dn2 = assign63270_e98047_d_n2;
        locals.var_t2_dn4 = assign63270_e98047_d_n4;
        locals.var_t2_dn5 = assign63270_e98047_d_n5;
        locals.var_t2_dn6 = assign63270_e98047_d_n6;
        locals.var_t2_dn7 = assign63270_e98047_d_n7;
        locals.var_t2_dn8 = assign63270_e98047_d_n8;
        locals.var_t2_dn9 = assign63270_e98047_d_n9;
        locals.var_t2_dn10 = assign63270_e98047_d_n10;
        locals.var_t2_dn13 = assign63270_e98047_d_n13;

        let (assign63280_e98070, assign63280_e98070_d_n0, assign63280_e98070_d_n2, assign63280_e98070_d_n4, assign63280_e98070_d_n5, assign63280_e98070_d_n6, assign63280_e98070_d_n7, assign63280_e98070_d_n8, assign63280_e98070_d_n9, assign63280_e98070_d_n10, assign63280_e98070_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63280_e98058: f64 = (locals.var_t1 + locals.var_t2);
        let assign63280_e98059: f64 = (0.5 * assign63280_e98058);
        let assign63280_e98060: f64 = (locals.var_t10 + assign63280_e98059);
        let assign63280_e98062: f64 = (assign63280_e98060 - locals.var_vfb);
        let assign63280_e98064: f64 = (assign63280_e98062 + locals.var_uc_vthsti);
        let assign63280_e98066: f64 = (assign63280_e98064 + locals.var_dvthscsti);
        let assign63280_e98068: f64 = (assign63280_e98066 - locals.var_vbsz__blk438);
        (assign63280_e98068, (((locals.var_t10_dn0 + (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0))) + locals.var_dvthscsti_dn0) - locals.var_vbsz__blk438_dn0), (((locals.var_t10_dn2 + (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2))) + locals.var_dvthscsti_dn2) - locals.var_vbsz__blk438_dn2), (((locals.var_t10_dn4 + (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4))) + locals.var_dvthscsti_dn4) - locals.var_vbsz__blk438_dn4), (((locals.var_t10_dn5 + (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5))) + locals.var_dvthscsti_dn5) - locals.var_vbsz__blk438_dn5), (((locals.var_t10_dn6 + (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6))) + locals.var_dvthscsti_dn6) - locals.var_vbsz__blk438_dn6), (((locals.var_t10_dn7 + (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7))) + locals.var_dvthscsti_dn7) - locals.var_vbsz__blk438_dn7), (((locals.var_t10_dn8 + (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8))) + locals.var_dvthscsti_dn8) - locals.var_vbsz__blk438_dn8), (((locals.var_t10_dn9 + (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9))) + locals.var_dvthscsti_dn9) - locals.var_vbsz__blk438_dn9), (((locals.var_t10_dn10 + (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10))) + locals.var_dvthscsti_dn10) - locals.var_vbsz__blk438_dn10), (((locals.var_t10_dn13 + (0.5 * (locals.var_t1_dn13 + locals.var_t2_dn13))) + locals.var_dvthscsti_dn13) - locals.var_vbsz__blk438_dn13),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign63280_e98070;
        locals.var_t3_dn0 = assign63280_e98070_d_n0;
        locals.var_t3_dn2 = assign63280_e98070_d_n2;
        locals.var_t3_dn4 = assign63280_e98070_d_n4;
        locals.var_t3_dn5 = assign63280_e98070_d_n5;
        locals.var_t3_dn6 = assign63280_e98070_d_n6;
        locals.var_t3_dn7 = assign63280_e98070_d_n7;
        locals.var_t3_dn8 = assign63280_e98070_d_n8;
        locals.var_t3_dn9 = assign63280_e98070_d_n9;
        locals.var_t3_dn10 = assign63280_e98070_d_n10;
        locals.var_t3_dn13 = assign63280_e98070_d_n13;

        let (assign63290_e98083, assign63290_e98083_d_n0, assign63290_e98083_d_n2, assign63290_e98083_d_n4, assign63290_e98083_d_n5, assign63290_e98083_d_n6, assign63290_e98083_d_n7, assign63290_e98083_d_n8, assign63290_e98083_d_n9, assign63290_e98083_d_n10, assign63290_e98083_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63290_e98079: f64 = (locals.var_beta * locals.var_t3);
        let assign63290_e98081: f64 = (assign63290_e98079 - 1.0);
        (assign63290_e98081, ((locals.var_beta_dn0 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn0)), ((locals.var_beta_dn2 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn2)), ((locals.var_beta_dn4 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn4)), ((locals.var_beta_dn5 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn5)), ((locals.var_beta_dn6 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn6)), ((locals.var_beta_dn7 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn7)), ((locals.var_beta_dn8 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn8)), ((locals.var_beta_dn9 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn9)), ((locals.var_beta_dn10 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn10)), ((locals.var_beta_dn13 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign63290_e98083;
        locals.var_t4_dn0 = assign63290_e98083_d_n0;
        locals.var_t4_dn2 = assign63290_e98083_d_n2;
        locals.var_t4_dn4 = assign63290_e98083_d_n4;
        locals.var_t4_dn5 = assign63290_e98083_d_n5;
        locals.var_t4_dn6 = assign63290_e98083_d_n6;
        locals.var_t4_dn7 = assign63290_e98083_d_n7;
        locals.var_t4_dn8 = assign63290_e98083_d_n8;
        locals.var_t4_dn9 = assign63290_e98083_d_n9;
        locals.var_t4_dn10 = assign63290_e98083_d_n10;
        locals.var_t4_dn13 = assign63290_e98083_d_n13;

        let (assign63300_e98094, assign63300_e98094_d_n0, assign63300_e98094_d_n2, assign63300_e98094_d_n4, assign63300_e98094_d_n5, assign63300_e98094_d_n6, assign63300_e98094_d_n7, assign63300_e98094_d_n8, assign63300_e98094_d_n9, assign63300_e98094_d_n10, assign63300_e98094_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63300_e98092: f64 = (4.0 / locals.var_costi5);
        (assign63300_e98092, (-((4.0 * locals.var_costi5_dn0) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn2) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn4) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn5) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn6) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn7) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn8) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn9) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn10) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn13) / (locals.var_costi5 * locals.var_costi5))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign63300_e98094;
        locals.var_t5_dn0 = assign63300_e98094_d_n0;
        locals.var_t5_dn2 = assign63300_e98094_d_n2;
        locals.var_t5_dn4 = assign63300_e98094_d_n4;
        locals.var_t5_dn5 = assign63300_e98094_d_n5;
        locals.var_t5_dn6 = assign63300_e98094_d_n6;
        locals.var_t5_dn7 = assign63300_e98094_d_n7;
        locals.var_t5_dn8 = assign63300_e98094_d_n8;
        locals.var_t5_dn9 = assign63300_e98094_d_n9;
        locals.var_t5_dn10 = assign63300_e98094_d_n10;
        locals.var_t5_dn13 = assign63300_e98094_d_n13;

        let (assign63310_e98107, assign63310_e98107_d_n0, assign63310_e98107_d_n2, assign63310_e98107_d_n4, assign63310_e98107_d_n5, assign63310_e98107_d_n6, assign63310_e98107_d_n7, assign63310_e98107_d_n8, assign63310_e98107_d_n9, assign63310_e98107_d_n10, assign63310_e98107_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63310_e98104: f64 = (locals.var_t4 * locals.var_t5);
        let assign63310_e98105: f64 = (1.0 + assign63310_e98104);
        (assign63310_e98105, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn13 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63310_e98107;
        locals.var_t1_dn0 = assign63310_e98107_d_n0;
        locals.var_t1_dn2 = assign63310_e98107_d_n2;
        locals.var_t1_dn4 = assign63310_e98107_d_n4;
        locals.var_t1_dn5 = assign63310_e98107_d_n5;
        locals.var_t1_dn6 = assign63310_e98107_d_n6;
        locals.var_t1_dn7 = assign63310_e98107_d_n7;
        locals.var_t1_dn8 = assign63310_e98107_d_n8;
        locals.var_t1_dn9 = assign63310_e98107_d_n9;
        locals.var_t1_dn10 = assign63310_e98107_d_n10;
        locals.var_t1_dn13 = assign63310_e98107_d_n13;

        let (assign63320_e98118, assign63320_e98118_d_n0, assign63320_e98118_d_n2, assign63320_e98118_d_n4, assign63320_e98118_d_n5, assign63320_e98118_d_n6, assign63320_e98118_d_n7, assign63320_e98118_d_n8, assign63320_e98118_d_n9, assign63320_e98118_d_n10, assign63320_e98118_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63320_e98116: f64 = (locals.var_beta * locals.var_t5);
        (assign63320_e98116, ((locals.var_beta_dn0 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn0)), ((locals.var_beta_dn2 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn2)), ((locals.var_beta_dn4 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn4)), ((locals.var_beta_dn5 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn5)), ((locals.var_beta_dn6 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn6)), ((locals.var_beta_dn7 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn7)), ((locals.var_beta_dn8 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn8)), ((locals.var_beta_dn9 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn9)), ((locals.var_beta_dn10 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn10)), ((locals.var_beta_dn13 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign63320_e98118;
        locals.var_t6_dn0 = assign63320_e98118_d_n0;
        locals.var_t6_dn2 = assign63320_e98118_d_n2;
        locals.var_t6_dn4 = assign63320_e98118_d_n4;
        locals.var_t6_dn5 = assign63320_e98118_d_n5;
        locals.var_t6_dn6 = assign63320_e98118_d_n6;
        locals.var_t6_dn7 = assign63320_e98118_d_n7;
        locals.var_t6_dn8 = assign63320_e98118_d_n8;
        locals.var_t6_dn9 = assign63320_e98118_d_n9;
        locals.var_t6_dn10 = assign63320_e98118_d_n10;
        locals.var_t6_dn13 = assign63320_e98118_d_n13;

        let (assign63330_e98129, assign63330_e98129_d_n0, assign63330_e98129_d_n2, assign63330_e98129_d_n4, assign63330_e98129_d_n5, assign63330_e98129_d_n6, assign63330_e98129_d_n7, assign63330_e98129_d_n8, assign63330_e98129_d_n9, assign63330_e98129_d_n10, assign63330_e98129_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63330_e98127: f64 = (locals.var_t4 * locals.var_t5);
        (assign63330_e98127, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn13 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn13)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign63330_e98129;
        locals.var_t7_dn0 = assign63330_e98129_d_n0;
        locals.var_t7_dn2 = assign63330_e98129_d_n2;
        locals.var_t7_dn4 = assign63330_e98129_d_n4;
        locals.var_t7_dn5 = assign63330_e98129_d_n5;
        locals.var_t7_dn6 = assign63330_e98129_d_n6;
        locals.var_t7_dn7 = assign63330_e98129_d_n7;
        locals.var_t7_dn8 = assign63330_e98129_d_n8;
        locals.var_t7_dn9 = assign63330_e98129_d_n9;
        locals.var_t7_dn10 = assign63330_e98129_d_n10;
        locals.var_t7_dn13 = assign63330_e98129_d_n13;

        let (assign63340_e98147, assign63340_e98147_d_n0, assign63340_e98147_d_n2, assign63340_e98147_d_n4, assign63340_e98147_d_n5, assign63340_e98147_d_n6, assign63340_e98147_d_n7, assign63340_e98147_d_n8, assign63340_e98147_d_n9, assign63340_e98147_d_n10, assign63340_e98147_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63340_e98138: f64 = (locals.var_t1 * locals.var_t1);
        let assign63340_e98141: f64 = (4.0 * 0.01);
        let assign63340_e98143: f64 = (assign63340_e98141 * 0.01);
        let assign63340_e98144: f64 = (assign63340_e98138 + assign63340_e98143);
        let assign63340_e98145: f64 = (assign63340_e98144).sqrt();
        (assign63340_e98145, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63340_e98145)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63340_e98145)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63340_e98145)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63340_e98145)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63340_e98145)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63340_e98145)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63340_e98145)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63340_e98145)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63340_e98145)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign63340_e98145)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign63340_e98147;
        locals.var_tmf2_dn0 = assign63340_e98147_d_n0;
        locals.var_tmf2_dn2 = assign63340_e98147_d_n2;
        locals.var_tmf2_dn4 = assign63340_e98147_d_n4;
        locals.var_tmf2_dn5 = assign63340_e98147_d_n5;
        locals.var_tmf2_dn6 = assign63340_e98147_d_n6;
        locals.var_tmf2_dn7 = assign63340_e98147_d_n7;
        locals.var_tmf2_dn8 = assign63340_e98147_d_n8;
        locals.var_tmf2_dn9 = assign63340_e98147_d_n9;
        locals.var_tmf2_dn10 = assign63340_e98147_d_n10;
        locals.var_tmf2_dn13 = assign63340_e98147_d_n13;

        let (assign63350_e98162, assign63350_e98162_d_n0, assign63350_e98162_d_n2, assign63350_e98162_d_n4, assign63350_e98162_d_n5, assign63350_e98162_d_n6, assign63350_e98162_d_n7, assign63350_e98162_d_n8, assign63350_e98162_d_n9, assign63350_e98162_d_n10, assign63350_e98162_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63350_e98158: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63350_e98159: f64 = (1.0 + assign63350_e98158);
        let assign63350_e98160: f64 = (0.5 * assign63350_e98159);
        (assign63350_e98160, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign63350_e98162;
        locals.var_t2_dn0 = assign63350_e98162_d_n0;
        locals.var_t2_dn2 = assign63350_e98162_d_n2;
        locals.var_t2_dn4 = assign63350_e98162_d_n4;
        locals.var_t2_dn5 = assign63350_e98162_d_n5;
        locals.var_t2_dn6 = assign63350_e98162_d_n6;
        locals.var_t2_dn7 = assign63350_e98162_d_n7;
        locals.var_t2_dn8 = assign63350_e98162_d_n8;
        locals.var_t2_dn9 = assign63350_e98162_d_n9;
        locals.var_t2_dn10 = assign63350_e98162_d_n10;
        locals.var_t2_dn13 = assign63350_e98162_d_n13;

        let (assign63360_e98175, assign63360_e98175_d_n0, assign63360_e98175_d_n2, assign63360_e98175_d_n4, assign63360_e98175_d_n5, assign63360_e98175_d_n6, assign63360_e98175_d_n7, assign63360_e98175_d_n8, assign63360_e98175_d_n9, assign63360_e98175_d_n10, assign63360_e98175_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63360_e98172: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63360_e98173: f64 = (0.5 * assign63360_e98172);
        (assign63360_e98173, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63360_e98175;
        locals.var_t1_dn0 = assign63360_e98175_d_n0;
        locals.var_t1_dn2 = assign63360_e98175_d_n2;
        locals.var_t1_dn4 = assign63360_e98175_d_n4;
        locals.var_t1_dn5 = assign63360_e98175_d_n5;
        locals.var_t1_dn6 = assign63360_e98175_d_n6;
        locals.var_t1_dn7 = assign63360_e98175_d_n7;
        locals.var_t1_dn8 = assign63360_e98175_d_n8;
        locals.var_t1_dn9 = assign63360_e98175_d_n9;
        locals.var_t1_dn10 = assign63360_e98175_d_n10;
        locals.var_t1_dn13 = assign63360_e98175_d_n13;

        let assign63370_e98178: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1504 = assign63370_e98178;

        let (assign63380_e98189, assign63380_e98189_d_n0, assign63380_e98189_d_n2, assign63380_e98189_d_n4, assign63380_e98189_d_n5, assign63380_e98189_d_n6, assign63380_e98189_d_n7, assign63380_e98189_d_n8, assign63380_e98189_d_n9, assign63380_e98189_d_n10, assign63380_e98189_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63380_e98189;
        locals.var_t1_dn0 = assign63380_e98189_d_n0;
        locals.var_t1_dn2 = assign63380_e98189_d_n2;
        locals.var_t1_dn4 = assign63380_e98189_d_n4;
        locals.var_t1_dn5 = assign63380_e98189_d_n5;
        locals.var_t1_dn6 = assign63380_e98189_d_n6;
        locals.var_t1_dn7 = assign63380_e98189_d_n7;
        locals.var_t1_dn8 = assign63380_e98189_d_n8;
        locals.var_t1_dn9 = assign63380_e98189_d_n9;
        locals.var_t1_dn10 = assign63380_e98189_d_n10;
        locals.var_t1_dn13 = assign63380_e98189_d_n13;

        let (assign63390_e98200, assign63390_e98200_d_n0, assign63390_e98200_d_n2, assign63390_e98200_d_n4, assign63390_e98200_d_n5, assign63390_e98200_d_n6, assign63390_e98200_d_n7, assign63390_e98200_d_n8, assign63390_e98200_d_n9, assign63390_e98200_d_n10, assign63390_e98200_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign63390_e98200;
        locals.var_t2_dn0 = assign63390_e98200_d_n0;
        locals.var_t2_dn2 = assign63390_e98200_d_n2;
        locals.var_t2_dn4 = assign63390_e98200_d_n4;
        locals.var_t2_dn5 = assign63390_e98200_d_n5;
        locals.var_t2_dn6 = assign63390_e98200_d_n6;
        locals.var_t2_dn7 = assign63390_e98200_d_n7;
        locals.var_t2_dn8 = assign63390_e98200_d_n8;
        locals.var_t2_dn9 = assign63390_e98200_d_n9;
        locals.var_t2_dn10 = assign63390_e98200_d_n10;
        locals.var_t2_dn13 = assign63390_e98200_d_n13;

        let (assign63400_e98211, assign63400_e98211_d_n0, assign63400_e98211_d_n2, assign63400_e98211_d_n4, assign63400_e98211_d_n5, assign63400_e98211_d_n6, assign63400_e98211_d_n7, assign63400_e98211_d_n8, assign63400_e98211_d_n9, assign63400_e98211_d_n10, assign63400_e98211_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63400_e98209: f64 = (locals.var_t1 + 1e-25);
        (assign63400_e98209, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63400_e98211;
        locals.var_t1_dn0 = assign63400_e98211_d_n0;
        locals.var_t1_dn2 = assign63400_e98211_d_n2;
        locals.var_t1_dn4 = assign63400_e98211_d_n4;
        locals.var_t1_dn5 = assign63400_e98211_d_n5;
        locals.var_t1_dn6 = assign63400_e98211_d_n6;
        locals.var_t1_dn7 = assign63400_e98211_d_n7;
        locals.var_t1_dn8 = assign63400_e98211_d_n8;
        locals.var_t1_dn9 = assign63400_e98211_d_n9;
        locals.var_t1_dn10 = assign63400_e98211_d_n10;
        locals.var_t1_dn13 = assign63400_e98211_d_n13;

        let (assign63410_e98221, assign63410_e98221_d_n0, assign63410_e98221_d_n2, assign63410_e98221_d_n4, assign63410_e98221_d_n5, assign63410_e98221_d_n6, assign63410_e98221_d_n7, assign63410_e98221_d_n8, assign63410_e98221_d_n9, assign63410_e98221_d_n10, assign63410_e98221_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63410_e98219: f64 = (locals.var_t1).sqrt();
        (assign63410_e98219, (locals.var_t1_dn0 / (2.0 * assign63410_e98219)), (locals.var_t1_dn2 / (2.0 * assign63410_e98219)), (locals.var_t1_dn4 / (2.0 * assign63410_e98219)), (locals.var_t1_dn5 / (2.0 * assign63410_e98219)), (locals.var_t1_dn6 / (2.0 * assign63410_e98219)), (locals.var_t1_dn7 / (2.0 * assign63410_e98219)), (locals.var_t1_dn8 / (2.0 * assign63410_e98219)), (locals.var_t1_dn9 / (2.0 * assign63410_e98219)), (locals.var_t1_dn10 / (2.0 * assign63410_e98219)), (locals.var_t1_dn13 / (2.0 * assign63410_e98219)),)
    } else {
        (locals.var_costi6, locals.var_costi6_dn0, locals.var_costi6_dn2, locals.var_costi6_dn4, locals.var_costi6_dn5, locals.var_costi6_dn6, locals.var_costi6_dn7, locals.var_costi6_dn8, locals.var_costi6_dn9, locals.var_costi6_dn10, locals.var_costi6_dn13,)
    }
};
        locals.var_costi6 = assign63410_e98221;
        locals.var_costi6_dn0 = assign63410_e98221_d_n0;
        locals.var_costi6_dn2 = assign63410_e98221_d_n2;
        locals.var_costi6_dn4 = assign63410_e98221_d_n4;
        locals.var_costi6_dn5 = assign63410_e98221_d_n5;
        locals.var_costi6_dn6 = assign63410_e98221_d_n6;
        locals.var_costi6_dn7 = assign63410_e98221_d_n7;
        locals.var_costi6_dn8 = assign63410_e98221_d_n8;
        locals.var_costi6_dn9 = assign63410_e98221_d_n9;
        locals.var_costi6_dn10 = assign63410_e98221_d_n10;
        locals.var_costi6_dn13 = assign63410_e98221_d_n13;

        let (assign63420_e98234, assign63420_e98234_d_n0, assign63420_e98234_d_n2, assign63420_e98234_d_n4, assign63420_e98234_d_n5, assign63420_e98234_d_n6, assign63420_e98234_d_n7, assign63420_e98234_d_n8, assign63420_e98234_d_n9, assign63420_e98234_d_n10, assign63420_e98234_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63420_e98231: f64 = (1.0 - locals.var_costi6);
        let assign63420_e98232: f64 = (locals.var_costi4 * assign63420_e98231);
        (assign63420_e98232, ((locals.var_costi4_dn0 * assign63420_e98231) + (locals.var_costi4 * (-locals.var_costi6_dn0))), ((locals.var_costi4_dn2 * assign63420_e98231) + (locals.var_costi4 * (-locals.var_costi6_dn2))), ((locals.var_costi4_dn4 * assign63420_e98231) + (locals.var_costi4 * (-locals.var_costi6_dn4))), ((locals.var_costi4_dn5 * assign63420_e98231) + (locals.var_costi4 * (-locals.var_costi6_dn5))), ((locals.var_costi4_dn6 * assign63420_e98231) + (locals.var_costi4 * (-locals.var_costi6_dn6))), ((locals.var_costi4_dn7 * assign63420_e98231) + (locals.var_costi4 * (-locals.var_costi6_dn7))), ((locals.var_costi4_dn8 * assign63420_e98231) + (locals.var_costi4 * (-locals.var_costi6_dn8))), ((locals.var_costi4_dn9 * assign63420_e98231) + (locals.var_costi4 * (-locals.var_costi6_dn9))), ((locals.var_costi4_dn10 * assign63420_e98231) + (locals.var_costi4 * (-locals.var_costi6_dn10))), ((locals.var_costi4_dn13 * assign63420_e98231) + (locals.var_costi4 * (-locals.var_costi6_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign63420_e98234;
        locals.var_t0_dn0 = assign63420_e98234_d_n0;
        locals.var_t0_dn2 = assign63420_e98234_d_n2;
        locals.var_t0_dn4 = assign63420_e98234_d_n4;
        locals.var_t0_dn5 = assign63420_e98234_d_n5;
        locals.var_t0_dn6 = assign63420_e98234_d_n6;
        locals.var_t0_dn7 = assign63420_e98234_d_n7;
        locals.var_t0_dn8 = assign63420_e98234_d_n8;
        locals.var_t0_dn9 = assign63420_e98234_d_n9;
        locals.var_t0_dn10 = assign63420_e98234_d_n10;
        locals.var_t0_dn13 = assign63420_e98234_d_n13;

        let (assign63430_e98245, assign63430_e98245_d_n0, assign63430_e98245_d_n2, assign63430_e98245_d_n4, assign63430_e98245_d_n5, assign63430_e98245_d_n6, assign63430_e98245_d_n7, assign63430_e98245_d_n8, assign63430_e98245_d_n9, assign63430_e98245_d_n10, assign63430_e98245_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63430_e98243: f64 = (locals.var_vgssti + locals.var_t0);
        (assign63430_e98243, (locals.var_vgssti_dn0 + locals.var_t0_dn0), (locals.var_vgssti_dn2 + locals.var_t0_dn2), (locals.var_vgssti_dn4 + locals.var_t0_dn4), (locals.var_vgssti_dn5 + locals.var_t0_dn5), (locals.var_vgssti_dn6 + locals.var_t0_dn6), (locals.var_vgssti_dn7 + locals.var_t0_dn7), (locals.var_vgssti_dn8 + locals.var_t0_dn8), (locals.var_vgssti_dn9 + locals.var_t0_dn9), (locals.var_vgssti_dn10 + locals.var_t0_dn10), (locals.var_vgssti_dn13 + locals.var_t0_dn13),)
    } else {
        (locals.var_psasti, locals.var_psasti_dn0, locals.var_psasti_dn2, locals.var_psasti_dn4, locals.var_psasti_dn5, locals.var_psasti_dn6, locals.var_psasti_dn7, locals.var_psasti_dn8, locals.var_psasti_dn9, locals.var_psasti_dn10, locals.var_psasti_dn13,)
    }
};
        locals.var_psasti = assign63430_e98245;
        locals.var_psasti_dn0 = assign63430_e98245_d_n0;
        locals.var_psasti_dn2 = assign63430_e98245_d_n2;
        locals.var_psasti_dn4 = assign63430_e98245_d_n4;
        locals.var_psasti_dn5 = assign63430_e98245_d_n5;
        locals.var_psasti_dn6 = assign63430_e98245_d_n6;
        locals.var_psasti_dn7 = assign63430_e98245_d_n7;
        locals.var_psasti_dn8 = assign63430_e98245_d_n8;
        locals.var_psasti_dn9 = assign63430_e98245_d_n9;
        locals.var_psasti_dn10 = assign63430_e98245_d_n10;
        locals.var_psasti_dn13 = assign63430_e98245_d_n13;

        let (assign63440_e98262, assign63440_e98262_d_n0, assign63440_e98262_d_n2, assign63440_e98262_d_n4, assign63440_e98262_d_n5, assign63440_e98262_d_n6, assign63440_e98262_d_n7, assign63440_e98262_d_n8, assign63440_e98262_d_n9, assign63440_e98262_d_n10, assign63440_e98262_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63440_e98257: f64 = (locals.var_vgssti + 1e-25);
        let assign63440_e98258: f64 = (2.0 / assign63440_e98257);
        let assign63440_e98259: f64 = (locals.var_beta + assign63440_e98258);
        let assign63440_e98260: f64 = (1.0 / assign63440_e98259);
        (assign63440_e98260, (-((locals.var_beta_dn0 + (-((2.0 * locals.var_vgssti_dn0) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((locals.var_beta_dn2 + (-((2.0 * locals.var_vgssti_dn2) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((locals.var_beta_dn4 + (-((2.0 * locals.var_vgssti_dn4) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((locals.var_beta_dn5 + (-((2.0 * locals.var_vgssti_dn5) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((locals.var_beta_dn6 + (-((2.0 * locals.var_vgssti_dn6) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((locals.var_beta_dn7 + (-((2.0 * locals.var_vgssti_dn7) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((locals.var_beta_dn8 + (-((2.0 * locals.var_vgssti_dn8) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((locals.var_beta_dn9 + (-((2.0 * locals.var_vgssti_dn9) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((locals.var_beta_dn10 + (-((2.0 * locals.var_vgssti_dn10) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((locals.var_beta_dn13 + (-((2.0 * locals.var_vgssti_dn13) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign63440_e98262;
        locals.var_t0_dn0 = assign63440_e98262_d_n0;
        locals.var_t0_dn2 = assign63440_e98262_d_n2;
        locals.var_t0_dn4 = assign63440_e98262_d_n4;
        locals.var_t0_dn5 = assign63440_e98262_d_n5;
        locals.var_t0_dn6 = assign63440_e98262_d_n6;
        locals.var_t0_dn7 = assign63440_e98262_d_n7;
        locals.var_t0_dn8 = assign63440_e98262_d_n8;
        locals.var_t0_dn9 = assign63440_e98262_d_n9;
        locals.var_t0_dn10 = assign63440_e98262_d_n10;
        locals.var_t0_dn13 = assign63440_e98262_d_n13;

        let (assign63450_e98282, assign63450_e98282_d_n0, assign63450_e98282_d_n2, assign63450_e98282_d_n4, assign63450_e98282_d_n5, assign63450_e98282_d_n6, assign63450_e98282_d_n7, assign63450_e98282_d_n8, assign63450_e98282_d_n9, assign63450_e98282_d_n10, assign63450_e98282_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63450_e98271: f64 = (1.0 / locals.var_costi1);
        let assign63450_e98273: f64 = (assign63450_e98271 / locals.var_costi3);
        let assign63450_e98276: f64 = (locals.var_vgssti * locals.var_vgssti);
        let assign63450_e98277: f64 = (assign63450_e98273 * assign63450_e98276);
        let assign63450_e98278: f64 = (assign63450_e98277).ln();
        let assign63450_e98280: f64 = (assign63450_e98278 * locals.var_t0);
        (assign63450_e98280, (((((((((-(locals.var_costi1_dn0 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63450_e98271 * locals.var_costi3_dn0)) / (locals.var_costi3 * locals.var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((locals.var_vgssti_dn0 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn0)))) / assign63450_e98277) * locals.var_t0) + (assign63450_e98278 * locals.var_t0_dn0)), (((((((((-(locals.var_costi1_dn2 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63450_e98271 * locals.var_costi3_dn2)) / (locals.var_costi3 * locals.var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((locals.var_vgssti_dn2 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn2)))) / assign63450_e98277) * locals.var_t0) + (assign63450_e98278 * locals.var_t0_dn2)), (((((((((-(locals.var_costi1_dn4 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63450_e98271 * locals.var_costi3_dn4)) / (locals.var_costi3 * locals.var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((locals.var_vgssti_dn4 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn4)))) / assign63450_e98277) * locals.var_t0) + (assign63450_e98278 * locals.var_t0_dn4)), (((((((((-(locals.var_costi1_dn5 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63450_e98271 * locals.var_costi3_dn5)) / (locals.var_costi3 * locals.var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((locals.var_vgssti_dn5 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn5)))) / assign63450_e98277) * locals.var_t0) + (assign63450_e98278 * locals.var_t0_dn5)), (((((((((-(locals.var_costi1_dn6 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63450_e98271 * locals.var_costi3_dn6)) / (locals.var_costi3 * locals.var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((locals.var_vgssti_dn6 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn6)))) / assign63450_e98277) * locals.var_t0) + (assign63450_e98278 * locals.var_t0_dn6)), (((((((((-(locals.var_costi1_dn7 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63450_e98271 * locals.var_costi3_dn7)) / (locals.var_costi3 * locals.var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((locals.var_vgssti_dn7 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn7)))) / assign63450_e98277) * locals.var_t0) + (assign63450_e98278 * locals.var_t0_dn7)), (((((((((-(locals.var_costi1_dn8 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63450_e98271 * locals.var_costi3_dn8)) / (locals.var_costi3 * locals.var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((locals.var_vgssti_dn8 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn8)))) / assign63450_e98277) * locals.var_t0) + (assign63450_e98278 * locals.var_t0_dn8)), (((((((((-(locals.var_costi1_dn9 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63450_e98271 * locals.var_costi3_dn9)) / (locals.var_costi3 * locals.var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((locals.var_vgssti_dn9 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn9)))) / assign63450_e98277) * locals.var_t0) + (assign63450_e98278 * locals.var_t0_dn9)), (((((((((-(locals.var_costi1_dn10 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63450_e98271 * locals.var_costi3_dn10)) / (locals.var_costi3 * locals.var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((locals.var_vgssti_dn10 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn10)))) / assign63450_e98277) * locals.var_t0) + (assign63450_e98278 * locals.var_t0_dn10)), (((((((((-(locals.var_costi1_dn13 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63450_e98271 * locals.var_costi3_dn13)) / (locals.var_costi3 * locals.var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((locals.var_vgssti_dn13 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn13)))) / assign63450_e98277) * locals.var_t0) + (assign63450_e98278 * locals.var_t0_dn13)),)
    } else {
        (locals.var_psbsti, locals.var_psbsti_dn0, locals.var_psbsti_dn2, locals.var_psbsti_dn4, locals.var_psbsti_dn5, locals.var_psbsti_dn6, locals.var_psbsti_dn7, locals.var_psbsti_dn8, locals.var_psbsti_dn9, locals.var_psbsti_dn10, locals.var_psbsti_dn13,)
    }
};
        locals.var_psbsti = assign63450_e98282;
        locals.var_psbsti_dn0 = assign63450_e98282_d_n0;
        locals.var_psbsti_dn2 = assign63450_e98282_d_n2;
        locals.var_psbsti_dn4 = assign63450_e98282_d_n4;
        locals.var_psbsti_dn5 = assign63450_e98282_d_n5;
        locals.var_psbsti_dn6 = assign63450_e98282_d_n6;
        locals.var_psbsti_dn7 = assign63450_e98282_d_n7;
        locals.var_psbsti_dn8 = assign63450_e98282_d_n8;
        locals.var_psbsti_dn9 = assign63450_e98282_d_n9;
        locals.var_psbsti_dn10 = assign63450_e98282_d_n10;
        locals.var_psbsti_dn13 = assign63450_e98282_d_n13;

        let (assign63460_e98295, assign63460_e98295_d_n0, assign63460_e98295_d_n2, assign63460_e98295_d_n4, assign63460_e98295_d_n5, assign63460_e98295_d_n6, assign63460_e98295_d_n7, assign63460_e98295_d_n8, assign63460_e98295_d_n9, assign63460_e98295_d_n10, assign63460_e98295_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63460_e98291: f64 = (locals.var_psbsti - locals.var_psasti);
        let assign63460_e98293: f64 = (assign63460_e98291 - 0.002);
        (assign63460_e98293, (locals.var_psbsti_dn0 - locals.var_psasti_dn0), (locals.var_psbsti_dn2 - locals.var_psasti_dn2), (locals.var_psbsti_dn4 - locals.var_psasti_dn4), (locals.var_psbsti_dn5 - locals.var_psasti_dn5), (locals.var_psbsti_dn6 - locals.var_psasti_dn6), (locals.var_psbsti_dn7 - locals.var_psasti_dn7), (locals.var_psbsti_dn8 - locals.var_psasti_dn8), (locals.var_psbsti_dn9 - locals.var_psasti_dn9), (locals.var_psbsti_dn10 - locals.var_psasti_dn10), (locals.var_psbsti_dn13 - locals.var_psasti_dn13),)
    } else {
        (locals.var_psab, locals.var_psab_dn0, locals.var_psab_dn2, locals.var_psab_dn4, locals.var_psab_dn5, locals.var_psab_dn6, locals.var_psab_dn7, locals.var_psab_dn8, locals.var_psab_dn9, locals.var_psab_dn10, locals.var_psab_dn13,)
    }
};
        locals.var_psab = assign63460_e98295;
        locals.var_psab_dn0 = assign63460_e98295_d_n0;
        locals.var_psab_dn2 = assign63460_e98295_d_n2;
        locals.var_psab_dn4 = assign63460_e98295_d_n4;
        locals.var_psab_dn5 = assign63460_e98295_d_n5;
        locals.var_psab_dn6 = assign63460_e98295_d_n6;
        locals.var_psab_dn7 = assign63460_e98295_d_n7;
        locals.var_psab_dn8 = assign63460_e98295_d_n8;
        locals.var_psab_dn9 = assign63460_e98295_d_n9;
        locals.var_psab_dn10 = assign63460_e98295_d_n10;
        locals.var_psab_dn13 = assign63460_e98295_d_n13;

        let (assign63470_e98313, assign63470_e98313_d_n0, assign63470_e98313_d_n2, assign63470_e98313_d_n4, assign63470_e98313_d_n5, assign63470_e98313_d_n6, assign63470_e98313_d_n7, assign63470_e98313_d_n8, assign63470_e98313_d_n9, assign63470_e98313_d_n10, assign63470_e98313_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63470_e98304: f64 = (locals.var_psab * locals.var_psab);
        let assign63470_e98307: f64 = (4.0 * 0.002);
        let assign63470_e98309: f64 = (assign63470_e98307 * locals.var_psbsti);
        let assign63470_e98310: f64 = (assign63470_e98304 + assign63470_e98309);
        let assign63470_e98311: f64 = (assign63470_e98310).sqrt();
        (assign63470_e98311, ((((locals.var_psab_dn0 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn0)) + (assign63470_e98307 * locals.var_psbsti_dn0)) / (2.0 * assign63470_e98311)), ((((locals.var_psab_dn2 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn2)) + (assign63470_e98307 * locals.var_psbsti_dn2)) / (2.0 * assign63470_e98311)), ((((locals.var_psab_dn4 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn4)) + (assign63470_e98307 * locals.var_psbsti_dn4)) / (2.0 * assign63470_e98311)), ((((locals.var_psab_dn5 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn5)) + (assign63470_e98307 * locals.var_psbsti_dn5)) / (2.0 * assign63470_e98311)), ((((locals.var_psab_dn6 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn6)) + (assign63470_e98307 * locals.var_psbsti_dn6)) / (2.0 * assign63470_e98311)), ((((locals.var_psab_dn7 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn7)) + (assign63470_e98307 * locals.var_psbsti_dn7)) / (2.0 * assign63470_e98311)), ((((locals.var_psab_dn8 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn8)) + (assign63470_e98307 * locals.var_psbsti_dn8)) / (2.0 * assign63470_e98311)), ((((locals.var_psab_dn9 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn9)) + (assign63470_e98307 * locals.var_psbsti_dn9)) / (2.0 * assign63470_e98311)), ((((locals.var_psab_dn10 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn10)) + (assign63470_e98307 * locals.var_psbsti_dn10)) / (2.0 * assign63470_e98311)), ((((locals.var_psab_dn13 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn13)) + (assign63470_e98307 * locals.var_psbsti_dn13)) / (2.0 * assign63470_e98311)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign63470_e98313;
        locals.var_t0_dn0 = assign63470_e98313_d_n0;
        locals.var_t0_dn2 = assign63470_e98313_d_n2;
        locals.var_t0_dn4 = assign63470_e98313_d_n4;
        locals.var_t0_dn5 = assign63470_e98313_d_n5;
        locals.var_t0_dn6 = assign63470_e98313_d_n6;
        locals.var_t0_dn7 = assign63470_e98313_d_n7;
        locals.var_t0_dn8 = assign63470_e98313_d_n8;
        locals.var_t0_dn9 = assign63470_e98313_d_n9;
        locals.var_t0_dn10 = assign63470_e98313_d_n10;
        locals.var_t0_dn13 = assign63470_e98313_d_n13;

    }

    pub(super) fn stamp_transient_block_215(
        locals: &mut StampLocals,
    ) {
        let (assign63480_e98328, assign63480_e98328_d_n0, assign63480_e98328_d_n2, assign63480_e98328_d_n4, assign63480_e98328_d_n5, assign63480_e98328_d_n6, assign63480_e98328_d_n7, assign63480_e98328_d_n8, assign63480_e98328_d_n9, assign63480_e98328_d_n10, assign63480_e98328_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63480_e98324: f64 = (locals.var_psab + locals.var_t0);
        let assign63480_e98325: f64 = (0.5 * assign63480_e98324);
        let assign63480_e98326: f64 = (locals.var_psbsti - assign63480_e98325);
        (assign63480_e98326, (locals.var_psbsti_dn0 - (0.5 * (locals.var_psab_dn0 + locals.var_t0_dn0))), (locals.var_psbsti_dn2 - (0.5 * (locals.var_psab_dn2 + locals.var_t0_dn2))), (locals.var_psbsti_dn4 - (0.5 * (locals.var_psab_dn4 + locals.var_t0_dn4))), (locals.var_psbsti_dn5 - (0.5 * (locals.var_psab_dn5 + locals.var_t0_dn5))), (locals.var_psbsti_dn6 - (0.5 * (locals.var_psab_dn6 + locals.var_t0_dn6))), (locals.var_psbsti_dn7 - (0.5 * (locals.var_psab_dn7 + locals.var_t0_dn7))), (locals.var_psbsti_dn8 - (0.5 * (locals.var_psab_dn8 + locals.var_t0_dn8))), (locals.var_psbsti_dn9 - (0.5 * (locals.var_psab_dn9 + locals.var_t0_dn9))), (locals.var_psbsti_dn10 - (0.5 * (locals.var_psab_dn10 + locals.var_t0_dn10))), (locals.var_psbsti_dn13 - (0.5 * (locals.var_psab_dn13 + locals.var_t0_dn13))),)
    } else {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn4, locals.var_psti_dn5, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn8, locals.var_psti_dn9, locals.var_psti_dn10, locals.var_psti_dn13,)
    }
};
        locals.var_psti = assign63480_e98328;
        locals.var_psti_dn0 = assign63480_e98328_d_n0;
        locals.var_psti_dn2 = assign63480_e98328_d_n2;
        locals.var_psti_dn4 = assign63480_e98328_d_n4;
        locals.var_psti_dn5 = assign63480_e98328_d_n5;
        locals.var_psti_dn6 = assign63480_e98328_d_n6;
        locals.var_psti_dn7 = assign63480_e98328_d_n7;
        locals.var_psti_dn8 = assign63480_e98328_d_n8;
        locals.var_psti_dn9 = assign63480_e98328_d_n9;
        locals.var_psti_dn10 = assign63480_e98328_d_n10;
        locals.var_psti_dn13 = assign63480_e98328_d_n13;

        let (assign63490_e98342, assign63490_e98342_d_n0, assign63490_e98342_d_n2, assign63490_e98342_d_n4, assign63490_e98342_d_n5, assign63490_e98342_d_n6, assign63490_e98342_d_n7, assign63490_e98342_d_n8, assign63490_e98342_d_n9, assign63490_e98342_d_n10, assign63490_e98342_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63490_e98338: f64 = (locals.var_beta * locals.var_psti);
        let assign63490_e98339: f64 = (assign63490_e98338).exp();
        let assign63490_e98340: f64 = (locals.var_costi1 * assign63490_e98339);
        (assign63490_e98340, ((locals.var_costi1_dn0 * assign63490_e98339) + (locals.var_costi1 * (assign63490_e98339 * ((locals.var_beta_dn0 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn0))))), ((locals.var_costi1_dn2 * assign63490_e98339) + (locals.var_costi1 * (assign63490_e98339 * ((locals.var_beta_dn2 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn2))))), ((locals.var_costi1_dn4 * assign63490_e98339) + (locals.var_costi1 * (assign63490_e98339 * ((locals.var_beta_dn4 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn4))))), ((locals.var_costi1_dn5 * assign63490_e98339) + (locals.var_costi1 * (assign63490_e98339 * ((locals.var_beta_dn5 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn5))))), ((locals.var_costi1_dn6 * assign63490_e98339) + (locals.var_costi1 * (assign63490_e98339 * ((locals.var_beta_dn6 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn6))))), ((locals.var_costi1_dn7 * assign63490_e98339) + (locals.var_costi1 * (assign63490_e98339 * ((locals.var_beta_dn7 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn7))))), ((locals.var_costi1_dn8 * assign63490_e98339) + (locals.var_costi1 * (assign63490_e98339 * ((locals.var_beta_dn8 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn8))))), ((locals.var_costi1_dn9 * assign63490_e98339) + (locals.var_costi1 * (assign63490_e98339 * ((locals.var_beta_dn9 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn9))))), ((locals.var_costi1_dn10 * assign63490_e98339) + (locals.var_costi1 * (assign63490_e98339 * ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10))))), ((locals.var_costi1_dn13 * assign63490_e98339) + (locals.var_costi1 * (assign63490_e98339 * ((locals.var_beta_dn13 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn13))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign63490_e98342;
        locals.var_t0_dn0 = assign63490_e98342_d_n0;
        locals.var_t0_dn2 = assign63490_e98342_d_n2;
        locals.var_t0_dn4 = assign63490_e98342_d_n4;
        locals.var_t0_dn5 = assign63490_e98342_d_n5;
        locals.var_t0_dn6 = assign63490_e98342_d_n6;
        locals.var_t0_dn7 = assign63490_e98342_d_n7;
        locals.var_t0_dn8 = assign63490_e98342_d_n8;
        locals.var_t0_dn9 = assign63490_e98342_d_n9;
        locals.var_t0_dn10 = assign63490_e98342_d_n10;
        locals.var_t0_dn13 = assign63490_e98342_d_n13;

        let (assign63500_e98359, assign63500_e98359_d_n0, assign63500_e98359_d_n2, assign63500_e98359_d_n4, assign63500_e98359_d_n5, assign63500_e98359_d_n6, assign63500_e98359_d_n7, assign63500_e98359_d_n8, assign63500_e98359_d_n9, assign63500_e98359_d_n10, assign63500_e98359_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63500_e98352: f64 = (locals.var_psti - locals.var_vbsz__blk438);
        let assign63500_e98353: f64 = (locals.var_beta * assign63500_e98352);
        let assign63500_e98355: f64 = (assign63500_e98353 - 1.0);
        let assign63500_e98357: f64 = (assign63500_e98355 + locals.var_t0);
        (assign63500_e98357, (((locals.var_beta_dn0 * assign63500_e98352) + (locals.var_beta * (locals.var_psti_dn0 - locals.var_vbsz__blk438_dn0))) + locals.var_t0_dn0), (((locals.var_beta_dn2 * assign63500_e98352) + (locals.var_beta * (locals.var_psti_dn2 - locals.var_vbsz__blk438_dn2))) + locals.var_t0_dn2), (((locals.var_beta_dn4 * assign63500_e98352) + (locals.var_beta * (locals.var_psti_dn4 - locals.var_vbsz__blk438_dn4))) + locals.var_t0_dn4), (((locals.var_beta_dn5 * assign63500_e98352) + (locals.var_beta * (locals.var_psti_dn5 - locals.var_vbsz__blk438_dn5))) + locals.var_t0_dn5), (((locals.var_beta_dn6 * assign63500_e98352) + (locals.var_beta * (locals.var_psti_dn6 - locals.var_vbsz__blk438_dn6))) + locals.var_t0_dn6), (((locals.var_beta_dn7 * assign63500_e98352) + (locals.var_beta * (locals.var_psti_dn7 - locals.var_vbsz__blk438_dn7))) + locals.var_t0_dn7), (((locals.var_beta_dn8 * assign63500_e98352) + (locals.var_beta * (locals.var_psti_dn8 - locals.var_vbsz__blk438_dn8))) + locals.var_t0_dn8), (((locals.var_beta_dn9 * assign63500_e98352) + (locals.var_beta * (locals.var_psti_dn9 - locals.var_vbsz__blk438_dn9))) + locals.var_t0_dn9), (((locals.var_beta_dn10 * assign63500_e98352) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbsz__blk438_dn10))) + locals.var_t0_dn10), (((locals.var_beta_dn13 * assign63500_e98352) + (locals.var_beta * (locals.var_psti_dn13 - locals.var_vbsz__blk438_dn13))) + locals.var_t0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63500_e98359;
        locals.var_t1_dn0 = assign63500_e98359_d_n0;
        locals.var_t1_dn2 = assign63500_e98359_d_n2;
        locals.var_t1_dn4 = assign63500_e98359_d_n4;
        locals.var_t1_dn5 = assign63500_e98359_d_n5;
        locals.var_t1_dn6 = assign63500_e98359_d_n6;
        locals.var_t1_dn7 = assign63500_e98359_d_n7;
        locals.var_t1_dn8 = assign63500_e98359_d_n8;
        locals.var_t1_dn9 = assign63500_e98359_d_n9;
        locals.var_t1_dn10 = assign63500_e98359_d_n10;
        locals.var_t1_dn13 = assign63500_e98359_d_n13;

        let (assign63510_e98377, assign63510_e98377_d_n0, assign63510_e98377_d_n2, assign63510_e98377_d_n4, assign63510_e98377_d_n5, assign63510_e98377_d_n6, assign63510_e98377_d_n7, assign63510_e98377_d_n8, assign63510_e98377_d_n9, assign63510_e98377_d_n10, assign63510_e98377_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63510_e98368: f64 = (locals.var_t1 * locals.var_t1);
        let assign63510_e98371: f64 = (4.0 * 0.01);
        let assign63510_e98373: f64 = (assign63510_e98371 * 0.01);
        let assign63510_e98374: f64 = (assign63510_e98368 + assign63510_e98373);
        let assign63510_e98375: f64 = (assign63510_e98374).sqrt();
        (assign63510_e98375, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63510_e98375)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63510_e98375)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63510_e98375)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63510_e98375)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63510_e98375)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63510_e98375)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63510_e98375)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63510_e98375)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63510_e98375)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign63510_e98375)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign63510_e98377;
        locals.var_tmf2_dn0 = assign63510_e98377_d_n0;
        locals.var_tmf2_dn2 = assign63510_e98377_d_n2;
        locals.var_tmf2_dn4 = assign63510_e98377_d_n4;
        locals.var_tmf2_dn5 = assign63510_e98377_d_n5;
        locals.var_tmf2_dn6 = assign63510_e98377_d_n6;
        locals.var_tmf2_dn7 = assign63510_e98377_d_n7;
        locals.var_tmf2_dn8 = assign63510_e98377_d_n8;
        locals.var_tmf2_dn9 = assign63510_e98377_d_n9;
        locals.var_tmf2_dn10 = assign63510_e98377_d_n10;
        locals.var_tmf2_dn13 = assign63510_e98377_d_n13;

        let (assign63520_e98392, assign63520_e98392_d_n0, assign63520_e98392_d_n2, assign63520_e98392_d_n4, assign63520_e98392_d_n5, assign63520_e98392_d_n6, assign63520_e98392_d_n7, assign63520_e98392_d_n8, assign63520_e98392_d_n9, assign63520_e98392_d_n10, assign63520_e98392_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63520_e98388: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63520_e98389: f64 = (1.0 + assign63520_e98388);
        let assign63520_e98390: f64 = (0.5 * assign63520_e98389);
        (assign63520_e98390, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign63520_e98392;
        locals.var_t0_dn0 = assign63520_e98392_d_n0;
        locals.var_t0_dn2 = assign63520_e98392_d_n2;
        locals.var_t0_dn4 = assign63520_e98392_d_n4;
        locals.var_t0_dn5 = assign63520_e98392_d_n5;
        locals.var_t0_dn6 = assign63520_e98392_d_n6;
        locals.var_t0_dn7 = assign63520_e98392_d_n7;
        locals.var_t0_dn8 = assign63520_e98392_d_n8;
        locals.var_t0_dn9 = assign63520_e98392_d_n9;
        locals.var_t0_dn10 = assign63520_e98392_d_n10;
        locals.var_t0_dn13 = assign63520_e98392_d_n13;

        let (assign63530_e98405, assign63530_e98405_d_n0, assign63530_e98405_d_n2, assign63530_e98405_d_n4, assign63530_e98405_d_n5, assign63530_e98405_d_n6, assign63530_e98405_d_n7, assign63530_e98405_d_n8, assign63530_e98405_d_n9, assign63530_e98405_d_n10, assign63530_e98405_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63530_e98402: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63530_e98403: f64 = (0.5 * assign63530_e98402);
        (assign63530_e98403, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63530_e98405;
        locals.var_t1_dn0 = assign63530_e98405_d_n0;
        locals.var_t1_dn2 = assign63530_e98405_d_n2;
        locals.var_t1_dn4 = assign63530_e98405_d_n4;
        locals.var_t1_dn5 = assign63530_e98405_d_n5;
        locals.var_t1_dn6 = assign63530_e98405_d_n6;
        locals.var_t1_dn7 = assign63530_e98405_d_n7;
        locals.var_t1_dn8 = assign63530_e98405_d_n8;
        locals.var_t1_dn9 = assign63530_e98405_d_n9;
        locals.var_t1_dn10 = assign63530_e98405_d_n10;
        locals.var_t1_dn13 = assign63530_e98405_d_n13;

        let assign63540_e98408: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1505 = assign63540_e98408;

        let (assign63550_e98419, assign63550_e98419_d_n0, assign63550_e98419_d_n2, assign63550_e98419_d_n4, assign63550_e98419_d_n5, assign63550_e98419_d_n6, assign63550_e98419_d_n7, assign63550_e98419_d_n8, assign63550_e98419_d_n9, assign63550_e98419_d_n10, assign63550_e98419_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63550_e98419;
        locals.var_t1_dn0 = assign63550_e98419_d_n0;
        locals.var_t1_dn2 = assign63550_e98419_d_n2;
        locals.var_t1_dn4 = assign63550_e98419_d_n4;
        locals.var_t1_dn5 = assign63550_e98419_d_n5;
        locals.var_t1_dn6 = assign63550_e98419_d_n6;
        locals.var_t1_dn7 = assign63550_e98419_d_n7;
        locals.var_t1_dn8 = assign63550_e98419_d_n8;
        locals.var_t1_dn9 = assign63550_e98419_d_n9;
        locals.var_t1_dn10 = assign63550_e98419_d_n10;
        locals.var_t1_dn13 = assign63550_e98419_d_n13;

        let (assign63560_e98430, assign63560_e98430_d_n0, assign63560_e98430_d_n2, assign63560_e98430_d_n4, assign63560_e98430_d_n5, assign63560_e98430_d_n6, assign63560_e98430_d_n7, assign63560_e98430_d_n8, assign63560_e98430_d_n9, assign63560_e98430_d_n10, assign63560_e98430_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign63560_e98430;
        locals.var_t0_dn0 = assign63560_e98430_d_n0;
        locals.var_t0_dn2 = assign63560_e98430_d_n2;
        locals.var_t0_dn4 = assign63560_e98430_d_n4;
        locals.var_t0_dn5 = assign63560_e98430_d_n5;
        locals.var_t0_dn6 = assign63560_e98430_d_n6;
        locals.var_t0_dn7 = assign63560_e98430_d_n7;
        locals.var_t0_dn8 = assign63560_e98430_d_n8;
        locals.var_t0_dn9 = assign63560_e98430_d_n9;
        locals.var_t0_dn10 = assign63560_e98430_d_n10;
        locals.var_t0_dn13 = assign63560_e98430_d_n13;

        let (assign63570_e98441, assign63570_e98441_d_n0, assign63570_e98441_d_n2, assign63570_e98441_d_n4, assign63570_e98441_d_n5, assign63570_e98441_d_n6, assign63570_e98441_d_n7, assign63570_e98441_d_n8, assign63570_e98441_d_n9, assign63570_e98441_d_n10, assign63570_e98441_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63570_e98439: f64 = (locals.var_t1 + 1e-25);
        (assign63570_e98439, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63570_e98441;
        locals.var_t1_dn0 = assign63570_e98441_d_n0;
        locals.var_t1_dn2 = assign63570_e98441_d_n2;
        locals.var_t1_dn4 = assign63570_e98441_d_n4;
        locals.var_t1_dn5 = assign63570_e98441_d_n5;
        locals.var_t1_dn6 = assign63570_e98441_d_n6;
        locals.var_t1_dn7 = assign63570_e98441_d_n7;
        locals.var_t1_dn8 = assign63570_e98441_d_n8;
        locals.var_t1_dn9 = assign63570_e98441_d_n9;
        locals.var_t1_dn10 = assign63570_e98441_d_n10;
        locals.var_t1_dn13 = assign63570_e98441_d_n13;

        let (assign63580_e98451, assign63580_e98451_d_n0, assign63580_e98451_d_n2, assign63580_e98451_d_n4, assign63580_e98451_d_n5, assign63580_e98451_d_n6, assign63580_e98451_d_n7, assign63580_e98451_d_n8, assign63580_e98451_d_n9, assign63580_e98451_d_n10, assign63580_e98451_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63580_e98449: f64 = (locals.var_t1).sqrt();
        (assign63580_e98449, (locals.var_t1_dn0 / (2.0 * assign63580_e98449)), (locals.var_t1_dn2 / (2.0 * assign63580_e98449)), (locals.var_t1_dn4 / (2.0 * assign63580_e98449)), (locals.var_t1_dn5 / (2.0 * assign63580_e98449)), (locals.var_t1_dn6 / (2.0 * assign63580_e98449)), (locals.var_t1_dn7 / (2.0 * assign63580_e98449)), (locals.var_t1_dn8 / (2.0 * assign63580_e98449)), (locals.var_t1_dn9 / (2.0 * assign63580_e98449)), (locals.var_t1_dn10 / (2.0 * assign63580_e98449)), (locals.var_t1_dn13 / (2.0 * assign63580_e98449)),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn4, locals.var_sq1sti_dn5, locals.var_sq1sti_dn6, locals.var_sq1sti_dn7, locals.var_sq1sti_dn8, locals.var_sq1sti_dn9, locals.var_sq1sti_dn10, locals.var_sq1sti_dn13,)
    }
};
        locals.var_sq1sti = assign63580_e98451;
        locals.var_sq1sti_dn0 = assign63580_e98451_d_n0;
        locals.var_sq1sti_dn2 = assign63580_e98451_d_n2;
        locals.var_sq1sti_dn4 = assign63580_e98451_d_n4;
        locals.var_sq1sti_dn5 = assign63580_e98451_d_n5;
        locals.var_sq1sti_dn6 = assign63580_e98451_d_n6;
        locals.var_sq1sti_dn7 = assign63580_e98451_d_n7;
        locals.var_sq1sti_dn8 = assign63580_e98451_d_n8;
        locals.var_sq1sti_dn9 = assign63580_e98451_d_n9;
        locals.var_sq1sti_dn10 = assign63580_e98451_d_n10;
        locals.var_sq1sti_dn13 = assign63580_e98451_d_n13;

        let (assign63590_e98466, assign63590_e98466_d_n0, assign63590_e98466_d_n2, assign63590_e98466_d_n4, assign63590_e98466_d_n5, assign63590_e98466_d_n6, assign63590_e98466_d_n7, assign63590_e98466_d_n8, assign63590_e98466_d_n9, assign63590_e98466_d_n10, assign63590_e98466_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63590_e98461: f64 = (locals.var_psti - locals.var_vbsz__blk438);
        let assign63590_e98462: f64 = (locals.var_beta * assign63590_e98461);
        let assign63590_e98464: f64 = (assign63590_e98462 - 1.0);
        (assign63590_e98464, ((locals.var_beta_dn0 * assign63590_e98461) + (locals.var_beta * (locals.var_psti_dn0 - locals.var_vbsz__blk438_dn0))), ((locals.var_beta_dn2 * assign63590_e98461) + (locals.var_beta * (locals.var_psti_dn2 - locals.var_vbsz__blk438_dn2))), ((locals.var_beta_dn4 * assign63590_e98461) + (locals.var_beta * (locals.var_psti_dn4 - locals.var_vbsz__blk438_dn4))), ((locals.var_beta_dn5 * assign63590_e98461) + (locals.var_beta * (locals.var_psti_dn5 - locals.var_vbsz__blk438_dn5))), ((locals.var_beta_dn6 * assign63590_e98461) + (locals.var_beta * (locals.var_psti_dn6 - locals.var_vbsz__blk438_dn6))), ((locals.var_beta_dn7 * assign63590_e98461) + (locals.var_beta * (locals.var_psti_dn7 - locals.var_vbsz__blk438_dn7))), ((locals.var_beta_dn8 * assign63590_e98461) + (locals.var_beta * (locals.var_psti_dn8 - locals.var_vbsz__blk438_dn8))), ((locals.var_beta_dn9 * assign63590_e98461) + (locals.var_beta * (locals.var_psti_dn9 - locals.var_vbsz__blk438_dn9))), ((locals.var_beta_dn10 * assign63590_e98461) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbsz__blk438_dn10))), ((locals.var_beta_dn13 * assign63590_e98461) + (locals.var_beta * (locals.var_psti_dn13 - locals.var_vbsz__blk438_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63590_e98466;
        locals.var_t1_dn0 = assign63590_e98466_d_n0;
        locals.var_t1_dn2 = assign63590_e98466_d_n2;
        locals.var_t1_dn4 = assign63590_e98466_d_n4;
        locals.var_t1_dn5 = assign63590_e98466_d_n5;
        locals.var_t1_dn6 = assign63590_e98466_d_n6;
        locals.var_t1_dn7 = assign63590_e98466_d_n7;
        locals.var_t1_dn8 = assign63590_e98466_d_n8;
        locals.var_t1_dn9 = assign63590_e98466_d_n9;
        locals.var_t1_dn10 = assign63590_e98466_d_n10;
        locals.var_t1_dn13 = assign63590_e98466_d_n13;

        let (assign63600_e98484, assign63600_e98484_d_n0, assign63600_e98484_d_n2, assign63600_e98484_d_n4, assign63600_e98484_d_n5, assign63600_e98484_d_n6, assign63600_e98484_d_n7, assign63600_e98484_d_n8, assign63600_e98484_d_n9, assign63600_e98484_d_n10, assign63600_e98484_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63600_e98475: f64 = (locals.var_t1 * locals.var_t1);
        let assign63600_e98478: f64 = (4.0 * 0.01);
        let assign63600_e98480: f64 = (assign63600_e98478 * 0.01);
        let assign63600_e98481: f64 = (assign63600_e98475 + assign63600_e98480);
        let assign63600_e98482: f64 = (assign63600_e98481).sqrt();
        (assign63600_e98482, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63600_e98482)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63600_e98482)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63600_e98482)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63600_e98482)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63600_e98482)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63600_e98482)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63600_e98482)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63600_e98482)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63600_e98482)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign63600_e98482)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign63600_e98484;
        locals.var_tmf2_dn0 = assign63600_e98484_d_n0;
        locals.var_tmf2_dn2 = assign63600_e98484_d_n2;
        locals.var_tmf2_dn4 = assign63600_e98484_d_n4;
        locals.var_tmf2_dn5 = assign63600_e98484_d_n5;
        locals.var_tmf2_dn6 = assign63600_e98484_d_n6;
        locals.var_tmf2_dn7 = assign63600_e98484_d_n7;
        locals.var_tmf2_dn8 = assign63600_e98484_d_n8;
        locals.var_tmf2_dn9 = assign63600_e98484_d_n9;
        locals.var_tmf2_dn10 = assign63600_e98484_d_n10;
        locals.var_tmf2_dn13 = assign63600_e98484_d_n13;

        let (assign63610_e98499, assign63610_e98499_d_n0, assign63610_e98499_d_n2, assign63610_e98499_d_n4, assign63610_e98499_d_n5, assign63610_e98499_d_n6, assign63610_e98499_d_n7, assign63610_e98499_d_n8, assign63610_e98499_d_n9, assign63610_e98499_d_n10, assign63610_e98499_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63610_e98495: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63610_e98496: f64 = (1.0 + assign63610_e98495);
        let assign63610_e98497: f64 = (0.5 * assign63610_e98496);
        (assign63610_e98497, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign63610_e98499;
        locals.var_t0_dn0 = assign63610_e98499_d_n0;
        locals.var_t0_dn2 = assign63610_e98499_d_n2;
        locals.var_t0_dn4 = assign63610_e98499_d_n4;
        locals.var_t0_dn5 = assign63610_e98499_d_n5;
        locals.var_t0_dn6 = assign63610_e98499_d_n6;
        locals.var_t0_dn7 = assign63610_e98499_d_n7;
        locals.var_t0_dn8 = assign63610_e98499_d_n8;
        locals.var_t0_dn9 = assign63610_e98499_d_n9;
        locals.var_t0_dn10 = assign63610_e98499_d_n10;
        locals.var_t0_dn13 = assign63610_e98499_d_n13;

        let (assign63620_e98512, assign63620_e98512_d_n0, assign63620_e98512_d_n2, assign63620_e98512_d_n4, assign63620_e98512_d_n5, assign63620_e98512_d_n6, assign63620_e98512_d_n7, assign63620_e98512_d_n8, assign63620_e98512_d_n9, assign63620_e98512_d_n10, assign63620_e98512_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63620_e98509: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63620_e98510: f64 = (0.5 * assign63620_e98509);
        (assign63620_e98510, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63620_e98512;
        locals.var_t1_dn0 = assign63620_e98512_d_n0;
        locals.var_t1_dn2 = assign63620_e98512_d_n2;
        locals.var_t1_dn4 = assign63620_e98512_d_n4;
        locals.var_t1_dn5 = assign63620_e98512_d_n5;
        locals.var_t1_dn6 = assign63620_e98512_d_n6;
        locals.var_t1_dn7 = assign63620_e98512_d_n7;
        locals.var_t1_dn8 = assign63620_e98512_d_n8;
        locals.var_t1_dn9 = assign63620_e98512_d_n9;
        locals.var_t1_dn10 = assign63620_e98512_d_n10;
        locals.var_t1_dn13 = assign63620_e98512_d_n13;

        let assign63630_e98515: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1506 = assign63630_e98515;

        let (assign63640_e98526, assign63640_e98526_d_n0, assign63640_e98526_d_n2, assign63640_e98526_d_n4, assign63640_e98526_d_n5, assign63640_e98526_d_n6, assign63640_e98526_d_n7, assign63640_e98526_d_n8, assign63640_e98526_d_n9, assign63640_e98526_d_n10, assign63640_e98526_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1506 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63640_e98526;
        locals.var_t1_dn0 = assign63640_e98526_d_n0;
        locals.var_t1_dn2 = assign63640_e98526_d_n2;
        locals.var_t1_dn4 = assign63640_e98526_d_n4;
        locals.var_t1_dn5 = assign63640_e98526_d_n5;
        locals.var_t1_dn6 = assign63640_e98526_d_n6;
        locals.var_t1_dn7 = assign63640_e98526_d_n7;
        locals.var_t1_dn8 = assign63640_e98526_d_n8;
        locals.var_t1_dn9 = assign63640_e98526_d_n9;
        locals.var_t1_dn10 = assign63640_e98526_d_n10;
        locals.var_t1_dn13 = assign63640_e98526_d_n13;

        let (assign63650_e98537, assign63650_e98537_d_n0, assign63650_e98537_d_n2, assign63650_e98537_d_n4, assign63650_e98537_d_n5, assign63650_e98537_d_n6, assign63650_e98537_d_n7, assign63650_e98537_d_n8, assign63650_e98537_d_n9, assign63650_e98537_d_n10, assign63650_e98537_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1506 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign63650_e98537;
        locals.var_t0_dn0 = assign63650_e98537_d_n0;
        locals.var_t0_dn2 = assign63650_e98537_d_n2;
        locals.var_t0_dn4 = assign63650_e98537_d_n4;
        locals.var_t0_dn5 = assign63650_e98537_d_n5;
        locals.var_t0_dn6 = assign63650_e98537_d_n6;
        locals.var_t0_dn7 = assign63650_e98537_d_n7;
        locals.var_t0_dn8 = assign63650_e98537_d_n8;
        locals.var_t0_dn9 = assign63650_e98537_d_n9;
        locals.var_t0_dn10 = assign63650_e98537_d_n10;
        locals.var_t0_dn13 = assign63650_e98537_d_n13;

        let (assign63660_e98548, assign63660_e98548_d_n0, assign63660_e98548_d_n2, assign63660_e98548_d_n4, assign63660_e98548_d_n5, assign63660_e98548_d_n6, assign63660_e98548_d_n7, assign63660_e98548_d_n8, assign63660_e98548_d_n9, assign63660_e98548_d_n10, assign63660_e98548_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63660_e98546: f64 = (locals.var_t1 + 1e-25);
        (assign63660_e98546, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63660_e98548;
        locals.var_t1_dn0 = assign63660_e98548_d_n0;
        locals.var_t1_dn2 = assign63660_e98548_d_n2;
        locals.var_t1_dn4 = assign63660_e98548_d_n4;
        locals.var_t1_dn5 = assign63660_e98548_d_n5;
        locals.var_t1_dn6 = assign63660_e98548_d_n6;
        locals.var_t1_dn7 = assign63660_e98548_d_n7;
        locals.var_t1_dn8 = assign63660_e98548_d_n8;
        locals.var_t1_dn9 = assign63660_e98548_d_n9;
        locals.var_t1_dn10 = assign63660_e98548_d_n10;
        locals.var_t1_dn13 = assign63660_e98548_d_n13;

        let (assign63670_e98558, assign63670_e98558_d_n0, assign63670_e98558_d_n2, assign63670_e98558_d_n4, assign63670_e98558_d_n5, assign63670_e98558_d_n6, assign63670_e98558_d_n7, assign63670_e98558_d_n8, assign63670_e98558_d_n9, assign63670_e98558_d_n10, assign63670_e98558_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63670_e98556: f64 = (locals.var_t1).sqrt();
        (assign63670_e98556, (locals.var_t1_dn0 / (2.0 * assign63670_e98556)), (locals.var_t1_dn2 / (2.0 * assign63670_e98556)), (locals.var_t1_dn4 / (2.0 * assign63670_e98556)), (locals.var_t1_dn5 / (2.0 * assign63670_e98556)), (locals.var_t1_dn6 / (2.0 * assign63670_e98556)), (locals.var_t1_dn7 / (2.0 * assign63670_e98556)), (locals.var_t1_dn8 / (2.0 * assign63670_e98556)), (locals.var_t1_dn9 / (2.0 * assign63670_e98556)), (locals.var_t1_dn10 / (2.0 * assign63670_e98556)), (locals.var_t1_dn13 / (2.0 * assign63670_e98556)),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn4, locals.var_sq2sti_dn5, locals.var_sq2sti_dn6, locals.var_sq2sti_dn7, locals.var_sq2sti_dn8, locals.var_sq2sti_dn9, locals.var_sq2sti_dn10, locals.var_sq2sti_dn13,)
    }
};
        locals.var_sq2sti = assign63670_e98558;
        locals.var_sq2sti_dn0 = assign63670_e98558_d_n0;
        locals.var_sq2sti_dn2 = assign63670_e98558_d_n2;
        locals.var_sq2sti_dn4 = assign63670_e98558_d_n4;
        locals.var_sq2sti_dn5 = assign63670_e98558_d_n5;
        locals.var_sq2sti_dn6 = assign63670_e98558_d_n6;
        locals.var_sq2sti_dn7 = assign63670_e98558_d_n7;
        locals.var_sq2sti_dn8 = assign63670_e98558_d_n8;
        locals.var_sq2sti_dn9 = assign63670_e98558_d_n9;
        locals.var_sq2sti_dn10 = assign63670_e98558_d_n10;
        locals.var_sq2sti_dn13 = assign63670_e98558_d_n13;

        let (assign63680_e98569, assign63680_e98569_d_n0, assign63680_e98569_d_n2, assign63680_e98569_d_n4, assign63680_e98569_d_n5, assign63680_e98569_d_n6, assign63680_e98569_d_n7, assign63680_e98569_d_n8, assign63680_e98569_d_n9, assign63680_e98569_d_n10, assign63680_e98569_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63680_e98567: f64 = (0.5 / locals.var_sq2sti);
        (assign63680_e98567, (-((0.5 * locals.var_sq2sti_dn0) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn2) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn4) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn5) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn6) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn7) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn8) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn9) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn10) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn13) / (locals.var_sq2sti * locals.var_sq2sti))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign63680_e98569;
        locals.var_t2_dn0 = assign63680_e98569_d_n0;
        locals.var_t2_dn2 = assign63680_e98569_d_n2;
        locals.var_t2_dn4 = assign63680_e98569_d_n4;
        locals.var_t2_dn5 = assign63680_e98569_d_n5;
        locals.var_t2_dn6 = assign63680_e98569_d_n6;
        locals.var_t2_dn7 = assign63680_e98569_d_n7;
        locals.var_t2_dn8 = assign63680_e98569_d_n8;
        locals.var_t2_dn9 = assign63680_e98569_d_n9;
        locals.var_t2_dn10 = assign63680_e98569_d_n10;
        locals.var_t2_dn13 = assign63680_e98569_d_n13;

        let (assign63690_e98582, assign63690_e98582_d_n0, assign63690_e98582_d_n2, assign63690_e98582_d_n4, assign63690_e98582_d_n5, assign63690_e98582_d_n6, assign63690_e98582_d_n7, assign63690_e98582_d_n8, assign63690_e98582_d_n9, assign63690_e98582_d_n10, assign63690_e98582_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63690_e98579: f64 = (locals.var_sq1sti - locals.var_sq2sti);
        let assign63690_e98580: f64 = (locals.var_costi0 * assign63690_e98579);
        (assign63690_e98580, ((locals.var_costi0_dn0 * assign63690_e98579) + (locals.var_costi0 * (locals.var_sq1sti_dn0 - locals.var_sq2sti_dn0))), ((locals.var_costi0_dn2 * assign63690_e98579) + (locals.var_costi0 * (locals.var_sq1sti_dn2 - locals.var_sq2sti_dn2))), ((locals.var_costi0_dn4 * assign63690_e98579) + (locals.var_costi0 * (locals.var_sq1sti_dn4 - locals.var_sq2sti_dn4))), ((locals.var_costi0_dn5 * assign63690_e98579) + (locals.var_costi0 * (locals.var_sq1sti_dn5 - locals.var_sq2sti_dn5))), ((locals.var_costi0_dn6 * assign63690_e98579) + (locals.var_costi0 * (locals.var_sq1sti_dn6 - locals.var_sq2sti_dn6))), ((locals.var_costi0_dn7 * assign63690_e98579) + (locals.var_costi0 * (locals.var_sq1sti_dn7 - locals.var_sq2sti_dn7))), ((locals.var_costi0_dn8 * assign63690_e98579) + (locals.var_costi0 * (locals.var_sq1sti_dn8 - locals.var_sq2sti_dn8))), ((locals.var_costi0_dn9 * assign63690_e98579) + (locals.var_costi0 * (locals.var_sq1sti_dn9 - locals.var_sq2sti_dn9))), ((locals.var_costi0_dn10 * assign63690_e98579) + (locals.var_costi0 * (locals.var_sq1sti_dn10 - locals.var_sq2sti_dn10))), ((locals.var_costi0_dn13 * assign63690_e98579) + (locals.var_costi0 * (locals.var_sq1sti_dn13 - locals.var_sq2sti_dn13))),)
    } else {
        (locals.var_qn0sti, locals.var_qn0sti_dn0, locals.var_qn0sti_dn2, locals.var_qn0sti_dn4, locals.var_qn0sti_dn5, locals.var_qn0sti_dn6, locals.var_qn0sti_dn7, locals.var_qn0sti_dn8, locals.var_qn0sti_dn9, locals.var_qn0sti_dn10, locals.var_qn0sti_dn13,)
    }
};
        locals.var_qn0sti = assign63690_e98582;
        locals.var_qn0sti_dn0 = assign63690_e98582_d_n0;
        locals.var_qn0sti_dn2 = assign63690_e98582_d_n2;
        locals.var_qn0sti_dn4 = assign63690_e98582_d_n4;
        locals.var_qn0sti_dn5 = assign63690_e98582_d_n5;
        locals.var_qn0sti_dn6 = assign63690_e98582_d_n6;
        locals.var_qn0sti_dn7 = assign63690_e98582_d_n7;
        locals.var_qn0sti_dn8 = assign63690_e98582_d_n8;
        locals.var_qn0sti_dn9 = assign63690_e98582_d_n9;
        locals.var_qn0sti_dn10 = assign63690_e98582_d_n10;
        locals.var_qn0sti_dn13 = assign63690_e98582_d_n13;

        let (assign63700_e98593, assign63700_e98593_d_n0, assign63700_e98593_d_n2, assign63700_e98593_d_n4, assign63700_e98593_d_n5, assign63700_e98593_d_n6, assign63700_e98593_d_n7, assign63700_e98593_d_n8, assign63700_e98593_d_n9, assign63700_e98593_d_n10, assign63700_e98593_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63700_e98591: f64 = (locals.var_psasti - locals.var_psti);
        (assign63700_e98591, (locals.var_psasti_dn0 - locals.var_psti_dn0), (locals.var_psasti_dn2 - locals.var_psti_dn2), (locals.var_psasti_dn4 - locals.var_psti_dn4), (locals.var_psasti_dn5 - locals.var_psti_dn5), (locals.var_psasti_dn6 - locals.var_psti_dn6), (locals.var_psasti_dn7 - locals.var_psti_dn7), (locals.var_psasti_dn8 - locals.var_psti_dn8), (locals.var_psasti_dn9 - locals.var_psti_dn9), (locals.var_psasti_dn10 - locals.var_psti_dn10), (locals.var_psasti_dn13 - locals.var_psti_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63700_e98593;
        locals.var_t1_dn0 = assign63700_e98593_d_n0;
        locals.var_t1_dn2 = assign63700_e98593_d_n2;
        locals.var_t1_dn4 = assign63700_e98593_d_n4;
        locals.var_t1_dn5 = assign63700_e98593_d_n5;
        locals.var_t1_dn6 = assign63700_e98593_d_n6;
        locals.var_t1_dn7 = assign63700_e98593_d_n7;
        locals.var_t1_dn8 = assign63700_e98593_d_n8;
        locals.var_t1_dn9 = assign63700_e98593_d_n9;
        locals.var_t1_dn10 = assign63700_e98593_d_n10;
        locals.var_t1_dn13 = assign63700_e98593_d_n13;

        let (assign63710_e98611, assign63710_e98611_d_n0, assign63710_e98611_d_n2, assign63710_e98611_d_n4, assign63710_e98611_d_n5, assign63710_e98611_d_n6, assign63710_e98611_d_n7, assign63710_e98611_d_n8, assign63710_e98611_d_n9, assign63710_e98611_d_n10, assign63710_e98611_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63710_e98602: f64 = (locals.var_t1 * locals.var_t1);
        let assign63710_e98605: f64 = (4.0 * 0.1);
        let assign63710_e98607: f64 = (assign63710_e98605 * 0.1);
        let assign63710_e98608: f64 = (assign63710_e98602 + assign63710_e98607);
        let assign63710_e98609: f64 = (assign63710_e98608).sqrt();
        (assign63710_e98609, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63710_e98609)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63710_e98609)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63710_e98609)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63710_e98609)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63710_e98609)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63710_e98609)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63710_e98609)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63710_e98609)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63710_e98609)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign63710_e98609)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign63710_e98611;
        locals.var_tmf2_dn0 = assign63710_e98611_d_n0;
        locals.var_tmf2_dn2 = assign63710_e98611_d_n2;
        locals.var_tmf2_dn4 = assign63710_e98611_d_n4;
        locals.var_tmf2_dn5 = assign63710_e98611_d_n5;
        locals.var_tmf2_dn6 = assign63710_e98611_d_n6;
        locals.var_tmf2_dn7 = assign63710_e98611_d_n7;
        locals.var_tmf2_dn8 = assign63710_e98611_d_n8;
        locals.var_tmf2_dn9 = assign63710_e98611_d_n9;
        locals.var_tmf2_dn10 = assign63710_e98611_d_n10;
        locals.var_tmf2_dn13 = assign63710_e98611_d_n13;

        let (assign63720_e98626, assign63720_e98626_d_n0, assign63720_e98626_d_n2, assign63720_e98626_d_n4, assign63720_e98626_d_n5, assign63720_e98626_d_n6, assign63720_e98626_d_n7, assign63720_e98626_d_n8, assign63720_e98626_d_n9, assign63720_e98626_d_n10, assign63720_e98626_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63720_e98622: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63720_e98623: f64 = (1.0 + assign63720_e98622);
        let assign63720_e98624: f64 = (0.5 * assign63720_e98623);
        (assign63720_e98624, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign63720_e98626;
        locals.var_t2_dn0 = assign63720_e98626_d_n0;
        locals.var_t2_dn2 = assign63720_e98626_d_n2;
        locals.var_t2_dn4 = assign63720_e98626_d_n4;
        locals.var_t2_dn5 = assign63720_e98626_d_n5;
        locals.var_t2_dn6 = assign63720_e98626_d_n6;
        locals.var_t2_dn7 = assign63720_e98626_d_n7;
        locals.var_t2_dn8 = assign63720_e98626_d_n8;
        locals.var_t2_dn9 = assign63720_e98626_d_n9;
        locals.var_t2_dn10 = assign63720_e98626_d_n10;
        locals.var_t2_dn13 = assign63720_e98626_d_n13;

    }

    pub(super) fn stamp_transient_block_216(
        locals: &mut StampLocals,
    ) {
        let (assign63730_e98639, assign63730_e98639_d_n0, assign63730_e98639_d_n2, assign63730_e98639_d_n4, assign63730_e98639_d_n5, assign63730_e98639_d_n6, assign63730_e98639_d_n7, assign63730_e98639_d_n8, assign63730_e98639_d_n9, assign63730_e98639_d_n10, assign63730_e98639_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63730_e98636: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63730_e98637: f64 = (0.5 * assign63730_e98636);
        (assign63730_e98637, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63730_e98639;
        locals.var_t1_dn0 = assign63730_e98639_d_n0;
        locals.var_t1_dn2 = assign63730_e98639_d_n2;
        locals.var_t1_dn4 = assign63730_e98639_d_n4;
        locals.var_t1_dn5 = assign63730_e98639_d_n5;
        locals.var_t1_dn6 = assign63730_e98639_d_n6;
        locals.var_t1_dn7 = assign63730_e98639_d_n7;
        locals.var_t1_dn8 = assign63730_e98639_d_n8;
        locals.var_t1_dn9 = assign63730_e98639_d_n9;
        locals.var_t1_dn10 = assign63730_e98639_d_n10;
        locals.var_t1_dn13 = assign63730_e98639_d_n13;

        let assign63740_e98642: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1507 = assign63740_e98642;

        let (assign63750_e98653, assign63750_e98653_d_n0, assign63750_e98653_d_n2, assign63750_e98653_d_n4, assign63750_e98653_d_n5, assign63750_e98653_d_n6, assign63750_e98653_d_n7, assign63750_e98653_d_n8, assign63750_e98653_d_n9, assign63750_e98653_d_n10, assign63750_e98653_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63750_e98653;
        locals.var_t1_dn0 = assign63750_e98653_d_n0;
        locals.var_t1_dn2 = assign63750_e98653_d_n2;
        locals.var_t1_dn4 = assign63750_e98653_d_n4;
        locals.var_t1_dn5 = assign63750_e98653_d_n5;
        locals.var_t1_dn6 = assign63750_e98653_d_n6;
        locals.var_t1_dn7 = assign63750_e98653_d_n7;
        locals.var_t1_dn8 = assign63750_e98653_d_n8;
        locals.var_t1_dn9 = assign63750_e98653_d_n9;
        locals.var_t1_dn10 = assign63750_e98653_d_n10;
        locals.var_t1_dn13 = assign63750_e98653_d_n13;

        let (assign63760_e98664, assign63760_e98664_d_n0, assign63760_e98664_d_n2, assign63760_e98664_d_n4, assign63760_e98664_d_n5, assign63760_e98664_d_n6, assign63760_e98664_d_n7, assign63760_e98664_d_n8, assign63760_e98664_d_n9, assign63760_e98664_d_n10, assign63760_e98664_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign63760_e98664;
        locals.var_t2_dn0 = assign63760_e98664_d_n0;
        locals.var_t2_dn2 = assign63760_e98664_d_n2;
        locals.var_t2_dn4 = assign63760_e98664_d_n4;
        locals.var_t2_dn5 = assign63760_e98664_d_n5;
        locals.var_t2_dn6 = assign63760_e98664_d_n6;
        locals.var_t2_dn7 = assign63760_e98664_d_n7;
        locals.var_t2_dn8 = assign63760_e98664_d_n8;
        locals.var_t2_dn9 = assign63760_e98664_d_n9;
        locals.var_t2_dn10 = assign63760_e98664_d_n10;
        locals.var_t2_dn13 = assign63760_e98664_d_n13;

        let (assign63770_e98675, assign63770_e98675_d_n0, assign63770_e98675_d_n2, assign63770_e98675_d_n4, assign63770_e98675_d_n5, assign63770_e98675_d_n6, assign63770_e98675_d_n7, assign63770_e98675_d_n8, assign63770_e98675_d_n9, assign63770_e98675_d_n10, assign63770_e98675_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63770_e98673: f64 = (locals.var_t1 + 1e-25);
        (assign63770_e98673, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign63770_e98675;
        locals.var_t1_dn0 = assign63770_e98675_d_n0;
        locals.var_t1_dn2 = assign63770_e98675_d_n2;
        locals.var_t1_dn4 = assign63770_e98675_d_n4;
        locals.var_t1_dn5 = assign63770_e98675_d_n5;
        locals.var_t1_dn6 = assign63770_e98675_d_n6;
        locals.var_t1_dn7 = assign63770_e98675_d_n7;
        locals.var_t1_dn8 = assign63770_e98675_d_n8;
        locals.var_t1_dn9 = assign63770_e98675_d_n9;
        locals.var_t1_dn10 = assign63770_e98675_d_n10;
        locals.var_t1_dn13 = assign63770_e98675_d_n13;

        let (assign63780_e98686, assign63780_e98686_d_n0, assign63780_e98686_d_n2, assign63780_e98686_d_n4, assign63780_e98686_d_n5, assign63780_e98686_d_n6, assign63780_e98686_d_n7, assign63780_e98686_d_n8, assign63780_e98686_d_n9, assign63780_e98686_d_n10, assign63780_e98686_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63780_e98684: f64 = (locals.var_vds / locals.var_t1);
        (assign63780_e98684, (((locals.var_vds_dn0 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn2 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn4 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn5 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn6 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn7 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn8 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn9 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn10 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn13 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign63780_e98686;
        locals.var_tx_dn0 = assign63780_e98686_d_n0;
        locals.var_tx_dn2 = assign63780_e98686_d_n2;
        locals.var_tx_dn4 = assign63780_e98686_d_n4;
        locals.var_tx_dn5 = assign63780_e98686_d_n5;
        locals.var_tx_dn6 = assign63780_e98686_d_n6;
        locals.var_tx_dn7 = assign63780_e98686_d_n7;
        locals.var_tx_dn8 = assign63780_e98686_d_n8;
        locals.var_tx_dn9 = assign63780_e98686_d_n9;
        locals.var_tx_dn10 = assign63780_e98686_d_n10;
        locals.var_tx_dn13 = assign63780_e98686_d_n13;

        let (assign63790_e98699, assign63790_e98699_d_n0, assign63790_e98699_d_n2, assign63790_e98699_d_n4, assign63790_e98699_d_n5, assign63790_e98699_d_n6, assign63790_e98699_d_n7, assign63790_e98699_d_n8, assign63790_e98699_d_n9, assign63790_e98699_d_n10, assign63790_e98699_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63790_e98696: f64 = (locals.var_t1 * locals.var_t1);
        let assign63790_e98697: f64 = (1.0 / assign63790_e98696);
        (assign63790_e98697, (-(((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (assign63790_e98696 * assign63790_e98696))), (-(((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (assign63790_e98696 * assign63790_e98696))), (-(((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (assign63790_e98696 * assign63790_e98696))), (-(((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (assign63790_e98696 * assign63790_e98696))), (-(((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (assign63790_e98696 * assign63790_e98696))), (-(((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (assign63790_e98696 * assign63790_e98696))), (-(((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (assign63790_e98696 * assign63790_e98696))), (-(((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (assign63790_e98696 * assign63790_e98696))), (-(((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (assign63790_e98696 * assign63790_e98696))), (-(((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (assign63790_e98696 * assign63790_e98696))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign63790_e98699;
        locals.var_t2_dn0 = assign63790_e98699_d_n0;
        locals.var_t2_dn2 = assign63790_e98699_d_n2;
        locals.var_t2_dn4 = assign63790_e98699_d_n4;
        locals.var_t2_dn5 = assign63790_e98699_d_n5;
        locals.var_t2_dn6 = assign63790_e98699_d_n6;
        locals.var_t2_dn7 = assign63790_e98699_d_n7;
        locals.var_t2_dn8 = assign63790_e98699_d_n8;
        locals.var_t2_dn9 = assign63790_e98699_d_n9;
        locals.var_t2_dn10 = assign63790_e98699_d_n10;
        locals.var_t2_dn13 = assign63790_e98699_d_n13;

        let (assign63800_e98710, assign63800_e98710_d_n0, assign63800_e98710_d_n2, assign63800_e98710_d_n4, assign63800_e98710_d_n5, assign63800_e98710_d_n6, assign63800_e98710_d_n7, assign63800_e98710_d_n8, assign63800_e98710_d_n9, assign63800_e98710_d_n10, assign63800_e98710_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63800_e98708: f64 = (locals.var_tx * locals.var_tx);
        (assign63800_e98708, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)), ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)), ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn13 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign63800_e98710;
        locals.var_x2_dn0 = assign63800_e98710_d_n0;
        locals.var_x2_dn2 = assign63800_e98710_d_n2;
        locals.var_x2_dn4 = assign63800_e98710_d_n4;
        locals.var_x2_dn5 = assign63800_e98710_d_n5;
        locals.var_x2_dn6 = assign63800_e98710_d_n6;
        locals.var_x2_dn7 = assign63800_e98710_d_n7;
        locals.var_x2_dn8 = assign63800_e98710_d_n8;
        locals.var_x2_dn9 = assign63800_e98710_d_n9;
        locals.var_x2_dn10 = assign63800_e98710_d_n10;
        locals.var_x2_dn13 = assign63800_e98710_d_n13;

        let (assign63810_e98721, assign63810_e98721_d_n0, assign63810_e98721_d_n2, assign63810_e98721_d_n4, assign63810_e98721_d_n5, assign63810_e98721_d_n6, assign63810_e98721_d_n7, assign63810_e98721_d_n8, assign63810_e98721_d_n9, assign63810_e98721_d_n10, assign63810_e98721_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63810_e98719: f64 = 1.0;
        (assign63810_e98719, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign63810_e98721;
        locals.var_xmax2_dn0 = assign63810_e98721_d_n0;
        locals.var_xmax2_dn2 = assign63810_e98721_d_n2;
        locals.var_xmax2_dn4 = assign63810_e98721_d_n4;
        locals.var_xmax2_dn5 = assign63810_e98721_d_n5;
        locals.var_xmax2_dn6 = assign63810_e98721_d_n6;
        locals.var_xmax2_dn7 = assign63810_e98721_d_n7;
        locals.var_xmax2_dn8 = assign63810_e98721_d_n8;
        locals.var_xmax2_dn9 = assign63810_e98721_d_n9;
        locals.var_xmax2_dn10 = assign63810_e98721_d_n10;
        locals.var_xmax2_dn13 = assign63810_e98721_d_n13;

        let (assign63820_e98730, assign63820_e98730_d_n0, assign63820_e98730_d_n2, assign63820_e98730_d_n4, assign63820_e98730_d_n5, assign63820_e98730_d_n6, assign63820_e98730_d_n7, assign63820_e98730_d_n8, assign63820_e98730_d_n9, assign63820_e98730_d_n10, assign63820_e98730_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign63820_e98730;
        locals.var_xp_dn0 = assign63820_e98730_d_n0;
        locals.var_xp_dn2 = assign63820_e98730_d_n2;
        locals.var_xp_dn4 = assign63820_e98730_d_n4;
        locals.var_xp_dn5 = assign63820_e98730_d_n5;
        locals.var_xp_dn6 = assign63820_e98730_d_n6;
        locals.var_xp_dn7 = assign63820_e98730_d_n7;
        locals.var_xp_dn8 = assign63820_e98730_d_n8;
        locals.var_xp_dn9 = assign63820_e98730_d_n9;
        locals.var_xp_dn10 = assign63820_e98730_d_n10;
        locals.var_xp_dn13 = assign63820_e98730_d_n13;

        let (assign63830_e98739, assign63830_e98739_d_n0, assign63830_e98739_d_n2, assign63830_e98739_d_n4, assign63830_e98739_d_n5, assign63830_e98739_d_n6, assign63830_e98739_d_n7, assign63830_e98739_d_n8, assign63830_e98739_d_n9, assign63830_e98739_d_n10, assign63830_e98739_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign63830_e98739;
        locals.var_xmp_dn0 = assign63830_e98739_d_n0;
        locals.var_xmp_dn2 = assign63830_e98739_d_n2;
        locals.var_xmp_dn4 = assign63830_e98739_d_n4;
        locals.var_xmp_dn5 = assign63830_e98739_d_n5;
        locals.var_xmp_dn6 = assign63830_e98739_d_n6;
        locals.var_xmp_dn7 = assign63830_e98739_d_n7;
        locals.var_xmp_dn8 = assign63830_e98739_d_n8;
        locals.var_xmp_dn9 = assign63830_e98739_d_n9;
        locals.var_xmp_dn10 = assign63830_e98739_d_n10;
        locals.var_xmp_dn13 = assign63830_e98739_d_n13;

        let (assign63840_e98748,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign63840_e98748;

        let (assign63850_e98757,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign63850_e98757;

        let (assign63860_e98766, assign63860_e98766_d_n0, assign63860_e98766_d_n2, assign63860_e98766_d_n4, assign63860_e98766_d_n5, assign63860_e98766_d_n6, assign63860_e98766_d_n7, assign63860_e98766_d_n8, assign63860_e98766_d_n9, assign63860_e98766_d_n10, assign63860_e98766_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign63860_e98766;
        locals.var_arg_dn0 = assign63860_e98766_d_n0;
        locals.var_arg_dn2 = assign63860_e98766_d_n2;
        locals.var_arg_dn4 = assign63860_e98766_d_n4;
        locals.var_arg_dn5 = assign63860_e98766_d_n5;
        locals.var_arg_dn6 = assign63860_e98766_d_n6;
        locals.var_arg_dn7 = assign63860_e98766_d_n7;
        locals.var_arg_dn8 = assign63860_e98766_d_n8;
        locals.var_arg_dn9 = assign63860_e98766_d_n9;
        locals.var_arg_dn10 = assign63860_e98766_d_n10;
        locals.var_arg_dn13 = assign63860_e98766_d_n13;

        let (assign63870_e98775, assign63870_e98775_d_n0, assign63870_e98775_d_n2, assign63870_e98775_d_n4, assign63870_e98775_d_n5, assign63870_e98775_d_n6, assign63870_e98775_d_n7, assign63870_e98775_d_n8, assign63870_e98775_d_n9, assign63870_e98775_d_n10, assign63870_e98775_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign63870_e98775;
        locals.var_dnm_dn0 = assign63870_e98775_d_n0;
        locals.var_dnm_dn2 = assign63870_e98775_d_n2;
        locals.var_dnm_dn4 = assign63870_e98775_d_n4;
        locals.var_dnm_dn5 = assign63870_e98775_d_n5;
        locals.var_dnm_dn6 = assign63870_e98775_d_n6;
        locals.var_dnm_dn7 = assign63870_e98775_d_n7;
        locals.var_dnm_dn8 = assign63870_e98775_d_n8;
        locals.var_dnm_dn9 = assign63870_e98775_d_n9;
        locals.var_dnm_dn10 = assign63870_e98775_d_n10;
        locals.var_dnm_dn13 = assign63870_e98775_d_n13;

        let (assign63880_e98786, assign63880_e98786_d_n0, assign63880_e98786_d_n2, assign63880_e98786_d_n4, assign63880_e98786_d_n5, assign63880_e98786_d_n6, assign63880_e98786_d_n7, assign63880_e98786_d_n8, assign63880_e98786_d_n9, assign63880_e98786_d_n10, assign63880_e98786_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63880_e98784: f64 = (locals.var_xp * locals.var_x2);
        (assign63880_e98784, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign63880_e98786;
        locals.var_xp_dn0 = assign63880_e98786_d_n0;
        locals.var_xp_dn2 = assign63880_e98786_d_n2;
        locals.var_xp_dn4 = assign63880_e98786_d_n4;
        locals.var_xp_dn5 = assign63880_e98786_d_n5;
        locals.var_xp_dn6 = assign63880_e98786_d_n6;
        locals.var_xp_dn7 = assign63880_e98786_d_n7;
        locals.var_xp_dn8 = assign63880_e98786_d_n8;
        locals.var_xp_dn9 = assign63880_e98786_d_n9;
        locals.var_xp_dn10 = assign63880_e98786_d_n10;
        locals.var_xp_dn13 = assign63880_e98786_d_n13;

        let (assign63890_e98797, assign63890_e98797_d_n0, assign63890_e98797_d_n2, assign63890_e98797_d_n4, assign63890_e98797_d_n5, assign63890_e98797_d_n6, assign63890_e98797_d_n7, assign63890_e98797_d_n8, assign63890_e98797_d_n9, assign63890_e98797_d_n10, assign63890_e98797_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63890_e98795: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63890_e98795, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign63890_e98797;
        locals.var_xmp_dn0 = assign63890_e98797_d_n0;
        locals.var_xmp_dn2 = assign63890_e98797_d_n2;
        locals.var_xmp_dn4 = assign63890_e98797_d_n4;
        locals.var_xmp_dn5 = assign63890_e98797_d_n5;
        locals.var_xmp_dn6 = assign63890_e98797_d_n6;
        locals.var_xmp_dn7 = assign63890_e98797_d_n7;
        locals.var_xmp_dn8 = assign63890_e98797_d_n8;
        locals.var_xmp_dn9 = assign63890_e98797_d_n9;
        locals.var_xmp_dn10 = assign63890_e98797_d_n10;
        locals.var_xmp_dn13 = assign63890_e98797_d_n13;

        let (assign63900_e98808, assign63900_e98808_d_n0, assign63900_e98808_d_n2, assign63900_e98808_d_n4, assign63900_e98808_d_n5, assign63900_e98808_d_n6, assign63900_e98808_d_n7, assign63900_e98808_d_n8, assign63900_e98808_d_n9, assign63900_e98808_d_n10, assign63900_e98808_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63900_e98806: f64 = (locals.var_xp * locals.var_x2);
        (assign63900_e98806, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign63900_e98808;
        locals.var_xp_dn0 = assign63900_e98808_d_n0;
        locals.var_xp_dn2 = assign63900_e98808_d_n2;
        locals.var_xp_dn4 = assign63900_e98808_d_n4;
        locals.var_xp_dn5 = assign63900_e98808_d_n5;
        locals.var_xp_dn6 = assign63900_e98808_d_n6;
        locals.var_xp_dn7 = assign63900_e98808_d_n7;
        locals.var_xp_dn8 = assign63900_e98808_d_n8;
        locals.var_xp_dn9 = assign63900_e98808_d_n9;
        locals.var_xp_dn10 = assign63900_e98808_d_n10;
        locals.var_xp_dn13 = assign63900_e98808_d_n13;

        let (assign63910_e98819, assign63910_e98819_d_n0, assign63910_e98819_d_n2, assign63910_e98819_d_n4, assign63910_e98819_d_n5, assign63910_e98819_d_n6, assign63910_e98819_d_n7, assign63910_e98819_d_n8, assign63910_e98819_d_n9, assign63910_e98819_d_n10, assign63910_e98819_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63910_e98817: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63910_e98817, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign63910_e98819;
        locals.var_xmp_dn0 = assign63910_e98819_d_n0;
        locals.var_xmp_dn2 = assign63910_e98819_d_n2;
        locals.var_xmp_dn4 = assign63910_e98819_d_n4;
        locals.var_xmp_dn5 = assign63910_e98819_d_n5;
        locals.var_xmp_dn6 = assign63910_e98819_d_n6;
        locals.var_xmp_dn7 = assign63910_e98819_d_n7;
        locals.var_xmp_dn8 = assign63910_e98819_d_n8;
        locals.var_xmp_dn9 = assign63910_e98819_d_n9;
        locals.var_xmp_dn10 = assign63910_e98819_d_n10;
        locals.var_xmp_dn13 = assign63910_e98819_d_n13;

        let (assign63920_e98830, assign63920_e98830_d_n0, assign63920_e98830_d_n2, assign63920_e98830_d_n4, assign63920_e98830_d_n5, assign63920_e98830_d_n6, assign63920_e98830_d_n7, assign63920_e98830_d_n8, assign63920_e98830_d_n9, assign63920_e98830_d_n10, assign63920_e98830_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63920_e98828: f64 = (locals.var_xp * locals.var_x2);
        (assign63920_e98828, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign63920_e98830;
        locals.var_xp_dn0 = assign63920_e98830_d_n0;
        locals.var_xp_dn2 = assign63920_e98830_d_n2;
        locals.var_xp_dn4 = assign63920_e98830_d_n4;
        locals.var_xp_dn5 = assign63920_e98830_d_n5;
        locals.var_xp_dn6 = assign63920_e98830_d_n6;
        locals.var_xp_dn7 = assign63920_e98830_d_n7;
        locals.var_xp_dn8 = assign63920_e98830_d_n8;
        locals.var_xp_dn9 = assign63920_e98830_d_n9;
        locals.var_xp_dn10 = assign63920_e98830_d_n10;
        locals.var_xp_dn13 = assign63920_e98830_d_n13;

        let (assign63930_e98841, assign63930_e98841_d_n0, assign63930_e98841_d_n2, assign63930_e98841_d_n4, assign63930_e98841_d_n5, assign63930_e98841_d_n6, assign63930_e98841_d_n7, assign63930_e98841_d_n8, assign63930_e98841_d_n9, assign63930_e98841_d_n10, assign63930_e98841_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63930_e98839: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63930_e98839, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign63930_e98841;
        locals.var_xmp_dn0 = assign63930_e98841_d_n0;
        locals.var_xmp_dn2 = assign63930_e98841_d_n2;
        locals.var_xmp_dn4 = assign63930_e98841_d_n4;
        locals.var_xmp_dn5 = assign63930_e98841_d_n5;
        locals.var_xmp_dn6 = assign63930_e98841_d_n6;
        locals.var_xmp_dn7 = assign63930_e98841_d_n7;
        locals.var_xmp_dn8 = assign63930_e98841_d_n8;
        locals.var_xmp_dn9 = assign63930_e98841_d_n9;
        locals.var_xmp_dn10 = assign63930_e98841_d_n10;
        locals.var_xmp_dn13 = assign63930_e98841_d_n13;

        let (assign63940_e98852, assign63940_e98852_d_n0, assign63940_e98852_d_n2, assign63940_e98852_d_n4, assign63940_e98852_d_n5, assign63940_e98852_d_n6, assign63940_e98852_d_n7, assign63940_e98852_d_n8, assign63940_e98852_d_n9, assign63940_e98852_d_n10, assign63940_e98852_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63940_e98850: f64 = (locals.var_xp * locals.var_x2);
        (assign63940_e98850, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign63940_e98852;
        locals.var_xp_dn0 = assign63940_e98852_d_n0;
        locals.var_xp_dn2 = assign63940_e98852_d_n2;
        locals.var_xp_dn4 = assign63940_e98852_d_n4;
        locals.var_xp_dn5 = assign63940_e98852_d_n5;
        locals.var_xp_dn6 = assign63940_e98852_d_n6;
        locals.var_xp_dn7 = assign63940_e98852_d_n7;
        locals.var_xp_dn8 = assign63940_e98852_d_n8;
        locals.var_xp_dn9 = assign63940_e98852_d_n9;
        locals.var_xp_dn10 = assign63940_e98852_d_n10;
        locals.var_xp_dn13 = assign63940_e98852_d_n13;

        let (assign63950_e98863, assign63950_e98863_d_n0, assign63950_e98863_d_n2, assign63950_e98863_d_n4, assign63950_e98863_d_n5, assign63950_e98863_d_n6, assign63950_e98863_d_n7, assign63950_e98863_d_n8, assign63950_e98863_d_n9, assign63950_e98863_d_n10, assign63950_e98863_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63950_e98861: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63950_e98861, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign63950_e98863;
        locals.var_xmp_dn0 = assign63950_e98863_d_n0;
        locals.var_xmp_dn2 = assign63950_e98863_d_n2;
        locals.var_xmp_dn4 = assign63950_e98863_d_n4;
        locals.var_xmp_dn5 = assign63950_e98863_d_n5;
        locals.var_xmp_dn6 = assign63950_e98863_d_n6;
        locals.var_xmp_dn7 = assign63950_e98863_d_n7;
        locals.var_xmp_dn8 = assign63950_e98863_d_n8;
        locals.var_xmp_dn9 = assign63950_e98863_d_n9;
        locals.var_xmp_dn10 = assign63950_e98863_d_n10;
        locals.var_xmp_dn13 = assign63950_e98863_d_n13;

        let (assign63960_e98874, assign63960_e98874_d_n0, assign63960_e98874_d_n2, assign63960_e98874_d_n4, assign63960_e98874_d_n5, assign63960_e98874_d_n6, assign63960_e98874_d_n7, assign63960_e98874_d_n8, assign63960_e98874_d_n9, assign63960_e98874_d_n10, assign63960_e98874_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63960_e98872: f64 = (locals.var_xp + locals.var_xmp);
        (assign63960_e98872, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign63960_e98874;
        locals.var_arg_dn0 = assign63960_e98874_d_n0;
        locals.var_arg_dn2 = assign63960_e98874_d_n2;
        locals.var_arg_dn4 = assign63960_e98874_d_n4;
        locals.var_arg_dn5 = assign63960_e98874_d_n5;
        locals.var_arg_dn6 = assign63960_e98874_d_n6;
        locals.var_arg_dn7 = assign63960_e98874_d_n7;
        locals.var_arg_dn8 = assign63960_e98874_d_n8;
        locals.var_arg_dn9 = assign63960_e98874_d_n9;
        locals.var_arg_dn10 = assign63960_e98874_d_n10;
        locals.var_arg_dn13 = assign63960_e98874_d_n13;

        let (assign63970_e98883, assign63970_e98883_d_n0, assign63970_e98883_d_n2, assign63970_e98883_d_n4, assign63970_e98883_d_n5, assign63970_e98883_d_n6, assign63970_e98883_d_n7, assign63970_e98883_d_n8, assign63970_e98883_d_n9, assign63970_e98883_d_n10, assign63970_e98883_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign63970_e98883;
        locals.var_dnm_dn0 = assign63970_e98883_d_n0;
        locals.var_dnm_dn2 = assign63970_e98883_d_n2;
        locals.var_dnm_dn4 = assign63970_e98883_d_n4;
        locals.var_dnm_dn5 = assign63970_e98883_d_n5;
        locals.var_dnm_dn6 = assign63970_e98883_d_n6;
        locals.var_dnm_dn7 = assign63970_e98883_d_n7;
        locals.var_dnm_dn8 = assign63970_e98883_d_n8;
        locals.var_dnm_dn9 = assign63970_e98883_d_n9;
        locals.var_dnm_dn10 = assign63970_e98883_d_n10;
        locals.var_dnm_dn13 = assign63970_e98883_d_n13;

        let assign63980_e98898: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1508 = assign63980_e98898;

        let assign63990_e98901: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1509 = assign63990_e98901;

        let (assign64000_e98914,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1508 != 0.0)) && (locals.var_guard1509 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64000_e98914;

        let assign64010_e98917: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1510 = assign64010_e98917;

        let (assign64020_e98933,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1508 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64020_e98933;

        let assign64030_e98936: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1511 = assign64030_e98936;

        let (assign64040_e98955,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1508 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64040_e98955;

        let assign64050_e98958: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1512 = assign64050_e98958;

        let (assign64060_e98980,) = {
    if ((((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1508 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 == 0.0)) && (locals.var_guard1512 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64060_e98980;

    }

    pub(super) fn stamp_transient_block_217(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64070_e98991,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1508 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign64070_e98991;

        let mut assign64080_loop_guard: usize = 0;
        while {
            let assign64080_cond_e99003: f64 = if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1508 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign64080_cond_e99003 != 0.0
        } {
            assign64080_loop_guard += 1;
            assert!(assign64080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign64080_body0_e99015, assign64080_body0_e99015_d_n0, assign64080_body0_e99015_d_n2, assign64080_body0_e99015_d_n4, assign64080_body0_e99015_d_n5, assign64080_body0_e99015_d_n6, assign64080_body0_e99015_d_n7, assign64080_body0_e99015_d_n8, assign64080_body0_e99015_d_n9, assign64080_body0_e99015_d_n10, assign64080_body0_e99015_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1508 != 0.0)) {
        let assign64080_body0_e99013: f64 = (locals.var_dnm).sqrt();
        (assign64080_body0_e99013, (locals.var_dnm_dn0 / (2.0 * assign64080_body0_e99013)), (locals.var_dnm_dn2 / (2.0 * assign64080_body0_e99013)), (locals.var_dnm_dn4 / (2.0 * assign64080_body0_e99013)), (locals.var_dnm_dn5 / (2.0 * assign64080_body0_e99013)), (locals.var_dnm_dn6 / (2.0 * assign64080_body0_e99013)), (locals.var_dnm_dn7 / (2.0 * assign64080_body0_e99013)), (locals.var_dnm_dn8 / (2.0 * assign64080_body0_e99013)), (locals.var_dnm_dn9 / (2.0 * assign64080_body0_e99013)), (locals.var_dnm_dn10 / (2.0 * assign64080_body0_e99013)), (locals.var_dnm_dn13 / (2.0 * assign64080_body0_e99013)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign64080_body0_e99015;
            locals.var_dnm_dn0 = assign64080_body0_e99015_d_n0;
            locals.var_dnm_dn2 = assign64080_body0_e99015_d_n2;
            locals.var_dnm_dn4 = assign64080_body0_e99015_d_n4;
            locals.var_dnm_dn5 = assign64080_body0_e99015_d_n5;
            locals.var_dnm_dn6 = assign64080_body0_e99015_d_n6;
            locals.var_dnm_dn7 = assign64080_body0_e99015_d_n7;
            locals.var_dnm_dn8 = assign64080_body0_e99015_d_n8;
            locals.var_dnm_dn9 = assign64080_body0_e99015_d_n9;
            locals.var_dnm_dn10 = assign64080_body0_e99015_d_n10;
            locals.var_dnm_dn13 = assign64080_body0_e99015_d_n13;
            let (assign64080_body1_e99028,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1508 != 0.0)) {
        let assign64080_body1_e99026: f64 = (locals.var_m0 + 1.0);
        (assign64080_body1_e99026,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign64080_body1_e99028;
        }

        let (assign64090_e99051, assign64090_e99051_d_n0, assign64090_e99051_d_n2, assign64090_e99051_d_n4, assign64090_e99051_d_n5, assign64090_e99051_d_n6, assign64090_e99051_d_n7, assign64090_e99051_d_n8, assign64090_e99051_d_n9, assign64090_e99051_d_n10, assign64090_e99051_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1508 == 0.0)) {
        let (assign64090_e99049, assign64090_e99049_d_n0, assign64090_e99049_d_n2, assign64090_e99049_d_n4, assign64090_e99049_d_n5, assign64090_e99049_d_n6, assign64090_e99049_d_n7, assign64090_e99049_d_n8, assign64090_e99049_d_n9, assign64090_e99049_d_n10, assign64090_e99049_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign64090_e99046: f64 = (2.0 * 4.0);
                let assign64090_e99047: f64 = (1.0 / assign64090_e99046);
                let assign64090_e99048: f64 = (locals.var_dnm).powf(assign64090_e99047);
                (assign64090_e99048, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn0)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn2)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn4)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn5)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn6)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn7)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn8)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn9)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn10)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64090_e99047) as f64).is_finite() && ((assign64090_e99047) as f64).fract() == 0.0 { if assign64090_e99047 == 0.0 { 0.0 } else { (assign64090_e99047 * ((locals.var_dnm).powf(assign64090_e99047 - 1.0) * locals.var_dnm_dn13)) } } else { (assign64090_e99048 * (assign64090_e99047 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign64090_e99049, assign64090_e99049_d_n0, assign64090_e99049_d_n2, assign64090_e99049_d_n4, assign64090_e99049_d_n5, assign64090_e99049_d_n6, assign64090_e99049_d_n7, assign64090_e99049_d_n8, assign64090_e99049_d_n9, assign64090_e99049_d_n10, assign64090_e99049_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign64090_e99051;
        locals.var_dnm_dn0 = assign64090_e99051_d_n0;
        locals.var_dnm_dn2 = assign64090_e99051_d_n2;
        locals.var_dnm_dn4 = assign64090_e99051_d_n4;
        locals.var_dnm_dn5 = assign64090_e99051_d_n5;
        locals.var_dnm_dn6 = assign64090_e99051_d_n6;
        locals.var_dnm_dn7 = assign64090_e99051_d_n7;
        locals.var_dnm_dn8 = assign64090_e99051_d_n8;
        locals.var_dnm_dn9 = assign64090_e99051_d_n9;
        locals.var_dnm_dn10 = assign64090_e99051_d_n10;
        locals.var_dnm_dn13 = assign64090_e99051_d_n13;

        let (assign64100_e99062, assign64100_e99062_d_n0, assign64100_e99062_d_n2, assign64100_e99062_d_n4, assign64100_e99062_d_n5, assign64100_e99062_d_n6, assign64100_e99062_d_n7, assign64100_e99062_d_n8, assign64100_e99062_d_n9, assign64100_e99062_d_n10, assign64100_e99062_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64100_e99060: f64 = (1.0 / locals.var_dnm);
        (assign64100_e99060, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign64100_e99062;
        locals.var_dnm_dn0 = assign64100_e99062_d_n0;
        locals.var_dnm_dn2 = assign64100_e99062_d_n2;
        locals.var_dnm_dn4 = assign64100_e99062_d_n4;
        locals.var_dnm_dn5 = assign64100_e99062_d_n5;
        locals.var_dnm_dn6 = assign64100_e99062_d_n6;
        locals.var_dnm_dn7 = assign64100_e99062_d_n7;
        locals.var_dnm_dn8 = assign64100_e99062_d_n8;
        locals.var_dnm_dn9 = assign64100_e99062_d_n9;
        locals.var_dnm_dn10 = assign64100_e99062_d_n10;
        locals.var_dnm_dn13 = assign64100_e99062_d_n13;

        let (assign64110_e99075, assign64110_e99075_d_n0, assign64110_e99075_d_n2, assign64110_e99075_d_n4, assign64110_e99075_d_n5, assign64110_e99075_d_n6, assign64110_e99075_d_n7, assign64110_e99075_d_n8, assign64110_e99075_d_n9, assign64110_e99075_d_n10, assign64110_e99075_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64110_e99071: f64 = locals.var_tx;
        let assign64110_e99073: f64 = (assign64110_e99071 * locals.var_dnm);
        (assign64110_e99073, ((locals.var_tx_dn0 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn2)), ((locals.var_tx_dn4 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn4)), ((locals.var_tx_dn5 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn5)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn6)), ((locals.var_tx_dn7 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn7)), ((locals.var_tx_dn8 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn8)), ((locals.var_tx_dn9 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn9)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn10)), ((locals.var_tx_dn13 * locals.var_dnm) + (assign64110_e99071 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign64110_e99075;
        locals.var_ty_dn0 = assign64110_e99075_d_n0;
        locals.var_ty_dn2 = assign64110_e99075_d_n2;
        locals.var_ty_dn4 = assign64110_e99075_d_n4;
        locals.var_ty_dn5 = assign64110_e99075_d_n5;
        locals.var_ty_dn6 = assign64110_e99075_d_n6;
        locals.var_ty_dn7 = assign64110_e99075_d_n7;
        locals.var_ty_dn8 = assign64110_e99075_d_n8;
        locals.var_ty_dn9 = assign64110_e99075_d_n9;
        locals.var_ty_dn10 = assign64110_e99075_d_n10;
        locals.var_ty_dn13 = assign64110_e99075_d_n13;

        let (assign64120_e99090, assign64120_e99090_d_n0, assign64120_e99090_d_n2, assign64120_e99090_d_n4, assign64120_e99090_d_n5, assign64120_e99090_d_n6, assign64120_e99090_d_n7, assign64120_e99090_d_n8, assign64120_e99090_d_n9, assign64120_e99090_d_n10, assign64120_e99090_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64120_e99084: f64 = locals.var_xmp;
        let assign64120_e99086: f64 = (assign64120_e99084 * locals.var_dnm);
        let assign64120_e99088: f64 = (assign64120_e99086 / locals.var_arg);
        (assign64120_e99088, (((((locals.var_xmp_dn0 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn0)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn2 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn2)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn4 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn4)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn5 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn5)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn6 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn6)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn7 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn7)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn8 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn8)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn9 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn9)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn10 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn10)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn13 * locals.var_dnm) + (assign64120_e99084 * locals.var_dnm_dn13)) * locals.var_arg) - (assign64120_e99086 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign64120_e99090;
        locals.var_t2_dn0 = assign64120_e99090_d_n0;
        locals.var_t2_dn2 = assign64120_e99090_d_n2;
        locals.var_t2_dn4 = assign64120_e99090_d_n4;
        locals.var_t2_dn5 = assign64120_e99090_d_n5;
        locals.var_t2_dn6 = assign64120_e99090_d_n6;
        locals.var_t2_dn7 = assign64120_e99090_d_n7;
        locals.var_t2_dn8 = assign64120_e99090_d_n8;
        locals.var_t2_dn9 = assign64120_e99090_d_n9;
        locals.var_t2_dn10 = assign64120_e99090_d_n10;
        locals.var_t2_dn13 = assign64120_e99090_d_n13;

        let (assign64130_e99105, assign64130_e99105_d_n0, assign64130_e99105_d_n2, assign64130_e99105_d_n4, assign64130_e99105_d_n5, assign64130_e99105_d_n6, assign64130_e99105_d_n7, assign64130_e99105_d_n8, assign64130_e99105_d_n9, assign64130_e99105_d_n10, assign64130_e99105_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64130_e99099: f64 = (2.0 * locals.var_uc_wsti);
        let assign64130_e99101: f64 = (assign64130_e99099 * p.p7);
        let assign64130_e99103: f64 = (assign64130_e99101 * locals.var_beta_inv);
        (assign64130_e99103, ((((2.0 * locals.var_uc_wsti_dn0) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn0)), ((((2.0 * locals.var_uc_wsti_dn2) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn2)), ((((2.0 * locals.var_uc_wsti_dn4) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn4)), ((((2.0 * locals.var_uc_wsti_dn5) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn5)), ((((2.0 * locals.var_uc_wsti_dn6) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn6)), ((((2.0 * locals.var_uc_wsti_dn7) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn7)), ((((2.0 * locals.var_uc_wsti_dn8) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn8)), ((((2.0 * locals.var_uc_wsti_dn9) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn9)), ((((2.0 * locals.var_uc_wsti_dn10) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn10)), ((((2.0 * locals.var_uc_wsti_dn13) * p.p7) * locals.var_beta_inv) + (assign64130_e99101 * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_costi7, locals.var_costi7_dn0, locals.var_costi7_dn2, locals.var_costi7_dn4, locals.var_costi7_dn5, locals.var_costi7_dn6, locals.var_costi7_dn7, locals.var_costi7_dn8, locals.var_costi7_dn9, locals.var_costi7_dn10, locals.var_costi7_dn13,)
    }
};
        locals.var_costi7 = assign64130_e99105;
        locals.var_costi7_dn0 = assign64130_e99105_d_n0;
        locals.var_costi7_dn2 = assign64130_e99105_d_n2;
        locals.var_costi7_dn4 = assign64130_e99105_d_n4;
        locals.var_costi7_dn5 = assign64130_e99105_d_n5;
        locals.var_costi7_dn6 = assign64130_e99105_d_n6;
        locals.var_costi7_dn7 = assign64130_e99105_d_n7;
        locals.var_costi7_dn8 = assign64130_e99105_d_n8;
        locals.var_costi7_dn9 = assign64130_e99105_d_n9;
        locals.var_costi7_dn10 = assign64130_e99105_d_n10;
        locals.var_costi7_dn13 = assign64130_e99105_d_n13;

        let (assign64140_e99114, assign64140_e99114_d_n0, assign64140_e99114_d_n2, assign64140_e99114_d_n4, assign64140_e99114_d_n5, assign64140_e99114_d_n6, assign64140_e99114_d_n7, assign64140_e99114_d_n8, assign64140_e99114_d_n9, assign64140_e99114_d_n10, assign64140_e99114_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign64140_e99114;
        locals.var_t1_dn0 = assign64140_e99114_d_n0;
        locals.var_t1_dn2 = assign64140_e99114_d_n2;
        locals.var_t1_dn4 = assign64140_e99114_d_n4;
        locals.var_t1_dn5 = assign64140_e99114_d_n5;
        locals.var_t1_dn6 = assign64140_e99114_d_n6;
        locals.var_t1_dn7 = assign64140_e99114_d_n7;
        locals.var_t1_dn8 = assign64140_e99114_d_n8;
        locals.var_t1_dn9 = assign64140_e99114_d_n9;
        locals.var_t1_dn10 = assign64140_e99114_d_n10;
        locals.var_t1_dn13 = assign64140_e99114_d_n13;

        let (assign64150_e99131, assign64150_e99131_d_n0, assign64150_e99131_d_n2, assign64150_e99131_d_n4, assign64150_e99131_d_n5, assign64150_e99131_d_n6, assign64150_e99131_d_n7, assign64150_e99131_d_n8, assign64150_e99131_d_n9, assign64150_e99131_d_n10, assign64150_e99131_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64150_e99123: f64 = (locals.var_costi7 * locals.var_mu);
        let assign64150_e99125: f64 = (assign64150_e99123 * locals.var_qn0sti);
        let assign64150_e99127: f64 = (assign64150_e99125 * locals.var_ty);
        let assign64150_e99129: f64 = (assign64150_e99127 / locals.var_t1);
        (assign64150_e99129, (((((((((locals.var_costi7_dn0 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn0)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn0)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn0)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn2 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn2)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn2)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn2)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn4 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn4)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn4)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn4)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn5 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn5)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn5)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn5)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn6 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn6)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn6)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn6)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn7 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn7)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn7)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn7)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn8 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn8)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn8)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn8)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn9 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn9)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn9)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn9)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn10 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn10)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn10)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn10)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn13 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn13)) * locals.var_qn0sti) + (assign64150_e99123 * locals.var_qn0sti_dn13)) * locals.var_ty) + (assign64150_e99125 * locals.var_ty_dn13)) * locals.var_t1) - (assign64150_e99127 * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn4, locals.var_idssti_dn5, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn8, locals.var_idssti_dn9, locals.var_idssti_dn10, locals.var_idssti_dn13,)
    }
};
        locals.var_idssti = assign64150_e99131;
        locals.var_idssti_dn0 = assign64150_e99131_d_n0;
        locals.var_idssti_dn2 = assign64150_e99131_d_n2;
        locals.var_idssti_dn4 = assign64150_e99131_d_n4;
        locals.var_idssti_dn5 = assign64150_e99131_d_n5;
        locals.var_idssti_dn6 = assign64150_e99131_d_n6;
        locals.var_idssti_dn7 = assign64150_e99131_d_n7;
        locals.var_idssti_dn8 = assign64150_e99131_d_n8;
        locals.var_idssti_dn9 = assign64150_e99131_d_n9;
        locals.var_idssti_dn10 = assign64150_e99131_d_n10;
        locals.var_idssti_dn13 = assign64150_e99131_d_n13;

        let (assign64160_e99142, assign64160_e99142_d_n0, assign64160_e99142_d_n2, assign64160_e99142_d_n4, assign64160_e99142_d_n5, assign64160_e99142_d_n6, assign64160_e99142_d_n7, assign64160_e99142_d_n8, assign64160_e99142_d_n9, assign64160_e99142_d_n10, assign64160_e99142_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign64160_e99140: f64 = (locals.var_ids + locals.var_idssti);
        (assign64160_e99140, (locals.var_ids_dn0 + locals.var_idssti_dn0), (locals.var_ids_dn2 + locals.var_idssti_dn2), (locals.var_ids_dn4 + locals.var_idssti_dn4), (locals.var_ids_dn5 + locals.var_idssti_dn5), (locals.var_ids_dn6 + locals.var_idssti_dn6), (locals.var_ids_dn7 + locals.var_idssti_dn7), (locals.var_ids_dn8 + locals.var_idssti_dn8), (locals.var_ids_dn9 + locals.var_idssti_dn9), (locals.var_ids_dn10 + locals.var_idssti_dn10), (locals.var_ids_dn13 + locals.var_idssti_dn13),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign64160_e99142;
        locals.var_ids_dn0 = assign64160_e99142_d_n0;
        locals.var_ids_dn2 = assign64160_e99142_d_n2;
        locals.var_ids_dn4 = assign64160_e99142_d_n4;
        locals.var_ids_dn5 = assign64160_e99142_d_n5;
        locals.var_ids_dn6 = assign64160_e99142_d_n6;
        locals.var_ids_dn7 = assign64160_e99142_d_n7;
        locals.var_ids_dn8 = assign64160_e99142_d_n8;
        locals.var_ids_dn9 = assign64160_e99142_d_n9;
        locals.var_ids_dn10 = assign64160_e99142_d_n10;
        locals.var_ids_dn13 = assign64160_e99142_d_n13;

        let assign64170_e99153: f64 = if (((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign64170_e99153;

        let (assign64180_e99164, assign64180_e99164_d_n0, assign64180_e99164_d_n2, assign64180_e99164_d_n4, assign64180_e99164_d_n5, assign64180_e99164_d_n6, assign64180_e99164_d_n7, assign64180_e99164_d_n8, assign64180_e99164_d_n9, assign64180_e99164_d_n10, assign64180_e99164_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64180_e99162: f64 = (locals.var_vgvt * locals.var_vgvt);
        (assign64180_e99162, ((locals.var_vgvt_dn0 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn0)), ((locals.var_vgvt_dn2 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn2)), ((locals.var_vgvt_dn4 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn4)), ((locals.var_vgvt_dn5 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn5)), ((locals.var_vgvt_dn6 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn6)), ((locals.var_vgvt_dn7 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn7)), ((locals.var_vgvt_dn8 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn8)), ((locals.var_vgvt_dn9 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn9)), ((locals.var_vgvt_dn10 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn10)), ((locals.var_vgvt_dn13 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn13)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn13,)
    }
};
        locals.var_kusai00 = assign64180_e99164;
        locals.var_kusai00_dn0 = assign64180_e99164_d_n0;
        locals.var_kusai00_dn2 = assign64180_e99164_d_n2;
        locals.var_kusai00_dn4 = assign64180_e99164_d_n4;
        locals.var_kusai00_dn5 = assign64180_e99164_d_n5;
        locals.var_kusai00_dn6 = assign64180_e99164_d_n6;
        locals.var_kusai00_dn7 = assign64180_e99164_d_n7;
        locals.var_kusai00_dn8 = assign64180_e99164_d_n8;
        locals.var_kusai00_dn9 = assign64180_e99164_d_n9;
        locals.var_kusai00_dn10 = assign64180_e99164_d_n10;
        locals.var_kusai00_dn13 = assign64180_e99164_d_n13;

        let (assign64190_e99179, assign64190_e99179_d_n0, assign64190_e99179_d_n2, assign64190_e99179_d_n4, assign64190_e99179_d_n5, assign64190_e99179_d_n6, assign64190_e99179_d_n7, assign64190_e99179_d_n8, assign64190_e99179_d_n9, assign64190_e99179_d_n10, assign64190_e99179_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64190_e99173: f64 = (2.0 * locals.var_beta_inv);
        let assign64190_e99175: f64 = (assign64190_e99173 * locals.var_cox_inv);
        let assign64190_e99177: f64 = (assign64190_e99175 * locals.var_idd);
        (assign64190_e99177, (((((2.0 * locals.var_beta_inv_dn0) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn0)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn0)), (((((2.0 * locals.var_beta_inv_dn2) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn2)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn2)), (((((2.0 * locals.var_beta_inv_dn4) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn4)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn4)), (((((2.0 * locals.var_beta_inv_dn5) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn5)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn5)), (((((2.0 * locals.var_beta_inv_dn6) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn6)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn6)), (((((2.0 * locals.var_beta_inv_dn7) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn7)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn7)), (((((2.0 * locals.var_beta_inv_dn8) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn8)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn8)), (((((2.0 * locals.var_beta_inv_dn9) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn9)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn9)), (((((2.0 * locals.var_beta_inv_dn10) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn10)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn10)), (((((2.0 * locals.var_beta_inv_dn13) * locals.var_cox_inv) + (assign64190_e99173 * locals.var_cox_inv_dn13)) * locals.var_idd) + (assign64190_e99175 * locals.var_idd_dn13)),)
    } else {
        (locals.var_kusaidd, locals.var_kusaidd_dn0, locals.var_kusaidd_dn2, locals.var_kusaidd_dn4, locals.var_kusaidd_dn5, locals.var_kusaidd_dn6, locals.var_kusaidd_dn7, locals.var_kusaidd_dn8, locals.var_kusaidd_dn9, locals.var_kusaidd_dn10, locals.var_kusaidd_dn13,)
    }
};
        locals.var_kusaidd = assign64190_e99179;
        locals.var_kusaidd_dn0 = assign64190_e99179_d_n0;
        locals.var_kusaidd_dn2 = assign64190_e99179_d_n2;
        locals.var_kusaidd_dn4 = assign64190_e99179_d_n4;
        locals.var_kusaidd_dn5 = assign64190_e99179_d_n5;
        locals.var_kusaidd_dn6 = assign64190_e99179_d_n6;
        locals.var_kusaidd_dn7 = assign64190_e99179_d_n7;
        locals.var_kusaidd_dn8 = assign64190_e99179_d_n8;
        locals.var_kusaidd_dn9 = assign64190_e99179_d_n9;
        locals.var_kusaidd_dn10 = assign64190_e99179_d_n10;
        locals.var_kusaidd_dn13 = assign64190_e99179_d_n13;

        let (assign64200_e99190, assign64200_e99190_d_n0, assign64200_e99190_d_n2, assign64200_e99190_d_n4, assign64200_e99190_d_n5, assign64200_e99190_d_n6, assign64200_e99190_d_n7, assign64200_e99190_d_n8, assign64200_e99190_d_n9, assign64200_e99190_d_n10, assign64200_e99190_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64200_e99188: f64 = (locals.var_kusai00 - locals.var_kusaidd);
        (assign64200_e99188, (locals.var_kusai00_dn0 - locals.var_kusaidd_dn0), (locals.var_kusai00_dn2 - locals.var_kusaidd_dn2), (locals.var_kusai00_dn4 - locals.var_kusaidd_dn4), (locals.var_kusai00_dn5 - locals.var_kusaidd_dn5), (locals.var_kusai00_dn6 - locals.var_kusaidd_dn6), (locals.var_kusai00_dn7 - locals.var_kusaidd_dn7), (locals.var_kusai00_dn8 - locals.var_kusaidd_dn8), (locals.var_kusai00_dn9 - locals.var_kusaidd_dn9), (locals.var_kusai00_dn10 - locals.var_kusaidd_dn10), (locals.var_kusai00_dn13 - locals.var_kusaidd_dn13),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn13,)
    }
};
        locals.var_kusail = assign64200_e99190;
        locals.var_kusail_dn0 = assign64200_e99190_d_n0;
        locals.var_kusail_dn2 = assign64200_e99190_d_n2;
        locals.var_kusail_dn4 = assign64200_e99190_d_n4;
        locals.var_kusail_dn5 = assign64200_e99190_d_n5;
        locals.var_kusail_dn6 = assign64200_e99190_d_n6;
        locals.var_kusail_dn7 = assign64200_e99190_d_n7;
        locals.var_kusail_dn8 = assign64200_e99190_d_n8;
        locals.var_kusail_dn9 = assign64200_e99190_d_n9;
        locals.var_kusail_dn10 = assign64200_e99190_d_n10;
        locals.var_kusail_dn13 = assign64200_e99190_d_n13;

        let (assign64210_e99208, assign64210_e99208_d_n0, assign64210_e99208_d_n2, assign64210_e99208_d_n4, assign64210_e99208_d_n5, assign64210_e99208_d_n6, assign64210_e99208_d_n7, assign64210_e99208_d_n8, assign64210_e99208_d_n9, assign64210_e99208_d_n10, assign64210_e99208_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64210_e99199: f64 = (locals.var_kusai00 * locals.var_kusai00);
        let assign64210_e99202: f64 = (4.0 * 0.001);
        let assign64210_e99204: f64 = (assign64210_e99202 * 0.001);
        let assign64210_e99205: f64 = (assign64210_e99199 + assign64210_e99204);
        let assign64210_e99206: f64 = (assign64210_e99205).sqrt();
        (assign64210_e99206, (((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn9 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn9)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)) / (2.0 * assign64210_e99206)), (((locals.var_kusai00_dn13 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn13)) / (2.0 * assign64210_e99206)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign64210_e99208;
        locals.var_tmf2_dn0 = assign64210_e99208_d_n0;
        locals.var_tmf2_dn2 = assign64210_e99208_d_n2;
        locals.var_tmf2_dn4 = assign64210_e99208_d_n4;
        locals.var_tmf2_dn5 = assign64210_e99208_d_n5;
        locals.var_tmf2_dn6 = assign64210_e99208_d_n6;
        locals.var_tmf2_dn7 = assign64210_e99208_d_n7;
        locals.var_tmf2_dn8 = assign64210_e99208_d_n8;
        locals.var_tmf2_dn9 = assign64210_e99208_d_n9;
        locals.var_tmf2_dn10 = assign64210_e99208_d_n10;
        locals.var_tmf2_dn13 = assign64210_e99208_d_n13;

        let (assign64220_e99223, assign64220_e99223_d_n0, assign64220_e99223_d_n2, assign64220_e99223_d_n4, assign64220_e99223_d_n5, assign64220_e99223_d_n6, assign64220_e99223_d_n7, assign64220_e99223_d_n8, assign64220_e99223_d_n9, assign64220_e99223_d_n10, assign64220_e99223_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64220_e99219: f64 = (locals.var_kusai00 / locals.var_tmf2);
        let assign64220_e99220: f64 = (1.0 + assign64220_e99219);
        let assign64220_e99221: f64 = (0.5 * assign64220_e99220);
        (assign64220_e99221, (0.5 * (((locals.var_kusai00_dn0 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn2 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn4 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn5 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn6 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn7 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn8 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn9 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn10 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn13 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign64220_e99223;
        locals.var_t0_dn0 = assign64220_e99223_d_n0;
        locals.var_t0_dn2 = assign64220_e99223_d_n2;
        locals.var_t0_dn4 = assign64220_e99223_d_n4;
        locals.var_t0_dn5 = assign64220_e99223_d_n5;
        locals.var_t0_dn6 = assign64220_e99223_d_n6;
        locals.var_t0_dn7 = assign64220_e99223_d_n7;
        locals.var_t0_dn8 = assign64220_e99223_d_n8;
        locals.var_t0_dn9 = assign64220_e99223_d_n9;
        locals.var_t0_dn10 = assign64220_e99223_d_n10;
        locals.var_t0_dn13 = assign64220_e99223_d_n13;

        let (assign64230_e99236, assign64230_e99236_d_n0, assign64230_e99236_d_n2, assign64230_e99236_d_n4, assign64230_e99236_d_n5, assign64230_e99236_d_n6, assign64230_e99236_d_n7, assign64230_e99236_d_n8, assign64230_e99236_d_n9, assign64230_e99236_d_n10, assign64230_e99236_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64230_e99233: f64 = (locals.var_kusai00 + locals.var_tmf2);
        let assign64230_e99234: f64 = (0.5 * assign64230_e99233);
        (assign64230_e99234, (0.5 * (locals.var_kusai00_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_kusai00_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_kusai00_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_kusai00_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_kusai00_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_kusai00_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_kusai00_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_kusai00_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_kusai00_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_kusai00_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn13,)
    }
};
        locals.var_kusai00 = assign64230_e99236;
        locals.var_kusai00_dn0 = assign64230_e99236_d_n0;
        locals.var_kusai00_dn2 = assign64230_e99236_d_n2;
        locals.var_kusai00_dn4 = assign64230_e99236_d_n4;
        locals.var_kusai00_dn5 = assign64230_e99236_d_n5;
        locals.var_kusai00_dn6 = assign64230_e99236_d_n6;
        locals.var_kusai00_dn7 = assign64230_e99236_d_n7;
        locals.var_kusai00_dn8 = assign64230_e99236_d_n8;
        locals.var_kusai00_dn9 = assign64230_e99236_d_n9;
        locals.var_kusai00_dn10 = assign64230_e99236_d_n10;
        locals.var_kusai00_dn13 = assign64230_e99236_d_n13;

        let assign64240_e99239: f64 = if locals.var_kusai00 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1514 = assign64240_e99239;

        let (assign64250_e99250, assign64250_e99250_d_n0, assign64250_e99250_d_n2, assign64250_e99250_d_n4, assign64250_e99250_d_n5, assign64250_e99250_d_n6, assign64250_e99250_d_n7, assign64250_e99250_d_n8, assign64250_e99250_d_n9, assign64250_e99250_d_n10, assign64250_e99250_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn13,)
    }
};
        locals.var_kusai00 = assign64250_e99250;
        locals.var_kusai00_dn0 = assign64250_e99250_d_n0;
        locals.var_kusai00_dn2 = assign64250_e99250_d_n2;
        locals.var_kusai00_dn4 = assign64250_e99250_d_n4;
        locals.var_kusai00_dn5 = assign64250_e99250_d_n5;
        locals.var_kusai00_dn6 = assign64250_e99250_d_n6;
        locals.var_kusai00_dn7 = assign64250_e99250_d_n7;
        locals.var_kusai00_dn8 = assign64250_e99250_d_n8;
        locals.var_kusai00_dn9 = assign64250_e99250_d_n9;
        locals.var_kusai00_dn10 = assign64250_e99250_d_n10;
        locals.var_kusai00_dn13 = assign64250_e99250_d_n13;

        let (assign64260_e99261, assign64260_e99261_d_n0, assign64260_e99261_d_n2, assign64260_e99261_d_n4, assign64260_e99261_d_n5, assign64260_e99261_d_n6, assign64260_e99261_d_n7, assign64260_e99261_d_n8, assign64260_e99261_d_n9, assign64260_e99261_d_n10, assign64260_e99261_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign64260_e99261;
        locals.var_t0_dn0 = assign64260_e99261_d_n0;
        locals.var_t0_dn2 = assign64260_e99261_d_n2;
        locals.var_t0_dn4 = assign64260_e99261_d_n4;
        locals.var_t0_dn5 = assign64260_e99261_d_n5;
        locals.var_t0_dn6 = assign64260_e99261_d_n6;
        locals.var_t0_dn7 = assign64260_e99261_d_n7;
        locals.var_t0_dn8 = assign64260_e99261_d_n8;
        locals.var_t0_dn9 = assign64260_e99261_d_n9;
        locals.var_t0_dn10 = assign64260_e99261_d_n10;
        locals.var_t0_dn13 = assign64260_e99261_d_n13;

        let (assign64270_e99279, assign64270_e99279_d_n0, assign64270_e99279_d_n2, assign64270_e99279_d_n4, assign64270_e99279_d_n5, assign64270_e99279_d_n6, assign64270_e99279_d_n7, assign64270_e99279_d_n8, assign64270_e99279_d_n9, assign64270_e99279_d_n10, assign64270_e99279_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64270_e99270: f64 = (locals.var_kusail * locals.var_kusail);
        let assign64270_e99273: f64 = (4.0 * 0.001);
        let assign64270_e99275: f64 = (assign64270_e99273 * 0.001);
        let assign64270_e99276: f64 = (assign64270_e99270 + assign64270_e99275);
        let assign64270_e99277: f64 = (assign64270_e99276).sqrt();
        (assign64270_e99277, (((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn9 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn9)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)) / (2.0 * assign64270_e99277)), (((locals.var_kusail_dn13 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn13)) / (2.0 * assign64270_e99277)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign64270_e99279;
        locals.var_tmf2_dn0 = assign64270_e99279_d_n0;
        locals.var_tmf2_dn2 = assign64270_e99279_d_n2;
        locals.var_tmf2_dn4 = assign64270_e99279_d_n4;
        locals.var_tmf2_dn5 = assign64270_e99279_d_n5;
        locals.var_tmf2_dn6 = assign64270_e99279_d_n6;
        locals.var_tmf2_dn7 = assign64270_e99279_d_n7;
        locals.var_tmf2_dn8 = assign64270_e99279_d_n8;
        locals.var_tmf2_dn9 = assign64270_e99279_d_n9;
        locals.var_tmf2_dn10 = assign64270_e99279_d_n10;
        locals.var_tmf2_dn13 = assign64270_e99279_d_n13;

        let (assign64280_e99294, assign64280_e99294_d_n0, assign64280_e99294_d_n2, assign64280_e99294_d_n4, assign64280_e99294_d_n5, assign64280_e99294_d_n6, assign64280_e99294_d_n7, assign64280_e99294_d_n8, assign64280_e99294_d_n9, assign64280_e99294_d_n10, assign64280_e99294_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64280_e99290: f64 = (locals.var_kusail / locals.var_tmf2);
        let assign64280_e99291: f64 = (1.0 + assign64280_e99290);
        let assign64280_e99292: f64 = (0.5 * assign64280_e99291);
        (assign64280_e99292, (0.5 * (((locals.var_kusail_dn0 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn2 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn4 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn5 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn6 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn7 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn8 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn9 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn10 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn13 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign64280_e99294;
        locals.var_t0_dn0 = assign64280_e99294_d_n0;
        locals.var_t0_dn2 = assign64280_e99294_d_n2;
        locals.var_t0_dn4 = assign64280_e99294_d_n4;
        locals.var_t0_dn5 = assign64280_e99294_d_n5;
        locals.var_t0_dn6 = assign64280_e99294_d_n6;
        locals.var_t0_dn7 = assign64280_e99294_d_n7;
        locals.var_t0_dn8 = assign64280_e99294_d_n8;
        locals.var_t0_dn9 = assign64280_e99294_d_n9;
        locals.var_t0_dn10 = assign64280_e99294_d_n10;
        locals.var_t0_dn13 = assign64280_e99294_d_n13;

        let (assign64290_e99307, assign64290_e99307_d_n0, assign64290_e99307_d_n2, assign64290_e99307_d_n4, assign64290_e99307_d_n5, assign64290_e99307_d_n6, assign64290_e99307_d_n7, assign64290_e99307_d_n8, assign64290_e99307_d_n9, assign64290_e99307_d_n10, assign64290_e99307_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64290_e99304: f64 = (locals.var_kusail + locals.var_tmf2);
        let assign64290_e99305: f64 = (0.5 * assign64290_e99304);
        (assign64290_e99305, (0.5 * (locals.var_kusail_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_kusail_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_kusail_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_kusail_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_kusail_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_kusail_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_kusail_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_kusail_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_kusail_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_kusail_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn13,)
    }
};
        locals.var_kusail = assign64290_e99307;
        locals.var_kusail_dn0 = assign64290_e99307_d_n0;
        locals.var_kusail_dn2 = assign64290_e99307_d_n2;
        locals.var_kusail_dn4 = assign64290_e99307_d_n4;
        locals.var_kusail_dn5 = assign64290_e99307_d_n5;
        locals.var_kusail_dn6 = assign64290_e99307_d_n6;
        locals.var_kusail_dn7 = assign64290_e99307_d_n7;
        locals.var_kusail_dn8 = assign64290_e99307_d_n8;
        locals.var_kusail_dn9 = assign64290_e99307_d_n9;
        locals.var_kusail_dn10 = assign64290_e99307_d_n10;
        locals.var_kusail_dn13 = assign64290_e99307_d_n13;

        let assign64300_e99310: f64 = if locals.var_kusail < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1515 = assign64300_e99310;

        let (assign64310_e99321, assign64310_e99321_d_n0, assign64310_e99321_d_n2, assign64310_e99321_d_n4, assign64310_e99321_d_n5, assign64310_e99321_d_n6, assign64310_e99321_d_n7, assign64310_e99321_d_n8, assign64310_e99321_d_n9, assign64310_e99321_d_n10, assign64310_e99321_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn13,)
    }
};
        locals.var_kusail = assign64310_e99321;
        locals.var_kusail_dn0 = assign64310_e99321_d_n0;
        locals.var_kusail_dn2 = assign64310_e99321_d_n2;
        locals.var_kusail_dn4 = assign64310_e99321_d_n4;
        locals.var_kusail_dn5 = assign64310_e99321_d_n5;
        locals.var_kusail_dn6 = assign64310_e99321_d_n6;
        locals.var_kusail_dn7 = assign64310_e99321_d_n7;
        locals.var_kusail_dn8 = assign64310_e99321_d_n8;
        locals.var_kusail_dn9 = assign64310_e99321_d_n9;
        locals.var_kusail_dn10 = assign64310_e99321_d_n10;
        locals.var_kusail_dn13 = assign64310_e99321_d_n13;

        let (assign64320_e99332, assign64320_e99332_d_n0, assign64320_e99332_d_n2, assign64320_e99332_d_n4, assign64320_e99332_d_n5, assign64320_e99332_d_n6, assign64320_e99332_d_n7, assign64320_e99332_d_n8, assign64320_e99332_d_n9, assign64320_e99332_d_n10, assign64320_e99332_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign64320_e99332;
        locals.var_t0_dn0 = assign64320_e99332_d_n0;
        locals.var_t0_dn2 = assign64320_e99332_d_n2;
        locals.var_t0_dn4 = assign64320_e99332_d_n4;
        locals.var_t0_dn5 = assign64320_e99332_d_n5;
        locals.var_t0_dn6 = assign64320_e99332_d_n6;
        locals.var_t0_dn7 = assign64320_e99332_d_n7;
        locals.var_t0_dn8 = assign64320_e99332_d_n8;
        locals.var_t0_dn9 = assign64320_e99332_d_n9;
        locals.var_t0_dn10 = assign64320_e99332_d_n10;
        locals.var_t0_dn13 = assign64320_e99332_d_n13;

    }

    pub(super) fn stamp_transient_block_218(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64330_e99343, assign64330_e99343_d_n0, assign64330_e99343_d_n2, assign64330_e99343_d_n4, assign64330_e99343_d_n5, assign64330_e99343_d_n6, assign64330_e99343_d_n7, assign64330_e99343_d_n8, assign64330_e99343_d_n9, assign64330_e99343_d_n10, assign64330_e99343_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign64330_e99341: f64 = (locals.var_kusai00 - locals.var_kusail);
        (assign64330_e99341, (locals.var_kusai00_dn0 - locals.var_kusail_dn0), (locals.var_kusai00_dn2 - locals.var_kusail_dn2), (locals.var_kusai00_dn4 - locals.var_kusail_dn4), (locals.var_kusai00_dn5 - locals.var_kusail_dn5), (locals.var_kusai00_dn6 - locals.var_kusail_dn6), (locals.var_kusai00_dn7 - locals.var_kusail_dn7), (locals.var_kusai00_dn8 - locals.var_kusail_dn8), (locals.var_kusai00_dn9 - locals.var_kusail_dn9), (locals.var_kusai00_dn10 - locals.var_kusail_dn10), (locals.var_kusai00_dn13 - locals.var_kusail_dn13),)
    } else {
        (locals.var_kusai00l, locals.var_kusai00l_dn0, locals.var_kusai00l_dn2, locals.var_kusai00l_dn4, locals.var_kusai00l_dn5, locals.var_kusai00l_dn6, locals.var_kusai00l_dn7, locals.var_kusai00l_dn8, locals.var_kusai00l_dn9, locals.var_kusai00l_dn10, locals.var_kusai00l_dn13,)
    }
};
        locals.var_kusai00l = assign64330_e99343;
        locals.var_kusai00l_dn0 = assign64330_e99343_d_n0;
        locals.var_kusai00l_dn2 = assign64330_e99343_d_n2;
        locals.var_kusai00l_dn4 = assign64330_e99343_d_n4;
        locals.var_kusai00l_dn5 = assign64330_e99343_d_n5;
        locals.var_kusai00l_dn6 = assign64330_e99343_d_n6;
        locals.var_kusai00l_dn7 = assign64330_e99343_d_n7;
        locals.var_kusai00l_dn8 = assign64330_e99343_d_n8;
        locals.var_kusai00l_dn9 = assign64330_e99343_d_n9;
        locals.var_kusai00l_dn10 = assign64330_e99343_d_n10;
        locals.var_kusai00l_dn13 = assign64330_e99343_d_n13;

        let assign64340_e99347: f64 = (10.0 * 2.220446049250313e-16);
        let assign64340_e99352: f64 = (10.0 * 2.220446049250313e-16);
        let assign64340_e99354: f64 = if ((locals.var_qn0 < assign64340_e99347) || (locals.var_kusai00l < assign64340_e99352)) { 1.0 } else { 0.0 };
        locals.var_guard1516 = assign64340_e99354;

        let (assign64350_e99365,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1516 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign64350_e99365;

        let (assign64360_e99377,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1516 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign64360_e99377;

        let (assign64370_e99384,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_end_of_part_1 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign64370_e99384;

        let assign64380_e99391: f64 = if ((locals.var_flg_noqi == 0.0) && (locals.var_vgvt > 1e-12)) { 1.0 } else { 0.0 };
        locals.var_guard1517 = assign64380_e99391;

        let (assign64390_e99404, assign64390_e99404_d_n0, assign64390_e99404_d_n2, assign64390_e99404_d_n4, assign64390_e99404_d_n5, assign64390_e99404_d_n6, assign64390_e99404_d_n7, assign64390_e99404_d_n8, assign64390_e99404_d_n9, assign64390_e99404_d_n10, assign64390_e99404_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1517 != 0.0)) {
        let assign64390_e99398: f64 = (locals.var_fac1 * locals.var_beta);
        let assign64390_e99401: f64 = (2.0 * locals.var_xi0p12);
        let assign64390_e99402: f64 = (assign64390_e99398 / assign64390_e99401);
        (assign64390_e99402, (((((locals.var_fac1_dn0 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn0)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn0))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn2 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn2)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn2))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn4 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn4)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn4))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn5 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn5)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn5))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn6 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn6)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn6))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn7 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn7)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn7))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn8 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn8)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn8))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn9 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn9)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn9))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn10 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn10)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn10))) / (assign64390_e99401 * assign64390_e99401)), (((((locals.var_fac1_dn13 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn13)) * assign64390_e99401) - (assign64390_e99398 * (2.0 * locals.var_xi0p12_dn13))) / (assign64390_e99401 * assign64390_e99401)),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn13,)
    }
};
        locals.var_delta = assign64390_e99404;
        locals.var_delta_dn0 = assign64390_e99404_d_n0;
        locals.var_delta_dn2 = assign64390_e99404_d_n2;
        locals.var_delta_dn4 = assign64390_e99404_d_n4;
        locals.var_delta_dn5 = assign64390_e99404_d_n5;
        locals.var_delta_dn6 = assign64390_e99404_d_n6;
        locals.var_delta_dn7 = assign64390_e99404_d_n7;
        locals.var_delta_dn8 = assign64390_e99404_d_n8;
        locals.var_delta_dn9 = assign64390_e99404_d_n9;
        locals.var_delta_dn10 = assign64390_e99404_d_n10;
        locals.var_delta_dn13 = assign64390_e99404_d_n13;

        let (assign64400_e99417, assign64400_e99417_d_n0, assign64400_e99417_d_n2, assign64400_e99417_d_n4, assign64400_e99417_d_n5, assign64400_e99417_d_n6, assign64400_e99417_d_n7, assign64400_e99417_d_n8, assign64400_e99417_d_n9, assign64400_e99417_d_n10, assign64400_e99417_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1517 != 0.0)) {
        let assign64400_e99412: f64 = (1.0 + locals.var_delta);
        let assign64400_e99413: f64 = (locals.var_vgvt / assign64400_e99412);
        let assign64400_e99415: f64 = (assign64400_e99413 + locals.var_ps0);
        (assign64400_e99415, ((((locals.var_vgvt_dn0 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn0)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn0), ((((locals.var_vgvt_dn2 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn2)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn2), ((((locals.var_vgvt_dn4 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn4)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn4), ((((locals.var_vgvt_dn5 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn5)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn5), ((((locals.var_vgvt_dn6 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn6)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn6), ((((locals.var_vgvt_dn7 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn7)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn7), ((((locals.var_vgvt_dn8 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn8)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn8), ((((locals.var_vgvt_dn9 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn9)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn9), ((((locals.var_vgvt_dn10 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn10)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn10), ((((locals.var_vgvt_dn13 * assign64400_e99412) - (locals.var_vgvt * locals.var_delta_dn13)) / (assign64400_e99412 * assign64400_e99412)) + locals.var_ps0_dn13),)
    } else {
        (locals.var_pslsat, locals.var_pslsat_dn0, locals.var_pslsat_dn2, locals.var_pslsat_dn4, locals.var_pslsat_dn5, locals.var_pslsat_dn6, locals.var_pslsat_dn7, locals.var_pslsat_dn8, locals.var_pslsat_dn9, locals.var_pslsat_dn10, locals.var_pslsat_dn13,)
    }
};
        locals.var_pslsat = assign64400_e99417;
        locals.var_pslsat_dn0 = assign64400_e99417_d_n0;
        locals.var_pslsat_dn2 = assign64400_e99417_d_n2;
        locals.var_pslsat_dn4 = assign64400_e99417_d_n4;
        locals.var_pslsat_dn5 = assign64400_e99417_d_n5;
        locals.var_pslsat_dn6 = assign64400_e99417_d_n6;
        locals.var_pslsat_dn7 = assign64400_e99417_d_n7;
        locals.var_pslsat_dn8 = assign64400_e99417_d_n8;
        locals.var_pslsat_dn9 = assign64400_e99417_d_n9;
        locals.var_pslsat_dn10 = assign64400_e99417_d_n10;
        locals.var_pslsat_dn13 = assign64400_e99417_d_n13;

        let (assign64410_e99425, assign64410_e99425_d_n0, assign64410_e99425_d_n2, assign64410_e99425_d_n4, assign64410_e99425_d_n5, assign64410_e99425_d_n6, assign64410_e99425_d_n7, assign64410_e99425_d_n8, assign64410_e99425_d_n9, assign64410_e99425_d_n10, assign64410_e99425_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1517 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pslsat, locals.var_pslsat_dn0, locals.var_pslsat_dn2, locals.var_pslsat_dn4, locals.var_pslsat_dn5, locals.var_pslsat_dn6, locals.var_pslsat_dn7, locals.var_pslsat_dn8, locals.var_pslsat_dn9, locals.var_pslsat_dn10, locals.var_pslsat_dn13,)
    }
};
        locals.var_pslsat = assign64410_e99425;
        locals.var_pslsat_dn0 = assign64410_e99425_d_n0;
        locals.var_pslsat_dn2 = assign64410_e99425_d_n2;
        locals.var_pslsat_dn4 = assign64410_e99425_d_n4;
        locals.var_pslsat_dn5 = assign64410_e99425_d_n5;
        locals.var_pslsat_dn6 = assign64410_e99425_d_n6;
        locals.var_pslsat_dn7 = assign64410_e99425_d_n7;
        locals.var_pslsat_dn8 = assign64410_e99425_d_n8;
        locals.var_pslsat_dn9 = assign64410_e99425_d_n9;
        locals.var_pslsat_dn10 = assign64410_e99425_d_n10;
        locals.var_pslsat_dn13 = assign64410_e99425_d_n13;

        let (assign64450_e99447, assign64450_e99447_d_n0, assign64450_e99447_d_n2, assign64450_e99447_d_n4, assign64450_e99447_d_n5, assign64450_e99447_d_n6, assign64450_e99447_d_n7, assign64450_e99447_d_n8, assign64450_e99447_d_n9, assign64450_e99447_d_n10, assign64450_e99447_d_n13,) = {
    if (locals.var_guard443 == 0.0) {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    } else {
        (locals.var_idsorg, locals.var_idsorg_dn0, locals.var_idsorg_dn2, locals.var_idsorg_dn4, locals.var_idsorg_dn5, locals.var_idsorg_dn6, locals.var_idsorg_dn7, locals.var_idsorg_dn8, locals.var_idsorg_dn9, locals.var_idsorg_dn10, locals.var_idsorg_dn13,)
    }
};
        locals.var_idsorg = assign64450_e99447;
        locals.var_idsorg_dn0 = assign64450_e99447_d_n0;
        locals.var_idsorg_dn2 = assign64450_e99447_d_n2;
        locals.var_idsorg_dn4 = assign64450_e99447_d_n4;
        locals.var_idsorg_dn5 = assign64450_e99447_d_n5;
        locals.var_idsorg_dn6 = assign64450_e99447_d_n6;
        locals.var_idsorg_dn7 = assign64450_e99447_d_n7;
        locals.var_idsorg_dn8 = assign64450_e99447_d_n8;
        locals.var_idsorg_dn9 = assign64450_e99447_d_n9;
        locals.var_idsorg_dn10 = assign64450_e99447_d_n10;
        locals.var_idsorg_dn13 = assign64450_e99447_d_n13;

        let (assign64460_e99452, assign64460_e99452_d_n0, assign64460_e99452_d_n2, assign64460_e99452_d_n4, assign64460_e99452_d_n5, assign64460_e99452_d_n6, assign64460_e99452_d_n7, assign64460_e99452_d_n8, assign64460_e99452_d_n9, assign64460_e99452_d_n10, assign64460_e99452_d_n13,) = {
    if (locals.var_guard443 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn4, locals.var_idspt1_dn5, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn8, locals.var_idspt1_dn9, locals.var_idspt1_dn10, locals.var_idspt1_dn13,)
    }
};
        locals.var_idspt1 = assign64460_e99452;
        locals.var_idspt1_dn0 = assign64460_e99452_d_n0;
        locals.var_idspt1_dn2 = assign64460_e99452_d_n2;
        locals.var_idspt1_dn4 = assign64460_e99452_d_n4;
        locals.var_idspt1_dn5 = assign64460_e99452_d_n5;
        locals.var_idspt1_dn6 = assign64460_e99452_d_n6;
        locals.var_idspt1_dn7 = assign64460_e99452_d_n7;
        locals.var_idspt1_dn8 = assign64460_e99452_d_n8;
        locals.var_idspt1_dn9 = assign64460_e99452_d_n9;
        locals.var_idspt1_dn10 = assign64460_e99452_d_n10;
        locals.var_idspt1_dn13 = assign64460_e99452_d_n13;

        let assign64470_e99459: f64 = if ((p.p450 > 0.0) && (p.p454 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1519 = assign64470_e99459;

        let (assign64480_e99466,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        (1e-5,)
    } else {
        (locals.var_t_sub,)
    }
};
        locals.var_t_sub = assign64480_e99466;

        let (assign64490_e99481, assign64490_e99481_d_n0, assign64490_e99481_d_n2, assign64490_e99481_d_n4, assign64490_e99481_d_n5, assign64490_e99481_d_n6, assign64490_e99481_d_n7, assign64490_e99481_d_n8, assign64490_e99481_d_n9, assign64490_e99481_d_n10, assign64490_e99481_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64490_e99473: f64 = (locals.var_vgs - locals.var_vfb);
        let assign64490_e99475: f64 = (assign64490_e99473 + locals.var_dvth);
        let assign64490_e99477: f64 = (assign64490_e99475 - locals.var_dppg);
        let assign64490_e99479: f64 = (assign64490_e99477 - p.p455);
        (assign64490_e99479, (locals.var_dvth_dn0 - locals.var_dppg_dn0), (locals.var_dvth_dn2 - locals.var_dppg_dn2), (locals.var_dvth_dn4 - locals.var_dppg_dn4), ((locals.var_vgs_dn5 + locals.var_dvth_dn5) - locals.var_dppg_dn5), ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), (locals.var_dvth_dn8 - locals.var_dppg_dn8), (locals.var_dvth_dn9 - locals.var_dppg_dn9), (locals.var_dvth_dn10 - locals.var_dppg_dn10), (locals.var_dvth_dn13 - locals.var_dppg_dn13),)
    } else {
        (locals.var_vgp__blk1525, locals.var_vgp__blk1525_dn0, locals.var_vgp__blk1525_dn2, locals.var_vgp__blk1525_dn4, locals.var_vgp__blk1525_dn5, locals.var_vgp__blk1525_dn6, locals.var_vgp__blk1525_dn7, locals.var_vgp__blk1525_dn8, locals.var_vgp__blk1525_dn9, locals.var_vgp__blk1525_dn10, locals.var_vgp__blk1525_dn13,)
    }
};
        locals.var_vgp__blk1525 = assign64490_e99481;
        locals.var_vgp__blk1525_dn0 = assign64490_e99481_d_n0;
        locals.var_vgp__blk1525_dn2 = assign64490_e99481_d_n2;
        locals.var_vgp__blk1525_dn4 = assign64490_e99481_d_n4;
        locals.var_vgp__blk1525_dn5 = assign64490_e99481_d_n5;
        locals.var_vgp__blk1525_dn6 = assign64490_e99481_d_n6;
        locals.var_vgp__blk1525_dn7 = assign64490_e99481_d_n7;
        locals.var_vgp__blk1525_dn8 = assign64490_e99481_d_n8;
        locals.var_vgp__blk1525_dn9 = assign64490_e99481_d_n9;
        locals.var_vgp__blk1525_dn10 = assign64490_e99481_d_n10;
        locals.var_vgp__blk1525_dn13 = assign64490_e99481_d_n13;

        let (assign64500_e99490,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64500_e99488: f64 = (locals.var_vth + p.p455);
        (assign64500_e99488,)
    } else {
        (locals.var_wk_vth,)
    }
};
        locals.var_wk_vth = assign64500_e99490;

        let (assign64510_e99510, assign64510_e99510_d_n0, assign64510_e99510_d_n2, assign64510_e99510_d_n4, assign64510_e99510_d_n5, assign64510_e99510_d_n6, assign64510_e99510_d_n7, assign64510_e99510_d_n8, assign64510_e99510_d_n9, assign64510_e99510_d_n10, assign64510_e99510_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64510_e99497: f64 = (locals.var_vbipn - locals.var_vbscl__blk435);
        let assign64510_e99500: f64 = (locals.var_vbipn - locals.var_vbscl__blk435);
        let assign64510_e99501: f64 = (assign64510_e99497 * assign64510_e99500);
        let assign64510_e99504: f64 = (4.0 * 0.01);
        let assign64510_e99506: f64 = (assign64510_e99504 * 0.01);
        let assign64510_e99507: f64 = (assign64510_e99501 + assign64510_e99506);
        let assign64510_e99508: f64 = (assign64510_e99507).sqrt();
        (assign64510_e99508, ((((locals.var_vbipn_dn0 - locals.var_vbscl__blk435_dn0) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn0 - locals.var_vbscl__blk435_dn0))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn2 - locals.var_vbscl__blk435_dn2) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn2 - locals.var_vbscl__blk435_dn2))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn4 - locals.var_vbscl__blk435_dn4) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn4 - locals.var_vbscl__blk435_dn4))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn5 - locals.var_vbscl__blk435_dn5) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn5 - locals.var_vbscl__blk435_dn5))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn6 - locals.var_vbscl__blk435_dn6) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn6 - locals.var_vbscl__blk435_dn6))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn7 - locals.var_vbscl__blk435_dn7) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn7 - locals.var_vbscl__blk435_dn7))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn8 - locals.var_vbscl__blk435_dn8) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn8 - locals.var_vbscl__blk435_dn8))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn9 - locals.var_vbscl__blk435_dn9) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn9 - locals.var_vbscl__blk435_dn9))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn10 - locals.var_vbscl__blk435_dn10) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn10 - locals.var_vbscl__blk435_dn10))) / (2.0 * assign64510_e99508)), ((((locals.var_vbipn_dn13 - locals.var_vbscl__blk435_dn13) * assign64510_e99500) + (assign64510_e99497 * (locals.var_vbipn_dn13 - locals.var_vbscl__blk435_dn13))) / (2.0 * assign64510_e99508)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign64510_e99510;
        locals.var_tmf1_dn0 = assign64510_e99510_d_n0;
        locals.var_tmf1_dn2 = assign64510_e99510_d_n2;
        locals.var_tmf1_dn4 = assign64510_e99510_d_n4;
        locals.var_tmf1_dn5 = assign64510_e99510_d_n5;
        locals.var_tmf1_dn6 = assign64510_e99510_d_n6;
        locals.var_tmf1_dn7 = assign64510_e99510_d_n7;
        locals.var_tmf1_dn8 = assign64510_e99510_d_n8;
        locals.var_tmf1_dn9 = assign64510_e99510_d_n9;
        locals.var_tmf1_dn10 = assign64510_e99510_d_n10;
        locals.var_tmf1_dn13 = assign64510_e99510_d_n13;

        let (assign64520_e99523, assign64520_e99523_d_n0, assign64520_e99523_d_n2, assign64520_e99523_d_n4, assign64520_e99523_d_n5, assign64520_e99523_d_n6, assign64520_e99523_d_n7, assign64520_e99523_d_n8, assign64520_e99523_d_n9, assign64520_e99523_d_n10, assign64520_e99523_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64520_e99518: f64 = (locals.var_vbipn - locals.var_vbscl__blk435);
        let assign64520_e99520: f64 = (assign64520_e99518 + locals.var_tmf1);
        let assign64520_e99521: f64 = (0.5 * assign64520_e99520);
        (assign64520_e99521, (0.5 * ((locals.var_vbipn_dn0 - locals.var_vbscl__blk435_dn0) + locals.var_tmf1_dn0)), (0.5 * ((locals.var_vbipn_dn2 - locals.var_vbscl__blk435_dn2) + locals.var_tmf1_dn2)), (0.5 * ((locals.var_vbipn_dn4 - locals.var_vbscl__blk435_dn4) + locals.var_tmf1_dn4)), (0.5 * ((locals.var_vbipn_dn5 - locals.var_vbscl__blk435_dn5) + locals.var_tmf1_dn5)), (0.5 * ((locals.var_vbipn_dn6 - locals.var_vbscl__blk435_dn6) + locals.var_tmf1_dn6)), (0.5 * ((locals.var_vbipn_dn7 - locals.var_vbscl__blk435_dn7) + locals.var_tmf1_dn7)), (0.5 * ((locals.var_vbipn_dn8 - locals.var_vbscl__blk435_dn8) + locals.var_tmf1_dn8)), (0.5 * ((locals.var_vbipn_dn9 - locals.var_vbscl__blk435_dn9) + locals.var_tmf1_dn9)), (0.5 * ((locals.var_vbipn_dn10 - locals.var_vbscl__blk435_dn10) + locals.var_tmf1_dn10)), (0.5 * ((locals.var_vbipn_dn13 - locals.var_vbscl__blk435_dn13) + locals.var_tmf1_dn13)),)
    } else {
        (locals.var_vpositive, locals.var_vpositive_dn0, locals.var_vpositive_dn2, locals.var_vpositive_dn4, locals.var_vpositive_dn5, locals.var_vpositive_dn6, locals.var_vpositive_dn7, locals.var_vpositive_dn8, locals.var_vpositive_dn9, locals.var_vpositive_dn10, locals.var_vpositive_dn13,)
    }
};
        locals.var_vpositive = assign64520_e99523;
        locals.var_vpositive_dn0 = assign64520_e99523_d_n0;
        locals.var_vpositive_dn2 = assign64520_e99523_d_n2;
        locals.var_vpositive_dn4 = assign64520_e99523_d_n4;
        locals.var_vpositive_dn5 = assign64520_e99523_d_n5;
        locals.var_vpositive_dn6 = assign64520_e99523_d_n6;
        locals.var_vpositive_dn7 = assign64520_e99523_d_n7;
        locals.var_vpositive_dn8 = assign64520_e99523_d_n8;
        locals.var_vpositive_dn9 = assign64520_e99523_d_n9;
        locals.var_vpositive_dn10 = assign64520_e99523_d_n10;
        locals.var_vpositive_dn13 = assign64520_e99523_d_n13;

        let (assign64530_e99545, assign64530_e99545_d_n0, assign64530_e99545_d_n2, assign64530_e99545_d_n4, assign64530_e99545_d_n5, assign64530_e99545_d_n6, assign64530_e99545_d_n7, assign64530_e99545_d_n8, assign64530_e99545_d_n9, assign64530_e99545_d_n10, assign64530_e99545_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64530_e99530: f64 = (2.0 * 1.6021918e-19);
        let assign64530_e99532: f64 = (assign64530_e99530 * locals.var_vpositive);
        let assign64530_e99534: f64 = (assign64530_e99532 / 1.034943e-10);
        let assign64530_e99536: f64 = (assign64530_e99534 * locals.var_nsub);
        let assign64530_e99538: f64 = (assign64530_e99536 * locals.var_uc_njunc);
        let assign64530_e99541: f64 = (locals.var_nsub + locals.var_uc_njunc);
        let assign64530_e99542: f64 = (assign64530_e99538 / assign64530_e99541);
        let assign64530_e99543: f64 = (assign64530_e99542).sqrt();
        (assign64530_e99543, (((((((((assign64530_e99530 * locals.var_vpositive_dn0) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn0)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn0)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn2) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn2)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn2)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn4) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn4)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn4)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn5) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn5)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn5)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn6) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn6)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn6)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn7) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn7)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn7)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn8) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn8)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn8)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn9) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn9)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn9)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn10) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn10)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn10)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)), (((((((((assign64530_e99530 * locals.var_vpositive_dn13) / 1.034943e-10) * locals.var_nsub) + (assign64530_e99534 * locals.var_nsub_dn13)) * locals.var_uc_njunc) * assign64530_e99541) - (assign64530_e99538 * locals.var_nsub_dn13)) / (assign64530_e99541 * assign64530_e99541)) / (2.0 * assign64530_e99543)),)
    } else {
        (locals.var_ec__blk1520, locals.var_ec__blk1520_dn0, locals.var_ec__blk1520_dn2, locals.var_ec__blk1520_dn4, locals.var_ec__blk1520_dn5, locals.var_ec__blk1520_dn6, locals.var_ec__blk1520_dn7, locals.var_ec__blk1520_dn8, locals.var_ec__blk1520_dn9, locals.var_ec__blk1520_dn10, locals.var_ec__blk1520_dn13,)
    }
};
        locals.var_ec__blk1520 = assign64530_e99545;
        locals.var_ec__blk1520_dn0 = assign64530_e99545_d_n0;
        locals.var_ec__blk1520_dn2 = assign64530_e99545_d_n2;
        locals.var_ec__blk1520_dn4 = assign64530_e99545_d_n4;
        locals.var_ec__blk1520_dn5 = assign64530_e99545_d_n5;
        locals.var_ec__blk1520_dn6 = assign64530_e99545_d_n6;
        locals.var_ec__blk1520_dn7 = assign64530_e99545_d_n7;
        locals.var_ec__blk1520_dn8 = assign64530_e99545_d_n8;
        locals.var_ec__blk1520_dn9 = assign64530_e99545_d_n9;
        locals.var_ec__blk1520_dn10 = assign64530_e99545_d_n10;
        locals.var_ec__blk1520_dn13 = assign64530_e99545_d_n13;

        let (assign64540_e99554, assign64540_e99554_d_n0, assign64540_e99554_d_n2, assign64540_e99554_d_n4, assign64540_e99554_d_n5, assign64540_e99554_d_n6, assign64540_e99554_d_n7, assign64540_e99554_d_n8, assign64540_e99554_d_n9, assign64540_e99554_d_n10, assign64540_e99554_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64540_e99552: f64 = (locals.var_ec__blk1520 * locals.var_leff);
        (assign64540_e99552, (locals.var_ec__blk1520_dn0 * locals.var_leff), (locals.var_ec__blk1520_dn2 * locals.var_leff), (locals.var_ec__blk1520_dn4 * locals.var_leff), (locals.var_ec__blk1520_dn5 * locals.var_leff), (locals.var_ec__blk1520_dn6 * locals.var_leff), (locals.var_ec__blk1520_dn7 * locals.var_leff), (locals.var_ec__blk1520_dn8 * locals.var_leff), (locals.var_ec__blk1520_dn9 * locals.var_leff), (locals.var_ec__blk1520_dn10 * locals.var_leff), (locals.var_ec__blk1520_dn13 * locals.var_leff),)
    } else {
        (locals.var_wk, locals.var_wk_dn0, locals.var_wk_dn2, locals.var_wk_dn4, locals.var_wk_dn5, locals.var_wk_dn6, locals.var_wk_dn7, locals.var_wk_dn8, locals.var_wk_dn9, locals.var_wk_dn10, locals.var_wk_dn13,)
    }
};
        locals.var_wk = assign64540_e99554;
        locals.var_wk_dn0 = assign64540_e99554_d_n0;
        locals.var_wk_dn2 = assign64540_e99554_d_n2;
        locals.var_wk_dn4 = assign64540_e99554_d_n4;
        locals.var_wk_dn5 = assign64540_e99554_d_n5;
        locals.var_wk_dn6 = assign64540_e99554_d_n6;
        locals.var_wk_dn7 = assign64540_e99554_d_n7;
        locals.var_wk_dn8 = assign64540_e99554_d_n8;
        locals.var_wk_dn9 = assign64540_e99554_d_n9;
        locals.var_wk_dn10 = assign64540_e99554_d_n10;
        locals.var_wk_dn13 = assign64540_e99554_d_n13;

        let (assign64550_e99570, assign64550_e99570_d_n0, assign64550_e99570_d_n2, assign64550_e99570_d_n4, assign64550_e99570_d_n5, assign64550_e99570_d_n6, assign64550_e99570_d_n7, assign64550_e99570_d_n8, assign64550_e99570_d_n9, assign64550_e99570_d_n10, assign64550_e99570_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64550_e99560: f64 = (-0.25);
        let assign64550_e99562: f64 = (assign64550_e99560 * locals.var_wk);
        let assign64550_e99564: f64 = (assign64550_e99562 * locals.var_wk);
        let assign64550_e99567: f64 = (locals.var_vds + locals.var_wk);
        let assign64550_e99568: f64 = (assign64550_e99564 / assign64550_e99567);
        (assign64550_e99568, ((((((assign64550_e99560 * locals.var_wk_dn0) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn0)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn0 + locals.var_wk_dn0))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn2) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn2)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn2 + locals.var_wk_dn2))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn4) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn4)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn4 + locals.var_wk_dn4))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn5) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn5)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn5 + locals.var_wk_dn5))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn6) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn6)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn6 + locals.var_wk_dn6))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn7) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn7)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn7 + locals.var_wk_dn7))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn8) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn8)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn8 + locals.var_wk_dn8))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn9) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn9)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn9 + locals.var_wk_dn9))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn10) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn10)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn10 + locals.var_wk_dn10))) / (assign64550_e99567 * assign64550_e99567)), ((((((assign64550_e99560 * locals.var_wk_dn13) * locals.var_wk) + (assign64550_e99562 * locals.var_wk_dn13)) * assign64550_e99567) - (assign64550_e99564 * (locals.var_vds_dn13 + locals.var_wk_dn13))) / (assign64550_e99567 * assign64550_e99567)),)
    } else {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn13,)
    }
};
        locals.var_dphi_vds = assign64550_e99570;
        locals.var_dphi_vds_dn0 = assign64550_e99570_d_n0;
        locals.var_dphi_vds_dn2 = assign64550_e99570_d_n2;
        locals.var_dphi_vds_dn4 = assign64550_e99570_d_n4;
        locals.var_dphi_vds_dn5 = assign64550_e99570_d_n5;
        locals.var_dphi_vds_dn6 = assign64550_e99570_d_n6;
        locals.var_dphi_vds_dn7 = assign64550_e99570_d_n7;
        locals.var_dphi_vds_dn8 = assign64550_e99570_d_n8;
        locals.var_dphi_vds_dn9 = assign64550_e99570_d_n9;
        locals.var_dphi_vds_dn10 = assign64550_e99570_d_n10;
        locals.var_dphi_vds_dn13 = assign64550_e99570_d_n13;

        let assign64560_e99573: f64 = if p.p457 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1538 = assign64560_e99573;

        let (assign64570_e99582, assign64570_e99582_d_n0, assign64570_e99582_d_n2, assign64570_e99582_d_n4, assign64570_e99582_d_n5, assign64570_e99582_d_n6, assign64570_e99582_d_n7, assign64570_e99582_d_n8, assign64570_e99582_d_n9, assign64570_e99582_d_n10, assign64570_e99582_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 != 0.0)) {
        (p.p457, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0__blk1523, locals.var_ps0__blk1523_dn0, locals.var_ps0__blk1523_dn2, locals.var_ps0__blk1523_dn4, locals.var_ps0__blk1523_dn5, locals.var_ps0__blk1523_dn6, locals.var_ps0__blk1523_dn7, locals.var_ps0__blk1523_dn8, locals.var_ps0__blk1523_dn9, locals.var_ps0__blk1523_dn10, locals.var_ps0__blk1523_dn13,)
    }
};
        locals.var_ps0__blk1523 = assign64570_e99582;
        locals.var_ps0__blk1523_dn0 = assign64570_e99582_d_n0;
        locals.var_ps0__blk1523_dn2 = assign64570_e99582_d_n2;
        locals.var_ps0__blk1523_dn4 = assign64570_e99582_d_n4;
        locals.var_ps0__blk1523_dn5 = assign64570_e99582_d_n5;
        locals.var_ps0__blk1523_dn6 = assign64570_e99582_d_n6;
        locals.var_ps0__blk1523_dn7 = assign64570_e99582_d_n7;
        locals.var_ps0__blk1523_dn8 = assign64570_e99582_d_n8;
        locals.var_ps0__blk1523_dn9 = assign64570_e99582_d_n9;
        locals.var_ps0__blk1523_dn10 = assign64570_e99582_d_n10;
        locals.var_ps0__blk1523_dn13 = assign64570_e99582_d_n13;

        let (assign64580_e99592, assign64580_e99592_d_n0, assign64580_e99592_d_n2, assign64580_e99592_d_n4, assign64580_e99592_d_n5, assign64580_e99592_d_n6, assign64580_e99592_d_n7, assign64580_e99592_d_n8, assign64580_e99592_d_n9, assign64580_e99592_d_n10, assign64580_e99592_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn13,)
    } else {
        (locals.var_vbscl__blk1539, locals.var_vbscl__blk1539_dn0, locals.var_vbscl__blk1539_dn2, locals.var_vbscl__blk1539_dn4, locals.var_vbscl__blk1539_dn5, locals.var_vbscl__blk1539_dn6, locals.var_vbscl__blk1539_dn7, locals.var_vbscl__blk1539_dn8, locals.var_vbscl__blk1539_dn9, locals.var_vbscl__blk1539_dn10, locals.var_vbscl__blk1539_dn13,)
    }
};
        locals.var_vbscl__blk1539 = assign64580_e99592;
        locals.var_vbscl__blk1539_dn0 = assign64580_e99592_d_n0;
        locals.var_vbscl__blk1539_dn2 = assign64580_e99592_d_n2;
        locals.var_vbscl__blk1539_dn4 = assign64580_e99592_d_n4;
        locals.var_vbscl__blk1539_dn5 = assign64580_e99592_d_n5;
        locals.var_vbscl__blk1539_dn6 = assign64580_e99592_d_n6;
        locals.var_vbscl__blk1539_dn7 = assign64580_e99592_d_n7;
        locals.var_vbscl__blk1539_dn8 = assign64580_e99592_d_n8;
        locals.var_vbscl__blk1539_dn9 = assign64580_e99592_d_n9;
        locals.var_vbscl__blk1539_dn10 = assign64580_e99592_d_n10;
        locals.var_vbscl__blk1539_dn13 = assign64580_e99592_d_n13;

        let (assign64590_e99602,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        (locals.var_wk_vth,)
    } else {
        (locals.var_vth__blk1540,)
    }
};
        locals.var_vth__blk1540 = assign64590_e99602;

        let (assign64600_e99626, assign64600_e99626_d_n0, assign64600_e99626_d_n2, assign64600_e99626_d_n4, assign64600_e99626_d_n5, assign64600_e99626_d_n6, assign64600_e99626_d_n7, assign64600_e99626_d_n8, assign64600_e99626_d_n9, assign64600_e99626_d_n10, assign64600_e99626_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        let assign64600_e99615: f64 = (locals.var_vgp__blk1525 - locals.var_vbscl__blk1539);
        let assign64600_e99616: f64 = (locals.var_beta * assign64600_e99615);
        let assign64600_e99618: f64 = (assign64600_e99616 - 1.0);
        let assign64600_e99619: f64 = (4.0 * assign64600_e99618);
        let assign64600_e99622: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign64600_e99623: f64 = (assign64600_e99619 / assign64600_e99622);
        let assign64600_e99624: f64 = (1.0 + assign64600_e99623);
        (assign64600_e99624, ((((4.0 * ((locals.var_beta_dn0 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn0 - locals.var_vbscl__blk1539_dn0)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn2 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn2 - locals.var_vbscl__blk1539_dn2)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn4 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn4 - locals.var_vbscl__blk1539_dn4)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn5 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn5 - locals.var_vbscl__blk1539_dn5)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn6 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn6 - locals.var_vbscl__blk1539_dn6)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn7 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn7 - locals.var_vbscl__blk1539_dn7)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn8 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn8 - locals.var_vbscl__blk1539_dn8)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn9 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn9 - locals.var_vbscl__blk1539_dn9)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn10 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn10 - locals.var_vbscl__blk1539_dn10)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign64600_e99622 * assign64600_e99622)), ((((4.0 * ((locals.var_beta_dn13 * assign64600_e99615) + (locals.var_beta * (locals.var_vgp__blk1525_dn13 - locals.var_vbscl__blk1539_dn13)))) * assign64600_e99622) - (assign64600_e99619 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign64600_e99622 * assign64600_e99622)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign64600_e99626;
        locals.var_tx_dn0 = assign64600_e99626_d_n0;
        locals.var_tx_dn2 = assign64600_e99626_d_n2;
        locals.var_tx_dn4 = assign64600_e99626_d_n4;
        locals.var_tx_dn5 = assign64600_e99626_d_n5;
        locals.var_tx_dn6 = assign64600_e99626_d_n6;
        locals.var_tx_dn7 = assign64600_e99626_d_n7;
        locals.var_tx_dn8 = assign64600_e99626_d_n8;
        locals.var_tx_dn9 = assign64600_e99626_d_n9;
        locals.var_tx_dn10 = assign64600_e99626_d_n10;
        locals.var_tx_dn13 = assign64600_e99626_d_n13;

        let (assign64610_e99645, assign64610_e99645_d_n0, assign64610_e99645_d_n2, assign64610_e99645_d_n4, assign64610_e99645_d_n5, assign64610_e99645_d_n6, assign64610_e99645_d_n7, assign64610_e99645_d_n8, assign64610_e99645_d_n9, assign64610_e99645_d_n10, assign64610_e99645_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        let assign64610_e99637: f64 = (10.0 * 2.220446049250313e-16);
        let (assign64610_e99643, assign64610_e99643_d_n0, assign64610_e99643_d_n2, assign64610_e99643_d_n4, assign64610_e99643_d_n5, assign64610_e99643_d_n6, assign64610_e99643_d_n7, assign64610_e99643_d_n8, assign64610_e99643_d_n9, assign64610_e99643_d_n10, assign64610_e99643_d_n13,) = {
            if (locals.var_tx >= assign64610_e99637) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
            } else {
                let assign64610_e99642: f64 = (10.0 * 2.220446049250313e-16);
                (assign64610_e99642, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign64610_e99643, assign64610_e99643_d_n0, assign64610_e99643_d_n2, assign64610_e99643_d_n4, assign64610_e99643_d_n5, assign64610_e99643_d_n6, assign64610_e99643_d_n7, assign64610_e99643_d_n8, assign64610_e99643_d_n9, assign64610_e99643_d_n10, assign64610_e99643_d_n13,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign64610_e99645;
        locals.var_tx_dn0 = assign64610_e99645_d_n0;
        locals.var_tx_dn2 = assign64610_e99645_d_n2;
        locals.var_tx_dn4 = assign64610_e99645_d_n4;
        locals.var_tx_dn5 = assign64610_e99645_d_n5;
        locals.var_tx_dn6 = assign64610_e99645_d_n6;
        locals.var_tx_dn7 = assign64610_e99645_d_n7;
        locals.var_tx_dn8 = assign64610_e99645_d_n8;
        locals.var_tx_dn9 = assign64610_e99645_d_n9;
        locals.var_tx_dn10 = assign64610_e99645_d_n10;
        locals.var_tx_dn13 = assign64610_e99645_d_n13;

        let (assign64620_e99666, assign64620_e99666_d_n0, assign64620_e99666_d_n2, assign64620_e99666_d_n4, assign64620_e99666_d_n5, assign64620_e99666_d_n6, assign64620_e99666_d_n7, assign64620_e99666_d_n8, assign64620_e99666_d_n9, assign64620_e99666_d_n10, assign64620_e99666_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        let assign64620_e99656: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign64620_e99658: f64 = (assign64620_e99656 * 0.5);
        let assign64620_e99661: f64 = (locals.var_tx).sqrt();
        let assign64620_e99662: f64 = (1.0 - assign64620_e99661);
        let assign64620_e99663: f64 = (assign64620_e99658 * assign64620_e99662);
        let assign64620_e99664: f64 = (locals.var_vgp__blk1525 + assign64620_e99663);
        (assign64620_e99664, (locals.var_vgp__blk1525_dn0 + (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn0 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn2 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn4 + (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn4 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn5 + (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn5 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn6 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn7 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn8 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn9 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn10 / (2.0 * assign64620_e99661)))))), (locals.var_vgp__blk1525_dn13 + (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) * 0.5) * assign64620_e99662) + (assign64620_e99658 * (-(locals.var_tx_dn13 / (2.0 * assign64620_e99661)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign64620_e99666;
        locals.var_ps0_inia_dn0 = assign64620_e99666_d_n0;
        locals.var_ps0_inia_dn2 = assign64620_e99666_d_n2;
        locals.var_ps0_inia_dn4 = assign64620_e99666_d_n4;
        locals.var_ps0_inia_dn5 = assign64620_e99666_d_n5;
        locals.var_ps0_inia_dn6 = assign64620_e99666_d_n6;
        locals.var_ps0_inia_dn7 = assign64620_e99666_d_n7;
        locals.var_ps0_inia_dn8 = assign64620_e99666_d_n8;
        locals.var_ps0_inia_dn9 = assign64620_e99666_d_n9;
        locals.var_ps0_inia_dn10 = assign64620_e99666_d_n10;
        locals.var_ps0_inia_dn13 = assign64620_e99666_d_n13;

        let (assign64630_e99680, assign64630_e99680_d_n0, assign64630_e99680_d_n2, assign64630_e99680_d_n4, assign64630_e99680_d_n5, assign64630_e99680_d_n6, assign64630_e99680_d_n7, assign64630_e99680_d_n8, assign64630_e99680_d_n9, assign64630_e99680_d_n10, assign64630_e99680_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        let assign64630_e99677: f64 = (locals.var_ps0_inia - locals.var_vbscl__blk1539);
        let assign64630_e99678: f64 = (locals.var_beta * assign64630_e99677);
        (assign64630_e99678, ((locals.var_beta_dn0 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbscl__blk1539_dn0))), ((locals.var_beta_dn2 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbscl__blk1539_dn2))), ((locals.var_beta_dn4 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn4 - locals.var_vbscl__blk1539_dn4))), ((locals.var_beta_dn5 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn5 - locals.var_vbscl__blk1539_dn5))), ((locals.var_beta_dn6 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbscl__blk1539_dn6))), ((locals.var_beta_dn7 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbscl__blk1539_dn7))), ((locals.var_beta_dn8 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn8 - locals.var_vbscl__blk1539_dn8))), ((locals.var_beta_dn9 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn9 - locals.var_vbscl__blk1539_dn9))), ((locals.var_beta_dn10 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbscl__blk1539_dn10))), ((locals.var_beta_dn13 * assign64630_e99677) + (locals.var_beta * (locals.var_ps0_inia_dn13 - locals.var_vbscl__blk1539_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign64630_e99680;
        locals.var_chi_dn0 = assign64630_e99680_d_n0;
        locals.var_chi_dn2 = assign64630_e99680_d_n2;
        locals.var_chi_dn4 = assign64630_e99680_d_n4;
        locals.var_chi_dn5 = assign64630_e99680_d_n5;
        locals.var_chi_dn6 = assign64630_e99680_d_n6;
        locals.var_chi_dn7 = assign64630_e99680_d_n7;
        locals.var_chi_dn8 = assign64630_e99680_d_n8;
        locals.var_chi_dn9 = assign64630_e99680_d_n9;
        locals.var_chi_dn10 = assign64630_e99680_d_n10;
        locals.var_chi_dn13 = assign64630_e99680_d_n13;

        let assign64640_e99683: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1541 = assign64640_e99683;

        let (assign64650_e99699, assign64650_e99699_d_n0, assign64650_e99699_d_n2, assign64650_e99699_d_n4, assign64650_e99699_d_n5, assign64650_e99699_d_n6, assign64650_e99699_d_n7, assign64650_e99699_d_n8, assign64650_e99699_d_n9, assign64650_e99699_d_n10, assign64650_e99699_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64650_e99696: f64 = (locals.var_vgp__blk1525 - locals.var_vbscl__blk1539);
        let assign64650_e99697: f64 = (locals.var_beta * assign64650_e99696);
        (assign64650_e99697, ((locals.var_beta_dn0 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn0 - locals.var_vbscl__blk1539_dn0))), ((locals.var_beta_dn2 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn2 - locals.var_vbscl__blk1539_dn2))), ((locals.var_beta_dn4 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn4 - locals.var_vbscl__blk1539_dn4))), ((locals.var_beta_dn5 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn5 - locals.var_vbscl__blk1539_dn5))), ((locals.var_beta_dn6 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn6 - locals.var_vbscl__blk1539_dn6))), ((locals.var_beta_dn7 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn7 - locals.var_vbscl__blk1539_dn7))), ((locals.var_beta_dn8 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn8 - locals.var_vbscl__blk1539_dn8))), ((locals.var_beta_dn9 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn9 - locals.var_vbscl__blk1539_dn9))), ((locals.var_beta_dn10 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn10 - locals.var_vbscl__blk1539_dn10))), ((locals.var_beta_dn13 * assign64650_e99696) + (locals.var_beta * (locals.var_vgp__blk1525_dn13 - locals.var_vbscl__blk1539_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign64650_e99699;
        locals.var_ty_dn0 = assign64650_e99699_d_n0;
        locals.var_ty_dn2 = assign64650_e99699_d_n2;
        locals.var_ty_dn4 = assign64650_e99699_d_n4;
        locals.var_ty_dn5 = assign64650_e99699_d_n5;
        locals.var_ty_dn6 = assign64650_e99699_d_n6;
        locals.var_ty_dn7 = assign64650_e99699_d_n7;
        locals.var_ty_dn8 = assign64650_e99699_d_n8;
        locals.var_ty_dn9 = assign64650_e99699_d_n9;
        locals.var_ty_dn10 = assign64650_e99699_d_n10;
        locals.var_ty_dn13 = assign64650_e99699_d_n13;

    }

    pub(super) fn stamp_transient_block_219(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64660_e99719, assign64660_e99719_d_n0, assign64660_e99719_d_n2, assign64660_e99719_d_n4, assign64660_e99719_d_n5, assign64660_e99719_d_n6, assign64660_e99719_d_n7, assign64660_e99719_d_n8, assign64660_e99719_d_n9, assign64660_e99719_d_n10, assign64660_e99719_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64660_e99712: f64 = (1.414213562373095 / 108.0);
        let assign64660_e99714: f64 = (assign64660_e99712 * locals.var_beta);
        let assign64660_e99716: f64 = (assign64660_e99714 * locals.var_fac1);
        let assign64660_e99717: f64 = (1.0 / assign64660_e99716);
        (assign64660_e99717, (-((((assign64660_e99712 * locals.var_beta_dn0) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn0)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn2) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn2)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn4) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn4)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn5) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn5)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn6) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn6)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn7) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn7)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn8) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn8)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn9) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn9)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn10) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn10)) / (assign64660_e99716 * assign64660_e99716))), (-((((assign64660_e99712 * locals.var_beta_dn13) * locals.var_fac1) + (assign64660_e99714 * locals.var_fac1_dn13)) / (assign64660_e99716 * assign64660_e99716))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign64660_e99719;
        locals.var_t1_dn0 = assign64660_e99719_d_n0;
        locals.var_t1_dn2 = assign64660_e99719_d_n2;
        locals.var_t1_dn4 = assign64660_e99719_d_n4;
        locals.var_t1_dn5 = assign64660_e99719_d_n5;
        locals.var_t1_dn6 = assign64660_e99719_d_n6;
        locals.var_t1_dn7 = assign64660_e99719_d_n7;
        locals.var_t1_dn8 = assign64660_e99719_d_n8;
        locals.var_t1_dn9 = assign64660_e99719_d_n9;
        locals.var_t1_dn10 = assign64660_e99719_d_n10;
        locals.var_t1_dn13 = assign64660_e99719_d_n13;

        let (assign64670_e99735, assign64670_e99735_d_n0, assign64670_e99735_d_n2, assign64670_e99735_d_n4, assign64670_e99735_d_n5, assign64670_e99735_d_n6, assign64670_e99735_d_n7, assign64670_e99735_d_n8, assign64670_e99735_d_n9, assign64670_e99735_d_n10, assign64670_e99735_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64670_e99732: f64 = (3.0 * locals.var_t1);
        let assign64670_e99733: f64 = (81.0 + assign64670_e99732);
        (assign64670_e99733, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn4), (3.0 * locals.var_t1_dn5), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn8), (3.0 * locals.var_t1_dn9), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign64670_e99735;
        locals.var_t2_dn0 = assign64670_e99735_d_n0;
        locals.var_t2_dn2 = assign64670_e99735_d_n2;
        locals.var_t2_dn4 = assign64670_e99735_d_n4;
        locals.var_t2_dn5 = assign64670_e99735_d_n5;
        locals.var_t2_dn6 = assign64670_e99735_d_n6;
        locals.var_t2_dn7 = assign64670_e99735_d_n7;
        locals.var_t2_dn8 = assign64670_e99735_d_n8;
        locals.var_t2_dn9 = assign64670_e99735_d_n9;
        locals.var_t2_dn10 = assign64670_e99735_d_n10;
        locals.var_t2_dn13 = assign64670_e99735_d_n13;

        let (assign64680_e99758, assign64680_e99758_d_n0, assign64680_e99758_d_n2, assign64680_e99758_d_n4, assign64680_e99758_d_n5, assign64680_e99758_d_n6, assign64680_e99758_d_n7, assign64680_e99758_d_n8, assign64680_e99758_d_n9, assign64680_e99758_d_n10, assign64680_e99758_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64680_e99746: f64 = (-2916.0);
        let assign64680_e99749: f64 = (81.0 * locals.var_t1);
        let assign64680_e99750: f64 = (assign64680_e99746 - assign64680_e99749);
        let assign64680_e99753: f64 = (27.0 * locals.var_t1);
        let assign64680_e99755: f64 = (assign64680_e99753 * locals.var_ty);
        let assign64680_e99756: f64 = (assign64680_e99750 + assign64680_e99755);
        (assign64680_e99756, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn13)) + (((27.0 * locals.var_t1_dn13) * locals.var_ty) + (assign64680_e99753 * locals.var_ty_dn13))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign64680_e99758;
        locals.var_t3_dn0 = assign64680_e99758_d_n0;
        locals.var_t3_dn2 = assign64680_e99758_d_n2;
        locals.var_t3_dn4 = assign64680_e99758_d_n4;
        locals.var_t3_dn5 = assign64680_e99758_d_n5;
        locals.var_t3_dn6 = assign64680_e99758_d_n6;
        locals.var_t3_dn7 = assign64680_e99758_d_n7;
        locals.var_t3_dn8 = assign64680_e99758_d_n8;
        locals.var_t3_dn9 = assign64680_e99758_d_n9;
        locals.var_t3_dn10 = assign64680_e99758_d_n10;
        locals.var_t3_dn13 = assign64680_e99758_d_n13;

        let (assign64690_e99782, assign64690_e99782_d_n0, assign64690_e99782_d_n2, assign64690_e99782_d_n4, assign64690_e99782_d_n5, assign64690_e99782_d_n6, assign64690_e99782_d_n7, assign64690_e99782_d_n8, assign64690_e99782_d_n9, assign64690_e99782_d_n10, assign64690_e99782_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64690_e99772: f64 = (54.0 + locals.var_t1);
        let assign64690_e99773: f64 = (81.0 * assign64690_e99772);
        let assign64690_e99774: f64 = (1458.0 - assign64690_e99773);
        let assign64690_e99777: f64 = (27.0 * locals.var_t1);
        let assign64690_e99779: f64 = (assign64690_e99777 * locals.var_ty);
        let assign64690_e99780: f64 = (assign64690_e99774 + assign64690_e99779);
        (assign64690_e99780, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn13)) + (((27.0 * locals.var_t1_dn13) * locals.var_ty) + (assign64690_e99777 * locals.var_ty_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign64690_e99782;
        locals.var_t4_dn0 = assign64690_e99782_d_n0;
        locals.var_t4_dn2 = assign64690_e99782_d_n2;
        locals.var_t4_dn4 = assign64690_e99782_d_n4;
        locals.var_t4_dn5 = assign64690_e99782_d_n5;
        locals.var_t4_dn6 = assign64690_e99782_d_n6;
        locals.var_t4_dn7 = assign64690_e99782_d_n7;
        locals.var_t4_dn8 = assign64690_e99782_d_n8;
        locals.var_t4_dn9 = assign64690_e99782_d_n9;
        locals.var_t4_dn10 = assign64690_e99782_d_n10;
        locals.var_t4_dn13 = assign64690_e99782_d_n13;

        let (assign64700_e99796, assign64700_e99796_d_n0, assign64700_e99796_d_n2, assign64700_e99796_d_n4, assign64700_e99796_d_n5, assign64700_e99796_d_n6, assign64700_e99796_d_n7, assign64700_e99796_d_n8, assign64700_e99796_d_n9, assign64700_e99796_d_n10, assign64700_e99796_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64700_e99794: f64 = (locals.var_t4 * locals.var_t4);
        (assign64700_e99794, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn13 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign64700_e99796;
        locals.var_t4_dn0 = assign64700_e99796_d_n0;
        locals.var_t4_dn2 = assign64700_e99796_d_n2;
        locals.var_t4_dn4 = assign64700_e99796_d_n4;
        locals.var_t4_dn5 = assign64700_e99796_d_n5;
        locals.var_t4_dn6 = assign64700_e99796_d_n6;
        locals.var_t4_dn7 = assign64700_e99796_d_n7;
        locals.var_t4_dn8 = assign64700_e99796_d_n8;
        locals.var_t4_dn9 = assign64700_e99796_d_n9;
        locals.var_t4_dn10 = assign64700_e99796_d_n10;
        locals.var_t4_dn13 = assign64700_e99796_d_n13;

        let (assign64710_e99837, assign64710_e99837_d_n0, assign64710_e99837_d_n2, assign64710_e99837_d_n4, assign64710_e99837_d_n5, assign64710_e99837_d_n6, assign64710_e99837_d_n7, assign64710_e99837_d_n8, assign64710_e99837_d_n9, assign64710_e99837_d_n10, assign64710_e99837_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64710_e99809: f64 = (4.0 * locals.var_t2);
        let assign64710_e99811: f64 = (assign64710_e99809 * locals.var_t2);
        let assign64710_e99813: f64 = (assign64710_e99811 * locals.var_t2);
        let assign64710_e99815: f64 = (assign64710_e99813 + locals.var_t4);
        let assign64710_e99816: f64 = (assign64710_e99815).sqrt();
        let assign64710_e99817: f64 = (locals.var_t3 + assign64710_e99816);
        let (assign64710_e99835, assign64710_e99835_d_n0, assign64710_e99835_d_n2, assign64710_e99835_d_n4, assign64710_e99835_d_n5, assign64710_e99835_d_n6, assign64710_e99835_d_n7, assign64710_e99835_d_n8, assign64710_e99835_d_n9, assign64710_e99835_d_n10, assign64710_e99835_d_n13,) = {
            if (assign64710_e99817 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign64710_e99824: f64 = (4.0 * locals.var_t2);
                let assign64710_e99826: f64 = (assign64710_e99824 * locals.var_t2);
                let assign64710_e99828: f64 = (assign64710_e99826 * locals.var_t2);
                let assign64710_e99830: f64 = (assign64710_e99828 + locals.var_t4);
                let assign64710_e99831: f64 = (assign64710_e99830).sqrt();
                let assign64710_e99832: f64 = (locals.var_t3 + assign64710_e99831);
                let assign64710_e99834: f64 = (assign64710_e99832).powf(0.3333333333333333);
                (assign64710_e99834, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn0)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn0)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn2)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn2)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn4)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn4)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn5)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn5)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn6)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn6)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn7)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn7)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn8)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn8)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn9)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn9)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn10)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn10)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign64710_e99831))) / assign64710_e99832))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64710_e99832).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn13 + (((((((4.0 * locals.var_t2_dn13) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn13)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn13)) + locals.var_t4_dn13) / (2.0 * assign64710_e99831))))) } } else { (assign64710_e99834 * (0.3333333333333333 * ((locals.var_t3_dn13 + (((((((4.0 * locals.var_t2_dn13) * locals.var_t2) + (assign64710_e99824 * locals.var_t2_dn13)) * locals.var_t2) + (assign64710_e99826 * locals.var_t2_dn13)) + locals.var_t4_dn13) / (2.0 * assign64710_e99831))) / assign64710_e99832))) },)
            }
        };
        (assign64710_e99835, assign64710_e99835_d_n0, assign64710_e99835_d_n2, assign64710_e99835_d_n4, assign64710_e99835_d_n5, assign64710_e99835_d_n6, assign64710_e99835_d_n7, assign64710_e99835_d_n8, assign64710_e99835_d_n9, assign64710_e99835_d_n10, assign64710_e99835_d_n13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign64710_e99837;
        locals.var_t5_dn0 = assign64710_e99837_d_n0;
        locals.var_t5_dn2 = assign64710_e99837_d_n2;
        locals.var_t5_dn4 = assign64710_e99837_d_n4;
        locals.var_t5_dn5 = assign64710_e99837_d_n5;
        locals.var_t5_dn6 = assign64710_e99837_d_n6;
        locals.var_t5_dn7 = assign64710_e99837_d_n7;
        locals.var_t5_dn8 = assign64710_e99837_d_n8;
        locals.var_t5_dn9 = assign64710_e99837_d_n9;
        locals.var_t5_dn10 = assign64710_e99837_d_n10;
        locals.var_t5_dn13 = assign64710_e99837_d_n13;

        let (assign64720_e99865, assign64720_e99865_d_n0, assign64720_e99865_d_n2, assign64720_e99865_d_n4, assign64720_e99865_d_n5, assign64720_e99865_d_n6, assign64720_e99865_d_n7, assign64720_e99865_d_n8, assign64720_e99865_d_n9, assign64720_e99865_d_n10, assign64720_e99865_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64720_e99850: f64 = (1.259921049894873 * locals.var_t2);
        let assign64720_e99853: f64 = (3.0 * locals.var_t5);
        let assign64720_e99854: f64 = (assign64720_e99850 / assign64720_e99853);
        let assign64720_e99855: f64 = (3.0 - assign64720_e99854);
        let assign64720_e99859: f64 = (3.0 * 1.259921049894873);
        let assign64720_e99860: f64 = (1.0 / assign64720_e99859);
        let assign64720_e99862: f64 = (assign64720_e99860 * locals.var_t5);
        let assign64720_e99863: f64 = (assign64720_e99855 + assign64720_e99862);
        (assign64720_e99863, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn0))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn2))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn4) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn4))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn4)), ((-((((1.259921049894873 * locals.var_t2_dn5) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn5))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn5)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn6))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn7))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn8) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn8))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn8)), ((-((((1.259921049894873 * locals.var_t2_dn9) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn9))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn9)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn10))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn13) * assign64720_e99853) - (assign64720_e99850 * (3.0 * locals.var_t5_dn13))) / (assign64720_e99853 * assign64720_e99853))) + (assign64720_e99860 * locals.var_t5_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign64720_e99865;
        locals.var_tx_dn0 = assign64720_e99865_d_n0;
        locals.var_tx_dn2 = assign64720_e99865_d_n2;
        locals.var_tx_dn4 = assign64720_e99865_d_n4;
        locals.var_tx_dn5 = assign64720_e99865_d_n5;
        locals.var_tx_dn6 = assign64720_e99865_d_n6;
        locals.var_tx_dn7 = assign64720_e99865_d_n7;
        locals.var_tx_dn8 = assign64720_e99865_d_n8;
        locals.var_tx_dn9 = assign64720_e99865_d_n9;
        locals.var_tx_dn10 = assign64720_e99865_d_n10;
        locals.var_tx_dn13 = assign64720_e99865_d_n13;

        let (assign64730_e99881, assign64730_e99881_d_n0, assign64730_e99881_d_n2, assign64730_e99881_d_n4, assign64730_e99881_d_n5, assign64730_e99881_d_n6, assign64730_e99881_d_n7, assign64730_e99881_d_n8, assign64730_e99881_d_n9, assign64730_e99881_d_n10, assign64730_e99881_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign64730_e99877: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign64730_e99879: f64 = (assign64730_e99877 + locals.var_vbscl__blk1539);
        (assign64730_e99879, (((locals.var_tx_dn0 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk1539_dn0), (((locals.var_tx_dn2 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk1539_dn2), (((locals.var_tx_dn4 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk1539_dn4), (((locals.var_tx_dn5 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk1539_dn5), (((locals.var_tx_dn6 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk1539_dn6), (((locals.var_tx_dn7 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk1539_dn7), (((locals.var_tx_dn8 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk1539_dn8), (((locals.var_tx_dn9 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk1539_dn9), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk1539_dn10), (((locals.var_tx_dn13 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn13)) + locals.var_vbscl__blk1539_dn13),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign64730_e99881;
        locals.var_ps0_inia_dn0 = assign64730_e99881_d_n0;
        locals.var_ps0_inia_dn2 = assign64730_e99881_d_n2;
        locals.var_ps0_inia_dn4 = assign64730_e99881_d_n4;
        locals.var_ps0_inia_dn5 = assign64730_e99881_d_n5;
        locals.var_ps0_inia_dn6 = assign64730_e99881_d_n6;
        locals.var_ps0_inia_dn7 = assign64730_e99881_d_n7;
        locals.var_ps0_inia_dn8 = assign64730_e99881_d_n8;
        locals.var_ps0_inia_dn9 = assign64730_e99881_d_n9;
        locals.var_ps0_inia_dn10 = assign64730_e99881_d_n10;
        locals.var_ps0_inia_dn13 = assign64730_e99881_d_n13;

        let (assign64740_e99893, assign64740_e99893_d_n0, assign64740_e99893_d_n2, assign64740_e99893_d_n4, assign64740_e99893_d_n5, assign64740_e99893_d_n6, assign64740_e99893_d_n7, assign64740_e99893_d_n8, assign64740_e99893_d_n9, assign64740_e99893_d_n10, assign64740_e99893_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign64740_e99893;
        locals.var_ps0_ini_dn0 = assign64740_e99893_d_n0;
        locals.var_ps0_ini_dn2 = assign64740_e99893_d_n2;
        locals.var_ps0_ini_dn4 = assign64740_e99893_d_n4;
        locals.var_ps0_ini_dn5 = assign64740_e99893_d_n5;
        locals.var_ps0_ini_dn6 = assign64740_e99893_d_n6;
        locals.var_ps0_ini_dn7 = assign64740_e99893_d_n7;
        locals.var_ps0_ini_dn8 = assign64740_e99893_d_n8;
        locals.var_ps0_ini_dn9 = assign64740_e99893_d_n9;
        locals.var_ps0_ini_dn10 = assign64740_e99893_d_n10;
        locals.var_ps0_ini_dn13 = assign64740_e99893_d_n13;

        let assign64750_e99896: f64 = if locals.var_vgs <= locals.var_vth__blk1540 { 1.0 } else { 0.0 };
        locals.var_guard1542 = assign64750_e99896;

        let (assign64760_e99911, assign64760_e99911_d_n0, assign64760_e99911_d_n2, assign64760_e99911_d_n4, assign64760_e99911_d_n5, assign64760_e99911_d_n6, assign64760_e99911_d_n7, assign64760_e99911_d_n8, assign64760_e99911_d_n9, assign64760_e99911_d_n10, assign64760_e99911_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign64760_e99911;
        locals.var_ps0_ini_dn0 = assign64760_e99911_d_n0;
        locals.var_ps0_ini_dn2 = assign64760_e99911_d_n2;
        locals.var_ps0_ini_dn4 = assign64760_e99911_d_n4;
        locals.var_ps0_ini_dn5 = assign64760_e99911_d_n5;
        locals.var_ps0_ini_dn6 = assign64760_e99911_d_n6;
        locals.var_ps0_ini_dn7 = assign64760_e99911_d_n7;
        locals.var_ps0_ini_dn8 = assign64760_e99911_d_n8;
        locals.var_ps0_ini_dn9 = assign64760_e99911_d_n9;
        locals.var_ps0_ini_dn10 = assign64760_e99911_d_n10;
        locals.var_ps0_ini_dn13 = assign64760_e99911_d_n13;

        let (assign64770_e99931, assign64770_e99931_d_n0, assign64770_e99931_d_n2, assign64770_e99931_d_n4, assign64770_e99931_d_n5, assign64770_e99931_d_n6, assign64770_e99931_d_n7, assign64770_e99931_d_n8, assign64770_e99931_d_n9, assign64770_e99931_d_n10, assign64770_e99931_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64770_e99927: f64 = (1.0 / locals.var_cnst1);
        let assign64770_e99929: f64 = (assign64770_e99927 / locals.var_cnstcoxi);
        (assign64770_e99929, ((((-(locals.var_cnst1_dn0 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn0)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn2 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn2)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn4 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn4)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn5 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn5)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn6 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn6)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn7 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn7)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn8 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn8)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn9 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn9)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn10 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn10)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn13 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64770_e99927 * locals.var_cnstcoxi_dn13)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign64770_e99931;
        locals.var_t1_dn0 = assign64770_e99931_d_n0;
        locals.var_t1_dn2 = assign64770_e99931_d_n2;
        locals.var_t1_dn4 = assign64770_e99931_d_n4;
        locals.var_t1_dn5 = assign64770_e99931_d_n5;
        locals.var_t1_dn6 = assign64770_e99931_d_n6;
        locals.var_t1_dn7 = assign64770_e99931_d_n7;
        locals.var_t1_dn8 = assign64770_e99931_d_n8;
        locals.var_t1_dn9 = assign64770_e99931_d_n9;
        locals.var_t1_dn10 = assign64770_e99931_d_n10;
        locals.var_t1_dn13 = assign64770_e99931_d_n13;

        let (assign64780_e99951, assign64780_e99951_d_n0, assign64780_e99951_d_n2, assign64780_e99951_d_n4, assign64780_e99951_d_n5, assign64780_e99951_d_n6, assign64780_e99951_d_n7, assign64780_e99951_d_n8, assign64780_e99951_d_n9, assign64780_e99951_d_n10, assign64780_e99951_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64780_e99947: f64 = (locals.var_t1 * locals.var_vgp__blk1525);
        let assign64780_e99949: f64 = (assign64780_e99947 * locals.var_vgp__blk1525);
        (assign64780_e99949, ((((locals.var_t1_dn0 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn0)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn2)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn2)), ((((locals.var_t1_dn4 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn4)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn4)), ((((locals.var_t1_dn5 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn5)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn5)), ((((locals.var_t1_dn6 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn6)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn7)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn7)), ((((locals.var_t1_dn8 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn8)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn8)), ((((locals.var_t1_dn9 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn9)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn9)), ((((locals.var_t1_dn10 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn10)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn10)), ((((locals.var_t1_dn13 * locals.var_vgp__blk1525) + (locals.var_t1 * locals.var_vgp__blk1525_dn13)) * locals.var_vgp__blk1525) + (assign64780_e99947 * locals.var_vgp__blk1525_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign64780_e99951;
        locals.var_t2_dn0 = assign64780_e99951_d_n0;
        locals.var_t2_dn2 = assign64780_e99951_d_n2;
        locals.var_t2_dn4 = assign64780_e99951_d_n4;
        locals.var_t2_dn5 = assign64780_e99951_d_n5;
        locals.var_t2_dn6 = assign64780_e99951_d_n6;
        locals.var_t2_dn7 = assign64780_e99951_d_n7;
        locals.var_t2_dn8 = assign64780_e99951_d_n8;
        locals.var_t2_dn9 = assign64780_e99951_d_n9;
        locals.var_t2_dn10 = assign64780_e99951_d_n10;
        locals.var_t2_dn13 = assign64780_e99951_d_n13;

        let (assign64790_e99971, assign64790_e99971_d_n0, assign64790_e99971_d_n2, assign64790_e99971_d_n4, assign64790_e99971_d_n5, assign64790_e99971_d_n6, assign64790_e99971_d_n7, assign64790_e99971_d_n8, assign64790_e99971_d_n9, assign64790_e99971_d_n10, assign64790_e99971_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64790_e99968: f64 = (2.0 / locals.var_vgp__blk1525);
        let assign64790_e99969: f64 = (locals.var_beta + assign64790_e99968);
        (assign64790_e99969, (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp__blk1525_dn0) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp__blk1525_dn2) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp__blk1525_dn4) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp__blk1525_dn5) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp__blk1525_dn6) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp__blk1525_dn7) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp__blk1525_dn8) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp__blk1525_dn9) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp__blk1525_dn10) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))), (locals.var_beta_dn13 + (-((2.0 * locals.var_vgp__blk1525_dn13) / (locals.var_vgp__blk1525 * locals.var_vgp__blk1525)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign64790_e99971;
        locals.var_t3_dn0 = assign64790_e99971_d_n0;
        locals.var_t3_dn2 = assign64790_e99971_d_n2;
        locals.var_t3_dn4 = assign64790_e99971_d_n4;
        locals.var_t3_dn5 = assign64790_e99971_d_n5;
        locals.var_t3_dn6 = assign64790_e99971_d_n6;
        locals.var_t3_dn7 = assign64790_e99971_d_n7;
        locals.var_t3_dn8 = assign64790_e99971_d_n8;
        locals.var_t3_dn9 = assign64790_e99971_d_n9;
        locals.var_t3_dn10 = assign64790_e99971_d_n10;
        locals.var_t3_dn13 = assign64790_e99971_d_n13;

        let (assign64800_e99992, assign64800_e99992_d_n0, assign64800_e99992_d_n2, assign64800_e99992_d_n4, assign64800_e99992_d_n5, assign64800_e99992_d_n6, assign64800_e99992_d_n7, assign64800_e99992_d_n8, assign64800_e99992_d_n9, assign64800_e99992_d_n10, assign64800_e99992_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64800_e99986: f64 = (locals.var_t2).ln();
        let assign64800_e99988: f64 = (assign64800_e99986 / locals.var_t3);
        let assign64800_e99990: f64 = (assign64800_e99988 + p.p456);
        (assign64800_e99990, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn4 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn5 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn8 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn9 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn13 / locals.var_t2) * locals.var_t3) - (assign64800_e99986 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn4, locals.var_ps0_inib_dn5, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn8, locals.var_ps0_inib_dn9, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn13,)
    }
};
        locals.var_ps0_inib = assign64800_e99992;
        locals.var_ps0_inib_dn0 = assign64800_e99992_d_n0;
        locals.var_ps0_inib_dn2 = assign64800_e99992_d_n2;
        locals.var_ps0_inib_dn4 = assign64800_e99992_d_n4;
        locals.var_ps0_inib_dn5 = assign64800_e99992_d_n5;
        locals.var_ps0_inib_dn6 = assign64800_e99992_d_n6;
        locals.var_ps0_inib_dn7 = assign64800_e99992_d_n7;
        locals.var_ps0_inib_dn8 = assign64800_e99992_d_n8;
        locals.var_ps0_inib_dn9 = assign64800_e99992_d_n9;
        locals.var_ps0_inib_dn10 = assign64800_e99992_d_n10;
        locals.var_ps0_inib_dn13 = assign64800_e99992_d_n13;

        let (assign64810_e100012, assign64810_e100012_d_n0, assign64810_e100012_d_n2, assign64810_e100012_d_n4, assign64810_e100012_d_n5, assign64810_e100012_d_n6, assign64810_e100012_d_n7, assign64810_e100012_d_n8, assign64810_e100012_d_n9, assign64810_e100012_d_n10, assign64810_e100012_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64810_e100008: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign64810_e100010: f64 = (assign64810_e100008 - 0.0008);
        (assign64810_e100010, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn4 - locals.var_ps0_inia_dn4), (locals.var_ps0_inib_dn5 - locals.var_ps0_inia_dn5), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn8 - locals.var_ps0_inia_dn8), (locals.var_ps0_inib_dn9 - locals.var_ps0_inia_dn9), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn13 - locals.var_ps0_inia_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign64810_e100012;
        locals.var_tmf1_dn0 = assign64810_e100012_d_n0;
        locals.var_tmf1_dn2 = assign64810_e100012_d_n2;
        locals.var_tmf1_dn4 = assign64810_e100012_d_n4;
        locals.var_tmf1_dn5 = assign64810_e100012_d_n5;
        locals.var_tmf1_dn6 = assign64810_e100012_d_n6;
        locals.var_tmf1_dn7 = assign64810_e100012_d_n7;
        locals.var_tmf1_dn8 = assign64810_e100012_d_n8;
        locals.var_tmf1_dn9 = assign64810_e100012_d_n9;
        locals.var_tmf1_dn10 = assign64810_e100012_d_n10;
        locals.var_tmf1_dn13 = assign64810_e100012_d_n13;

        let (assign64820_e100032, assign64820_e100032_d_n0, assign64820_e100032_d_n2, assign64820_e100032_d_n4, assign64820_e100032_d_n5, assign64820_e100032_d_n6, assign64820_e100032_d_n7, assign64820_e100032_d_n8, assign64820_e100032_d_n9, assign64820_e100032_d_n10, assign64820_e100032_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64820_e100028: f64 = (4.0 * locals.var_ps0_inib);
        let assign64820_e100030: f64 = (assign64820_e100028 * 0.0008);
        (assign64820_e100030, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn4) * 0.0008), ((4.0 * locals.var_ps0_inib_dn5) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn8) * 0.0008), ((4.0 * locals.var_ps0_inib_dn9) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn13) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign64820_e100032;
        locals.var_tmf2_dn0 = assign64820_e100032_d_n0;
        locals.var_tmf2_dn2 = assign64820_e100032_d_n2;
        locals.var_tmf2_dn4 = assign64820_e100032_d_n4;
        locals.var_tmf2_dn5 = assign64820_e100032_d_n5;
        locals.var_tmf2_dn6 = assign64820_e100032_d_n6;
        locals.var_tmf2_dn7 = assign64820_e100032_d_n7;
        locals.var_tmf2_dn8 = assign64820_e100032_d_n8;
        locals.var_tmf2_dn9 = assign64820_e100032_d_n9;
        locals.var_tmf2_dn10 = assign64820_e100032_d_n10;
        locals.var_tmf2_dn13 = assign64820_e100032_d_n13;

        let (assign64830_e100054, assign64830_e100054_d_n0, assign64830_e100054_d_n2, assign64830_e100054_d_n4, assign64830_e100054_d_n5, assign64830_e100054_d_n6, assign64830_e100054_d_n7, assign64830_e100054_d_n8, assign64830_e100054_d_n9, assign64830_e100054_d_n10, assign64830_e100054_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let (assign64830_e100052, assign64830_e100052_d_n0, assign64830_e100052_d_n2, assign64830_e100052_d_n4, assign64830_e100052_d_n5, assign64830_e100052_d_n6, assign64830_e100052_d_n7, assign64830_e100052_d_n8, assign64830_e100052_d_n9, assign64830_e100052_d_n10, assign64830_e100052_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign64830_e100051: f64 = (-locals.var_tmf2);
                (assign64830_e100051, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign64830_e100052, assign64830_e100052_d_n0, assign64830_e100052_d_n2, assign64830_e100052_d_n4, assign64830_e100052_d_n5, assign64830_e100052_d_n6, assign64830_e100052_d_n7, assign64830_e100052_d_n8, assign64830_e100052_d_n9, assign64830_e100052_d_n10, assign64830_e100052_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign64830_e100054;
        locals.var_tmf2_dn0 = assign64830_e100054_d_n0;
        locals.var_tmf2_dn2 = assign64830_e100054_d_n2;
        locals.var_tmf2_dn4 = assign64830_e100054_d_n4;
        locals.var_tmf2_dn5 = assign64830_e100054_d_n5;
        locals.var_tmf2_dn6 = assign64830_e100054_d_n6;
        locals.var_tmf2_dn7 = assign64830_e100054_d_n7;
        locals.var_tmf2_dn8 = assign64830_e100054_d_n8;
        locals.var_tmf2_dn9 = assign64830_e100054_d_n9;
        locals.var_tmf2_dn10 = assign64830_e100054_d_n10;
        locals.var_tmf2_dn13 = assign64830_e100054_d_n13;

        let (assign64840_e100075, assign64840_e100075_d_n0, assign64840_e100075_d_n2, assign64840_e100075_d_n4, assign64840_e100075_d_n5, assign64840_e100075_d_n6, assign64840_e100075_d_n7, assign64840_e100075_d_n8, assign64840_e100075_d_n9, assign64840_e100075_d_n10, assign64840_e100075_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64840_e100070: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign64840_e100072: f64 = (assign64840_e100070 + locals.var_tmf2);
        let assign64840_e100073: f64 = (assign64840_e100072).sqrt();
        (assign64840_e100073, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign64840_e100073)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign64840_e100073)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign64840_e100075;
        locals.var_tmf2_dn0 = assign64840_e100075_d_n0;
        locals.var_tmf2_dn2 = assign64840_e100075_d_n2;
        locals.var_tmf2_dn4 = assign64840_e100075_d_n4;
        locals.var_tmf2_dn5 = assign64840_e100075_d_n5;
        locals.var_tmf2_dn6 = assign64840_e100075_d_n6;
        locals.var_tmf2_dn7 = assign64840_e100075_d_n7;
        locals.var_tmf2_dn8 = assign64840_e100075_d_n8;
        locals.var_tmf2_dn9 = assign64840_e100075_d_n9;
        locals.var_tmf2_dn10 = assign64840_e100075_d_n10;
        locals.var_tmf2_dn13 = assign64840_e100075_d_n13;

        let (assign64850_e100097, assign64850_e100097_d_n0, assign64850_e100097_d_n2, assign64850_e100097_d_n4, assign64850_e100097_d_n5, assign64850_e100097_d_n6, assign64850_e100097_d_n7, assign64850_e100097_d_n8, assign64850_e100097_d_n9, assign64850_e100097_d_n10, assign64850_e100097_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1541 == 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64850_e100093: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign64850_e100094: f64 = (0.5 * assign64850_e100093);
        let assign64850_e100095: f64 = (locals.var_ps0_inib - assign64850_e100094);
        (assign64850_e100095, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_ps0_inib_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_ps0_inib_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign64850_e100097;
        locals.var_ps0_ini_dn0 = assign64850_e100097_d_n0;
        locals.var_ps0_ini_dn2 = assign64850_e100097_d_n2;
        locals.var_ps0_ini_dn4 = assign64850_e100097_d_n4;
        locals.var_ps0_ini_dn5 = assign64850_e100097_d_n5;
        locals.var_ps0_ini_dn6 = assign64850_e100097_d_n6;
        locals.var_ps0_ini_dn7 = assign64850_e100097_d_n7;
        locals.var_ps0_ini_dn8 = assign64850_e100097_d_n8;
        locals.var_ps0_ini_dn9 = assign64850_e100097_d_n9;
        locals.var_ps0_ini_dn10 = assign64850_e100097_d_n10;
        locals.var_ps0_ini_dn13 = assign64850_e100097_d_n13;

        let (assign64860_e100111, assign64860_e100111_d_n0, assign64860_e100111_d_n2, assign64860_e100111_d_n4, assign64860_e100111_d_n5, assign64860_e100111_d_n6, assign64860_e100111_d_n7, assign64860_e100111_d_n8, assign64860_e100111_d_n9, assign64860_e100111_d_n10, assign64860_e100111_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        let assign64860_e100108: f64 = (1e-12 / 2.0);
        let assign64860_e100109: f64 = (locals.var_vbscl__blk1539 + assign64860_e100108);
        (assign64860_e100109, locals.var_vbscl__blk1539_dn0, locals.var_vbscl__blk1539_dn2, locals.var_vbscl__blk1539_dn4, locals.var_vbscl__blk1539_dn5, locals.var_vbscl__blk1539_dn6, locals.var_vbscl__blk1539_dn7, locals.var_vbscl__blk1539_dn8, locals.var_vbscl__blk1539_dn9, locals.var_vbscl__blk1539_dn10, locals.var_vbscl__blk1539_dn13,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign64860_e100111;
        locals.var_tx_dn0 = assign64860_e100111_d_n0;
        locals.var_tx_dn2 = assign64860_e100111_d_n2;
        locals.var_tx_dn4 = assign64860_e100111_d_n4;
        locals.var_tx_dn5 = assign64860_e100111_d_n5;
        locals.var_tx_dn6 = assign64860_e100111_d_n6;
        locals.var_tx_dn7 = assign64860_e100111_d_n7;
        locals.var_tx_dn8 = assign64860_e100111_d_n8;
        locals.var_tx_dn9 = assign64860_e100111_d_n9;
        locals.var_tx_dn10 = assign64860_e100111_d_n10;
        locals.var_tx_dn13 = assign64860_e100111_d_n13;

        let assign64870_e100114: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard1543 = assign64870_e100114;

        let (assign64880_e100126, assign64880_e100126_d_n0, assign64880_e100126_d_n2, assign64880_e100126_d_n4, assign64880_e100126_d_n5, assign64880_e100126_d_n6, assign64880_e100126_d_n7, assign64880_e100126_d_n8, assign64880_e100126_d_n9, assign64880_e100126_d_n10, assign64880_e100126_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign64880_e100126;
        locals.var_ps0_ini_dn0 = assign64880_e100126_d_n0;
        locals.var_ps0_ini_dn2 = assign64880_e100126_d_n2;
        locals.var_ps0_ini_dn4 = assign64880_e100126_d_n4;
        locals.var_ps0_ini_dn5 = assign64880_e100126_d_n5;
        locals.var_ps0_ini_dn6 = assign64880_e100126_d_n6;
        locals.var_ps0_ini_dn7 = assign64880_e100126_d_n7;
        locals.var_ps0_ini_dn8 = assign64880_e100126_d_n8;
        locals.var_ps0_ini_dn9 = assign64880_e100126_d_n9;
        locals.var_ps0_ini_dn10 = assign64880_e100126_d_n10;
        locals.var_ps0_ini_dn13 = assign64880_e100126_d_n13;

        let (assign64890_e100136, assign64890_e100136_d_n0, assign64890_e100136_d_n2, assign64890_e100136_d_n4, assign64890_e100136_d_n5, assign64890_e100136_d_n6, assign64890_e100136_d_n7, assign64890_e100136_d_n8, assign64890_e100136_d_n9, assign64890_e100136_d_n10, assign64890_e100136_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    } else {
        (locals.var_ps0__blk1523, locals.var_ps0__blk1523_dn0, locals.var_ps0__blk1523_dn2, locals.var_ps0__blk1523_dn4, locals.var_ps0__blk1523_dn5, locals.var_ps0__blk1523_dn6, locals.var_ps0__blk1523_dn7, locals.var_ps0__blk1523_dn8, locals.var_ps0__blk1523_dn9, locals.var_ps0__blk1523_dn10, locals.var_ps0__blk1523_dn13,)
    }
};
        locals.var_ps0__blk1523 = assign64890_e100136;
        locals.var_ps0__blk1523_dn0 = assign64890_e100136_d_n0;
        locals.var_ps0__blk1523_dn2 = assign64890_e100136_d_n2;
        locals.var_ps0__blk1523_dn4 = assign64890_e100136_d_n4;
        locals.var_ps0__blk1523_dn5 = assign64890_e100136_d_n5;
        locals.var_ps0__blk1523_dn6 = assign64890_e100136_d_n6;
        locals.var_ps0__blk1523_dn7 = assign64890_e100136_d_n7;
        locals.var_ps0__blk1523_dn8 = assign64890_e100136_d_n8;
        locals.var_ps0__blk1523_dn9 = assign64890_e100136_d_n9;
        locals.var_ps0__blk1523_dn10 = assign64890_e100136_d_n10;
        locals.var_ps0__blk1523_dn13 = assign64890_e100136_d_n13;

        let assign64900_e100139: f64 = if p.p451 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1544 = assign64900_e100139;

    }

    pub(super) fn stamp_transient_block_220(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64910_e100151, assign64910_e100151_d_n0, assign64910_e100151_d_n2, assign64910_e100151_d_n4, assign64910_e100151_d_n5, assign64910_e100151_d_n6, assign64910_e100151_d_n7, assign64910_e100151_d_n8, assign64910_e100151_d_n9, assign64910_e100151_d_n10, assign64910_e100151_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) {
        (locals.var_ps0__blk1523, locals.var_ps0__blk1523_dn0, locals.var_ps0__blk1523_dn2, locals.var_ps0__blk1523_dn4, locals.var_ps0__blk1523_dn5, locals.var_ps0__blk1523_dn6, locals.var_ps0__blk1523_dn7, locals.var_ps0__blk1523_dn8, locals.var_ps0__blk1523_dn9, locals.var_ps0__blk1523_dn10, locals.var_ps0__blk1523_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign64910_e100151;
        locals.var_ps0_ini_dn0 = assign64910_e100151_d_n0;
        locals.var_ps0_ini_dn2 = assign64910_e100151_d_n2;
        locals.var_ps0_ini_dn4 = assign64910_e100151_d_n4;
        locals.var_ps0_ini_dn5 = assign64910_e100151_d_n5;
        locals.var_ps0_ini_dn6 = assign64910_e100151_d_n6;
        locals.var_ps0_ini_dn7 = assign64910_e100151_d_n7;
        locals.var_ps0_ini_dn8 = assign64910_e100151_d_n8;
        locals.var_ps0_ini_dn9 = assign64910_e100151_d_n9;
        locals.var_ps0_ini_dn10 = assign64910_e100151_d_n10;
        locals.var_ps0_ini_dn13 = assign64910_e100151_d_n13;

        let (assign64920_e100163, assign64920_e100163_d_n0, assign64920_e100163_d_n2, assign64920_e100163_d_n4, assign64920_e100163_d_n5, assign64920_e100163_d_n6, assign64920_e100163_d_n7, assign64920_e100163_d_n8, assign64920_e100163_d_n9, assign64920_e100163_d_n10, assign64920_e100163_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn13,)
    } else {
        (locals.var_vbscl__blk1545, locals.var_vbscl__blk1545_dn0, locals.var_vbscl__blk1545_dn2, locals.var_vbscl__blk1545_dn4, locals.var_vbscl__blk1545_dn5, locals.var_vbscl__blk1545_dn6, locals.var_vbscl__blk1545_dn7, locals.var_vbscl__blk1545_dn8, locals.var_vbscl__blk1545_dn9, locals.var_vbscl__blk1545_dn10, locals.var_vbscl__blk1545_dn13,)
    }
};
        locals.var_vbscl__blk1545 = assign64920_e100163;
        locals.var_vbscl__blk1545_dn0 = assign64920_e100163_d_n0;
        locals.var_vbscl__blk1545_dn2 = assign64920_e100163_d_n2;
        locals.var_vbscl__blk1545_dn4 = assign64920_e100163_d_n4;
        locals.var_vbscl__blk1545_dn5 = assign64920_e100163_d_n5;
        locals.var_vbscl__blk1545_dn6 = assign64920_e100163_d_n6;
        locals.var_vbscl__blk1545_dn7 = assign64920_e100163_d_n7;
        locals.var_vbscl__blk1545_dn8 = assign64920_e100163_d_n8;
        locals.var_vbscl__blk1545_dn9 = assign64920_e100163_d_n9;
        locals.var_vbscl__blk1545_dn10 = assign64920_e100163_d_n10;
        locals.var_vbscl__blk1545_dn13 = assign64920_e100163_d_n13;

        let (assign64930_e100183,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) {
        let assign64930_e100175: f64 = (locals.var_vfb - locals.var_dvth);
        let assign64930_e100177: f64 = (assign64930_e100175 + locals.var_dppg);
        let assign64930_e100179: f64 = (assign64930_e100177 + locals.var_vbscl__blk1545);
        let assign64930_e100181: f64 = (assign64930_e100179 + p.p455);
        (assign64930_e100181,)
    } else {
        (locals.var_vgs_fb,)
    }
};
        locals.var_vgs_fb = assign64930_e100183;

        let assign64940_e100186: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard1554 = assign64940_e100186;

        let (assign64950_e100201,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign64950_e100199: f64 = (-1.0);
        (assign64950_e100199,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign64950_e100201;

        let (assign64960_e100223, assign64960_e100223_d_n0, assign64960_e100223_d_n2, assign64960_e100223_d_n4, assign64960_e100223_d_n5, assign64960_e100223_d_n6, assign64960_e100223_d_n7, assign64960_e100223_d_n8, assign64960_e100223_d_n9, assign64960_e100223_d_n10, assign64960_e100223_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign64960_e100215: f64 = (2.0 * locals.var_beta_inv);
        let assign64960_e100217: f64 = (-locals.var_vgs_min);
        let assign64960_e100219: f64 = (assign64960_e100217 / locals.var_fac1);
        let assign64960_e100220: f64 = (assign64960_e100219).ln();
        let assign64960_e100221: f64 = (assign64960_e100215 * assign64960_e100220);
        (assign64960_e100221, (((2.0 * locals.var_beta_inv_dn0) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn2) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn4) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn5) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn6) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn7) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn8) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn9) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn10) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))), (((2.0 * locals.var_beta_inv_dn13) * assign64960_e100220) + (assign64960_e100215 * ((-((assign64960_e100217 * locals.var_fac1_dn13) / (locals.var_fac1 * locals.var_fac1))) / assign64960_e100219))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn13,)
    }
};
        locals.var_ps0_min = assign64960_e100223;
        locals.var_ps0_min_dn0 = assign64960_e100223_d_n0;
        locals.var_ps0_min_dn2 = assign64960_e100223_d_n2;
        locals.var_ps0_min_dn4 = assign64960_e100223_d_n4;
        locals.var_ps0_min_dn5 = assign64960_e100223_d_n5;
        locals.var_ps0_min_dn6 = assign64960_e100223_d_n6;
        locals.var_ps0_min_dn7 = assign64960_e100223_d_n7;
        locals.var_ps0_min_dn8 = assign64960_e100223_d_n8;
        locals.var_ps0_min_dn9 = assign64960_e100223_d_n9;
        locals.var_ps0_min_dn10 = assign64960_e100223_d_n10;
        locals.var_ps0_min_dn13 = assign64960_e100223_d_n13;

        let (assign64970_e100241, assign64970_e100241_d_n0, assign64970_e100241_d_n2, assign64970_e100241_d_n4, assign64970_e100241_d_n5, assign64970_e100241_d_n6, assign64970_e100241_d_n7, assign64970_e100241_d_n8, assign64970_e100241_d_n9, assign64970_e100241_d_n10, assign64970_e100241_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign64970_e100238: f64 = (locals.var_vgp__blk1525 - locals.var_vbscl__blk1545);
        let assign64970_e100239: f64 = (locals.var_beta * assign64970_e100238);
        (assign64970_e100239, ((locals.var_beta_dn0 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn0 - locals.var_vbscl__blk1545_dn0))), ((locals.var_beta_dn2 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn2 - locals.var_vbscl__blk1545_dn2))), ((locals.var_beta_dn4 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn4 - locals.var_vbscl__blk1545_dn4))), ((locals.var_beta_dn5 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn5 - locals.var_vbscl__blk1545_dn5))), ((locals.var_beta_dn6 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn6 - locals.var_vbscl__blk1545_dn6))), ((locals.var_beta_dn7 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn7 - locals.var_vbscl__blk1545_dn7))), ((locals.var_beta_dn8 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn8 - locals.var_vbscl__blk1545_dn8))), ((locals.var_beta_dn9 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn9 - locals.var_vbscl__blk1545_dn9))), ((locals.var_beta_dn10 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn10 - locals.var_vbscl__blk1545_dn10))), ((locals.var_beta_dn13 * assign64970_e100238) + (locals.var_beta * (locals.var_vgp__blk1525_dn13 - locals.var_vbscl__blk1545_dn13))),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign64970_e100241;
        locals.var_tx_dn0 = assign64970_e100241_d_n0;
        locals.var_tx_dn2 = assign64970_e100241_d_n2;
        locals.var_tx_dn4 = assign64970_e100241_d_n4;
        locals.var_tx_dn5 = assign64970_e100241_d_n5;
        locals.var_tx_dn6 = assign64970_e100241_d_n6;
        locals.var_tx_dn7 = assign64970_e100241_d_n7;
        locals.var_tx_dn8 = assign64970_e100241_d_n8;
        locals.var_tx_dn9 = assign64970_e100241_d_n9;
        locals.var_tx_dn10 = assign64970_e100241_d_n10;
        locals.var_tx_dn13 = assign64970_e100241_d_n13;

        let (assign64980_e100259, assign64980_e100259_d_n0, assign64980_e100259_d_n2, assign64980_e100259_d_n4, assign64980_e100259_d_n5, assign64980_e100259_d_n6, assign64980_e100259_d_n7, assign64980_e100259_d_n8, assign64980_e100259_d_n9, assign64980_e100259_d_n10, assign64980_e100259_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign64980_e100256: f64 = (locals.var_beta * locals.var_cnst0);
        let assign64980_e100257: f64 = (1.0 / assign64980_e100256);
        (assign64980_e100257, (-(((locals.var_beta_dn0 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn0)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn2 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn2)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn4 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn4)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn5 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn5)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn6 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn6)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn7 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn7)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn8 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn8)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn9 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn9)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn10 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn10)) / (assign64980_e100256 * assign64980_e100256))), (-(((locals.var_beta_dn13 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn13)) / (assign64980_e100256 * assign64980_e100256))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign64980_e100259;
        locals.var_t1_dn0 = assign64980_e100259_d_n0;
        locals.var_t1_dn2 = assign64980_e100259_d_n2;
        locals.var_t1_dn4 = assign64980_e100259_d_n4;
        locals.var_t1_dn5 = assign64980_e100259_d_n5;
        locals.var_t1_dn6 = assign64980_e100259_d_n6;
        locals.var_t1_dn7 = assign64980_e100259_d_n7;
        locals.var_t1_dn8 = assign64980_e100259_d_n8;
        locals.var_t1_dn9 = assign64980_e100259_d_n9;
        locals.var_t1_dn10 = assign64980_e100259_d_n10;
        locals.var_t1_dn13 = assign64980_e100259_d_n13;

        let (assign64990_e100275, assign64990_e100275_d_n0, assign64990_e100275_d_n2, assign64990_e100275_d_n4, assign64990_e100275_d_n5, assign64990_e100275_d_n6, assign64990_e100275_d_n7, assign64990_e100275_d_n8, assign64990_e100275_d_n9, assign64990_e100275_d_n10, assign64990_e100275_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign64990_e100273: f64 = (locals.var_t1 * locals.var_cox);
        (assign64990_e100273, ((locals.var_t1_dn0 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn0)), ((locals.var_t1_dn2 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn2)), ((locals.var_t1_dn4 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn4)), ((locals.var_t1_dn5 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn5)), ((locals.var_t1_dn6 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn6)), ((locals.var_t1_dn7 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn7)), ((locals.var_t1_dn8 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn8)), ((locals.var_t1_dn9 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn9)), ((locals.var_t1_dn10 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn10)), ((locals.var_t1_dn13 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign64990_e100275;
        locals.var_ty_dn0 = assign64990_e100275_d_n0;
        locals.var_ty_dn2 = assign64990_e100275_d_n2;
        locals.var_ty_dn4 = assign64990_e100275_d_n4;
        locals.var_ty_dn5 = assign64990_e100275_d_n5;
        locals.var_ty_dn6 = assign64990_e100275_d_n6;
        locals.var_ty_dn7 = assign64990_e100275_d_n7;
        locals.var_ty_dn8 = assign64990_e100275_d_n8;
        locals.var_ty_dn9 = assign64990_e100275_d_n9;
        locals.var_ty_dn10 = assign64990_e100275_d_n10;
        locals.var_ty_dn13 = assign64990_e100275_d_n13;

        let (assign65000_e100295, assign65000_e100295_d_n0, assign65000_e100295_d_n2, assign65000_e100295_d_n4, assign65000_e100295_d_n5, assign65000_e100295_d_n6, assign65000_e100295_d_n7, assign65000_e100295_d_n8, assign65000_e100295_d_n9, assign65000_e100295_d_n10, assign65000_e100295_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65000_e100290: f64 = (3.0 * 1.414213562373095);
        let assign65000_e100292: f64 = (assign65000_e100290 * locals.var_ty);
        let assign65000_e100293: f64 = (2.0 + assign65000_e100292);
        (assign65000_e100293, (assign65000_e100290 * locals.var_ty_dn0), (assign65000_e100290 * locals.var_ty_dn2), (assign65000_e100290 * locals.var_ty_dn4), (assign65000_e100290 * locals.var_ty_dn5), (assign65000_e100290 * locals.var_ty_dn6), (assign65000_e100290 * locals.var_ty_dn7), (assign65000_e100290 * locals.var_ty_dn8), (assign65000_e100290 * locals.var_ty_dn9), (assign65000_e100290 * locals.var_ty_dn10), (assign65000_e100290 * locals.var_ty_dn13),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn13,)
    }
};
        locals.var_ac41 = assign65000_e100295;
        locals.var_ac41_dn0 = assign65000_e100295_d_n0;
        locals.var_ac41_dn2 = assign65000_e100295_d_n2;
        locals.var_ac41_dn4 = assign65000_e100295_d_n4;
        locals.var_ac41_dn5 = assign65000_e100295_d_n5;
        locals.var_ac41_dn6 = assign65000_e100295_d_n6;
        locals.var_ac41_dn7 = assign65000_e100295_d_n7;
        locals.var_ac41_dn8 = assign65000_e100295_d_n8;
        locals.var_ac41_dn9 = assign65000_e100295_d_n9;
        locals.var_ac41_dn10 = assign65000_e100295_d_n10;
        locals.var_ac41_dn13 = assign65000_e100295_d_n13;

        let (assign65010_e100315, assign65010_e100315_d_n0, assign65010_e100315_d_n2, assign65010_e100315_d_n4, assign65010_e100315_d_n5, assign65010_e100315_d_n6, assign65010_e100315_d_n7, assign65010_e100315_d_n8, assign65010_e100315_d_n9, assign65010_e100315_d_n10, assign65010_e100315_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65010_e100309: f64 = (8.0 * locals.var_ac41);
        let assign65010_e100311: f64 = (assign65010_e100309 * locals.var_ac41);
        let assign65010_e100313: f64 = (assign65010_e100311 * locals.var_ac41);
        (assign65010_e100313, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn13) * locals.var_ac41) + (assign65010_e100309 * locals.var_ac41_dn13)) * locals.var_ac41) + (assign65010_e100311 * locals.var_ac41_dn13)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn13,)
    }
};
        locals.var_ac4 = assign65010_e100315;
        locals.var_ac4_dn0 = assign65010_e100315_d_n0;
        locals.var_ac4_dn2 = assign65010_e100315_d_n2;
        locals.var_ac4_dn4 = assign65010_e100315_d_n4;
        locals.var_ac4_dn5 = assign65010_e100315_d_n5;
        locals.var_ac4_dn6 = assign65010_e100315_d_n6;
        locals.var_ac4_dn7 = assign65010_e100315_d_n7;
        locals.var_ac4_dn8 = assign65010_e100315_d_n8;
        locals.var_ac4_dn9 = assign65010_e100315_d_n9;
        locals.var_ac4_dn10 = assign65010_e100315_d_n10;
        locals.var_ac4_dn13 = assign65010_e100315_d_n13;

        let (assign65020_e100331, assign65020_e100331_d_n0, assign65020_e100331_d_n2, assign65020_e100331_d_n4, assign65020_e100331_d_n5, assign65020_e100331_d_n6, assign65020_e100331_d_n7, assign65020_e100331_d_n8, assign65020_e100331_d_n9, assign65020_e100331_d_n10, assign65020_e100331_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65020_e100329: f64 = (locals.var_tx - 2.0);
        (assign65020_e100329, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign65020_e100331;
        locals.var_t4_dn0 = assign65020_e100331_d_n0;
        locals.var_t4_dn2 = assign65020_e100331_d_n2;
        locals.var_t4_dn4 = assign65020_e100331_d_n4;
        locals.var_t4_dn5 = assign65020_e100331_d_n5;
        locals.var_t4_dn6 = assign65020_e100331_d_n6;
        locals.var_t4_dn7 = assign65020_e100331_d_n7;
        locals.var_t4_dn8 = assign65020_e100331_d_n8;
        locals.var_t4_dn9 = assign65020_e100331_d_n9;
        locals.var_t4_dn10 = assign65020_e100331_d_n10;
        locals.var_t4_dn13 = assign65020_e100331_d_n13;

        let (assign65030_e100349, assign65030_e100349_d_n0, assign65030_e100349_d_n2, assign65030_e100349_d_n4, assign65030_e100349_d_n5, assign65030_e100349_d_n6, assign65030_e100349_d_n7, assign65030_e100349_d_n8, assign65030_e100349_d_n9, assign65030_e100349_d_n10, assign65030_e100349_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65030_e100345: f64 = (9.0 * locals.var_ty);
        let assign65030_e100347: f64 = (assign65030_e100345 * locals.var_t4);
        (assign65030_e100347, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn4) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn4)), (((9.0 * locals.var_ty_dn5) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn5)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn8) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn8)), (((9.0 * locals.var_ty_dn9) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn9)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn13) * locals.var_t4) + (assign65030_e100345 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign65030_e100349;
        locals.var_t5_dn0 = assign65030_e100349_d_n0;
        locals.var_t5_dn2 = assign65030_e100349_d_n2;
        locals.var_t5_dn4 = assign65030_e100349_d_n4;
        locals.var_t5_dn5 = assign65030_e100349_d_n5;
        locals.var_t5_dn6 = assign65030_e100349_d_n6;
        locals.var_t5_dn7 = assign65030_e100349_d_n7;
        locals.var_t5_dn8 = assign65030_e100349_d_n8;
        locals.var_t5_dn9 = assign65030_e100349_d_n9;
        locals.var_t5_dn10 = assign65030_e100349_d_n10;
        locals.var_t5_dn13 = assign65030_e100349_d_n13;

        let (assign65040_e100367, assign65040_e100367_d_n0, assign65040_e100367_d_n2, assign65040_e100367_d_n4, assign65040_e100367_d_n5, assign65040_e100367_d_n6, assign65040_e100367_d_n7, assign65040_e100367_d_n8, assign65040_e100367_d_n9, assign65040_e100367_d_n10, assign65040_e100367_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65040_e100363: f64 = (7.0 * 1.414213562373095);
        let assign65040_e100365: f64 = (assign65040_e100363 - locals.var_t5);
        (assign65040_e100365, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn4), (-locals.var_t5_dn5), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn8), (-locals.var_t5_dn9), (-locals.var_t5_dn10), (-locals.var_t5_dn13),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn13,)
    }
};
        locals.var_ac31 = assign65040_e100367;
        locals.var_ac31_dn0 = assign65040_e100367_d_n0;
        locals.var_ac31_dn2 = assign65040_e100367_d_n2;
        locals.var_ac31_dn4 = assign65040_e100367_d_n4;
        locals.var_ac31_dn5 = assign65040_e100367_d_n5;
        locals.var_ac31_dn6 = assign65040_e100367_d_n6;
        locals.var_ac31_dn7 = assign65040_e100367_d_n7;
        locals.var_ac31_dn8 = assign65040_e100367_d_n8;
        locals.var_ac31_dn9 = assign65040_e100367_d_n9;
        locals.var_ac31_dn10 = assign65040_e100367_d_n10;
        locals.var_ac31_dn13 = assign65040_e100367_d_n13;

        let (assign65050_e100383, assign65050_e100383_d_n0, assign65050_e100383_d_n2, assign65050_e100383_d_n4, assign65050_e100383_d_n5, assign65050_e100383_d_n6, assign65050_e100383_d_n7, assign65050_e100383_d_n8, assign65050_e100383_d_n9, assign65050_e100383_d_n10, assign65050_e100383_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65050_e100381: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign65050_e100381, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn13 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn13)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn13,)
    }
};
        locals.var_ac3 = assign65050_e100383;
        locals.var_ac3_dn0 = assign65050_e100383_d_n0;
        locals.var_ac3_dn2 = assign65050_e100383_d_n2;
        locals.var_ac3_dn4 = assign65050_e100383_d_n4;
        locals.var_ac3_dn5 = assign65050_e100383_d_n5;
        locals.var_ac3_dn6 = assign65050_e100383_d_n6;
        locals.var_ac3_dn7 = assign65050_e100383_d_n7;
        locals.var_ac3_dn8 = assign65050_e100383_d_n8;
        locals.var_ac3_dn9 = assign65050_e100383_d_n9;
        locals.var_ac3_dn10 = assign65050_e100383_d_n10;
        locals.var_ac3_dn13 = assign65050_e100383_d_n13;

        let assign65060_e100387: f64 = (locals.var_ac3 * 1e-8);
        let assign65060_e100388: f64 = if locals.var_ac4 < assign65060_e100387 { 1.0 } else { 0.0 };
        locals.var_guard1555 = assign65060_e100388;

        let (assign65070_e100417, assign65070_e100417_d_n0, assign65070_e100417_d_n2, assign65070_e100417_d_n4, assign65070_e100417_d_n5, assign65070_e100417_d_n6, assign65070_e100417_d_n7, assign65070_e100417_d_n8, assign65070_e100417_d_n9, assign65070_e100417_d_n10, assign65070_e100417_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) && (locals.var_guard1555 != 0.0)) {
        let assign65070_e100403: f64 = (-7.0);
        let assign65070_e100405: f64 = (assign65070_e100403 * 1.414213562373095);
        let assign65070_e100407: f64 = (assign65070_e100405 + locals.var_ac31);
        let assign65070_e100410: f64 = (0.5 * locals.var_ac4);
        let assign65070_e100412: f64 = (assign65070_e100410 / locals.var_ac31);
        let assign65070_e100413: f64 = (assign65070_e100407 + assign65070_e100412);
        let assign65070_e100415: f64 = (assign65070_e100413 + locals.var_t5);
        (assign65070_e100415, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn4 + ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn4), ((locals.var_ac31_dn5 + ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn5), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn8 + ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn8), ((locals.var_ac31_dn9 + ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn9), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn13 + ((((0.5 * locals.var_ac4_dn13) * locals.var_ac31) - (assign65070_e100410 * locals.var_ac31_dn13)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn13),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign65070_e100417;
        locals.var_ac1_dn0 = assign65070_e100417_d_n0;
        locals.var_ac1_dn2 = assign65070_e100417_d_n2;
        locals.var_ac1_dn4 = assign65070_e100417_d_n4;
        locals.var_ac1_dn5 = assign65070_e100417_d_n5;
        locals.var_ac1_dn6 = assign65070_e100417_d_n6;
        locals.var_ac1_dn7 = assign65070_e100417_d_n7;
        locals.var_ac1_dn8 = assign65070_e100417_d_n8;
        locals.var_ac1_dn9 = assign65070_e100417_d_n9;
        locals.var_ac1_dn10 = assign65070_e100417_d_n10;
        locals.var_ac1_dn13 = assign65070_e100417_d_n13;

        let (assign65080_e100437, assign65080_e100437_d_n0, assign65080_e100437_d_n2, assign65080_e100437_d_n4, assign65080_e100437_d_n5, assign65080_e100437_d_n6, assign65080_e100437_d_n7, assign65080_e100437_d_n8, assign65080_e100437_d_n9, assign65080_e100437_d_n10, assign65080_e100437_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) && (locals.var_guard1555 == 0.0)) {
        let assign65080_e100434: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign65080_e100435: f64 = (assign65080_e100434).sqrt();
        (assign65080_e100435, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign65080_e100435)), ((locals.var_ac4_dn13 + locals.var_ac3_dn13) / (2.0 * assign65080_e100435)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn13,)
    }
};
        locals.var_ac2 = assign65080_e100437;
        locals.var_ac2_dn0 = assign65080_e100437_d_n0;
        locals.var_ac2_dn2 = assign65080_e100437_d_n2;
        locals.var_ac2_dn4 = assign65080_e100437_d_n4;
        locals.var_ac2_dn5 = assign65080_e100437_d_n5;
        locals.var_ac2_dn6 = assign65080_e100437_d_n6;
        locals.var_ac2_dn7 = assign65080_e100437_d_n7;
        locals.var_ac2_dn8 = assign65080_e100437_d_n8;
        locals.var_ac2_dn9 = assign65080_e100437_d_n9;
        locals.var_ac2_dn10 = assign65080_e100437_d_n10;
        locals.var_ac2_dn13 = assign65080_e100437_d_n13;

        let (assign65090_e100461, assign65090_e100461_d_n0, assign65090_e100461_d_n2, assign65090_e100461_d_n4, assign65090_e100461_d_n5, assign65090_e100461_d_n6, assign65090_e100461_d_n7, assign65090_e100461_d_n8, assign65090_e100461_d_n9, assign65090_e100461_d_n10, assign65090_e100461_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) && (locals.var_guard1555 == 0.0)) {
        let assign65090_e100453: f64 = (-7.0);
        let assign65090_e100455: f64 = (assign65090_e100453 * 1.414213562373095);
        let assign65090_e100457: f64 = (assign65090_e100455 + locals.var_ac2);
        let assign65090_e100459: f64 = (assign65090_e100457 + locals.var_t5);
        (assign65090_e100459, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn4 + locals.var_t5_dn4), (locals.var_ac2_dn5 + locals.var_t5_dn5), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn8 + locals.var_t5_dn8), (locals.var_ac2_dn9 + locals.var_t5_dn9), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn13 + locals.var_t5_dn13),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign65090_e100461;
        locals.var_ac1_dn0 = assign65090_e100461_d_n0;
        locals.var_ac1_dn2 = assign65090_e100461_d_n2;
        locals.var_ac1_dn4 = assign65090_e100461_d_n4;
        locals.var_ac1_dn5 = assign65090_e100461_d_n5;
        locals.var_ac1_dn6 = assign65090_e100461_d_n6;
        locals.var_ac1_dn7 = assign65090_e100461_d_n7;
        locals.var_ac1_dn8 = assign65090_e100461_d_n8;
        locals.var_ac1_dn9 = assign65090_e100461_d_n9;
        locals.var_ac1_dn10 = assign65090_e100461_d_n10;
        locals.var_ac1_dn13 = assign65090_e100461_d_n13;

        let (assign65100_e100482, assign65100_e100482_d_n0, assign65100_e100482_d_n2, assign65100_e100482_d_n4, assign65100_e100482_d_n5, assign65100_e100482_d_n6, assign65100_e100482_d_n7, assign65100_e100482_d_n8, assign65100_e100482_d_n9, assign65100_e100482_d_n10, assign65100_e100482_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let (assign65100_e100480, assign65100_e100480_d_n0, assign65100_e100480_d_n2, assign65100_e100480_d_n4, assign65100_e100480_d_n5, assign65100_e100480_d_n6, assign65100_e100480_d_n7, assign65100_e100480_d_n8, assign65100_e100480_d_n9, assign65100_e100480_d_n10, assign65100_e100480_d_n13,) = {
            if (locals.var_ac1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65100_e100479: f64 = (locals.var_ac1).powf(0.3333333333333333);
                (assign65100_e100479, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn13)) } } else { (assign65100_e100479 * (0.3333333333333333 * (locals.var_ac1_dn13 / locals.var_ac1))) },)
            }
        };
        (assign65100_e100480, assign65100_e100480_d_n0, assign65100_e100480_d_n2, assign65100_e100480_d_n4, assign65100_e100480_d_n5, assign65100_e100480_d_n6, assign65100_e100480_d_n7, assign65100_e100480_d_n8, assign65100_e100480_d_n9, assign65100_e100480_d_n10, assign65100_e100480_d_n13,)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn13,)
    }
};
        locals.var_acd = assign65100_e100482;
        locals.var_acd_dn0 = assign65100_e100482_d_n0;
        locals.var_acd_dn2 = assign65100_e100482_d_n2;
        locals.var_acd_dn4 = assign65100_e100482_d_n4;
        locals.var_acd_dn5 = assign65100_e100482_d_n5;
        locals.var_acd_dn6 = assign65100_e100482_d_n6;
        locals.var_acd_dn7 = assign65100_e100482_d_n7;
        locals.var_acd_dn8 = assign65100_e100482_d_n8;
        locals.var_acd_dn9 = assign65100_e100482_d_n9;
        locals.var_acd_dn10 = assign65100_e100482_d_n10;
        locals.var_acd_dn13 = assign65100_e100482_d_n13;

        let (assign65110_e100513, assign65110_e100513_d_n0, assign65110_e100513_d_n2, assign65110_e100513_d_n4, assign65110_e100513_d_n5, assign65110_e100513_d_n6, assign65110_e100513_d_n7, assign65110_e100513_d_n8, assign65110_e100513_d_n9, assign65110_e100513_d_n10, assign65110_e100513_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65110_e100495: f64 = (-4.0);
        let assign65110_e100497: f64 = (assign65110_e100495 * 1.414213562373095);
        let assign65110_e100500: f64 = (12.0 * locals.var_ty);
        let assign65110_e100501: f64 = (assign65110_e100497 - assign65110_e100500);
        let assign65110_e100504: f64 = (2.0 * locals.var_acd);
        let assign65110_e100505: f64 = (assign65110_e100501 + assign65110_e100504);
        let assign65110_e100508: f64 = (1.414213562373095 * locals.var_acd);
        let assign65110_e100510: f64 = (assign65110_e100508 * locals.var_acd);
        let assign65110_e100511: f64 = (assign65110_e100505 + assign65110_e100510);
        (assign65110_e100511, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn13)) + (2.0 * locals.var_acd_dn13)) + (((1.414213562373095 * locals.var_acd_dn13) * locals.var_acd) + (assign65110_e100508 * locals.var_acd_dn13))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn13,)
    }
};
        locals.var_acn = assign65110_e100513;
        locals.var_acn_dn0 = assign65110_e100513_d_n0;
        locals.var_acn_dn2 = assign65110_e100513_d_n2;
        locals.var_acn_dn4 = assign65110_e100513_d_n4;
        locals.var_acn_dn5 = assign65110_e100513_d_n5;
        locals.var_acn_dn6 = assign65110_e100513_d_n6;
        locals.var_acn_dn7 = assign65110_e100513_d_n7;
        locals.var_acn_dn8 = assign65110_e100513_d_n8;
        locals.var_acn_dn9 = assign65110_e100513_d_n9;
        locals.var_acn_dn10 = assign65110_e100513_d_n10;
        locals.var_acn_dn13 = assign65110_e100513_d_n13;

        let (assign65120_e100529, assign65120_e100529_d_n0, assign65120_e100529_d_n2, assign65120_e100529_d_n4, assign65120_e100529_d_n5, assign65120_e100529_d_n6, assign65120_e100529_d_n7, assign65120_e100529_d_n8, assign65120_e100529_d_n9, assign65120_e100529_d_n10, assign65120_e100529_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65120_e100527: f64 = (1.0 / locals.var_acd);
        (assign65120_e100527, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn4 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn5 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn8 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn9 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn13 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65120_e100529;
        locals.var_t1_dn0 = assign65120_e100529_d_n0;
        locals.var_t1_dn2 = assign65120_e100529_d_n2;
        locals.var_t1_dn4 = assign65120_e100529_d_n4;
        locals.var_t1_dn5 = assign65120_e100529_d_n5;
        locals.var_t1_dn6 = assign65120_e100529_d_n6;
        locals.var_t1_dn7 = assign65120_e100529_d_n7;
        locals.var_t1_dn8 = assign65120_e100529_d_n8;
        locals.var_t1_dn9 = assign65120_e100529_d_n9;
        locals.var_t1_dn10 = assign65120_e100529_d_n10;
        locals.var_t1_dn13 = assign65120_e100529_d_n13;

        let (assign65130_e100545, assign65130_e100545_d_n0, assign65130_e100545_d_n2, assign65130_e100545_d_n4, assign65130_e100545_d_n5, assign65130_e100545_d_n6, assign65130_e100545_d_n7, assign65130_e100545_d_n8, assign65130_e100545_d_n9, assign65130_e100545_d_n10, assign65130_e100545_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65130_e100543: f64 = (locals.var_acn * locals.var_t1);
        (assign65130_e100543, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn4 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn4)), ((locals.var_acn_dn5 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn5)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn8 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn8)), ((locals.var_acn_dn9 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn9)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn13 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign65130_e100545;
        locals.var_chi_dn0 = assign65130_e100545_d_n0;
        locals.var_chi_dn2 = assign65130_e100545_d_n2;
        locals.var_chi_dn4 = assign65130_e100545_d_n4;
        locals.var_chi_dn5 = assign65130_e100545_d_n5;
        locals.var_chi_dn6 = assign65130_e100545_d_n6;
        locals.var_chi_dn7 = assign65130_e100545_d_n7;
        locals.var_chi_dn8 = assign65130_e100545_d_n8;
        locals.var_chi_dn9 = assign65130_e100545_d_n9;
        locals.var_chi_dn10 = assign65130_e100545_d_n10;
        locals.var_chi_dn13 = assign65130_e100545_d_n13;

        let (assign65140_e100563, assign65140_e100563_d_n0, assign65140_e100563_d_n2, assign65140_e100563_d_n4, assign65140_e100563_d_n5, assign65140_e100563_d_n6, assign65140_e100563_d_n7, assign65140_e100563_d_n8, assign65140_e100563_d_n9, assign65140_e100563_d_n10, assign65140_e100563_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65140_e100559: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign65140_e100561: f64 = (assign65140_e100559 + locals.var_vbscl__blk1545);
        (assign65140_e100561, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk1545_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk1545_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk1545_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk1545_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk1545_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk1545_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk1545_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk1545_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk1545_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) + locals.var_vbscl__blk1545_dn13),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn4, locals.var_psa_dn5, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn8, locals.var_psa_dn9, locals.var_psa_dn10, locals.var_psa_dn13,)
    }
};
        locals.var_psa = assign65140_e100563;
        locals.var_psa_dn0 = assign65140_e100563_d_n0;
        locals.var_psa_dn2 = assign65140_e100563_d_n2;
        locals.var_psa_dn4 = assign65140_e100563_d_n4;
        locals.var_psa_dn5 = assign65140_e100563_d_n5;
        locals.var_psa_dn6 = assign65140_e100563_d_n6;
        locals.var_psa_dn7 = assign65140_e100563_d_n7;
        locals.var_psa_dn8 = assign65140_e100563_d_n8;
        locals.var_psa_dn9 = assign65140_e100563_d_n9;
        locals.var_psa_dn10 = assign65140_e100563_d_n10;
        locals.var_psa_dn13 = assign65140_e100563_d_n13;

        let (assign65150_e100579, assign65150_e100579_d_n0, assign65150_e100579_d_n2, assign65150_e100579_d_n4, assign65150_e100579_d_n5, assign65150_e100579_d_n6, assign65150_e100579_d_n7, assign65150_e100579_d_n8, assign65150_e100579_d_n9, assign65150_e100579_d_n10, assign65150_e100579_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65150_e100577: f64 = (locals.var_psa - locals.var_vbscl__blk1545);
        (assign65150_e100577, (locals.var_psa_dn0 - locals.var_vbscl__blk1545_dn0), (locals.var_psa_dn2 - locals.var_vbscl__blk1545_dn2), (locals.var_psa_dn4 - locals.var_vbscl__blk1545_dn4), (locals.var_psa_dn5 - locals.var_vbscl__blk1545_dn5), (locals.var_psa_dn6 - locals.var_vbscl__blk1545_dn6), (locals.var_psa_dn7 - locals.var_vbscl__blk1545_dn7), (locals.var_psa_dn8 - locals.var_vbscl__blk1545_dn8), (locals.var_psa_dn9 - locals.var_vbscl__blk1545_dn9), (locals.var_psa_dn10 - locals.var_vbscl__blk1545_dn10), (locals.var_psa_dn13 - locals.var_vbscl__blk1545_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65150_e100579;
        locals.var_t1_dn0 = assign65150_e100579_d_n0;
        locals.var_t1_dn2 = assign65150_e100579_d_n2;
        locals.var_t1_dn4 = assign65150_e100579_d_n4;
        locals.var_t1_dn5 = assign65150_e100579_d_n5;
        locals.var_t1_dn6 = assign65150_e100579_d_n6;
        locals.var_t1_dn7 = assign65150_e100579_d_n7;
        locals.var_t1_dn8 = assign65150_e100579_d_n8;
        locals.var_t1_dn9 = assign65150_e100579_d_n9;
        locals.var_t1_dn10 = assign65150_e100579_d_n10;
        locals.var_t1_dn13 = assign65150_e100579_d_n13;

        let (assign65160_e100595, assign65160_e100595_d_n0, assign65160_e100595_d_n2, assign65160_e100595_d_n4, assign65160_e100595_d_n5, assign65160_e100595_d_n6, assign65160_e100595_d_n7, assign65160_e100595_d_n8, assign65160_e100595_d_n9, assign65160_e100595_d_n10, assign65160_e100595_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65160_e100593: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign65160_e100593, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn13 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn13)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign65160_e100595;
        locals.var_t2_dn0 = assign65160_e100595_d_n0;
        locals.var_t2_dn2 = assign65160_e100595_d_n2;
        locals.var_t2_dn4 = assign65160_e100595_d_n4;
        locals.var_t2_dn5 = assign65160_e100595_d_n5;
        locals.var_t2_dn6 = assign65160_e100595_d_n6;
        locals.var_t2_dn7 = assign65160_e100595_d_n7;
        locals.var_t2_dn8 = assign65160_e100595_d_n8;
        locals.var_t2_dn9 = assign65160_e100595_d_n9;
        locals.var_t2_dn10 = assign65160_e100595_d_n10;
        locals.var_t2_dn13 = assign65160_e100595_d_n13;

    }

    pub(super) fn stamp_transient_block_221(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65170_e100614, assign65170_e100614_d_n0, assign65170_e100614_d_n2, assign65170_e100614_d_n4, assign65170_e100614_d_n5, assign65170_e100614_d_n6, assign65170_e100614_d_n7, assign65170_e100614_d_n8, assign65170_e100614_d_n9, assign65170_e100614_d_n10, assign65170_e100614_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65170_e100610: f64 = (locals.var_t2 * locals.var_t2);
        let assign65170_e100611: f64 = (1.0 + assign65170_e100610);
        let assign65170_e100612: f64 = (assign65170_e100611).sqrt();
        (assign65170_e100612, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign65170_e100612)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign65170_e100612)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign65170_e100614;
        locals.var_t3_dn0 = assign65170_e100614_d_n0;
        locals.var_t3_dn2 = assign65170_e100614_d_n2;
        locals.var_t3_dn4 = assign65170_e100614_d_n4;
        locals.var_t3_dn5 = assign65170_e100614_d_n5;
        locals.var_t3_dn6 = assign65170_e100614_d_n6;
        locals.var_t3_dn7 = assign65170_e100614_d_n7;
        locals.var_t3_dn8 = assign65170_e100614_d_n8;
        locals.var_t3_dn9 = assign65170_e100614_d_n9;
        locals.var_t3_dn10 = assign65170_e100614_d_n10;
        locals.var_t3_dn13 = assign65170_e100614_d_n13;

        let (assign65180_e100632, assign65180_e100632_d_n0, assign65180_e100632_d_n2, assign65180_e100632_d_n4, assign65180_e100632_d_n5, assign65180_e100632_d_n6, assign65180_e100632_d_n7, assign65180_e100632_d_n8, assign65180_e100632_d_n9, assign65180_e100632_d_n10, assign65180_e100632_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 != 0.0)) {
        let assign65180_e100628: f64 = (locals.var_t1 / locals.var_t3);
        let assign65180_e100630: f64 = (assign65180_e100628 + locals.var_vbscl__blk1545);
        (assign65180_e100630, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn10), ((((locals.var_t1_dn13 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1545_dn13),)
    } else {
        (locals.var_ps0__blk1523, locals.var_ps0__blk1523_dn0, locals.var_ps0__blk1523_dn2, locals.var_ps0__blk1523_dn4, locals.var_ps0__blk1523_dn5, locals.var_ps0__blk1523_dn6, locals.var_ps0__blk1523_dn7, locals.var_ps0__blk1523_dn8, locals.var_ps0__blk1523_dn9, locals.var_ps0__blk1523_dn10, locals.var_ps0__blk1523_dn13,)
    }
};
        locals.var_ps0__blk1523 = assign65180_e100632;
        locals.var_ps0__blk1523_dn0 = assign65180_e100632_d_n0;
        locals.var_ps0__blk1523_dn2 = assign65180_e100632_d_n2;
        locals.var_ps0__blk1523_dn4 = assign65180_e100632_d_n4;
        locals.var_ps0__blk1523_dn5 = assign65180_e100632_d_n5;
        locals.var_ps0__blk1523_dn6 = assign65180_e100632_d_n6;
        locals.var_ps0__blk1523_dn7 = assign65180_e100632_d_n7;
        locals.var_ps0__blk1523_dn8 = assign65180_e100632_d_n8;
        locals.var_ps0__blk1523_dn9 = assign65180_e100632_d_n9;
        locals.var_ps0__blk1523_dn10 = assign65180_e100632_d_n10;
        locals.var_ps0__blk1523_dn13 = assign65180_e100632_d_n13;

        let (assign65190_e100652, assign65190_e100652_d_n0, assign65190_e100652_d_n2, assign65190_e100652_d_n4, assign65190_e100652_d_n5, assign65190_e100652_d_n6, assign65190_e100652_d_n7, assign65190_e100652_d_n8, assign65190_e100652_d_n9, assign65190_e100652_d_n10, assign65190_e100652_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65190_e100648: f64 = (locals.var_vbscl__blk1545 - p.p456);
        let assign65190_e100649: f64 = (locals.var_beta * assign65190_e100648);
        let assign65190_e100650: f64 = (assign65190_e100649).exp();
        (assign65190_e100650, (assign65190_e100650 * ((locals.var_beta_dn0 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn0))), (assign65190_e100650 * ((locals.var_beta_dn2 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn2))), (assign65190_e100650 * ((locals.var_beta_dn4 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn4))), (assign65190_e100650 * ((locals.var_beta_dn5 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn5))), (assign65190_e100650 * ((locals.var_beta_dn6 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn6))), (assign65190_e100650 * ((locals.var_beta_dn7 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn7))), (assign65190_e100650 * ((locals.var_beta_dn8 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn8))), (assign65190_e100650 * ((locals.var_beta_dn9 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn9))), (assign65190_e100650 * ((locals.var_beta_dn10 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn10))), (assign65190_e100650 * ((locals.var_beta_dn13 * assign65190_e100648) + (locals.var_beta * locals.var_vbscl__blk1545_dn13))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn4, locals.var_exp_bvbsvds_dn5, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn8, locals.var_exp_bvbsvds_dn9, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn13,)
    }
};
        locals.var_exp_bvbsvds = assign65190_e100652;
        locals.var_exp_bvbsvds_dn0 = assign65190_e100652_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign65190_e100652_d_n2;
        locals.var_exp_bvbsvds_dn4 = assign65190_e100652_d_n4;
        locals.var_exp_bvbsvds_dn5 = assign65190_e100652_d_n5;
        locals.var_exp_bvbsvds_dn6 = assign65190_e100652_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign65190_e100652_d_n7;
        locals.var_exp_bvbsvds_dn8 = assign65190_e100652_d_n8;
        locals.var_exp_bvbsvds_dn9 = assign65190_e100652_d_n9;
        locals.var_exp_bvbsvds_dn10 = assign65190_e100652_d_n10;
        locals.var_exp_bvbsvds_dn13 = assign65190_e100652_d_n13;

        let (assign65200_e100667,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign65200_e100667;

        let (assign65210_e100682, assign65210_e100682_d_n0, assign65210_e100682_d_n2, assign65210_e100682_d_n4, assign65210_e100682_d_n5, assign65210_e100682_d_n6, assign65210_e100682_d_n7, assign65210_e100682_d_n8, assign65210_e100682_d_n9, assign65210_e100682_d_n10, assign65210_e100682_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    } else {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn13,)
    }
};
        locals.var_phi_s0 = assign65210_e100682;
        locals.var_phi_s0_dn0 = assign65210_e100682_d_n0;
        locals.var_phi_s0_dn2 = assign65210_e100682_d_n2;
        locals.var_phi_s0_dn4 = assign65210_e100682_d_n4;
        locals.var_phi_s0_dn5 = assign65210_e100682_d_n5;
        locals.var_phi_s0_dn6 = assign65210_e100682_d_n6;
        locals.var_phi_s0_dn7 = assign65210_e100682_d_n7;
        locals.var_phi_s0_dn8 = assign65210_e100682_d_n8;
        locals.var_phi_s0_dn9 = assign65210_e100682_d_n9;
        locals.var_phi_s0_dn10 = assign65210_e100682_d_n10;
        locals.var_phi_s0_dn13 = assign65210_e100682_d_n13;

        let (assign65220_e100705, assign65220_e100705_d_n0, assign65220_e100705_d_n2, assign65220_e100705_d_n4, assign65220_e100705_d_n5, assign65220_e100705_d_n6, assign65220_e100705_d_n7, assign65220_e100705_d_n8, assign65220_e100705_d_n9, assign65220_e100705_d_n10, assign65220_e100705_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65220_e100697: f64 = (locals.var_q_nsub * locals.var_t_sub);
        let assign65220_e100699: f64 = (assign65220_e100697 * locals.var_t_sub);
        let assign65220_e100701: f64 = (assign65220_e100699 / 2.0);
        let assign65220_e100703: f64 = (assign65220_e100701 / 1.034943e-10);
        (assign65220_e100703, ((((locals.var_q_nsub_dn0 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn4 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn5 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn8 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn9 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn13 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1547, locals.var_dphi_sb__blk1547_dn0, locals.var_dphi_sb__blk1547_dn2, locals.var_dphi_sb__blk1547_dn4, locals.var_dphi_sb__blk1547_dn5, locals.var_dphi_sb__blk1547_dn6, locals.var_dphi_sb__blk1547_dn7, locals.var_dphi_sb__blk1547_dn8, locals.var_dphi_sb__blk1547_dn9, locals.var_dphi_sb__blk1547_dn10, locals.var_dphi_sb__blk1547_dn13,)
    }
};
        locals.var_dphi_sb__blk1547 = assign65220_e100705;
        locals.var_dphi_sb__blk1547_dn0 = assign65220_e100705_d_n0;
        locals.var_dphi_sb__blk1547_dn2 = assign65220_e100705_d_n2;
        locals.var_dphi_sb__blk1547_dn4 = assign65220_e100705_d_n4;
        locals.var_dphi_sb__blk1547_dn5 = assign65220_e100705_d_n5;
        locals.var_dphi_sb__blk1547_dn6 = assign65220_e100705_d_n6;
        locals.var_dphi_sb__blk1547_dn7 = assign65220_e100705_d_n7;
        locals.var_dphi_sb__blk1547_dn8 = assign65220_e100705_d_n8;
        locals.var_dphi_sb__blk1547_dn9 = assign65220_e100705_d_n9;
        locals.var_dphi_sb__blk1547_dn10 = assign65220_e100705_d_n10;
        locals.var_dphi_sb__blk1547_dn13 = assign65220_e100705_d_n13;

        let (assign65230_e100725, assign65230_e100725_d_n0, assign65230_e100725_d_n2, assign65230_e100725_d_n4, assign65230_e100725_d_n5, assign65230_e100725_d_n6, assign65230_e100725_d_n7, assign65230_e100725_d_n8, assign65230_e100725_d_n9, assign65230_e100725_d_n10, assign65230_e100725_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65230_e100720: f64 = (2.0 * locals.var_beta);
        let assign65230_e100722: f64 = (assign65230_e100720 * locals.var_dphi_sb__blk1547);
        let assign65230_e100723: f64 = (assign65230_e100722).sqrt();
        (assign65230_e100723, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn0)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn2)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn4)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn5)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn6)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn7)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn8)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn9)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn10)) / (2.0 * assign65230_e100723)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb__blk1547) + (assign65230_e100720 * locals.var_dphi_sb__blk1547_dn13)) / (2.0 * assign65230_e100723)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign65230_e100725;
        locals.var_t0_dn0 = assign65230_e100725_d_n0;
        locals.var_t0_dn2 = assign65230_e100725_d_n2;
        locals.var_t0_dn4 = assign65230_e100725_d_n4;
        locals.var_t0_dn5 = assign65230_e100725_d_n5;
        locals.var_t0_dn6 = assign65230_e100725_d_n6;
        locals.var_t0_dn7 = assign65230_e100725_d_n7;
        locals.var_t0_dn8 = assign65230_e100725_d_n8;
        locals.var_t0_dn9 = assign65230_e100725_d_n9;
        locals.var_t0_dn10 = assign65230_e100725_d_n10;
        locals.var_t0_dn13 = assign65230_e100725_d_n13;

        let (assign65240_e100747, assign65240_e100747_d_n0, assign65240_e100747_d_n2, assign65240_e100747_d_n4, assign65240_e100747_d_n5, assign65240_e100747_d_n6, assign65240_e100747_d_n7, assign65240_e100747_d_n8, assign65240_e100747_d_n9, assign65240_e100747_d_n10, assign65240_e100747_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65240_e100739: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign65240_e100741: f64 = (-locals.var_t0);
        let assign65240_e100742: f64 = { let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign65240_e100743: f64 = (assign65240_e100739 + assign65240_e100742);
        let assign65240_e100745: f64 = (assign65240_e100743 / 2.0);
        (assign65240_e100745, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign65240_e100741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65240_e100747;
        locals.var_t1_dn0 = assign65240_e100747_d_n0;
        locals.var_t1_dn2 = assign65240_e100747_d_n2;
        locals.var_t1_dn4 = assign65240_e100747_d_n4;
        locals.var_t1_dn5 = assign65240_e100747_d_n5;
        locals.var_t1_dn6 = assign65240_e100747_d_n6;
        locals.var_t1_dn7 = assign65240_e100747_d_n7;
        locals.var_t1_dn8 = assign65240_e100747_d_n8;
        locals.var_t1_dn9 = assign65240_e100747_d_n9;
        locals.var_t1_dn10 = assign65240_e100747_d_n10;
        locals.var_t1_dn13 = assign65240_e100747_d_n13;

        let (assign65250_e100765, assign65250_e100765_d_n0, assign65250_e100765_d_n2, assign65250_e100765_d_n4, assign65250_e100765_d_n5, assign65250_e100765_d_n6, assign65250_e100765_d_n7, assign65250_e100765_d_n8, assign65250_e100765_d_n9, assign65250_e100765_d_n10, assign65250_e100765_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65250_e100761: f64 = (locals.var_t1).ln();
        let assign65250_e100763: f64 = (assign65250_e100761 / locals.var_dphi_sb__blk1547);
        (assign65250_e100763, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn0)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn2)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn4)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn5)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn6)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn7)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn8)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn9)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn10)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb__blk1547) - (assign65250_e100761 * locals.var_dphi_sb__blk1547_dn13)) / (locals.var_dphi_sb__blk1547 * locals.var_dphi_sb__blk1547)),)
    } else {
        (locals.var_c_sb__blk1548, locals.var_c_sb__blk1548_dn0, locals.var_c_sb__blk1548_dn2, locals.var_c_sb__blk1548_dn4, locals.var_c_sb__blk1548_dn5, locals.var_c_sb__blk1548_dn6, locals.var_c_sb__blk1548_dn7, locals.var_c_sb__blk1548_dn8, locals.var_c_sb__blk1548_dn9, locals.var_c_sb__blk1548_dn10, locals.var_c_sb__blk1548_dn13,)
    }
};
        locals.var_c_sb__blk1548 = assign65250_e100765;
        locals.var_c_sb__blk1548_dn0 = assign65250_e100765_d_n0;
        locals.var_c_sb__blk1548_dn2 = assign65250_e100765_d_n2;
        locals.var_c_sb__blk1548_dn4 = assign65250_e100765_d_n4;
        locals.var_c_sb__blk1548_dn5 = assign65250_e100765_d_n5;
        locals.var_c_sb__blk1548_dn6 = assign65250_e100765_d_n6;
        locals.var_c_sb__blk1548_dn7 = assign65250_e100765_d_n7;
        locals.var_c_sb__blk1548_dn8 = assign65250_e100765_d_n8;
        locals.var_c_sb__blk1548_dn9 = assign65250_e100765_d_n9;
        locals.var_c_sb__blk1548_dn10 = assign65250_e100765_d_n10;
        locals.var_c_sb__blk1548_dn13 = assign65250_e100765_d_n13;

        let (assign65260_e100780,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign65260_e100780;

    }

    pub(super) fn stamp_transient_block_222(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign65270_loop_guard: usize = 0;
        while {
            let assign65270_cond_e100796: f64 = (locals.var_lp_s0_max + 1.0);
            let assign65270_cond_e100798: f64 = if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_lp_s0 <= assign65270_cond_e100796)) { 1.0 } else { 0.0 };
            assign65270_cond_e100798 != 0.0
        } {
            assign65270_loop_guard += 1;
            assert!(assign65270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign65270_body0_e100815, assign65270_body0_e100815_d_n0, assign65270_body0_e100815_d_n2, assign65270_body0_e100815_d_n4, assign65270_body0_e100815_d_n5, assign65270_body0_e100815_d_n6, assign65270_body0_e100815_d_n7, assign65270_body0_e100815_d_n8, assign65270_body0_e100815_d_n9, assign65270_body0_e100815_d_n10, assign65270_body0_e100815_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body0_e100813: f64 = (locals.var_phi_s0 - locals.var_vbscl__blk1545);
        (assign65270_body0_e100813, (locals.var_phi_s0_dn0 - locals.var_vbscl__blk1545_dn0), (locals.var_phi_s0_dn2 - locals.var_vbscl__blk1545_dn2), (locals.var_phi_s0_dn4 - locals.var_vbscl__blk1545_dn4), (locals.var_phi_s0_dn5 - locals.var_vbscl__blk1545_dn5), (locals.var_phi_s0_dn6 - locals.var_vbscl__blk1545_dn6), (locals.var_phi_s0_dn7 - locals.var_vbscl__blk1545_dn7), (locals.var_phi_s0_dn8 - locals.var_vbscl__blk1545_dn8), (locals.var_phi_s0_dn9 - locals.var_vbscl__blk1545_dn9), (locals.var_phi_s0_dn10 - locals.var_vbscl__blk1545_dn10), (locals.var_phi_s0_dn13 - locals.var_vbscl__blk1545_dn13),)
    } else {
        (locals.var_phi_0, locals.var_phi_0_dn0, locals.var_phi_0_dn2, locals.var_phi_0_dn4, locals.var_phi_0_dn5, locals.var_phi_0_dn6, locals.var_phi_0_dn7, locals.var_phi_0_dn8, locals.var_phi_0_dn9, locals.var_phi_0_dn10, locals.var_phi_0_dn13,)
    }
};
            locals.var_phi_0 = assign65270_body0_e100815;
            locals.var_phi_0_dn0 = assign65270_body0_e100815_d_n0;
            locals.var_phi_0_dn2 = assign65270_body0_e100815_d_n2;
            locals.var_phi_0_dn4 = assign65270_body0_e100815_d_n4;
            locals.var_phi_0_dn5 = assign65270_body0_e100815_d_n5;
            locals.var_phi_0_dn6 = assign65270_body0_e100815_d_n6;
            locals.var_phi_0_dn7 = assign65270_body0_e100815_d_n7;
            locals.var_phi_0_dn8 = assign65270_body0_e100815_d_n8;
            locals.var_phi_0_dn9 = assign65270_body0_e100815_d_n9;
            locals.var_phi_0_dn10 = assign65270_body0_e100815_d_n10;
            locals.var_phi_0_dn13 = assign65270_body0_e100815_d_n13;
            let (assign65270_body1_e100832, assign65270_body1_e100832_d_n0, assign65270_body1_e100832_d_n2, assign65270_body1_e100832_d_n4, assign65270_body1_e100832_d_n5, assign65270_body1_e100832_d_n6, assign65270_body1_e100832_d_n7, assign65270_body1_e100832_d_n8, assign65270_body1_e100832_d_n9, assign65270_body1_e100832_d_n10, assign65270_body1_e100832_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body1_e100830: f64 = (locals.var_beta * locals.var_phi_0);
        (assign65270_body1_e100830, ((locals.var_beta_dn0 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn0)), ((locals.var_beta_dn2 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn2)), ((locals.var_beta_dn4 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn4)), ((locals.var_beta_dn5 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn5)), ((locals.var_beta_dn6 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn6)), ((locals.var_beta_dn7 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn7)), ((locals.var_beta_dn8 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn8)), ((locals.var_beta_dn9 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn9)), ((locals.var_beta_dn10 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn10)), ((locals.var_beta_dn13 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign65270_body1_e100832;
            locals.var_chi_dn0 = assign65270_body1_e100832_d_n0;
            locals.var_chi_dn2 = assign65270_body1_e100832_d_n2;
            locals.var_chi_dn4 = assign65270_body1_e100832_d_n4;
            locals.var_chi_dn5 = assign65270_body1_e100832_d_n5;
            locals.var_chi_dn6 = assign65270_body1_e100832_d_n6;
            locals.var_chi_dn7 = assign65270_body1_e100832_d_n7;
            locals.var_chi_dn8 = assign65270_body1_e100832_d_n8;
            locals.var_chi_dn9 = assign65270_body1_e100832_d_n9;
            locals.var_chi_dn10 = assign65270_body1_e100832_d_n10;
            locals.var_chi_dn13 = assign65270_body1_e100832_d_n13;
            let (assign65270_body2_e100851, assign65270_body2_e100851_d_n0, assign65270_body2_e100851_d_n2, assign65270_body2_e100851_d_n4, assign65270_body2_e100851_d_n5, assign65270_body2_e100851_d_n6, assign65270_body2_e100851_d_n7, assign65270_body2_e100851_d_n8, assign65270_body2_e100851_d_n9, assign65270_body2_e100851_d_n10, assign65270_body2_e100851_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body2_e100848: f64 = (locals.var_phi_0 - locals.var_dphi_sb__blk1547);
        let assign65270_body2_e100849: f64 = (locals.var_c_sb__blk1548 * assign65270_body2_e100848);
        (assign65270_body2_e100849, ((locals.var_c_sb__blk1548_dn0 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn0 - locals.var_dphi_sb__blk1547_dn0))), ((locals.var_c_sb__blk1548_dn2 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn2 - locals.var_dphi_sb__blk1547_dn2))), ((locals.var_c_sb__blk1548_dn4 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn4 - locals.var_dphi_sb__blk1547_dn4))), ((locals.var_c_sb__blk1548_dn5 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn5 - locals.var_dphi_sb__blk1547_dn5))), ((locals.var_c_sb__blk1548_dn6 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn6 - locals.var_dphi_sb__blk1547_dn6))), ((locals.var_c_sb__blk1548_dn7 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn7 - locals.var_dphi_sb__blk1547_dn7))), ((locals.var_c_sb__blk1548_dn8 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn8 - locals.var_dphi_sb__blk1547_dn8))), ((locals.var_c_sb__blk1548_dn9 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn9 - locals.var_dphi_sb__blk1547_dn9))), ((locals.var_c_sb__blk1548_dn10 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn10 - locals.var_dphi_sb__blk1547_dn10))), ((locals.var_c_sb__blk1548_dn13 * assign65270_body2_e100848) + (locals.var_c_sb__blk1548 * (locals.var_phi_0_dn13 - locals.var_dphi_sb__blk1547_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign65270_body2_e100851;
            locals.var_ty_dn0 = assign65270_body2_e100851_d_n0;
            locals.var_ty_dn2 = assign65270_body2_e100851_d_n2;
            locals.var_ty_dn4 = assign65270_body2_e100851_d_n4;
            locals.var_ty_dn5 = assign65270_body2_e100851_d_n5;
            locals.var_ty_dn6 = assign65270_body2_e100851_d_n6;
            locals.var_ty_dn7 = assign65270_body2_e100851_d_n7;
            locals.var_ty_dn8 = assign65270_body2_e100851_d_n8;
            locals.var_ty_dn9 = assign65270_body2_e100851_d_n9;
            locals.var_ty_dn10 = assign65270_body2_e100851_d_n10;
            locals.var_ty_dn13 = assign65270_body2_e100851_d_n13;
            let assign65270_body3_e100854: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1556 = assign65270_body3_e100854;
            let (assign65270_body4_e100872, assign65270_body4_e100872_d_n0, assign65270_body4_e100872_d_n2, assign65270_body4_e100872_d_n4, assign65270_body4_e100872_d_n5, assign65270_body4_e100872_d_n6, assign65270_body4_e100872_d_n7, assign65270_body4_e100872_d_n8, assign65270_body4_e100872_d_n9, assign65270_body4_e100872_d_n10, assign65270_body4_e100872_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65270_body4_e100870: f64 = (locals.var_ty).exp();
        (assign65270_body4_e100870, (assign65270_body4_e100870 * locals.var_ty_dn0), (assign65270_body4_e100870 * locals.var_ty_dn2), (assign65270_body4_e100870 * locals.var_ty_dn4), (assign65270_body4_e100870 * locals.var_ty_dn5), (assign65270_body4_e100870 * locals.var_ty_dn6), (assign65270_body4_e100870 * locals.var_ty_dn7), (assign65270_body4_e100870 * locals.var_ty_dn8), (assign65270_body4_e100870 * locals.var_ty_dn9), (assign65270_body4_e100870 * locals.var_ty_dn10), (assign65270_body4_e100870 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign65270_body4_e100872;
            locals.var_t1_dn0 = assign65270_body4_e100872_d_n0;
            locals.var_t1_dn2 = assign65270_body4_e100872_d_n2;
            locals.var_t1_dn4 = assign65270_body4_e100872_d_n4;
            locals.var_t1_dn5 = assign65270_body4_e100872_d_n5;
            locals.var_t1_dn6 = assign65270_body4_e100872_d_n6;
            locals.var_t1_dn7 = assign65270_body4_e100872_d_n7;
            locals.var_t1_dn8 = assign65270_body4_e100872_d_n8;
            locals.var_t1_dn9 = assign65270_body4_e100872_d_n9;
            locals.var_t1_dn10 = assign65270_body4_e100872_d_n10;
            locals.var_t1_dn13 = assign65270_body4_e100872_d_n13;
            let (assign65270_body5_e100893, assign65270_body5_e100893_d_n0, assign65270_body5_e100893_d_n2, assign65270_body5_e100893_d_n4, assign65270_body5_e100893_d_n5, assign65270_body5_e100893_d_n6, assign65270_body5_e100893_d_n7, assign65270_body5_e100893_d_n8, assign65270_body5_e100893_d_n9, assign65270_body5_e100893_d_n10, assign65270_body5_e100893_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65270_body5_e100888: f64 = (-locals.var_c_sb__blk1548);
        let assign65270_body5_e100890: f64 = (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547);
        let assign65270_body5_e100891: f64 = (assign65270_body5_e100890).exp();
        (assign65270_body5_e100891, (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn0) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn0))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn2) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn2))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn4) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn4))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn5) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn5))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn6) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn6))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn7) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn7))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn8) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn8))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn9) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn9))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn10) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn10))), (assign65270_body5_e100891 * (((-locals.var_c_sb__blk1548_dn13) * locals.var_dphi_sb__blk1547) + (assign65270_body5_e100888 * locals.var_dphi_sb__blk1547_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign65270_body5_e100893;
            locals.var_t0_dn0 = assign65270_body5_e100893_d_n0;
            locals.var_t0_dn2 = assign65270_body5_e100893_d_n2;
            locals.var_t0_dn4 = assign65270_body5_e100893_d_n4;
            locals.var_t0_dn5 = assign65270_body5_e100893_d_n5;
            locals.var_t0_dn6 = assign65270_body5_e100893_d_n6;
            locals.var_t0_dn7 = assign65270_body5_e100893_d_n7;
            locals.var_t0_dn8 = assign65270_body5_e100893_d_n8;
            locals.var_t0_dn9 = assign65270_body5_e100893_d_n9;
            locals.var_t0_dn10 = assign65270_body5_e100893_d_n10;
            locals.var_t0_dn13 = assign65270_body5_e100893_d_n13;
            let (assign65270_body6_e100912, assign65270_body6_e100912_d_n0, assign65270_body6_e100912_d_n2, assign65270_body6_e100912_d_n4, assign65270_body6_e100912_d_n5, assign65270_body6_e100912_d_n6, assign65270_body6_e100912_d_n7, assign65270_body6_e100912_d_n8, assign65270_body6_e100912_d_n9, assign65270_body6_e100912_d_n10, assign65270_body6_e100912_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65270_body6_e100910: f64 = (locals.var_t1 - locals.var_t0);
        (assign65270_body6_e100910, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign65270_body6_e100912;
            locals.var_t2_dn0 = assign65270_body6_e100912_d_n0;
            locals.var_t2_dn2 = assign65270_body6_e100912_d_n2;
            locals.var_t2_dn4 = assign65270_body6_e100912_d_n4;
            locals.var_t2_dn5 = assign65270_body6_e100912_d_n5;
            locals.var_t2_dn6 = assign65270_body6_e100912_d_n6;
            locals.var_t2_dn7 = assign65270_body6_e100912_d_n7;
            locals.var_t2_dn8 = assign65270_body6_e100912_d_n8;
            locals.var_t2_dn9 = assign65270_body6_e100912_d_n9;
            locals.var_t2_dn10 = assign65270_body6_e100912_d_n10;
            locals.var_t2_dn13 = assign65270_body6_e100912_d_n13;
            let (assign65270_body7_e100934, assign65270_body7_e100934_d_n0, assign65270_body7_e100934_d_n2, assign65270_body7_e100934_d_n4, assign65270_body7_e100934_d_n5, assign65270_body7_e100934_d_n6, assign65270_body7_e100934_d_n7, assign65270_body7_e100934_d_n8, assign65270_body7_e100934_d_n9, assign65270_body7_e100934_d_n10, assign65270_body7_e100934_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65270_body7_e100929: f64 = (1.0 + locals.var_t2);
        let assign65270_body7_e100930: f64 = (assign65270_body7_e100929).ln();
        let assign65270_body7_e100932: f64 = (assign65270_body7_e100930 / locals.var_c_sb__blk1548);
        (assign65270_body7_e100932, ((((locals.var_t2_dn0 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn0)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn2 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn2)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn4 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn4)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn5 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn5)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn6 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn6)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn7 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn7)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn8 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn8)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn9 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn9)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn10 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn10)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)), ((((locals.var_t2_dn13 / assign65270_body7_e100929) * locals.var_c_sb__blk1548) - (assign65270_body7_e100930 * locals.var_c_sb__blk1548_dn13)) / (locals.var_c_sb__blk1548 * locals.var_c_sb__blk1548)),)
    } else {
        (locals.var_phi_b__blk1551, locals.var_phi_b__blk1551_dn0, locals.var_phi_b__blk1551_dn2, locals.var_phi_b__blk1551_dn4, locals.var_phi_b__blk1551_dn5, locals.var_phi_b__blk1551_dn6, locals.var_phi_b__blk1551_dn7, locals.var_phi_b__blk1551_dn8, locals.var_phi_b__blk1551_dn9, locals.var_phi_b__blk1551_dn10, locals.var_phi_b__blk1551_dn13,)
    }
};
            locals.var_phi_b__blk1551 = assign65270_body7_e100934;
            locals.var_phi_b__blk1551_dn0 = assign65270_body7_e100934_d_n0;
            locals.var_phi_b__blk1551_dn2 = assign65270_body7_e100934_d_n2;
            locals.var_phi_b__blk1551_dn4 = assign65270_body7_e100934_d_n4;
            locals.var_phi_b__blk1551_dn5 = assign65270_body7_e100934_d_n5;
            locals.var_phi_b__blk1551_dn6 = assign65270_body7_e100934_d_n6;
            locals.var_phi_b__blk1551_dn7 = assign65270_body7_e100934_d_n7;
            locals.var_phi_b__blk1551_dn8 = assign65270_body7_e100934_d_n8;
            locals.var_phi_b__blk1551_dn9 = assign65270_body7_e100934_d_n9;
            locals.var_phi_b__blk1551_dn10 = assign65270_body7_e100934_d_n10;
            locals.var_phi_b__blk1551_dn13 = assign65270_body7_e100934_d_n13;
            let (assign65270_body8_e100955, assign65270_body8_e100955_d_n0, assign65270_body8_e100955_d_n2, assign65270_body8_e100955_d_n4, assign65270_body8_e100955_d_n5, assign65270_body8_e100955_d_n6, assign65270_body8_e100955_d_n7, assign65270_body8_e100955_d_n8, assign65270_body8_e100955_d_n9, assign65270_body8_e100955_d_n10, assign65270_body8_e100955_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65270_body8_e100952: f64 = (1.0 + locals.var_t2);
        let assign65270_body8_e100953: f64 = (locals.var_t1 / assign65270_body8_e100952);
        (assign65270_body8_e100953, (((locals.var_t1_dn0 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn0)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn2 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn2)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn4 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn4)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn5 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn5)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn6 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn6)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn7 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn7)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn8 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn8)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn9 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn9)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn10 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn10)) / (assign65270_body8_e100952 * assign65270_body8_e100952)), (((locals.var_t1_dn13 * assign65270_body8_e100952) - (locals.var_t1 * locals.var_t2_dn13)) / (assign65270_body8_e100952 * assign65270_body8_e100952)),)
    } else {
        (locals.var_phi_b_dpss__blk1552, locals.var_phi_b_dpss__blk1552_dn0, locals.var_phi_b_dpss__blk1552_dn2, locals.var_phi_b_dpss__blk1552_dn4, locals.var_phi_b_dpss__blk1552_dn5, locals.var_phi_b_dpss__blk1552_dn6, locals.var_phi_b_dpss__blk1552_dn7, locals.var_phi_b_dpss__blk1552_dn8, locals.var_phi_b_dpss__blk1552_dn9, locals.var_phi_b_dpss__blk1552_dn10, locals.var_phi_b_dpss__blk1552_dn13,)
    }
};
            locals.var_phi_b_dpss__blk1552 = assign65270_body8_e100955;
            locals.var_phi_b_dpss__blk1552_dn0 = assign65270_body8_e100955_d_n0;
            locals.var_phi_b_dpss__blk1552_dn2 = assign65270_body8_e100955_d_n2;
            locals.var_phi_b_dpss__blk1552_dn4 = assign65270_body8_e100955_d_n4;
            locals.var_phi_b_dpss__blk1552_dn5 = assign65270_body8_e100955_d_n5;
            locals.var_phi_b_dpss__blk1552_dn6 = assign65270_body8_e100955_d_n6;
            locals.var_phi_b_dpss__blk1552_dn7 = assign65270_body8_e100955_d_n7;
            locals.var_phi_b_dpss__blk1552_dn8 = assign65270_body8_e100955_d_n8;
            locals.var_phi_b_dpss__blk1552_dn9 = assign65270_body8_e100955_d_n9;
            locals.var_phi_b_dpss__blk1552_dn10 = assign65270_body8_e100955_d_n10;
            locals.var_phi_b_dpss__blk1552_dn13 = assign65270_body8_e100955_d_n13;
            let (assign65270_body9_e100975, assign65270_body9_e100975_d_n0, assign65270_body9_e100975_d_n2, assign65270_body9_e100975_d_n4, assign65270_body9_e100975_d_n5, assign65270_body9_e100975_d_n6, assign65270_body9_e100975_d_n7, assign65270_body9_e100975_d_n8, assign65270_body9_e100975_d_n9, assign65270_body9_e100975_d_n10, assign65270_body9_e100975_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65270_body9_e100973: f64 = (locals.var_phi_0 - locals.var_dphi_sb__blk1547);
        (assign65270_body9_e100973, (locals.var_phi_0_dn0 - locals.var_dphi_sb__blk1547_dn0), (locals.var_phi_0_dn2 - locals.var_dphi_sb__blk1547_dn2), (locals.var_phi_0_dn4 - locals.var_dphi_sb__blk1547_dn4), (locals.var_phi_0_dn5 - locals.var_dphi_sb__blk1547_dn5), (locals.var_phi_0_dn6 - locals.var_dphi_sb__blk1547_dn6), (locals.var_phi_0_dn7 - locals.var_dphi_sb__blk1547_dn7), (locals.var_phi_0_dn8 - locals.var_dphi_sb__blk1547_dn8), (locals.var_phi_0_dn9 - locals.var_dphi_sb__blk1547_dn9), (locals.var_phi_0_dn10 - locals.var_dphi_sb__blk1547_dn10), (locals.var_phi_0_dn13 - locals.var_dphi_sb__blk1547_dn13),)
    } else {
        (locals.var_phi_b__blk1551, locals.var_phi_b__blk1551_dn0, locals.var_phi_b__blk1551_dn2, locals.var_phi_b__blk1551_dn4, locals.var_phi_b__blk1551_dn5, locals.var_phi_b__blk1551_dn6, locals.var_phi_b__blk1551_dn7, locals.var_phi_b__blk1551_dn8, locals.var_phi_b__blk1551_dn9, locals.var_phi_b__blk1551_dn10, locals.var_phi_b__blk1551_dn13,)
    }
};
            locals.var_phi_b__blk1551 = assign65270_body9_e100975;
            locals.var_phi_b__blk1551_dn0 = assign65270_body9_e100975_d_n0;
            locals.var_phi_b__blk1551_dn2 = assign65270_body9_e100975_d_n2;
            locals.var_phi_b__blk1551_dn4 = assign65270_body9_e100975_d_n4;
            locals.var_phi_b__blk1551_dn5 = assign65270_body9_e100975_d_n5;
            locals.var_phi_b__blk1551_dn6 = assign65270_body9_e100975_d_n6;
            locals.var_phi_b__blk1551_dn7 = assign65270_body9_e100975_d_n7;
            locals.var_phi_b__blk1551_dn8 = assign65270_body9_e100975_d_n8;
            locals.var_phi_b__blk1551_dn9 = assign65270_body9_e100975_d_n9;
            locals.var_phi_b__blk1551_dn10 = assign65270_body9_e100975_d_n10;
            locals.var_phi_b__blk1551_dn13 = assign65270_body9_e100975_d_n13;
            let (assign65270_body10_e100993, assign65270_body10_e100993_d_n0, assign65270_body10_e100993_d_n2, assign65270_body10_e100993_d_n4, assign65270_body10_e100993_d_n5, assign65270_body10_e100993_d_n6, assign65270_body10_e100993_d_n7, assign65270_body10_e100993_d_n8, assign65270_body10_e100993_d_n9, assign65270_body10_e100993_d_n10, assign65270_body10_e100993_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1556 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss__blk1552, locals.var_phi_b_dpss__blk1552_dn0, locals.var_phi_b_dpss__blk1552_dn2, locals.var_phi_b_dpss__blk1552_dn4, locals.var_phi_b_dpss__blk1552_dn5, locals.var_phi_b_dpss__blk1552_dn6, locals.var_phi_b_dpss__blk1552_dn7, locals.var_phi_b_dpss__blk1552_dn8, locals.var_phi_b_dpss__blk1552_dn9, locals.var_phi_b_dpss__blk1552_dn10, locals.var_phi_b_dpss__blk1552_dn13,)
    }
};
            locals.var_phi_b_dpss__blk1552 = assign65270_body10_e100993;
            locals.var_phi_b_dpss__blk1552_dn0 = assign65270_body10_e100993_d_n0;
            locals.var_phi_b_dpss__blk1552_dn2 = assign65270_body10_e100993_d_n2;
            locals.var_phi_b_dpss__blk1552_dn4 = assign65270_body10_e100993_d_n4;
            locals.var_phi_b_dpss__blk1552_dn5 = assign65270_body10_e100993_d_n5;
            locals.var_phi_b_dpss__blk1552_dn6 = assign65270_body10_e100993_d_n6;
            locals.var_phi_b_dpss__blk1552_dn7 = assign65270_body10_e100993_d_n7;
            locals.var_phi_b_dpss__blk1552_dn8 = assign65270_body10_e100993_d_n8;
            locals.var_phi_b_dpss__blk1552_dn9 = assign65270_body10_e100993_d_n9;
            locals.var_phi_b_dpss__blk1552_dn10 = assign65270_body10_e100993_d_n10;
            locals.var_phi_b_dpss__blk1552_dn13 = assign65270_body10_e100993_d_n13;
            let (assign65270_body11_e101010, assign65270_body11_e101010_d_n0, assign65270_body11_e101010_d_n2, assign65270_body11_e101010_d_n4, assign65270_body11_e101010_d_n5, assign65270_body11_e101010_d_n6, assign65270_body11_e101010_d_n7, assign65270_body11_e101010_d_n8, assign65270_body11_e101010_d_n9, assign65270_body11_e101010_d_n10, assign65270_body11_e101010_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body11_e101008: f64 = (locals.var_beta * locals.var_phi_b__blk1551);
        (assign65270_body11_e101008, ((locals.var_beta_dn0 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b__blk1551) + (locals.var_beta * locals.var_phi_b__blk1551_dn13)),)
    } else {
        (locals.var_chib__blk1550, locals.var_chib__blk1550_dn0, locals.var_chib__blk1550_dn2, locals.var_chib__blk1550_dn4, locals.var_chib__blk1550_dn5, locals.var_chib__blk1550_dn6, locals.var_chib__blk1550_dn7, locals.var_chib__blk1550_dn8, locals.var_chib__blk1550_dn9, locals.var_chib__blk1550_dn10, locals.var_chib__blk1550_dn13,)
    }
};
            locals.var_chib__blk1550 = assign65270_body11_e101010;
            locals.var_chib__blk1550_dn0 = assign65270_body11_e101010_d_n0;
            locals.var_chib__blk1550_dn2 = assign65270_body11_e101010_d_n2;
            locals.var_chib__blk1550_dn4 = assign65270_body11_e101010_d_n4;
            locals.var_chib__blk1550_dn5 = assign65270_body11_e101010_d_n5;
            locals.var_chib__blk1550_dn6 = assign65270_body11_e101010_d_n6;
            locals.var_chib__blk1550_dn7 = assign65270_body11_e101010_d_n7;
            locals.var_chib__blk1550_dn8 = assign65270_body11_e101010_d_n8;
            locals.var_chib__blk1550_dn9 = assign65270_body11_e101010_d_n9;
            locals.var_chib__blk1550_dn10 = assign65270_body11_e101010_d_n10;
            locals.var_chib__blk1550_dn13 = assign65270_body11_e101010_d_n13;
            let assign65270_body12_e101012: f64 = (locals.var_chi).abs();
            let assign65270_body12_e101014: f64 = if assign65270_body12_e101012 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard1557 = assign65270_body12_e101014;
            let (assign65270_body13_e101038, assign65270_body13_e101038_d_n0, assign65270_body13_e101038_d_n2, assign65270_body13_e101038_d_n4, assign65270_body13_e101038_d_n5, assign65270_body13_e101038_d_n6, assign65270_body13_e101038_d_n7, assign65270_body13_e101038_d_n8, assign65270_body13_e101038_d_n9, assign65270_body13_e101038_d_n10, assign65270_body13_e101038_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 != 0.0)) {
        let assign65270_body13_e101032: f64 = (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552);
        let assign65270_body13_e101033: f64 = (1.0 - assign65270_body13_e101032);
        let assign65270_body13_e101035: f64 = (assign65270_body13_e101033 / 2.0);
        let assign65270_body13_e101036: f64 = (assign65270_body13_e101035).sqrt();
        (assign65270_body13_e101036, (((-((locals.var_phi_b_dpss__blk1552_dn0 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn0))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn2 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn2))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn4 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn4))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn5 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn5))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn6 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn6))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn7 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn7))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn8 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn8))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn9 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn9))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn10 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn10))) / 2.0) / (2.0 * assign65270_body13_e101036)), (((-((locals.var_phi_b_dpss__blk1552_dn13 * locals.var_phi_b_dpss__blk1552) + (locals.var_phi_b_dpss__blk1552 * locals.var_phi_b_dpss__blk1552_dn13))) / 2.0) / (2.0 * assign65270_body13_e101036)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign65270_body13_e101038;
            locals.var_t0_dn0 = assign65270_body13_e101038_d_n0;
            locals.var_t0_dn2 = assign65270_body13_e101038_d_n2;
            locals.var_t0_dn4 = assign65270_body13_e101038_d_n4;
            locals.var_t0_dn5 = assign65270_body13_e101038_d_n5;
            locals.var_t0_dn6 = assign65270_body13_e101038_d_n6;
            locals.var_t0_dn7 = assign65270_body13_e101038_d_n7;
            locals.var_t0_dn8 = assign65270_body13_e101038_d_n8;
            locals.var_t0_dn9 = assign65270_body13_e101038_d_n9;
            locals.var_t0_dn10 = assign65270_body13_e101038_d_n10;
            locals.var_t0_dn13 = assign65270_body13_e101038_d_n13;
            let (assign65270_body14_e101057, assign65270_body14_e101057_d_n0, assign65270_body14_e101057_d_n2, assign65270_body14_e101057_d_n4, assign65270_body14_e101057_d_n5, assign65270_body14_e101057_d_n6, assign65270_body14_e101057_d_n7, assign65270_body14_e101057_d_n8, assign65270_body14_e101057_d_n9, assign65270_body14_e101057_d_n10, assign65270_body14_e101057_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 != 0.0)) {
        let assign65270_body14_e101055: f64 = (locals.var_chi * locals.var_t0);
        (assign65270_body14_e101055, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn13 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign65270_body14_e101057;
            locals.var_fb_dn0 = assign65270_body14_e101057_d_n0;
            locals.var_fb_dn2 = assign65270_body14_e101057_d_n2;
            locals.var_fb_dn4 = assign65270_body14_e101057_d_n4;
            locals.var_fb_dn5 = assign65270_body14_e101057_d_n5;
            locals.var_fb_dn6 = assign65270_body14_e101057_d_n6;
            locals.var_fb_dn7 = assign65270_body14_e101057_d_n7;
            locals.var_fb_dn8 = assign65270_body14_e101057_d_n8;
            locals.var_fb_dn9 = assign65270_body14_e101057_d_n9;
            locals.var_fb_dn10 = assign65270_body14_e101057_d_n10;
            locals.var_fb_dn13 = assign65270_body14_e101057_d_n13;
            let (assign65270_body15_e101076, assign65270_body15_e101076_d_n0, assign65270_body15_e101076_d_n2, assign65270_body15_e101076_d_n4, assign65270_body15_e101076_d_n5, assign65270_body15_e101076_d_n6, assign65270_body15_e101076_d_n7, assign65270_body15_e101076_d_n8, assign65270_body15_e101076_d_n9, assign65270_body15_e101076_d_n10, assign65270_body15_e101076_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 != 0.0)) {
        let assign65270_body15_e101074: f64 = (locals.var_beta * locals.var_t0);
        (assign65270_body15_e101074, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn13 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb_dpss__blk1553, locals.var_fb_dpss__blk1553_dn0, locals.var_fb_dpss__blk1553_dn2, locals.var_fb_dpss__blk1553_dn4, locals.var_fb_dpss__blk1553_dn5, locals.var_fb_dpss__blk1553_dn6, locals.var_fb_dpss__blk1553_dn7, locals.var_fb_dpss__blk1553_dn8, locals.var_fb_dpss__blk1553_dn9, locals.var_fb_dpss__blk1553_dn10, locals.var_fb_dpss__blk1553_dn13,)
    }
};
            locals.var_fb_dpss__blk1553 = assign65270_body15_e101076;
            locals.var_fb_dpss__blk1553_dn0 = assign65270_body15_e101076_d_n0;
            locals.var_fb_dpss__blk1553_dn2 = assign65270_body15_e101076_d_n2;
            locals.var_fb_dpss__blk1553_dn4 = assign65270_body15_e101076_d_n4;
            locals.var_fb_dpss__blk1553_dn5 = assign65270_body15_e101076_d_n5;
            locals.var_fb_dpss__blk1553_dn6 = assign65270_body15_e101076_d_n6;
            locals.var_fb_dpss__blk1553_dn7 = assign65270_body15_e101076_d_n7;
            locals.var_fb_dpss__blk1553_dn8 = assign65270_body15_e101076_d_n8;
            locals.var_fb_dpss__blk1553_dn9 = assign65270_body15_e101076_d_n9;
            locals.var_fb_dpss__blk1553_dn10 = assign65270_body15_e101076_d_n10;
            locals.var_fb_dpss__blk1553_dn13 = assign65270_body15_e101076_d_n13;
            let assign65270_body16_e101079: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1558 = assign65270_body16_e101079;
            let (assign65270_body17_e101099, assign65270_body17_e101099_d_n0, assign65270_body17_e101099_d_n2, assign65270_body17_e101099_d_n4, assign65270_body17_e101099_d_n5, assign65270_body17_e101099_d_n6, assign65270_body17_e101099_d_n7, assign65270_body17_e101099_d_n8, assign65270_body17_e101099_d_n9, assign65270_body17_e101099_d_n10, assign65270_body17_e101099_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65270_body17_e101097: f64 = (-locals.var_fb);
        (assign65270_body17_e101097, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign65270_body17_e101099;
            locals.var_fb_dn0 = assign65270_body17_e101099_d_n0;
            locals.var_fb_dn2 = assign65270_body17_e101099_d_n2;
            locals.var_fb_dn4 = assign65270_body17_e101099_d_n4;
            locals.var_fb_dn5 = assign65270_body17_e101099_d_n5;
            locals.var_fb_dn6 = assign65270_body17_e101099_d_n6;
            locals.var_fb_dn7 = assign65270_body17_e101099_d_n7;
            locals.var_fb_dn8 = assign65270_body17_e101099_d_n8;
            locals.var_fb_dn9 = assign65270_body17_e101099_d_n9;
            locals.var_fb_dn10 = assign65270_body17_e101099_d_n10;
            locals.var_fb_dn13 = assign65270_body17_e101099_d_n13;
            let (assign65270_body18_e101119, assign65270_body18_e101119_d_n0, assign65270_body18_e101119_d_n2, assign65270_body18_e101119_d_n4, assign65270_body18_e101119_d_n5, assign65270_body18_e101119_d_n6, assign65270_body18_e101119_d_n7, assign65270_body18_e101119_d_n8, assign65270_body18_e101119_d_n9, assign65270_body18_e101119_d_n10, assign65270_body18_e101119_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65270_body18_e101117: f64 = (-locals.var_fb_dpss__blk1553);
        (assign65270_body18_e101117, (-locals.var_fb_dpss__blk1553_dn0), (-locals.var_fb_dpss__blk1553_dn2), (-locals.var_fb_dpss__blk1553_dn4), (-locals.var_fb_dpss__blk1553_dn5), (-locals.var_fb_dpss__blk1553_dn6), (-locals.var_fb_dpss__blk1553_dn7), (-locals.var_fb_dpss__blk1553_dn8), (-locals.var_fb_dpss__blk1553_dn9), (-locals.var_fb_dpss__blk1553_dn10), (-locals.var_fb_dpss__blk1553_dn13),)
    } else {
        (locals.var_fb_dpss__blk1553, locals.var_fb_dpss__blk1553_dn0, locals.var_fb_dpss__blk1553_dn2, locals.var_fb_dpss__blk1553_dn4, locals.var_fb_dpss__blk1553_dn5, locals.var_fb_dpss__blk1553_dn6, locals.var_fb_dpss__blk1553_dn7, locals.var_fb_dpss__blk1553_dn8, locals.var_fb_dpss__blk1553_dn9, locals.var_fb_dpss__blk1553_dn10, locals.var_fb_dpss__blk1553_dn13,)
    }
};
            locals.var_fb_dpss__blk1553 = assign65270_body18_e101119;
            locals.var_fb_dpss__blk1553_dn0 = assign65270_body18_e101119_d_n0;
            locals.var_fb_dpss__blk1553_dn2 = assign65270_body18_e101119_d_n2;
            locals.var_fb_dpss__blk1553_dn4 = assign65270_body18_e101119_d_n4;
            locals.var_fb_dpss__blk1553_dn5 = assign65270_body18_e101119_d_n5;
            locals.var_fb_dpss__blk1553_dn6 = assign65270_body18_e101119_d_n6;
            locals.var_fb_dpss__blk1553_dn7 = assign65270_body18_e101119_d_n7;
            locals.var_fb_dpss__blk1553_dn8 = assign65270_body18_e101119_d_n8;
            locals.var_fb_dpss__blk1553_dn9 = assign65270_body18_e101119_d_n9;
            locals.var_fb_dpss__blk1553_dn10 = assign65270_body18_e101119_d_n10;
            locals.var_fb_dpss__blk1553_dn13 = assign65270_body18_e101119_d_n13;
            let assign65270_body19_e101121: f64 = (locals.var_chi).abs();
            let assign65270_body19_e101123: f64 = if assign65270_body19_e101121 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard1559 = assign65270_body19_e101123;
            let (assign65270_body20_e101165, assign65270_body20_e101165_d_n0, assign65270_body20_e101165_d_n2, assign65270_body20_e101165_d_n4, assign65270_body20_e101165_d_n5, assign65270_body20_e101165_d_n6, assign65270_body20_e101165_d_n7, assign65270_body20_e101165_d_n8, assign65270_body20_e101165_d_n9, assign65270_body20_e101165_d_n10, assign65270_body20_e101165_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body20_e101143: f64 = (locals.var_chi * locals.var_chi);
        let assign65270_body20_e101145: f64 = (assign65270_body20_e101143 / 2.0);
        let assign65270_body20_e101149: f64 = (locals.var_chi / 3.0);
        let assign65270_body20_e101153: f64 = (locals.var_chi / 4.0);
        let assign65270_body20_e101157: f64 = (locals.var_chi / 5.0);
        let assign65270_body20_e101158: f64 = (1.0 - assign65270_body20_e101157);
        let assign65270_body20_e101159: f64 = (assign65270_body20_e101153 * assign65270_body20_e101158);
        let assign65270_body20_e101160: f64 = (1.0 - assign65270_body20_e101159);
        let assign65270_body20_e101161: f64 = (assign65270_body20_e101149 * assign65270_body20_e101160);
        let assign65270_body20_e101162: f64 = (1.0 - assign65270_body20_e101161);
        let assign65270_body20_e101163: f64 = (assign65270_body20_e101145 * assign65270_body20_e101162);
        (assign65270_body20_e101163, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn0 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn0 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn2 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn2 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn4 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn4 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn5 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn5 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn6 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn6 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn7 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn7 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn8 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn8 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn9 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn9 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn10 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn10 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign65270_body20_e101162) + (assign65270_body20_e101145 * (-(((locals.var_chi_dn13 / 3.0) * assign65270_body20_e101160) + (assign65270_body20_e101149 * (-(((locals.var_chi_dn13 / 4.0) * assign65270_body20_e101158) + (assign65270_body20_e101153 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign65270_body20_e101165;
            locals.var_t0_dn0 = assign65270_body20_e101165_d_n0;
            locals.var_t0_dn2 = assign65270_body20_e101165_d_n2;
            locals.var_t0_dn4 = assign65270_body20_e101165_d_n4;
            locals.var_t0_dn5 = assign65270_body20_e101165_d_n5;
            locals.var_t0_dn6 = assign65270_body20_e101165_d_n6;
            locals.var_t0_dn7 = assign65270_body20_e101165_d_n7;
            locals.var_t0_dn8 = assign65270_body20_e101165_d_n8;
            locals.var_t0_dn9 = assign65270_body20_e101165_d_n9;
            locals.var_t0_dn10 = assign65270_body20_e101165_d_n10;
            locals.var_t0_dn13 = assign65270_body20_e101165_d_n13;
            let (assign65270_body21_e101203, assign65270_body21_e101203_d_n0, assign65270_body21_e101203_d_n2, assign65270_body21_e101203_d_n4, assign65270_body21_e101203_d_n5, assign65270_body21_e101203_d_n6, assign65270_body21_e101203_d_n7, assign65270_body21_e101203_d_n8, assign65270_body21_e101203_d_n9, assign65270_body21_e101203_d_n10, assign65270_body21_e101203_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body21_e101187: f64 = (locals.var_chi / 2.0);
        let assign65270_body21_e101191: f64 = (locals.var_chi / 3.0);
        let assign65270_body21_e101195: f64 = (locals.var_chi / 4.0);
        let assign65270_body21_e101196: f64 = (1.0 - assign65270_body21_e101195);
        let assign65270_body21_e101197: f64 = (assign65270_body21_e101191 * assign65270_body21_e101196);
        let assign65270_body21_e101198: f64 = (1.0 - assign65270_body21_e101197);
        let assign65270_body21_e101199: f64 = (assign65270_body21_e101187 * assign65270_body21_e101198);
        let assign65270_body21_e101200: f64 = (1.0 - assign65270_body21_e101199);
        let assign65270_body21_e101201: f64 = (locals.var_chi * assign65270_body21_e101200);
        (assign65270_body21_e101201, ((locals.var_chi_dn0 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn0 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn2 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn4 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn5 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn6 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn7 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn8 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn9 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn10 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign65270_body21_e101200) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign65270_body21_e101198) + (assign65270_body21_e101187 * (-(((locals.var_chi_dn13 / 3.0) * assign65270_body21_e101196) + (assign65270_body21_e101191 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign65270_body21_e101203;
            locals.var_t1_dn0 = assign65270_body21_e101203_d_n0;
            locals.var_t1_dn2 = assign65270_body21_e101203_d_n2;
            locals.var_t1_dn4 = assign65270_body21_e101203_d_n4;
            locals.var_t1_dn5 = assign65270_body21_e101203_d_n5;
            locals.var_t1_dn6 = assign65270_body21_e101203_d_n6;
            locals.var_t1_dn7 = assign65270_body21_e101203_d_n7;
            locals.var_t1_dn8 = assign65270_body21_e101203_d_n8;
            locals.var_t1_dn9 = assign65270_body21_e101203_d_n9;
            locals.var_t1_dn10 = assign65270_body21_e101203_d_n10;
            locals.var_t1_dn13 = assign65270_body21_e101203_d_n13;
            let (assign65270_body22_e101245, assign65270_body22_e101245_d_n0, assign65270_body22_e101245_d_n2, assign65270_body22_e101245_d_n4, assign65270_body22_e101245_d_n5, assign65270_body22_e101245_d_n6, assign65270_body22_e101245_d_n7, assign65270_body22_e101245_d_n8, assign65270_body22_e101245_d_n9, assign65270_body22_e101245_d_n10, assign65270_body22_e101245_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body22_e101223: f64 = (locals.var_chib__blk1550 * locals.var_chib__blk1550);
        let assign65270_body22_e101225: f64 = (assign65270_body22_e101223 / 2.0);
        let assign65270_body22_e101229: f64 = (locals.var_chib__blk1550 / 3.0);
        let assign65270_body22_e101233: f64 = (locals.var_chib__blk1550 / 4.0);
        let assign65270_body22_e101237: f64 = (locals.var_chib__blk1550 / 5.0);
        let assign65270_body22_e101238: f64 = (1.0 - assign65270_body22_e101237);
        let assign65270_body22_e101239: f64 = (assign65270_body22_e101233 * assign65270_body22_e101238);
        let assign65270_body22_e101240: f64 = (1.0 - assign65270_body22_e101239);
        let assign65270_body22_e101241: f64 = (assign65270_body22_e101229 * assign65270_body22_e101240);
        let assign65270_body22_e101242: f64 = (1.0 - assign65270_body22_e101241);
        let assign65270_body22_e101243: f64 = (assign65270_body22_e101225 * assign65270_body22_e101242);
        (assign65270_body22_e101243, (((((locals.var_chib__blk1550_dn0 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn0)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn0 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn0 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn0 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn2 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn2)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn2 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn2 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn2 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn4 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn4)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn4 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn4 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn4 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn5 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn5)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn5 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn5 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn5 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn6 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn6)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn6 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn6 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn6 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn7 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn7)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn7 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn7 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn7 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn8 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn8)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn8 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn8 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn8 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn9 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn9)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn9 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn9 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn9 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn10 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn10)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn10 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn10 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn10 / 5.0)))))))))), (((((locals.var_chib__blk1550_dn13 * locals.var_chib__blk1550) + (locals.var_chib__blk1550 * locals.var_chib__blk1550_dn13)) / 2.0) * assign65270_body22_e101242) + (assign65270_body22_e101225 * (-(((locals.var_chib__blk1550_dn13 / 3.0) * assign65270_body22_e101240) + (assign65270_body22_e101229 * (-(((locals.var_chib__blk1550_dn13 / 4.0) * assign65270_body22_e101238) + (assign65270_body22_e101233 * (-(locals.var_chib__blk1550_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign65270_body22_e101245;
            locals.var_t2_dn0 = assign65270_body22_e101245_d_n0;
            locals.var_t2_dn2 = assign65270_body22_e101245_d_n2;
            locals.var_t2_dn4 = assign65270_body22_e101245_d_n4;
            locals.var_t2_dn5 = assign65270_body22_e101245_d_n5;
            locals.var_t2_dn6 = assign65270_body22_e101245_d_n6;
            locals.var_t2_dn7 = assign65270_body22_e101245_d_n7;
            locals.var_t2_dn8 = assign65270_body22_e101245_d_n8;
            locals.var_t2_dn9 = assign65270_body22_e101245_d_n9;
            locals.var_t2_dn10 = assign65270_body22_e101245_d_n10;
            locals.var_t2_dn13 = assign65270_body22_e101245_d_n13;
            let (assign65270_body23_e101283, assign65270_body23_e101283_d_n0, assign65270_body23_e101283_d_n2, assign65270_body23_e101283_d_n4, assign65270_body23_e101283_d_n5, assign65270_body23_e101283_d_n6, assign65270_body23_e101283_d_n7, assign65270_body23_e101283_d_n8, assign65270_body23_e101283_d_n9, assign65270_body23_e101283_d_n10, assign65270_body23_e101283_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body23_e101267: f64 = (locals.var_chib__blk1550 / 2.0);
        let assign65270_body23_e101271: f64 = (locals.var_chib__blk1550 / 3.0);
        let assign65270_body23_e101275: f64 = (locals.var_chib__blk1550 / 4.0);
        let assign65270_body23_e101276: f64 = (1.0 - assign65270_body23_e101275);
        let assign65270_body23_e101277: f64 = (assign65270_body23_e101271 * assign65270_body23_e101276);
        let assign65270_body23_e101278: f64 = (1.0 - assign65270_body23_e101277);
        let assign65270_body23_e101279: f64 = (assign65270_body23_e101267 * assign65270_body23_e101278);
        let assign65270_body23_e101280: f64 = (1.0 - assign65270_body23_e101279);
        let assign65270_body23_e101281: f64 = (locals.var_chib__blk1550 * assign65270_body23_e101280);
        (assign65270_body23_e101281, ((locals.var_chib__blk1550_dn0 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn0 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn0 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn0 / 4.0)))))))))), ((locals.var_chib__blk1550_dn2 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn2 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn2 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn2 / 4.0)))))))))), ((locals.var_chib__blk1550_dn4 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn4 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn4 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn4 / 4.0)))))))))), ((locals.var_chib__blk1550_dn5 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn5 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn5 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn5 / 4.0)))))))))), ((locals.var_chib__blk1550_dn6 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn6 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn6 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn6 / 4.0)))))))))), ((locals.var_chib__blk1550_dn7 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn7 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn7 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn7 / 4.0)))))))))), ((locals.var_chib__blk1550_dn8 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn8 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn8 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn8 / 4.0)))))))))), ((locals.var_chib__blk1550_dn9 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn9 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn9 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn9 / 4.0)))))))))), ((locals.var_chib__blk1550_dn10 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn10 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn10 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn10 / 4.0)))))))))), ((locals.var_chib__blk1550_dn13 * assign65270_body23_e101280) + (locals.var_chib__blk1550 * (-(((locals.var_chib__blk1550_dn13 / 2.0) * assign65270_body23_e101278) + (assign65270_body23_e101267 * (-(((locals.var_chib__blk1550_dn13 / 3.0) * assign65270_body23_e101276) + (assign65270_body23_e101271 * (-(locals.var_chib__blk1550_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign65270_body23_e101283;
            locals.var_t3_dn0 = assign65270_body23_e101283_d_n0;
            locals.var_t3_dn2 = assign65270_body23_e101283_d_n2;
            locals.var_t3_dn4 = assign65270_body23_e101283_d_n4;
            locals.var_t3_dn5 = assign65270_body23_e101283_d_n5;
            locals.var_t3_dn6 = assign65270_body23_e101283_d_n6;
            locals.var_t3_dn7 = assign65270_body23_e101283_d_n7;
            locals.var_t3_dn8 = assign65270_body23_e101283_d_n8;
            locals.var_t3_dn9 = assign65270_body23_e101283_d_n9;
            locals.var_t3_dn10 = assign65270_body23_e101283_d_n10;
            locals.var_t3_dn13 = assign65270_body23_e101283_d_n13;
            let (assign65270_body24_e101306, assign65270_body24_e101306_d_n0, assign65270_body24_e101306_d_n2, assign65270_body24_e101306_d_n4, assign65270_body24_e101306_d_n5, assign65270_body24_e101306_d_n6, assign65270_body24_e101306_d_n7, assign65270_body24_e101306_d_n8, assign65270_body24_e101306_d_n9, assign65270_body24_e101306_d_n10, assign65270_body24_e101306_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body24_e101303: f64 = (locals.var_t0 - locals.var_t2);
        let assign65270_body24_e101304: f64 = (assign65270_body24_e101303).sqrt();
        (assign65270_body24_e101304, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn4 - locals.var_t2_dn4) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn5 - locals.var_t2_dn5) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn8 - locals.var_t2_dn8) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn9 - locals.var_t2_dn9) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign65270_body24_e101304)), ((locals.var_t0_dn13 - locals.var_t2_dn13) / (2.0 * assign65270_body24_e101304)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign65270_body24_e101306;
            locals.var_fb_dn0 = assign65270_body24_e101306_d_n0;
            locals.var_fb_dn2 = assign65270_body24_e101306_d_n2;
            locals.var_fb_dn4 = assign65270_body24_e101306_d_n4;
            locals.var_fb_dn5 = assign65270_body24_e101306_d_n5;
            locals.var_fb_dn6 = assign65270_body24_e101306_d_n6;
            locals.var_fb_dn7 = assign65270_body24_e101306_d_n7;
            locals.var_fb_dn8 = assign65270_body24_e101306_d_n8;
            locals.var_fb_dn9 = assign65270_body24_e101306_d_n9;
            locals.var_fb_dn10 = assign65270_body24_e101306_d_n10;
            locals.var_fb_dn13 = assign65270_body24_e101306_d_n13;
            let (assign65270_body25_e101336, assign65270_body25_e101336_d_n0, assign65270_body25_e101336_d_n2, assign65270_body25_e101336_d_n4, assign65270_body25_e101336_d_n5, assign65270_body25_e101336_d_n6, assign65270_body25_e101336_d_n7, assign65270_body25_e101336_d_n8, assign65270_body25_e101336_d_n9, assign65270_body25_e101336_d_n10, assign65270_body25_e101336_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65270_body25_e101326: f64 = (locals.var_beta * 0.5);
        let assign65270_body25_e101330: f64 = (locals.var_phi_b_dpss__blk1552 * locals.var_t3);
        let assign65270_body25_e101331: f64 = (locals.var_t1 - assign65270_body25_e101330);
        let assign65270_body25_e101332: f64 = (assign65270_body25_e101326 * assign65270_body25_e101331);
        let assign65270_body25_e101334: f64 = (assign65270_body25_e101332 / locals.var_fb);
        (assign65270_body25_e101334, ((((((locals.var_beta_dn0 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss__blk1552_dn0 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn0))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss__blk1552_dn2 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn2))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss__blk1552_dn4 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn4))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss__blk1552_dn5 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn5))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss__blk1552_dn6 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn6))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss__blk1552_dn7 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn7))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss__blk1552_dn8 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn8))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss__blk1552_dn9 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn9))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss__blk1552_dn10 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn10))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign65270_body25_e101331) + (assign65270_body25_e101326 * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss__blk1552_dn13 * locals.var_t3) + (locals.var_phi_b_dpss__blk1552 * locals.var_t3_dn13))))) * locals.var_fb) - (assign65270_body25_e101332 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss__blk1553, locals.var_fb_dpss__blk1553_dn0, locals.var_fb_dpss__blk1553_dn2, locals.var_fb_dpss__blk1553_dn4, locals.var_fb_dpss__blk1553_dn5, locals.var_fb_dpss__blk1553_dn6, locals.var_fb_dpss__blk1553_dn7, locals.var_fb_dpss__blk1553_dn8, locals.var_fb_dpss__blk1553_dn9, locals.var_fb_dpss__blk1553_dn10, locals.var_fb_dpss__blk1553_dn13,)
    }
};
            locals.var_fb_dpss__blk1553 = assign65270_body25_e101336;
            locals.var_fb_dpss__blk1553_dn0 = assign65270_body25_e101336_d_n0;
            locals.var_fb_dpss__blk1553_dn2 = assign65270_body25_e101336_d_n2;
            locals.var_fb_dpss__blk1553_dn4 = assign65270_body25_e101336_d_n4;
            locals.var_fb_dpss__blk1553_dn5 = assign65270_body25_e101336_d_n5;
            locals.var_fb_dpss__blk1553_dn6 = assign65270_body25_e101336_d_n6;
            locals.var_fb_dpss__blk1553_dn7 = assign65270_body25_e101336_d_n7;
            locals.var_fb_dpss__blk1553_dn8 = assign65270_body25_e101336_d_n8;
            locals.var_fb_dpss__blk1553_dn9 = assign65270_body25_e101336_d_n9;
            locals.var_fb_dpss__blk1553_dn10 = assign65270_body25_e101336_d_n10;
            locals.var_fb_dpss__blk1553_dn13 = assign65270_body25_e101336_d_n13;
            let (assign65270_body26_e101359, assign65270_body26_e101359_d_n0, assign65270_body26_e101359_d_n2, assign65270_body26_e101359_d_n4, assign65270_body26_e101359_d_n5, assign65270_body26_e101359_d_n6, assign65270_body26_e101359_d_n7, assign65270_body26_e101359_d_n8, assign65270_body26_e101359_d_n9, assign65270_body26_e101359_d_n10, assign65270_body26_e101359_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 == 0.0)) {
        let assign65270_body26_e101356: f64 = (-locals.var_chi);
        let assign65270_body26_e101357: f64 = (assign65270_body26_e101356).exp();
        (assign65270_body26_e101357, (assign65270_body26_e101357 * (-locals.var_chi_dn0)), (assign65270_body26_e101357 * (-locals.var_chi_dn2)), (assign65270_body26_e101357 * (-locals.var_chi_dn4)), (assign65270_body26_e101357 * (-locals.var_chi_dn5)), (assign65270_body26_e101357 * (-locals.var_chi_dn6)), (assign65270_body26_e101357 * (-locals.var_chi_dn7)), (assign65270_body26_e101357 * (-locals.var_chi_dn8)), (assign65270_body26_e101357 * (-locals.var_chi_dn9)), (assign65270_body26_e101357 * (-locals.var_chi_dn10)), (assign65270_body26_e101357 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign65270_body26_e101359;
            locals.var_t0_dn0 = assign65270_body26_e101359_d_n0;
            locals.var_t0_dn2 = assign65270_body26_e101359_d_n2;
            locals.var_t0_dn4 = assign65270_body26_e101359_d_n4;
            locals.var_t0_dn5 = assign65270_body26_e101359_d_n5;
            locals.var_t0_dn6 = assign65270_body26_e101359_d_n6;
            locals.var_t0_dn7 = assign65270_body26_e101359_d_n7;
            locals.var_t0_dn8 = assign65270_body26_e101359_d_n8;
            locals.var_t0_dn9 = assign65270_body26_e101359_d_n9;
            locals.var_t0_dn10 = assign65270_body26_e101359_d_n10;
            locals.var_t0_dn13 = assign65270_body26_e101359_d_n13;
            let (assign65270_body27_e101382, assign65270_body27_e101382_d_n0, assign65270_body27_e101382_d_n2, assign65270_body27_e101382_d_n4, assign65270_body27_e101382_d_n5, assign65270_body27_e101382_d_n6, assign65270_body27_e101382_d_n7, assign65270_body27_e101382_d_n8, assign65270_body27_e101382_d_n9, assign65270_body27_e101382_d_n10, assign65270_body27_e101382_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 == 0.0)) {
        let assign65270_body27_e101379: f64 = (-locals.var_chib__blk1550);
        let assign65270_body27_e101380: f64 = (assign65270_body27_e101379).exp();
        (assign65270_body27_e101380, (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn0)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn2)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn4)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn5)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn6)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn7)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn8)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn9)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn10)), (assign65270_body27_e101380 * (-locals.var_chib__blk1550_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign65270_body27_e101382;
            locals.var_t1_dn0 = assign65270_body27_e101382_d_n0;
            locals.var_t1_dn2 = assign65270_body27_e101382_d_n2;
            locals.var_t1_dn4 = assign65270_body27_e101382_d_n4;
            locals.var_t1_dn5 = assign65270_body27_e101382_d_n5;
            locals.var_t1_dn6 = assign65270_body27_e101382_d_n6;
            locals.var_t1_dn7 = assign65270_body27_e101382_d_n7;
            locals.var_t1_dn8 = assign65270_body27_e101382_d_n8;
            locals.var_t1_dn9 = assign65270_body27_e101382_d_n9;
            locals.var_t1_dn10 = assign65270_body27_e101382_d_n10;
            locals.var_t1_dn13 = assign65270_body27_e101382_d_n13;
            let (assign65270_body28_e101410, assign65270_body28_e101410_d_n0, assign65270_body28_e101410_d_n2, assign65270_body28_e101410_d_n4, assign65270_body28_e101410_d_n5, assign65270_body28_e101410_d_n6, assign65270_body28_e101410_d_n7, assign65270_body28_e101410_d_n8, assign65270_body28_e101410_d_n9, assign65270_body28_e101410_d_n10, assign65270_body28_e101410_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 == 0.0)) {
        let assign65270_body28_e101403: f64 = (locals.var_chi - locals.var_chib__blk1550);
        let assign65270_body28_e101406: f64 = (locals.var_t0 - locals.var_t1);
        let assign65270_body28_e101407: f64 = (assign65270_body28_e101403 + assign65270_body28_e101406);
        let assign65270_body28_e101408: f64 = (assign65270_body28_e101407).sqrt();
        (assign65270_body28_e101408, (((locals.var_chi_dn0 - locals.var_chib__blk1550_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn2 - locals.var_chib__blk1550_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn4 - locals.var_chib__blk1550_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn5 - locals.var_chib__blk1550_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn6 - locals.var_chib__blk1550_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn7 - locals.var_chib__blk1550_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn8 - locals.var_chib__blk1550_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn9 - locals.var_chib__blk1550_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn10 - locals.var_chib__blk1550_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign65270_body28_e101408)), (((locals.var_chi_dn13 - locals.var_chib__blk1550_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)) / (2.0 * assign65270_body28_e101408)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign65270_body28_e101410;
            locals.var_fb_dn0 = assign65270_body28_e101410_d_n0;
            locals.var_fb_dn2 = assign65270_body28_e101410_d_n2;
            locals.var_fb_dn4 = assign65270_body28_e101410_d_n4;
            locals.var_fb_dn5 = assign65270_body28_e101410_d_n5;
            locals.var_fb_dn6 = assign65270_body28_e101410_d_n6;
            locals.var_fb_dn7 = assign65270_body28_e101410_d_n7;
            locals.var_fb_dn8 = assign65270_body28_e101410_d_n8;
            locals.var_fb_dn9 = assign65270_body28_e101410_d_n9;
            locals.var_fb_dn10 = assign65270_body28_e101410_d_n10;
            locals.var_fb_dn13 = assign65270_body28_e101410_d_n13;
            let (assign65270_body29_e101445, assign65270_body29_e101445_d_n0, assign65270_body29_e101445_d_n2, assign65270_body29_e101445_d_n4, assign65270_body29_e101445_d_n5, assign65270_body29_e101445_d_n6, assign65270_body29_e101445_d_n7, assign65270_body29_e101445_d_n8, assign65270_body29_e101445_d_n9, assign65270_body29_e101445_d_n10, assign65270_body29_e101445_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1557 == 0.0)) && (locals.var_guard1559 == 0.0)) {
        let assign65270_body29_e101431: f64 = (locals.var_beta * 0.5);
        let assign65270_body29_e101434: f64 = (1.0 - locals.var_t0);
        let assign65270_body29_e101438: f64 = (1.0 - locals.var_t1);
        let assign65270_body29_e101439: f64 = (locals.var_phi_b_dpss__blk1552 * assign65270_body29_e101438);
        let assign65270_body29_e101440: f64 = (assign65270_body29_e101434 - assign65270_body29_e101439);
        let assign65270_body29_e101441: f64 = (assign65270_body29_e101431 * assign65270_body29_e101440);
        let assign65270_body29_e101443: f64 = (assign65270_body29_e101441 / locals.var_fb);
        (assign65270_body29_e101443, ((((((locals.var_beta_dn0 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss__blk1552_dn0 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss__blk1552_dn2 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss__blk1552_dn4 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss__blk1552_dn5 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss__blk1552_dn6 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss__blk1552_dn7 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss__blk1552_dn8 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss__blk1552_dn9 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss__blk1552_dn10 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign65270_body29_e101440) + (assign65270_body29_e101431 * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss__blk1552_dn13 * assign65270_body29_e101438) + (locals.var_phi_b_dpss__blk1552 * (-locals.var_t1_dn13)))))) * locals.var_fb) - (assign65270_body29_e101441 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss__blk1553, locals.var_fb_dpss__blk1553_dn0, locals.var_fb_dpss__blk1553_dn2, locals.var_fb_dpss__blk1553_dn4, locals.var_fb_dpss__blk1553_dn5, locals.var_fb_dpss__blk1553_dn6, locals.var_fb_dpss__blk1553_dn7, locals.var_fb_dpss__blk1553_dn8, locals.var_fb_dpss__blk1553_dn9, locals.var_fb_dpss__blk1553_dn10, locals.var_fb_dpss__blk1553_dn13,)
    }
};
            locals.var_fb_dpss__blk1553 = assign65270_body29_e101445;
            locals.var_fb_dpss__blk1553_dn0 = assign65270_body29_e101445_d_n0;
            locals.var_fb_dpss__blk1553_dn2 = assign65270_body29_e101445_d_n2;
            locals.var_fb_dpss__blk1553_dn4 = assign65270_body29_e101445_d_n4;
            locals.var_fb_dpss__blk1553_dn5 = assign65270_body29_e101445_d_n5;
            locals.var_fb_dpss__blk1553_dn6 = assign65270_body29_e101445_d_n6;
            locals.var_fb_dpss__blk1553_dn7 = assign65270_body29_e101445_d_n7;
            locals.var_fb_dpss__blk1553_dn8 = assign65270_body29_e101445_d_n8;
            locals.var_fb_dpss__blk1553_dn9 = assign65270_body29_e101445_d_n9;
            locals.var_fb_dpss__blk1553_dn10 = assign65270_body29_e101445_d_n10;
            locals.var_fb_dpss__blk1553_dn13 = assign65270_body29_e101445_d_n13;
            let assign65270_body30_e101452: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard1560 = assign65270_body30_e101452;
            let (assign65270_body31_e101470,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1560 != 0.0)) {
        let assign65270_body31_e101468: f64 = (-1.0);
        (assign65270_body31_e101468,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign65270_body31_e101470;
            let assign65270_body32_e101473: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1561 = assign65270_body32_e101473;
            let (assign65270_body33_e101491, assign65270_body33_e101491_d_n0, assign65270_body33_e101491_d_n2, assign65270_body33_e101491_d_n4, assign65270_body33_e101491_d_n5, assign65270_body33_e101491_d_n6, assign65270_body33_e101491_d_n7, assign65270_body33_e101491_d_n8, assign65270_body33_e101491_d_n9, assign65270_body33_e101491_d_n10, assign65270_body33_e101491_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65270_body33_e101489: f64 = (-locals.var_fb);
        (assign65270_body33_e101489, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign65270_body33_e101491;
            locals.var_fs02_dn0 = assign65270_body33_e101491_d_n0;
            locals.var_fs02_dn2 = assign65270_body33_e101491_d_n2;
            locals.var_fs02_dn4 = assign65270_body33_e101491_d_n4;
            locals.var_fs02_dn5 = assign65270_body33_e101491_d_n5;
            locals.var_fs02_dn6 = assign65270_body33_e101491_d_n6;
            locals.var_fs02_dn7 = assign65270_body33_e101491_d_n7;
            locals.var_fs02_dn8 = assign65270_body33_e101491_d_n8;
            locals.var_fs02_dn9 = assign65270_body33_e101491_d_n9;
            locals.var_fs02_dn10 = assign65270_body33_e101491_d_n10;
            locals.var_fs02_dn13 = assign65270_body33_e101491_d_n13;
            let (assign65270_body34_e101509, assign65270_body34_e101509_d_n0, assign65270_body34_e101509_d_n2, assign65270_body34_e101509_d_n4, assign65270_body34_e101509_d_n5, assign65270_body34_e101509_d_n6, assign65270_body34_e101509_d_n7, assign65270_body34_e101509_d_n8, assign65270_body34_e101509_d_n9, assign65270_body34_e101509_d_n10, assign65270_body34_e101509_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65270_body34_e101507: f64 = (-locals.var_fb_dpss__blk1553);
        (assign65270_body34_e101507, (-locals.var_fb_dpss__blk1553_dn0), (-locals.var_fb_dpss__blk1553_dn2), (-locals.var_fb_dpss__blk1553_dn4), (-locals.var_fb_dpss__blk1553_dn5), (-locals.var_fb_dpss__blk1553_dn6), (-locals.var_fb_dpss__blk1553_dn7), (-locals.var_fb_dpss__blk1553_dn8), (-locals.var_fb_dpss__blk1553_dn9), (-locals.var_fb_dpss__blk1553_dn10), (-locals.var_fb_dpss__blk1553_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign65270_body34_e101509;
            locals.var_fs02_dps0_dn0 = assign65270_body34_e101509_d_n0;
            locals.var_fs02_dps0_dn2 = assign65270_body34_e101509_d_n2;
            locals.var_fs02_dps0_dn4 = assign65270_body34_e101509_d_n4;
            locals.var_fs02_dps0_dn5 = assign65270_body34_e101509_d_n5;
            locals.var_fs02_dps0_dn6 = assign65270_body34_e101509_d_n6;
            locals.var_fs02_dps0_dn7 = assign65270_body34_e101509_d_n7;
            locals.var_fs02_dps0_dn8 = assign65270_body34_e101509_d_n8;
            locals.var_fs02_dps0_dn9 = assign65270_body34_e101509_d_n9;
            locals.var_fs02_dps0_dn10 = assign65270_body34_e101509_d_n10;
            locals.var_fs02_dps0_dn13 = assign65270_body34_e101509_d_n13;
            let assign65270_body35_e101512: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard1562 = assign65270_body35_e101512;
            let (assign65270_body36_e101532, assign65270_body36_e101532_d_n0, assign65270_body36_e101532_d_n2, assign65270_body36_e101532_d_n4, assign65270_body36_e101532_d_n5, assign65270_body36_e101532_d_n6, assign65270_body36_e101532_d_n7, assign65270_body36_e101532_d_n8, assign65270_body36_e101532_d_n9, assign65270_body36_e101532_d_n10, assign65270_body36_e101532_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign65270_body36_e101532;
            locals.var_fs02_dn0 = assign65270_body36_e101532_d_n0;
            locals.var_fs02_dn2 = assign65270_body36_e101532_d_n2;
            locals.var_fs02_dn4 = assign65270_body36_e101532_d_n4;
            locals.var_fs02_dn5 = assign65270_body36_e101532_d_n5;
            locals.var_fs02_dn6 = assign65270_body36_e101532_d_n6;
            locals.var_fs02_dn7 = assign65270_body36_e101532_d_n7;
            locals.var_fs02_dn8 = assign65270_body36_e101532_d_n8;
            locals.var_fs02_dn9 = assign65270_body36_e101532_d_n9;
            locals.var_fs02_dn10 = assign65270_body36_e101532_d_n10;
            locals.var_fs02_dn13 = assign65270_body36_e101532_d_n13;
            let (assign65270_body37_e101552, assign65270_body37_e101552_d_n0, assign65270_body37_e101552_d_n2, assign65270_body37_e101552_d_n4, assign65270_body37_e101552_d_n5, assign65270_body37_e101552_d_n6, assign65270_body37_e101552_d_n7, assign65270_body37_e101552_d_n8, assign65270_body37_e101552_d_n9, assign65270_body37_e101552_d_n10, assign65270_body37_e101552_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 != 0.0)) {
        (locals.var_fb_dpss__blk1553, locals.var_fb_dpss__blk1553_dn0, locals.var_fb_dpss__blk1553_dn2, locals.var_fb_dpss__blk1553_dn4, locals.var_fb_dpss__blk1553_dn5, locals.var_fb_dpss__blk1553_dn6, locals.var_fb_dpss__blk1553_dn7, locals.var_fb_dpss__blk1553_dn8, locals.var_fb_dpss__blk1553_dn9, locals.var_fb_dpss__blk1553_dn10, locals.var_fb_dpss__blk1553_dn13,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign65270_body37_e101552;
            locals.var_fs02_dps0_dn0 = assign65270_body37_e101552_d_n0;
            locals.var_fs02_dps0_dn2 = assign65270_body37_e101552_d_n2;
            locals.var_fs02_dps0_dn4 = assign65270_body37_e101552_d_n4;
            locals.var_fs02_dps0_dn5 = assign65270_body37_e101552_d_n5;
            locals.var_fs02_dps0_dn6 = assign65270_body37_e101552_d_n6;
            locals.var_fs02_dps0_dn7 = assign65270_body37_e101552_d_n7;
            locals.var_fs02_dps0_dn8 = assign65270_body37_e101552_d_n8;
            locals.var_fs02_dps0_dn9 = assign65270_body37_e101552_d_n9;
            locals.var_fs02_dps0_dn10 = assign65270_body37_e101552_d_n10;
            locals.var_fs02_dps0_dn13 = assign65270_body37_e101552_d_n13;
            let (assign65270_body38_e101577, assign65270_body38_e101577_d_n0, assign65270_body38_e101577_d_n2, assign65270_body38_e101577_d_n4, assign65270_body38_e101577_d_n5, assign65270_body38_e101577_d_n6, assign65270_body38_e101577_d_n7, assign65270_body38_e101577_d_n8, assign65270_body38_e101577_d_n9, assign65270_body38_e101577_d_n10, assign65270_body38_e101577_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body38_e101574: f64 = (locals.var_phi_s0 - p.p456);
        let assign65270_body38_e101575: f64 = (locals.var_beta * assign65270_body38_e101574);
        (assign65270_body38_e101575, ((locals.var_beta_dn0 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn0)), ((locals.var_beta_dn2 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn2)), ((locals.var_beta_dn4 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn4)), ((locals.var_beta_dn5 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn5)), ((locals.var_beta_dn6 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn6)), ((locals.var_beta_dn7 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn7)), ((locals.var_beta_dn8 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn8)), ((locals.var_beta_dn9 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn9)), ((locals.var_beta_dn10 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn10)), ((locals.var_beta_dn13 * assign65270_body38_e101574) + (locals.var_beta * locals.var_phi_s0_dn13)),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn13,)
    }
};
            locals.var_rho = assign65270_body38_e101577;
            locals.var_rho_dn0 = assign65270_body38_e101577_d_n0;
            locals.var_rho_dn2 = assign65270_body38_e101577_d_n2;
            locals.var_rho_dn4 = assign65270_body38_e101577_d_n4;
            locals.var_rho_dn5 = assign65270_body38_e101577_d_n5;
            locals.var_rho_dn6 = assign65270_body38_e101577_d_n6;
            locals.var_rho_dn7 = assign65270_body38_e101577_d_n7;
            locals.var_rho_dn8 = assign65270_body38_e101577_d_n8;
            locals.var_rho_dn9 = assign65270_body38_e101577_d_n9;
            locals.var_rho_dn10 = assign65270_body38_e101577_d_n10;
            locals.var_rho_dn13 = assign65270_body38_e101577_d_n13;
            let (assign65270_body39_e101599, assign65270_body39_e101599_d_n0, assign65270_body39_e101599_d_n2, assign65270_body39_e101599_d_n4, assign65270_body39_e101599_d_n5, assign65270_body39_e101599_d_n6, assign65270_body39_e101599_d_n7, assign65270_body39_e101599_d_n8, assign65270_body39_e101599_d_n9, assign65270_body39_e101599_d_n10, assign65270_body39_e101599_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body39_e101597: f64 = (locals.var_rho).exp();
        (assign65270_body39_e101597, (assign65270_body39_e101597 * locals.var_rho_dn0), (assign65270_body39_e101597 * locals.var_rho_dn2), (assign65270_body39_e101597 * locals.var_rho_dn4), (assign65270_body39_e101597 * locals.var_rho_dn5), (assign65270_body39_e101597 * locals.var_rho_dn6), (assign65270_body39_e101597 * locals.var_rho_dn7), (assign65270_body39_e101597 * locals.var_rho_dn8), (assign65270_body39_e101597 * locals.var_rho_dn9), (assign65270_body39_e101597 * locals.var_rho_dn10), (assign65270_body39_e101597 * locals.var_rho_dn13),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn4, locals.var_exp_rho_dn5, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn8, locals.var_exp_rho_dn9, locals.var_exp_rho_dn10, locals.var_exp_rho_dn13,)
    }
};
            locals.var_exp_rho = assign65270_body39_e101599;
            locals.var_exp_rho_dn0 = assign65270_body39_e101599_d_n0;
            locals.var_exp_rho_dn2 = assign65270_body39_e101599_d_n2;
            locals.var_exp_rho_dn4 = assign65270_body39_e101599_d_n4;
            locals.var_exp_rho_dn5 = assign65270_body39_e101599_d_n5;
            locals.var_exp_rho_dn6 = assign65270_body39_e101599_d_n6;
            locals.var_exp_rho_dn7 = assign65270_body39_e101599_d_n7;
            locals.var_exp_rho_dn8 = assign65270_body39_e101599_d_n8;
            locals.var_exp_rho_dn9 = assign65270_body39_e101599_d_n9;
            locals.var_exp_rho_dn10 = assign65270_body39_e101599_d_n10;
            locals.var_exp_rho_dn13 = assign65270_body39_e101599_d_n13;
            let (assign65270_body40_e101628, assign65270_body40_e101628_d_n0, assign65270_body40_e101628_d_n2, assign65270_body40_e101628_d_n4, assign65270_body40_e101628_d_n5, assign65270_body40_e101628_d_n6, assign65270_body40_e101628_d_n7, assign65270_body40_e101628_d_n8, assign65270_body40_e101628_d_n9, assign65270_body40_e101628_d_n10, assign65270_body40_e101628_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body40_e101623: f64 = (locals.var_chi + 1.0);
        let assign65270_body40_e101624: f64 = (locals.var_exp_bvbsvds * assign65270_body40_e101623);
        let assign65270_body40_e101625: f64 = (locals.var_exp_rho - assign65270_body40_e101624);
        let assign65270_body40_e101626: f64 = (locals.var_cnst1 * assign65270_body40_e101625);
        (assign65270_body40_e101626, ((locals.var_cnst1_dn0 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1_dn2 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1_dn4 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn4 - ((locals.var_exp_bvbsvds_dn4 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn4))))), ((locals.var_cnst1_dn5 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn5 - ((locals.var_exp_bvbsvds_dn5 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn5))))), ((locals.var_cnst1_dn6 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1_dn7 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1_dn8 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn8 - ((locals.var_exp_bvbsvds_dn8 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn8))))), ((locals.var_cnst1_dn9 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn9 - ((locals.var_exp_bvbsvds_dn9 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn9))))), ((locals.var_cnst1_dn10 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1_dn13 * assign65270_body40_e101625) + (locals.var_cnst1 * (locals.var_exp_rho_dn13 - ((locals.var_exp_bvbsvds_dn13 * assign65270_body40_e101623) + (locals.var_exp_bvbsvds * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign65270_body40_e101628;
            locals.var_fs01_dn0 = assign65270_body40_e101628_d_n0;
            locals.var_fs01_dn2 = assign65270_body40_e101628_d_n2;
            locals.var_fs01_dn4 = assign65270_body40_e101628_d_n4;
            locals.var_fs01_dn5 = assign65270_body40_e101628_d_n5;
            locals.var_fs01_dn6 = assign65270_body40_e101628_d_n6;
            locals.var_fs01_dn7 = assign65270_body40_e101628_d_n7;
            locals.var_fs01_dn8 = assign65270_body40_e101628_d_n8;
            locals.var_fs01_dn9 = assign65270_body40_e101628_d_n9;
            locals.var_fs01_dn10 = assign65270_body40_e101628_d_n10;
            locals.var_fs01_dn13 = assign65270_body40_e101628_d_n13;
            let (assign65270_body41_e101655, assign65270_body41_e101655_d_n0, assign65270_body41_e101655_d_n2, assign65270_body41_e101655_d_n4, assign65270_body41_e101655_d_n5, assign65270_body41_e101655_d_n6, assign65270_body41_e101655_d_n7, assign65270_body41_e101655_d_n8, assign65270_body41_e101655_d_n9, assign65270_body41_e101655_d_n10, assign65270_body41_e101655_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body41_e101649: f64 = (locals.var_cnst1 * locals.var_beta);
        let assign65270_body41_e101652: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign65270_body41_e101653: f64 = (assign65270_body41_e101649 * assign65270_body41_e101652);
        (assign65270_body41_e101653, ((((locals.var_cnst1_dn0 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn0)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), ((((locals.var_cnst1_dn2 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn2)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), ((((locals.var_cnst1_dn4 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn4)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn4 - locals.var_exp_bvbsvds_dn4))), ((((locals.var_cnst1_dn5 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn5)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn5 - locals.var_exp_bvbsvds_dn5))), ((((locals.var_cnst1_dn6 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn6)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), ((((locals.var_cnst1_dn7 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn7)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1_dn8 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn8)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn8 - locals.var_exp_bvbsvds_dn8))), ((((locals.var_cnst1_dn9 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn9)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn9 - locals.var_exp_bvbsvds_dn9))), ((((locals.var_cnst1_dn10 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn10)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), ((((locals.var_cnst1_dn13 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn13)) * assign65270_body41_e101652) + (assign65270_body41_e101649 * (locals.var_exp_rho_dn13 - locals.var_exp_bvbsvds_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign65270_body41_e101655;
            locals.var_fs01_dps0_dn0 = assign65270_body41_e101655_d_n0;
            locals.var_fs01_dps0_dn2 = assign65270_body41_e101655_d_n2;
            locals.var_fs01_dps0_dn4 = assign65270_body41_e101655_d_n4;
            locals.var_fs01_dps0_dn5 = assign65270_body41_e101655_d_n5;
            locals.var_fs01_dps0_dn6 = assign65270_body41_e101655_d_n6;
            locals.var_fs01_dps0_dn7 = assign65270_body41_e101655_d_n7;
            locals.var_fs01_dps0_dn8 = assign65270_body41_e101655_d_n8;
            locals.var_fs01_dps0_dn9 = assign65270_body41_e101655_d_n9;
            locals.var_fs01_dps0_dn10 = assign65270_body41_e101655_d_n10;
            locals.var_fs01_dps0_dn13 = assign65270_body41_e101655_d_n13;
            let (assign65270_body42_e101681, assign65270_body42_e101681_d_n0, assign65270_body42_e101681_d_n2, assign65270_body42_e101681_d_n4, assign65270_body42_e101681_d_n5, assign65270_body42_e101681_d_n6, assign65270_body42_e101681_d_n7, assign65270_body42_e101681_d_n8, assign65270_body42_e101681_d_n9, assign65270_body42_e101681_d_n10, assign65270_body42_e101681_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body42_e101676: f64 = (locals.var_fb * locals.var_fb);
        let assign65270_body42_e101678: f64 = (assign65270_body42_e101676 + locals.var_fs01);
        let assign65270_body42_e101679: f64 = (assign65270_body42_e101678).sqrt();
        (assign65270_body42_e101679, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign65270_body42_e101679)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign65270_body42_e101679)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign65270_body42_e101681;
            locals.var_fs02_dn0 = assign65270_body42_e101681_d_n0;
            locals.var_fs02_dn2 = assign65270_body42_e101681_d_n2;
            locals.var_fs02_dn4 = assign65270_body42_e101681_d_n4;
            locals.var_fs02_dn5 = assign65270_body42_e101681_d_n5;
            locals.var_fs02_dn6 = assign65270_body42_e101681_d_n6;
            locals.var_fs02_dn7 = assign65270_body42_e101681_d_n7;
            locals.var_fs02_dn8 = assign65270_body42_e101681_d_n8;
            locals.var_fs02_dn9 = assign65270_body42_e101681_d_n9;
            locals.var_fs02_dn10 = assign65270_body42_e101681_d_n10;
            locals.var_fs02_dn13 = assign65270_body42_e101681_d_n13;
            let (assign65270_body43_e101712, assign65270_body43_e101712_d_n0, assign65270_body43_e101712_d_n2, assign65270_body43_e101712_d_n4, assign65270_body43_e101712_d_n5, assign65270_body43_e101712_d_n6, assign65270_body43_e101712_d_n7, assign65270_body43_e101712_d_n8, assign65270_body43_e101712_d_n9, assign65270_body43_e101712_d_n10, assign65270_body43_e101712_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1562 == 0.0)) {
        let assign65270_body43_e101703: f64 = (2.0 * locals.var_fb_dpss__blk1553);
        let assign65270_body43_e101705: f64 = (assign65270_body43_e101703 * locals.var_fb);
        let assign65270_body43_e101707: f64 = (assign65270_body43_e101705 + locals.var_fs01_dps0);
        let assign65270_body43_e101708: f64 = (0.5 * assign65270_body43_e101707);
        let assign65270_body43_e101710: f64 = (assign65270_body43_e101708 / locals.var_fs02);
        (assign65270_body43_e101710, ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn0) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn2) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn4) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn5) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn6) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn7) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn8) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn9) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn10) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1553_dn13) * locals.var_fb) + (assign65270_body43_e101703 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign65270_body43_e101708 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign65270_body43_e101712;
            locals.var_fs02_dps0_dn0 = assign65270_body43_e101712_d_n0;
            locals.var_fs02_dps0_dn2 = assign65270_body43_e101712_d_n2;
            locals.var_fs02_dps0_dn4 = assign65270_body43_e101712_d_n4;
            locals.var_fs02_dps0_dn5 = assign65270_body43_e101712_d_n5;
            locals.var_fs02_dps0_dn6 = assign65270_body43_e101712_d_n6;
            locals.var_fs02_dps0_dn7 = assign65270_body43_e101712_d_n7;
            locals.var_fs02_dps0_dn8 = assign65270_body43_e101712_d_n8;
            locals.var_fs02_dps0_dn9 = assign65270_body43_e101712_d_n9;
            locals.var_fs02_dps0_dn10 = assign65270_body43_e101712_d_n10;
            locals.var_fs02_dps0_dn13 = assign65270_body43_e101712_d_n13;
            let (assign65270_body44_e101734, assign65270_body44_e101734_d_n0, assign65270_body44_e101734_d_n2, assign65270_body44_e101734_d_n4, assign65270_body44_e101734_d_n5, assign65270_body44_e101734_d_n6, assign65270_body44_e101734_d_n7, assign65270_body44_e101734_d_n8, assign65270_body44_e101734_d_n9, assign65270_body44_e101734_d_n10, assign65270_body44_e101734_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body44_e101726: f64 = (-locals.var_vgp__blk1525);
        let assign65270_body44_e101728: f64 = (assign65270_body44_e101726 + locals.var_phi_s0);
        let assign65270_body44_e101731: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign65270_body44_e101732: f64 = (assign65270_body44_e101728 + assign65270_body44_e101731);
        (assign65270_body44_e101732, (((-locals.var_vgp__blk1525_dn0) + locals.var_phi_s0_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgp__blk1525_dn2) + locals.var_phi_s0_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (((-locals.var_vgp__blk1525_dn4) + locals.var_phi_s0_dn4) + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (((-locals.var_vgp__blk1525_dn5) + locals.var_phi_s0_dn5) + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgp__blk1525_dn6) + locals.var_phi_s0_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgp__blk1525_dn7) + locals.var_phi_s0_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgp__blk1525_dn8) + locals.var_phi_s0_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgp__blk1525_dn9) + locals.var_phi_s0_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (((-locals.var_vgp__blk1525_dn10) + locals.var_phi_s0_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (((-locals.var_vgp__blk1525_dn13) + locals.var_phi_s0_dn13) + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign65270_body44_e101734;
            locals.var_fs0_dn0 = assign65270_body44_e101734_d_n0;
            locals.var_fs0_dn2 = assign65270_body44_e101734_d_n2;
            locals.var_fs0_dn4 = assign65270_body44_e101734_d_n4;
            locals.var_fs0_dn5 = assign65270_body44_e101734_d_n5;
            locals.var_fs0_dn6 = assign65270_body44_e101734_d_n6;
            locals.var_fs0_dn7 = assign65270_body44_e101734_d_n7;
            locals.var_fs0_dn8 = assign65270_body44_e101734_d_n8;
            locals.var_fs0_dn9 = assign65270_body44_e101734_d_n9;
            locals.var_fs0_dn10 = assign65270_body44_e101734_d_n10;
            locals.var_fs0_dn13 = assign65270_body44_e101734_d_n13;
            let (assign65270_body45_e101753, assign65270_body45_e101753_d_n0, assign65270_body45_e101753_d_n2, assign65270_body45_e101753_d_n4, assign65270_body45_e101753_d_n5, assign65270_body45_e101753_d_n6, assign65270_body45_e101753_d_n7, assign65270_body45_e101753_d_n8, assign65270_body45_e101753_d_n9, assign65270_body45_e101753_d_n10, assign65270_body45_e101753_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body45_e101750: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign65270_body45_e101751: f64 = (1.0 + assign65270_body45_e101750);
        (assign65270_body45_e101751, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign65270_body45_e101753;
            locals.var_fs0_dps0_dn0 = assign65270_body45_e101753_d_n0;
            locals.var_fs0_dps0_dn2 = assign65270_body45_e101753_d_n2;
            locals.var_fs0_dps0_dn4 = assign65270_body45_e101753_d_n4;
            locals.var_fs0_dps0_dn5 = assign65270_body45_e101753_d_n5;
            locals.var_fs0_dps0_dn6 = assign65270_body45_e101753_d_n6;
            locals.var_fs0_dps0_dn7 = assign65270_body45_e101753_d_n7;
            locals.var_fs0_dps0_dn8 = assign65270_body45_e101753_d_n8;
            locals.var_fs0_dps0_dn9 = assign65270_body45_e101753_d_n9;
            locals.var_fs0_dps0_dn10 = assign65270_body45_e101753_d_n10;
            locals.var_fs0_dps0_dn13 = assign65270_body45_e101753_d_n13;
            let assign65270_body46_e101756: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1563 = assign65270_body46_e101756;
            let (assign65270_body47_e101775,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 != 0.0)) {
        let assign65270_body47_e101773: f64 = (locals.var_lp_s0_max + 1.0);
        (assign65270_body47_e101773,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign65270_body47_e101775;
            let (assign65270_body48_e101796, assign65270_body48_e101796_d_n0, assign65270_body48_e101796_d_n2, assign65270_body48_e101796_d_n4, assign65270_body48_e101796_d_n5, assign65270_body48_e101796_d_n6, assign65270_body48_e101796_d_n7, assign65270_body48_e101796_d_n8, assign65270_body48_e101796_d_n9, assign65270_body48_e101796_d_n10, assign65270_body48_e101796_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 == 0.0)) {
        let assign65270_body48_e101792: f64 = (-locals.var_fs0);
        let assign65270_body48_e101794: f64 = (assign65270_body48_e101792 / locals.var_fs0_dps0);
        (assign65270_body48_e101794, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign65270_body48_e101792 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign65270_body48_e101796;
            locals.var_dps0_dn0 = assign65270_body48_e101796_d_n0;
            locals.var_dps0_dn2 = assign65270_body48_e101796_d_n2;
            locals.var_dps0_dn4 = assign65270_body48_e101796_d_n4;
            locals.var_dps0_dn5 = assign65270_body48_e101796_d_n5;
            locals.var_dps0_dn6 = assign65270_body48_e101796_d_n6;
            locals.var_dps0_dn7 = assign65270_body48_e101796_d_n7;
            locals.var_dps0_dn8 = assign65270_body48_e101796_d_n8;
            locals.var_dps0_dn9 = assign65270_body48_e101796_d_n9;
            locals.var_dps0_dn10 = assign65270_body48_e101796_d_n10;
            locals.var_dps0_dn13 = assign65270_body48_e101796_d_n13;
            let (assign65270_body49_e101827, assign65270_body49_e101827_d_n0, assign65270_body49_e101827_d_n2, assign65270_body49_e101827_d_n4, assign65270_body49_e101827_d_n5, assign65270_body49_e101827_d_n6, assign65270_body49_e101827_d_n7, assign65270_body49_e101827_d_n8, assign65270_body49_e101827_d_n9, assign65270_body49_e101827_d_n10, assign65270_body49_e101827_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 == 0.0)) {
        let assign65270_body49_e101814: f64 = (0.5 * 0.1);
        let assign65270_body49_e101818: f64 = (locals.var_phi_s0).abs();
        let (assign65270_body49_e101823, assign65270_body49_e101823_d_n0, assign65270_body49_e101823_d_n2, assign65270_body49_e101823_d_n4, assign65270_body49_e101823_d_n5, assign65270_body49_e101823_d_n6, assign65270_body49_e101823_d_n7, assign65270_body49_e101823_d_n8, assign65270_body49_e101823_d_n9, assign65270_body49_e101823_d_n10, assign65270_body49_e101823_d_n13,) = {
            if (1.0 >= assign65270_body49_e101818) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65270_body49_e101822: f64 = (locals.var_phi_s0).abs();
                (assign65270_body49_e101822, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn0 } else { (-locals.var_phi_s0_dn0) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn2 } else { (-locals.var_phi_s0_dn2) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn4 } else { (-locals.var_phi_s0_dn4) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn5 } else { (-locals.var_phi_s0_dn5) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn6 } else { (-locals.var_phi_s0_dn6) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn7 } else { (-locals.var_phi_s0_dn7) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn8 } else { (-locals.var_phi_s0_dn8) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn9 } else { (-locals.var_phi_s0_dn9) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn10 } else { (-locals.var_phi_s0_dn10) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn13 } else { (-locals.var_phi_s0_dn13) },)
            }
        };
        let assign65270_body49_e101824: f64 = (1.0 + assign65270_body49_e101823);
        let assign65270_body49_e101825: f64 = (assign65270_body49_e101814 * assign65270_body49_e101824);
        (assign65270_body49_e101825, (assign65270_body49_e101814 * assign65270_body49_e101823_d_n0), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n2), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n4), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n5), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n6), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n7), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n8), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n9), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n10), (assign65270_body49_e101814 * assign65270_body49_e101823_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign65270_body49_e101827;
            locals.var_dplim_dn0 = assign65270_body49_e101827_d_n0;
            locals.var_dplim_dn2 = assign65270_body49_e101827_d_n2;
            locals.var_dplim_dn4 = assign65270_body49_e101827_d_n4;
            locals.var_dplim_dn5 = assign65270_body49_e101827_d_n5;
            locals.var_dplim_dn6 = assign65270_body49_e101827_d_n6;
            locals.var_dplim_dn7 = assign65270_body49_e101827_d_n7;
            locals.var_dplim_dn8 = assign65270_body49_e101827_d_n8;
            locals.var_dplim_dn9 = assign65270_body49_e101827_d_n9;
            locals.var_dplim_dn10 = assign65270_body49_e101827_d_n10;
            locals.var_dplim_dn13 = assign65270_body49_e101827_d_n13;
            let assign65270_body50_e101829: f64 = (locals.var_dps0).abs();
            let assign65270_body50_e101831: f64 = if assign65270_body50_e101829 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1564 = assign65270_body50_e101831;
            let (assign65270_body51_e101859, assign65270_body51_e101859_d_n0, assign65270_body51_e101859_d_n2, assign65270_body51_e101859_d_n4, assign65270_body51_e101859_d_n5, assign65270_body51_e101859_d_n6, assign65270_body51_e101859_d_n7, assign65270_body51_e101859_d_n8, assign65270_body51_e101859_d_n9, assign65270_body51_e101859_d_n10, assign65270_body51_e101859_d_n13,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1564 != 0.0)) {
        let (assign65270_body51_e101856,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign65270_body51_e101855: f64 = (-1.0);
                (assign65270_body51_e101855,)
            }
        };
        let assign65270_body51_e101857: f64 = (locals.var_dplim * assign65270_body51_e101856);
        (assign65270_body51_e101857, (locals.var_dplim_dn0 * assign65270_body51_e101856), (locals.var_dplim_dn2 * assign65270_body51_e101856), (locals.var_dplim_dn4 * assign65270_body51_e101856), (locals.var_dplim_dn5 * assign65270_body51_e101856), (locals.var_dplim_dn6 * assign65270_body51_e101856), (locals.var_dplim_dn7 * assign65270_body51_e101856), (locals.var_dplim_dn8 * assign65270_body51_e101856), (locals.var_dplim_dn9 * assign65270_body51_e101856), (locals.var_dplim_dn10 * assign65270_body51_e101856), (locals.var_dplim_dn13 * assign65270_body51_e101856),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign65270_body51_e101859;
            locals.var_dps0_dn0 = assign65270_body51_e101859_d_n0;
            locals.var_dps0_dn2 = assign65270_body51_e101859_d_n2;
            locals.var_dps0_dn4 = assign65270_body51_e101859_d_n4;
            locals.var_dps0_dn5 = assign65270_body51_e101859_d_n5;
            locals.var_dps0_dn6 = assign65270_body51_e101859_d_n6;
            locals.var_dps0_dn7 = assign65270_body51_e101859_d_n7;
            locals.var_dps0_dn8 = assign65270_body51_e101859_d_n8;
            locals.var_dps0_dn9 = assign65270_body51_e101859_d_n9;
            locals.var_dps0_dn10 = assign65270_body51_e101859_d_n10;
            locals.var_dps0_dn13 = assign65270_body51_e101859_d_n13;
            let (assign65270_body52_e101879, assign65270_body52_e101879_d_n0, assign65270_body52_e101879_d_n2, assign65270_body52_e101879_d_n4, assign65270_body52_e101879_d_n5, assign65270_body52_e101879_d_n6, assign65270_body52_e101879_d_n7, assign65270_body52_e101879_d_n8, assign65270_body52_e101879_d_n9, assign65270_body52_e101879_d_n10, assign65270_body52_e101879_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 == 0.0)) {
        let assign65270_body52_e101877: f64 = (locals.var_phi_s0 + locals.var_dps0);
        (assign65270_body52_e101877, (locals.var_phi_s0_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_dn4 + locals.var_dps0_dn4), (locals.var_phi_s0_dn5 + locals.var_dps0_dn5), (locals.var_phi_s0_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_dn8 + locals.var_dps0_dn8), (locals.var_phi_s0_dn9 + locals.var_dps0_dn9), (locals.var_phi_s0_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn13,)
    }
};
            locals.var_phi_s0 = assign65270_body52_e101879;
            locals.var_phi_s0_dn0 = assign65270_body52_e101879_d_n0;
            locals.var_phi_s0_dn2 = assign65270_body52_e101879_d_n2;
            locals.var_phi_s0_dn4 = assign65270_body52_e101879_d_n4;
            locals.var_phi_s0_dn5 = assign65270_body52_e101879_d_n5;
            locals.var_phi_s0_dn6 = assign65270_body52_e101879_d_n6;
            locals.var_phi_s0_dn7 = assign65270_body52_e101879_d_n7;
            locals.var_phi_s0_dn8 = assign65270_body52_e101879_d_n8;
            locals.var_phi_s0_dn9 = assign65270_body52_e101879_d_n9;
            locals.var_phi_s0_dn10 = assign65270_body52_e101879_d_n10;
            locals.var_phi_s0_dn13 = assign65270_body52_e101879_d_n13;
            let assign65270_body53_e101881: f64 = (locals.var_dps0).abs();
            let assign65270_body53_e101885: f64 = (locals.var_fs0).abs();
            let assign65270_body53_e101888: f64 = if ((assign65270_body53_e101881 <= 1e-12) && (assign65270_body53_e101885 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1565 = assign65270_body53_e101888;
            let (assign65270_body54_e101908,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1565 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign65270_body54_e101908;
            let (assign65270_body55_e101925,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        let assign65270_body55_e101923: f64 = (locals.var_lp_s0 + 1.0);
        (assign65270_body55_e101923,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign65270_body55_e101925;
        }

    }

    pub(super) fn stamp_transient_block_223(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65280_e101940, assign65280_e101940_d_n0, assign65280_e101940_d_n2, assign65280_e101940_d_n4, assign65280_e101940_d_n5, assign65280_e101940_d_n6, assign65280_e101940_d_n7, assign65280_e101940_d_n8, assign65280_e101940_d_n9, assign65280_e101940_d_n10, assign65280_e101940_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1538 == 0.0)) && (locals.var_guard1544 != 0.0)) && (locals.var_guard1554 == 0.0)) {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn13,)
    } else {
        (locals.var_ps0__blk1523, locals.var_ps0__blk1523_dn0, locals.var_ps0__blk1523_dn2, locals.var_ps0__blk1523_dn4, locals.var_ps0__blk1523_dn5, locals.var_ps0__blk1523_dn6, locals.var_ps0__blk1523_dn7, locals.var_ps0__blk1523_dn8, locals.var_ps0__blk1523_dn9, locals.var_ps0__blk1523_dn10, locals.var_ps0__blk1523_dn13,)
    }
};
        locals.var_ps0__blk1523 = assign65280_e101940;
        locals.var_ps0__blk1523_dn0 = assign65280_e101940_d_n0;
        locals.var_ps0__blk1523_dn2 = assign65280_e101940_d_n2;
        locals.var_ps0__blk1523_dn4 = assign65280_e101940_d_n4;
        locals.var_ps0__blk1523_dn5 = assign65280_e101940_d_n5;
        locals.var_ps0__blk1523_dn6 = assign65280_e101940_d_n6;
        locals.var_ps0__blk1523_dn7 = assign65280_e101940_d_n7;
        locals.var_ps0__blk1523_dn8 = assign65280_e101940_d_n8;
        locals.var_ps0__blk1523_dn9 = assign65280_e101940_d_n9;
        locals.var_ps0__blk1523_dn10 = assign65280_e101940_d_n10;
        locals.var_ps0__blk1523_dn13 = assign65280_e101940_d_n13;

        let (assign65290_e101952, assign65290_e101952_d_n0, assign65290_e101952_d_n2, assign65290_e101952_d_n4, assign65290_e101952_d_n5, assign65290_e101952_d_n6, assign65290_e101952_d_n7, assign65290_e101952_d_n8, assign65290_e101952_d_n9, assign65290_e101952_d_n10, assign65290_e101952_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65290_e101946: f64 = (-locals.var_beta);
        let assign65290_e101949: f64 = (locals.var_ps0__blk1523 - locals.var_dphi_vds);
        let assign65290_e101950: f64 = (assign65290_e101946 * assign65290_e101949);
        (assign65290_e101950, (((-locals.var_beta_dn0) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn0 - locals.var_dphi_vds_dn0))), (((-locals.var_beta_dn2) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn2 - locals.var_dphi_vds_dn2))), (((-locals.var_beta_dn4) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn4 - locals.var_dphi_vds_dn4))), (((-locals.var_beta_dn5) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn5 - locals.var_dphi_vds_dn5))), (((-locals.var_beta_dn6) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn6 - locals.var_dphi_vds_dn6))), (((-locals.var_beta_dn7) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn7 - locals.var_dphi_vds_dn7))), (((-locals.var_beta_dn8) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn8 - locals.var_dphi_vds_dn8))), (((-locals.var_beta_dn9) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn9 - locals.var_dphi_vds_dn9))), (((-locals.var_beta_dn10) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn10 - locals.var_dphi_vds_dn10))), (((-locals.var_beta_dn13) * assign65290_e101949) + (assign65290_e101946 * (locals.var_ps0__blk1523_dn13 - locals.var_dphi_vds_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign65290_e101952;
        locals.var_t5_dn0 = assign65290_e101952_d_n0;
        locals.var_t5_dn2 = assign65290_e101952_d_n2;
        locals.var_t5_dn4 = assign65290_e101952_d_n4;
        locals.var_t5_dn5 = assign65290_e101952_d_n5;
        locals.var_t5_dn6 = assign65290_e101952_d_n6;
        locals.var_t5_dn7 = assign65290_e101952_d_n7;
        locals.var_t5_dn8 = assign65290_e101952_d_n8;
        locals.var_t5_dn9 = assign65290_e101952_d_n9;
        locals.var_t5_dn10 = assign65290_e101952_d_n10;
        locals.var_t5_dn13 = assign65290_e101952_d_n13;

        let (assign65300_e101960, assign65300_e101960_d_n0, assign65300_e101960_d_n2, assign65300_e101960_d_n4, assign65300_e101960_d_n5, assign65300_e101960_d_n6, assign65300_e101960_d_n7, assign65300_e101960_d_n8, assign65300_e101960_d_n9, assign65300_e101960_d_n10, assign65300_e101960_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65300_e101958: f64 = (locals.var_t5).abs();
        (assign65300_e101958, if locals.var_t5 >= 0.0 { locals.var_t5_dn0 } else { (-locals.var_t5_dn0) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn2 } else { (-locals.var_t5_dn2) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn4 } else { (-locals.var_t5_dn4) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn5 } else { (-locals.var_t5_dn5) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn6 } else { (-locals.var_t5_dn6) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn7 } else { (-locals.var_t5_dn7) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn8 } else { (-locals.var_t5_dn8) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn9 } else { (-locals.var_t5_dn9) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn10 } else { (-locals.var_t5_dn10) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn13 } else { (-locals.var_t5_dn13) },)
    } else {
        (locals.var_t5abs, locals.var_t5abs_dn0, locals.var_t5abs_dn2, locals.var_t5abs_dn4, locals.var_t5abs_dn5, locals.var_t5abs_dn6, locals.var_t5abs_dn7, locals.var_t5abs_dn8, locals.var_t5abs_dn9, locals.var_t5abs_dn10, locals.var_t5abs_dn13,)
    }
};
        locals.var_t5abs = assign65300_e101960;
        locals.var_t5abs_dn0 = assign65300_e101960_d_n0;
        locals.var_t5abs_dn2 = assign65300_e101960_d_n2;
        locals.var_t5abs_dn4 = assign65300_e101960_d_n4;
        locals.var_t5abs_dn5 = assign65300_e101960_d_n5;
        locals.var_t5abs_dn6 = assign65300_e101960_d_n6;
        locals.var_t5abs_dn7 = assign65300_e101960_d_n7;
        locals.var_t5abs_dn8 = assign65300_e101960_d_n8;
        locals.var_t5abs_dn9 = assign65300_e101960_d_n9;
        locals.var_t5abs_dn10 = assign65300_e101960_d_n10;
        locals.var_t5abs_dn13 = assign65300_e101960_d_n13;

        let (assign65310_e101968, assign65310_e101968_d_n0, assign65310_e101968_d_n2, assign65310_e101968_d_n4, assign65310_e101968_d_n5, assign65310_e101968_d_n6, assign65310_e101968_d_n7, assign65310_e101968_d_n8, assign65310_e101968_d_n9, assign65310_e101968_d_n10, assign65310_e101968_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65310_e101966: f64 = (locals.var_t5).exp();
        (assign65310_e101966, (assign65310_e101966 * locals.var_t5_dn0), (assign65310_e101966 * locals.var_t5_dn2), (assign65310_e101966 * locals.var_t5_dn4), (assign65310_e101966 * locals.var_t5_dn5), (assign65310_e101966 * locals.var_t5_dn6), (assign65310_e101966 * locals.var_t5_dn7), (assign65310_e101966 * locals.var_t5_dn8), (assign65310_e101966 * locals.var_t5_dn9), (assign65310_e101966 * locals.var_t5_dn10), (assign65310_e101966 * locals.var_t5_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign65310_e101968;
        locals.var_t6_dn0 = assign65310_e101968_d_n0;
        locals.var_t6_dn2 = assign65310_e101968_d_n2;
        locals.var_t6_dn4 = assign65310_e101968_d_n4;
        locals.var_t6_dn5 = assign65310_e101968_d_n5;
        locals.var_t6_dn6 = assign65310_e101968_d_n6;
        locals.var_t6_dn7 = assign65310_e101968_d_n7;
        locals.var_t6_dn8 = assign65310_e101968_d_n8;
        locals.var_t6_dn9 = assign65310_e101968_d_n9;
        locals.var_t6_dn10 = assign65310_e101968_d_n10;
        locals.var_t6_dn13 = assign65310_e101968_d_n13;

        let (assign65320_e101979, assign65320_e101979_d_n0, assign65320_e101979_d_n2, assign65320_e101979_d_n4, assign65320_e101979_d_n5, assign65320_e101979_d_n6, assign65320_e101979_d_n7, assign65320_e101979_d_n8, assign65320_e101979_d_n9, assign65320_e101979_d_n10, assign65320_e101979_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65320_e101975: f64 = (locals.var_t6 - 1.0);
        let assign65320_e101977: f64 = (assign65320_e101975 - locals.var_t5);
        (assign65320_e101977, (locals.var_t6_dn0 - locals.var_t5_dn0), (locals.var_t6_dn2 - locals.var_t5_dn2), (locals.var_t6_dn4 - locals.var_t5_dn4), (locals.var_t6_dn5 - locals.var_t5_dn5), (locals.var_t6_dn6 - locals.var_t5_dn6), (locals.var_t6_dn7 - locals.var_t5_dn7), (locals.var_t6_dn8 - locals.var_t5_dn8), (locals.var_t6_dn9 - locals.var_t5_dn9), (locals.var_t6_dn10 - locals.var_t5_dn10), (locals.var_t6_dn13 - locals.var_t5_dn13),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign65320_e101979;
        locals.var_t7_dn0 = assign65320_e101979_d_n0;
        locals.var_t7_dn2 = assign65320_e101979_d_n2;
        locals.var_t7_dn4 = assign65320_e101979_d_n4;
        locals.var_t7_dn5 = assign65320_e101979_d_n5;
        locals.var_t7_dn6 = assign65320_e101979_d_n6;
        locals.var_t7_dn7 = assign65320_e101979_d_n7;
        locals.var_t7_dn8 = assign65320_e101979_d_n8;
        locals.var_t7_dn9 = assign65320_e101979_d_n9;
        locals.var_t7_dn10 = assign65320_e101979_d_n10;
        locals.var_t7_dn13 = assign65320_e101979_d_n13;

        let assign65330_e101982: f64 = if locals.var_t5 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard1566 = assign65330_e101982;

        let (assign65340_e101995, assign65340_e101995_d_n0, assign65340_e101995_d_n2, assign65340_e101995_d_n4, assign65340_e101995_d_n5, assign65340_e101995_d_n6, assign65340_e101995_d_n7, assign65340_e101995_d_n8, assign65340_e101995_d_n9, assign65340_e101995_d_n10, assign65340_e101995_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1566 != 0.0)) {
        let assign65340_e101990: f64 = (-locals.var_cnst0);
        let assign65340_e101992: f64 = (locals.var_t7).sqrt();
        let assign65340_e101993: f64 = (assign65340_e101990 * assign65340_e101992);
        (assign65340_e101993, (((-locals.var_cnst0_dn0) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn0 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn2) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn2 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn4) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn4 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn5) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn5 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn6) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn6 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn7) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn7 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn8) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn8 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn9) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn9 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn10) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn10 / (2.0 * assign65340_e101992)))), (((-locals.var_cnst0_dn13) * assign65340_e101992) + (assign65340_e101990 * (locals.var_t7_dn13 / (2.0 * assign65340_e101992)))),)
    } else {
        (locals.var_qbu__blk1537, locals.var_qbu__blk1537_dn0, locals.var_qbu__blk1537_dn2, locals.var_qbu__blk1537_dn4, locals.var_qbu__blk1537_dn5, locals.var_qbu__blk1537_dn6, locals.var_qbu__blk1537_dn7, locals.var_qbu__blk1537_dn8, locals.var_qbu__blk1537_dn9, locals.var_qbu__blk1537_dn10, locals.var_qbu__blk1537_dn13,)
    }
};
        locals.var_qbu__blk1537 = assign65340_e101995;
        locals.var_qbu__blk1537_dn0 = assign65340_e101995_d_n0;
        locals.var_qbu__blk1537_dn2 = assign65340_e101995_d_n2;
        locals.var_qbu__blk1537_dn4 = assign65340_e101995_d_n4;
        locals.var_qbu__blk1537_dn5 = assign65340_e101995_d_n5;
        locals.var_qbu__blk1537_dn6 = assign65340_e101995_d_n6;
        locals.var_qbu__blk1537_dn7 = assign65340_e101995_d_n7;
        locals.var_qbu__blk1537_dn8 = assign65340_e101995_d_n8;
        locals.var_qbu__blk1537_dn9 = assign65340_e101995_d_n9;
        locals.var_qbu__blk1537_dn10 = assign65340_e101995_d_n10;
        locals.var_qbu__blk1537_dn13 = assign65340_e101995_d_n13;

        let assign65350_e101998: f64 = if locals.var_t5abs > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard1567 = assign65350_e101998;

        let (assign65360_e102013, assign65360_e102013_d_n0, assign65360_e102013_d_n2, assign65360_e102013_d_n4, assign65360_e102013_d_n5, assign65360_e102013_d_n6, assign65360_e102013_d_n7, assign65360_e102013_d_n8, assign65360_e102013_d_n9, assign65360_e102013_d_n10, assign65360_e102013_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1566 == 0.0)) && (locals.var_guard1567 != 0.0)) {
        let assign65360_e102010: f64 = (locals.var_t7).sqrt();
        let assign65360_e102011: f64 = (locals.var_cnst0 * assign65360_e102010);
        (assign65360_e102011, ((locals.var_cnst0_dn0 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn0 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn2 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn2 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn4 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn4 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn5 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn5 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn6 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn6 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn7 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn7 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn8 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn8 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn9 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn9 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn10 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn10 / (2.0 * assign65360_e102010)))), ((locals.var_cnst0_dn13 * assign65360_e102010) + (locals.var_cnst0 * (locals.var_t7_dn13 / (2.0 * assign65360_e102010)))),)
    } else {
        (locals.var_qbu__blk1537, locals.var_qbu__blk1537_dn0, locals.var_qbu__blk1537_dn2, locals.var_qbu__blk1537_dn4, locals.var_qbu__blk1537_dn5, locals.var_qbu__blk1537_dn6, locals.var_qbu__blk1537_dn7, locals.var_qbu__blk1537_dn8, locals.var_qbu__blk1537_dn9, locals.var_qbu__blk1537_dn10, locals.var_qbu__blk1537_dn13,)
    }
};
        locals.var_qbu__blk1537 = assign65360_e102013;
        locals.var_qbu__blk1537_dn0 = assign65360_e102013_d_n0;
        locals.var_qbu__blk1537_dn2 = assign65360_e102013_d_n2;
        locals.var_qbu__blk1537_dn4 = assign65360_e102013_d_n4;
        locals.var_qbu__blk1537_dn5 = assign65360_e102013_d_n5;
        locals.var_qbu__blk1537_dn6 = assign65360_e102013_d_n6;
        locals.var_qbu__blk1537_dn7 = assign65360_e102013_d_n7;
        locals.var_qbu__blk1537_dn8 = assign65360_e102013_d_n8;
        locals.var_qbu__blk1537_dn9 = assign65360_e102013_d_n9;
        locals.var_qbu__blk1537_dn10 = assign65360_e102013_d_n10;
        locals.var_qbu__blk1537_dn13 = assign65360_e102013_d_n13;

        let (assign65370_e102042, assign65370_e102042_d_n0, assign65370_e102042_d_n2, assign65370_e102042_d_n4, assign65370_e102042_d_n5, assign65370_e102042_d_n6, assign65370_e102042_d_n7, assign65370_e102042_d_n8, assign65370_e102042_d_n9, assign65370_e102042_d_n10, assign65370_e102042_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1566 == 0.0)) && (locals.var_guard1567 == 0.0)) {
        let assign65370_e102025: f64 = (-locals.var_t5);
        let assign65370_e102027: f64 = (assign65370_e102025 * 0.7071067811865475);
        let assign65370_e102031: f64 = (locals.var_t5abs * 0.3333333333333333);
        let assign65370_e102035: f64 = (0.25 * locals.var_t5abs);
        let assign65370_e102036: f64 = (1.0 + assign65370_e102035);
        let assign65370_e102037: f64 = (assign65370_e102031 * assign65370_e102036);
        let assign65370_e102038: f64 = (1.0 + assign65370_e102037);
        let assign65370_e102039: f64 = (assign65370_e102038).sqrt();
        let assign65370_e102040: f64 = (assign65370_e102027 * assign65370_e102039);
        (assign65370_e102040, ((((-locals.var_t5_dn0) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn0 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn0))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn2) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn2 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn2))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn4) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn4 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn4))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn5) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn5 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn5))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn6) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn6 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn6))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn7) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn7 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn7))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn8) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn8 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn8))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn9) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn9 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn9))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn10) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn10 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn10))) / (2.0 * assign65370_e102039)))), ((((-locals.var_t5_dn13) * 0.7071067811865475) * assign65370_e102039) + (assign65370_e102027 * ((((locals.var_t5abs_dn13 * 0.3333333333333333) * assign65370_e102036) + (assign65370_e102031 * (0.25 * locals.var_t5abs_dn13))) / (2.0 * assign65370_e102039)))),)
    } else {
        (locals.var_qbu__blk1537, locals.var_qbu__blk1537_dn0, locals.var_qbu__blk1537_dn2, locals.var_qbu__blk1537_dn4, locals.var_qbu__blk1537_dn5, locals.var_qbu__blk1537_dn6, locals.var_qbu__blk1537_dn7, locals.var_qbu__blk1537_dn8, locals.var_qbu__blk1537_dn9, locals.var_qbu__blk1537_dn10, locals.var_qbu__blk1537_dn13,)
    }
};
        locals.var_qbu__blk1537 = assign65370_e102042;
        locals.var_qbu__blk1537_dn0 = assign65370_e102042_d_n0;
        locals.var_qbu__blk1537_dn2 = assign65370_e102042_d_n2;
        locals.var_qbu__blk1537_dn4 = assign65370_e102042_d_n4;
        locals.var_qbu__blk1537_dn5 = assign65370_e102042_d_n5;
        locals.var_qbu__blk1537_dn6 = assign65370_e102042_d_n6;
        locals.var_qbu__blk1537_dn7 = assign65370_e102042_d_n7;
        locals.var_qbu__blk1537_dn8 = assign65370_e102042_d_n8;
        locals.var_qbu__blk1537_dn9 = assign65370_e102042_d_n9;
        locals.var_qbu__blk1537_dn10 = assign65370_e102042_d_n10;
        locals.var_qbu__blk1537_dn13 = assign65370_e102042_d_n13;

        let (assign65380_e102058, assign65380_e102058_d_n0, assign65380_e102058_d_n2, assign65380_e102058_d_n4, assign65380_e102058_d_n5, assign65380_e102058_d_n6, assign65380_e102058_d_n7, assign65380_e102058_d_n8, assign65380_e102058_d_n9, assign65380_e102058_d_n10, assign65380_e102058_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65380_e102049: f64 = (locals.var_qbu__blk1537 * locals.var_qbu__blk1537);
        let assign65380_e102052: f64 = (4.0 * 1e-6);
        let assign65380_e102054: f64 = (assign65380_e102052 * 1e-6);
        let assign65380_e102055: f64 = (assign65380_e102049 + assign65380_e102054);
        let assign65380_e102056: f64 = (assign65380_e102055).sqrt();
        (assign65380_e102056, (((locals.var_qbu__blk1537_dn0 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn0)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn2 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn2)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn4 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn4)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn5 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn5)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn6 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn6)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn7 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn7)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn8 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn8)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn9 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn9)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn10 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn10)) / (2.0 * assign65380_e102056)), (((locals.var_qbu__blk1537_dn13 * locals.var_qbu__blk1537) + (locals.var_qbu__blk1537 * locals.var_qbu__blk1537_dn13)) / (2.0 * assign65380_e102056)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign65380_e102058;
        locals.var_tmf1_dn0 = assign65380_e102058_d_n0;
        locals.var_tmf1_dn2 = assign65380_e102058_d_n2;
        locals.var_tmf1_dn4 = assign65380_e102058_d_n4;
        locals.var_tmf1_dn5 = assign65380_e102058_d_n5;
        locals.var_tmf1_dn6 = assign65380_e102058_d_n6;
        locals.var_tmf1_dn7 = assign65380_e102058_d_n7;
        locals.var_tmf1_dn8 = assign65380_e102058_d_n8;
        locals.var_tmf1_dn9 = assign65380_e102058_d_n9;
        locals.var_tmf1_dn10 = assign65380_e102058_d_n10;
        locals.var_tmf1_dn13 = assign65380_e102058_d_n13;

        let (assign65390_e102069, assign65390_e102069_d_n0, assign65390_e102069_d_n2, assign65390_e102069_d_n4, assign65390_e102069_d_n5, assign65390_e102069_d_n6, assign65390_e102069_d_n7, assign65390_e102069_d_n8, assign65390_e102069_d_n9, assign65390_e102069_d_n10, assign65390_e102069_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65390_e102066: f64 = (locals.var_qbu__blk1537 + locals.var_tmf1);
        let assign65390_e102067: f64 = (0.5 * assign65390_e102066);
        (assign65390_e102067, (0.5 * (locals.var_qbu__blk1537_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_qbu__blk1537_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_qbu__blk1537_dn4 + locals.var_tmf1_dn4)), (0.5 * (locals.var_qbu__blk1537_dn5 + locals.var_tmf1_dn5)), (0.5 * (locals.var_qbu__blk1537_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_qbu__blk1537_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_qbu__blk1537_dn8 + locals.var_tmf1_dn8)), (0.5 * (locals.var_qbu__blk1537_dn9 + locals.var_tmf1_dn9)), (0.5 * (locals.var_qbu__blk1537_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_qbu__blk1537_dn13 + locals.var_tmf1_dn13)),)
    } else {
        (locals.var_wqbu, locals.var_wqbu_dn0, locals.var_wqbu_dn2, locals.var_wqbu_dn4, locals.var_wqbu_dn5, locals.var_wqbu_dn6, locals.var_wqbu_dn7, locals.var_wqbu_dn8, locals.var_wqbu_dn9, locals.var_wqbu_dn10, locals.var_wqbu_dn13,)
    }
};
        locals.var_wqbu = assign65390_e102069;
        locals.var_wqbu_dn0 = assign65390_e102069_d_n0;
        locals.var_wqbu_dn2 = assign65390_e102069_d_n2;
        locals.var_wqbu_dn4 = assign65390_e102069_d_n4;
        locals.var_wqbu_dn5 = assign65390_e102069_d_n5;
        locals.var_wqbu_dn6 = assign65390_e102069_d_n6;
        locals.var_wqbu_dn7 = assign65390_e102069_d_n7;
        locals.var_wqbu_dn8 = assign65390_e102069_d_n8;
        locals.var_wqbu_dn9 = assign65390_e102069_d_n9;
        locals.var_wqbu_dn10 = assign65390_e102069_d_n10;
        locals.var_wqbu_dn13 = assign65390_e102069_d_n13;

        let (assign65400_e102080, assign65400_e102080_d_n0, assign65400_e102080_d_n2, assign65400_e102080_d_n4, assign65400_e102080_d_n5, assign65400_e102080_d_n6, assign65400_e102080_d_n7, assign65400_e102080_d_n8, assign65400_e102080_d_n9, assign65400_e102080_d_n10, assign65400_e102080_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65400_e102077: f64 = (1.6021918e-19 * locals.var_nsub);
        let assign65400_e102078: f64 = (locals.var_wqbu / assign65400_e102077);
        (assign65400_e102078, (((locals.var_wqbu_dn0 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn0))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn2 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn2))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn4 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn4))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn5 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn5))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn6 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn6))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn7 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn7))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn8 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn8))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn9 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn9))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn10 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn10))) / (assign65400_e102077 * assign65400_e102077)), (((locals.var_wqbu_dn13 * assign65400_e102077) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn13))) / (assign65400_e102077 * assign65400_e102077)),)
    } else {
        (locals.var_wdep__blk1533, locals.var_wdep__blk1533_dn0, locals.var_wdep__blk1533_dn2, locals.var_wdep__blk1533_dn4, locals.var_wdep__blk1533_dn5, locals.var_wdep__blk1533_dn6, locals.var_wdep__blk1533_dn7, locals.var_wdep__blk1533_dn8, locals.var_wdep__blk1533_dn9, locals.var_wdep__blk1533_dn10, locals.var_wdep__blk1533_dn13,)
    }
};
        locals.var_wdep__blk1533 = assign65400_e102080;
        locals.var_wdep__blk1533_dn0 = assign65400_e102080_d_n0;
        locals.var_wdep__blk1533_dn2 = assign65400_e102080_d_n2;
        locals.var_wdep__blk1533_dn4 = assign65400_e102080_d_n4;
        locals.var_wdep__blk1533_dn5 = assign65400_e102080_d_n5;
        locals.var_wdep__blk1533_dn6 = assign65400_e102080_d_n6;
        locals.var_wdep__blk1533_dn7 = assign65400_e102080_d_n7;
        locals.var_wdep__blk1533_dn8 = assign65400_e102080_d_n8;
        locals.var_wdep__blk1533_dn9 = assign65400_e102080_d_n9;
        locals.var_wdep__blk1533_dn10 = assign65400_e102080_d_n10;
        locals.var_wdep__blk1533_dn13 = assign65400_e102080_d_n13;

        let (assign65410_e102089, assign65410_e102089_d_n0, assign65410_e102089_d_n2, assign65410_e102089_d_n4, assign65410_e102089_d_n5, assign65410_e102089_d_n6, assign65410_e102089_d_n7, assign65410_e102089_d_n8, assign65410_e102089_d_n9, assign65410_e102089_d_n10, assign65410_e102089_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65410_e102087: f64 = (locals.var_wdep__blk1533 - p.p452);
        (assign65410_e102087, locals.var_wdep__blk1533_dn0, locals.var_wdep__blk1533_dn2, locals.var_wdep__blk1533_dn4, locals.var_wdep__blk1533_dn5, locals.var_wdep__blk1533_dn6, locals.var_wdep__blk1533_dn7, locals.var_wdep__blk1533_dn8, locals.var_wdep__blk1533_dn9, locals.var_wdep__blk1533_dn10, locals.var_wdep__blk1533_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65410_e102089;
        locals.var_t1_dn0 = assign65410_e102089_d_n0;
        locals.var_t1_dn2 = assign65410_e102089_d_n2;
        locals.var_t1_dn4 = assign65410_e102089_d_n4;
        locals.var_t1_dn5 = assign65410_e102089_d_n5;
        locals.var_t1_dn6 = assign65410_e102089_d_n6;
        locals.var_t1_dn7 = assign65410_e102089_d_n7;
        locals.var_t1_dn8 = assign65410_e102089_d_n8;
        locals.var_t1_dn9 = assign65410_e102089_d_n9;
        locals.var_t1_dn10 = assign65410_e102089_d_n10;
        locals.var_t1_dn13 = assign65410_e102089_d_n13;

        let (assign65420_e102098, assign65420_e102098_d_n0, assign65420_e102098_d_n2, assign65420_e102098_d_n4, assign65420_e102098_d_n5, assign65420_e102098_d_n6, assign65420_e102098_d_n7, assign65420_e102098_d_n8, assign65420_e102098_d_n9, assign65420_e102098_d_n10, assign65420_e102098_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65420_e102096: f64 = (locals.var_wdep__blk1533 * 0.01);
        (assign65420_e102096, (locals.var_wdep__blk1533_dn0 * 0.01), (locals.var_wdep__blk1533_dn2 * 0.01), (locals.var_wdep__blk1533_dn4 * 0.01), (locals.var_wdep__blk1533_dn5 * 0.01), (locals.var_wdep__blk1533_dn6 * 0.01), (locals.var_wdep__blk1533_dn7 * 0.01), (locals.var_wdep__blk1533_dn8 * 0.01), (locals.var_wdep__blk1533_dn9 * 0.01), (locals.var_wdep__blk1533_dn10 * 0.01), (locals.var_wdep__blk1533_dn13 * 0.01),)
    } else {
        (locals.var_delta_1, locals.var_delta_1_dn0, locals.var_delta_1_dn2, locals.var_delta_1_dn4, locals.var_delta_1_dn5, locals.var_delta_1_dn6, locals.var_delta_1_dn7, locals.var_delta_1_dn8, locals.var_delta_1_dn9, locals.var_delta_1_dn10, locals.var_delta_1_dn13,)
    }
};
        locals.var_delta_1 = assign65420_e102098;
        locals.var_delta_1_dn0 = assign65420_e102098_d_n0;
        locals.var_delta_1_dn2 = assign65420_e102098_d_n2;
        locals.var_delta_1_dn4 = assign65420_e102098_d_n4;
        locals.var_delta_1_dn5 = assign65420_e102098_d_n5;
        locals.var_delta_1_dn6 = assign65420_e102098_d_n6;
        locals.var_delta_1_dn7 = assign65420_e102098_d_n7;
        locals.var_delta_1_dn8 = assign65420_e102098_d_n8;
        locals.var_delta_1_dn9 = assign65420_e102098_d_n9;
        locals.var_delta_1_dn10 = assign65420_e102098_d_n10;
        locals.var_delta_1_dn13 = assign65420_e102098_d_n13;

        let (assign65430_e102114, assign65430_e102114_d_n0, assign65430_e102114_d_n2, assign65430_e102114_d_n4, assign65430_e102114_d_n5, assign65430_e102114_d_n6, assign65430_e102114_d_n7, assign65430_e102114_d_n8, assign65430_e102114_d_n9, assign65430_e102114_d_n10, assign65430_e102114_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65430_e102105: f64 = (locals.var_t1 * locals.var_t1);
        let assign65430_e102108: f64 = (4.0 * locals.var_delta_1);
        let assign65430_e102110: f64 = (assign65430_e102108 * locals.var_delta_1);
        let assign65430_e102111: f64 = (assign65430_e102105 + assign65430_e102110);
        let assign65430_e102112: f64 = (assign65430_e102111).sqrt();
        (assign65430_e102112, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (((4.0 * locals.var_delta_1_dn0) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn0))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (((4.0 * locals.var_delta_1_dn2) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn2))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (((4.0 * locals.var_delta_1_dn4) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn4))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (((4.0 * locals.var_delta_1_dn5) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn5))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (((4.0 * locals.var_delta_1_dn6) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn6))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (((4.0 * locals.var_delta_1_dn7) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn7))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (((4.0 * locals.var_delta_1_dn8) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn8))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (((4.0 * locals.var_delta_1_dn9) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn9))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (((4.0 * locals.var_delta_1_dn10) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn10))) / (2.0 * assign65430_e102112)), ((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) + (((4.0 * locals.var_delta_1_dn13) * locals.var_delta_1) + (assign65430_e102108 * locals.var_delta_1_dn13))) / (2.0 * assign65430_e102112)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign65430_e102114;
        locals.var_tmf1_dn0 = assign65430_e102114_d_n0;
        locals.var_tmf1_dn2 = assign65430_e102114_d_n2;
        locals.var_tmf1_dn4 = assign65430_e102114_d_n4;
        locals.var_tmf1_dn5 = assign65430_e102114_d_n5;
        locals.var_tmf1_dn6 = assign65430_e102114_d_n6;
        locals.var_tmf1_dn7 = assign65430_e102114_d_n7;
        locals.var_tmf1_dn8 = assign65430_e102114_d_n8;
        locals.var_tmf1_dn9 = assign65430_e102114_d_n9;
        locals.var_tmf1_dn10 = assign65430_e102114_d_n10;
        locals.var_tmf1_dn13 = assign65430_e102114_d_n13;

        let (assign65440_e102125, assign65440_e102125_d_n0, assign65440_e102125_d_n2, assign65440_e102125_d_n4, assign65440_e102125_d_n5, assign65440_e102125_d_n6, assign65440_e102125_d_n7, assign65440_e102125_d_n8, assign65440_e102125_d_n9, assign65440_e102125_d_n10, assign65440_e102125_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65440_e102122: f64 = (locals.var_t1 + locals.var_tmf1);
        let assign65440_e102123: f64 = (0.5 * assign65440_e102122);
        (assign65440_e102123, (0.5 * (locals.var_t1_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf1_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf1_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf1_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf1_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf1_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign65440_e102125;
        locals.var_t2_dn0 = assign65440_e102125_d_n0;
        locals.var_t2_dn2 = assign65440_e102125_d_n2;
        locals.var_t2_dn4 = assign65440_e102125_d_n4;
        locals.var_t2_dn5 = assign65440_e102125_d_n5;
        locals.var_t2_dn6 = assign65440_e102125_d_n6;
        locals.var_t2_dn7 = assign65440_e102125_d_n7;
        locals.var_t2_dn8 = assign65440_e102125_d_n8;
        locals.var_t2_dn9 = assign65440_e102125_d_n9;
        locals.var_t2_dn10 = assign65440_e102125_d_n10;
        locals.var_t2_dn13 = assign65440_e102125_d_n13;

        let (assign65450_e102138, assign65450_e102138_d_n0, assign65450_e102138_d_n2, assign65450_e102138_d_n4, assign65450_e102138_d_n5, assign65450_e102138_d_n6, assign65450_e102138_d_n7, assign65450_e102138_d_n8, assign65450_e102138_d_n9, assign65450_e102138_d_n10, assign65450_e102138_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65450_e102132: f64 = (locals.var_t2 / locals.var_wdep__blk1533);
        let assign65450_e102134: f64 = (assign65450_e102132 * locals.var_t2);
        let assign65450_e102136: f64 = (assign65450_e102134 / locals.var_wdep__blk1533);
        (assign65450_e102136, ((((((((locals.var_t2_dn0 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn0)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn0)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn0)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn2 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn2)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn2)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn2)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn4 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn4)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn4)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn4)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn5 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn5)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn5)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn5)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn6 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn6)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn6)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn6)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn7 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn7)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn7)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn7)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn8 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn8)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn8)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn8)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn9 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn9)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn9)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn9)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn10 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn10)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn10)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn10)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)), ((((((((locals.var_t2_dn13 * locals.var_wdep__blk1533) - (locals.var_t2 * locals.var_wdep__blk1533_dn13)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)) * locals.var_t2) + (assign65450_e102132 * locals.var_t2_dn13)) * locals.var_wdep__blk1533) - (assign65450_e102134 * locals.var_wdep__blk1533_dn13)) / (locals.var_wdep__blk1533 * locals.var_wdep__blk1533)),)
    } else {
        (locals.var_wfactor, locals.var_wfactor_dn0, locals.var_wfactor_dn2, locals.var_wfactor_dn4, locals.var_wfactor_dn5, locals.var_wfactor_dn6, locals.var_wfactor_dn7, locals.var_wfactor_dn8, locals.var_wfactor_dn9, locals.var_wfactor_dn10, locals.var_wfactor_dn13,)
    }
};
        locals.var_wfactor = assign65450_e102138;
        locals.var_wfactor_dn0 = assign65450_e102138_d_n0;
        locals.var_wfactor_dn2 = assign65450_e102138_d_n2;
        locals.var_wfactor_dn4 = assign65450_e102138_d_n4;
        locals.var_wfactor_dn5 = assign65450_e102138_d_n5;
        locals.var_wfactor_dn6 = assign65450_e102138_d_n6;
        locals.var_wfactor_dn7 = assign65450_e102138_d_n7;
        locals.var_wfactor_dn8 = assign65450_e102138_d_n8;
        locals.var_wfactor_dn9 = assign65450_e102138_d_n9;
        locals.var_wfactor_dn10 = assign65450_e102138_d_n10;
        locals.var_wfactor_dn13 = assign65450_e102138_d_n13;

        let (assign65460_e102151, assign65460_e102151_d_n0, assign65460_e102151_d_n2, assign65460_e102151_d_n4, assign65460_e102151_d_n5, assign65460_e102151_d_n6, assign65460_e102151_d_n7, assign65460_e102151_d_n8, assign65460_e102151_d_n9, assign65460_e102151_d_n10, assign65460_e102151_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65460_e102145: f64 = (locals.var_ps0__blk1523 - locals.var_dphi_vds);
        let assign65460_e102147: f64 = (assign65460_e102145 * locals.var_wfactor);
        let assign65460_e102149: f64 = (assign65460_e102147 + locals.var_dphi_vds);
        (assign65460_e102149, ((((locals.var_ps0__blk1523_dn0 - locals.var_dphi_vds_dn0) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn0)) + locals.var_dphi_vds_dn0), ((((locals.var_ps0__blk1523_dn2 - locals.var_dphi_vds_dn2) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn2)) + locals.var_dphi_vds_dn2), ((((locals.var_ps0__blk1523_dn4 - locals.var_dphi_vds_dn4) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn4)) + locals.var_dphi_vds_dn4), ((((locals.var_ps0__blk1523_dn5 - locals.var_dphi_vds_dn5) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn5)) + locals.var_dphi_vds_dn5), ((((locals.var_ps0__blk1523_dn6 - locals.var_dphi_vds_dn6) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn6)) + locals.var_dphi_vds_dn6), ((((locals.var_ps0__blk1523_dn7 - locals.var_dphi_vds_dn7) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn7)) + locals.var_dphi_vds_dn7), ((((locals.var_ps0__blk1523_dn8 - locals.var_dphi_vds_dn8) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn8)) + locals.var_dphi_vds_dn8), ((((locals.var_ps0__blk1523_dn9 - locals.var_dphi_vds_dn9) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn9)) + locals.var_dphi_vds_dn9), ((((locals.var_ps0__blk1523_dn10 - locals.var_dphi_vds_dn10) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn10)) + locals.var_dphi_vds_dn10), ((((locals.var_ps0__blk1523_dn13 - locals.var_dphi_vds_dn13) * locals.var_wfactor) + (assign65460_e102145 * locals.var_wfactor_dn13)) + locals.var_dphi_vds_dn13),)
    } else {
        (locals.var_phim, locals.var_phim_dn0, locals.var_phim_dn2, locals.var_phim_dn4, locals.var_phim_dn5, locals.var_phim_dn6, locals.var_phim_dn7, locals.var_phim_dn8, locals.var_phim_dn9, locals.var_phim_dn10, locals.var_phim_dn13,)
    }
};
        locals.var_phim = assign65460_e102151;
        locals.var_phim_dn0 = assign65460_e102151_d_n0;
        locals.var_phim_dn2 = assign65460_e102151_d_n2;
        locals.var_phim_dn4 = assign65460_e102151_d_n4;
        locals.var_phim_dn5 = assign65460_e102151_d_n5;
        locals.var_phim_dn6 = assign65460_e102151_d_n6;
        locals.var_phim_dn7 = assign65460_e102151_d_n7;
        locals.var_phim_dn8 = assign65460_e102151_d_n8;
        locals.var_phim_dn9 = assign65460_e102151_d_n9;
        locals.var_phim_dn10 = assign65460_e102151_d_n10;
        locals.var_phim_dn13 = assign65460_e102151_d_n13;

        let (assign65470_e102173, assign65470_e102173_d_n0, assign65470_e102173_d_n2, assign65470_e102173_d_n4, assign65470_e102173_d_n5, assign65470_e102173_d_n6, assign65470_e102173_d_n7, assign65470_e102173_d_n8, assign65470_e102173_d_n9, assign65470_e102173_d_n10, assign65470_e102173_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65470_e102160: f64 = (locals.var_vbipn - locals.var_vbscl__blk435);
        let assign65470_e102161: f64 = (locals.var_phim - assign65470_e102160);
        let assign65470_e102162: f64 = (locals.var_beta * assign65470_e102161);
        let assign65470_e102163: f64 = (assign65470_e102162).exp();
        let assign65470_e102166: f64 = (-locals.var_beta);
        let assign65470_e102168: f64 = (assign65470_e102166 * locals.var_vds);
        let assign65470_e102169: f64 = (assign65470_e102168).exp();
        let assign65470_e102170: f64 = (1.0 - assign65470_e102169);
        let assign65470_e102171: f64 = (assign65470_e102163 * assign65470_e102170);
        (assign65470_e102171, (((assign65470_e102163 * ((locals.var_beta_dn0 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn0 - (locals.var_vbipn_dn0 - locals.var_vbscl__blk435_dn0))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn0) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn0)))))), (((assign65470_e102163 * ((locals.var_beta_dn2 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn2 - (locals.var_vbipn_dn2 - locals.var_vbscl__blk435_dn2))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn2) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn2)))))), (((assign65470_e102163 * ((locals.var_beta_dn4 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn4 - (locals.var_vbipn_dn4 - locals.var_vbscl__blk435_dn4))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn4) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn4)))))), (((assign65470_e102163 * ((locals.var_beta_dn5 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn5 - (locals.var_vbipn_dn5 - locals.var_vbscl__blk435_dn5))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn5) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn5)))))), (((assign65470_e102163 * ((locals.var_beta_dn6 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn6 - (locals.var_vbipn_dn6 - locals.var_vbscl__blk435_dn6))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn6) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn6)))))), (((assign65470_e102163 * ((locals.var_beta_dn7 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn7 - (locals.var_vbipn_dn7 - locals.var_vbscl__blk435_dn7))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn7) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn7)))))), (((assign65470_e102163 * ((locals.var_beta_dn8 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn8 - (locals.var_vbipn_dn8 - locals.var_vbscl__blk435_dn8))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn8) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn8)))))), (((assign65470_e102163 * ((locals.var_beta_dn9 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn9 - (locals.var_vbipn_dn9 - locals.var_vbscl__blk435_dn9))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn9) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn9)))))), (((assign65470_e102163 * ((locals.var_beta_dn10 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn10 - (locals.var_vbipn_dn10 - locals.var_vbscl__blk435_dn10))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn10) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn10)))))), (((assign65470_e102163 * ((locals.var_beta_dn13 * assign65470_e102161) + (locals.var_beta * (locals.var_phim_dn13 - (locals.var_vbipn_dn13 - locals.var_vbscl__blk435_dn13))))) * assign65470_e102170) + (assign65470_e102163 * (-(assign65470_e102169 * (((-locals.var_beta_dn13) * locals.var_vds) + (assign65470_e102166 * locals.var_vds_dn13)))))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign65470_e102173;
        locals.var_ty_dn0 = assign65470_e102173_d_n0;
        locals.var_ty_dn2 = assign65470_e102173_d_n2;
        locals.var_ty_dn4 = assign65470_e102173_d_n4;
        locals.var_ty_dn5 = assign65470_e102173_d_n5;
        locals.var_ty_dn6 = assign65470_e102173_d_n6;
        locals.var_ty_dn7 = assign65470_e102173_d_n7;
        locals.var_ty_dn8 = assign65470_e102173_d_n8;
        locals.var_ty_dn9 = assign65470_e102173_d_n9;
        locals.var_ty_dn10 = assign65470_e102173_d_n10;
        locals.var_ty_dn13 = assign65470_e102173_d_n13;

        let (assign65480_e102187,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65480_e102180: f64 = (2.0 * 1.6021918e-19);
        let assign65480_e102182: f64 = (assign65480_e102180 * locals.var_uc_njunc);
        let assign65480_e102184: f64 = (assign65480_e102182 * 1.034943e-10);
        let assign65480_e102185: f64 = (assign65480_e102184).sqrt();
        (assign65480_e102185,)
    } else {
        (locals.var_conpt00,)
    }
};
        locals.var_conpt00 = assign65480_e102187;

        let (assign65490_e102197, assign65490_e102197_d_n0, assign65490_e102197_d_n2, assign65490_e102197_d_n4, assign65490_e102197_d_n5, assign65490_e102197_d_n6, assign65490_e102197_d_n7, assign65490_e102197_d_n8, assign65490_e102197_d_n9, assign65490_e102197_d_n10, assign65490_e102197_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65490_e102194: f64 = (locals.var_beta_inv).sqrt();
        let assign65490_e102195: f64 = (locals.var_conpt00 * assign65490_e102194);
        (assign65490_e102195, (locals.var_conpt00 * (locals.var_beta_inv_dn0 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn2 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn4 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn5 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn6 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn7 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn8 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn9 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn10 / (2.0 * assign65490_e102194))), (locals.var_conpt00 * (locals.var_beta_inv_dn13 / (2.0 * assign65490_e102194))),)
    } else {
        (locals.var_conpt0, locals.var_conpt0_dn0, locals.var_conpt0_dn2, locals.var_conpt0_dn4, locals.var_conpt0_dn5, locals.var_conpt0_dn6, locals.var_conpt0_dn7, locals.var_conpt0_dn8, locals.var_conpt0_dn9, locals.var_conpt0_dn10, locals.var_conpt0_dn13,)
    }
};
        locals.var_conpt0 = assign65490_e102197;
        locals.var_conpt0_dn0 = assign65490_e102197_d_n0;
        locals.var_conpt0_dn2 = assign65490_e102197_d_n2;
        locals.var_conpt0_dn4 = assign65490_e102197_d_n4;
        locals.var_conpt0_dn5 = assign65490_e102197_d_n5;
        locals.var_conpt0_dn6 = assign65490_e102197_d_n6;
        locals.var_conpt0_dn7 = assign65490_e102197_d_n7;
        locals.var_conpt0_dn8 = assign65490_e102197_d_n8;
        locals.var_conpt0_dn9 = assign65490_e102197_d_n9;
        locals.var_conpt0_dn10 = assign65490_e102197_d_n10;
        locals.var_conpt0_dn13 = assign65490_e102197_d_n13;

        let (assign65500_e102208, assign65500_e102208_d_n0, assign65500_e102208_d_n2, assign65500_e102208_d_n4, assign65500_e102208_d_n5, assign65500_e102208_d_n6, assign65500_e102208_d_n7, assign65500_e102208_d_n8, assign65500_e102208_d_n9, assign65500_e102208_d_n10, assign65500_e102208_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65500_e102205: f64 = (locals.var_phim - locals.var_dphi_vds);
        let assign65500_e102206: f64 = (locals.var_beta * assign65500_e102205);
        (assign65500_e102206, ((locals.var_beta_dn0 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn0 - locals.var_dphi_vds_dn0))), ((locals.var_beta_dn2 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn2 - locals.var_dphi_vds_dn2))), ((locals.var_beta_dn4 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn4 - locals.var_dphi_vds_dn4))), ((locals.var_beta_dn5 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn5 - locals.var_dphi_vds_dn5))), ((locals.var_beta_dn6 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn6 - locals.var_dphi_vds_dn6))), ((locals.var_beta_dn7 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn7 - locals.var_dphi_vds_dn7))), ((locals.var_beta_dn8 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn8 - locals.var_dphi_vds_dn8))), ((locals.var_beta_dn9 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn9 - locals.var_dphi_vds_dn9))), ((locals.var_beta_dn10 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn10 - locals.var_dphi_vds_dn10))), ((locals.var_beta_dn13 * assign65500_e102205) + (locals.var_beta * (locals.var_phim_dn13 - locals.var_dphi_vds_dn13))),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn4, locals.var_t1w_dn5, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn8, locals.var_t1w_dn9, locals.var_t1w_dn10, locals.var_t1w_dn13,)
    }
};
        locals.var_t1w = assign65500_e102208;
        locals.var_t1w_dn0 = assign65500_e102208_d_n0;
        locals.var_t1w_dn2 = assign65500_e102208_d_n2;
        locals.var_t1w_dn4 = assign65500_e102208_d_n4;
        locals.var_t1w_dn5 = assign65500_e102208_d_n5;
        locals.var_t1w_dn6 = assign65500_e102208_d_n6;
        locals.var_t1w_dn7 = assign65500_e102208_d_n7;
        locals.var_t1w_dn8 = assign65500_e102208_d_n8;
        locals.var_t1w_dn9 = assign65500_e102208_d_n9;
        locals.var_t1w_dn10 = assign65500_e102208_d_n10;
        locals.var_t1w_dn13 = assign65500_e102208_d_n13;

        let assign65510_e102213: f64 = (0.2 * locals.var_beta);
        let assign65510_e102214: f64 = assign65510_e102213;
        let assign65510_e102218: f64 = (0.2 * locals.var_beta);
        let assign65510_e102221: f64 = if ((locals.var_t1w < assign65510_e102214) && (assign65510_e102218 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1568 = assign65510_e102221;

        let (assign65520_e102236, assign65520_e102236_d_n0, assign65520_e102236_d_n2, assign65520_e102236_d_n4, assign65520_e102236_d_n5, assign65520_e102236_d_n6, assign65520_e102236_d_n7, assign65520_e102236_d_n8, assign65520_e102236_d_n9, assign65520_e102236_d_n10, assign65520_e102236_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65520_e102231: f64 = (0.2 * locals.var_beta);
        let assign65520_e102232: f64 = assign65520_e102231;
        let assign65520_e102234: f64 = (assign65520_e102232 - locals.var_t1w);
        (assign65520_e102234, ((0.2 * locals.var_beta_dn0) - locals.var_t1w_dn0), ((0.2 * locals.var_beta_dn2) - locals.var_t1w_dn2), ((0.2 * locals.var_beta_dn4) - locals.var_t1w_dn4), ((0.2 * locals.var_beta_dn5) - locals.var_t1w_dn5), ((0.2 * locals.var_beta_dn6) - locals.var_t1w_dn6), ((0.2 * locals.var_beta_dn7) - locals.var_t1w_dn7), ((0.2 * locals.var_beta_dn8) - locals.var_t1w_dn8), ((0.2 * locals.var_beta_dn9) - locals.var_t1w_dn9), ((0.2 * locals.var_beta_dn10) - locals.var_t1w_dn10), ((0.2 * locals.var_beta_dn13) - locals.var_t1w_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign65520_e102236;
        locals.var_tmf1_dn0 = assign65520_e102236_d_n0;
        locals.var_tmf1_dn2 = assign65520_e102236_d_n2;
        locals.var_tmf1_dn4 = assign65520_e102236_d_n4;
        locals.var_tmf1_dn5 = assign65520_e102236_d_n5;
        locals.var_tmf1_dn6 = assign65520_e102236_d_n6;
        locals.var_tmf1_dn7 = assign65520_e102236_d_n7;
        locals.var_tmf1_dn8 = assign65520_e102236_d_n8;
        locals.var_tmf1_dn9 = assign65520_e102236_d_n9;
        locals.var_tmf1_dn10 = assign65520_e102236_d_n10;
        locals.var_tmf1_dn13 = assign65520_e102236_d_n13;

        let (assign65530_e102247, assign65530_e102247_d_n0, assign65530_e102247_d_n2, assign65530_e102247_d_n4, assign65530_e102247_d_n5, assign65530_e102247_d_n6, assign65530_e102247_d_n7, assign65530_e102247_d_n8, assign65530_e102247_d_n9, assign65530_e102247_d_n10, assign65530_e102247_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65530_e102245: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign65530_e102245, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign65530_e102247;
        locals.var_x2_dn0 = assign65530_e102247_d_n0;
        locals.var_x2_dn2 = assign65530_e102247_d_n2;
        locals.var_x2_dn4 = assign65530_e102247_d_n4;
        locals.var_x2_dn5 = assign65530_e102247_d_n5;
        locals.var_x2_dn6 = assign65530_e102247_d_n6;
        locals.var_x2_dn7 = assign65530_e102247_d_n7;
        locals.var_x2_dn8 = assign65530_e102247_d_n8;
        locals.var_x2_dn9 = assign65530_e102247_d_n9;
        locals.var_x2_dn10 = assign65530_e102247_d_n10;
        locals.var_x2_dn13 = assign65530_e102247_d_n13;

    }
}
