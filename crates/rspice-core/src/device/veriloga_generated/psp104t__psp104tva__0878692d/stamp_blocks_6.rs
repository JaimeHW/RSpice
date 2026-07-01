#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_40(
        locals: &mut StampLocals,
    ) {
        let (assign49460_e63530, assign49460_e63530_d_n4, assign49460_e63530_d_n6, assign49460_e63530_d_n7, assign49460_e63530_d_n8, assign49460_e63530_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        let assign49460_e63528: f64 = (locals.var_sp_s_y0__blk1457).exp();
        (assign49460_e63528, (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn4), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn6), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn7), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn8), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn9),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49460_e63530;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49460_e63530_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49460_e63530_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49460_e63530_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49460_e63530_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49460_e63530_d_n9;
        locals.var_sp_s_delta0__blk1458_rv = 0.0;

        let (assign49470_e63566, assign49470_e63566_d_n4, assign49470_e63566_d_n6, assign49470_e63566_d_n7, assign49470_e63566_d_n8, assign49470_e63566_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 == 0.0)) {
        let assign49470_e63546: f64 = (locals.var_sp_s_y0__blk1457 - 230.25850929940458);
        let assign49470_e63551: f64 = (locals.var_sp_s_y0__blk1457 - 230.25850929940458);
        let assign49470_e63555: f64 = (locals.var_sp_s_y0__blk1457 - 230.25850929940458);
        let assign49470_e63557: f64 = (assign49470_e63555 * 0.3333333333333333);
        let assign49470_e63558: f64 = (1.0 + assign49470_e63557);
        let assign49470_e63559: f64 = (assign49470_e63551 * assign49470_e63558);
        let assign49470_e63560: f64 = (0.5 * assign49470_e63559);
        let assign49470_e63561: f64 = (1.0 + assign49470_e63560);
        let assign49470_e63562: f64 = (assign49470_e63546 * assign49470_e63561);
        let assign49470_e63563: f64 = (1.0 + assign49470_e63562);
        let assign49470_e63564: f64 = (1e100 * assign49470_e63563);
        (assign49470_e63564, (1e100 * ((locals.var_sp_s_y0__blk1457_dn4 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn4 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn6 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn6 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn7 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn7 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn8 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn8 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn9 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn9 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn9 * 0.3333333333333333))))))),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49470_e63566;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49470_e63566_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49470_e63566_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49470_e63566_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49470_e63566_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49470_e63566_d_n9;
        locals.var_sp_s_delta0__blk1458_rv = 0.0;

        let (assign49480_e63579, assign49480_e63579_d_n4, assign49480_e63579_d_n6, assign49480_e63579_d_n7, assign49480_e63579_d_n8, assign49480_e63579_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49480_e63577: f64 = (1.0 / locals.var_sp_s_delta0__blk1458);
        (assign49480_e63577, (-(locals.var_sp_s_delta0__blk1458_dn4 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn6 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn7 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn8 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn9 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign49480_e63579;
        locals.var_sp_s_delta1__blk1459_dn4 = assign49480_e63579_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign49480_e63579_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign49480_e63579_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign49480_e63579_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign49480_e63579_d_n9;
        locals.var_sp_s_delta1__blk1459_rv = 0.0;

        let (assign49490_e63596, assign49490_e63596_d_n4, assign49490_e63596_d_n6, assign49490_e63596_d_n7, assign49490_e63596_d_n8, assign49490_e63596_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49490_e63592: f64 = (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457);
        let assign49490_e63593: f64 = (2.0 + assign49490_e63592);
        let assign49490_e63594: f64 = (1.0 / assign49490_e63593);
        (assign49490_e63594, (-(((locals.var_sp_s_y0__blk1457_dn4 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn4)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn6 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn6)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn7 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn7)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn8 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn8)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn9 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn9)) / (assign49490_e63593 * assign49490_e63593))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49490_e63596;
        locals.var_sp_s_temp__blk1448_dn4 = assign49490_e63596_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49490_e63596_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49490_e63596_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49490_e63596_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49490_e63596_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign49500_e63611, assign49500_e63611_d_n4, assign49500_e63611_d_n6, assign49500_e63611_d_n7, assign49500_e63611_d_n8, assign49500_e63611_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49500_e63607: f64 = (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457);
        let assign49500_e63609: f64 = (assign49500_e63607 * locals.var_sp_s_temp__blk1448);
        (assign49500_e63609, ((((locals.var_sp_s_y0__blk1457_dn4 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn4)), ((((locals.var_sp_s_y0__blk1457_dn6 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn6)), ((((locals.var_sp_s_y0__blk1457_dn7 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn7)), ((((locals.var_sp_s_y0__blk1457_dn8 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn8)), ((((locals.var_sp_s_y0__blk1457_dn9 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9,)
    }
};
        locals.var_sp_s_xi0__blk1460 = assign49500_e63611;
        locals.var_sp_s_xi0__blk1460_dn4 = assign49500_e63611_d_n4;
        locals.var_sp_s_xi0__blk1460_dn6 = assign49500_e63611_d_n6;
        locals.var_sp_s_xi0__blk1460_dn7 = assign49500_e63611_d_n7;
        locals.var_sp_s_xi0__blk1460_dn8 = assign49500_e63611_d_n8;
        locals.var_sp_s_xi0__blk1460_dn9 = assign49500_e63611_d_n9;
        locals.var_sp_s_xi0__blk1460_rv = 0.0;

        let (assign49510_e63628, assign49510_e63628_d_n4, assign49510_e63628_d_n6, assign49510_e63628_d_n7, assign49510_e63628_d_n8, assign49510_e63628_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49510_e63623: f64 = (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448);
        let assign49510_e63625: f64 = (assign49510_e63623 * locals.var_sp_s_temp__blk1448);
        let assign49510_e63626: f64 = (4.0 * assign49510_e63625);
        (assign49510_e63626, (4.0 * ((((locals.var_sp_s_y0__blk1457_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn4))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn6))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn7))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn8))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn9))),)
    } else {
        (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9,)
    }
};
        locals.var_sp_s_xi1__blk1461 = assign49510_e63628;
        locals.var_sp_s_xi1__blk1461_dn4 = assign49510_e63628_d_n4;
        locals.var_sp_s_xi1__blk1461_dn6 = assign49510_e63628_d_n6;
        locals.var_sp_s_xi1__blk1461_dn7 = assign49510_e63628_d_n7;
        locals.var_sp_s_xi1__blk1461_dn8 = assign49510_e63628_d_n8;
        locals.var_sp_s_xi1__blk1461_dn9 = assign49510_e63628_d_n9;
        locals.var_sp_s_xi1__blk1461_rv = 0.0;

        let (assign49520_e63649, assign49520_e63649_d_n4, assign49520_e63649_d_n6, assign49520_e63649_d_n7, assign49520_e63649_d_n8, assign49520_e63649_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49520_e63639: f64 = (8.0 * locals.var_sp_s_temp__blk1448);
        let assign49520_e63642: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
        let assign49520_e63643: f64 = (assign49520_e63639 - assign49520_e63642);
        let assign49520_e63645: f64 = (assign49520_e63643 * locals.var_sp_s_temp__blk1448);
        let assign49520_e63647: f64 = (assign49520_e63645 * locals.var_sp_s_temp__blk1448);
        (assign49520_e63647, ((((((8.0 * locals.var_sp_s_temp__blk1448_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn4)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn8)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9,)
    }
};
        locals.var_sp_s_xi2__blk1462 = assign49520_e63649;
        locals.var_sp_s_xi2__blk1462_dn4 = assign49520_e63649_d_n4;
        locals.var_sp_s_xi2__blk1462_dn6 = assign49520_e63649_d_n6;
        locals.var_sp_s_xi2__blk1462_dn7 = assign49520_e63649_d_n7;
        locals.var_sp_s_xi2__blk1462_dn8 = assign49520_e63649_d_n8;
        locals.var_sp_s_xi2__blk1462_dn9 = assign49520_e63649_d_n9;
        locals.var_sp_s_xi2__blk1462_rv = 0.0;

        let (assign49530_e63662, assign49530_e63662_d_n4, assign49530_e63662_d_n6, assign49530_e63662_d_n7, assign49530_e63662_d_n8, assign49530_e63662_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49530_e63660: f64 = (locals.var_sp_s_yg__blk1451 - locals.var_sp_s_y0__blk1457);
        (assign49530_e63660, (locals.var_sp_s_yg__blk1451_dn4 - locals.var_sp_s_y0__blk1457_dn4), (locals.var_sp_s_yg__blk1451_dn6 - locals.var_sp_s_y0__blk1457_dn6), (locals.var_sp_s_yg__blk1451_dn7 - locals.var_sp_s_y0__blk1457_dn7), (locals.var_sp_s_yg__blk1451_dn8 - locals.var_sp_s_y0__blk1457_dn8), (locals.var_sp_s_yg__blk1451_dn9 - locals.var_sp_s_y0__blk1457_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49530_e63662;
        locals.var_sp_s_temp__blk1448_dn4 = assign49530_e63662_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49530_e63662_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49530_e63662_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49530_e63662_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49530_e63662_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign49540_e63675, assign49540_e63675_d_n4, assign49540_e63675_d_n6, assign49540_e63675_d_n7, assign49540_e63675_d_n8, assign49540_e63675_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49540_e63673: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459);
        (assign49540_e63673, ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn9)),)
    } else {
        (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9,)
    }
};
        locals.var_sp_s_temp1__blk1449 = assign49540_e63675;
        locals.var_sp_s_temp1__blk1449_dn4 = assign49540_e63675_d_n4;
        locals.var_sp_s_temp1__blk1449_dn6 = assign49540_e63675_d_n6;
        locals.var_sp_s_temp1__blk1449_dn7 = assign49540_e63675_d_n7;
        locals.var_sp_s_temp1__blk1449_dn8 = assign49540_e63675_d_n8;
        locals.var_sp_s_temp1__blk1449_dn9 = assign49540_e63675_d_n9;
        locals.var_sp_s_temp1__blk1449_rv = 0.0;

        let (assign49550_e63702, assign49550_e63702_d_n4, assign49550_e63702_d_n6, assign49550_e63702_d_n7, assign49550_e63702_d_n8, assign49550_e63702_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49550_e63686: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign49550_e63690: f64 = (locals.var_sp_s_delta0__blk1458 - 1.0);
        let assign49550_e63692: f64 = (assign49550_e63690 - locals.var_sp_s_temp1__blk1449);
        let assign49550_e63696: f64 = (1.0 - locals.var_sp_s_xi1__blk1461);
        let assign49550_e63697: f64 = (locals.var_delta_ns__blk1364 * assign49550_e63696);
        let assign49550_e63698: f64 = (assign49550_e63692 + assign49550_e63697);
        let assign49550_e63699: f64 = (locals.var_gf2__blk1325 * assign49550_e63698);
        let assign49550_e63700: f64 = (assign49550_e63686 + assign49550_e63699);
        (assign49550_e63700, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn4 - locals.var_sp_s_temp1__blk1449_dn4) + ((locals.var_delta_ns__blk1364_dn4 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn4))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn6 - locals.var_sp_s_temp1__blk1449_dn6) + ((locals.var_delta_ns__blk1364_dn6 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn6))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn7 - locals.var_sp_s_temp1__blk1449_dn7) + ((locals.var_delta_ns__blk1364_dn7 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn7))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn8 - locals.var_sp_s_temp1__blk1449_dn8) + ((locals.var_delta_ns__blk1364_dn8 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn8))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn9 - locals.var_sp_s_temp1__blk1449_dn9) + ((locals.var_delta_ns__blk1364_dn9 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn9))))))),)
    } else {
        (locals.var_sp_s_pc__blk1463, locals.var_sp_s_pc__blk1463_dn4, locals.var_sp_s_pc__blk1463_dn6, locals.var_sp_s_pc__blk1463_dn7, locals.var_sp_s_pc__blk1463_dn8, locals.var_sp_s_pc__blk1463_dn9,)
    }
};
        locals.var_sp_s_pc__blk1463 = assign49550_e63702;
        locals.var_sp_s_pc__blk1463_dn4 = assign49550_e63702_d_n4;
        locals.var_sp_s_pc__blk1463_dn6 = assign49550_e63702_d_n6;
        locals.var_sp_s_pc__blk1463_dn7 = assign49550_e63702_d_n7;
        locals.var_sp_s_pc__blk1463_dn8 = assign49550_e63702_d_n8;
        locals.var_sp_s_pc__blk1463_dn9 = assign49550_e63702_d_n9;
        locals.var_sp_s_pc__blk1463_rv = 0.0;

        let (assign49560_e63733, assign49560_e63733_d_n4, assign49560_e63733_d_n6, assign49560_e63733_d_n7, assign49560_e63733_d_n8, assign49560_e63733_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49560_e63713: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign49560_e63717: f64 = (locals.var_sp_s_delta0__blk1458 - locals.var_sp_s_y0__blk1457);
        let assign49560_e63719: f64 = (assign49560_e63717 - 1.0);
        let assign49560_e63721: f64 = (assign49560_e63719 + locals.var_sp_s_temp1__blk1449);
        let assign49560_e63725: f64 = (locals.var_sp_s_y0__blk1457 - 1.0);
        let assign49560_e63727: f64 = (assign49560_e63725 - locals.var_sp_s_xi0__blk1460);
        let assign49560_e63728: f64 = (locals.var_delta_ns__blk1364 * assign49560_e63727);
        let assign49560_e63729: f64 = (assign49560_e63721 + assign49560_e63728);
        let assign49560_e63730: f64 = (locals.var_gf2__blk1325 * assign49560_e63729);
        let assign49560_e63731: f64 = (assign49560_e63713 - assign49560_e63730);
        (assign49560_e63731, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn4 - locals.var_sp_s_y0__blk1457_dn4) + locals.var_sp_s_temp1__blk1449_dn4) + ((locals.var_delta_ns__blk1364_dn4 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn4 - locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn6 - locals.var_sp_s_y0__blk1457_dn6) + locals.var_sp_s_temp1__blk1449_dn6) + ((locals.var_delta_ns__blk1364_dn6 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn6 - locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn7 - locals.var_sp_s_y0__blk1457_dn7) + locals.var_sp_s_temp1__blk1449_dn7) + ((locals.var_delta_ns__blk1364_dn7 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn7 - locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn8 - locals.var_sp_s_y0__blk1457_dn8) + locals.var_sp_s_temp1__blk1449_dn8) + ((locals.var_delta_ns__blk1364_dn8 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn8 - locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn9 - locals.var_sp_s_y0__blk1457_dn9) + locals.var_sp_s_temp1__blk1449_dn9) + ((locals.var_delta_ns__blk1364_dn9 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn9 - locals.var_sp_s_xi0__blk1460_dn9))))))),)
    } else {
        (locals.var_sp_s_qc__blk1464, locals.var_sp_s_qc__blk1464_dn4, locals.var_sp_s_qc__blk1464_dn6, locals.var_sp_s_qc__blk1464_dn7, locals.var_sp_s_qc__blk1464_dn8, locals.var_sp_s_qc__blk1464_dn9,)
    }
};
        locals.var_sp_s_qc__blk1464 = assign49560_e63733;
        locals.var_sp_s_qc__blk1464_dn4 = assign49560_e63733_d_n4;
        locals.var_sp_s_qc__blk1464_dn6 = assign49560_e63733_d_n6;
        locals.var_sp_s_qc__blk1464_dn7 = assign49560_e63733_d_n7;
        locals.var_sp_s_qc__blk1464_dn8 = assign49560_e63733_d_n8;
        locals.var_sp_s_qc__blk1464_dn9 = assign49560_e63733_d_n9;
        locals.var_sp_s_qc__blk1464_rv = 0.0;

        let (assign49570_e63754, assign49570_e63754_d_n4, assign49570_e63754_d_n6, assign49570_e63754_d_n7, assign49570_e63754_d_n8, assign49570_e63754_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49570_e63746: f64 = (locals.var_sp_s_delta0__blk1458 + locals.var_sp_s_temp1__blk1449);
        let assign49570_e63749: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462);
        let assign49570_e63750: f64 = (assign49570_e63746 - assign49570_e63749);
        let assign49570_e63751: f64 = (locals.var_gf2__blk1325 * assign49570_e63750);
        let assign49570_e63752: f64 = (2.0 - assign49570_e63751);
        (assign49570_e63752, (-((locals.var_gf2__blk1325_dn4 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn4 + locals.var_sp_s_temp1__blk1449_dn4) - ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn6 + locals.var_sp_s_temp1__blk1449_dn6) - ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn7 + locals.var_sp_s_temp1__blk1449_dn7) - ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn8 + locals.var_sp_s_temp1__blk1449_dn8) - ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn9 + locals.var_sp_s_temp1__blk1449_dn9) - ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn9)))))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49570_e63754;
        locals.var_sp_s_temp__blk1448_dn4 = assign49570_e63754_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49570_e63754_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49570_e63754_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49570_e63754_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49570_e63754_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign49580_e63773, assign49580_e63773_d_n4, assign49580_e63773_d_n6, assign49580_e63773_d_n7, assign49580_e63773_d_n8, assign49580_e63773_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49580_e63765: f64 = (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463);
        let assign49580_e63769: f64 = (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448);
        let assign49580_e63770: f64 = (2.0 * assign49580_e63769);
        let assign49580_e63771: f64 = (assign49580_e63765 - assign49580_e63770);
        (assign49580_e63771, (((locals.var_sp_s_pc__blk1463_dn4 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn4)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn4)))), (((locals.var_sp_s_pc__blk1463_dn6 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn6)))), (((locals.var_sp_s_pc__blk1463_dn7 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn7)))), (((locals.var_sp_s_pc__blk1463_dn8 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn8)))), (((locals.var_sp_s_pc__blk1463_dn9 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn9)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn9)))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49580_e63773;
        locals.var_sp_s_temp__blk1448_dn4 = assign49580_e63773_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49580_e63773_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49580_e63773_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49580_e63773_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49580_e63773_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign49590_e63794, assign49590_e63794_d_n4, assign49590_e63794_d_n6, assign49590_e63794_d_n7, assign49590_e63794_d_n8, assign49590_e63794_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49590_e63783: f64 = (-locals.var_sp_s_y0__blk1457);
        let assign49590_e63788: f64 = (locals.var_sp_s_temp__blk1448).sqrt();
        let assign49590_e63789: f64 = (locals.var_sp_s_pc__blk1463 + assign49590_e63788);
        let assign49590_e63790: f64 = (locals.var_sp_s_qc__blk1464 / assign49590_e63789);
        let assign49590_e63791: f64 = (2.0 * assign49590_e63790);
        let assign49590_e63792: f64 = (assign49590_e63783 - assign49590_e63791);
        (assign49590_e63792, ((-locals.var_sp_s_y0__blk1457_dn4) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn4 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn4 + (locals.var_sp_s_temp__blk1448_dn4 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn6) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn6 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn6 + (locals.var_sp_s_temp__blk1448_dn6 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn7) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn7 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn7 + (locals.var_sp_s_temp__blk1448_dn7 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn8) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn8 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn8 + (locals.var_sp_s_temp__blk1448_dn8 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn9) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn9 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn9 + (locals.var_sp_s_temp__blk1448_dn9 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))),)
    } else {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    }
};
        locals.var_x_s__blk1363 = assign49590_e63794;
        locals.var_x_s__blk1363_dn4 = assign49590_e63794_d_n4;
        locals.var_x_s__blk1363_dn6 = assign49590_e63794_d_n6;
        locals.var_x_s__blk1363_dn7 = assign49590_e63794_d_n7;
        locals.var_x_s__blk1363_dn8 = assign49590_e63794_d_n8;
        locals.var_x_s__blk1363_dn9 = assign49590_e63794_d_n9;
        locals.var_x_s__blk1363_rv = 0.0;

        let (assign49600_e63812, assign49600_e63812_d_n4, assign49600_e63812_d_n6, assign49600_e63812_d_n7, assign49600_e63812_d_n8, assign49600_e63812_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49600_e63808: f64 = (locals.var_gf__blk1324 * 0.7324648775608221);
        let assign49600_e63809: f64 = (1.25 + assign49600_e63808);
        let assign49600_e63810: f64 = (1.0 / assign49600_e63809);
        (assign49600_e63810, (-((locals.var_gf__blk1324_dn4 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn6 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn7 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn8 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn9 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))),)
    } else {
        (locals.var_sp_xg1__blk1465, locals.var_sp_xg1__blk1465_dn4, locals.var_sp_xg1__blk1465_dn6, locals.var_sp_xg1__blk1465_dn7, locals.var_sp_xg1__blk1465_dn8, locals.var_sp_xg1__blk1465_dn9,)
    }
};
        locals.var_sp_xg1__blk1465 = assign49600_e63812;
        locals.var_sp_xg1__blk1465_dn4 = assign49600_e63812_d_n4;
        locals.var_sp_xg1__blk1465_dn6 = assign49600_e63812_d_n6;
        locals.var_sp_xg1__blk1465_dn7 = assign49600_e63812_d_n7;
        locals.var_sp_xg1__blk1465_dn8 = assign49600_e63812_d_n8;
        locals.var_sp_xg1__blk1465_dn9 = assign49600_e63812_d_n9;
        locals.var_sp_xg1__blk1465_rv = 0.0;

        let (assign49610_e63832, assign49610_e63832_d_n4, assign49610_e63832_d_n6, assign49610_e63832_d_n7, assign49610_e63832_d_n8, assign49610_e63832_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49610_e63824: f64 = (locals.var_xi__blk1360 * 1.25);
        let assign49610_e63826: f64 = (assign49610_e63824 * locals.var_sp_xg1__blk1465);
        let assign49610_e63828: f64 = (assign49610_e63826 - 1.0);
        let assign49610_e63830: f64 = (assign49610_e63828 * locals.var_sp_xg1__blk1465);
        (assign49610_e63830, (((((locals.var_xi__blk1360_dn4 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn4)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn4)), (((((locals.var_xi__blk1360_dn6 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn6)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn6)), (((((locals.var_xi__blk1360_dn7 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn7)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn7)), (((((locals.var_xi__blk1360_dn8 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn8)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn8)), (((((locals.var_xi__blk1360_dn9 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn9)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn9)),)
    } else {
        (locals.var_sp_s_a_fac__blk1466, locals.var_sp_s_a_fac__blk1466_dn4, locals.var_sp_s_a_fac__blk1466_dn6, locals.var_sp_s_a_fac__blk1466_dn7, locals.var_sp_s_a_fac__blk1466_dn8, locals.var_sp_s_a_fac__blk1466_dn9,)
    }
};
        locals.var_sp_s_a_fac__blk1466 = assign49610_e63832;
        locals.var_sp_s_a_fac__blk1466_dn4 = assign49610_e63832_d_n4;
        locals.var_sp_s_a_fac__blk1466_dn6 = assign49610_e63832_d_n6;
        locals.var_sp_s_a_fac__blk1466_dn7 = assign49610_e63832_d_n7;
        locals.var_sp_s_a_fac__blk1466_dn8 = assign49610_e63832_d_n8;
        locals.var_sp_s_a_fac__blk1466_dn9 = assign49610_e63832_d_n9;
        locals.var_sp_s_a_fac__blk1466_rv = 0.0;

        let (assign49620_e63852, assign49620_e63852_d_n4, assign49620_e63852_d_n6, assign49620_e63852_d_n7, assign49620_e63852_d_n8, assign49620_e63852_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49620_e63844: f64 = (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362);
        let assign49620_e63848: f64 = (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343);
        let assign49620_e63849: f64 = (1.0 + assign49620_e63848);
        let assign49620_e63850: f64 = (assign49620_e63844 * assign49620_e63849);
        (assign49620_e63850, ((((locals.var_xg__blk1343_dn4 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn4)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn4 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn4)))), ((((locals.var_xg__blk1343_dn6 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn6)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn6 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn6)))), ((((locals.var_xg__blk1343_dn7 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn7)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn7 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn7)))), ((((locals.var_xg__blk1343_dn8 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn8)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn8 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn8)))), ((((locals.var_xg__blk1343_dn9 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn9)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn9 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn9)))),)
    } else {
        (locals.var_sp_s_xbar__blk1467, locals.var_sp_s_xbar__blk1467_dn4, locals.var_sp_s_xbar__blk1467_dn6, locals.var_sp_s_xbar__blk1467_dn7, locals.var_sp_s_xbar__blk1467_dn8, locals.var_sp_s_xbar__blk1467_dn9,)
    }
};
        locals.var_sp_s_xbar__blk1467 = assign49620_e63852;
        locals.var_sp_s_xbar__blk1467_dn4 = assign49620_e63852_d_n4;
        locals.var_sp_s_xbar__blk1467_dn6 = assign49620_e63852_d_n6;
        locals.var_sp_s_xbar__blk1467_dn7 = assign49620_e63852_d_n7;
        locals.var_sp_s_xbar__blk1467_dn8 = assign49620_e63852_d_n8;
        locals.var_sp_s_xbar__blk1467_dn9 = assign49620_e63852_d_n9;
        locals.var_sp_s_xbar__blk1467_rv = 0.0;

        let assign49630_e63854: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49630_e63856: f64 = (-230.25850929940458);
        let assign49630_e63857: f64 = if assign49630_e63854 > assign49630_e63856 { 1.0 } else { 0.0 };
        locals.var_guard1488 = assign49630_e63857;
        locals.var_guard1488_rv = 0.0;

        let (assign49640_e63873, assign49640_e63873_d_n4, assign49640_e63873_d_n6, assign49640_e63873_d_n7, assign49640_e63873_d_n8, assign49640_e63873_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign49640_e63870: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49640_e63871: f64 = (assign49640_e63870).exp();
        (assign49640_e63871, (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn4)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn6)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn7)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn8)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn9)),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49640_e63873;
        locals.var_sp_s_temp__blk1448_dn4 = assign49640_e63873_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49640_e63873_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49640_e63873_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49640_e63873_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49640_e63873_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign49650_e63916, assign49650_e63916_d_n4, assign49650_e63916_d_n6, assign49650_e63916_d_n7, assign49650_e63916_d_n8, assign49650_e63916_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1488 == 0.0)) {
        let assign49650_e63889: f64 = (-230.25850929940458);
        let assign49650_e63891: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49650_e63892: f64 = (assign49650_e63889 - assign49650_e63891);
        let assign49650_e63896: f64 = (-230.25850929940458);
        let assign49650_e63898: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49650_e63899: f64 = (assign49650_e63896 - assign49650_e63898);
        let assign49650_e63902: f64 = (-230.25850929940458);
        let assign49650_e63904: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49650_e63905: f64 = (assign49650_e63902 - assign49650_e63904);
        let assign49650_e63907: f64 = (assign49650_e63905 * 0.3333333333333333);
        let assign49650_e63908: f64 = (1.0 + assign49650_e63907);
        let assign49650_e63909: f64 = (assign49650_e63899 * assign49650_e63908);
        let assign49650_e63910: f64 = (0.5 * assign49650_e63909);
        let assign49650_e63911: f64 = (1.0 + assign49650_e63910);
        let assign49650_e63912: f64 = (assign49650_e63892 * assign49650_e63911);
        let assign49650_e63913: f64 = (1.0 + assign49650_e63912);
        let assign49650_e63914: f64 = (1e-100 / assign49650_e63913);
        (assign49650_e63914, (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn4)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn4)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn4)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn6)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn6)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn6)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn7)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn7)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn7)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn8)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn8)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn8)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn9)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn9)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn9)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49650_e63916;
        locals.var_sp_s_temp__blk1448_dn4 = assign49650_e63916_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49650_e63916_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49650_e63916_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49650_e63916_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49650_e63916_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign49660_e63930, assign49660_e63930_d_n4, assign49660_e63930_d_n6, assign49660_e63930_d_n7, assign49660_e63930_d_n8, assign49660_e63930_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49660_e63928: f64 = (1.0 - locals.var_sp_s_temp__blk1448);
        (assign49660_e63928, (-locals.var_sp_s_temp__blk1448_dn4), (-locals.var_sp_s_temp__blk1448_dn6), (-locals.var_sp_s_temp__blk1448_dn7), (-locals.var_sp_s_temp__blk1448_dn8), (-locals.var_sp_s_temp__blk1448_dn9),)
    } else {
        (locals.var_sp_s_w__blk1468, locals.var_sp_s_w__blk1468_dn4, locals.var_sp_s_w__blk1468_dn6, locals.var_sp_s_w__blk1468_dn7, locals.var_sp_s_w__blk1468_dn8, locals.var_sp_s_w__blk1468_dn9,)
    }
};
        locals.var_sp_s_w__blk1468 = assign49660_e63930;
        locals.var_sp_s_w__blk1468_dn4 = assign49660_e63930_d_n4;
        locals.var_sp_s_w__blk1468_dn6 = assign49660_e63930_d_n6;
        locals.var_sp_s_w__blk1468_dn7 = assign49660_e63930_d_n7;
        locals.var_sp_s_w__blk1468_dn8 = assign49660_e63930_d_n8;
        locals.var_sp_s_w__blk1468_dn9 = assign49660_e63930_d_n9;
        locals.var_sp_s_w__blk1468_rv = 0.0;

        let (assign49670_e63957, assign49670_e63957_d_n4, assign49670_e63957_d_n6, assign49670_e63957_d_n7, assign49670_e63957_d_n8, assign49670_e63957_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49670_e63943: f64 = (locals.var_gf2__blk1325 * 0.5);
        let assign49670_e63944: f64 = (locals.var_xg__blk1343 + assign49670_e63943);
        let assign49670_e63949: f64 = (locals.var_gf2__blk1325 * 0.25);
        let assign49670_e63950: f64 = (locals.var_xg__blk1343 + assign49670_e63949);
        let assign49670_e63952: f64 = (assign49670_e63950 - locals.var_sp_s_w__blk1468);
        let assign49670_e63953: f64 = (assign49670_e63952).sqrt();
        let assign49670_e63954: f64 = (locals.var_gf__blk1324 * assign49670_e63953);
        let assign49670_e63955: f64 = (assign49670_e63944 - assign49670_e63954);
        (assign49670_e63955, ((locals.var_xg__blk1343_dn4 + (locals.var_gf2__blk1325_dn4 * 0.5)) - ((locals.var_gf__blk1324_dn4 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn4 + (locals.var_gf2__blk1325_dn4 * 0.25)) - locals.var_sp_s_w__blk1468_dn4) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn6 + (locals.var_gf2__blk1325_dn6 * 0.5)) - ((locals.var_gf__blk1324_dn6 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn6 + (locals.var_gf2__blk1325_dn6 * 0.25)) - locals.var_sp_s_w__blk1468_dn6) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn7 + (locals.var_gf2__blk1325_dn7 * 0.5)) - ((locals.var_gf__blk1324_dn7 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn7 + (locals.var_gf2__blk1325_dn7 * 0.25)) - locals.var_sp_s_w__blk1468_dn7) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn8 + (locals.var_gf2__blk1325_dn8 * 0.5)) - ((locals.var_gf__blk1324_dn8 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn8 + (locals.var_gf2__blk1325_dn8 * 0.25)) - locals.var_sp_s_w__blk1468_dn8) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn9 + (locals.var_gf2__blk1325_dn9 * 0.5)) - ((locals.var_gf__blk1324_dn9 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn9 + (locals.var_gf2__blk1325_dn9 * 0.25)) - locals.var_sp_s_w__blk1468_dn9) / (2.0 * assign49670_e63953))))),)
    } else {
        (locals.var_sp_s_x1__blk1469, locals.var_sp_s_x1__blk1469_dn4, locals.var_sp_s_x1__blk1469_dn6, locals.var_sp_s_x1__blk1469_dn7, locals.var_sp_s_x1__blk1469_dn8, locals.var_sp_s_x1__blk1469_dn9,)
    }
};
        locals.var_sp_s_x1__blk1469 = assign49670_e63957;
        locals.var_sp_s_x1__blk1469_dn4 = assign49670_e63957_d_n4;
        locals.var_sp_s_x1__blk1469_dn6 = assign49670_e63957_d_n6;
        locals.var_sp_s_x1__blk1469_dn7 = assign49670_e63957_d_n7;
        locals.var_sp_s_x1__blk1469_dn8 = assign49670_e63957_d_n8;
        locals.var_sp_s_x1__blk1469_dn9 = assign49670_e63957_d_n9;
        locals.var_sp_s_x1__blk1469_rv = 0.0;

        let (assign49680_e63971, assign49680_e63971_d_n4, assign49680_e63971_d_n6, assign49680_e63971_d_n7, assign49680_e63971_d_n8, assign49680_e63971_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49680_e63969: f64 = (locals.var_xn_s__blk1349 + 3.0);
        (assign49680_e63969, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9,)
    } else {
        (locals.var_sp_s_bx__blk1470, locals.var_sp_s_bx__blk1470_dn4, locals.var_sp_s_bx__blk1470_dn6, locals.var_sp_s_bx__blk1470_dn7, locals.var_sp_s_bx__blk1470_dn8, locals.var_sp_s_bx__blk1470_dn9,)
    }
};
        locals.var_sp_s_bx__blk1470 = assign49680_e63971;
        locals.var_sp_s_bx__blk1470_dn4 = assign49680_e63971_d_n4;
        locals.var_sp_s_bx__blk1470_dn6 = assign49680_e63971_d_n6;
        locals.var_sp_s_bx__blk1470_dn7 = assign49680_e63971_d_n7;
        locals.var_sp_s_bx__blk1470_dn8 = assign49680_e63971_d_n8;
        locals.var_sp_s_bx__blk1470_dn9 = assign49680_e63971_d_n9;
        locals.var_sp_s_bx__blk1470_rv = 0.0;

        let (assign49690_e64009, assign49690_e64009_d_n4, assign49690_e64009_d_n6, assign49690_e64009_d_n7, assign49690_e64009_d_n8, assign49690_e64009_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49690_e63984: f64 = (locals.var_sp_s_x1__blk1469 + locals.var_sp_s_bx__blk1470);
        let assign49690_e63987: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
        let assign49690_e63990: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
        let assign49690_e63991: f64 = (assign49690_e63987 * assign49690_e63990);
        let assign49690_e63993: f64 = (assign49690_e63991 + 5.0);
        let assign49690_e63994: f64 = (assign49690_e63993).sqrt();
        let assign49690_e63995: f64 = (assign49690_e63984 - assign49690_e63994);
        let assign49690_e63996: f64 = (0.5 * assign49690_e63995);
        let assign49690_e64001: f64 = (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470);
        let assign49690_e64003: f64 = (assign49690_e64001 + 5.0);
        let assign49690_e64004: f64 = (assign49690_e64003).sqrt();
        let assign49690_e64005: f64 = (locals.var_sp_s_bx__blk1470 - assign49690_e64004);
        let assign49690_e64006: f64 = (0.5 * assign49690_e64005);
        let assign49690_e64007: f64 = (assign49690_e63996 - assign49690_e64006);
        (assign49690_e64007, ((0.5 * ((locals.var_sp_s_x1__blk1469_dn4 + locals.var_sp_s_bx__blk1470_dn4) - ((((locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn4 - (((locals.var_sp_s_bx__blk1470_dn4 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn4)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn6 + locals.var_sp_s_bx__blk1470_dn6) - ((((locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn6 - (((locals.var_sp_s_bx__blk1470_dn6 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn6)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn7 + locals.var_sp_s_bx__blk1470_dn7) - ((((locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn7 - (((locals.var_sp_s_bx__blk1470_dn7 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn7)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn8 + locals.var_sp_s_bx__blk1470_dn8) - ((((locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn8 - (((locals.var_sp_s_bx__blk1470_dn8 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn8)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn9 + locals.var_sp_s_bx__blk1470_dn9) - ((((locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn9 - (((locals.var_sp_s_bx__blk1470_dn9 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn9)) / (2.0 * assign49690_e64004))))),)
    } else {
        (locals.var_sp_s_eta__blk1453, locals.var_sp_s_eta__blk1453_dn4, locals.var_sp_s_eta__blk1453_dn6, locals.var_sp_s_eta__blk1453_dn7, locals.var_sp_s_eta__blk1453_dn8, locals.var_sp_s_eta__blk1453_dn9,)
    }
};
        locals.var_sp_s_eta__blk1453 = assign49690_e64009;
        locals.var_sp_s_eta__blk1453_dn4 = assign49690_e64009_d_n4;
        locals.var_sp_s_eta__blk1453_dn6 = assign49690_e64009_d_n6;
        locals.var_sp_s_eta__blk1453_dn7 = assign49690_e64009_d_n7;
        locals.var_sp_s_eta__blk1453_dn8 = assign49690_e64009_d_n8;
        locals.var_sp_s_eta__blk1453_dn9 = assign49690_e64009_d_n9;
        locals.var_sp_s_eta__blk1453_rv = 0.0;

        let (assign49700_e64023, assign49700_e64023_d_n4, assign49700_e64023_d_n6, assign49700_e64023_d_n7, assign49700_e64023_d_n8, assign49700_e64023_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49700_e64021: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_eta__blk1453);
        (assign49700_e64021, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_eta__blk1453_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_eta__blk1453_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_eta__blk1453_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_eta__blk1453_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_eta__blk1453_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49700_e64023;
        locals.var_sp_s_temp__blk1448_dn4 = assign49700_e64023_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49700_e64023_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49700_e64023_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49700_e64023_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49700_e64023_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign49710_e64037, assign49710_e64037_d_n4, assign49710_e64037_d_n6, assign49710_e64037_d_n7, assign49710_e64037_d_n8, assign49710_e64037_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49710_e64034: f64 = (-locals.var_sp_s_eta__blk1453);
        let assign49710_e64035: f64 = (assign49710_e64034).exp();
        (assign49710_e64035, (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn4)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn6)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn7)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn8)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn9)),)
    } else {
        (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9,)
    }
};
        locals.var_sp_s_temp1__blk1449 = assign49710_e64037;
        locals.var_sp_s_temp1__blk1449_dn4 = assign49710_e64037_d_n4;
        locals.var_sp_s_temp1__blk1449_dn6 = assign49710_e64037_d_n6;
        locals.var_sp_s_temp1__blk1449_dn7 = assign49710_e64037_d_n7;
        locals.var_sp_s_temp1__blk1449_dn8 = assign49710_e64037_d_n8;
        locals.var_sp_s_temp1__blk1449_dn9 = assign49710_e64037_d_n9;
        locals.var_sp_s_temp1__blk1449_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_41(
        locals: &mut StampLocals,
    ) {
        let (assign49720_e64055, assign49720_e64055_d_n4, assign49720_e64055_d_n6, assign49720_e64055_d_n7, assign49720_e64055_d_n8, assign49720_e64055_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49720_e64051: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
        let assign49720_e64052: f64 = (2.0 + assign49720_e64051);
        let assign49720_e64053: f64 = (1.0 / assign49720_e64052);
        (assign49720_e64053, (-(((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) / (assign49720_e64052 * assign49720_e64052))),)
    } else {
        (locals.var_sp_s_temp2__blk1450, locals.var_sp_s_temp2__blk1450_dn4, locals.var_sp_s_temp2__blk1450_dn6, locals.var_sp_s_temp2__blk1450_dn7, locals.var_sp_s_temp2__blk1450_dn8, locals.var_sp_s_temp2__blk1450_dn9,)
    }
};
        locals.var_sp_s_temp2__blk1450 = assign49720_e64055;
        locals.var_sp_s_temp2__blk1450_dn4 = assign49720_e64055_d_n4;
        locals.var_sp_s_temp2__blk1450_dn6 = assign49720_e64055_d_n6;
        locals.var_sp_s_temp2__blk1450_dn7 = assign49720_e64055_d_n7;
        locals.var_sp_s_temp2__blk1450_dn8 = assign49720_e64055_d_n8;
        locals.var_sp_s_temp2__blk1450_dn9 = assign49720_e64055_d_n9;
        locals.var_sp_s_temp2__blk1450_rv = 0.0;

        let (assign49730_e64071, assign49730_e64071_d_n4, assign49730_e64071_d_n6, assign49730_e64071_d_n7, assign49730_e64071_d_n8, assign49730_e64071_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49730_e64067: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
        let assign49730_e64069: f64 = (assign49730_e64067 * locals.var_sp_s_temp2__blk1450);
        (assign49730_e64069, ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn4)), ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn6)), ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn7)), ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn8)), ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn9)),)
    } else {
        (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9,)
    }
};
        locals.var_sp_s_xi0__blk1460 = assign49730_e64071;
        locals.var_sp_s_xi0__blk1460_dn4 = assign49730_e64071_d_n4;
        locals.var_sp_s_xi0__blk1460_dn6 = assign49730_e64071_d_n6;
        locals.var_sp_s_xi0__blk1460_dn7 = assign49730_e64071_d_n7;
        locals.var_sp_s_xi0__blk1460_dn8 = assign49730_e64071_d_n8;
        locals.var_sp_s_xi0__blk1460_dn9 = assign49730_e64071_d_n9;
        locals.var_sp_s_xi0__blk1460_rv = 0.0;

        let (assign49740_e64089, assign49740_e64089_d_n4, assign49740_e64089_d_n6, assign49740_e64089_d_n7, assign49740_e64089_d_n8, assign49740_e64089_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49740_e64084: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450);
        let assign49740_e64086: f64 = (assign49740_e64084 * locals.var_sp_s_temp2__blk1450);
        let assign49740_e64087: f64 = (4.0 * assign49740_e64086);
        (assign49740_e64087, (4.0 * ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn4))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn8))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn9))),)
    } else {
        (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9,)
    }
};
        locals.var_sp_s_xi1__blk1461 = assign49740_e64089;
        locals.var_sp_s_xi1__blk1461_dn4 = assign49740_e64089_d_n4;
        locals.var_sp_s_xi1__blk1461_dn6 = assign49740_e64089_d_n6;
        locals.var_sp_s_xi1__blk1461_dn7 = assign49740_e64089_d_n7;
        locals.var_sp_s_xi1__blk1461_dn8 = assign49740_e64089_d_n8;
        locals.var_sp_s_xi1__blk1461_dn9 = assign49740_e64089_d_n9;
        locals.var_sp_s_xi1__blk1461_rv = 0.0;

        let (assign49750_e64111, assign49750_e64111_d_n4, assign49750_e64111_d_n6, assign49750_e64111_d_n7, assign49750_e64111_d_n8, assign49750_e64111_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49750_e64101: f64 = (8.0 * locals.var_sp_s_temp2__blk1450);
        let assign49750_e64104: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
        let assign49750_e64105: f64 = (assign49750_e64101 - assign49750_e64104);
        let assign49750_e64107: f64 = (assign49750_e64105 * locals.var_sp_s_temp2__blk1450);
        let assign49750_e64109: f64 = (assign49750_e64107 * locals.var_sp_s_temp2__blk1450);
        (assign49750_e64109, ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn4)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn8)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn9)),)
    } else {
        (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9,)
    }
};
        locals.var_sp_s_xi2__blk1462 = assign49750_e64111;
        locals.var_sp_s_xi2__blk1462_dn4 = assign49750_e64111_d_n4;
        locals.var_sp_s_xi2__blk1462_dn6 = assign49750_e64111_d_n6;
        locals.var_sp_s_xi2__blk1462_dn7 = assign49750_e64111_d_n7;
        locals.var_sp_s_xi2__blk1462_dn8 = assign49750_e64111_d_n8;
        locals.var_sp_s_xi2__blk1462_dn9 = assign49750_e64111_d_n9;
        locals.var_sp_s_xi2__blk1462_rv = 0.0;

        let (assign49760_e64164, assign49760_e64164_d_n4, assign49760_e64164_d_n6, assign49760_e64164_d_n7, assign49760_e64164_d_n8, assign49760_e64164_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49760_e64124: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign49760_e64128: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
        let assign49760_e64130: f64 = (assign49760_e64128 - 1.0);
        let assign49760_e64134: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
        let assign49760_e64136: f64 = (assign49760_e64134 + locals.var_sp_s_xi0__blk1460);
        let assign49760_e64137: f64 = (locals.var_delta_ns__blk1364 * assign49760_e64136);
        let assign49760_e64138: f64 = (assign49760_e64130 - assign49760_e64137);
        let assign49760_e64139: f64 = (locals.var_gf2__blk1325 * assign49760_e64138);
        let assign49760_e64140: f64 = (assign49760_e64124 - assign49760_e64139);
        let (assign49760_e64162, assign49760_e64162_d_n4, assign49760_e64162_d_n6, assign49760_e64162_d_n7, assign49760_e64162_d_n8, assign49760_e64162_d_n9,) = {
            if (1e-40 > assign49760_e64140) {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign49760_e64145: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
                let assign49760_e64149: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
                let assign49760_e64151: f64 = (assign49760_e64149 - 1.0);
                let assign49760_e64155: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
                let assign49760_e64157: f64 = (assign49760_e64155 + locals.var_sp_s_xi0__blk1460);
                let assign49760_e64158: f64 = (locals.var_delta_ns__blk1364 * assign49760_e64157);
                let assign49760_e64159: f64 = (assign49760_e64151 - assign49760_e64158);
                let assign49760_e64160: f64 = (locals.var_gf2__blk1325 * assign49760_e64159);
                let assign49760_e64161: f64 = (assign49760_e64145 - assign49760_e64160);
                (assign49760_e64161, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn4 + locals.var_sp_s_eta__blk1453_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn6 + locals.var_sp_s_eta__blk1453_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn7 + locals.var_sp_s_eta__blk1453_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn8 + locals.var_sp_s_eta__blk1453_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn9 + locals.var_sp_s_eta__blk1453_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))),)
            }
        };
        (assign49760_e64162, assign49760_e64162_d_n4, assign49760_e64162_d_n6, assign49760_e64162_d_n7, assign49760_e64162_d_n8, assign49760_e64162_d_n9,)
    } else {
        (locals.var_sp_s_a__blk1454, locals.var_sp_s_a__blk1454_dn4, locals.var_sp_s_a__blk1454_dn6, locals.var_sp_s_a__blk1454_dn7, locals.var_sp_s_a__blk1454_dn8, locals.var_sp_s_a__blk1454_dn9,)
    }
};
        locals.var_sp_s_a__blk1454 = assign49760_e64164;
        locals.var_sp_s_a__blk1454_dn4 = assign49760_e64164_d_n4;
        locals.var_sp_s_a__blk1454_dn6 = assign49760_e64164_d_n6;
        locals.var_sp_s_a__blk1454_dn7 = assign49760_e64164_d_n7;
        locals.var_sp_s_a__blk1454_dn8 = assign49760_e64164_d_n8;
        locals.var_sp_s_a__blk1454_dn9 = assign49760_e64164_d_n9;
        locals.var_sp_s_a__blk1454_rv = 0.0;

        let (assign49770_e64186, assign49770_e64186_d_n4, assign49770_e64186_d_n6, assign49770_e64186_d_n7, assign49770_e64186_d_n8, assign49770_e64186_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49770_e64180: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462);
        let assign49770_e64181: f64 = (locals.var_sp_s_temp1__blk1449 - assign49770_e64180);
        let assign49770_e64182: f64 = (locals.var_gf2__blk1325 * assign49770_e64181);
        let assign49770_e64183: f64 = (0.5 * assign49770_e64182);
        let assign49770_e64184: f64 = (1.0 - assign49770_e64183);
        (assign49770_e64184, (-(0.5 * ((locals.var_gf2__blk1325_dn4 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn4 - ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn4))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn6 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn6 - ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn7 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn7 - ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn8 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn8 - ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn8))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn9 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn9 - ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn9))))))),)
    } else {
        (locals.var_sp_s_b__blk1471, locals.var_sp_s_b__blk1471_dn4, locals.var_sp_s_b__blk1471_dn6, locals.var_sp_s_b__blk1471_dn7, locals.var_sp_s_b__blk1471_dn8, locals.var_sp_s_b__blk1471_dn9,)
    }
};
        locals.var_sp_s_b__blk1471 = assign49770_e64186;
        locals.var_sp_s_b__blk1471_dn4 = assign49770_e64186_d_n4;
        locals.var_sp_s_b__blk1471_dn6 = assign49770_e64186_d_n6;
        locals.var_sp_s_b__blk1471_dn7 = assign49770_e64186_d_n7;
        locals.var_sp_s_b__blk1471_dn8 = assign49770_e64186_d_n8;
        locals.var_sp_s_b__blk1471_dn9 = assign49770_e64186_d_n9;
        locals.var_sp_s_b__blk1471_rv = 0.0;

        let (assign49780_e64212, assign49780_e64212_d_n4, assign49780_e64212_d_n6, assign49780_e64212_d_n7, assign49780_e64212_d_n8, assign49780_e64212_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49780_e64198: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign49780_e64202: f64 = (1.0 - locals.var_sp_s_temp1__blk1449);
        let assign49780_e64206: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
        let assign49780_e64207: f64 = (locals.var_delta_ns__blk1364 * assign49780_e64206);
        let assign49780_e64208: f64 = (assign49780_e64202 - assign49780_e64207);
        let assign49780_e64209: f64 = (locals.var_gf2__blk1325 * assign49780_e64208);
        let assign49780_e64210: f64 = (assign49780_e64198 + assign49780_e64209);
        (assign49780_e64210, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn9)))))),)
    } else {
        (locals.var_sp_s_c__blk1455, locals.var_sp_s_c__blk1455_dn4, locals.var_sp_s_c__blk1455_dn6, locals.var_sp_s_c__blk1455_dn7, locals.var_sp_s_c__blk1455_dn8, locals.var_sp_s_c__blk1455_dn9,)
    }
};
        locals.var_sp_s_c__blk1455 = assign49780_e64212;
        locals.var_sp_s_c__blk1455_dn4 = assign49780_e64212_d_n4;
        locals.var_sp_s_c__blk1455_dn6 = assign49780_e64212_d_n6;
        locals.var_sp_s_c__blk1455_dn7 = assign49780_e64212_d_n7;
        locals.var_sp_s_c__blk1455_dn8 = assign49780_e64212_d_n8;
        locals.var_sp_s_c__blk1455_dn9 = assign49780_e64212_d_n9;
        locals.var_sp_s_c__blk1455_rv = 0.0;

        let (assign49790_e64231, assign49790_e64231_d_n4, assign49790_e64231_d_n6, assign49790_e64231_d_n7, assign49790_e64231_d_n8, assign49790_e64231_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49790_e64224: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_eta__blk1453);
        let assign49790_e64227: f64 = (locals.var_sp_s_a__blk1454 / locals.var_gf2__blk1325);
        let assign49790_e64228: f64 = (assign49790_e64227).ln();
        let assign49790_e64229: f64 = (assign49790_e64224 + assign49790_e64228);
        (assign49790_e64229, ((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_eta__blk1453_dn4) + ((((locals.var_sp_s_a__blk1454_dn4 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn4)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_eta__blk1453_dn6) + ((((locals.var_sp_s_a__blk1454_dn6 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn6)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_eta__blk1453_dn7) + ((((locals.var_sp_s_a__blk1454_dn7 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn7)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_eta__blk1453_dn8) + ((((locals.var_sp_s_a__blk1454_dn8 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn8)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_eta__blk1453_dn9) + ((((locals.var_sp_s_a__blk1454_dn9 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn9)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)),)
    } else {
        (locals.var_sp_s_tau__blk1456, locals.var_sp_s_tau__blk1456_dn4, locals.var_sp_s_tau__blk1456_dn6, locals.var_sp_s_tau__blk1456_dn7, locals.var_sp_s_tau__blk1456_dn8, locals.var_sp_s_tau__blk1456_dn9,)
    }
};
        locals.var_sp_s_tau__blk1456 = assign49790_e64231;
        locals.var_sp_s_tau__blk1456_dn4 = assign49790_e64231_d_n4;
        locals.var_sp_s_tau__blk1456_dn6 = assign49790_e64231_d_n6;
        locals.var_sp_s_tau__blk1456_dn7 = assign49790_e64231_d_n7;
        locals.var_sp_s_tau__blk1456_dn8 = assign49790_e64231_d_n8;
        locals.var_sp_s_tau__blk1456_dn9 = assign49790_e64231_d_n9;
        locals.var_sp_s_tau__blk1456_rv = 0.0;

        let (assign49800_e64245, assign49800_e64245_d_n4, assign49800_e64245_d_n6, assign49800_e64245_d_n7, assign49800_e64245_d_n8, assign49800_e64245_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49800_e64243: f64 = (locals.var_sp_s_a__blk1454 + locals.var_sp_s_c__blk1455);
        (assign49800_e64243, (locals.var_sp_s_a__blk1454_dn4 + locals.var_sp_s_c__blk1455_dn4), (locals.var_sp_s_a__blk1454_dn6 + locals.var_sp_s_c__blk1455_dn6), (locals.var_sp_s_a__blk1454_dn7 + locals.var_sp_s_c__blk1455_dn7), (locals.var_sp_s_a__blk1454_dn8 + locals.var_sp_s_c__blk1455_dn8), (locals.var_sp_s_a__blk1454_dn9 + locals.var_sp_s_c__blk1455_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign49800_e64245;
        locals.var_nu_dn4 = assign49800_e64245_d_n4;
        locals.var_nu_dn6 = assign49800_e64245_d_n6;
        locals.var_nu_dn7 = assign49800_e64245_d_n7;
        locals.var_nu_dn8 = assign49800_e64245_d_n8;
        locals.var_nu_dn9 = assign49800_e64245_d_n9;
        locals.var_nu_rv = 0.0;

        let (assign49810_e64271, assign49810_e64271_d_n4, assign49810_e64271_d_n6, assign49810_e64271_d_n7, assign49810_e64271_d_n8, assign49810_e64271_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49810_e64257: f64 = (locals.var_nu * locals.var_nu);
        let assign49810_e64262: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign49810_e64263: f64 = (0.5 * assign49810_e64262);
        let assign49810_e64266: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
        let assign49810_e64267: f64 = (assign49810_e64263 - assign49810_e64266);
        let assign49810_e64268: f64 = (locals.var_sp_s_tau__blk1456 * assign49810_e64267);
        let assign49810_e64269: f64 = (assign49810_e64257 + assign49810_e64268);
        (assign49810_e64269, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau__blk1456_dn4 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4))) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1456_dn6 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6))) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1456_dn7 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7))) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1456_dn8 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8))) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau__blk1456_dn9 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9))) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign49810_e64271;
        locals.var_mutau_dn4 = assign49810_e64271_d_n4;
        locals.var_mutau_dn6 = assign49810_e64271_d_n6;
        locals.var_mutau_dn7 = assign49810_e64271_d_n7;
        locals.var_mutau_dn8 = assign49810_e64271_d_n8;
        locals.var_mutau_dn9 = assign49810_e64271_d_n9;
        locals.var_mutau_rv = 0.0;

        let (assign49820_e64311, assign49820_e64311_d_n4, assign49820_e64311_d_n6, assign49820_e64311_d_n7, assign49820_e64311_d_n8, assign49820_e64311_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49820_e64284: f64 = (locals.var_sp_s_a__blk1454 * locals.var_nu);
        let assign49820_e64286: f64 = (assign49820_e64284 * locals.var_sp_s_tau__blk1456);
        let assign49820_e64290: f64 = (locals.var_nu / locals.var_mutau);
        let assign49820_e64292: f64 = (assign49820_e64290 * locals.var_sp_s_tau__blk1456);
        let assign49820_e64294: f64 = (assign49820_e64292 * locals.var_sp_s_tau__blk1456);
        let assign49820_e64296: f64 = (assign49820_e64294 * locals.var_sp_s_c__blk1455);
        let assign49820_e64299: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign49820_e64301: f64 = (assign49820_e64299 * 0.3333333333333333);
        let assign49820_e64304: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
        let assign49820_e64305: f64 = (assign49820_e64301 - assign49820_e64304);
        let assign49820_e64306: f64 = (assign49820_e64296 * assign49820_e64305);
        let assign49820_e64307: f64 = (locals.var_mutau + assign49820_e64306);
        let assign49820_e64308: f64 = (assign49820_e64286 / assign49820_e64307);
        let assign49820_e64309: f64 = (locals.var_sp_s_eta__blk1453 + assign49820_e64308);
        (assign49820_e64309, (locals.var_sp_s_eta__blk1453_dn4 + (((((((locals.var_sp_s_a__blk1454_dn4 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn4)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn4)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn6 + (((((((locals.var_sp_s_a__blk1454_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn6)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn6)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn7 + (((((((locals.var_sp_s_a__blk1454_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn7)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn7)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn8 + (((((((locals.var_sp_s_a__blk1454_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn8)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn8)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn9 + (((((((locals.var_sp_s_a__blk1454_dn9 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn9)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn9)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))))) / (assign49820_e64307 * assign49820_e64307))),)
    } else {
        (locals.var_sp_s_x0__blk1472, locals.var_sp_s_x0__blk1472_dn4, locals.var_sp_s_x0__blk1472_dn6, locals.var_sp_s_x0__blk1472_dn7, locals.var_sp_s_x0__blk1472_dn8, locals.var_sp_s_x0__blk1472_dn9,)
    }
};
        locals.var_sp_s_x0__blk1472 = assign49820_e64311;
        locals.var_sp_s_x0__blk1472_dn4 = assign49820_e64311_d_n4;
        locals.var_sp_s_x0__blk1472_dn6 = assign49820_e64311_d_n6;
        locals.var_sp_s_x0__blk1472_dn7 = assign49820_e64311_d_n7;
        locals.var_sp_s_x0__blk1472_dn8 = assign49820_e64311_d_n8;
        locals.var_sp_s_x0__blk1472_dn9 = assign49820_e64311_d_n9;
        locals.var_sp_s_x0__blk1472_rv = 0.0;

        let assign49830_e64314: f64 = if locals.var_sp_s_x0__blk1472 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1489 = assign49830_e64314;
        locals.var_guard1489_rv = 0.0;

        let (assign49840_e64329, assign49840_e64329_d_n4, assign49840_e64329_d_n6, assign49840_e64329_d_n7, assign49840_e64329_d_n8, assign49840_e64329_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign49840_e64327: f64 = (locals.var_sp_s_x0__blk1472).exp();
        (assign49840_e64327, (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn4), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn6), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn7), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn8), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn9),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49840_e64329;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49840_e64329_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49840_e64329_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49840_e64329_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49840_e64329_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49840_e64329_d_n9;
        locals.var_sp_s_delta0__blk1458_rv = 0.0;

        let (assign49850_e64345, assign49850_e64345_d_n4, assign49850_e64345_d_n6, assign49850_e64345_d_n7, assign49850_e64345_d_n8, assign49850_e64345_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign49850_e64343: f64 = (1.0 / locals.var_sp_s_delta0__blk1458);
        (assign49850_e64343, (-(locals.var_sp_s_delta0__blk1458_dn4 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn6 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn7 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn8 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn9 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign49850_e64345;
        locals.var_sp_s_delta1__blk1459_dn4 = assign49850_e64345_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign49850_e64345_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign49850_e64345_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign49850_e64345_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign49850_e64345_d_n9;
        locals.var_sp_s_delta1__blk1459_rv = 0.0;

        let (assign49860_e64361, assign49860_e64361_d_n4, assign49860_e64361_d_n6, assign49860_e64361_d_n7, assign49860_e64361_d_n8, assign49860_e64361_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign49860_e64359: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458);
        (assign49860_e64359, ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn9)),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49860_e64361;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49860_e64361_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49860_e64361_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49860_e64361_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49860_e64361_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49860_e64361_d_n9;
        locals.var_sp_s_delta0__blk1458_rv = 0.0;

        let assign49870_e64365: f64 = (locals.var_xn_s__blk1349 - 230.25850929940458);
        let assign49870_e64366: f64 = if locals.var_sp_s_x0__blk1472 > assign49870_e64365 { 1.0 } else { 0.0 };
        locals.var_guard1490 = assign49870_e64366;
        locals.var_guard1490_rv = 0.0;

        let (assign49880_e64386, assign49880_e64386_d_n4, assign49880_e64386_d_n6, assign49880_e64386_d_n7, assign49880_e64386_d_n8, assign49880_e64386_d_n9,) = {
    if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 != 0.0)) {
        let assign49880_e64383: f64 = (locals.var_sp_s_x0__blk1472 - locals.var_xn_s__blk1349);
        let assign49880_e64384: f64 = (assign49880_e64383).exp();
        (assign49880_e64384, (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn4 - locals.var_xn_s__blk1349_dn4)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn6 - locals.var_xn_s__blk1349_dn6)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn7 - locals.var_xn_s__blk1349_dn7)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn8 - locals.var_xn_s__blk1349_dn8)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn9 - locals.var_xn_s__blk1349_dn9)),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49880_e64386;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49880_e64386_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49880_e64386_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49880_e64386_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49880_e64386_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49880_e64386_d_n9;
        locals.var_sp_s_delta0__blk1458_rv = 0.0;

        let (assign49890_e64405, assign49890_e64405_d_n4, assign49890_e64405_d_n6, assign49890_e64405_d_n7, assign49890_e64405_d_n8, assign49890_e64405_d_n9,) = {
    if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 != 0.0)) {
        let assign49890_e64403: f64 = (locals.var_delta_ns__blk1364 / locals.var_sp_s_delta0__blk1458);
        (assign49890_e64403, (((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn4)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn6)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn7)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn8)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn9)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign49890_e64405;
        locals.var_sp_s_delta1__blk1459_dn4 = assign49890_e64405_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign49890_e64405_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign49890_e64405_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign49890_e64405_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign49890_e64405_d_n9;
        locals.var_sp_s_delta1__blk1459_rv = 0.0;

        let (assign49900_e64451, assign49900_e64451_d_n4, assign49900_e64451_d_n6, assign49900_e64451_d_n7, assign49900_e64451_d_n8, assign49900_e64451_d_n9,) = {
    if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 == 0.0)) {
        let assign49900_e64425: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_x0__blk1472);
        let assign49900_e64427: f64 = (assign49900_e64425 - 230.25850929940458);
        let assign49900_e64432: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_x0__blk1472);
        let assign49900_e64434: f64 = (assign49900_e64432 - 230.25850929940458);
        let assign49900_e64438: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_x0__blk1472);
        let assign49900_e64440: f64 = (assign49900_e64438 - 230.25850929940458);
        let assign49900_e64442: f64 = (assign49900_e64440 * 0.3333333333333333);
        let assign49900_e64443: f64 = (1.0 + assign49900_e64442);
        let assign49900_e64444: f64 = (assign49900_e64434 * assign49900_e64443);
        let assign49900_e64445: f64 = (0.5 * assign49900_e64444);
        let assign49900_e64446: f64 = (1.0 + assign49900_e64445);
        let assign49900_e64447: f64 = (assign49900_e64427 * assign49900_e64446);
        let assign49900_e64448: f64 = (1.0 + assign49900_e64447);
        let assign49900_e64449: f64 = (1e-100 / assign49900_e64448);
        (assign49900_e64449, (-((1e-100 * (((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_x0__blk1472_dn4) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_x0__blk1472_dn6) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_x0__blk1472_dn7) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_x0__blk1472_dn8) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_x0__blk1472_dn9) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49900_e64451;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49900_e64451_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49900_e64451_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49900_e64451_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49900_e64451_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49900_e64451_d_n9;
        locals.var_sp_s_delta0__blk1458_rv = 0.0;

        let (assign49910_e64491, assign49910_e64491_d_n4, assign49910_e64491_d_n6, assign49910_e64491_d_n7, assign49910_e64491_d_n8, assign49910_e64491_d_n9,) = {
    if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 == 0.0)) {
        let assign49910_e64471: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign49910_e64476: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign49910_e64480: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign49910_e64482: f64 = (assign49910_e64480 * 0.3333333333333333);
        let assign49910_e64483: f64 = (1.0 + assign49910_e64482);
        let assign49910_e64484: f64 = (assign49910_e64476 * assign49910_e64483);
        let assign49910_e64485: f64 = (0.5 * assign49910_e64484);
        let assign49910_e64486: f64 = (1.0 + assign49910_e64485);
        let assign49910_e64487: f64 = (assign49910_e64471 * assign49910_e64486);
        let assign49910_e64488: f64 = (1.0 + assign49910_e64487);
        let assign49910_e64489: f64 = (1e-100 / assign49910_e64488);
        (assign49910_e64489, (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn4 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn4 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn4 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn6 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn6 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn6 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn7 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn7 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn7 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn8 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn8 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn8 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn9 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn9 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn9 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign49910_e64491;
        locals.var_sp_s_delta1__blk1459_dn4 = assign49910_e64491_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign49910_e64491_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign49910_e64491_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign49910_e64491_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign49910_e64491_d_n9;
        locals.var_sp_s_delta1__blk1459_rv = 0.0;

        let (assign49920_e64509, assign49920_e64509_d_n4, assign49920_e64509_d_n6, assign49920_e64509_d_n7, assign49920_e64509_d_n8, assign49920_e64509_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49920_e64505: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
        let assign49920_e64506: f64 = (2.0 + assign49920_e64505);
        let assign49920_e64507: f64 = (1.0 / assign49920_e64506);
        (assign49920_e64507, (-(((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) / (assign49920_e64506 * assign49920_e64506))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49920_e64509;
        locals.var_sp_s_temp__blk1448_dn4 = assign49920_e64509_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49920_e64509_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49920_e64509_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49920_e64509_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49920_e64509_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign49930_e64525, assign49930_e64525_d_n4, assign49930_e64525_d_n6, assign49930_e64525_d_n7, assign49930_e64525_d_n8, assign49930_e64525_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49930_e64521: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
        let assign49930_e64523: f64 = (assign49930_e64521 * locals.var_sp_s_temp__blk1448);
        (assign49930_e64523, ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn4)), ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn6)), ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn7)), ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn8)), ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9,)
    }
};
        locals.var_sp_s_xi0__blk1460 = assign49930_e64525;
        locals.var_sp_s_xi0__blk1460_dn4 = assign49930_e64525_d_n4;
        locals.var_sp_s_xi0__blk1460_dn6 = assign49930_e64525_d_n6;
        locals.var_sp_s_xi0__blk1460_dn7 = assign49930_e64525_d_n7;
        locals.var_sp_s_xi0__blk1460_dn8 = assign49930_e64525_d_n8;
        locals.var_sp_s_xi0__blk1460_dn9 = assign49930_e64525_d_n9;
        locals.var_sp_s_xi0__blk1460_rv = 0.0;

        let (assign49940_e64543, assign49940_e64543_d_n4, assign49940_e64543_d_n6, assign49940_e64543_d_n7, assign49940_e64543_d_n8, assign49940_e64543_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49940_e64538: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448);
        let assign49940_e64540: f64 = (assign49940_e64538 * locals.var_sp_s_temp__blk1448);
        let assign49940_e64541: f64 = (4.0 * assign49940_e64540);
        (assign49940_e64541, (4.0 * ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn4))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn8))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn9))),)
    } else {
        (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9,)
    }
};
        locals.var_sp_s_xi1__blk1461 = assign49940_e64543;
        locals.var_sp_s_xi1__blk1461_dn4 = assign49940_e64543_d_n4;
        locals.var_sp_s_xi1__blk1461_dn6 = assign49940_e64543_d_n6;
        locals.var_sp_s_xi1__blk1461_dn7 = assign49940_e64543_d_n7;
        locals.var_sp_s_xi1__blk1461_dn8 = assign49940_e64543_d_n8;
        locals.var_sp_s_xi1__blk1461_dn9 = assign49940_e64543_d_n9;
        locals.var_sp_s_xi1__blk1461_rv = 0.0;

        let (assign49950_e64565, assign49950_e64565_d_n4, assign49950_e64565_d_n6, assign49950_e64565_d_n7, assign49950_e64565_d_n8, assign49950_e64565_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49950_e64555: f64 = (8.0 * locals.var_sp_s_temp__blk1448);
        let assign49950_e64558: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
        let assign49950_e64559: f64 = (assign49950_e64555 - assign49950_e64558);
        let assign49950_e64561: f64 = (assign49950_e64559 * locals.var_sp_s_temp__blk1448);
        let assign49950_e64563: f64 = (assign49950_e64561 * locals.var_sp_s_temp__blk1448);
        (assign49950_e64563, ((((((8.0 * locals.var_sp_s_temp__blk1448_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn4)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn8)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9,)
    }
};
        locals.var_sp_s_xi2__blk1462 = assign49950_e64565;
        locals.var_sp_s_xi2__blk1462_dn4 = assign49950_e64565_d_n4;
        locals.var_sp_s_xi2__blk1462_dn6 = assign49950_e64565_d_n6;
        locals.var_sp_s_xi2__blk1462_dn7 = assign49950_e64565_d_n7;
        locals.var_sp_s_xi2__blk1462_dn8 = assign49950_e64565_d_n8;
        locals.var_sp_s_xi2__blk1462_dn9 = assign49950_e64565_d_n9;
        locals.var_sp_s_xi2__blk1462_rv = 0.0;

        let (assign49960_e64579, assign49960_e64579_d_n4, assign49960_e64579_d_n6, assign49960_e64579_d_n7, assign49960_e64579_d_n8, assign49960_e64579_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49960_e64577: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_x0__blk1472);
        (assign49960_e64577, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_x0__blk1472_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_x0__blk1472_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_x0__blk1472_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_x0__blk1472_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_x0__blk1472_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49960_e64579;
        locals.var_sp_s_temp__blk1448_dn4 = assign49960_e64579_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49960_e64579_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49960_e64579_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49960_e64579_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49960_e64579_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign49970_e64607, assign49970_e64607_d_n4, assign49970_e64607_d_n6, assign49970_e64607_d_n7, assign49970_e64607_d_n8, assign49970_e64607_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49970_e64591: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign49970_e64595: f64 = (1.0 - locals.var_sp_s_delta1__blk1459);
        let assign49970_e64597: f64 = (assign49970_e64595 + locals.var_sp_s_delta0__blk1458);
        let assign49970_e64601: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
        let assign49970_e64602: f64 = (locals.var_delta_ns__blk1364 * assign49970_e64601);
        let assign49970_e64603: f64 = (assign49970_e64597 - assign49970_e64602);
        let assign49970_e64604: f64 = (locals.var_gf2__blk1325 * assign49970_e64603);
        let assign49970_e64605: f64 = (assign49970_e64591 + assign49970_e64604);
        (assign49970_e64605, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn9)))))),)
    } else {
        (locals.var_sp_s_pc__blk1463, locals.var_sp_s_pc__blk1463_dn4, locals.var_sp_s_pc__blk1463_dn6, locals.var_sp_s_pc__blk1463_dn7, locals.var_sp_s_pc__blk1463_dn8, locals.var_sp_s_pc__blk1463_dn9,)
    }
};
        locals.var_sp_s_pc__blk1463 = assign49970_e64607;
        locals.var_sp_s_pc__blk1463_dn4 = assign49970_e64607_d_n4;
        locals.var_sp_s_pc__blk1463_dn6 = assign49970_e64607_d_n6;
        locals.var_sp_s_pc__blk1463_dn7 = assign49970_e64607_d_n7;
        locals.var_sp_s_pc__blk1463_dn8 = assign49970_e64607_d_n8;
        locals.var_sp_s_pc__blk1463_dn9 = assign49970_e64607_d_n9;
        locals.var_sp_s_pc__blk1463_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_42(
        locals: &mut StampLocals,
    ) {
        let (assign49980_e64639, assign49980_e64639_d_n4, assign49980_e64639_d_n6, assign49980_e64639_d_n7, assign49980_e64639_d_n8, assign49980_e64639_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49980_e64619: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign49980_e64623: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_x0__blk1472);
        let assign49980_e64625: f64 = (assign49980_e64623 - 1.0);
        let assign49980_e64627: f64 = (assign49980_e64625 + locals.var_sp_s_delta0__blk1458);
        let assign49980_e64631: f64 = (locals.var_sp_s_x0__blk1472 + 1.0);
        let assign49980_e64633: f64 = (assign49980_e64631 + locals.var_sp_s_xi0__blk1460);
        let assign49980_e64634: f64 = (locals.var_delta_ns__blk1364 * assign49980_e64633);
        let assign49980_e64635: f64 = (assign49980_e64627 - assign49980_e64634);
        let assign49980_e64636: f64 = (locals.var_gf2__blk1325 * assign49980_e64635);
        let assign49980_e64637: f64 = (assign49980_e64619 - assign49980_e64636);
        (assign49980_e64637, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_x0__blk1472_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_x0__blk1472_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_x0__blk1472_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_x0__blk1472_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_x0__blk1472_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))),)
    } else {
        (locals.var_sp_s_qc__blk1464, locals.var_sp_s_qc__blk1464_dn4, locals.var_sp_s_qc__blk1464_dn6, locals.var_sp_s_qc__blk1464_dn7, locals.var_sp_s_qc__blk1464_dn8, locals.var_sp_s_qc__blk1464_dn9,)
    }
};
        locals.var_sp_s_qc__blk1464 = assign49980_e64639;
        locals.var_sp_s_qc__blk1464_dn4 = assign49980_e64639_d_n4;
        locals.var_sp_s_qc__blk1464_dn6 = assign49980_e64639_d_n6;
        locals.var_sp_s_qc__blk1464_dn7 = assign49980_e64639_d_n7;
        locals.var_sp_s_qc__blk1464_dn8 = assign49980_e64639_d_n8;
        locals.var_sp_s_qc__blk1464_dn9 = assign49980_e64639_d_n9;
        locals.var_sp_s_qc__blk1464_rv = 0.0;

        let (assign49990_e64661, assign49990_e64661_d_n4, assign49990_e64661_d_n6, assign49990_e64661_d_n7, assign49990_e64661_d_n8, assign49990_e64661_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49990_e64653: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_delta0__blk1458);
        let assign49990_e64656: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462);
        let assign49990_e64657: f64 = (assign49990_e64653 - assign49990_e64656);
        let assign49990_e64658: f64 = (locals.var_gf2__blk1325 * assign49990_e64657);
        let assign49990_e64659: f64 = (2.0 - assign49990_e64658);
        (assign49990_e64659, (-((locals.var_gf2__blk1325_dn4 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn9)))))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49990_e64661;
        locals.var_sp_s_temp__blk1448_dn4 = assign49990_e64661_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49990_e64661_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49990_e64661_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49990_e64661_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49990_e64661_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign50000_e64681, assign50000_e64681_d_n4, assign50000_e64681_d_n6, assign50000_e64681_d_n7, assign50000_e64681_d_n8, assign50000_e64681_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign50000_e64673: f64 = (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463);
        let assign50000_e64677: f64 = (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448);
        let assign50000_e64678: f64 = (2.0 * assign50000_e64677);
        let assign50000_e64679: f64 = (assign50000_e64673 - assign50000_e64678);
        (assign50000_e64679, (((locals.var_sp_s_pc__blk1463_dn4 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn4)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn4)))), (((locals.var_sp_s_pc__blk1463_dn6 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn6)))), (((locals.var_sp_s_pc__blk1463_dn7 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn7)))), (((locals.var_sp_s_pc__blk1463_dn8 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn8)))), (((locals.var_sp_s_pc__blk1463_dn9 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn9)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn9)))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign50000_e64681;
        locals.var_sp_s_temp__blk1448_dn4 = assign50000_e64681_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign50000_e64681_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign50000_e64681_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign50000_e64681_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign50000_e64681_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign50010_e64702, assign50010_e64702_d_n4, assign50010_e64702_d_n6, assign50010_e64702_d_n7, assign50010_e64702_d_n8, assign50010_e64702_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign50010_e64696: f64 = (locals.var_sp_s_temp__blk1448).sqrt();
        let assign50010_e64697: f64 = (locals.var_sp_s_pc__blk1463 + assign50010_e64696);
        let assign50010_e64698: f64 = (locals.var_sp_s_qc__blk1464 / assign50010_e64697);
        let assign50010_e64699: f64 = (2.0 * assign50010_e64698);
        let assign50010_e64700: f64 = (locals.var_sp_s_x0__blk1472 + assign50010_e64699);
        (assign50010_e64700, (locals.var_sp_s_x0__blk1472_dn4 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn4 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn4 + (locals.var_sp_s_temp__blk1448_dn4 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn6 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn6 + (locals.var_sp_s_temp__blk1448_dn6 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn7 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn7 + (locals.var_sp_s_temp__blk1448_dn7 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn8 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn8 + (locals.var_sp_s_temp__blk1448_dn8 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn9 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn9 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn9 + (locals.var_sp_s_temp__blk1448_dn9 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))),)
    } else {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    }
};
        locals.var_x_s__blk1363 = assign50010_e64702;
        locals.var_x_s__blk1363_dn4 = assign50010_e64702_d_n4;
        locals.var_x_s__blk1363_dn6 = assign50010_e64702_d_n6;
        locals.var_x_s__blk1363_dn7 = assign50010_e64702_d_n7;
        locals.var_x_s__blk1363_dn8 = assign50010_e64702_d_n8;
        locals.var_x_s__blk1363_dn9 = assign50010_e64702_d_n9;
        locals.var_x_s__blk1363_rv = 0.0;

        let (assign50020_e64708, assign50020_e64708_d_n4, assign50020_e64708_d_n6, assign50020_e64708_d_n7, assign50020_e64708_d_n8, assign50020_e64708_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi1s__blk1366, locals.var_xi1s__blk1366_dn4, locals.var_xi1s__blk1366_dn6, locals.var_xi1s__blk1366_dn7, locals.var_xi1s__blk1366_dn8, locals.var_xi1s__blk1366_dn9,)
    }
};
        locals.var_xi1s__blk1366 = assign50020_e64708;
        locals.var_xi1s__blk1366_dn4 = assign50020_e64708_d_n4;
        locals.var_xi1s__blk1366_dn6 = assign50020_e64708_d_n6;
        locals.var_xi1s__blk1366_dn7 = assign50020_e64708_d_n7;
        locals.var_xi1s__blk1366_dn8 = assign50020_e64708_d_n8;
        locals.var_xi1s__blk1366_dn9 = assign50020_e64708_d_n9;
        locals.var_xi1s__blk1366_rv = 0.0;

        let (assign50030_e64714, assign50030_e64714_d_n4, assign50030_e64714_d_n6, assign50030_e64714_d_n7, assign50030_e64714_d_n8, assign50030_e64714_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi2s__blk1367, locals.var_xi2s__blk1367_dn4, locals.var_xi2s__blk1367_dn6, locals.var_xi2s__blk1367_dn7, locals.var_xi2s__blk1367_dn8, locals.var_xi2s__blk1367_dn9,)
    }
};
        locals.var_xi2s__blk1367 = assign50030_e64714;
        locals.var_xi2s__blk1367_dn4 = assign50030_e64714_d_n4;
        locals.var_xi2s__blk1367_dn6 = assign50030_e64714_d_n6;
        locals.var_xi2s__blk1367_dn7 = assign50030_e64714_d_n7;
        locals.var_xi2s__blk1367_dn8 = assign50030_e64714_d_n8;
        locals.var_xi2s__blk1367_dn9 = assign50030_e64714_d_n9;
        locals.var_xi2s__blk1367_rv = 0.0;

        let (assign50040_e64720, assign50040_e64720_d_n4, assign50040_e64720_d_n6, assign50040_e64720_d_n7, assign50040_e64720_d_n8, assign50040_e64720_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50040_e64720;
        locals.var_delta_1s__blk1368_dn4 = assign50040_e64720_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50040_e64720_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50040_e64720_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50040_e64720_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50040_e64720_d_n9;
        locals.var_delta_1s__blk1368_rv = 0.0;

        let (assign50050_e64726, assign50050_e64726_d_n4, assign50050_e64726_d_n6, assign50050_e64726_d_n7, assign50050_e64726_d_n8, assign50050_e64726_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    }
};
        locals.var_es__blk1369 = assign50050_e64726;
        locals.var_es__blk1369_dn4 = assign50050_e64726_d_n4;
        locals.var_es__blk1369_dn6 = assign50050_e64726_d_n6;
        locals.var_es__blk1369_dn7 = assign50050_e64726_d_n7;
        locals.var_es__blk1369_dn8 = assign50050_e64726_d_n8;
        locals.var_es__blk1369_dn9 = assign50050_e64726_d_n9;
        locals.var_es__blk1369_rv = 0.0;

        let (assign50060_e64732, assign50060_e64732_d_n4, assign50060_e64732_d_n6, assign50060_e64732_d_n7, assign50060_e64732_d_n8, assign50060_e64732_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    }
};
        locals.var_ds__blk1370 = assign50060_e64732;
        locals.var_ds__blk1370_dn4 = assign50060_e64732_d_n4;
        locals.var_ds__blk1370_dn6 = assign50060_e64732_d_n6;
        locals.var_ds__blk1370_dn7 = assign50060_e64732_d_n7;
        locals.var_ds__blk1370_dn8 = assign50060_e64732_d_n8;
        locals.var_ds__blk1370_dn9 = assign50060_e64732_d_n9;
        locals.var_ds__blk1370_rv = 0.0;

        let (assign50070_e64738, assign50070_e64738_d_n4, assign50070_e64738_d_n6, assign50070_e64738_d_n7, assign50070_e64738_d_n8, assign50070_e64738_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    }
};
        locals.var_ps__blk1371 = assign50070_e64738;
        locals.var_ps__blk1371_dn4 = assign50070_e64738_d_n4;
        locals.var_ps__blk1371_dn6 = assign50070_e64738_d_n6;
        locals.var_ps__blk1371_dn7 = assign50070_e64738_d_n7;
        locals.var_ps__blk1371_dn8 = assign50070_e64738_d_n8;
        locals.var_ps__blk1371_dn9 = assign50070_e64738_d_n9;
        locals.var_ps__blk1371_rv = 0.0;

        let (assign50080_e64744, assign50080_e64744_d_n4, assign50080_e64744_d_n6, assign50080_e64744_d_n7, assign50080_e64744_d_n8, assign50080_e64744_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9,)
    }
};
        locals.var_sqs__blk1372 = assign50080_e64744;
        locals.var_sqs__blk1372_dn4 = assign50080_e64744_d_n4;
        locals.var_sqs__blk1372_dn6 = assign50080_e64744_d_n6;
        locals.var_sqs__blk1372_dn7 = assign50080_e64744_d_n7;
        locals.var_sqs__blk1372_dn8 = assign50080_e64744_d_n8;
        locals.var_sqs__blk1372_dn9 = assign50080_e64744_d_n9;
        locals.var_sqs__blk1372_rv = 0.0;

        let (assign50090_e64750, assign50090_e64750_d_n4, assign50090_e64750_d_n6, assign50090_e64750_d_n7, assign50090_e64750_d_n8, assign50090_e64750_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9,)
    }
};
        locals.var_alphas__blk1373 = assign50090_e64750;
        locals.var_alphas__blk1373_dn4 = assign50090_e64750_d_n4;
        locals.var_alphas__blk1373_dn6 = assign50090_e64750_d_n6;
        locals.var_alphas__blk1373_dn7 = assign50090_e64750_d_n7;
        locals.var_alphas__blk1373_dn8 = assign50090_e64750_d_n8;
        locals.var_alphas__blk1373_dn9 = assign50090_e64750_d_n9;
        locals.var_alphas__blk1373_rv = 0.0;

        let (assign50100_e64756, assign50100_e64756_d_n4, assign50100_e64756_d_n6, assign50100_e64756_d_n7, assign50100_e64756_d_n8, assign50100_e64756_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rxcor__blk1374, locals.var_rxcor__blk1374_dn4, locals.var_rxcor__blk1374_dn6, locals.var_rxcor__blk1374_dn7, locals.var_rxcor__blk1374_dn8, locals.var_rxcor__blk1374_dn9,)
    }
};
        locals.var_rxcor__blk1374 = assign50100_e64756;
        locals.var_rxcor__blk1374_dn4 = assign50100_e64756_d_n4;
        locals.var_rxcor__blk1374_dn6 = assign50100_e64756_d_n6;
        locals.var_rxcor__blk1374_dn7 = assign50100_e64756_d_n7;
        locals.var_rxcor__blk1374_dn8 = assign50100_e64756_d_n8;
        locals.var_rxcor__blk1374_dn9 = assign50100_e64756_d_n9;
        locals.var_rxcor__blk1374_rv = 0.0;

        let (assign50110_e64764, assign50110_e64764_d_n4, assign50110_e64764_d_n6, assign50110_e64764_d_n7, assign50110_e64764_d_n8, assign50110_e64764_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign50110_e64762: f64 = (locals.var_xg__blk1343 - locals.var_x_s__blk1363);
        (assign50110_e64762, (locals.var_xg__blk1343_dn4 - locals.var_x_s__blk1363_dn4), (locals.var_xg__blk1343_dn6 - locals.var_x_s__blk1363_dn6), (locals.var_xg__blk1343_dn7 - locals.var_x_s__blk1363_dn7), (locals.var_xg__blk1343_dn8 - locals.var_x_s__blk1363_dn8), (locals.var_xg__blk1343_dn9 - locals.var_x_s__blk1363_dn9),)
    } else {
        (locals.var_xgs__blk1375, locals.var_xgs__blk1375_dn4, locals.var_xgs__blk1375_dn6, locals.var_xgs__blk1375_dn7, locals.var_xgs__blk1375_dn8, locals.var_xgs__blk1375_dn9,)
    }
};
        locals.var_xgs__blk1375 = assign50110_e64764;
        locals.var_xgs__blk1375_dn4 = assign50110_e64764_d_n4;
        locals.var_xgs__blk1375_dn6 = assign50110_e64764_d_n6;
        locals.var_xgs__blk1375_dn7 = assign50110_e64764_d_n7;
        locals.var_xgs__blk1375_dn8 = assign50110_e64764_d_n8;
        locals.var_xgs__blk1375_dn9 = assign50110_e64764_d_n9;
        locals.var_xgs__blk1375_rv = 0.0;

        let (assign50120_e64770, assign50120_e64770_d_n4, assign50120_e64770_d_n6, assign50120_e64770_d_n7, assign50120_e64770_d_n8, assign50120_e64770_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9,)
    }
};
        locals.var_qis__blk1376 = assign50120_e64770;
        locals.var_qis__blk1376_dn4 = assign50120_e64770_d_n4;
        locals.var_qis__blk1376_dn6 = assign50120_e64770_d_n6;
        locals.var_qis__blk1376_dn7 = assign50120_e64770_d_n7;
        locals.var_qis__blk1376_dn8 = assign50120_e64770_d_n8;
        locals.var_qis__blk1376_dn9 = assign50120_e64770_d_n9;
        locals.var_qis__blk1376_rv = 0.0;

        let (assign50130_e64778, assign50130_e64778_d_n4, assign50130_e64778_d_n6, assign50130_e64778_d_n7, assign50130_e64778_d_n8, assign50130_e64778_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign50130_e64776: f64 = (locals.var_phit1__blk1339 * locals.var_xgs__blk1375);
        (assign50130_e64776, ((locals.var_phit1__blk1339_dn4 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn4)), ((locals.var_phit1__blk1339_dn6 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn6)), ((locals.var_phit1__blk1339_dn7 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn7)), ((locals.var_phit1__blk1339_dn8 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn8)), ((locals.var_phit1__blk1339_dn9 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn9)),)
    } else {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    }
};
        locals.var_qbs__blk1377 = assign50130_e64778;
        locals.var_qbs__blk1377_dn4 = assign50130_e64778_d_n4;
        locals.var_qbs__blk1377_dn6 = assign50130_e64778_d_n6;
        locals.var_qbs__blk1377_dn7 = assign50130_e64778_d_n7;
        locals.var_qbs__blk1377_dn8 = assign50130_e64778_d_n8;
        locals.var_qbs__blk1377_dn9 = assign50130_e64778_d_n9;
        locals.var_qbs__blk1377_rv = 0.0;

        let (assign50140_e64784, assign50140_e64784_d_n4, assign50140_e64784_d_n6, assign50140_e64784_d_n7, assign50140_e64784_d_n8, assign50140_e64784_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9,)
    }
};
        locals.var_rhob__blk1378 = assign50140_e64784;
        locals.var_rhob__blk1378_dn4 = assign50140_e64784_d_n4;
        locals.var_rhob__blk1378_dn6 = assign50140_e64784_d_n6;
        locals.var_rhob__blk1378_dn7 = assign50140_e64784_d_n7;
        locals.var_rhob__blk1378_dn8 = assign50140_e64784_d_n8;
        locals.var_rhob__blk1378_dn9 = assign50140_e64784_d_n9;
        locals.var_rhob__blk1378_rv = 0.0;

        let (assign50150_e64790, assign50150_e64790_d_n4, assign50150_e64790_d_n6, assign50150_e64790_d_n7, assign50150_e64790_d_n8, assign50150_e64790_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9,)
    }
};
        locals.var_rhog__blk1379 = assign50150_e64790;
        locals.var_rhog__blk1379_dn4 = assign50150_e64790_d_n4;
        locals.var_rhog__blk1379_dn6 = assign50150_e64790_d_n6;
        locals.var_rhog__blk1379_dn7 = assign50150_e64790_d_n7;
        locals.var_rhog__blk1379_dn8 = assign50150_e64790_d_n8;
        locals.var_rhog__blk1379_dn9 = assign50150_e64790_d_n9;
        locals.var_rhog__blk1379_rv = 0.0;

        let (assign50160_e64796, assign50160_e64796_d_n4, assign50160_e64796_d_n6, assign50160_e64796_d_n7, assign50160_e64796_d_n8, assign50160_e64796_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gmobs__blk1383, locals.var_gmobs__blk1383_dn4, locals.var_gmobs__blk1383_dn6, locals.var_gmobs__blk1383_dn7, locals.var_gmobs__blk1383_dn8, locals.var_gmobs__blk1383_dn9,)
    }
};
        locals.var_gmobs__blk1383 = assign50160_e64796;
        locals.var_gmobs__blk1383_dn4 = assign50160_e64796_d_n4;
        locals.var_gmobs__blk1383_dn6 = assign50160_e64796_d_n6;
        locals.var_gmobs__blk1383_dn7 = assign50160_e64796_d_n7;
        locals.var_gmobs__blk1383_dn8 = assign50160_e64796_d_n8;
        locals.var_gmobs__blk1383_dn9 = assign50160_e64796_d_n9;
        locals.var_gmobs__blk1383_rv = 0.0;

        let (assign50170_e64802, assign50170_e64802_d_n4, assign50170_e64802_d_n6, assign50170_e64802_d_n7, assign50170_e64802_d_n8, assign50170_e64802_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9,)
    }
};
        locals.var_xitsb__blk1384 = assign50170_e64802;
        locals.var_xitsb__blk1384_dn4 = assign50170_e64802_d_n4;
        locals.var_xitsb__blk1384_dn6 = assign50170_e64802_d_n6;
        locals.var_xitsb__blk1384_dn7 = assign50170_e64802_d_n7;
        locals.var_xitsb__blk1384_dn8 = assign50170_e64802_d_n8;
        locals.var_xitsb__blk1384_dn9 = assign50170_e64802_d_n9;
        locals.var_xitsb__blk1384_rv = 0.0;

        let (assign50180_e64808, assign50180_e64808_d_n4, assign50180_e64808_d_n6, assign50180_e64808_d_n7, assign50180_e64808_d_n8, assign50180_e64808_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9,)
    }
};
        locals.var_factheta__blk1386 = assign50180_e64808;
        locals.var_factheta__blk1386_dn4 = assign50180_e64808_d_n4;
        locals.var_factheta__blk1386_dn6 = assign50180_e64808_d_n6;
        locals.var_factheta__blk1386_dn7 = assign50180_e64808_d_n7;
        locals.var_factheta__blk1386_dn8 = assign50180_e64808_d_n8;
        locals.var_factheta__blk1386_dn9 = assign50180_e64808_d_n9;
        locals.var_factheta__blk1386_rv = 0.0;

        let assign50190_e64811: f64 = if locals.var_xg__blk1343 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1491 = assign50190_e64811;
        locals.var_guard1491_rv = 0.0;

        let (assign50200_e64825, assign50200_e64825_d_n4, assign50200_e64825_d_n6, assign50200_e64825_d_n7, assign50200_e64825_d_n8, assign50200_e64825_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50200_e64821: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
        let assign50200_e64822: f64 = (2.0 + assign50200_e64821);
        let assign50200_e64823: f64 = (1.0 / assign50200_e64822);
        (assign50200_e64823, (-(((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)) / (assign50200_e64822 * assign50200_e64822))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign50200_e64825;
        locals.var_temp__blk949_dn4 = assign50200_e64825_d_n4;
        locals.var_temp__blk949_dn6 = assign50200_e64825_d_n6;
        locals.var_temp__blk949_dn7 = assign50200_e64825_d_n7;
        locals.var_temp__blk949_dn8 = assign50200_e64825_d_n8;
        locals.var_temp__blk949_dn9 = assign50200_e64825_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign50210_e64837, assign50210_e64837_d_n4, assign50210_e64837_d_n6, assign50210_e64837_d_n7, assign50210_e64837_d_n8, assign50210_e64837_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50210_e64833: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
        let assign50210_e64835: f64 = (assign50210_e64833 * locals.var_temp__blk949);
        (assign50210_e64835, ((((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn4)), ((((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn6)), ((((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn7)), ((((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn8)), ((((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_xi0s__blk1365, locals.var_xi0s__blk1365_dn4, locals.var_xi0s__blk1365_dn6, locals.var_xi0s__blk1365_dn7, locals.var_xi0s__blk1365_dn8, locals.var_xi0s__blk1365_dn9,)
    }
};
        locals.var_xi0s__blk1365 = assign50210_e64837;
        locals.var_xi0s__blk1365_dn4 = assign50210_e64837_d_n4;
        locals.var_xi0s__blk1365_dn6 = assign50210_e64837_d_n6;
        locals.var_xi0s__blk1365_dn7 = assign50210_e64837_d_n7;
        locals.var_xi0s__blk1365_dn8 = assign50210_e64837_d_n8;
        locals.var_xi0s__blk1365_dn9 = assign50210_e64837_d_n9;
        locals.var_xi0s__blk1365_rv = 0.0;

        let (assign50220_e64851, assign50220_e64851_d_n4, assign50220_e64851_d_n6, assign50220_e64851_d_n7, assign50220_e64851_d_n8, assign50220_e64851_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50220_e64846: f64 = (locals.var_x_s__blk1363 * locals.var_temp__blk949);
        let assign50220_e64848: f64 = (assign50220_e64846 * locals.var_temp__blk949);
        let assign50220_e64849: f64 = (4.0 * assign50220_e64848);
        (assign50220_e64849, (4.0 * ((((locals.var_x_s__blk1363_dn4 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn4))), (4.0 * ((((locals.var_x_s__blk1363_dn6 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn6))), (4.0 * ((((locals.var_x_s__blk1363_dn7 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn7))), (4.0 * ((((locals.var_x_s__blk1363_dn8 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn8))), (4.0 * ((((locals.var_x_s__blk1363_dn9 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_xi1s__blk1366, locals.var_xi1s__blk1366_dn4, locals.var_xi1s__blk1366_dn6, locals.var_xi1s__blk1366_dn7, locals.var_xi1s__blk1366_dn8, locals.var_xi1s__blk1366_dn9,)
    }
};
        locals.var_xi1s__blk1366 = assign50220_e64851;
        locals.var_xi1s__blk1366_dn4 = assign50220_e64851_d_n4;
        locals.var_xi1s__blk1366_dn6 = assign50220_e64851_d_n6;
        locals.var_xi1s__blk1366_dn7 = assign50220_e64851_d_n7;
        locals.var_xi1s__blk1366_dn8 = assign50220_e64851_d_n8;
        locals.var_xi1s__blk1366_dn9 = assign50220_e64851_d_n9;
        locals.var_xi1s__blk1366_rv = 0.0;

        let (assign50230_e64869, assign50230_e64869_d_n4, assign50230_e64869_d_n6, assign50230_e64869_d_n7, assign50230_e64869_d_n8, assign50230_e64869_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50230_e64859: f64 = (8.0 * locals.var_temp__blk949);
        let assign50230_e64862: f64 = (12.0 * locals.var_xi0s__blk1365);
        let assign50230_e64863: f64 = (assign50230_e64859 - assign50230_e64862);
        let assign50230_e64865: f64 = (assign50230_e64863 * locals.var_temp__blk949);
        let assign50230_e64867: f64 = (assign50230_e64865 * locals.var_temp__blk949);
        (assign50230_e64867, ((((((8.0 * locals.var_temp__blk949_dn4) - (12.0 * locals.var_xi0s__blk1365_dn4)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn4)), ((((((8.0 * locals.var_temp__blk949_dn6) - (12.0 * locals.var_xi0s__blk1365_dn6)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn6)), ((((((8.0 * locals.var_temp__blk949_dn7) - (12.0 * locals.var_xi0s__blk1365_dn7)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn7)), ((((((8.0 * locals.var_temp__blk949_dn8) - (12.0 * locals.var_xi0s__blk1365_dn8)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn8)), ((((((8.0 * locals.var_temp__blk949_dn9) - (12.0 * locals.var_xi0s__blk1365_dn9)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_xi2s__blk1367, locals.var_xi2s__blk1367_dn4, locals.var_xi2s__blk1367_dn6, locals.var_xi2s__blk1367_dn7, locals.var_xi2s__blk1367_dn8, locals.var_xi2s__blk1367_dn9,)
    }
};
        locals.var_xi2s__blk1367 = assign50230_e64869;
        locals.var_xi2s__blk1367_dn4 = assign50230_e64869_d_n4;
        locals.var_xi2s__blk1367_dn6 = assign50230_e64869_d_n6;
        locals.var_xi2s__blk1367_dn7 = assign50230_e64869_d_n7;
        locals.var_xi2s__blk1367_dn8 = assign50230_e64869_d_n8;
        locals.var_xi2s__blk1367_dn9 = assign50230_e64869_d_n9;
        locals.var_xi2s__blk1367_rv = 0.0;

        let (assign50240_e64877, assign50240_e64877_d_n4, assign50240_e64877_d_n6, assign50240_e64877_d_n7, assign50240_e64877_d_n8, assign50240_e64877_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50240_e64877;
        locals.var_delta_1s__blk1368_dn4 = assign50240_e64877_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50240_e64877_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50240_e64877_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50240_e64877_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50240_e64877_d_n9;
        locals.var_delta_1s__blk1368_rv = 0.0;

        let assign50250_e64880: f64 = if locals.var_x_s__blk1363 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1492 = assign50250_e64880;
        locals.var_guard1492_rv = 0.0;

        let (assign50260_e64891, assign50260_e64891_d_n4, assign50260_e64891_d_n6, assign50260_e64891_d_n7, assign50260_e64891_d_n8, assign50260_e64891_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        let assign50260_e64889: f64 = (locals.var_x_s__blk1363).exp();
        (assign50260_e64889, (assign50260_e64889 * locals.var_x_s__blk1363_dn4), (assign50260_e64889 * locals.var_x_s__blk1363_dn6), (assign50260_e64889 * locals.var_x_s__blk1363_dn7), (assign50260_e64889 * locals.var_x_s__blk1363_dn8), (assign50260_e64889 * locals.var_x_s__blk1363_dn9),)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50260_e64891;
        locals.var_delta_1s__blk1368_dn4 = assign50260_e64891_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50260_e64891_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50260_e64891_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50260_e64891_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50260_e64891_d_n9;
        locals.var_delta_1s__blk1368_rv = 0.0;

        let (assign50270_e64903, assign50270_e64903_d_n4, assign50270_e64903_d_n6, assign50270_e64903_d_n7, assign50270_e64903_d_n8, assign50270_e64903_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        let assign50270_e64901: f64 = (1.0 / locals.var_delta_1s__blk1368);
        (assign50270_e64901, (-(locals.var_delta_1s__blk1368_dn4 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn6 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn7 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn8 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn9 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))),)
    } else {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    }
};
        locals.var_es__blk1369 = assign50270_e64903;
        locals.var_es__blk1369_dn4 = assign50270_e64903_d_n4;
        locals.var_es__blk1369_dn6 = assign50270_e64903_d_n6;
        locals.var_es__blk1369_dn7 = assign50270_e64903_d_n7;
        locals.var_es__blk1369_dn8 = assign50270_e64903_d_n8;
        locals.var_es__blk1369_dn9 = assign50270_e64903_d_n9;
        locals.var_es__blk1369_rv = 0.0;

        let (assign50280_e64915, assign50280_e64915_d_n4, assign50280_e64915_d_n6, assign50280_e64915_d_n7, assign50280_e64915_d_n8, assign50280_e64915_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        let assign50280_e64913: f64 = (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368);
        (assign50280_e64913, ((locals.var_delta_ns__blk1364_dn4 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn9)),)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50280_e64915;
        locals.var_delta_1s__blk1368_dn4 = assign50280_e64915_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50280_e64915_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50280_e64915_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50280_e64915_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50280_e64915_d_n9;
        locals.var_delta_1s__blk1368_rv = 0.0;

        let assign50290_e64919: f64 = (locals.var_xn_s__blk1349 - 230.25850929940458);
        let assign50290_e64920: f64 = if locals.var_x_s__blk1363 > assign50290_e64919 { 1.0 } else { 0.0 };
        locals.var_guard1493 = assign50290_e64920;
        locals.var_guard1493_rv = 0.0;

        let (assign50300_e64936, assign50300_e64936_d_n4, assign50300_e64936_d_n6, assign50300_e64936_d_n7, assign50300_e64936_d_n8, assign50300_e64936_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign50300_e64933: f64 = (locals.var_x_s__blk1363 - locals.var_xn_s__blk1349);
        let assign50300_e64934: f64 = (assign50300_e64933).exp();
        (assign50300_e64934, (assign50300_e64934 * (locals.var_x_s__blk1363_dn4 - locals.var_xn_s__blk1349_dn4)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn6 - locals.var_xn_s__blk1349_dn6)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn7 - locals.var_xn_s__blk1349_dn7)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn8 - locals.var_xn_s__blk1349_dn8)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn9 - locals.var_xn_s__blk1349_dn9)),)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50300_e64936;
        locals.var_delta_1s__blk1368_dn4 = assign50300_e64936_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50300_e64936_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50300_e64936_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50300_e64936_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50300_e64936_d_n9;
        locals.var_delta_1s__blk1368_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_43(
        locals: &mut StampLocals,
    ) {
        let (assign50310_e64951, assign50310_e64951_d_n4, assign50310_e64951_d_n6, assign50310_e64951_d_n7, assign50310_e64951_d_n8, assign50310_e64951_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign50310_e64949: f64 = (locals.var_delta_ns__blk1364 / locals.var_delta_1s__blk1368);
        (assign50310_e64949, (((locals.var_delta_ns__blk1364_dn4 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn4)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn6 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn6)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn7 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn7)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn8 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn8)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn9 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn9)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)),)
    } else {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    }
};
        locals.var_es__blk1369 = assign50310_e64951;
        locals.var_es__blk1369_dn4 = assign50310_e64951_d_n4;
        locals.var_es__blk1369_dn6 = assign50310_e64951_d_n6;
        locals.var_es__blk1369_dn7 = assign50310_e64951_d_n7;
        locals.var_es__blk1369_dn8 = assign50310_e64951_d_n8;
        locals.var_es__blk1369_dn9 = assign50310_e64951_d_n9;
        locals.var_es__blk1369_rv = 0.0;

        let (assign50320_e64993, assign50320_e64993_d_n4, assign50320_e64993_d_n6, assign50320_e64993_d_n7, assign50320_e64993_d_n8, assign50320_e64993_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) {
        let assign50320_e64967: f64 = (locals.var_xn_s__blk1349 - locals.var_x_s__blk1363);
        let assign50320_e64969: f64 = (assign50320_e64967 - 230.25850929940458);
        let assign50320_e64974: f64 = (locals.var_xn_s__blk1349 - locals.var_x_s__blk1363);
        let assign50320_e64976: f64 = (assign50320_e64974 - 230.25850929940458);
        let assign50320_e64980: f64 = (locals.var_xn_s__blk1349 - locals.var_x_s__blk1363);
        let assign50320_e64982: f64 = (assign50320_e64980 - 230.25850929940458);
        let assign50320_e64984: f64 = (assign50320_e64982 * 0.3333333333333333);
        let assign50320_e64985: f64 = (1.0 + assign50320_e64984);
        let assign50320_e64986: f64 = (assign50320_e64976 * assign50320_e64985);
        let assign50320_e64987: f64 = (0.5 * assign50320_e64986);
        let assign50320_e64988: f64 = (1.0 + assign50320_e64987);
        let assign50320_e64989: f64 = (assign50320_e64969 * assign50320_e64988);
        let assign50320_e64990: f64 = (1.0 + assign50320_e64989);
        let assign50320_e64991: f64 = (1e-100 / assign50320_e64990);
        (assign50320_e64991, (-((1e-100 * (((locals.var_xn_s__blk1349_dn4 - locals.var_x_s__blk1363_dn4) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn4 - locals.var_x_s__blk1363_dn4) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn4 - locals.var_x_s__blk1363_dn4) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn6 - locals.var_x_s__blk1363_dn6) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn6 - locals.var_x_s__blk1363_dn6) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn6 - locals.var_x_s__blk1363_dn6) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn7 - locals.var_x_s__blk1363_dn7) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn7 - locals.var_x_s__blk1363_dn7) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn7 - locals.var_x_s__blk1363_dn7) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn8 - locals.var_x_s__blk1363_dn8) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn8 - locals.var_x_s__blk1363_dn8) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn8 - locals.var_x_s__blk1363_dn8) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn9 - locals.var_x_s__blk1363_dn9) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn9 - locals.var_x_s__blk1363_dn9) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn9 - locals.var_x_s__blk1363_dn9) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))),)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50320_e64993;
        locals.var_delta_1s__blk1368_dn4 = assign50320_e64993_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50320_e64993_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50320_e64993_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50320_e64993_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50320_e64993_d_n9;
        locals.var_delta_1s__blk1368_rv = 0.0;

        let (assign50330_e65029, assign50330_e65029_d_n4, assign50330_e65029_d_n6, assign50330_e65029_d_n7, assign50330_e65029_d_n8, assign50330_e65029_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) {
        let assign50330_e65009: f64 = (locals.var_x_s__blk1363 - 230.25850929940458);
        let assign50330_e65014: f64 = (locals.var_x_s__blk1363 - 230.25850929940458);
        let assign50330_e65018: f64 = (locals.var_x_s__blk1363 - 230.25850929940458);
        let assign50330_e65020: f64 = (assign50330_e65018 * 0.3333333333333333);
        let assign50330_e65021: f64 = (1.0 + assign50330_e65020);
        let assign50330_e65022: f64 = (assign50330_e65014 * assign50330_e65021);
        let assign50330_e65023: f64 = (0.5 * assign50330_e65022);
        let assign50330_e65024: f64 = (1.0 + assign50330_e65023);
        let assign50330_e65025: f64 = (assign50330_e65009 * assign50330_e65024);
        let assign50330_e65026: f64 = (1.0 + assign50330_e65025);
        let assign50330_e65027: f64 = (1e-100 / assign50330_e65026);
        (assign50330_e65027, (-((1e-100 * ((locals.var_x_s__blk1363_dn4 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn4 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn4 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn6 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn6 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn6 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn7 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn7 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn7 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn8 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn8 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn8 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn9 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn9 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn9 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))),)
    } else {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    }
};
        locals.var_es__blk1369 = assign50330_e65029;
        locals.var_es__blk1369_dn4 = assign50330_e65029_d_n4;
        locals.var_es__blk1369_dn6 = assign50330_e65029_d_n6;
        locals.var_es__blk1369_dn7 = assign50330_e65029_d_n7;
        locals.var_es__blk1369_dn8 = assign50330_e65029_d_n8;
        locals.var_es__blk1369_dn9 = assign50330_e65029_d_n9;
        locals.var_es__blk1369_rv = 0.0;

        let (assign50340_e65045, assign50340_e65045_d_n4, assign50340_e65045_d_n6, assign50340_e65045_d_n7, assign50340_e65045_d_n8, assign50340_e65045_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50340_e65039: f64 = (locals.var_x_s__blk1363 + 1.0);
        let assign50340_e65041: f64 = (assign50340_e65039 + locals.var_xi0s__blk1365);
        let assign50340_e65042: f64 = (locals.var_delta_ns__blk1364 * assign50340_e65041);
        let assign50340_e65043: f64 = (locals.var_delta_1s__blk1368 - assign50340_e65042);
        (assign50340_e65043, (locals.var_delta_1s__blk1368_dn4 - ((locals.var_delta_ns__blk1364_dn4 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn4 + locals.var_xi0s__blk1365_dn4)))), (locals.var_delta_1s__blk1368_dn6 - ((locals.var_delta_ns__blk1364_dn6 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn6 + locals.var_xi0s__blk1365_dn6)))), (locals.var_delta_1s__blk1368_dn7 - ((locals.var_delta_ns__blk1364_dn7 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn7 + locals.var_xi0s__blk1365_dn7)))), (locals.var_delta_1s__blk1368_dn8 - ((locals.var_delta_ns__blk1364_dn8 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn8 + locals.var_xi0s__blk1365_dn8)))), (locals.var_delta_1s__blk1368_dn9 - ((locals.var_delta_ns__blk1364_dn9 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn9 + locals.var_xi0s__blk1365_dn9)))),)
    } else {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    }
};
        locals.var_ds__blk1370 = assign50340_e65045;
        locals.var_ds__blk1370_dn4 = assign50340_e65045_d_n4;
        locals.var_ds__blk1370_dn6 = assign50340_e65045_d_n6;
        locals.var_ds__blk1370_dn7 = assign50340_e65045_d_n7;
        locals.var_ds__blk1370_dn8 = assign50340_e65045_d_n8;
        locals.var_ds__blk1370_dn9 = assign50340_e65045_d_n9;
        locals.var_ds__blk1370_rv = 0.0;

        let assign50350_e65048: f64 = if locals.var_x_s__blk1363 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1494 = assign50350_e65048;
        locals.var_guard1494_rv = 0.0;

        let (assign50360_e65074, assign50360_e65074_d_n4, assign50360_e65074_d_n6, assign50360_e65074_d_n7, assign50360_e65074_d_n8, assign50360_e65074_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign50360_e65059: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
        let assign50360_e65066: f64 = (0.25 * locals.var_x_s__blk1363);
        let assign50360_e65067: f64 = (1.0 - assign50360_e65066);
        let assign50360_e65068: f64 = (locals.var_x_s__blk1363 * assign50360_e65067);
        let assign50360_e65069: f64 = (0.3333333333333333 * assign50360_e65068);
        let assign50360_e65070: f64 = (1.0 - assign50360_e65069);
        let assign50360_e65071: f64 = (assign50360_e65059 * assign50360_e65070);
        let assign50360_e65072: f64 = (0.5 * assign50360_e65071);
        (assign50360_e65072, (0.5 * ((((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn4 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn4))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn6 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn6))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn7 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn7))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn8 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn8))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn9 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn9))))))))),)
    } else {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    }
};
        locals.var_ps__blk1371 = assign50360_e65074;
        locals.var_ps__blk1371_dn4 = assign50360_e65074_d_n4;
        locals.var_ps__blk1371_dn6 = assign50360_e65074_d_n6;
        locals.var_ps__blk1371_dn7 = assign50360_e65074_d_n7;
        locals.var_ps__blk1371_dn8 = assign50360_e65074_d_n8;
        locals.var_ps__blk1371_dn9 = assign50360_e65074_d_n9;
        locals.var_ps__blk1371_rv = 0.0;

        let (assign50370_e65098, assign50370_e65098_d_n4, assign50370_e65098_d_n6, assign50370_e65098_d_n7, assign50370_e65098_d_n8, assign50370_e65098_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign50370_e65085: f64 = (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363);
        let assign50370_e65087: f64 = (assign50370_e65085 * locals.var_x_s__blk1363);
        let assign50370_e65089: f64 = (assign50370_e65087 * locals.var_x_s__blk1363);
        let assign50370_e65093: f64 = (1.75 * locals.var_x_s__blk1363);
        let assign50370_e65094: f64 = (1.0 + assign50370_e65093);
        let assign50370_e65095: f64 = (assign50370_e65089 * assign50370_e65094);
        let assign50370_e65096: f64 = (0.16666666666666666 * assign50370_e65095);
        (assign50370_e65096, (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn4 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn4)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn4)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn4)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn4)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn6 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn6)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn6)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn6)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn6)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn7 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn7)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn7)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn7)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn7)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn8 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn8)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn8)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn8)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn8)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn9 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn9)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn9)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn9)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn9)))),)
    } else {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    }
};
        locals.var_ds__blk1370 = assign50370_e65098;
        locals.var_ds__blk1370_dn4 = assign50370_e65098_d_n4;
        locals.var_ds__blk1370_dn6 = assign50370_e65098_d_n6;
        locals.var_ds__blk1370_dn7 = assign50370_e65098_d_n7;
        locals.var_ds__blk1370_dn8 = assign50370_e65098_d_n8;
        locals.var_ds__blk1370_dn9 = assign50370_e65098_d_n9;
        locals.var_ds__blk1370_rv = 0.0;

        let (assign50380_e65119, assign50380_e65119_d_n4, assign50380_e65119_d_n6, assign50380_e65119_d_n7, assign50380_e65119_d_n8, assign50380_e65119_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign50380_e65112: f64 = (0.25 * locals.var_x_s__blk1363);
        let assign50380_e65113: f64 = (1.0 - assign50380_e65112);
        let assign50380_e65114: f64 = (locals.var_x_s__blk1363 * assign50380_e65113);
        let assign50380_e65115: f64 = (0.3333333333333333 * assign50380_e65114);
        let assign50380_e65116: f64 = (1.0 - assign50380_e65115);
        let assign50380_e65117: f64 = (assign50380_e65116).sqrt();
        (assign50380_e65117, ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn4 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn4)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn6 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn6)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn7 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn7)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn8 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn8)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn9 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn9)))))) / (2.0 * assign50380_e65117)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign50380_e65119;
        locals.var_temp__blk949_dn4 = assign50380_e65119_d_n4;
        locals.var_temp__blk949_dn6 = assign50380_e65119_d_n6;
        locals.var_temp__blk949_dn7 = assign50380_e65119_d_n7;
        locals.var_temp__blk949_dn8 = assign50380_e65119_d_n8;
        locals.var_temp__blk949_dn9 = assign50380_e65119_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign50390_e65133, assign50390_e65133_d_n4, assign50390_e65133_d_n6, assign50390_e65133_d_n7, assign50390_e65133_d_n8, assign50390_e65133_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign50390_e65130: f64 = (locals.var_x_s__blk1363 * locals.var_temp__blk949);
        let assign50390_e65131: f64 = (0.7071067811865475 * assign50390_e65130);
        (assign50390_e65131, (0.7071067811865475 * ((locals.var_x_s__blk1363_dn4 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn6 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn7 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn8 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn9 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9,)
    }
};
        locals.var_sqs__blk1372 = assign50390_e65133;
        locals.var_sqs__blk1372_dn4 = assign50390_e65133_d_n4;
        locals.var_sqs__blk1372_dn6 = assign50390_e65133_d_n6;
        locals.var_sqs__blk1372_dn7 = assign50390_e65133_d_n7;
        locals.var_sqs__blk1372_dn8 = assign50390_e65133_d_n8;
        locals.var_sqs__blk1372_dn9 = assign50390_e65133_d_n9;
        locals.var_sqs__blk1372_rv = 0.0;

        let (assign50400_e65161, assign50400_e65161_d_n4, assign50400_e65161_d_n6, assign50400_e65161_d_n7, assign50400_e65161_d_n8, assign50400_e65161_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign50400_e65147: f64 = (0.5 * locals.var_x_s__blk1363);
        let assign50400_e65148: f64 = (1.0 - assign50400_e65147);
        let assign50400_e65152: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
        let assign50400_e65153: f64 = (0.16666666666666666 * assign50400_e65152);
        let assign50400_e65154: f64 = (assign50400_e65148 + assign50400_e65153);
        let assign50400_e65155: f64 = (locals.var_gf__blk1324 * assign50400_e65154);
        let assign50400_e65157: f64 = (assign50400_e65155 / locals.var_temp__blk949);
        let assign50400_e65158: f64 = (0.7071067811865475 * assign50400_e65157);
        let assign50400_e65159: f64 = (1.0 + assign50400_e65158);
        (assign50400_e65159, (0.7071067811865475 * (((((locals.var_gf__blk1324_dn4 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn4)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn6 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn6)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn7 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn7)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn8 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn8)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn9 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn9)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949))),)
    } else {
        (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9,)
    }
};
        locals.var_alphas__blk1373 = assign50400_e65161;
        locals.var_alphas__blk1373_dn4 = assign50400_e65161_d_n4;
        locals.var_alphas__blk1373_dn6 = assign50400_e65161_d_n6;
        locals.var_alphas__blk1373_dn7 = assign50400_e65161_d_n7;
        locals.var_alphas__blk1373_dn8 = assign50400_e65161_d_n8;
        locals.var_alphas__blk1373_dn9 = assign50400_e65161_d_n9;
        locals.var_alphas__blk1373_rv = 0.0;

        let (assign50410_e65176, assign50410_e65176_d_n4, assign50410_e65176_d_n6, assign50410_e65176_d_n7, assign50410_e65176_d_n8, assign50410_e65176_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign50410_e65172: f64 = (locals.var_x_s__blk1363 - 1.0);
        let assign50410_e65174: f64 = (assign50410_e65172 + locals.var_es__blk1369);
        (assign50410_e65174, (locals.var_x_s__blk1363_dn4 + locals.var_es__blk1369_dn4), (locals.var_x_s__blk1363_dn6 + locals.var_es__blk1369_dn6), (locals.var_x_s__blk1363_dn7 + locals.var_es__blk1369_dn7), (locals.var_x_s__blk1363_dn8 + locals.var_es__blk1369_dn8), (locals.var_x_s__blk1363_dn9 + locals.var_es__blk1369_dn9),)
    } else {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    }
};
        locals.var_ps__blk1371 = assign50410_e65176;
        locals.var_ps__blk1371_dn4 = assign50410_e65176_d_n4;
        locals.var_ps__blk1371_dn6 = assign50410_e65176_d_n6;
        locals.var_ps__blk1371_dn7 = assign50410_e65176_d_n7;
        locals.var_ps__blk1371_dn8 = assign50410_e65176_d_n8;
        locals.var_ps__blk1371_dn9 = assign50410_e65176_d_n9;
        locals.var_ps__blk1371_rv = 0.0;

        let (assign50420_e65188, assign50420_e65188_d_n4, assign50420_e65188_d_n6, assign50420_e65188_d_n7, assign50420_e65188_d_n8, assign50420_e65188_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign50420_e65186: f64 = (locals.var_ps__blk1371).sqrt();
        (assign50420_e65186, (locals.var_ps__blk1371_dn4 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn6 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn7 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn8 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn9 / (2.0 * assign50420_e65186)),)
    } else {
        (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9,)
    }
};
        locals.var_sqs__blk1372 = assign50420_e65188;
        locals.var_sqs__blk1372_dn4 = assign50420_e65188_d_n4;
        locals.var_sqs__blk1372_dn6 = assign50420_e65188_d_n6;
        locals.var_sqs__blk1372_dn7 = assign50420_e65188_d_n7;
        locals.var_sqs__blk1372_dn8 = assign50420_e65188_d_n8;
        locals.var_sqs__blk1372_dn9 = assign50420_e65188_d_n9;
        locals.var_sqs__blk1372_rv = 0.0;

        let (assign50430_e65209, assign50430_e65209_d_n4, assign50430_e65209_d_n6, assign50430_e65209_d_n7, assign50430_e65209_d_n8, assign50430_e65209_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign50430_e65202: f64 = (1.0 - locals.var_es__blk1369);
        let assign50430_e65203: f64 = (locals.var_gf__blk1324 * assign50430_e65202);
        let assign50430_e65205: f64 = (assign50430_e65203 / locals.var_sqs__blk1372);
        let assign50430_e65206: f64 = (0.5 * assign50430_e65205);
        let assign50430_e65207: f64 = (1.0 + assign50430_e65206);
        (assign50430_e65207, (0.5 * (((((locals.var_gf__blk1324_dn4 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn4))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn4)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn6 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn6))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn6)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn7 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn7))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn7)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn8 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn8))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn8)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn9 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn9))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn9)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))),)
    } else {
        (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9,)
    }
};
        locals.var_alphas__blk1373 = assign50430_e65209;
        locals.var_alphas__blk1373_dn4 = assign50430_e65209_d_n4;
        locals.var_alphas__blk1373_dn6 = assign50430_e65209_d_n6;
        locals.var_alphas__blk1373_dn7 = assign50430_e65209_d_n7;
        locals.var_alphas__blk1373_dn8 = assign50430_e65209_d_n8;
        locals.var_alphas__blk1373_dn9 = assign50430_e65209_d_n9;
        locals.var_alphas__blk1373_rv = 0.0;

        let (assign50440_e65229, assign50440_e65229_d_n4, assign50440_e65229_d_n6, assign50440_e65229_d_n7, assign50440_e65229_d_n8, assign50440_e65229_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50440_e65218: f64 = (0.2 * locals.var_xcor_t);
        let assign50440_e65220: f64 = (assign50440_e65218 * locals.var_vsbx__blk1323);
        let assign50440_e65221: f64 = (1.0 + assign50440_e65220);
        let assign50440_e65225: f64 = (locals.var_xcor_t * locals.var_vsbx__blk1323);
        let assign50440_e65226: f64 = (1.0 + assign50440_e65225);
        let assign50440_e65227: f64 = (assign50440_e65221 / assign50440_e65226);
        (assign50440_e65227, ((((((0.2 * locals.var_xcor_t_dn4) * locals.var_vsbx__blk1323) + (assign50440_e65218 * locals.var_vsbx__blk1323_dn4)) * assign50440_e65226) - (assign50440_e65221 * ((locals.var_xcor_t_dn4 * locals.var_vsbx__blk1323) + (locals.var_xcor_t * locals.var_vsbx__blk1323_dn4)))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn6) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn6))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn7) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn7))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn8) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn8))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn9) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn9))) / (assign50440_e65226 * assign50440_e65226)),)
    } else {
        (locals.var_rxcor__blk1374, locals.var_rxcor__blk1374_dn4, locals.var_rxcor__blk1374_dn6, locals.var_rxcor__blk1374_dn7, locals.var_rxcor__blk1374_dn8, locals.var_rxcor__blk1374_dn9,)
    }
};
        locals.var_rxcor__blk1374 = assign50440_e65229;
        locals.var_rxcor__blk1374_dn4 = assign50440_e65229_d_n4;
        locals.var_rxcor__blk1374_dn6 = assign50440_e65229_d_n6;
        locals.var_rxcor__blk1374_dn7 = assign50440_e65229_d_n7;
        locals.var_rxcor__blk1374_dn8 = assign50440_e65229_d_n8;
        locals.var_rxcor__blk1374_dn9 = assign50440_e65229_d_n9;
        locals.var_rxcor__blk1374_rv = 0.0;

        let assign50450_e65232: f64 = if locals.var_ds__blk1370 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1495 = assign50450_e65232;
        locals.var_guard1495_rv = 0.0;

        let (assign50460_e65247, assign50460_e65247_d_n4, assign50460_e65247_d_n6, assign50460_e65247_d_n7, assign50460_e65247_d_n8, assign50460_e65247_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50460_e65243: f64 = (locals.var_ps__blk1371 + locals.var_ds__blk1370);
        let assign50460_e65244: f64 = (assign50460_e65243).sqrt();
        let assign50460_e65245: f64 = (locals.var_gf__blk1324 * assign50460_e65244);
        (assign50460_e65245, ((locals.var_gf__blk1324_dn4 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn4 + locals.var_ds__blk1370_dn4) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn6 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn6 + locals.var_ds__blk1370_dn6) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn7 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn7 + locals.var_ds__blk1370_dn7) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn8 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn8 + locals.var_ds__blk1370_dn8) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn9 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn9 + locals.var_ds__blk1370_dn9) / (2.0 * assign50460_e65244)))),)
    } else {
        (locals.var_xgs__blk1375, locals.var_xgs__blk1375_dn4, locals.var_xgs__blk1375_dn6, locals.var_xgs__blk1375_dn7, locals.var_xgs__blk1375_dn8, locals.var_xgs__blk1375_dn9,)
    }
};
        locals.var_xgs__blk1375 = assign50460_e65247;
        locals.var_xgs__blk1375_dn4 = assign50460_e65247_d_n4;
        locals.var_xgs__blk1375_dn6 = assign50460_e65247_d_n6;
        locals.var_xgs__blk1375_dn7 = assign50460_e65247_d_n7;
        locals.var_xgs__blk1375_dn8 = assign50460_e65247_d_n8;
        locals.var_xgs__blk1375_dn9 = assign50460_e65247_d_n9;
        locals.var_xgs__blk1375_rv = 0.0;

        let (assign50470_e65267, assign50470_e65267_d_n4, assign50470_e65267_d_n6, assign50470_e65267_d_n7, assign50470_e65267_d_n8, assign50470_e65267_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50470_e65257: f64 = (locals.var_gf2__blk1325 * locals.var_ds__blk1370);
        let assign50470_e65259: f64 = (assign50470_e65257 * locals.var_phit1__blk1339);
        let assign50470_e65263: f64 = (locals.var_gf__blk1324 * locals.var_sqs__blk1372);
        let assign50470_e65264: f64 = (locals.var_xgs__blk1375 + assign50470_e65263);
        let assign50470_e65265: f64 = (assign50470_e65259 / assign50470_e65264);
        (assign50470_e65265, (((((((locals.var_gf2__blk1325_dn4 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn4)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn4)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn4 + ((locals.var_gf__blk1324_dn4 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn4))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn6 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn6)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn6)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn6 + ((locals.var_gf__blk1324_dn6 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn6))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn7 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn7)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn7)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn7 + ((locals.var_gf__blk1324_dn7 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn7))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn8 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn8)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn8)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn8 + ((locals.var_gf__blk1324_dn8 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn8))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn9 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn9)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn9)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn9 + ((locals.var_gf__blk1324_dn9 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn9))))) / (assign50470_e65264 * assign50470_e65264)),)
    } else {
        (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9,)
    }
};
        locals.var_qis__blk1376 = assign50470_e65267;
        locals.var_qis__blk1376_dn4 = assign50470_e65267_d_n4;
        locals.var_qis__blk1376_dn6 = assign50470_e65267_d_n6;
        locals.var_qis__blk1376_dn7 = assign50470_e65267_d_n7;
        locals.var_qis__blk1376_dn8 = assign50470_e65267_d_n8;
        locals.var_qis__blk1376_dn9 = assign50470_e65267_d_n9;
        locals.var_qis__blk1376_rv = 0.0;

        let (assign50480_e65281, assign50480_e65281_d_n4, assign50480_e65281_d_n6, assign50480_e65281_d_n7, assign50480_e65281_d_n8, assign50480_e65281_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50480_e65277: f64 = (locals.var_sqs__blk1372 * locals.var_gf__blk1324);
        let assign50480_e65279: f64 = (assign50480_e65277 * locals.var_phit1__blk1339);
        (assign50480_e65279, ((((locals.var_sqs__blk1372_dn4 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn4)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn4)), ((((locals.var_sqs__blk1372_dn6 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn6)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn6)), ((((locals.var_sqs__blk1372_dn7 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn7)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn7)), ((((locals.var_sqs__blk1372_dn8 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn8)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn8)), ((((locals.var_sqs__blk1372_dn9 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn9)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn9)),)
    } else {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    }
};
        locals.var_qbs__blk1377 = assign50480_e65281;
        locals.var_qbs__blk1377_dn4 = assign50480_e65281_d_n4;
        locals.var_qbs__blk1377_dn6 = assign50480_e65281_d_n6;
        locals.var_qbs__blk1377_dn7 = assign50480_e65281_d_n7;
        locals.var_qbs__blk1377_dn8 = assign50480_e65281_d_n8;
        locals.var_qbs__blk1377_dn9 = assign50480_e65281_d_n9;
        locals.var_qbs__blk1377_rv = 0.0;

        let assign50490_e65284: f64 = if locals.var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1496 = assign50490_e65284;
        locals.var_guard1496_rv = 0.0;

        let (assign50500_e65302, assign50500_e65302_d_n4, assign50500_e65302_d_n6, assign50500_e65302_d_n7, assign50500_e65302_d_n8, assign50500_e65302_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1496 != 0.0)) {
        let assign50500_e65298: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1323);
        let assign50500_e65299: f64 = (1.0 - assign50500_e65298);
        let assign50500_e65300: f64 = (1.0 / assign50500_e65299);
        (assign50500_e65300, (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn4)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn6)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn7)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn8)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn9)) / (assign50500_e65299 * assign50500_e65299))),)
    } else {
        (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9,)
    }
};
        locals.var_rhob__blk1378 = assign50500_e65302;
        locals.var_rhob__blk1378_dn4 = assign50500_e65302_d_n4;
        locals.var_rhob__blk1378_dn6 = assign50500_e65302_d_n6;
        locals.var_rhob__blk1378_dn7 = assign50500_e65302_d_n7;
        locals.var_rhob__blk1378_dn8 = assign50500_e65302_d_n8;
        locals.var_rhob__blk1378_dn9 = assign50500_e65302_d_n9;
        locals.var_rhob__blk1378_rv = 0.0;

        let (assign50510_e65319, assign50510_e65319_d_n4, assign50510_e65319_d_n6, assign50510_e65319_d_n7, assign50510_e65319_d_n8, assign50510_e65319_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1496 == 0.0)) {
        let assign50510_e65316: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1323);
        let assign50510_e65317: f64 = (1.0 + assign50510_e65316);
        (assign50510_e65317, (locals.var_rsb_i * locals.var_vsbx__blk1323_dn4), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn6), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn7), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn8), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn9),)
    } else {
        (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9,)
    }
};
        locals.var_rhob__blk1378 = assign50510_e65319;
        locals.var_rhob__blk1378_dn4 = assign50510_e65319_d_n4;
        locals.var_rhob__blk1378_dn6 = assign50510_e65319_d_n6;
        locals.var_rhob__blk1378_dn7 = assign50510_e65319_d_n7;
        locals.var_rhob__blk1378_dn8 = assign50510_e65319_d_n8;
        locals.var_rhob__blk1378_dn9 = assign50510_e65319_d_n9;
        locals.var_rhob__blk1378_rv = 0.0;

        let assign50520_e65322: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1497 = assign50520_e65322;
        locals.var_guard1497_rv = 0.0;

        let (assign50530_e65338, assign50530_e65338_d_n4, assign50530_e65338_d_n6, assign50530_e65338_d_n7, assign50530_e65338_d_n8, assign50530_e65338_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign50530_e65335: f64 = (locals.var_rsg_i * locals.var_qis__blk1376);
        let assign50530_e65336: f64 = (1.0 - assign50530_e65335);
        (assign50530_e65336, (-(locals.var_rsg_i * locals.var_qis__blk1376_dn4)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn6)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn7)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn8)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn9)),)
    } else {
        (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9,)
    }
};
        locals.var_rhog__blk1379 = assign50530_e65338;
        locals.var_rhog__blk1379_dn4 = assign50530_e65338_d_n4;
        locals.var_rhog__blk1379_dn6 = assign50530_e65338_d_n6;
        locals.var_rhog__blk1379_dn7 = assign50530_e65338_d_n7;
        locals.var_rhog__blk1379_dn8 = assign50530_e65338_d_n8;
        locals.var_rhog__blk1379_dn9 = assign50530_e65338_d_n9;
        locals.var_rhog__blk1379_rv = 0.0;

        let (assign50540_e65357, assign50540_e65357_d_n4, assign50540_e65357_d_n6, assign50540_e65357_d_n7, assign50540_e65357_d_n8, assign50540_e65357_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        let assign50540_e65353: f64 = (locals.var_rsg_i * locals.var_qis__blk1376);
        let assign50540_e65354: f64 = (1.0 + assign50540_e65353);
        let assign50540_e65355: f64 = (1.0 / assign50540_e65354);
        (assign50540_e65355, (-((locals.var_rsg_i * locals.var_qis__blk1376_dn4) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn6) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn7) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn8) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn9) / (assign50540_e65354 * assign50540_e65354))),)
    } else {
        (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9,)
    }
};
        locals.var_rhog__blk1379 = assign50540_e65357;
        locals.var_rhog__blk1379_dn4 = assign50540_e65357_d_n4;
        locals.var_rhog__blk1379_dn6 = assign50540_e65357_d_n6;
        locals.var_rhog__blk1379_dn7 = assign50540_e65357_d_n7;
        locals.var_rhog__blk1379_dn8 = assign50540_e65357_d_n8;
        locals.var_rhog__blk1379_dn9 = assign50540_e65357_d_n9;
        locals.var_rhog__blk1379_rv = 0.0;

        let (assign50550_e65373, assign50550_e65373_d_n4, assign50550_e65373_d_n6, assign50550_e65373_d_n7, assign50550_e65373_d_n8, assign50550_e65373_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50550_e65367: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
        let assign50550_e65369: f64 = (assign50550_e65367 * locals.var_rhog__blk1379);
        let assign50550_e65371: f64 = (assign50550_e65369 * locals.var_qis__blk1376);
        (assign50550_e65371, ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn4)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn4)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn6)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn7)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn8)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn8)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn9)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn9)),)
    } else {
        (locals.var_gr__blk1380, locals.var_gr__blk1380_dn4, locals.var_gr__blk1380_dn6, locals.var_gr__blk1380_dn7, locals.var_gr__blk1380_dn8, locals.var_gr__blk1380_dn9,)
    }
};
        locals.var_gr__blk1380 = assign50550_e65373;
        locals.var_gr__blk1380_dn4 = assign50550_e65373_d_n4;
        locals.var_gr__blk1380_dn6 = assign50550_e65373_d_n6;
        locals.var_gr__blk1380_dn7 = assign50550_e65373_d_n7;
        locals.var_gr__blk1380_dn8 = assign50550_e65373_d_n8;
        locals.var_gr__blk1380_dn9 = assign50550_e65373_d_n9;
        locals.var_gr__blk1380_rv = 0.0;

        let (assign50560_e65389, assign50560_e65389_d_n4, assign50560_e65389_d_n6, assign50560_e65389_d_n7, assign50560_e65389_d_n8, assign50560_e65389_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50560_e65385: f64 = (locals.var_eta_mu * locals.var_qis__blk1376);
        let assign50560_e65386: f64 = (locals.var_qbs__blk1377 + assign50560_e65385);
        let assign50560_e65387: f64 = (locals.var_e_eff0 * assign50560_e65386);
        (assign50560_e65387, (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn4 + (locals.var_eta_mu * locals.var_qis__blk1376_dn4))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn6 + (locals.var_eta_mu * locals.var_qis__blk1376_dn6))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn7 + (locals.var_eta_mu * locals.var_qis__blk1376_dn7))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn8 + (locals.var_eta_mu * locals.var_qis__blk1376_dn8))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn9 + (locals.var_eta_mu * locals.var_qis__blk1376_dn9))),)
    } else {
        (locals.var_eeffs__blk1381, locals.var_eeffs__blk1381_dn4, locals.var_eeffs__blk1381_dn6, locals.var_eeffs__blk1381_dn7, locals.var_eeffs__blk1381_dn8, locals.var_eeffs__blk1381_dn9,)
    }
};
        locals.var_eeffs__blk1381 = assign50560_e65389;
        locals.var_eeffs__blk1381_dn4 = assign50560_e65389_d_n4;
        locals.var_eeffs__blk1381_dn6 = assign50560_e65389_d_n6;
        locals.var_eeffs__blk1381_dn7 = assign50560_e65389_d_n7;
        locals.var_eeffs__blk1381_dn8 = assign50560_e65389_d_n8;
        locals.var_eeffs__blk1381_dn9 = assign50560_e65389_d_n9;
        locals.var_eeffs__blk1381_rv = 0.0;

        let (assign50570_e65406, assign50570_e65406_d_n4, assign50570_e65406_d_n6, assign50570_e65406_d_n7, assign50570_e65406_d_n8, assign50570_e65406_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50570_e65400: f64 = (locals.var_ps__blk1371 + locals.var_ds__blk1370);
        let assign50570_e65402: f64 = (assign50570_e65400 + 1e-14);
        let assign50570_e65403: f64 = (locals.var_ps__blk1371 / assign50570_e65402);
        let assign50570_e65404: f64 = (assign50570_e65403).ln();
        (assign50570_e65404, ((((locals.var_ps__blk1371_dn4 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn4 + locals.var_ds__blk1370_dn4))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn6 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn6 + locals.var_ds__blk1370_dn6))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn7 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn7 + locals.var_ds__blk1370_dn7))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn8 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn8 + locals.var_ds__blk1370_dn8))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn9 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn9 + locals.var_ds__blk1370_dn9))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign50570_e65406;
        locals.var_temp1_dn4 = assign50570_e65406_d_n4;
        locals.var_temp1_dn6 = assign50570_e65406_d_n6;
        locals.var_temp1_dn7 = assign50570_e65406_d_n7;
        locals.var_temp1_dn8 = assign50570_e65406_d_n8;
        locals.var_temp1_dn9 = assign50570_e65406_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign50580_e65429, assign50580_e65429_d_n4, assign50580_e65429_d_n6, assign50580_e65429_d_n7, assign50580_e65429_d_n8, assign50580_e65429_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50580_e65416: f64 = (locals.var_eeffs__blk1381 * locals.var_mue_t);
        let assign50580_e65418: f64 = (assign50580_e65416).powf(locals.var_themu_t);
        let assign50580_e65422: f64 = (0.5 * locals.var_thecs_t);
        let assign50580_e65424: f64 = (assign50580_e65422 * locals.var_temp1);
        let assign50580_e65425: f64 = (assign50580_e65424).exp();
        let assign50580_e65426: f64 = (locals.var_cs_t * assign50580_e65425);
        let assign50580_e65427: f64 = (assign50580_e65418 + assign50580_e65426);
        (assign50580_e65427, (if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * ((locals.var_eeffs__blk1381_dn4 * locals.var_mue_t) + (locals.var_eeffs__blk1381 * locals.var_mue_t_dn4)))) } } else { (assign50580_e65418 * ((locals.var_themu_t_dn4 * (assign50580_e65416).ln()) + (locals.var_themu_t * (((locals.var_eeffs__blk1381_dn4 * locals.var_mue_t) + (locals.var_eeffs__blk1381 * locals.var_mue_t_dn4)) / assign50580_e65416)))) } + ((locals.var_cs_t_dn4 * assign50580_e65425) + (locals.var_cs_t * (assign50580_e65425 * (((0.5 * locals.var_thecs_t_dn4) * locals.var_temp1) + (assign50580_e65422 * locals.var_temp1_dn4)))))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn6 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn6 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn7 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn7 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn8 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn8 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn8)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn9 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn9 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn9)))),)
    } else {
        (locals.var_mutmp__blk1382, locals.var_mutmp__blk1382_dn4, locals.var_mutmp__blk1382_dn6, locals.var_mutmp__blk1382_dn7, locals.var_mutmp__blk1382_dn8, locals.var_mutmp__blk1382_dn9,)
    }
};
        locals.var_mutmp__blk1382 = assign50580_e65429;
        locals.var_mutmp__blk1382_dn4 = assign50580_e65429_d_n4;
        locals.var_mutmp__blk1382_dn6 = assign50580_e65429_d_n6;
        locals.var_mutmp__blk1382_dn7 = assign50580_e65429_d_n7;
        locals.var_mutmp__blk1382_dn8 = assign50580_e65429_d_n8;
        locals.var_mutmp__blk1382_dn9 = assign50580_e65429_d_n9;
        locals.var_mutmp__blk1382_rv = 0.0;

        let (assign50590_e65445, assign50590_e65445_d_n4, assign50590_e65445_d_n6, assign50590_e65445_d_n7, assign50590_e65445_d_n8, assign50590_e65445_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50590_e65439: f64 = (1.0 + locals.var_mutmp__blk1382);
        let assign50590_e65441: f64 = (assign50590_e65439 + locals.var_gr__blk1380);
        let assign50590_e65443: f64 = (assign50590_e65441 * locals.var_rxcor__blk1374);
        (assign50590_e65443, (((locals.var_mutmp__blk1382_dn4 + locals.var_gr__blk1380_dn4) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn4)), (((locals.var_mutmp__blk1382_dn6 + locals.var_gr__blk1380_dn6) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn6)), (((locals.var_mutmp__blk1382_dn7 + locals.var_gr__blk1380_dn7) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn7)), (((locals.var_mutmp__blk1382_dn8 + locals.var_gr__blk1380_dn8) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn8)), (((locals.var_mutmp__blk1382_dn9 + locals.var_gr__blk1380_dn9) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn9)),)
    } else {
        (locals.var_gmobs__blk1383, locals.var_gmobs__blk1383_dn4, locals.var_gmobs__blk1383_dn6, locals.var_gmobs__blk1383_dn7, locals.var_gmobs__blk1383_dn8, locals.var_gmobs__blk1383_dn9,)
    }
};
        locals.var_gmobs__blk1383 = assign50590_e65445;
        locals.var_gmobs__blk1383_dn4 = assign50590_e65445_d_n4;
        locals.var_gmobs__blk1383_dn6 = assign50590_e65445_d_n6;
        locals.var_gmobs__blk1383_dn7 = assign50590_e65445_d_n7;
        locals.var_gmobs__blk1383_dn8 = assign50590_e65445_d_n8;
        locals.var_gmobs__blk1383_dn9 = assign50590_e65445_d_n9;
        locals.var_gmobs__blk1383_rv = 0.0;

        let assign50600_e65448: f64 = if locals.var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1498 = assign50600_e65448;
        locals.var_guard1498_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_44(
        locals: &mut StampLocals,
    ) {
        let (assign50610_e65466, assign50610_e65466_d_n4, assign50610_e65466_d_n6, assign50610_e65466_d_n7, assign50610_e65466_d_n8, assign50610_e65466_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign50610_e65462: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1323);
        let assign50610_e65463: f64 = (1.0 - assign50610_e65462);
        let assign50610_e65464: f64 = (1.0 / assign50610_e65463);
        (assign50610_e65464, (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn4)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn6)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn7)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn8)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn9)) / (assign50610_e65463 * assign50610_e65463))),)
    } else {
        (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9,)
    }
};
        locals.var_xitsb__blk1384 = assign50610_e65466;
        locals.var_xitsb__blk1384_dn4 = assign50610_e65466_d_n4;
        locals.var_xitsb__blk1384_dn6 = assign50610_e65466_d_n6;
        locals.var_xitsb__blk1384_dn7 = assign50610_e65466_d_n7;
        locals.var_xitsb__blk1384_dn8 = assign50610_e65466_d_n8;
        locals.var_xitsb__blk1384_dn9 = assign50610_e65466_d_n9;
        locals.var_xitsb__blk1384_rv = 0.0;

        let (assign50620_e65483, assign50620_e65483_d_n4, assign50620_e65483_d_n6, assign50620_e65483_d_n7, assign50620_e65483_d_n8, assign50620_e65483_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1498 == 0.0)) {
        let assign50620_e65480: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1323);
        let assign50620_e65481: f64 = (1.0 + assign50620_e65480);
        (assign50620_e65481, (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn4), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn6), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn7), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn8), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn9),)
    } else {
        (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9,)
    }
};
        locals.var_xitsb__blk1384 = assign50620_e65483;
        locals.var_xitsb__blk1384_dn4 = assign50620_e65483_d_n4;
        locals.var_xitsb__blk1384_dn6 = assign50620_e65483_d_n6;
        locals.var_xitsb__blk1384_dn7 = assign50620_e65483_d_n7;
        locals.var_xitsb__blk1384_dn8 = assign50620_e65483_d_n8;
        locals.var_xitsb__blk1384_dn9 = assign50620_e65483_d_n9;
        locals.var_xitsb__blk1384_rv = 0.0;

        let (assign50630_e65495, assign50630_e65495_d_n4, assign50630_e65495_d_n6, assign50630_e65495_d_n7, assign50630_e65495_d_n8, assign50630_e65495_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50630_e65493: f64 = (locals.var_qis__blk1376 * locals.var_xitsb__blk1384);
        (assign50630_e65493, ((locals.var_qis__blk1376_dn4 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn4)), ((locals.var_qis__blk1376_dn6 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn6)), ((locals.var_qis__blk1376_dn7 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn7)), ((locals.var_qis__blk1376_dn8 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn8)), ((locals.var_qis__blk1376_dn9 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign50630_e65495;
        locals.var_temp2_dn4 = assign50630_e65495_d_n4;
        locals.var_temp2_dn6 = assign50630_e65495_d_n6;
        locals.var_temp2_dn7 = assign50630_e65495_d_n7;
        locals.var_temp2_dn8 = assign50630_e65495_d_n8;
        locals.var_temp2_dn9 = assign50630_e65495_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign50640_e65509, assign50640_e65509_d_n4, assign50640_e65509_d_n6, assign50640_e65509_d_n7, assign50640_e65509_d_n8, assign50640_e65509_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50640_e65506: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign50640_e65507: f64 = (locals.var_temp2 / assign50640_e65506);
        (assign50640_e65507, (((locals.var_temp2_dn4 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn4)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn6 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn7 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn8 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn9 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn9)) / (assign50640_e65506 * assign50640_e65506)),)
    } else {
        (locals.var_wsat__blk1385, locals.var_wsat__blk1385_dn4, locals.var_wsat__blk1385_dn6, locals.var_wsat__blk1385_dn7, locals.var_wsat__blk1385_dn8, locals.var_wsat__blk1385_dn9,)
    }
};
        locals.var_wsat__blk1385 = assign50640_e65509;
        locals.var_wsat__blk1385_dn4 = assign50640_e65509_d_n4;
        locals.var_wsat__blk1385_dn6 = assign50640_e65509_d_n6;
        locals.var_wsat__blk1385_dn7 = assign50640_e65509_d_n7;
        locals.var_wsat__blk1385_dn8 = assign50640_e65509_d_n8;
        locals.var_wsat__blk1385_dn9 = assign50640_e65509_d_n9;
        locals.var_wsat__blk1385_rv = 0.0;

        let assign50650_e65512: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign50650_e65512;
        locals.var_guard1499_rv = 0.0;

        let (assign50660_e65530, assign50660_e65530_d_n4, assign50660_e65530_d_n6, assign50660_e65530_d_n7, assign50660_e65530_d_n8, assign50660_e65530_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign50660_e65526: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1385);
        let assign50660_e65527: f64 = (1.0 - assign50660_e65526);
        let assign50660_e65528: f64 = (1.0 / assign50660_e65527);
        (assign50660_e65528, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn4)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn6)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn7)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn8)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn9)) / (assign50660_e65527 * assign50660_e65527))),)
    } else {
        (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9,)
    }
};
        locals.var_factheta__blk1386 = assign50660_e65530;
        locals.var_factheta__blk1386_dn4 = assign50660_e65530_d_n4;
        locals.var_factheta__blk1386_dn6 = assign50660_e65530_d_n6;
        locals.var_factheta__blk1386_dn7 = assign50660_e65530_d_n7;
        locals.var_factheta__blk1386_dn8 = assign50660_e65530_d_n8;
        locals.var_factheta__blk1386_dn9 = assign50660_e65530_d_n9;
        locals.var_factheta__blk1386_rv = 0.0;

        let (assign50670_e65547, assign50670_e65547_d_n4, assign50670_e65547_d_n6, assign50670_e65547_d_n7, assign50670_e65547_d_n8, assign50670_e65547_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1499 == 0.0)) {
        let assign50670_e65544: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1385);
        let assign50670_e65545: f64 = (1.0 + assign50670_e65544);
        (assign50670_e65545, (locals.var_thesatg_i * locals.var_wsat__blk1385_dn4), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn8), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn9),)
    } else {
        (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9,)
    }
};
        locals.var_factheta__blk1386 = assign50670_e65547;
        locals.var_factheta__blk1386_dn4 = assign50670_e65547_d_n4;
        locals.var_factheta__blk1386_dn6 = assign50670_e65547_d_n6;
        locals.var_factheta__blk1386_dn7 = assign50670_e65547_d_n7;
        locals.var_factheta__blk1386_dn8 = assign50670_e65547_d_n8;
        locals.var_factheta__blk1386_dn9 = assign50670_e65547_d_n9;
        locals.var_factheta__blk1386_rv = 0.0;

        let (assign50770_e65646, assign50770_e65646_d_n4, assign50770_e65646_d_n6, assign50770_e65646_d_n7, assign50770_e65646_d_n8, assign50770_e65646_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_vgb1_dc, locals.var_vgb1_dc_dn4, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8, locals.var_vgb1_dc_dn9,)
    } else {
        (locals.var_vgb1__blk1321, locals.var_vgb1__blk1321_dn4, locals.var_vgb1__blk1321_dn6, locals.var_vgb1__blk1321_dn7, locals.var_vgb1__blk1321_dn8, locals.var_vgb1__blk1321_dn9,)
    }
};
        locals.var_vgb1__blk1321 = assign50770_e65646;
        locals.var_vgb1__blk1321_dn4 = assign50770_e65646_d_n4;
        locals.var_vgb1__blk1321_dn6 = assign50770_e65646_d_n6;
        locals.var_vgb1__blk1321_dn7 = assign50770_e65646_d_n7;
        locals.var_vgb1__blk1321_dn8 = assign50770_e65646_d_n8;
        locals.var_vgb1__blk1321_dn9 = assign50770_e65646_d_n9;
        locals.var_vgb1__blk1321_rv = 0.0;

        let (assign50780_e65653, assign50780_e65653_d_n4, assign50780_e65653_d_n6, assign50780_e65653_d_n7, assign50780_e65653_d_n8, assign50780_e65653_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_vsbx_dc, locals.var_vsbx_dc_dn4, locals.var_vsbx_dc_dn6, locals.var_vsbx_dc_dn7, locals.var_vsbx_dc_dn8, locals.var_vsbx_dc_dn9,)
    } else {
        (locals.var_vsbx__blk1323, locals.var_vsbx__blk1323_dn4, locals.var_vsbx__blk1323_dn6, locals.var_vsbx__blk1323_dn7, locals.var_vsbx__blk1323_dn8, locals.var_vsbx__blk1323_dn9,)
    }
};
        locals.var_vsbx__blk1323 = assign50780_e65653;
        locals.var_vsbx__blk1323_dn4 = assign50780_e65653_d_n4;
        locals.var_vsbx__blk1323_dn6 = assign50780_e65653_d_n6;
        locals.var_vsbx__blk1323_dn7 = assign50780_e65653_d_n7;
        locals.var_vsbx__blk1323_dn8 = assign50780_e65653_d_n8;
        locals.var_vsbx__blk1323_dn9 = assign50780_e65653_d_n9;
        locals.var_vsbx__blk1323_rv = 0.0;

        let (assign50790_e65660, assign50790_e65660_d_n4, assign50790_e65660_d_n6, assign50790_e65660_d_n7, assign50790_e65660_d_n8, assign50790_e65660_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_phit1_dc, locals.var_phit1_dc_dn4, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8, locals.var_phit1_dc_dn9,)
    } else {
        (locals.var_phit1__blk1339, locals.var_phit1__blk1339_dn4, locals.var_phit1__blk1339_dn6, locals.var_phit1__blk1339_dn7, locals.var_phit1__blk1339_dn8, locals.var_phit1__blk1339_dn9,)
    }
};
        locals.var_phit1__blk1339 = assign50790_e65660;
        locals.var_phit1__blk1339_dn4 = assign50790_e65660_d_n4;
        locals.var_phit1__blk1339_dn6 = assign50790_e65660_d_n6;
        locals.var_phit1__blk1339_dn7 = assign50790_e65660_d_n7;
        locals.var_phit1__blk1339_dn8 = assign50790_e65660_d_n8;
        locals.var_phit1__blk1339_dn9 = assign50790_e65660_d_n9;
        locals.var_phit1__blk1339_rv = 0.0;

        let (assign50800_e65667, assign50800_e65667_d_n4, assign50800_e65667_d_n6, assign50800_e65667_d_n7, assign50800_e65667_d_n8, assign50800_e65667_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_inv_phit1_dc, locals.var_inv_phit1_dc_dn4, locals.var_inv_phit1_dc_dn6, locals.var_inv_phit1_dc_dn7, locals.var_inv_phit1_dc_dn8, locals.var_inv_phit1_dc_dn9,)
    } else {
        (locals.var_inv_phit1__blk1340, locals.var_inv_phit1__blk1340_dn4, locals.var_inv_phit1__blk1340_dn6, locals.var_inv_phit1__blk1340_dn7, locals.var_inv_phit1__blk1340_dn8, locals.var_inv_phit1__blk1340_dn9,)
    }
};
        locals.var_inv_phit1__blk1340 = assign50800_e65667;
        locals.var_inv_phit1__blk1340_dn4 = assign50800_e65667_d_n4;
        locals.var_inv_phit1__blk1340_dn6 = assign50800_e65667_d_n6;
        locals.var_inv_phit1__blk1340_dn7 = assign50800_e65667_d_n7;
        locals.var_inv_phit1__blk1340_dn8 = assign50800_e65667_d_n8;
        locals.var_inv_phit1__blk1340_dn9 = assign50800_e65667_d_n9;
        locals.var_inv_phit1__blk1340_rv = 0.0;

        let (assign50810_e65674, assign50810_e65674_d_n4, assign50810_e65674_d_n6, assign50810_e65674_d_n7, assign50810_e65674_d_n8, assign50810_e65674_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_gf_dc, locals.var_gf_dc_dn4, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8, locals.var_gf_dc_dn9,)
    } else {
        (locals.var_gf__blk1324, locals.var_gf__blk1324_dn4, locals.var_gf__blk1324_dn6, locals.var_gf__blk1324_dn7, locals.var_gf__blk1324_dn8, locals.var_gf__blk1324_dn9,)
    }
};
        locals.var_gf__blk1324 = assign50810_e65674;
        locals.var_gf__blk1324_dn4 = assign50810_e65674_d_n4;
        locals.var_gf__blk1324_dn6 = assign50810_e65674_d_n6;
        locals.var_gf__blk1324_dn7 = assign50810_e65674_d_n7;
        locals.var_gf__blk1324_dn8 = assign50810_e65674_d_n8;
        locals.var_gf__blk1324_dn9 = assign50810_e65674_d_n9;
        locals.var_gf__blk1324_rv = 0.0;

        let (assign50820_e65681, assign50820_e65681_d_n4, assign50820_e65681_d_n6, assign50820_e65681_d_n7, assign50820_e65681_d_n8, assign50820_e65681_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_gf2_dc, locals.var_gf2_dc_dn4, locals.var_gf2_dc_dn6, locals.var_gf2_dc_dn7, locals.var_gf2_dc_dn8, locals.var_gf2_dc_dn9,)
    } else {
        (locals.var_gf2__blk1325, locals.var_gf2__blk1325_dn4, locals.var_gf2__blk1325_dn6, locals.var_gf2__blk1325_dn7, locals.var_gf2__blk1325_dn8, locals.var_gf2__blk1325_dn9,)
    }
};
        locals.var_gf2__blk1325 = assign50820_e65681;
        locals.var_gf2__blk1325_dn4 = assign50820_e65681_d_n4;
        locals.var_gf2__blk1325_dn6 = assign50820_e65681_d_n6;
        locals.var_gf2__blk1325_dn7 = assign50820_e65681_d_n7;
        locals.var_gf2__blk1325_dn8 = assign50820_e65681_d_n8;
        locals.var_gf2__blk1325_dn9 = assign50820_e65681_d_n9;
        locals.var_gf2__blk1325_rv = 0.0;

        let (assign50830_e65688, assign50830_e65688_d_n4, assign50830_e65688_d_n6, assign50830_e65688_d_n7, assign50830_e65688_d_n8, assign50830_e65688_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_inv_gf2_dc, locals.var_inv_gf2_dc_dn4, locals.var_inv_gf2_dc_dn6, locals.var_inv_gf2_dc_dn7, locals.var_inv_gf2_dc_dn8, locals.var_inv_gf2_dc_dn9,)
    } else {
        (locals.var_inv_gf2__blk1341, locals.var_inv_gf2__blk1341_dn4, locals.var_inv_gf2__blk1341_dn6, locals.var_inv_gf2__blk1341_dn7, locals.var_inv_gf2__blk1341_dn8, locals.var_inv_gf2__blk1341_dn9,)
    }
};
        locals.var_inv_gf2__blk1341 = assign50830_e65688;
        locals.var_inv_gf2__blk1341_dn4 = assign50830_e65688_d_n4;
        locals.var_inv_gf2__blk1341_dn6 = assign50830_e65688_d_n6;
        locals.var_inv_gf2__blk1341_dn7 = assign50830_e65688_d_n7;
        locals.var_inv_gf2__blk1341_dn8 = assign50830_e65688_d_n8;
        locals.var_inv_gf2__blk1341_dn9 = assign50830_e65688_d_n9;
        locals.var_inv_gf2__blk1341_rv = 0.0;

        let (assign50840_e65695, assign50840_e65695_d_n4, assign50840_e65695_d_n6, assign50840_e65695_d_n7, assign50840_e65695_d_n8, assign50840_e65695_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xg_dc, locals.var_xg_dc_dn4, locals.var_xg_dc_dn6, locals.var_xg_dc_dn7, locals.var_xg_dc_dn8, locals.var_xg_dc_dn9,)
    } else {
        (locals.var_xg__blk1343, locals.var_xg__blk1343_dn4, locals.var_xg__blk1343_dn6, locals.var_xg__blk1343_dn7, locals.var_xg__blk1343_dn8, locals.var_xg__blk1343_dn9,)
    }
};
        locals.var_xg__blk1343 = assign50840_e65695;
        locals.var_xg__blk1343_dn4 = assign50840_e65695_d_n4;
        locals.var_xg__blk1343_dn6 = assign50840_e65695_d_n6;
        locals.var_xg__blk1343_dn7 = assign50840_e65695_d_n7;
        locals.var_xg__blk1343_dn8 = assign50840_e65695_d_n8;
        locals.var_xg__blk1343_dn9 = assign50840_e65695_d_n9;
        locals.var_xg__blk1343_rv = 0.0;

        let (assign50850_e65702, assign50850_e65702_d_n4, assign50850_e65702_d_n6, assign50850_e65702_d_n7, assign50850_e65702_d_n8, assign50850_e65702_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xno_s_dc, locals.var_xno_s_dc_dn4, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8, locals.var_xno_s_dc_dn9,)
    } else {
        (locals.var_xno_s__blk1348, locals.var_xno_s__blk1348_dn4, locals.var_xno_s__blk1348_dn6, locals.var_xno_s__blk1348_dn7, locals.var_xno_s__blk1348_dn8, locals.var_xno_s__blk1348_dn9,)
    }
};
        locals.var_xno_s__blk1348 = assign50850_e65702;
        locals.var_xno_s__blk1348_dn4 = assign50850_e65702_d_n4;
        locals.var_xno_s__blk1348_dn6 = assign50850_e65702_d_n6;
        locals.var_xno_s__blk1348_dn7 = assign50850_e65702_d_n7;
        locals.var_xno_s__blk1348_dn8 = assign50850_e65702_d_n8;
        locals.var_xno_s__blk1348_dn9 = assign50850_e65702_d_n9;
        locals.var_xno_s__blk1348_rv = 0.0;

        let (assign50860_e65709, assign50860_e65709_d_n4, assign50860_e65709_d_n6, assign50860_e65709_d_n7, assign50860_e65709_d_n8, assign50860_e65709_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xn_s_dc, locals.var_xn_s_dc_dn4, locals.var_xn_s_dc_dn6, locals.var_xn_s_dc_dn7, locals.var_xn_s_dc_dn8, locals.var_xn_s_dc_dn9,)
    } else {
        (locals.var_xn_s__blk1349, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9,)
    }
};
        locals.var_xn_s__blk1349 = assign50860_e65709;
        locals.var_xn_s__blk1349_dn4 = assign50860_e65709_d_n4;
        locals.var_xn_s__blk1349_dn6 = assign50860_e65709_d_n6;
        locals.var_xn_s__blk1349_dn7 = assign50860_e65709_d_n7;
        locals.var_xn_s__blk1349_dn8 = assign50860_e65709_d_n8;
        locals.var_xn_s__blk1349_dn9 = assign50860_e65709_d_n9;
        locals.var_xn_s__blk1349_rv = 0.0;

        let (assign50870_e65716, assign50870_e65716_d_n4, assign50870_e65716_d_n6, assign50870_e65716_d_n7, assign50870_e65716_d_n8, assign50870_e65716_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xi_dc, locals.var_xi_dc_dn4, locals.var_xi_dc_dn6, locals.var_xi_dc_dn7, locals.var_xi_dc_dn8, locals.var_xi_dc_dn9,)
    } else {
        (locals.var_xi__blk1360, locals.var_xi__blk1360_dn4, locals.var_xi__blk1360_dn6, locals.var_xi__blk1360_dn7, locals.var_xi__blk1360_dn8, locals.var_xi__blk1360_dn9,)
    }
};
        locals.var_xi__blk1360 = assign50870_e65716;
        locals.var_xi__blk1360_dn4 = assign50870_e65716_d_n4;
        locals.var_xi__blk1360_dn6 = assign50870_e65716_d_n6;
        locals.var_xi__blk1360_dn7 = assign50870_e65716_d_n7;
        locals.var_xi__blk1360_dn8 = assign50870_e65716_d_n8;
        locals.var_xi__blk1360_dn9 = assign50870_e65716_d_n9;
        locals.var_xi__blk1360_rv = 0.0;

        let (assign50880_e65723, assign50880_e65723_d_n4, assign50880_e65723_d_n6, assign50880_e65723_d_n7, assign50880_e65723_d_n8, assign50880_e65723_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_margin_dc, locals.var_margin_dc_dn4, locals.var_margin_dc_dn6, locals.var_margin_dc_dn7, locals.var_margin_dc_dn8, locals.var_margin_dc_dn9,)
    } else {
        (locals.var_margin__blk1361, locals.var_margin__blk1361_dn4, locals.var_margin__blk1361_dn6, locals.var_margin__blk1361_dn7, locals.var_margin__blk1361_dn8, locals.var_margin__blk1361_dn9,)
    }
};
        locals.var_margin__blk1361 = assign50880_e65723;
        locals.var_margin__blk1361_dn4 = assign50880_e65723_d_n4;
        locals.var_margin__blk1361_dn6 = assign50880_e65723_d_n6;
        locals.var_margin__blk1361_dn7 = assign50880_e65723_d_n7;
        locals.var_margin__blk1361_dn8 = assign50880_e65723_d_n8;
        locals.var_margin__blk1361_dn9 = assign50880_e65723_d_n9;
        locals.var_margin__blk1361_rv = 0.0;

        let (assign50890_e65730, assign50890_e65730_d_n4, assign50890_e65730_d_n6, assign50890_e65730_d_n7, assign50890_e65730_d_n8, assign50890_e65730_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_inv_xi_dc, locals.var_inv_xi_dc_dn4, locals.var_inv_xi_dc_dn6, locals.var_inv_xi_dc_dn7, locals.var_inv_xi_dc_dn8, locals.var_inv_xi_dc_dn9,)
    } else {
        (locals.var_inv_xi__blk1362, locals.var_inv_xi__blk1362_dn4, locals.var_inv_xi__blk1362_dn6, locals.var_inv_xi__blk1362_dn7, locals.var_inv_xi__blk1362_dn8, locals.var_inv_xi__blk1362_dn9,)
    }
};
        locals.var_inv_xi__blk1362 = assign50890_e65730;
        locals.var_inv_xi__blk1362_dn4 = assign50890_e65730_d_n4;
        locals.var_inv_xi__blk1362_dn6 = assign50890_e65730_d_n6;
        locals.var_inv_xi__blk1362_dn7 = assign50890_e65730_d_n7;
        locals.var_inv_xi__blk1362_dn8 = assign50890_e65730_d_n8;
        locals.var_inv_xi__blk1362_dn9 = assign50890_e65730_d_n9;
        locals.var_inv_xi__blk1362_rv = 0.0;

        let (assign50900_e65737, assign50900_e65737_d_n4, assign50900_e65737_d_n6, assign50900_e65737_d_n7, assign50900_e65737_d_n8, assign50900_e65737_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_sp_s_x1_dc, locals.var_sp_s_x1_dc_dn4, locals.var_sp_s_x1_dc_dn6, locals.var_sp_s_x1_dc_dn7, locals.var_sp_s_x1_dc_dn8, locals.var_sp_s_x1_dc_dn9,)
    } else {
        (locals.var_sp_s_x1__blk1469, locals.var_sp_s_x1__blk1469_dn4, locals.var_sp_s_x1__blk1469_dn6, locals.var_sp_s_x1__blk1469_dn7, locals.var_sp_s_x1__blk1469_dn8, locals.var_sp_s_x1__blk1469_dn9,)
    }
};
        locals.var_sp_s_x1__blk1469 = assign50900_e65737;
        locals.var_sp_s_x1__blk1469_dn4 = assign50900_e65737_d_n4;
        locals.var_sp_s_x1__blk1469_dn6 = assign50900_e65737_d_n6;
        locals.var_sp_s_x1__blk1469_dn7 = assign50900_e65737_d_n7;
        locals.var_sp_s_x1__blk1469_dn8 = assign50900_e65737_d_n8;
        locals.var_sp_s_x1__blk1469_dn9 = assign50900_e65737_d_n9;
        locals.var_sp_s_x1__blk1469_rv = 0.0;

        let (assign50910_e65744, assign50910_e65744_d_n4, assign50910_e65744_d_n6, assign50910_e65744_d_n7, assign50910_e65744_d_n8, assign50910_e65744_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_delta_ns_dc, locals.var_delta_ns_dc_dn4, locals.var_delta_ns_dc_dn6, locals.var_delta_ns_dc_dn7, locals.var_delta_ns_dc_dn8, locals.var_delta_ns_dc_dn9,)
    } else {
        (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9,)
    }
};
        locals.var_delta_ns__blk1364 = assign50910_e65744;
        locals.var_delta_ns__blk1364_dn4 = assign50910_e65744_d_n4;
        locals.var_delta_ns__blk1364_dn6 = assign50910_e65744_d_n6;
        locals.var_delta_ns__blk1364_dn7 = assign50910_e65744_d_n7;
        locals.var_delta_ns__blk1364_dn8 = assign50910_e65744_d_n8;
        locals.var_delta_ns__blk1364_dn9 = assign50910_e65744_d_n9;
        locals.var_delta_ns__blk1364_rv = 0.0;

        let (assign50920_e65751, assign50920_e65751_d_n4, assign50920_e65751_d_n6, assign50920_e65751_d_n7, assign50920_e65751_d_n8, assign50920_e65751_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_x_s_dc, locals.var_x_s_dc_dn4, locals.var_x_s_dc_dn6, locals.var_x_s_dc_dn7, locals.var_x_s_dc_dn8, locals.var_x_s_dc_dn9,)
    } else {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    }
};
        locals.var_x_s__blk1363 = assign50920_e65751;
        locals.var_x_s__blk1363_dn4 = assign50920_e65751_d_n4;
        locals.var_x_s__blk1363_dn6 = assign50920_e65751_d_n6;
        locals.var_x_s__blk1363_dn7 = assign50920_e65751_d_n7;
        locals.var_x_s__blk1363_dn8 = assign50920_e65751_d_n8;
        locals.var_x_s__blk1363_dn9 = assign50920_e65751_d_n9;
        locals.var_x_s__blk1363_rv = 0.0;

        let (assign50930_e65758, assign50930_e65758_d_n4, assign50930_e65758_d_n6, assign50930_e65758_d_n7, assign50930_e65758_d_n8, assign50930_e65758_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xi1s_dc, locals.var_xi1s_dc_dn4, locals.var_xi1s_dc_dn6, locals.var_xi1s_dc_dn7, locals.var_xi1s_dc_dn8, locals.var_xi1s_dc_dn9,)
    } else {
        (locals.var_xi1s__blk1366, locals.var_xi1s__blk1366_dn4, locals.var_xi1s__blk1366_dn6, locals.var_xi1s__blk1366_dn7, locals.var_xi1s__blk1366_dn8, locals.var_xi1s__blk1366_dn9,)
    }
};
        locals.var_xi1s__blk1366 = assign50930_e65758;
        locals.var_xi1s__blk1366_dn4 = assign50930_e65758_d_n4;
        locals.var_xi1s__blk1366_dn6 = assign50930_e65758_d_n6;
        locals.var_xi1s__blk1366_dn7 = assign50930_e65758_d_n7;
        locals.var_xi1s__blk1366_dn8 = assign50930_e65758_d_n8;
        locals.var_xi1s__blk1366_dn9 = assign50930_e65758_d_n9;
        locals.var_xi1s__blk1366_rv = 0.0;

        let (assign50940_e65765, assign50940_e65765_d_n4, assign50940_e65765_d_n6, assign50940_e65765_d_n7, assign50940_e65765_d_n8, assign50940_e65765_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xi2s_dc, locals.var_xi2s_dc_dn4, locals.var_xi2s_dc_dn6, locals.var_xi2s_dc_dn7, locals.var_xi2s_dc_dn8, locals.var_xi2s_dc_dn9,)
    } else {
        (locals.var_xi2s__blk1367, locals.var_xi2s__blk1367_dn4, locals.var_xi2s__blk1367_dn6, locals.var_xi2s__blk1367_dn7, locals.var_xi2s__blk1367_dn8, locals.var_xi2s__blk1367_dn9,)
    }
};
        locals.var_xi2s__blk1367 = assign50940_e65765;
        locals.var_xi2s__blk1367_dn4 = assign50940_e65765_d_n4;
        locals.var_xi2s__blk1367_dn6 = assign50940_e65765_d_n6;
        locals.var_xi2s__blk1367_dn7 = assign50940_e65765_d_n7;
        locals.var_xi2s__blk1367_dn8 = assign50940_e65765_d_n8;
        locals.var_xi2s__blk1367_dn9 = assign50940_e65765_d_n9;
        locals.var_xi2s__blk1367_rv = 0.0;

        let (assign50950_e65772, assign50950_e65772_d_n4, assign50950_e65772_d_n6, assign50950_e65772_d_n7, assign50950_e65772_d_n8, assign50950_e65772_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_delta_1s_dc, locals.var_delta_1s_dc_dn4, locals.var_delta_1s_dc_dn6, locals.var_delta_1s_dc_dn7, locals.var_delta_1s_dc_dn8, locals.var_delta_1s_dc_dn9,)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50950_e65772;
        locals.var_delta_1s__blk1368_dn4 = assign50950_e65772_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50950_e65772_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50950_e65772_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50950_e65772_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50950_e65772_d_n9;
        locals.var_delta_1s__blk1368_rv = 0.0;

        let (assign50960_e65779, assign50960_e65779_d_n4, assign50960_e65779_d_n6, assign50960_e65779_d_n7, assign50960_e65779_d_n8, assign50960_e65779_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_es_dc, locals.var_es_dc_dn4, locals.var_es_dc_dn6, locals.var_es_dc_dn7, locals.var_es_dc_dn8, locals.var_es_dc_dn9,)
    } else {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    }
};
        locals.var_es__blk1369 = assign50960_e65779;
        locals.var_es__blk1369_dn4 = assign50960_e65779_d_n4;
        locals.var_es__blk1369_dn6 = assign50960_e65779_d_n6;
        locals.var_es__blk1369_dn7 = assign50960_e65779_d_n7;
        locals.var_es__blk1369_dn8 = assign50960_e65779_d_n8;
        locals.var_es__blk1369_dn9 = assign50960_e65779_d_n9;
        locals.var_es__blk1369_rv = 0.0;

        let (assign50970_e65786, assign50970_e65786_d_n4, assign50970_e65786_d_n6, assign50970_e65786_d_n7, assign50970_e65786_d_n8, assign50970_e65786_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_ps_dc, locals.var_ps_dc_dn4, locals.var_ps_dc_dn6, locals.var_ps_dc_dn7, locals.var_ps_dc_dn8, locals.var_ps_dc_dn9,)
    } else {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    }
};
        locals.var_ps__blk1371 = assign50970_e65786;
        locals.var_ps__blk1371_dn4 = assign50970_e65786_d_n4;
        locals.var_ps__blk1371_dn6 = assign50970_e65786_d_n6;
        locals.var_ps__blk1371_dn7 = assign50970_e65786_d_n7;
        locals.var_ps__blk1371_dn8 = assign50970_e65786_d_n8;
        locals.var_ps__blk1371_dn9 = assign50970_e65786_d_n9;
        locals.var_ps__blk1371_rv = 0.0;

        let (assign50980_e65793, assign50980_e65793_d_n4, assign50980_e65793_d_n6, assign50980_e65793_d_n7, assign50980_e65793_d_n8, assign50980_e65793_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_ds_dc, locals.var_ds_dc_dn4, locals.var_ds_dc_dn6, locals.var_ds_dc_dn7, locals.var_ds_dc_dn8, locals.var_ds_dc_dn9,)
    } else {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    }
};
        locals.var_ds__blk1370 = assign50980_e65793;
        locals.var_ds__blk1370_dn4 = assign50980_e65793_d_n4;
        locals.var_ds__blk1370_dn6 = assign50980_e65793_d_n6;
        locals.var_ds__blk1370_dn7 = assign50980_e65793_d_n7;
        locals.var_ds__blk1370_dn8 = assign50980_e65793_d_n8;
        locals.var_ds__blk1370_dn9 = assign50980_e65793_d_n9;
        locals.var_ds__blk1370_rv = 0.0;

        let (assign50990_e65800, assign50990_e65800_d_n4, assign50990_e65800_d_n6, assign50990_e65800_d_n7, assign50990_e65800_d_n8, assign50990_e65800_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_sqs_dc, locals.var_sqs_dc_dn4, locals.var_sqs_dc_dn6, locals.var_sqs_dc_dn7, locals.var_sqs_dc_dn8, locals.var_sqs_dc_dn9,)
    } else {
        (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9,)
    }
};
        locals.var_sqs__blk1372 = assign50990_e65800;
        locals.var_sqs__blk1372_dn4 = assign50990_e65800_d_n4;
        locals.var_sqs__blk1372_dn6 = assign50990_e65800_d_n6;
        locals.var_sqs__blk1372_dn7 = assign50990_e65800_d_n7;
        locals.var_sqs__blk1372_dn8 = assign50990_e65800_d_n8;
        locals.var_sqs__blk1372_dn9 = assign50990_e65800_d_n9;
        locals.var_sqs__blk1372_rv = 0.0;

        let (assign51000_e65807, assign51000_e65807_d_n4, assign51000_e65807_d_n6, assign51000_e65807_d_n7, assign51000_e65807_d_n8, assign51000_e65807_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_alphas_dc, locals.var_alphas_dc_dn4, locals.var_alphas_dc_dn6, locals.var_alphas_dc_dn7, locals.var_alphas_dc_dn8, locals.var_alphas_dc_dn9,)
    } else {
        (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9,)
    }
};
        locals.var_alphas__blk1373 = assign51000_e65807;
        locals.var_alphas__blk1373_dn4 = assign51000_e65807_d_n4;
        locals.var_alphas__blk1373_dn6 = assign51000_e65807_d_n6;
        locals.var_alphas__blk1373_dn7 = assign51000_e65807_d_n7;
        locals.var_alphas__blk1373_dn8 = assign51000_e65807_d_n8;
        locals.var_alphas__blk1373_dn9 = assign51000_e65807_d_n9;
        locals.var_alphas__blk1373_rv = 0.0;

        let (assign51010_e65814, assign51010_e65814_d_n4, assign51010_e65814_d_n6, assign51010_e65814_d_n7, assign51010_e65814_d_n8, assign51010_e65814_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_rxcor_dc, locals.var_rxcor_dc_dn4, locals.var_rxcor_dc_dn6, locals.var_rxcor_dc_dn7, locals.var_rxcor_dc_dn8, locals.var_rxcor_dc_dn9,)
    } else {
        (locals.var_rxcor__blk1374, locals.var_rxcor__blk1374_dn4, locals.var_rxcor__blk1374_dn6, locals.var_rxcor__blk1374_dn7, locals.var_rxcor__blk1374_dn8, locals.var_rxcor__blk1374_dn9,)
    }
};
        locals.var_rxcor__blk1374 = assign51010_e65814;
        locals.var_rxcor__blk1374_dn4 = assign51010_e65814_d_n4;
        locals.var_rxcor__blk1374_dn6 = assign51010_e65814_d_n6;
        locals.var_rxcor__blk1374_dn7 = assign51010_e65814_d_n7;
        locals.var_rxcor__blk1374_dn8 = assign51010_e65814_d_n8;
        locals.var_rxcor__blk1374_dn9 = assign51010_e65814_d_n9;
        locals.var_rxcor__blk1374_rv = 0.0;

        let (assign51020_e65821, assign51020_e65821_d_n4, assign51020_e65821_d_n6, assign51020_e65821_d_n7, assign51020_e65821_d_n8, assign51020_e65821_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xgs_dc, locals.var_xgs_dc_dn4, locals.var_xgs_dc_dn6, locals.var_xgs_dc_dn7, locals.var_xgs_dc_dn8, locals.var_xgs_dc_dn9,)
    } else {
        (locals.var_xgs__blk1375, locals.var_xgs__blk1375_dn4, locals.var_xgs__blk1375_dn6, locals.var_xgs__blk1375_dn7, locals.var_xgs__blk1375_dn8, locals.var_xgs__blk1375_dn9,)
    }
};
        locals.var_xgs__blk1375 = assign51020_e65821;
        locals.var_xgs__blk1375_dn4 = assign51020_e65821_d_n4;
        locals.var_xgs__blk1375_dn6 = assign51020_e65821_d_n6;
        locals.var_xgs__blk1375_dn7 = assign51020_e65821_d_n7;
        locals.var_xgs__blk1375_dn8 = assign51020_e65821_d_n8;
        locals.var_xgs__blk1375_dn9 = assign51020_e65821_d_n9;
        locals.var_xgs__blk1375_rv = 0.0;

        let (assign51030_e65828, assign51030_e65828_d_n4, assign51030_e65828_d_n6, assign51030_e65828_d_n7, assign51030_e65828_d_n8, assign51030_e65828_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_qis_dc, locals.var_qis_dc_dn4, locals.var_qis_dc_dn6, locals.var_qis_dc_dn7, locals.var_qis_dc_dn8, locals.var_qis_dc_dn9,)
    } else {
        (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9,)
    }
};
        locals.var_qis__blk1376 = assign51030_e65828;
        locals.var_qis__blk1376_dn4 = assign51030_e65828_d_n4;
        locals.var_qis__blk1376_dn6 = assign51030_e65828_d_n6;
        locals.var_qis__blk1376_dn7 = assign51030_e65828_d_n7;
        locals.var_qis__blk1376_dn8 = assign51030_e65828_d_n8;
        locals.var_qis__blk1376_dn9 = assign51030_e65828_d_n9;
        locals.var_qis__blk1376_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51040_e65835, assign51040_e65835_d_n4, assign51040_e65835_d_n6, assign51040_e65835_d_n7, assign51040_e65835_d_n8, assign51040_e65835_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_qbs_dc, locals.var_qbs_dc_dn4, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8, locals.var_qbs_dc_dn9,)
    } else {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    }
};
        locals.var_qbs__blk1377 = assign51040_e65835;
        locals.var_qbs__blk1377_dn4 = assign51040_e65835_d_n4;
        locals.var_qbs__blk1377_dn6 = assign51040_e65835_d_n6;
        locals.var_qbs__blk1377_dn7 = assign51040_e65835_d_n7;
        locals.var_qbs__blk1377_dn8 = assign51040_e65835_d_n8;
        locals.var_qbs__blk1377_dn9 = assign51040_e65835_d_n9;
        locals.var_qbs__blk1377_rv = 0.0;

        let (assign51050_e65842, assign51050_e65842_d_n4, assign51050_e65842_d_n6, assign51050_e65842_d_n7, assign51050_e65842_d_n8, assign51050_e65842_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_rhob_dc, locals.var_rhob_dc_dn4, locals.var_rhob_dc_dn6, locals.var_rhob_dc_dn7, locals.var_rhob_dc_dn8, locals.var_rhob_dc_dn9,)
    } else {
        (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9,)
    }
};
        locals.var_rhob__blk1378 = assign51050_e65842;
        locals.var_rhob__blk1378_dn4 = assign51050_e65842_d_n4;
        locals.var_rhob__blk1378_dn6 = assign51050_e65842_d_n6;
        locals.var_rhob__blk1378_dn7 = assign51050_e65842_d_n7;
        locals.var_rhob__blk1378_dn8 = assign51050_e65842_d_n8;
        locals.var_rhob__blk1378_dn9 = assign51050_e65842_d_n9;
        locals.var_rhob__blk1378_rv = 0.0;

        let (assign51060_e65849, assign51060_e65849_d_n4, assign51060_e65849_d_n6, assign51060_e65849_d_n7, assign51060_e65849_d_n8, assign51060_e65849_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_rhog_dc, locals.var_rhog_dc_dn4, locals.var_rhog_dc_dn6, locals.var_rhog_dc_dn7, locals.var_rhog_dc_dn8, locals.var_rhog_dc_dn9,)
    } else {
        (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9,)
    }
};
        locals.var_rhog__blk1379 = assign51060_e65849;
        locals.var_rhog__blk1379_dn4 = assign51060_e65849_d_n4;
        locals.var_rhog__blk1379_dn6 = assign51060_e65849_d_n6;
        locals.var_rhog__blk1379_dn7 = assign51060_e65849_d_n7;
        locals.var_rhog__blk1379_dn8 = assign51060_e65849_d_n8;
        locals.var_rhog__blk1379_dn9 = assign51060_e65849_d_n9;
        locals.var_rhog__blk1379_rv = 0.0;

        let (assign51070_e65856, assign51070_e65856_d_n4, assign51070_e65856_d_n6, assign51070_e65856_d_n7, assign51070_e65856_d_n8, assign51070_e65856_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_gmobs_dc, locals.var_gmobs_dc_dn4, locals.var_gmobs_dc_dn6, locals.var_gmobs_dc_dn7, locals.var_gmobs_dc_dn8, locals.var_gmobs_dc_dn9,)
    } else {
        (locals.var_gmobs__blk1383, locals.var_gmobs__blk1383_dn4, locals.var_gmobs__blk1383_dn6, locals.var_gmobs__blk1383_dn7, locals.var_gmobs__blk1383_dn8, locals.var_gmobs__blk1383_dn9,)
    }
};
        locals.var_gmobs__blk1383 = assign51070_e65856;
        locals.var_gmobs__blk1383_dn4 = assign51070_e65856_d_n4;
        locals.var_gmobs__blk1383_dn6 = assign51070_e65856_d_n6;
        locals.var_gmobs__blk1383_dn7 = assign51070_e65856_d_n7;
        locals.var_gmobs__blk1383_dn8 = assign51070_e65856_d_n8;
        locals.var_gmobs__blk1383_dn9 = assign51070_e65856_d_n9;
        locals.var_gmobs__blk1383_rv = 0.0;

        let (assign51080_e65863, assign51080_e65863_d_n4, assign51080_e65863_d_n6, assign51080_e65863_d_n7, assign51080_e65863_d_n8, assign51080_e65863_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xitsb_dc, locals.var_xitsb_dc_dn4, locals.var_xitsb_dc_dn6, locals.var_xitsb_dc_dn7, locals.var_xitsb_dc_dn8, locals.var_xitsb_dc_dn9,)
    } else {
        (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9,)
    }
};
        locals.var_xitsb__blk1384 = assign51080_e65863;
        locals.var_xitsb__blk1384_dn4 = assign51080_e65863_d_n4;
        locals.var_xitsb__blk1384_dn6 = assign51080_e65863_d_n6;
        locals.var_xitsb__blk1384_dn7 = assign51080_e65863_d_n7;
        locals.var_xitsb__blk1384_dn8 = assign51080_e65863_d_n8;
        locals.var_xitsb__blk1384_dn9 = assign51080_e65863_d_n9;
        locals.var_xitsb__blk1384_rv = 0.0;

        let (assign51090_e65870, assign51090_e65870_d_n4, assign51090_e65870_d_n6, assign51090_e65870_d_n7, assign51090_e65870_d_n8, assign51090_e65870_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_factheta_dc, locals.var_factheta_dc_dn4, locals.var_factheta_dc_dn6, locals.var_factheta_dc_dn7, locals.var_factheta_dc_dn8, locals.var_factheta_dc_dn9,)
    } else {
        (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9,)
    }
};
        locals.var_factheta__blk1386 = assign51090_e65870;
        locals.var_factheta__blk1386_dn4 = assign51090_e65870_d_n4;
        locals.var_factheta__blk1386_dn6 = assign51090_e65870_d_n6;
        locals.var_factheta__blk1386_dn7 = assign51090_e65870_d_n7;
        locals.var_factheta__blk1386_dn8 = assign51090_e65870_d_n8;
        locals.var_factheta__blk1386_dn9 = assign51090_e65870_d_n9;
        locals.var_factheta__blk1386_rv = 0.0;

        let (assign51110_e65881, assign51110_e65881_d_n4,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_thesat_t, locals.var_thesat_t_dn4,)
    } else {
        (locals.var_thesatloc__blk1319, locals.var_thesatloc__blk1319_dn4,)
    }
};
        locals.var_thesatloc__blk1319 = assign51110_e65881;
        locals.var_thesatloc__blk1319_dn4 = assign51110_e65881_d_n4;
        locals.var_thesatloc__blk1319_rv = 0.0;

        let (assign51120_e65885,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_ar,)
    } else {
        (locals.var_arloc__blk1320,)
    }
};
        locals.var_arloc__blk1320 = assign51120_e65885;
        locals.var_arloc__blk1320_rv = 0.0;

        let assign51130_e65888: f64 = if p.p48 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign51130_e65888;
        locals.var_guard1500_rv = 0.0;

        let (assign51140_e65894, assign51140_e65894_d_n4,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1500 != 0.0)) {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4,)
    } else {
        (locals.var_thesatloc__blk1319, locals.var_thesatloc__blk1319_dn4,)
    }
};
        locals.var_thesatloc__blk1319 = assign51140_e65894;
        locals.var_thesatloc__blk1319_dn4 = assign51140_e65894_d_n4;
        locals.var_thesatloc__blk1319_rv = 0.0;

        let (assign51150_e65900,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1500 != 0.0)) {
        (locals.var_arac,)
    } else {
        (locals.var_arloc__blk1320,)
    }
};
        locals.var_arloc__blk1320 = assign51150_e65900;
        locals.var_arloc__blk1320_rv = 0.0;

        let (assign51160_e65904, assign51160_e65904_d_n4, assign51160_e65904_d_n6, assign51160_e65904_d_n7, assign51160_e65904_d_n8, assign51160_e65904_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesat1__blk1388, locals.var_thesat1__blk1388_dn4, locals.var_thesat1__blk1388_dn6, locals.var_thesat1__blk1388_dn7, locals.var_thesat1__blk1388_dn8, locals.var_thesat1__blk1388_dn9,)
    }
};
        locals.var_thesat1__blk1388 = assign51160_e65904;
        locals.var_thesat1__blk1388_dn4 = assign51160_e65904_d_n4;
        locals.var_thesat1__blk1388_dn6 = assign51160_e65904_d_n6;
        locals.var_thesat1__blk1388_dn7 = assign51160_e65904_d_n7;
        locals.var_thesat1__blk1388_dn8 = assign51160_e65904_d_n8;
        locals.var_thesat1__blk1388_dn9 = assign51160_e65904_d_n9;
        locals.var_thesat1__blk1388_rv = 0.0;

        let (assign51170_e65910, assign51170_e65910_d_n4, assign51170_e65910_d_n6, assign51170_e65910_d_n7, assign51170_e65910_d_n8, assign51170_e65910_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        let assign51170_e65908: f64 = (locals.var_phit1__blk1339 * 4.60517018598809);
        (assign51170_e65908, (locals.var_phit1__blk1339_dn4 * 4.60517018598809), (locals.var_phit1__blk1339_dn6 * 4.60517018598809), (locals.var_phit1__blk1339_dn7 * 4.60517018598809), (locals.var_phit1__blk1339_dn8 * 4.60517018598809), (locals.var_phit1__blk1339_dn9 * 4.60517018598809),)
    } else {
        (locals.var_vdsat_lim__blk1387, locals.var_vdsat_lim__blk1387_dn4, locals.var_vdsat_lim__blk1387_dn6, locals.var_vdsat_lim__blk1387_dn7, locals.var_vdsat_lim__blk1387_dn8, locals.var_vdsat_lim__blk1387_dn9,)
    }
};
        locals.var_vdsat_lim__blk1387 = assign51170_e65910;
        locals.var_vdsat_lim__blk1387_dn4 = assign51170_e65910_d_n4;
        locals.var_vdsat_lim__blk1387_dn6 = assign51170_e65910_d_n6;
        locals.var_vdsat_lim__blk1387_dn7 = assign51170_e65910_d_n7;
        locals.var_vdsat_lim__blk1387_dn8 = assign51170_e65910_d_n8;
        locals.var_vdsat_lim__blk1387_dn9 = assign51170_e65910_d_n9;
        locals.var_vdsat_lim__blk1387_rv = 0.0;

        let (assign51180_e65914, assign51180_e65914_d_n4, assign51180_e65914_d_n6, assign51180_e65914_d_n7, assign51180_e65914_d_n8, assign51180_e65914_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_vdsat_lim__blk1387, locals.var_vdsat_lim__blk1387_dn4, locals.var_vdsat_lim__blk1387_dn6, locals.var_vdsat_lim__blk1387_dn7, locals.var_vdsat_lim__blk1387_dn8, locals.var_vdsat_lim__blk1387_dn9,)
    } else {
        (locals.var_v_dsat__blk1404, locals.var_v_dsat__blk1404_dn4, locals.var_v_dsat__blk1404_dn6, locals.var_v_dsat__blk1404_dn7, locals.var_v_dsat__blk1404_dn8, locals.var_v_dsat__blk1404_dn9,)
    }
};
        locals.var_v_dsat__blk1404 = assign51180_e65914;
        locals.var_v_dsat__blk1404_dn4 = assign51180_e65914_d_n4;
        locals.var_v_dsat__blk1404_dn6 = assign51180_e65914_d_n6;
        locals.var_v_dsat__blk1404_dn7 = assign51180_e65914_d_n7;
        locals.var_v_dsat__blk1404_dn8 = assign51180_e65914_d_n8;
        locals.var_v_dsat__blk1404_dn9 = assign51180_e65914_d_n9;
        locals.var_v_dsat__blk1404_rv = 0.0;

        let (assign51190_e65918, assign51190_e65918_d_n4, assign51190_e65918_d_n6, assign51190_e65918_d_n7, assign51190_e65918_d_n8, assign51190_e65918_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_v_ds, 0.0, 0.0, locals.var_v_ds_dn7, locals.var_v_ds_dn8, 0.0,)
    } else {
        (locals.var_vdse__blk1405, locals.var_vdse__blk1405_dn4, locals.var_vdse__blk1405_dn6, locals.var_vdse__blk1405_dn7, locals.var_vdse__blk1405_dn8, locals.var_vdse__blk1405_dn9,)
    }
};
        locals.var_vdse__blk1405 = assign51190_e65918;
        locals.var_vdse__blk1405_dn4 = assign51190_e65918_d_n4;
        locals.var_vdse__blk1405_dn6 = assign51190_e65918_d_n6;
        locals.var_vdse__blk1405_dn7 = assign51190_e65918_d_n7;
        locals.var_vdse__blk1405_dn8 = assign51190_e65918_d_n8;
        locals.var_vdse__blk1405_dn9 = assign51190_e65918_d_n9;
        locals.var_vdse__blk1405_rv = 0.0;

        let (assign51200_e65924, assign51200_e65924_d_n4, assign51200_e65924_d_n6, assign51200_e65924_d_n7, assign51200_e65924_d_n8, assign51200_e65924_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        let assign51200_e65922: f64 = (locals.var_v_ds * locals.var_inv_phit1__blk1340);
        (assign51200_e65922, (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn4), (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn6), ((locals.var_v_ds_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_v_ds_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn8)), (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn9),)
    } else {
        (locals.var_udse__blk1406, locals.var_udse__blk1406_dn4, locals.var_udse__blk1406_dn6, locals.var_udse__blk1406_dn7, locals.var_udse__blk1406_dn8, locals.var_udse__blk1406_dn9,)
    }
};
        locals.var_udse__blk1406 = assign51200_e65924;
        locals.var_udse__blk1406_dn4 = assign51200_e65924_d_n4;
        locals.var_udse__blk1406_dn6 = assign51200_e65924_d_n6;
        locals.var_udse__blk1406_dn7 = assign51200_e65924_d_n7;
        locals.var_udse__blk1406_dn8 = assign51200_e65924_d_n8;
        locals.var_udse__blk1406_dn9 = assign51200_e65924_d_n9;
        locals.var_udse__blk1406_rv = 0.0;

        let (assign51210_e65928, assign51210_e65928_d_n4, assign51210_e65928_d_n6, assign51210_e65928_d_n7, assign51210_e65928_d_n8, assign51210_e65928_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    } else {
        (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9,)
    }
};
        locals.var_x_d__blk1410 = assign51210_e65928;
        locals.var_x_d__blk1410_dn4 = assign51210_e65928_d_n4;
        locals.var_x_d__blk1410_dn6 = assign51210_e65928_d_n6;
        locals.var_x_d__blk1410_dn7 = assign51210_e65928_d_n7;
        locals.var_x_d__blk1410_dn8 = assign51210_e65928_d_n8;
        locals.var_x_d__blk1410_dn9 = assign51210_e65928_d_n9;
        locals.var_x_d__blk1410_rv = 0.0;

        let (assign51220_e65932, assign51220_e65932_d_n4, assign51220_e65932_d_n6, assign51220_e65932_d_n7, assign51220_e65932_d_n8, assign51220_e65932_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9,)
    }
};
        locals.var_x_ds__blk1411 = assign51220_e65932;
        locals.var_x_ds__blk1411_dn4 = assign51220_e65932_d_n4;
        locals.var_x_ds__blk1411_dn6 = assign51220_e65932_d_n6;
        locals.var_x_ds__blk1411_dn7 = assign51220_e65932_d_n7;
        locals.var_x_ds__blk1411_dn8 = assign51220_e65932_d_n8;
        locals.var_x_ds__blk1411_dn9 = assign51220_e65932_d_n9;
        locals.var_x_ds__blk1411_rv = 0.0;

        let (assign51230_e65936, assign51230_e65936_d_n4, assign51230_e65936_d_n6, assign51230_e65936_d_n7, assign51230_e65936_d_n8, assign51230_e65936_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1414, locals.var_dps__blk1414_dn4, locals.var_dps__blk1414_dn6, locals.var_dps__blk1414_dn7, locals.var_dps__blk1414_dn8, locals.var_dps__blk1414_dn9,)
    }
};
        locals.var_dps__blk1414 = assign51230_e65936;
        locals.var_dps__blk1414_dn4 = assign51230_e65936_d_n4;
        locals.var_dps__blk1414_dn6 = assign51230_e65936_d_n6;
        locals.var_dps__blk1414_dn7 = assign51230_e65936_d_n7;
        locals.var_dps__blk1414_dn8 = assign51230_e65936_d_n8;
        locals.var_dps__blk1414_dn9 = assign51230_e65936_d_n9;
        locals.var_dps__blk1414_rv = 0.0;

        let (assign51240_e65940, assign51240_e65940_d_n4, assign51240_e65940_d_n6, assign51240_e65940_d_n7, assign51240_e65940_d_n8, assign51240_e65940_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    } else {
        (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9,)
    }
};
        locals.var_ed__blk1416 = assign51240_e65940;
        locals.var_ed__blk1416_dn4 = assign51240_e65940_d_n4;
        locals.var_ed__blk1416_dn6 = assign51240_e65940_d_n6;
        locals.var_ed__blk1416_dn7 = assign51240_e65940_d_n7;
        locals.var_ed__blk1416_dn8 = assign51240_e65940_d_n8;
        locals.var_ed__blk1416_dn9 = assign51240_e65940_d_n9;
        locals.var_ed__blk1416_rv = 0.0;

        let (assign51250_e65944, assign51250_e65944_d_n4, assign51250_e65944_d_n6, assign51250_e65944_d_n7, assign51250_e65944_d_n8, assign51250_e65944_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    } else {
        (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9,)
    }
};
        locals.var_pd__blk1417 = assign51250_e65944;
        locals.var_pd__blk1417_dn4 = assign51250_e65944_d_n4;
        locals.var_pd__blk1417_dn6 = assign51250_e65944_d_n6;
        locals.var_pd__blk1417_dn7 = assign51250_e65944_d_n7;
        locals.var_pd__blk1417_dn8 = assign51250_e65944_d_n8;
        locals.var_pd__blk1417_dn9 = assign51250_e65944_d_n9;
        locals.var_pd__blk1417_rv = 0.0;

        let (assign51260_e65948, assign51260_e65948_d_n4, assign51260_e65948_d_n6, assign51260_e65948_d_n7, assign51260_e65948_d_n8, assign51260_e65948_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    } else {
        (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9,)
    }
};
        locals.var_dd__blk1419 = assign51260_e65948;
        locals.var_dd__blk1419_dn4 = assign51260_e65948_d_n4;
        locals.var_dd__blk1419_dn6 = assign51260_e65948_d_n6;
        locals.var_dd__blk1419_dn7 = assign51260_e65948_d_n7;
        locals.var_dd__blk1419_dn8 = assign51260_e65948_d_n8;
        locals.var_dd__blk1419_dn9 = assign51260_e65948_d_n9;
        locals.var_dd__blk1419_rv = 0.0;

        let (assign51270_e65952, assign51270_e65952_d_n4, assign51270_e65952_d_n6, assign51270_e65952_d_n7, assign51270_e65952_d_n8, assign51270_e65952_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    } else {
        (locals.var_qbd__blk1420, locals.var_qbd__blk1420_dn4, locals.var_qbd__blk1420_dn6, locals.var_qbd__blk1420_dn7, locals.var_qbd__blk1420_dn8, locals.var_qbd__blk1420_dn9,)
    }
};
        locals.var_qbd__blk1420 = assign51270_e65952;
        locals.var_qbd__blk1420_dn4 = assign51270_e65952_d_n4;
        locals.var_qbd__blk1420_dn6 = assign51270_e65952_d_n6;
        locals.var_qbd__blk1420_dn7 = assign51270_e65952_d_n7;
        locals.var_qbd__blk1420_dn8 = assign51270_e65952_d_n8;
        locals.var_qbd__blk1420_dn9 = assign51270_e65952_d_n9;
        locals.var_qbd__blk1420_rv = 0.0;

        let (assign51280_e65956, assign51280_e65956_d_n4, assign51280_e65956_d_n6, assign51280_e65956_d_n7, assign51280_e65956_d_n8, assign51280_e65956_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    } else {
        (locals.var_x_m__blk1421, locals.var_x_m__blk1421_dn4, locals.var_x_m__blk1421_dn6, locals.var_x_m__blk1421_dn7, locals.var_x_m__blk1421_dn8, locals.var_x_m__blk1421_dn9,)
    }
};
        locals.var_x_m__blk1421 = assign51280_e65956;
        locals.var_x_m__blk1421_dn4 = assign51280_e65956_d_n4;
        locals.var_x_m__blk1421_dn6 = assign51280_e65956_d_n6;
        locals.var_x_m__blk1421_dn7 = assign51280_e65956_d_n7;
        locals.var_x_m__blk1421_dn8 = assign51280_e65956_d_n8;
        locals.var_x_m__blk1421_dn9 = assign51280_e65956_d_n9;
        locals.var_x_m__blk1421_rv = 0.0;

        let (assign51290_e65960, assign51290_e65960_d_n4, assign51290_e65960_d_n6, assign51290_e65960_d_n7, assign51290_e65960_d_n8, assign51290_e65960_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    } else {
        (locals.var_em__blk1422, locals.var_em__blk1422_dn4, locals.var_em__blk1422_dn6, locals.var_em__blk1422_dn7, locals.var_em__blk1422_dn8, locals.var_em__blk1422_dn9,)
    }
};
        locals.var_em__blk1422 = assign51290_e65960;
        locals.var_em__blk1422_dn4 = assign51290_e65960_d_n4;
        locals.var_em__blk1422_dn6 = assign51290_e65960_d_n6;
        locals.var_em__blk1422_dn7 = assign51290_e65960_d_n7;
        locals.var_em__blk1422_dn8 = assign51290_e65960_d_n8;
        locals.var_em__blk1422_dn9 = assign51290_e65960_d_n9;
        locals.var_em__blk1422_rv = 0.0;

        let (assign51300_e65964, assign51300_e65964_d_n4, assign51300_e65964_d_n6, assign51300_e65964_d_n7, assign51300_e65964_d_n8, assign51300_e65964_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    } else {
        (locals.var_dm__blk1424, locals.var_dm__blk1424_dn4, locals.var_dm__blk1424_dn6, locals.var_dm__blk1424_dn7, locals.var_dm__blk1424_dn8, locals.var_dm__blk1424_dn9,)
    }
};
        locals.var_dm__blk1424 = assign51300_e65964;
        locals.var_dm__blk1424_dn4 = assign51300_e65964_d_n4;
        locals.var_dm__blk1424_dn6 = assign51300_e65964_d_n6;
        locals.var_dm__blk1424_dn7 = assign51300_e65964_d_n7;
        locals.var_dm__blk1424_dn8 = assign51300_e65964_d_n8;
        locals.var_dm__blk1424_dn9 = assign51300_e65964_d_n9;
        locals.var_dm__blk1424_rv = 0.0;

        let (assign51310_e65968, assign51310_e65968_d_n4, assign51310_e65968_d_n6, assign51310_e65968_d_n7, assign51310_e65968_d_n8, assign51310_e65968_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    } else {
        (locals.var_pm__blk1425, locals.var_pm__blk1425_dn4, locals.var_pm__blk1425_dn6, locals.var_pm__blk1425_dn7, locals.var_pm__blk1425_dn8, locals.var_pm__blk1425_dn9,)
    }
};
        locals.var_pm__blk1425 = assign51310_e65968;
        locals.var_pm__blk1425_dn4 = assign51310_e65968_d_n4;
        locals.var_pm__blk1425_dn6 = assign51310_e65968_d_n6;
        locals.var_pm__blk1425_dn7 = assign51310_e65968_d_n7;
        locals.var_pm__blk1425_dn8 = assign51310_e65968_d_n8;
        locals.var_pm__blk1425_dn9 = assign51310_e65968_d_n9;
        locals.var_pm__blk1425_rv = 0.0;

        let (assign51320_e65974, assign51320_e65974_d_n4, assign51320_e65974_d_n6, assign51320_e65974_d_n7, assign51320_e65974_d_n8, assign51320_e65974_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        let assign51320_e65972: f64 = (locals.var_xg__blk1343 - locals.var_x_s__blk1363);
        (assign51320_e65972, (locals.var_xg__blk1343_dn4 - locals.var_x_s__blk1363_dn4), (locals.var_xg__blk1343_dn6 - locals.var_x_s__blk1363_dn6), (locals.var_xg__blk1343_dn7 - locals.var_x_s__blk1363_dn7), (locals.var_xg__blk1343_dn8 - locals.var_x_s__blk1363_dn8), (locals.var_xg__blk1343_dn9 - locals.var_x_s__blk1363_dn9),)
    } else {
        (locals.var_xgm__blk1426, locals.var_xgm__blk1426_dn4, locals.var_xgm__blk1426_dn6, locals.var_xgm__blk1426_dn7, locals.var_xgm__blk1426_dn8, locals.var_xgm__blk1426_dn9,)
    }
};
        locals.var_xgm__blk1426 = assign51320_e65974;
        locals.var_xgm__blk1426_dn4 = assign51320_e65974_d_n4;
        locals.var_xgm__blk1426_dn6 = assign51320_e65974_d_n6;
        locals.var_xgm__blk1426_dn7 = assign51320_e65974_d_n7;
        locals.var_xgm__blk1426_dn8 = assign51320_e65974_d_n8;
        locals.var_xgm__blk1426_dn9 = assign51320_e65974_d_n9;
        locals.var_xgm__blk1426_rv = 0.0;

        let (assign51330_e65978, assign51330_e65978_d_n4, assign51330_e65978_d_n6, assign51330_e65978_d_n7, assign51330_e65978_d_n8, assign51330_e65978_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eta_p__blk1427, locals.var_eta_p__blk1427_dn4, locals.var_eta_p__blk1427_dn6, locals.var_eta_p__blk1427_dn7, locals.var_eta_p__blk1427_dn8, locals.var_eta_p__blk1427_dn9,)
    }
};
        locals.var_eta_p__blk1427 = assign51330_e65978;
        locals.var_eta_p__blk1427_dn4 = assign51330_e65978_d_n4;
        locals.var_eta_p__blk1427_dn6 = assign51330_e65978_d_n6;
        locals.var_eta_p__blk1427_dn7 = assign51330_e65978_d_n7;
        locals.var_eta_p__blk1427_dn8 = assign51330_e65978_d_n8;
        locals.var_eta_p__blk1427_dn9 = assign51330_e65978_d_n9;
        locals.var_eta_p__blk1427_rv = 0.0;

        let (assign51340_e65982, assign51340_e65982_d_n4, assign51340_e65982_d_n6, assign51340_e65982_d_n7, assign51340_e65982_d_n8, assign51340_e65982_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_alpha__blk1429, locals.var_alpha__blk1429_dn4, locals.var_alpha__blk1429_dn6, locals.var_alpha__blk1429_dn7, locals.var_alpha__blk1429_dn8, locals.var_alpha__blk1429_dn9,)
    }
};
        locals.var_alpha__blk1429 = assign51340_e65982;
        locals.var_alpha__blk1429_dn4 = assign51340_e65982_d_n4;
        locals.var_alpha__blk1429_dn6 = assign51340_e65982_d_n6;
        locals.var_alpha__blk1429_dn7 = assign51340_e65982_d_n7;
        locals.var_alpha__blk1429_dn8 = assign51340_e65982_d_n8;
        locals.var_alpha__blk1429_dn9 = assign51340_e65982_d_n9;
        locals.var_alpha__blk1429_rv = 0.0;

        let (assign51350_e65986, assign51350_e65986_d_n4, assign51350_e65986_d_n6, assign51350_e65986_d_n7, assign51350_e65986_d_n8, assign51350_e65986_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sqm__blk1428, locals.var_sqm__blk1428_dn4, locals.var_sqm__blk1428_dn6, locals.var_sqm__blk1428_dn7, locals.var_sqm__blk1428_dn8, locals.var_sqm__blk1428_dn9,)
    }
};
        locals.var_sqm__blk1428 = assign51350_e65986;
        locals.var_sqm__blk1428_dn4 = assign51350_e65986_d_n4;
        locals.var_sqm__blk1428_dn6 = assign51350_e65986_d_n6;
        locals.var_sqm__blk1428_dn7 = assign51350_e65986_d_n7;
        locals.var_sqm__blk1428_dn8 = assign51350_e65986_d_n8;
        locals.var_sqm__blk1428_dn9 = assign51350_e65986_d_n9;
        locals.var_sqm__blk1428_rv = 0.0;

        let (assign51360_e65990, assign51360_e65990_d_n4, assign51360_e65990_d_n6, assign51360_e65990_d_n7, assign51360_e65990_d_n8, assign51360_e65990_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9,)
    } else {
        (locals.var_qim__blk1438, locals.var_qim__blk1438_dn4, locals.var_qim__blk1438_dn6, locals.var_qim__blk1438_dn7, locals.var_qim__blk1438_dn8, locals.var_qim__blk1438_dn9,)
    }
};
        locals.var_qim__blk1438 = assign51360_e65990;
        locals.var_qim__blk1438_dn4 = assign51360_e65990_d_n4;
        locals.var_qim__blk1438_dn6 = assign51360_e65990_d_n6;
        locals.var_qim__blk1438_dn7 = assign51360_e65990_d_n7;
        locals.var_qim__blk1438_dn8 = assign51360_e65990_d_n8;
        locals.var_qim__blk1438_dn9 = assign51360_e65990_d_n9;
        locals.var_qim__blk1438_rv = 0.0;

        let (assign51370_e65996, assign51370_e65996_d_n4, assign51370_e65996_d_n6, assign51370_e65996_d_n7, assign51370_e65996_d_n8, assign51370_e65996_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        let assign51370_e65994: f64 = (locals.var_xgm__blk1426 * locals.var_phit1__blk1339);
        (assign51370_e65994, ((locals.var_xgm__blk1426_dn4 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn4)), ((locals.var_xgm__blk1426_dn6 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn6)), ((locals.var_xgm__blk1426_dn7 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn7)), ((locals.var_xgm__blk1426_dn8 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn8)), ((locals.var_xgm__blk1426_dn9 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn9)),)
    } else {
        (locals.var_qeff1__blk1442, locals.var_qeff1__blk1442_dn4, locals.var_qeff1__blk1442_dn6, locals.var_qeff1__blk1442_dn7, locals.var_qeff1__blk1442_dn8, locals.var_qeff1__blk1442_dn9,)
    }
};
        locals.var_qeff1__blk1442 = assign51370_e65996;
        locals.var_qeff1__blk1442_dn4 = assign51370_e65996_d_n4;
        locals.var_qeff1__blk1442_dn6 = assign51370_e65996_d_n6;
        locals.var_qeff1__blk1442_dn7 = assign51370_e65996_d_n7;
        locals.var_qeff1__blk1442_dn8 = assign51370_e65996_d_n8;
        locals.var_qeff1__blk1442_dn9 = assign51370_e65996_d_n9;
        locals.var_qeff1__blk1442_rv = 0.0;

        let (assign51380_e66000, assign51380_e66000_d_n4, assign51380_e66000_d_n6, assign51380_e66000_d_n7, assign51380_e66000_d_n8, assign51380_e66000_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qim1__blk1439, locals.var_qim1__blk1439_dn4, locals.var_qim1__blk1439_dn6, locals.var_qim1__blk1439_dn7, locals.var_qim1__blk1439_dn8, locals.var_qim1__blk1439_dn9,)
    }
};
        locals.var_qim1__blk1439 = assign51380_e66000;
        locals.var_qim1__blk1439_dn4 = assign51380_e66000_d_n4;
        locals.var_qim1__blk1439_dn6 = assign51380_e66000_d_n6;
        locals.var_qim1__blk1439_dn7 = assign51380_e66000_d_n7;
        locals.var_qim1__blk1439_dn8 = assign51380_e66000_d_n8;
        locals.var_qim1__blk1439_dn9 = assign51380_e66000_d_n9;
        locals.var_qim1__blk1439_rv = 0.0;

        let (assign51390_e66004, assign51390_e66004_d_n4, assign51390_e66004_d_n6, assign51390_e66004_d_n7, assign51390_e66004_d_n8, assign51390_e66004_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    } else {
        (locals.var_qbm__blk1440, locals.var_qbm__blk1440_dn4, locals.var_qbm__blk1440_dn6, locals.var_qbm__blk1440_dn7, locals.var_qbm__blk1440_dn8, locals.var_qbm__blk1440_dn9,)
    }
};
        locals.var_qbm__blk1440 = assign51390_e66004;
        locals.var_qbm__blk1440_dn4 = assign51390_e66004_d_n4;
        locals.var_qbm__blk1440_dn6 = assign51390_e66004_d_n6;
        locals.var_qbm__blk1440_dn7 = assign51390_e66004_d_n7;
        locals.var_qbm__blk1440_dn8 = assign51390_e66004_d_n8;
        locals.var_qbm__blk1440_dn9 = assign51390_e66004_d_n9;
        locals.var_qbm__blk1440_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_46(
        locals: &mut StampLocals,
    ) {
        let (assign51400_e66008, assign51400_e66008_d_n4, assign51400_e66008_d_n6, assign51400_e66008_d_n7, assign51400_e66008_d_n8, assign51400_e66008_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_s1__blk1445, locals.var_s1__blk1445_dn4, locals.var_s1__blk1445_dn6, locals.var_s1__blk1445_dn7, locals.var_s1__blk1445_dn8, locals.var_s1__blk1445_dn9,)
    }
};
        locals.var_s1__blk1445 = assign51400_e66008;
        locals.var_s1__blk1445_dn4 = assign51400_e66008_d_n4;
        locals.var_s1__blk1445_dn6 = assign51400_e66008_d_n6;
        locals.var_s1__blk1445_dn7 = assign51400_e66008_d_n7;
        locals.var_s1__blk1445_dn8 = assign51400_e66008_d_n8;
        locals.var_s1__blk1445_dn9 = assign51400_e66008_d_n9;
        locals.var_s1__blk1445_rv = 0.0;

        let (assign51410_e66012, assign51410_e66012_d_n4, assign51410_e66012_d_n6, assign51410_e66012_d_n7, assign51410_e66012_d_n8, assign51410_e66012_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gmob__blk1444, locals.var_gmob__blk1444_dn4, locals.var_gmob__blk1444_dn6, locals.var_gmob__blk1444_dn7, locals.var_gmob__blk1444_dn8, locals.var_gmob__blk1444_dn9,)
    }
};
        locals.var_gmob__blk1444 = assign51410_e66012;
        locals.var_gmob__blk1444_dn4 = assign51410_e66012_d_n4;
        locals.var_gmob__blk1444_dn6 = assign51410_e66012_d_n6;
        locals.var_gmob__blk1444_dn7 = assign51410_e66012_d_n7;
        locals.var_gmob__blk1444_dn8 = assign51410_e66012_d_n8;
        locals.var_gmob__blk1444_dn9 = assign51410_e66012_d_n9;
        locals.var_gmob__blk1444_rv = 0.0;

        let (assign51420_e66016, assign51420_e66016_d_n4, assign51420_e66016_d_n6, assign51420_e66016_d_n7, assign51420_e66016_d_n8, assign51420_e66016_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_thesatloc__blk1319, locals.var_thesatloc__blk1319_dn4, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesateff__blk1447, locals.var_thesateff__blk1447_dn4, locals.var_thesateff__blk1447_dn6, locals.var_thesateff__blk1447_dn7, locals.var_thesateff__blk1447_dn8, locals.var_thesateff__blk1447_dn9,)
    }
};
        locals.var_thesateff__blk1447 = assign51420_e66016;
        locals.var_thesateff__blk1447_dn4 = assign51420_e66016_d_n4;
        locals.var_thesateff__blk1447_dn6 = assign51420_e66016_d_n6;
        locals.var_thesateff__blk1447_dn7 = assign51420_e66016_d_n7;
        locals.var_thesateff__blk1447_dn8 = assign51420_e66016_d_n8;
        locals.var_thesateff__blk1447_dn9 = assign51420_e66016_d_n9;
        locals.var_thesateff__blk1447_rv = 0.0;

        let (assign51430_e66020, assign51430_e66020_d_n4, assign51430_e66020_d_n6, assign51430_e66020_d_n7, assign51430_e66020_d_n8, assign51430_e66020_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qeff1__blk1442, locals.var_qeff1__blk1442_dn4, locals.var_qeff1__blk1442_dn6, locals.var_qeff1__blk1442_dn7, locals.var_qeff1__blk1442_dn8, locals.var_qeff1__blk1442_dn9,)
    } else {
        (locals.var_voxm__blk1446, locals.var_voxm__blk1446_dn4, locals.var_voxm__blk1446_dn6, locals.var_voxm__blk1446_dn7, locals.var_voxm__blk1446_dn8, locals.var_voxm__blk1446_dn9,)
    }
};
        locals.var_voxm__blk1446 = assign51430_e66020;
        locals.var_voxm__blk1446_dn4 = assign51430_e66020_d_n4;
        locals.var_voxm__blk1446_dn6 = assign51430_e66020_d_n6;
        locals.var_voxm__blk1446_dn7 = assign51430_e66020_d_n7;
        locals.var_voxm__blk1446_dn8 = assign51430_e66020_d_n8;
        locals.var_voxm__blk1446_dn9 = assign51430_e66020_d_n9;
        locals.var_voxm__blk1446_rv = 0.0;

        let assign51440_e66023: f64 = if locals.var_xg__blk1343 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1501 = assign51440_e66023;
        locals.var_guard1501_rv = 0.0;

        let assign51450_e66026: f64 = if locals.var_ds__blk1370 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1502 = assign51450_e66026;
        locals.var_guard1502_rv = 0.0;

        let (assign51460_e66036, assign51460_e66036_d_n4, assign51460_e66036_d_n6, assign51460_e66036_d_n7, assign51460_e66036_d_n8, assign51460_e66036_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51460_e66034: f64 = (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386);
        (assign51460_e66034, ((locals.var_thesatloc__blk1319_dn4 * locals.var_factheta__blk1386) + (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn4)), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn6), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn7), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn8), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn9),)
    } else {
        (locals.var_thesateff__blk1447, locals.var_thesateff__blk1447_dn4, locals.var_thesateff__blk1447_dn6, locals.var_thesateff__blk1447_dn7, locals.var_thesateff__blk1447_dn8, locals.var_thesateff__blk1447_dn9,)
    }
};
        locals.var_thesateff__blk1447 = assign51460_e66036;
        locals.var_thesateff__blk1447_dn4 = assign51460_e66036_d_n4;
        locals.var_thesateff__blk1447_dn6 = assign51460_e66036_d_n6;
        locals.var_thesateff__blk1447_dn7 = assign51460_e66036_d_n7;
        locals.var_thesateff__blk1447_dn8 = assign51460_e66036_d_n8;
        locals.var_thesateff__blk1447_dn9 = assign51460_e66036_d_n9;
        locals.var_thesateff__blk1447_rv = 0.0;

        let (assign51470_e66046, assign51470_e66046_d_n4, assign51470_e66046_d_n6, assign51470_e66046_d_n7, assign51470_e66046_d_n8, assign51470_e66046_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51470_e66044: f64 = (locals.var_thesateff__blk1447 / locals.var_gmobs__blk1383);
        (assign51470_e66044, (((locals.var_thesateff__blk1447_dn4 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn4)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn6 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn6)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn7 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn7)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn8 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn8)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn9 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn9)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)),)
    } else {
        (locals.var_thesat1__blk1388, locals.var_thesat1__blk1388_dn4, locals.var_thesat1__blk1388_dn6, locals.var_thesat1__blk1388_dn7, locals.var_thesat1__blk1388_dn8, locals.var_thesat1__blk1388_dn9,)
    }
};
        locals.var_thesat1__blk1388 = assign51470_e66046;
        locals.var_thesat1__blk1388_dn4 = assign51470_e66046_d_n4;
        locals.var_thesat1__blk1388_dn6 = assign51470_e66046_d_n6;
        locals.var_thesat1__blk1388_dn7 = assign51470_e66046_d_n7;
        locals.var_thesat1__blk1388_dn8 = assign51470_e66046_d_n8;
        locals.var_thesat1__blk1388_dn9 = assign51470_e66046_d_n9;
        locals.var_thesat1__blk1388_rv = 0.0;

        let (assign51480_e66058, assign51480_e66058_d_n4, assign51480_e66058_d_n6, assign51480_e66058_d_n7, assign51480_e66058_d_n8, assign51480_e66058_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51480_e66055: f64 = (0.5 * locals.var_gf2__blk1325);
        let assign51480_e66056: f64 = (locals.var_xgs__blk1375 + assign51480_e66055);
        (assign51480_e66056, (locals.var_xgs__blk1375_dn4 + (0.5 * locals.var_gf2__blk1325_dn4)), (locals.var_xgs__blk1375_dn6 + (0.5 * locals.var_gf2__blk1325_dn6)), (locals.var_xgs__blk1375_dn7 + (0.5 * locals.var_gf2__blk1325_dn7)), (locals.var_xgs__blk1375_dn8 + (0.5 * locals.var_gf2__blk1325_dn8)), (locals.var_xgs__blk1375_dn9 + (0.5 * locals.var_gf2__blk1325_dn9)),)
    } else {
        (locals.var_asat__blk1389, locals.var_asat__blk1389_dn4, locals.var_asat__blk1389_dn6, locals.var_asat__blk1389_dn7, locals.var_asat__blk1389_dn8, locals.var_asat__blk1389_dn9,)
    }
};
        locals.var_asat__blk1389 = assign51480_e66058;
        locals.var_asat__blk1389_dn4 = assign51480_e66058_d_n4;
        locals.var_asat__blk1389_dn6 = assign51480_e66058_d_n6;
        locals.var_asat__blk1389_dn7 = assign51480_e66058_d_n7;
        locals.var_asat__blk1389_dn8 = assign51480_e66058_d_n8;
        locals.var_asat__blk1389_dn9 = assign51480_e66058_d_n9;
        locals.var_asat__blk1389_rv = 0.0;

        let (assign51490_e66072, assign51490_e66072_d_n4, assign51490_e66072_d_n6, assign51490_e66072_d_n7, assign51490_e66072_d_n8, assign51490_e66072_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51490_e66066: f64 = (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_asat__blk1389;
        let assign51490_e66068: f64 = (assign51490_e66066 * __rspice_inv_cse_0);
        let assign51490_e66070: f64 = (assign51490_e66068 * __rspice_inv_cse_0);
        (assign51490_e66070, ((((((((locals.var_gf2__blk1325_dn4 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn4)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn4)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn4)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn6 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn6)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn6)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn6)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn7 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn7)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn7)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn7)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn8 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn8)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn8)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn8)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn9 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn9)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn9)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn9)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51490_e66072;
        locals.var_temp__blk949_dn4 = assign51490_e66072_d_n4;
        locals.var_temp__blk949_dn6 = assign51490_e66072_d_n6;
        locals.var_temp__blk949_dn7 = assign51490_e66072_d_n7;
        locals.var_temp__blk949_dn8 = assign51490_e66072_d_n8;
        locals.var_temp__blk949_dn9 = assign51490_e66072_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign51500_e66075: f64 = if locals.var_temp__blk949 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1503 = assign51500_e66075;
        locals.var_guard1503_rv = 0.0;

        let (assign51510_e66087, assign51510_e66087_d_n4, assign51510_e66087_d_n6, assign51510_e66087_d_n7, assign51510_e66087_d_n8, assign51510_e66087_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign51510_e66085: f64 = (1.0 - locals.var_temp__blk949);
        (assign51510_e66085, (-locals.var_temp__blk949_dn4), (-locals.var_temp__blk949_dn6), (-locals.var_temp__blk949_dn7), (-locals.var_temp__blk949_dn8), (-locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign51510_e66087;
        locals.var_temp1_dn4 = assign51510_e66087_d_n4;
        locals.var_temp1_dn6 = assign51510_e66087_d_n6;
        locals.var_temp1_dn7 = assign51510_e66087_d_n7;
        locals.var_temp1_dn8 = assign51510_e66087_d_n8;
        locals.var_temp1_dn9 = assign51510_e66087_d_n9;
        locals.var_temp1_rv = 0.0;

        let assign51520_e66090: f64 = if locals.var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1504 = assign51520_e66090;
        locals.var_guard1504_rv = 0.0;

        let (assign51530_e66102, assign51530_e66102_d_n4, assign51530_e66102_d_n6, assign51530_e66102_d_n7, assign51530_e66102_d_n8, assign51530_e66102_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign51530_e66102;
        locals.var_temp2_dn4 = assign51530_e66102_d_n4;
        locals.var_temp2_dn6 = assign51530_e66102_d_n6;
        locals.var_temp2_dn7 = assign51530_e66102_d_n7;
        locals.var_temp2_dn8 = assign51530_e66102_d_n8;
        locals.var_temp2_dn9 = assign51530_e66102_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign51540_e66118, assign51540_e66118_d_n4, assign51540_e66118_d_n6, assign51540_e66118_d_n7, assign51540_e66118_d_n8, assign51540_e66118_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 == 0.0)) {
        let assign51540_e66115: f64 = (locals.var_temp1).sqrt();
        let assign51540_e66116: f64 = (1.0 - assign51540_e66115);
        (assign51540_e66116, (-(locals.var_temp1_dn4 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn6 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn7 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn8 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn9 / (2.0 * assign51540_e66115))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign51540_e66118;
        locals.var_temp2_dn4 = assign51540_e66118_d_n4;
        locals.var_temp2_dn6 = assign51540_e66118_d_n6;
        locals.var_temp2_dn7 = assign51540_e66118_d_n7;
        locals.var_temp2_dn8 = assign51540_e66118_d_n8;
        locals.var_temp2_dn9 = assign51540_e66118_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign51550_e66131, assign51550_e66131_d_n4, assign51550_e66131_d_n6, assign51550_e66131_d_n7, assign51550_e66131_d_n8, assign51550_e66131_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 == 0.0)) {
        let assign51550_e66129: f64 = (0.5 * locals.var_temp__blk949);
        (assign51550_e66129, (0.5 * locals.var_temp__blk949_dn4), (0.5 * locals.var_temp__blk949_dn6), (0.5 * locals.var_temp__blk949_dn7), (0.5 * locals.var_temp__blk949_dn8), (0.5 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign51550_e66131;
        locals.var_temp2_dn4 = assign51550_e66131_d_n4;
        locals.var_temp2_dn6 = assign51550_e66131_d_n6;
        locals.var_temp2_dn7 = assign51550_e66131_d_n7;
        locals.var_temp2_dn8 = assign51550_e66131_d_n8;
        locals.var_temp2_dn9 = assign51550_e66131_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign51560_e66141, assign51560_e66141_d_n4, assign51560_e66141_d_n6, assign51560_e66141_d_n7, assign51560_e66141_d_n8, assign51560_e66141_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51560_e66139: f64 = (locals.var_temp2 * locals.var_asat__blk1389);
        (assign51560_e66139, ((locals.var_temp2_dn4 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn4)), ((locals.var_temp2_dn6 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn6)), ((locals.var_temp2_dn7 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn7)), ((locals.var_temp2_dn8 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn8)), ((locals.var_temp2_dn9 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn9)),)
    } else {
        (locals.var_x_inf0__blk1390, locals.var_x_inf0__blk1390_dn4, locals.var_x_inf0__blk1390_dn6, locals.var_x_inf0__blk1390_dn7, locals.var_x_inf0__blk1390_dn8, locals.var_x_inf0__blk1390_dn9,)
    }
};
        locals.var_x_inf0__blk1390 = assign51560_e66141;
        locals.var_x_inf0__blk1390_dn4 = assign51560_e66141_d_n4;
        locals.var_x_inf0__blk1390_dn6 = assign51560_e66141_d_n6;
        locals.var_x_inf0__blk1390_dn7 = assign51560_e66141_d_n7;
        locals.var_x_inf0__blk1390_dn8 = assign51560_e66141_d_n8;
        locals.var_x_inf0__blk1390_dn9 = assign51560_e66141_d_n9;
        locals.var_x_inf0__blk1390_rv = 0.0;

        let assign51570_e66148: f64 = if ((locals.var_cs_t > 0.0) && (locals.var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1505 = assign51570_e66148;
        locals.var_guard1505_rv = 0.0;

        let (assign51580_e66162, assign51580_e66162_d_n4, assign51580_e66162_d_n6, assign51580_e66162_d_n7, assign51580_e66162_d_n8, assign51580_e66162_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51580_e66158: f64 = (0.475 * locals.var_phit1__blk1339);
        let assign51580_e66160: f64 = (assign51580_e66158 * locals.var_x_inf0__blk1390);
        (assign51580_e66160, (((0.475 * locals.var_phit1__blk1339_dn4) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn4)), (((0.475 * locals.var_phit1__blk1339_dn6) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn6)), (((0.475 * locals.var_phit1__blk1339_dn7) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn7)), (((0.475 * locals.var_phit1__blk1339_dn8) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn8)), (((0.475 * locals.var_phit1__blk1339_dn9) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn9)),)
    } else {
        (locals.var_midphi0__blk1391, locals.var_midphi0__blk1391_dn4, locals.var_midphi0__blk1391_dn6, locals.var_midphi0__blk1391_dn7, locals.var_midphi0__blk1391_dn8, locals.var_midphi0__blk1391_dn9,)
    }
};
        locals.var_midphi0__blk1391 = assign51580_e66162;
        locals.var_midphi0__blk1391_dn4 = assign51580_e66162_d_n4;
        locals.var_midphi0__blk1391_dn6 = assign51580_e66162_d_n6;
        locals.var_midphi0__blk1391_dn7 = assign51580_e66162_d_n7;
        locals.var_midphi0__blk1391_dn8 = assign51580_e66162_d_n8;
        locals.var_midphi0__blk1391_dn9 = assign51580_e66162_d_n9;
        locals.var_midphi0__blk1391_rv = 0.0;

        let (assign51590_e66176, assign51590_e66176_d_n4, assign51590_e66176_d_n6, assign51590_e66176_d_n7, assign51590_e66176_d_n8, assign51590_e66176_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51590_e66173: f64 = (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391);
        let assign51590_e66174: f64 = (locals.var_qis__blk1376 - assign51590_e66173);
        (assign51590_e66174, (locals.var_qis__blk1376_dn4 - ((locals.var_alphas__blk1373_dn4 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn4))), (locals.var_qis__blk1376_dn6 - ((locals.var_alphas__blk1373_dn6 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn6))), (locals.var_qis__blk1376_dn7 - ((locals.var_alphas__blk1373_dn7 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn7))), (locals.var_qis__blk1376_dn8 - ((locals.var_alphas__blk1373_dn8 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn8))), (locals.var_qis__blk1376_dn9 - ((locals.var_alphas__blk1373_dn9 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn9))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51590_e66176;
        locals.var_temp__blk949_dn4 = assign51590_e66176_d_n4;
        locals.var_temp__blk949_dn6 = assign51590_e66176_d_n6;
        locals.var_temp__blk949_dn7 = assign51590_e66176_d_n7;
        locals.var_temp__blk949_dn8 = assign51590_e66176_d_n8;
        locals.var_temp__blk949_dn9 = assign51590_e66176_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign51600_e66195, assign51600_e66195_d_n4, assign51600_e66195_d_n6, assign51600_e66195_d_n7, assign51600_e66195_d_n8, assign51600_e66195_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51600_e66188: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
        let assign51600_e66190: f64 = (assign51600_e66188 + 1e-12);
        let assign51600_e66191: f64 = (assign51600_e66190).sqrt();
        let assign51600_e66192: f64 = (locals.var_temp__blk949 + assign51600_e66191);
        let assign51600_e66193: f64 = (0.5 * assign51600_e66192);
        (assign51600_e66193, (0.5 * (locals.var_temp__blk949_dn4 + (((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn6 + (((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn7 + (((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn8 + (((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn9 + (((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) / (2.0 * assign51600_e66191)))),)
    } else {
        (locals.var_qisat__blk1392, locals.var_qisat__blk1392_dn4, locals.var_qisat__blk1392_dn6, locals.var_qisat__blk1392_dn7, locals.var_qisat__blk1392_dn8, locals.var_qisat__blk1392_dn9,)
    }
};
        locals.var_qisat__blk1392 = assign51600_e66195;
        locals.var_qisat__blk1392_dn4 = assign51600_e66195_d_n4;
        locals.var_qisat__blk1392_dn6 = assign51600_e66195_d_n6;
        locals.var_qisat__blk1392_dn7 = assign51600_e66195_d_n7;
        locals.var_qisat__blk1392_dn8 = assign51600_e66195_d_n8;
        locals.var_qisat__blk1392_dn9 = assign51600_e66195_d_n9;
        locals.var_qisat__blk1392_rv = 0.0;

        let (assign51610_e66215, assign51610_e66215_d_n4, assign51610_e66215_d_n6, assign51610_e66215_d_n7, assign51610_e66215_d_n8, assign51610_e66215_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51610_e66205: f64 = (locals.var_phit1__blk1339 * locals.var_xgs__blk1375);
        let assign51610_e66207: f64 = (assign51610_e66205 - locals.var_qis__blk1376);
        let assign51610_e66210: f64 = (locals.var_alphas__blk1373 - 1.0);
        let assign51610_e66212: f64 = (assign51610_e66210 * locals.var_midphi0__blk1391);
        let assign51610_e66213: f64 = (assign51610_e66207 + assign51610_e66212);
        (assign51610_e66213, ((((locals.var_phit1__blk1339_dn4 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn4)) - locals.var_qis__blk1376_dn4) + ((locals.var_alphas__blk1373_dn4 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn4))), ((((locals.var_phit1__blk1339_dn6 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn6)) - locals.var_qis__blk1376_dn6) + ((locals.var_alphas__blk1373_dn6 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn6))), ((((locals.var_phit1__blk1339_dn7 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn7)) - locals.var_qis__blk1376_dn7) + ((locals.var_alphas__blk1373_dn7 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn7))), ((((locals.var_phit1__blk1339_dn8 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn8)) - locals.var_qis__blk1376_dn8) + ((locals.var_alphas__blk1373_dn8 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn8))), ((((locals.var_phit1__blk1339_dn9 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn9)) - locals.var_qis__blk1376_dn9) + ((locals.var_alphas__blk1373_dn9 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn9))),)
    } else {
        (locals.var_qbsat__blk1393, locals.var_qbsat__blk1393_dn4, locals.var_qbsat__blk1393_dn6, locals.var_qbsat__blk1393_dn7, locals.var_qbsat__blk1393_dn8, locals.var_qbsat__blk1393_dn9,)
    }
};
        locals.var_qbsat__blk1393 = assign51610_e66215;
        locals.var_qbsat__blk1393_dn4 = assign51610_e66215_d_n4;
        locals.var_qbsat__blk1393_dn6 = assign51610_e66215_d_n6;
        locals.var_qbsat__blk1393_dn7 = assign51610_e66215_d_n7;
        locals.var_qbsat__blk1393_dn8 = assign51610_e66215_d_n8;
        locals.var_qbsat__blk1393_dn9 = assign51610_e66215_d_n9;
        locals.var_qbsat__blk1393_rv = 0.0;

        let (assign51620_e66233, assign51620_e66233_d_n4, assign51620_e66233_d_n6, assign51620_e66233_d_n7, assign51620_e66233_d_n8, assign51620_e66233_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51620_e66226: f64 = (0.5 * locals.var_gf2__blk1325);
        let assign51620_e66228: f64 = (assign51620_e66226 * locals.var_phit1__blk1339);
        let assign51620_e66230: f64 = (assign51620_e66228 / locals.var_qbsat__blk1393);
        let assign51620_e66231: f64 = (1.0 + assign51620_e66230);
        (assign51620_e66231, ((((((0.5 * locals.var_gf2__blk1325_dn4) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn4)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn4)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn6) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn6)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn6)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn7) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn7)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn7)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn8) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn8)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn8)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn9) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn9)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn9)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)),)
    } else {
        (locals.var_alphasat__blk1394, locals.var_alphasat__blk1394_dn4, locals.var_alphasat__blk1394_dn6, locals.var_alphasat__blk1394_dn7, locals.var_alphasat__blk1394_dn8, locals.var_alphasat__blk1394_dn9,)
    }
};
        locals.var_alphasat__blk1394 = assign51620_e66233;
        locals.var_alphasat__blk1394_dn4 = assign51620_e66233_d_n4;
        locals.var_alphasat__blk1394_dn6 = assign51620_e66233_d_n6;
        locals.var_alphasat__blk1394_dn7 = assign51620_e66233_d_n7;
        locals.var_alphasat__blk1394_dn8 = assign51620_e66233_d_n8;
        locals.var_alphasat__blk1394_dn9 = assign51620_e66233_d_n9;
        locals.var_alphasat__blk1394_rv = 0.0;

        let (assign51630_e66247, assign51630_e66247_d_n4, assign51630_e66247_d_n6, assign51630_e66247_d_n7, assign51630_e66247_d_n8, assign51630_e66247_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51630_e66244: f64 = (locals.var_eta_mu * locals.var_qisat__blk1392);
        let assign51630_e66245: f64 = (locals.var_qbsat__blk1393 + assign51630_e66244);
        (assign51630_e66245, (locals.var_qbsat__blk1393_dn4 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn4)), (locals.var_qbsat__blk1393_dn6 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn6)), (locals.var_qbsat__blk1393_dn7 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn7)), (locals.var_qbsat__blk1393_dn8 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn8)), (locals.var_qbsat__blk1393_dn9 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51630_e66247;
        locals.var_temp__blk949_dn4 = assign51630_e66247_d_n4;
        locals.var_temp__blk949_dn6 = assign51630_e66247_d_n6;
        locals.var_temp__blk949_dn7 = assign51630_e66247_d_n7;
        locals.var_temp__blk949_dn8 = assign51630_e66247_d_n8;
        locals.var_temp__blk949_dn9 = assign51630_e66247_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign51640_e66263, assign51640_e66263_d_n4, assign51640_e66263_d_n6, assign51640_e66263_d_n7, assign51640_e66263_d_n8, assign51640_e66263_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51640_e66257: f64 = (locals.var_e_eff0 * locals.var_temp__blk949);
        let assign51640_e66259: f64 = (assign51640_e66257 * locals.var_mue_t);
        let assign51640_e66261: f64 = (assign51640_e66259).powf(locals.var_themu_t);
        (assign51640_e66261, if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * (((locals.var_e_eff0 * locals.var_temp__blk949_dn4) * locals.var_mue_t) + (assign51640_e66257 * locals.var_mue_t_dn4)))) } } else { (assign51640_e66261 * ((locals.var_themu_t_dn4 * (assign51640_e66259).ln()) + (locals.var_themu_t * ((((locals.var_e_eff0 * locals.var_temp__blk949_dn4) * locals.var_mue_t) + (assign51640_e66257 * locals.var_mue_t_dn4)) / assign51640_e66259)))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn6) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn6) * locals.var_mue_t) / assign51640_e66259))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn7) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn7) * locals.var_mue_t) / assign51640_e66259))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn8) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn8) * locals.var_mue_t) / assign51640_e66259))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn9) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn9) * locals.var_mue_t) / assign51640_e66259))) },)
    } else {
        (locals.var_gmobmusat__blk1395, locals.var_gmobmusat__blk1395_dn4, locals.var_gmobmusat__blk1395_dn6, locals.var_gmobmusat__blk1395_dn7, locals.var_gmobmusat__blk1395_dn8, locals.var_gmobmusat__blk1395_dn9,)
    }
};
        locals.var_gmobmusat__blk1395 = assign51640_e66263;
        locals.var_gmobmusat__blk1395_dn4 = assign51640_e66263_d_n4;
        locals.var_gmobmusat__blk1395_dn6 = assign51640_e66263_d_n6;
        locals.var_gmobmusat__blk1395_dn7 = assign51640_e66263_d_n7;
        locals.var_gmobmusat__blk1395_dn8 = assign51640_e66263_d_n8;
        locals.var_gmobmusat__blk1395_dn9 = assign51640_e66263_d_n9;
        locals.var_gmobmusat__blk1395_rv = 0.0;

        let (assign51650_e66285, assign51650_e66285_d_n4, assign51650_e66285_d_n6, assign51650_e66285_d_n7, assign51650_e66285_d_n8, assign51650_e66285_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51650_e66275: f64 = (1.0 - locals.var_eta_mu);
        let assign51650_e66276: f64 = (locals.var_alphasat__blk1394 * assign51650_e66275);
        let assign51650_e66278: f64 = (assign51650_e66276 - 1.0);
        let assign51650_e66279: f64 = (locals.var_themu_t * assign51650_e66278);
        let assign51650_e66281: f64 = (assign51650_e66279 / locals.var_temp__blk949);
        let assign51650_e66283: f64 = (assign51650_e66281 * locals.var_gmobmusat__blk1395);
        (assign51650_e66283, (((((((locals.var_themu_t_dn4 * assign51650_e66278) + (locals.var_themu_t * (locals.var_alphasat__blk1394_dn4 * assign51650_e66275))) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn4)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn6 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn6)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn7 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn7)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn8 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn8)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn9 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign51650_e66285;
        locals.var_temp1_dn4 = assign51650_e66285_d_n4;
        locals.var_temp1_dn6 = assign51650_e66285_d_n6;
        locals.var_temp1_dn7 = assign51650_e66285_d_n7;
        locals.var_temp1_dn8 = assign51650_e66285_d_n8;
        locals.var_temp1_dn9 = assign51650_e66285_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign51660_e66297, assign51660_e66297_d_n4, assign51660_e66297_d_n6, assign51660_e66297_d_n7, assign51660_e66297_d_n8, assign51660_e66297_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51660_e66295: f64 = (locals.var_qisat__blk1392 / locals.var_qbsat__blk1393);
        (assign51660_e66295, (((locals.var_qisat__blk1392_dn4 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn4)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn6 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn6)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn7 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn7)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn8 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn8)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn9 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn9)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51660_e66297;
        locals.var_temp__blk949_dn4 = assign51660_e66297_d_n4;
        locals.var_temp__blk949_dn6 = assign51660_e66297_d_n6;
        locals.var_temp__blk949_dn7 = assign51660_e66297_d_n7;
        locals.var_temp__blk949_dn8 = assign51660_e66297_d_n8;
        locals.var_temp__blk949_dn9 = assign51660_e66297_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign51670_e66314, assign51670_e66314_d_n4, assign51670_e66314_d_n6, assign51670_e66314_d_n7, assign51670_e66314_d_n8, assign51670_e66314_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51670_e66308: f64 = (1.0 + locals.var_temp__blk949);
        let assign51670_e66310: f64 = (-locals.var_thecs_t);
        let assign51670_e66311: f64 = (assign51670_e66308).powf(assign51670_e66310);
        let assign51670_e66312: f64 = (locals.var_cs_t * assign51670_e66311);
        (assign51670_e66312, ((locals.var_cs_t_dn4 * assign51670_e66311) + (locals.var_cs_t * if (-locals.var_thecs_t_dn4) == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn4)) } } else { (assign51670_e66311 * (((-locals.var_thecs_t_dn4) * (assign51670_e66308).ln()) + (assign51670_e66310 * (locals.var_temp__blk949_dn4 / assign51670_e66308)))) })), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn6)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn6 / assign51670_e66308))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn7)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn7 / assign51670_e66308))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn8)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn8 / assign51670_e66308))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn9)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn9 / assign51670_e66308))) }),)
    } else {
        (locals.var_gmobcssat__blk1396, locals.var_gmobcssat__blk1396_dn4, locals.var_gmobcssat__blk1396_dn6, locals.var_gmobcssat__blk1396_dn7, locals.var_gmobcssat__blk1396_dn8, locals.var_gmobcssat__blk1396_dn9,)
    }
};
        locals.var_gmobcssat__blk1396 = assign51670_e66314;
        locals.var_gmobcssat__blk1396_dn4 = assign51670_e66314_d_n4;
        locals.var_gmobcssat__blk1396_dn6 = assign51670_e66314_d_n6;
        locals.var_gmobcssat__blk1396_dn7 = assign51670_e66314_d_n7;
        locals.var_gmobcssat__blk1396_dn8 = assign51670_e66314_d_n8;
        locals.var_gmobcssat__blk1396_dn9 = assign51670_e66314_d_n9;
        locals.var_gmobcssat__blk1396_rv = 0.0;

        let (assign51680_e66338, assign51680_e66338_d_n4, assign51680_e66338_d_n6, assign51680_e66338_d_n7, assign51680_e66338_d_n8, assign51680_e66338_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51680_e66325: f64 = (locals.var_alphasat__blk1394 - 1.0);
        let assign51680_e66329: f64 = (locals.var_temp__blk949 + 1.0);
        let assign51680_e66330: f64 = (1.0 / assign51680_e66329);
        let assign51680_e66331: f64 = (assign51680_e66325 + assign51680_e66330);
        let assign51680_e66332: f64 = (locals.var_thecs_t * assign51680_e66331);
        let assign51680_e66334: f64 = (assign51680_e66332 / locals.var_qbsat__blk1393);
        let assign51680_e66336: f64 = (assign51680_e66334 * locals.var_gmobcssat__blk1396);
        (assign51680_e66336, (((((((locals.var_thecs_t_dn4 * assign51680_e66331) + (locals.var_thecs_t * (locals.var_alphasat__blk1394_dn4 + (-(locals.var_temp__blk949_dn4 / (assign51680_e66329 * assign51680_e66329)))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn4)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn4)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn6 + (-(locals.var_temp__blk949_dn6 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn6)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn6)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn7 + (-(locals.var_temp__blk949_dn7 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn7)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn7)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn8 + (-(locals.var_temp__blk949_dn8 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn8)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn8)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn9 + (-(locals.var_temp__blk949_dn9 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn9)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign51680_e66338;
        locals.var_temp2_dn4 = assign51680_e66338_d_n4;
        locals.var_temp2_dn6 = assign51680_e66338_d_n6;
        locals.var_temp2_dn7 = assign51680_e66338_d_n7;
        locals.var_temp2_dn8 = assign51680_e66338_d_n8;
        locals.var_temp2_dn9 = assign51680_e66338_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign51690_e66354, assign51690_e66354_d_n4, assign51690_e66354_d_n6, assign51690_e66354_d_n7, assign51690_e66354_d_n8, assign51690_e66354_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51690_e66348: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
        let assign51690_e66350: f64 = (assign51690_e66348 * locals.var_rhog__blk1379);
        let assign51690_e66352: f64 = (assign51690_e66350 * locals.var_qisat__blk1392);
        (assign51690_e66352, ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn4)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn4)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn6)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn7)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn8)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn8)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn9)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn9)),)
    } else {
        (locals.var_grsat__blk1397, locals.var_grsat__blk1397_dn4, locals.var_grsat__blk1397_dn6, locals.var_grsat__blk1397_dn7, locals.var_grsat__blk1397_dn8, locals.var_grsat__blk1397_dn9,)
    }
};
        locals.var_grsat__blk1397 = assign51690_e66354;
        locals.var_grsat__blk1397_dn4 = assign51690_e66354_d_n4;
        locals.var_grsat__blk1397_dn6 = assign51690_e66354_d_n6;
        locals.var_grsat__blk1397_dn7 = assign51690_e66354_d_n7;
        locals.var_grsat__blk1397_dn8 = assign51690_e66354_d_n8;
        locals.var_grsat__blk1397_dn9 = assign51690_e66354_d_n9;
        locals.var_grsat__blk1397_rv = 0.0;

        let (assign51700_e66376, assign51700_e66376_d_n4, assign51700_e66376_d_n6, assign51700_e66376_d_n7, assign51700_e66376_d_n8, assign51700_e66376_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51700_e66366: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
        let assign51700_e66368: f64 = (assign51700_e66366 * locals.var_rhog__blk1379);
        let assign51700_e66370: f64 = (assign51700_e66368 * locals.var_alphasat__blk1394);
        let assign51700_e66371: f64 = (locals.var_temp1 - assign51700_e66370);
        let assign51700_e66373: f64 = (assign51700_e66371 / locals.var_temp2);
        let assign51700_e66374: f64 = (1.0 + assign51700_e66373);
        (assign51700_e66374, ((((locals.var_temp1_dn4 - ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn4)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn4))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn4)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn6 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn6)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn6))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn7 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn7)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn7))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn8 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn8)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn8))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn9 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn9)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn9))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn9)) / (locals.var_temp2 * locals.var_temp2)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51700_e66376;
        locals.var_temp__blk949_dn4 = assign51700_e66376_d_n4;
        locals.var_temp__blk949_dn6 = assign51700_e66376_d_n6;
        locals.var_temp__blk949_dn7 = assign51700_e66376_d_n7;
        locals.var_temp__blk949_dn8 = assign51700_e66376_d_n8;
        locals.var_temp__blk949_dn9 = assign51700_e66376_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign51710_e66379: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1506 = assign51710_e66379;
        locals.var_guard1506_rv = 0.0;

        let (assign51720_e66399, assign51720_e66399_d_n4, assign51720_e66399_d_n6, assign51720_e66399_d_n7, assign51720_e66399_d_n8, assign51720_e66399_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1506 != 0.0)) {
        let assign51720_e66393: f64 = (2.0 * locals.var_temp__blk949);
        let assign51720_e66394: f64 = (assign51720_e66393).exp();
        let assign51720_e66395: f64 = (1.0 + assign51720_e66394);
        let assign51720_e66396: f64 = (assign51720_e66395).ln();
        let assign51720_e66397: f64 = (0.5 * assign51720_e66396);
        (assign51720_e66397, (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn4)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn6)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn7)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn8)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn9)) / assign51720_e66395)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign51720_e66399;
        locals.var_temp1_dn4 = assign51720_e66399_d_n4;
        locals.var_temp1_dn6 = assign51720_e66399_d_n6;
        locals.var_temp1_dn7 = assign51720_e66399_d_n7;
        locals.var_temp1_dn8 = assign51720_e66399_d_n8;
        locals.var_temp1_dn9 = assign51720_e66399_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign51730_e66412, assign51730_e66412_d_n4, assign51730_e66412_d_n6, assign51730_e66412_d_n7, assign51730_e66412_d_n8, assign51730_e66412_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1506 == 0.0)) {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign51730_e66412;
        locals.var_temp1_dn4 = assign51730_e66412_d_n4;
        locals.var_temp1_dn6 = assign51730_e66412_d_n6;
        locals.var_temp1_dn7 = assign51730_e66412_d_n7;
        locals.var_temp1_dn8 = assign51730_e66412_d_n8;
        locals.var_temp1_dn9 = assign51730_e66412_d_n9;
        locals.var_temp1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_47(
        locals: &mut StampLocals,
    ) {
        let (assign51740_e66435, assign51740_e66435_d_n4, assign51740_e66435_d_n6, assign51740_e66435_d_n7, assign51740_e66435_d_n8, assign51740_e66435_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51740_e66421: f64 = (-locals.var_midphi0__blk1391);
        let assign51740_e66423: f64 = (assign51740_e66421 * locals.var_temp2);
        let assign51740_e66425: f64 = (assign51740_e66423 * locals.var_temp1);
        let assign51740_e66428: f64 = (1.0 + locals.var_gmobmusat__blk1395);
        let assign51740_e66430: f64 = (assign51740_e66428 + locals.var_gmobcssat__blk1396);
        let assign51740_e66432: f64 = (assign51740_e66430 + locals.var_grsat__blk1397);
        let assign51740_e66433: f64 = (assign51740_e66425 / assign51740_e66432);
        (assign51740_e66433, ((((((((-locals.var_midphi0__blk1391_dn4) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn4)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn4)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn4 + locals.var_gmobcssat__blk1396_dn4) + locals.var_grsat__blk1397_dn4))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn6) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn6)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn6)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn6 + locals.var_gmobcssat__blk1396_dn6) + locals.var_grsat__blk1397_dn6))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn7) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn7)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn7)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn7 + locals.var_gmobcssat__blk1396_dn7) + locals.var_grsat__blk1397_dn7))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn8) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn8)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn8)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn8 + locals.var_gmobcssat__blk1396_dn8) + locals.var_grsat__blk1397_dn8))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn9) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn9)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn9)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn9 + locals.var_gmobcssat__blk1396_dn9) + locals.var_grsat__blk1397_dn9))) / (assign51740_e66432 * assign51740_e66432)),)
    } else {
        (locals.var_delta_gmob__blk1398, locals.var_delta_gmob__blk1398_dn4, locals.var_delta_gmob__blk1398_dn6, locals.var_delta_gmob__blk1398_dn7, locals.var_delta_gmob__blk1398_dn8, locals.var_delta_gmob__blk1398_dn9,)
    }
};
        locals.var_delta_gmob__blk1398 = assign51740_e66435;
        locals.var_delta_gmob__blk1398_dn4 = assign51740_e66435_d_n4;
        locals.var_delta_gmob__blk1398_dn6 = assign51740_e66435_d_n6;
        locals.var_delta_gmob__blk1398_dn7 = assign51740_e66435_d_n7;
        locals.var_delta_gmob__blk1398_dn8 = assign51740_e66435_d_n8;
        locals.var_delta_gmob__blk1398_dn9 = assign51740_e66435_d_n9;
        locals.var_delta_gmob__blk1398_rv = 0.0;

        let (assign51750_e66458, assign51750_e66458_d_n4, assign51750_e66458_d_n6, assign51750_e66458_d_n7, assign51750_e66458_d_n8, assign51750_e66458_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51750_e66450: f64 = (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398);
        let assign51750_e66451: f64 = (1.0 + assign51750_e66450);
        let assign51750_e66452: f64 = (assign51750_e66451).sqrt();
        let assign51750_e66453: f64 = (1.0 + assign51750_e66452);
        let assign51750_e66454: f64 = (locals.var_delta_gmob__blk1398 / assign51750_e66453);
        let assign51750_e66455: f64 = (1.0 + assign51750_e66454);
        let assign51750_e66456: f64 = (locals.var_x_inf0__blk1390 * assign51750_e66455);
        (assign51750_e66456, ((locals.var_x_inf0__blk1390_dn4 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn4 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn4 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn4)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn6 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn6 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn6 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn6)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn7 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn7 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn7 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn7)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn8 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn8 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn8 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn8)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn9 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn9 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn9 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn9)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))),)
    } else {
        (locals.var_x_inf__blk1399, locals.var_x_inf__blk1399_dn4, locals.var_x_inf__blk1399_dn6, locals.var_x_inf__blk1399_dn7, locals.var_x_inf__blk1399_dn8, locals.var_x_inf__blk1399_dn9,)
    }
};
        locals.var_x_inf__blk1399 = assign51750_e66458;
        locals.var_x_inf__blk1399_dn4 = assign51750_e66458_d_n4;
        locals.var_x_inf__blk1399_dn6 = assign51750_e66458_d_n6;
        locals.var_x_inf__blk1399_dn7 = assign51750_e66458_d_n7;
        locals.var_x_inf__blk1399_dn8 = assign51750_e66458_d_n8;
        locals.var_x_inf__blk1399_dn9 = assign51750_e66458_d_n9;
        locals.var_x_inf__blk1399_rv = 0.0;

        let (assign51760_e66469, assign51760_e66469_d_n4, assign51760_e66469_d_n6, assign51760_e66469_d_n7, assign51760_e66469_d_n8, assign51760_e66469_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 == 0.0)) {
        (locals.var_x_inf0__blk1390, locals.var_x_inf0__blk1390_dn4, locals.var_x_inf0__blk1390_dn6, locals.var_x_inf0__blk1390_dn7, locals.var_x_inf0__blk1390_dn8, locals.var_x_inf0__blk1390_dn9,)
    } else {
        (locals.var_x_inf__blk1399, locals.var_x_inf__blk1399_dn4, locals.var_x_inf__blk1399_dn6, locals.var_x_inf__blk1399_dn7, locals.var_x_inf__blk1399_dn8, locals.var_x_inf__blk1399_dn9,)
    }
};
        locals.var_x_inf__blk1399 = assign51760_e66469;
        locals.var_x_inf__blk1399_dn4 = assign51760_e66469_d_n4;
        locals.var_x_inf__blk1399_dn6 = assign51760_e66469_d_n6;
        locals.var_x_inf__blk1399_dn7 = assign51760_e66469_d_n7;
        locals.var_x_inf__blk1399_dn8 = assign51760_e66469_d_n8;
        locals.var_x_inf__blk1399_dn9 = assign51760_e66469_d_n9;
        locals.var_x_inf__blk1399_rv = 0.0;

        let (assign51770_e66483, assign51770_e66483_d_n4, assign51770_e66483_d_n6, assign51770_e66483_d_n7, assign51770_e66483_d_n8, assign51770_e66483_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51770_e66477: f64 = (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388);
        let assign51770_e66479: f64 = (assign51770_e66477 * locals.var_x_inf__blk1399);
        let assign51770_e66481: f64 = (assign51770_e66479 * 0.7071067811865475);
        (assign51770_e66481, (((((locals.var_phit1__blk1339_dn4 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn4)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn4)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn6 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn6)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn6)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn7 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn7)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn7)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn8 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn8)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn8)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn9 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn9)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn9)) * 0.7071067811865475),)
    } else {
        (locals.var_ysat__blk1400, locals.var_ysat__blk1400_dn4, locals.var_ysat__blk1400_dn6, locals.var_ysat__blk1400_dn7, locals.var_ysat__blk1400_dn8, locals.var_ysat__blk1400_dn9,)
    }
};
        locals.var_ysat__blk1400 = assign51770_e66483;
        locals.var_ysat__blk1400_dn4 = assign51770_e66483_d_n4;
        locals.var_ysat__blk1400_dn6 = assign51770_e66483_d_n6;
        locals.var_ysat__blk1400_dn7 = assign51770_e66483_d_n7;
        locals.var_ysat__blk1400_dn8 = assign51770_e66483_d_n8;
        locals.var_ysat__blk1400_dn9 = assign51770_e66483_d_n9;
        locals.var_ysat__blk1400_rv = 0.0;

        let assign51780_e66486: f64 = (-1.0);
        let assign51780_e66487: f64 = if locals.var_chnl_type == assign51780_e66486 { 1.0 } else { 0.0 };
        locals.var_guard1507 = assign51780_e66487;
        locals.var_guard1507_rv = 0.0;

        let (assign51790_e66502, assign51790_e66502_d_n4, assign51790_e66502_d_n6, assign51790_e66502_d_n7, assign51790_e66502_d_n8, assign51790_e66502_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign51790_e66498: f64 = (1.0 + locals.var_ysat__blk1400);
        let assign51790_e66499: f64 = (assign51790_e66498).sqrt();
        let assign51790_e66500: f64 = (locals.var_ysat__blk1400 / assign51790_e66499);
        (assign51790_e66500, (((locals.var_ysat__blk1400_dn4 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn4 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn6 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn6 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn7 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn7 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn8 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn8 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn9 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn9 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)),)
    } else {
        (locals.var_ysat__blk1400, locals.var_ysat__blk1400_dn4, locals.var_ysat__blk1400_dn6, locals.var_ysat__blk1400_dn7, locals.var_ysat__blk1400_dn8, locals.var_ysat__blk1400_dn9,)
    }
};
        locals.var_ysat__blk1400 = assign51790_e66502;
        locals.var_ysat__blk1400_dn4 = assign51790_e66502_d_n4;
        locals.var_ysat__blk1400_dn6 = assign51790_e66502_d_n6;
        locals.var_ysat__blk1400_dn7 = assign51790_e66502_d_n7;
        locals.var_ysat__blk1400_dn8 = assign51790_e66502_d_n8;
        locals.var_ysat__blk1400_dn9 = assign51790_e66502_d_n9;
        locals.var_ysat__blk1400_rv = 0.0;

        let (assign51800_e66519, assign51800_e66519_d_n4, assign51800_e66519_d_n6, assign51800_e66519_d_n7, assign51800_e66519_d_n8, assign51800_e66519_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51800_e66513: f64 = (4.0 * locals.var_ysat__blk1400);
        let assign51800_e66514: f64 = (1.0 + assign51800_e66513);
        let assign51800_e66515: f64 = (assign51800_e66514).sqrt();
        let assign51800_e66516: f64 = (1.0 + assign51800_e66515);
        let assign51800_e66517: f64 = (2.0 / assign51800_e66516);
        (assign51800_e66517, (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn4) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn6) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn7) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn8) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn9) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))),)
    } else {
        (locals.var_za__blk1401, locals.var_za__blk1401_dn4, locals.var_za__blk1401_dn6, locals.var_za__blk1401_dn7, locals.var_za__blk1401_dn8, locals.var_za__blk1401_dn9,)
    }
};
        locals.var_za__blk1401 = assign51800_e66519;
        locals.var_za__blk1401_dn4 = assign51800_e66519_d_n4;
        locals.var_za__blk1401_dn6 = assign51800_e66519_d_n6;
        locals.var_za__blk1401_dn7 = assign51800_e66519_d_n7;
        locals.var_za__blk1401_dn8 = assign51800_e66519_d_n8;
        locals.var_za__blk1401_dn9 = assign51800_e66519_d_n9;
        locals.var_za__blk1401_rv = 0.0;

        let (assign51810_e66529, assign51810_e66529_d_n4, assign51810_e66529_d_n6, assign51810_e66529_d_n7, assign51810_e66529_d_n8, assign51810_e66529_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51810_e66527: f64 = (locals.var_za__blk1401 * locals.var_ysat__blk1400);
        (assign51810_e66527, ((locals.var_za__blk1401_dn4 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn4)), ((locals.var_za__blk1401_dn6 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn6)), ((locals.var_za__blk1401_dn7 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn7)), ((locals.var_za__blk1401_dn8 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn8)), ((locals.var_za__blk1401_dn9 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51810_e66529;
        locals.var_temp__blk949_dn4 = assign51810_e66529_d_n4;
        locals.var_temp__blk949_dn6 = assign51810_e66529_d_n6;
        locals.var_temp__blk949_dn7 = assign51810_e66529_d_n7;
        locals.var_temp__blk949_dn8 = assign51810_e66529_d_n8;
        locals.var_temp__blk949_dn9 = assign51810_e66529_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign51820_e66561, assign51820_e66561_d_n4, assign51820_e66561_d_n6, assign51820_e66561_d_n7, assign51820_e66561_d_n8, assign51820_e66561_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51820_e66537: f64 = (locals.var_x_inf__blk1399 * locals.var_za__blk1401);
        let assign51820_e66541: f64 = (0.86 * locals.var_temp__blk949);
        let assign51820_e66545: f64 = (locals.var_temp__blk949 * locals.var_za__blk1401);
        let assign51820_e66546: f64 = (1.0 - assign51820_e66545);
        let assign51820_e66547: f64 = (assign51820_e66541 * assign51820_e66546);
        let assign51820_e66551: f64 = (4.0 * locals.var_temp__blk949);
        let assign51820_e66553: f64 = (assign51820_e66551 * locals.var_temp__blk949);
        let assign51820_e66555: f64 = (assign51820_e66553 * locals.var_za__blk1401);
        let assign51820_e66556: f64 = (1.0 + assign51820_e66555);
        let assign51820_e66557: f64 = (assign51820_e66547 / assign51820_e66556);
        let assign51820_e66558: f64 = (1.0 + assign51820_e66557);
        let assign51820_e66559: f64 = (assign51820_e66537 * assign51820_e66558);
        (assign51820_e66559, ((((locals.var_x_inf__blk1399_dn4 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn4)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn4) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn4 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn4))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn4) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn4)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn4)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn6 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn6)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn6) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn6 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn6))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn6) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn6)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn6)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn7 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn7)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn7) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn7 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn7))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn7) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn7)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn7)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn8 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn8)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn8) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn8 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn8))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn8) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn8)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn8)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn9 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn9)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn9) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn9 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn9))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn9) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn9)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn9)))) / (assign51820_e66556 * assign51820_e66556)))),)
    } else {
        (locals.var_x_0__blk1402, locals.var_x_0__blk1402_dn4, locals.var_x_0__blk1402_dn6, locals.var_x_0__blk1402_dn7, locals.var_x_0__blk1402_dn8, locals.var_x_0__blk1402_dn9,)
    }
};
        locals.var_x_0__blk1402 = assign51820_e66561;
        locals.var_x_0__blk1402_dn4 = assign51820_e66561_d_n4;
        locals.var_x_0__blk1402_dn6 = assign51820_e66561_d_n6;
        locals.var_x_0__blk1402_dn7 = assign51820_e66561_d_n7;
        locals.var_x_0__blk1402_dn8 = assign51820_e66561_d_n8;
        locals.var_x_0__blk1402_dn9 = assign51820_e66561_d_n9;
        locals.var_x_0__blk1402_rv = 0.0;

        let (assign51830_e66571, assign51830_e66571_d_n4, assign51830_e66571_d_n6, assign51830_e66571_d_n7, assign51830_e66571_d_n8, assign51830_e66571_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51830_e66569: f64 = (0.99 * locals.var_x_0__blk1402);
        (assign51830_e66569, (0.99 * locals.var_x_0__blk1402_dn4), (0.99 * locals.var_x_0__blk1402_dn6), (0.99 * locals.var_x_0__blk1402_dn7), (0.99 * locals.var_x_0__blk1402_dn8), (0.99 * locals.var_x_0__blk1402_dn9),)
    } else {
        (locals.var_x_sat__blk1403, locals.var_x_sat__blk1403_dn4, locals.var_x_sat__blk1403_dn6, locals.var_x_sat__blk1403_dn7, locals.var_x_sat__blk1403_dn8, locals.var_x_sat__blk1403_dn9,)
    }
};
        locals.var_x_sat__blk1403 = assign51830_e66571;
        locals.var_x_sat__blk1403_dn4 = assign51830_e66571_d_n4;
        locals.var_x_sat__blk1403_dn6 = assign51830_e66571_d_n6;
        locals.var_x_sat__blk1403_dn7 = assign51830_e66571_d_n7;
        locals.var_x_sat__blk1403_dn8 = assign51830_e66571_d_n8;
        locals.var_x_sat__blk1403_dn9 = assign51830_e66571_d_n9;
        locals.var_x_sat__blk1403_rv = 0.0;

        let (assign51840_e66589, assign51840_e66589_d_n4, assign51840_e66589_d_n6, assign51840_e66589_d_n7, assign51840_e66589_d_n8, assign51840_e66589_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51840_e66581: f64 = (2.0 * locals.var_asat__blk1389);
        let assign51840_e66582: f64 = (locals.var_x_sat__blk1403 - assign51840_e66581);
        let assign51840_e66583: f64 = (locals.var_x_sat__blk1403 * assign51840_e66582);
        let assign51840_e66585: f64 = (assign51840_e66583 * locals.var_inv_gf2__blk1341);
        let assign51840_e66587: f64 = (assign51840_e66585 / locals.var_ds__blk1370);
        (assign51840_e66587, (((((((locals.var_x_sat__blk1403_dn4 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn4 - (2.0 * locals.var_asat__blk1389_dn4)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn4)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn4)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn6 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn6 - (2.0 * locals.var_asat__blk1389_dn6)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn6)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn6)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn7 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn7 - (2.0 * locals.var_asat__blk1389_dn7)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn7)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn7)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn8 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn8 - (2.0 * locals.var_asat__blk1389_dn8)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn8)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn8)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn9 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn9 - (2.0 * locals.var_asat__blk1389_dn9)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn9)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn9)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51840_e66589;
        locals.var_temp__blk949_dn4 = assign51840_e66589_d_n4;
        locals.var_temp__blk949_dn6 = assign51840_e66589_d_n6;
        locals.var_temp__blk949_dn7 = assign51840_e66589_d_n7;
        locals.var_temp__blk949_dn8 = assign51840_e66589_d_n8;
        locals.var_temp__blk949_dn9 = assign51840_e66589_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign51850_e66611, assign51850_e66611_d_n4, assign51850_e66611_d_n6, assign51850_e66611_d_n7, assign51850_e66611_d_n8, assign51850_e66611_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51850_e66600: f64 = (-0.99);
        let (assign51850_e66605, assign51850_e66605_d_n4, assign51850_e66605_d_n6, assign51850_e66605_d_n7, assign51850_e66605_d_n8, assign51850_e66605_d_n9,) = {
            if (locals.var_temp__blk949 > assign51850_e66600) {
                (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
            } else {
                let assign51850_e66604: f64 = (-0.99);
                (assign51850_e66604, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign51850_e66606: f64 = (1.0 + assign51850_e66605);
        let assign51850_e66607: f64 = (assign51850_e66606).ln();
        let assign51850_e66608: f64 = (locals.var_x_sat__blk1403 - assign51850_e66607);
        let assign51850_e66609: f64 = (locals.var_phit1__blk1339 * assign51850_e66608);
        (assign51850_e66609, ((locals.var_phit1__blk1339_dn4 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn4 - (assign51850_e66605_d_n4 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn6 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn6 - (assign51850_e66605_d_n6 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn7 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn7 - (assign51850_e66605_d_n7 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn8 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn8 - (assign51850_e66605_d_n8 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn9 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn9 - (assign51850_e66605_d_n9 / assign51850_e66606)))),)
    } else {
        (locals.var_v_dsat__blk1404, locals.var_v_dsat__blk1404_dn4, locals.var_v_dsat__blk1404_dn6, locals.var_v_dsat__blk1404_dn7, locals.var_v_dsat__blk1404_dn8, locals.var_v_dsat__blk1404_dn9,)
    }
};
        locals.var_v_dsat__blk1404 = assign51850_e66611;
        locals.var_v_dsat__blk1404_dn4 = assign51850_e66611_d_n4;
        locals.var_v_dsat__blk1404_dn6 = assign51850_e66611_d_n6;
        locals.var_v_dsat__blk1404_dn7 = assign51850_e66611_d_n7;
        locals.var_v_dsat__blk1404_dn8 = assign51850_e66611_d_n8;
        locals.var_v_dsat__blk1404_dn9 = assign51850_e66611_d_n9;
        locals.var_v_dsat__blk1404_rv = 0.0;

        let (assign51860_e66620, assign51860_e66620_d_n4, assign51860_e66620_d_n6, assign51860_e66620_d_n7, assign51860_e66620_d_n8, assign51860_e66620_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 == 0.0)) {
        (locals.var_vdsat_lim__blk1387, locals.var_vdsat_lim__blk1387_dn4, locals.var_vdsat_lim__blk1387_dn6, locals.var_vdsat_lim__blk1387_dn7, locals.var_vdsat_lim__blk1387_dn8, locals.var_vdsat_lim__blk1387_dn9,)
    } else {
        (locals.var_v_dsat__blk1404, locals.var_v_dsat__blk1404_dn4, locals.var_v_dsat__blk1404_dn6, locals.var_v_dsat__blk1404_dn7, locals.var_v_dsat__blk1404_dn8, locals.var_v_dsat__blk1404_dn9,)
    }
};
        locals.var_v_dsat__blk1404 = assign51860_e66620;
        locals.var_v_dsat__blk1404_dn4 = assign51860_e66620_d_n4;
        locals.var_v_dsat__blk1404_dn6 = assign51860_e66620_d_n6;
        locals.var_v_dsat__blk1404_dn7 = assign51860_e66620_d_n7;
        locals.var_v_dsat__blk1404_dn8 = assign51860_e66620_d_n8;
        locals.var_v_dsat__blk1404_dn9 = assign51860_e66620_d_n9;
        locals.var_v_dsat__blk1404_rv = 0.0;

        let (assign51870_e66628, assign51870_e66628_d_n4, assign51870_e66628_d_n6, assign51870_e66628_d_n7, assign51870_e66628_d_n8, assign51870_e66628_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51870_e66626: f64 = (1.0 + locals.var_arloc__blk1320);
        (assign51870_e66626, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51870_e66628;
        locals.var_temp__blk949_dn4 = assign51870_e66628_d_n4;
        locals.var_temp__blk949_dn6 = assign51870_e66628_d_n6;
        locals.var_temp__blk949_dn7 = assign51870_e66628_d_n7;
        locals.var_temp__blk949_dn8 = assign51870_e66628_d_n8;
        locals.var_temp__blk949_dn9 = assign51870_e66628_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign51880_e66639, assign51880_e66639_d_n4, assign51880_e66639_d_n6, assign51880_e66639_d_n7, assign51880_e66639_d_n8, assign51880_e66639_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51880_e66633: f64 = (locals.var_temp__blk949).sqrt();
        let assign51880_e66635: f64 = (assign51880_e66633 * locals.var_v_ds);
        let assign51880_e66637: f64 = (assign51880_e66635 / locals.var_v_dsat__blk1404);
        (assign51880_e66637, (((((locals.var_temp__blk949_dn4 / (2.0 * assign51880_e66633)) * locals.var_v_ds) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn4)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), (((((locals.var_temp__blk949_dn6 / (2.0 * assign51880_e66633)) * locals.var_v_ds) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn6)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), ((((((locals.var_temp__blk949_dn7 / (2.0 * assign51880_e66633)) * locals.var_v_ds) + (assign51880_e66633 * locals.var_v_ds_dn7)) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn7)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), ((((((locals.var_temp__blk949_dn8 / (2.0 * assign51880_e66633)) * locals.var_v_ds) + (assign51880_e66633 * locals.var_v_ds_dn8)) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn8)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), (((((locals.var_temp__blk949_dn9 / (2.0 * assign51880_e66633)) * locals.var_v_ds) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn9)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign51880_e66639;
        locals.var_temp1_dn4 = assign51880_e66639_d_n4;
        locals.var_temp1_dn6 = assign51880_e66639_d_n6;
        locals.var_temp1_dn7 = assign51880_e66639_d_n7;
        locals.var_temp1_dn8 = assign51880_e66639_d_n8;
        locals.var_temp1_dn9 = assign51880_e66639_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign51890_e66649, assign51890_e66649_d_n4, assign51890_e66649_d_n6, assign51890_e66649_d_n7, assign51890_e66649_d_n8, assign51890_e66649_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51890_e66645: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign51890_e66647: f64 = (assign51890_e66645 + locals.var_temp__blk949);
        (assign51890_e66647, (((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)) + locals.var_temp__blk949_dn4), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk949_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk949_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk949_dn8), (((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)) + locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign51890_e66649;
        locals.var_temp2_dn4 = assign51890_e66649_d_n4;
        locals.var_temp2_dn6 = assign51890_e66649_d_n6;
        locals.var_temp2_dn7 = assign51890_e66649_d_n7;
        locals.var_temp2_dn8 = assign51890_e66649_d_n8;
        locals.var_temp2_dn9 = assign51890_e66649_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign51900_e66657, assign51900_e66657_d_n4, assign51900_e66657_d_n6, assign51900_e66657_d_n7, assign51900_e66657_d_n8, assign51900_e66657_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51900_e66655: f64 = (2.0 * locals.var_temp1);
        (assign51900_e66655, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51900_e66657;
        locals.var_temp__blk949_dn4 = assign51900_e66657_d_n4;
        locals.var_temp__blk949_dn6 = assign51900_e66657_d_n6;
        locals.var_temp__blk949_dn7 = assign51900_e66657_d_n7;
        locals.var_temp__blk949_dn8 = assign51900_e66657_d_n8;
        locals.var_temp__blk949_dn9 = assign51900_e66657_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign51910_e66675, assign51910_e66675_d_n4, assign51910_e66675_d_n6, assign51910_e66675_d_n7, assign51910_e66675_d_n8, assign51910_e66675_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51910_e66663: f64 = (locals.var_v_dsat__blk1404 * locals.var_temp__blk949);
        let assign51910_e66666: f64 = (locals.var_temp2 - locals.var_temp__blk949);
        let assign51910_e66667: f64 = (assign51910_e66666).sqrt();
        let assign51910_e66670: f64 = (locals.var_temp2 + locals.var_temp__blk949);
        let assign51910_e66671: f64 = (assign51910_e66670).sqrt();
        let assign51910_e66672: f64 = (assign51910_e66667 + assign51910_e66671);
        let assign51910_e66673: f64 = (assign51910_e66663 / assign51910_e66672);
        (assign51910_e66673, (((((locals.var_v_dsat__blk1404_dn4 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn4)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn4 - locals.var_temp__blk949_dn4) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn4 + locals.var_temp__blk949_dn4) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn6 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn6)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn6 - locals.var_temp__blk949_dn6) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn6 + locals.var_temp__blk949_dn6) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn7 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn7)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn7 - locals.var_temp__blk949_dn7) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn7 + locals.var_temp__blk949_dn7) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn8 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn8)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn8 - locals.var_temp__blk949_dn8) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn8 + locals.var_temp__blk949_dn8) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn9 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn9)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn9 - locals.var_temp__blk949_dn9) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn9 + locals.var_temp__blk949_dn9) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)),)
    } else {
        (locals.var_vdse__blk1405, locals.var_vdse__blk1405_dn4, locals.var_vdse__blk1405_dn6, locals.var_vdse__blk1405_dn7, locals.var_vdse__blk1405_dn8, locals.var_vdse__blk1405_dn9,)
    }
};
        locals.var_vdse__blk1405 = assign51910_e66675;
        locals.var_vdse__blk1405_dn4 = assign51910_e66675_d_n4;
        locals.var_vdse__blk1405_dn6 = assign51910_e66675_d_n6;
        locals.var_vdse__blk1405_dn7 = assign51910_e66675_d_n7;
        locals.var_vdse__blk1405_dn8 = assign51910_e66675_d_n8;
        locals.var_vdse__blk1405_dn9 = assign51910_e66675_d_n9;
        locals.var_vdse__blk1405_rv = 0.0;

        let (assign51920_e66683, assign51920_e66683_d_n4, assign51920_e66683_d_n6, assign51920_e66683_d_n7, assign51920_e66683_d_n8, assign51920_e66683_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51920_e66681: f64 = (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340);
        (assign51920_e66681, ((locals.var_vdse__blk1405_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn4)), ((locals.var_vdse__blk1405_dn6 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn6)), ((locals.var_vdse__blk1405_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_vdse__blk1405_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn8)), ((locals.var_vdse__blk1405_dn9 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn9)),)
    } else {
        (locals.var_udse__blk1406, locals.var_udse__blk1406_dn4, locals.var_udse__blk1406_dn6, locals.var_udse__blk1406_dn7, locals.var_udse__blk1406_dn8, locals.var_udse__blk1406_dn9,)
    }
};
        locals.var_udse__blk1406 = assign51920_e66683;
        locals.var_udse__blk1406_dn4 = assign51920_e66683_d_n4;
        locals.var_udse__blk1406_dn6 = assign51920_e66683_d_n6;
        locals.var_udse__blk1406_dn7 = assign51920_e66683_d_n7;
        locals.var_udse__blk1406_dn8 = assign51920_e66683_d_n8;
        locals.var_udse__blk1406_dn9 = assign51920_e66683_d_n9;
        locals.var_udse__blk1406_rv = 0.0;

        let (assign51930_e66691, assign51930_e66691_d_n4, assign51930_e66691_d_n6, assign51930_e66691_d_n7, assign51930_e66691_d_n8, assign51930_e66691_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51930_e66689: f64 = (locals.var_xn_s__blk1349 + locals.var_udse__blk1406);
        (assign51930_e66689, (locals.var_xn_s__blk1349_dn4 + locals.var_udse__blk1406_dn4), (locals.var_xn_s__blk1349_dn6 + locals.var_udse__blk1406_dn6), (locals.var_xn_s__blk1349_dn7 + locals.var_udse__blk1406_dn7), (locals.var_xn_s__blk1349_dn8 + locals.var_udse__blk1406_dn8), (locals.var_xn_s__blk1349_dn9 + locals.var_udse__blk1406_dn9),)
    } else {
        (locals.var_xn_d__blk1407, locals.var_xn_d__blk1407_dn4, locals.var_xn_d__blk1407_dn6, locals.var_xn_d__blk1407_dn7, locals.var_xn_d__blk1407_dn8, locals.var_xn_d__blk1407_dn9,)
    }
};
        locals.var_xn_d__blk1407 = assign51930_e66691;
        locals.var_xn_d__blk1407_dn4 = assign51930_e66691_d_n4;
        locals.var_xn_d__blk1407_dn6 = assign51930_e66691_d_n6;
        locals.var_xn_d__blk1407_dn7 = assign51930_e66691_d_n7;
        locals.var_xn_d__blk1407_dn8 = assign51930_e66691_d_n8;
        locals.var_xn_d__blk1407_dn9 = assign51930_e66691_d_n9;
        locals.var_xn_d__blk1407_rv = 0.0;

        let assign51940_e66694: f64 = if locals.var_udse__blk1406 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1508 = assign51940_e66694;
        locals.var_guard1508_rv = 0.0;

        let (assign51950_e66704, assign51950_e66704_d_n4, assign51950_e66704_d_n6, assign51950_e66704_d_n7, assign51950_e66704_d_n8, assign51950_e66704_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1508 != 0.0)) {
        let assign51950_e66701: f64 = (-locals.var_udse__blk1406);
        let assign51950_e66702: f64 = (assign51950_e66701).exp();
        (assign51950_e66702, (assign51950_e66702 * (-locals.var_udse__blk1406_dn4)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn6)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn7)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn8)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn9)),)
    } else {
        (locals.var_k_ds__blk1408, locals.var_k_ds__blk1408_dn4, locals.var_k_ds__blk1408_dn6, locals.var_k_ds__blk1408_dn7, locals.var_k_ds__blk1408_dn8, locals.var_k_ds__blk1408_dn9,)
    }
};
        locals.var_k_ds__blk1408 = assign51950_e66704;
        locals.var_k_ds__blk1408_dn4 = assign51950_e66704_d_n4;
        locals.var_k_ds__blk1408_dn6 = assign51950_e66704_d_n6;
        locals.var_k_ds__blk1408_dn7 = assign51950_e66704_d_n7;
        locals.var_k_ds__blk1408_dn8 = assign51950_e66704_d_n8;
        locals.var_k_ds__blk1408_dn9 = assign51950_e66704_d_n9;
        locals.var_k_ds__blk1408_rv = 0.0;

        let (assign51960_e66735, assign51960_e66735_d_n4, assign51960_e66735_d_n6, assign51960_e66735_d_n7, assign51960_e66735_d_n8, assign51960_e66735_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1508 == 0.0)) {
        let assign51960_e66715: f64 = (locals.var_udse__blk1406 - 460.51701859880916);
        let assign51960_e66720: f64 = (locals.var_udse__blk1406 - 460.51701859880916);
        let assign51960_e66724: f64 = (locals.var_udse__blk1406 - 460.51701859880916);
        let assign51960_e66726: f64 = (assign51960_e66724 * 0.3333333333333333);
        let assign51960_e66727: f64 = (1.0 + assign51960_e66726);
        let assign51960_e66728: f64 = (assign51960_e66720 * assign51960_e66727);
        let assign51960_e66729: f64 = (0.5 * assign51960_e66728);
        let assign51960_e66730: f64 = (1.0 + assign51960_e66729);
        let assign51960_e66731: f64 = (assign51960_e66715 * assign51960_e66730);
        let assign51960_e66732: f64 = (1.0 + assign51960_e66731);
        let assign51960_e66733: f64 = (1e-200 / assign51960_e66732);
        (assign51960_e66733, (-((1e-200 * ((locals.var_udse__blk1406_dn4 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn4 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn4 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn6 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn6 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn6 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn7 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn7 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn7 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn8 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn8 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn8 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn9 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn9 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn9 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))),)
    } else {
        (locals.var_k_ds__blk1408, locals.var_k_ds__blk1408_dn4, locals.var_k_ds__blk1408_dn6, locals.var_k_ds__blk1408_dn7, locals.var_k_ds__blk1408_dn8, locals.var_k_ds__blk1408_dn9,)
    }
};
        locals.var_k_ds__blk1408 = assign51960_e66735;
        locals.var_k_ds__blk1408_dn4 = assign51960_e66735_d_n4;
        locals.var_k_ds__blk1408_dn6 = assign51960_e66735_d_n6;
        locals.var_k_ds__blk1408_dn7 = assign51960_e66735_d_n7;
        locals.var_k_ds__blk1408_dn8 = assign51960_e66735_d_n8;
        locals.var_k_ds__blk1408_dn9 = assign51960_e66735_d_n9;
        locals.var_k_ds__blk1408_rv = 0.0;

        let (assign51970_e66743, assign51970_e66743_d_n4, assign51970_e66743_d_n6, assign51970_e66743_d_n7, assign51970_e66743_d_n8, assign51970_e66743_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51970_e66741: f64 = (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408);
        (assign51970_e66741, ((locals.var_delta_ns__blk1364_dn4 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn9)),)
    } else {
        (locals.var_delta_nd__blk1409, locals.var_delta_nd__blk1409_dn4, locals.var_delta_nd__blk1409_dn6, locals.var_delta_nd__blk1409_dn7, locals.var_delta_nd__blk1409_dn8, locals.var_delta_nd__blk1409_dn9,)
    }
};
        locals.var_delta_nd__blk1409 = assign51970_e66743;
        locals.var_delta_nd__blk1409_dn4 = assign51970_e66743_d_n4;
        locals.var_delta_nd__blk1409_dn6 = assign51970_e66743_d_n6;
        locals.var_delta_nd__blk1409_dn7 = assign51970_e66743_d_n7;
        locals.var_delta_nd__blk1409_dn8 = assign51970_e66743_d_n8;
        locals.var_delta_nd__blk1409_dn9 = assign51970_e66743_d_n9;
        locals.var_delta_nd__blk1409_rv = 0.0;

        let assign51980_e66745: f64 = (locals.var_xg__blk1343).abs();
        let assign51980_e66747: f64 = if assign51980_e66745 <= locals.var_margin__blk1361 { 1.0 } else { 0.0 };
        locals.var_guard1509 = assign51980_e66747;
        locals.var_guard1509_rv = 0.0;

        let (assign51990_e66761, assign51990_e66761_d_n4, assign51990_e66761_d_n6, assign51990_e66761_d_n7, assign51990_e66761_d_n8, assign51990_e66761_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 != 0.0)) {
        let assign51990_e66755: f64 = (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362);
        let assign51990_e66757: f64 = (assign51990_e66755 * 0.16666666666666666);
        let assign51990_e66759: f64 = (assign51990_e66757 * 0.7071067811865475);
        (assign51990_e66759, ((((locals.var_inv_xi__blk1362_dn4 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn6 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn7 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn8 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn9 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn9)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9,)
    }
};
        locals.var_sp_s_temp1__blk1449 = assign51990_e66761;
        locals.var_sp_s_temp1__blk1449_dn4 = assign51990_e66761_d_n4;
        locals.var_sp_s_temp1__blk1449_dn6 = assign51990_e66761_d_n6;
        locals.var_sp_s_temp1__blk1449_dn7 = assign51990_e66761_d_n7;
        locals.var_sp_s_temp1__blk1449_dn8 = assign51990_e66761_d_n8;
        locals.var_sp_s_temp1__blk1449_dn9 = assign51990_e66761_d_n9;
        locals.var_sp_s_temp1__blk1449_rv = 0.0;

        let (assign52000_e66783, assign52000_e66783_d_n4, assign52000_e66783_d_n6, assign52000_e66783_d_n7, assign52000_e66783_d_n8, assign52000_e66783_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 != 0.0)) {
        let assign52000_e66769: f64 = (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362);
        let assign52000_e66774: f64 = (1.0 - locals.var_delta_nd__blk1409);
        let assign52000_e66775: f64 = (locals.var_xg__blk1343 * assign52000_e66774);
        let assign52000_e66777: f64 = (assign52000_e66775 * locals.var_gf__blk1324);
        let assign52000_e66779: f64 = (assign52000_e66777 * locals.var_sp_s_temp1__blk1449);
        let assign52000_e66780: f64 = (1.0 + assign52000_e66779);
        let assign52000_e66781: f64 = (assign52000_e66769 * assign52000_e66780);
        (assign52000_e66781, ((((locals.var_xg__blk1343_dn4 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn4)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn4 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn4))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn4)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn4)))), ((((locals.var_xg__blk1343_dn6 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn6)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn6 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn6))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn6)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn6)))), ((((locals.var_xg__blk1343_dn7 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn7)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn7 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn7))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn7)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn7)))), ((((locals.var_xg__blk1343_dn8 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn8)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn8 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn8))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn8)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn8)))), ((((locals.var_xg__blk1343_dn9 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn9)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn9 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn9))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn9)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn9)))),)
    } else {
        (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9,)
    }
};
        locals.var_x_d__blk1410 = assign52000_e66783;
        locals.var_x_d__blk1410_dn4 = assign52000_e66783_d_n4;
        locals.var_x_d__blk1410_dn6 = assign52000_e66783_d_n6;
        locals.var_x_d__blk1410_dn7 = assign52000_e66783_d_n7;
        locals.var_x_d__blk1410_dn8 = assign52000_e66783_d_n8;
        locals.var_x_d__blk1410_dn9 = assign52000_e66783_d_n9;
        locals.var_x_d__blk1410_rv = 0.0;

        let (assign52010_e66794, assign52010_e66794_d_n4, assign52010_e66794_d_n6, assign52010_e66794_d_n7, assign52010_e66794_d_n8, assign52010_e66794_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52010_e66792: f64 = (locals.var_xn_d__blk1407 + 3.0);
        (assign52010_e66792, locals.var_xn_d__blk1407_dn4, locals.var_xn_d__blk1407_dn6, locals.var_xn_d__blk1407_dn7, locals.var_xn_d__blk1407_dn8, locals.var_xn_d__blk1407_dn9,)
    } else {
        (locals.var_sp_s_bx__blk1470, locals.var_sp_s_bx__blk1470_dn4, locals.var_sp_s_bx__blk1470_dn6, locals.var_sp_s_bx__blk1470_dn7, locals.var_sp_s_bx__blk1470_dn8, locals.var_sp_s_bx__blk1470_dn9,)
    }
};
        locals.var_sp_s_bx__blk1470 = assign52010_e66794;
        locals.var_sp_s_bx__blk1470_dn4 = assign52010_e66794_d_n4;
        locals.var_sp_s_bx__blk1470_dn6 = assign52010_e66794_d_n6;
        locals.var_sp_s_bx__blk1470_dn7 = assign52010_e66794_d_n7;
        locals.var_sp_s_bx__blk1470_dn8 = assign52010_e66794_d_n8;
        locals.var_sp_s_bx__blk1470_dn9 = assign52010_e66794_d_n9;
        locals.var_sp_s_bx__blk1470_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_48(
        locals: &mut StampLocals,
    ) {
        let (assign52020_e66829, assign52020_e66829_d_n4, assign52020_e66829_d_n6, assign52020_e66829_d_n7, assign52020_e66829_d_n8, assign52020_e66829_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52020_e66804: f64 = (locals.var_sp_s_x1__blk1469 + locals.var_sp_s_bx__blk1470);
        let assign52020_e66807: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
        let assign52020_e66810: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
        let assign52020_e66811: f64 = (assign52020_e66807 * assign52020_e66810);
        let assign52020_e66813: f64 = (assign52020_e66811 + 5.0);
        let assign52020_e66814: f64 = (assign52020_e66813).sqrt();
        let assign52020_e66815: f64 = (assign52020_e66804 - assign52020_e66814);
        let assign52020_e66816: f64 = (0.5 * assign52020_e66815);
        let assign52020_e66821: f64 = (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470);
        let assign52020_e66823: f64 = (assign52020_e66821 + 5.0);
        let assign52020_e66824: f64 = (assign52020_e66823).sqrt();
        let assign52020_e66825: f64 = (locals.var_sp_s_bx__blk1470 - assign52020_e66824);
        let assign52020_e66826: f64 = (0.5 * assign52020_e66825);
        let assign52020_e66827: f64 = (assign52020_e66816 - assign52020_e66826);
        (assign52020_e66827, ((0.5 * ((locals.var_sp_s_x1__blk1469_dn4 + locals.var_sp_s_bx__blk1470_dn4) - ((((locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn4 - (((locals.var_sp_s_bx__blk1470_dn4 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn4)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn6 + locals.var_sp_s_bx__blk1470_dn6) - ((((locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn6 - (((locals.var_sp_s_bx__blk1470_dn6 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn6)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn7 + locals.var_sp_s_bx__blk1470_dn7) - ((((locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn7 - (((locals.var_sp_s_bx__blk1470_dn7 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn7)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn8 + locals.var_sp_s_bx__blk1470_dn8) - ((((locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn8 - (((locals.var_sp_s_bx__blk1470_dn8 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn8)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn9 + locals.var_sp_s_bx__blk1470_dn9) - ((((locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn9 - (((locals.var_sp_s_bx__blk1470_dn9 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn9)) / (2.0 * assign52020_e66824))))),)
    } else {
        (locals.var_sp_s_eta__blk1453, locals.var_sp_s_eta__blk1453_dn4, locals.var_sp_s_eta__blk1453_dn6, locals.var_sp_s_eta__blk1453_dn7, locals.var_sp_s_eta__blk1453_dn8, locals.var_sp_s_eta__blk1453_dn9,)
    }
};
        locals.var_sp_s_eta__blk1453 = assign52020_e66829;
        locals.var_sp_s_eta__blk1453_dn4 = assign52020_e66829_d_n4;
        locals.var_sp_s_eta__blk1453_dn6 = assign52020_e66829_d_n6;
        locals.var_sp_s_eta__blk1453_dn7 = assign52020_e66829_d_n7;
        locals.var_sp_s_eta__blk1453_dn8 = assign52020_e66829_d_n8;
        locals.var_sp_s_eta__blk1453_dn9 = assign52020_e66829_d_n9;
        locals.var_sp_s_eta__blk1453_rv = 0.0;

        let (assign52030_e66840, assign52030_e66840_d_n4, assign52030_e66840_d_n6, assign52030_e66840_d_n7, assign52030_e66840_d_n8, assign52030_e66840_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52030_e66838: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_eta__blk1453);
        (assign52030_e66838, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_eta__blk1453_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_eta__blk1453_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_eta__blk1453_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_eta__blk1453_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_eta__blk1453_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign52030_e66840;
        locals.var_sp_s_temp__blk1448_dn4 = assign52030_e66840_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign52030_e66840_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign52030_e66840_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign52030_e66840_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign52030_e66840_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign52040_e66851, assign52040_e66851_d_n4, assign52040_e66851_d_n6, assign52040_e66851_d_n7, assign52040_e66851_d_n8, assign52040_e66851_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52040_e66848: f64 = (-locals.var_sp_s_eta__blk1453);
        let assign52040_e66849: f64 = (assign52040_e66848).exp();
        (assign52040_e66849, (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn4)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn6)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn7)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn8)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn9)),)
    } else {
        (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9,)
    }
};
        locals.var_sp_s_temp1__blk1449 = assign52040_e66851;
        locals.var_sp_s_temp1__blk1449_dn4 = assign52040_e66851_d_n4;
        locals.var_sp_s_temp1__blk1449_dn6 = assign52040_e66851_d_n6;
        locals.var_sp_s_temp1__blk1449_dn7 = assign52040_e66851_d_n7;
        locals.var_sp_s_temp1__blk1449_dn8 = assign52040_e66851_d_n8;
        locals.var_sp_s_temp1__blk1449_dn9 = assign52040_e66851_d_n9;
        locals.var_sp_s_temp1__blk1449_rv = 0.0;

        let (assign52050_e66866, assign52050_e66866_d_n4, assign52050_e66866_d_n6, assign52050_e66866_d_n7, assign52050_e66866_d_n8, assign52050_e66866_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52050_e66862: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
        let assign52050_e66863: f64 = (2.0 + assign52050_e66862);
        let assign52050_e66864: f64 = (1.0 / assign52050_e66863);
        (assign52050_e66864, (-(((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) / (assign52050_e66863 * assign52050_e66863))),)
    } else {
        (locals.var_sp_s_temp2__blk1450, locals.var_sp_s_temp2__blk1450_dn4, locals.var_sp_s_temp2__blk1450_dn6, locals.var_sp_s_temp2__blk1450_dn7, locals.var_sp_s_temp2__blk1450_dn8, locals.var_sp_s_temp2__blk1450_dn9,)
    }
};
        locals.var_sp_s_temp2__blk1450 = assign52050_e66866;
        locals.var_sp_s_temp2__blk1450_dn4 = assign52050_e66866_d_n4;
        locals.var_sp_s_temp2__blk1450_dn6 = assign52050_e66866_d_n6;
        locals.var_sp_s_temp2__blk1450_dn7 = assign52050_e66866_d_n7;
        locals.var_sp_s_temp2__blk1450_dn8 = assign52050_e66866_d_n8;
        locals.var_sp_s_temp2__blk1450_dn9 = assign52050_e66866_d_n9;
        locals.var_sp_s_temp2__blk1450_rv = 0.0;

        let (assign52060_e66879, assign52060_e66879_d_n4, assign52060_e66879_d_n6, assign52060_e66879_d_n7, assign52060_e66879_d_n8, assign52060_e66879_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52060_e66875: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
        let assign52060_e66877: f64 = (assign52060_e66875 * locals.var_sp_s_temp2__blk1450);
        (assign52060_e66877, ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn4)), ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn6)), ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn7)), ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn8)), ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn9)),)
    } else {
        (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9,)
    }
};
        locals.var_sp_s_xi0__blk1460 = assign52060_e66879;
        locals.var_sp_s_xi0__blk1460_dn4 = assign52060_e66879_d_n4;
        locals.var_sp_s_xi0__blk1460_dn6 = assign52060_e66879_d_n6;
        locals.var_sp_s_xi0__blk1460_dn7 = assign52060_e66879_d_n7;
        locals.var_sp_s_xi0__blk1460_dn8 = assign52060_e66879_d_n8;
        locals.var_sp_s_xi0__blk1460_dn9 = assign52060_e66879_d_n9;
        locals.var_sp_s_xi0__blk1460_rv = 0.0;

        let (assign52070_e66894, assign52070_e66894_d_n4, assign52070_e66894_d_n6, assign52070_e66894_d_n7, assign52070_e66894_d_n8, assign52070_e66894_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52070_e66889: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450);
        let assign52070_e66891: f64 = (assign52070_e66889 * locals.var_sp_s_temp2__blk1450);
        let assign52070_e66892: f64 = (4.0 * assign52070_e66891);
        (assign52070_e66892, (4.0 * ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn4))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn8))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn9))),)
    } else {
        (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9,)
    }
};
        locals.var_sp_s_xi1__blk1461 = assign52070_e66894;
        locals.var_sp_s_xi1__blk1461_dn4 = assign52070_e66894_d_n4;
        locals.var_sp_s_xi1__blk1461_dn6 = assign52070_e66894_d_n6;
        locals.var_sp_s_xi1__blk1461_dn7 = assign52070_e66894_d_n7;
        locals.var_sp_s_xi1__blk1461_dn8 = assign52070_e66894_d_n8;
        locals.var_sp_s_xi1__blk1461_dn9 = assign52070_e66894_d_n9;
        locals.var_sp_s_xi1__blk1461_rv = 0.0;

        let (assign52080_e66913, assign52080_e66913_d_n4, assign52080_e66913_d_n6, assign52080_e66913_d_n7, assign52080_e66913_d_n8, assign52080_e66913_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52080_e66903: f64 = (8.0 * locals.var_sp_s_temp2__blk1450);
        let assign52080_e66906: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
        let assign52080_e66907: f64 = (assign52080_e66903 - assign52080_e66906);
        let assign52080_e66909: f64 = (assign52080_e66907 * locals.var_sp_s_temp2__blk1450);
        let assign52080_e66911: f64 = (assign52080_e66909 * locals.var_sp_s_temp2__blk1450);
        (assign52080_e66911, ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn4)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn8)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn9)),)
    } else {
        (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9,)
    }
};
        locals.var_sp_s_xi2__blk1462 = assign52080_e66913;
        locals.var_sp_s_xi2__blk1462_dn4 = assign52080_e66913_d_n4;
        locals.var_sp_s_xi2__blk1462_dn6 = assign52080_e66913_d_n6;
        locals.var_sp_s_xi2__blk1462_dn7 = assign52080_e66913_d_n7;
        locals.var_sp_s_xi2__blk1462_dn8 = assign52080_e66913_d_n8;
        locals.var_sp_s_xi2__blk1462_dn9 = assign52080_e66913_d_n9;
        locals.var_sp_s_xi2__blk1462_rv = 0.0;

        let (assign52090_e66963, assign52090_e66963_d_n4, assign52090_e66963_d_n6, assign52090_e66963_d_n7, assign52090_e66963_d_n8, assign52090_e66963_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52090_e66923: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign52090_e66927: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
        let assign52090_e66929: f64 = (assign52090_e66927 - 1.0);
        let assign52090_e66933: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
        let assign52090_e66935: f64 = (assign52090_e66933 + locals.var_sp_s_xi0__blk1460);
        let assign52090_e66936: f64 = (locals.var_delta_nd__blk1409 * assign52090_e66935);
        let assign52090_e66937: f64 = (assign52090_e66929 - assign52090_e66936);
        let assign52090_e66938: f64 = (locals.var_gf2__blk1325 * assign52090_e66937);
        let assign52090_e66939: f64 = (assign52090_e66923 - assign52090_e66938);
        let (assign52090_e66961, assign52090_e66961_d_n4, assign52090_e66961_d_n6, assign52090_e66961_d_n7, assign52090_e66961_d_n8, assign52090_e66961_d_n9,) = {
            if (1e-40 > assign52090_e66939) {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52090_e66944: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
                let assign52090_e66948: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
                let assign52090_e66950: f64 = (assign52090_e66948 - 1.0);
                let assign52090_e66954: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
                let assign52090_e66956: f64 = (assign52090_e66954 + locals.var_sp_s_xi0__blk1460);
                let assign52090_e66957: f64 = (locals.var_delta_nd__blk1409 * assign52090_e66956);
                let assign52090_e66958: f64 = (assign52090_e66950 - assign52090_e66957);
                let assign52090_e66959: f64 = (locals.var_gf2__blk1325 * assign52090_e66958);
                let assign52090_e66960: f64 = (assign52090_e66944 - assign52090_e66959);
                (assign52090_e66960, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn4 + locals.var_sp_s_eta__blk1453_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn6 + locals.var_sp_s_eta__blk1453_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn7 + locals.var_sp_s_eta__blk1453_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn8 + locals.var_sp_s_eta__blk1453_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn9 + locals.var_sp_s_eta__blk1453_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))),)
            }
        };
        (assign52090_e66961, assign52090_e66961_d_n4, assign52090_e66961_d_n6, assign52090_e66961_d_n7, assign52090_e66961_d_n8, assign52090_e66961_d_n9,)
    } else {
        (locals.var_sp_s_a__blk1454, locals.var_sp_s_a__blk1454_dn4, locals.var_sp_s_a__blk1454_dn6, locals.var_sp_s_a__blk1454_dn7, locals.var_sp_s_a__blk1454_dn8, locals.var_sp_s_a__blk1454_dn9,)
    }
};
        locals.var_sp_s_a__blk1454 = assign52090_e66963;
        locals.var_sp_s_a__blk1454_dn4 = assign52090_e66963_d_n4;
        locals.var_sp_s_a__blk1454_dn6 = assign52090_e66963_d_n6;
        locals.var_sp_s_a__blk1454_dn7 = assign52090_e66963_d_n7;
        locals.var_sp_s_a__blk1454_dn8 = assign52090_e66963_d_n8;
        locals.var_sp_s_a__blk1454_dn9 = assign52090_e66963_d_n9;
        locals.var_sp_s_a__blk1454_rv = 0.0;

        let (assign52100_e66982, assign52100_e66982_d_n4, assign52100_e66982_d_n6, assign52100_e66982_d_n7, assign52100_e66982_d_n8, assign52100_e66982_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52100_e66976: f64 = (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462);
        let assign52100_e66977: f64 = (locals.var_sp_s_temp1__blk1449 - assign52100_e66976);
        let assign52100_e66978: f64 = (locals.var_gf2__blk1325 * assign52100_e66977);
        let assign52100_e66979: f64 = (0.5 * assign52100_e66978);
        let assign52100_e66980: f64 = (1.0 - assign52100_e66979);
        (assign52100_e66980, (-(0.5 * ((locals.var_gf2__blk1325_dn4 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn4 - ((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn4))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn6 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn6 - ((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn7 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn7 - ((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn8 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn8 - ((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn8))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn9 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn9 - ((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn9))))))),)
    } else {
        (locals.var_sp_s_b__blk1471, locals.var_sp_s_b__blk1471_dn4, locals.var_sp_s_b__blk1471_dn6, locals.var_sp_s_b__blk1471_dn7, locals.var_sp_s_b__blk1471_dn8, locals.var_sp_s_b__blk1471_dn9,)
    }
};
        locals.var_sp_s_b__blk1471 = assign52100_e66982;
        locals.var_sp_s_b__blk1471_dn4 = assign52100_e66982_d_n4;
        locals.var_sp_s_b__blk1471_dn6 = assign52100_e66982_d_n6;
        locals.var_sp_s_b__blk1471_dn7 = assign52100_e66982_d_n7;
        locals.var_sp_s_b__blk1471_dn8 = assign52100_e66982_d_n8;
        locals.var_sp_s_b__blk1471_dn9 = assign52100_e66982_d_n9;
        locals.var_sp_s_b__blk1471_rv = 0.0;

        let (assign52110_e67005, assign52110_e67005_d_n4, assign52110_e67005_d_n6, assign52110_e67005_d_n7, assign52110_e67005_d_n8, assign52110_e67005_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52110_e66991: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign52110_e66995: f64 = (1.0 - locals.var_sp_s_temp1__blk1449);
        let assign52110_e66999: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
        let assign52110_e67000: f64 = (locals.var_delta_nd__blk1409 * assign52110_e66999);
        let assign52110_e67001: f64 = (assign52110_e66995 - assign52110_e67000);
        let assign52110_e67002: f64 = (locals.var_gf2__blk1325 * assign52110_e67001);
        let assign52110_e67003: f64 = (assign52110_e66991 + assign52110_e67002);
        (assign52110_e67003, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn9)))))),)
    } else {
        (locals.var_sp_s_c__blk1455, locals.var_sp_s_c__blk1455_dn4, locals.var_sp_s_c__blk1455_dn6, locals.var_sp_s_c__blk1455_dn7, locals.var_sp_s_c__blk1455_dn8, locals.var_sp_s_c__blk1455_dn9,)
    }
};
        locals.var_sp_s_c__blk1455 = assign52110_e67005;
        locals.var_sp_s_c__blk1455_dn4 = assign52110_e67005_d_n4;
        locals.var_sp_s_c__blk1455_dn6 = assign52110_e67005_d_n6;
        locals.var_sp_s_c__blk1455_dn7 = assign52110_e67005_d_n7;
        locals.var_sp_s_c__blk1455_dn8 = assign52110_e67005_d_n8;
        locals.var_sp_s_c__blk1455_dn9 = assign52110_e67005_d_n9;
        locals.var_sp_s_c__blk1455_rv = 0.0;

        let (assign52120_e67021, assign52120_e67021_d_n4, assign52120_e67021_d_n6, assign52120_e67021_d_n7, assign52120_e67021_d_n8, assign52120_e67021_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52120_e67014: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_eta__blk1453);
        let assign52120_e67017: f64 = (locals.var_sp_s_a__blk1454 / locals.var_gf2__blk1325);
        let assign52120_e67018: f64 = (assign52120_e67017).ln();
        let assign52120_e67019: f64 = (assign52120_e67014 + assign52120_e67018);
        (assign52120_e67019, ((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_eta__blk1453_dn4) + ((((locals.var_sp_s_a__blk1454_dn4 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn4)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_eta__blk1453_dn6) + ((((locals.var_sp_s_a__blk1454_dn6 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn6)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_eta__blk1453_dn7) + ((((locals.var_sp_s_a__blk1454_dn7 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn7)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_eta__blk1453_dn8) + ((((locals.var_sp_s_a__blk1454_dn8 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn8)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_eta__blk1453_dn9) + ((((locals.var_sp_s_a__blk1454_dn9 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn9)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)),)
    } else {
        (locals.var_sp_s_tau__blk1456, locals.var_sp_s_tau__blk1456_dn4, locals.var_sp_s_tau__blk1456_dn6, locals.var_sp_s_tau__blk1456_dn7, locals.var_sp_s_tau__blk1456_dn8, locals.var_sp_s_tau__blk1456_dn9,)
    }
};
        locals.var_sp_s_tau__blk1456 = assign52120_e67021;
        locals.var_sp_s_tau__blk1456_dn4 = assign52120_e67021_d_n4;
        locals.var_sp_s_tau__blk1456_dn6 = assign52120_e67021_d_n6;
        locals.var_sp_s_tau__blk1456_dn7 = assign52120_e67021_d_n7;
        locals.var_sp_s_tau__blk1456_dn8 = assign52120_e67021_d_n8;
        locals.var_sp_s_tau__blk1456_dn9 = assign52120_e67021_d_n9;
        locals.var_sp_s_tau__blk1456_rv = 0.0;

        let (assign52130_e67032, assign52130_e67032_d_n4, assign52130_e67032_d_n6, assign52130_e67032_d_n7, assign52130_e67032_d_n8, assign52130_e67032_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52130_e67030: f64 = (locals.var_sp_s_a__blk1454 + locals.var_sp_s_c__blk1455);
        (assign52130_e67030, (locals.var_sp_s_a__blk1454_dn4 + locals.var_sp_s_c__blk1455_dn4), (locals.var_sp_s_a__blk1454_dn6 + locals.var_sp_s_c__blk1455_dn6), (locals.var_sp_s_a__blk1454_dn7 + locals.var_sp_s_c__blk1455_dn7), (locals.var_sp_s_a__blk1454_dn8 + locals.var_sp_s_c__blk1455_dn8), (locals.var_sp_s_a__blk1454_dn9 + locals.var_sp_s_c__blk1455_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign52130_e67032;
        locals.var_nu_dn4 = assign52130_e67032_d_n4;
        locals.var_nu_dn6 = assign52130_e67032_d_n6;
        locals.var_nu_dn7 = assign52130_e67032_d_n7;
        locals.var_nu_dn8 = assign52130_e67032_d_n8;
        locals.var_nu_dn9 = assign52130_e67032_d_n9;
        locals.var_nu_rv = 0.0;

        let (assign52140_e67055, assign52140_e67055_d_n4, assign52140_e67055_d_n6, assign52140_e67055_d_n7, assign52140_e67055_d_n8, assign52140_e67055_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52140_e67041: f64 = (locals.var_nu * locals.var_nu);
        let assign52140_e67046: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign52140_e67047: f64 = (0.5 * assign52140_e67046);
        let assign52140_e67050: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
        let assign52140_e67051: f64 = (assign52140_e67047 - assign52140_e67050);
        let assign52140_e67052: f64 = (locals.var_sp_s_tau__blk1456 * assign52140_e67051);
        let assign52140_e67053: f64 = (assign52140_e67041 + assign52140_e67052);
        (assign52140_e67053, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau__blk1456_dn4 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4))) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1456_dn6 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6))) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1456_dn7 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7))) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1456_dn8 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8))) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau__blk1456_dn9 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9))) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign52140_e67055;
        locals.var_mutau_dn4 = assign52140_e67055_d_n4;
        locals.var_mutau_dn6 = assign52140_e67055_d_n6;
        locals.var_mutau_dn7 = assign52140_e67055_d_n7;
        locals.var_mutau_dn8 = assign52140_e67055_d_n8;
        locals.var_mutau_dn9 = assign52140_e67055_d_n9;
        locals.var_mutau_rv = 0.0;

        let (assign52150_e67092, assign52150_e67092_d_n4, assign52150_e67092_d_n6, assign52150_e67092_d_n7, assign52150_e67092_d_n8, assign52150_e67092_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52150_e67065: f64 = (locals.var_sp_s_a__blk1454 * locals.var_nu);
        let assign52150_e67067: f64 = (assign52150_e67065 * locals.var_sp_s_tau__blk1456);
        let assign52150_e67071: f64 = (locals.var_nu / locals.var_mutau);
        let assign52150_e67073: f64 = (assign52150_e67071 * locals.var_sp_s_tau__blk1456);
        let assign52150_e67075: f64 = (assign52150_e67073 * locals.var_sp_s_tau__blk1456);
        let assign52150_e67077: f64 = (assign52150_e67075 * locals.var_sp_s_c__blk1455);
        let assign52150_e67080: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign52150_e67082: f64 = (assign52150_e67080 * 0.3333333333333333);
        let assign52150_e67085: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
        let assign52150_e67086: f64 = (assign52150_e67082 - assign52150_e67085);
        let assign52150_e67087: f64 = (assign52150_e67077 * assign52150_e67086);
        let assign52150_e67088: f64 = (locals.var_mutau + assign52150_e67087);
        let assign52150_e67089: f64 = (assign52150_e67067 / assign52150_e67088);
        let assign52150_e67090: f64 = (locals.var_sp_s_eta__blk1453 + assign52150_e67089);
        (assign52150_e67090, (locals.var_sp_s_eta__blk1453_dn4 + (((((((locals.var_sp_s_a__blk1454_dn4 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn4)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn4)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn4)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn6 + (((((((locals.var_sp_s_a__blk1454_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn6)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn6)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn7 + (((((((locals.var_sp_s_a__blk1454_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn7)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn7)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn8 + (((((((locals.var_sp_s_a__blk1454_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn8)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn8)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn9 + (((((((locals.var_sp_s_a__blk1454_dn9 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn9)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn9)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn9)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))))) / (assign52150_e67088 * assign52150_e67088))),)
    } else {
        (locals.var_sp_s_x0__blk1472, locals.var_sp_s_x0__blk1472_dn4, locals.var_sp_s_x0__blk1472_dn6, locals.var_sp_s_x0__blk1472_dn7, locals.var_sp_s_x0__blk1472_dn8, locals.var_sp_s_x0__blk1472_dn9,)
    }
};
        locals.var_sp_s_x0__blk1472 = assign52150_e67092;
        locals.var_sp_s_x0__blk1472_dn4 = assign52150_e67092_d_n4;
        locals.var_sp_s_x0__blk1472_dn6 = assign52150_e67092_d_n6;
        locals.var_sp_s_x0__blk1472_dn7 = assign52150_e67092_d_n7;
        locals.var_sp_s_x0__blk1472_dn8 = assign52150_e67092_d_n8;
        locals.var_sp_s_x0__blk1472_dn9 = assign52150_e67092_d_n9;
        locals.var_sp_s_x0__blk1472_rv = 0.0;

        let assign52160_e67095: f64 = if locals.var_sp_s_x0__blk1472 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1510 = assign52160_e67095;
        locals.var_guard1510_rv = 0.0;

        let (assign52170_e67107, assign52170_e67107_d_n4, assign52170_e67107_d_n6, assign52170_e67107_d_n7, assign52170_e67107_d_n8, assign52170_e67107_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 != 0.0)) {
        let assign52170_e67105: f64 = (locals.var_sp_s_x0__blk1472).exp();
        (assign52170_e67105, (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn4), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn6), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn7), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn8), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn9),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign52170_e67107;
        locals.var_sp_s_delta0__blk1458_dn4 = assign52170_e67107_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign52170_e67107_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign52170_e67107_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign52170_e67107_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign52170_e67107_d_n9;
        locals.var_sp_s_delta0__blk1458_rv = 0.0;

        let (assign52180_e67120, assign52180_e67120_d_n4, assign52180_e67120_d_n6, assign52180_e67120_d_n7, assign52180_e67120_d_n8, assign52180_e67120_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 != 0.0)) {
        let assign52180_e67118: f64 = (1.0 / locals.var_sp_s_delta0__blk1458);
        (assign52180_e67118, (-(locals.var_sp_s_delta0__blk1458_dn4 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn6 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn7 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn8 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn9 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign52180_e67120;
        locals.var_sp_s_delta1__blk1459_dn4 = assign52180_e67120_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign52180_e67120_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign52180_e67120_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign52180_e67120_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign52180_e67120_d_n9;
        locals.var_sp_s_delta1__blk1459_rv = 0.0;

        let (assign52190_e67133, assign52190_e67133_d_n4, assign52190_e67133_d_n6, assign52190_e67133_d_n7, assign52190_e67133_d_n8, assign52190_e67133_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 != 0.0)) {
        let assign52190_e67131: f64 = (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458);
        (assign52190_e67131, ((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn4)), ((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn6)), ((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn7)), ((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn8)), ((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn9)),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign52190_e67133;
        locals.var_sp_s_delta0__blk1458_dn4 = assign52190_e67133_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign52190_e67133_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign52190_e67133_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign52190_e67133_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign52190_e67133_d_n9;
        locals.var_sp_s_delta0__blk1458_rv = 0.0;

        let assign52200_e67137: f64 = (locals.var_xn_d__blk1407 - 230.25850929940458);
        let assign52200_e67138: f64 = if locals.var_sp_s_x0__blk1472 > assign52200_e67137 { 1.0 } else { 0.0 };
        locals.var_guard1511 = assign52200_e67138;
        locals.var_guard1511_rv = 0.0;

        let (assign52210_e67155, assign52210_e67155_d_n4, assign52210_e67155_d_n6, assign52210_e67155_d_n7, assign52210_e67155_d_n8, assign52210_e67155_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 != 0.0)) {
        let assign52210_e67152: f64 = (locals.var_sp_s_x0__blk1472 - locals.var_xn_d__blk1407);
        let assign52210_e67153: f64 = (assign52210_e67152).exp();
        (assign52210_e67153, (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn4 - locals.var_xn_d__blk1407_dn4)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn6 - locals.var_xn_d__blk1407_dn6)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn7 - locals.var_xn_d__blk1407_dn7)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn8 - locals.var_xn_d__blk1407_dn8)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn9 - locals.var_xn_d__blk1407_dn9)),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign52210_e67155;
        locals.var_sp_s_delta0__blk1458_dn4 = assign52210_e67155_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign52210_e67155_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign52210_e67155_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign52210_e67155_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign52210_e67155_d_n9;
        locals.var_sp_s_delta0__blk1458_rv = 0.0;

        let (assign52220_e67171, assign52220_e67171_d_n4, assign52220_e67171_d_n6, assign52220_e67171_d_n7, assign52220_e67171_d_n8, assign52220_e67171_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 != 0.0)) {
        let assign52220_e67169: f64 = (locals.var_delta_nd__blk1409 / locals.var_sp_s_delta0__blk1458);
        (assign52220_e67169, (((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn4)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn6)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn7)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn8)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn9)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign52220_e67171;
        locals.var_sp_s_delta1__blk1459_dn4 = assign52220_e67171_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign52220_e67171_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign52220_e67171_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign52220_e67171_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign52220_e67171_d_n9;
        locals.var_sp_s_delta1__blk1459_rv = 0.0;

        let (assign52230_e67214, assign52230_e67214_d_n4, assign52230_e67214_d_n6, assign52230_e67214_d_n7, assign52230_e67214_d_n8, assign52230_e67214_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 == 0.0)) {
        let assign52230_e67188: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_x0__blk1472);
        let assign52230_e67190: f64 = (assign52230_e67188 - 230.25850929940458);
        let assign52230_e67195: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_x0__blk1472);
        let assign52230_e67197: f64 = (assign52230_e67195 - 230.25850929940458);
        let assign52230_e67201: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_x0__blk1472);
        let assign52230_e67203: f64 = (assign52230_e67201 - 230.25850929940458);
        let assign52230_e67205: f64 = (assign52230_e67203 * 0.3333333333333333);
        let assign52230_e67206: f64 = (1.0 + assign52230_e67205);
        let assign52230_e67207: f64 = (assign52230_e67197 * assign52230_e67206);
        let assign52230_e67208: f64 = (0.5 * assign52230_e67207);
        let assign52230_e67209: f64 = (1.0 + assign52230_e67208);
        let assign52230_e67210: f64 = (assign52230_e67190 * assign52230_e67209);
        let assign52230_e67211: f64 = (1.0 + assign52230_e67210);
        let assign52230_e67212: f64 = (1e-100 / assign52230_e67211);
        (assign52230_e67212, (-((1e-100 * (((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_x0__blk1472_dn4) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_x0__blk1472_dn6) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_x0__blk1472_dn7) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_x0__blk1472_dn8) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_x0__blk1472_dn9) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign52230_e67214;
        locals.var_sp_s_delta0__blk1458_dn4 = assign52230_e67214_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign52230_e67214_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign52230_e67214_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign52230_e67214_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign52230_e67214_d_n9;
        locals.var_sp_s_delta0__blk1458_rv = 0.0;

        let (assign52240_e67251, assign52240_e67251_d_n4, assign52240_e67251_d_n6, assign52240_e67251_d_n7, assign52240_e67251_d_n8, assign52240_e67251_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 == 0.0)) {
        let assign52240_e67231: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign52240_e67236: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign52240_e67240: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign52240_e67242: f64 = (assign52240_e67240 * 0.3333333333333333);
        let assign52240_e67243: f64 = (1.0 + assign52240_e67242);
        let assign52240_e67244: f64 = (assign52240_e67236 * assign52240_e67243);
        let assign52240_e67245: f64 = (0.5 * assign52240_e67244);
        let assign52240_e67246: f64 = (1.0 + assign52240_e67245);
        let assign52240_e67247: f64 = (assign52240_e67231 * assign52240_e67246);
        let assign52240_e67248: f64 = (1.0 + assign52240_e67247);
        let assign52240_e67249: f64 = (1e-100 / assign52240_e67248);
        (assign52240_e67249, (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn4 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn4 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn4 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn6 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn6 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn6 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn7 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn7 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn7 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn8 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn8 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn8 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn9 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn9 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn9 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign52240_e67251;
        locals.var_sp_s_delta1__blk1459_dn4 = assign52240_e67251_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign52240_e67251_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign52240_e67251_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign52240_e67251_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign52240_e67251_d_n9;
        locals.var_sp_s_delta1__blk1459_rv = 0.0;

        let (assign52250_e67266, assign52250_e67266_d_n4, assign52250_e67266_d_n6, assign52250_e67266_d_n7, assign52250_e67266_d_n8, assign52250_e67266_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52250_e67262: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
        let assign52250_e67263: f64 = (2.0 + assign52250_e67262);
        let assign52250_e67264: f64 = (1.0 / assign52250_e67263);
        (assign52250_e67264, (-(((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) / (assign52250_e67263 * assign52250_e67263))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign52250_e67266;
        locals.var_sp_s_temp__blk1448_dn4 = assign52250_e67266_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign52250_e67266_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign52250_e67266_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign52250_e67266_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign52250_e67266_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign52260_e67279, assign52260_e67279_d_n4, assign52260_e67279_d_n6, assign52260_e67279_d_n7, assign52260_e67279_d_n8, assign52260_e67279_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52260_e67275: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
        let assign52260_e67277: f64 = (assign52260_e67275 * locals.var_sp_s_temp__blk1448);
        (assign52260_e67277, ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn4)), ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn6)), ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn7)), ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn8)), ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9,)
    }
};
        locals.var_sp_s_xi0__blk1460 = assign52260_e67279;
        locals.var_sp_s_xi0__blk1460_dn4 = assign52260_e67279_d_n4;
        locals.var_sp_s_xi0__blk1460_dn6 = assign52260_e67279_d_n6;
        locals.var_sp_s_xi0__blk1460_dn7 = assign52260_e67279_d_n7;
        locals.var_sp_s_xi0__blk1460_dn8 = assign52260_e67279_d_n8;
        locals.var_sp_s_xi0__blk1460_dn9 = assign52260_e67279_d_n9;
        locals.var_sp_s_xi0__blk1460_rv = 0.0;

        let (assign52270_e67294, assign52270_e67294_d_n4, assign52270_e67294_d_n6, assign52270_e67294_d_n7, assign52270_e67294_d_n8, assign52270_e67294_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52270_e67289: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448);
        let assign52270_e67291: f64 = (assign52270_e67289 * locals.var_sp_s_temp__blk1448);
        let assign52270_e67292: f64 = (4.0 * assign52270_e67291);
        (assign52270_e67292, (4.0 * ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn4))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn8))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn9))),)
    } else {
        (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9,)
    }
};
        locals.var_sp_s_xi1__blk1461 = assign52270_e67294;
        locals.var_sp_s_xi1__blk1461_dn4 = assign52270_e67294_d_n4;
        locals.var_sp_s_xi1__blk1461_dn6 = assign52270_e67294_d_n6;
        locals.var_sp_s_xi1__blk1461_dn7 = assign52270_e67294_d_n7;
        locals.var_sp_s_xi1__blk1461_dn8 = assign52270_e67294_d_n8;
        locals.var_sp_s_xi1__blk1461_dn9 = assign52270_e67294_d_n9;
        locals.var_sp_s_xi1__blk1461_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_49(
        locals: &mut StampLocals,
    ) {
        let (assign52280_e67313, assign52280_e67313_d_n4, assign52280_e67313_d_n6, assign52280_e67313_d_n7, assign52280_e67313_d_n8, assign52280_e67313_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52280_e67303: f64 = (8.0 * locals.var_sp_s_temp__blk1448);
        let assign52280_e67306: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
        let assign52280_e67307: f64 = (assign52280_e67303 - assign52280_e67306);
        let assign52280_e67309: f64 = (assign52280_e67307 * locals.var_sp_s_temp__blk1448);
        let assign52280_e67311: f64 = (assign52280_e67309 * locals.var_sp_s_temp__blk1448);
        (assign52280_e67311, ((((((8.0 * locals.var_sp_s_temp__blk1448_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn4)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn8)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9,)
    }
};
        locals.var_sp_s_xi2__blk1462 = assign52280_e67313;
        locals.var_sp_s_xi2__blk1462_dn4 = assign52280_e67313_d_n4;
        locals.var_sp_s_xi2__blk1462_dn6 = assign52280_e67313_d_n6;
        locals.var_sp_s_xi2__blk1462_dn7 = assign52280_e67313_d_n7;
        locals.var_sp_s_xi2__blk1462_dn8 = assign52280_e67313_d_n8;
        locals.var_sp_s_xi2__blk1462_dn9 = assign52280_e67313_d_n9;
        locals.var_sp_s_xi2__blk1462_rv = 0.0;

        let (assign52290_e67324, assign52290_e67324_d_n4, assign52290_e67324_d_n6, assign52290_e67324_d_n7, assign52290_e67324_d_n8, assign52290_e67324_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52290_e67322: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_x0__blk1472);
        (assign52290_e67322, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_x0__blk1472_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_x0__blk1472_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_x0__blk1472_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_x0__blk1472_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_x0__blk1472_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign52290_e67324;
        locals.var_sp_s_temp__blk1448_dn4 = assign52290_e67324_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign52290_e67324_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign52290_e67324_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign52290_e67324_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign52290_e67324_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign52300_e67349, assign52300_e67349_d_n4, assign52300_e67349_d_n6, assign52300_e67349_d_n7, assign52300_e67349_d_n8, assign52300_e67349_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52300_e67333: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign52300_e67337: f64 = (1.0 - locals.var_sp_s_delta1__blk1459);
        let assign52300_e67339: f64 = (assign52300_e67337 + locals.var_sp_s_delta0__blk1458);
        let assign52300_e67343: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
        let assign52300_e67344: f64 = (locals.var_delta_nd__blk1409 * assign52300_e67343);
        let assign52300_e67345: f64 = (assign52300_e67339 - assign52300_e67344);
        let assign52300_e67346: f64 = (locals.var_gf2__blk1325 * assign52300_e67345);
        let assign52300_e67347: f64 = (assign52300_e67333 + assign52300_e67346);
        (assign52300_e67347, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn9)))))),)
    } else {
        (locals.var_sp_s_pc__blk1463, locals.var_sp_s_pc__blk1463_dn4, locals.var_sp_s_pc__blk1463_dn6, locals.var_sp_s_pc__blk1463_dn7, locals.var_sp_s_pc__blk1463_dn8, locals.var_sp_s_pc__blk1463_dn9,)
    }
};
        locals.var_sp_s_pc__blk1463 = assign52300_e67349;
        locals.var_sp_s_pc__blk1463_dn4 = assign52300_e67349_d_n4;
        locals.var_sp_s_pc__blk1463_dn6 = assign52300_e67349_d_n6;
        locals.var_sp_s_pc__blk1463_dn7 = assign52300_e67349_d_n7;
        locals.var_sp_s_pc__blk1463_dn8 = assign52300_e67349_d_n8;
        locals.var_sp_s_pc__blk1463_dn9 = assign52300_e67349_d_n9;
        locals.var_sp_s_pc__blk1463_rv = 0.0;

        let (assign52310_e67378, assign52310_e67378_d_n4, assign52310_e67378_d_n6, assign52310_e67378_d_n7, assign52310_e67378_d_n8, assign52310_e67378_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52310_e67358: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign52310_e67362: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_x0__blk1472);
        let assign52310_e67364: f64 = (assign52310_e67362 - 1.0);
        let assign52310_e67366: f64 = (assign52310_e67364 + locals.var_sp_s_delta0__blk1458);
        let assign52310_e67370: f64 = (locals.var_sp_s_x0__blk1472 + 1.0);
        let assign52310_e67372: f64 = (assign52310_e67370 + locals.var_sp_s_xi0__blk1460);
        let assign52310_e67373: f64 = (locals.var_delta_nd__blk1409 * assign52310_e67372);
        let assign52310_e67374: f64 = (assign52310_e67366 - assign52310_e67373);
        let assign52310_e67375: f64 = (locals.var_gf2__blk1325 * assign52310_e67374);
        let assign52310_e67376: f64 = (assign52310_e67358 - assign52310_e67375);
        (assign52310_e67376, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_x0__blk1472_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_x0__blk1472_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_x0__blk1472_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_x0__blk1472_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_x0__blk1472_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))),)
    } else {
        (locals.var_sp_s_qc__blk1464, locals.var_sp_s_qc__blk1464_dn4, locals.var_sp_s_qc__blk1464_dn6, locals.var_sp_s_qc__blk1464_dn7, locals.var_sp_s_qc__blk1464_dn8, locals.var_sp_s_qc__blk1464_dn9,)
    }
};
        locals.var_sp_s_qc__blk1464 = assign52310_e67378;
        locals.var_sp_s_qc__blk1464_dn4 = assign52310_e67378_d_n4;
        locals.var_sp_s_qc__blk1464_dn6 = assign52310_e67378_d_n6;
        locals.var_sp_s_qc__blk1464_dn7 = assign52310_e67378_d_n7;
        locals.var_sp_s_qc__blk1464_dn8 = assign52310_e67378_d_n8;
        locals.var_sp_s_qc__blk1464_dn9 = assign52310_e67378_d_n9;
        locals.var_sp_s_qc__blk1464_rv = 0.0;

        let (assign52320_e67397, assign52320_e67397_d_n4, assign52320_e67397_d_n6, assign52320_e67397_d_n7, assign52320_e67397_d_n8, assign52320_e67397_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52320_e67389: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_delta0__blk1458);
        let assign52320_e67392: f64 = (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462);
        let assign52320_e67393: f64 = (assign52320_e67389 - assign52320_e67392);
        let assign52320_e67394: f64 = (locals.var_gf2__blk1325 * assign52320_e67393);
        let assign52320_e67395: f64 = (2.0 - assign52320_e67394);
        (assign52320_e67395, (-((locals.var_gf2__blk1325_dn4 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn9)))))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign52320_e67397;
        locals.var_sp_s_temp__blk1448_dn4 = assign52320_e67397_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign52320_e67397_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign52320_e67397_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign52320_e67397_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign52320_e67397_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign52330_e67414, assign52330_e67414_d_n4, assign52330_e67414_d_n6, assign52330_e67414_d_n7, assign52330_e67414_d_n8, assign52330_e67414_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52330_e67406: f64 = (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463);
        let assign52330_e67410: f64 = (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448);
        let assign52330_e67411: f64 = (2.0 * assign52330_e67410);
        let assign52330_e67412: f64 = (assign52330_e67406 - assign52330_e67411);
        (assign52330_e67412, (((locals.var_sp_s_pc__blk1463_dn4 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn4)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn4)))), (((locals.var_sp_s_pc__blk1463_dn6 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn6)))), (((locals.var_sp_s_pc__blk1463_dn7 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn7)))), (((locals.var_sp_s_pc__blk1463_dn8 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn8)))), (((locals.var_sp_s_pc__blk1463_dn9 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn9)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn9)))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign52330_e67414;
        locals.var_sp_s_temp__blk1448_dn4 = assign52330_e67414_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign52330_e67414_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign52330_e67414_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign52330_e67414_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign52330_e67414_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign52340_e67432, assign52340_e67432_d_n4, assign52340_e67432_d_n6, assign52340_e67432_d_n7, assign52340_e67432_d_n8, assign52340_e67432_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52340_e67426: f64 = (locals.var_sp_s_temp__blk1448).sqrt();
        let assign52340_e67427: f64 = (locals.var_sp_s_pc__blk1463 + assign52340_e67426);
        let assign52340_e67428: f64 = (locals.var_sp_s_qc__blk1464 / assign52340_e67427);
        let assign52340_e67429: f64 = (2.0 * assign52340_e67428);
        let assign52340_e67430: f64 = (locals.var_sp_s_x0__blk1472 + assign52340_e67429);
        (assign52340_e67430, (locals.var_sp_s_x0__blk1472_dn4 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn4 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn4 + (locals.var_sp_s_temp__blk1448_dn4 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn6 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn6 + (locals.var_sp_s_temp__blk1448_dn6 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn7 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn7 + (locals.var_sp_s_temp__blk1448_dn7 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn8 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn8 + (locals.var_sp_s_temp__blk1448_dn8 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn9 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn9 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn9 + (locals.var_sp_s_temp__blk1448_dn9 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))),)
    } else {
        (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9,)
    }
};
        locals.var_x_d__blk1410 = assign52340_e67432;
        locals.var_x_d__blk1410_dn4 = assign52340_e67432_d_n4;
        locals.var_x_d__blk1410_dn6 = assign52340_e67432_d_n6;
        locals.var_x_d__blk1410_dn7 = assign52340_e67432_d_n7;
        locals.var_x_d__blk1410_dn8 = assign52340_e67432_d_n8;
        locals.var_x_d__blk1410_dn9 = assign52340_e67432_d_n9;
        locals.var_x_d__blk1410_rv = 0.0;

        let (assign52350_e67440, assign52350_e67440_d_n4, assign52350_e67440_d_n6, assign52350_e67440_d_n7, assign52350_e67440_d_n8, assign52350_e67440_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52350_e67438: f64 = (locals.var_x_d__blk1410 - locals.var_x_s__blk1363);
        (assign52350_e67438, (locals.var_x_d__blk1410_dn4 - locals.var_x_s__blk1363_dn4), (locals.var_x_d__blk1410_dn6 - locals.var_x_s__blk1363_dn6), (locals.var_x_d__blk1410_dn7 - locals.var_x_s__blk1363_dn7), (locals.var_x_d__blk1410_dn8 - locals.var_x_s__blk1363_dn8), (locals.var_x_d__blk1410_dn9 - locals.var_x_s__blk1363_dn9),)
    } else {
        (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9,)
    }
};
        locals.var_x_ds__blk1411 = assign52350_e67440;
        locals.var_x_ds__blk1411_dn4 = assign52350_e67440_d_n4;
        locals.var_x_ds__blk1411_dn6 = assign52350_e67440_d_n6;
        locals.var_x_ds__blk1411_dn7 = assign52350_e67440_d_n7;
        locals.var_x_ds__blk1411_dn8 = assign52350_e67440_d_n8;
        locals.var_x_ds__blk1411_dn9 = assign52350_e67440_d_n9;
        locals.var_x_ds__blk1411_rv = 0.0;

        let assign52360_e67443: f64 = if locals.var_x_ds__blk1411 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1512 = assign52360_e67443;
        locals.var_guard1512_rv = 0.0;

        let (assign52370_e67471, assign52370_e67471_d_n4, assign52370_e67471_d_n6, assign52370_e67471_d_n7, assign52370_e67471_d_n8, assign52370_e67471_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52370_e67452: f64 = (locals.var_xg__blk1343 - locals.var_x_s__blk1363);
        let assign52370_e67453: f64 = (2.0 * assign52370_e67452);
        let assign52370_e67457: f64 = (1.0 - locals.var_es__blk1369);
        let assign52370_e67460: f64 = (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408);
        let assign52370_e67461: f64 = (assign52370_e67457 + assign52370_e67460);
        let assign52370_e67465: f64 = (1.0 + locals.var_xi1s__blk1366);
        let assign52370_e67466: f64 = (locals.var_delta_nd__blk1409 * assign52370_e67465);
        let assign52370_e67467: f64 = (assign52370_e67461 - assign52370_e67466);
        let assign52370_e67468: f64 = (locals.var_gf2__blk1325 * assign52370_e67467);
        let assign52370_e67469: f64 = (assign52370_e67453 + assign52370_e67468);
        (assign52370_e67469, ((2.0 * (locals.var_xg__blk1343_dn4 - locals.var_x_s__blk1363_dn4)) + ((locals.var_gf2__blk1325_dn4 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn4) + ((locals.var_delta_1s__blk1368_dn4 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn4))) - ((locals.var_delta_nd__blk1409_dn4 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn4)))))), ((2.0 * (locals.var_xg__blk1343_dn6 - locals.var_x_s__blk1363_dn6)) + ((locals.var_gf2__blk1325_dn6 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn6) + ((locals.var_delta_1s__blk1368_dn6 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn6))) - ((locals.var_delta_nd__blk1409_dn6 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn6)))))), ((2.0 * (locals.var_xg__blk1343_dn7 - locals.var_x_s__blk1363_dn7)) + ((locals.var_gf2__blk1325_dn7 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn7) + ((locals.var_delta_1s__blk1368_dn7 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn7))) - ((locals.var_delta_nd__blk1409_dn7 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn7)))))), ((2.0 * (locals.var_xg__blk1343_dn8 - locals.var_x_s__blk1363_dn8)) + ((locals.var_gf2__blk1325_dn8 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn8) + ((locals.var_delta_1s__blk1368_dn8 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn8))) - ((locals.var_delta_nd__blk1409_dn8 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn8)))))), ((2.0 * (locals.var_xg__blk1343_dn9 - locals.var_x_s__blk1363_dn9)) + ((locals.var_gf2__blk1325_dn9 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn9) + ((locals.var_delta_1s__blk1368_dn9 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn9))) - ((locals.var_delta_nd__blk1409_dn9 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn9)))))),)
    } else {
        (locals.var_pc__blk1412, locals.var_pc__blk1412_dn4, locals.var_pc__blk1412_dn6, locals.var_pc__blk1412_dn7, locals.var_pc__blk1412_dn8, locals.var_pc__blk1412_dn9,)
    }
};
        locals.var_pc__blk1412 = assign52370_e67471;
        locals.var_pc__blk1412_dn4 = assign52370_e67471_d_n4;
        locals.var_pc__blk1412_dn6 = assign52370_e67471_d_n6;
        locals.var_pc__blk1412_dn7 = assign52370_e67471_d_n7;
        locals.var_pc__blk1412_dn8 = assign52370_e67471_d_n8;
        locals.var_pc__blk1412_dn9 = assign52370_e67471_d_n9;
        locals.var_pc__blk1412_rv = 0.0;

        let (assign52380_e67485, assign52380_e67485_d_n4, assign52380_e67485_d_n6, assign52380_e67485_d_n7, assign52380_e67485_d_n8, assign52380_e67485_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52380_e67480: f64 = (1.0 - locals.var_k_ds__blk1408);
        let assign52380_e67481: f64 = (locals.var_gf2__blk1325 * assign52380_e67480);
        let assign52380_e67483: f64 = (assign52380_e67481 * locals.var_ds__blk1370);
        (assign52380_e67483, ((((locals.var_gf2__blk1325_dn4 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn4))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn4)), ((((locals.var_gf2__blk1325_dn6 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn6))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn6)), ((((locals.var_gf2__blk1325_dn7 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn7))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn7)), ((((locals.var_gf2__blk1325_dn8 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn8))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn8)), ((((locals.var_gf2__blk1325_dn9 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn9))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn9)),)
    } else {
        (locals.var_qc__blk1413, locals.var_qc__blk1413_dn4, locals.var_qc__blk1413_dn6, locals.var_qc__blk1413_dn7, locals.var_qc__blk1413_dn8, locals.var_qc__blk1413_dn9,)
    }
};
        locals.var_qc__blk1413 = assign52380_e67485;
        locals.var_qc__blk1413_dn4 = assign52380_e67485_d_n4;
        locals.var_qc__blk1413_dn6 = assign52380_e67485_d_n6;
        locals.var_qc__blk1413_dn7 = assign52380_e67485_d_n7;
        locals.var_qc__blk1413_dn8 = assign52380_e67485_d_n8;
        locals.var_qc__blk1413_dn9 = assign52380_e67485_d_n9;
        locals.var_qc__blk1413_rv = 0.0;

        let (assign52390_e67505, assign52390_e67505_d_n4, assign52390_e67505_d_n6, assign52390_e67505_d_n7, assign52390_e67505_d_n8, assign52390_e67505_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52390_e67496: f64 = (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408);
        let assign52390_e67497: f64 = (locals.var_es__blk1369 + assign52390_e67496);
        let assign52390_e67500: f64 = (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367);
        let assign52390_e67501: f64 = (assign52390_e67497 - assign52390_e67500);
        let assign52390_e67502: f64 = (locals.var_gf2__blk1325 * assign52390_e67501);
        let assign52390_e67503: f64 = (2.0 - assign52390_e67502);
        (assign52390_e67503, (-((locals.var_gf2__blk1325_dn4 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn4 + ((locals.var_delta_1s__blk1368_dn4 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn4))) - ((locals.var_delta_nd__blk1409_dn4 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn6 + ((locals.var_delta_1s__blk1368_dn6 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn6))) - ((locals.var_delta_nd__blk1409_dn6 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn7 + ((locals.var_delta_1s__blk1368_dn7 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn7))) - ((locals.var_delta_nd__blk1409_dn7 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn8 + ((locals.var_delta_1s__blk1368_dn8 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn8))) - ((locals.var_delta_nd__blk1409_dn8 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn9 + ((locals.var_delta_1s__blk1368_dn9 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn9))) - ((locals.var_delta_nd__blk1409_dn9 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn9)))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52390_e67505;
        locals.var_temp__blk949_dn4 = assign52390_e67505_d_n4;
        locals.var_temp__blk949_dn6 = assign52390_e67505_d_n6;
        locals.var_temp__blk949_dn7 = assign52390_e67505_d_n7;
        locals.var_temp__blk949_dn8 = assign52390_e67505_d_n8;
        locals.var_temp__blk949_dn9 = assign52390_e67505_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign52400_e67521, assign52400_e67521_d_n4, assign52400_e67521_d_n6, assign52400_e67521_d_n7, assign52400_e67521_d_n8, assign52400_e67521_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52400_e67513: f64 = (locals.var_pc__blk1412 * locals.var_pc__blk1412);
        let assign52400_e67517: f64 = (locals.var_temp__blk949 * locals.var_qc__blk1413);
        let assign52400_e67518: f64 = (2.0 * assign52400_e67517);
        let assign52400_e67519: f64 = (assign52400_e67513 - assign52400_e67518);
        (assign52400_e67519, (((locals.var_pc__blk1412_dn4 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn4)) - (2.0 * ((locals.var_temp__blk949_dn4 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn4)))), (((locals.var_pc__blk1412_dn6 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn6)) - (2.0 * ((locals.var_temp__blk949_dn6 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn6)))), (((locals.var_pc__blk1412_dn7 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn7)) - (2.0 * ((locals.var_temp__blk949_dn7 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn7)))), (((locals.var_pc__blk1412_dn8 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn8)) - (2.0 * ((locals.var_temp__blk949_dn8 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn8)))), (((locals.var_pc__blk1412_dn9 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn9)) - (2.0 * ((locals.var_temp__blk949_dn9 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn9)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52400_e67521;
        locals.var_temp__blk949_dn4 = assign52400_e67521_d_n4;
        locals.var_temp__blk949_dn6 = assign52400_e67521_d_n6;
        locals.var_temp__blk949_dn7 = assign52400_e67521_d_n7;
        locals.var_temp__blk949_dn8 = assign52400_e67521_d_n8;
        locals.var_temp__blk949_dn9 = assign52400_e67521_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign52410_e67536, assign52410_e67536_d_n4, assign52410_e67536_d_n6, assign52410_e67536_d_n7, assign52410_e67536_d_n8, assign52410_e67536_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52410_e67531: f64 = (locals.var_temp__blk949).sqrt();
        let assign52410_e67532: f64 = (locals.var_pc__blk1412 + assign52410_e67531);
        let assign52410_e67533: f64 = (locals.var_qc__blk1413 / assign52410_e67532);
        let assign52410_e67534: f64 = (2.0 * assign52410_e67533);
        (assign52410_e67534, (2.0 * (((locals.var_qc__blk1413_dn4 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn4 + (locals.var_temp__blk949_dn4 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn6 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn6 + (locals.var_temp__blk949_dn6 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn7 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn7 + (locals.var_temp__blk949_dn7 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn8 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn8 + (locals.var_temp__blk949_dn8 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn9 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn9 + (locals.var_temp__blk949_dn9 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))),)
    } else {
        (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9,)
    }
};
        locals.var_x_ds__blk1411 = assign52410_e67536;
        locals.var_x_ds__blk1411_dn4 = assign52410_e67536_d_n4;
        locals.var_x_ds__blk1411_dn6 = assign52410_e67536_d_n6;
        locals.var_x_ds__blk1411_dn7 = assign52410_e67536_d_n7;
        locals.var_x_ds__blk1411_dn8 = assign52410_e67536_d_n8;
        locals.var_x_ds__blk1411_dn9 = assign52410_e67536_d_n9;
        locals.var_x_ds__blk1411_rv = 0.0;

        let (assign52420_e67546, assign52420_e67546_d_n4, assign52420_e67546_d_n6, assign52420_e67546_d_n7, assign52420_e67546_d_n8, assign52420_e67546_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52420_e67544: f64 = (locals.var_x_s__blk1363 + locals.var_x_ds__blk1411);
        (assign52420_e67544, (locals.var_x_s__blk1363_dn4 + locals.var_x_ds__blk1411_dn4), (locals.var_x_s__blk1363_dn6 + locals.var_x_ds__blk1411_dn6), (locals.var_x_s__blk1363_dn7 + locals.var_x_ds__blk1411_dn7), (locals.var_x_s__blk1363_dn8 + locals.var_x_ds__blk1411_dn8), (locals.var_x_s__blk1363_dn9 + locals.var_x_ds__blk1411_dn9),)
    } else {
        (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9,)
    }
};
        locals.var_x_d__blk1410 = assign52420_e67546;
        locals.var_x_d__blk1410_dn4 = assign52420_e67546_d_n4;
        locals.var_x_d__blk1410_dn6 = assign52420_e67546_d_n6;
        locals.var_x_d__blk1410_dn7 = assign52420_e67546_d_n7;
        locals.var_x_d__blk1410_dn8 = assign52420_e67546_d_n8;
        locals.var_x_d__blk1410_dn9 = assign52420_e67546_d_n9;
        locals.var_x_d__blk1410_rv = 0.0;

        let (assign52430_e67554, assign52430_e67554_d_n4, assign52430_e67554_d_n6, assign52430_e67554_d_n7, assign52430_e67554_d_n8, assign52430_e67554_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52430_e67552: f64 = (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339);
        (assign52430_e67552, ((locals.var_x_ds__blk1411_dn4 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn4)), ((locals.var_x_ds__blk1411_dn6 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn6)), ((locals.var_x_ds__blk1411_dn7 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn7)), ((locals.var_x_ds__blk1411_dn8 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn8)), ((locals.var_x_ds__blk1411_dn9 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn9)),)
    } else {
        (locals.var_dps__blk1414, locals.var_dps__blk1414_dn4, locals.var_dps__blk1414_dn6, locals.var_dps__blk1414_dn7, locals.var_dps__blk1414_dn8, locals.var_dps__blk1414_dn9,)
    }
};
        locals.var_dps__blk1414 = assign52430_e67554;
        locals.var_dps__blk1414_dn4 = assign52430_e67554_d_n4;
        locals.var_dps__blk1414_dn6 = assign52430_e67554_d_n6;
        locals.var_dps__blk1414_dn7 = assign52430_e67554_d_n7;
        locals.var_dps__blk1414_dn8 = assign52430_e67554_d_n8;
        locals.var_dps__blk1414_dn9 = assign52430_e67554_d_n9;
        locals.var_dps__blk1414_rv = 0.0;

        let (assign52440_e67568, assign52440_e67568_d_n4, assign52440_e67568_d_n6, assign52440_e67568_d_n7, assign52440_e67568_d_n8, assign52440_e67568_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52440_e67560: f64 = (locals.var_x_d__blk1410 * locals.var_x_d__blk1410);
        let assign52440_e67564: f64 = (locals.var_x_d__blk1410 * locals.var_x_d__blk1410);
        let assign52440_e67565: f64 = (2.0 + assign52440_e67564);
        let assign52440_e67566: f64 = (assign52440_e67560 / assign52440_e67565);
        (assign52440_e67566, (((((locals.var_x_d__blk1410_dn4 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn4)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn4 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn4)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn6 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn6)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn6 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn6)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn7 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn7)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn7 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn7)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn8 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn8)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn8 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn8)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn9 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn9)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn9 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn9)))) / (assign52440_e67565 * assign52440_e67565)),)
    } else {
        (locals.var_xi0d__blk1415, locals.var_xi0d__blk1415_dn4, locals.var_xi0d__blk1415_dn6, locals.var_xi0d__blk1415_dn7, locals.var_xi0d__blk1415_dn8, locals.var_xi0d__blk1415_dn9,)
    }
};
        locals.var_xi0d__blk1415 = assign52440_e67568;
        locals.var_xi0d__blk1415_dn4 = assign52440_e67568_d_n4;
        locals.var_xi0d__blk1415_dn6 = assign52440_e67568_d_n6;
        locals.var_xi0d__blk1415_dn7 = assign52440_e67568_d_n7;
        locals.var_xi0d__blk1415_dn8 = assign52440_e67568_d_n8;
        locals.var_xi0d__blk1415_dn9 = assign52440_e67568_d_n9;
        locals.var_xi0d__blk1415_rv = 0.0;

        let assign52450_e67571: f64 = if locals.var_x_d__blk1410 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign52450_e67571;
        locals.var_guard1513_rv = 0.0;

        let (assign52460_e67581, assign52460_e67581_d_n4, assign52460_e67581_d_n6, assign52460_e67581_d_n7, assign52460_e67581_d_n8, assign52460_e67581_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign52460_e67578: f64 = (-locals.var_x_d__blk1410);
        let assign52460_e67579: f64 = (assign52460_e67578).exp();
        (assign52460_e67579, (assign52460_e67579 * (-locals.var_x_d__blk1410_dn4)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn6)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn7)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn8)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn9)),)
    } else {
        (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9,)
    }
};
        locals.var_ed__blk1416 = assign52460_e67581;
        locals.var_ed__blk1416_dn4 = assign52460_e67581_d_n4;
        locals.var_ed__blk1416_dn6 = assign52460_e67581_d_n6;
        locals.var_ed__blk1416_dn7 = assign52460_e67581_d_n7;
        locals.var_ed__blk1416_dn8 = assign52460_e67581_d_n8;
        locals.var_ed__blk1416_dn9 = assign52460_e67581_d_n9;
        locals.var_ed__blk1416_rv = 0.0;

        let assign52470_e67584: f64 = if locals.var_x_d__blk1410 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1514 = assign52470_e67584;
        locals.var_guard1514_rv = 0.0;

        let (assign52480_e67610, assign52480_e67610_d_n4, assign52480_e67610_d_n6, assign52480_e67610_d_n7, assign52480_e67610_d_n8, assign52480_e67610_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        let assign52480_e67595: f64 = (locals.var_x_d__blk1410 * locals.var_x_d__blk1410);
        let assign52480_e67602: f64 = (0.25 * locals.var_x_d__blk1410);
        let assign52480_e67603: f64 = (1.0 - assign52480_e67602);
        let assign52480_e67604: f64 = (locals.var_x_d__blk1410 * assign52480_e67603);
        let assign52480_e67605: f64 = (0.3333333333333333 * assign52480_e67604);
        let assign52480_e67606: f64 = (1.0 - assign52480_e67605);
        let assign52480_e67607: f64 = (assign52480_e67595 * assign52480_e67606);
        let assign52480_e67608: f64 = (0.5 * assign52480_e67607);
        (assign52480_e67608, (0.5 * ((((locals.var_x_d__blk1410_dn4 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn4)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn4 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn4))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn6 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn6)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn6 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn6))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn7 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn7)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn7 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn7))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn8 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn8)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn8 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn8))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn9 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn9)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn9 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn9))))))))),)
    } else {
        (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9,)
    }
};
        locals.var_pd__blk1417 = assign52480_e67610;
        locals.var_pd__blk1417_dn4 = assign52480_e67610_d_n4;
        locals.var_pd__blk1417_dn6 = assign52480_e67610_d_n6;
        locals.var_pd__blk1417_dn7 = assign52480_e67610_d_n7;
        locals.var_pd__blk1417_dn8 = assign52480_e67610_d_n8;
        locals.var_pd__blk1417_dn9 = assign52480_e67610_d_n9;
        locals.var_pd__blk1417_rv = 0.0;

        let (assign52490_e67631, assign52490_e67631_d_n4, assign52490_e67631_d_n6, assign52490_e67631_d_n7, assign52490_e67631_d_n8, assign52490_e67631_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        let assign52490_e67624: f64 = (0.25 * locals.var_x_d__blk1410);
        let assign52490_e67625: f64 = (1.0 - assign52490_e67624);
        let assign52490_e67626: f64 = (locals.var_x_d__blk1410 * assign52490_e67625);
        let assign52490_e67627: f64 = (0.3333333333333333 * assign52490_e67626);
        let assign52490_e67628: f64 = (1.0 - assign52490_e67627);
        let assign52490_e67629: f64 = (assign52490_e67628).sqrt();
        (assign52490_e67629, ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn4 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn4)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn6 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn6)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn7 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn7)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn8 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn8)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn9 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn9)))))) / (2.0 * assign52490_e67629)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52490_e67631;
        locals.var_temp__blk949_dn4 = assign52490_e67631_d_n4;
        locals.var_temp__blk949_dn6 = assign52490_e67631_d_n6;
        locals.var_temp__blk949_dn7 = assign52490_e67631_d_n7;
        locals.var_temp__blk949_dn8 = assign52490_e67631_d_n8;
        locals.var_temp__blk949_dn9 = assign52490_e67631_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign52500_e67645, assign52500_e67645_d_n4, assign52500_e67645_d_n6, assign52500_e67645_d_n7, assign52500_e67645_d_n8, assign52500_e67645_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        let assign52500_e67642: f64 = (locals.var_x_d__blk1410 * locals.var_temp__blk949);
        let assign52500_e67643: f64 = (0.7071067811865475 * assign52500_e67642);
        (assign52500_e67643, (0.7071067811865475 * ((locals.var_x_d__blk1410_dn4 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn6 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn7 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn8 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn9 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqd__blk1418, locals.var_sqd__blk1418_dn4, locals.var_sqd__blk1418_dn6, locals.var_sqd__blk1418_dn7, locals.var_sqd__blk1418_dn8, locals.var_sqd__blk1418_dn9,)
    }
};
        locals.var_sqd__blk1418 = assign52500_e67645;
        locals.var_sqd__blk1418_dn4 = assign52500_e67645_d_n4;
        locals.var_sqd__blk1418_dn6 = assign52500_e67645_d_n6;
        locals.var_sqd__blk1418_dn7 = assign52500_e67645_d_n7;
        locals.var_sqd__blk1418_dn8 = assign52500_e67645_d_n8;
        locals.var_sqd__blk1418_dn9 = assign52500_e67645_d_n9;
        locals.var_sqd__blk1418_rv = 0.0;

        let (assign52510_e67669, assign52510_e67669_d_n4, assign52510_e67669_d_n6, assign52510_e67669_d_n7, assign52510_e67669_d_n8, assign52510_e67669_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        let assign52510_e67655: f64 = (0.16666666666666666 * locals.var_delta_nd__blk1409);
        let assign52510_e67657: f64 = (assign52510_e67655 * locals.var_x_d__blk1410);
        let assign52510_e67659: f64 = (assign52510_e67657 * locals.var_x_d__blk1410);
        let assign52510_e67661: f64 = (assign52510_e67659 * locals.var_x_d__blk1410);
        let assign52510_e67665: f64 = (1.75 * locals.var_x_d__blk1410);
        let assign52510_e67666: f64 = (1.0 + assign52510_e67665);
        let assign52510_e67667: f64 = (assign52510_e67661 * assign52510_e67666);
        (assign52510_e67667, (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn4) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn4)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn4)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn4)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn4))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn6) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn6)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn6)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn6)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn6))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn7) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn7)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn7)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn7)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn7))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn8) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn8)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn8)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn8)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn8))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn9) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn9)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn9)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn9)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn9))),)
    } else {
        (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9,)
    }
};
        locals.var_dd__blk1419 = assign52510_e67669;
        locals.var_dd__blk1419_dn4 = assign52510_e67669_d_n4;
        locals.var_dd__blk1419_dn6 = assign52510_e67669_d_n6;
        locals.var_dd__blk1419_dn7 = assign52510_e67669_d_n7;
        locals.var_dd__blk1419_dn8 = assign52510_e67669_d_n8;
        locals.var_dd__blk1419_dn9 = assign52510_e67669_d_n9;
        locals.var_dd__blk1419_rv = 0.0;

        let (assign52520_e67684, assign52520_e67684_d_n4, assign52520_e67684_d_n6, assign52520_e67684_d_n7, assign52520_e67684_d_n8, assign52520_e67684_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 == 0.0)) {
        let assign52520_e67680: f64 = (locals.var_x_d__blk1410 - 1.0);
        let assign52520_e67682: f64 = (assign52520_e67680 + locals.var_ed__blk1416);
        (assign52520_e67682, (locals.var_x_d__blk1410_dn4 + locals.var_ed__blk1416_dn4), (locals.var_x_d__blk1410_dn6 + locals.var_ed__blk1416_dn6), (locals.var_x_d__blk1410_dn7 + locals.var_ed__blk1416_dn7), (locals.var_x_d__blk1410_dn8 + locals.var_ed__blk1416_dn8), (locals.var_x_d__blk1410_dn9 + locals.var_ed__blk1416_dn9),)
    } else {
        (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9,)
    }
};
        locals.var_pd__blk1417 = assign52520_e67684;
        locals.var_pd__blk1417_dn4 = assign52520_e67684_d_n4;
        locals.var_pd__blk1417_dn6 = assign52520_e67684_d_n6;
        locals.var_pd__blk1417_dn7 = assign52520_e67684_d_n7;
        locals.var_pd__blk1417_dn8 = assign52520_e67684_d_n8;
        locals.var_pd__blk1417_dn9 = assign52520_e67684_d_n9;
        locals.var_pd__blk1417_rv = 0.0;

        let (assign52530_e67696, assign52530_e67696_d_n4, assign52530_e67696_d_n6, assign52530_e67696_d_n7, assign52530_e67696_d_n8, assign52530_e67696_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 == 0.0)) {
        let assign52530_e67694: f64 = (locals.var_pd__blk1417).sqrt();
        (assign52530_e67694, (locals.var_pd__blk1417_dn4 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn6 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn7 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn8 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn9 / (2.0 * assign52530_e67694)),)
    } else {
        (locals.var_sqd__blk1418, locals.var_sqd__blk1418_dn4, locals.var_sqd__blk1418_dn6, locals.var_sqd__blk1418_dn7, locals.var_sqd__blk1418_dn8, locals.var_sqd__blk1418_dn9,)
    }
};
        locals.var_sqd__blk1418 = assign52530_e67696;
        locals.var_sqd__blk1418_dn4 = assign52530_e67696_d_n4;
        locals.var_sqd__blk1418_dn6 = assign52530_e67696_d_n6;
        locals.var_sqd__blk1418_dn7 = assign52530_e67696_d_n7;
        locals.var_sqd__blk1418_dn8 = assign52530_e67696_d_n8;
        locals.var_sqd__blk1418_dn9 = assign52530_e67696_d_n9;
        locals.var_sqd__blk1418_rv = 0.0;

        let (assign52540_e67717, assign52540_e67717_d_n4, assign52540_e67717_d_n6, assign52540_e67717_d_n7, assign52540_e67717_d_n8, assign52540_e67717_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 == 0.0)) {
        let assign52540_e67708: f64 = (1.0 / locals.var_ed__blk1416);
        let assign52540_e67710: f64 = (assign52540_e67708 - locals.var_x_d__blk1410);
        let assign52540_e67712: f64 = (assign52540_e67710 - 1.0);
        let assign52540_e67714: f64 = (assign52540_e67712 - locals.var_xi0d__blk1415);
        let assign52540_e67715: f64 = (locals.var_delta_nd__blk1409 * assign52540_e67714);
        (assign52540_e67715, ((locals.var_delta_nd__blk1409_dn4 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn4 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn4) - locals.var_xi0d__blk1415_dn4))), ((locals.var_delta_nd__blk1409_dn6 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn6 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn6) - locals.var_xi0d__blk1415_dn6))), ((locals.var_delta_nd__blk1409_dn7 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn7 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn7) - locals.var_xi0d__blk1415_dn7))), ((locals.var_delta_nd__blk1409_dn8 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn8 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn8) - locals.var_xi0d__blk1415_dn8))), ((locals.var_delta_nd__blk1409_dn9 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn9 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn9) - locals.var_xi0d__blk1415_dn9))),)
    } else {
        (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9,)
    }
};
        locals.var_dd__blk1419 = assign52540_e67717;
        locals.var_dd__blk1419_dn4 = assign52540_e67717_d_n4;
        locals.var_dd__blk1419_dn6 = assign52540_e67717_d_n6;
        locals.var_dd__blk1419_dn7 = assign52540_e67717_d_n7;
        locals.var_dd__blk1419_dn8 = assign52540_e67717_d_n8;
        locals.var_dd__blk1419_dn9 = assign52540_e67717_d_n9;
        locals.var_dd__blk1419_rv = 0.0;

        let assign52550_e67721: f64 = (locals.var_xn_d__blk1407 - 230.25850929940458);
        let assign52550_e67722: f64 = if locals.var_x_d__blk1410 > assign52550_e67721 { 1.0 } else { 0.0 };
        locals.var_guard1515 = assign52550_e67722;
        locals.var_guard1515_rv = 0.0;

        let (assign52560_e67736, assign52560_e67736_d_n4, assign52560_e67736_d_n6, assign52560_e67736_d_n7, assign52560_e67736_d_n8, assign52560_e67736_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign52560_e67733: f64 = (locals.var_x_d__blk1410 - locals.var_xn_d__blk1407);
        let assign52560_e67734: f64 = (assign52560_e67733).exp();
        (assign52560_e67734, (assign52560_e67734 * (locals.var_x_d__blk1410_dn4 - locals.var_xn_d__blk1407_dn4)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn6 - locals.var_xn_d__blk1407_dn6)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn7 - locals.var_xn_d__blk1407_dn7)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn8 - locals.var_xn_d__blk1407_dn8)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn9 - locals.var_xn_d__blk1407_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52560_e67736;
        locals.var_temp__blk949_dn4 = assign52560_e67736_d_n4;
        locals.var_temp__blk949_dn6 = assign52560_e67736_d_n6;
        locals.var_temp__blk949_dn7 = assign52560_e67736_d_n7;
        locals.var_temp__blk949_dn8 = assign52560_e67736_d_n8;
        locals.var_temp__blk949_dn9 = assign52560_e67736_d_n9;
        locals.var_temp__blk949_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_50(
        locals: &mut StampLocals,
    ) {
        let (assign52570_e67749, assign52570_e67749_d_n4, assign52570_e67749_d_n6, assign52570_e67749_d_n7, assign52570_e67749_d_n8, assign52570_e67749_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign52570_e67747: f64 = (locals.var_delta_nd__blk1409 / locals.var_temp__blk949);
        (assign52570_e67747, (((locals.var_delta_nd__blk1409_dn4 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn6 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn7 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn8 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn9 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)),)
    } else {
        (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9,)
    }
};
        locals.var_ed__blk1416 = assign52570_e67749;
        locals.var_ed__blk1416_dn4 = assign52570_e67749_d_n4;
        locals.var_ed__blk1416_dn6 = assign52570_e67749_d_n6;
        locals.var_ed__blk1416_dn7 = assign52570_e67749_d_n7;
        locals.var_ed__blk1416_dn8 = assign52570_e67749_d_n8;
        locals.var_ed__blk1416_dn9 = assign52570_e67749_d_n9;
        locals.var_ed__blk1416_rv = 0.0;

        let (assign52580_e67768, assign52580_e67768_d_n4, assign52580_e67768_d_n6, assign52580_e67768_d_n7, assign52580_e67768_d_n8, assign52580_e67768_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign52580_e67762: f64 = (locals.var_x_d__blk1410 + 1.0);
        let assign52580_e67764: f64 = (assign52580_e67762 + locals.var_xi0d__blk1415);
        let assign52580_e67765: f64 = (locals.var_delta_nd__blk1409 * assign52580_e67764);
        let assign52580_e67766: f64 = (locals.var_temp__blk949 - assign52580_e67765);
        (assign52580_e67766, (locals.var_temp__blk949_dn4 - ((locals.var_delta_nd__blk1409_dn4 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn4 + locals.var_xi0d__blk1415_dn4)))), (locals.var_temp__blk949_dn6 - ((locals.var_delta_nd__blk1409_dn6 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn6 + locals.var_xi0d__blk1415_dn6)))), (locals.var_temp__blk949_dn7 - ((locals.var_delta_nd__blk1409_dn7 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn7 + locals.var_xi0d__blk1415_dn7)))), (locals.var_temp__blk949_dn8 - ((locals.var_delta_nd__blk1409_dn8 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn8 + locals.var_xi0d__blk1415_dn8)))), (locals.var_temp__blk949_dn9 - ((locals.var_delta_nd__blk1409_dn9 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn9 + locals.var_xi0d__blk1415_dn9)))),)
    } else {
        (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9,)
    }
};
        locals.var_dd__blk1419 = assign52580_e67768;
        locals.var_dd__blk1419_dn4 = assign52580_e67768_d_n4;
        locals.var_dd__blk1419_dn6 = assign52580_e67768_d_n6;
        locals.var_dd__blk1419_dn7 = assign52580_e67768_d_n7;
        locals.var_dd__blk1419_dn8 = assign52580_e67768_d_n8;
        locals.var_dd__blk1419_dn9 = assign52580_e67768_d_n9;
        locals.var_dd__blk1419_rv = 0.0;

        let (assign52590_e67802, assign52590_e67802_d_n4, assign52590_e67802_d_n6, assign52590_e67802_d_n7, assign52590_e67802_d_n8, assign52590_e67802_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 == 0.0)) {
        let assign52590_e67782: f64 = (locals.var_x_d__blk1410 - 230.25850929940458);
        let assign52590_e67787: f64 = (locals.var_x_d__blk1410 - 230.25850929940458);
        let assign52590_e67791: f64 = (locals.var_x_d__blk1410 - 230.25850929940458);
        let assign52590_e67793: f64 = (assign52590_e67791 * 0.3333333333333333);
        let assign52590_e67794: f64 = (1.0 + assign52590_e67793);
        let assign52590_e67795: f64 = (assign52590_e67787 * assign52590_e67794);
        let assign52590_e67796: f64 = (0.5 * assign52590_e67795);
        let assign52590_e67797: f64 = (1.0 + assign52590_e67796);
        let assign52590_e67798: f64 = (assign52590_e67782 * assign52590_e67797);
        let assign52590_e67799: f64 = (1.0 + assign52590_e67798);
        let assign52590_e67800: f64 = (1e-100 / assign52590_e67799);
        (assign52590_e67800, (-((1e-100 * ((locals.var_x_d__blk1410_dn4 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn4 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn4 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn6 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn6 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn6 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn7 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn7 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn7 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn8 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn8 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn8 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn9 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn9 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn9 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))),)
    } else {
        (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9,)
    }
};
        locals.var_ed__blk1416 = assign52590_e67802;
        locals.var_ed__blk1416_dn4 = assign52590_e67802_d_n4;
        locals.var_ed__blk1416_dn6 = assign52590_e67802_d_n6;
        locals.var_ed__blk1416_dn7 = assign52590_e67802_d_n7;
        locals.var_ed__blk1416_dn8 = assign52590_e67802_d_n8;
        locals.var_ed__blk1416_dn9 = assign52590_e67802_d_n9;
        locals.var_ed__blk1416_rv = 0.0;

        let (assign52600_e67842, assign52600_e67842_d_n4, assign52600_e67842_d_n6, assign52600_e67842_d_n7, assign52600_e67842_d_n8, assign52600_e67842_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 == 0.0)) {
        let assign52600_e67816: f64 = (locals.var_xn_d__blk1407 - locals.var_x_d__blk1410);
        let assign52600_e67818: f64 = (assign52600_e67816 - 230.25850929940458);
        let assign52600_e67823: f64 = (locals.var_xn_d__blk1407 - locals.var_x_d__blk1410);
        let assign52600_e67825: f64 = (assign52600_e67823 - 230.25850929940458);
        let assign52600_e67829: f64 = (locals.var_xn_d__blk1407 - locals.var_x_d__blk1410);
        let assign52600_e67831: f64 = (assign52600_e67829 - 230.25850929940458);
        let assign52600_e67833: f64 = (assign52600_e67831 * 0.3333333333333333);
        let assign52600_e67834: f64 = (1.0 + assign52600_e67833);
        let assign52600_e67835: f64 = (assign52600_e67825 * assign52600_e67834);
        let assign52600_e67836: f64 = (0.5 * assign52600_e67835);
        let assign52600_e67837: f64 = (1.0 + assign52600_e67836);
        let assign52600_e67838: f64 = (assign52600_e67818 * assign52600_e67837);
        let assign52600_e67839: f64 = (1.0 + assign52600_e67838);
        let assign52600_e67840: f64 = (1e-100 / assign52600_e67839);
        (assign52600_e67840, (-((1e-100 * (((locals.var_xn_d__blk1407_dn4 - locals.var_x_d__blk1410_dn4) * assign52600_e67837) + (assign52600_e67818 * (0.5 * (((locals.var_xn_d__blk1407_dn4 - locals.var_x_d__blk1410_dn4) * assign52600_e67834) + (assign52600_e67825 * ((locals.var_xn_d__blk1407_dn4 - locals.var_x_d__blk1410_dn4) * 0.3333333333333333))))))) / (assign52600_e67839 * assign52600_e67839))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn6 - locals.var_x_d__blk1410_dn6) * assign52600_e67837) + (assign52600_e67818 * (0.5 * (((locals.var_xn_d__blk1407_dn6 - locals.var_x_d__blk1410_dn6) * assign52600_e67834) + (assign52600_e67825 * ((locals.var_xn_d__blk1407_dn6 - locals.var_x_d__blk1410_dn6) * 0.3333333333333333))))))) / (assign52600_e67839 * assign52600_e67839))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn7 - locals.var_x_d__blk1410_dn7) * assign52600_e67837) + (assign52600_e67818 * (0.5 * (((locals.var_xn_d__blk1407_dn7 - locals.var_x_d__blk1410_dn7) * assign52600_e67834) + (assign52600_e67825 * ((locals.var_xn_d__blk1407_dn7 - locals.var_x_d__blk1410_dn7) * 0.3333333333333333))))))) / (assign52600_e67839 * assign52600_e67839))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn8 - locals.var_x_d__blk1410_dn8) * assign52600_e67837) + (assign52600_e67818 * (0.5 * (((locals.var_xn_d__blk1407_dn8 - locals.var_x_d__blk1410_dn8) * assign52600_e67834) + (assign52600_e67825 * ((locals.var_xn_d__blk1407_dn8 - locals.var_x_d__blk1410_dn8) * 0.3333333333333333))))))) / (assign52600_e67839 * assign52600_e67839))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn9 - locals.var_x_d__blk1410_dn9) * assign52600_e67837) + (assign52600_e67818 * (0.5 * (((locals.var_xn_d__blk1407_dn9 - locals.var_x_d__blk1410_dn9) * assign52600_e67834) + (assign52600_e67825 * ((locals.var_xn_d__blk1407_dn9 - locals.var_x_d__blk1410_dn9) * 0.3333333333333333))))))) / (assign52600_e67839 * assign52600_e67839))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52600_e67842;
        locals.var_temp__blk949_dn4 = assign52600_e67842_d_n4;
        locals.var_temp__blk949_dn6 = assign52600_e67842_d_n6;
        locals.var_temp__blk949_dn7 = assign52600_e67842_d_n7;
        locals.var_temp__blk949_dn8 = assign52600_e67842_d_n8;
        locals.var_temp__blk949_dn9 = assign52600_e67842_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign52610_e67862, assign52610_e67862_d_n4, assign52610_e67862_d_n6, assign52610_e67862_d_n7, assign52610_e67862_d_n8, assign52610_e67862_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 == 0.0)) {
        let assign52610_e67856: f64 = (locals.var_x_d__blk1410 + 1.0);
        let assign52610_e67858: f64 = (assign52610_e67856 + locals.var_xi0d__blk1415);
        let assign52610_e67859: f64 = (locals.var_delta_nd__blk1409 * assign52610_e67858);
        let assign52610_e67860: f64 = (locals.var_temp__blk949 - assign52610_e67859);
        (assign52610_e67860, (locals.var_temp__blk949_dn4 - ((locals.var_delta_nd__blk1409_dn4 * assign52610_e67858) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn4 + locals.var_xi0d__blk1415_dn4)))), (locals.var_temp__blk949_dn6 - ((locals.var_delta_nd__blk1409_dn6 * assign52610_e67858) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn6 + locals.var_xi0d__blk1415_dn6)))), (locals.var_temp__blk949_dn7 - ((locals.var_delta_nd__blk1409_dn7 * assign52610_e67858) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn7 + locals.var_xi0d__blk1415_dn7)))), (locals.var_temp__blk949_dn8 - ((locals.var_delta_nd__blk1409_dn8 * assign52610_e67858) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn8 + locals.var_xi0d__blk1415_dn8)))), (locals.var_temp__blk949_dn9 - ((locals.var_delta_nd__blk1409_dn9 * assign52610_e67858) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn9 + locals.var_xi0d__blk1415_dn9)))),)
    } else {
        (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9,)
    }
};
        locals.var_dd__blk1419 = assign52610_e67862;
        locals.var_dd__blk1419_dn4 = assign52610_e67862_d_n4;
        locals.var_dd__blk1419_dn6 = assign52610_e67862_d_n6;
        locals.var_dd__blk1419_dn7 = assign52610_e67862_d_n7;
        locals.var_dd__blk1419_dn8 = assign52610_e67862_d_n8;
        locals.var_dd__blk1419_dn9 = assign52610_e67862_d_n9;
        locals.var_dd__blk1419_rv = 0.0;

        let (assign52620_e67875, assign52620_e67875_d_n4, assign52620_e67875_d_n6, assign52620_e67875_d_n7, assign52620_e67875_d_n8, assign52620_e67875_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) {
        let assign52620_e67871: f64 = (locals.var_x_d__blk1410 - 1.0);
        let assign52620_e67873: f64 = (assign52620_e67871 + locals.var_ed__blk1416);
        (assign52620_e67873, (locals.var_x_d__blk1410_dn4 + locals.var_ed__blk1416_dn4), (locals.var_x_d__blk1410_dn6 + locals.var_ed__blk1416_dn6), (locals.var_x_d__blk1410_dn7 + locals.var_ed__blk1416_dn7), (locals.var_x_d__blk1410_dn8 + locals.var_ed__blk1416_dn8), (locals.var_x_d__blk1410_dn9 + locals.var_ed__blk1416_dn9),)
    } else {
        (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9,)
    }
};
        locals.var_pd__blk1417 = assign52620_e67875;
        locals.var_pd__blk1417_dn4 = assign52620_e67875_d_n4;
        locals.var_pd__blk1417_dn6 = assign52620_e67875_d_n6;
        locals.var_pd__blk1417_dn7 = assign52620_e67875_d_n7;
        locals.var_pd__blk1417_dn8 = assign52620_e67875_d_n8;
        locals.var_pd__blk1417_dn9 = assign52620_e67875_d_n9;
        locals.var_pd__blk1417_rv = 0.0;

        let (assign52630_e67885, assign52630_e67885_d_n4, assign52630_e67885_d_n6, assign52630_e67885_d_n7, assign52630_e67885_d_n8, assign52630_e67885_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) {
        let assign52630_e67883: f64 = (locals.var_pd__blk1417).sqrt();
        (assign52630_e67883, (locals.var_pd__blk1417_dn4 / (2.0 * assign52630_e67883)), (locals.var_pd__blk1417_dn6 / (2.0 * assign52630_e67883)), (locals.var_pd__blk1417_dn7 / (2.0 * assign52630_e67883)), (locals.var_pd__blk1417_dn8 / (2.0 * assign52630_e67883)), (locals.var_pd__blk1417_dn9 / (2.0 * assign52630_e67883)),)
    } else {
        (locals.var_sqd__blk1418, locals.var_sqd__blk1418_dn4, locals.var_sqd__blk1418_dn6, locals.var_sqd__blk1418_dn7, locals.var_sqd__blk1418_dn8, locals.var_sqd__blk1418_dn9,)
    }
};
        locals.var_sqd__blk1418 = assign52630_e67885;
        locals.var_sqd__blk1418_dn4 = assign52630_e67885_d_n4;
        locals.var_sqd__blk1418_dn6 = assign52630_e67885_d_n6;
        locals.var_sqd__blk1418_dn7 = assign52630_e67885_d_n7;
        locals.var_sqd__blk1418_dn8 = assign52630_e67885_d_n8;
        locals.var_sqd__blk1418_dn9 = assign52630_e67885_d_n9;
        locals.var_sqd__blk1418_rv = 0.0;

        let (assign52640_e67895, assign52640_e67895_d_n4, assign52640_e67895_d_n6, assign52640_e67895_d_n7, assign52640_e67895_d_n8, assign52640_e67895_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52640_e67891: f64 = (locals.var_sqd__blk1418 * locals.var_gf__blk1324);
        let assign52640_e67893: f64 = (assign52640_e67891 * locals.var_phit1__blk1339);
        (assign52640_e67893, ((((locals.var_sqd__blk1418_dn4 * locals.var_gf__blk1324) + (locals.var_sqd__blk1418 * locals.var_gf__blk1324_dn4)) * locals.var_phit1__blk1339) + (assign52640_e67891 * locals.var_phit1__blk1339_dn4)), ((((locals.var_sqd__blk1418_dn6 * locals.var_gf__blk1324) + (locals.var_sqd__blk1418 * locals.var_gf__blk1324_dn6)) * locals.var_phit1__blk1339) + (assign52640_e67891 * locals.var_phit1__blk1339_dn6)), ((((locals.var_sqd__blk1418_dn7 * locals.var_gf__blk1324) + (locals.var_sqd__blk1418 * locals.var_gf__blk1324_dn7)) * locals.var_phit1__blk1339) + (assign52640_e67891 * locals.var_phit1__blk1339_dn7)), ((((locals.var_sqd__blk1418_dn8 * locals.var_gf__blk1324) + (locals.var_sqd__blk1418 * locals.var_gf__blk1324_dn8)) * locals.var_phit1__blk1339) + (assign52640_e67891 * locals.var_phit1__blk1339_dn8)), ((((locals.var_sqd__blk1418_dn9 * locals.var_gf__blk1324) + (locals.var_sqd__blk1418 * locals.var_gf__blk1324_dn9)) * locals.var_phit1__blk1339) + (assign52640_e67891 * locals.var_phit1__blk1339_dn9)),)
    } else {
        (locals.var_qbd__blk1420, locals.var_qbd__blk1420_dn4, locals.var_qbd__blk1420_dn6, locals.var_qbd__blk1420_dn7, locals.var_qbd__blk1420_dn8, locals.var_qbd__blk1420_dn9,)
    }
};
        locals.var_qbd__blk1420 = assign52640_e67895;
        locals.var_qbd__blk1420_dn4 = assign52640_e67895_d_n4;
        locals.var_qbd__blk1420_dn6 = assign52640_e67895_d_n6;
        locals.var_qbd__blk1420_dn7 = assign52640_e67895_d_n7;
        locals.var_qbd__blk1420_dn8 = assign52640_e67895_d_n8;
        locals.var_qbd__blk1420_dn9 = assign52640_e67895_d_n9;
        locals.var_qbd__blk1420_rv = 0.0;

        let (assign52650_e67905, assign52650_e67905_d_n4, assign52650_e67905_d_n6, assign52650_e67905_d_n7, assign52650_e67905_d_n8, assign52650_e67905_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52650_e67902: f64 = (locals.var_x_s__blk1363 + locals.var_x_d__blk1410);
        let assign52650_e67903: f64 = (0.5 * assign52650_e67902);
        (assign52650_e67903, (0.5 * (locals.var_x_s__blk1363_dn4 + locals.var_x_d__blk1410_dn4)), (0.5 * (locals.var_x_s__blk1363_dn6 + locals.var_x_d__blk1410_dn6)), (0.5 * (locals.var_x_s__blk1363_dn7 + locals.var_x_d__blk1410_dn7)), (0.5 * (locals.var_x_s__blk1363_dn8 + locals.var_x_d__blk1410_dn8)), (0.5 * (locals.var_x_s__blk1363_dn9 + locals.var_x_d__blk1410_dn9)),)
    } else {
        (locals.var_x_m__blk1421, locals.var_x_m__blk1421_dn4, locals.var_x_m__blk1421_dn6, locals.var_x_m__blk1421_dn7, locals.var_x_m__blk1421_dn8, locals.var_x_m__blk1421_dn9,)
    }
};
        locals.var_x_m__blk1421 = assign52650_e67905;
        locals.var_x_m__blk1421_dn4 = assign52650_e67905_d_n4;
        locals.var_x_m__blk1421_dn6 = assign52650_e67905_d_n6;
        locals.var_x_m__blk1421_dn7 = assign52650_e67905_d_n7;
        locals.var_x_m__blk1421_dn8 = assign52650_e67905_d_n8;
        locals.var_x_m__blk1421_dn9 = assign52650_e67905_d_n9;
        locals.var_x_m__blk1421_rv = 0.0;

        let (assign52660_e67911, assign52660_e67911_d_n4, assign52660_e67911_d_n6, assign52660_e67911_d_n7, assign52660_e67911_d_n8, assign52660_e67911_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_em__blk1422, locals.var_em__blk1422_dn4, locals.var_em__blk1422_dn6, locals.var_em__blk1422_dn7, locals.var_em__blk1422_dn8, locals.var_em__blk1422_dn9,)
    }
};
        locals.var_em__blk1422 = assign52660_e67911;
        locals.var_em__blk1422_dn4 = assign52660_e67911_d_n4;
        locals.var_em__blk1422_dn6 = assign52660_e67911_d_n6;
        locals.var_em__blk1422_dn7 = assign52660_e67911_d_n7;
        locals.var_em__blk1422_dn8 = assign52660_e67911_d_n8;
        locals.var_em__blk1422_dn9 = assign52660_e67911_d_n9;
        locals.var_em__blk1422_rv = 0.0;

        let (assign52670_e67919, assign52670_e67919_d_n4, assign52670_e67919_d_n6, assign52670_e67919_d_n7, assign52670_e67919_d_n8, assign52670_e67919_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52670_e67917: f64 = (locals.var_ed__blk1416 * locals.var_es__blk1369);
        (assign52670_e67917, ((locals.var_ed__blk1416_dn4 * locals.var_es__blk1369) + (locals.var_ed__blk1416 * locals.var_es__blk1369_dn4)), ((locals.var_ed__blk1416_dn6 * locals.var_es__blk1369) + (locals.var_ed__blk1416 * locals.var_es__blk1369_dn6)), ((locals.var_ed__blk1416_dn7 * locals.var_es__blk1369) + (locals.var_ed__blk1416 * locals.var_es__blk1369_dn7)), ((locals.var_ed__blk1416_dn8 * locals.var_es__blk1369) + (locals.var_ed__blk1416 * locals.var_es__blk1369_dn8)), ((locals.var_ed__blk1416_dn9 * locals.var_es__blk1369) + (locals.var_ed__blk1416 * locals.var_es__blk1369_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52670_e67919;
        locals.var_temp__blk949_dn4 = assign52670_e67919_d_n4;
        locals.var_temp__blk949_dn6 = assign52670_e67919_d_n6;
        locals.var_temp__blk949_dn7 = assign52670_e67919_d_n7;
        locals.var_temp__blk949_dn8 = assign52670_e67919_d_n8;
        locals.var_temp__blk949_dn9 = assign52670_e67919_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign52680_e67922: f64 = if locals.var_temp__blk949 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1516 = assign52680_e67922;
        locals.var_guard1516_rv = 0.0;

        let (assign52690_e67931, assign52690_e67931_d_n4, assign52690_e67931_d_n6, assign52690_e67931_d_n7, assign52690_e67931_d_n8, assign52690_e67931_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1516 != 0.0)) {
        let assign52690_e67929: f64 = (locals.var_temp__blk949).sqrt();
        (assign52690_e67929, (locals.var_temp__blk949_dn4 / (2.0 * assign52690_e67929)), (locals.var_temp__blk949_dn6 / (2.0 * assign52690_e67929)), (locals.var_temp__blk949_dn7 / (2.0 * assign52690_e67929)), (locals.var_temp__blk949_dn8 / (2.0 * assign52690_e67929)), (locals.var_temp__blk949_dn9 / (2.0 * assign52690_e67929)),)
    } else {
        (locals.var_em__blk1422, locals.var_em__blk1422_dn4, locals.var_em__blk1422_dn6, locals.var_em__blk1422_dn7, locals.var_em__blk1422_dn8, locals.var_em__blk1422_dn9,)
    }
};
        locals.var_em__blk1422 = assign52690_e67931;
        locals.var_em__blk1422_dn4 = assign52690_e67931_d_n4;
        locals.var_em__blk1422_dn6 = assign52690_e67931_d_n6;
        locals.var_em__blk1422_dn7 = assign52690_e67931_d_n7;
        locals.var_em__blk1422_dn8 = assign52690_e67931_d_n8;
        locals.var_em__blk1422_dn9 = assign52690_e67931_d_n9;
        locals.var_em__blk1422_rv = 0.0;

        let (assign52700_e67941, assign52700_e67941_d_n4, assign52700_e67941_d_n6, assign52700_e67941_d_n7, assign52700_e67941_d_n8, assign52700_e67941_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52700_e67938: f64 = (locals.var_ds__blk1370 + locals.var_dd__blk1419);
        let assign52700_e67939: f64 = (0.5 * assign52700_e67938);
        (assign52700_e67939, (0.5 * (locals.var_ds__blk1370_dn4 + locals.var_dd__blk1419_dn4)), (0.5 * (locals.var_ds__blk1370_dn6 + locals.var_dd__blk1419_dn6)), (0.5 * (locals.var_ds__blk1370_dn7 + locals.var_dd__blk1419_dn7)), (0.5 * (locals.var_ds__blk1370_dn8 + locals.var_dd__blk1419_dn8)), (0.5 * (locals.var_ds__blk1370_dn9 + locals.var_dd__blk1419_dn9)),)
    } else {
        (locals.var_d_bar__blk1423, locals.var_d_bar__blk1423_dn4, locals.var_d_bar__blk1423_dn6, locals.var_d_bar__blk1423_dn7, locals.var_d_bar__blk1423_dn8, locals.var_d_bar__blk1423_dn9,)
    }
};
        locals.var_d_bar__blk1423 = assign52700_e67941;
        locals.var_d_bar__blk1423_dn4 = assign52700_e67941_d_n4;
        locals.var_d_bar__blk1423_dn6 = assign52700_e67941_d_n6;
        locals.var_d_bar__blk1423_dn7 = assign52700_e67941_d_n7;
        locals.var_d_bar__blk1423_dn8 = assign52700_e67941_d_n8;
        locals.var_d_bar__blk1423_dn9 = assign52700_e67941_d_n9;
        locals.var_d_bar__blk1423_rv = 0.0;

        let (assign52710_e67959, assign52710_e67959_d_n4, assign52710_e67959_d_n6, assign52710_e67959_d_n7, assign52710_e67959_d_n8, assign52710_e67959_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52710_e67949: f64 = (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411);
        let assign52710_e67953: f64 = (2.0 * locals.var_inv_gf2__blk1341);
        let assign52710_e67954: f64 = (locals.var_em__blk1422 - assign52710_e67953);
        let assign52710_e67955: f64 = (assign52710_e67949 * assign52710_e67954);
        let assign52710_e67956: f64 = (0.125 * assign52710_e67955);
        let assign52710_e67957: f64 = (locals.var_d_bar__blk1423 + assign52710_e67956);
        (assign52710_e67957, (locals.var_d_bar__blk1423_dn4 + (0.125 * ((((locals.var_x_ds__blk1411_dn4 * locals.var_x_ds__blk1411) + (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411_dn4)) * assign52710_e67954) + (assign52710_e67949 * (locals.var_em__blk1422_dn4 - (2.0 * locals.var_inv_gf2__blk1341_dn4)))))), (locals.var_d_bar__blk1423_dn6 + (0.125 * ((((locals.var_x_ds__blk1411_dn6 * locals.var_x_ds__blk1411) + (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411_dn6)) * assign52710_e67954) + (assign52710_e67949 * (locals.var_em__blk1422_dn6 - (2.0 * locals.var_inv_gf2__blk1341_dn6)))))), (locals.var_d_bar__blk1423_dn7 + (0.125 * ((((locals.var_x_ds__blk1411_dn7 * locals.var_x_ds__blk1411) + (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411_dn7)) * assign52710_e67954) + (assign52710_e67949 * (locals.var_em__blk1422_dn7 - (2.0 * locals.var_inv_gf2__blk1341_dn7)))))), (locals.var_d_bar__blk1423_dn8 + (0.125 * ((((locals.var_x_ds__blk1411_dn8 * locals.var_x_ds__blk1411) + (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411_dn8)) * assign52710_e67954) + (assign52710_e67949 * (locals.var_em__blk1422_dn8 - (2.0 * locals.var_inv_gf2__blk1341_dn8)))))), (locals.var_d_bar__blk1423_dn9 + (0.125 * ((((locals.var_x_ds__blk1411_dn9 * locals.var_x_ds__blk1411) + (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411_dn9)) * assign52710_e67954) + (assign52710_e67949 * (locals.var_em__blk1422_dn9 - (2.0 * locals.var_inv_gf2__blk1341_dn9)))))),)
    } else {
        (locals.var_dm__blk1424, locals.var_dm__blk1424_dn4, locals.var_dm__blk1424_dn6, locals.var_dm__blk1424_dn7, locals.var_dm__blk1424_dn8, locals.var_dm__blk1424_dn9,)
    }
};
        locals.var_dm__blk1424 = assign52710_e67959;
        locals.var_dm__blk1424_dn4 = assign52710_e67959_d_n4;
        locals.var_dm__blk1424_dn6 = assign52710_e67959_d_n6;
        locals.var_dm__blk1424_dn7 = assign52710_e67959_d_n7;
        locals.var_dm__blk1424_dn8 = assign52710_e67959_d_n8;
        locals.var_dm__blk1424_dn9 = assign52710_e67959_d_n9;
        locals.var_dm__blk1424_rv = 0.0;

        let assign52720_e67962: f64 = if locals.var_x_m__blk1421 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1517 = assign52720_e67962;
        locals.var_guard1517_rv = 0.0;

        let (assign52730_e67986, assign52730_e67986_d_n4, assign52730_e67986_d_n6, assign52730_e67986_d_n7, assign52730_e67986_d_n8, assign52730_e67986_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign52730_e67971: f64 = (locals.var_x_m__blk1421 * locals.var_x_m__blk1421);
        let assign52730_e67978: f64 = (0.25 * locals.var_x_m__blk1421);
        let assign52730_e67979: f64 = (1.0 - assign52730_e67978);
        let assign52730_e67980: f64 = (locals.var_x_m__blk1421 * assign52730_e67979);
        let assign52730_e67981: f64 = (0.3333333333333333 * assign52730_e67980);
        let assign52730_e67982: f64 = (1.0 - assign52730_e67981);
        let assign52730_e67983: f64 = (assign52730_e67971 * assign52730_e67982);
        let assign52730_e67984: f64 = (0.5 * assign52730_e67983);
        (assign52730_e67984, (0.5 * ((((locals.var_x_m__blk1421_dn4 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn4)) * assign52730_e67982) + (assign52730_e67971 * (-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn4 * assign52730_e67979) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn4))))))))), (0.5 * ((((locals.var_x_m__blk1421_dn6 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn6)) * assign52730_e67982) + (assign52730_e67971 * (-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn6 * assign52730_e67979) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn6))))))))), (0.5 * ((((locals.var_x_m__blk1421_dn7 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn7)) * assign52730_e67982) + (assign52730_e67971 * (-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn7 * assign52730_e67979) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn7))))))))), (0.5 * ((((locals.var_x_m__blk1421_dn8 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn8)) * assign52730_e67982) + (assign52730_e67971 * (-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn8 * assign52730_e67979) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn8))))))))), (0.5 * ((((locals.var_x_m__blk1421_dn9 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn9)) * assign52730_e67982) + (assign52730_e67971 * (-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn9 * assign52730_e67979) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn9))))))))),)
    } else {
        (locals.var_pm__blk1425, locals.var_pm__blk1425_dn4, locals.var_pm__blk1425_dn6, locals.var_pm__blk1425_dn7, locals.var_pm__blk1425_dn8, locals.var_pm__blk1425_dn9,)
    }
};
        locals.var_pm__blk1425 = assign52730_e67986;
        locals.var_pm__blk1425_dn4 = assign52730_e67986_d_n4;
        locals.var_pm__blk1425_dn6 = assign52730_e67986_d_n6;
        locals.var_pm__blk1425_dn7 = assign52730_e67986_d_n7;
        locals.var_pm__blk1425_dn8 = assign52730_e67986_d_n8;
        locals.var_pm__blk1425_dn9 = assign52730_e67986_d_n9;
        locals.var_pm__blk1425_rv = 0.0;

        let (assign52740_e67999, assign52740_e67999_d_n4, assign52740_e67999_d_n6, assign52740_e67999_d_n7, assign52740_e67999_d_n8, assign52740_e67999_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign52740_e67995: f64 = (locals.var_dm__blk1424 + locals.var_pm__blk1425);
        let assign52740_e67996: f64 = (assign52740_e67995).sqrt();
        let assign52740_e67997: f64 = (locals.var_gf__blk1324 * assign52740_e67996);
        (assign52740_e67997, ((locals.var_gf__blk1324_dn4 * assign52740_e67996) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn4 + locals.var_pm__blk1425_dn4) / (2.0 * assign52740_e67996)))), ((locals.var_gf__blk1324_dn6 * assign52740_e67996) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn6 + locals.var_pm__blk1425_dn6) / (2.0 * assign52740_e67996)))), ((locals.var_gf__blk1324_dn7 * assign52740_e67996) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn7 + locals.var_pm__blk1425_dn7) / (2.0 * assign52740_e67996)))), ((locals.var_gf__blk1324_dn8 * assign52740_e67996) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn8 + locals.var_pm__blk1425_dn8) / (2.0 * assign52740_e67996)))), ((locals.var_gf__blk1324_dn9 * assign52740_e67996) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn9 + locals.var_pm__blk1425_dn9) / (2.0 * assign52740_e67996)))),)
    } else {
        (locals.var_xgm__blk1426, locals.var_xgm__blk1426_dn4, locals.var_xgm__blk1426_dn6, locals.var_xgm__blk1426_dn7, locals.var_xgm__blk1426_dn8, locals.var_xgm__blk1426_dn9,)
    }
};
        locals.var_xgm__blk1426 = assign52740_e67999;
        locals.var_xgm__blk1426_dn4 = assign52740_e67999_d_n4;
        locals.var_xgm__blk1426_dn6 = assign52740_e67999_d_n6;
        locals.var_xgm__blk1426_dn7 = assign52740_e67999_d_n7;
        locals.var_xgm__blk1426_dn8 = assign52740_e67999_d_n8;
        locals.var_xgm__blk1426_dn9 = assign52740_e67999_d_n9;
        locals.var_xgm__blk1426_rv = 0.0;

        let assign52750_e68002: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1518 = assign52750_e68002;
        locals.var_guard1518_rv = 0.0;

        let (assign52760_e68019, assign52760_e68019_d_n4, assign52760_e68019_d_n6, assign52760_e68019_d_n7, assign52760_e68019_d_n8, assign52760_e68019_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) && (locals.var_guard1518 != 0.0)) {
        let assign52760_e68014: f64 = (locals.var_kp * locals.var_xgm__blk1426);
        let assign52760_e68015: f64 = (1.0 + assign52760_e68014);
        let assign52760_e68016: f64 = (assign52760_e68015).sqrt();
        let assign52760_e68017: f64 = (1.0 / assign52760_e68016);
        (assign52760_e68017, (-((((locals.var_kp_dn4 * locals.var_xgm__blk1426) + (locals.var_kp * locals.var_xgm__blk1426_dn4)) / (2.0 * assign52760_e68016)) / (assign52760_e68016 * assign52760_e68016))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn6) / (2.0 * assign52760_e68016)) / (assign52760_e68016 * assign52760_e68016))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn7) / (2.0 * assign52760_e68016)) / (assign52760_e68016 * assign52760_e68016))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn8) / (2.0 * assign52760_e68016)) / (assign52760_e68016 * assign52760_e68016))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn9) / (2.0 * assign52760_e68016)) / (assign52760_e68016 * assign52760_e68016))),)
    } else {
        (locals.var_eta_p__blk1427, locals.var_eta_p__blk1427_dn4, locals.var_eta_p__blk1427_dn6, locals.var_eta_p__blk1427_dn7, locals.var_eta_p__blk1427_dn8, locals.var_eta_p__blk1427_dn9,)
    }
};
        locals.var_eta_p__blk1427 = assign52760_e68019;
        locals.var_eta_p__blk1427_dn4 = assign52760_e68019_d_n4;
        locals.var_eta_p__blk1427_dn6 = assign52760_e68019_d_n6;
        locals.var_eta_p__blk1427_dn7 = assign52760_e68019_d_n7;
        locals.var_eta_p__blk1427_dn8 = assign52760_e68019_d_n8;
        locals.var_eta_p__blk1427_dn9 = assign52760_e68019_d_n9;
        locals.var_eta_p__blk1427_rv = 0.0;

        let (assign52770_e68038, assign52770_e68038_d_n4, assign52770_e68038_d_n6, assign52770_e68038_d_n7, assign52770_e68038_d_n8, assign52770_e68038_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign52770_e68031: f64 = (0.25 * locals.var_x_m__blk1421);
        let assign52770_e68032: f64 = (1.0 - assign52770_e68031);
        let assign52770_e68033: f64 = (locals.var_x_m__blk1421 * assign52770_e68032);
        let assign52770_e68034: f64 = (0.3333333333333333 * assign52770_e68033);
        let assign52770_e68035: f64 = (1.0 - assign52770_e68034);
        let assign52770_e68036: f64 = (assign52770_e68035).sqrt();
        (assign52770_e68036, ((-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn4 * assign52770_e68032) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn4)))))) / (2.0 * assign52770_e68036)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn6 * assign52770_e68032) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn6)))))) / (2.0 * assign52770_e68036)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn7 * assign52770_e68032) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn7)))))) / (2.0 * assign52770_e68036)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn8 * assign52770_e68032) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn8)))))) / (2.0 * assign52770_e68036)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn9 * assign52770_e68032) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn9)))))) / (2.0 * assign52770_e68036)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52770_e68038;
        locals.var_temp__blk949_dn4 = assign52770_e68038_d_n4;
        locals.var_temp__blk949_dn6 = assign52770_e68038_d_n6;
        locals.var_temp__blk949_dn7 = assign52770_e68038_d_n7;
        locals.var_temp__blk949_dn8 = assign52770_e68038_d_n8;
        locals.var_temp__blk949_dn9 = assign52770_e68038_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign52780_e68050, assign52780_e68050_d_n4, assign52780_e68050_d_n6, assign52780_e68050_d_n7, assign52780_e68050_d_n8, assign52780_e68050_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign52780_e68047: f64 = (locals.var_x_m__blk1421 * locals.var_temp__blk949);
        let assign52780_e68048: f64 = (0.7071067811865475 * assign52780_e68047);
        (assign52780_e68048, (0.7071067811865475 * ((locals.var_x_m__blk1421_dn4 * locals.var_temp__blk949) + (locals.var_x_m__blk1421 * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_m__blk1421_dn6 * locals.var_temp__blk949) + (locals.var_x_m__blk1421 * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_m__blk1421_dn7 * locals.var_temp__blk949) + (locals.var_x_m__blk1421 * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_m__blk1421_dn8 * locals.var_temp__blk949) + (locals.var_x_m__blk1421 * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_m__blk1421_dn9 * locals.var_temp__blk949) + (locals.var_x_m__blk1421 * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqm__blk1428, locals.var_sqm__blk1428_dn4, locals.var_sqm__blk1428_dn6, locals.var_sqm__blk1428_dn7, locals.var_sqm__blk1428_dn8, locals.var_sqm__blk1428_dn9,)
    }
};
        locals.var_sqm__blk1428 = assign52780_e68050;
        locals.var_sqm__blk1428_dn4 = assign52780_e68050_d_n4;
        locals.var_sqm__blk1428_dn6 = assign52780_e68050_d_n6;
        locals.var_sqm__blk1428_dn7 = assign52780_e68050_d_n7;
        locals.var_sqm__blk1428_dn8 = assign52780_e68050_d_n8;
        locals.var_sqm__blk1428_dn9 = assign52780_e68050_d_n9;
        locals.var_sqm__blk1428_rv = 0.0;

        let (assign52790_e68076, assign52790_e68076_d_n4, assign52790_e68076_d_n6, assign52790_e68076_d_n7, assign52790_e68076_d_n8, assign52790_e68076_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign52790_e68062: f64 = (0.5 * locals.var_x_m__blk1421);
        let assign52790_e68063: f64 = (1.0 - assign52790_e68062);
        let assign52790_e68067: f64 = (locals.var_x_m__blk1421 * locals.var_x_m__blk1421);
        let assign52790_e68068: f64 = (0.16666666666666666 * assign52790_e68067);
        let assign52790_e68069: f64 = (assign52790_e68063 + assign52790_e68068);
        let assign52790_e68070: f64 = (locals.var_gf__blk1324 * assign52790_e68069);
        let assign52790_e68072: f64 = (assign52790_e68070 / locals.var_temp__blk949);
        let assign52790_e68073: f64 = (0.7071067811865475 * assign52790_e68072);
        let assign52790_e68074: f64 = (locals.var_eta_p__blk1427 + assign52790_e68073);
        (assign52790_e68074, (locals.var_eta_p__blk1427_dn4 + (0.7071067811865475 * (((((locals.var_gf__blk1324_dn4 * assign52790_e68069) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_m__blk1421_dn4)) + (0.16666666666666666 * ((locals.var_x_m__blk1421_dn4 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn4)))))) * locals.var_temp__blk949) - (assign52790_e68070 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p__blk1427_dn6 + (0.7071067811865475 * (((((locals.var_gf__blk1324_dn6 * assign52790_e68069) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_m__blk1421_dn6)) + (0.16666666666666666 * ((locals.var_x_m__blk1421_dn6 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn6)))))) * locals.var_temp__blk949) - (assign52790_e68070 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p__blk1427_dn7 + (0.7071067811865475 * (((((locals.var_gf__blk1324_dn7 * assign52790_e68069) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_m__blk1421_dn7)) + (0.16666666666666666 * ((locals.var_x_m__blk1421_dn7 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn7)))))) * locals.var_temp__blk949) - (assign52790_e68070 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p__blk1427_dn8 + (0.7071067811865475 * (((((locals.var_gf__blk1324_dn8 * assign52790_e68069) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_m__blk1421_dn8)) + (0.16666666666666666 * ((locals.var_x_m__blk1421_dn8 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn8)))))) * locals.var_temp__blk949) - (assign52790_e68070 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p__blk1427_dn9 + (0.7071067811865475 * (((((locals.var_gf__blk1324_dn9 * assign52790_e68069) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_m__blk1421_dn9)) + (0.16666666666666666 * ((locals.var_x_m__blk1421_dn9 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn9)))))) * locals.var_temp__blk949) - (assign52790_e68070 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))),)
    } else {
        (locals.var_alpha__blk1429, locals.var_alpha__blk1429_dn4, locals.var_alpha__blk1429_dn6, locals.var_alpha__blk1429_dn7, locals.var_alpha__blk1429_dn8, locals.var_alpha__blk1429_dn9,)
    }
};
        locals.var_alpha__blk1429 = assign52790_e68076;
        locals.var_alpha__blk1429_dn4 = assign52790_e68076_d_n4;
        locals.var_alpha__blk1429_dn6 = assign52790_e68076_d_n6;
        locals.var_alpha__blk1429_dn7 = assign52790_e68076_d_n7;
        locals.var_alpha__blk1429_dn8 = assign52790_e68076_d_n8;
        locals.var_alpha__blk1429_dn9 = assign52790_e68076_d_n9;
        locals.var_alpha__blk1429_rv = 0.0;

        let (assign52800_e68089, assign52800_e68089_d_n4, assign52800_e68089_d_n6, assign52800_e68089_d_n7, assign52800_e68089_d_n8, assign52800_e68089_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) {
        let assign52800_e68085: f64 = (locals.var_x_m__blk1421 - 1.0);
        let assign52800_e68087: f64 = (assign52800_e68085 + locals.var_em__blk1422);
        (assign52800_e68087, (locals.var_x_m__blk1421_dn4 + locals.var_em__blk1422_dn4), (locals.var_x_m__blk1421_dn6 + locals.var_em__blk1422_dn6), (locals.var_x_m__blk1421_dn7 + locals.var_em__blk1422_dn7), (locals.var_x_m__blk1421_dn8 + locals.var_em__blk1422_dn8), (locals.var_x_m__blk1421_dn9 + locals.var_em__blk1422_dn9),)
    } else {
        (locals.var_pm__blk1425, locals.var_pm__blk1425_dn4, locals.var_pm__blk1425_dn6, locals.var_pm__blk1425_dn7, locals.var_pm__blk1425_dn8, locals.var_pm__blk1425_dn9,)
    }
};
        locals.var_pm__blk1425 = assign52800_e68089;
        locals.var_pm__blk1425_dn4 = assign52800_e68089_d_n4;
        locals.var_pm__blk1425_dn6 = assign52800_e68089_d_n6;
        locals.var_pm__blk1425_dn7 = assign52800_e68089_d_n7;
        locals.var_pm__blk1425_dn8 = assign52800_e68089_d_n8;
        locals.var_pm__blk1425_dn9 = assign52800_e68089_d_n9;
        locals.var_pm__blk1425_rv = 0.0;

        let (assign52810_e68103, assign52810_e68103_d_n4, assign52810_e68103_d_n6, assign52810_e68103_d_n7, assign52810_e68103_d_n8, assign52810_e68103_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) {
        let assign52810_e68099: f64 = (locals.var_dm__blk1424 + locals.var_pm__blk1425);
        let assign52810_e68100: f64 = (assign52810_e68099).sqrt();
        let assign52810_e68101: f64 = (locals.var_gf__blk1324 * assign52810_e68100);
        (assign52810_e68101, ((locals.var_gf__blk1324_dn4 * assign52810_e68100) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn4 + locals.var_pm__blk1425_dn4) / (2.0 * assign52810_e68100)))), ((locals.var_gf__blk1324_dn6 * assign52810_e68100) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn6 + locals.var_pm__blk1425_dn6) / (2.0 * assign52810_e68100)))), ((locals.var_gf__blk1324_dn7 * assign52810_e68100) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn7 + locals.var_pm__blk1425_dn7) / (2.0 * assign52810_e68100)))), ((locals.var_gf__blk1324_dn8 * assign52810_e68100) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn8 + locals.var_pm__blk1425_dn8) / (2.0 * assign52810_e68100)))), ((locals.var_gf__blk1324_dn9 * assign52810_e68100) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn9 + locals.var_pm__blk1425_dn9) / (2.0 * assign52810_e68100)))),)
    } else {
        (locals.var_xgm__blk1426, locals.var_xgm__blk1426_dn4, locals.var_xgm__blk1426_dn6, locals.var_xgm__blk1426_dn7, locals.var_xgm__blk1426_dn8, locals.var_xgm__blk1426_dn9,)
    }
};
        locals.var_xgm__blk1426 = assign52810_e68103;
        locals.var_xgm__blk1426_dn4 = assign52810_e68103_d_n4;
        locals.var_xgm__blk1426_dn6 = assign52810_e68103_d_n6;
        locals.var_xgm__blk1426_dn7 = assign52810_e68103_d_n7;
        locals.var_xgm__blk1426_dn8 = assign52810_e68103_d_n8;
        locals.var_xgm__blk1426_dn9 = assign52810_e68103_d_n9;
        locals.var_xgm__blk1426_rv = 0.0;

        let assign52820_e68106: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1519 = assign52820_e68106;
        locals.var_guard1519_rv = 0.0;

        let (assign52830_e68125, assign52830_e68125_d_n4, assign52830_e68125_d_n6, assign52830_e68125_d_n7, assign52830_e68125_d_n8, assign52830_e68125_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52830_e68117: f64 = (1.0 - locals.var_em__blk1422);
        let assign52830_e68121: f64 = (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341);
        let assign52830_e68122: f64 = (2.0 * assign52830_e68121);
        let assign52830_e68123: f64 = (assign52830_e68117 + assign52830_e68122);
        (assign52830_e68123, ((-locals.var_em__blk1422_dn4) + (2.0 * ((locals.var_xgm__blk1426_dn4 * locals.var_inv_gf2__blk1341) + (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341_dn4)))), ((-locals.var_em__blk1422_dn6) + (2.0 * ((locals.var_xgm__blk1426_dn6 * locals.var_inv_gf2__blk1341) + (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341_dn6)))), ((-locals.var_em__blk1422_dn7) + (2.0 * ((locals.var_xgm__blk1426_dn7 * locals.var_inv_gf2__blk1341) + (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341_dn7)))), ((-locals.var_em__blk1422_dn8) + (2.0 * ((locals.var_xgm__blk1426_dn8 * locals.var_inv_gf2__blk1341) + (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341_dn8)))), ((-locals.var_em__blk1422_dn9) + (2.0 * ((locals.var_xgm__blk1426_dn9 * locals.var_inv_gf2__blk1341) + (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341_dn9)))),)
    } else {
        (locals.var_d0__blk1430, locals.var_d0__blk1430_dn4, locals.var_d0__blk1430_dn6, locals.var_d0__blk1430_dn7, locals.var_d0__blk1430_dn8, locals.var_d0__blk1430_dn9,)
    }
};
        locals.var_d0__blk1430 = assign52830_e68125;
        locals.var_d0__blk1430_dn4 = assign52830_e68125_d_n4;
        locals.var_d0__blk1430_dn6 = assign52830_e68125_d_n6;
        locals.var_d0__blk1430_dn7 = assign52830_e68125_d_n7;
        locals.var_d0__blk1430_dn8 = assign52830_e68125_d_n8;
        locals.var_d0__blk1430_dn9 = assign52830_e68125_d_n9;
        locals.var_d0__blk1430_rv = 0.0;

        let (assign52840_e68143, assign52840_e68143_d_n4, assign52840_e68143_d_n6, assign52840_e68143_d_n7, assign52840_e68143_d_n8, assign52840_e68143_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52840_e68138: f64 = (locals.var_kp * locals.var_xgm__blk1426);
        let assign52840_e68139: f64 = (1.0 + assign52840_e68138);
        let assign52840_e68140: f64 = (assign52840_e68139).sqrt();
        let assign52840_e68141: f64 = (1.0 / assign52840_e68140);
        (assign52840_e68141, (-((((locals.var_kp_dn4 * locals.var_xgm__blk1426) + (locals.var_kp * locals.var_xgm__blk1426_dn4)) / (2.0 * assign52840_e68140)) / (assign52840_e68140 * assign52840_e68140))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn6) / (2.0 * assign52840_e68140)) / (assign52840_e68140 * assign52840_e68140))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn7) / (2.0 * assign52840_e68140)) / (assign52840_e68140 * assign52840_e68140))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn8) / (2.0 * assign52840_e68140)) / (assign52840_e68140 * assign52840_e68140))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn9) / (2.0 * assign52840_e68140)) / (assign52840_e68140 * assign52840_e68140))),)
    } else {
        (locals.var_eta_p__blk1427, locals.var_eta_p__blk1427_dn4, locals.var_eta_p__blk1427_dn6, locals.var_eta_p__blk1427_dn7, locals.var_eta_p__blk1427_dn8, locals.var_eta_p__blk1427_dn9,)
    }
};
        locals.var_eta_p__blk1427 = assign52840_e68143;
        locals.var_eta_p__blk1427_dn4 = assign52840_e68143_d_n4;
        locals.var_eta_p__blk1427_dn6 = assign52840_e68143_d_n6;
        locals.var_eta_p__blk1427_dn7 = assign52840_e68143_d_n7;
        locals.var_eta_p__blk1427_dn8 = assign52840_e68143_d_n8;
        locals.var_eta_p__blk1427_dn9 = assign52840_e68143_d_n9;
        locals.var_eta_p__blk1427_rv = 0.0;

        let (assign52850_e68158, assign52850_e68158_d_n4, assign52850_e68158_d_n6, assign52850_e68158_d_n7, assign52850_e68158_d_n8, assign52850_e68158_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52850_e68155: f64 = (locals.var_eta_p__blk1427 + 1.0);
        let assign52850_e68156: f64 = (locals.var_eta_p__blk1427 / assign52850_e68155);
        (assign52850_e68156, (((locals.var_eta_p__blk1427_dn4 * assign52850_e68155) - (locals.var_eta_p__blk1427 * locals.var_eta_p__blk1427_dn4)) / (assign52850_e68155 * assign52850_e68155)), (((locals.var_eta_p__blk1427_dn6 * assign52850_e68155) - (locals.var_eta_p__blk1427 * locals.var_eta_p__blk1427_dn6)) / (assign52850_e68155 * assign52850_e68155)), (((locals.var_eta_p__blk1427_dn7 * assign52850_e68155) - (locals.var_eta_p__blk1427 * locals.var_eta_p__blk1427_dn7)) / (assign52850_e68155 * assign52850_e68155)), (((locals.var_eta_p__blk1427_dn8 * assign52850_e68155) - (locals.var_eta_p__blk1427 * locals.var_eta_p__blk1427_dn8)) / (assign52850_e68155 * assign52850_e68155)), (((locals.var_eta_p__blk1427_dn9 * assign52850_e68155) - (locals.var_eta_p__blk1427 * locals.var_eta_p__blk1427_dn9)) / (assign52850_e68155 * assign52850_e68155)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52850_e68158;
        locals.var_temp__blk949_dn4 = assign52850_e68158_d_n4;
        locals.var_temp__blk949_dn6 = assign52850_e68158_d_n6;
        locals.var_temp__blk949_dn7 = assign52850_e68158_d_n7;
        locals.var_temp__blk949_dn8 = assign52850_e68158_d_n8;
        locals.var_temp__blk949_dn9 = assign52850_e68158_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign52860_e68177, assign52860_e68177_d_n4, assign52860_e68177_d_n6, assign52860_e68177_d_n7, assign52860_e68177_d_n8, assign52860_e68177_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52860_e68170: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
        let assign52860_e68172: f64 = (assign52860_e68170 * locals.var_gf2__blk1325);
        let assign52860_e68174: f64 = (assign52860_e68172 * locals.var_dm__blk1424);
        let assign52860_e68175: f64 = (locals.var_kp * assign52860_e68174);
        (assign52860_e68175, ((locals.var_kp_dn4 * assign52860_e68174) + (locals.var_kp * ((((((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) * locals.var_gf2__blk1325) + (assign52860_e68170 * locals.var_gf2__blk1325_dn4)) * locals.var_dm__blk1424) + (assign52860_e68172 * locals.var_dm__blk1424_dn4)))), (locals.var_kp * ((((((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) * locals.var_gf2__blk1325) + (assign52860_e68170 * locals.var_gf2__blk1325_dn6)) * locals.var_dm__blk1424) + (assign52860_e68172 * locals.var_dm__blk1424_dn6))), (locals.var_kp * ((((((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) * locals.var_gf2__blk1325) + (assign52860_e68170 * locals.var_gf2__blk1325_dn7)) * locals.var_dm__blk1424) + (assign52860_e68172 * locals.var_dm__blk1424_dn7))), (locals.var_kp * ((((((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) * locals.var_gf2__blk1325) + (assign52860_e68170 * locals.var_gf2__blk1325_dn8)) * locals.var_dm__blk1424) + (assign52860_e68172 * locals.var_dm__blk1424_dn8))), (locals.var_kp * ((((((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) * locals.var_gf2__blk1325) + (assign52860_e68170 * locals.var_gf2__blk1325_dn9)) * locals.var_dm__blk1424) + (assign52860_e68172 * locals.var_dm__blk1424_dn9))),)
    } else {
        (locals.var_x_pm__blk1431, locals.var_x_pm__blk1431_dn4, locals.var_x_pm__blk1431_dn6, locals.var_x_pm__blk1431_dn7, locals.var_x_pm__blk1431_dn8, locals.var_x_pm__blk1431_dn9,)
    }
};
        locals.var_x_pm__blk1431 = assign52860_e68177;
        locals.var_x_pm__blk1431_dn4 = assign52860_e68177_d_n4;
        locals.var_x_pm__blk1431_dn6 = assign52860_e68177_d_n6;
        locals.var_x_pm__blk1431_dn7 = assign52860_e68177_d_n7;
        locals.var_x_pm__blk1431_dn8 = assign52860_e68177_d_n8;
        locals.var_x_pm__blk1431_dn9 = assign52860_e68177_d_n9;
        locals.var_x_pm__blk1431_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        locals: &mut StampLocals,
    ) {
        let (assign52870_e68200, assign52870_e68200_d_n4, assign52870_e68200_d_n6, assign52870_e68200_d_n7, assign52870_e68200_d_n8, assign52870_e68200_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52870_e68189: f64 = (locals.var_xgm__blk1426 - locals.var_x_pm__blk1431);
        let assign52870_e68190: f64 = (2.0 * assign52870_e68189);
        let assign52870_e68194: f64 = (1.0 - locals.var_em__blk1422);
        let assign52870_e68196: f64 = (assign52870_e68194 + locals.var_dm__blk1424);
        let assign52870_e68197: f64 = (locals.var_gf2__blk1325 * assign52870_e68196);
        let assign52870_e68198: f64 = (assign52870_e68190 + assign52870_e68197);
        (assign52870_e68198, ((2.0 * (locals.var_xgm__blk1426_dn4 - locals.var_x_pm__blk1431_dn4)) + ((locals.var_gf2__blk1325_dn4 * assign52870_e68196) + (locals.var_gf2__blk1325 * ((-locals.var_em__blk1422_dn4) + locals.var_dm__blk1424_dn4)))), ((2.0 * (locals.var_xgm__blk1426_dn6 - locals.var_x_pm__blk1431_dn6)) + ((locals.var_gf2__blk1325_dn6 * assign52870_e68196) + (locals.var_gf2__blk1325 * ((-locals.var_em__blk1422_dn6) + locals.var_dm__blk1424_dn6)))), ((2.0 * (locals.var_xgm__blk1426_dn7 - locals.var_x_pm__blk1431_dn7)) + ((locals.var_gf2__blk1325_dn7 * assign52870_e68196) + (locals.var_gf2__blk1325 * ((-locals.var_em__blk1422_dn7) + locals.var_dm__blk1424_dn7)))), ((2.0 * (locals.var_xgm__blk1426_dn8 - locals.var_x_pm__blk1431_dn8)) + ((locals.var_gf2__blk1325_dn8 * assign52870_e68196) + (locals.var_gf2__blk1325 * ((-locals.var_em__blk1422_dn8) + locals.var_dm__blk1424_dn8)))), ((2.0 * (locals.var_xgm__blk1426_dn9 - locals.var_x_pm__blk1431_dn9)) + ((locals.var_gf2__blk1325_dn9 * assign52870_e68196) + (locals.var_gf2__blk1325 * ((-locals.var_em__blk1422_dn9) + locals.var_dm__blk1424_dn9)))),)
    } else {
        (locals.var_p_pd__blk1432, locals.var_p_pd__blk1432_dn4, locals.var_p_pd__blk1432_dn6, locals.var_p_pd__blk1432_dn7, locals.var_p_pd__blk1432_dn8, locals.var_p_pd__blk1432_dn9,)
    }
};
        locals.var_p_pd__blk1432 = assign52870_e68200;
        locals.var_p_pd__blk1432_dn4 = assign52870_e68200_d_n4;
        locals.var_p_pd__blk1432_dn6 = assign52870_e68200_d_n6;
        locals.var_p_pd__blk1432_dn7 = assign52870_e68200_d_n7;
        locals.var_p_pd__blk1432_dn8 = assign52870_e68200_d_n8;
        locals.var_p_pd__blk1432_dn9 = assign52870_e68200_d_n9;
        locals.var_p_pd__blk1432_rv = 0.0;

        let (assign52880_e68217, assign52880_e68217_d_n4, assign52880_e68217_d_n6, assign52880_e68217_d_n7, assign52880_e68217_d_n8, assign52880_e68217_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52880_e68213: f64 = (2.0 * locals.var_xgm__blk1426);
        let assign52880_e68214: f64 = (locals.var_x_pm__blk1431 - assign52880_e68213);
        let assign52880_e68215: f64 = (locals.var_x_pm__blk1431 * assign52880_e68214);
        (assign52880_e68215, ((locals.var_x_pm__blk1431_dn4 * assign52880_e68214) + (locals.var_x_pm__blk1431 * (locals.var_x_pm__blk1431_dn4 - (2.0 * locals.var_xgm__blk1426_dn4)))), ((locals.var_x_pm__blk1431_dn6 * assign52880_e68214) + (locals.var_x_pm__blk1431 * (locals.var_x_pm__blk1431_dn6 - (2.0 * locals.var_xgm__blk1426_dn6)))), ((locals.var_x_pm__blk1431_dn7 * assign52880_e68214) + (locals.var_x_pm__blk1431 * (locals.var_x_pm__blk1431_dn7 - (2.0 * locals.var_xgm__blk1426_dn7)))), ((locals.var_x_pm__blk1431_dn8 * assign52880_e68214) + (locals.var_x_pm__blk1431 * (locals.var_x_pm__blk1431_dn8 - (2.0 * locals.var_xgm__blk1426_dn8)))), ((locals.var_x_pm__blk1431_dn9 * assign52880_e68214) + (locals.var_x_pm__blk1431 * (locals.var_x_pm__blk1431_dn9 - (2.0 * locals.var_xgm__blk1426_dn9)))),)
    } else {
        (locals.var_q_pd__blk1433, locals.var_q_pd__blk1433_dn4, locals.var_q_pd__blk1433_dn6, locals.var_q_pd__blk1433_dn7, locals.var_q_pd__blk1433_dn8, locals.var_q_pd__blk1433_dn9,)
    }
};
        locals.var_q_pd__blk1433 = assign52880_e68217;
        locals.var_q_pd__blk1433_dn4 = assign52880_e68217_d_n4;
        locals.var_q_pd__blk1433_dn6 = assign52880_e68217_d_n6;
        locals.var_q_pd__blk1433_dn7 = assign52880_e68217_d_n7;
        locals.var_q_pd__blk1433_dn8 = assign52880_e68217_d_n8;
        locals.var_q_pd__blk1433_dn9 = assign52880_e68217_d_n9;
        locals.var_q_pd__blk1433_rv = 0.0;

        let (assign52890_e68236, assign52890_e68236_d_n4, assign52890_e68236_d_n6, assign52890_e68236_d_n7, assign52890_e68236_d_n8, assign52890_e68236_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52890_e68231: f64 = (locals.var_em__blk1422 + locals.var_dm__blk1424);
        let assign52890_e68232: f64 = (locals.var_gf2__blk1325 * assign52890_e68231);
        let assign52890_e68233: f64 = (0.5 * assign52890_e68232);
        let assign52890_e68234: f64 = (1.0 - assign52890_e68233);
        (assign52890_e68234, (-(0.5 * ((locals.var_gf2__blk1325_dn4 * assign52890_e68231) + (locals.var_gf2__blk1325 * (locals.var_em__blk1422_dn4 + locals.var_dm__blk1424_dn4))))), (-(0.5 * ((locals.var_gf2__blk1325_dn6 * assign52890_e68231) + (locals.var_gf2__blk1325 * (locals.var_em__blk1422_dn6 + locals.var_dm__blk1424_dn6))))), (-(0.5 * ((locals.var_gf2__blk1325_dn7 * assign52890_e68231) + (locals.var_gf2__blk1325 * (locals.var_em__blk1422_dn7 + locals.var_dm__blk1424_dn7))))), (-(0.5 * ((locals.var_gf2__blk1325_dn8 * assign52890_e68231) + (locals.var_gf2__blk1325 * (locals.var_em__blk1422_dn8 + locals.var_dm__blk1424_dn8))))), (-(0.5 * ((locals.var_gf2__blk1325_dn9 * assign52890_e68231) + (locals.var_gf2__blk1325 * (locals.var_em__blk1422_dn9 + locals.var_dm__blk1424_dn9))))),)
    } else {
        (locals.var_xi_pd__blk1434, locals.var_xi_pd__blk1434_dn4, locals.var_xi_pd__blk1434_dn6, locals.var_xi_pd__blk1434_dn7, locals.var_xi_pd__blk1434_dn8, locals.var_xi_pd__blk1434_dn9,)
    }
};
        locals.var_xi_pd__blk1434 = assign52890_e68236;
        locals.var_xi_pd__blk1434_dn4 = assign52890_e68236_d_n4;
        locals.var_xi_pd__blk1434_dn6 = assign52890_e68236_d_n6;
        locals.var_xi_pd__blk1434_dn7 = assign52890_e68236_d_n7;
        locals.var_xi_pd__blk1434_dn8 = assign52890_e68236_d_n8;
        locals.var_xi_pd__blk1434_dn9 = assign52890_e68236_d_n9;
        locals.var_xi_pd__blk1434_rv = 0.0;

        let (assign52900_e68257, assign52900_e68257_d_n4, assign52900_e68257_d_n6, assign52900_e68257_d_n7, assign52900_e68257_d_n8, assign52900_e68257_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52900_e68247: f64 = (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432);
        let assign52900_e68250: f64 = (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432);
        let assign52900_e68253: f64 = (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433);
        let assign52900_e68254: f64 = (assign52900_e68250 - assign52900_e68253);
        let assign52900_e68255: f64 = (assign52900_e68247 / assign52900_e68254);
        (assign52900_e68255, (((((locals.var_q_pd__blk1433_dn4 * locals.var_p_pd__blk1432) + (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432_dn4)) * assign52900_e68254) - (assign52900_e68247 * (((locals.var_p_pd__blk1432_dn4 * locals.var_p_pd__blk1432) + (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432_dn4)) - ((locals.var_xi_pd__blk1434_dn4 * locals.var_q_pd__blk1433) + (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433_dn4))))) / (assign52900_e68254 * assign52900_e68254)), (((((locals.var_q_pd__blk1433_dn6 * locals.var_p_pd__blk1432) + (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432_dn6)) * assign52900_e68254) - (assign52900_e68247 * (((locals.var_p_pd__blk1432_dn6 * locals.var_p_pd__blk1432) + (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432_dn6)) - ((locals.var_xi_pd__blk1434_dn6 * locals.var_q_pd__blk1433) + (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433_dn6))))) / (assign52900_e68254 * assign52900_e68254)), (((((locals.var_q_pd__blk1433_dn7 * locals.var_p_pd__blk1432) + (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432_dn7)) * assign52900_e68254) - (assign52900_e68247 * (((locals.var_p_pd__blk1432_dn7 * locals.var_p_pd__blk1432) + (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432_dn7)) - ((locals.var_xi_pd__blk1434_dn7 * locals.var_q_pd__blk1433) + (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433_dn7))))) / (assign52900_e68254 * assign52900_e68254)), (((((locals.var_q_pd__blk1433_dn8 * locals.var_p_pd__blk1432) + (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432_dn8)) * assign52900_e68254) - (assign52900_e68247 * (((locals.var_p_pd__blk1432_dn8 * locals.var_p_pd__blk1432) + (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432_dn8)) - ((locals.var_xi_pd__blk1434_dn8 * locals.var_q_pd__blk1433) + (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433_dn8))))) / (assign52900_e68254 * assign52900_e68254)), (((((locals.var_q_pd__blk1433_dn9 * locals.var_p_pd__blk1432) + (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432_dn9)) * assign52900_e68254) - (assign52900_e68247 * (((locals.var_p_pd__blk1432_dn9 * locals.var_p_pd__blk1432) + (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432_dn9)) - ((locals.var_xi_pd__blk1434_dn9 * locals.var_q_pd__blk1433) + (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433_dn9))))) / (assign52900_e68254 * assign52900_e68254)),)
    } else {
        (locals.var_u_pd__blk1435, locals.var_u_pd__blk1435_dn4, locals.var_u_pd__blk1435_dn6, locals.var_u_pd__blk1435_dn7, locals.var_u_pd__blk1435_dn8, locals.var_u_pd__blk1435_dn9,)
    }
};
        locals.var_u_pd__blk1435 = assign52900_e68257;
        locals.var_u_pd__blk1435_dn4 = assign52900_e68257_d_n4;
        locals.var_u_pd__blk1435_dn6 = assign52900_e68257_d_n6;
        locals.var_u_pd__blk1435_dn7 = assign52900_e68257_d_n7;
        locals.var_u_pd__blk1435_dn8 = assign52900_e68257_d_n8;
        locals.var_u_pd__blk1435_dn9 = assign52900_e68257_d_n9;
        locals.var_u_pd__blk1435_rv = 0.0;

        let (assign52910_e68270, assign52910_e68270_d_n4, assign52910_e68270_d_n6, assign52910_e68270_d_n7, assign52910_e68270_d_n8, assign52910_e68270_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52910_e68268: f64 = (locals.var_x_m__blk1421 + locals.var_u_pd__blk1435);
        (assign52910_e68268, (locals.var_x_m__blk1421_dn4 + locals.var_u_pd__blk1435_dn4), (locals.var_x_m__blk1421_dn6 + locals.var_u_pd__blk1435_dn6), (locals.var_x_m__blk1421_dn7 + locals.var_u_pd__blk1435_dn7), (locals.var_x_m__blk1421_dn8 + locals.var_u_pd__blk1435_dn8), (locals.var_x_m__blk1421_dn9 + locals.var_u_pd__blk1435_dn9),)
    } else {
        (locals.var_x_m__blk1421, locals.var_x_m__blk1421_dn4, locals.var_x_m__blk1421_dn6, locals.var_x_m__blk1421_dn7, locals.var_x_m__blk1421_dn8, locals.var_x_m__blk1421_dn9,)
    }
};
        locals.var_x_m__blk1421 = assign52910_e68270;
        locals.var_x_m__blk1421_dn4 = assign52910_e68270_d_n4;
        locals.var_x_m__blk1421_dn6 = assign52910_e68270_d_n6;
        locals.var_x_m__blk1421_dn7 = assign52910_e68270_d_n7;
        locals.var_x_m__blk1421_dn8 = assign52910_e68270_d_n8;
        locals.var_x_m__blk1421_dn9 = assign52910_e68270_d_n9;
        locals.var_x_m__blk1421_rv = 0.0;

        let (assign52920_e68282, assign52920_e68282_d_n4, assign52920_e68282_d_n6, assign52920_e68282_d_n7, assign52920_e68282_d_n8, assign52920_e68282_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52920_e68280: f64 = (locals.var_u_pd__blk1435).exp();
        (assign52920_e68280, (assign52920_e68280 * locals.var_u_pd__blk1435_dn4), (assign52920_e68280 * locals.var_u_pd__blk1435_dn6), (assign52920_e68280 * locals.var_u_pd__blk1435_dn7), (assign52920_e68280 * locals.var_u_pd__blk1435_dn8), (assign52920_e68280 * locals.var_u_pd__blk1435_dn9),)
    } else {
        (locals.var_km__blk1436, locals.var_km__blk1436_dn4, locals.var_km__blk1436_dn6, locals.var_km__blk1436_dn7, locals.var_km__blk1436_dn8, locals.var_km__blk1436_dn9,)
    }
};
        locals.var_km__blk1436 = assign52920_e68282;
        locals.var_km__blk1436_dn4 = assign52920_e68282_d_n4;
        locals.var_km__blk1436_dn6 = assign52920_e68282_d_n6;
        locals.var_km__blk1436_dn7 = assign52920_e68282_d_n7;
        locals.var_km__blk1436_dn8 = assign52920_e68282_d_n8;
        locals.var_km__blk1436_dn9 = assign52920_e68282_d_n9;
        locals.var_km__blk1436_rv = 0.0;

        let (assign52930_e68295, assign52930_e68295_d_n4, assign52930_e68295_d_n6, assign52930_e68295_d_n7, assign52930_e68295_d_n8, assign52930_e68295_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52930_e68293: f64 = (locals.var_em__blk1422 / locals.var_km__blk1436);
        (assign52930_e68293, (((locals.var_em__blk1422_dn4 * locals.var_km__blk1436) - (locals.var_em__blk1422 * locals.var_km__blk1436_dn4)) / (locals.var_km__blk1436 * locals.var_km__blk1436)), (((locals.var_em__blk1422_dn6 * locals.var_km__blk1436) - (locals.var_em__blk1422 * locals.var_km__blk1436_dn6)) / (locals.var_km__blk1436 * locals.var_km__blk1436)), (((locals.var_em__blk1422_dn7 * locals.var_km__blk1436) - (locals.var_em__blk1422 * locals.var_km__blk1436_dn7)) / (locals.var_km__blk1436 * locals.var_km__blk1436)), (((locals.var_em__blk1422_dn8 * locals.var_km__blk1436) - (locals.var_em__blk1422 * locals.var_km__blk1436_dn8)) / (locals.var_km__blk1436 * locals.var_km__blk1436)), (((locals.var_em__blk1422_dn9 * locals.var_km__blk1436) - (locals.var_em__blk1422 * locals.var_km__blk1436_dn9)) / (locals.var_km__blk1436 * locals.var_km__blk1436)),)
    } else {
        (locals.var_em__blk1422, locals.var_em__blk1422_dn4, locals.var_em__blk1422_dn6, locals.var_em__blk1422_dn7, locals.var_em__blk1422_dn8, locals.var_em__blk1422_dn9,)
    }
};
        locals.var_em__blk1422 = assign52930_e68295;
        locals.var_em__blk1422_dn4 = assign52930_e68295_d_n4;
        locals.var_em__blk1422_dn6 = assign52930_e68295_d_n6;
        locals.var_em__blk1422_dn7 = assign52930_e68295_d_n7;
        locals.var_em__blk1422_dn8 = assign52930_e68295_d_n8;
        locals.var_em__blk1422_dn9 = assign52930_e68295_d_n9;
        locals.var_em__blk1422_rv = 0.0;

        let (assign52940_e68308, assign52940_e68308_d_n4, assign52940_e68308_d_n6, assign52940_e68308_d_n7, assign52940_e68308_d_n8, assign52940_e68308_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52940_e68306: f64 = (locals.var_dm__blk1424 * locals.var_km__blk1436);
        (assign52940_e68306, ((locals.var_dm__blk1424_dn4 * locals.var_km__blk1436) + (locals.var_dm__blk1424 * locals.var_km__blk1436_dn4)), ((locals.var_dm__blk1424_dn6 * locals.var_km__blk1436) + (locals.var_dm__blk1424 * locals.var_km__blk1436_dn6)), ((locals.var_dm__blk1424_dn7 * locals.var_km__blk1436) + (locals.var_dm__blk1424 * locals.var_km__blk1436_dn7)), ((locals.var_dm__blk1424_dn8 * locals.var_km__blk1436) + (locals.var_dm__blk1424 * locals.var_km__blk1436_dn8)), ((locals.var_dm__blk1424_dn9 * locals.var_km__blk1436) + (locals.var_dm__blk1424 * locals.var_km__blk1436_dn9)),)
    } else {
        (locals.var_dm__blk1424, locals.var_dm__blk1424_dn4, locals.var_dm__blk1424_dn6, locals.var_dm__blk1424_dn7, locals.var_dm__blk1424_dn8, locals.var_dm__blk1424_dn9,)
    }
};
        locals.var_dm__blk1424 = assign52940_e68308;
        locals.var_dm__blk1424_dn4 = assign52940_e68308_d_n4;
        locals.var_dm__blk1424_dn6 = assign52940_e68308_d_n6;
        locals.var_dm__blk1424_dn7 = assign52940_e68308_d_n7;
        locals.var_dm__blk1424_dn8 = assign52940_e68308_d_n8;
        locals.var_dm__blk1424_dn9 = assign52940_e68308_d_n9;
        locals.var_dm__blk1424_rv = 0.0;

        let (assign52950_e68323, assign52950_e68323_d_n4, assign52950_e68323_d_n6, assign52950_e68323_d_n7, assign52950_e68323_d_n8, assign52950_e68323_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52950_e68319: f64 = (locals.var_x_m__blk1421 - 1.0);
        let assign52950_e68321: f64 = (assign52950_e68319 + locals.var_em__blk1422);
        (assign52950_e68321, (locals.var_x_m__blk1421_dn4 + locals.var_em__blk1422_dn4), (locals.var_x_m__blk1421_dn6 + locals.var_em__blk1422_dn6), (locals.var_x_m__blk1421_dn7 + locals.var_em__blk1422_dn7), (locals.var_x_m__blk1421_dn8 + locals.var_em__blk1422_dn8), (locals.var_x_m__blk1421_dn9 + locals.var_em__blk1422_dn9),)
    } else {
        (locals.var_pm__blk1425, locals.var_pm__blk1425_dn4, locals.var_pm__blk1425_dn6, locals.var_pm__blk1425_dn7, locals.var_pm__blk1425_dn8, locals.var_pm__blk1425_dn9,)
    }
};
        locals.var_pm__blk1425 = assign52950_e68323;
        locals.var_pm__blk1425_dn4 = assign52950_e68323_d_n4;
        locals.var_pm__blk1425_dn6 = assign52950_e68323_d_n6;
        locals.var_pm__blk1425_dn7 = assign52950_e68323_d_n7;
        locals.var_pm__blk1425_dn8 = assign52950_e68323_d_n8;
        locals.var_pm__blk1425_dn9 = assign52950_e68323_d_n9;
        locals.var_pm__blk1425_rv = 0.0;

        let (assign52960_e68339, assign52960_e68339_d_n4, assign52960_e68339_d_n6, assign52960_e68339_d_n7, assign52960_e68339_d_n8, assign52960_e68339_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52960_e68335: f64 = (locals.var_dm__blk1424 + locals.var_pm__blk1425);
        let assign52960_e68336: f64 = (assign52960_e68335).sqrt();
        let assign52960_e68337: f64 = (locals.var_gf__blk1324 * assign52960_e68336);
        (assign52960_e68337, ((locals.var_gf__blk1324_dn4 * assign52960_e68336) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn4 + locals.var_pm__blk1425_dn4) / (2.0 * assign52960_e68336)))), ((locals.var_gf__blk1324_dn6 * assign52960_e68336) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn6 + locals.var_pm__blk1425_dn6) / (2.0 * assign52960_e68336)))), ((locals.var_gf__blk1324_dn7 * assign52960_e68336) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn7 + locals.var_pm__blk1425_dn7) / (2.0 * assign52960_e68336)))), ((locals.var_gf__blk1324_dn8 * assign52960_e68336) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn8 + locals.var_pm__blk1425_dn8) / (2.0 * assign52960_e68336)))), ((locals.var_gf__blk1324_dn9 * assign52960_e68336) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn9 + locals.var_pm__blk1425_dn9) / (2.0 * assign52960_e68336)))),)
    } else {
        (locals.var_xgm__blk1426, locals.var_xgm__blk1426_dn4, locals.var_xgm__blk1426_dn6, locals.var_xgm__blk1426_dn7, locals.var_xgm__blk1426_dn8, locals.var_xgm__blk1426_dn9,)
    }
};
        locals.var_xgm__blk1426 = assign52960_e68339;
        locals.var_xgm__blk1426_dn4 = assign52960_e68339_d_n4;
        locals.var_xgm__blk1426_dn6 = assign52960_e68339_d_n6;
        locals.var_xgm__blk1426_dn7 = assign52960_e68339_d_n7;
        locals.var_xgm__blk1426_dn8 = assign52960_e68339_d_n8;
        locals.var_xgm__blk1426_dn9 = assign52960_e68339_d_n9;
        locals.var_xgm__blk1426_rv = 0.0;

        let (assign52970_e68360, assign52970_e68360_d_n4, assign52970_e68360_d_n6, assign52970_e68360_d_n7, assign52970_e68360_d_n8, assign52970_e68360_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52970_e68350: f64 = (1.0 - locals.var_em__blk1422);
        let assign52970_e68354: f64 = (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427);
        let assign52970_e68356: f64 = (assign52970_e68354 * locals.var_inv_gf2__blk1341);
        let assign52970_e68357: f64 = (2.0 * assign52970_e68356);
        let assign52970_e68358: f64 = (assign52970_e68350 + assign52970_e68357);
        (assign52970_e68358, ((-locals.var_em__blk1422_dn4) + (2.0 * ((((locals.var_xgm__blk1426_dn4 * locals.var_eta_p__blk1427) + (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427_dn4)) * locals.var_inv_gf2__blk1341) + (assign52970_e68354 * locals.var_inv_gf2__blk1341_dn4)))), ((-locals.var_em__blk1422_dn6) + (2.0 * ((((locals.var_xgm__blk1426_dn6 * locals.var_eta_p__blk1427) + (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427_dn6)) * locals.var_inv_gf2__blk1341) + (assign52970_e68354 * locals.var_inv_gf2__blk1341_dn6)))), ((-locals.var_em__blk1422_dn7) + (2.0 * ((((locals.var_xgm__blk1426_dn7 * locals.var_eta_p__blk1427) + (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427_dn7)) * locals.var_inv_gf2__blk1341) + (assign52970_e68354 * locals.var_inv_gf2__blk1341_dn7)))), ((-locals.var_em__blk1422_dn8) + (2.0 * ((((locals.var_xgm__blk1426_dn8 * locals.var_eta_p__blk1427) + (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427_dn8)) * locals.var_inv_gf2__blk1341) + (assign52970_e68354 * locals.var_inv_gf2__blk1341_dn8)))), ((-locals.var_em__blk1422_dn9) + (2.0 * ((((locals.var_xgm__blk1426_dn9 * locals.var_eta_p__blk1427) + (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427_dn9)) * locals.var_inv_gf2__blk1341) + (assign52970_e68354 * locals.var_inv_gf2__blk1341_dn9)))),)
    } else {
        (locals.var_km0__blk1437, locals.var_km0__blk1437_dn4, locals.var_km0__blk1437_dn6, locals.var_km0__blk1437_dn7, locals.var_km0__blk1437_dn8, locals.var_km0__blk1437_dn9,)
    }
};
        locals.var_km0__blk1437 = assign52970_e68360;
        locals.var_km0__blk1437_dn4 = assign52970_e68360_d_n4;
        locals.var_km0__blk1437_dn6 = assign52970_e68360_d_n6;
        locals.var_km0__blk1437_dn7 = assign52970_e68360_d_n7;
        locals.var_km0__blk1437_dn8 = assign52970_e68360_d_n8;
        locals.var_km0__blk1437_dn9 = assign52970_e68360_d_n9;
        locals.var_km0__blk1437_rv = 0.0;

        let (assign52980_e68383, assign52980_e68383_d_n4, assign52980_e68383_d_n6, assign52980_e68383_d_n7, assign52980_e68383_d_n8, assign52980_e68383_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52980_e68371: f64 = (locals.var_x_ds__blk1411 * locals.var_km__blk1436);
        let assign52980_e68374: f64 = (locals.var_d0__blk1430 + locals.var_d_bar__blk1423);
        let assign52980_e68375: f64 = (assign52980_e68371 * assign52980_e68374);
        let assign52980_e68379: f64 = (locals.var_km__blk1436 * locals.var_d_bar__blk1423);
        let assign52980_e68380: f64 = (locals.var_km0__blk1437 + assign52980_e68379);
        let assign52980_e68381: f64 = (assign52980_e68375 / assign52980_e68380);
        (assign52980_e68381, (((((((locals.var_x_ds__blk1411_dn4 * locals.var_km__blk1436) + (locals.var_x_ds__blk1411 * locals.var_km__blk1436_dn4)) * assign52980_e68374) + (assign52980_e68371 * (locals.var_d0__blk1430_dn4 + locals.var_d_bar__blk1423_dn4))) * assign52980_e68380) - (assign52980_e68375 * (locals.var_km0__blk1437_dn4 + ((locals.var_km__blk1436_dn4 * locals.var_d_bar__blk1423) + (locals.var_km__blk1436 * locals.var_d_bar__blk1423_dn4))))) / (assign52980_e68380 * assign52980_e68380)), (((((((locals.var_x_ds__blk1411_dn6 * locals.var_km__blk1436) + (locals.var_x_ds__blk1411 * locals.var_km__blk1436_dn6)) * assign52980_e68374) + (assign52980_e68371 * (locals.var_d0__blk1430_dn6 + locals.var_d_bar__blk1423_dn6))) * assign52980_e68380) - (assign52980_e68375 * (locals.var_km0__blk1437_dn6 + ((locals.var_km__blk1436_dn6 * locals.var_d_bar__blk1423) + (locals.var_km__blk1436 * locals.var_d_bar__blk1423_dn6))))) / (assign52980_e68380 * assign52980_e68380)), (((((((locals.var_x_ds__blk1411_dn7 * locals.var_km__blk1436) + (locals.var_x_ds__blk1411 * locals.var_km__blk1436_dn7)) * assign52980_e68374) + (assign52980_e68371 * (locals.var_d0__blk1430_dn7 + locals.var_d_bar__blk1423_dn7))) * assign52980_e68380) - (assign52980_e68375 * (locals.var_km0__blk1437_dn7 + ((locals.var_km__blk1436_dn7 * locals.var_d_bar__blk1423) + (locals.var_km__blk1436 * locals.var_d_bar__blk1423_dn7))))) / (assign52980_e68380 * assign52980_e68380)), (((((((locals.var_x_ds__blk1411_dn8 * locals.var_km__blk1436) + (locals.var_x_ds__blk1411 * locals.var_km__blk1436_dn8)) * assign52980_e68374) + (assign52980_e68371 * (locals.var_d0__blk1430_dn8 + locals.var_d_bar__blk1423_dn8))) * assign52980_e68380) - (assign52980_e68375 * (locals.var_km0__blk1437_dn8 + ((locals.var_km__blk1436_dn8 * locals.var_d_bar__blk1423) + (locals.var_km__blk1436 * locals.var_d_bar__blk1423_dn8))))) / (assign52980_e68380 * assign52980_e68380)), (((((((locals.var_x_ds__blk1411_dn9 * locals.var_km__blk1436) + (locals.var_x_ds__blk1411 * locals.var_km__blk1436_dn9)) * assign52980_e68374) + (assign52980_e68371 * (locals.var_d0__blk1430_dn9 + locals.var_d_bar__blk1423_dn9))) * assign52980_e68380) - (assign52980_e68375 * (locals.var_km0__blk1437_dn9 + ((locals.var_km__blk1436_dn9 * locals.var_d_bar__blk1423) + (locals.var_km__blk1436 * locals.var_d_bar__blk1423_dn9))))) / (assign52980_e68380 * assign52980_e68380)),)
    } else {
        (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9,)
    }
};
        locals.var_x_ds__blk1411 = assign52980_e68383;
        locals.var_x_ds__blk1411_dn4 = assign52980_e68383_d_n4;
        locals.var_x_ds__blk1411_dn6 = assign52980_e68383_d_n6;
        locals.var_x_ds__blk1411_dn7 = assign52980_e68383_d_n7;
        locals.var_x_ds__blk1411_dn8 = assign52980_e68383_d_n8;
        locals.var_x_ds__blk1411_dn9 = assign52980_e68383_d_n9;
        locals.var_x_ds__blk1411_rv = 0.0;

        let (assign52990_e68396, assign52990_e68396_d_n4, assign52990_e68396_d_n6, assign52990_e68396_d_n7, assign52990_e68396_d_n8, assign52990_e68396_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign52990_e68394: f64 = (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339);
        (assign52990_e68394, ((locals.var_x_ds__blk1411_dn4 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn4)), ((locals.var_x_ds__blk1411_dn6 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn6)), ((locals.var_x_ds__blk1411_dn7 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn7)), ((locals.var_x_ds__blk1411_dn8 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn8)), ((locals.var_x_ds__blk1411_dn9 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn9)),)
    } else {
        (locals.var_dps__blk1414, locals.var_dps__blk1414_dn4, locals.var_dps__blk1414_dn6, locals.var_dps__blk1414_dn7, locals.var_dps__blk1414_dn8, locals.var_dps__blk1414_dn9,)
    }
};
        locals.var_dps__blk1414 = assign52990_e68396;
        locals.var_dps__blk1414_dn4 = assign52990_e68396_d_n4;
        locals.var_dps__blk1414_dn6 = assign52990_e68396_d_n6;
        locals.var_dps__blk1414_dn7 = assign52990_e68396_d_n7;
        locals.var_dps__blk1414_dn8 = assign52990_e68396_d_n8;
        locals.var_dps__blk1414_dn9 = assign52990_e68396_d_n9;
        locals.var_dps__blk1414_rv = 0.0;

        let (assign53000_e68406, assign53000_e68406_d_n4, assign53000_e68406_d_n6, assign53000_e68406_d_n7, assign53000_e68406_d_n8, assign53000_e68406_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) {
        let assign53000_e68404: f64 = (locals.var_pm__blk1425).sqrt();
        (assign53000_e68404, (locals.var_pm__blk1425_dn4 / (2.0 * assign53000_e68404)), (locals.var_pm__blk1425_dn6 / (2.0 * assign53000_e68404)), (locals.var_pm__blk1425_dn7 / (2.0 * assign53000_e68404)), (locals.var_pm__blk1425_dn8 / (2.0 * assign53000_e68404)), (locals.var_pm__blk1425_dn9 / (2.0 * assign53000_e68404)),)
    } else {
        (locals.var_sqm__blk1428, locals.var_sqm__blk1428_dn4, locals.var_sqm__blk1428_dn6, locals.var_sqm__blk1428_dn7, locals.var_sqm__blk1428_dn8, locals.var_sqm__blk1428_dn9,)
    }
};
        locals.var_sqm__blk1428 = assign53000_e68406;
        locals.var_sqm__blk1428_dn4 = assign53000_e68406_d_n4;
        locals.var_sqm__blk1428_dn6 = assign53000_e68406_d_n6;
        locals.var_sqm__blk1428_dn7 = assign53000_e68406_d_n7;
        locals.var_sqm__blk1428_dn8 = assign53000_e68406_d_n8;
        locals.var_sqm__blk1428_dn9 = assign53000_e68406_d_n9;
        locals.var_sqm__blk1428_rv = 0.0;

        let (assign53010_e68425, assign53010_e68425_d_n4, assign53010_e68425_d_n6, assign53010_e68425_d_n7, assign53010_e68425_d_n8, assign53010_e68425_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) {
        let assign53010_e68418: f64 = (1.0 - locals.var_em__blk1422);
        let assign53010_e68419: f64 = (locals.var_gf__blk1324 * assign53010_e68418);
        let assign53010_e68421: f64 = (assign53010_e68419 / locals.var_sqm__blk1428);
        let assign53010_e68422: f64 = (0.5 * assign53010_e68421);
        let assign53010_e68423: f64 = (locals.var_eta_p__blk1427 + assign53010_e68422);
        (assign53010_e68423, (locals.var_eta_p__blk1427_dn4 + (0.5 * (((((locals.var_gf__blk1324_dn4 * assign53010_e68418) + (locals.var_gf__blk1324 * (-locals.var_em__blk1422_dn4))) * locals.var_sqm__blk1428) - (assign53010_e68419 * locals.var_sqm__blk1428_dn4)) / (locals.var_sqm__blk1428 * locals.var_sqm__blk1428)))), (locals.var_eta_p__blk1427_dn6 + (0.5 * (((((locals.var_gf__blk1324_dn6 * assign53010_e68418) + (locals.var_gf__blk1324 * (-locals.var_em__blk1422_dn6))) * locals.var_sqm__blk1428) - (assign53010_e68419 * locals.var_sqm__blk1428_dn6)) / (locals.var_sqm__blk1428 * locals.var_sqm__blk1428)))), (locals.var_eta_p__blk1427_dn7 + (0.5 * (((((locals.var_gf__blk1324_dn7 * assign53010_e68418) + (locals.var_gf__blk1324 * (-locals.var_em__blk1422_dn7))) * locals.var_sqm__blk1428) - (assign53010_e68419 * locals.var_sqm__blk1428_dn7)) / (locals.var_sqm__blk1428 * locals.var_sqm__blk1428)))), (locals.var_eta_p__blk1427_dn8 + (0.5 * (((((locals.var_gf__blk1324_dn8 * assign53010_e68418) + (locals.var_gf__blk1324 * (-locals.var_em__blk1422_dn8))) * locals.var_sqm__blk1428) - (assign53010_e68419 * locals.var_sqm__blk1428_dn8)) / (locals.var_sqm__blk1428 * locals.var_sqm__blk1428)))), (locals.var_eta_p__blk1427_dn9 + (0.5 * (((((locals.var_gf__blk1324_dn9 * assign53010_e68418) + (locals.var_gf__blk1324 * (-locals.var_em__blk1422_dn9))) * locals.var_sqm__blk1428) - (assign53010_e68419 * locals.var_sqm__blk1428_dn9)) / (locals.var_sqm__blk1428 * locals.var_sqm__blk1428)))),)
    } else {
        (locals.var_alpha__blk1429, locals.var_alpha__blk1429_dn4, locals.var_alpha__blk1429_dn6, locals.var_alpha__blk1429_dn7, locals.var_alpha__blk1429_dn8, locals.var_alpha__blk1429_dn9,)
    }
};
        locals.var_alpha__blk1429 = assign53010_e68425;
        locals.var_alpha__blk1429_dn4 = assign53010_e68425_d_n4;
        locals.var_alpha__blk1429_dn6 = assign53010_e68425_d_n6;
        locals.var_alpha__blk1429_dn7 = assign53010_e68425_d_n7;
        locals.var_alpha__blk1429_dn8 = assign53010_e68425_d_n8;
        locals.var_alpha__blk1429_dn9 = assign53010_e68425_d_n9;
        locals.var_alpha__blk1429_rv = 0.0;

        let (assign53020_e68441, assign53020_e68441_d_n4, assign53020_e68441_d_n6, assign53020_e68441_d_n7, assign53020_e68441_d_n8, assign53020_e68441_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53020_e68432: f64 = (locals.var_gf2__blk1325 * locals.var_dm__blk1424);
        let assign53020_e68436: f64 = (locals.var_gf__blk1324 * locals.var_sqm__blk1428);
        let assign53020_e68437: f64 = (locals.var_xgm__blk1426 + assign53020_e68436);
        let assign53020_e68438: f64 = (assign53020_e68432 / assign53020_e68437);
        let assign53020_e68439: f64 = (locals.var_phit1__blk1339 * assign53020_e68438);
        (assign53020_e68439, ((locals.var_phit1__blk1339_dn4 * assign53020_e68438) + (locals.var_phit1__blk1339 * (((((locals.var_gf2__blk1325_dn4 * locals.var_dm__blk1424) + (locals.var_gf2__blk1325 * locals.var_dm__blk1424_dn4)) * assign53020_e68437) - (assign53020_e68432 * (locals.var_xgm__blk1426_dn4 + ((locals.var_gf__blk1324_dn4 * locals.var_sqm__blk1428) + (locals.var_gf__blk1324 * locals.var_sqm__blk1428_dn4))))) / (assign53020_e68437 * assign53020_e68437)))), ((locals.var_phit1__blk1339_dn6 * assign53020_e68438) + (locals.var_phit1__blk1339 * (((((locals.var_gf2__blk1325_dn6 * locals.var_dm__blk1424) + (locals.var_gf2__blk1325 * locals.var_dm__blk1424_dn6)) * assign53020_e68437) - (assign53020_e68432 * (locals.var_xgm__blk1426_dn6 + ((locals.var_gf__blk1324_dn6 * locals.var_sqm__blk1428) + (locals.var_gf__blk1324 * locals.var_sqm__blk1428_dn6))))) / (assign53020_e68437 * assign53020_e68437)))), ((locals.var_phit1__blk1339_dn7 * assign53020_e68438) + (locals.var_phit1__blk1339 * (((((locals.var_gf2__blk1325_dn7 * locals.var_dm__blk1424) + (locals.var_gf2__blk1325 * locals.var_dm__blk1424_dn7)) * assign53020_e68437) - (assign53020_e68432 * (locals.var_xgm__blk1426_dn7 + ((locals.var_gf__blk1324_dn7 * locals.var_sqm__blk1428) + (locals.var_gf__blk1324 * locals.var_sqm__blk1428_dn7))))) / (assign53020_e68437 * assign53020_e68437)))), ((locals.var_phit1__blk1339_dn8 * assign53020_e68438) + (locals.var_phit1__blk1339 * (((((locals.var_gf2__blk1325_dn8 * locals.var_dm__blk1424) + (locals.var_gf2__blk1325 * locals.var_dm__blk1424_dn8)) * assign53020_e68437) - (assign53020_e68432 * (locals.var_xgm__blk1426_dn8 + ((locals.var_gf__blk1324_dn8 * locals.var_sqm__blk1428) + (locals.var_gf__blk1324 * locals.var_sqm__blk1428_dn8))))) / (assign53020_e68437 * assign53020_e68437)))), ((locals.var_phit1__blk1339_dn9 * assign53020_e68438) + (locals.var_phit1__blk1339 * (((((locals.var_gf2__blk1325_dn9 * locals.var_dm__blk1424) + (locals.var_gf2__blk1325 * locals.var_dm__blk1424_dn9)) * assign53020_e68437) - (assign53020_e68432 * (locals.var_xgm__blk1426_dn9 + ((locals.var_gf__blk1324_dn9 * locals.var_sqm__blk1428) + (locals.var_gf__blk1324 * locals.var_sqm__blk1428_dn9))))) / (assign53020_e68437 * assign53020_e68437)))),)
    } else {
        (locals.var_qim__blk1438, locals.var_qim__blk1438_dn4, locals.var_qim__blk1438_dn6, locals.var_qim__blk1438_dn7, locals.var_qim__blk1438_dn8, locals.var_qim__blk1438_dn9,)
    }
};
        locals.var_qim__blk1438 = assign53020_e68441;
        locals.var_qim__blk1438_dn4 = assign53020_e68441_d_n4;
        locals.var_qim__blk1438_dn6 = assign53020_e68441_d_n6;
        locals.var_qim__blk1438_dn7 = assign53020_e68441_d_n7;
        locals.var_qim__blk1438_dn8 = assign53020_e68441_d_n8;
        locals.var_qim__blk1438_dn9 = assign53020_e68441_d_n9;
        locals.var_qim__blk1438_rv = 0.0;

        let (assign53030_e68451, assign53030_e68451_d_n4, assign53030_e68451_d_n6, assign53030_e68451_d_n7, assign53030_e68451_d_n8, assign53030_e68451_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53030_e68448: f64 = (locals.var_phit1__blk1339 * locals.var_alpha__blk1429);
        let assign53030_e68449: f64 = (locals.var_qim__blk1438 + assign53030_e68448);
        (assign53030_e68449, (locals.var_qim__blk1438_dn4 + ((locals.var_phit1__blk1339_dn4 * locals.var_alpha__blk1429) + (locals.var_phit1__blk1339 * locals.var_alpha__blk1429_dn4))), (locals.var_qim__blk1438_dn6 + ((locals.var_phit1__blk1339_dn6 * locals.var_alpha__blk1429) + (locals.var_phit1__blk1339 * locals.var_alpha__blk1429_dn6))), (locals.var_qim__blk1438_dn7 + ((locals.var_phit1__blk1339_dn7 * locals.var_alpha__blk1429) + (locals.var_phit1__blk1339 * locals.var_alpha__blk1429_dn7))), (locals.var_qim__blk1438_dn8 + ((locals.var_phit1__blk1339_dn8 * locals.var_alpha__blk1429) + (locals.var_phit1__blk1339 * locals.var_alpha__blk1429_dn8))), (locals.var_qim__blk1438_dn9 + ((locals.var_phit1__blk1339_dn9 * locals.var_alpha__blk1429) + (locals.var_phit1__blk1339 * locals.var_alpha__blk1429_dn9))),)
    } else {
        (locals.var_qim1__blk1439, locals.var_qim1__blk1439_dn4, locals.var_qim1__blk1439_dn6, locals.var_qim1__blk1439_dn7, locals.var_qim1__blk1439_dn8, locals.var_qim1__blk1439_dn9,)
    }
};
        locals.var_qim1__blk1439 = assign53030_e68451;
        locals.var_qim1__blk1439_dn4 = assign53030_e68451_d_n4;
        locals.var_qim1__blk1439_dn6 = assign53030_e68451_d_n6;
        locals.var_qim1__blk1439_dn7 = assign53030_e68451_d_n7;
        locals.var_qim1__blk1439_dn8 = assign53030_e68451_d_n8;
        locals.var_qim1__blk1439_dn9 = assign53030_e68451_d_n9;
        locals.var_qim1__blk1439_rv = 0.0;

        let (assign53040_e68461, assign53040_e68461_d_n4, assign53040_e68461_d_n6, assign53040_e68461_d_n7, assign53040_e68461_d_n8, assign53040_e68461_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53040_e68457: f64 = (locals.var_sqm__blk1428 * locals.var_gf__blk1324);
        let assign53040_e68459: f64 = (assign53040_e68457 * locals.var_phit1__blk1339);
        (assign53040_e68459, ((((locals.var_sqm__blk1428_dn4 * locals.var_gf__blk1324) + (locals.var_sqm__blk1428 * locals.var_gf__blk1324_dn4)) * locals.var_phit1__blk1339) + (assign53040_e68457 * locals.var_phit1__blk1339_dn4)), ((((locals.var_sqm__blk1428_dn6 * locals.var_gf__blk1324) + (locals.var_sqm__blk1428 * locals.var_gf__blk1324_dn6)) * locals.var_phit1__blk1339) + (assign53040_e68457 * locals.var_phit1__blk1339_dn6)), ((((locals.var_sqm__blk1428_dn7 * locals.var_gf__blk1324) + (locals.var_sqm__blk1428 * locals.var_gf__blk1324_dn7)) * locals.var_phit1__blk1339) + (assign53040_e68457 * locals.var_phit1__blk1339_dn7)), ((((locals.var_sqm__blk1428_dn8 * locals.var_gf__blk1324) + (locals.var_sqm__blk1428 * locals.var_gf__blk1324_dn8)) * locals.var_phit1__blk1339) + (assign53040_e68457 * locals.var_phit1__blk1339_dn8)), ((((locals.var_sqm__blk1428_dn9 * locals.var_gf__blk1324) + (locals.var_sqm__blk1428 * locals.var_gf__blk1324_dn9)) * locals.var_phit1__blk1339) + (assign53040_e68457 * locals.var_phit1__blk1339_dn9)),)
    } else {
        (locals.var_qbm__blk1440, locals.var_qbm__blk1440_dn4, locals.var_qbm__blk1440_dn6, locals.var_qbm__blk1440_dn7, locals.var_qbm__blk1440_dn8, locals.var_qbm__blk1440_dn9,)
    }
};
        locals.var_qbm__blk1440 = assign53040_e68461;
        locals.var_qbm__blk1440_dn4 = assign53040_e68461_d_n4;
        locals.var_qbm__blk1440_dn6 = assign53040_e68461_d_n6;
        locals.var_qbm__blk1440_dn7 = assign53040_e68461_d_n7;
        locals.var_qbm__blk1440_dn8 = assign53040_e68461_d_n8;
        locals.var_qbm__blk1440_dn9 = assign53040_e68461_d_n9;
        locals.var_qbm__blk1440_rv = 0.0;

        let assign53050_e68464: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1520 = assign53050_e68464;
        locals.var_guard1520_rv = 0.0;

        let (assign53060_e68476, assign53060_e68476_d_n4, assign53060_e68476_d_n6, assign53060_e68476_d_n7, assign53060_e68476_d_n8, assign53060_e68476_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1520 != 0.0)) {
        let assign53060_e68473: f64 = (locals.var_rsg_i * locals.var_qim__blk1438);
        let assign53060_e68474: f64 = (1.0 - assign53060_e68473);
        (assign53060_e68474, (-(locals.var_rsg_i * locals.var_qim__blk1438_dn4)), (-(locals.var_rsg_i * locals.var_qim__blk1438_dn6)), (-(locals.var_rsg_i * locals.var_qim__blk1438_dn7)), (-(locals.var_rsg_i * locals.var_qim__blk1438_dn8)), (-(locals.var_rsg_i * locals.var_qim__blk1438_dn9)),)
    } else {
        (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9,)
    }
};
        locals.var_rhog__blk1379 = assign53060_e68476;
        locals.var_rhog__blk1379_dn4 = assign53060_e68476_d_n4;
        locals.var_rhog__blk1379_dn6 = assign53060_e68476_d_n6;
        locals.var_rhog__blk1379_dn7 = assign53060_e68476_d_n7;
        locals.var_rhog__blk1379_dn8 = assign53060_e68476_d_n8;
        locals.var_rhog__blk1379_dn9 = assign53060_e68476_d_n9;
        locals.var_rhog__blk1379_rv = 0.0;

        let (assign53070_e68491, assign53070_e68491_d_n4, assign53070_e68491_d_n6, assign53070_e68491_d_n7, assign53070_e68491_d_n8, assign53070_e68491_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1520 == 0.0)) {
        let assign53070_e68487: f64 = (locals.var_rsg_i * locals.var_qim__blk1438);
        let assign53070_e68488: f64 = (1.0 + assign53070_e68487);
        let assign53070_e68489: f64 = (1.0 / assign53070_e68488);
        (assign53070_e68489, (-((locals.var_rsg_i * locals.var_qim__blk1438_dn4) / (assign53070_e68488 * assign53070_e68488))), (-((locals.var_rsg_i * locals.var_qim__blk1438_dn6) / (assign53070_e68488 * assign53070_e68488))), (-((locals.var_rsg_i * locals.var_qim__blk1438_dn7) / (assign53070_e68488 * assign53070_e68488))), (-((locals.var_rsg_i * locals.var_qim__blk1438_dn8) / (assign53070_e68488 * assign53070_e68488))), (-((locals.var_rsg_i * locals.var_qim__blk1438_dn9) / (assign53070_e68488 * assign53070_e68488))),)
    } else {
        (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9,)
    }
};
        locals.var_rhog__blk1379 = assign53070_e68491;
        locals.var_rhog__blk1379_dn4 = assign53070_e68491_d_n4;
        locals.var_rhog__blk1379_dn6 = assign53070_e68491_d_n6;
        locals.var_rhog__blk1379_dn7 = assign53070_e68491_d_n7;
        locals.var_rhog__blk1379_dn8 = assign53070_e68491_d_n8;
        locals.var_rhog__blk1379_dn9 = assign53070_e68491_d_n9;
        locals.var_rhog__blk1379_rv = 0.0;

        let (assign53080_e68503, assign53080_e68503_d_n4, assign53080_e68503_d_n6, assign53080_e68503_d_n7, assign53080_e68503_d_n8, assign53080_e68503_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53080_e68497: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
        let assign53080_e68499: f64 = (assign53080_e68497 * locals.var_rhog__blk1379);
        let assign53080_e68501: f64 = (assign53080_e68499 * locals.var_qim__blk1438);
        (assign53080_e68501, ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign53080_e68497 * locals.var_rhog__blk1379_dn4)) * locals.var_qim__blk1438) + (assign53080_e68499 * locals.var_qim__blk1438_dn4)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign53080_e68497 * locals.var_rhog__blk1379_dn6)) * locals.var_qim__blk1438) + (assign53080_e68499 * locals.var_qim__blk1438_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign53080_e68497 * locals.var_rhog__blk1379_dn7)) * locals.var_qim__blk1438) + (assign53080_e68499 * locals.var_qim__blk1438_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign53080_e68497 * locals.var_rhog__blk1379_dn8)) * locals.var_qim__blk1438) + (assign53080_e68499 * locals.var_qim__blk1438_dn8)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign53080_e68497 * locals.var_rhog__blk1379_dn9)) * locals.var_qim__blk1438) + (assign53080_e68499 * locals.var_qim__blk1438_dn9)),)
    } else {
        (locals.var_gr__blk1380, locals.var_gr__blk1380_dn4, locals.var_gr__blk1380_dn6, locals.var_gr__blk1380_dn7, locals.var_gr__blk1380_dn8, locals.var_gr__blk1380_dn9,)
    }
};
        locals.var_gr__blk1380 = assign53080_e68503;
        locals.var_gr__blk1380_dn4 = assign53080_e68503_d_n4;
        locals.var_gr__blk1380_dn6 = assign53080_e68503_d_n6;
        locals.var_gr__blk1380_dn7 = assign53080_e68503_d_n7;
        locals.var_gr__blk1380_dn8 = assign53080_e68503_d_n8;
        locals.var_gr__blk1380_dn9 = assign53080_e68503_d_n9;
        locals.var_gr__blk1380_rv = 0.0;

        let (assign53090_e68513, assign53090_e68513_d_n4, assign53090_e68513_d_n6, assign53090_e68513_d_n7, assign53090_e68513_d_n8, assign53090_e68513_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53090_e68510: f64 = (locals.var_eta_mu * locals.var_qim__blk1438);
        let assign53090_e68511: f64 = (locals.var_qbm__blk1440 + assign53090_e68510);
        (assign53090_e68511, (locals.var_qbm__blk1440_dn4 + (locals.var_eta_mu * locals.var_qim__blk1438_dn4)), (locals.var_qbm__blk1440_dn6 + (locals.var_eta_mu * locals.var_qim__blk1438_dn6)), (locals.var_qbm__blk1440_dn7 + (locals.var_eta_mu * locals.var_qim__blk1438_dn7)), (locals.var_qbm__blk1440_dn8 + (locals.var_eta_mu * locals.var_qim__blk1438_dn8)), (locals.var_qbm__blk1440_dn9 + (locals.var_eta_mu * locals.var_qim__blk1438_dn9)),)
    } else {
        (locals.var_qeff__blk1441, locals.var_qeff__blk1441_dn4, locals.var_qeff__blk1441_dn6, locals.var_qeff__blk1441_dn7, locals.var_qeff__blk1441_dn8, locals.var_qeff__blk1441_dn9,)
    }
};
        locals.var_qeff__blk1441 = assign53090_e68513;
        locals.var_qeff__blk1441_dn4 = assign53090_e68513_d_n4;
        locals.var_qeff__blk1441_dn6 = assign53090_e68513_d_n6;
        locals.var_qeff__blk1441_dn7 = assign53090_e68513_d_n7;
        locals.var_qeff__blk1441_dn8 = assign53090_e68513_d_n8;
        locals.var_qeff__blk1441_dn9 = assign53090_e68513_d_n9;
        locals.var_qeff__blk1441_rv = 0.0;

        let (assign53100_e68523, assign53100_e68523_d_n4, assign53100_e68523_d_n6, assign53100_e68523_d_n7, assign53100_e68523_d_n8, assign53100_e68523_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53100_e68520: f64 = (locals.var_eta_mu1 * locals.var_qim__blk1438);
        let assign53100_e68521: f64 = (locals.var_qbm__blk1440 + assign53100_e68520);
        (assign53100_e68521, (locals.var_qbm__blk1440_dn4 + (locals.var_eta_mu1 * locals.var_qim__blk1438_dn4)), (locals.var_qbm__blk1440_dn6 + (locals.var_eta_mu1 * locals.var_qim__blk1438_dn6)), (locals.var_qbm__blk1440_dn7 + (locals.var_eta_mu1 * locals.var_qim__blk1438_dn7)), (locals.var_qbm__blk1440_dn8 + (locals.var_eta_mu1 * locals.var_qim__blk1438_dn8)), (locals.var_qbm__blk1440_dn9 + (locals.var_eta_mu1 * locals.var_qim__blk1438_dn9)),)
    } else {
        (locals.var_qeff1__blk1442, locals.var_qeff1__blk1442_dn4, locals.var_qeff1__blk1442_dn6, locals.var_qeff1__blk1442_dn7, locals.var_qeff1__blk1442_dn8, locals.var_qeff1__blk1442_dn9,)
    }
};
        locals.var_qeff1__blk1442 = assign53100_e68523;
        locals.var_qeff1__blk1442_dn4 = assign53100_e68523_d_n4;
        locals.var_qeff1__blk1442_dn6 = assign53100_e68523_d_n6;
        locals.var_qeff1__blk1442_dn7 = assign53100_e68523_d_n7;
        locals.var_qeff1__blk1442_dn8 = assign53100_e68523_d_n8;
        locals.var_qeff1__blk1442_dn9 = assign53100_e68523_d_n9;
        locals.var_qeff1__blk1442_rv = 0.0;

        let (assign53110_e68531, assign53110_e68531_d_n4, assign53110_e68531_d_n6, assign53110_e68531_d_n7, assign53110_e68531_d_n8, assign53110_e68531_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53110_e68529: f64 = (locals.var_e_eff0 * locals.var_qeff__blk1441);
        (assign53110_e68529, (locals.var_e_eff0 * locals.var_qeff__blk1441_dn4), (locals.var_e_eff0 * locals.var_qeff__blk1441_dn6), (locals.var_e_eff0 * locals.var_qeff__blk1441_dn7), (locals.var_e_eff0 * locals.var_qeff__blk1441_dn8), (locals.var_e_eff0 * locals.var_qeff__blk1441_dn9),)
    } else {
        (locals.var_eeffm__blk1443, locals.var_eeffm__blk1443_dn4, locals.var_eeffm__blk1443_dn6, locals.var_eeffm__blk1443_dn7, locals.var_eeffm__blk1443_dn8, locals.var_eeffm__blk1443_dn9,)
    }
};
        locals.var_eeffm__blk1443 = assign53110_e68531;
        locals.var_eeffm__blk1443_dn4 = assign53110_e68531_d_n4;
        locals.var_eeffm__blk1443_dn6 = assign53110_e68531_d_n6;
        locals.var_eeffm__blk1443_dn7 = assign53110_e68531_d_n7;
        locals.var_eeffm__blk1443_dn8 = assign53110_e68531_d_n8;
        locals.var_eeffm__blk1443_dn9 = assign53110_e68531_d_n9;
        locals.var_eeffm__blk1443_rv = 0.0;

        let (assign53120_e68544, assign53120_e68544_d_n4, assign53120_e68544_d_n6, assign53120_e68544_d_n7, assign53120_e68544_d_n8, assign53120_e68544_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53120_e68538: f64 = (locals.var_pm__blk1425 + locals.var_dm__blk1424);
        let assign53120_e68540: f64 = (assign53120_e68538 + 1e-14);
        let assign53120_e68541: f64 = (locals.var_pm__blk1425 / assign53120_e68540);
        let assign53120_e68542: f64 = (assign53120_e68541).ln();
        (assign53120_e68542, ((((locals.var_pm__blk1425_dn4 * assign53120_e68540) - (locals.var_pm__blk1425 * (locals.var_pm__blk1425_dn4 + locals.var_dm__blk1424_dn4))) / (assign53120_e68540 * assign53120_e68540)) / assign53120_e68541), ((((locals.var_pm__blk1425_dn6 * assign53120_e68540) - (locals.var_pm__blk1425 * (locals.var_pm__blk1425_dn6 + locals.var_dm__blk1424_dn6))) / (assign53120_e68540 * assign53120_e68540)) / assign53120_e68541), ((((locals.var_pm__blk1425_dn7 * assign53120_e68540) - (locals.var_pm__blk1425 * (locals.var_pm__blk1425_dn7 + locals.var_dm__blk1424_dn7))) / (assign53120_e68540 * assign53120_e68540)) / assign53120_e68541), ((((locals.var_pm__blk1425_dn8 * assign53120_e68540) - (locals.var_pm__blk1425 * (locals.var_pm__blk1425_dn8 + locals.var_dm__blk1424_dn8))) / (assign53120_e68540 * assign53120_e68540)) / assign53120_e68541), ((((locals.var_pm__blk1425_dn9 * assign53120_e68540) - (locals.var_pm__blk1425 * (locals.var_pm__blk1425_dn9 + locals.var_dm__blk1424_dn9))) / (assign53120_e68540 * assign53120_e68540)) / assign53120_e68541),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign53120_e68544;
        locals.var_temp1_dn4 = assign53120_e68544_d_n4;
        locals.var_temp1_dn6 = assign53120_e68544_d_n6;
        locals.var_temp1_dn7 = assign53120_e68544_d_n7;
        locals.var_temp1_dn8 = assign53120_e68544_d_n8;
        locals.var_temp1_dn9 = assign53120_e68544_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign53130_e68563, assign53130_e68563_d_n4, assign53130_e68563_d_n6, assign53130_e68563_d_n7, assign53130_e68563_d_n8, assign53130_e68563_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53130_e68550: f64 = (locals.var_eeffm__blk1443 * locals.var_mue_t);
        let assign53130_e68552: f64 = (assign53130_e68550).powf(locals.var_themu_t);
        let assign53130_e68556: f64 = (0.5 * locals.var_thecs_t);
        let assign53130_e68558: f64 = (assign53130_e68556 * locals.var_temp1);
        let assign53130_e68559: f64 = (assign53130_e68558).exp();
        let assign53130_e68560: f64 = (locals.var_cs_t * assign53130_e68559);
        let assign53130_e68561: f64 = (assign53130_e68552 + assign53130_e68560);
        (assign53130_e68561, (if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign53130_e68550).powf(locals.var_themu_t - 1.0) * ((locals.var_eeffm__blk1443_dn4 * locals.var_mue_t) + (locals.var_eeffm__blk1443 * locals.var_mue_t_dn4)))) } } else { (assign53130_e68552 * ((locals.var_themu_t_dn4 * (assign53130_e68550).ln()) + (locals.var_themu_t * (((locals.var_eeffm__blk1443_dn4 * locals.var_mue_t) + (locals.var_eeffm__blk1443 * locals.var_mue_t_dn4)) / assign53130_e68550)))) } + ((locals.var_cs_t_dn4 * assign53130_e68559) + (locals.var_cs_t * (assign53130_e68559 * (((0.5 * locals.var_thecs_t_dn4) * locals.var_temp1) + (assign53130_e68556 * locals.var_temp1_dn4)))))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign53130_e68550).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1443_dn6 * locals.var_mue_t))) } } else { (assign53130_e68552 * (locals.var_themu_t * ((locals.var_eeffm__blk1443_dn6 * locals.var_mue_t) / assign53130_e68550))) } + (locals.var_cs_t * (assign53130_e68559 * (assign53130_e68556 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign53130_e68550).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1443_dn7 * locals.var_mue_t))) } } else { (assign53130_e68552 * (locals.var_themu_t * ((locals.var_eeffm__blk1443_dn7 * locals.var_mue_t) / assign53130_e68550))) } + (locals.var_cs_t * (assign53130_e68559 * (assign53130_e68556 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign53130_e68550).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1443_dn8 * locals.var_mue_t))) } } else { (assign53130_e68552 * (locals.var_themu_t * ((locals.var_eeffm__blk1443_dn8 * locals.var_mue_t) / assign53130_e68550))) } + (locals.var_cs_t * (assign53130_e68559 * (assign53130_e68556 * locals.var_temp1_dn8)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign53130_e68550).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1443_dn9 * locals.var_mue_t))) } } else { (assign53130_e68552 * (locals.var_themu_t * ((locals.var_eeffm__blk1443_dn9 * locals.var_mue_t) / assign53130_e68550))) } + (locals.var_cs_t * (assign53130_e68559 * (assign53130_e68556 * locals.var_temp1_dn9)))),)
    } else {
        (locals.var_mutmp__blk1382, locals.var_mutmp__blk1382_dn4, locals.var_mutmp__blk1382_dn6, locals.var_mutmp__blk1382_dn7, locals.var_mutmp__blk1382_dn8, locals.var_mutmp__blk1382_dn9,)
    }
};
        locals.var_mutmp__blk1382 = assign53130_e68563;
        locals.var_mutmp__blk1382_dn4 = assign53130_e68563_d_n4;
        locals.var_mutmp__blk1382_dn6 = assign53130_e68563_d_n6;
        locals.var_mutmp__blk1382_dn7 = assign53130_e68563_d_n7;
        locals.var_mutmp__blk1382_dn8 = assign53130_e68563_d_n8;
        locals.var_mutmp__blk1382_dn9 = assign53130_e68563_d_n9;
        locals.var_mutmp__blk1382_rv = 0.0;

        let (assign53140_e68575, assign53140_e68575_d_n4, assign53140_e68575_d_n6, assign53140_e68575_d_n7, assign53140_e68575_d_n8, assign53140_e68575_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53140_e68569: f64 = (1.0 + locals.var_mutmp__blk1382);
        let assign53140_e68571: f64 = (assign53140_e68569 + locals.var_gr__blk1380);
        let assign53140_e68573: f64 = (assign53140_e68571 * locals.var_rxcor__blk1374);
        (assign53140_e68573, (((locals.var_mutmp__blk1382_dn4 + locals.var_gr__blk1380_dn4) * locals.var_rxcor__blk1374) + (assign53140_e68571 * locals.var_rxcor__blk1374_dn4)), (((locals.var_mutmp__blk1382_dn6 + locals.var_gr__blk1380_dn6) * locals.var_rxcor__blk1374) + (assign53140_e68571 * locals.var_rxcor__blk1374_dn6)), (((locals.var_mutmp__blk1382_dn7 + locals.var_gr__blk1380_dn7) * locals.var_rxcor__blk1374) + (assign53140_e68571 * locals.var_rxcor__blk1374_dn7)), (((locals.var_mutmp__blk1382_dn8 + locals.var_gr__blk1380_dn8) * locals.var_rxcor__blk1374) + (assign53140_e68571 * locals.var_rxcor__blk1374_dn8)), (((locals.var_mutmp__blk1382_dn9 + locals.var_gr__blk1380_dn9) * locals.var_rxcor__blk1374) + (assign53140_e68571 * locals.var_rxcor__blk1374_dn9)),)
    } else {
        (locals.var_gmob__blk1444, locals.var_gmob__blk1444_dn4, locals.var_gmob__blk1444_dn6, locals.var_gmob__blk1444_dn7, locals.var_gmob__blk1444_dn8, locals.var_gmob__blk1444_dn9,)
    }
};
        locals.var_gmob__blk1444 = assign53140_e68575;
        locals.var_gmob__blk1444_dn4 = assign53140_e68575_d_n4;
        locals.var_gmob__blk1444_dn6 = assign53140_e68575_d_n6;
        locals.var_gmob__blk1444_dn7 = assign53140_e68575_d_n7;
        locals.var_gmob__blk1444_dn8 = assign53140_e68575_d_n8;
        locals.var_gmob__blk1444_dn9 = assign53140_e68575_d_n9;
        locals.var_gmob__blk1444_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        locals: &mut StampLocals,
    ) {
        let (assign53150_e68596, assign53150_e68596_d_n4, assign53150_e68596_d_n6, assign53150_e68596_d_n7, assign53150_e68596_d_n8, assign53150_e68596_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53150_e68582: f64 = (locals.var_v_ds - locals.var_dps__blk1414);
        let assign53150_e68584: f64 = (assign53150_e68582 * locals.var_inv_vp);
        let assign53150_e68585: f64 = (1.0 + assign53150_e68584);
        let assign53150_e68589: f64 = (locals.var_vdse__blk1405 - locals.var_dps__blk1414);
        let assign53150_e68591: f64 = (assign53150_e68589 * locals.var_inv_vp);
        let assign53150_e68592: f64 = (1.0 + assign53150_e68591);
        let assign53150_e68593: f64 = (assign53150_e68585 / assign53150_e68592);
        let assign53150_e68594: f64 = (assign53150_e68593).ln();
        (assign53150_e68594, ((((((-locals.var_dps__blk1414_dn4) * locals.var_inv_vp) * assign53150_e68592) - (assign53150_e68585 * ((locals.var_vdse__blk1405_dn4 - locals.var_dps__blk1414_dn4) * locals.var_inv_vp))) / (assign53150_e68592 * assign53150_e68592)) / assign53150_e68593), ((((((-locals.var_dps__blk1414_dn6) * locals.var_inv_vp) * assign53150_e68592) - (assign53150_e68585 * ((locals.var_vdse__blk1405_dn6 - locals.var_dps__blk1414_dn6) * locals.var_inv_vp))) / (assign53150_e68592 * assign53150_e68592)) / assign53150_e68593), ((((((locals.var_v_ds_dn7 - locals.var_dps__blk1414_dn7) * locals.var_inv_vp) * assign53150_e68592) - (assign53150_e68585 * ((locals.var_vdse__blk1405_dn7 - locals.var_dps__blk1414_dn7) * locals.var_inv_vp))) / (assign53150_e68592 * assign53150_e68592)) / assign53150_e68593), ((((((locals.var_v_ds_dn8 - locals.var_dps__blk1414_dn8) * locals.var_inv_vp) * assign53150_e68592) - (assign53150_e68585 * ((locals.var_vdse__blk1405_dn8 - locals.var_dps__blk1414_dn8) * locals.var_inv_vp))) / (assign53150_e68592 * assign53150_e68592)) / assign53150_e68593), ((((((-locals.var_dps__blk1414_dn9) * locals.var_inv_vp) * assign53150_e68592) - (assign53150_e68585 * ((locals.var_vdse__blk1405_dn9 - locals.var_dps__blk1414_dn9) * locals.var_inv_vp))) / (assign53150_e68592 * assign53150_e68592)) / assign53150_e68593),)
    } else {
        (locals.var_s1__blk1445, locals.var_s1__blk1445_dn4, locals.var_s1__blk1445_dn6, locals.var_s1__blk1445_dn7, locals.var_s1__blk1445_dn8, locals.var_s1__blk1445_dn9,)
    }
};
        locals.var_s1__blk1445 = assign53150_e68596;
        locals.var_s1__blk1445_dn4 = assign53150_e68596_d_n4;
        locals.var_s1__blk1445_dn6 = assign53150_e68596_d_n6;
        locals.var_s1__blk1445_dn7 = assign53150_e68596_d_n7;
        locals.var_s1__blk1445_dn8 = assign53150_e68596_d_n8;
        locals.var_s1__blk1445_dn9 = assign53150_e68596_d_n9;
        locals.var_s1__blk1445_rv = 0.0;

        let (assign53160_e68604, assign53160_e68604_d_n4, assign53160_e68604_d_n6, assign53160_e68604_d_n7, assign53160_e68604_d_n8, assign53160_e68604_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53160_e68602: f64 = (locals.var_qim__blk1438 * locals.var_xitsb__blk1384);
        (assign53160_e68602, ((locals.var_qim__blk1438_dn4 * locals.var_xitsb__blk1384) + (locals.var_qim__blk1438 * locals.var_xitsb__blk1384_dn4)), ((locals.var_qim__blk1438_dn6 * locals.var_xitsb__blk1384) + (locals.var_qim__blk1438 * locals.var_xitsb__blk1384_dn6)), ((locals.var_qim__blk1438_dn7 * locals.var_xitsb__blk1384) + (locals.var_qim__blk1438 * locals.var_xitsb__blk1384_dn7)), ((locals.var_qim__blk1438_dn8 * locals.var_xitsb__blk1384) + (locals.var_qim__blk1438 * locals.var_xitsb__blk1384_dn8)), ((locals.var_qim__blk1438_dn9 * locals.var_xitsb__blk1384) + (locals.var_qim__blk1438 * locals.var_xitsb__blk1384_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign53160_e68604;
        locals.var_temp2_dn4 = assign53160_e68604_d_n4;
        locals.var_temp2_dn6 = assign53160_e68604_d_n6;
        locals.var_temp2_dn7 = assign53160_e68604_d_n7;
        locals.var_temp2_dn8 = assign53160_e68604_d_n8;
        locals.var_temp2_dn9 = assign53160_e68604_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign53170_e68614, assign53170_e68614_d_n4, assign53170_e68614_d_n6, assign53170_e68614_d_n7, assign53170_e68614_d_n8, assign53170_e68614_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53170_e68611: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign53170_e68612: f64 = (locals.var_temp2 / assign53170_e68611);
        (assign53170_e68612, (((locals.var_temp2_dn4 * assign53170_e68611) - (locals.var_temp2 * locals.var_temp2_dn4)) / (assign53170_e68611 * assign53170_e68611)), (((locals.var_temp2_dn6 * assign53170_e68611) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign53170_e68611 * assign53170_e68611)), (((locals.var_temp2_dn7 * assign53170_e68611) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign53170_e68611 * assign53170_e68611)), (((locals.var_temp2_dn8 * assign53170_e68611) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign53170_e68611 * assign53170_e68611)), (((locals.var_temp2_dn9 * assign53170_e68611) - (locals.var_temp2 * locals.var_temp2_dn9)) / (assign53170_e68611 * assign53170_e68611)),)
    } else {
        (locals.var_wsat__blk1385, locals.var_wsat__blk1385_dn4, locals.var_wsat__blk1385_dn6, locals.var_wsat__blk1385_dn7, locals.var_wsat__blk1385_dn8, locals.var_wsat__blk1385_dn9,)
    }
};
        locals.var_wsat__blk1385 = assign53170_e68614;
        locals.var_wsat__blk1385_dn4 = assign53170_e68614_d_n4;
        locals.var_wsat__blk1385_dn6 = assign53170_e68614_d_n6;
        locals.var_wsat__blk1385_dn7 = assign53170_e68614_d_n7;
        locals.var_wsat__blk1385_dn8 = assign53170_e68614_d_n8;
        locals.var_wsat__blk1385_dn9 = assign53170_e68614_d_n9;
        locals.var_wsat__blk1385_rv = 0.0;

        let assign53180_e68617: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1521 = assign53180_e68617;
        locals.var_guard1521_rv = 0.0;

        let (assign53190_e68631, assign53190_e68631_d_n4, assign53190_e68631_d_n6, assign53190_e68631_d_n7, assign53190_e68631_d_n8, assign53190_e68631_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1521 != 0.0)) {
        let assign53190_e68627: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1385);
        let assign53190_e68628: f64 = (1.0 - assign53190_e68627);
        let assign53190_e68629: f64 = (1.0 / assign53190_e68628);
        (assign53190_e68629, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn4)) / (assign53190_e68628 * assign53190_e68628))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn6)) / (assign53190_e68628 * assign53190_e68628))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn7)) / (assign53190_e68628 * assign53190_e68628))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn8)) / (assign53190_e68628 * assign53190_e68628))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn9)) / (assign53190_e68628 * assign53190_e68628))),)
    } else {
        (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9,)
    }
};
        locals.var_factheta__blk1386 = assign53190_e68631;
        locals.var_factheta__blk1386_dn4 = assign53190_e68631_d_n4;
        locals.var_factheta__blk1386_dn6 = assign53190_e68631_d_n6;
        locals.var_factheta__blk1386_dn7 = assign53190_e68631_d_n7;
        locals.var_factheta__blk1386_dn8 = assign53190_e68631_d_n8;
        locals.var_factheta__blk1386_dn9 = assign53190_e68631_d_n9;
        locals.var_factheta__blk1386_rv = 0.0;

        let (assign53200_e68644, assign53200_e68644_d_n4, assign53200_e68644_d_n6, assign53200_e68644_d_n7, assign53200_e68644_d_n8, assign53200_e68644_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1521 == 0.0)) {
        let assign53200_e68641: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1385);
        let assign53200_e68642: f64 = (1.0 + assign53200_e68641);
        (assign53200_e68642, (locals.var_thesatg_i * locals.var_wsat__blk1385_dn4), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn8), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn9),)
    } else {
        (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9,)
    }
};
        locals.var_factheta__blk1386 = assign53200_e68644;
        locals.var_factheta__blk1386_dn4 = assign53200_e68644_d_n4;
        locals.var_factheta__blk1386_dn6 = assign53200_e68644_d_n6;
        locals.var_factheta__blk1386_dn7 = assign53200_e68644_d_n7;
        locals.var_factheta__blk1386_dn8 = assign53200_e68644_d_n8;
        locals.var_factheta__blk1386_dn9 = assign53200_e68644_d_n9;
        locals.var_factheta__blk1386_rv = 0.0;

        let (assign53210_e68652, assign53210_e68652_d_n4, assign53210_e68652_d_n6, assign53210_e68652_d_n7, assign53210_e68652_d_n8, assign53210_e68652_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53210_e68650: f64 = (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386);
        (assign53210_e68650, ((locals.var_thesatloc__blk1319_dn4 * locals.var_factheta__blk1386) + (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn4)), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn6), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn7), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn8), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn9),)
    } else {
        (locals.var_thesateff__blk1447, locals.var_thesateff__blk1447_dn4, locals.var_thesateff__blk1447_dn6, locals.var_thesateff__blk1447_dn7, locals.var_thesateff__blk1447_dn8, locals.var_thesateff__blk1447_dn9,)
    }
};
        locals.var_thesateff__blk1447 = assign53210_e68652;
        locals.var_thesateff__blk1447_dn4 = assign53210_e68652_d_n4;
        locals.var_thesateff__blk1447_dn6 = assign53210_e68652_d_n6;
        locals.var_thesateff__blk1447_dn7 = assign53210_e68652_d_n7;
        locals.var_thesateff__blk1447_dn8 = assign53210_e68652_d_n8;
        locals.var_thesateff__blk1447_dn9 = assign53210_e68652_d_n9;
        locals.var_thesateff__blk1447_rv = 0.0;

        let (assign53220_e68660, assign53220_e68660_d_n4, assign53220_e68660_d_n6, assign53220_e68660_d_n7, assign53220_e68660_d_n8, assign53220_e68660_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign53220_e68658: f64 = (locals.var_xgm__blk1426 * locals.var_phit1__blk1339);
        (assign53220_e68658, ((locals.var_xgm__blk1426_dn4 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn4)), ((locals.var_xgm__blk1426_dn6 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn6)), ((locals.var_xgm__blk1426_dn7 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn7)), ((locals.var_xgm__blk1426_dn8 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn8)), ((locals.var_xgm__blk1426_dn9 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn9)),)
    } else {
        (locals.var_voxm__blk1446, locals.var_voxm__blk1446_dn4, locals.var_voxm__blk1446_dn6, locals.var_voxm__blk1446_dn7, locals.var_voxm__blk1446_dn8, locals.var_voxm__blk1446_dn9,)
    }
};
        locals.var_voxm__blk1446 = assign53220_e68660;
        locals.var_voxm__blk1446_dn4 = assign53220_e68660_d_n4;
        locals.var_voxm__blk1446_dn6 = assign53220_e68660_d_n6;
        locals.var_voxm__blk1446_dn7 = assign53220_e68660_d_n7;
        locals.var_voxm__blk1446_dn8 = assign53220_e68660_d_n8;
        locals.var_voxm__blk1446_dn9 = assign53220_e68660_d_n9;
        locals.var_voxm__blk1446_rv = 0.0;

        let (assign53230_e68664, assign53230_e68664_d_n4, assign53230_e68664_d_n6, assign53230_e68664_d_n7, assign53230_e68664_d_n8, assign53230_e68664_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_vgb1__blk1321, locals.var_vgb1__blk1321_dn4, locals.var_vgb1__blk1321_dn6, locals.var_vgb1__blk1321_dn7, locals.var_vgb1__blk1321_dn8, locals.var_vgb1__blk1321_dn9,)
    } else {
        (locals.var_vgb1_ac, locals.var_vgb1_ac_dn4, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, locals.var_vgb1_ac_dn9,)
    }
};
        locals.var_vgb1_ac = assign53230_e68664;
        locals.var_vgb1_ac_dn4 = assign53230_e68664_d_n4;
        locals.var_vgb1_ac_dn6 = assign53230_e68664_d_n6;
        locals.var_vgb1_ac_dn7 = assign53230_e68664_d_n7;
        locals.var_vgb1_ac_dn8 = assign53230_e68664_d_n8;
        locals.var_vgb1_ac_dn9 = assign53230_e68664_d_n9;
        locals.var_vgb1_ac_rv = 0.0;

        let (assign53240_e68668, assign53240_e68668_d_n4, assign53240_e68668_d_n6, assign53240_e68668_d_n7, assign53240_e68668_d_n8, assign53240_e68668_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_phit1__blk1339, locals.var_phit1__blk1339_dn4, locals.var_phit1__blk1339_dn6, locals.var_phit1__blk1339_dn7, locals.var_phit1__blk1339_dn8, locals.var_phit1__blk1339_dn9,)
    } else {
        (locals.var_phit1_ac, locals.var_phit1_ac_dn4, locals.var_phit1_ac_dn6, locals.var_phit1_ac_dn7, locals.var_phit1_ac_dn8, locals.var_phit1_ac_dn9,)
    }
};
        locals.var_phit1_ac = assign53240_e68668;
        locals.var_phit1_ac_dn4 = assign53240_e68668_d_n4;
        locals.var_phit1_ac_dn6 = assign53240_e68668_d_n6;
        locals.var_phit1_ac_dn7 = assign53240_e68668_d_n7;
        locals.var_phit1_ac_dn8 = assign53240_e68668_d_n8;
        locals.var_phit1_ac_dn9 = assign53240_e68668_d_n9;
        locals.var_phit1_ac_rv = 0.0;

        let (assign53250_e68672, assign53250_e68672_d_n4, assign53250_e68672_d_n6, assign53250_e68672_d_n7, assign53250_e68672_d_n8, assign53250_e68672_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_gf__blk1324, locals.var_gf__blk1324_dn4, locals.var_gf__blk1324_dn6, locals.var_gf__blk1324_dn7, locals.var_gf__blk1324_dn8, locals.var_gf__blk1324_dn9,)
    } else {
        (locals.var_gf_ac, locals.var_gf_ac_dn4, locals.var_gf_ac_dn6, locals.var_gf_ac_dn7, locals.var_gf_ac_dn8, locals.var_gf_ac_dn9,)
    }
};
        locals.var_gf_ac = assign53250_e68672;
        locals.var_gf_ac_dn4 = assign53250_e68672_d_n4;
        locals.var_gf_ac_dn6 = assign53250_e68672_d_n6;
        locals.var_gf_ac_dn7 = assign53250_e68672_d_n7;
        locals.var_gf_ac_dn8 = assign53250_e68672_d_n8;
        locals.var_gf_ac_dn9 = assign53250_e68672_d_n9;
        locals.var_gf_ac_rv = 0.0;

        let (assign53260_e68676, assign53260_e68676_d_n4, assign53260_e68676_d_n6, assign53260_e68676_d_n7, assign53260_e68676_d_n8, assign53260_e68676_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_xg__blk1343, locals.var_xg__blk1343_dn4, locals.var_xg__blk1343_dn6, locals.var_xg__blk1343_dn7, locals.var_xg__blk1343_dn8, locals.var_xg__blk1343_dn9,)
    } else {
        (locals.var_xg_ac, locals.var_xg_ac_dn4, locals.var_xg_ac_dn6, locals.var_xg_ac_dn7, locals.var_xg_ac_dn8, locals.var_xg_ac_dn9,)
    }
};
        locals.var_xg_ac = assign53260_e68676;
        locals.var_xg_ac_dn4 = assign53260_e68676_d_n4;
        locals.var_xg_ac_dn6 = assign53260_e68676_d_n6;
        locals.var_xg_ac_dn7 = assign53260_e68676_d_n7;
        locals.var_xg_ac_dn8 = assign53260_e68676_d_n8;
        locals.var_xg_ac_dn9 = assign53260_e68676_d_n9;
        locals.var_xg_ac_rv = 0.0;

        let (assign53270_e68680, assign53270_e68680_d_n4, assign53270_e68680_d_n6, assign53270_e68680_d_n7, assign53270_e68680_d_n8, assign53270_e68680_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_xno_s__blk1348, locals.var_xno_s__blk1348_dn4, locals.var_xno_s__blk1348_dn6, locals.var_xno_s__blk1348_dn7, locals.var_xno_s__blk1348_dn8, locals.var_xno_s__blk1348_dn9,)
    } else {
        (locals.var_xno_s_ac, locals.var_xno_s_ac_dn4, locals.var_xno_s_ac_dn6, locals.var_xno_s_ac_dn7, locals.var_xno_s_ac_dn8, locals.var_xno_s_ac_dn9,)
    }
};
        locals.var_xno_s_ac = assign53270_e68680;
        locals.var_xno_s_ac_dn4 = assign53270_e68680_d_n4;
        locals.var_xno_s_ac_dn6 = assign53270_e68680_d_n6;
        locals.var_xno_s_ac_dn7 = assign53270_e68680_d_n7;
        locals.var_xno_s_ac_dn8 = assign53270_e68680_d_n8;
        locals.var_xno_s_ac_dn9 = assign53270_e68680_d_n9;
        locals.var_xno_s_ac_rv = 0.0;

        let (assign53280_e68684, assign53280_e68684_d_n4, assign53280_e68684_d_n6, assign53280_e68684_d_n7, assign53280_e68684_d_n8, assign53280_e68684_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    } else {
        (locals.var_qbs_ac, locals.var_qbs_ac_dn4, locals.var_qbs_ac_dn6, locals.var_qbs_ac_dn7, locals.var_qbs_ac_dn8, locals.var_qbs_ac_dn9,)
    }
};
        locals.var_qbs_ac = assign53280_e68684;
        locals.var_qbs_ac_dn4 = assign53280_e68684_d_n4;
        locals.var_qbs_ac_dn6 = assign53280_e68684_d_n6;
        locals.var_qbs_ac_dn7 = assign53280_e68684_d_n7;
        locals.var_qbs_ac_dn8 = assign53280_e68684_d_n8;
        locals.var_qbs_ac_dn9 = assign53280_e68684_d_n9;
        locals.var_qbs_ac_rv = 0.0;

        let (assign53290_e68688, assign53290_e68688_d_n4, assign53290_e68688_d_n6, assign53290_e68688_d_n7, assign53290_e68688_d_n8, assign53290_e68688_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_dps__blk1414, locals.var_dps__blk1414_dn4, locals.var_dps__blk1414_dn6, locals.var_dps__blk1414_dn7, locals.var_dps__blk1414_dn8, locals.var_dps__blk1414_dn9,)
    } else {
        (locals.var_dps_ac, locals.var_dps_ac_dn4, locals.var_dps_ac_dn6, locals.var_dps_ac_dn7, locals.var_dps_ac_dn8, locals.var_dps_ac_dn9,)
    }
};
        locals.var_dps_ac = assign53290_e68688;
        locals.var_dps_ac_dn4 = assign53290_e68688_d_n4;
        locals.var_dps_ac_dn6 = assign53290_e68688_d_n6;
        locals.var_dps_ac_dn7 = assign53290_e68688_d_n7;
        locals.var_dps_ac_dn8 = assign53290_e68688_d_n8;
        locals.var_dps_ac_dn9 = assign53290_e68688_d_n9;
        locals.var_dps_ac_rv = 0.0;

        let (assign53300_e68692, assign53300_e68692_d_n4, assign53300_e68692_d_n6, assign53300_e68692_d_n7, assign53300_e68692_d_n8, assign53300_e68692_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qbd__blk1420, locals.var_qbd__blk1420_dn4, locals.var_qbd__blk1420_dn6, locals.var_qbd__blk1420_dn7, locals.var_qbd__blk1420_dn8, locals.var_qbd__blk1420_dn9,)
    } else {
        (locals.var_qbd_ac, locals.var_qbd_ac_dn4, locals.var_qbd_ac_dn6, locals.var_qbd_ac_dn7, locals.var_qbd_ac_dn8, locals.var_qbd_ac_dn9,)
    }
};
        locals.var_qbd_ac = assign53300_e68692;
        locals.var_qbd_ac_dn4 = assign53300_e68692_d_n4;
        locals.var_qbd_ac_dn6 = assign53300_e68692_d_n6;
        locals.var_qbd_ac_dn7 = assign53300_e68692_d_n7;
        locals.var_qbd_ac_dn8 = assign53300_e68692_d_n8;
        locals.var_qbd_ac_dn9 = assign53300_e68692_d_n9;
        locals.var_qbd_ac_rv = 0.0;

        let (assign53310_e68696, assign53310_e68696_d_n4, assign53310_e68696_d_n6, assign53310_e68696_d_n7, assign53310_e68696_d_n8, assign53310_e68696_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_eta_p__blk1427, locals.var_eta_p__blk1427_dn4, locals.var_eta_p__blk1427_dn6, locals.var_eta_p__blk1427_dn7, locals.var_eta_p__blk1427_dn8, locals.var_eta_p__blk1427_dn9,)
    } else {
        (locals.var_eta_p_ac, locals.var_eta_p_ac_dn4, locals.var_eta_p_ac_dn6, locals.var_eta_p_ac_dn7, locals.var_eta_p_ac_dn8, locals.var_eta_p_ac_dn9,)
    }
};
        locals.var_eta_p_ac = assign53310_e68696;
        locals.var_eta_p_ac_dn4 = assign53310_e68696_d_n4;
        locals.var_eta_p_ac_dn6 = assign53310_e68696_d_n6;
        locals.var_eta_p_ac_dn7 = assign53310_e68696_d_n7;
        locals.var_eta_p_ac_dn8 = assign53310_e68696_d_n8;
        locals.var_eta_p_ac_dn9 = assign53310_e68696_d_n9;
        locals.var_eta_p_ac_rv = 0.0;

        let (assign53320_e68700, assign53320_e68700_d_n4, assign53320_e68700_d_n6, assign53320_e68700_d_n7, assign53320_e68700_d_n8, assign53320_e68700_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_alpha__blk1429, locals.var_alpha__blk1429_dn4, locals.var_alpha__blk1429_dn6, locals.var_alpha__blk1429_dn7, locals.var_alpha__blk1429_dn8, locals.var_alpha__blk1429_dn9,)
    } else {
        (locals.var_alpha_ac, locals.var_alpha_ac_dn4, locals.var_alpha_ac_dn6, locals.var_alpha_ac_dn7, locals.var_alpha_ac_dn8, locals.var_alpha_ac_dn9,)
    }
};
        locals.var_alpha_ac = assign53320_e68700;
        locals.var_alpha_ac_dn4 = assign53320_e68700_d_n4;
        locals.var_alpha_ac_dn6 = assign53320_e68700_d_n6;
        locals.var_alpha_ac_dn7 = assign53320_e68700_d_n7;
        locals.var_alpha_ac_dn8 = assign53320_e68700_d_n8;
        locals.var_alpha_ac_dn9 = assign53320_e68700_d_n9;
        locals.var_alpha_ac_rv = 0.0;

        let (assign53330_e68704, assign53330_e68704_d_n4, assign53330_e68704_d_n6, assign53330_e68704_d_n7, assign53330_e68704_d_n8, assign53330_e68704_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qim__blk1438, locals.var_qim__blk1438_dn4, locals.var_qim__blk1438_dn6, locals.var_qim__blk1438_dn7, locals.var_qim__blk1438_dn8, locals.var_qim__blk1438_dn9,)
    } else {
        (locals.var_qim_ac, locals.var_qim_ac_dn4, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, locals.var_qim_ac_dn9,)
    }
};
        locals.var_qim_ac = assign53330_e68704;
        locals.var_qim_ac_dn4 = assign53330_e68704_d_n4;
        locals.var_qim_ac_dn6 = assign53330_e68704_d_n6;
        locals.var_qim_ac_dn7 = assign53330_e68704_d_n7;
        locals.var_qim_ac_dn8 = assign53330_e68704_d_n8;
        locals.var_qim_ac_dn9 = assign53330_e68704_d_n9;
        locals.var_qim_ac_rv = 0.0;

        let (assign53340_e68708, assign53340_e68708_d_n4, assign53340_e68708_d_n6, assign53340_e68708_d_n7, assign53340_e68708_d_n8, assign53340_e68708_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qim1__blk1439, locals.var_qim1__blk1439_dn4, locals.var_qim1__blk1439_dn6, locals.var_qim1__blk1439_dn7, locals.var_qim1__blk1439_dn8, locals.var_qim1__blk1439_dn9,)
    } else {
        (locals.var_qim1_ac, locals.var_qim1_ac_dn4, locals.var_qim1_ac_dn6, locals.var_qim1_ac_dn7, locals.var_qim1_ac_dn8, locals.var_qim1_ac_dn9,)
    }
};
        locals.var_qim1_ac = assign53340_e68708;
        locals.var_qim1_ac_dn4 = assign53340_e68708_d_n4;
        locals.var_qim1_ac_dn6 = assign53340_e68708_d_n6;
        locals.var_qim1_ac_dn7 = assign53340_e68708_d_n7;
        locals.var_qim1_ac_dn8 = assign53340_e68708_d_n8;
        locals.var_qim1_ac_dn9 = assign53340_e68708_d_n9;
        locals.var_qim1_ac_rv = 0.0;

        let (assign53350_e68712, assign53350_e68712_d_n4, assign53350_e68712_d_n6, assign53350_e68712_d_n7, assign53350_e68712_d_n8, assign53350_e68712_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qeff1__blk1442, locals.var_qeff1__blk1442_dn4, locals.var_qeff1__blk1442_dn6, locals.var_qeff1__blk1442_dn7, locals.var_qeff1__blk1442_dn8, locals.var_qeff1__blk1442_dn9,)
    } else {
        (locals.var_qeff1_ac, locals.var_qeff1_ac_dn4, locals.var_qeff1_ac_dn6, locals.var_qeff1_ac_dn7, locals.var_qeff1_ac_dn8, locals.var_qeff1_ac_dn9,)
    }
};
        locals.var_qeff1_ac = assign53350_e68712;
        locals.var_qeff1_ac_dn4 = assign53350_e68712_d_n4;
        locals.var_qeff1_ac_dn6 = assign53350_e68712_d_n6;
        locals.var_qeff1_ac_dn7 = assign53350_e68712_d_n7;
        locals.var_qeff1_ac_dn8 = assign53350_e68712_d_n8;
        locals.var_qeff1_ac_dn9 = assign53350_e68712_d_n9;
        locals.var_qeff1_ac_rv = 0.0;

        let (assign53360_e68716, assign53360_e68716_d_n4, assign53360_e68716_d_n6, assign53360_e68716_d_n7, assign53360_e68716_d_n8, assign53360_e68716_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_gmob__blk1444, locals.var_gmob__blk1444_dn4, locals.var_gmob__blk1444_dn6, locals.var_gmob__blk1444_dn7, locals.var_gmob__blk1444_dn8, locals.var_gmob__blk1444_dn9,)
    } else {
        (locals.var_gmob_ac, locals.var_gmob_ac_dn4, locals.var_gmob_ac_dn6, locals.var_gmob_ac_dn7, locals.var_gmob_ac_dn8, locals.var_gmob_ac_dn9,)
    }
};
        locals.var_gmob_ac = assign53360_e68716;
        locals.var_gmob_ac_dn4 = assign53360_e68716_d_n4;
        locals.var_gmob_ac_dn6 = assign53360_e68716_d_n6;
        locals.var_gmob_ac_dn7 = assign53360_e68716_d_n7;
        locals.var_gmob_ac_dn8 = assign53360_e68716_d_n8;
        locals.var_gmob_ac_dn9 = assign53360_e68716_d_n9;
        locals.var_gmob_ac_rv = 0.0;

        let (assign53370_e68720, assign53370_e68720_d_n4, assign53370_e68720_d_n6, assign53370_e68720_d_n7, assign53370_e68720_d_n8, assign53370_e68720_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_s1__blk1445, locals.var_s1__blk1445_dn4, locals.var_s1__blk1445_dn6, locals.var_s1__blk1445_dn7, locals.var_s1__blk1445_dn8, locals.var_s1__blk1445_dn9,)
    } else {
        (locals.var_s1_ac, locals.var_s1_ac_dn4, locals.var_s1_ac_dn6, locals.var_s1_ac_dn7, locals.var_s1_ac_dn8, locals.var_s1_ac_dn9,)
    }
};
        locals.var_s1_ac = assign53370_e68720;
        locals.var_s1_ac_dn4 = assign53370_e68720_d_n4;
        locals.var_s1_ac_dn6 = assign53370_e68720_d_n6;
        locals.var_s1_ac_dn7 = assign53370_e68720_d_n7;
        locals.var_s1_ac_dn8 = assign53370_e68720_d_n8;
        locals.var_s1_ac_dn9 = assign53370_e68720_d_n9;
        locals.var_s1_ac_rv = 0.0;

        let (assign53380_e68724, assign53380_e68724_d_n4, assign53380_e68724_d_n6, assign53380_e68724_d_n7, assign53380_e68724_d_n8, assign53380_e68724_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_thesateff__blk1447, locals.var_thesateff__blk1447_dn4, locals.var_thesateff__blk1447_dn6, locals.var_thesateff__blk1447_dn7, locals.var_thesateff__blk1447_dn8, locals.var_thesateff__blk1447_dn9,)
    } else {
        (locals.var_thesateff_ac, locals.var_thesateff_ac_dn4, locals.var_thesateff_ac_dn6, locals.var_thesateff_ac_dn7, locals.var_thesateff_ac_dn8, locals.var_thesateff_ac_dn9,)
    }
};
        locals.var_thesateff_ac = assign53380_e68724;
        locals.var_thesateff_ac_dn4 = assign53380_e68724_d_n4;
        locals.var_thesateff_ac_dn6 = assign53380_e68724_d_n6;
        locals.var_thesateff_ac_dn7 = assign53380_e68724_d_n7;
        locals.var_thesateff_ac_dn8 = assign53380_e68724_d_n8;
        locals.var_thesateff_ac_dn9 = assign53380_e68724_d_n9;
        locals.var_thesateff_ac_rv = 0.0;

        let (assign53390_e68728, assign53390_e68728_d_n4, assign53390_e68728_d_n6, assign53390_e68728_d_n7, assign53390_e68728_d_n8, assign53390_e68728_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_voxm__blk1446, locals.var_voxm__blk1446_dn4, locals.var_voxm__blk1446_dn6, locals.var_voxm__blk1446_dn7, locals.var_voxm__blk1446_dn8, locals.var_voxm__blk1446_dn9,)
    } else {
        (locals.var_voxm_ac, locals.var_voxm_ac_dn4, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8, locals.var_voxm_ac_dn9,)
    }
};
        locals.var_voxm_ac = assign53390_e68728;
        locals.var_voxm_ac_dn4 = assign53390_e68728_d_n4;
        locals.var_voxm_ac_dn6 = assign53390_e68728_d_n6;
        locals.var_voxm_ac_dn7 = assign53390_e68728_d_n7;
        locals.var_voxm_ac_dn8 = assign53390_e68728_d_n8;
        locals.var_voxm_ac_dn9 = assign53390_e68728_d_n9;
        locals.var_voxm_ac_rv = 0.0;

        let (assign53400_e68733, assign53400_e68733_d_n4,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_phib_dc, locals.var_phib_dc_dn4,)
    } else {
        (locals.var_phib_ac, locals.var_phib_ac_dn4,)
    }
};
        locals.var_phib_ac = assign53400_e68733;
        locals.var_phib_ac_dn4 = assign53400_e68733_d_n4;
        locals.var_phib_ac_rv = 0.0;

        let (assign53410_e68738, assign53410_e68738_d_n4, assign53410_e68738_d_n6, assign53410_e68738_d_n7, assign53410_e68738_d_n8, assign53410_e68738_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_vgb1_dc, locals.var_vgb1_dc_dn4, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8, locals.var_vgb1_dc_dn9,)
    } else {
        (locals.var_vgb1_ac, locals.var_vgb1_ac_dn4, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, locals.var_vgb1_ac_dn9,)
    }
};
        locals.var_vgb1_ac = assign53410_e68738;
        locals.var_vgb1_ac_dn4 = assign53410_e68738_d_n4;
        locals.var_vgb1_ac_dn6 = assign53410_e68738_d_n6;
        locals.var_vgb1_ac_dn7 = assign53410_e68738_d_n7;
        locals.var_vgb1_ac_dn8 = assign53410_e68738_d_n8;
        locals.var_vgb1_ac_dn9 = assign53410_e68738_d_n9;
        locals.var_vgb1_ac_rv = 0.0;

        let (assign53420_e68743, assign53420_e68743_d_n4, assign53420_e68743_d_n6, assign53420_e68743_d_n7, assign53420_e68743_d_n8, assign53420_e68743_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_phit1_dc, locals.var_phit1_dc_dn4, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8, locals.var_phit1_dc_dn9,)
    } else {
        (locals.var_phit1_ac, locals.var_phit1_ac_dn4, locals.var_phit1_ac_dn6, locals.var_phit1_ac_dn7, locals.var_phit1_ac_dn8, locals.var_phit1_ac_dn9,)
    }
};
        locals.var_phit1_ac = assign53420_e68743;
        locals.var_phit1_ac_dn4 = assign53420_e68743_d_n4;
        locals.var_phit1_ac_dn6 = assign53420_e68743_d_n6;
        locals.var_phit1_ac_dn7 = assign53420_e68743_d_n7;
        locals.var_phit1_ac_dn8 = assign53420_e68743_d_n8;
        locals.var_phit1_ac_dn9 = assign53420_e68743_d_n9;
        locals.var_phit1_ac_rv = 0.0;

        let (assign53430_e68748, assign53430_e68748_d_n4, assign53430_e68748_d_n6, assign53430_e68748_d_n7, assign53430_e68748_d_n8, assign53430_e68748_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_gf_dc, locals.var_gf_dc_dn4, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8, locals.var_gf_dc_dn9,)
    } else {
        (locals.var_gf_ac, locals.var_gf_ac_dn4, locals.var_gf_ac_dn6, locals.var_gf_ac_dn7, locals.var_gf_ac_dn8, locals.var_gf_ac_dn9,)
    }
};
        locals.var_gf_ac = assign53430_e68748;
        locals.var_gf_ac_dn4 = assign53430_e68748_d_n4;
        locals.var_gf_ac_dn6 = assign53430_e68748_d_n6;
        locals.var_gf_ac_dn7 = assign53430_e68748_d_n7;
        locals.var_gf_ac_dn8 = assign53430_e68748_d_n8;
        locals.var_gf_ac_dn9 = assign53430_e68748_d_n9;
        locals.var_gf_ac_rv = 0.0;

        let (assign53440_e68753, assign53440_e68753_d_n4, assign53440_e68753_d_n6, assign53440_e68753_d_n7, assign53440_e68753_d_n8, assign53440_e68753_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_xg_dc, locals.var_xg_dc_dn4, locals.var_xg_dc_dn6, locals.var_xg_dc_dn7, locals.var_xg_dc_dn8, locals.var_xg_dc_dn9,)
    } else {
        (locals.var_xg_ac, locals.var_xg_ac_dn4, locals.var_xg_ac_dn6, locals.var_xg_ac_dn7, locals.var_xg_ac_dn8, locals.var_xg_ac_dn9,)
    }
};
        locals.var_xg_ac = assign53440_e68753;
        locals.var_xg_ac_dn4 = assign53440_e68753_d_n4;
        locals.var_xg_ac_dn6 = assign53440_e68753_d_n6;
        locals.var_xg_ac_dn7 = assign53440_e68753_d_n7;
        locals.var_xg_ac_dn8 = assign53440_e68753_d_n8;
        locals.var_xg_ac_dn9 = assign53440_e68753_d_n9;
        locals.var_xg_ac_rv = 0.0;

        let (assign53450_e68758, assign53450_e68758_d_n4, assign53450_e68758_d_n6, assign53450_e68758_d_n7, assign53450_e68758_d_n8, assign53450_e68758_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_xno_s_dc, locals.var_xno_s_dc_dn4, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8, locals.var_xno_s_dc_dn9,)
    } else {
        (locals.var_xno_s_ac, locals.var_xno_s_ac_dn4, locals.var_xno_s_ac_dn6, locals.var_xno_s_ac_dn7, locals.var_xno_s_ac_dn8, locals.var_xno_s_ac_dn9,)
    }
};
        locals.var_xno_s_ac = assign53450_e68758;
        locals.var_xno_s_ac_dn4 = assign53450_e68758_d_n4;
        locals.var_xno_s_ac_dn6 = assign53450_e68758_d_n6;
        locals.var_xno_s_ac_dn7 = assign53450_e68758_d_n7;
        locals.var_xno_s_ac_dn8 = assign53450_e68758_d_n8;
        locals.var_xno_s_ac_dn9 = assign53450_e68758_d_n9;
        locals.var_xno_s_ac_rv = 0.0;

        let (assign53460_e68763, assign53460_e68763_d_n4, assign53460_e68763_d_n6, assign53460_e68763_d_n7, assign53460_e68763_d_n8, assign53460_e68763_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_qbs_dc, locals.var_qbs_dc_dn4, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8, locals.var_qbs_dc_dn9,)
    } else {
        (locals.var_qbs_ac, locals.var_qbs_ac_dn4, locals.var_qbs_ac_dn6, locals.var_qbs_ac_dn7, locals.var_qbs_ac_dn8, locals.var_qbs_ac_dn9,)
    }
};
        locals.var_qbs_ac = assign53460_e68763;
        locals.var_qbs_ac_dn4 = assign53460_e68763_d_n4;
        locals.var_qbs_ac_dn6 = assign53460_e68763_d_n6;
        locals.var_qbs_ac_dn7 = assign53460_e68763_d_n7;
        locals.var_qbs_ac_dn8 = assign53460_e68763_d_n8;
        locals.var_qbs_ac_dn9 = assign53460_e68763_d_n9;
        locals.var_qbs_ac_rv = 0.0;

        let (assign53470_e68768, assign53470_e68768_d_n4, assign53470_e68768_d_n6, assign53470_e68768_d_n7, assign53470_e68768_d_n8, assign53470_e68768_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_dps_dc, locals.var_dps_dc_dn4, locals.var_dps_dc_dn6, locals.var_dps_dc_dn7, locals.var_dps_dc_dn8, locals.var_dps_dc_dn9,)
    } else {
        (locals.var_dps_ac, locals.var_dps_ac_dn4, locals.var_dps_ac_dn6, locals.var_dps_ac_dn7, locals.var_dps_ac_dn8, locals.var_dps_ac_dn9,)
    }
};
        locals.var_dps_ac = assign53470_e68768;
        locals.var_dps_ac_dn4 = assign53470_e68768_d_n4;
        locals.var_dps_ac_dn6 = assign53470_e68768_d_n6;
        locals.var_dps_ac_dn7 = assign53470_e68768_d_n7;
        locals.var_dps_ac_dn8 = assign53470_e68768_d_n8;
        locals.var_dps_ac_dn9 = assign53470_e68768_d_n9;
        locals.var_dps_ac_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_53(
        locals: &mut StampLocals,
    ) {
        let (assign53480_e68773, assign53480_e68773_d_n4, assign53480_e68773_d_n6, assign53480_e68773_d_n7, assign53480_e68773_d_n8, assign53480_e68773_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_qbd_dc, locals.var_qbd_dc_dn4, locals.var_qbd_dc_dn6, locals.var_qbd_dc_dn7, locals.var_qbd_dc_dn8, locals.var_qbd_dc_dn9,)
    } else {
        (locals.var_qbd_ac, locals.var_qbd_ac_dn4, locals.var_qbd_ac_dn6, locals.var_qbd_ac_dn7, locals.var_qbd_ac_dn8, locals.var_qbd_ac_dn9,)
    }
};
        locals.var_qbd_ac = assign53480_e68773;
        locals.var_qbd_ac_dn4 = assign53480_e68773_d_n4;
        locals.var_qbd_ac_dn6 = assign53480_e68773_d_n6;
        locals.var_qbd_ac_dn7 = assign53480_e68773_d_n7;
        locals.var_qbd_ac_dn8 = assign53480_e68773_d_n8;
        locals.var_qbd_ac_dn9 = assign53480_e68773_d_n9;
        locals.var_qbd_ac_rv = 0.0;

        let (assign53490_e68778, assign53490_e68778_d_n4, assign53490_e68778_d_n6, assign53490_e68778_d_n7, assign53490_e68778_d_n8, assign53490_e68778_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_eta_p_dc, locals.var_eta_p_dc_dn4, locals.var_eta_p_dc_dn6, locals.var_eta_p_dc_dn7, locals.var_eta_p_dc_dn8, locals.var_eta_p_dc_dn9,)
    } else {
        (locals.var_eta_p_ac, locals.var_eta_p_ac_dn4, locals.var_eta_p_ac_dn6, locals.var_eta_p_ac_dn7, locals.var_eta_p_ac_dn8, locals.var_eta_p_ac_dn9,)
    }
};
        locals.var_eta_p_ac = assign53490_e68778;
        locals.var_eta_p_ac_dn4 = assign53490_e68778_d_n4;
        locals.var_eta_p_ac_dn6 = assign53490_e68778_d_n6;
        locals.var_eta_p_ac_dn7 = assign53490_e68778_d_n7;
        locals.var_eta_p_ac_dn8 = assign53490_e68778_d_n8;
        locals.var_eta_p_ac_dn9 = assign53490_e68778_d_n9;
        locals.var_eta_p_ac_rv = 0.0;

        let (assign53500_e68783, assign53500_e68783_d_n4, assign53500_e68783_d_n6, assign53500_e68783_d_n7, assign53500_e68783_d_n8, assign53500_e68783_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_alpha_dc, locals.var_alpha_dc_dn4, locals.var_alpha_dc_dn6, locals.var_alpha_dc_dn7, locals.var_alpha_dc_dn8, locals.var_alpha_dc_dn9,)
    } else {
        (locals.var_alpha_ac, locals.var_alpha_ac_dn4, locals.var_alpha_ac_dn6, locals.var_alpha_ac_dn7, locals.var_alpha_ac_dn8, locals.var_alpha_ac_dn9,)
    }
};
        locals.var_alpha_ac = assign53500_e68783;
        locals.var_alpha_ac_dn4 = assign53500_e68783_d_n4;
        locals.var_alpha_ac_dn6 = assign53500_e68783_d_n6;
        locals.var_alpha_ac_dn7 = assign53500_e68783_d_n7;
        locals.var_alpha_ac_dn8 = assign53500_e68783_d_n8;
        locals.var_alpha_ac_dn9 = assign53500_e68783_d_n9;
        locals.var_alpha_ac_rv = 0.0;

        let (assign53510_e68788, assign53510_e68788_d_n4, assign53510_e68788_d_n6, assign53510_e68788_d_n7, assign53510_e68788_d_n8, assign53510_e68788_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_qim_dc, locals.var_qim_dc_dn4, locals.var_qim_dc_dn6, locals.var_qim_dc_dn7, locals.var_qim_dc_dn8, locals.var_qim_dc_dn9,)
    } else {
        (locals.var_qim_ac, locals.var_qim_ac_dn4, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, locals.var_qim_ac_dn9,)
    }
};
        locals.var_qim_ac = assign53510_e68788;
        locals.var_qim_ac_dn4 = assign53510_e68788_d_n4;
        locals.var_qim_ac_dn6 = assign53510_e68788_d_n6;
        locals.var_qim_ac_dn7 = assign53510_e68788_d_n7;
        locals.var_qim_ac_dn8 = assign53510_e68788_d_n8;
        locals.var_qim_ac_dn9 = assign53510_e68788_d_n9;
        locals.var_qim_ac_rv = 0.0;

        let (assign53520_e68793, assign53520_e68793_d_n4, assign53520_e68793_d_n6, assign53520_e68793_d_n7, assign53520_e68793_d_n8, assign53520_e68793_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_qim1_dc, locals.var_qim1_dc_dn4, locals.var_qim1_dc_dn6, locals.var_qim1_dc_dn7, locals.var_qim1_dc_dn8, locals.var_qim1_dc_dn9,)
    } else {
        (locals.var_qim1_ac, locals.var_qim1_ac_dn4, locals.var_qim1_ac_dn6, locals.var_qim1_ac_dn7, locals.var_qim1_ac_dn8, locals.var_qim1_ac_dn9,)
    }
};
        locals.var_qim1_ac = assign53520_e68793;
        locals.var_qim1_ac_dn4 = assign53520_e68793_d_n4;
        locals.var_qim1_ac_dn6 = assign53520_e68793_d_n6;
        locals.var_qim1_ac_dn7 = assign53520_e68793_d_n7;
        locals.var_qim1_ac_dn8 = assign53520_e68793_d_n8;
        locals.var_qim1_ac_dn9 = assign53520_e68793_d_n9;
        locals.var_qim1_ac_rv = 0.0;

        let (assign53530_e68798, assign53530_e68798_d_n4, assign53530_e68798_d_n6, assign53530_e68798_d_n7, assign53530_e68798_d_n8, assign53530_e68798_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_qeff1_dc, locals.var_qeff1_dc_dn4, locals.var_qeff1_dc_dn6, locals.var_qeff1_dc_dn7, locals.var_qeff1_dc_dn8, locals.var_qeff1_dc_dn9,)
    } else {
        (locals.var_qeff1_ac, locals.var_qeff1_ac_dn4, locals.var_qeff1_ac_dn6, locals.var_qeff1_ac_dn7, locals.var_qeff1_ac_dn8, locals.var_qeff1_ac_dn9,)
    }
};
        locals.var_qeff1_ac = assign53530_e68798;
        locals.var_qeff1_ac_dn4 = assign53530_e68798_d_n4;
        locals.var_qeff1_ac_dn6 = assign53530_e68798_d_n6;
        locals.var_qeff1_ac_dn7 = assign53530_e68798_d_n7;
        locals.var_qeff1_ac_dn8 = assign53530_e68798_d_n8;
        locals.var_qeff1_ac_dn9 = assign53530_e68798_d_n9;
        locals.var_qeff1_ac_rv = 0.0;

        let (assign53540_e68803, assign53540_e68803_d_n4, assign53540_e68803_d_n6, assign53540_e68803_d_n7, assign53540_e68803_d_n8, assign53540_e68803_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_gmob_dc, locals.var_gmob_dc_dn4, locals.var_gmob_dc_dn6, locals.var_gmob_dc_dn7, locals.var_gmob_dc_dn8, locals.var_gmob_dc_dn9,)
    } else {
        (locals.var_gmob_ac, locals.var_gmob_ac_dn4, locals.var_gmob_ac_dn6, locals.var_gmob_ac_dn7, locals.var_gmob_ac_dn8, locals.var_gmob_ac_dn9,)
    }
};
        locals.var_gmob_ac = assign53540_e68803;
        locals.var_gmob_ac_dn4 = assign53540_e68803_d_n4;
        locals.var_gmob_ac_dn6 = assign53540_e68803_d_n6;
        locals.var_gmob_ac_dn7 = assign53540_e68803_d_n7;
        locals.var_gmob_ac_dn8 = assign53540_e68803_d_n8;
        locals.var_gmob_ac_dn9 = assign53540_e68803_d_n9;
        locals.var_gmob_ac_rv = 0.0;

        let (assign53550_e68808, assign53550_e68808_d_n4, assign53550_e68808_d_n6, assign53550_e68808_d_n7, assign53550_e68808_d_n8, assign53550_e68808_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_s1_dc, locals.var_s1_dc_dn4, locals.var_s1_dc_dn6, locals.var_s1_dc_dn7, locals.var_s1_dc_dn8, locals.var_s1_dc_dn9,)
    } else {
        (locals.var_s1_ac, locals.var_s1_ac_dn4, locals.var_s1_ac_dn6, locals.var_s1_ac_dn7, locals.var_s1_ac_dn8, locals.var_s1_ac_dn9,)
    }
};
        locals.var_s1_ac = assign53550_e68808;
        locals.var_s1_ac_dn4 = assign53550_e68808_d_n4;
        locals.var_s1_ac_dn6 = assign53550_e68808_d_n6;
        locals.var_s1_ac_dn7 = assign53550_e68808_d_n7;
        locals.var_s1_ac_dn8 = assign53550_e68808_d_n8;
        locals.var_s1_ac_dn9 = assign53550_e68808_d_n9;
        locals.var_s1_ac_rv = 0.0;

        let (assign53560_e68813, assign53560_e68813_d_n4, assign53560_e68813_d_n6, assign53560_e68813_d_n7, assign53560_e68813_d_n8, assign53560_e68813_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_thesateff_dc, locals.var_thesateff_dc_dn4, locals.var_thesateff_dc_dn6, locals.var_thesateff_dc_dn7, locals.var_thesateff_dc_dn8, locals.var_thesateff_dc_dn9,)
    } else {
        (locals.var_thesateff_ac, locals.var_thesateff_ac_dn4, locals.var_thesateff_ac_dn6, locals.var_thesateff_ac_dn7, locals.var_thesateff_ac_dn8, locals.var_thesateff_ac_dn9,)
    }
};
        locals.var_thesateff_ac = assign53560_e68813;
        locals.var_thesateff_ac_dn4 = assign53560_e68813_d_n4;
        locals.var_thesateff_ac_dn6 = assign53560_e68813_d_n6;
        locals.var_thesateff_ac_dn7 = assign53560_e68813_d_n7;
        locals.var_thesateff_ac_dn8 = assign53560_e68813_d_n8;
        locals.var_thesateff_ac_dn9 = assign53560_e68813_d_n9;
        locals.var_thesateff_ac_rv = 0.0;

        let (assign53570_e68818, assign53570_e68818_d_n4, assign53570_e68818_d_n6, assign53570_e68818_d_n7, assign53570_e68818_d_n8, assign53570_e68818_d_n9,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_voxm_dc, locals.var_voxm_dc_dn4, locals.var_voxm_dc_dn6, locals.var_voxm_dc_dn7, locals.var_voxm_dc_dn8, locals.var_voxm_dc_dn9,)
    } else {
        (locals.var_voxm_ac, locals.var_voxm_ac_dn4, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8, locals.var_voxm_ac_dn9,)
    }
};
        locals.var_voxm_ac = assign53570_e68818;
        locals.var_voxm_ac_dn4 = assign53570_e68818_d_n4;
        locals.var_voxm_ac_dn6 = assign53570_e68818_d_n6;
        locals.var_voxm_ac_dn7 = assign53570_e68818_d_n7;
        locals.var_voxm_ac_dn8 = assign53570_e68818_d_n8;
        locals.var_voxm_ac_dn9 = assign53570_e68818_d_n9;
        locals.var_voxm_ac_rv = 0.0;

        locals.var_cox_qm = locals.var_cox_i;
        locals.var_cox_qm_dn4 = 0.0;
        locals.var_cox_qm_dn6 = 0.0;
        locals.var_cox_qm_dn7 = 0.0;
        locals.var_cox_qm_dn8 = 0.0;
        locals.var_cox_qm_dn9 = 0.0;
        locals.var_cox_qm_rv = 0.0;

        let assign53600_e68827: f64 = if locals.var_qq > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1522 = assign53600_e68827;
        locals.var_guard1522_rv = 0.0;

        let (assign53610_e68846, assign53610_e68846_d_n4, assign53610_e68846_d_n6, assign53610_e68846_d_n7, assign53610_e68846_d_n8, assign53610_e68846_d_n9,) = {
    if (locals.var_guard1522 != 0.0) {
        let assign53610_e68834: f64 = (locals.var_qeff1_ac * locals.var_qeff1_ac);
        let assign53610_e68836: f64 = (assign53610_e68834 + locals.var_qlim2);
        let assign53610_e68838: f64 = (-1.0);
        let assign53610_e68840: f64 = (assign53610_e68838 * 0.16666666666666666);
        let assign53610_e68841: f64 = (assign53610_e68836).powf(assign53610_e68840);
        let assign53610_e68842: f64 = (locals.var_qq * assign53610_e68841);
        let assign53610_e68843: f64 = (1.0 + assign53610_e68842);
        let assign53610_e68844: f64 = (locals.var_cox_i / assign53610_e68843);
        (assign53610_e68844, (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53610_e68840) as f64).is_finite() && ((assign53610_e68840) as f64).fract() == 0.0 { if assign53610_e68840 == 0.0 { 0.0 } else { (assign53610_e68840 * ((assign53610_e68836).powf(assign53610_e68840 - 1.0) * (((locals.var_qeff1_ac_dn4 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn4)) + locals.var_qlim2_dn4))) } } else { (assign53610_e68841 * (assign53610_e68840 * ((((locals.var_qeff1_ac_dn4 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn4)) + locals.var_qlim2_dn4) / assign53610_e68836))) })) / (assign53610_e68843 * assign53610_e68843))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53610_e68840) as f64).is_finite() && ((assign53610_e68840) as f64).fract() == 0.0 { if assign53610_e68840 == 0.0 { 0.0 } else { (assign53610_e68840 * ((assign53610_e68836).powf(assign53610_e68840 - 1.0) * ((locals.var_qeff1_ac_dn6 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn6)))) } } else { (assign53610_e68841 * (assign53610_e68840 * (((locals.var_qeff1_ac_dn6 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn6)) / assign53610_e68836))) })) / (assign53610_e68843 * assign53610_e68843))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53610_e68840) as f64).is_finite() && ((assign53610_e68840) as f64).fract() == 0.0 { if assign53610_e68840 == 0.0 { 0.0 } else { (assign53610_e68840 * ((assign53610_e68836).powf(assign53610_e68840 - 1.0) * ((locals.var_qeff1_ac_dn7 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn7)))) } } else { (assign53610_e68841 * (assign53610_e68840 * (((locals.var_qeff1_ac_dn7 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn7)) / assign53610_e68836))) })) / (assign53610_e68843 * assign53610_e68843))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53610_e68840) as f64).is_finite() && ((assign53610_e68840) as f64).fract() == 0.0 { if assign53610_e68840 == 0.0 { 0.0 } else { (assign53610_e68840 * ((assign53610_e68836).powf(assign53610_e68840 - 1.0) * ((locals.var_qeff1_ac_dn8 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn8)))) } } else { (assign53610_e68841 * (assign53610_e68840 * (((locals.var_qeff1_ac_dn8 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn8)) / assign53610_e68836))) })) / (assign53610_e68843 * assign53610_e68843))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53610_e68840) as f64).is_finite() && ((assign53610_e68840) as f64).fract() == 0.0 { if assign53610_e68840 == 0.0 { 0.0 } else { (assign53610_e68840 * ((assign53610_e68836).powf(assign53610_e68840 - 1.0) * ((locals.var_qeff1_ac_dn9 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn9)))) } } else { (assign53610_e68841 * (assign53610_e68840 * (((locals.var_qeff1_ac_dn9 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn9)) / assign53610_e68836))) })) / (assign53610_e68843 * assign53610_e68843))),)
    } else {
        (locals.var_cox_qm, locals.var_cox_qm_dn4, locals.var_cox_qm_dn6, locals.var_cox_qm_dn7, locals.var_cox_qm_dn8, locals.var_cox_qm_dn9,)
    }
};
        locals.var_cox_qm = assign53610_e68846;
        locals.var_cox_qm_dn4 = assign53610_e68846_d_n4;
        locals.var_cox_qm_dn6 = assign53610_e68846_d_n6;
        locals.var_cox_qm_dn7 = assign53610_e68846_d_n7;
        locals.var_cox_qm_dn8 = assign53610_e68846_d_n8;
        locals.var_cox_qm_dn9 = assign53610_e68846_d_n9;
        locals.var_cox_qm_rv = 0.0;

        locals.var_gdl_ac = 1.0;
        locals.var_gdl_ac_dn4 = 0.0;
        locals.var_gdl_ac_dn6 = 0.0;
        locals.var_gdl_ac_dn7 = 0.0;
        locals.var_gdl_ac_dn8 = 0.0;
        locals.var_gdl_ac_dn9 = 0.0;
        locals.var_gdl_ac_rv = 0.0;

        locals.var_gmob_dl_ac = 1.0;
        locals.var_gmob_dl_ac_dn4 = 0.0;
        locals.var_gmob_dl_ac_dn6 = 0.0;
        locals.var_gmob_dl_ac_dn7 = 0.0;
        locals.var_gmob_dl_ac_dn8 = 0.0;
        locals.var_gmob_dl_ac_dn9 = 0.0;
        locals.var_gmob_dl_ac_rv = 0.0;

        locals.var_thesat1_ac = 0.0;
        locals.var_thesat1_ac_dn4 = 0.0;
        locals.var_thesat1_ac_dn6 = 0.0;
        locals.var_thesat1_ac_dn7 = 0.0;
        locals.var_thesat1_ac_dn8 = 0.0;
        locals.var_thesat1_ac_dn9 = 0.0;
        locals.var_thesat1_ac_rv = 0.0;

        locals.var_gvsat_ac = 1.0;
        locals.var_gvsat_ac_dn4 = 0.0;
        locals.var_gvsat_ac_dn6 = 0.0;
        locals.var_gvsat_ac_dn7 = 0.0;
        locals.var_gvsat_ac_dn8 = 0.0;
        locals.var_gvsat_ac_dn9 = 0.0;
        locals.var_gvsat_ac_rv = 0.0;

        locals.var_h_ac = 1.0;
        locals.var_h_ac_dn4 = 0.0;
        locals.var_h_ac_dn6 = 0.0;
        locals.var_h_ac_dn7 = 0.0;
        locals.var_h_ac_dn8 = 0.0;
        locals.var_h_ac_dn9 = 0.0;
        locals.var_h_ac_rv = 0.0;

        locals.var_qg_1 = locals.var_voxm_ac;
        locals.var_qg_1_dn4 = locals.var_voxm_ac_dn4;
        locals.var_qg_1_dn6 = locals.var_voxm_ac_dn6;
        locals.var_qg_1_dn7 = locals.var_voxm_ac_dn7;
        locals.var_qg_1_dn8 = locals.var_voxm_ac_dn8;
        locals.var_qg_1_dn9 = locals.var_voxm_ac_dn9;
        locals.var_qg_1_rv = 0.0;

        locals.var_qi = 0.0;
        locals.var_qi_dn4 = 0.0;
        locals.var_qi_dn6 = 0.0;
        locals.var_qi_dn7 = 0.0;
        locals.var_qi_dn8 = 0.0;
        locals.var_qi_dn9 = 0.0;
        locals.var_qi_rv = 0.0;

        locals.var_qd_1 = 0.0;
        locals.var_qd_1_dn4 = 0.0;
        locals.var_qd_1_dn6 = 0.0;
        locals.var_qd_1_dn7 = 0.0;
        locals.var_qd_1_dn8 = 0.0;
        locals.var_qd_1_dn9 = 0.0;
        locals.var_qd_1_rv = 0.0;

        locals.var_qb_1 = locals.var_qg_1;
        locals.var_qb_1_dn4 = locals.var_qg_1_dn4;
        locals.var_qb_1_dn6 = locals.var_qg_1_dn6;
        locals.var_qb_1_dn7 = locals.var_qg_1_dn7;
        locals.var_qb_1_dn8 = locals.var_qg_1_dn8;
        locals.var_qb_1_dn9 = locals.var_qg_1_dn9;
        locals.var_qb_1_rv = 0.0;

        let assign53710_e68858: f64 = if locals.var_xg_ac > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1523 = assign53710_e68858;
        locals.var_guard1523_rv = 0.0;

        let (assign53720_e68872, assign53720_e68872_d_n4, assign53720_e68872_d_n6, assign53720_e68872_d_n7, assign53720_e68872_d_n8, assign53720_e68872_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53720_e68863: f64 = (locals.var_alp1ac_i / locals.var_qim1_ac);
        let assign53720_e68864: f64 = (locals.var_alpac_i + assign53720_e68863);
        let assign53720_e68866: f64 = (assign53720_e68864 * locals.var_qim_ac);
        let assign53720_e68868: f64 = (assign53720_e68866 / locals.var_qim1_ac);
        let assign53720_e68870: f64 = (assign53720_e68868 * locals.var_s1_ac);
        (assign53720_e68870, ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn4) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53720_e68864 * locals.var_qim_ac_dn4)) * locals.var_qim1_ac) - (assign53720_e68866 * locals.var_qim1_ac_dn4)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53720_e68868 * locals.var_s1_ac_dn4)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn6) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53720_e68864 * locals.var_qim_ac_dn6)) * locals.var_qim1_ac) - (assign53720_e68866 * locals.var_qim1_ac_dn6)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53720_e68868 * locals.var_s1_ac_dn6)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn7) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53720_e68864 * locals.var_qim_ac_dn7)) * locals.var_qim1_ac) - (assign53720_e68866 * locals.var_qim1_ac_dn7)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53720_e68868 * locals.var_s1_ac_dn7)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn8) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53720_e68864 * locals.var_qim_ac_dn8)) * locals.var_qim1_ac) - (assign53720_e68866 * locals.var_qim1_ac_dn8)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53720_e68868 * locals.var_s1_ac_dn8)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn9) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53720_e68864 * locals.var_qim_ac_dn9)) * locals.var_qim1_ac) - (assign53720_e68866 * locals.var_qim1_ac_dn9)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53720_e68868 * locals.var_s1_ac_dn9)),)
    } else {
        (locals.var_dl__blk1280, locals.var_dl__blk1280_dn4, locals.var_dl__blk1280_dn6, locals.var_dl__blk1280_dn7, locals.var_dl__blk1280_dn8, locals.var_dl__blk1280_dn9,)
    }
};
        locals.var_dl__blk1280 = assign53720_e68872;
        locals.var_dl__blk1280_dn4 = assign53720_e68872_d_n4;
        locals.var_dl__blk1280_dn6 = assign53720_e68872_d_n6;
        locals.var_dl__blk1280_dn7 = assign53720_e68872_d_n7;
        locals.var_dl__blk1280_dn8 = assign53720_e68872_d_n8;
        locals.var_dl__blk1280_dn9 = assign53720_e68872_d_n9;
        locals.var_dl__blk1280_rv = 0.0;

        let assign53730_e68875: f64 = if locals.var_dl__blk1280 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1524 = assign53730_e68875;
        locals.var_guard1524_rv = 0.0;

        let (assign53740_e68889, assign53740_e68889_d_n4, assign53740_e68889_d_n6, assign53740_e68889_d_n7, assign53740_e68889_d_n8, assign53740_e68889_d_n9,) = {
    if ((locals.var_guard1523 != 0.0) && (locals.var_guard1524 != 0.0)) {
        let assign53740_e68882: f64 = (1.0 + locals.var_dl__blk1280);
        let assign53740_e68885: f64 = (locals.var_dl__blk1280 * locals.var_dl__blk1280);
        let assign53740_e68886: f64 = (assign53740_e68882 + assign53740_e68885);
        let assign53740_e68887: f64 = (1.0 / assign53740_e68886);
        (assign53740_e68887, (-((locals.var_dl__blk1280_dn4 + ((locals.var_dl__blk1280_dn4 * locals.var_dl__blk1280) + (locals.var_dl__blk1280 * locals.var_dl__blk1280_dn4))) / (assign53740_e68886 * assign53740_e68886))), (-((locals.var_dl__blk1280_dn6 + ((locals.var_dl__blk1280_dn6 * locals.var_dl__blk1280) + (locals.var_dl__blk1280 * locals.var_dl__blk1280_dn6))) / (assign53740_e68886 * assign53740_e68886))), (-((locals.var_dl__blk1280_dn7 + ((locals.var_dl__blk1280_dn7 * locals.var_dl__blk1280) + (locals.var_dl__blk1280 * locals.var_dl__blk1280_dn7))) / (assign53740_e68886 * assign53740_e68886))), (-((locals.var_dl__blk1280_dn8 + ((locals.var_dl__blk1280_dn8 * locals.var_dl__blk1280) + (locals.var_dl__blk1280 * locals.var_dl__blk1280_dn8))) / (assign53740_e68886 * assign53740_e68886))), (-((locals.var_dl__blk1280_dn9 + ((locals.var_dl__blk1280_dn9 * locals.var_dl__blk1280) + (locals.var_dl__blk1280 * locals.var_dl__blk1280_dn9))) / (assign53740_e68886 * assign53740_e68886))),)
    } else {
        (locals.var_gdl_ac, locals.var_gdl_ac_dn4, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8, locals.var_gdl_ac_dn9,)
    }
};
        locals.var_gdl_ac = assign53740_e68889;
        locals.var_gdl_ac_dn4 = assign53740_e68889_d_n4;
        locals.var_gdl_ac_dn6 = assign53740_e68889_d_n6;
        locals.var_gdl_ac_dn7 = assign53740_e68889_d_n7;
        locals.var_gdl_ac_dn8 = assign53740_e68889_d_n8;
        locals.var_gdl_ac_dn9 = assign53740_e68889_d_n9;
        locals.var_gdl_ac_rv = 0.0;

        let (assign53750_e68898, assign53750_e68898_d_n4, assign53750_e68898_d_n6, assign53750_e68898_d_n7, assign53750_e68898_d_n8, assign53750_e68898_d_n9,) = {
    if ((locals.var_guard1523 != 0.0) && (locals.var_guard1524 == 0.0)) {
        let assign53750_e68896: f64 = (1.0 - locals.var_dl__blk1280);
        (assign53750_e68896, (-locals.var_dl__blk1280_dn4), (-locals.var_dl__blk1280_dn6), (-locals.var_dl__blk1280_dn7), (-locals.var_dl__blk1280_dn8), (-locals.var_dl__blk1280_dn9),)
    } else {
        (locals.var_gdl_ac, locals.var_gdl_ac_dn4, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8, locals.var_gdl_ac_dn9,)
    }
};
        locals.var_gdl_ac = assign53750_e68898;
        locals.var_gdl_ac_dn4 = assign53750_e68898_d_n4;
        locals.var_gdl_ac_dn6 = assign53750_e68898_d_n6;
        locals.var_gdl_ac_dn7 = assign53750_e68898_d_n7;
        locals.var_gdl_ac_dn8 = assign53750_e68898_d_n8;
        locals.var_gdl_ac_dn9 = assign53750_e68898_d_n9;
        locals.var_gdl_ac_rv = 0.0;

        let (assign53760_e68904, assign53760_e68904_d_n4, assign53760_e68904_d_n6, assign53760_e68904_d_n7, assign53760_e68904_d_n8, assign53760_e68904_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53760_e68902: f64 = (locals.var_gmob_ac * locals.var_gdl_ac);
        (assign53760_e68902, ((locals.var_gmob_ac_dn4 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn4)), ((locals.var_gmob_ac_dn6 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn6)), ((locals.var_gmob_ac_dn7 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn7)), ((locals.var_gmob_ac_dn8 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn8)), ((locals.var_gmob_ac_dn9 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn9)),)
    } else {
        (locals.var_gmob_dl_ac, locals.var_gmob_dl_ac_dn4, locals.var_gmob_dl_ac_dn6, locals.var_gmob_dl_ac_dn7, locals.var_gmob_dl_ac_dn8, locals.var_gmob_dl_ac_dn9,)
    }
};
        locals.var_gmob_dl_ac = assign53760_e68904;
        locals.var_gmob_dl_ac_dn4 = assign53760_e68904_d_n4;
        locals.var_gmob_dl_ac_dn6 = assign53760_e68904_d_n6;
        locals.var_gmob_dl_ac_dn7 = assign53760_e68904_d_n7;
        locals.var_gmob_dl_ac_dn8 = assign53760_e68904_d_n8;
        locals.var_gmob_dl_ac_dn9 = assign53760_e68904_d_n9;
        locals.var_gmob_dl_ac_rv = 0.0;

        let (assign53770_e68910, assign53770_e68910_d_n4, assign53770_e68910_d_n6, assign53770_e68910_d_n7, assign53770_e68910_d_n8, assign53770_e68910_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53770_e68908: f64 = (locals.var_thesateff_ac / locals.var_gmob_dl_ac);
        (assign53770_e68908, (((locals.var_thesateff_ac_dn4 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn4)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn6 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn6)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn7 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn7)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn8 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn8)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn9 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn9)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)),)
    } else {
        (locals.var_thesat1_ac, locals.var_thesat1_ac_dn4, locals.var_thesat1_ac_dn6, locals.var_thesat1_ac_dn7, locals.var_thesat1_ac_dn8, locals.var_thesat1_ac_dn9,)
    }
};
        locals.var_thesat1_ac = assign53770_e68910;
        locals.var_thesat1_ac_dn4 = assign53770_e68910_d_n4;
        locals.var_thesat1_ac_dn6 = assign53770_e68910_d_n6;
        locals.var_thesat1_ac_dn7 = assign53770_e68910_d_n7;
        locals.var_thesat1_ac_dn8 = assign53770_e68910_d_n8;
        locals.var_thesat1_ac_dn9 = assign53770_e68910_d_n9;
        locals.var_thesat1_ac_rv = 0.0;

        let (assign53780_e68920, assign53780_e68920_d_n4, assign53780_e68920_d_n6, assign53780_e68920_d_n7, assign53780_e68920_d_n8, assign53780_e68920_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53780_e68914: f64 = (locals.var_thesat1_ac * locals.var_thesat1_ac);
        let assign53780_e68916: f64 = (assign53780_e68914 * locals.var_dps_ac);
        let assign53780_e68918: f64 = (assign53780_e68916 * locals.var_dps_ac);
        (assign53780_e68918, ((((((locals.var_thesat1_ac_dn4 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn4)) * locals.var_dps_ac) + (assign53780_e68914 * locals.var_dps_ac_dn4)) * locals.var_dps_ac) + (assign53780_e68916 * locals.var_dps_ac_dn4)), ((((((locals.var_thesat1_ac_dn6 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn6)) * locals.var_dps_ac) + (assign53780_e68914 * locals.var_dps_ac_dn6)) * locals.var_dps_ac) + (assign53780_e68916 * locals.var_dps_ac_dn6)), ((((((locals.var_thesat1_ac_dn7 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn7)) * locals.var_dps_ac) + (assign53780_e68914 * locals.var_dps_ac_dn7)) * locals.var_dps_ac) + (assign53780_e68916 * locals.var_dps_ac_dn7)), ((((((locals.var_thesat1_ac_dn8 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn8)) * locals.var_dps_ac) + (assign53780_e68914 * locals.var_dps_ac_dn8)) * locals.var_dps_ac) + (assign53780_e68916 * locals.var_dps_ac_dn8)), ((((((locals.var_thesat1_ac_dn9 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn9)) * locals.var_dps_ac) + (assign53780_e68914 * locals.var_dps_ac_dn9)) * locals.var_dps_ac) + (assign53780_e68916 * locals.var_dps_ac_dn9)),)
    } else {
        (locals.var_zsat__blk1281, locals.var_zsat__blk1281_dn4, locals.var_zsat__blk1281_dn6, locals.var_zsat__blk1281_dn7, locals.var_zsat__blk1281_dn8, locals.var_zsat__blk1281_dn9,)
    }
};
        locals.var_zsat__blk1281 = assign53780_e68920;
        locals.var_zsat__blk1281_dn4 = assign53780_e68920_d_n4;
        locals.var_zsat__blk1281_dn6 = assign53780_e68920_d_n6;
        locals.var_zsat__blk1281_dn7 = assign53780_e68920_d_n7;
        locals.var_zsat__blk1281_dn8 = assign53780_e68920_d_n8;
        locals.var_zsat__blk1281_dn9 = assign53780_e68920_d_n9;
        locals.var_zsat__blk1281_rv = 0.0;

        let assign53790_e68923: f64 = (-1.0);
        let assign53790_e68924: f64 = if locals.var_chnl_type == assign53790_e68923 { 1.0 } else { 0.0 };
        locals.var_guard1525 = assign53790_e68924;
        locals.var_guard1525_rv = 0.0;

        let (assign53800_e68936, assign53800_e68936_d_n4, assign53800_e68936_d_n6, assign53800_e68936_d_n7, assign53800_e68936_d_n8, assign53800_e68936_d_n9,) = {
    if ((locals.var_guard1523 != 0.0) && (locals.var_guard1525 != 0.0)) {
        let assign53800_e68932: f64 = (locals.var_thesat1_ac * locals.var_dps_ac);
        let assign53800_e68933: f64 = (1.0 + assign53800_e68932);
        let assign53800_e68934: f64 = (locals.var_zsat__blk1281 / assign53800_e68933);
        (assign53800_e68934, (((locals.var_zsat__blk1281_dn4 * assign53800_e68933) - (locals.var_zsat__blk1281 * ((locals.var_thesat1_ac_dn4 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn4)))) / (assign53800_e68933 * assign53800_e68933)), (((locals.var_zsat__blk1281_dn6 * assign53800_e68933) - (locals.var_zsat__blk1281 * ((locals.var_thesat1_ac_dn6 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn6)))) / (assign53800_e68933 * assign53800_e68933)), (((locals.var_zsat__blk1281_dn7 * assign53800_e68933) - (locals.var_zsat__blk1281 * ((locals.var_thesat1_ac_dn7 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn7)))) / (assign53800_e68933 * assign53800_e68933)), (((locals.var_zsat__blk1281_dn8 * assign53800_e68933) - (locals.var_zsat__blk1281 * ((locals.var_thesat1_ac_dn8 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn8)))) / (assign53800_e68933 * assign53800_e68933)), (((locals.var_zsat__blk1281_dn9 * assign53800_e68933) - (locals.var_zsat__blk1281 * ((locals.var_thesat1_ac_dn9 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn9)))) / (assign53800_e68933 * assign53800_e68933)),)
    } else {
        (locals.var_zsat__blk1281, locals.var_zsat__blk1281_dn4, locals.var_zsat__blk1281_dn6, locals.var_zsat__blk1281_dn7, locals.var_zsat__blk1281_dn8, locals.var_zsat__blk1281_dn9,)
    }
};
        locals.var_zsat__blk1281 = assign53800_e68936;
        locals.var_zsat__blk1281_dn4 = assign53800_e68936_d_n4;
        locals.var_zsat__blk1281_dn6 = assign53800_e68936_d_n6;
        locals.var_zsat__blk1281_dn7 = assign53800_e68936_d_n7;
        locals.var_zsat__blk1281_dn8 = assign53800_e68936_d_n8;
        locals.var_zsat__blk1281_dn9 = assign53800_e68936_d_n9;
        locals.var_zsat__blk1281_rv = 0.0;

        let (assign53810_e68951, assign53810_e68951_d_n4, assign53810_e68951_d_n6, assign53810_e68951_d_n7, assign53810_e68951_d_n8, assign53810_e68951_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53810_e68944: f64 = (2.0 * locals.var_zsat__blk1281);
        let assign53810_e68945: f64 = (1.0 + assign53810_e68944);
        let assign53810_e68946: f64 = (assign53810_e68945).sqrt();
        let assign53810_e68947: f64 = (1.0 + assign53810_e68946);
        let assign53810_e68948: f64 = (locals.var_gmob_dl_ac * assign53810_e68947);
        let assign53810_e68949: f64 = (0.5 * assign53810_e68948);
        (assign53810_e68949, (0.5 * ((locals.var_gmob_dl_ac_dn4 * assign53810_e68947) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1281_dn4) / (2.0 * assign53810_e68946))))), (0.5 * ((locals.var_gmob_dl_ac_dn6 * assign53810_e68947) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1281_dn6) / (2.0 * assign53810_e68946))))), (0.5 * ((locals.var_gmob_dl_ac_dn7 * assign53810_e68947) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1281_dn7) / (2.0 * assign53810_e68946))))), (0.5 * ((locals.var_gmob_dl_ac_dn8 * assign53810_e68947) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1281_dn8) / (2.0 * assign53810_e68946))))), (0.5 * ((locals.var_gmob_dl_ac_dn9 * assign53810_e68947) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1281_dn9) / (2.0 * assign53810_e68946))))),)
    } else {
        (locals.var_gvsat_ac, locals.var_gvsat_ac_dn4, locals.var_gvsat_ac_dn6, locals.var_gvsat_ac_dn7, locals.var_gvsat_ac_dn8, locals.var_gvsat_ac_dn9,)
    }
};
        locals.var_gvsat_ac = assign53810_e68951;
        locals.var_gvsat_ac_dn4 = assign53810_e68951_d_n4;
        locals.var_gvsat_ac_dn6 = assign53810_e68951_d_n6;
        locals.var_gvsat_ac_dn7 = assign53810_e68951_d_n7;
        locals.var_gvsat_ac_dn8 = assign53810_e68951_d_n8;
        locals.var_gvsat_ac_dn9 = assign53810_e68951_d_n9;
        locals.var_gvsat_ac_rv = 0.0;

        let (assign53820_e68957, assign53820_e68957_d_n4, assign53820_e68957_d_n6, assign53820_e68957_d_n7, assign53820_e68957_d_n8, assign53820_e68957_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53820_e68955: f64 = (locals.var_gmob_dl_ac / locals.var_gvsat_ac);
        (assign53820_e68955, (((locals.var_gmob_dl_ac_dn4 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn4)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn6 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn6)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn7 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn7)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn8 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn8)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn9 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn9)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign53820_e68957;
        locals.var_temp__blk949_dn4 = assign53820_e68957_d_n4;
        locals.var_temp__blk949_dn6 = assign53820_e68957_d_n6;
        locals.var_temp__blk949_dn7 = assign53820_e68957_d_n7;
        locals.var_temp__blk949_dn8 = assign53820_e68957_d_n8;
        locals.var_temp__blk949_dn9 = assign53820_e68957_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign53830_e68971, assign53830_e68971_d_n4, assign53830_e68971_d_n6, assign53830_e68971_d_n7, assign53830_e68971_d_n8, assign53830_e68971_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53830_e68964: f64 = (locals.var_zsat__blk1281 * locals.var_temp__blk949);
        let assign53830_e68966: f64 = (assign53830_e68964 * locals.var_temp__blk949);
        let assign53830_e68967: f64 = (0.5 * assign53830_e68966);
        let assign53830_e68968: f64 = (1.0 + assign53830_e68967);
        let assign53830_e68969: f64 = (locals.var_alpha_ac * assign53830_e68968);
        (assign53830_e68969, ((locals.var_alpha_ac_dn4 * assign53830_e68968) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1281_dn4 * locals.var_temp__blk949) + (locals.var_zsat__blk1281 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign53830_e68964 * locals.var_temp__blk949_dn4))))), ((locals.var_alpha_ac_dn6 * assign53830_e68968) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1281_dn6 * locals.var_temp__blk949) + (locals.var_zsat__blk1281 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign53830_e68964 * locals.var_temp__blk949_dn6))))), ((locals.var_alpha_ac_dn7 * assign53830_e68968) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1281_dn7 * locals.var_temp__blk949) + (locals.var_zsat__blk1281 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign53830_e68964 * locals.var_temp__blk949_dn7))))), ((locals.var_alpha_ac_dn8 * assign53830_e68968) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1281_dn8 * locals.var_temp__blk949) + (locals.var_zsat__blk1281 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign53830_e68964 * locals.var_temp__blk949_dn8))))), ((locals.var_alpha_ac_dn9 * assign53830_e68968) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1281_dn9 * locals.var_temp__blk949) + (locals.var_zsat__blk1281 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign53830_e68964 * locals.var_temp__blk949_dn9))))),)
    } else {
        (locals.var_alpha1__blk1282, locals.var_alpha1__blk1282_dn4, locals.var_alpha1__blk1282_dn6, locals.var_alpha1__blk1282_dn7, locals.var_alpha1__blk1282_dn8, locals.var_alpha1__blk1282_dn9,)
    }
};
        locals.var_alpha1__blk1282 = assign53830_e68971;
        locals.var_alpha1__blk1282_dn4 = assign53830_e68971_d_n4;
        locals.var_alpha1__blk1282_dn6 = assign53830_e68971_d_n6;
        locals.var_alpha1__blk1282_dn7 = assign53830_e68971_d_n7;
        locals.var_alpha1__blk1282_dn8 = assign53830_e68971_d_n8;
        locals.var_alpha1__blk1282_dn9 = assign53830_e68971_d_n9;
        locals.var_alpha1__blk1282_rv = 0.0;

        let (assign53840_e68979, assign53840_e68979_d_n4, assign53840_e68979_d_n6, assign53840_e68979_d_n7, assign53840_e68979_d_n8, assign53840_e68979_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53840_e68975: f64 = (locals.var_temp__blk949 * locals.var_qim1_ac);
        let assign53840_e68977: f64 = (assign53840_e68975 / locals.var_alpha1__blk1282);
        (assign53840_e68977, (((((locals.var_temp__blk949_dn4 * locals.var_qim1_ac) + (locals.var_temp__blk949 * locals.var_qim1_ac_dn4)) * locals.var_alpha1__blk1282) - (assign53840_e68975 * locals.var_alpha1__blk1282_dn4)) / (locals.var_alpha1__blk1282 * locals.var_alpha1__blk1282)), (((((locals.var_temp__blk949_dn6 * locals.var_qim1_ac) + (locals.var_temp__blk949 * locals.var_qim1_ac_dn6)) * locals.var_alpha1__blk1282) - (assign53840_e68975 * locals.var_alpha1__blk1282_dn6)) / (locals.var_alpha1__blk1282 * locals.var_alpha1__blk1282)), (((((locals.var_temp__blk949_dn7 * locals.var_qim1_ac) + (locals.var_temp__blk949 * locals.var_qim1_ac_dn7)) * locals.var_alpha1__blk1282) - (assign53840_e68975 * locals.var_alpha1__blk1282_dn7)) / (locals.var_alpha1__blk1282 * locals.var_alpha1__blk1282)), (((((locals.var_temp__blk949_dn8 * locals.var_qim1_ac) + (locals.var_temp__blk949 * locals.var_qim1_ac_dn8)) * locals.var_alpha1__blk1282) - (assign53840_e68975 * locals.var_alpha1__blk1282_dn8)) / (locals.var_alpha1__blk1282 * locals.var_alpha1__blk1282)), (((((locals.var_temp__blk949_dn9 * locals.var_qim1_ac) + (locals.var_temp__blk949 * locals.var_qim1_ac_dn9)) * locals.var_alpha1__blk1282) - (assign53840_e68975 * locals.var_alpha1__blk1282_dn9)) / (locals.var_alpha1__blk1282 * locals.var_alpha1__blk1282)),)
    } else {
        (locals.var_h_ac, locals.var_h_ac_dn4, locals.var_h_ac_dn6, locals.var_h_ac_dn7, locals.var_h_ac_dn8, locals.var_h_ac_dn9,)
    }
};
        locals.var_h_ac = assign53840_e68979;
        locals.var_h_ac_dn4 = assign53840_e68979_d_n4;
        locals.var_h_ac_dn6 = assign53840_e68979_d_n6;
        locals.var_h_ac_dn7 = assign53840_e68979_d_n7;
        locals.var_h_ac_dn8 = assign53840_e68979_d_n8;
        locals.var_h_ac_dn9 = assign53840_e68979_d_n9;
        locals.var_h_ac_rv = 0.0;

        let (assign53850_e68987, assign53850_e68987_d_n4, assign53850_e68987_d_n6, assign53850_e68987_d_n7, assign53850_e68987_d_n8, assign53850_e68987_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53850_e68984: f64 = (locals.var_dps_ac / locals.var_h_ac);
        let assign53850_e68985: f64 = (0.5 * assign53850_e68984);
        (assign53850_e68985, (0.5 * (((locals.var_dps_ac_dn4 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn4)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn6 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn6)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn7 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn7)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn8 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn8)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn9 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn9)) / (locals.var_h_ac * locals.var_h_ac))),)
    } else {
        (locals.var_fj, locals.var_fj_dn4, locals.var_fj_dn6, locals.var_fj_dn7, locals.var_fj_dn8, locals.var_fj_dn9,)
    }
};
        locals.var_fj = assign53850_e68987;
        locals.var_fj_dn4 = assign53850_e68987_d_n4;
        locals.var_fj_dn6 = assign53850_e68987_d_n6;
        locals.var_fj_dn7 = assign53850_e68987_d_n7;
        locals.var_fj_dn8 = assign53850_e68987_d_n8;
        locals.var_fj_dn9 = assign53850_e68987_d_n9;
        locals.var_fj_rv = 0.0;

        let (assign53860_e68993, assign53860_e68993_d_n4, assign53860_e68993_d_n6, assign53860_e68993_d_n7, assign53860_e68993_d_n8, assign53860_e68993_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53860_e68991: f64 = (locals.var_fj * locals.var_fj);
        (assign53860_e68991, ((locals.var_fj_dn4 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn4)), ((locals.var_fj_dn6 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn6)), ((locals.var_fj_dn7 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn7)), ((locals.var_fj_dn8 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn8)), ((locals.var_fj_dn9 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn9)),)
    } else {
        (locals.var_fj2, locals.var_fj2_dn4, locals.var_fj2_dn6, locals.var_fj2_dn7, locals.var_fj2_dn8, locals.var_fj2_dn9,)
    }
};
        locals.var_fj2 = assign53860_e68993;
        locals.var_fj2_dn4 = assign53860_e68993_d_n4;
        locals.var_fj2_dn6 = assign53860_e68993_d_n6;
        locals.var_fj2_dn7 = assign53860_e68993_d_n7;
        locals.var_fj2_dn8 = assign53860_e68993_d_n8;
        locals.var_fj2_dn9 = assign53860_e68993_d_n9;
        locals.var_fj2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign53870_e69013, assign53870_e69013_d_n4, assign53870_e69013_d_n6, assign53870_e69013_d_n7, assign53870_e69013_d_n8, assign53870_e69013_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53870_e68999: f64 = (locals.var_eta_p_ac * locals.var_dps_ac);
        let assign53870_e69002: f64 = (locals.var_fj * locals.var_gdl_ac);
        let assign53870_e69004: f64 = (assign53870_e69002 * 0.3333333333333333);
        let assign53870_e69006: f64 = (assign53870_e69004 - 1.0);
        let assign53870_e69008: f64 = (assign53870_e69006 + locals.var_gdl_ac);
        let assign53870_e69009: f64 = (assign53870_e68999 * assign53870_e69008);
        let assign53870_e69010: f64 = (0.5 * assign53870_e69009);
        let assign53870_e69011: f64 = (locals.var_voxm_ac + assign53870_e69010);
        (assign53870_e69011, (locals.var_voxm_ac_dn4 + (0.5 * ((((locals.var_eta_p_ac_dn4 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn4)) * assign53870_e69008) + (assign53870_e68999 * ((((locals.var_fj_dn4 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn4)) * 0.3333333333333333) + locals.var_gdl_ac_dn4))))), (locals.var_voxm_ac_dn6 + (0.5 * ((((locals.var_eta_p_ac_dn6 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn6)) * assign53870_e69008) + (assign53870_e68999 * ((((locals.var_fj_dn6 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn6)) * 0.3333333333333333) + locals.var_gdl_ac_dn6))))), (locals.var_voxm_ac_dn7 + (0.5 * ((((locals.var_eta_p_ac_dn7 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn7)) * assign53870_e69008) + (assign53870_e68999 * ((((locals.var_fj_dn7 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn7)) * 0.3333333333333333) + locals.var_gdl_ac_dn7))))), (locals.var_voxm_ac_dn8 + (0.5 * ((((locals.var_eta_p_ac_dn8 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn8)) * assign53870_e69008) + (assign53870_e68999 * ((((locals.var_fj_dn8 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn8)) * 0.3333333333333333) + locals.var_gdl_ac_dn8))))), (locals.var_voxm_ac_dn9 + (0.5 * ((((locals.var_eta_p_ac_dn9 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn9)) * assign53870_e69008) + (assign53870_e68999 * ((((locals.var_fj_dn9 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn9)) * 0.3333333333333333) + locals.var_gdl_ac_dn9))))),)
    } else {
        (locals.var_qg_1, locals.var_qg_1_dn4, locals.var_qg_1_dn6, locals.var_qg_1_dn7, locals.var_qg_1_dn8, locals.var_qg_1_dn9,)
    }
};
        locals.var_qg_1 = assign53870_e69013;
        locals.var_qg_1_dn4 = assign53870_e69013_d_n4;
        locals.var_qg_1_dn6 = assign53870_e69013_d_n6;
        locals.var_qg_1_dn7 = assign53870_e69013_d_n7;
        locals.var_qg_1_dn8 = assign53870_e69013_d_n8;
        locals.var_qg_1_dn9 = assign53870_e69013_d_n9;
        locals.var_qg_1_rv = 0.0;

        let (assign53880_e69021, assign53880_e69021_d_n4, assign53880_e69021_d_n6, assign53880_e69021_d_n7, assign53880_e69021_d_n8, assign53880_e69021_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53880_e69017: f64 = (locals.var_alpha_ac * locals.var_dps_ac);
        let assign53880_e69019: f64 = (assign53880_e69017 * 0.16666666666666666);
        (assign53880_e69019, (((locals.var_alpha_ac_dn4 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn4)) * 0.16666666666666666), (((locals.var_alpha_ac_dn6 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn6)) * 0.16666666666666666), (((locals.var_alpha_ac_dn7 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn7)) * 0.16666666666666666), (((locals.var_alpha_ac_dn8 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn8)) * 0.16666666666666666), (((locals.var_alpha_ac_dn9 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn9)) * 0.16666666666666666),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign53880_e69021;
        locals.var_temp__blk949_dn4 = assign53880_e69021_d_n4;
        locals.var_temp__blk949_dn6 = assign53880_e69021_d_n6;
        locals.var_temp__blk949_dn7 = assign53880_e69021_d_n7;
        locals.var_temp__blk949_dn8 = assign53880_e69021_d_n8;
        locals.var_temp__blk949_dn9 = assign53880_e69021_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign53890_e69024: f64 = if p.p49 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1526 = assign53890_e69024;
        locals.var_guard1526_rv = 0.0;

        let (assign53900_e69030, assign53900_e69030_d_n4, assign53900_e69030_d_n6, assign53900_e69030_d_n7, assign53900_e69030_d_n8, assign53900_e69030_d_n9,) = {
    if ((locals.var_guard1523 != 0.0) && (locals.var_guard1526 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qclm, locals.var_qclm_dn4, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8, locals.var_qclm_dn9,)
    }
};
        locals.var_qclm = assign53900_e69030;
        locals.var_qclm_dn4 = assign53900_e69030_d_n4;
        locals.var_qclm_dn6 = assign53900_e69030_d_n6;
        locals.var_qclm_dn7 = assign53900_e69030_d_n7;
        locals.var_qclm_dn8 = assign53900_e69030_d_n8;
        locals.var_qclm_dn9 = assign53900_e69030_d_n9;
        locals.var_qclm_rv = 0.0;

        let (assign53910_e69050, assign53910_e69050_d_n4, assign53910_e69050_d_n6, assign53910_e69050_d_n7, assign53910_e69050_d_n8, assign53910_e69050_d_n9,) = {
    if ((locals.var_guard1523 != 0.0) && (locals.var_guard1526 != 0.0)) {
        let assign53910_e69036: f64 = (0.5 * locals.var_gdl_ac);
        let assign53910_e69038: f64 = (assign53910_e69036 * locals.var_gdl_ac);
        let assign53910_e69042: f64 = (3.0 * locals.var_temp__blk949);
        let assign53910_e69045: f64 = (2.0 - locals.var_fj);
        let assign53910_e69046: f64 = (assign53910_e69042 * assign53910_e69045);
        let assign53910_e69047: f64 = (locals.var_qim_ac - assign53910_e69046);
        let assign53910_e69048: f64 = (assign53910_e69038 * assign53910_e69047);
        (assign53910_e69048, (((((0.5 * locals.var_gdl_ac_dn4) * locals.var_gdl_ac) + (assign53910_e69036 * locals.var_gdl_ac_dn4)) * assign53910_e69047) + (assign53910_e69038 * (locals.var_qim_ac_dn4 - (((3.0 * locals.var_temp__blk949_dn4) * assign53910_e69045) + (assign53910_e69042 * (-locals.var_fj_dn4)))))), (((((0.5 * locals.var_gdl_ac_dn6) * locals.var_gdl_ac) + (assign53910_e69036 * locals.var_gdl_ac_dn6)) * assign53910_e69047) + (assign53910_e69038 * (locals.var_qim_ac_dn6 - (((3.0 * locals.var_temp__blk949_dn6) * assign53910_e69045) + (assign53910_e69042 * (-locals.var_fj_dn6)))))), (((((0.5 * locals.var_gdl_ac_dn7) * locals.var_gdl_ac) + (assign53910_e69036 * locals.var_gdl_ac_dn7)) * assign53910_e69047) + (assign53910_e69038 * (locals.var_qim_ac_dn7 - (((3.0 * locals.var_temp__blk949_dn7) * assign53910_e69045) + (assign53910_e69042 * (-locals.var_fj_dn7)))))), (((((0.5 * locals.var_gdl_ac_dn8) * locals.var_gdl_ac) + (assign53910_e69036 * locals.var_gdl_ac_dn8)) * assign53910_e69047) + (assign53910_e69038 * (locals.var_qim_ac_dn8 - (((3.0 * locals.var_temp__blk949_dn8) * assign53910_e69045) + (assign53910_e69042 * (-locals.var_fj_dn8)))))), (((((0.5 * locals.var_gdl_ac_dn9) * locals.var_gdl_ac) + (assign53910_e69036 * locals.var_gdl_ac_dn9)) * assign53910_e69047) + (assign53910_e69038 * (locals.var_qim_ac_dn9 - (((3.0 * locals.var_temp__blk949_dn9) * assign53910_e69045) + (assign53910_e69042 * (-locals.var_fj_dn9)))))),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn4, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9,)
    }
};
        locals.var_qd_1 = assign53910_e69050;
        locals.var_qd_1_dn4 = assign53910_e69050_d_n4;
        locals.var_qd_1_dn6 = assign53910_e69050_d_n6;
        locals.var_qd_1_dn7 = assign53910_e69050_d_n7;
        locals.var_qd_1_dn8 = assign53910_e69050_d_n8;
        locals.var_qd_1_dn9 = assign53910_e69050_d_n9;
        locals.var_qd_1_rv = 0.0;

        let (assign53920_e69067, assign53920_e69067_d_n4, assign53920_e69067_d_n6, assign53920_e69067_d_n7, assign53920_e69067_d_n8, assign53920_e69067_d_n9,) = {
    if ((locals.var_guard1523 != 0.0) && (locals.var_guard1526 == 0.0)) {
        let assign53920_e69057: f64 = (1.0 - locals.var_gdl_ac);
        let assign53920_e69062: f64 = (locals.var_alpha_ac * locals.var_dps_ac);
        let assign53920_e69063: f64 = (0.5 * assign53920_e69062);
        let assign53920_e69064: f64 = (locals.var_qim_ac - assign53920_e69063);
        let assign53920_e69065: f64 = (assign53920_e69057 * assign53920_e69064);
        (assign53920_e69065, (((-locals.var_gdl_ac_dn4) * assign53920_e69064) + (assign53920_e69057 * (locals.var_qim_ac_dn4 - (0.5 * ((locals.var_alpha_ac_dn4 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn4)))))), (((-locals.var_gdl_ac_dn6) * assign53920_e69064) + (assign53920_e69057 * (locals.var_qim_ac_dn6 - (0.5 * ((locals.var_alpha_ac_dn6 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn6)))))), (((-locals.var_gdl_ac_dn7) * assign53920_e69064) + (assign53920_e69057 * (locals.var_qim_ac_dn7 - (0.5 * ((locals.var_alpha_ac_dn7 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn7)))))), (((-locals.var_gdl_ac_dn8) * assign53920_e69064) + (assign53920_e69057 * (locals.var_qim_ac_dn8 - (0.5 * ((locals.var_alpha_ac_dn8 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn8)))))), (((-locals.var_gdl_ac_dn9) * assign53920_e69064) + (assign53920_e69057 * (locals.var_qim_ac_dn9 - (0.5 * ((locals.var_alpha_ac_dn9 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn9)))))),)
    } else {
        (locals.var_qclm, locals.var_qclm_dn4, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8, locals.var_qclm_dn9,)
    }
};
        locals.var_qclm = assign53920_e69067;
        locals.var_qclm_dn4 = assign53920_e69067_d_n4;
        locals.var_qclm_dn6 = assign53920_e69067_d_n6;
        locals.var_qclm_dn7 = assign53920_e69067_d_n7;
        locals.var_qclm_dn8 = assign53920_e69067_d_n8;
        locals.var_qclm_dn9 = assign53920_e69067_d_n9;
        locals.var_qclm_rv = 0.0;

        let (assign53930_e69096, assign53930_e69096_d_n4, assign53930_e69096_d_n6, assign53930_e69096_d_n7, assign53930_e69096_d_n8, assign53930_e69096_d_n9,) = {
    if ((locals.var_guard1523 != 0.0) && (locals.var_guard1526 == 0.0)) {
        let assign53930_e69075: f64 = (locals.var_gdl_ac * locals.var_gdl_ac);
        let assign53930_e69080: f64 = (1.0 - locals.var_fj);
        let assign53930_e69083: f64 = (0.2 * locals.var_fj2);
        let assign53930_e69084: f64 = (assign53930_e69080 - assign53930_e69083);
        let assign53930_e69085: f64 = (locals.var_temp__blk949 * assign53930_e69084);
        let assign53930_e69086: f64 = (locals.var_qim_ac - assign53930_e69085);
        let assign53930_e69087: f64 = (assign53930_e69075 * assign53930_e69086);
        let assign53930_e69091: f64 = (1.0 + locals.var_gdl_ac);
        let assign53930_e69092: f64 = (locals.var_qclm * assign53930_e69091);
        let assign53930_e69093: f64 = (assign53930_e69087 + assign53930_e69092);
        let assign53930_e69094: f64 = (0.5 * assign53930_e69093);
        (assign53930_e69094, (0.5 * (((((locals.var_gdl_ac_dn4 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn4)) * assign53930_e69086) + (assign53930_e69075 * (locals.var_qim_ac_dn4 - ((locals.var_temp__blk949_dn4 * assign53930_e69084) + (locals.var_temp__blk949 * ((-locals.var_fj_dn4) - (0.2 * locals.var_fj2_dn4))))))) + ((locals.var_qclm_dn4 * assign53930_e69091) + (locals.var_qclm * locals.var_gdl_ac_dn4)))), (0.5 * (((((locals.var_gdl_ac_dn6 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn6)) * assign53930_e69086) + (assign53930_e69075 * (locals.var_qim_ac_dn6 - ((locals.var_temp__blk949_dn6 * assign53930_e69084) + (locals.var_temp__blk949 * ((-locals.var_fj_dn6) - (0.2 * locals.var_fj2_dn6))))))) + ((locals.var_qclm_dn6 * assign53930_e69091) + (locals.var_qclm * locals.var_gdl_ac_dn6)))), (0.5 * (((((locals.var_gdl_ac_dn7 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn7)) * assign53930_e69086) + (assign53930_e69075 * (locals.var_qim_ac_dn7 - ((locals.var_temp__blk949_dn7 * assign53930_e69084) + (locals.var_temp__blk949 * ((-locals.var_fj_dn7) - (0.2 * locals.var_fj2_dn7))))))) + ((locals.var_qclm_dn7 * assign53930_e69091) + (locals.var_qclm * locals.var_gdl_ac_dn7)))), (0.5 * (((((locals.var_gdl_ac_dn8 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn8)) * assign53930_e69086) + (assign53930_e69075 * (locals.var_qim_ac_dn8 - ((locals.var_temp__blk949_dn8 * assign53930_e69084) + (locals.var_temp__blk949 * ((-locals.var_fj_dn8) - (0.2 * locals.var_fj2_dn8))))))) + ((locals.var_qclm_dn8 * assign53930_e69091) + (locals.var_qclm * locals.var_gdl_ac_dn8)))), (0.5 * (((((locals.var_gdl_ac_dn9 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn9)) * assign53930_e69086) + (assign53930_e69075 * (locals.var_qim_ac_dn9 - ((locals.var_temp__blk949_dn9 * assign53930_e69084) + (locals.var_temp__blk949 * ((-locals.var_fj_dn9) - (0.2 * locals.var_fj2_dn9))))))) + ((locals.var_qclm_dn9 * assign53930_e69091) + (locals.var_qclm * locals.var_gdl_ac_dn9)))),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn4, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9,)
    }
};
        locals.var_qd_1 = assign53930_e69096;
        locals.var_qd_1_dn4 = assign53930_e69096_d_n4;
        locals.var_qd_1_dn6 = assign53930_e69096_d_n6;
        locals.var_qd_1_dn7 = assign53930_e69096_d_n7;
        locals.var_qd_1_dn8 = assign53930_e69096_d_n8;
        locals.var_qd_1_dn9 = assign53930_e69096_d_n9;
        locals.var_qd_1_rv = 0.0;

        let (assign53940_e69108, assign53940_e69108_d_n4, assign53940_e69108_d_n6, assign53940_e69108_d_n7, assign53940_e69108_d_n8, assign53940_e69108_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53940_e69102: f64 = (locals.var_temp__blk949 * locals.var_fj);
        let assign53940_e69103: f64 = (locals.var_qim_ac + assign53940_e69102);
        let assign53940_e69104: f64 = (locals.var_gdl_ac * assign53940_e69103);
        let assign53940_e69106: f64 = (assign53940_e69104 + locals.var_qclm);
        (assign53940_e69106, (((locals.var_gdl_ac_dn4 * assign53940_e69103) + (locals.var_gdl_ac * (locals.var_qim_ac_dn4 + ((locals.var_temp__blk949_dn4 * locals.var_fj) + (locals.var_temp__blk949 * locals.var_fj_dn4))))) + locals.var_qclm_dn4), (((locals.var_gdl_ac_dn6 * assign53940_e69103) + (locals.var_gdl_ac * (locals.var_qim_ac_dn6 + ((locals.var_temp__blk949_dn6 * locals.var_fj) + (locals.var_temp__blk949 * locals.var_fj_dn6))))) + locals.var_qclm_dn6), (((locals.var_gdl_ac_dn7 * assign53940_e69103) + (locals.var_gdl_ac * (locals.var_qim_ac_dn7 + ((locals.var_temp__blk949_dn7 * locals.var_fj) + (locals.var_temp__blk949 * locals.var_fj_dn7))))) + locals.var_qclm_dn7), (((locals.var_gdl_ac_dn8 * assign53940_e69103) + (locals.var_gdl_ac * (locals.var_qim_ac_dn8 + ((locals.var_temp__blk949_dn8 * locals.var_fj) + (locals.var_temp__blk949 * locals.var_fj_dn8))))) + locals.var_qclm_dn8), (((locals.var_gdl_ac_dn9 * assign53940_e69103) + (locals.var_gdl_ac * (locals.var_qim_ac_dn9 + ((locals.var_temp__blk949_dn9 * locals.var_fj) + (locals.var_temp__blk949 * locals.var_fj_dn9))))) + locals.var_qclm_dn9),)
    } else {
        (locals.var_qi, locals.var_qi_dn4, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, locals.var_qi_dn9,)
    }
};
        locals.var_qi = assign53940_e69108;
        locals.var_qi_dn4 = assign53940_e69108_d_n4;
        locals.var_qi_dn6 = assign53940_e69108_d_n6;
        locals.var_qi_dn7 = assign53940_e69108_d_n7;
        locals.var_qi_dn8 = assign53940_e69108_d_n8;
        locals.var_qi_dn9 = assign53940_e69108_d_n9;
        locals.var_qi_rv = 0.0;

        let (assign53950_e69114, assign53950_e69114_d_n4, assign53950_e69114_d_n6, assign53950_e69114_d_n7, assign53950_e69114_d_n8, assign53950_e69114_d_n9,) = {
    if (locals.var_guard1523 != 0.0) {
        let assign53950_e69112: f64 = (locals.var_qg_1 - locals.var_qi);
        (assign53950_e69112, (locals.var_qg_1_dn4 - locals.var_qi_dn4), (locals.var_qg_1_dn6 - locals.var_qi_dn6), (locals.var_qg_1_dn7 - locals.var_qi_dn7), (locals.var_qg_1_dn8 - locals.var_qi_dn8), (locals.var_qg_1_dn9 - locals.var_qi_dn9),)
    } else {
        (locals.var_qb_1, locals.var_qb_1_dn4, locals.var_qb_1_dn6, locals.var_qb_1_dn7, locals.var_qb_1_dn8, locals.var_qb_1_dn9,)
    }
};
        locals.var_qb_1 = assign53950_e69114;
        locals.var_qb_1_dn4 = assign53950_e69114_d_n4;
        locals.var_qb_1_dn6 = assign53950_e69114_d_n6;
        locals.var_qb_1_dn7 = assign53950_e69114_d_n7;
        locals.var_qb_1_dn8 = assign53950_e69114_d_n8;
        locals.var_qb_1_dn9 = assign53950_e69114_d_n9;
        locals.var_qb_1_rv = 0.0;

        let assign53960_e69117: f64 = (locals.var_qg_1 * locals.var_cox_qm);
        locals.var_qg = assign53960_e69117;
        locals.var_qg_dn4 = ((locals.var_qg_1_dn4 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn4));
        locals.var_qg_dn6 = ((locals.var_qg_1_dn6 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn6));
        locals.var_qg_dn7 = ((locals.var_qg_1_dn7 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn7));
        locals.var_qg_dn8 = ((locals.var_qg_1_dn8 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn8));
        locals.var_qg_dn9 = ((locals.var_qg_1_dn9 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn9));
        locals.var_qg_rv = 0.0;

        let assign53970_e69119: f64 = (-locals.var_qd_1);
        let assign53970_e69121: f64 = (assign53970_e69119 * locals.var_cox_qm);
        locals.var_qd = assign53970_e69121;
        locals.var_qd_dn4 = (((-locals.var_qd_1_dn4) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn4));
        locals.var_qd_dn6 = (((-locals.var_qd_1_dn6) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn6));
        locals.var_qd_dn7 = (((-locals.var_qd_1_dn7) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn7));
        locals.var_qd_dn8 = (((-locals.var_qd_1_dn8) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn8));
        locals.var_qd_dn9 = (((-locals.var_qd_1_dn9) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn9));
        locals.var_qd_rv = 0.0;

        let assign53980_e69123: f64 = (-locals.var_qb_1);
        let assign53980_e69125: f64 = (assign53980_e69123 * locals.var_cox_qm);
        locals.var_qb = assign53980_e69125;
        locals.var_qb_dn4 = (((-locals.var_qb_1_dn4) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn4));
        locals.var_qb_dn6 = (((-locals.var_qb_1_dn6) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn6));
        locals.var_qb_dn7 = (((-locals.var_qb_1_dn7) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn7));
        locals.var_qb_dn8 = (((-locals.var_qb_1_dn8) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn8));
        locals.var_qb_dn9 = (((-locals.var_qb_1_dn9) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn9));
        locals.var_qb_rv = 0.0;

        locals.var_qsinr = 0.0;
        locals.var_qsinr_dn4 = 0.0;
        locals.var_qsinr_dn6 = 0.0;
        locals.var_qsinr_dn7 = 0.0;
        locals.var_qsinr_dn8 = 0.0;
        locals.var_qsinr_dn9 = 0.0;
        locals.var_qsinr_rv = 0.0;

        locals.var_qdinr = 0.0;
        locals.var_qdinr_dn4 = 0.0;
        locals.var_qdinr_dn6 = 0.0;
        locals.var_qdinr_dn7 = 0.0;
        locals.var_qdinr_dn8 = 0.0;
        locals.var_qdinr_dn9 = 0.0;
        locals.var_qdinr_rv = 0.0;

        locals.var_qginr = 0.0;
        locals.var_qginr_dn4 = 0.0;
        locals.var_qginr_dn6 = 0.0;
        locals.var_qginr_dn7 = 0.0;
        locals.var_qginr_dn8 = 0.0;
        locals.var_qginr_dn9 = 0.0;
        locals.var_qginr_rv = 0.0;

        let assign54020_e69135: f64 = if ((locals.var_cinr_i > 0.0) || (locals.var_cinrd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1527 = assign54020_e69135;
        locals.var_guard1527_rv = 0.0;

        let (assign54030_e69139, assign54030_e69139_d_n4, assign54030_e69139_d_n6, assign54030_e69139_d_n7, assign54030_e69139_d_n8, assign54030_e69139_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_finracc, locals.var_finracc_dn4, locals.var_finracc_dn6, locals.var_finracc_dn7, locals.var_finracc_dn8, locals.var_finracc_dn9,)
    }
};
        locals.var_finracc = assign54030_e69139;
        locals.var_finracc_dn4 = assign54030_e69139_d_n4;
        locals.var_finracc_dn6 = assign54030_e69139_d_n6;
        locals.var_finracc_dn7 = assign54030_e69139_d_n7;
        locals.var_finracc_dn8 = assign54030_e69139_d_n8;
        locals.var_finracc_dn9 = assign54030_e69139_d_n9;
        locals.var_finracc_rv = 0.0;

        let (assign54040_e69143, assign54040_e69143_d_n4, assign54040_e69143_d_n6, assign54040_e69143_d_n7, assign54040_e69143_d_n8, assign54040_e69143_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        (locals.var_vgb1_ac, locals.var_vgb1_ac_dn4, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, locals.var_vgb1_ac_dn9,)
    } else {
        (locals.var_dvinracc, locals.var_dvinracc_dn4, locals.var_dvinracc_dn6, locals.var_dvinracc_dn7, locals.var_dvinracc_dn8, locals.var_dvinracc_dn9,)
    }
};
        locals.var_dvinracc = assign54040_e69143;
        locals.var_dvinracc_dn4 = assign54040_e69143_d_n4;
        locals.var_dvinracc_dn6 = assign54040_e69143_d_n6;
        locals.var_dvinracc_dn7 = assign54040_e69143_d_n7;
        locals.var_dvinracc_dn8 = assign54040_e69143_d_n8;
        locals.var_dvinracc_dn9 = assign54040_e69143_d_n9;
        locals.var_dvinracc_rv = 0.0;

        let assign54050_e69146: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1528 = assign54050_e69146;
        locals.var_guard1528_rv = 0.0;

        let (assign54060_e69156, assign54060_e69156_d_n4, assign54060_e69156_d_n6, assign54060_e69156_d_n7, assign54060_e69156_d_n8, assign54060_e69156_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
        let assign54060_e69152: f64 = (locals.var_vgb1_ac - locals.var_dvfbinr_i);
        let assign54060_e69154: f64 = (assign54060_e69152 + locals.var_vinr_max);
        (assign54060_e69154, locals.var_vgb1_ac_dn4, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, locals.var_vgb1_ac_dn9,)
    } else {
        (locals.var_vginr, locals.var_vginr_dn4, locals.var_vginr_dn6, locals.var_vginr_dn7, locals.var_vginr_dn8, locals.var_vginr_dn9,)
    }
};
        locals.var_vginr = assign54060_e69156;
        locals.var_vginr_dn4 = assign54060_e69156_d_n4;
        locals.var_vginr_dn6 = assign54060_e69156_d_n6;
        locals.var_vginr_dn7 = assign54060_e69156_d_n7;
        locals.var_vginr_dn8 = assign54060_e69156_d_n8;
        locals.var_vginr_dn9 = assign54060_e69156_d_n9;
        locals.var_vginr_rv = 0.0;

        let (assign54070_e69177, assign54070_e69177_d_n4, assign54070_e69177_d_n6, assign54070_e69177_d_n7, assign54070_e69177_d_n8, assign54070_e69177_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
        let assign54070_e69163: f64 = (locals.var_vginr + locals.var_vinr_max);
        let assign54070_e69166: f64 = (locals.var_vginr - locals.var_vinr_max);
        let assign54070_e69169: f64 = (locals.var_vginr - locals.var_vinr_max);
        let assign54070_e69170: f64 = (assign54070_e69166 * assign54070_e69169);
        let assign54070_e69172: f64 = (assign54070_e69170 + locals.var_ainr);
        let assign54070_e69173: f64 = (assign54070_e69172).sqrt();
        let assign54070_e69174: f64 = (assign54070_e69163 + assign54070_e69173);
        let assign54070_e69175: f64 = (0.5 * assign54070_e69174);
        (assign54070_e69175, (0.5 * (locals.var_vginr_dn4 + (((locals.var_vginr_dn4 * assign54070_e69169) + (assign54070_e69166 * locals.var_vginr_dn4)) / (2.0 * assign54070_e69173)))), (0.5 * (locals.var_vginr_dn6 + (((locals.var_vginr_dn6 * assign54070_e69169) + (assign54070_e69166 * locals.var_vginr_dn6)) / (2.0 * assign54070_e69173)))), (0.5 * (locals.var_vginr_dn7 + (((locals.var_vginr_dn7 * assign54070_e69169) + (assign54070_e69166 * locals.var_vginr_dn7)) / (2.0 * assign54070_e69173)))), (0.5 * (locals.var_vginr_dn8 + (((locals.var_vginr_dn8 * assign54070_e69169) + (assign54070_e69166 * locals.var_vginr_dn8)) / (2.0 * assign54070_e69173)))), (0.5 * (locals.var_vginr_dn9 + (((locals.var_vginr_dn9 * assign54070_e69169) + (assign54070_e69166 * locals.var_vginr_dn9)) / (2.0 * assign54070_e69173)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign54070_e69177;
        locals.var_temp__blk949_dn4 = assign54070_e69177_d_n4;
        locals.var_temp__blk949_dn6 = assign54070_e69177_d_n6;
        locals.var_temp__blk949_dn7 = assign54070_e69177_d_n7;
        locals.var_temp__blk949_dn8 = assign54070_e69177_d_n8;
        locals.var_temp__blk949_dn9 = assign54070_e69177_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign54080_e69191, assign54080_e69191_d_n4, assign54080_e69191_d_n6, assign54080_e69191_d_n7, assign54080_e69191_d_n8, assign54080_e69191_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
        let assign54080_e69184: f64 = (2.0 * locals.var_temp__blk949);
        let assign54080_e69186: f64 = (assign54080_e69184 - locals.var_vinr_max);
        let assign54080_e69188: f64 = (assign54080_e69186 - locals.var_vginr);
        let assign54080_e69189: f64 = (locals.var_temp__blk949 * assign54080_e69188);
        (assign54080_e69189, ((locals.var_temp__blk949_dn4 * assign54080_e69188) + (locals.var_temp__blk949 * ((2.0 * locals.var_temp__blk949_dn4) - locals.var_vginr_dn4))), ((locals.var_temp__blk949_dn6 * assign54080_e69188) + (locals.var_temp__blk949 * ((2.0 * locals.var_temp__blk949_dn6) - locals.var_vginr_dn6))), ((locals.var_temp__blk949_dn7 * assign54080_e69188) + (locals.var_temp__blk949 * ((2.0 * locals.var_temp__blk949_dn7) - locals.var_vginr_dn7))), ((locals.var_temp__blk949_dn8 * assign54080_e69188) + (locals.var_temp__blk949 * ((2.0 * locals.var_temp__blk949_dn8) - locals.var_vginr_dn8))), ((locals.var_temp__blk949_dn9 * assign54080_e69188) + (locals.var_temp__blk949 * ((2.0 * locals.var_temp__blk949_dn9) - locals.var_vginr_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54080_e69191;
        locals.var_temp1_dn4 = assign54080_e69191_d_n4;
        locals.var_temp1_dn6 = assign54080_e69191_d_n6;
        locals.var_temp1_dn7 = assign54080_e69191_d_n7;
        locals.var_temp1_dn8 = assign54080_e69191_d_n8;
        locals.var_temp1_dn9 = assign54080_e69191_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign54090_e69199, assign54090_e69199_d_n4, assign54090_e69199_d_n6, assign54090_e69199_d_n7, assign54090_e69199_d_n8, assign54090_e69199_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
        let assign54090_e69197: f64 = (locals.var_vinr_max / locals.var_temp__blk949);
        (assign54090_e69197, (-((locals.var_vinr_max * locals.var_temp__blk949_dn4) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (-((locals.var_vinr_max * locals.var_temp__blk949_dn6) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (-((locals.var_vinr_max * locals.var_temp__blk949_dn7) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (-((locals.var_vinr_max * locals.var_temp__blk949_dn8) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (-((locals.var_vinr_max * locals.var_temp__blk949_dn9) / (locals.var_temp__blk949 * locals.var_temp__blk949))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign54090_e69199;
        locals.var_temp2_dn4 = assign54090_e69199_d_n4;
        locals.var_temp2_dn6 = assign54090_e69199_d_n6;
        locals.var_temp2_dn7 = assign54090_e69199_d_n7;
        locals.var_temp2_dn8 = assign54090_e69199_d_n8;
        locals.var_temp2_dn9 = assign54090_e69199_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign54100_e69207, assign54100_e69207_d_n4, assign54100_e69207_d_n6, assign54100_e69207_d_n7, assign54100_e69207_d_n8, assign54100_e69207_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
        let assign54100_e69205: f64 = (locals.var_vginr * locals.var_temp2);
        (assign54100_e69205, ((locals.var_vginr_dn4 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn4)), ((locals.var_vginr_dn6 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn6)), ((locals.var_vginr_dn7 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn7)), ((locals.var_vginr_dn8 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn8)), ((locals.var_vginr_dn9 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn9)),)
    } else {
        (locals.var_vginreff, locals.var_vginreff_dn4, locals.var_vginreff_dn6, locals.var_vginreff_dn7, locals.var_vginreff_dn8, locals.var_vginreff_dn9,)
    }
};
        locals.var_vginreff = assign54100_e69207;
        locals.var_vginreff_dn4 = assign54100_e69207_d_n4;
        locals.var_vginreff_dn6 = assign54100_e69207_d_n6;
        locals.var_vginreff_dn7 = assign54100_e69207_d_n7;
        locals.var_vginreff_dn8 = assign54100_e69207_d_n8;
        locals.var_vginreff_dn9 = assign54100_e69207_d_n9;
        locals.var_vginreff_rv = 0.0;

        let (assign54110_e69218, assign54110_e69218_d_n4, assign54110_e69218_d_n6, assign54110_e69218_d_n7, assign54110_e69218_d_n8, assign54110_e69218_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
        let assign54110_e69214: f64 = (locals.var_vginreff * locals.var_fcinracc_i);
        let assign54110_e69215: f64 = (1.0 - assign54110_e69214);
        let assign54110_e69216: f64 = (assign54110_e69215).sqrt();
        (assign54110_e69216, ((-(locals.var_vginreff_dn4 * locals.var_fcinracc_i)) / (2.0 * assign54110_e69216)), ((-(locals.var_vginreff_dn6 * locals.var_fcinracc_i)) / (2.0 * assign54110_e69216)), ((-(locals.var_vginreff_dn7 * locals.var_fcinracc_i)) / (2.0 * assign54110_e69216)), ((-(locals.var_vginreff_dn8 * locals.var_fcinracc_i)) / (2.0 * assign54110_e69216)), ((-(locals.var_vginreff_dn9 * locals.var_fcinracc_i)) / (2.0 * assign54110_e69216)),)
    } else {
        (locals.var_fqinr, locals.var_fqinr_dn4, locals.var_fqinr_dn6, locals.var_fqinr_dn7, locals.var_fqinr_dn8, locals.var_fqinr_dn9,)
    }
};
        locals.var_fqinr = assign54110_e69218;
        locals.var_fqinr_dn4 = assign54110_e69218_d_n4;
        locals.var_fqinr_dn6 = assign54110_e69218_d_n6;
        locals.var_fqinr_dn7 = assign54110_e69218_d_n7;
        locals.var_fqinr_dn8 = assign54110_e69218_d_n8;
        locals.var_fqinr_dn9 = assign54110_e69218_d_n9;
        locals.var_fqinr_rv = 0.0;

        let (assign54120_e69232, assign54120_e69232_d_n4, assign54120_e69232_d_n6, assign54120_e69232_d_n7, assign54120_e69232_d_n8, assign54120_e69232_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
        let assign54120_e69224: f64 = (1.0 - locals.var_fqinr);
        let assign54120_e69226: f64 = (assign54120_e69224 / locals.var_fcinracc_i);
        let assign54120_e69228: f64 = (assign54120_e69226 + locals.var_vginr);
        let assign54120_e69230: f64 = (assign54120_e69228 - locals.var_vginreff);
        (assign54120_e69230, ((((-locals.var_fqinr_dn4) / locals.var_fcinracc_i) + locals.var_vginr_dn4) - locals.var_vginreff_dn4), ((((-locals.var_fqinr_dn6) / locals.var_fcinracc_i) + locals.var_vginr_dn6) - locals.var_vginreff_dn6), ((((-locals.var_fqinr_dn7) / locals.var_fcinracc_i) + locals.var_vginr_dn7) - locals.var_vginreff_dn7), ((((-locals.var_fqinr_dn8) / locals.var_fcinracc_i) + locals.var_vginr_dn8) - locals.var_vginreff_dn8), ((((-locals.var_fqinr_dn9) / locals.var_fcinracc_i) + locals.var_vginr_dn9) - locals.var_vginreff_dn9),)
    } else {
        (locals.var_dvinracc, locals.var_dvinracc_dn4, locals.var_dvinracc_dn6, locals.var_dvinracc_dn7, locals.var_dvinracc_dn8, locals.var_dvinracc_dn9,)
    }
};
        locals.var_dvinracc = assign54120_e69232;
        locals.var_dvinracc_dn4 = assign54120_e69232_d_n4;
        locals.var_dvinracc_dn6 = assign54120_e69232_d_n6;
        locals.var_dvinracc_dn7 = assign54120_e69232_d_n7;
        locals.var_dvinracc_dn8 = assign54120_e69232_d_n8;
        locals.var_dvinracc_dn9 = assign54120_e69232_d_n9;
        locals.var_dvinracc_rv = 0.0;

        let (assign54130_e69256, assign54130_e69256_d_n4, assign54130_e69256_d_n6, assign54130_e69256_d_n7, assign54130_e69256_d_n8, assign54130_e69256_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
        let assign54130_e69238: f64 = (0.5 / locals.var_fqinr);
        let assign54130_e69240: f64 = (assign54130_e69238 - 1.0);
        let assign54130_e69245: f64 = (locals.var_vinr_max - locals.var_temp__blk949);
        let assign54130_e69246: f64 = (locals.var_vginr * assign54130_e69245);
        let assign54130_e69247: f64 = (locals.var_temp1 + assign54130_e69246);
        let assign54130_e69248: f64 = (assign54130_e69240 * assign54130_e69247);
        let assign54130_e69250: f64 = (assign54130_e69248 * locals.var_temp2);
        let assign54130_e69252: f64 = (assign54130_e69250 / locals.var_temp1);
        let assign54130_e69254: f64 = (assign54130_e69252 + 1.0);
        (assign54130_e69254, ((((((((-((0.5 * locals.var_fqinr_dn4) / (locals.var_fqinr * locals.var_fqinr))) * assign54130_e69247) + (assign54130_e69240 * (locals.var_temp1_dn4 + ((locals.var_vginr_dn4 * assign54130_e69245) + (locals.var_vginr * (-locals.var_temp__blk949_dn4)))))) * locals.var_temp2) + (assign54130_e69248 * locals.var_temp2_dn4)) * locals.var_temp1) - (assign54130_e69250 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn6) / (locals.var_fqinr * locals.var_fqinr))) * assign54130_e69247) + (assign54130_e69240 * (locals.var_temp1_dn6 + ((locals.var_vginr_dn6 * assign54130_e69245) + (locals.var_vginr * (-locals.var_temp__blk949_dn6)))))) * locals.var_temp2) + (assign54130_e69248 * locals.var_temp2_dn6)) * locals.var_temp1) - (assign54130_e69250 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn7) / (locals.var_fqinr * locals.var_fqinr))) * assign54130_e69247) + (assign54130_e69240 * (locals.var_temp1_dn7 + ((locals.var_vginr_dn7 * assign54130_e69245) + (locals.var_vginr * (-locals.var_temp__blk949_dn7)))))) * locals.var_temp2) + (assign54130_e69248 * locals.var_temp2_dn7)) * locals.var_temp1) - (assign54130_e69250 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn8) / (locals.var_fqinr * locals.var_fqinr))) * assign54130_e69247) + (assign54130_e69240 * (locals.var_temp1_dn8 + ((locals.var_vginr_dn8 * assign54130_e69245) + (locals.var_vginr * (-locals.var_temp__blk949_dn8)))))) * locals.var_temp2) + (assign54130_e69248 * locals.var_temp2_dn8)) * locals.var_temp1) - (assign54130_e69250 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn9) / (locals.var_fqinr * locals.var_fqinr))) * assign54130_e69247) + (assign54130_e69240 * (locals.var_temp1_dn9 + ((locals.var_vginr_dn9 * assign54130_e69245) + (locals.var_vginr * (-locals.var_temp__blk949_dn9)))))) * locals.var_temp2) + (assign54130_e69248 * locals.var_temp2_dn9)) * locals.var_temp1) - (assign54130_e69250 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)),)
    } else {
        (locals.var_finracc, locals.var_finracc_dn4, locals.var_finracc_dn6, locals.var_finracc_dn7, locals.var_finracc_dn8, locals.var_finracc_dn9,)
    }
};
        locals.var_finracc = assign54130_e69256;
        locals.var_finracc_dn4 = assign54130_e69256_d_n4;
        locals.var_finracc_dn6 = assign54130_e69256_d_n6;
        locals.var_finracc_dn7 = assign54130_e69256_d_n7;
        locals.var_finracc_dn8 = assign54130_e69256_d_n8;
        locals.var_finracc_dn9 = assign54130_e69256_d_n9;
        locals.var_finracc_rv = 0.0;

        let (assign54140_e69260, assign54140_e69260_d_n4, assign54140_e69260_d_n6, assign54140_e69260_d_n7, assign54140_e69260_d_n8, assign54140_e69260_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_finrdep, locals.var_finrdep_dn4, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8, locals.var_finrdep_dn9,)
    }
};
        locals.var_finrdep = assign54140_e69260;
        locals.var_finrdep_dn4 = assign54140_e69260_d_n4;
        locals.var_finrdep_dn6 = assign54140_e69260_d_n6;
        locals.var_finrdep_dn7 = assign54140_e69260_d_n7;
        locals.var_finrdep_dn8 = assign54140_e69260_d_n8;
        locals.var_finrdep_dn9 = assign54140_e69260_d_n9;
        locals.var_finrdep_rv = 0.0;

        let (assign54150_e69264, assign54150_e69264_d_n4, assign54150_e69264_d_n6, assign54150_e69264_d_n7, assign54150_e69264_d_n8, assign54150_e69264_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvinrdep, locals.var_dvinrdep_dn4, locals.var_dvinrdep_dn6, locals.var_dvinrdep_dn7, locals.var_dvinrdep_dn8, locals.var_dvinrdep_dn9,)
    }
};
        locals.var_dvinrdep = assign54150_e69264;
        locals.var_dvinrdep_dn4 = assign54150_e69264_d_n4;
        locals.var_dvinrdep_dn6 = assign54150_e69264_d_n6;
        locals.var_dvinrdep_dn7 = assign54150_e69264_d_n7;
        locals.var_dvinrdep_dn8 = assign54150_e69264_d_n8;
        locals.var_dvinrdep_dn9 = assign54150_e69264_d_n9;
        locals.var_dvinrdep_rv = 0.0;

        let assign54160_e69267: f64 = if locals.var_fcinrdep_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1529 = assign54160_e69267;
        locals.var_guard1529_rv = 0.0;

        let (assign54170_e69283, assign54170_e69283_d_n4, assign54170_e69283_d_n6, assign54170_e69283_d_n7, assign54170_e69283_d_n8, assign54170_e69283_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) {
        let assign54170_e69273: f64 = (0.5 * locals.var_phib_ac);
        let assign54170_e69278: f64 = (locals.var_gf_ac * 0.7071067811865475);
        let assign54170_e69279: f64 = (1.0 + assign54170_e69278);
        let assign54170_e69280: f64 = (locals.var_phit1_ac * assign54170_e69279);
        let assign54170_e69281: f64 = (assign54170_e69273 + assign54170_e69280);
        (assign54170_e69281, ((0.5 * locals.var_phib_ac_dn4) + ((locals.var_phit1_ac_dn4 * assign54170_e69279) + (locals.var_phit1_ac * (locals.var_gf_ac_dn4 * 0.7071067811865475)))), ((locals.var_phit1_ac_dn6 * assign54170_e69279) + (locals.var_phit1_ac * (locals.var_gf_ac_dn6 * 0.7071067811865475))), ((locals.var_phit1_ac_dn7 * assign54170_e69279) + (locals.var_phit1_ac * (locals.var_gf_ac_dn7 * 0.7071067811865475))), ((locals.var_phit1_ac_dn8 * assign54170_e69279) + (locals.var_phit1_ac * (locals.var_gf_ac_dn8 * 0.7071067811865475))), ((locals.var_phit1_ac_dn9 * assign54170_e69279) + (locals.var_phit1_ac * (locals.var_gf_ac_dn9 * 0.7071067811865475))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign54170_e69283;
        locals.var_temp__blk949_dn4 = assign54170_e69283_d_n4;
        locals.var_temp__blk949_dn6 = assign54170_e69283_d_n6;
        locals.var_temp__blk949_dn7 = assign54170_e69283_d_n7;
        locals.var_temp__blk949_dn8 = assign54170_e69283_d_n8;
        locals.var_temp__blk949_dn9 = assign54170_e69283_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign54180_e69291, assign54180_e69291_d_n4, assign54180_e69291_d_n6, assign54180_e69291_d_n7, assign54180_e69291_d_n8, assign54180_e69291_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) {
        let assign54180_e69289: f64 = (locals.var_vgb1_ac / locals.var_temp__blk949);
        (assign54180_e69289, (((locals.var_vgb1_ac_dn4 * locals.var_temp__blk949) - (locals.var_vgb1_ac * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_vgb1_ac_dn6 * locals.var_temp__blk949) - (locals.var_vgb1_ac * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_vgb1_ac_dn7 * locals.var_temp__blk949) - (locals.var_vgb1_ac * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_vgb1_ac_dn8 * locals.var_temp__blk949) - (locals.var_vgb1_ac * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_vgb1_ac_dn9 * locals.var_temp__blk949) - (locals.var_vgb1_ac * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)),)
    } else {
        (locals.var_xginrdep, locals.var_xginrdep_dn4, locals.var_xginrdep_dn6, locals.var_xginrdep_dn7, locals.var_xginrdep_dn8, locals.var_xginrdep_dn9,)
    }
};
        locals.var_xginrdep = assign54180_e69291;
        locals.var_xginrdep_dn4 = assign54180_e69291_d_n4;
        locals.var_xginrdep_dn6 = assign54180_e69291_d_n6;
        locals.var_xginrdep_dn7 = assign54180_e69291_d_n7;
        locals.var_xginrdep_dn8 = assign54180_e69291_d_n8;
        locals.var_xginrdep_dn9 = assign54180_e69291_d_n9;
        locals.var_xginrdep_rv = 0.0;

        let assign54190_e69293: f64 = (locals.var_xginrdep).abs();
        let assign54190_e69295: f64 = if assign54190_e69293 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1530 = assign54190_e69295;
        locals.var_guard1530_rv = 0.0;

        let (assign54200_e69309, assign54200_e69309_d_n4, assign54200_e69309_d_n6, assign54200_e69309_d_n7, assign54200_e69309_d_n8, assign54200_e69309_d_n9,) = {
    if (((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) && (locals.var_guard1530 != 0.0)) {
        let assign54200_e69304: f64 = (-locals.var_xginrdep);
        let assign54200_e69305: f64 = (assign54200_e69304).exp();
        let assign54200_e69306: f64 = (1.0 + assign54200_e69305);
        let assign54200_e69307: f64 = (1.0 / assign54200_e69306);
        (assign54200_e69307, (-((assign54200_e69305 * (-locals.var_xginrdep_dn4)) / (assign54200_e69306 * assign54200_e69306))), (-((assign54200_e69305 * (-locals.var_xginrdep_dn6)) / (assign54200_e69306 * assign54200_e69306))), (-((assign54200_e69305 * (-locals.var_xginrdep_dn7)) / (assign54200_e69306 * assign54200_e69306))), (-((assign54200_e69305 * (-locals.var_xginrdep_dn8)) / (assign54200_e69306 * assign54200_e69306))), (-((assign54200_e69305 * (-locals.var_xginrdep_dn9)) / (assign54200_e69306 * assign54200_e69306))),)
    } else {
        (locals.var_finrdep, locals.var_finrdep_dn4, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8, locals.var_finrdep_dn9,)
    }
};
        locals.var_finrdep = assign54200_e69309;
        locals.var_finrdep_dn4 = assign54200_e69309_d_n4;
        locals.var_finrdep_dn6 = assign54200_e69309_d_n6;
        locals.var_finrdep_dn7 = assign54200_e69309_d_n7;
        locals.var_finrdep_dn8 = assign54200_e69309_d_n8;
        locals.var_finrdep_dn9 = assign54200_e69309_d_n9;
        locals.var_finrdep_rv = 0.0;

        let assign54210_e69312: f64 = if locals.var_xginrdep < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1531 = assign54210_e69312;
        locals.var_guard1531_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        locals: &mut StampLocals,
    ) {
        let (assign54220_e69348, assign54220_e69348_d_n4, assign54220_e69348_d_n6, assign54220_e69348_d_n7, assign54220_e69348_d_n8, assign54220_e69348_d_n9,) = {
    if ((((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) && (locals.var_guard1530 == 0.0)) && (locals.var_guard1531 != 0.0)) {
        let assign54220_e69324: f64 = (-230.25850929940458);
        let assign54220_e69326: f64 = (assign54220_e69324 + locals.var_xginrdep);
        let assign54220_e69330: f64 = (-230.25850929940458);
        let assign54220_e69332: f64 = (assign54220_e69330 + locals.var_xginrdep);
        let assign54220_e69335: f64 = (-230.25850929940458);
        let assign54220_e69337: f64 = (assign54220_e69335 + locals.var_xginrdep);
        let assign54220_e69339: f64 = (assign54220_e69337 * 0.3333333333333333);
        let assign54220_e69340: f64 = (1.0 + assign54220_e69339);
        let assign54220_e69341: f64 = (assign54220_e69332 * assign54220_e69340);
        let assign54220_e69342: f64 = (0.5 * assign54220_e69341);
        let assign54220_e69343: f64 = (1.0 + assign54220_e69342);
        let assign54220_e69344: f64 = (assign54220_e69326 * assign54220_e69343);
        let assign54220_e69345: f64 = (1.0 + assign54220_e69344);
        let assign54220_e69346: f64 = (1e-100 / assign54220_e69345);
        (assign54220_e69346, (-((1e-100 * ((locals.var_xginrdep_dn4 * assign54220_e69343) + (assign54220_e69326 * (0.5 * ((locals.var_xginrdep_dn4 * assign54220_e69340) + (assign54220_e69332 * (locals.var_xginrdep_dn4 * 0.3333333333333333))))))) / (assign54220_e69345 * assign54220_e69345))), (-((1e-100 * ((locals.var_xginrdep_dn6 * assign54220_e69343) + (assign54220_e69326 * (0.5 * ((locals.var_xginrdep_dn6 * assign54220_e69340) + (assign54220_e69332 * (locals.var_xginrdep_dn6 * 0.3333333333333333))))))) / (assign54220_e69345 * assign54220_e69345))), (-((1e-100 * ((locals.var_xginrdep_dn7 * assign54220_e69343) + (assign54220_e69326 * (0.5 * ((locals.var_xginrdep_dn7 * assign54220_e69340) + (assign54220_e69332 * (locals.var_xginrdep_dn7 * 0.3333333333333333))))))) / (assign54220_e69345 * assign54220_e69345))), (-((1e-100 * ((locals.var_xginrdep_dn8 * assign54220_e69343) + (assign54220_e69326 * (0.5 * ((locals.var_xginrdep_dn8 * assign54220_e69340) + (assign54220_e69332 * (locals.var_xginrdep_dn8 * 0.3333333333333333))))))) / (assign54220_e69345 * assign54220_e69345))), (-((1e-100 * ((locals.var_xginrdep_dn9 * assign54220_e69343) + (assign54220_e69326 * (0.5 * ((locals.var_xginrdep_dn9 * assign54220_e69340) + (assign54220_e69332 * (locals.var_xginrdep_dn9 * 0.3333333333333333))))))) / (assign54220_e69345 * assign54220_e69345))),)
    } else {
        (locals.var_finrdep, locals.var_finrdep_dn4, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8, locals.var_finrdep_dn9,)
    }
};
        locals.var_finrdep = assign54220_e69348;
        locals.var_finrdep_dn4 = assign54220_e69348_d_n4;
        locals.var_finrdep_dn6 = assign54220_e69348_d_n6;
        locals.var_finrdep_dn7 = assign54220_e69348_d_n7;
        locals.var_finrdep_dn8 = assign54220_e69348_d_n8;
        locals.var_finrdep_dn9 = assign54220_e69348_d_n9;
        locals.var_finrdep_rv = 0.0;

        let assign54230_e69351: f64 = if locals.var_xginrdep < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1532 = assign54230_e69351;
        locals.var_guard1532_rv = 0.0;

        let (assign54240_e69363, assign54240_e69363_d_n4, assign54240_e69363_d_n6, assign54240_e69363_d_n7, assign54240_e69363_d_n8, assign54240_e69363_d_n9,) = {
    if (((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) && (locals.var_guard1532 != 0.0)) {
        let assign54240_e69359: f64 = (locals.var_xginrdep).exp();
        let assign54240_e69360: f64 = (1.0 + assign54240_e69359);
        let assign54240_e69361: f64 = (assign54240_e69360).ln();
        (assign54240_e69361, ((assign54240_e69359 * locals.var_xginrdep_dn4) / assign54240_e69360), ((assign54240_e69359 * locals.var_xginrdep_dn6) / assign54240_e69360), ((assign54240_e69359 * locals.var_xginrdep_dn7) / assign54240_e69360), ((assign54240_e69359 * locals.var_xginrdep_dn8) / assign54240_e69360), ((assign54240_e69359 * locals.var_xginrdep_dn9) / assign54240_e69360),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54240_e69363;
        locals.var_temp1_dn4 = assign54240_e69363_d_n4;
        locals.var_temp1_dn6 = assign54240_e69363_d_n6;
        locals.var_temp1_dn7 = assign54240_e69363_d_n7;
        locals.var_temp1_dn8 = assign54240_e69363_d_n8;
        locals.var_temp1_dn9 = assign54240_e69363_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign54250_e69372, assign54250_e69372_d_n4, assign54250_e69372_d_n6, assign54250_e69372_d_n7, assign54250_e69372_d_n8, assign54250_e69372_d_n9,) = {
    if (((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) && (locals.var_guard1532 == 0.0)) {
        (locals.var_xginrdep, locals.var_xginrdep_dn4, locals.var_xginrdep_dn6, locals.var_xginrdep_dn7, locals.var_xginrdep_dn8, locals.var_xginrdep_dn9,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54250_e69372;
        locals.var_temp1_dn4 = assign54250_e69372_d_n4;
        locals.var_temp1_dn6 = assign54250_e69372_d_n6;
        locals.var_temp1_dn7 = assign54250_e69372_d_n7;
        locals.var_temp1_dn8 = assign54250_e69372_d_n8;
        locals.var_temp1_dn9 = assign54250_e69372_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign54260_e69380, assign54260_e69380_d_n4, assign54260_e69380_d_n6, assign54260_e69380_d_n7, assign54260_e69380_d_n8, assign54260_e69380_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) {
        let assign54260_e69378: f64 = (locals.var_temp__blk949 * locals.var_temp1);
        (assign54260_e69378, ((locals.var_temp__blk949_dn4 * locals.var_temp1) + (locals.var_temp__blk949 * locals.var_temp1_dn4)), ((locals.var_temp__blk949_dn6 * locals.var_temp1) + (locals.var_temp__blk949 * locals.var_temp1_dn6)), ((locals.var_temp__blk949_dn7 * locals.var_temp1) + (locals.var_temp__blk949 * locals.var_temp1_dn7)), ((locals.var_temp__blk949_dn8 * locals.var_temp1) + (locals.var_temp__blk949 * locals.var_temp1_dn8)), ((locals.var_temp__blk949_dn9 * locals.var_temp1) + (locals.var_temp__blk949 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_dvinrdep, locals.var_dvinrdep_dn4, locals.var_dvinrdep_dn6, locals.var_dvinrdep_dn7, locals.var_dvinrdep_dn8, locals.var_dvinrdep_dn9,)
    }
};
        locals.var_dvinrdep = assign54260_e69380;
        locals.var_dvinrdep_dn4 = assign54260_e69380_d_n4;
        locals.var_dvinrdep_dn6 = assign54260_e69380_d_n6;
        locals.var_dvinrdep_dn7 = assign54260_e69380_d_n7;
        locals.var_dvinrdep_dn8 = assign54260_e69380_d_n8;
        locals.var_dvinrdep_dn9 = assign54260_e69380_d_n9;
        locals.var_dvinrdep_rv = 0.0;

        let (assign54270_e69390, assign54270_e69390_d_n4, assign54270_e69390_d_n6, assign54270_e69390_d_n7, assign54270_e69390_d_n8, assign54270_e69390_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        let assign54270_e69385: f64 = (locals.var_finrdep - locals.var_finracc);
        let assign54270_e69386: f64 = (locals.var_fcinrdep_i * assign54270_e69385);
        let assign54270_e69388: f64 = (assign54270_e69386 + locals.var_finracc);
        (assign54270_e69388, ((locals.var_fcinrdep_i * (locals.var_finrdep_dn4 - locals.var_finracc_dn4)) + locals.var_finracc_dn4), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn6 - locals.var_finracc_dn6)) + locals.var_finracc_dn6), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn7 - locals.var_finracc_dn7)) + locals.var_finracc_dn7), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn8 - locals.var_finracc_dn8)) + locals.var_finracc_dn8), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn9 - locals.var_finracc_dn9)) + locals.var_finracc_dn9),)
    } else {
        (locals.var_finr, locals.var_finr_dn4, locals.var_finr_dn6, locals.var_finr_dn7, locals.var_finr_dn8, locals.var_finr_dn9,)
    }
};
        locals.var_finr = assign54270_e69390;
        locals.var_finr_dn4 = assign54270_e69390_d_n4;
        locals.var_finr_dn6 = assign54270_e69390_d_n6;
        locals.var_finr_dn7 = assign54270_e69390_d_n7;
        locals.var_finr_dn8 = assign54270_e69390_d_n8;
        locals.var_finr_dn9 = assign54270_e69390_d_n9;
        locals.var_finr_rv = 0.0;

        let (assign54280_e69400, assign54280_e69400_d_n4, assign54280_e69400_d_n6, assign54280_e69400_d_n7, assign54280_e69400_d_n8, assign54280_e69400_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        let assign54280_e69395: f64 = (locals.var_dvinrdep - locals.var_dvinracc);
        let assign54280_e69396: f64 = (locals.var_fcinrdep_i * assign54280_e69395);
        let assign54280_e69398: f64 = (assign54280_e69396 + locals.var_dvinracc);
        (assign54280_e69398, ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn4 - locals.var_dvinracc_dn4)) + locals.var_dvinracc_dn4), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn6 - locals.var_dvinracc_dn6)) + locals.var_dvinracc_dn6), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn7 - locals.var_dvinracc_dn7)) + locals.var_dvinracc_dn7), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn8 - locals.var_dvinracc_dn8)) + locals.var_dvinracc_dn8), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn9 - locals.var_dvinracc_dn9)) + locals.var_dvinracc_dn9),)
    } else {
        (locals.var_dvinr, locals.var_dvinr_dn4, locals.var_dvinr_dn6, locals.var_dvinr_dn7, locals.var_dvinr_dn8, locals.var_dvinr_dn9,)
    }
};
        locals.var_dvinr = assign54280_e69400;
        locals.var_dvinr_dn4 = assign54280_e69400_d_n4;
        locals.var_dvinr_dn6 = assign54280_e69400_d_n6;
        locals.var_dvinr_dn7 = assign54280_e69400_d_n7;
        locals.var_dvinr_dn8 = assign54280_e69400_d_n8;
        locals.var_dvinr_dn9 = assign54280_e69400_d_n9;
        locals.var_dvinr_rv = 0.0;

        let (assign54290_e69414, assign54290_e69414_d_n4, assign54290_e69414_d_n6, assign54290_e69414_d_n7, assign54290_e69414_d_n8, assign54290_e69414_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        let assign54290_e69405: f64 = (locals.var_phit1_ac * locals.var_xno_s_ac);
        let assign54290_e69406: f64 = (locals.var_vgb1_ac - assign54290_e69405);
        let assign54290_e69408: f64 = (assign54290_e69406 - locals.var_voxm_ac);
        let assign54290_e69411: f64 = (0.5 * locals.var_dps_ac);
        let assign54290_e69412: f64 = (assign54290_e69408 - assign54290_e69411);
        (assign54290_e69412, (((locals.var_vgb1_ac_dn4 - ((locals.var_phit1_ac_dn4 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn4))) - locals.var_voxm_ac_dn4) - (0.5 * locals.var_dps_ac_dn4)), (((locals.var_vgb1_ac_dn6 - ((locals.var_phit1_ac_dn6 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn6))) - locals.var_voxm_ac_dn6) - (0.5 * locals.var_dps_ac_dn6)), (((locals.var_vgb1_ac_dn7 - ((locals.var_phit1_ac_dn7 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn7))) - locals.var_voxm_ac_dn7) - (0.5 * locals.var_dps_ac_dn7)), (((locals.var_vgb1_ac_dn8 - ((locals.var_phit1_ac_dn8 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn8))) - locals.var_voxm_ac_dn8) - (0.5 * locals.var_dps_ac_dn8)), (((locals.var_vgb1_ac_dn9 - ((locals.var_phit1_ac_dn9 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn9))) - locals.var_voxm_ac_dn9) - (0.5 * locals.var_dps_ac_dn9)),)
    } else {
        (locals.var_vgsinr, locals.var_vgsinr_dn4, locals.var_vgsinr_dn6, locals.var_vgsinr_dn7, locals.var_vgsinr_dn8, locals.var_vgsinr_dn9,)
    }
};
        locals.var_vgsinr = assign54290_e69414;
        locals.var_vgsinr_dn4 = assign54290_e69414_d_n4;
        locals.var_vgsinr_dn6 = assign54290_e69414_d_n6;
        locals.var_vgsinr_dn7 = assign54290_e69414_d_n7;
        locals.var_vgsinr_dn8 = assign54290_e69414_d_n8;
        locals.var_vgsinr_dn9 = assign54290_e69414_d_n9;
        locals.var_vgsinr_rv = 0.0;

        let (assign54300_e69422, assign54300_e69422_d_n4, assign54300_e69422_d_n6, assign54300_e69422_d_n7, assign54300_e69422_d_n8, assign54300_e69422_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        let assign54300_e69418: f64 = (locals.var_vgb1_ac - locals.var_vgsinr);
        let assign54300_e69420: f64 = (assign54300_e69418 - locals.var_qbs_ac);
        (assign54300_e69420, ((locals.var_vgb1_ac_dn4 - locals.var_vgsinr_dn4) - locals.var_qbs_ac_dn4), ((locals.var_vgb1_ac_dn6 - locals.var_vgsinr_dn6) - locals.var_qbs_ac_dn6), ((locals.var_vgb1_ac_dn7 - locals.var_vgsinr_dn7) - locals.var_qbs_ac_dn7), ((locals.var_vgb1_ac_dn8 - locals.var_vgsinr_dn8) - locals.var_qbs_ac_dn8), ((locals.var_vgb1_ac_dn9 - locals.var_vgsinr_dn9) - locals.var_qbs_ac_dn9),)
    } else {
        (locals.var_vsginr, locals.var_vsginr_dn4, locals.var_vsginr_dn6, locals.var_vsginr_dn7, locals.var_vsginr_dn8, locals.var_vsginr_dn9,)
    }
};
        locals.var_vsginr = assign54300_e69422;
        locals.var_vsginr_dn4 = assign54300_e69422_d_n4;
        locals.var_vsginr_dn6 = assign54300_e69422_d_n6;
        locals.var_vsginr_dn7 = assign54300_e69422_d_n7;
        locals.var_vsginr_dn8 = assign54300_e69422_d_n8;
        locals.var_vsginr_dn9 = assign54300_e69422_d_n9;
        locals.var_vsginr_rv = 0.0;

        let (assign54310_e69430, assign54310_e69430_d_n4, assign54310_e69430_d_n6, assign54310_e69430_d_n7, assign54310_e69430_d_n8, assign54310_e69430_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        let assign54310_e69426: f64 = (locals.var_dps_ac + locals.var_vgsinr);
        let assign54310_e69428: f64 = (assign54310_e69426 - locals.var_v_ds);
        (assign54310_e69428, (locals.var_dps_ac_dn4 + locals.var_vgsinr_dn4), (locals.var_dps_ac_dn6 + locals.var_vgsinr_dn6), ((locals.var_dps_ac_dn7 + locals.var_vgsinr_dn7) - locals.var_v_ds_dn7), ((locals.var_dps_ac_dn8 + locals.var_vgsinr_dn8) - locals.var_v_ds_dn8), (locals.var_dps_ac_dn9 + locals.var_vgsinr_dn9),)
    } else {
        (locals.var_vgdinr, locals.var_vgdinr_dn4, locals.var_vgdinr_dn6, locals.var_vgdinr_dn7, locals.var_vgdinr_dn8, locals.var_vgdinr_dn9,)
    }
};
        locals.var_vgdinr = assign54310_e69430;
        locals.var_vgdinr_dn4 = assign54310_e69430_d_n4;
        locals.var_vgdinr_dn6 = assign54310_e69430_d_n6;
        locals.var_vgdinr_dn7 = assign54310_e69430_d_n7;
        locals.var_vgdinr_dn8 = assign54310_e69430_d_n8;
        locals.var_vgdinr_dn9 = assign54310_e69430_d_n9;
        locals.var_vgdinr_rv = 0.0;

        let (assign54320_e69438, assign54320_e69438_d_n4, assign54320_e69438_d_n6, assign54320_e69438_d_n7, assign54320_e69438_d_n8, assign54320_e69438_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        let assign54320_e69434: f64 = (locals.var_vgb1_ac - locals.var_vgdinr);
        let assign54320_e69436: f64 = (assign54320_e69434 - locals.var_qbd_ac);
        (assign54320_e69436, ((locals.var_vgb1_ac_dn4 - locals.var_vgdinr_dn4) - locals.var_qbd_ac_dn4), ((locals.var_vgb1_ac_dn6 - locals.var_vgdinr_dn6) - locals.var_qbd_ac_dn6), ((locals.var_vgb1_ac_dn7 - locals.var_vgdinr_dn7) - locals.var_qbd_ac_dn7), ((locals.var_vgb1_ac_dn8 - locals.var_vgdinr_dn8) - locals.var_qbd_ac_dn8), ((locals.var_vgb1_ac_dn9 - locals.var_vgdinr_dn9) - locals.var_qbd_ac_dn9),)
    } else {
        (locals.var_vdginr, locals.var_vdginr_dn4, locals.var_vdginr_dn6, locals.var_vdginr_dn7, locals.var_vdginr_dn8, locals.var_vdginr_dn9,)
    }
};
        locals.var_vdginr = assign54320_e69438;
        locals.var_vdginr_dn4 = assign54320_e69438_d_n4;
        locals.var_vdginr_dn6 = assign54320_e69438_d_n6;
        locals.var_vdginr_dn7 = assign54320_e69438_d_n7;
        locals.var_vdginr_dn8 = assign54320_e69438_d_n8;
        locals.var_vdginr_dn9 = assign54320_e69438_d_n9;
        locals.var_vdginr_rv = 0.0;

        let assign54330_e69441: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1533 = assign54330_e69441;
        locals.var_guard1533_rv = 0.0;

        let (assign54340_e69455, assign54340_e69455_d_n4, assign54340_e69455_d_n6, assign54340_e69455_d_n7, assign54340_e69455_d_n8, assign54340_e69455_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 != 0.0)) {
        let assign54340_e69448: f64 = (locals.var_cinrd_i * locals.var_vgdinr);
        let assign54340_e69451: f64 = (locals.var_cinr_i * locals.var_vgsinr);
        let assign54340_e69452: f64 = (assign54340_e69448 + assign54340_e69451);
        let assign54340_e69453: f64 = (locals.var_finr * assign54340_e69452);
        (assign54340_e69453, ((locals.var_finr_dn4 * assign54340_e69452) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn4) + (locals.var_cinr_i * locals.var_vgsinr_dn4)))), ((locals.var_finr_dn6 * assign54340_e69452) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn6) + (locals.var_cinr_i * locals.var_vgsinr_dn6)))), ((locals.var_finr_dn7 * assign54340_e69452) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn7) + (locals.var_cinr_i * locals.var_vgsinr_dn7)))), ((locals.var_finr_dn8 * assign54340_e69452) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn8) + (locals.var_cinr_i * locals.var_vgsinr_dn8)))), ((locals.var_finr_dn9 * assign54340_e69452) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn9) + (locals.var_cinr_i * locals.var_vgsinr_dn9)))),)
    } else {
        (locals.var_qginr, locals.var_qginr_dn4, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8, locals.var_qginr_dn9,)
    }
};
        locals.var_qginr = assign54340_e69455;
        locals.var_qginr_dn4 = assign54340_e69455_d_n4;
        locals.var_qginr_dn6 = assign54340_e69455_d_n6;
        locals.var_qginr_dn7 = assign54340_e69455_d_n7;
        locals.var_qginr_dn8 = assign54340_e69455_d_n8;
        locals.var_qginr_dn9 = assign54340_e69455_d_n9;
        locals.var_qginr_rv = 0.0;

        let (assign54350_e69465, assign54350_e69465_d_n4, assign54350_e69465_d_n6, assign54350_e69465_d_n7, assign54350_e69465_d_n8, assign54350_e69465_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 != 0.0)) {
        let assign54350_e69462: f64 = (locals.var_vsginr - locals.var_dvinr);
        let assign54350_e69463: f64 = (locals.var_cinr_i * assign54350_e69462);
        (assign54350_e69463, (locals.var_cinr_i * (locals.var_vsginr_dn4 - locals.var_dvinr_dn4)), (locals.var_cinr_i * (locals.var_vsginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinr_i * (locals.var_vsginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinr_i * (locals.var_vsginr_dn8 - locals.var_dvinr_dn8)), (locals.var_cinr_i * (locals.var_vsginr_dn9 - locals.var_dvinr_dn9)),)
    } else {
        (locals.var_qsinr, locals.var_qsinr_dn4, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8, locals.var_qsinr_dn9,)
    }
};
        locals.var_qsinr = assign54350_e69465;
        locals.var_qsinr_dn4 = assign54350_e69465_d_n4;
        locals.var_qsinr_dn6 = assign54350_e69465_d_n6;
        locals.var_qsinr_dn7 = assign54350_e69465_d_n7;
        locals.var_qsinr_dn8 = assign54350_e69465_d_n8;
        locals.var_qsinr_dn9 = assign54350_e69465_d_n9;
        locals.var_qsinr_rv = 0.0;

        let (assign54360_e69475, assign54360_e69475_d_n4, assign54360_e69475_d_n6, assign54360_e69475_d_n7, assign54360_e69475_d_n8, assign54360_e69475_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 != 0.0)) {
        let assign54360_e69472: f64 = (locals.var_vdginr - locals.var_dvinr);
        let assign54360_e69473: f64 = (locals.var_cinrd_i * assign54360_e69472);
        (assign54360_e69473, (locals.var_cinrd_i * (locals.var_vdginr_dn4 - locals.var_dvinr_dn4)), (locals.var_cinrd_i * (locals.var_vdginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinrd_i * (locals.var_vdginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinrd_i * (locals.var_vdginr_dn8 - locals.var_dvinr_dn8)), (locals.var_cinrd_i * (locals.var_vdginr_dn9 - locals.var_dvinr_dn9)),)
    } else {
        (locals.var_qdinr, locals.var_qdinr_dn4, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8, locals.var_qdinr_dn9,)
    }
};
        locals.var_qdinr = assign54360_e69475;
        locals.var_qdinr_dn4 = assign54360_e69475_d_n4;
        locals.var_qdinr_dn6 = assign54360_e69475_d_n6;
        locals.var_qdinr_dn7 = assign54360_e69475_d_n7;
        locals.var_qdinr_dn8 = assign54360_e69475_d_n8;
        locals.var_qdinr_dn9 = assign54360_e69475_d_n9;
        locals.var_qdinr_rv = 0.0;

        let (assign54370_e69490, assign54370_e69490_d_n4, assign54370_e69490_d_n6, assign54370_e69490_d_n7, assign54370_e69490_d_n8, assign54370_e69490_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 == 0.0)) {
        let assign54370_e69483: f64 = (locals.var_cinr_i * locals.var_vgdinr);
        let assign54370_e69486: f64 = (locals.var_cinrd_i * locals.var_vgsinr);
        let assign54370_e69487: f64 = (assign54370_e69483 + assign54370_e69486);
        let assign54370_e69488: f64 = (locals.var_finr * assign54370_e69487);
        (assign54370_e69488, ((locals.var_finr_dn4 * assign54370_e69487) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn4) + (locals.var_cinrd_i * locals.var_vgsinr_dn4)))), ((locals.var_finr_dn6 * assign54370_e69487) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn6) + (locals.var_cinrd_i * locals.var_vgsinr_dn6)))), ((locals.var_finr_dn7 * assign54370_e69487) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn7) + (locals.var_cinrd_i * locals.var_vgsinr_dn7)))), ((locals.var_finr_dn8 * assign54370_e69487) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn8) + (locals.var_cinrd_i * locals.var_vgsinr_dn8)))), ((locals.var_finr_dn9 * assign54370_e69487) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn9) + (locals.var_cinrd_i * locals.var_vgsinr_dn9)))),)
    } else {
        (locals.var_qginr, locals.var_qginr_dn4, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8, locals.var_qginr_dn9,)
    }
};
        locals.var_qginr = assign54370_e69490;
        locals.var_qginr_dn4 = assign54370_e69490_d_n4;
        locals.var_qginr_dn6 = assign54370_e69490_d_n6;
        locals.var_qginr_dn7 = assign54370_e69490_d_n7;
        locals.var_qginr_dn8 = assign54370_e69490_d_n8;
        locals.var_qginr_dn9 = assign54370_e69490_d_n9;
        locals.var_qginr_rv = 0.0;

        let (assign54380_e69501, assign54380_e69501_d_n4, assign54380_e69501_d_n6, assign54380_e69501_d_n7, assign54380_e69501_d_n8, assign54380_e69501_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 == 0.0)) {
        let assign54380_e69498: f64 = (locals.var_vsginr - locals.var_dvinr);
        let assign54380_e69499: f64 = (locals.var_cinrd_i * assign54380_e69498);
        (assign54380_e69499, (locals.var_cinrd_i * (locals.var_vsginr_dn4 - locals.var_dvinr_dn4)), (locals.var_cinrd_i * (locals.var_vsginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinrd_i * (locals.var_vsginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinrd_i * (locals.var_vsginr_dn8 - locals.var_dvinr_dn8)), (locals.var_cinrd_i * (locals.var_vsginr_dn9 - locals.var_dvinr_dn9)),)
    } else {
        (locals.var_qsinr, locals.var_qsinr_dn4, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8, locals.var_qsinr_dn9,)
    }
};
        locals.var_qsinr = assign54380_e69501;
        locals.var_qsinr_dn4 = assign54380_e69501_d_n4;
        locals.var_qsinr_dn6 = assign54380_e69501_d_n6;
        locals.var_qsinr_dn7 = assign54380_e69501_d_n7;
        locals.var_qsinr_dn8 = assign54380_e69501_d_n8;
        locals.var_qsinr_dn9 = assign54380_e69501_d_n9;
        locals.var_qsinr_rv = 0.0;

        let (assign54390_e69512, assign54390_e69512_d_n4, assign54390_e69512_d_n6, assign54390_e69512_d_n7, assign54390_e69512_d_n8, assign54390_e69512_d_n9,) = {
    if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 == 0.0)) {
        let assign54390_e69509: f64 = (locals.var_vdginr - locals.var_dvinr);
        let assign54390_e69510: f64 = (locals.var_cinr_i * assign54390_e69509);
        (assign54390_e69510, (locals.var_cinr_i * (locals.var_vdginr_dn4 - locals.var_dvinr_dn4)), (locals.var_cinr_i * (locals.var_vdginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinr_i * (locals.var_vdginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinr_i * (locals.var_vdginr_dn8 - locals.var_dvinr_dn8)), (locals.var_cinr_i * (locals.var_vdginr_dn9 - locals.var_dvinr_dn9)),)
    } else {
        (locals.var_qdinr, locals.var_qdinr_dn4, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8, locals.var_qdinr_dn9,)
    }
};
        locals.var_qdinr = assign54390_e69512;
        locals.var_qdinr_dn4 = assign54390_e69512_d_n4;
        locals.var_qdinr_dn6 = assign54390_e69512_d_n6;
        locals.var_qdinr_dn7 = assign54390_e69512_d_n7;
        locals.var_qdinr_dn8 = assign54390_e69512_d_n8;
        locals.var_qdinr_dn9 = assign54390_e69512_d_n9;
        locals.var_qdinr_rv = 0.0;

        let (assign54400_e69518, assign54400_e69518_d_n4, assign54400_e69518_d_n6, assign54400_e69518_d_n7, assign54400_e69518_d_n8, assign54400_e69518_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        let assign54400_e69516: f64 = (locals.var_qg + locals.var_qginr);
        (assign54400_e69516, (locals.var_qg_dn4 + locals.var_qginr_dn4), (locals.var_qg_dn6 + locals.var_qginr_dn6), (locals.var_qg_dn7 + locals.var_qginr_dn7), (locals.var_qg_dn8 + locals.var_qginr_dn8), (locals.var_qg_dn9 + locals.var_qginr_dn9),)
    } else {
        (locals.var_qg, locals.var_qg_dn4, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9,)
    }
};
        locals.var_qg = assign54400_e69518;
        locals.var_qg_dn4 = assign54400_e69518_d_n4;
        locals.var_qg_dn6 = assign54400_e69518_d_n6;
        locals.var_qg_dn7 = assign54400_e69518_d_n7;
        locals.var_qg_dn8 = assign54400_e69518_d_n8;
        locals.var_qg_dn9 = assign54400_e69518_d_n9;
        locals.var_qg_rv = 0.0;

        let (assign54410_e69524, assign54410_e69524_d_n4, assign54410_e69524_d_n6, assign54410_e69524_d_n7, assign54410_e69524_d_n8, assign54410_e69524_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        let assign54410_e69522: f64 = (locals.var_qd + locals.var_qdinr);
        (assign54410_e69522, (locals.var_qd_dn4 + locals.var_qdinr_dn4), (locals.var_qd_dn6 + locals.var_qdinr_dn6), (locals.var_qd_dn7 + locals.var_qdinr_dn7), (locals.var_qd_dn8 + locals.var_qdinr_dn8), (locals.var_qd_dn9 + locals.var_qdinr_dn9),)
    } else {
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9,)
    }
};
        locals.var_qd = assign54410_e69524;
        locals.var_qd_dn4 = assign54410_e69524_d_n4;
        locals.var_qd_dn6 = assign54410_e69524_d_n6;
        locals.var_qd_dn7 = assign54410_e69524_d_n7;
        locals.var_qd_dn8 = assign54410_e69524_d_n8;
        locals.var_qd_dn9 = assign54410_e69524_d_n9;
        locals.var_qd_rv = 0.0;

        let (assign54420_e69534, assign54420_e69534_d_n4, assign54420_e69534_d_n6, assign54420_e69534_d_n7, assign54420_e69534_d_n8, assign54420_e69534_d_n9,) = {
    if (locals.var_guard1527 != 0.0) {
        let assign54420_e69528: f64 = (locals.var_qb - locals.var_qginr);
        let assign54420_e69530: f64 = (assign54420_e69528 - locals.var_qdinr);
        let assign54420_e69532: f64 = (assign54420_e69530 - locals.var_qsinr);
        (assign54420_e69532, (((locals.var_qb_dn4 - locals.var_qginr_dn4) - locals.var_qdinr_dn4) - locals.var_qsinr_dn4), (((locals.var_qb_dn6 - locals.var_qginr_dn6) - locals.var_qdinr_dn6) - locals.var_qsinr_dn6), (((locals.var_qb_dn7 - locals.var_qginr_dn7) - locals.var_qdinr_dn7) - locals.var_qsinr_dn7), (((locals.var_qb_dn8 - locals.var_qginr_dn8) - locals.var_qdinr_dn8) - locals.var_qsinr_dn8), (((locals.var_qb_dn9 - locals.var_qginr_dn9) - locals.var_qdinr_dn9) - locals.var_qsinr_dn9),)
    } else {
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9,)
    }
};
        locals.var_qb = assign54420_e69534;
        locals.var_qb_dn4 = assign54420_e69534_d_n4;
        locals.var_qb_dn6 = assign54420_e69534_d_n6;
        locals.var_qb_dn7 = assign54420_e69534_d_n7;
        locals.var_qb_dn8 = assign54420_e69534_d_n8;
        locals.var_qb_dn9 = assign54420_e69534_d_n9;
        locals.var_qb_rv = 0.0;

        locals.var_qg_ov_s = 0.0;
        locals.var_qg_ov_s_dn4 = 0.0;
        locals.var_qg_ov_s_dn6 = 0.0;
        locals.var_qg_ov_s_dn7 = 0.0;
        locals.var_qg_ov_s_dn8 = 0.0;
        locals.var_qg_ov_s_dn9 = 0.0;
        locals.var_qg_ov_s_rv = 0.0;

        locals.var_yb_ov_s = 0.0;
        locals.var_yb_ov_s_dn4 = 0.0;
        locals.var_yb_ov_s_dn6 = 0.0;
        locals.var_yb_ov_s_dn7 = 0.0;
        locals.var_yb_ov_s_dn8 = 0.0;
        locals.var_yb_ov_s_dn9 = 0.0;
        locals.var_yb_ov_s_rv = 0.0;

        let assign54470_e69549: f64 = if ((locals.var_cgov_i > 0.0) && (locals.var_fcgovacc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1534 = assign54470_e69549;
        locals.var_guard1534_rv = 0.0;

        let (assign54480_e69559, assign54480_e69559_d_n4, assign54480_e69559_d_n6, assign54480_e69559_d_n7, assign54480_e69559_d_n8, assign54480_e69559_d_n9,) = {
    if (locals.var_guard1534 != 0.0) {
        let assign54480_e69554: f64 = (0.5 * locals.var_xgb_ov);
        let assign54480_e69556: f64 = (assign54480_e69554 + locals.var_dxgb_ov_s);
        let assign54480_e69557: f64 = (locals.var_cgovaccg_i * assign54480_e69556);
        (assign54480_e69557, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn4)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign54480_e69559;
        locals.var_temp__blk949_dn4 = assign54480_e69559_d_n4;
        locals.var_temp__blk949_dn6 = assign54480_e69559_d_n6;
        locals.var_temp__blk949_dn7 = assign54480_e69559_d_n7;
        locals.var_temp__blk949_dn8 = assign54480_e69559_d_n8;
        locals.var_temp__blk949_dn9 = assign54480_e69559_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign54490_e69562: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1535 = assign54490_e69562;
        locals.var_guard1535_rv = 0.0;

        let assign54500_e69565: f64 = (-230.25850929940458);
        let assign54500_e69566: f64 = if locals.var_temp__blk949 > assign54500_e69565 { 1.0 } else { 0.0 };
        locals.var_guard1536 = assign54500_e69566;
        locals.var_guard1536_rv = 0.0;

        let (assign54510_e69575, assign54510_e69575_d_n4, assign54510_e69575_d_n6, assign54510_e69575_d_n7, assign54510_e69575_d_n8, assign54510_e69575_d_n9,) = {
    if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1536 != 0.0)) {
        let assign54510_e69573: f64 = (locals.var_temp__blk949).exp();
        (assign54510_e69573, (assign54510_e69573 * locals.var_temp__blk949_dn4), (assign54510_e69573 * locals.var_temp__blk949_dn6), (assign54510_e69573 * locals.var_temp__blk949_dn7), (assign54510_e69573 * locals.var_temp__blk949_dn8), (assign54510_e69573 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn4, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, locals.var_yb_ov_s_dn9,)
    }
};
        locals.var_yb_ov_s = assign54510_e69575;
        locals.var_yb_ov_s_dn4 = assign54510_e69575_d_n4;
        locals.var_yb_ov_s_dn6 = assign54510_e69575_d_n6;
        locals.var_yb_ov_s_dn7 = assign54510_e69575_d_n7;
        locals.var_yb_ov_s_dn8 = assign54510_e69575_d_n8;
        locals.var_yb_ov_s_dn9 = assign54510_e69575_d_n9;
        locals.var_yb_ov_s_rv = 0.0;

        let (assign54520_e69609, assign54520_e69609_d_n4, assign54520_e69609_d_n6, assign54520_e69609_d_n7, assign54520_e69609_d_n8, assign54520_e69609_d_n9,) = {
    if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1536 == 0.0)) {
        let assign54520_e69585: f64 = (-230.25850929940458);
        let assign54520_e69587: f64 = (assign54520_e69585 - locals.var_temp__blk949);
        let assign54520_e69591: f64 = (-230.25850929940458);
        let assign54520_e69593: f64 = (assign54520_e69591 - locals.var_temp__blk949);
        let assign54520_e69596: f64 = (-230.25850929940458);
        let assign54520_e69598: f64 = (assign54520_e69596 - locals.var_temp__blk949);
        let assign54520_e69600: f64 = (assign54520_e69598 * 0.3333333333333333);
        let assign54520_e69601: f64 = (1.0 + assign54520_e69600);
        let assign54520_e69602: f64 = (assign54520_e69593 * assign54520_e69601);
        let assign54520_e69603: f64 = (0.5 * assign54520_e69602);
        let assign54520_e69604: f64 = (1.0 + assign54520_e69603);
        let assign54520_e69605: f64 = (assign54520_e69587 * assign54520_e69604);
        let assign54520_e69606: f64 = (1.0 + assign54520_e69605);
        let assign54520_e69607: f64 = (1e-100 / assign54520_e69606);
        (assign54520_e69607, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign54520_e69604) + (assign54520_e69587 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign54520_e69601) + (assign54520_e69593 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign54520_e69606 * assign54520_e69606))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign54520_e69604) + (assign54520_e69587 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign54520_e69601) + (assign54520_e69593 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign54520_e69606 * assign54520_e69606))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign54520_e69604) + (assign54520_e69587 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign54520_e69601) + (assign54520_e69593 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign54520_e69606 * assign54520_e69606))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign54520_e69604) + (assign54520_e69587 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign54520_e69601) + (assign54520_e69593 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign54520_e69606 * assign54520_e69606))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign54520_e69604) + (assign54520_e69587 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign54520_e69601) + (assign54520_e69593 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign54520_e69606 * assign54520_e69606))),)
    } else {
        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn4, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, locals.var_yb_ov_s_dn9,)
    }
};
        locals.var_yb_ov_s = assign54520_e69609;
        locals.var_yb_ov_s_dn4 = assign54520_e69609_d_n4;
        locals.var_yb_ov_s_dn6 = assign54520_e69609_d_n6;
        locals.var_yb_ov_s_dn7 = assign54520_e69609_d_n7;
        locals.var_yb_ov_s_dn8 = assign54520_e69609_d_n8;
        locals.var_yb_ov_s_dn9 = assign54520_e69609_d_n9;
        locals.var_yb_ov_s_rv = 0.0;

        let assign54530_e69612: f64 = if locals.var_yb_ov_s > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1537 = assign54530_e69612;
        locals.var_guard1537_rv = 0.0;

        let (assign54540_e69623, assign54540_e69623_d_n4, assign54540_e69623_d_n6, assign54540_e69623_d_n7, assign54540_e69623_d_n8, assign54540_e69623_d_n9,) = {
    if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1537 != 0.0)) {
        let assign54540_e69620: f64 = (1.0 + locals.var_yb_ov_s);
        let assign54540_e69621: f64 = (assign54540_e69620).ln();
        (assign54540_e69621, (locals.var_yb_ov_s_dn4 / assign54540_e69620), (locals.var_yb_ov_s_dn6 / assign54540_e69620), (locals.var_yb_ov_s_dn7 / assign54540_e69620), (locals.var_yb_ov_s_dn8 / assign54540_e69620), (locals.var_yb_ov_s_dn9 / assign54540_e69620),)
    } else {
        (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn4, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8, locals.var_xgbeff_ov_s_dn9,)
    }
};
        locals.var_xgbeff_ov_s = assign54540_e69623;
        locals.var_xgbeff_ov_s_dn4 = assign54540_e69623_d_n4;
        locals.var_xgbeff_ov_s_dn6 = assign54540_e69623_d_n6;
        locals.var_xgbeff_ov_s_dn7 = assign54540_e69623_d_n7;
        locals.var_xgbeff_ov_s_dn8 = assign54540_e69623_d_n8;
        locals.var_xgbeff_ov_s_dn9 = assign54540_e69623_d_n9;
        locals.var_xgbeff_ov_s_rv = 0.0;

        let (assign54550_e69642, assign54550_e69642_d_n4, assign54550_e69642_d_n6, assign54550_e69642_d_n7, assign54550_e69642_d_n8, assign54550_e69642_d_n9,) = {
    if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1537 != 0.0)) {
        let assign54550_e69633: f64 = (1.0 + locals.var_xgbeff_ov_s);
        let assign54550_e69634: f64 = (assign54550_e69633).ln();
        let assign54550_e69637: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54550_e69638: f64 = (assign54550_e69634 / assign54550_e69637);
        let assign54550_e69639: f64 = (1.0 - assign54550_e69638);
        let assign54550_e69640: f64 = (locals.var_xgbeff_ov_s * assign54550_e69639);
        (assign54550_e69640, ((locals.var_xgbeff_ov_s_dn4 * assign54550_e69639) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn4 / assign54550_e69633) * assign54550_e69637) - (assign54550_e69634 * locals.var_xgbeff_ov_s_dn4)) / (assign54550_e69637 * assign54550_e69637))))), ((locals.var_xgbeff_ov_s_dn6 * assign54550_e69639) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54550_e69633) * assign54550_e69637) - (assign54550_e69634 * locals.var_xgbeff_ov_s_dn6)) / (assign54550_e69637 * assign54550_e69637))))), ((locals.var_xgbeff_ov_s_dn7 * assign54550_e69639) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54550_e69633) * assign54550_e69637) - (assign54550_e69634 * locals.var_xgbeff_ov_s_dn7)) / (assign54550_e69637 * assign54550_e69637))))), ((locals.var_xgbeff_ov_s_dn8 * assign54550_e69639) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54550_e69633) * assign54550_e69637) - (assign54550_e69634 * locals.var_xgbeff_ov_s_dn8)) / (assign54550_e69637 * assign54550_e69637))))), ((locals.var_xgbeff_ov_s_dn9 * assign54550_e69639) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn9 / assign54550_e69633) * assign54550_e69637) - (assign54550_e69634 * locals.var_xgbeff_ov_s_dn9)) / (assign54550_e69637 * assign54550_e69637))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54550_e69642;
        locals.var_temp1_dn4 = assign54550_e69642_d_n4;
        locals.var_temp1_dn6 = assign54550_e69642_d_n6;
        locals.var_temp1_dn7 = assign54550_e69642_d_n7;
        locals.var_temp1_dn8 = assign54550_e69642_d_n8;
        locals.var_temp1_dn9 = assign54550_e69642_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign54560_e69651, assign54560_e69651_d_n4, assign54560_e69651_d_n6, assign54560_e69651_d_n7, assign54560_e69651_d_n8, assign54560_e69651_d_n9,) = {
    if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1537 == 0.0)) {
        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn4, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, locals.var_yb_ov_s_dn9,)
    } else {
        (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn4, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8, locals.var_xgbeff_ov_s_dn9,)
    }
};
        locals.var_xgbeff_ov_s = assign54560_e69651;
        locals.var_xgbeff_ov_s_dn4 = assign54560_e69651_d_n4;
        locals.var_xgbeff_ov_s_dn6 = assign54560_e69651_d_n6;
        locals.var_xgbeff_ov_s_dn7 = assign54560_e69651_d_n7;
        locals.var_xgbeff_ov_s_dn8 = assign54560_e69651_d_n8;
        locals.var_xgbeff_ov_s_dn9 = assign54560_e69651_d_n9;
        locals.var_xgbeff_ov_s_rv = 0.0;

    }
}
