#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_102(
        locals: &mut StampLocals,
    ) {
        let (assign35720_e40101, assign35720_e40101_d_n4, assign35720_e40101_d_n6, assign35720_e40101_d_n7, assign35720_e40101_d_n8, assign35720_e40101_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35720_e40090: f64 = (-0.0625);
        let assign35720_e40094: f64 = (locals.var_temp4 * locals.var_temp4);
        let assign35720_e40095: f64 = (locals.var_temp + assign35720_e40094);
        let assign35720_e40096: f64 = (assign35720_e40095).ln();
        let assign35720_e40097: f64 = (assign35720_e40090 * assign35720_e40096);
        let assign35720_e40098: f64 = (assign35720_e40097).exp();
        let assign35720_e40099: f64 = (locals.var_xd * assign35720_e40098);
        (assign35720_e40099, ((locals.var_xd_dn4 * assign35720_e40098) + (locals.var_xd * (assign35720_e40098 * (assign35720_e40090 * ((locals.var_temp_dn4 + ((locals.var_temp4_dn4 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn4))) / assign35720_e40095))))), ((locals.var_xd_dn6 * assign35720_e40098) + (locals.var_xd * (assign35720_e40098 * (assign35720_e40090 * ((locals.var_temp_dn6 + ((locals.var_temp4_dn6 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn6))) / assign35720_e40095))))), ((locals.var_xd_dn7 * assign35720_e40098) + (locals.var_xd * (assign35720_e40098 * (assign35720_e40090 * ((locals.var_temp_dn7 + ((locals.var_temp4_dn7 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn7))) / assign35720_e40095))))), ((locals.var_xd_dn8 * assign35720_e40098) + (locals.var_xd * (assign35720_e40098 * (assign35720_e40090 * ((locals.var_temp_dn8 + ((locals.var_temp4_dn8 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn8))) / assign35720_e40095))))), ((locals.var_xd_dn9 * assign35720_e40098) + (locals.var_xd * (assign35720_e40098 * (assign35720_e40090 * ((locals.var_temp_dn9 + ((locals.var_temp4_dn9 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn9))) / assign35720_e40095))))),)
    } else {
        (locals.var_xdeff__blk1000, locals.var_xdeff__blk1000_dn4, locals.var_xdeff__blk1000_dn6, locals.var_xdeff__blk1000_dn7, locals.var_xdeff__blk1000_dn8, locals.var_xdeff__blk1000_dn9,)
    }
};
        locals.var_xdeff__blk1000 = assign35720_e40101;
        locals.var_xdeff__blk1000_dn4 = assign35720_e40101_d_n4;
        locals.var_xdeff__blk1000_dn6 = assign35720_e40101_d_n6;
        locals.var_xdeff__blk1000_dn7 = assign35720_e40101_d_n7;
        locals.var_xdeff__blk1000_dn8 = assign35720_e40101_d_n8;
        locals.var_xdeff__blk1000_dn9 = assign35720_e40101_d_n9;
        locals.var_xdeff__blk1000_rv = 0.0;

        let (assign35730_e40109, assign35730_e40109_d_n4, assign35730_e40109_d_n6, assign35730_e40109_d_n7, assign35730_e40109_d_n8, assign35730_e40109_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35730_e40106: f64 = (locals.var_k1__blk932 + 1.0);
        let assign35730_e40107: f64 = (1.0 / assign35730_e40106);
        (assign35730_e40107, (-(locals.var_k1__blk932_dn4 / (assign35730_e40106 * assign35730_e40106))), (-(locals.var_k1__blk932_dn6 / (assign35730_e40106 * assign35730_e40106))), (-(locals.var_k1__blk932_dn7 / (assign35730_e40106 * assign35730_e40106))), (-(locals.var_k1__blk932_dn8 / (assign35730_e40106 * assign35730_e40106))), (-(locals.var_k1__blk932_dn9 / (assign35730_e40106 * assign35730_e40106))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign35730_e40109;
        locals.var_q_temp1__blk814_dn4 = assign35730_e40109_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign35730_e40109_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign35730_e40109_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign35730_e40109_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign35730_e40109_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign35740_e40117, assign35740_e40117_d_n4, assign35740_e40117_d_n6, assign35740_e40117_d_n7, assign35740_e40117_d_n8, assign35740_e40117_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35740_e40114: f64 = (locals.var_k2__blk933 + 1.0);
        let assign35740_e40115: f64 = (1.0 / assign35740_e40114);
        (assign35740_e40115, (-(locals.var_k2__blk933_dn4 / (assign35740_e40114 * assign35740_e40114))), (-(locals.var_k2__blk933_dn6 / (assign35740_e40114 * assign35740_e40114))), (-(locals.var_k2__blk933_dn7 / (assign35740_e40114 * assign35740_e40114))), (-(locals.var_k2__blk933_dn8 / (assign35740_e40114 * assign35740_e40114))), (-(locals.var_k2__blk933_dn9 / (assign35740_e40114 * assign35740_e40114))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign35740_e40117;
        locals.var_q_temp2__blk815_dn4 = assign35740_e40117_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign35740_e40117_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign35740_e40117_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign35740_e40117_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign35740_e40117_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign35750_e40134, assign35750_e40134_d_n4, assign35750_e40134_d_n6, assign35750_e40134_d_n7, assign35750_e40134_d_n8, assign35750_e40134_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35750_e40122: f64 = (locals.var_k2__blk933 * locals.var_q_temp2__blk815);
        let assign35750_e40123: f64 = (locals.var_k1__blk932 + assign35750_e40122);
        let assign35750_e40125: f64 = (assign35750_e40123 * locals.var_diff_min__blk904);
        let assign35750_e40127: f64 = (assign35750_e40125 / locals.var_a0__blk905);
        let assign35750_e40128: f64 = (assign35750_e40127).ln();
        let assign35750_e40130: f64 = (assign35750_e40128 + locals.var_xdeff__blk1000);
        let assign35750_e40132: f64 = (assign35750_e40130 + 3.0);
        (assign35750_e40132, ((((((((locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn4))) * locals.var_diff_min__blk904) + (assign35750_e40123 * locals.var_diff_min__blk904_dn4)) * locals.var_a0__blk905) - (assign35750_e40125 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign35750_e40127) + locals.var_xdeff__blk1000_dn4), ((((((((locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn6))) * locals.var_diff_min__blk904) + (assign35750_e40123 * locals.var_diff_min__blk904_dn6)) * locals.var_a0__blk905) - (assign35750_e40125 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign35750_e40127) + locals.var_xdeff__blk1000_dn6), ((((((((locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn7))) * locals.var_diff_min__blk904) + (assign35750_e40123 * locals.var_diff_min__blk904_dn7)) * locals.var_a0__blk905) - (assign35750_e40125 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign35750_e40127) + locals.var_xdeff__blk1000_dn7), ((((((((locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn8))) * locals.var_diff_min__blk904) + (assign35750_e40123 * locals.var_diff_min__blk904_dn8)) * locals.var_a0__blk905) - (assign35750_e40125 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign35750_e40127) + locals.var_xdeff__blk1000_dn8), ((((((((locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn9))) * locals.var_diff_min__blk904) + (assign35750_e40123 * locals.var_diff_min__blk904_dn9)) * locals.var_a0__blk905) - (assign35750_e40125 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign35750_e40127) + locals.var_xdeff__blk1000_dn9),)
    } else {
        (locals.var_q_x1sat__blk817, locals.var_q_x1sat__blk817_dn4, locals.var_q_x1sat__blk817_dn6, locals.var_q_x1sat__blk817_dn7, locals.var_q_x1sat__blk817_dn8, locals.var_q_x1sat__blk817_dn9,)
    }
};
        locals.var_q_x1sat__blk817 = assign35750_e40134;
        locals.var_q_x1sat__blk817_dn4 = assign35750_e40134_d_n4;
        locals.var_q_x1sat__blk817_dn6 = assign35750_e40134_d_n6;
        locals.var_q_x1sat__blk817_dn7 = assign35750_e40134_d_n7;
        locals.var_q_x1sat__blk817_dn8 = assign35750_e40134_d_n8;
        locals.var_q_x1sat__blk817_dn9 = assign35750_e40134_d_n9;
        locals.var_q_x1sat__blk817_rv = 0.0;

        let (assign35760_e40151, assign35760_e40151_d_n4, assign35760_e40151_d_n6, assign35760_e40151_d_n7, assign35760_e40151_d_n8, assign35760_e40151_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35760_e40139: f64 = (locals.var_k1__blk932 * locals.var_q_temp1__blk814);
        let assign35760_e40140: f64 = (locals.var_k2__blk933 + assign35760_e40139);
        let assign35760_e40142: f64 = (assign35760_e40140 * locals.var_diff_min__blk904);
        let assign35760_e40144: f64 = (assign35760_e40142 / locals.var_a0__blk905);
        let assign35760_e40145: f64 = (assign35760_e40144).ln();
        let assign35760_e40147: f64 = (assign35760_e40145 + locals.var_xdeff__blk1000);
        let assign35760_e40149: f64 = (assign35760_e40147 + 3.0);
        (assign35760_e40149, ((((((((locals.var_k2__blk933_dn4 + ((locals.var_k1__blk932_dn4 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn4))) * locals.var_diff_min__blk904) + (assign35760_e40140 * locals.var_diff_min__blk904_dn4)) * locals.var_a0__blk905) - (assign35760_e40142 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign35760_e40144) + locals.var_xdeff__blk1000_dn4), ((((((((locals.var_k2__blk933_dn6 + ((locals.var_k1__blk932_dn6 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn6))) * locals.var_diff_min__blk904) + (assign35760_e40140 * locals.var_diff_min__blk904_dn6)) * locals.var_a0__blk905) - (assign35760_e40142 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign35760_e40144) + locals.var_xdeff__blk1000_dn6), ((((((((locals.var_k2__blk933_dn7 + ((locals.var_k1__blk932_dn7 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn7))) * locals.var_diff_min__blk904) + (assign35760_e40140 * locals.var_diff_min__blk904_dn7)) * locals.var_a0__blk905) - (assign35760_e40142 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign35760_e40144) + locals.var_xdeff__blk1000_dn7), ((((((((locals.var_k2__blk933_dn8 + ((locals.var_k1__blk932_dn8 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn8))) * locals.var_diff_min__blk904) + (assign35760_e40140 * locals.var_diff_min__blk904_dn8)) * locals.var_a0__blk905) - (assign35760_e40142 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign35760_e40144) + locals.var_xdeff__blk1000_dn8), ((((((((locals.var_k2__blk933_dn9 + ((locals.var_k1__blk932_dn9 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn9))) * locals.var_diff_min__blk904) + (assign35760_e40140 * locals.var_diff_min__blk904_dn9)) * locals.var_a0__blk905) - (assign35760_e40142 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign35760_e40144) + locals.var_xdeff__blk1000_dn9),)
    } else {
        (locals.var_q_x2sat__blk818, locals.var_q_x2sat__blk818_dn4, locals.var_q_x2sat__blk818_dn6, locals.var_q_x2sat__blk818_dn7, locals.var_q_x2sat__blk818_dn8, locals.var_q_x2sat__blk818_dn9,)
    }
};
        locals.var_q_x2sat__blk818 = assign35760_e40151;
        locals.var_q_x2sat__blk818_dn4 = assign35760_e40151_d_n4;
        locals.var_q_x2sat__blk818_dn6 = assign35760_e40151_d_n6;
        locals.var_q_x2sat__blk818_dn7 = assign35760_e40151_d_n7;
        locals.var_q_x2sat__blk818_dn8 = assign35760_e40151_d_n8;
        locals.var_q_x2sat__blk818_dn9 = assign35760_e40151_d_n9;
        locals.var_q_x2sat__blk818_rv = 0.0;

        let assign35770_e40154: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign35770_e40156: f64 = (assign35770_e40154 * 0.3333333333333);
        let assign35770_e40158: f64 = if assign35770_e40156 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1165 = assign35770_e40158;
        locals.var_guard1165_rv = 0.0;

        let (assign35780_e40172, assign35780_e40172_d_n4, assign35780_e40172_d_n6, assign35780_e40172_d_n7, assign35780_e40172_d_n8, assign35780_e40172_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1165 != 0.0)) {
        let assign35780_e40165: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign35780_e40167: f64 = (assign35780_e40165 * 0.3333333333333);
        let assign35780_e40168: f64 = (assign35780_e40167).exp();
        let assign35780_e40169: f64 = (1.0 + assign35780_e40168);
        let assign35780_e40170: f64 = (assign35780_e40169).ln();
        (assign35780_e40170, ((assign35780_e40168 * ((locals.var_q_x1sat__blk817_dn4 - locals.var_x1_wi0__blk908_dn4) * 0.3333333333333)) / assign35780_e40169), ((assign35780_e40168 * ((locals.var_q_x1sat__blk817_dn6 - locals.var_x1_wi0__blk908_dn6) * 0.3333333333333)) / assign35780_e40169), ((assign35780_e40168 * ((locals.var_q_x1sat__blk817_dn7 - locals.var_x1_wi0__blk908_dn7) * 0.3333333333333)) / assign35780_e40169), ((assign35780_e40168 * ((locals.var_q_x1sat__blk817_dn8 - locals.var_x1_wi0__blk908_dn8) * 0.3333333333333)) / assign35780_e40169), ((assign35780_e40168 * ((locals.var_q_x1sat__blk817_dn9 - locals.var_x1_wi0__blk908_dn9) * 0.3333333333333)) / assign35780_e40169),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign35780_e40172;
        locals.var_q_temp3__blk816_dn4 = assign35780_e40172_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign35780_e40172_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign35780_e40172_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign35780_e40172_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign35780_e40172_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign35790_e40183, assign35790_e40183_d_n4, assign35790_e40183_d_n6, assign35790_e40183_d_n7, assign35790_e40183_d_n8, assign35790_e40183_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1165 == 0.0)) {
        let assign35790_e40179: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign35790_e40181: f64 = (assign35790_e40179 * 0.3333333333333);
        (assign35790_e40181, ((locals.var_q_x1sat__blk817_dn4 - locals.var_x1_wi0__blk908_dn4) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn6 - locals.var_x1_wi0__blk908_dn6) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn7 - locals.var_x1_wi0__blk908_dn7) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn8 - locals.var_x1_wi0__blk908_dn8) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn9 - locals.var_x1_wi0__blk908_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign35790_e40183;
        locals.var_q_temp3__blk816_dn4 = assign35790_e40183_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign35790_e40183_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign35790_e40183_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign35790_e40183_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign35790_e40183_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign35800_e40191, assign35800_e40191_d_n4, assign35800_e40191_d_n6, assign35800_e40191_d_n7, assign35800_e40191_d_n8, assign35800_e40191_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35800_e40188: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign35800_e40189: f64 = (locals.var_q_x1sat__blk817 - assign35800_e40188);
        (assign35800_e40189, (locals.var_q_x1sat__blk817_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x1sat__blk817_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x1sat__blk817_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x1sat__blk817_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x1sat__blk817_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x1__blk821, locals.var_q_x1__blk821_dn4, locals.var_q_x1__blk821_dn6, locals.var_q_x1__blk821_dn7, locals.var_q_x1__blk821_dn8, locals.var_q_x1__blk821_dn9,)
    }
};
        locals.var_q_x1__blk821 = assign35800_e40191;
        locals.var_q_x1__blk821_dn4 = assign35800_e40191_d_n4;
        locals.var_q_x1__blk821_dn6 = assign35800_e40191_d_n6;
        locals.var_q_x1__blk821_dn7 = assign35800_e40191_d_n7;
        locals.var_q_x1__blk821_dn8 = assign35800_e40191_d_n8;
        locals.var_q_x1__blk821_dn9 = assign35800_e40191_d_n9;
        locals.var_q_x1__blk821_rv = 0.0;

        let assign35810_e40194: f64 = (locals.var_q_x2sat__blk818 - locals.var_x2_wi0__blk909);
        let assign35810_e40196: f64 = (assign35810_e40194 * 0.3333333333333);
        let assign35810_e40198: f64 = if assign35810_e40196 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1166 = assign35810_e40198;
        locals.var_guard1166_rv = 0.0;

        let (assign35820_e40212, assign35820_e40212_d_n4, assign35820_e40212_d_n6, assign35820_e40212_d_n7, assign35820_e40212_d_n8, assign35820_e40212_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1166 != 0.0)) {
        let assign35820_e40205: f64 = (locals.var_q_x2sat__blk818 - locals.var_x2_wi0__blk909);
        let assign35820_e40207: f64 = (assign35820_e40205 * 0.3333333333333);
        let assign35820_e40208: f64 = (assign35820_e40207).exp();
        let assign35820_e40209: f64 = (1.0 + assign35820_e40208);
        let assign35820_e40210: f64 = (assign35820_e40209).ln();
        (assign35820_e40210, ((assign35820_e40208 * ((locals.var_q_x2sat__blk818_dn4 - locals.var_x2_wi0__blk909_dn4) * 0.3333333333333)) / assign35820_e40209), ((assign35820_e40208 * ((locals.var_q_x2sat__blk818_dn6 - locals.var_x2_wi0__blk909_dn6) * 0.3333333333333)) / assign35820_e40209), ((assign35820_e40208 * ((locals.var_q_x2sat__blk818_dn7 - locals.var_x2_wi0__blk909_dn7) * 0.3333333333333)) / assign35820_e40209), ((assign35820_e40208 * ((locals.var_q_x2sat__blk818_dn8 - locals.var_x2_wi0__blk909_dn8) * 0.3333333333333)) / assign35820_e40209), ((assign35820_e40208 * ((locals.var_q_x2sat__blk818_dn9 - locals.var_x2_wi0__blk909_dn9) * 0.3333333333333)) / assign35820_e40209),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign35820_e40212;
        locals.var_q_temp3__blk816_dn4 = assign35820_e40212_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign35820_e40212_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign35820_e40212_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign35820_e40212_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign35820_e40212_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign35830_e40223, assign35830_e40223_d_n4, assign35830_e40223_d_n6, assign35830_e40223_d_n7, assign35830_e40223_d_n8, assign35830_e40223_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1166 == 0.0)) {
        let assign35830_e40219: f64 = (locals.var_q_x2sat__blk818 - locals.var_x2_wi0__blk909);
        let assign35830_e40221: f64 = (assign35830_e40219 * 0.3333333333333);
        (assign35830_e40221, ((locals.var_q_x2sat__blk818_dn4 - locals.var_x2_wi0__blk909_dn4) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn6 - locals.var_x2_wi0__blk909_dn6) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn7 - locals.var_x2_wi0__blk909_dn7) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn8 - locals.var_x2_wi0__blk909_dn8) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn9 - locals.var_x2_wi0__blk909_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign35830_e40223;
        locals.var_q_temp3__blk816_dn4 = assign35830_e40223_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign35830_e40223_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign35830_e40223_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign35830_e40223_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign35830_e40223_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign35840_e40231, assign35840_e40231_d_n4, assign35840_e40231_d_n6, assign35840_e40231_d_n7, assign35840_e40231_d_n8, assign35840_e40231_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35840_e40228: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign35840_e40229: f64 = (locals.var_q_x2sat__blk818 - assign35840_e40228);
        (assign35840_e40229, (locals.var_q_x2sat__blk818_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x2sat__blk818_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x2sat__blk818_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x2sat__blk818_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x2sat__blk818_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x2__blk822, locals.var_q_x2__blk822_dn4, locals.var_q_x2__blk822_dn6, locals.var_q_x2__blk822_dn7, locals.var_q_x2__blk822_dn8, locals.var_q_x2__blk822_dn9,)
    }
};
        locals.var_q_x2__blk822 = assign35840_e40231;
        locals.var_q_x2__blk822_dn4 = assign35840_e40231_d_n4;
        locals.var_q_x2__blk822_dn6 = assign35840_e40231_d_n6;
        locals.var_q_x2__blk822_dn7 = assign35840_e40231_d_n7;
        locals.var_q_x2__blk822_dn8 = assign35840_e40231_d_n8;
        locals.var_q_x2__blk822_dn9 = assign35840_e40231_d_n9;
        locals.var_q_x2__blk822_rv = 0.0;

        let (assign35850_e40241, assign35850_e40241_d_n4, assign35850_e40241_d_n6, assign35850_e40241_d_n7, assign35850_e40241_d_n8, assign35850_e40241_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35850_e40235: f64 = (locals.var_k1__blk932 * locals.var_xg1x__blk930);
        let assign35850_e40237: f64 = (assign35850_e40235 + locals.var_q_x2__blk822);
        let assign35850_e40239: f64 = (assign35850_e40237 * locals.var_q_temp1__blk814);
        (assign35850_e40239, (((((locals.var_k1__blk932_dn4 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn4)) + locals.var_q_x2__blk822_dn4) * locals.var_q_temp1__blk814) + (assign35850_e40237 * locals.var_q_temp1__blk814_dn4)), (((((locals.var_k1__blk932_dn6 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn6)) + locals.var_q_x2__blk822_dn6) * locals.var_q_temp1__blk814) + (assign35850_e40237 * locals.var_q_temp1__blk814_dn6)), (((((locals.var_k1__blk932_dn7 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn7)) + locals.var_q_x2__blk822_dn7) * locals.var_q_temp1__blk814) + (assign35850_e40237 * locals.var_q_temp1__blk814_dn7)), (((((locals.var_k1__blk932_dn8 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn8)) + locals.var_q_x2__blk822_dn8) * locals.var_q_temp1__blk814) + (assign35850_e40237 * locals.var_q_temp1__blk814_dn8)), (((((locals.var_k1__blk932_dn9 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn9)) + locals.var_q_x2__blk822_dn9) * locals.var_q_temp1__blk814) + (assign35850_e40237 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_x1_wi__blk819, locals.var_q_x1_wi__blk819_dn4, locals.var_q_x1_wi__blk819_dn6, locals.var_q_x1_wi__blk819_dn7, locals.var_q_x1_wi__blk819_dn8, locals.var_q_x1_wi__blk819_dn9,)
    }
};
        locals.var_q_x1_wi__blk819 = assign35850_e40241;
        locals.var_q_x1_wi__blk819_dn4 = assign35850_e40241_d_n4;
        locals.var_q_x1_wi__blk819_dn6 = assign35850_e40241_d_n6;
        locals.var_q_x1_wi__blk819_dn7 = assign35850_e40241_d_n7;
        locals.var_q_x1_wi__blk819_dn8 = assign35850_e40241_d_n8;
        locals.var_q_x1_wi__blk819_dn9 = assign35850_e40241_d_n9;
        locals.var_q_x1_wi__blk819_rv = 0.0;

        let (assign35860_e40251, assign35860_e40251_d_n4, assign35860_e40251_d_n6, assign35860_e40251_d_n7, assign35860_e40251_d_n8, assign35860_e40251_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35860_e40245: f64 = (locals.var_k2__blk933 * locals.var_xg2x__blk931);
        let assign35860_e40247: f64 = (assign35860_e40245 + locals.var_q_x1__blk821);
        let assign35860_e40249: f64 = (assign35860_e40247 * locals.var_q_temp2__blk815);
        (assign35860_e40249, (((((locals.var_k2__blk933_dn4 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn4)) + locals.var_q_x1__blk821_dn4) * locals.var_q_temp2__blk815) + (assign35860_e40247 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_k2__blk933_dn6 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn6)) + locals.var_q_x1__blk821_dn6) * locals.var_q_temp2__blk815) + (assign35860_e40247 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_k2__blk933_dn7 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn7)) + locals.var_q_x1__blk821_dn7) * locals.var_q_temp2__blk815) + (assign35860_e40247 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_k2__blk933_dn8 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn8)) + locals.var_q_x1__blk821_dn8) * locals.var_q_temp2__blk815) + (assign35860_e40247 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_k2__blk933_dn9 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn9)) + locals.var_q_x1__blk821_dn9) * locals.var_q_temp2__blk815) + (assign35860_e40247 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_x2_wi__blk820, locals.var_q_x2_wi__blk820_dn4, locals.var_q_x2_wi__blk820_dn6, locals.var_q_x2_wi__blk820_dn7, locals.var_q_x2_wi__blk820_dn8, locals.var_q_x2_wi__blk820_dn9,)
    }
};
        locals.var_q_x2_wi__blk820 = assign35860_e40251;
        locals.var_q_x2_wi__blk820_dn4 = assign35860_e40251_d_n4;
        locals.var_q_x2_wi__blk820_dn6 = assign35860_e40251_d_n6;
        locals.var_q_x2_wi__blk820_dn7 = assign35860_e40251_d_n7;
        locals.var_q_x2_wi__blk820_dn8 = assign35860_e40251_d_n8;
        locals.var_q_x2_wi__blk820_dn9 = assign35860_e40251_d_n9;
        locals.var_q_x2_wi__blk820_rv = 0.0;

        let assign35870_e40254: f64 = (locals.var_q_x1sat__blk817 - locals.var_q_x1_wi__blk819);
        let assign35870_e40256: f64 = (assign35870_e40254 * 0.3333333333333);
        let assign35870_e40258: f64 = if assign35870_e40256 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1167 = assign35870_e40258;
        locals.var_guard1167_rv = 0.0;

        let (assign35880_e40272, assign35880_e40272_d_n4, assign35880_e40272_d_n6, assign35880_e40272_d_n7, assign35880_e40272_d_n8, assign35880_e40272_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1167 != 0.0)) {
        let assign35880_e40265: f64 = (locals.var_q_x1sat__blk817 - locals.var_q_x1_wi__blk819);
        let assign35880_e40267: f64 = (assign35880_e40265 * 0.3333333333333);
        let assign35880_e40268: f64 = (assign35880_e40267).exp();
        let assign35880_e40269: f64 = (1.0 + assign35880_e40268);
        let assign35880_e40270: f64 = (assign35880_e40269).ln();
        (assign35880_e40270, ((assign35880_e40268 * ((locals.var_q_x1sat__blk817_dn4 - locals.var_q_x1_wi__blk819_dn4) * 0.3333333333333)) / assign35880_e40269), ((assign35880_e40268 * ((locals.var_q_x1sat__blk817_dn6 - locals.var_q_x1_wi__blk819_dn6) * 0.3333333333333)) / assign35880_e40269), ((assign35880_e40268 * ((locals.var_q_x1sat__blk817_dn7 - locals.var_q_x1_wi__blk819_dn7) * 0.3333333333333)) / assign35880_e40269), ((assign35880_e40268 * ((locals.var_q_x1sat__blk817_dn8 - locals.var_q_x1_wi__blk819_dn8) * 0.3333333333333)) / assign35880_e40269), ((assign35880_e40268 * ((locals.var_q_x1sat__blk817_dn9 - locals.var_q_x1_wi__blk819_dn9) * 0.3333333333333)) / assign35880_e40269),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign35880_e40272;
        locals.var_q_temp3__blk816_dn4 = assign35880_e40272_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign35880_e40272_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign35880_e40272_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign35880_e40272_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign35880_e40272_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign35890_e40283, assign35890_e40283_d_n4, assign35890_e40283_d_n6, assign35890_e40283_d_n7, assign35890_e40283_d_n8, assign35890_e40283_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1167 == 0.0)) {
        let assign35890_e40279: f64 = (locals.var_q_x1sat__blk817 - locals.var_q_x1_wi__blk819);
        let assign35890_e40281: f64 = (assign35890_e40279 * 0.3333333333333);
        (assign35890_e40281, ((locals.var_q_x1sat__blk817_dn4 - locals.var_q_x1_wi__blk819_dn4) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn6 - locals.var_q_x1_wi__blk819_dn6) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn7 - locals.var_q_x1_wi__blk819_dn7) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn8 - locals.var_q_x1_wi__blk819_dn8) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn9 - locals.var_q_x1_wi__blk819_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign35890_e40283;
        locals.var_q_temp3__blk816_dn4 = assign35890_e40283_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign35890_e40283_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign35890_e40283_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign35890_e40283_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign35890_e40283_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign35900_e40291, assign35900_e40291_d_n4, assign35900_e40291_d_n6, assign35900_e40291_d_n7, assign35900_e40291_d_n8, assign35900_e40291_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35900_e40288: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign35900_e40289: f64 = (locals.var_q_x1sat__blk817 - assign35900_e40288);
        (assign35900_e40289, (locals.var_q_x1sat__blk817_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x1sat__blk817_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x1sat__blk817_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x1sat__blk817_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x1sat__blk817_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x1__blk821, locals.var_q_x1__blk821_dn4, locals.var_q_x1__blk821_dn6, locals.var_q_x1__blk821_dn7, locals.var_q_x1__blk821_dn8, locals.var_q_x1__blk821_dn9,)
    }
};
        locals.var_q_x1__blk821 = assign35900_e40291;
        locals.var_q_x1__blk821_dn4 = assign35900_e40291_d_n4;
        locals.var_q_x1__blk821_dn6 = assign35900_e40291_d_n6;
        locals.var_q_x1__blk821_dn7 = assign35900_e40291_d_n7;
        locals.var_q_x1__blk821_dn8 = assign35900_e40291_d_n8;
        locals.var_q_x1__blk821_dn9 = assign35900_e40291_d_n9;
        locals.var_q_x1__blk821_rv = 0.0;

        let assign35910_e40294: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign35910_e40296: f64 = (assign35910_e40294 * 0.3333333333333);
        let assign35910_e40298: f64 = if assign35910_e40296 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1168 = assign35910_e40298;
        locals.var_guard1168_rv = 0.0;

        let (assign35920_e40312, assign35920_e40312_d_n4, assign35920_e40312_d_n6, assign35920_e40312_d_n7, assign35920_e40312_d_n8, assign35920_e40312_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1168 != 0.0)) {
        let assign35920_e40305: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign35920_e40307: f64 = (assign35920_e40305 * 0.3333333333333);
        let assign35920_e40308: f64 = (assign35920_e40307).exp();
        let assign35920_e40309: f64 = (1.0 + assign35920_e40308);
        let assign35920_e40310: f64 = (assign35920_e40309).ln();
        (assign35920_e40310, ((assign35920_e40308 * ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) * 0.3333333333333)) / assign35920_e40309), ((assign35920_e40308 * ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) * 0.3333333333333)) / assign35920_e40309), ((assign35920_e40308 * ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) * 0.3333333333333)) / assign35920_e40309), ((assign35920_e40308 * ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) * 0.3333333333333)) / assign35920_e40309), ((assign35920_e40308 * ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) * 0.3333333333333)) / assign35920_e40309),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign35920_e40312;
        locals.var_q_temp3__blk816_dn4 = assign35920_e40312_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign35920_e40312_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign35920_e40312_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign35920_e40312_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign35920_e40312_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign35930_e40323, assign35930_e40323_d_n4, assign35930_e40323_d_n6, assign35930_e40323_d_n7, assign35930_e40323_d_n8, assign35930_e40323_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1168 == 0.0)) {
        let assign35930_e40319: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign35930_e40321: f64 = (assign35930_e40319 * 0.3333333333333);
        (assign35930_e40321, ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign35930_e40323;
        locals.var_q_temp3__blk816_dn4 = assign35930_e40323_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign35930_e40323_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign35930_e40323_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign35930_e40323_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign35930_e40323_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign35940_e40331, assign35940_e40331_d_n4, assign35940_e40331_d_n6, assign35940_e40331_d_n7, assign35940_e40331_d_n8, assign35940_e40331_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35940_e40328: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign35940_e40329: f64 = (locals.var_q_x2sat__blk818 - assign35940_e40328);
        (assign35940_e40329, (locals.var_q_x2sat__blk818_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x2sat__blk818_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x2sat__blk818_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x2sat__blk818_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x2sat__blk818_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x2__blk822, locals.var_q_x2__blk822_dn4, locals.var_q_x2__blk822_dn6, locals.var_q_x2__blk822_dn7, locals.var_q_x2__blk822_dn8, locals.var_q_x2__blk822_dn9,)
    }
};
        locals.var_q_x2__blk822 = assign35940_e40331;
        locals.var_q_x2__blk822_dn4 = assign35940_e40331_d_n4;
        locals.var_q_x2__blk822_dn6 = assign35940_e40331_d_n6;
        locals.var_q_x2__blk822_dn7 = assign35940_e40331_d_n7;
        locals.var_q_x2__blk822_dn8 = assign35940_e40331_d_n8;
        locals.var_q_x2__blk822_dn9 = assign35940_e40331_d_n9;
        locals.var_q_x2__blk822_rv = 0.0;

        let (assign35950_e40337, assign35950_e40337_d_n4, assign35950_e40337_d_n6, assign35950_e40337_d_n7, assign35950_e40337_d_n8, assign35950_e40337_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35950_e40335: f64 = (locals.var_xg1x__blk930 - locals.var_q_x1__blk821);
        (assign35950_e40335, (locals.var_xg1x__blk930_dn4 - locals.var_q_x1__blk821_dn4), (locals.var_xg1x__blk930_dn6 - locals.var_q_x1__blk821_dn6), (locals.var_xg1x__blk930_dn7 - locals.var_q_x1__blk821_dn7), (locals.var_xg1x__blk930_dn8 - locals.var_q_x1__blk821_dn8), (locals.var_xg1x__blk930_dn9 - locals.var_q_x1__blk821_dn9),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign35950_e40337;
        locals.var_q1d__blk1001_dn4 = assign35950_e40337_d_n4;
        locals.var_q1d__blk1001_dn6 = assign35950_e40337_d_n6;
        locals.var_q1d__blk1001_dn7 = assign35950_e40337_d_n7;
        locals.var_q1d__blk1001_dn8 = assign35950_e40337_d_n8;
        locals.var_q1d__blk1001_dn9 = assign35950_e40337_d_n9;
        locals.var_q1d__blk1001_rv = 0.0;

        let (assign35960_e40343, assign35960_e40343_d_n4, assign35960_e40343_d_n6, assign35960_e40343_d_n7, assign35960_e40343_d_n8, assign35960_e40343_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35960_e40341: f64 = (locals.var_xg2x__blk931 - locals.var_q_x2__blk822);
        (assign35960_e40341, (locals.var_xg2x__blk931_dn4 - locals.var_q_x2__blk822_dn4), (locals.var_xg2x__blk931_dn6 - locals.var_q_x2__blk822_dn6), (locals.var_xg2x__blk931_dn7 - locals.var_q_x2__blk822_dn7), (locals.var_xg2x__blk931_dn8 - locals.var_q_x2__blk822_dn8), (locals.var_xg2x__blk931_dn9 - locals.var_q_x2__blk822_dn9),)
    } else {
        (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9,)
    }
};
        locals.var_q2d__blk1002 = assign35960_e40343;
        locals.var_q2d__blk1002_dn4 = assign35960_e40343_d_n4;
        locals.var_q2d__blk1002_dn6 = assign35960_e40343_d_n6;
        locals.var_q2d__blk1002_dn7 = assign35960_e40343_d_n7;
        locals.var_q2d__blk1002_dn8 = assign35960_e40343_d_n8;
        locals.var_q2d__blk1002_dn9 = assign35960_e40343_d_n9;
        locals.var_q2d__blk1002_rv = 0.0;

        let (assign35970_e40347, assign35970_e40347_d_n4, assign35970_e40347_d_n6, assign35970_e40347_d_n7, assign35970_e40347_d_n8, assign35970_e40347_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign35970_e40347;
        locals.var_q_rac_qsq__blk828_dn4 = assign35970_e40347_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign35970_e40347_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign35970_e40347_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign35970_e40347_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign35970_e40347_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign35980_e40351, assign35980_e40351_d_n4, assign35980_e40351_d_n6, assign35980_e40351_d_n7, assign35980_e40351_d_n8, assign35980_e40351_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign35980_e40351;
        locals.var_q_invexpq__blk831_dn4 = assign35980_e40351_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign35980_e40351_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign35980_e40351_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign35980_e40351_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign35980_e40351_d_n9;
        locals.var_q_invexpq__blk831_rv = 0.0;

        let (assign35990_e40357, assign35990_e40357_d_n4, assign35990_e40357_d_n6, assign35990_e40357_d_n7, assign35990_e40357_d_n8, assign35990_e40357_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35990_e40355: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign35990_e40355, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign35990_e40357;
        locals.var_q_k1q1__blk823_dn4 = assign35990_e40357_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign35990_e40357_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign35990_e40357_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign35990_e40357_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign35990_e40357_d_n9;
        locals.var_q_k1q1__blk823_rv = 0.0;

        let assign36000_e40360: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36000_e40362: f64 = (assign36000_e40360 - locals.var_xdeff__blk1000);
        let assign36000_e40364: f64 = if assign36000_e40362 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1169 = assign36000_e40364;
        locals.var_guard1169_rv = 0.0;

        let (assign36010_e40375, assign36010_e40375_d_n4, assign36010_e40375_d_n6, assign36010_e40375_d_n7, assign36010_e40375_d_n8, assign36010_e40375_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1169 != 0.0)) {
        let assign36010_e40370: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36010_e40372: f64 = (assign36010_e40370 - locals.var_xdeff__blk1000);
        let assign36010_e40373: f64 = (assign36010_e40372).exp();
        (assign36010_e40373, (assign36010_e40373 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign36010_e40373 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign36010_e40373 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign36010_e40373 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign36010_e40373 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36010_e40375;
        locals.var_q_temp1__blk814_dn4 = assign36010_e40375_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36010_e40375_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36010_e40375_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36010_e40375_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36010_e40375_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign36020_e40416, assign36020_e40416_d_n4, assign36020_e40416_d_n6, assign36020_e40416_d_n7, assign36020_e40416_d_n8, assign36020_e40416_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1169 == 0.0)) {
        let assign36020_e40384: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36020_e40386: f64 = (assign36020_e40384 - locals.var_xdeff__blk1000);
        let assign36020_e40388: f64 = (assign36020_e40386 - 80.0);
        let assign36020_e40393: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36020_e40395: f64 = (assign36020_e40393 - locals.var_xdeff__blk1000);
        let assign36020_e40397: f64 = (assign36020_e40395 - 80.0);
        let assign36020_e40398: f64 = (0.5 * assign36020_e40397);
        let assign36020_e40402: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36020_e40404: f64 = (assign36020_e40402 - locals.var_xdeff__blk1000);
        let assign36020_e40406: f64 = (assign36020_e40404 - 80.0);
        let assign36020_e40408: f64 = (assign36020_e40406 * 0.3333333333333);
        let assign36020_e40409: f64 = (1.0 + assign36020_e40408);
        let assign36020_e40410: f64 = (assign36020_e40398 * assign36020_e40409);
        let assign36020_e40411: f64 = (1.0 + assign36020_e40410);
        let assign36020_e40412: f64 = (assign36020_e40388 * assign36020_e40411);
        let assign36020_e40413: f64 = (1.0 + assign36020_e40412);
        let assign36020_e40414: f64 = (5.54062e34 * assign36020_e40413);
        (assign36020_e40414, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign36020_e40411) + (assign36020_e40388 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign36020_e40409) + (assign36020_e40398 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign36020_e40411) + (assign36020_e40388 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign36020_e40409) + (assign36020_e40398 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign36020_e40411) + (assign36020_e40388 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign36020_e40409) + (assign36020_e40398 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign36020_e40411) + (assign36020_e40388 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign36020_e40409) + (assign36020_e40398 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign36020_e40411) + (assign36020_e40388 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign36020_e40409) + (assign36020_e40398 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36020_e40416;
        locals.var_q_temp1__blk814_dn4 = assign36020_e40416_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36020_e40416_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36020_e40416_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36020_e40416_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36020_e40416_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_103(
        locals: &mut StampLocals,
    ) {
        let (assign36030_e40422, assign36030_e40422_d_n4, assign36030_e40422_d_n6, assign36030_e40422_d_n7, assign36030_e40422_d_n8, assign36030_e40422_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36030_e40420: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign36030_e40420, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign36030_e40422;
        locals.var_q_aexp__blk824_dn4 = assign36030_e40422_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign36030_e40422_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign36030_e40422_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign36030_e40422_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign36030_e40422_d_n9;
        locals.var_q_aexp__blk824_rv = 0.0;

        let (assign36040_e40430, assign36040_e40430_d_n4, assign36040_e40430_d_n6, assign36040_e40430_d_n7, assign36040_e40430_d_n8, assign36040_e40430_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36040_e40426: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign36040_e40428: f64 = (assign36040_e40426 - locals.var_q_aexp__blk824);
        (assign36040_e40428, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign36040_e40430;
        locals.var_q_qsq__blk825_dn4 = assign36040_e40430_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign36040_e40430_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign36040_e40430_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign36040_e40430_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign36040_e40430_d_n9;
        locals.var_q_qsq__blk825_rv = 0.0;

        let (assign36050_e40440, assign36050_e40440_d_n4, assign36050_e40440_d_n6, assign36050_e40440_d_n7, assign36050_e40440_d_n8, assign36050_e40440_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36050_e40434: f64 = (2.0 * locals.var_k1__blk932);
        let assign36050_e40436: f64 = (assign36050_e40434 * locals.var_q_k1q1__blk823);
        let assign36050_e40438: f64 = (assign36050_e40436 + locals.var_q_aexp__blk824);
        (assign36050_e40438, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign36050_e40434 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign36050_e40434 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign36050_e40434 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign36050_e40434 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign36050_e40434 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign36050_e40440;
        locals.var_q_d1_qsq__blk826_dn4 = assign36050_e40440_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign36050_e40440_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign36050_e40440_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign36050_e40440_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign36050_e40440_d_n9;
        locals.var_q_d1_qsq__blk826_rv = 0.0;

        let (assign36060_e40450, assign36060_e40450_d_n4, assign36060_e40450_d_n6, assign36060_e40450_d_n7, assign36060_e40450_d_n8, assign36060_e40450_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36060_e40444: f64 = (2.0 * locals.var_k1__blk932);
        let assign36060_e40446: f64 = (assign36060_e40444 * locals.var_k1__blk932);
        let assign36060_e40448: f64 = (assign36060_e40446 - locals.var_q_aexp__blk824);
        (assign36060_e40448, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign36060_e40444 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign36060_e40444 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign36060_e40444 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign36060_e40444 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign36060_e40444 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign36060_e40450;
        locals.var_q_d2_qsq__blk827_dn4 = assign36060_e40450_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign36060_e40450_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign36060_e40450_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign36060_e40450_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign36060_e40450_d_n9;
        locals.var_q_d2_qsq__blk827_rv = 0.0;

        let assign36070_e40453: f64 = (-0.005);
        let assign36070_e40454: f64 = if locals.var_q_qsq__blk825 < assign36070_e40453 { 1.0 } else { 0.0 };
        locals.var_guard1170 = assign36070_e40454;
        locals.var_guard1170_rv = 0.0;

        let (assign36080_e40462, assign36080_e40462_d_n4, assign36080_e40462_d_n6, assign36080_e40462_d_n7, assign36080_e40462_d_n8, assign36080_e40462_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36080_e40459: f64 = (locals.var_q_qsq__blk825).abs();
        let assign36080_e40460: f64 = (assign36080_e40459).sqrt();
        (assign36080_e40460, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign36080_e40460)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign36080_e40460)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign36080_e40460)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign36080_e40460)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign36080_e40460)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign36080_e40462;
        locals.var_q_rac_qsq__blk828_dn4 = assign36080_e40462_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign36080_e40462_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign36080_e40462_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign36080_e40462_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign36080_e40462_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign36090_e40473, assign36090_e40473_d_n4, assign36090_e40473_d_n6, assign36090_e40473_d_n7, assign36090_e40473_d_n8, assign36090_e40473_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36090_e40469: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign36090_e40470: f64 = (assign36090_e40469).tan();
        let assign36090_e40471: f64 = (locals.var_q_rac_qsq__blk828 / assign36090_e40470);
        (assign36090_e40471, (((locals.var_q_rac_qsq__blk828_dn4 * assign36090_e40470) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign36090_e40469).cos() * (assign36090_e40469).cos())))) / (assign36090_e40470 * assign36090_e40470)), (((locals.var_q_rac_qsq__blk828_dn6 * assign36090_e40470) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign36090_e40469).cos() * (assign36090_e40469).cos())))) / (assign36090_e40470 * assign36090_e40470)), (((locals.var_q_rac_qsq__blk828_dn7 * assign36090_e40470) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign36090_e40469).cos() * (assign36090_e40469).cos())))) / (assign36090_e40470 * assign36090_e40470)), (((locals.var_q_rac_qsq__blk828_dn8 * assign36090_e40470) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign36090_e40469).cos() * (assign36090_e40469).cos())))) / (assign36090_e40470 * assign36090_e40470)), (((locals.var_q_rac_qsq__blk828_dn9 * assign36090_e40470) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign36090_e40469).cos() * (assign36090_e40469).cos())))) / (assign36090_e40470 * assign36090_e40470)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign36090_e40473;
        locals.var_q_qcoth__blk829_dn4 = assign36090_e40473_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign36090_e40473_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign36090_e40473_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign36090_e40473_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign36090_e40473_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign36100_e40483, assign36100_e40483_d_n4, assign36100_e40483_d_n6, assign36100_e40483_d_n7, assign36100_e40483_d_n8, assign36100_e40483_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36100_e40479: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign36100_e40481: f64 = (assign36100_e40479 / locals.var_q_qsq__blk825);
        (assign36100_e40481, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign36100_e40479 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign36100_e40479 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign36100_e40479 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign36100_e40479 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign36100_e40479 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36100_e40483;
        locals.var_q_temp1__blk814_dn4 = assign36100_e40483_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36100_e40483_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36100_e40483_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36100_e40483_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36100_e40483_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign36110_e40497, assign36110_e40497_d_n4, assign36110_e40497_d_n6, assign36110_e40497_d_n7, assign36110_e40497_d_n8, assign36110_e40497_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36110_e40491: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign36110_e40492: f64 = (locals.var_q_qcoth__blk829 * assign36110_e40491);
        let assign36110_e40493: f64 = (locals.var_q_qsq__blk825 + assign36110_e40492);
        let assign36110_e40495: f64 = (assign36110_e40493 * locals.var_q_temp1__blk814);
        (assign36110_e40495, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign36110_e40491) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign36110_e40493 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign36110_e40491) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign36110_e40493 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign36110_e40491) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign36110_e40493 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign36110_e40491) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign36110_e40493 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign36110_e40491) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign36110_e40493 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign36110_e40497;
        locals.var_q_d1_qcoth__blk830_dn4 = assign36110_e40497_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign36110_e40497_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign36110_e40497_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign36110_e40497_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign36110_e40497_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign36120_e40519, assign36120_e40519_d_n4, assign36120_e40519_d_n6, assign36120_e40519_d_n7, assign36120_e40519_d_n8, assign36120_e40519_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36120_e40504: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign36120_e40507: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign36120_e40508: f64 = (assign36120_e40504 * assign36120_e40507);
        let assign36120_e40509: f64 = (locals.var_q_d1_qsq__blk826 - assign36120_e40508);
        let assign36120_e40511: f64 = (assign36120_e40509 * locals.var_q_temp1__blk814);
        let assign36120_e40514: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign36120_e40516: f64 = (assign36120_e40514 / locals.var_q_d1_qsq__blk826);
        let assign36120_e40517: f64 = (assign36120_e40511 + assign36120_e40516);
        (assign36120_e40517, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign36120_e40507) + (assign36120_e40504 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign36120_e40509 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign36120_e40514 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign36120_e40507) + (assign36120_e40504 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign36120_e40509 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign36120_e40514 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign36120_e40507) + (assign36120_e40504 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign36120_e40509 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign36120_e40514 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign36120_e40507) + (assign36120_e40504 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign36120_e40509 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign36120_e40514 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign36120_e40507) + (assign36120_e40504 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign36120_e40509 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign36120_e40514 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign36120_e40519;
        locals.var_q_d2_qcoth__blk832_dn4 = assign36120_e40519_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign36120_e40519_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign36120_e40519_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign36120_e40519_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign36120_e40519_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign36130_e40529, assign36130_e40529_d_n4, assign36130_e40529_d_n6, assign36130_e40529_d_n7, assign36130_e40529_d_n8, assign36130_e40529_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36130_e40526: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign36130_e40527: f64 = (1.0 - assign36130_e40526);
        (assign36130_e40527, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36130_e40529;
        locals.var_q_temp2__blk815_dn4 = assign36130_e40529_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36130_e40529_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36130_e40529_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36130_e40529_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36130_e40529_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign36140_e40539, assign36140_e40539_d_n4, assign36140_e40539_d_n6, assign36140_e40539_d_n7, assign36140_e40539_d_n8, assign36140_e40539_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36140_e40535: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign36140_e40537: f64 = (assign36140_e40535 * locals.var_q_temp2__blk815);
        (assign36140_e40537, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36140_e40535 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36140_e40535 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36140_e40535 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36140_e40535 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36140_e40535 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign36140_e40539;
        locals.var_q_d1_ln__blk835_dn4 = assign36140_e40539_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign36140_e40539_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign36140_e40539_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign36140_e40539_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign36140_e40539_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign36150_e40557, assign36150_e40557_d_n4, assign36150_e40557_d_n6, assign36150_e40557_d_n7, assign36150_e40557_d_n8, assign36150_e40557_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36150_e40545: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign36150_e40550: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign36150_e40551: f64 = (locals.var_q_d1_ln__blk835 + assign36150_e40550);
        let assign36150_e40552: f64 = (locals.var_q_d1_qsq__blk826 * assign36150_e40551);
        let assign36150_e40553: f64 = (assign36150_e40545 - assign36150_e40552);
        let assign36150_e40555: f64 = (assign36150_e40553 / locals.var_q_qsq__blk825);
        (assign36150_e40555, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign36150_e40551) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign36150_e40553 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign36150_e40551) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign36150_e40553 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign36150_e40551) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign36150_e40553 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign36150_e40551) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign36150_e40553 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign36150_e40551) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign36150_e40553 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign36150_e40557;
        locals.var_q_d2_ln__blk836_dn4 = assign36150_e40557_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign36150_e40557_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign36150_e40557_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign36150_e40557_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign36150_e40557_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let assign36160_e40560: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1171 = assign36160_e40560;
        locals.var_guard1171_rv = 0.0;

        let (assign36170_e40571, assign36170_e40571_d_n4, assign36170_e40571_d_n6, assign36170_e40571_d_n7, assign36170_e40571_d_n8, assign36170_e40571_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36170_e40568: f64 = (locals.var_q_qsq__blk825).abs();
        let assign36170_e40569: f64 = (assign36170_e40568).sqrt();
        (assign36170_e40569, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign36170_e40569)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign36170_e40569)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign36170_e40569)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign36170_e40569)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign36170_e40569)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign36170_e40571;
        locals.var_q_rac_qsq__blk828_dn4 = assign36170_e40571_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign36170_e40571_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign36170_e40571_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign36170_e40571_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign36170_e40571_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign36180_e40582, assign36180_e40582_d_n4, assign36180_e40582_d_n6, assign36180_e40582_d_n7, assign36180_e40582_d_n8, assign36180_e40582_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36180_e40579: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign36180_e40580: f64 = (assign36180_e40579).exp();
        (assign36180_e40580, (assign36180_e40580 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign36180_e40580 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign36180_e40580 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign36180_e40580 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign36180_e40580 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign36180_e40582;
        locals.var_q_invexpq__blk831_dn4 = assign36180_e40582_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign36180_e40582_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign36180_e40582_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign36180_e40582_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign36180_e40582_d_n9;
        locals.var_q_invexpq__blk831_rv = 0.0;

        let (assign36190_e40599, assign36190_e40599_d_n4, assign36190_e40599_d_n6, assign36190_e40599_d_n7, assign36190_e40599_d_n8, assign36190_e40599_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36190_e40592: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign36190_e40593: f64 = (locals.var_q_rac_qsq__blk828 * assign36190_e40592);
        let assign36190_e40596: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign36190_e40597: f64 = (assign36190_e40593 / assign36190_e40596);
        (assign36190_e40597, (((((locals.var_q_rac_qsq__blk828_dn4 * assign36190_e40592) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign36190_e40596) - (assign36190_e40593 * (-locals.var_q_invexpq__blk831_dn4))) / (assign36190_e40596 * assign36190_e40596)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign36190_e40592) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign36190_e40596) - (assign36190_e40593 * (-locals.var_q_invexpq__blk831_dn6))) / (assign36190_e40596 * assign36190_e40596)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign36190_e40592) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign36190_e40596) - (assign36190_e40593 * (-locals.var_q_invexpq__blk831_dn7))) / (assign36190_e40596 * assign36190_e40596)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign36190_e40592) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign36190_e40596) - (assign36190_e40593 * (-locals.var_q_invexpq__blk831_dn8))) / (assign36190_e40596 * assign36190_e40596)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign36190_e40592) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign36190_e40596) - (assign36190_e40593 * (-locals.var_q_invexpq__blk831_dn9))) / (assign36190_e40596 * assign36190_e40596)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign36190_e40599;
        locals.var_q_qcoth__blk829_dn4 = assign36190_e40599_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign36190_e40599_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign36190_e40599_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign36190_e40599_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign36190_e40599_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign36200_e40612, assign36200_e40612_d_n4, assign36200_e40612_d_n6, assign36200_e40612_d_n7, assign36200_e40612_d_n8, assign36200_e40612_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36200_e40608: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign36200_e40610: f64 = (assign36200_e40608 / locals.var_q_qsq__blk825);
        (assign36200_e40610, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign36200_e40608 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign36200_e40608 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign36200_e40608 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign36200_e40608 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign36200_e40608 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36200_e40612;
        locals.var_q_temp1__blk814_dn4 = assign36200_e40612_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36200_e40612_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36200_e40612_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36200_e40612_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36200_e40612_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign36210_e40629, assign36210_e40629_d_n4, assign36210_e40629_d_n6, assign36210_e40629_d_n7, assign36210_e40629_d_n8, assign36210_e40629_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36210_e40623: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign36210_e40624: f64 = (locals.var_q_qcoth__blk829 * assign36210_e40623);
        let assign36210_e40625: f64 = (locals.var_q_qsq__blk825 + assign36210_e40624);
        let assign36210_e40627: f64 = (assign36210_e40625 * locals.var_q_temp1__blk814);
        (assign36210_e40627, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign36210_e40623) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign36210_e40625 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign36210_e40623) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign36210_e40625 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign36210_e40623) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign36210_e40625 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign36210_e40623) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign36210_e40625 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign36210_e40623) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign36210_e40625 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign36210_e40629;
        locals.var_q_d1_qcoth__blk830_dn4 = assign36210_e40629_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign36210_e40629_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign36210_e40629_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign36210_e40629_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign36210_e40629_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign36220_e40654, assign36220_e40654_d_n4, assign36220_e40654_d_n6, assign36220_e40654_d_n7, assign36220_e40654_d_n8, assign36220_e40654_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36220_e40639: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign36220_e40642: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign36220_e40643: f64 = (assign36220_e40639 * assign36220_e40642);
        let assign36220_e40644: f64 = (locals.var_q_d1_qsq__blk826 - assign36220_e40643);
        let assign36220_e40646: f64 = (assign36220_e40644 * locals.var_q_temp1__blk814);
        let assign36220_e40649: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign36220_e40651: f64 = (assign36220_e40649 / locals.var_q_d1_qsq__blk826);
        let assign36220_e40652: f64 = (assign36220_e40646 + assign36220_e40651);
        (assign36220_e40652, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign36220_e40642) + (assign36220_e40639 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign36220_e40644 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign36220_e40649 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign36220_e40642) + (assign36220_e40639 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign36220_e40644 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign36220_e40649 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign36220_e40642) + (assign36220_e40639 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign36220_e40644 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign36220_e40649 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign36220_e40642) + (assign36220_e40639 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign36220_e40644 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign36220_e40649 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign36220_e40642) + (assign36220_e40639 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign36220_e40644 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign36220_e40649 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign36220_e40654;
        locals.var_q_d2_qcoth__blk832_dn4 = assign36220_e40654_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign36220_e40654_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign36220_e40654_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign36220_e40654_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign36220_e40654_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign36230_e40667, assign36230_e40667_d_n4, assign36230_e40667_d_n6, assign36230_e40667_d_n7, assign36230_e40667_d_n8, assign36230_e40667_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36230_e40664: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign36230_e40665: f64 = (1.0 - assign36230_e40664);
        (assign36230_e40665, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36230_e40667;
        locals.var_q_temp2__blk815_dn4 = assign36230_e40667_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36230_e40667_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36230_e40667_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36230_e40667_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36230_e40667_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign36240_e40680, assign36240_e40680_d_n4, assign36240_e40680_d_n6, assign36240_e40680_d_n7, assign36240_e40680_d_n8, assign36240_e40680_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36240_e40676: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign36240_e40678: f64 = (assign36240_e40676 * locals.var_q_temp2__blk815);
        (assign36240_e40678, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36240_e40676 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36240_e40676 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36240_e40676 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36240_e40676 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36240_e40676 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign36240_e40680;
        locals.var_q_d1_ln__blk835_dn4 = assign36240_e40680_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign36240_e40680_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign36240_e40680_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign36240_e40680_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign36240_e40680_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign36250_e40701, assign36250_e40701_d_n4, assign36250_e40701_d_n6, assign36250_e40701_d_n7, assign36250_e40701_d_n8, assign36250_e40701_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36250_e40689: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign36250_e40694: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign36250_e40695: f64 = (locals.var_q_d1_ln__blk835 + assign36250_e40694);
        let assign36250_e40696: f64 = (locals.var_q_d1_qsq__blk826 * assign36250_e40695);
        let assign36250_e40697: f64 = (assign36250_e40689 - assign36250_e40696);
        let assign36250_e40699: f64 = (assign36250_e40697 / locals.var_q_qsq__blk825);
        (assign36250_e40699, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign36250_e40695) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign36250_e40697 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign36250_e40695) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign36250_e40697 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign36250_e40695) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign36250_e40697 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign36250_e40695) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign36250_e40697 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign36250_e40695) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign36250_e40697 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign36250_e40701;
        locals.var_q_d2_ln__blk836_dn4 = assign36250_e40701_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign36250_e40701_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign36250_e40701_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign36250_e40701_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign36250_e40701_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let (assign36260_e40729, assign36260_e40729_d_n4, assign36260_e40729_d_n6, assign36260_e40729_d_n7, assign36260_e40729_d_n8, assign36260_e40729_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36260_e40713: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign36260_e40717: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign36260_e40721: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign36260_e40722: f64 = (1.0 - assign36260_e40721);
        let assign36260_e40723: f64 = (assign36260_e40717 * assign36260_e40722);
        let assign36260_e40724: f64 = (1.0 - assign36260_e40723);
        let assign36260_e40725: f64 = (assign36260_e40713 * assign36260_e40724);
        let assign36260_e40726: f64 = (1.0 - assign36260_e40725);
        let assign36260_e40727: f64 = (0.1666666666667 * assign36260_e40726);
        (assign36260_e40727, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign36260_e40724) + (assign36260_e40713 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign36260_e40722) + (assign36260_e40717 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign36260_e40724) + (assign36260_e40713 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign36260_e40722) + (assign36260_e40717 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign36260_e40724) + (assign36260_e40713 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign36260_e40722) + (assign36260_e40717 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign36260_e40724) + (assign36260_e40713 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign36260_e40722) + (assign36260_e40717 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign36260_e40724) + (assign36260_e40713 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign36260_e40722) + (assign36260_e40717 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign36260_e40729;
        locals.var_q_temp3__blk816_dn4 = assign36260_e40729_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign36260_e40729_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign36260_e40729_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign36260_e40729_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign36260_e40729_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign36270_e40743, assign36270_e40743_d_n4, assign36270_e40743_d_n6, assign36270_e40743_d_n7, assign36270_e40743_d_n8, assign36270_e40743_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36270_e40740: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign36270_e40741: f64 = (2.0 + assign36270_e40740);
        (assign36270_e40741, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign36270_e40743;
        locals.var_q_qcoth__blk829_dn4 = assign36270_e40743_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign36270_e40743_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign36270_e40743_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign36270_e40743_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign36270_e40743_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign36280_e40771, assign36280_e40771_d_n4, assign36280_e40771_d_n6, assign36280_e40771_d_n7, assign36280_e40771_d_n8, assign36280_e40771_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36280_e40755: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign36280_e40759: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign36280_e40763: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign36280_e40764: f64 = (1.0 - assign36280_e40763);
        let assign36280_e40765: f64 = (assign36280_e40759 * assign36280_e40764);
        let assign36280_e40766: f64 = (1.0 - assign36280_e40765);
        let assign36280_e40767: f64 = (assign36280_e40755 * assign36280_e40766);
        let assign36280_e40768: f64 = (1.0 - assign36280_e40767);
        let assign36280_e40769: f64 = (0.1666666666667 * assign36280_e40768);
        (assign36280_e40769, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign36280_e40766) + (assign36280_e40755 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign36280_e40764) + (assign36280_e40759 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign36280_e40766) + (assign36280_e40755 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign36280_e40764) + (assign36280_e40759 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign36280_e40766) + (assign36280_e40755 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign36280_e40764) + (assign36280_e40759 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign36280_e40766) + (assign36280_e40755 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign36280_e40764) + (assign36280_e40759 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign36280_e40766) + (assign36280_e40755 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign36280_e40764) + (assign36280_e40759 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36280_e40771;
        locals.var_q_temp1__blk814_dn4 = assign36280_e40771_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36280_e40771_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36280_e40771_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36280_e40771_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36280_e40771_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign36290_e40783, assign36290_e40783_d_n4, assign36290_e40783_d_n6, assign36290_e40783_d_n7, assign36290_e40783_d_n8, assign36290_e40783_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36290_e40781: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign36290_e40781, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign36290_e40783;
        locals.var_q_d1_qcoth__blk830_dn4 = assign36290_e40783_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign36290_e40783_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign36290_e40783_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign36290_e40783_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign36290_e40783_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign36300_e40811, assign36300_e40811_d_n4, assign36300_e40811_d_n6, assign36300_e40811_d_n7, assign36300_e40811_d_n8, assign36300_e40811_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36300_e40795: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign36300_e40799: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign36300_e40803: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign36300_e40804: f64 = (1.0 - assign36300_e40803);
        let assign36300_e40805: f64 = (assign36300_e40799 * assign36300_e40804);
        let assign36300_e40806: f64 = (1.0 - assign36300_e40805);
        let assign36300_e40807: f64 = (assign36300_e40795 * assign36300_e40806);
        let assign36300_e40808: f64 = (1.0 - assign36300_e40807);
        let assign36300_e40809: f64 = (0.0055555555556 * assign36300_e40808);
        (assign36300_e40809, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign36300_e40806) + (assign36300_e40795 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign36300_e40804) + (assign36300_e40799 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign36300_e40806) + (assign36300_e40795 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign36300_e40804) + (assign36300_e40799 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign36300_e40806) + (assign36300_e40795 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign36300_e40804) + (assign36300_e40799 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign36300_e40806) + (assign36300_e40795 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign36300_e40804) + (assign36300_e40799 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign36300_e40806) + (assign36300_e40795 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign36300_e40804) + (assign36300_e40799 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36300_e40811;
        locals.var_q_temp2__blk815_dn4 = assign36300_e40811_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36300_e40811_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36300_e40811_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36300_e40811_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36300_e40811_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_104(
        locals: &mut StampLocals,
    ) {
        let (assign36310_e40829, assign36310_e40829_d_n4, assign36310_e40829_d_n6, assign36310_e40829_d_n7, assign36310_e40829_d_n8, assign36310_e40829_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36310_e40821: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign36310_e40824: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign36310_e40826: f64 = (assign36310_e40824 * locals.var_q_temp2__blk815);
        let assign36310_e40827: f64 = (assign36310_e40821 - assign36310_e40826);
        (assign36310_e40827, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign36310_e40824 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign36310_e40824 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign36310_e40824 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign36310_e40824 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign36310_e40824 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign36310_e40829;
        locals.var_q_d2_qcoth__blk832_dn4 = assign36310_e40829_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign36310_e40829_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign36310_e40829_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign36310_e40829_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign36310_e40829_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign36320_e40844, assign36320_e40844_d_n4, assign36320_e40844_d_n6, assign36320_e40844_d_n7, assign36320_e40844_d_n8, assign36320_e40844_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36320_e40838: f64 = (-0.5);
        let assign36320_e40840: f64 = (assign36320_e40838 * locals.var_q_d1_qsq__blk826);
        let assign36320_e40842: f64 = (assign36320_e40840 * locals.var_q_temp3__blk816);
        (assign36320_e40842, (((assign36320_e40838 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign36320_e40840 * locals.var_q_temp3__blk816_dn4)), (((assign36320_e40838 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign36320_e40840 * locals.var_q_temp3__blk816_dn6)), (((assign36320_e40838 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign36320_e40840 * locals.var_q_temp3__blk816_dn7)), (((assign36320_e40838 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign36320_e40840 * locals.var_q_temp3__blk816_dn8)), (((assign36320_e40838 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign36320_e40840 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign36320_e40844;
        locals.var_q_d1_ln__blk835_dn4 = assign36320_e40844_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign36320_e40844_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign36320_e40844_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign36320_e40844_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign36320_e40844_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign36330_e40879, assign36330_e40879_d_n4, assign36330_e40879_d_n6, assign36330_e40879_d_n7, assign36330_e40879_d_n8, assign36330_e40879_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36330_e40853: f64 = (-0.5);
        let assign36330_e40855: f64 = (assign36330_e40853 * locals.var_q_d2_qsq__blk827);
        let assign36330_e40857: f64 = (assign36330_e40855 * locals.var_q_temp3__blk816);
        let assign36330_e40860: f64 = (0.25 * 0.0055555555556);
        let assign36330_e40862: f64 = (assign36330_e40860 * locals.var_q_d1_qsq__blk826);
        let assign36330_e40864: f64 = (assign36330_e40862 * locals.var_q_d1_qsq__blk826);
        let assign36330_e40868: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign36330_e40872: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign36330_e40873: f64 = (2.0 - assign36330_e40872);
        let assign36330_e40874: f64 = (assign36330_e40868 * assign36330_e40873);
        let assign36330_e40875: f64 = (1.0 - assign36330_e40874);
        let assign36330_e40876: f64 = (assign36330_e40864 * assign36330_e40875);
        let assign36330_e40877: f64 = (assign36330_e40857 + assign36330_e40876);
        (assign36330_e40877, ((((assign36330_e40853 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign36330_e40855 * locals.var_q_temp3__blk816_dn4)) + (((((assign36330_e40860 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign36330_e40862 * locals.var_q_d1_qsq__blk826_dn4)) * assign36330_e40875) + (assign36330_e40864 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign36330_e40873) + (assign36330_e40868 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign36330_e40853 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign36330_e40855 * locals.var_q_temp3__blk816_dn6)) + (((((assign36330_e40860 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign36330_e40862 * locals.var_q_d1_qsq__blk826_dn6)) * assign36330_e40875) + (assign36330_e40864 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign36330_e40873) + (assign36330_e40868 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign36330_e40853 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign36330_e40855 * locals.var_q_temp3__blk816_dn7)) + (((((assign36330_e40860 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign36330_e40862 * locals.var_q_d1_qsq__blk826_dn7)) * assign36330_e40875) + (assign36330_e40864 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign36330_e40873) + (assign36330_e40868 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign36330_e40853 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign36330_e40855 * locals.var_q_temp3__blk816_dn8)) + (((((assign36330_e40860 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign36330_e40862 * locals.var_q_d1_qsq__blk826_dn8)) * assign36330_e40875) + (assign36330_e40864 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign36330_e40873) + (assign36330_e40868 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign36330_e40853 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign36330_e40855 * locals.var_q_temp3__blk816_dn9)) + (((((assign36330_e40860 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign36330_e40862 * locals.var_q_d1_qsq__blk826_dn9)) * assign36330_e40875) + (assign36330_e40864 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign36330_e40873) + (assign36330_e40868 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign36330_e40879;
        locals.var_q_d2_ln__blk836_dn4 = assign36330_e40879_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign36330_e40879_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign36330_e40879_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign36330_e40879_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign36330_e40879_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let assign36340_e40882: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign36340_e40882;
        locals.var_guard1172_rv = 0.0;

        let (assign36350_e40898, assign36350_e40898_d_n4, assign36350_e40898_d_n6, assign36350_e40898_d_n7, assign36350_e40898_d_n8, assign36350_e40898_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1172 != 0.0)) {
        let assign36350_e40888: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign36350_e40893: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign36350_e40894: f64 = (locals.var_q_invexpq__blk831 * assign36350_e40893);
        let assign36350_e40895: f64 = (1.0 - assign36350_e40894);
        let assign36350_e40896: f64 = (assign36350_e40888 / assign36350_e40895);
        (assign36350_e40896, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign36350_e40895) - (assign36350_e40888 * (-((locals.var_q_invexpq__blk831_dn4 * assign36350_e40893) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign36350_e40895 * assign36350_e40895)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign36350_e40895) - (assign36350_e40888 * (-((locals.var_q_invexpq__blk831_dn6 * assign36350_e40893) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign36350_e40895 * assign36350_e40895)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign36350_e40895) - (assign36350_e40888 * (-((locals.var_q_invexpq__blk831_dn7 * assign36350_e40893) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign36350_e40895 * assign36350_e40895)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign36350_e40895) - (assign36350_e40888 * (-((locals.var_q_invexpq__blk831_dn8 * assign36350_e40893) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign36350_e40895 * assign36350_e40895)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign36350_e40895) - (assign36350_e40888 * (-((locals.var_q_invexpq__blk831_dn9 * assign36350_e40893) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign36350_e40895 * assign36350_e40895)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36350_e40898;
        locals.var_q_temp2__blk815_dn4 = assign36350_e40898_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36350_e40898_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36350_e40898_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36350_e40898_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36350_e40898_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign36360_e40906, assign36360_e40906_d_n4, assign36360_e40906_d_n6, assign36360_e40906_d_n7, assign36360_e40906_d_n8, assign36360_e40906_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1172 != 0.0)) {
        let assign36360_e40904: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign36360_e40904, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign36360_e40906;
        locals.var_q_sh_term__blk833_dn4 = assign36360_e40906_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign36360_e40906_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign36360_e40906_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign36360_e40906_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign36360_e40906_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign36370_e40915, assign36370_e40915_d_n4, assign36370_e40915_d_n6, assign36370_e40915_d_n7, assign36370_e40915_d_n8, assign36370_e40915_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1172 != 0.0)) {
        let assign36370_e40911: f64 = (locals.var_q_temp2__blk815).ln();
        let assign36370_e40913: f64 = (assign36370_e40911 - locals.var_q_rac_qsq__blk828);
        (assign36370_e40913, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign36370_e40915;
        locals.var_q_ln_term__blk834_dn4 = assign36370_e40915_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign36370_e40915_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign36370_e40915_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign36370_e40915_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign36370_e40915_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let assign36380_e40918: f64 = (-0.005);
        let assign36380_e40919: f64 = if locals.var_q_qsq__blk825 < assign36380_e40918 { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign36380_e40919;
        locals.var_guard1173_rv = 0.0;

        let (assign36390_e40931, assign36390_e40931_d_n4, assign36390_e40931_d_n6, assign36390_e40931_d_n7, assign36390_e40931_d_n8, assign36390_e40931_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1172 == 0.0)) && (locals.var_guard1173 != 0.0)) {
        let assign36390_e40928: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign36390_e40929: f64 = (assign36390_e40928).sin();
        (assign36390_e40929, ((assign36390_e40928).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign36390_e40928).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign36390_e40928).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign36390_e40928).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign36390_e40928).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36390_e40931;
        locals.var_q_temp2__blk815_dn4 = assign36390_e40931_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36390_e40931_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36390_e40931_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36390_e40931_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36390_e40931_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign36400_e40945, assign36400_e40945_d_n4, assign36400_e40945_d_n6, assign36400_e40945_d_n7, assign36400_e40945_d_n8, assign36400_e40945_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1172 == 0.0)) && (locals.var_guard1173 != 0.0)) {
        let assign36400_e40939: f64 = (-locals.var_q_qsq__blk825);
        let assign36400_e40942: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign36400_e40943: f64 = (assign36400_e40939 / assign36400_e40942);
        (assign36400_e40943, ((((-locals.var_q_qsq__blk825_dn4) * assign36400_e40942) - (assign36400_e40939 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign36400_e40942 * assign36400_e40942)), ((((-locals.var_q_qsq__blk825_dn6) * assign36400_e40942) - (assign36400_e40939 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign36400_e40942 * assign36400_e40942)), ((((-locals.var_q_qsq__blk825_dn7) * assign36400_e40942) - (assign36400_e40939 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign36400_e40942 * assign36400_e40942)), ((((-locals.var_q_qsq__blk825_dn8) * assign36400_e40942) - (assign36400_e40939 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign36400_e40942 * assign36400_e40942)), ((((-locals.var_q_qsq__blk825_dn9) * assign36400_e40942) - (assign36400_e40939 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign36400_e40942 * assign36400_e40942)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign36400_e40945;
        locals.var_q_sh_term__blk833_dn4 = assign36400_e40945_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign36400_e40945_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign36400_e40945_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign36400_e40945_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign36400_e40945_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign36410_e40955, assign36410_e40955_d_n4, assign36410_e40955_d_n6, assign36410_e40955_d_n7, assign36410_e40955_d_n8, assign36410_e40955_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1172 == 0.0)) && (locals.var_guard1173 != 0.0)) {
        let assign36410_e40953: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign36410_e40953, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign36410_e40955;
        locals.var_q_ln_term__blk834_dn4 = assign36410_e40955_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign36410_e40955_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign36410_e40955_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign36410_e40955_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign36410_e40955_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let (assign36420_e40981, assign36420_e40981_d_n4, assign36420_e40981_d_n6, assign36420_e40981_d_n7, assign36420_e40981_d_n8, assign36420_e40981_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1172 == 0.0)) && (locals.var_guard1173 == 0.0)) {
        let assign36420_e40966: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign36420_e40970: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign36420_e40974: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign36420_e40975: f64 = (1.0 - assign36420_e40974);
        let assign36420_e40976: f64 = (assign36420_e40970 * assign36420_e40975);
        let assign36420_e40977: f64 = (1.0 - assign36420_e40976);
        let assign36420_e40978: f64 = (assign36420_e40966 * assign36420_e40977);
        let assign36420_e40979: f64 = (4.0 - assign36420_e40978);
        (assign36420_e40979, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign36420_e40977) + (assign36420_e40966 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign36420_e40975) + (assign36420_e40970 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign36420_e40977) + (assign36420_e40966 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign36420_e40975) + (assign36420_e40970 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign36420_e40977) + (assign36420_e40966 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign36420_e40975) + (assign36420_e40970 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign36420_e40977) + (assign36420_e40966 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign36420_e40975) + (assign36420_e40970 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign36420_e40977) + (assign36420_e40966 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign36420_e40975) + (assign36420_e40970 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign36420_e40981;
        locals.var_q_sh_term__blk833_dn4 = assign36420_e40981_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign36420_e40981_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign36420_e40981_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign36420_e40981_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign36420_e40981_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign36430_e40992, assign36430_e40992_d_n4, assign36430_e40992_d_n6, assign36430_e40992_d_n7, assign36430_e40992_d_n8, assign36430_e40992_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1172 == 0.0)) && (locals.var_guard1173 == 0.0)) {
        let assign36430_e40990: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign36430_e40990, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign36430_e40992;
        locals.var_q_ln_term__blk834_dn4 = assign36430_e40992_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign36430_e40992_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign36430_e40992_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign36430_e40992_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign36430_e40992_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let assign36440_e40995: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign36440_e40997: f64 = (assign36440_e40995 + locals.var_q_qcoth__blk829);
        let assign36440_e40999: f64 = if assign36440_e40997 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign36440_e40999;
        locals.var_guard1174_rv = 0.0;

        let (assign36450_e41007, assign36450_e41007_d_n4, assign36450_e41007_d_n6, assign36450_e41007_d_n7, assign36450_e41007_d_n8, assign36450_e41007_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 != 0.0)) {
        let assign36450_e41005: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign36450_e41005, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign36450_e41007;
        locals.var_q_expnum__blk837_dn4 = assign36450_e41007_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign36450_e41007_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign36450_e41007_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign36450_e41007_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign36450_e41007_d_n9;
        locals.var_q_expnum__blk837_rv = 0.0;

        let (assign36460_e41015, assign36460_e41015_d_n4, assign36460_e41015_d_n6, assign36460_e41015_d_n7, assign36460_e41015_d_n8, assign36460_e41015_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 != 0.0)) {
        let assign36460_e41013: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign36460_e41013, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign36460_e41015;
        locals.var_q_d1_expnum__blk838_dn4 = assign36460_e41015_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign36460_e41015_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign36460_e41015_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign36460_e41015_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign36460_e41015_d_n9;
        locals.var_q_d1_expnum__blk838_rv = 0.0;

        let (assign36470_e41021, assign36470_e41021_d_n4, assign36470_e41021_d_n6, assign36470_e41021_d_n7, assign36470_e41021_d_n8, assign36470_e41021_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign36470_e41021;
        locals.var_q_d2_expnum__blk839_dn4 = assign36470_e41021_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign36470_e41021_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign36470_e41021_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign36470_e41021_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign36470_e41021_d_n9;
        locals.var_q_d2_expnum__blk839_rv = 0.0;

        let (assign36480_e41032, assign36480_e41032_d_n4, assign36480_e41032_d_n6, assign36480_e41032_d_n7, assign36480_e41032_d_n8, assign36480_e41032_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign36480_e41029: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign36480_e41030: f64 = (1.0 / assign36480_e41029);
        (assign36480_e41030, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign36480_e41029 * assign36480_e41029))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign36480_e41029 * assign36480_e41029))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign36480_e41029 * assign36480_e41029))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign36480_e41029 * assign36480_e41029))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign36480_e41029 * assign36480_e41029))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36480_e41032;
        locals.var_q_temp2__blk815_dn4 = assign36480_e41032_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36480_e41032_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36480_e41032_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36480_e41032_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36480_e41032_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign36490_e41041, assign36490_e41041_d_n4, assign36490_e41041_d_n6, assign36490_e41041_d_n7, assign36490_e41041_d_n8, assign36490_e41041_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign36490_e41039: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign36490_e41039, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign36490_e41041;
        locals.var_q_temp3__blk816_dn4 = assign36490_e41041_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign36490_e41041_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign36490_e41041_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign36490_e41041_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign36490_e41041_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign36500_e41052, assign36500_e41052_d_n4, assign36500_e41052_d_n6, assign36500_e41052_d_n7, assign36500_e41052_d_n8, assign36500_e41052_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign36500_e41048: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign36500_e41050: f64 = (assign36500_e41048 * locals.var_q_temp2__blk815);
        (assign36500_e41050, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign36500_e41048 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign36500_e41048 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign36500_e41048 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign36500_e41048 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign36500_e41048 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign36500_e41052;
        locals.var_q_expnum__blk837_dn4 = assign36500_e41052_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign36500_e41052_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign36500_e41052_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign36500_e41052_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign36500_e41052_d_n9;
        locals.var_q_expnum__blk837_rv = 0.0;

        let (assign36510_e41069, assign36510_e41069_d_n4, assign36510_e41069_d_n6, assign36510_e41069_d_n7, assign36510_e41069_d_n8, assign36510_e41069_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign36510_e41059: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign36510_e41061: f64 = (assign36510_e41059 - locals.var_q_aexp__blk824);
        let assign36510_e41064: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign36510_e41065: f64 = (assign36510_e41061 - assign36510_e41064);
        let assign36510_e41067: f64 = (assign36510_e41065 * locals.var_q_temp2__blk815);
        (assign36510_e41067, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign36510_e41065 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign36510_e41065 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign36510_e41065 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign36510_e41065 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign36510_e41065 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign36510_e41069;
        locals.var_q_d1_expnum__blk838_dn4 = assign36510_e41069_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign36510_e41069_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign36510_e41069_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign36510_e41069_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign36510_e41069_d_n9;
        locals.var_q_d1_expnum__blk838_rv = 0.0;

        let (assign36520_e41096, assign36520_e41096_d_n4, assign36520_e41096_d_n6, assign36520_e41096_d_n7, assign36520_e41096_d_n8, assign36520_e41096_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign36520_e41076: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign36520_e41079: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign36520_e41081: f64 = (assign36520_e41079 * locals.var_q_d1_expnum__blk838);
        let assign36520_e41082: f64 = (assign36520_e41076 + assign36520_e41081);
        let assign36520_e41084: f64 = (assign36520_e41082 + locals.var_q_aexp__blk824);
        let assign36520_e41088: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign36520_e41089: f64 = (locals.var_q_d2_ln__blk836 + assign36520_e41088);
        let assign36520_e41091: f64 = (assign36520_e41089 * locals.var_q_sh_term__blk833);
        let assign36520_e41092: f64 = (assign36520_e41084 - assign36520_e41091);
        let assign36520_e41094: f64 = (assign36520_e41092 * locals.var_q_temp2__blk815);
        (assign36520_e41094, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign36520_e41079 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign36520_e41089 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign36520_e41092 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign36520_e41079 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign36520_e41089 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign36520_e41092 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign36520_e41079 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign36520_e41089 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign36520_e41092 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign36520_e41079 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign36520_e41089 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign36520_e41092 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign36520_e41079 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign36520_e41089 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign36520_e41092 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign36520_e41096;
        locals.var_q_d2_expnum__blk839_dn4 = assign36520_e41096_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign36520_e41096_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign36520_e41096_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign36520_e41096_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign36520_e41096_d_n9;
        locals.var_q_d2_expnum__blk839_rv = 0.0;

        let assign36530_e41099: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1175 = assign36530_e41099;
        locals.var_guard1175_rv = 0.0;

        let (assign36540_e41106, assign36540_e41106_d_n4, assign36540_e41106_d_n6, assign36540_e41106_d_n7, assign36540_e41106_d_n8, assign36540_e41106_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 != 0.0)) {
        let assign36540_e41104: f64 = (locals.var_q_expnum__blk837).ln();
        (assign36540_e41104, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign36540_e41106;
        locals.var_q_lnexpnum__blk840_dn4 = assign36540_e41106_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign36540_e41106_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign36540_e41106_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign36540_e41106_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign36540_e41106_d_n9;
        locals.var_q_lnexpnum__blk840_rv = 0.0;

        let (assign36550_e41114, assign36550_e41114_d_n4, assign36550_e41114_d_n6, assign36550_e41114_d_n7, assign36550_e41114_d_n8, assign36550_e41114_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 != 0.0)) {
        let assign36550_e41112: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign36550_e41112, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36550_e41114;
        locals.var_q_temp1__blk814_dn4 = assign36550_e41114_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36550_e41114_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36550_e41114_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36550_e41114_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36550_e41114_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign36560_e41122, assign36560_e41122_d_n4, assign36560_e41122_d_n6, assign36560_e41122_d_n7, assign36560_e41122_d_n8, assign36560_e41122_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 != 0.0)) {
        let assign36560_e41120: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign36560_e41120, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign36560_e41122;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign36560_e41122_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign36560_e41122_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign36560_e41122_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign36560_e41122_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign36560_e41122_d_n9;
        locals.var_q_d1_lnexpnum__blk841_rv = 0.0;

        let (assign36570_e41134, assign36570_e41134_d_n4, assign36570_e41134_d_n6, assign36570_e41134_d_n7, assign36570_e41134_d_n8, assign36570_e41134_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 != 0.0)) {
        let assign36570_e41128: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign36570_e41131: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign36570_e41132: f64 = (assign36570_e41128 - assign36570_e41131);
        (assign36570_e41132, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign36570_e41134;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign36570_e41134_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign36570_e41134_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign36570_e41134_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign36570_e41134_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign36570_e41134_d_n9;
        locals.var_q_d2_lnexpnum__blk842_rv = 0.0;

        let (assign36580_e41147, assign36580_e41147_d_n4, assign36580_e41147_d_n6, assign36580_e41147_d_n7, assign36580_e41147_d_n8, assign36580_e41147_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 == 0.0)) {
        let assign36580_e41141: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign36580_e41143: f64 = (-locals.var_q_k1q1__blk823);
        let assign36580_e41144: f64 = (assign36580_e41143).ln();
        let assign36580_e41145: f64 = (assign36580_e41141 + assign36580_e41144);
        (assign36580_e41145, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign36580_e41143)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign36580_e41143)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign36580_e41143)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign36580_e41143)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign36580_e41143)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign36580_e41147;
        locals.var_q_lnexpnum__blk840_dn4 = assign36580_e41147_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign36580_e41147_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign36580_e41147_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign36580_e41147_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign36580_e41147_d_n9;
        locals.var_q_lnexpnum__blk840_rv = 0.0;

        let (assign36590_e41156, assign36590_e41156_d_n4, assign36590_e41156_d_n6, assign36590_e41156_d_n7, assign36590_e41156_d_n8, assign36590_e41156_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 == 0.0)) {
        let assign36590_e41154: f64 = (1.0 / locals.var_q1d__blk1001);
        (assign36590_e41154, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36590_e41156;
        locals.var_q_temp1__blk814_dn4 = assign36590_e41156_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36590_e41156_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36590_e41156_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36590_e41156_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36590_e41156_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign36600_e41165, assign36600_e41165_d_n4, assign36600_e41165_d_n6, assign36600_e41165_d_n7, assign36600_e41165_d_n8, assign36600_e41165_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 == 0.0)) {
        let assign36600_e41163: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign36600_e41163, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign36600_e41165;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign36600_e41165_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign36600_e41165_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign36600_e41165_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign36600_e41165_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign36600_e41165_d_n9;
        locals.var_q_d1_lnexpnum__blk841_rv = 0.0;

        let (assign36610_e41175, assign36610_e41175_d_n4, assign36610_e41175_d_n6, assign36610_e41175_d_n7, assign36610_e41175_d_n8, assign36610_e41175_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 == 0.0)) {
        let assign36610_e41171: f64 = (-locals.var_q_temp1__blk814);
        let assign36610_e41173: f64 = (assign36610_e41171 * locals.var_q_temp1__blk814);
        (assign36610_e41173, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign36610_e41171 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign36610_e41171 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign36610_e41171 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign36610_e41171 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign36610_e41171 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign36610_e41175;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign36610_e41175_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign36610_e41175_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign36610_e41175_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign36610_e41175_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign36610_e41175_d_n9;
        locals.var_q_d2_lnexpnum__blk842_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_105(
        locals: &mut StampLocals,
    ) {
        let (assign36620_e41189, assign36620_e41189_d_n4, assign36620_e41189_d_n6, assign36620_e41189_d_n7, assign36620_e41189_d_n8, assign36620_e41189_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36620_e41179: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign36620_e41181: f64 = (assign36620_e41179 + locals.var_q1d__blk1001);
        let assign36620_e41184: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign36620_e41185: f64 = (assign36620_e41181 + assign36620_e41184);
        let assign36620_e41187: f64 = (assign36620_e41185 - locals.var_q_ln_term__blk834);
        (assign36620_e41187, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign36620_e41189;
        locals.var_q_q2_int__blk843_dn4 = assign36620_e41189_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign36620_e41189_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign36620_e41189_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign36620_e41189_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign36620_e41189_d_n9;
        locals.var_q_q2_int__blk843_rv = 0.0;

        let (assign36630_e41199, assign36630_e41199_d_n4, assign36630_e41199_d_n6, assign36630_e41199_d_n7, assign36630_e41199_d_n8, assign36630_e41199_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36630_e41194: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign36630_e41195: f64 = (1.0 + assign36630_e41194);
        let assign36630_e41197: f64 = (assign36630_e41195 - locals.var_q_d1_ln__blk835);
        (assign36630_e41197, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign36630_e41199;
        locals.var_q_d1_q2__blk844_dn4 = assign36630_e41199_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign36630_e41199_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign36630_e41199_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign36630_e41199_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign36630_e41199_d_n9;
        locals.var_q_d1_q2__blk844_rv = 0.0;

        let (assign36640_e41207, assign36640_e41207_d_n4, assign36640_e41207_d_n6, assign36640_e41207_d_n7, assign36640_e41207_d_n8, assign36640_e41207_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36640_e41203: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign36640_e41205: f64 = (assign36640_e41203 - locals.var_q_d2_ln__blk836);
        (assign36640_e41205, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign36640_e41207;
        locals.var_q_d2_q2__blk845_dn4 = assign36640_e41207_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign36640_e41207_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign36640_e41207_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign36640_e41207_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign36640_e41207_d_n9;
        locals.var_q_d2_q2__blk845_rv = 0.0;

        let (assign36650_e41215, assign36650_e41215_d_n4, assign36650_e41215_d_n6, assign36650_e41215_d_n7, assign36650_e41215_d_n8, assign36650_e41215_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36650_e41212: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign36650_e41213: f64 = (locals.var_q_k1q1__blk823 + assign36650_e41212);
        (assign36650_e41213, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign36650_e41215;
        locals.var_q_qi_int__blk846_dn4 = assign36650_e41215_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign36650_e41215_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign36650_e41215_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign36650_e41215_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign36650_e41215_d_n9;
        locals.var_q_qi_int__blk846_rv = 0.0;

        let (assign36660_e41223, assign36660_e41223_d_n4, assign36660_e41223_d_n6, assign36660_e41223_d_n7, assign36660_e41223_d_n8, assign36660_e41223_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36660_e41220: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign36660_e41221: f64 = (locals.var_k1__blk932 + assign36660_e41220);
        (assign36660_e41221, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign36660_e41223;
        locals.var_q_d1_qi__blk847_dn4 = assign36660_e41223_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign36660_e41223_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign36660_e41223_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign36660_e41223_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign36660_e41223_d_n9;
        locals.var_q_d1_qi__blk847_rv = 0.0;

        let (assign36670_e41229, assign36670_e41229_d_n4, assign36670_e41229_d_n6, assign36670_e41229_d_n7, assign36670_e41229_d_n8, assign36670_e41229_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36670_e41227: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign36670_e41227, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign36670_e41229;
        locals.var_q_d2_qi__blk848_dn4 = assign36670_e41229_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign36670_e41229_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign36670_e41229_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign36670_e41229_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign36670_e41229_d_n9;
        locals.var_q_d2_qi__blk848_rv = 0.0;

        let (assign36680_e41237, assign36680_e41237_d_n4, assign36680_e41237_d_n6, assign36680_e41237_d_n7, assign36680_e41237_d_n8, assign36680_e41237_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36680_e41233: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign36680_e41235: f64 = (assign36680_e41233 - locals.var_q_aexp__blk824);
        (assign36680_e41235, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign36680_e41237;
        locals.var_q_zero__blk849_dn4 = assign36680_e41237_d_n4;
        locals.var_q_zero__blk849_dn6 = assign36680_e41237_d_n6;
        locals.var_q_zero__blk849_dn7 = assign36680_e41237_d_n7;
        locals.var_q_zero__blk849_dn8 = assign36680_e41237_d_n8;
        locals.var_q_zero__blk849_dn9 = assign36680_e41237_d_n9;
        locals.var_q_zero__blk849_rv = 0.0;

        let (assign36690_e41249, assign36690_e41249_d_n4, assign36690_e41249_d_n6, assign36690_e41249_d_n7, assign36690_e41249_d_n8, assign36690_e41249_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36690_e41241: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign36690_e41244: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign36690_e41245: f64 = (assign36690_e41241 + assign36690_e41244);
        let assign36690_e41247: f64 = (assign36690_e41245 + locals.var_q_aexp__blk824);
        (assign36690_e41247, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign36690_e41249;
        locals.var_q_d1_zero__blk850_dn4 = assign36690_e41249_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign36690_e41249_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign36690_e41249_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign36690_e41249_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign36690_e41249_d_n9;
        locals.var_q_d1_zero__blk850_rv = 0.0;

        let (assign36700_e41267, assign36700_e41267_d_n4, assign36700_e41267_d_n6, assign36700_e41267_d_n7, assign36700_e41267_d_n8, assign36700_e41267_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36700_e41253: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign36700_e41256: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign36700_e41258: f64 = (assign36700_e41256 * locals.var_q_d1_expnum__blk838);
        let assign36700_e41259: f64 = (assign36700_e41253 + assign36700_e41258);
        let assign36700_e41262: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign36700_e41263: f64 = (assign36700_e41259 + assign36700_e41262);
        let assign36700_e41265: f64 = (assign36700_e41263 - locals.var_q_aexp__blk824);
        (assign36700_e41265, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign36700_e41256 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign36700_e41256 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign36700_e41256 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign36700_e41256 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign36700_e41256 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign36700_e41267;
        locals.var_q_d2_zero__blk851_dn4 = assign36700_e41267_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign36700_e41267_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign36700_e41267_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign36700_e41267_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign36700_e41267_d_n9;
        locals.var_q_d2_zero__blk851_rv = 0.0;

        let (assign36710_e41279, assign36710_e41279_d_n4, assign36710_e41279_d_n6, assign36710_e41279_d_n7, assign36710_e41279_d_n8, assign36710_e41279_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36710_e41271: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign36710_e41274: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign36710_e41276: f64 = (assign36710_e41274 * locals.var_q_d2_zero__blk851);
        let assign36710_e41277: f64 = (assign36710_e41271 - assign36710_e41276);
        (assign36710_e41277, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign36710_e41274 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign36710_e41274 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign36710_e41274 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign36710_e41274 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign36710_e41274 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign36710_e41279;
        locals.var_q_temp__blk860_dn4 = assign36710_e41279_d_n4;
        locals.var_q_temp__blk860_dn6 = assign36710_e41279_d_n6;
        locals.var_q_temp__blk860_dn7 = assign36710_e41279_d_n7;
        locals.var_q_temp__blk860_dn8 = assign36710_e41279_d_n8;
        locals.var_q_temp__blk860_dn9 = assign36710_e41279_d_n9;
        locals.var_q_temp__blk860_rv = 0.0;

        let (assign36720_e41294, assign36720_e41294_d_n4, assign36720_e41294_d_n6, assign36720_e41294_d_n7, assign36720_e41294_d_n8, assign36720_e41294_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36720_e41282: f64 = (-locals.var_q_zero__blk849);
        let assign36720_e41284: f64 = (assign36720_e41282 * locals.var_q_d1_zero__blk850);
        let assign36720_e41286: f64 = (assign36720_e41284 * locals.var_q_temp__blk860);
        let assign36720_e41289: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign36720_e41291: f64 = (assign36720_e41289 + 1e-200);
        let assign36720_e41292: f64 = (assign36720_e41286 / assign36720_e41291);
        (assign36720_e41292, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign36720_e41282 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign36720_e41284 * locals.var_q_temp__blk860_dn4)) * assign36720_e41291) - (assign36720_e41286 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign36720_e41291 * assign36720_e41291)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign36720_e41282 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign36720_e41284 * locals.var_q_temp__blk860_dn6)) * assign36720_e41291) - (assign36720_e41286 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign36720_e41291 * assign36720_e41291)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign36720_e41282 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign36720_e41284 * locals.var_q_temp__blk860_dn7)) * assign36720_e41291) - (assign36720_e41286 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign36720_e41291 * assign36720_e41291)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign36720_e41282 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign36720_e41284 * locals.var_q_temp__blk860_dn8)) * assign36720_e41291) - (assign36720_e41286 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign36720_e41291 * assign36720_e41291)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign36720_e41282 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign36720_e41284 * locals.var_q_temp__blk860_dn9)) * assign36720_e41291) - (assign36720_e41286 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign36720_e41291 * assign36720_e41291)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign36720_e41294;
        locals.var_q_eps2__blk852_dn4 = assign36720_e41294_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign36720_e41294_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign36720_e41294_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign36720_e41294_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign36720_e41294_d_n9;
        locals.var_q_eps2__blk852_rv = 0.0;

        let (assign36730_e41300, assign36730_e41300_d_n4, assign36730_e41300_d_n6, assign36730_e41300_d_n7, assign36730_e41300_d_n8, assign36730_e41300_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36730_e41298: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
        (assign36730_e41298, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign36730_e41300;
        locals.var_q1d__blk1001_dn4 = assign36730_e41300_d_n4;
        locals.var_q1d__blk1001_dn6 = assign36730_e41300_d_n6;
        locals.var_q1d__blk1001_dn7 = assign36730_e41300_d_n7;
        locals.var_q1d__blk1001_dn8 = assign36730_e41300_d_n8;
        locals.var_q1d__blk1001_dn9 = assign36730_e41300_d_n9;
        locals.var_q1d__blk1001_rv = 0.0;

        let (assign36740_e41306, assign36740_e41306_d_n4, assign36740_e41306_d_n6, assign36740_e41306_d_n7, assign36740_e41306_d_n8, assign36740_e41306_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36740_e41304: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign36740_e41304, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign36740_e41306;
        locals.var_q_k1q1__blk823_dn4 = assign36740_e41306_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign36740_e41306_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign36740_e41306_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign36740_e41306_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign36740_e41306_d_n9;
        locals.var_q_k1q1__blk823_rv = 0.0;

        let (assign36750_e41312, assign36750_e41312_d_n4, assign36750_e41312_d_n6, assign36750_e41312_d_n7, assign36750_e41312_d_n8, assign36750_e41312_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36750_e41310: f64 = (locals.var_k2__blk933 * locals.var_q2d__blk1002);
        (assign36750_e41310, ((locals.var_k2__blk933_dn4 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn9)),)
    } else {
        (locals.var_q_k2q2__blk853, locals.var_q_k2q2__blk853_dn4, locals.var_q_k2q2__blk853_dn6, locals.var_q_k2q2__blk853_dn7, locals.var_q_k2q2__blk853_dn8, locals.var_q_k2q2__blk853_dn9,)
    }
};
        locals.var_q_k2q2__blk853 = assign36750_e41312;
        locals.var_q_k2q2__blk853_dn4 = assign36750_e41312_d_n4;
        locals.var_q_k2q2__blk853_dn6 = assign36750_e41312_d_n6;
        locals.var_q_k2q2__blk853_dn7 = assign36750_e41312_d_n7;
        locals.var_q_k2q2__blk853_dn8 = assign36750_e41312_d_n8;
        locals.var_q_k2q2__blk853_dn9 = assign36750_e41312_d_n9;
        locals.var_q_k2q2__blk853_rv = 0.0;

        let (assign36760_e41318, assign36760_e41318_d_n4, assign36760_e41318_d_n6, assign36760_e41318_d_n7, assign36760_e41318_d_n8, assign36760_e41318_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36760_e41316: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_k2q2__blk853);
        (assign36760_e41316, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_k2q2__blk853_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_k2q2__blk853_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_k2q2__blk853_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_k2q2__blk853_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_k2q2__blk853_dn9),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign36760_e41318;
        locals.var_q_qi_int__blk846_dn4 = assign36760_e41318_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign36760_e41318_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign36760_e41318_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign36760_e41318_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign36760_e41318_d_n9;
        locals.var_q_qi_int__blk846_rv = 0.0;

        let (assign36770_e41326, assign36770_e41326_d_n4, assign36770_e41326_d_n6, assign36770_e41326_d_n7, assign36770_e41326_d_n8, assign36770_e41326_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36770_e41323: f64 = (0.065345483024 * locals.var_q_qi_int__blk846);
        let assign36770_e41324: f64 = (1.0 + assign36770_e41323);
        (assign36770_e41324, (0.065345483024 * locals.var_q_qi_int__blk846_dn4), (0.065345483024 * locals.var_q_qi_int__blk846_dn6), (0.065345483024 * locals.var_q_qi_int__blk846_dn7), (0.065345483024 * locals.var_q_qi_int__blk846_dn8), (0.065345483024 * locals.var_q_qi_int__blk846_dn9),)
    } else {
        (locals.var_q_a__blk854, locals.var_q_a__blk854_dn4, locals.var_q_a__blk854_dn6, locals.var_q_a__blk854_dn7, locals.var_q_a__blk854_dn8, locals.var_q_a__blk854_dn9,)
    }
};
        locals.var_q_a__blk854 = assign36770_e41326;
        locals.var_q_a__blk854_dn4 = assign36770_e41326_d_n4;
        locals.var_q_a__blk854_dn6 = assign36770_e41326_d_n6;
        locals.var_q_a__blk854_dn7 = assign36770_e41326_d_n7;
        locals.var_q_a__blk854_dn8 = assign36770_e41326_d_n8;
        locals.var_q_a__blk854_dn9 = assign36770_e41326_d_n9;
        locals.var_q_a__blk854_rv = 0.0;

        let (assign36780_e41338, assign36780_e41338_d_n4, assign36780_e41338_d_n6, assign36780_e41338_d_n7, assign36780_e41338_d_n8, assign36780_e41338_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36780_e41331: f64 = (8.5797362674 * locals.var_q_qi_int__blk846);
        let assign36780_e41332: f64 = (39.478417604 + assign36780_e41331);
        let assign36780_e41335: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign36780_e41336: f64 = (assign36780_e41332 + assign36780_e41335);
        (assign36780_e41336, ((8.5797362674 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9))),)
    } else {
        (locals.var_q_b__blk855, locals.var_q_b__blk855_dn4, locals.var_q_b__blk855_dn6, locals.var_q_b__blk855_dn7, locals.var_q_b__blk855_dn8, locals.var_q_b__blk855_dn9,)
    }
};
        locals.var_q_b__blk855 = assign36780_e41338;
        locals.var_q_b__blk855_dn4 = assign36780_e41338_d_n4;
        locals.var_q_b__blk855_dn6 = assign36780_e41338_d_n6;
        locals.var_q_b__blk855_dn7 = assign36780_e41338_d_n7;
        locals.var_q_b__blk855_dn8 = assign36780_e41338_d_n8;
        locals.var_q_b__blk855_dn9 = assign36780_e41338_d_n9;
        locals.var_q_b__blk855_rv = 0.0;

        let (assign36790_e41350, assign36790_e41350_d_n4, assign36790_e41350_d_n6, assign36790_e41350_d_n7, assign36790_e41350_d_n8, assign36790_e41350_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36790_e41343: f64 = (2.0 * locals.var_q_qi_int__blk846);
        let assign36790_e41346: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign36790_e41347: f64 = (assign36790_e41343 + assign36790_e41346);
        let assign36790_e41348: f64 = (39.478417604 * assign36790_e41347);
        (assign36790_e41348, (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9)))),)
    } else {
        (locals.var_q_c__blk856, locals.var_q_c__blk856_dn4, locals.var_q_c__blk856_dn6, locals.var_q_c__blk856_dn7, locals.var_q_c__blk856_dn8, locals.var_q_c__blk856_dn9,)
    }
};
        locals.var_q_c__blk856 = assign36790_e41350;
        locals.var_q_c__blk856_dn4 = assign36790_e41350_d_n4;
        locals.var_q_c__blk856_dn6 = assign36790_e41350_d_n6;
        locals.var_q_c__blk856_dn7 = assign36790_e41350_d_n7;
        locals.var_q_c__blk856_dn8 = assign36790_e41350_d_n8;
        locals.var_q_c__blk856_dn9 = assign36790_e41350_d_n9;
        locals.var_q_c__blk856_rv = 0.0;

        let (assign36800_e41363, assign36800_e41363_d_n4, assign36800_e41363_d_n6, assign36800_e41363_d_n7, assign36800_e41363_d_n8, assign36800_e41363_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36800_e41354: f64 = (locals.var_q_b__blk855 * locals.var_q_b__blk855);
        let assign36800_e41357: f64 = (4.0 * locals.var_q_a__blk854);
        let assign36800_e41359: f64 = (assign36800_e41357 * locals.var_q_c__blk856);
        let assign36800_e41360: f64 = (assign36800_e41354 - assign36800_e41359);
        let assign36800_e41361: f64 = (assign36800_e41360).sqrt();
        (assign36800_e41361, ((((locals.var_q_b__blk855_dn4 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn4)) - (((4.0 * locals.var_q_a__blk854_dn4) * locals.var_q_c__blk856) + (assign36800_e41357 * locals.var_q_c__blk856_dn4))) / (2.0 * assign36800_e41361)), ((((locals.var_q_b__blk855_dn6 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn6)) - (((4.0 * locals.var_q_a__blk854_dn6) * locals.var_q_c__blk856) + (assign36800_e41357 * locals.var_q_c__blk856_dn6))) / (2.0 * assign36800_e41361)), ((((locals.var_q_b__blk855_dn7 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn7)) - (((4.0 * locals.var_q_a__blk854_dn7) * locals.var_q_c__blk856) + (assign36800_e41357 * locals.var_q_c__blk856_dn7))) / (2.0 * assign36800_e41361)), ((((locals.var_q_b__blk855_dn8 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn8)) - (((4.0 * locals.var_q_a__blk854_dn8) * locals.var_q_c__blk856) + (assign36800_e41357 * locals.var_q_c__blk856_dn8))) / (2.0 * assign36800_e41361)), ((((locals.var_q_b__blk855_dn9 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn9)) - (((4.0 * locals.var_q_a__blk854_dn9) * locals.var_q_c__blk856) + (assign36800_e41357 * locals.var_q_c__blk856_dn9))) / (2.0 * assign36800_e41361)),)
    } else {
        (locals.var_q_disc__blk857, locals.var_q_disc__blk857_dn4, locals.var_q_disc__blk857_dn6, locals.var_q_disc__blk857_dn7, locals.var_q_disc__blk857_dn8, locals.var_q_disc__blk857_dn9,)
    }
};
        locals.var_q_disc__blk857 = assign36800_e41363;
        locals.var_q_disc__blk857_dn4 = assign36800_e41363_d_n4;
        locals.var_q_disc__blk857_dn6 = assign36800_e41363_d_n6;
        locals.var_q_disc__blk857_dn7 = assign36800_e41363_d_n7;
        locals.var_q_disc__blk857_dn8 = assign36800_e41363_d_n8;
        locals.var_q_disc__blk857_dn9 = assign36800_e41363_d_n9;
        locals.var_q_disc__blk857_rv = 0.0;

        let (assign36810_e41373, assign36810_e41373_d_n4, assign36810_e41373_d_n6, assign36810_e41373_d_n7, assign36810_e41373_d_n8, assign36810_e41373_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36810_e41367: f64 = (locals.var_q_disc__blk857 - locals.var_q_b__blk855);
        let assign36810_e41370: f64 = (2.0 * locals.var_q_a__blk854);
        let assign36810_e41371: f64 = (assign36810_e41367 / assign36810_e41370);
        (assign36810_e41371, ((((locals.var_q_disc__blk857_dn4 - locals.var_q_b__blk855_dn4) * assign36810_e41370) - (assign36810_e41367 * (2.0 * locals.var_q_a__blk854_dn4))) / (assign36810_e41370 * assign36810_e41370)), ((((locals.var_q_disc__blk857_dn6 - locals.var_q_b__blk855_dn6) * assign36810_e41370) - (assign36810_e41367 * (2.0 * locals.var_q_a__blk854_dn6))) / (assign36810_e41370 * assign36810_e41370)), ((((locals.var_q_disc__blk857_dn7 - locals.var_q_b__blk855_dn7) * assign36810_e41370) - (assign36810_e41367 * (2.0 * locals.var_q_a__blk854_dn7))) / (assign36810_e41370 * assign36810_e41370)), ((((locals.var_q_disc__blk857_dn8 - locals.var_q_b__blk855_dn8) * assign36810_e41370) - (assign36810_e41367 * (2.0 * locals.var_q_a__blk854_dn8))) / (assign36810_e41370 * assign36810_e41370)), ((((locals.var_q_disc__blk857_dn9 - locals.var_q_b__blk855_dn9) * assign36810_e41370) - (assign36810_e41367 * (2.0 * locals.var_q_a__blk854_dn9))) / (assign36810_e41370 * assign36810_e41370)),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign36810_e41373;
        locals.var_q_qsq__blk825_dn4 = assign36810_e41373_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign36810_e41373_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign36810_e41373_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign36810_e41373_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign36810_e41373_d_n9;
        locals.var_q_qsq__blk825_rv = 0.0;

        let (assign36820_e41381, assign36820_e41381_d_n4, assign36820_e41381_d_n6, assign36820_e41381_d_n7, assign36820_e41381_d_n8, assign36820_e41381_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36820_e41377: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign36820_e41379: f64 = (assign36820_e41377 - locals.var_q_qsq__blk825);
        (assign36820_e41379, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_qsq__blk825_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_qsq__blk825_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_qsq__blk825_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_qsq__blk825_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_qsq__blk825_dn9),)
    } else {
        (locals.var_q_delta__blk858, locals.var_q_delta__blk858_dn4, locals.var_q_delta__blk858_dn6, locals.var_q_delta__blk858_dn7, locals.var_q_delta__blk858_dn8, locals.var_q_delta__blk858_dn9,)
    }
};
        locals.var_q_delta__blk858 = assign36820_e41381;
        locals.var_q_delta__blk858_dn4 = assign36820_e41381_d_n4;
        locals.var_q_delta__blk858_dn6 = assign36820_e41381_d_n6;
        locals.var_q_delta__blk858_dn7 = assign36820_e41381_d_n7;
        locals.var_q_delta__blk858_dn8 = assign36820_e41381_d_n8;
        locals.var_q_delta__blk858_dn9 = assign36820_e41381_d_n9;
        locals.var_q_delta__blk858_rv = 0.0;

        let assign36830_e41384: f64 = if locals.var_q_delta__blk858 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1176 = assign36830_e41384;
        locals.var_guard1176_rv = 0.0;

        let (assign36840_e41401, assign36840_e41401_d_n4, assign36840_e41401_d_n6, assign36840_e41401_d_n7, assign36840_e41401_d_n8, assign36840_e41401_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1176 != 0.0)) {
        let assign36840_e41391: f64 = (locals.var_q_delta__blk858 / locals.var_a0__blk905);
        let assign36840_e41392: f64 = (assign36840_e41391).ln();
        let assign36840_e41394: f64 = (assign36840_e41392 + locals.var_xdeff__blk1000);
        let assign36840_e41396: f64 = (assign36840_e41394 - locals.var_xg1x__blk930);
        let assign36840_e41398: f64 = (assign36840_e41396 + locals.var_q1d__blk1001);
        let assign36840_e41399: f64 = (locals.var_q_delta__blk858 * assign36840_e41398);
        (assign36840_e41399, ((locals.var_q_delta__blk858_dn4 * assign36840_e41398) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn4 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign36840_e41391) + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4))), ((locals.var_q_delta__blk858_dn6 * assign36840_e41398) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn6 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign36840_e41391) + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6))), ((locals.var_q_delta__blk858_dn7 * assign36840_e41398) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn7 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign36840_e41391) + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7))), ((locals.var_q_delta__blk858_dn8 * assign36840_e41398) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn8 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign36840_e41391) + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8))), ((locals.var_q_delta__blk858_dn9 * assign36840_e41398) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn9 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign36840_e41391) + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9))),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign36840_e41401;
        locals.var_q_zero__blk849_dn4 = assign36840_e41401_d_n4;
        locals.var_q_zero__blk849_dn6 = assign36840_e41401_d_n6;
        locals.var_q_zero__blk849_dn7 = assign36840_e41401_d_n7;
        locals.var_q_zero__blk849_dn8 = assign36840_e41401_d_n8;
        locals.var_q_zero__blk849_dn9 = assign36840_e41401_d_n9;
        locals.var_q_zero__blk849_rv = 0.0;

        let (assign36850_e41413, assign36850_e41413_d_n4, assign36850_e41413_d_n6, assign36850_e41413_d_n7, assign36850_e41413_d_n8, assign36850_e41413_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1176 != 0.0)) {
        let assign36850_e41407: f64 = (2.0 * locals.var_k1__blk932);
        let assign36850_e41409: f64 = (assign36850_e41407 * locals.var_q_k1q1__blk823);
        let assign36850_e41411: f64 = (assign36850_e41409 + locals.var_q_delta__blk858);
        (assign36850_e41411, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign36850_e41407 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_delta__blk858_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign36850_e41407 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_delta__blk858_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign36850_e41407 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_delta__blk858_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign36850_e41407 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_delta__blk858_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign36850_e41407 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_delta__blk858_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign36850_e41413;
        locals.var_q_d1_zero__blk850_dn4 = assign36850_e41413_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign36850_e41413_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign36850_e41413_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign36850_e41413_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign36850_e41413_d_n9;
        locals.var_q_d1_zero__blk850_rv = 0.0;

        let (assign36860_e41423, assign36860_e41423_d_n4, assign36860_e41423_d_n6, assign36860_e41423_d_n7, assign36860_e41423_d_n8, assign36860_e41423_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1176 != 0.0)) {
        let assign36860_e41419: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36860_e41421: f64 = (assign36860_e41419 - locals.var_q_x1sat__blk817);
        (assign36860_e41421, ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_q_x1sat__blk817_dn4), ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_q_x1sat__blk817_dn6), ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_q_x1sat__blk817_dn7), ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_q_x1sat__blk817_dn8), ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_q_x1sat__blk817_dn9),)
    } else {
        (locals.var_q_dx1__blk859, locals.var_q_dx1__blk859_dn4, locals.var_q_dx1__blk859_dn6, locals.var_q_dx1__blk859_dn7, locals.var_q_dx1__blk859_dn8, locals.var_q_dx1__blk859_dn9,)
    }
};
        locals.var_q_dx1__blk859 = assign36860_e41423;
        locals.var_q_dx1__blk859_dn4 = assign36860_e41423_d_n4;
        locals.var_q_dx1__blk859_dn6 = assign36860_e41423_d_n6;
        locals.var_q_dx1__blk859_dn7 = assign36860_e41423_d_n7;
        locals.var_q_dx1__blk859_dn8 = assign36860_e41423_d_n8;
        locals.var_q_dx1__blk859_dn9 = assign36860_e41423_d_n9;
        locals.var_q_dx1__blk859_rv = 0.0;

        let assign36870_e41433: f64 = (locals.var_q_dx1__blk859 + 2.3025850929941);
        let assign36870_e41435: f64 = (locals.var_k1__blk932).ln();
        let assign36870_e41436: f64 = (assign36870_e41433 + assign36870_e41435);
        let assign36870_e41443: f64 = if ((((locals.var_q_zero__blk849 < 0.0) && (locals.var_q_d1_zero__blk850 > 0.0)) && (assign36870_e41436 > 0.0)) || (locals.var_q_dx1__blk859 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign36870_e41443;
        locals.var_guard1177_rv = 0.0;

        let (assign36880_e41455, assign36880_e41455_d_n4, assign36880_e41455_d_n6, assign36880_e41455_d_n7, assign36880_e41455_d_n8, assign36880_e41455_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1176 != 0.0)) && (locals.var_guard1177 != 0.0)) {
        let assign36880_e41452: f64 = (locals.var_q_zero__blk849 / locals.var_q_d1_zero__blk850);
        let assign36880_e41453: f64 = (locals.var_q1d__blk1001 - assign36880_e41452);
        (assign36880_e41453, (locals.var_q1d__blk1001_dn4 - (((locals.var_q_zero__blk849_dn4 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn4)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn6 - (((locals.var_q_zero__blk849_dn6 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn6)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn7 - (((locals.var_q_zero__blk849_dn7 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn7)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn8 - (((locals.var_q_zero__blk849_dn8 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn8)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn9 - (((locals.var_q_zero__blk849_dn9 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn9)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign36880_e41455;
        locals.var_q1d__blk1001_dn4 = assign36880_e41455_d_n4;
        locals.var_q1d__blk1001_dn6 = assign36880_e41455_d_n6;
        locals.var_q1d__blk1001_dn7 = assign36880_e41455_d_n7;
        locals.var_q1d__blk1001_dn8 = assign36880_e41455_d_n8;
        locals.var_q1d__blk1001_dn9 = assign36880_e41455_d_n9;
        locals.var_q1d__blk1001_rv = 0.0;

        let (assign36890_e41461, assign36890_e41461_d_n4, assign36890_e41461_d_n6, assign36890_e41461_d_n7, assign36890_e41461_d_n8, assign36890_e41461_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36890_e41459: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign36890_e41459, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign36890_e41461;
        locals.var_q_k1q1__blk823_dn4 = assign36890_e41461_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign36890_e41461_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign36890_e41461_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign36890_e41461_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign36890_e41461_d_n9;
        locals.var_q_k1q1__blk823_rv = 0.0;

        let (assign36900_e41467, assign36900_e41467_d_n4, assign36900_e41467_d_n6, assign36900_e41467_d_n7, assign36900_e41467_d_n8, assign36900_e41467_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36900_e41465: f64 = (locals.var_k2__blk933 * locals.var_q2d__blk1002);
        (assign36900_e41465, ((locals.var_k2__blk933_dn4 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn9)),)
    } else {
        (locals.var_q_k2q2__blk853, locals.var_q_k2q2__blk853_dn4, locals.var_q_k2q2__blk853_dn6, locals.var_q_k2q2__blk853_dn7, locals.var_q_k2q2__blk853_dn8, locals.var_q_k2q2__blk853_dn9,)
    }
};
        locals.var_q_k2q2__blk853 = assign36900_e41467;
        locals.var_q_k2q2__blk853_dn4 = assign36900_e41467_d_n4;
        locals.var_q_k2q2__blk853_dn6 = assign36900_e41467_d_n6;
        locals.var_q_k2q2__blk853_dn7 = assign36900_e41467_d_n7;
        locals.var_q_k2q2__blk853_dn8 = assign36900_e41467_d_n8;
        locals.var_q_k2q2__blk853_dn9 = assign36900_e41467_d_n9;
        locals.var_q_k2q2__blk853_rv = 0.0;

        let (assign36910_e41473, assign36910_e41473_d_n4, assign36910_e41473_d_n6, assign36910_e41473_d_n7, assign36910_e41473_d_n8, assign36910_e41473_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36910_e41471: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_k2q2__blk853);
        (assign36910_e41471, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_k2q2__blk853_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_k2q2__blk853_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_k2q2__blk853_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_k2q2__blk853_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_k2q2__blk853_dn9),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign36910_e41473;
        locals.var_q_qi_int__blk846_dn4 = assign36910_e41473_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign36910_e41473_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign36910_e41473_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign36910_e41473_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign36910_e41473_d_n9;
        locals.var_q_qi_int__blk846_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_106(
        locals: &mut StampLocals,
    ) {
        let (assign36920_e41481, assign36920_e41481_d_n4, assign36920_e41481_d_n6, assign36920_e41481_d_n7, assign36920_e41481_d_n8, assign36920_e41481_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36920_e41478: f64 = (0.065345483024 * locals.var_q_qi_int__blk846);
        let assign36920_e41479: f64 = (1.0 + assign36920_e41478);
        (assign36920_e41479, (0.065345483024 * locals.var_q_qi_int__blk846_dn4), (0.065345483024 * locals.var_q_qi_int__blk846_dn6), (0.065345483024 * locals.var_q_qi_int__blk846_dn7), (0.065345483024 * locals.var_q_qi_int__blk846_dn8), (0.065345483024 * locals.var_q_qi_int__blk846_dn9),)
    } else {
        (locals.var_q_a__blk854, locals.var_q_a__blk854_dn4, locals.var_q_a__blk854_dn6, locals.var_q_a__blk854_dn7, locals.var_q_a__blk854_dn8, locals.var_q_a__blk854_dn9,)
    }
};
        locals.var_q_a__blk854 = assign36920_e41481;
        locals.var_q_a__blk854_dn4 = assign36920_e41481_d_n4;
        locals.var_q_a__blk854_dn6 = assign36920_e41481_d_n6;
        locals.var_q_a__blk854_dn7 = assign36920_e41481_d_n7;
        locals.var_q_a__blk854_dn8 = assign36920_e41481_d_n8;
        locals.var_q_a__blk854_dn9 = assign36920_e41481_d_n9;
        locals.var_q_a__blk854_rv = 0.0;

        let (assign36930_e41493, assign36930_e41493_d_n4, assign36930_e41493_d_n6, assign36930_e41493_d_n7, assign36930_e41493_d_n8, assign36930_e41493_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36930_e41486: f64 = (8.5797362674 * locals.var_q_qi_int__blk846);
        let assign36930_e41487: f64 = (39.478417604 + assign36930_e41486);
        let assign36930_e41490: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign36930_e41491: f64 = (assign36930_e41487 + assign36930_e41490);
        (assign36930_e41491, ((8.5797362674 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9))),)
    } else {
        (locals.var_q_b__blk855, locals.var_q_b__blk855_dn4, locals.var_q_b__blk855_dn6, locals.var_q_b__blk855_dn7, locals.var_q_b__blk855_dn8, locals.var_q_b__blk855_dn9,)
    }
};
        locals.var_q_b__blk855 = assign36930_e41493;
        locals.var_q_b__blk855_dn4 = assign36930_e41493_d_n4;
        locals.var_q_b__blk855_dn6 = assign36930_e41493_d_n6;
        locals.var_q_b__blk855_dn7 = assign36930_e41493_d_n7;
        locals.var_q_b__blk855_dn8 = assign36930_e41493_d_n8;
        locals.var_q_b__blk855_dn9 = assign36930_e41493_d_n9;
        locals.var_q_b__blk855_rv = 0.0;

        let (assign36940_e41505, assign36940_e41505_d_n4, assign36940_e41505_d_n6, assign36940_e41505_d_n7, assign36940_e41505_d_n8, assign36940_e41505_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36940_e41498: f64 = (2.0 * locals.var_q_qi_int__blk846);
        let assign36940_e41501: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign36940_e41502: f64 = (assign36940_e41498 + assign36940_e41501);
        let assign36940_e41503: f64 = (39.478417604 * assign36940_e41502);
        (assign36940_e41503, (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9)))),)
    } else {
        (locals.var_q_c__blk856, locals.var_q_c__blk856_dn4, locals.var_q_c__blk856_dn6, locals.var_q_c__blk856_dn7, locals.var_q_c__blk856_dn8, locals.var_q_c__blk856_dn9,)
    }
};
        locals.var_q_c__blk856 = assign36940_e41505;
        locals.var_q_c__blk856_dn4 = assign36940_e41505_d_n4;
        locals.var_q_c__blk856_dn6 = assign36940_e41505_d_n6;
        locals.var_q_c__blk856_dn7 = assign36940_e41505_d_n7;
        locals.var_q_c__blk856_dn8 = assign36940_e41505_d_n8;
        locals.var_q_c__blk856_dn9 = assign36940_e41505_d_n9;
        locals.var_q_c__blk856_rv = 0.0;

        let (assign36950_e41518, assign36950_e41518_d_n4, assign36950_e41518_d_n6, assign36950_e41518_d_n7, assign36950_e41518_d_n8, assign36950_e41518_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36950_e41509: f64 = (locals.var_q_b__blk855 * locals.var_q_b__blk855);
        let assign36950_e41512: f64 = (4.0 * locals.var_q_a__blk854);
        let assign36950_e41514: f64 = (assign36950_e41512 * locals.var_q_c__blk856);
        let assign36950_e41515: f64 = (assign36950_e41509 - assign36950_e41514);
        let assign36950_e41516: f64 = (assign36950_e41515).sqrt();
        (assign36950_e41516, ((((locals.var_q_b__blk855_dn4 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn4)) - (((4.0 * locals.var_q_a__blk854_dn4) * locals.var_q_c__blk856) + (assign36950_e41512 * locals.var_q_c__blk856_dn4))) / (2.0 * assign36950_e41516)), ((((locals.var_q_b__blk855_dn6 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn6)) - (((4.0 * locals.var_q_a__blk854_dn6) * locals.var_q_c__blk856) + (assign36950_e41512 * locals.var_q_c__blk856_dn6))) / (2.0 * assign36950_e41516)), ((((locals.var_q_b__blk855_dn7 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn7)) - (((4.0 * locals.var_q_a__blk854_dn7) * locals.var_q_c__blk856) + (assign36950_e41512 * locals.var_q_c__blk856_dn7))) / (2.0 * assign36950_e41516)), ((((locals.var_q_b__blk855_dn8 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn8)) - (((4.0 * locals.var_q_a__blk854_dn8) * locals.var_q_c__blk856) + (assign36950_e41512 * locals.var_q_c__blk856_dn8))) / (2.0 * assign36950_e41516)), ((((locals.var_q_b__blk855_dn9 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn9)) - (((4.0 * locals.var_q_a__blk854_dn9) * locals.var_q_c__blk856) + (assign36950_e41512 * locals.var_q_c__blk856_dn9))) / (2.0 * assign36950_e41516)),)
    } else {
        (locals.var_q_disc__blk857, locals.var_q_disc__blk857_dn4, locals.var_q_disc__blk857_dn6, locals.var_q_disc__blk857_dn7, locals.var_q_disc__blk857_dn8, locals.var_q_disc__blk857_dn9,)
    }
};
        locals.var_q_disc__blk857 = assign36950_e41518;
        locals.var_q_disc__blk857_dn4 = assign36950_e41518_d_n4;
        locals.var_q_disc__blk857_dn6 = assign36950_e41518_d_n6;
        locals.var_q_disc__blk857_dn7 = assign36950_e41518_d_n7;
        locals.var_q_disc__blk857_dn8 = assign36950_e41518_d_n8;
        locals.var_q_disc__blk857_dn9 = assign36950_e41518_d_n9;
        locals.var_q_disc__blk857_rv = 0.0;

        let (assign36960_e41528, assign36960_e41528_d_n4, assign36960_e41528_d_n6, assign36960_e41528_d_n7, assign36960_e41528_d_n8, assign36960_e41528_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36960_e41522: f64 = (locals.var_q_disc__blk857 - locals.var_q_b__blk855);
        let assign36960_e41525: f64 = (2.0 * locals.var_q_a__blk854);
        let assign36960_e41526: f64 = (assign36960_e41522 / assign36960_e41525);
        (assign36960_e41526, ((((locals.var_q_disc__blk857_dn4 - locals.var_q_b__blk855_dn4) * assign36960_e41525) - (assign36960_e41522 * (2.0 * locals.var_q_a__blk854_dn4))) / (assign36960_e41525 * assign36960_e41525)), ((((locals.var_q_disc__blk857_dn6 - locals.var_q_b__blk855_dn6) * assign36960_e41525) - (assign36960_e41522 * (2.0 * locals.var_q_a__blk854_dn6))) / (assign36960_e41525 * assign36960_e41525)), ((((locals.var_q_disc__blk857_dn7 - locals.var_q_b__blk855_dn7) * assign36960_e41525) - (assign36960_e41522 * (2.0 * locals.var_q_a__blk854_dn7))) / (assign36960_e41525 * assign36960_e41525)), ((((locals.var_q_disc__blk857_dn8 - locals.var_q_b__blk855_dn8) * assign36960_e41525) - (assign36960_e41522 * (2.0 * locals.var_q_a__blk854_dn8))) / (assign36960_e41525 * assign36960_e41525)), ((((locals.var_q_disc__blk857_dn9 - locals.var_q_b__blk855_dn9) * assign36960_e41525) - (assign36960_e41522 * (2.0 * locals.var_q_a__blk854_dn9))) / (assign36960_e41525 * assign36960_e41525)),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign36960_e41528;
        locals.var_q_qsq__blk825_dn4 = assign36960_e41528_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign36960_e41528_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign36960_e41528_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign36960_e41528_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign36960_e41528_d_n9;
        locals.var_q_qsq__blk825_rv = 0.0;

        let assign36970_e41531: f64 = (-0.005);
        let assign36970_e41532: f64 = if locals.var_q_qsq__blk825 < assign36970_e41531 { 1.0 } else { 0.0 };
        locals.var_guard1178 = assign36970_e41532;
        locals.var_guard1178_rv = 0.0;

        let (assign36980_e41540, assign36980_e41540_d_n4, assign36980_e41540_d_n6, assign36980_e41540_d_n7, assign36980_e41540_d_n8, assign36980_e41540_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1178 != 0.0)) {
        let assign36980_e41537: f64 = (locals.var_q_qsq__blk825).abs();
        let assign36980_e41538: f64 = (assign36980_e41537).sqrt();
        (assign36980_e41538, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign36980_e41538)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign36980_e41538)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign36980_e41538)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign36980_e41538)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign36980_e41538)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign36980_e41540;
        locals.var_q_rac_qsq__blk828_dn4 = assign36980_e41540_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign36980_e41540_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign36980_e41540_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign36980_e41540_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign36980_e41540_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign36990_e41551, assign36990_e41551_d_n4, assign36990_e41551_d_n6, assign36990_e41551_d_n7, assign36990_e41551_d_n8, assign36990_e41551_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1178 != 0.0)) {
        let assign36990_e41547: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign36990_e41548: f64 = (assign36990_e41547).tan();
        let assign36990_e41549: f64 = (locals.var_q_rac_qsq__blk828 / assign36990_e41548);
        (assign36990_e41549, (((locals.var_q_rac_qsq__blk828_dn4 * assign36990_e41548) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign36990_e41547).cos() * (assign36990_e41547).cos())))) / (assign36990_e41548 * assign36990_e41548)), (((locals.var_q_rac_qsq__blk828_dn6 * assign36990_e41548) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign36990_e41547).cos() * (assign36990_e41547).cos())))) / (assign36990_e41548 * assign36990_e41548)), (((locals.var_q_rac_qsq__blk828_dn7 * assign36990_e41548) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign36990_e41547).cos() * (assign36990_e41547).cos())))) / (assign36990_e41548 * assign36990_e41548)), (((locals.var_q_rac_qsq__blk828_dn8 * assign36990_e41548) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign36990_e41547).cos() * (assign36990_e41547).cos())))) / (assign36990_e41548 * assign36990_e41548)), (((locals.var_q_rac_qsq__blk828_dn9 * assign36990_e41548) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign36990_e41547).cos() * (assign36990_e41547).cos())))) / (assign36990_e41548 * assign36990_e41548)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign36990_e41551;
        locals.var_q_qcoth__blk829_dn4 = assign36990_e41551_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign36990_e41551_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign36990_e41551_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign36990_e41551_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign36990_e41551_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign37000_e41567, assign37000_e41567_d_n4, assign37000_e41567_d_n6, assign37000_e41567_d_n7, assign37000_e41567_d_n8, assign37000_e41567_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1178 != 0.0)) {
        let assign37000_e41560: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign37000_e41561: f64 = (locals.var_q_qcoth__blk829 * assign37000_e41560);
        let assign37000_e41562: f64 = (locals.var_q_qsq__blk825 + assign37000_e41561);
        let assign37000_e41563: f64 = (0.25 * assign37000_e41562);
        let assign37000_e41565: f64 = (assign37000_e41563 / locals.var_q_qsq__blk825);
        (assign37000_e41565, ((((0.25 * (locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign37000_e41560) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4))))) * locals.var_q_qsq__blk825) - (assign37000_e41563 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign37000_e41560) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6))))) * locals.var_q_qsq__blk825) - (assign37000_e41563 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign37000_e41560) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7))))) * locals.var_q_qsq__blk825) - (assign37000_e41563 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign37000_e41560) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8))))) * locals.var_q_qsq__blk825) - (assign37000_e41563 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign37000_e41560) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9))))) * locals.var_q_qsq__blk825) - (assign37000_e41563 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37000_e41567;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37000_e41567_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37000_e41567_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37000_e41567_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37000_e41567_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37000_e41567_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let assign37010_e41570: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1179 = assign37010_e41570;
        locals.var_guard1179_rv = 0.0;

        let (assign37020_e41581, assign37020_e41581_d_n4, assign37020_e41581_d_n6, assign37020_e41581_d_n7, assign37020_e41581_d_n8, assign37020_e41581_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 != 0.0)) {
        let assign37020_e41578: f64 = (locals.var_q_qsq__blk825).abs();
        let assign37020_e41579: f64 = (assign37020_e41578).sqrt();
        (assign37020_e41579, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign37020_e41579)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign37020_e41579)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign37020_e41579)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign37020_e41579)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign37020_e41579)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign37020_e41581;
        locals.var_q_rac_qsq__blk828_dn4 = assign37020_e41581_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign37020_e41581_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign37020_e41581_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign37020_e41581_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign37020_e41581_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign37030_e41592, assign37030_e41592_d_n4, assign37030_e41592_d_n6, assign37030_e41592_d_n7, assign37030_e41592_d_n8, assign37030_e41592_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 != 0.0)) {
        let assign37030_e41589: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign37030_e41590: f64 = (assign37030_e41589).exp();
        (assign37030_e41590, (assign37030_e41590 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign37030_e41590 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign37030_e41590 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign37030_e41590 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign37030_e41590 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign37030_e41592;
        locals.var_q_invexpq__blk831_dn4 = assign37030_e41592_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign37030_e41592_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign37030_e41592_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign37030_e41592_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign37030_e41592_d_n9;
        locals.var_q_invexpq__blk831_rv = 0.0;

        let (assign37040_e41609, assign37040_e41609_d_n4, assign37040_e41609_d_n6, assign37040_e41609_d_n7, assign37040_e41609_d_n8, assign37040_e41609_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 != 0.0)) {
        let assign37040_e41602: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign37040_e41603: f64 = (locals.var_q_rac_qsq__blk828 * assign37040_e41602);
        let assign37040_e41606: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign37040_e41607: f64 = (assign37040_e41603 / assign37040_e41606);
        (assign37040_e41607, (((((locals.var_q_rac_qsq__blk828_dn4 * assign37040_e41602) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign37040_e41606) - (assign37040_e41603 * (-locals.var_q_invexpq__blk831_dn4))) / (assign37040_e41606 * assign37040_e41606)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign37040_e41602) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign37040_e41606) - (assign37040_e41603 * (-locals.var_q_invexpq__blk831_dn6))) / (assign37040_e41606 * assign37040_e41606)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign37040_e41602) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign37040_e41606) - (assign37040_e41603 * (-locals.var_q_invexpq__blk831_dn7))) / (assign37040_e41606 * assign37040_e41606)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign37040_e41602) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign37040_e41606) - (assign37040_e41603 * (-locals.var_q_invexpq__blk831_dn8))) / (assign37040_e41606 * assign37040_e41606)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign37040_e41602) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign37040_e41606) - (assign37040_e41603 * (-locals.var_q_invexpq__blk831_dn9))) / (assign37040_e41606 * assign37040_e41606)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37040_e41609;
        locals.var_q_qcoth__blk829_dn4 = assign37040_e41609_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37040_e41609_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37040_e41609_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37040_e41609_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37040_e41609_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign37050_e41628, assign37050_e41628_d_n4, assign37050_e41628_d_n6, assign37050_e41628_d_n7, assign37050_e41628_d_n8, assign37050_e41628_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 != 0.0)) {
        let assign37050_e41621: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign37050_e41622: f64 = (locals.var_q_qcoth__blk829 * assign37050_e41621);
        let assign37050_e41623: f64 = (locals.var_q_qsq__blk825 + assign37050_e41622);
        let assign37050_e41624: f64 = (0.25 * assign37050_e41623);
        let assign37050_e41626: f64 = (assign37050_e41624 / locals.var_q_qsq__blk825);
        (assign37050_e41626, ((((0.25 * (locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign37050_e41621) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4))))) * locals.var_q_qsq__blk825) - (assign37050_e41624 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign37050_e41621) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6))))) * locals.var_q_qsq__blk825) - (assign37050_e41624 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign37050_e41621) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7))))) * locals.var_q_qsq__blk825) - (assign37050_e41624 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign37050_e41621) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8))))) * locals.var_q_qsq__blk825) - (assign37050_e41624 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign37050_e41621) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9))))) * locals.var_q_qsq__blk825) - (assign37050_e41624 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37050_e41628;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37050_e41628_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37050_e41628_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37050_e41628_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37050_e41628_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37050_e41628_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign37060_e41654, assign37060_e41654_d_n4, assign37060_e41654_d_n6, assign37060_e41654_d_n7, assign37060_e41654_d_n8, assign37060_e41654_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 == 0.0)) {
        let assign37060_e41639: f64 = (locals.var_q_qsq__blk825 * 0.1666666666667);
        let assign37060_e41643: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign37060_e41647: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign37060_e41648: f64 = (1.0 - assign37060_e41647);
        let assign37060_e41649: f64 = (assign37060_e41643 * assign37060_e41648);
        let assign37060_e41650: f64 = (1.0 - assign37060_e41649);
        let assign37060_e41651: f64 = (assign37060_e41639 * assign37060_e41650);
        let assign37060_e41652: f64 = (2.0 + assign37060_e41651);
        (assign37060_e41652, (((locals.var_q_qsq__blk825_dn4 * 0.1666666666667) * assign37060_e41650) + (assign37060_e41639 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign37060_e41648) + (assign37060_e41643 * (-(locals.var_q_qsq__blk825_dn4 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn6 * 0.1666666666667) * assign37060_e41650) + (assign37060_e41639 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign37060_e41648) + (assign37060_e41643 * (-(locals.var_q_qsq__blk825_dn6 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn7 * 0.1666666666667) * assign37060_e41650) + (assign37060_e41639 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign37060_e41648) + (assign37060_e41643 * (-(locals.var_q_qsq__blk825_dn7 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn8 * 0.1666666666667) * assign37060_e41650) + (assign37060_e41639 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign37060_e41648) + (assign37060_e41643 * (-(locals.var_q_qsq__blk825_dn8 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn9 * 0.1666666666667) * assign37060_e41650) + (assign37060_e41639 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign37060_e41648) + (assign37060_e41643 * (-(locals.var_q_qsq__blk825_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37060_e41654;
        locals.var_q_qcoth__blk829_dn4 = assign37060_e41654_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37060_e41654_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37060_e41654_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37060_e41654_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37060_e41654_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign37070_e41682, assign37070_e41682_d_n4, assign37070_e41682_d_n6, assign37070_e41682_d_n7, assign37070_e41682_d_n8, assign37070_e41682_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 == 0.0)) {
        let assign37070_e41666: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign37070_e41670: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign37070_e41674: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign37070_e41675: f64 = (1.0 - assign37070_e41674);
        let assign37070_e41676: f64 = (assign37070_e41670 * assign37070_e41675);
        let assign37070_e41677: f64 = (1.0 - assign37070_e41676);
        let assign37070_e41678: f64 = (assign37070_e41666 * assign37070_e41677);
        let assign37070_e41679: f64 = (1.0 - assign37070_e41678);
        let assign37070_e41680: f64 = (0.1666666666667 * assign37070_e41679);
        (assign37070_e41680, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign37070_e41677) + (assign37070_e41666 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign37070_e41675) + (assign37070_e41670 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign37070_e41677) + (assign37070_e41666 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign37070_e41675) + (assign37070_e41670 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign37070_e41677) + (assign37070_e41666 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign37070_e41675) + (assign37070_e41670 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign37070_e41677) + (assign37070_e41666 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign37070_e41675) + (assign37070_e41670 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign37070_e41677) + (assign37070_e41666 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign37070_e41675) + (assign37070_e41670 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37070_e41682;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37070_e41682_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37070_e41682_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37070_e41682_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37070_e41682_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37070_e41682_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign37080_e41702, assign37080_e41702_d_n4, assign37080_e41702_d_n6, assign37080_e41702_d_n7, assign37080_e41702_d_n8, assign37080_e41702_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37080_e41687: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829);
        let assign37080_e41690: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign37080_e41691: f64 = (assign37080_e41687 + assign37080_e41690);
        let assign37080_e41693: f64 = (assign37080_e41691 + locals.var_q_qsq__blk825);
        let assign37080_e41696: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830);
        let assign37080_e41698: f64 = (assign37080_e41696 + 1.0);
        let assign37080_e41699: f64 = (assign37080_e41693 / assign37080_e41698);
        let assign37080_e41700: f64 = (locals.var_q_qsq__blk825 - assign37080_e41699);
        (assign37080_e41700, (locals.var_q_qsq__blk825_dn4 - (((((((locals.var_q_qi_int__blk846_dn4 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn4)) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4))) + locals.var_q_qsq__blk825_dn4) * assign37080_e41698) - (assign37080_e41693 * ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn4)))) / (assign37080_e41698 * assign37080_e41698))), (locals.var_q_qsq__blk825_dn6 - (((((((locals.var_q_qi_int__blk846_dn6 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn6)) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6))) + locals.var_q_qsq__blk825_dn6) * assign37080_e41698) - (assign37080_e41693 * ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn6)))) / (assign37080_e41698 * assign37080_e41698))), (locals.var_q_qsq__blk825_dn7 - (((((((locals.var_q_qi_int__blk846_dn7 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn7)) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7))) + locals.var_q_qsq__blk825_dn7) * assign37080_e41698) - (assign37080_e41693 * ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn7)))) / (assign37080_e41698 * assign37080_e41698))), (locals.var_q_qsq__blk825_dn8 - (((((((locals.var_q_qi_int__blk846_dn8 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn8)) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8))) + locals.var_q_qsq__blk825_dn8) * assign37080_e41698) - (assign37080_e41693 * ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn8)))) / (assign37080_e41698 * assign37080_e41698))), (locals.var_q_qsq__blk825_dn9 - (((((((locals.var_q_qi_int__blk846_dn9 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn9)) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9))) + locals.var_q_qsq__blk825_dn9) * assign37080_e41698) - (assign37080_e41693 * ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn9)))) / (assign37080_e41698 * assign37080_e41698))),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign37080_e41702;
        locals.var_q_qsq__blk825_dn4 = assign37080_e41702_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign37080_e41702_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign37080_e41702_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign37080_e41702_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign37080_e41702_d_n9;
        locals.var_q_qsq__blk825_rv = 0.0;

        let (assign37090_e41710, assign37090_e41710_d_n4, assign37090_e41710_d_n6, assign37090_e41710_d_n7, assign37090_e41710_d_n8, assign37090_e41710_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37090_e41706: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign37090_e41708: f64 = (assign37090_e41706 - locals.var_q_qsq__blk825);
        (assign37090_e41708, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_qsq__blk825_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_qsq__blk825_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_qsq__blk825_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_qsq__blk825_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_qsq__blk825_dn9),)
    } else {
        (locals.var_q_delta__blk858, locals.var_q_delta__blk858_dn4, locals.var_q_delta__blk858_dn6, locals.var_q_delta__blk858_dn7, locals.var_q_delta__blk858_dn8, locals.var_q_delta__blk858_dn9,)
    }
};
        locals.var_q_delta__blk858 = assign37090_e41710;
        locals.var_q_delta__blk858_dn4 = assign37090_e41710_d_n4;
        locals.var_q_delta__blk858_dn6 = assign37090_e41710_d_n6;
        locals.var_q_delta__blk858_dn7 = assign37090_e41710_d_n7;
        locals.var_q_delta__blk858_dn8 = assign37090_e41710_d_n8;
        locals.var_q_delta__blk858_dn9 = assign37090_e41710_d_n9;
        locals.var_q_delta__blk858_rv = 0.0;

        let assign37100_e41713: f64 = if locals.var_q_delta__blk858 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1180 = assign37100_e41713;
        locals.var_guard1180_rv = 0.0;

        let (assign37110_e41730, assign37110_e41730_d_n4, assign37110_e41730_d_n6, assign37110_e41730_d_n7, assign37110_e41730_d_n8, assign37110_e41730_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1180 != 0.0)) {
        let assign37110_e41720: f64 = (locals.var_q_delta__blk858 / locals.var_a0__blk905);
        let assign37110_e41721: f64 = (assign37110_e41720).ln();
        let assign37110_e41723: f64 = (assign37110_e41721 + locals.var_xdeff__blk1000);
        let assign37110_e41725: f64 = (assign37110_e41723 - locals.var_xg1x__blk930);
        let assign37110_e41727: f64 = (assign37110_e41725 + locals.var_q1d__blk1001);
        let assign37110_e41728: f64 = (locals.var_q_delta__blk858 * assign37110_e41727);
        (assign37110_e41728, ((locals.var_q_delta__blk858_dn4 * assign37110_e41727) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn4 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign37110_e41720) + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4))), ((locals.var_q_delta__blk858_dn6 * assign37110_e41727) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn6 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign37110_e41720) + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6))), ((locals.var_q_delta__blk858_dn7 * assign37110_e41727) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn7 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign37110_e41720) + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7))), ((locals.var_q_delta__blk858_dn8 * assign37110_e41727) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn8 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign37110_e41720) + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8))), ((locals.var_q_delta__blk858_dn9 * assign37110_e41727) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn9 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign37110_e41720) + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9))),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign37110_e41730;
        locals.var_q_zero__blk849_dn4 = assign37110_e41730_d_n4;
        locals.var_q_zero__blk849_dn6 = assign37110_e41730_d_n6;
        locals.var_q_zero__blk849_dn7 = assign37110_e41730_d_n7;
        locals.var_q_zero__blk849_dn8 = assign37110_e41730_d_n8;
        locals.var_q_zero__blk849_dn9 = assign37110_e41730_d_n9;
        locals.var_q_zero__blk849_rv = 0.0;

        let (assign37120_e41742, assign37120_e41742_d_n4, assign37120_e41742_d_n6, assign37120_e41742_d_n7, assign37120_e41742_d_n8, assign37120_e41742_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1180 != 0.0)) {
        let assign37120_e41736: f64 = (2.0 * locals.var_k1__blk932);
        let assign37120_e41738: f64 = (assign37120_e41736 * locals.var_q_k1q1__blk823);
        let assign37120_e41740: f64 = (assign37120_e41738 + locals.var_q_delta__blk858);
        (assign37120_e41740, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign37120_e41736 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_delta__blk858_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign37120_e41736 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_delta__blk858_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign37120_e41736 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_delta__blk858_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign37120_e41736 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_delta__blk858_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign37120_e41736 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_delta__blk858_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign37120_e41742;
        locals.var_q_d1_zero__blk850_dn4 = assign37120_e41742_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign37120_e41742_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign37120_e41742_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign37120_e41742_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign37120_e41742_d_n9;
        locals.var_q_d1_zero__blk850_rv = 0.0;

        let (assign37130_e41752, assign37130_e41752_d_n4, assign37130_e41752_d_n6, assign37130_e41752_d_n7, assign37130_e41752_d_n8, assign37130_e41752_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1180 != 0.0)) {
        let assign37130_e41748: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37130_e41750: f64 = (assign37130_e41748 - locals.var_q_x1sat__blk817);
        (assign37130_e41750, ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_q_x1sat__blk817_dn4), ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_q_x1sat__blk817_dn6), ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_q_x1sat__blk817_dn7), ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_q_x1sat__blk817_dn8), ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_q_x1sat__blk817_dn9),)
    } else {
        (locals.var_q_dx1__blk859, locals.var_q_dx1__blk859_dn4, locals.var_q_dx1__blk859_dn6, locals.var_q_dx1__blk859_dn7, locals.var_q_dx1__blk859_dn8, locals.var_q_dx1__blk859_dn9,)
    }
};
        locals.var_q_dx1__blk859 = assign37130_e41752;
        locals.var_q_dx1__blk859_dn4 = assign37130_e41752_d_n4;
        locals.var_q_dx1__blk859_dn6 = assign37130_e41752_d_n6;
        locals.var_q_dx1__blk859_dn7 = assign37130_e41752_d_n7;
        locals.var_q_dx1__blk859_dn8 = assign37130_e41752_d_n8;
        locals.var_q_dx1__blk859_dn9 = assign37130_e41752_d_n9;
        locals.var_q_dx1__blk859_rv = 0.0;

        let assign37140_e41762: f64 = (locals.var_q_dx1__blk859 + 2.3025850929941);
        let assign37140_e41764: f64 = (locals.var_k1__blk932).ln();
        let assign37140_e41765: f64 = (assign37140_e41762 + assign37140_e41764);
        let assign37140_e41772: f64 = if ((((locals.var_q_zero__blk849 < 0.0) && (locals.var_q_d1_zero__blk850 > 0.0)) && (assign37140_e41765 > 0.0)) || (locals.var_q_dx1__blk859 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1181 = assign37140_e41772;
        locals.var_guard1181_rv = 0.0;

        let (assign37150_e41784, assign37150_e41784_d_n4, assign37150_e41784_d_n6, assign37150_e41784_d_n7, assign37150_e41784_d_n8, assign37150_e41784_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1180 != 0.0)) && (locals.var_guard1181 != 0.0)) {
        let assign37150_e41781: f64 = (locals.var_q_zero__blk849 / locals.var_q_d1_zero__blk850);
        let assign37150_e41782: f64 = (locals.var_q1d__blk1001 - assign37150_e41781);
        (assign37150_e41782, (locals.var_q1d__blk1001_dn4 - (((locals.var_q_zero__blk849_dn4 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn4)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn6 - (((locals.var_q_zero__blk849_dn6 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn6)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn7 - (((locals.var_q_zero__blk849_dn7 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn7)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn8 - (((locals.var_q_zero__blk849_dn8 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn8)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn9 - (((locals.var_q_zero__blk849_dn9 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn9)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign37150_e41784;
        locals.var_q1d__blk1001_dn4 = assign37150_e41784_d_n4;
        locals.var_q1d__blk1001_dn6 = assign37150_e41784_d_n6;
        locals.var_q1d__blk1001_dn7 = assign37150_e41784_d_n7;
        locals.var_q1d__blk1001_dn8 = assign37150_e41784_d_n8;
        locals.var_q1d__blk1001_dn9 = assign37150_e41784_d_n9;
        locals.var_q1d__blk1001_rv = 0.0;

        let (assign37160_e41790, assign37160_e41790_d_n4, assign37160_e41790_d_n6, assign37160_e41790_d_n7, assign37160_e41790_d_n8, assign37160_e41790_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37160_e41788: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign37160_e41788, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign37160_e41790;
        locals.var_q_k1q1__blk823_dn4 = assign37160_e41790_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign37160_e41790_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign37160_e41790_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign37160_e41790_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign37160_e41790_d_n9;
        locals.var_q_k1q1__blk823_rv = 0.0;

        let assign37170_e41793: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37170_e41795: f64 = (assign37170_e41793 - locals.var_xdeff__blk1000);
        let assign37170_e41797: f64 = if assign37170_e41795 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1182 = assign37170_e41797;
        locals.var_guard1182_rv = 0.0;

        let (assign37180_e41808, assign37180_e41808_d_n4, assign37180_e41808_d_n6, assign37180_e41808_d_n7, assign37180_e41808_d_n8, assign37180_e41808_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1182 != 0.0)) {
        let assign37180_e41803: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37180_e41805: f64 = (assign37180_e41803 - locals.var_xdeff__blk1000);
        let assign37180_e41806: f64 = (assign37180_e41805).exp();
        (assign37180_e41806, (assign37180_e41806 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign37180_e41806 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign37180_e41806 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign37180_e41806 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign37180_e41806 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37180_e41808;
        locals.var_q_temp1__blk814_dn4 = assign37180_e41808_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37180_e41808_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37180_e41808_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37180_e41808_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37180_e41808_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign37190_e41849, assign37190_e41849_d_n4, assign37190_e41849_d_n6, assign37190_e41849_d_n7, assign37190_e41849_d_n8, assign37190_e41849_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1182 == 0.0)) {
        let assign37190_e41817: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37190_e41819: f64 = (assign37190_e41817 - locals.var_xdeff__blk1000);
        let assign37190_e41821: f64 = (assign37190_e41819 - 80.0);
        let assign37190_e41826: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37190_e41828: f64 = (assign37190_e41826 - locals.var_xdeff__blk1000);
        let assign37190_e41830: f64 = (assign37190_e41828 - 80.0);
        let assign37190_e41831: f64 = (0.5 * assign37190_e41830);
        let assign37190_e41835: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37190_e41837: f64 = (assign37190_e41835 - locals.var_xdeff__blk1000);
        let assign37190_e41839: f64 = (assign37190_e41837 - 80.0);
        let assign37190_e41841: f64 = (assign37190_e41839 * 0.3333333333333);
        let assign37190_e41842: f64 = (1.0 + assign37190_e41841);
        let assign37190_e41843: f64 = (assign37190_e41831 * assign37190_e41842);
        let assign37190_e41844: f64 = (1.0 + assign37190_e41843);
        let assign37190_e41845: f64 = (assign37190_e41821 * assign37190_e41844);
        let assign37190_e41846: f64 = (1.0 + assign37190_e41845);
        let assign37190_e41847: f64 = (5.54062e34 * assign37190_e41846);
        (assign37190_e41847, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign37190_e41844) + (assign37190_e41821 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign37190_e41842) + (assign37190_e41831 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign37190_e41844) + (assign37190_e41821 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign37190_e41842) + (assign37190_e41831 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign37190_e41844) + (assign37190_e41821 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign37190_e41842) + (assign37190_e41831 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign37190_e41844) + (assign37190_e41821 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign37190_e41842) + (assign37190_e41831 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign37190_e41844) + (assign37190_e41821 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign37190_e41842) + (assign37190_e41831 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37190_e41849;
        locals.var_q_temp1__blk814_dn4 = assign37190_e41849_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37190_e41849_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37190_e41849_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37190_e41849_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37190_e41849_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign37200_e41855, assign37200_e41855_d_n4, assign37200_e41855_d_n6, assign37200_e41855_d_n7, assign37200_e41855_d_n8, assign37200_e41855_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37200_e41853: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign37200_e41853, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign37200_e41855;
        locals.var_q_aexp__blk824_dn4 = assign37200_e41855_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign37200_e41855_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign37200_e41855_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign37200_e41855_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign37200_e41855_d_n9;
        locals.var_q_aexp__blk824_rv = 0.0;

        let (assign37210_e41863, assign37210_e41863_d_n4, assign37210_e41863_d_n6, assign37210_e41863_d_n7, assign37210_e41863_d_n8, assign37210_e41863_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37210_e41859: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign37210_e41861: f64 = (assign37210_e41859 - locals.var_q_aexp__blk824);
        (assign37210_e41861, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign37210_e41863;
        locals.var_q_qsq__blk825_dn4 = assign37210_e41863_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign37210_e41863_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign37210_e41863_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign37210_e41863_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign37210_e41863_d_n9;
        locals.var_q_qsq__blk825_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_107(
        locals: &mut StampLocals,
    ) {
        let (assign37220_e41873, assign37220_e41873_d_n4, assign37220_e41873_d_n6, assign37220_e41873_d_n7, assign37220_e41873_d_n8, assign37220_e41873_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37220_e41867: f64 = (2.0 * locals.var_k1__blk932);
        let assign37220_e41869: f64 = (assign37220_e41867 * locals.var_q_k1q1__blk823);
        let assign37220_e41871: f64 = (assign37220_e41869 + locals.var_q_aexp__blk824);
        (assign37220_e41871, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign37220_e41867 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign37220_e41867 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign37220_e41867 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign37220_e41867 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign37220_e41867 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign37220_e41873;
        locals.var_q_d1_qsq__blk826_dn4 = assign37220_e41873_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign37220_e41873_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign37220_e41873_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign37220_e41873_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign37220_e41873_d_n9;
        locals.var_q_d1_qsq__blk826_rv = 0.0;

        let (assign37230_e41883, assign37230_e41883_d_n4, assign37230_e41883_d_n6, assign37230_e41883_d_n7, assign37230_e41883_d_n8, assign37230_e41883_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37230_e41877: f64 = (2.0 * locals.var_k1__blk932);
        let assign37230_e41879: f64 = (assign37230_e41877 * locals.var_k1__blk932);
        let assign37230_e41881: f64 = (assign37230_e41879 - locals.var_q_aexp__blk824);
        (assign37230_e41881, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign37230_e41877 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign37230_e41877 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign37230_e41877 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign37230_e41877 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign37230_e41877 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign37230_e41883;
        locals.var_q_d2_qsq__blk827_dn4 = assign37230_e41883_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign37230_e41883_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign37230_e41883_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign37230_e41883_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign37230_e41883_d_n9;
        locals.var_q_d2_qsq__blk827_rv = 0.0;

        let assign37240_e41886: f64 = (-0.005);
        let assign37240_e41887: f64 = if locals.var_q_qsq__blk825 < assign37240_e41886 { 1.0 } else { 0.0 };
        locals.var_guard1183 = assign37240_e41887;
        locals.var_guard1183_rv = 0.0;

        let (assign37250_e41895, assign37250_e41895_d_n4, assign37250_e41895_d_n6, assign37250_e41895_d_n7, assign37250_e41895_d_n8, assign37250_e41895_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37250_e41892: f64 = (locals.var_q_qsq__blk825).abs();
        let assign37250_e41893: f64 = (assign37250_e41892).sqrt();
        (assign37250_e41893, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign37250_e41893)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign37250_e41893)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign37250_e41893)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign37250_e41893)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign37250_e41893)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign37250_e41895;
        locals.var_q_rac_qsq__blk828_dn4 = assign37250_e41895_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign37250_e41895_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign37250_e41895_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign37250_e41895_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign37250_e41895_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign37260_e41906, assign37260_e41906_d_n4, assign37260_e41906_d_n6, assign37260_e41906_d_n7, assign37260_e41906_d_n8, assign37260_e41906_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37260_e41902: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign37260_e41903: f64 = (assign37260_e41902).tan();
        let assign37260_e41904: f64 = (locals.var_q_rac_qsq__blk828 / assign37260_e41903);
        (assign37260_e41904, (((locals.var_q_rac_qsq__blk828_dn4 * assign37260_e41903) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign37260_e41902).cos() * (assign37260_e41902).cos())))) / (assign37260_e41903 * assign37260_e41903)), (((locals.var_q_rac_qsq__blk828_dn6 * assign37260_e41903) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign37260_e41902).cos() * (assign37260_e41902).cos())))) / (assign37260_e41903 * assign37260_e41903)), (((locals.var_q_rac_qsq__blk828_dn7 * assign37260_e41903) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign37260_e41902).cos() * (assign37260_e41902).cos())))) / (assign37260_e41903 * assign37260_e41903)), (((locals.var_q_rac_qsq__blk828_dn8 * assign37260_e41903) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign37260_e41902).cos() * (assign37260_e41902).cos())))) / (assign37260_e41903 * assign37260_e41903)), (((locals.var_q_rac_qsq__blk828_dn9 * assign37260_e41903) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign37260_e41902).cos() * (assign37260_e41902).cos())))) / (assign37260_e41903 * assign37260_e41903)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37260_e41906;
        locals.var_q_qcoth__blk829_dn4 = assign37260_e41906_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37260_e41906_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37260_e41906_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37260_e41906_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37260_e41906_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign37270_e41916, assign37270_e41916_d_n4, assign37270_e41916_d_n6, assign37270_e41916_d_n7, assign37270_e41916_d_n8, assign37270_e41916_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37270_e41912: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign37270_e41914: f64 = (assign37270_e41912 / locals.var_q_qsq__blk825);
        (assign37270_e41914, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign37270_e41912 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign37270_e41912 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign37270_e41912 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign37270_e41912 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign37270_e41912 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37270_e41916;
        locals.var_q_temp1__blk814_dn4 = assign37270_e41916_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37270_e41916_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37270_e41916_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37270_e41916_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37270_e41916_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign37280_e41930, assign37280_e41930_d_n4, assign37280_e41930_d_n6, assign37280_e41930_d_n7, assign37280_e41930_d_n8, assign37280_e41930_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37280_e41924: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign37280_e41925: f64 = (locals.var_q_qcoth__blk829 * assign37280_e41924);
        let assign37280_e41926: f64 = (locals.var_q_qsq__blk825 + assign37280_e41925);
        let assign37280_e41928: f64 = (assign37280_e41926 * locals.var_q_temp1__blk814);
        (assign37280_e41928, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign37280_e41924) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign37280_e41926 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign37280_e41924) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign37280_e41926 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign37280_e41924) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign37280_e41926 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign37280_e41924) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign37280_e41926 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign37280_e41924) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign37280_e41926 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37280_e41930;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37280_e41930_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37280_e41930_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37280_e41930_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37280_e41930_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37280_e41930_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign37290_e41952, assign37290_e41952_d_n4, assign37290_e41952_d_n6, assign37290_e41952_d_n7, assign37290_e41952_d_n8, assign37290_e41952_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37290_e41937: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign37290_e41940: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign37290_e41941: f64 = (assign37290_e41937 * assign37290_e41940);
        let assign37290_e41942: f64 = (locals.var_q_d1_qsq__blk826 - assign37290_e41941);
        let assign37290_e41944: f64 = (assign37290_e41942 * locals.var_q_temp1__blk814);
        let assign37290_e41947: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign37290_e41949: f64 = (assign37290_e41947 / locals.var_q_d1_qsq__blk826);
        let assign37290_e41950: f64 = (assign37290_e41944 + assign37290_e41949);
        (assign37290_e41950, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign37290_e41940) + (assign37290_e41937 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign37290_e41942 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign37290_e41947 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign37290_e41940) + (assign37290_e41937 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign37290_e41942 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign37290_e41947 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign37290_e41940) + (assign37290_e41937 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign37290_e41942 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign37290_e41947 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign37290_e41940) + (assign37290_e41937 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign37290_e41942 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign37290_e41947 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign37290_e41940) + (assign37290_e41937 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign37290_e41942 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign37290_e41947 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign37290_e41952;
        locals.var_q_d2_qcoth__blk832_dn4 = assign37290_e41952_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign37290_e41952_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign37290_e41952_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign37290_e41952_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign37290_e41952_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign37300_e41962, assign37300_e41962_d_n4, assign37300_e41962_d_n6, assign37300_e41962_d_n7, assign37300_e41962_d_n8, assign37300_e41962_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37300_e41959: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign37300_e41960: f64 = (1.0 - assign37300_e41959);
        (assign37300_e41960, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37300_e41962;
        locals.var_q_temp2__blk815_dn4 = assign37300_e41962_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37300_e41962_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37300_e41962_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37300_e41962_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37300_e41962_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign37310_e41972, assign37310_e41972_d_n4, assign37310_e41972_d_n6, assign37310_e41972_d_n7, assign37310_e41972_d_n8, assign37310_e41972_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37310_e41968: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign37310_e41970: f64 = (assign37310_e41968 * locals.var_q_temp2__blk815);
        (assign37310_e41970, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37310_e41968 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37310_e41968 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37310_e41968 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37310_e41968 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37310_e41968 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign37310_e41972;
        locals.var_q_d1_ln__blk835_dn4 = assign37310_e41972_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign37310_e41972_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign37310_e41972_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign37310_e41972_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign37310_e41972_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign37320_e41990, assign37320_e41990_d_n4, assign37320_e41990_d_n6, assign37320_e41990_d_n7, assign37320_e41990_d_n8, assign37320_e41990_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37320_e41978: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign37320_e41983: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign37320_e41984: f64 = (locals.var_q_d1_ln__blk835 + assign37320_e41983);
        let assign37320_e41985: f64 = (locals.var_q_d1_qsq__blk826 * assign37320_e41984);
        let assign37320_e41986: f64 = (assign37320_e41978 - assign37320_e41985);
        let assign37320_e41988: f64 = (assign37320_e41986 / locals.var_q_qsq__blk825);
        (assign37320_e41988, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign37320_e41984) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign37320_e41986 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign37320_e41984) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign37320_e41986 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign37320_e41984) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign37320_e41986 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign37320_e41984) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign37320_e41986 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign37320_e41984) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign37320_e41986 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign37320_e41990;
        locals.var_q_d2_ln__blk836_dn4 = assign37320_e41990_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign37320_e41990_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign37320_e41990_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign37320_e41990_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign37320_e41990_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let assign37330_e41993: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1184 = assign37330_e41993;
        locals.var_guard1184_rv = 0.0;

        let (assign37340_e42004, assign37340_e42004_d_n4, assign37340_e42004_d_n6, assign37340_e42004_d_n7, assign37340_e42004_d_n8, assign37340_e42004_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37340_e42001: f64 = (locals.var_q_qsq__blk825).abs();
        let assign37340_e42002: f64 = (assign37340_e42001).sqrt();
        (assign37340_e42002, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign37340_e42002)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign37340_e42002)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign37340_e42002)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign37340_e42002)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign37340_e42002)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign37340_e42004;
        locals.var_q_rac_qsq__blk828_dn4 = assign37340_e42004_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign37340_e42004_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign37340_e42004_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign37340_e42004_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign37340_e42004_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign37350_e42015, assign37350_e42015_d_n4, assign37350_e42015_d_n6, assign37350_e42015_d_n7, assign37350_e42015_d_n8, assign37350_e42015_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37350_e42012: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign37350_e42013: f64 = (assign37350_e42012).exp();
        (assign37350_e42013, (assign37350_e42013 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign37350_e42013 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign37350_e42013 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign37350_e42013 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign37350_e42013 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign37350_e42015;
        locals.var_q_invexpq__blk831_dn4 = assign37350_e42015_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign37350_e42015_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign37350_e42015_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign37350_e42015_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign37350_e42015_d_n9;
        locals.var_q_invexpq__blk831_rv = 0.0;

        let (assign37360_e42032, assign37360_e42032_d_n4, assign37360_e42032_d_n6, assign37360_e42032_d_n7, assign37360_e42032_d_n8, assign37360_e42032_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37360_e42025: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign37360_e42026: f64 = (locals.var_q_rac_qsq__blk828 * assign37360_e42025);
        let assign37360_e42029: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign37360_e42030: f64 = (assign37360_e42026 / assign37360_e42029);
        (assign37360_e42030, (((((locals.var_q_rac_qsq__blk828_dn4 * assign37360_e42025) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign37360_e42029) - (assign37360_e42026 * (-locals.var_q_invexpq__blk831_dn4))) / (assign37360_e42029 * assign37360_e42029)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign37360_e42025) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign37360_e42029) - (assign37360_e42026 * (-locals.var_q_invexpq__blk831_dn6))) / (assign37360_e42029 * assign37360_e42029)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign37360_e42025) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign37360_e42029) - (assign37360_e42026 * (-locals.var_q_invexpq__blk831_dn7))) / (assign37360_e42029 * assign37360_e42029)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign37360_e42025) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign37360_e42029) - (assign37360_e42026 * (-locals.var_q_invexpq__blk831_dn8))) / (assign37360_e42029 * assign37360_e42029)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign37360_e42025) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign37360_e42029) - (assign37360_e42026 * (-locals.var_q_invexpq__blk831_dn9))) / (assign37360_e42029 * assign37360_e42029)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37360_e42032;
        locals.var_q_qcoth__blk829_dn4 = assign37360_e42032_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37360_e42032_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37360_e42032_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37360_e42032_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37360_e42032_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign37370_e42045, assign37370_e42045_d_n4, assign37370_e42045_d_n6, assign37370_e42045_d_n7, assign37370_e42045_d_n8, assign37370_e42045_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37370_e42041: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign37370_e42043: f64 = (assign37370_e42041 / locals.var_q_qsq__blk825);
        (assign37370_e42043, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign37370_e42041 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign37370_e42041 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign37370_e42041 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign37370_e42041 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign37370_e42041 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37370_e42045;
        locals.var_q_temp1__blk814_dn4 = assign37370_e42045_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37370_e42045_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37370_e42045_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37370_e42045_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37370_e42045_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign37380_e42062, assign37380_e42062_d_n4, assign37380_e42062_d_n6, assign37380_e42062_d_n7, assign37380_e42062_d_n8, assign37380_e42062_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37380_e42056: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign37380_e42057: f64 = (locals.var_q_qcoth__blk829 * assign37380_e42056);
        let assign37380_e42058: f64 = (locals.var_q_qsq__blk825 + assign37380_e42057);
        let assign37380_e42060: f64 = (assign37380_e42058 * locals.var_q_temp1__blk814);
        (assign37380_e42060, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign37380_e42056) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign37380_e42058 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign37380_e42056) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign37380_e42058 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign37380_e42056) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign37380_e42058 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign37380_e42056) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign37380_e42058 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign37380_e42056) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign37380_e42058 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37380_e42062;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37380_e42062_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37380_e42062_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37380_e42062_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37380_e42062_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37380_e42062_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign37390_e42087, assign37390_e42087_d_n4, assign37390_e42087_d_n6, assign37390_e42087_d_n7, assign37390_e42087_d_n8, assign37390_e42087_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37390_e42072: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign37390_e42075: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign37390_e42076: f64 = (assign37390_e42072 * assign37390_e42075);
        let assign37390_e42077: f64 = (locals.var_q_d1_qsq__blk826 - assign37390_e42076);
        let assign37390_e42079: f64 = (assign37390_e42077 * locals.var_q_temp1__blk814);
        let assign37390_e42082: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign37390_e42084: f64 = (assign37390_e42082 / locals.var_q_d1_qsq__blk826);
        let assign37390_e42085: f64 = (assign37390_e42079 + assign37390_e42084);
        (assign37390_e42085, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign37390_e42075) + (assign37390_e42072 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign37390_e42077 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign37390_e42082 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign37390_e42075) + (assign37390_e42072 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign37390_e42077 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign37390_e42082 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign37390_e42075) + (assign37390_e42072 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign37390_e42077 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign37390_e42082 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign37390_e42075) + (assign37390_e42072 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign37390_e42077 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign37390_e42082 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign37390_e42075) + (assign37390_e42072 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign37390_e42077 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign37390_e42082 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign37390_e42087;
        locals.var_q_d2_qcoth__blk832_dn4 = assign37390_e42087_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign37390_e42087_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign37390_e42087_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign37390_e42087_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign37390_e42087_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign37400_e42100, assign37400_e42100_d_n4, assign37400_e42100_d_n6, assign37400_e42100_d_n7, assign37400_e42100_d_n8, assign37400_e42100_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37400_e42097: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign37400_e42098: f64 = (1.0 - assign37400_e42097);
        (assign37400_e42098, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37400_e42100;
        locals.var_q_temp2__blk815_dn4 = assign37400_e42100_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37400_e42100_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37400_e42100_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37400_e42100_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37400_e42100_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign37410_e42113, assign37410_e42113_d_n4, assign37410_e42113_d_n6, assign37410_e42113_d_n7, assign37410_e42113_d_n8, assign37410_e42113_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37410_e42109: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign37410_e42111: f64 = (assign37410_e42109 * locals.var_q_temp2__blk815);
        (assign37410_e42111, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37410_e42109 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37410_e42109 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37410_e42109 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37410_e42109 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37410_e42109 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign37410_e42113;
        locals.var_q_d1_ln__blk835_dn4 = assign37410_e42113_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign37410_e42113_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign37410_e42113_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign37410_e42113_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign37410_e42113_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign37420_e42134, assign37420_e42134_d_n4, assign37420_e42134_d_n6, assign37420_e42134_d_n7, assign37420_e42134_d_n8, assign37420_e42134_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37420_e42122: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign37420_e42127: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign37420_e42128: f64 = (locals.var_q_d1_ln__blk835 + assign37420_e42127);
        let assign37420_e42129: f64 = (locals.var_q_d1_qsq__blk826 * assign37420_e42128);
        let assign37420_e42130: f64 = (assign37420_e42122 - assign37420_e42129);
        let assign37420_e42132: f64 = (assign37420_e42130 / locals.var_q_qsq__blk825);
        (assign37420_e42132, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign37420_e42128) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign37420_e42130 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign37420_e42128) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign37420_e42130 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign37420_e42128) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign37420_e42130 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign37420_e42128) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign37420_e42130 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign37420_e42128) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign37420_e42130 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign37420_e42134;
        locals.var_q_d2_ln__blk836_dn4 = assign37420_e42134_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign37420_e42134_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign37420_e42134_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign37420_e42134_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign37420_e42134_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let (assign37430_e42162, assign37430_e42162_d_n4, assign37430_e42162_d_n6, assign37430_e42162_d_n7, assign37430_e42162_d_n8, assign37430_e42162_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37430_e42146: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign37430_e42150: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign37430_e42154: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign37430_e42155: f64 = (1.0 - assign37430_e42154);
        let assign37430_e42156: f64 = (assign37430_e42150 * assign37430_e42155);
        let assign37430_e42157: f64 = (1.0 - assign37430_e42156);
        let assign37430_e42158: f64 = (assign37430_e42146 * assign37430_e42157);
        let assign37430_e42159: f64 = (1.0 - assign37430_e42158);
        let assign37430_e42160: f64 = (0.1666666666667 * assign37430_e42159);
        (assign37430_e42160, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign37430_e42157) + (assign37430_e42146 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign37430_e42155) + (assign37430_e42150 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign37430_e42157) + (assign37430_e42146 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign37430_e42155) + (assign37430_e42150 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign37430_e42157) + (assign37430_e42146 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign37430_e42155) + (assign37430_e42150 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign37430_e42157) + (assign37430_e42146 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign37430_e42155) + (assign37430_e42150 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign37430_e42157) + (assign37430_e42146 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign37430_e42155) + (assign37430_e42150 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign37430_e42162;
        locals.var_q_temp3__blk816_dn4 = assign37430_e42162_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign37430_e42162_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign37430_e42162_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign37430_e42162_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign37430_e42162_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign37440_e42176, assign37440_e42176_d_n4, assign37440_e42176_d_n6, assign37440_e42176_d_n7, assign37440_e42176_d_n8, assign37440_e42176_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37440_e42173: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign37440_e42174: f64 = (2.0 + assign37440_e42173);
        (assign37440_e42174, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37440_e42176;
        locals.var_q_qcoth__blk829_dn4 = assign37440_e42176_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37440_e42176_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37440_e42176_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37440_e42176_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37440_e42176_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign37450_e42204, assign37450_e42204_d_n4, assign37450_e42204_d_n6, assign37450_e42204_d_n7, assign37450_e42204_d_n8, assign37450_e42204_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37450_e42188: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign37450_e42192: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign37450_e42196: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign37450_e42197: f64 = (1.0 - assign37450_e42196);
        let assign37450_e42198: f64 = (assign37450_e42192 * assign37450_e42197);
        let assign37450_e42199: f64 = (1.0 - assign37450_e42198);
        let assign37450_e42200: f64 = (assign37450_e42188 * assign37450_e42199);
        let assign37450_e42201: f64 = (1.0 - assign37450_e42200);
        let assign37450_e42202: f64 = (0.1666666666667 * assign37450_e42201);
        (assign37450_e42202, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign37450_e42199) + (assign37450_e42188 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign37450_e42197) + (assign37450_e42192 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign37450_e42199) + (assign37450_e42188 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign37450_e42197) + (assign37450_e42192 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign37450_e42199) + (assign37450_e42188 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign37450_e42197) + (assign37450_e42192 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign37450_e42199) + (assign37450_e42188 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign37450_e42197) + (assign37450_e42192 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign37450_e42199) + (assign37450_e42188 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign37450_e42197) + (assign37450_e42192 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37450_e42204;
        locals.var_q_temp1__blk814_dn4 = assign37450_e42204_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37450_e42204_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37450_e42204_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37450_e42204_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37450_e42204_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign37460_e42216, assign37460_e42216_d_n4, assign37460_e42216_d_n6, assign37460_e42216_d_n7, assign37460_e42216_d_n8, assign37460_e42216_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37460_e42214: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign37460_e42214, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37460_e42216;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37460_e42216_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37460_e42216_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37460_e42216_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37460_e42216_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37460_e42216_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign37470_e42244, assign37470_e42244_d_n4, assign37470_e42244_d_n6, assign37470_e42244_d_n7, assign37470_e42244_d_n8, assign37470_e42244_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37470_e42228: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign37470_e42232: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign37470_e42236: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign37470_e42237: f64 = (1.0 - assign37470_e42236);
        let assign37470_e42238: f64 = (assign37470_e42232 * assign37470_e42237);
        let assign37470_e42239: f64 = (1.0 - assign37470_e42238);
        let assign37470_e42240: f64 = (assign37470_e42228 * assign37470_e42239);
        let assign37470_e42241: f64 = (1.0 - assign37470_e42240);
        let assign37470_e42242: f64 = (0.0055555555556 * assign37470_e42241);
        (assign37470_e42242, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign37470_e42239) + (assign37470_e42228 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign37470_e42237) + (assign37470_e42232 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign37470_e42239) + (assign37470_e42228 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign37470_e42237) + (assign37470_e42232 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign37470_e42239) + (assign37470_e42228 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign37470_e42237) + (assign37470_e42232 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign37470_e42239) + (assign37470_e42228 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign37470_e42237) + (assign37470_e42232 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign37470_e42239) + (assign37470_e42228 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign37470_e42237) + (assign37470_e42232 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37470_e42244;
        locals.var_q_temp2__blk815_dn4 = assign37470_e42244_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37470_e42244_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37470_e42244_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37470_e42244_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37470_e42244_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign37480_e42262, assign37480_e42262_d_n4, assign37480_e42262_d_n6, assign37480_e42262_d_n7, assign37480_e42262_d_n8, assign37480_e42262_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37480_e42254: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign37480_e42257: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign37480_e42259: f64 = (assign37480_e42257 * locals.var_q_temp2__blk815);
        let assign37480_e42260: f64 = (assign37480_e42254 - assign37480_e42259);
        (assign37480_e42260, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign37480_e42257 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign37480_e42257 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign37480_e42257 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign37480_e42257 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign37480_e42257 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign37480_e42262;
        locals.var_q_d2_qcoth__blk832_dn4 = assign37480_e42262_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign37480_e42262_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign37480_e42262_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign37480_e42262_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign37480_e42262_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign37490_e42277, assign37490_e42277_d_n4, assign37490_e42277_d_n6, assign37490_e42277_d_n7, assign37490_e42277_d_n8, assign37490_e42277_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37490_e42271: f64 = (-0.5);
        let assign37490_e42273: f64 = (assign37490_e42271 * locals.var_q_d1_qsq__blk826);
        let assign37490_e42275: f64 = (assign37490_e42273 * locals.var_q_temp3__blk816);
        (assign37490_e42275, (((assign37490_e42271 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign37490_e42273 * locals.var_q_temp3__blk816_dn4)), (((assign37490_e42271 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign37490_e42273 * locals.var_q_temp3__blk816_dn6)), (((assign37490_e42271 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign37490_e42273 * locals.var_q_temp3__blk816_dn7)), (((assign37490_e42271 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign37490_e42273 * locals.var_q_temp3__blk816_dn8)), (((assign37490_e42271 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign37490_e42273 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign37490_e42277;
        locals.var_q_d1_ln__blk835_dn4 = assign37490_e42277_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign37490_e42277_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign37490_e42277_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign37490_e42277_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign37490_e42277_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_108(
        locals: &mut StampLocals,
    ) {
        let (assign37500_e42312, assign37500_e42312_d_n4, assign37500_e42312_d_n6, assign37500_e42312_d_n7, assign37500_e42312_d_n8, assign37500_e42312_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37500_e42286: f64 = (-0.5);
        let assign37500_e42288: f64 = (assign37500_e42286 * locals.var_q_d2_qsq__blk827);
        let assign37500_e42290: f64 = (assign37500_e42288 * locals.var_q_temp3__blk816);
        let assign37500_e42293: f64 = (0.25 * 0.0055555555556);
        let assign37500_e42295: f64 = (assign37500_e42293 * locals.var_q_d1_qsq__blk826);
        let assign37500_e42297: f64 = (assign37500_e42295 * locals.var_q_d1_qsq__blk826);
        let assign37500_e42301: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign37500_e42305: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign37500_e42306: f64 = (2.0 - assign37500_e42305);
        let assign37500_e42307: f64 = (assign37500_e42301 * assign37500_e42306);
        let assign37500_e42308: f64 = (1.0 - assign37500_e42307);
        let assign37500_e42309: f64 = (assign37500_e42297 * assign37500_e42308);
        let assign37500_e42310: f64 = (assign37500_e42290 + assign37500_e42309);
        (assign37500_e42310, ((((assign37500_e42286 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign37500_e42288 * locals.var_q_temp3__blk816_dn4)) + (((((assign37500_e42293 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign37500_e42295 * locals.var_q_d1_qsq__blk826_dn4)) * assign37500_e42308) + (assign37500_e42297 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign37500_e42306) + (assign37500_e42301 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign37500_e42286 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign37500_e42288 * locals.var_q_temp3__blk816_dn6)) + (((((assign37500_e42293 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign37500_e42295 * locals.var_q_d1_qsq__blk826_dn6)) * assign37500_e42308) + (assign37500_e42297 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign37500_e42306) + (assign37500_e42301 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign37500_e42286 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign37500_e42288 * locals.var_q_temp3__blk816_dn7)) + (((((assign37500_e42293 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign37500_e42295 * locals.var_q_d1_qsq__blk826_dn7)) * assign37500_e42308) + (assign37500_e42297 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign37500_e42306) + (assign37500_e42301 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign37500_e42286 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign37500_e42288 * locals.var_q_temp3__blk816_dn8)) + (((((assign37500_e42293 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign37500_e42295 * locals.var_q_d1_qsq__blk826_dn8)) * assign37500_e42308) + (assign37500_e42297 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign37500_e42306) + (assign37500_e42301 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign37500_e42286 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign37500_e42288 * locals.var_q_temp3__blk816_dn9)) + (((((assign37500_e42293 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign37500_e42295 * locals.var_q_d1_qsq__blk826_dn9)) * assign37500_e42308) + (assign37500_e42297 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign37500_e42306) + (assign37500_e42301 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign37500_e42312;
        locals.var_q_d2_ln__blk836_dn4 = assign37500_e42312_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign37500_e42312_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign37500_e42312_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign37500_e42312_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign37500_e42312_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let assign37510_e42315: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1185 = assign37510_e42315;
        locals.var_guard1185_rv = 0.0;

        let (assign37520_e42331, assign37520_e42331_d_n4, assign37520_e42331_d_n6, assign37520_e42331_d_n7, assign37520_e42331_d_n8, assign37520_e42331_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1185 != 0.0)) {
        let assign37520_e42321: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign37520_e42326: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign37520_e42327: f64 = (locals.var_q_invexpq__blk831 * assign37520_e42326);
        let assign37520_e42328: f64 = (1.0 - assign37520_e42327);
        let assign37520_e42329: f64 = (assign37520_e42321 / assign37520_e42328);
        (assign37520_e42329, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign37520_e42328) - (assign37520_e42321 * (-((locals.var_q_invexpq__blk831_dn4 * assign37520_e42326) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign37520_e42328 * assign37520_e42328)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign37520_e42328) - (assign37520_e42321 * (-((locals.var_q_invexpq__blk831_dn6 * assign37520_e42326) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign37520_e42328 * assign37520_e42328)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign37520_e42328) - (assign37520_e42321 * (-((locals.var_q_invexpq__blk831_dn7 * assign37520_e42326) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign37520_e42328 * assign37520_e42328)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign37520_e42328) - (assign37520_e42321 * (-((locals.var_q_invexpq__blk831_dn8 * assign37520_e42326) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign37520_e42328 * assign37520_e42328)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign37520_e42328) - (assign37520_e42321 * (-((locals.var_q_invexpq__blk831_dn9 * assign37520_e42326) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign37520_e42328 * assign37520_e42328)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37520_e42331;
        locals.var_q_temp2__blk815_dn4 = assign37520_e42331_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37520_e42331_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37520_e42331_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37520_e42331_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37520_e42331_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign37530_e42339, assign37530_e42339_d_n4, assign37530_e42339_d_n6, assign37530_e42339_d_n7, assign37530_e42339_d_n8, assign37530_e42339_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1185 != 0.0)) {
        let assign37530_e42337: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign37530_e42337, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign37530_e42339;
        locals.var_q_sh_term__blk833_dn4 = assign37530_e42339_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign37530_e42339_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign37530_e42339_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign37530_e42339_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign37530_e42339_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign37540_e42348, assign37540_e42348_d_n4, assign37540_e42348_d_n6, assign37540_e42348_d_n7, assign37540_e42348_d_n8, assign37540_e42348_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1185 != 0.0)) {
        let assign37540_e42344: f64 = (locals.var_q_temp2__blk815).ln();
        let assign37540_e42346: f64 = (assign37540_e42344 - locals.var_q_rac_qsq__blk828);
        (assign37540_e42346, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign37540_e42348;
        locals.var_q_ln_term__blk834_dn4 = assign37540_e42348_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign37540_e42348_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign37540_e42348_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign37540_e42348_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign37540_e42348_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let assign37550_e42351: f64 = (-0.005);
        let assign37550_e42352: f64 = if locals.var_q_qsq__blk825 < assign37550_e42351 { 1.0 } else { 0.0 };
        locals.var_guard1186 = assign37550_e42352;
        locals.var_guard1186_rv = 0.0;

        let (assign37560_e42364, assign37560_e42364_d_n4, assign37560_e42364_d_n6, assign37560_e42364_d_n7, assign37560_e42364_d_n8, assign37560_e42364_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1185 == 0.0)) && (locals.var_guard1186 != 0.0)) {
        let assign37560_e42361: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign37560_e42362: f64 = (assign37560_e42361).sin();
        (assign37560_e42362, ((assign37560_e42361).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign37560_e42361).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign37560_e42361).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign37560_e42361).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign37560_e42361).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37560_e42364;
        locals.var_q_temp2__blk815_dn4 = assign37560_e42364_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37560_e42364_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37560_e42364_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37560_e42364_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37560_e42364_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign37570_e42378, assign37570_e42378_d_n4, assign37570_e42378_d_n6, assign37570_e42378_d_n7, assign37570_e42378_d_n8, assign37570_e42378_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1185 == 0.0)) && (locals.var_guard1186 != 0.0)) {
        let assign37570_e42372: f64 = (-locals.var_q_qsq__blk825);
        let assign37570_e42375: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign37570_e42376: f64 = (assign37570_e42372 / assign37570_e42375);
        (assign37570_e42376, ((((-locals.var_q_qsq__blk825_dn4) * assign37570_e42375) - (assign37570_e42372 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign37570_e42375 * assign37570_e42375)), ((((-locals.var_q_qsq__blk825_dn6) * assign37570_e42375) - (assign37570_e42372 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign37570_e42375 * assign37570_e42375)), ((((-locals.var_q_qsq__blk825_dn7) * assign37570_e42375) - (assign37570_e42372 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign37570_e42375 * assign37570_e42375)), ((((-locals.var_q_qsq__blk825_dn8) * assign37570_e42375) - (assign37570_e42372 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign37570_e42375 * assign37570_e42375)), ((((-locals.var_q_qsq__blk825_dn9) * assign37570_e42375) - (assign37570_e42372 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign37570_e42375 * assign37570_e42375)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign37570_e42378;
        locals.var_q_sh_term__blk833_dn4 = assign37570_e42378_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign37570_e42378_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign37570_e42378_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign37570_e42378_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign37570_e42378_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign37580_e42388, assign37580_e42388_d_n4, assign37580_e42388_d_n6, assign37580_e42388_d_n7, assign37580_e42388_d_n8, assign37580_e42388_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1185 == 0.0)) && (locals.var_guard1186 != 0.0)) {
        let assign37580_e42386: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign37580_e42386, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign37580_e42388;
        locals.var_q_ln_term__blk834_dn4 = assign37580_e42388_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign37580_e42388_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign37580_e42388_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign37580_e42388_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign37580_e42388_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let (assign37590_e42414, assign37590_e42414_d_n4, assign37590_e42414_d_n6, assign37590_e42414_d_n7, assign37590_e42414_d_n8, assign37590_e42414_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1185 == 0.0)) && (locals.var_guard1186 == 0.0)) {
        let assign37590_e42399: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign37590_e42403: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign37590_e42407: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign37590_e42408: f64 = (1.0 - assign37590_e42407);
        let assign37590_e42409: f64 = (assign37590_e42403 * assign37590_e42408);
        let assign37590_e42410: f64 = (1.0 - assign37590_e42409);
        let assign37590_e42411: f64 = (assign37590_e42399 * assign37590_e42410);
        let assign37590_e42412: f64 = (4.0 - assign37590_e42411);
        (assign37590_e42412, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign37590_e42410) + (assign37590_e42399 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign37590_e42408) + (assign37590_e42403 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign37590_e42410) + (assign37590_e42399 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign37590_e42408) + (assign37590_e42403 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign37590_e42410) + (assign37590_e42399 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign37590_e42408) + (assign37590_e42403 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign37590_e42410) + (assign37590_e42399 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign37590_e42408) + (assign37590_e42403 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign37590_e42410) + (assign37590_e42399 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign37590_e42408) + (assign37590_e42403 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign37590_e42414;
        locals.var_q_sh_term__blk833_dn4 = assign37590_e42414_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign37590_e42414_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign37590_e42414_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign37590_e42414_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign37590_e42414_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign37600_e42425, assign37600_e42425_d_n4, assign37600_e42425_d_n6, assign37600_e42425_d_n7, assign37600_e42425_d_n8, assign37600_e42425_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1185 == 0.0)) && (locals.var_guard1186 == 0.0)) {
        let assign37600_e42423: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign37600_e42423, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign37600_e42425;
        locals.var_q_ln_term__blk834_dn4 = assign37600_e42425_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign37600_e42425_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign37600_e42425_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign37600_e42425_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign37600_e42425_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let assign37610_e42428: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign37610_e42430: f64 = (assign37610_e42428 + locals.var_q_qcoth__blk829);
        let assign37610_e42432: f64 = if assign37610_e42430 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1187 = assign37610_e42432;
        locals.var_guard1187_rv = 0.0;

        let (assign37620_e42440, assign37620_e42440_d_n4, assign37620_e42440_d_n6, assign37620_e42440_d_n7, assign37620_e42440_d_n8, assign37620_e42440_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 != 0.0)) {
        let assign37620_e42438: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign37620_e42438, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign37620_e42440;
        locals.var_q_expnum__blk837_dn4 = assign37620_e42440_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign37620_e42440_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign37620_e42440_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign37620_e42440_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign37620_e42440_d_n9;
        locals.var_q_expnum__blk837_rv = 0.0;

        let (assign37630_e42448, assign37630_e42448_d_n4, assign37630_e42448_d_n6, assign37630_e42448_d_n7, assign37630_e42448_d_n8, assign37630_e42448_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 != 0.0)) {
        let assign37630_e42446: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign37630_e42446, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign37630_e42448;
        locals.var_q_d1_expnum__blk838_dn4 = assign37630_e42448_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign37630_e42448_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign37630_e42448_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign37630_e42448_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign37630_e42448_d_n9;
        locals.var_q_d1_expnum__blk838_rv = 0.0;

        let (assign37640_e42454, assign37640_e42454_d_n4, assign37640_e42454_d_n6, assign37640_e42454_d_n7, assign37640_e42454_d_n8, assign37640_e42454_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign37640_e42454;
        locals.var_q_d2_expnum__blk839_dn4 = assign37640_e42454_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign37640_e42454_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign37640_e42454_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign37640_e42454_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign37640_e42454_d_n9;
        locals.var_q_d2_expnum__blk839_rv = 0.0;

        let (assign37650_e42465, assign37650_e42465_d_n4, assign37650_e42465_d_n6, assign37650_e42465_d_n7, assign37650_e42465_d_n8, assign37650_e42465_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
        let assign37650_e42462: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign37650_e42463: f64 = (1.0 / assign37650_e42462);
        (assign37650_e42463, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign37650_e42462 * assign37650_e42462))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign37650_e42462 * assign37650_e42462))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign37650_e42462 * assign37650_e42462))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign37650_e42462 * assign37650_e42462))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign37650_e42462 * assign37650_e42462))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37650_e42465;
        locals.var_q_temp2__blk815_dn4 = assign37650_e42465_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37650_e42465_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37650_e42465_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37650_e42465_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37650_e42465_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign37660_e42474, assign37660_e42474_d_n4, assign37660_e42474_d_n6, assign37660_e42474_d_n7, assign37660_e42474_d_n8, assign37660_e42474_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
        let assign37660_e42472: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign37660_e42472, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign37660_e42474;
        locals.var_q_temp3__blk816_dn4 = assign37660_e42474_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign37660_e42474_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign37660_e42474_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign37660_e42474_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign37660_e42474_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign37670_e42485, assign37670_e42485_d_n4, assign37670_e42485_d_n6, assign37670_e42485_d_n7, assign37670_e42485_d_n8, assign37670_e42485_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
        let assign37670_e42481: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign37670_e42483: f64 = (assign37670_e42481 * locals.var_q_temp2__blk815);
        (assign37670_e42483, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign37670_e42481 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign37670_e42481 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign37670_e42481 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign37670_e42481 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign37670_e42481 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign37670_e42485;
        locals.var_q_expnum__blk837_dn4 = assign37670_e42485_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign37670_e42485_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign37670_e42485_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign37670_e42485_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign37670_e42485_d_n9;
        locals.var_q_expnum__blk837_rv = 0.0;

        let (assign37680_e42502, assign37680_e42502_d_n4, assign37680_e42502_d_n6, assign37680_e42502_d_n7, assign37680_e42502_d_n8, assign37680_e42502_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
        let assign37680_e42492: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign37680_e42494: f64 = (assign37680_e42492 - locals.var_q_aexp__blk824);
        let assign37680_e42497: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign37680_e42498: f64 = (assign37680_e42494 - assign37680_e42497);
        let assign37680_e42500: f64 = (assign37680_e42498 * locals.var_q_temp2__blk815);
        (assign37680_e42500, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign37680_e42498 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign37680_e42498 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign37680_e42498 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign37680_e42498 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign37680_e42498 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign37680_e42502;
        locals.var_q_d1_expnum__blk838_dn4 = assign37680_e42502_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign37680_e42502_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign37680_e42502_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign37680_e42502_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign37680_e42502_d_n9;
        locals.var_q_d1_expnum__blk838_rv = 0.0;

        let (assign37690_e42529, assign37690_e42529_d_n4, assign37690_e42529_d_n6, assign37690_e42529_d_n7, assign37690_e42529_d_n8, assign37690_e42529_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
        let assign37690_e42509: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign37690_e42512: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign37690_e42514: f64 = (assign37690_e42512 * locals.var_q_d1_expnum__blk838);
        let assign37690_e42515: f64 = (assign37690_e42509 + assign37690_e42514);
        let assign37690_e42517: f64 = (assign37690_e42515 + locals.var_q_aexp__blk824);
        let assign37690_e42521: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign37690_e42522: f64 = (locals.var_q_d2_ln__blk836 + assign37690_e42521);
        let assign37690_e42524: f64 = (assign37690_e42522 * locals.var_q_sh_term__blk833);
        let assign37690_e42525: f64 = (assign37690_e42517 - assign37690_e42524);
        let assign37690_e42527: f64 = (assign37690_e42525 * locals.var_q_temp2__blk815);
        (assign37690_e42527, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign37690_e42512 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign37690_e42522 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign37690_e42525 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign37690_e42512 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign37690_e42522 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign37690_e42525 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign37690_e42512 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign37690_e42522 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign37690_e42525 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign37690_e42512 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign37690_e42522 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign37690_e42525 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign37690_e42512 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign37690_e42522 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign37690_e42525 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign37690_e42529;
        locals.var_q_d2_expnum__blk839_dn4 = assign37690_e42529_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign37690_e42529_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign37690_e42529_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign37690_e42529_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign37690_e42529_d_n9;
        locals.var_q_d2_expnum__blk839_rv = 0.0;

        let assign37700_e42532: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1188 = assign37700_e42532;
        locals.var_guard1188_rv = 0.0;

        let (assign37710_e42539, assign37710_e42539_d_n4, assign37710_e42539_d_n6, assign37710_e42539_d_n7, assign37710_e42539_d_n8, assign37710_e42539_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
        let assign37710_e42537: f64 = (locals.var_q_expnum__blk837).ln();
        (assign37710_e42537, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign37710_e42539;
        locals.var_q_lnexpnum__blk840_dn4 = assign37710_e42539_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign37710_e42539_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign37710_e42539_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign37710_e42539_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign37710_e42539_d_n9;
        locals.var_q_lnexpnum__blk840_rv = 0.0;

        let (assign37720_e42547, assign37720_e42547_d_n4, assign37720_e42547_d_n6, assign37720_e42547_d_n7, assign37720_e42547_d_n8, assign37720_e42547_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
        let assign37720_e42545: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign37720_e42545, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37720_e42547;
        locals.var_q_temp1__blk814_dn4 = assign37720_e42547_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37720_e42547_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37720_e42547_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37720_e42547_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37720_e42547_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign37730_e42555, assign37730_e42555_d_n4, assign37730_e42555_d_n6, assign37730_e42555_d_n7, assign37730_e42555_d_n8, assign37730_e42555_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
        let assign37730_e42553: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign37730_e42553, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign37730_e42555;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign37730_e42555_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign37730_e42555_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign37730_e42555_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign37730_e42555_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign37730_e42555_d_n9;
        locals.var_q_d1_lnexpnum__blk841_rv = 0.0;

        let (assign37740_e42567, assign37740_e42567_d_n4, assign37740_e42567_d_n6, assign37740_e42567_d_n7, assign37740_e42567_d_n8, assign37740_e42567_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
        let assign37740_e42561: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign37740_e42564: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign37740_e42565: f64 = (assign37740_e42561 - assign37740_e42564);
        (assign37740_e42565, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign37740_e42567;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign37740_e42567_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign37740_e42567_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign37740_e42567_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign37740_e42567_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign37740_e42567_d_n9;
        locals.var_q_d2_lnexpnum__blk842_rv = 0.0;

        let (assign37750_e42580, assign37750_e42580_d_n4, assign37750_e42580_d_n6, assign37750_e42580_d_n7, assign37750_e42580_d_n8, assign37750_e42580_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
        let assign37750_e42574: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign37750_e42576: f64 = (-locals.var_q_k1q1__blk823);
        let assign37750_e42577: f64 = (assign37750_e42576).ln();
        let assign37750_e42578: f64 = (assign37750_e42574 + assign37750_e42577);
        (assign37750_e42578, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign37750_e42576)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign37750_e42576)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign37750_e42576)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign37750_e42576)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign37750_e42576)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign37750_e42580;
        locals.var_q_lnexpnum__blk840_dn4 = assign37750_e42580_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign37750_e42580_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign37750_e42580_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign37750_e42580_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign37750_e42580_d_n9;
        locals.var_q_lnexpnum__blk840_rv = 0.0;

        let (assign37760_e42589, assign37760_e42589_d_n4, assign37760_e42589_d_n6, assign37760_e42589_d_n7, assign37760_e42589_d_n8, assign37760_e42589_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
        let assign37760_e42587: f64 = (1.0 / locals.var_q1d__blk1001);
        (assign37760_e42587, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37760_e42589;
        locals.var_q_temp1__blk814_dn4 = assign37760_e42589_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37760_e42589_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37760_e42589_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37760_e42589_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37760_e42589_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign37770_e42598, assign37770_e42598_d_n4, assign37770_e42598_d_n6, assign37770_e42598_d_n7, assign37770_e42598_d_n8, assign37770_e42598_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
        let assign37770_e42596: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign37770_e42596, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign37770_e42598;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign37770_e42598_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign37770_e42598_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign37770_e42598_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign37770_e42598_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign37770_e42598_d_n9;
        locals.var_q_d1_lnexpnum__blk841_rv = 0.0;

        let (assign37780_e42608, assign37780_e42608_d_n4, assign37780_e42608_d_n6, assign37780_e42608_d_n7, assign37780_e42608_d_n8, assign37780_e42608_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
        let assign37780_e42604: f64 = (-locals.var_q_temp1__blk814);
        let assign37780_e42606: f64 = (assign37780_e42604 * locals.var_q_temp1__blk814);
        (assign37780_e42606, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign37780_e42604 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign37780_e42604 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign37780_e42604 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign37780_e42604 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign37780_e42604 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign37780_e42608;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign37780_e42608_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign37780_e42608_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign37780_e42608_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign37780_e42608_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign37780_e42608_d_n9;
        locals.var_q_d2_lnexpnum__blk842_rv = 0.0;

        let (assign37790_e42622, assign37790_e42622_d_n4, assign37790_e42622_d_n6, assign37790_e42622_d_n7, assign37790_e42622_d_n8, assign37790_e42622_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37790_e42612: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign37790_e42614: f64 = (assign37790_e42612 + locals.var_q1d__blk1001);
        let assign37790_e42617: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign37790_e42618: f64 = (assign37790_e42614 + assign37790_e42617);
        let assign37790_e42620: f64 = (assign37790_e42618 - locals.var_q_ln_term__blk834);
        (assign37790_e42620, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign37790_e42622;
        locals.var_q_q2_int__blk843_dn4 = assign37790_e42622_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign37790_e42622_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign37790_e42622_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign37790_e42622_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign37790_e42622_d_n9;
        locals.var_q_q2_int__blk843_rv = 0.0;

        let (assign37800_e42632, assign37800_e42632_d_n4, assign37800_e42632_d_n6, assign37800_e42632_d_n7, assign37800_e42632_d_n8, assign37800_e42632_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37800_e42627: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign37800_e42628: f64 = (1.0 + assign37800_e42627);
        let assign37800_e42630: f64 = (assign37800_e42628 - locals.var_q_d1_ln__blk835);
        (assign37800_e42630, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign37800_e42632;
        locals.var_q_d1_q2__blk844_dn4 = assign37800_e42632_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign37800_e42632_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign37800_e42632_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign37800_e42632_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign37800_e42632_d_n9;
        locals.var_q_d1_q2__blk844_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_109(
        locals: &mut StampLocals,
    ) {
        let (assign37810_e42640, assign37810_e42640_d_n4, assign37810_e42640_d_n6, assign37810_e42640_d_n7, assign37810_e42640_d_n8, assign37810_e42640_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37810_e42636: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign37810_e42638: f64 = (assign37810_e42636 - locals.var_q_d2_ln__blk836);
        (assign37810_e42638, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign37810_e42640;
        locals.var_q_d2_q2__blk845_dn4 = assign37810_e42640_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign37810_e42640_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign37810_e42640_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign37810_e42640_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign37810_e42640_d_n9;
        locals.var_q_d2_q2__blk845_rv = 0.0;

        let (assign37820_e42648, assign37820_e42648_d_n4, assign37820_e42648_d_n6, assign37820_e42648_d_n7, assign37820_e42648_d_n8, assign37820_e42648_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37820_e42645: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign37820_e42646: f64 = (locals.var_q_k1q1__blk823 + assign37820_e42645);
        (assign37820_e42646, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign37820_e42648;
        locals.var_q_qi_int__blk846_dn4 = assign37820_e42648_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign37820_e42648_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign37820_e42648_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign37820_e42648_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign37820_e42648_d_n9;
        locals.var_q_qi_int__blk846_rv = 0.0;

        let (assign37830_e42656, assign37830_e42656_d_n4, assign37830_e42656_d_n6, assign37830_e42656_d_n7, assign37830_e42656_d_n8, assign37830_e42656_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37830_e42653: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign37830_e42654: f64 = (locals.var_k1__blk932 + assign37830_e42653);
        (assign37830_e42654, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign37830_e42656;
        locals.var_q_d1_qi__blk847_dn4 = assign37830_e42656_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign37830_e42656_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign37830_e42656_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign37830_e42656_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign37830_e42656_d_n9;
        locals.var_q_d1_qi__blk847_rv = 0.0;

        let (assign37840_e42662, assign37840_e42662_d_n4, assign37840_e42662_d_n6, assign37840_e42662_d_n7, assign37840_e42662_d_n8, assign37840_e42662_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37840_e42660: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign37840_e42660, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign37840_e42662;
        locals.var_q_d2_qi__blk848_dn4 = assign37840_e42662_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign37840_e42662_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign37840_e42662_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign37840_e42662_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign37840_e42662_d_n9;
        locals.var_q_d2_qi__blk848_rv = 0.0;

        let (assign37850_e42670, assign37850_e42670_d_n4, assign37850_e42670_d_n6, assign37850_e42670_d_n7, assign37850_e42670_d_n8, assign37850_e42670_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37850_e42666: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign37850_e42668: f64 = (assign37850_e42666 - locals.var_q_aexp__blk824);
        (assign37850_e42668, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign37850_e42670;
        locals.var_q_zero__blk849_dn4 = assign37850_e42670_d_n4;
        locals.var_q_zero__blk849_dn6 = assign37850_e42670_d_n6;
        locals.var_q_zero__blk849_dn7 = assign37850_e42670_d_n7;
        locals.var_q_zero__blk849_dn8 = assign37850_e42670_d_n8;
        locals.var_q_zero__blk849_dn9 = assign37850_e42670_d_n9;
        locals.var_q_zero__blk849_rv = 0.0;

        let (assign37860_e42682, assign37860_e42682_d_n4, assign37860_e42682_d_n6, assign37860_e42682_d_n7, assign37860_e42682_d_n8, assign37860_e42682_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37860_e42674: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign37860_e42677: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign37860_e42678: f64 = (assign37860_e42674 + assign37860_e42677);
        let assign37860_e42680: f64 = (assign37860_e42678 + locals.var_q_aexp__blk824);
        (assign37860_e42680, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign37860_e42682;
        locals.var_q_d1_zero__blk850_dn4 = assign37860_e42682_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign37860_e42682_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign37860_e42682_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign37860_e42682_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign37860_e42682_d_n9;
        locals.var_q_d1_zero__blk850_rv = 0.0;

        let (assign37870_e42700, assign37870_e42700_d_n4, assign37870_e42700_d_n6, assign37870_e42700_d_n7, assign37870_e42700_d_n8, assign37870_e42700_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37870_e42686: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign37870_e42689: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign37870_e42691: f64 = (assign37870_e42689 * locals.var_q_d1_expnum__blk838);
        let assign37870_e42692: f64 = (assign37870_e42686 + assign37870_e42691);
        let assign37870_e42695: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign37870_e42696: f64 = (assign37870_e42692 + assign37870_e42695);
        let assign37870_e42698: f64 = (assign37870_e42696 - locals.var_q_aexp__blk824);
        (assign37870_e42698, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign37870_e42689 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign37870_e42689 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign37870_e42689 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign37870_e42689 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign37870_e42689 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign37870_e42700;
        locals.var_q_d2_zero__blk851_dn4 = assign37870_e42700_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign37870_e42700_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign37870_e42700_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign37870_e42700_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign37870_e42700_d_n9;
        locals.var_q_d2_zero__blk851_rv = 0.0;

        let (assign37880_e42712, assign37880_e42712_d_n4, assign37880_e42712_d_n6, assign37880_e42712_d_n7, assign37880_e42712_d_n8, assign37880_e42712_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37880_e42704: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign37880_e42707: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign37880_e42709: f64 = (assign37880_e42707 * locals.var_q_d2_zero__blk851);
        let assign37880_e42710: f64 = (assign37880_e42704 - assign37880_e42709);
        (assign37880_e42710, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign37880_e42707 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign37880_e42707 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign37880_e42707 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign37880_e42707 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign37880_e42707 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign37880_e42712;
        locals.var_q_temp__blk860_dn4 = assign37880_e42712_d_n4;
        locals.var_q_temp__blk860_dn6 = assign37880_e42712_d_n6;
        locals.var_q_temp__blk860_dn7 = assign37880_e42712_d_n7;
        locals.var_q_temp__blk860_dn8 = assign37880_e42712_d_n8;
        locals.var_q_temp__blk860_dn9 = assign37880_e42712_d_n9;
        locals.var_q_temp__blk860_rv = 0.0;

        let (assign37890_e42727, assign37890_e42727_d_n4, assign37890_e42727_d_n6, assign37890_e42727_d_n7, assign37890_e42727_d_n8, assign37890_e42727_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37890_e42715: f64 = (-locals.var_q_zero__blk849);
        let assign37890_e42717: f64 = (assign37890_e42715 * locals.var_q_d1_zero__blk850);
        let assign37890_e42719: f64 = (assign37890_e42717 * locals.var_q_temp__blk860);
        let assign37890_e42722: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign37890_e42724: f64 = (assign37890_e42722 + 1e-200);
        let assign37890_e42725: f64 = (assign37890_e42719 / assign37890_e42724);
        (assign37890_e42725, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign37890_e42715 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign37890_e42717 * locals.var_q_temp__blk860_dn4)) * assign37890_e42724) - (assign37890_e42719 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign37890_e42724 * assign37890_e42724)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign37890_e42715 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign37890_e42717 * locals.var_q_temp__blk860_dn6)) * assign37890_e42724) - (assign37890_e42719 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign37890_e42724 * assign37890_e42724)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign37890_e42715 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign37890_e42717 * locals.var_q_temp__blk860_dn7)) * assign37890_e42724) - (assign37890_e42719 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign37890_e42724 * assign37890_e42724)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign37890_e42715 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign37890_e42717 * locals.var_q_temp__blk860_dn8)) * assign37890_e42724) - (assign37890_e42719 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign37890_e42724 * assign37890_e42724)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign37890_e42715 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign37890_e42717 * locals.var_q_temp__blk860_dn9)) * assign37890_e42724) - (assign37890_e42719 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign37890_e42724 * assign37890_e42724)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign37890_e42727;
        locals.var_q_eps2__blk852_dn4 = assign37890_e42727_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign37890_e42727_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign37890_e42727_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign37890_e42727_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign37890_e42727_d_n9;
        locals.var_q_eps2__blk852_rv = 0.0;

        let (assign37900_e42733, assign37900_e42733_d_n4, assign37900_e42733_d_n6, assign37900_e42733_d_n7, assign37900_e42733_d_n8, assign37900_e42733_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37900_e42731: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
        (assign37900_e42731, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign37900_e42733;
        locals.var_q1d__blk1001_dn4 = assign37900_e42733_d_n4;
        locals.var_q1d__blk1001_dn6 = assign37900_e42733_d_n6;
        locals.var_q1d__blk1001_dn7 = assign37900_e42733_d_n7;
        locals.var_q1d__blk1001_dn8 = assign37900_e42733_d_n8;
        locals.var_q1d__blk1001_dn9 = assign37900_e42733_d_n9;
        locals.var_q1d__blk1001_rv = 0.0;

        let (assign37910_e42739, assign37910_e42739_d_n4, assign37910_e42739_d_n6, assign37910_e42739_d_n7, assign37910_e42739_d_n8, assign37910_e42739_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37910_e42737: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign37910_e42737, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign37910_e42739;
        locals.var_q_k1q1__blk823_dn4 = assign37910_e42739_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign37910_e42739_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign37910_e42739_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign37910_e42739_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign37910_e42739_d_n9;
        locals.var_q_k1q1__blk823_rv = 0.0;

        let assign37920_e42742: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37920_e42744: f64 = (assign37920_e42742 - locals.var_xdeff__blk1000);
        let assign37920_e42746: f64 = if assign37920_e42744 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1189 = assign37920_e42746;
        locals.var_guard1189_rv = 0.0;

        let (assign37930_e42757, assign37930_e42757_d_n4, assign37930_e42757_d_n6, assign37930_e42757_d_n7, assign37930_e42757_d_n8, assign37930_e42757_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1189 != 0.0)) {
        let assign37930_e42752: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37930_e42754: f64 = (assign37930_e42752 - locals.var_xdeff__blk1000);
        let assign37930_e42755: f64 = (assign37930_e42754).exp();
        (assign37930_e42755, (assign37930_e42755 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign37930_e42755 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign37930_e42755 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign37930_e42755 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign37930_e42755 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37930_e42757;
        locals.var_q_temp1__blk814_dn4 = assign37930_e42757_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37930_e42757_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37930_e42757_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37930_e42757_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37930_e42757_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign37940_e42798, assign37940_e42798_d_n4, assign37940_e42798_d_n6, assign37940_e42798_d_n7, assign37940_e42798_d_n8, assign37940_e42798_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1189 == 0.0)) {
        let assign37940_e42766: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37940_e42768: f64 = (assign37940_e42766 - locals.var_xdeff__blk1000);
        let assign37940_e42770: f64 = (assign37940_e42768 - 80.0);
        let assign37940_e42775: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37940_e42777: f64 = (assign37940_e42775 - locals.var_xdeff__blk1000);
        let assign37940_e42779: f64 = (assign37940_e42777 - 80.0);
        let assign37940_e42780: f64 = (0.5 * assign37940_e42779);
        let assign37940_e42784: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37940_e42786: f64 = (assign37940_e42784 - locals.var_xdeff__blk1000);
        let assign37940_e42788: f64 = (assign37940_e42786 - 80.0);
        let assign37940_e42790: f64 = (assign37940_e42788 * 0.3333333333333);
        let assign37940_e42791: f64 = (1.0 + assign37940_e42790);
        let assign37940_e42792: f64 = (assign37940_e42780 * assign37940_e42791);
        let assign37940_e42793: f64 = (1.0 + assign37940_e42792);
        let assign37940_e42794: f64 = (assign37940_e42770 * assign37940_e42793);
        let assign37940_e42795: f64 = (1.0 + assign37940_e42794);
        let assign37940_e42796: f64 = (5.54062e34 * assign37940_e42795);
        (assign37940_e42796, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign37940_e42793) + (assign37940_e42770 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign37940_e42791) + (assign37940_e42780 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign37940_e42793) + (assign37940_e42770 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign37940_e42791) + (assign37940_e42780 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign37940_e42793) + (assign37940_e42770 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign37940_e42791) + (assign37940_e42780 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign37940_e42793) + (assign37940_e42770 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign37940_e42791) + (assign37940_e42780 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign37940_e42793) + (assign37940_e42770 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign37940_e42791) + (assign37940_e42780 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37940_e42798;
        locals.var_q_temp1__blk814_dn4 = assign37940_e42798_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37940_e42798_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37940_e42798_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37940_e42798_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37940_e42798_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign37950_e42804, assign37950_e42804_d_n4, assign37950_e42804_d_n6, assign37950_e42804_d_n7, assign37950_e42804_d_n8, assign37950_e42804_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37950_e42802: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign37950_e42802, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign37950_e42804;
        locals.var_q_aexp__blk824_dn4 = assign37950_e42804_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign37950_e42804_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign37950_e42804_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign37950_e42804_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign37950_e42804_d_n9;
        locals.var_q_aexp__blk824_rv = 0.0;

        let (assign37960_e42812, assign37960_e42812_d_n4, assign37960_e42812_d_n6, assign37960_e42812_d_n7, assign37960_e42812_d_n8, assign37960_e42812_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37960_e42808: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign37960_e42810: f64 = (assign37960_e42808 - locals.var_q_aexp__blk824);
        (assign37960_e42810, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign37960_e42812;
        locals.var_q_qsq__blk825_dn4 = assign37960_e42812_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign37960_e42812_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign37960_e42812_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign37960_e42812_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign37960_e42812_d_n9;
        locals.var_q_qsq__blk825_rv = 0.0;

        let (assign37970_e42822, assign37970_e42822_d_n4, assign37970_e42822_d_n6, assign37970_e42822_d_n7, assign37970_e42822_d_n8, assign37970_e42822_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37970_e42816: f64 = (2.0 * locals.var_k1__blk932);
        let assign37970_e42818: f64 = (assign37970_e42816 * locals.var_q_k1q1__blk823);
        let assign37970_e42820: f64 = (assign37970_e42818 + locals.var_q_aexp__blk824);
        (assign37970_e42820, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign37970_e42816 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign37970_e42816 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign37970_e42816 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign37970_e42816 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign37970_e42816 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign37970_e42822;
        locals.var_q_d1_qsq__blk826_dn4 = assign37970_e42822_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign37970_e42822_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign37970_e42822_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign37970_e42822_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign37970_e42822_d_n9;
        locals.var_q_d1_qsq__blk826_rv = 0.0;

        let (assign37980_e42832, assign37980_e42832_d_n4, assign37980_e42832_d_n6, assign37980_e42832_d_n7, assign37980_e42832_d_n8, assign37980_e42832_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37980_e42826: f64 = (2.0 * locals.var_k1__blk932);
        let assign37980_e42828: f64 = (assign37980_e42826 * locals.var_k1__blk932);
        let assign37980_e42830: f64 = (assign37980_e42828 - locals.var_q_aexp__blk824);
        (assign37980_e42830, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign37980_e42826 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign37980_e42826 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign37980_e42826 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign37980_e42826 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign37980_e42826 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign37980_e42832;
        locals.var_q_d2_qsq__blk827_dn4 = assign37980_e42832_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign37980_e42832_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign37980_e42832_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign37980_e42832_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign37980_e42832_d_n9;
        locals.var_q_d2_qsq__blk827_rv = 0.0;

        let assign37990_e42835: f64 = (-0.005);
        let assign37990_e42836: f64 = if locals.var_q_qsq__blk825 < assign37990_e42835 { 1.0 } else { 0.0 };
        locals.var_guard1190 = assign37990_e42836;
        locals.var_guard1190_rv = 0.0;

        let (assign38000_e42844, assign38000_e42844_d_n4, assign38000_e42844_d_n6, assign38000_e42844_d_n7, assign38000_e42844_d_n8, assign38000_e42844_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38000_e42841: f64 = (locals.var_q_qsq__blk825).abs();
        let assign38000_e42842: f64 = (assign38000_e42841).sqrt();
        (assign38000_e42842, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38000_e42842)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38000_e42842)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38000_e42842)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38000_e42842)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38000_e42842)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign38000_e42844;
        locals.var_q_rac_qsq__blk828_dn4 = assign38000_e42844_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign38000_e42844_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign38000_e42844_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign38000_e42844_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign38000_e42844_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign38010_e42855, assign38010_e42855_d_n4, assign38010_e42855_d_n6, assign38010_e42855_d_n7, assign38010_e42855_d_n8, assign38010_e42855_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38010_e42851: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign38010_e42852: f64 = (assign38010_e42851).tan();
        let assign38010_e42853: f64 = (locals.var_q_rac_qsq__blk828 / assign38010_e42852);
        (assign38010_e42853, (((locals.var_q_rac_qsq__blk828_dn4 * assign38010_e42852) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign38010_e42851).cos() * (assign38010_e42851).cos())))) / (assign38010_e42852 * assign38010_e42852)), (((locals.var_q_rac_qsq__blk828_dn6 * assign38010_e42852) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign38010_e42851).cos() * (assign38010_e42851).cos())))) / (assign38010_e42852 * assign38010_e42852)), (((locals.var_q_rac_qsq__blk828_dn7 * assign38010_e42852) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign38010_e42851).cos() * (assign38010_e42851).cos())))) / (assign38010_e42852 * assign38010_e42852)), (((locals.var_q_rac_qsq__blk828_dn8 * assign38010_e42852) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign38010_e42851).cos() * (assign38010_e42851).cos())))) / (assign38010_e42852 * assign38010_e42852)), (((locals.var_q_rac_qsq__blk828_dn9 * assign38010_e42852) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign38010_e42851).cos() * (assign38010_e42851).cos())))) / (assign38010_e42852 * assign38010_e42852)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38010_e42855;
        locals.var_q_qcoth__blk829_dn4 = assign38010_e42855_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38010_e42855_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38010_e42855_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38010_e42855_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38010_e42855_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign38020_e42865, assign38020_e42865_d_n4, assign38020_e42865_d_n6, assign38020_e42865_d_n7, assign38020_e42865_d_n8, assign38020_e42865_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38020_e42861: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign38020_e42863: f64 = (assign38020_e42861 / locals.var_q_qsq__blk825);
        (assign38020_e42863, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38020_e42861 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38020_e42861 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38020_e42861 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38020_e42861 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38020_e42861 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38020_e42865;
        locals.var_q_temp1__blk814_dn4 = assign38020_e42865_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38020_e42865_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38020_e42865_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38020_e42865_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38020_e42865_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign38030_e42879, assign38030_e42879_d_n4, assign38030_e42879_d_n6, assign38030_e42879_d_n7, assign38030_e42879_d_n8, assign38030_e42879_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38030_e42873: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign38030_e42874: f64 = (locals.var_q_qcoth__blk829 * assign38030_e42873);
        let assign38030_e42875: f64 = (locals.var_q_qsq__blk825 + assign38030_e42874);
        let assign38030_e42877: f64 = (assign38030_e42875 * locals.var_q_temp1__blk814);
        (assign38030_e42877, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38030_e42873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38030_e42875 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38030_e42873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38030_e42875 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38030_e42873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38030_e42875 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38030_e42873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38030_e42875 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38030_e42873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38030_e42875 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign38030_e42879;
        locals.var_q_d1_qcoth__blk830_dn4 = assign38030_e42879_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign38030_e42879_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign38030_e42879_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign38030_e42879_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign38030_e42879_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign38040_e42901, assign38040_e42901_d_n4, assign38040_e42901_d_n6, assign38040_e42901_d_n7, assign38040_e42901_d_n8, assign38040_e42901_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38040_e42886: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign38040_e42889: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign38040_e42890: f64 = (assign38040_e42886 * assign38040_e42889);
        let assign38040_e42891: f64 = (locals.var_q_d1_qsq__blk826 - assign38040_e42890);
        let assign38040_e42893: f64 = (assign38040_e42891 * locals.var_q_temp1__blk814);
        let assign38040_e42896: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign38040_e42898: f64 = (assign38040_e42896 / locals.var_q_d1_qsq__blk826);
        let assign38040_e42899: f64 = (assign38040_e42893 + assign38040_e42898);
        (assign38040_e42899, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38040_e42889) + (assign38040_e42886 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38040_e42891 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38040_e42896 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38040_e42889) + (assign38040_e42886 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38040_e42891 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38040_e42896 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38040_e42889) + (assign38040_e42886 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38040_e42891 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38040_e42896 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38040_e42889) + (assign38040_e42886 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38040_e42891 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38040_e42896 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38040_e42889) + (assign38040_e42886 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38040_e42891 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38040_e42896 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign38040_e42901;
        locals.var_q_d2_qcoth__blk832_dn4 = assign38040_e42901_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign38040_e42901_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign38040_e42901_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign38040_e42901_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign38040_e42901_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign38050_e42911, assign38050_e42911_d_n4, assign38050_e42911_d_n6, assign38050_e42911_d_n7, assign38050_e42911_d_n8, assign38050_e42911_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38050_e42908: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign38050_e42909: f64 = (1.0 - assign38050_e42908);
        (assign38050_e42909, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38050_e42911;
        locals.var_q_temp2__blk815_dn4 = assign38050_e42911_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38050_e42911_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38050_e42911_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38050_e42911_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38050_e42911_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign38060_e42921, assign38060_e42921_d_n4, assign38060_e42921_d_n6, assign38060_e42921_d_n7, assign38060_e42921_d_n8, assign38060_e42921_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38060_e42917: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign38060_e42919: f64 = (assign38060_e42917 * locals.var_q_temp2__blk815);
        (assign38060_e42919, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38060_e42917 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38060_e42917 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38060_e42917 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38060_e42917 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38060_e42917 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign38060_e42921;
        locals.var_q_d1_ln__blk835_dn4 = assign38060_e42921_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign38060_e42921_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign38060_e42921_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign38060_e42921_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign38060_e42921_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign38070_e42939, assign38070_e42939_d_n4, assign38070_e42939_d_n6, assign38070_e42939_d_n7, assign38070_e42939_d_n8, assign38070_e42939_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38070_e42927: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign38070_e42932: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign38070_e42933: f64 = (locals.var_q_d1_ln__blk835 + assign38070_e42932);
        let assign38070_e42934: f64 = (locals.var_q_d1_qsq__blk826 * assign38070_e42933);
        let assign38070_e42935: f64 = (assign38070_e42927 - assign38070_e42934);
        let assign38070_e42937: f64 = (assign38070_e42935 / locals.var_q_qsq__blk825);
        (assign38070_e42937, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38070_e42933) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38070_e42935 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38070_e42933) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38070_e42935 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38070_e42933) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38070_e42935 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38070_e42933) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38070_e42935 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38070_e42933) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38070_e42935 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign38070_e42939;
        locals.var_q_d2_ln__blk836_dn4 = assign38070_e42939_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign38070_e42939_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign38070_e42939_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign38070_e42939_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign38070_e42939_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let assign38080_e42942: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1191 = assign38080_e42942;
        locals.var_guard1191_rv = 0.0;

        let (assign38090_e42953, assign38090_e42953_d_n4, assign38090_e42953_d_n6, assign38090_e42953_d_n7, assign38090_e42953_d_n8, assign38090_e42953_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38090_e42950: f64 = (locals.var_q_qsq__blk825).abs();
        let assign38090_e42951: f64 = (assign38090_e42950).sqrt();
        (assign38090_e42951, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38090_e42951)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38090_e42951)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38090_e42951)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38090_e42951)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38090_e42951)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign38090_e42953;
        locals.var_q_rac_qsq__blk828_dn4 = assign38090_e42953_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign38090_e42953_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign38090_e42953_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign38090_e42953_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign38090_e42953_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_110(
        locals: &mut StampLocals,
    ) {
        let (assign38100_e42964, assign38100_e42964_d_n4, assign38100_e42964_d_n6, assign38100_e42964_d_n7, assign38100_e42964_d_n8, assign38100_e42964_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38100_e42961: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign38100_e42962: f64 = (assign38100_e42961).exp();
        (assign38100_e42962, (assign38100_e42962 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign38100_e42962 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign38100_e42962 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign38100_e42962 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign38100_e42962 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign38100_e42964;
        locals.var_q_invexpq__blk831_dn4 = assign38100_e42964_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign38100_e42964_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign38100_e42964_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign38100_e42964_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign38100_e42964_d_n9;
        locals.var_q_invexpq__blk831_rv = 0.0;

        let (assign38110_e42981, assign38110_e42981_d_n4, assign38110_e42981_d_n6, assign38110_e42981_d_n7, assign38110_e42981_d_n8, assign38110_e42981_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38110_e42974: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign38110_e42975: f64 = (locals.var_q_rac_qsq__blk828 * assign38110_e42974);
        let assign38110_e42978: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign38110_e42979: f64 = (assign38110_e42975 / assign38110_e42978);
        (assign38110_e42979, (((((locals.var_q_rac_qsq__blk828_dn4 * assign38110_e42974) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign38110_e42978) - (assign38110_e42975 * (-locals.var_q_invexpq__blk831_dn4))) / (assign38110_e42978 * assign38110_e42978)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign38110_e42974) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign38110_e42978) - (assign38110_e42975 * (-locals.var_q_invexpq__blk831_dn6))) / (assign38110_e42978 * assign38110_e42978)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign38110_e42974) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign38110_e42978) - (assign38110_e42975 * (-locals.var_q_invexpq__blk831_dn7))) / (assign38110_e42978 * assign38110_e42978)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign38110_e42974) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign38110_e42978) - (assign38110_e42975 * (-locals.var_q_invexpq__blk831_dn8))) / (assign38110_e42978 * assign38110_e42978)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign38110_e42974) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign38110_e42978) - (assign38110_e42975 * (-locals.var_q_invexpq__blk831_dn9))) / (assign38110_e42978 * assign38110_e42978)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38110_e42981;
        locals.var_q_qcoth__blk829_dn4 = assign38110_e42981_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38110_e42981_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38110_e42981_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38110_e42981_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38110_e42981_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign38120_e42994, assign38120_e42994_d_n4, assign38120_e42994_d_n6, assign38120_e42994_d_n7, assign38120_e42994_d_n8, assign38120_e42994_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38120_e42990: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign38120_e42992: f64 = (assign38120_e42990 / locals.var_q_qsq__blk825);
        (assign38120_e42992, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38120_e42990 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38120_e42990 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38120_e42990 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38120_e42990 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38120_e42990 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38120_e42994;
        locals.var_q_temp1__blk814_dn4 = assign38120_e42994_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38120_e42994_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38120_e42994_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38120_e42994_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38120_e42994_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign38130_e43011, assign38130_e43011_d_n4, assign38130_e43011_d_n6, assign38130_e43011_d_n7, assign38130_e43011_d_n8, assign38130_e43011_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38130_e43005: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign38130_e43006: f64 = (locals.var_q_qcoth__blk829 * assign38130_e43005);
        let assign38130_e43007: f64 = (locals.var_q_qsq__blk825 + assign38130_e43006);
        let assign38130_e43009: f64 = (assign38130_e43007 * locals.var_q_temp1__blk814);
        (assign38130_e43009, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38130_e43005) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38130_e43007 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38130_e43005) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38130_e43007 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38130_e43005) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38130_e43007 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38130_e43005) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38130_e43007 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38130_e43005) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38130_e43007 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign38130_e43011;
        locals.var_q_d1_qcoth__blk830_dn4 = assign38130_e43011_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign38130_e43011_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign38130_e43011_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign38130_e43011_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign38130_e43011_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign38140_e43036, assign38140_e43036_d_n4, assign38140_e43036_d_n6, assign38140_e43036_d_n7, assign38140_e43036_d_n8, assign38140_e43036_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38140_e43021: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign38140_e43024: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign38140_e43025: f64 = (assign38140_e43021 * assign38140_e43024);
        let assign38140_e43026: f64 = (locals.var_q_d1_qsq__blk826 - assign38140_e43025);
        let assign38140_e43028: f64 = (assign38140_e43026 * locals.var_q_temp1__blk814);
        let assign38140_e43031: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign38140_e43033: f64 = (assign38140_e43031 / locals.var_q_d1_qsq__blk826);
        let assign38140_e43034: f64 = (assign38140_e43028 + assign38140_e43033);
        (assign38140_e43034, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38140_e43024) + (assign38140_e43021 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38140_e43026 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43031 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38140_e43024) + (assign38140_e43021 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38140_e43026 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43031 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38140_e43024) + (assign38140_e43021 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38140_e43026 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43031 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38140_e43024) + (assign38140_e43021 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38140_e43026 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43031 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38140_e43024) + (assign38140_e43021 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38140_e43026 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43031 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign38140_e43036;
        locals.var_q_d2_qcoth__blk832_dn4 = assign38140_e43036_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign38140_e43036_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign38140_e43036_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign38140_e43036_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign38140_e43036_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign38150_e43049, assign38150_e43049_d_n4, assign38150_e43049_d_n6, assign38150_e43049_d_n7, assign38150_e43049_d_n8, assign38150_e43049_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38150_e43046: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign38150_e43047: f64 = (1.0 - assign38150_e43046);
        (assign38150_e43047, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38150_e43049;
        locals.var_q_temp2__blk815_dn4 = assign38150_e43049_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38150_e43049_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38150_e43049_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38150_e43049_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38150_e43049_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign38160_e43062, assign38160_e43062_d_n4, assign38160_e43062_d_n6, assign38160_e43062_d_n7, assign38160_e43062_d_n8, assign38160_e43062_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38160_e43058: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign38160_e43060: f64 = (assign38160_e43058 * locals.var_q_temp2__blk815);
        (assign38160_e43060, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43058 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43058 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43058 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43058 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43058 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign38160_e43062;
        locals.var_q_d1_ln__blk835_dn4 = assign38160_e43062_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign38160_e43062_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign38160_e43062_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign38160_e43062_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign38160_e43062_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign38170_e43083, assign38170_e43083_d_n4, assign38170_e43083_d_n6, assign38170_e43083_d_n7, assign38170_e43083_d_n8, assign38170_e43083_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38170_e43071: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign38170_e43076: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign38170_e43077: f64 = (locals.var_q_d1_ln__blk835 + assign38170_e43076);
        let assign38170_e43078: f64 = (locals.var_q_d1_qsq__blk826 * assign38170_e43077);
        let assign38170_e43079: f64 = (assign38170_e43071 - assign38170_e43078);
        let assign38170_e43081: f64 = (assign38170_e43079 / locals.var_q_qsq__blk825);
        (assign38170_e43081, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38170_e43077) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38170_e43079 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38170_e43077) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38170_e43079 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38170_e43077) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38170_e43079 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38170_e43077) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38170_e43079 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38170_e43077) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38170_e43079 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign38170_e43083;
        locals.var_q_d2_ln__blk836_dn4 = assign38170_e43083_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign38170_e43083_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign38170_e43083_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign38170_e43083_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign38170_e43083_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let (assign38180_e43111, assign38180_e43111_d_n4, assign38180_e43111_d_n6, assign38180_e43111_d_n7, assign38180_e43111_d_n8, assign38180_e43111_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38180_e43095: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign38180_e43099: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign38180_e43103: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign38180_e43104: f64 = (1.0 - assign38180_e43103);
        let assign38180_e43105: f64 = (assign38180_e43099 * assign38180_e43104);
        let assign38180_e43106: f64 = (1.0 - assign38180_e43105);
        let assign38180_e43107: f64 = (assign38180_e43095 * assign38180_e43106);
        let assign38180_e43108: f64 = (1.0 - assign38180_e43107);
        let assign38180_e43109: f64 = (0.1666666666667 * assign38180_e43108);
        (assign38180_e43109, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign38180_e43106) + (assign38180_e43095 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign38180_e43104) + (assign38180_e43099 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign38180_e43106) + (assign38180_e43095 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign38180_e43104) + (assign38180_e43099 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign38180_e43106) + (assign38180_e43095 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign38180_e43104) + (assign38180_e43099 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign38180_e43106) + (assign38180_e43095 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign38180_e43104) + (assign38180_e43099 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign38180_e43106) + (assign38180_e43095 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign38180_e43104) + (assign38180_e43099 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign38180_e43111;
        locals.var_q_temp3__blk816_dn4 = assign38180_e43111_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign38180_e43111_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign38180_e43111_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign38180_e43111_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign38180_e43111_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign38190_e43125, assign38190_e43125_d_n4, assign38190_e43125_d_n6, assign38190_e43125_d_n7, assign38190_e43125_d_n8, assign38190_e43125_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38190_e43122: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign38190_e43123: f64 = (2.0 + assign38190_e43122);
        (assign38190_e43123, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38190_e43125;
        locals.var_q_qcoth__blk829_dn4 = assign38190_e43125_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38190_e43125_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38190_e43125_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38190_e43125_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38190_e43125_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign38200_e43153, assign38200_e43153_d_n4, assign38200_e43153_d_n6, assign38200_e43153_d_n7, assign38200_e43153_d_n8, assign38200_e43153_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38200_e43137: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign38200_e43141: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign38200_e43145: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign38200_e43146: f64 = (1.0 - assign38200_e43145);
        let assign38200_e43147: f64 = (assign38200_e43141 * assign38200_e43146);
        let assign38200_e43148: f64 = (1.0 - assign38200_e43147);
        let assign38200_e43149: f64 = (assign38200_e43137 * assign38200_e43148);
        let assign38200_e43150: f64 = (1.0 - assign38200_e43149);
        let assign38200_e43151: f64 = (0.1666666666667 * assign38200_e43150);
        (assign38200_e43151, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign38200_e43148) + (assign38200_e43137 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign38200_e43146) + (assign38200_e43141 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign38200_e43148) + (assign38200_e43137 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign38200_e43146) + (assign38200_e43141 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign38200_e43148) + (assign38200_e43137 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign38200_e43146) + (assign38200_e43141 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign38200_e43148) + (assign38200_e43137 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign38200_e43146) + (assign38200_e43141 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign38200_e43148) + (assign38200_e43137 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign38200_e43146) + (assign38200_e43141 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38200_e43153;
        locals.var_q_temp1__blk814_dn4 = assign38200_e43153_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38200_e43153_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38200_e43153_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38200_e43153_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38200_e43153_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign38210_e43165, assign38210_e43165_d_n4, assign38210_e43165_d_n6, assign38210_e43165_d_n7, assign38210_e43165_d_n8, assign38210_e43165_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38210_e43163: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign38210_e43163, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign38210_e43165;
        locals.var_q_d1_qcoth__blk830_dn4 = assign38210_e43165_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign38210_e43165_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign38210_e43165_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign38210_e43165_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign38210_e43165_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign38220_e43193, assign38220_e43193_d_n4, assign38220_e43193_d_n6, assign38220_e43193_d_n7, assign38220_e43193_d_n8, assign38220_e43193_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38220_e43177: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign38220_e43181: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign38220_e43185: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign38220_e43186: f64 = (1.0 - assign38220_e43185);
        let assign38220_e43187: f64 = (assign38220_e43181 * assign38220_e43186);
        let assign38220_e43188: f64 = (1.0 - assign38220_e43187);
        let assign38220_e43189: f64 = (assign38220_e43177 * assign38220_e43188);
        let assign38220_e43190: f64 = (1.0 - assign38220_e43189);
        let assign38220_e43191: f64 = (0.0055555555556 * assign38220_e43190);
        (assign38220_e43191, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign38220_e43188) + (assign38220_e43177 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign38220_e43186) + (assign38220_e43181 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign38220_e43188) + (assign38220_e43177 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign38220_e43186) + (assign38220_e43181 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign38220_e43188) + (assign38220_e43177 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign38220_e43186) + (assign38220_e43181 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign38220_e43188) + (assign38220_e43177 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign38220_e43186) + (assign38220_e43181 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign38220_e43188) + (assign38220_e43177 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign38220_e43186) + (assign38220_e43181 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38220_e43193;
        locals.var_q_temp2__blk815_dn4 = assign38220_e43193_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38220_e43193_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38220_e43193_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38220_e43193_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38220_e43193_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign38230_e43211, assign38230_e43211_d_n4, assign38230_e43211_d_n6, assign38230_e43211_d_n7, assign38230_e43211_d_n8, assign38230_e43211_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38230_e43203: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign38230_e43206: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign38230_e43208: f64 = (assign38230_e43206 * locals.var_q_temp2__blk815);
        let assign38230_e43209: f64 = (assign38230_e43203 - assign38230_e43208);
        (assign38230_e43209, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign38230_e43206 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign38230_e43206 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign38230_e43206 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign38230_e43206 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign38230_e43206 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign38230_e43211;
        locals.var_q_d2_qcoth__blk832_dn4 = assign38230_e43211_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign38230_e43211_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign38230_e43211_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign38230_e43211_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign38230_e43211_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign38240_e43226, assign38240_e43226_d_n4, assign38240_e43226_d_n6, assign38240_e43226_d_n7, assign38240_e43226_d_n8, assign38240_e43226_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38240_e43220: f64 = (-0.5);
        let assign38240_e43222: f64 = (assign38240_e43220 * locals.var_q_d1_qsq__blk826);
        let assign38240_e43224: f64 = (assign38240_e43222 * locals.var_q_temp3__blk816);
        (assign38240_e43224, (((assign38240_e43220 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign38240_e43222 * locals.var_q_temp3__blk816_dn4)), (((assign38240_e43220 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign38240_e43222 * locals.var_q_temp3__blk816_dn6)), (((assign38240_e43220 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign38240_e43222 * locals.var_q_temp3__blk816_dn7)), (((assign38240_e43220 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign38240_e43222 * locals.var_q_temp3__blk816_dn8)), (((assign38240_e43220 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign38240_e43222 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign38240_e43226;
        locals.var_q_d1_ln__blk835_dn4 = assign38240_e43226_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign38240_e43226_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign38240_e43226_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign38240_e43226_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign38240_e43226_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign38250_e43261, assign38250_e43261_d_n4, assign38250_e43261_d_n6, assign38250_e43261_d_n7, assign38250_e43261_d_n8, assign38250_e43261_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38250_e43235: f64 = (-0.5);
        let assign38250_e43237: f64 = (assign38250_e43235 * locals.var_q_d2_qsq__blk827);
        let assign38250_e43239: f64 = (assign38250_e43237 * locals.var_q_temp3__blk816);
        let assign38250_e43242: f64 = (0.25 * 0.0055555555556);
        let assign38250_e43244: f64 = (assign38250_e43242 * locals.var_q_d1_qsq__blk826);
        let assign38250_e43246: f64 = (assign38250_e43244 * locals.var_q_d1_qsq__blk826);
        let assign38250_e43250: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign38250_e43254: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign38250_e43255: f64 = (2.0 - assign38250_e43254);
        let assign38250_e43256: f64 = (assign38250_e43250 * assign38250_e43255);
        let assign38250_e43257: f64 = (1.0 - assign38250_e43256);
        let assign38250_e43258: f64 = (assign38250_e43246 * assign38250_e43257);
        let assign38250_e43259: f64 = (assign38250_e43239 + assign38250_e43258);
        (assign38250_e43259, ((((assign38250_e43235 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign38250_e43237 * locals.var_q_temp3__blk816_dn4)) + (((((assign38250_e43242 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign38250_e43244 * locals.var_q_d1_qsq__blk826_dn4)) * assign38250_e43257) + (assign38250_e43246 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign38250_e43255) + (assign38250_e43250 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign38250_e43235 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign38250_e43237 * locals.var_q_temp3__blk816_dn6)) + (((((assign38250_e43242 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign38250_e43244 * locals.var_q_d1_qsq__blk826_dn6)) * assign38250_e43257) + (assign38250_e43246 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign38250_e43255) + (assign38250_e43250 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign38250_e43235 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign38250_e43237 * locals.var_q_temp3__blk816_dn7)) + (((((assign38250_e43242 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign38250_e43244 * locals.var_q_d1_qsq__blk826_dn7)) * assign38250_e43257) + (assign38250_e43246 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign38250_e43255) + (assign38250_e43250 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign38250_e43235 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign38250_e43237 * locals.var_q_temp3__blk816_dn8)) + (((((assign38250_e43242 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign38250_e43244 * locals.var_q_d1_qsq__blk826_dn8)) * assign38250_e43257) + (assign38250_e43246 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign38250_e43255) + (assign38250_e43250 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign38250_e43235 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign38250_e43237 * locals.var_q_temp3__blk816_dn9)) + (((((assign38250_e43242 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign38250_e43244 * locals.var_q_d1_qsq__blk826_dn9)) * assign38250_e43257) + (assign38250_e43246 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign38250_e43255) + (assign38250_e43250 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign38250_e43261;
        locals.var_q_d2_ln__blk836_dn4 = assign38250_e43261_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign38250_e43261_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign38250_e43261_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign38250_e43261_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign38250_e43261_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let assign38260_e43264: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1192 = assign38260_e43264;
        locals.var_guard1192_rv = 0.0;

        let (assign38270_e43280, assign38270_e43280_d_n4, assign38270_e43280_d_n6, assign38270_e43280_d_n7, assign38270_e43280_d_n8, assign38270_e43280_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign38270_e43270: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign38270_e43275: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign38270_e43276: f64 = (locals.var_q_invexpq__blk831 * assign38270_e43275);
        let assign38270_e43277: f64 = (1.0 - assign38270_e43276);
        let assign38270_e43278: f64 = (assign38270_e43270 / assign38270_e43277);
        (assign38270_e43278, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign38270_e43277) - (assign38270_e43270 * (-((locals.var_q_invexpq__blk831_dn4 * assign38270_e43275) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign38270_e43277 * assign38270_e43277)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign38270_e43277) - (assign38270_e43270 * (-((locals.var_q_invexpq__blk831_dn6 * assign38270_e43275) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign38270_e43277 * assign38270_e43277)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign38270_e43277) - (assign38270_e43270 * (-((locals.var_q_invexpq__blk831_dn7 * assign38270_e43275) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign38270_e43277 * assign38270_e43277)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign38270_e43277) - (assign38270_e43270 * (-((locals.var_q_invexpq__blk831_dn8 * assign38270_e43275) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign38270_e43277 * assign38270_e43277)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign38270_e43277) - (assign38270_e43270 * (-((locals.var_q_invexpq__blk831_dn9 * assign38270_e43275) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign38270_e43277 * assign38270_e43277)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38270_e43280;
        locals.var_q_temp2__blk815_dn4 = assign38270_e43280_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38270_e43280_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38270_e43280_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38270_e43280_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38270_e43280_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign38280_e43288, assign38280_e43288_d_n4, assign38280_e43288_d_n6, assign38280_e43288_d_n7, assign38280_e43288_d_n8, assign38280_e43288_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign38280_e43286: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign38280_e43286, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign38280_e43288;
        locals.var_q_sh_term__blk833_dn4 = assign38280_e43288_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign38280_e43288_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign38280_e43288_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign38280_e43288_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign38280_e43288_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign38290_e43297, assign38290_e43297_d_n4, assign38290_e43297_d_n6, assign38290_e43297_d_n7, assign38290_e43297_d_n8, assign38290_e43297_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign38290_e43293: f64 = (locals.var_q_temp2__blk815).ln();
        let assign38290_e43295: f64 = (assign38290_e43293 - locals.var_q_rac_qsq__blk828);
        (assign38290_e43295, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign38290_e43297;
        locals.var_q_ln_term__blk834_dn4 = assign38290_e43297_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign38290_e43297_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign38290_e43297_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign38290_e43297_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign38290_e43297_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let assign38300_e43300: f64 = (-0.005);
        let assign38300_e43301: f64 = if locals.var_q_qsq__blk825 < assign38300_e43300 { 1.0 } else { 0.0 };
        locals.var_guard1193 = assign38300_e43301;
        locals.var_guard1193_rv = 0.0;

        let (assign38310_e43313, assign38310_e43313_d_n4, assign38310_e43313_d_n6, assign38310_e43313_d_n7, assign38310_e43313_d_n8, assign38310_e43313_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
        let assign38310_e43310: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign38310_e43311: f64 = (assign38310_e43310).sin();
        (assign38310_e43311, ((assign38310_e43310).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign38310_e43310).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign38310_e43310).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign38310_e43310).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign38310_e43310).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38310_e43313;
        locals.var_q_temp2__blk815_dn4 = assign38310_e43313_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38310_e43313_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38310_e43313_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38310_e43313_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38310_e43313_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign38320_e43327, assign38320_e43327_d_n4, assign38320_e43327_d_n6, assign38320_e43327_d_n7, assign38320_e43327_d_n8, assign38320_e43327_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
        let assign38320_e43321: f64 = (-locals.var_q_qsq__blk825);
        let assign38320_e43324: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign38320_e43325: f64 = (assign38320_e43321 / assign38320_e43324);
        (assign38320_e43325, ((((-locals.var_q_qsq__blk825_dn4) * assign38320_e43324) - (assign38320_e43321 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign38320_e43324 * assign38320_e43324)), ((((-locals.var_q_qsq__blk825_dn6) * assign38320_e43324) - (assign38320_e43321 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign38320_e43324 * assign38320_e43324)), ((((-locals.var_q_qsq__blk825_dn7) * assign38320_e43324) - (assign38320_e43321 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign38320_e43324 * assign38320_e43324)), ((((-locals.var_q_qsq__blk825_dn8) * assign38320_e43324) - (assign38320_e43321 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign38320_e43324 * assign38320_e43324)), ((((-locals.var_q_qsq__blk825_dn9) * assign38320_e43324) - (assign38320_e43321 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign38320_e43324 * assign38320_e43324)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign38320_e43327;
        locals.var_q_sh_term__blk833_dn4 = assign38320_e43327_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign38320_e43327_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign38320_e43327_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign38320_e43327_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign38320_e43327_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign38330_e43337, assign38330_e43337_d_n4, assign38330_e43337_d_n6, assign38330_e43337_d_n7, assign38330_e43337_d_n8, assign38330_e43337_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
        let assign38330_e43335: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign38330_e43335, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign38330_e43337;
        locals.var_q_ln_term__blk834_dn4 = assign38330_e43337_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign38330_e43337_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign38330_e43337_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign38330_e43337_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign38330_e43337_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let (assign38340_e43363, assign38340_e43363_d_n4, assign38340_e43363_d_n6, assign38340_e43363_d_n7, assign38340_e43363_d_n8, assign38340_e43363_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 == 0.0)) {
        let assign38340_e43348: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign38340_e43352: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign38340_e43356: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign38340_e43357: f64 = (1.0 - assign38340_e43356);
        let assign38340_e43358: f64 = (assign38340_e43352 * assign38340_e43357);
        let assign38340_e43359: f64 = (1.0 - assign38340_e43358);
        let assign38340_e43360: f64 = (assign38340_e43348 * assign38340_e43359);
        let assign38340_e43361: f64 = (4.0 - assign38340_e43360);
        (assign38340_e43361, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign38340_e43359) + (assign38340_e43348 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign38340_e43357) + (assign38340_e43352 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign38340_e43359) + (assign38340_e43348 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign38340_e43357) + (assign38340_e43352 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign38340_e43359) + (assign38340_e43348 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign38340_e43357) + (assign38340_e43352 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign38340_e43359) + (assign38340_e43348 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign38340_e43357) + (assign38340_e43352 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign38340_e43359) + (assign38340_e43348 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign38340_e43357) + (assign38340_e43352 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign38340_e43363;
        locals.var_q_sh_term__blk833_dn4 = assign38340_e43363_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign38340_e43363_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign38340_e43363_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign38340_e43363_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign38340_e43363_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign38350_e43374, assign38350_e43374_d_n4, assign38350_e43374_d_n6, assign38350_e43374_d_n7, assign38350_e43374_d_n8, assign38350_e43374_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 == 0.0)) {
        let assign38350_e43372: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign38350_e43372, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign38350_e43374;
        locals.var_q_ln_term__blk834_dn4 = assign38350_e43374_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign38350_e43374_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign38350_e43374_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign38350_e43374_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign38350_e43374_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let assign38360_e43377: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign38360_e43379: f64 = (assign38360_e43377 + locals.var_q_qcoth__blk829);
        let assign38360_e43381: f64 = if assign38360_e43379 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1194 = assign38360_e43381;
        locals.var_guard1194_rv = 0.0;

        let (assign38370_e43389, assign38370_e43389_d_n4, assign38370_e43389_d_n6, assign38370_e43389_d_n7, assign38370_e43389_d_n8, assign38370_e43389_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
        let assign38370_e43387: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign38370_e43387, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign38370_e43389;
        locals.var_q_expnum__blk837_dn4 = assign38370_e43389_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign38370_e43389_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign38370_e43389_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign38370_e43389_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign38370_e43389_d_n9;
        locals.var_q_expnum__blk837_rv = 0.0;

        let (assign38380_e43397, assign38380_e43397_d_n4, assign38380_e43397_d_n6, assign38380_e43397_d_n7, assign38380_e43397_d_n8, assign38380_e43397_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
        let assign38380_e43395: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign38380_e43395, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign38380_e43397;
        locals.var_q_d1_expnum__blk838_dn4 = assign38380_e43397_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign38380_e43397_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign38380_e43397_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign38380_e43397_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign38380_e43397_d_n9;
        locals.var_q_d1_expnum__blk838_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_111(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign38390_e43403, assign38390_e43403_d_n4, assign38390_e43403_d_n6, assign38390_e43403_d_n7, assign38390_e43403_d_n8, assign38390_e43403_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign38390_e43403;
        locals.var_q_d2_expnum__blk839_dn4 = assign38390_e43403_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign38390_e43403_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign38390_e43403_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign38390_e43403_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign38390_e43403_d_n9;
        locals.var_q_d2_expnum__blk839_rv = 0.0;

        let (assign38400_e43414, assign38400_e43414_d_n4, assign38400_e43414_d_n6, assign38400_e43414_d_n7, assign38400_e43414_d_n8, assign38400_e43414_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
        let assign38400_e43411: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign38400_e43412: f64 = (1.0 / assign38400_e43411);
        (assign38400_e43412, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign38400_e43411 * assign38400_e43411))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign38400_e43411 * assign38400_e43411))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign38400_e43411 * assign38400_e43411))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign38400_e43411 * assign38400_e43411))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign38400_e43411 * assign38400_e43411))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38400_e43414;
        locals.var_q_temp2__blk815_dn4 = assign38400_e43414_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38400_e43414_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38400_e43414_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38400_e43414_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38400_e43414_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign38410_e43423, assign38410_e43423_d_n4, assign38410_e43423_d_n6, assign38410_e43423_d_n7, assign38410_e43423_d_n8, assign38410_e43423_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
        let assign38410_e43421: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign38410_e43421, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign38410_e43423;
        locals.var_q_temp3__blk816_dn4 = assign38410_e43423_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign38410_e43423_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign38410_e43423_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign38410_e43423_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign38410_e43423_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign38420_e43434, assign38420_e43434_d_n4, assign38420_e43434_d_n6, assign38420_e43434_d_n7, assign38420_e43434_d_n8, assign38420_e43434_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
        let assign38420_e43430: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign38420_e43432: f64 = (assign38420_e43430 * locals.var_q_temp2__blk815);
        (assign38420_e43432, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign38420_e43430 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign38420_e43430 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign38420_e43430 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign38420_e43430 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign38420_e43430 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign38420_e43434;
        locals.var_q_expnum__blk837_dn4 = assign38420_e43434_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign38420_e43434_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign38420_e43434_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign38420_e43434_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign38420_e43434_d_n9;
        locals.var_q_expnum__blk837_rv = 0.0;

        let (assign38430_e43451, assign38430_e43451_d_n4, assign38430_e43451_d_n6, assign38430_e43451_d_n7, assign38430_e43451_d_n8, assign38430_e43451_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
        let assign38430_e43441: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign38430_e43443: f64 = (assign38430_e43441 - locals.var_q_aexp__blk824);
        let assign38430_e43446: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign38430_e43447: f64 = (assign38430_e43443 - assign38430_e43446);
        let assign38430_e43449: f64 = (assign38430_e43447 * locals.var_q_temp2__blk815);
        (assign38430_e43449, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign38430_e43447 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign38430_e43447 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign38430_e43447 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign38430_e43447 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign38430_e43447 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign38430_e43451;
        locals.var_q_d1_expnum__blk838_dn4 = assign38430_e43451_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign38430_e43451_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign38430_e43451_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign38430_e43451_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign38430_e43451_d_n9;
        locals.var_q_d1_expnum__blk838_rv = 0.0;

        let (assign38440_e43478, assign38440_e43478_d_n4, assign38440_e43478_d_n6, assign38440_e43478_d_n7, assign38440_e43478_d_n8, assign38440_e43478_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
        let assign38440_e43458: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign38440_e43461: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign38440_e43463: f64 = (assign38440_e43461 * locals.var_q_d1_expnum__blk838);
        let assign38440_e43464: f64 = (assign38440_e43458 + assign38440_e43463);
        let assign38440_e43466: f64 = (assign38440_e43464 + locals.var_q_aexp__blk824);
        let assign38440_e43470: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign38440_e43471: f64 = (locals.var_q_d2_ln__blk836 + assign38440_e43470);
        let assign38440_e43473: f64 = (assign38440_e43471 * locals.var_q_sh_term__blk833);
        let assign38440_e43474: f64 = (assign38440_e43466 - assign38440_e43473);
        let assign38440_e43476: f64 = (assign38440_e43474 * locals.var_q_temp2__blk815);
        (assign38440_e43476, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign38440_e43461 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign38440_e43471 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign38440_e43474 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign38440_e43461 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign38440_e43471 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign38440_e43474 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign38440_e43461 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign38440_e43471 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign38440_e43474 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign38440_e43461 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign38440_e43471 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign38440_e43474 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign38440_e43461 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign38440_e43471 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign38440_e43474 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign38440_e43478;
        locals.var_q_d2_expnum__blk839_dn4 = assign38440_e43478_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign38440_e43478_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign38440_e43478_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign38440_e43478_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign38440_e43478_d_n9;
        locals.var_q_d2_expnum__blk839_rv = 0.0;

        let assign38450_e43481: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1195 = assign38450_e43481;
        locals.var_guard1195_rv = 0.0;

        let (assign38460_e43488, assign38460_e43488_d_n4, assign38460_e43488_d_n6, assign38460_e43488_d_n7, assign38460_e43488_d_n8, assign38460_e43488_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
        let assign38460_e43486: f64 = (locals.var_q_expnum__blk837).ln();
        (assign38460_e43486, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign38460_e43488;
        locals.var_q_lnexpnum__blk840_dn4 = assign38460_e43488_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign38460_e43488_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign38460_e43488_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign38460_e43488_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign38460_e43488_d_n9;
        locals.var_q_lnexpnum__blk840_rv = 0.0;

        let (assign38470_e43496, assign38470_e43496_d_n4, assign38470_e43496_d_n6, assign38470_e43496_d_n7, assign38470_e43496_d_n8, assign38470_e43496_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
        let assign38470_e43494: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign38470_e43494, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38470_e43496;
        locals.var_q_temp1__blk814_dn4 = assign38470_e43496_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38470_e43496_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38470_e43496_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38470_e43496_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38470_e43496_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign38480_e43504, assign38480_e43504_d_n4, assign38480_e43504_d_n6, assign38480_e43504_d_n7, assign38480_e43504_d_n8, assign38480_e43504_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
        let assign38480_e43502: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign38480_e43502, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign38480_e43504;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign38480_e43504_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign38480_e43504_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign38480_e43504_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign38480_e43504_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign38480_e43504_d_n9;
        locals.var_q_d1_lnexpnum__blk841_rv = 0.0;

        let (assign38490_e43516, assign38490_e43516_d_n4, assign38490_e43516_d_n6, assign38490_e43516_d_n7, assign38490_e43516_d_n8, assign38490_e43516_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
        let assign38490_e43510: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign38490_e43513: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign38490_e43514: f64 = (assign38490_e43510 - assign38490_e43513);
        (assign38490_e43514, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign38490_e43516;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign38490_e43516_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign38490_e43516_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign38490_e43516_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign38490_e43516_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign38490_e43516_d_n9;
        locals.var_q_d2_lnexpnum__blk842_rv = 0.0;

        let (assign38500_e43529, assign38500_e43529_d_n4, assign38500_e43529_d_n6, assign38500_e43529_d_n7, assign38500_e43529_d_n8, assign38500_e43529_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
        let assign38500_e43523: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign38500_e43525: f64 = (-locals.var_q_k1q1__blk823);
        let assign38500_e43526: f64 = (assign38500_e43525).ln();
        let assign38500_e43527: f64 = (assign38500_e43523 + assign38500_e43526);
        (assign38500_e43527, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign38500_e43525)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign38500_e43525)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign38500_e43525)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign38500_e43525)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign38500_e43525)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign38500_e43529;
        locals.var_q_lnexpnum__blk840_dn4 = assign38500_e43529_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign38500_e43529_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign38500_e43529_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign38500_e43529_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign38500_e43529_d_n9;
        locals.var_q_lnexpnum__blk840_rv = 0.0;

        let (assign38510_e43538, assign38510_e43538_d_n4, assign38510_e43538_d_n6, assign38510_e43538_d_n7, assign38510_e43538_d_n8, assign38510_e43538_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
        let assign38510_e43536: f64 = (1.0 / locals.var_q1d__blk1001);
        (assign38510_e43536, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38510_e43538;
        locals.var_q_temp1__blk814_dn4 = assign38510_e43538_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38510_e43538_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38510_e43538_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38510_e43538_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38510_e43538_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign38520_e43547, assign38520_e43547_d_n4, assign38520_e43547_d_n6, assign38520_e43547_d_n7, assign38520_e43547_d_n8, assign38520_e43547_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
        let assign38520_e43545: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign38520_e43545, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign38520_e43547;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign38520_e43547_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign38520_e43547_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign38520_e43547_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign38520_e43547_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign38520_e43547_d_n9;
        locals.var_q_d1_lnexpnum__blk841_rv = 0.0;

        let (assign38530_e43557, assign38530_e43557_d_n4, assign38530_e43557_d_n6, assign38530_e43557_d_n7, assign38530_e43557_d_n8, assign38530_e43557_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
        let assign38530_e43553: f64 = (-locals.var_q_temp1__blk814);
        let assign38530_e43555: f64 = (assign38530_e43553 * locals.var_q_temp1__blk814);
        (assign38530_e43555, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign38530_e43553 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign38530_e43553 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign38530_e43553 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign38530_e43553 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign38530_e43553 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign38530_e43557;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign38530_e43557_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign38530_e43557_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign38530_e43557_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign38530_e43557_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign38530_e43557_d_n9;
        locals.var_q_d2_lnexpnum__blk842_rv = 0.0;

        let (assign38540_e43571, assign38540_e43571_d_n4, assign38540_e43571_d_n6, assign38540_e43571_d_n7, assign38540_e43571_d_n8, assign38540_e43571_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38540_e43561: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign38540_e43563: f64 = (assign38540_e43561 + locals.var_q1d__blk1001);
        let assign38540_e43566: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign38540_e43567: f64 = (assign38540_e43563 + assign38540_e43566);
        let assign38540_e43569: f64 = (assign38540_e43567 - locals.var_q_ln_term__blk834);
        (assign38540_e43569, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign38540_e43571;
        locals.var_q_q2_int__blk843_dn4 = assign38540_e43571_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign38540_e43571_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign38540_e43571_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign38540_e43571_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign38540_e43571_d_n9;
        locals.var_q_q2_int__blk843_rv = 0.0;

        let (assign38550_e43581, assign38550_e43581_d_n4, assign38550_e43581_d_n6, assign38550_e43581_d_n7, assign38550_e43581_d_n8, assign38550_e43581_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38550_e43576: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign38550_e43577: f64 = (1.0 + assign38550_e43576);
        let assign38550_e43579: f64 = (assign38550_e43577 - locals.var_q_d1_ln__blk835);
        (assign38550_e43579, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign38550_e43581;
        locals.var_q_d1_q2__blk844_dn4 = assign38550_e43581_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign38550_e43581_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign38550_e43581_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign38550_e43581_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign38550_e43581_d_n9;
        locals.var_q_d1_q2__blk844_rv = 0.0;

        let (assign38560_e43589, assign38560_e43589_d_n4, assign38560_e43589_d_n6, assign38560_e43589_d_n7, assign38560_e43589_d_n8, assign38560_e43589_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38560_e43585: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign38560_e43587: f64 = (assign38560_e43585 - locals.var_q_d2_ln__blk836);
        (assign38560_e43587, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign38560_e43589;
        locals.var_q_d2_q2__blk845_dn4 = assign38560_e43589_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign38560_e43589_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign38560_e43589_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign38560_e43589_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign38560_e43589_d_n9;
        locals.var_q_d2_q2__blk845_rv = 0.0;

        let (assign38570_e43597, assign38570_e43597_d_n4, assign38570_e43597_d_n6, assign38570_e43597_d_n7, assign38570_e43597_d_n8, assign38570_e43597_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38570_e43594: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign38570_e43595: f64 = (locals.var_q_k1q1__blk823 + assign38570_e43594);
        (assign38570_e43595, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign38570_e43597;
        locals.var_q_qi_int__blk846_dn4 = assign38570_e43597_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign38570_e43597_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign38570_e43597_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign38570_e43597_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign38570_e43597_d_n9;
        locals.var_q_qi_int__blk846_rv = 0.0;

        let (assign38580_e43605, assign38580_e43605_d_n4, assign38580_e43605_d_n6, assign38580_e43605_d_n7, assign38580_e43605_d_n8, assign38580_e43605_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38580_e43602: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign38580_e43603: f64 = (locals.var_k1__blk932 + assign38580_e43602);
        (assign38580_e43603, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign38580_e43605;
        locals.var_q_d1_qi__blk847_dn4 = assign38580_e43605_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign38580_e43605_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign38580_e43605_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign38580_e43605_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign38580_e43605_d_n9;
        locals.var_q_d1_qi__blk847_rv = 0.0;

        let (assign38590_e43611, assign38590_e43611_d_n4, assign38590_e43611_d_n6, assign38590_e43611_d_n7, assign38590_e43611_d_n8, assign38590_e43611_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38590_e43609: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign38590_e43609, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign38590_e43611;
        locals.var_q_d2_qi__blk848_dn4 = assign38590_e43611_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign38590_e43611_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign38590_e43611_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign38590_e43611_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign38590_e43611_d_n9;
        locals.var_q_d2_qi__blk848_rv = 0.0;

        let (assign38600_e43619, assign38600_e43619_d_n4, assign38600_e43619_d_n6, assign38600_e43619_d_n7, assign38600_e43619_d_n8, assign38600_e43619_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38600_e43615: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign38600_e43617: f64 = (assign38600_e43615 - locals.var_q_aexp__blk824);
        (assign38600_e43617, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign38600_e43619;
        locals.var_q_zero__blk849_dn4 = assign38600_e43619_d_n4;
        locals.var_q_zero__blk849_dn6 = assign38600_e43619_d_n6;
        locals.var_q_zero__blk849_dn7 = assign38600_e43619_d_n7;
        locals.var_q_zero__blk849_dn8 = assign38600_e43619_d_n8;
        locals.var_q_zero__blk849_dn9 = assign38600_e43619_d_n9;
        locals.var_q_zero__blk849_rv = 0.0;

        let (assign38610_e43631, assign38610_e43631_d_n4, assign38610_e43631_d_n6, assign38610_e43631_d_n7, assign38610_e43631_d_n8, assign38610_e43631_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38610_e43623: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign38610_e43626: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign38610_e43627: f64 = (assign38610_e43623 + assign38610_e43626);
        let assign38610_e43629: f64 = (assign38610_e43627 + locals.var_q_aexp__blk824);
        (assign38610_e43629, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign38610_e43631;
        locals.var_q_d1_zero__blk850_dn4 = assign38610_e43631_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign38610_e43631_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign38610_e43631_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign38610_e43631_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign38610_e43631_d_n9;
        locals.var_q_d1_zero__blk850_rv = 0.0;

        let (assign38620_e43649, assign38620_e43649_d_n4, assign38620_e43649_d_n6, assign38620_e43649_d_n7, assign38620_e43649_d_n8, assign38620_e43649_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38620_e43635: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign38620_e43638: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign38620_e43640: f64 = (assign38620_e43638 * locals.var_q_d1_expnum__blk838);
        let assign38620_e43641: f64 = (assign38620_e43635 + assign38620_e43640);
        let assign38620_e43644: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign38620_e43645: f64 = (assign38620_e43641 + assign38620_e43644);
        let assign38620_e43647: f64 = (assign38620_e43645 - locals.var_q_aexp__blk824);
        (assign38620_e43647, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign38620_e43638 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign38620_e43638 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign38620_e43638 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign38620_e43638 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign38620_e43638 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign38620_e43649;
        locals.var_q_d2_zero__blk851_dn4 = assign38620_e43649_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign38620_e43649_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign38620_e43649_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign38620_e43649_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign38620_e43649_d_n9;
        locals.var_q_d2_zero__blk851_rv = 0.0;

        let (assign38630_e43661, assign38630_e43661_d_n4, assign38630_e43661_d_n6, assign38630_e43661_d_n7, assign38630_e43661_d_n8, assign38630_e43661_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38630_e43653: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign38630_e43656: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign38630_e43658: f64 = (assign38630_e43656 * locals.var_q_d2_zero__blk851);
        let assign38630_e43659: f64 = (assign38630_e43653 - assign38630_e43658);
        (assign38630_e43659, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign38630_e43656 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign38630_e43656 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign38630_e43656 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign38630_e43656 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign38630_e43656 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign38630_e43661;
        locals.var_q_temp__blk860_dn4 = assign38630_e43661_d_n4;
        locals.var_q_temp__blk860_dn6 = assign38630_e43661_d_n6;
        locals.var_q_temp__blk860_dn7 = assign38630_e43661_d_n7;
        locals.var_q_temp__blk860_dn8 = assign38630_e43661_d_n8;
        locals.var_q_temp__blk860_dn9 = assign38630_e43661_d_n9;
        locals.var_q_temp__blk860_rv = 0.0;

        let (assign38640_e43676, assign38640_e43676_d_n4, assign38640_e43676_d_n6, assign38640_e43676_d_n7, assign38640_e43676_d_n8, assign38640_e43676_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38640_e43664: f64 = (-locals.var_q_zero__blk849);
        let assign38640_e43666: f64 = (assign38640_e43664 * locals.var_q_d1_zero__blk850);
        let assign38640_e43668: f64 = (assign38640_e43666 * locals.var_q_temp__blk860);
        let assign38640_e43671: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign38640_e43673: f64 = (assign38640_e43671 + 1e-200);
        let assign38640_e43674: f64 = (assign38640_e43668 / assign38640_e43673);
        (assign38640_e43674, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign38640_e43664 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign38640_e43666 * locals.var_q_temp__blk860_dn4)) * assign38640_e43673) - (assign38640_e43668 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign38640_e43673 * assign38640_e43673)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign38640_e43664 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign38640_e43666 * locals.var_q_temp__blk860_dn6)) * assign38640_e43673) - (assign38640_e43668 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign38640_e43673 * assign38640_e43673)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign38640_e43664 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign38640_e43666 * locals.var_q_temp__blk860_dn7)) * assign38640_e43673) - (assign38640_e43668 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign38640_e43673 * assign38640_e43673)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign38640_e43664 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign38640_e43666 * locals.var_q_temp__blk860_dn8)) * assign38640_e43673) - (assign38640_e43668 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign38640_e43673 * assign38640_e43673)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign38640_e43664 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign38640_e43666 * locals.var_q_temp__blk860_dn9)) * assign38640_e43673) - (assign38640_e43668 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign38640_e43673 * assign38640_e43673)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign38640_e43676;
        locals.var_q_eps2__blk852_dn4 = assign38640_e43676_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign38640_e43676_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign38640_e43676_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign38640_e43676_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign38640_e43676_d_n9;
        locals.var_q_eps2__blk852_rv = 0.0;

        let (assign38650_e43682, assign38650_e43682_d_n4, assign38650_e43682_d_n6, assign38650_e43682_d_n7, assign38650_e43682_d_n8, assign38650_e43682_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38650_e43680: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
        (assign38650_e43680, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign38650_e43682;
        locals.var_q1d__blk1001_dn4 = assign38650_e43682_d_n4;
        locals.var_q1d__blk1001_dn6 = assign38650_e43682_d_n6;
        locals.var_q1d__blk1001_dn7 = assign38650_e43682_d_n7;
        locals.var_q1d__blk1001_dn8 = assign38650_e43682_d_n8;
        locals.var_q1d__blk1001_dn9 = assign38650_e43682_d_n9;
        locals.var_q1d__blk1001_rv = 0.0;

        let assign38660_e43685: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1196 = assign38660_e43685;
        locals.var_guard1196_rv = 0.0;

        let assign38670_e43687: f64 = (locals.var_q_eps2__blk852).abs();
        let assign38670_e43689: f64 = if assign38670_e43687 > 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign38670_e43689;
        locals.var_guard1197_rv = 0.0;

        let (assign38680_e43699, assign38680_e43699_d_n4, assign38680_e43699_d_n6, assign38680_e43699_d_n7, assign38680_e43699_d_n8, assign38680_e43699_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign38680_e43697: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign38680_e43697, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign38680_e43699;
        locals.var_q_k1q1__blk823_dn4 = assign38680_e43699_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign38680_e43699_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign38680_e43699_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign38680_e43699_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign38680_e43699_d_n9;
        locals.var_q_k1q1__blk823_rv = 0.0;

        let assign38690_e43702: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38690_e43704: f64 = (assign38690_e43702 - locals.var_xdeff__blk1000);
        let assign38690_e43706: f64 = if assign38690_e43704 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign38690_e43706;
        locals.var_guard1198_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_112(
        locals: &mut StampLocals,
    ) {
        let (assign38700_e43721, assign38700_e43721_d_n4, assign38700_e43721_d_n6, assign38700_e43721_d_n7, assign38700_e43721_d_n8, assign38700_e43721_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1198 != 0.0)) {
        let assign38700_e43716: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38700_e43718: f64 = (assign38700_e43716 - locals.var_xdeff__blk1000);
        let assign38700_e43719: f64 = (assign38700_e43718).exp();
        (assign38700_e43719, (assign38700_e43719 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign38700_e43719 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign38700_e43719 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign38700_e43719 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign38700_e43719 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38700_e43721;
        locals.var_q_temp1__blk814_dn4 = assign38700_e43721_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38700_e43721_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38700_e43721_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38700_e43721_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38700_e43721_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign38710_e43766, assign38710_e43766_d_n4, assign38710_e43766_d_n6, assign38710_e43766_d_n7, assign38710_e43766_d_n8, assign38710_e43766_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1198 == 0.0)) {
        let assign38710_e43734: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38710_e43736: f64 = (assign38710_e43734 - locals.var_xdeff__blk1000);
        let assign38710_e43738: f64 = (assign38710_e43736 - 80.0);
        let assign38710_e43743: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38710_e43745: f64 = (assign38710_e43743 - locals.var_xdeff__blk1000);
        let assign38710_e43747: f64 = (assign38710_e43745 - 80.0);
        let assign38710_e43748: f64 = (0.5 * assign38710_e43747);
        let assign38710_e43752: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38710_e43754: f64 = (assign38710_e43752 - locals.var_xdeff__blk1000);
        let assign38710_e43756: f64 = (assign38710_e43754 - 80.0);
        let assign38710_e43758: f64 = (assign38710_e43756 * 0.3333333333333);
        let assign38710_e43759: f64 = (1.0 + assign38710_e43758);
        let assign38710_e43760: f64 = (assign38710_e43748 * assign38710_e43759);
        let assign38710_e43761: f64 = (1.0 + assign38710_e43760);
        let assign38710_e43762: f64 = (assign38710_e43738 * assign38710_e43761);
        let assign38710_e43763: f64 = (1.0 + assign38710_e43762);
        let assign38710_e43764: f64 = (5.54062e34 * assign38710_e43763);
        (assign38710_e43764, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign38710_e43761) + (assign38710_e43738 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign38710_e43759) + (assign38710_e43748 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign38710_e43761) + (assign38710_e43738 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign38710_e43759) + (assign38710_e43748 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign38710_e43761) + (assign38710_e43738 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign38710_e43759) + (assign38710_e43748 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign38710_e43761) + (assign38710_e43738 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign38710_e43759) + (assign38710_e43748 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign38710_e43761) + (assign38710_e43738 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign38710_e43759) + (assign38710_e43748 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38710_e43766;
        locals.var_q_temp1__blk814_dn4 = assign38710_e43766_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38710_e43766_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38710_e43766_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38710_e43766_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38710_e43766_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign38720_e43776, assign38720_e43776_d_n4, assign38720_e43776_d_n6, assign38720_e43776_d_n7, assign38720_e43776_d_n8, assign38720_e43776_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign38720_e43774: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign38720_e43774, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign38720_e43776;
        locals.var_q_aexp__blk824_dn4 = assign38720_e43776_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign38720_e43776_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign38720_e43776_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign38720_e43776_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign38720_e43776_d_n9;
        locals.var_q_aexp__blk824_rv = 0.0;

        let (assign38730_e43788, assign38730_e43788_d_n4, assign38730_e43788_d_n6, assign38730_e43788_d_n7, assign38730_e43788_d_n8, assign38730_e43788_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign38730_e43784: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign38730_e43786: f64 = (assign38730_e43784 - locals.var_q_aexp__blk824);
        (assign38730_e43786, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign38730_e43788;
        locals.var_q_qsq__blk825_dn4 = assign38730_e43788_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign38730_e43788_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign38730_e43788_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign38730_e43788_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign38730_e43788_d_n9;
        locals.var_q_qsq__blk825_rv = 0.0;

        let (assign38740_e43802, assign38740_e43802_d_n4, assign38740_e43802_d_n6, assign38740_e43802_d_n7, assign38740_e43802_d_n8, assign38740_e43802_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign38740_e43796: f64 = (2.0 * locals.var_k1__blk932);
        let assign38740_e43798: f64 = (assign38740_e43796 * locals.var_q_k1q1__blk823);
        let assign38740_e43800: f64 = (assign38740_e43798 + locals.var_q_aexp__blk824);
        (assign38740_e43800, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign38740_e43796 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign38740_e43796 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign38740_e43796 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign38740_e43796 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign38740_e43796 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign38740_e43802;
        locals.var_q_d1_qsq__blk826_dn4 = assign38740_e43802_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign38740_e43802_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign38740_e43802_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign38740_e43802_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign38740_e43802_d_n9;
        locals.var_q_d1_qsq__blk826_rv = 0.0;

        let (assign38750_e43816, assign38750_e43816_d_n4, assign38750_e43816_d_n6, assign38750_e43816_d_n7, assign38750_e43816_d_n8, assign38750_e43816_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign38750_e43810: f64 = (2.0 * locals.var_k1__blk932);
        let assign38750_e43812: f64 = (assign38750_e43810 * locals.var_k1__blk932);
        let assign38750_e43814: f64 = (assign38750_e43812 - locals.var_q_aexp__blk824);
        (assign38750_e43814, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign38750_e43810 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign38750_e43810 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign38750_e43810 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign38750_e43810 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign38750_e43810 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign38750_e43816;
        locals.var_q_d2_qsq__blk827_dn4 = assign38750_e43816_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign38750_e43816_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign38750_e43816_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign38750_e43816_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign38750_e43816_d_n9;
        locals.var_q_d2_qsq__blk827_rv = 0.0;

        let assign38760_e43819: f64 = (-0.005);
        let assign38760_e43820: f64 = if locals.var_q_qsq__blk825 < assign38760_e43819 { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign38760_e43820;
        locals.var_guard1199_rv = 0.0;

        let (assign38770_e43832, assign38770_e43832_d_n4, assign38770_e43832_d_n6, assign38770_e43832_d_n7, assign38770_e43832_d_n8, assign38770_e43832_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38770_e43829: f64 = (locals.var_q_qsq__blk825).abs();
        let assign38770_e43830: f64 = (assign38770_e43829).sqrt();
        (assign38770_e43830, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38770_e43830)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38770_e43830)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38770_e43830)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38770_e43830)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38770_e43830)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign38770_e43832;
        locals.var_q_rac_qsq__blk828_dn4 = assign38770_e43832_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign38770_e43832_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign38770_e43832_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign38770_e43832_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign38770_e43832_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign38780_e43847, assign38780_e43847_d_n4, assign38780_e43847_d_n6, assign38780_e43847_d_n7, assign38780_e43847_d_n8, assign38780_e43847_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38780_e43843: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign38780_e43844: f64 = (assign38780_e43843).tan();
        let assign38780_e43845: f64 = (locals.var_q_rac_qsq__blk828 / assign38780_e43844);
        (assign38780_e43845, (((locals.var_q_rac_qsq__blk828_dn4 * assign38780_e43844) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign38780_e43843).cos() * (assign38780_e43843).cos())))) / (assign38780_e43844 * assign38780_e43844)), (((locals.var_q_rac_qsq__blk828_dn6 * assign38780_e43844) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign38780_e43843).cos() * (assign38780_e43843).cos())))) / (assign38780_e43844 * assign38780_e43844)), (((locals.var_q_rac_qsq__blk828_dn7 * assign38780_e43844) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign38780_e43843).cos() * (assign38780_e43843).cos())))) / (assign38780_e43844 * assign38780_e43844)), (((locals.var_q_rac_qsq__blk828_dn8 * assign38780_e43844) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign38780_e43843).cos() * (assign38780_e43843).cos())))) / (assign38780_e43844 * assign38780_e43844)), (((locals.var_q_rac_qsq__blk828_dn9 * assign38780_e43844) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign38780_e43843).cos() * (assign38780_e43843).cos())))) / (assign38780_e43844 * assign38780_e43844)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38780_e43847;
        locals.var_q_qcoth__blk829_dn4 = assign38780_e43847_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38780_e43847_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38780_e43847_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38780_e43847_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38780_e43847_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign38790_e43861, assign38790_e43861_d_n4, assign38790_e43861_d_n6, assign38790_e43861_d_n7, assign38790_e43861_d_n8, assign38790_e43861_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38790_e43857: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign38790_e43859: f64 = (assign38790_e43857 / locals.var_q_qsq__blk825);
        (assign38790_e43859, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38790_e43857 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38790_e43857 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38790_e43857 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38790_e43857 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38790_e43857 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38790_e43861;
        locals.var_q_temp1__blk814_dn4 = assign38790_e43861_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38790_e43861_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38790_e43861_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38790_e43861_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38790_e43861_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign38800_e43879, assign38800_e43879_d_n4, assign38800_e43879_d_n6, assign38800_e43879_d_n7, assign38800_e43879_d_n8, assign38800_e43879_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38800_e43873: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign38800_e43874: f64 = (locals.var_q_qcoth__blk829 * assign38800_e43873);
        let assign38800_e43875: f64 = (locals.var_q_qsq__blk825 + assign38800_e43874);
        let assign38800_e43877: f64 = (assign38800_e43875 * locals.var_q_temp1__blk814);
        (assign38800_e43877, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38800_e43873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38800_e43875 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38800_e43873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38800_e43875 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38800_e43873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38800_e43875 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38800_e43873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38800_e43875 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38800_e43873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38800_e43875 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign38800_e43879;
        locals.var_q_d1_qcoth__blk830_dn4 = assign38800_e43879_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign38800_e43879_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign38800_e43879_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign38800_e43879_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign38800_e43879_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign38810_e43905, assign38810_e43905_d_n4, assign38810_e43905_d_n6, assign38810_e43905_d_n7, assign38810_e43905_d_n8, assign38810_e43905_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38810_e43890: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign38810_e43893: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign38810_e43894: f64 = (assign38810_e43890 * assign38810_e43893);
        let assign38810_e43895: f64 = (locals.var_q_d1_qsq__blk826 - assign38810_e43894);
        let assign38810_e43897: f64 = (assign38810_e43895 * locals.var_q_temp1__blk814);
        let assign38810_e43900: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign38810_e43902: f64 = (assign38810_e43900 / locals.var_q_d1_qsq__blk826);
        let assign38810_e43903: f64 = (assign38810_e43897 + assign38810_e43902);
        (assign38810_e43903, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38810_e43893) + (assign38810_e43890 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38810_e43895 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38810_e43900 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38810_e43893) + (assign38810_e43890 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38810_e43895 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38810_e43900 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38810_e43893) + (assign38810_e43890 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38810_e43895 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38810_e43900 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38810_e43893) + (assign38810_e43890 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38810_e43895 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38810_e43900 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38810_e43893) + (assign38810_e43890 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38810_e43895 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38810_e43900 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign38810_e43905;
        locals.var_q_d2_qcoth__blk832_dn4 = assign38810_e43905_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign38810_e43905_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign38810_e43905_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign38810_e43905_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign38810_e43905_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign38820_e43919, assign38820_e43919_d_n4, assign38820_e43919_d_n6, assign38820_e43919_d_n7, assign38820_e43919_d_n8, assign38820_e43919_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38820_e43916: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign38820_e43917: f64 = (1.0 - assign38820_e43916);
        (assign38820_e43917, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38820_e43919;
        locals.var_q_temp2__blk815_dn4 = assign38820_e43919_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38820_e43919_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38820_e43919_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38820_e43919_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38820_e43919_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign38830_e43933, assign38830_e43933_d_n4, assign38830_e43933_d_n6, assign38830_e43933_d_n7, assign38830_e43933_d_n8, assign38830_e43933_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38830_e43929: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign38830_e43931: f64 = (assign38830_e43929 * locals.var_q_temp2__blk815);
        (assign38830_e43931, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38830_e43929 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38830_e43929 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38830_e43929 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38830_e43929 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38830_e43929 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign38830_e43933;
        locals.var_q_d1_ln__blk835_dn4 = assign38830_e43933_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign38830_e43933_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign38830_e43933_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign38830_e43933_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign38830_e43933_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign38840_e43955, assign38840_e43955_d_n4, assign38840_e43955_d_n6, assign38840_e43955_d_n7, assign38840_e43955_d_n8, assign38840_e43955_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38840_e43943: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign38840_e43948: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign38840_e43949: f64 = (locals.var_q_d1_ln__blk835 + assign38840_e43948);
        let assign38840_e43950: f64 = (locals.var_q_d1_qsq__blk826 * assign38840_e43949);
        let assign38840_e43951: f64 = (assign38840_e43943 - assign38840_e43950);
        let assign38840_e43953: f64 = (assign38840_e43951 / locals.var_q_qsq__blk825);
        (assign38840_e43953, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38840_e43949) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38840_e43951 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38840_e43949) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38840_e43951 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38840_e43949) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38840_e43951 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38840_e43949) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38840_e43951 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38840_e43949) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38840_e43951 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign38840_e43955;
        locals.var_q_d2_ln__blk836_dn4 = assign38840_e43955_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign38840_e43955_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign38840_e43955_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign38840_e43955_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign38840_e43955_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let assign38850_e43958: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign38850_e43958;
        locals.var_guard1200_rv = 0.0;

        let (assign38860_e43973, assign38860_e43973_d_n4, assign38860_e43973_d_n6, assign38860_e43973_d_n7, assign38860_e43973_d_n8, assign38860_e43973_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38860_e43970: f64 = (locals.var_q_qsq__blk825).abs();
        let assign38860_e43971: f64 = (assign38860_e43970).sqrt();
        (assign38860_e43971, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38860_e43971)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38860_e43971)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38860_e43971)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38860_e43971)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38860_e43971)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign38860_e43973;
        locals.var_q_rac_qsq__blk828_dn4 = assign38860_e43973_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign38860_e43973_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign38860_e43973_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign38860_e43973_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign38860_e43973_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign38870_e43988, assign38870_e43988_d_n4, assign38870_e43988_d_n6, assign38870_e43988_d_n7, assign38870_e43988_d_n8, assign38870_e43988_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38870_e43985: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign38870_e43986: f64 = (assign38870_e43985).exp();
        (assign38870_e43986, (assign38870_e43986 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign38870_e43986 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign38870_e43986 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign38870_e43986 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign38870_e43986 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign38870_e43988;
        locals.var_q_invexpq__blk831_dn4 = assign38870_e43988_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign38870_e43988_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign38870_e43988_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign38870_e43988_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign38870_e43988_d_n9;
        locals.var_q_invexpq__blk831_rv = 0.0;

        let (assign38880_e44009, assign38880_e44009_d_n4, assign38880_e44009_d_n6, assign38880_e44009_d_n7, assign38880_e44009_d_n8, assign38880_e44009_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38880_e44002: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign38880_e44003: f64 = (locals.var_q_rac_qsq__blk828 * assign38880_e44002);
        let assign38880_e44006: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign38880_e44007: f64 = (assign38880_e44003 / assign38880_e44006);
        (assign38880_e44007, (((((locals.var_q_rac_qsq__blk828_dn4 * assign38880_e44002) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign38880_e44006) - (assign38880_e44003 * (-locals.var_q_invexpq__blk831_dn4))) / (assign38880_e44006 * assign38880_e44006)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign38880_e44002) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign38880_e44006) - (assign38880_e44003 * (-locals.var_q_invexpq__blk831_dn6))) / (assign38880_e44006 * assign38880_e44006)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign38880_e44002) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign38880_e44006) - (assign38880_e44003 * (-locals.var_q_invexpq__blk831_dn7))) / (assign38880_e44006 * assign38880_e44006)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign38880_e44002) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign38880_e44006) - (assign38880_e44003 * (-locals.var_q_invexpq__blk831_dn8))) / (assign38880_e44006 * assign38880_e44006)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign38880_e44002) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign38880_e44006) - (assign38880_e44003 * (-locals.var_q_invexpq__blk831_dn9))) / (assign38880_e44006 * assign38880_e44006)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38880_e44009;
        locals.var_q_qcoth__blk829_dn4 = assign38880_e44009_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38880_e44009_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38880_e44009_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38880_e44009_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38880_e44009_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign38890_e44026, assign38890_e44026_d_n4, assign38890_e44026_d_n6, assign38890_e44026_d_n7, assign38890_e44026_d_n8, assign38890_e44026_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38890_e44022: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign38890_e44024: f64 = (assign38890_e44022 / locals.var_q_qsq__blk825);
        (assign38890_e44024, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38890_e44022 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38890_e44022 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38890_e44022 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38890_e44022 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38890_e44022 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38890_e44026;
        locals.var_q_temp1__blk814_dn4 = assign38890_e44026_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38890_e44026_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38890_e44026_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38890_e44026_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38890_e44026_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign38900_e44047, assign38900_e44047_d_n4, assign38900_e44047_d_n6, assign38900_e44047_d_n7, assign38900_e44047_d_n8, assign38900_e44047_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38900_e44041: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign38900_e44042: f64 = (locals.var_q_qcoth__blk829 * assign38900_e44041);
        let assign38900_e44043: f64 = (locals.var_q_qsq__blk825 + assign38900_e44042);
        let assign38900_e44045: f64 = (assign38900_e44043 * locals.var_q_temp1__blk814);
        (assign38900_e44045, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38900_e44041) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38900_e44043 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38900_e44041) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38900_e44043 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38900_e44041) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38900_e44043 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38900_e44041) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38900_e44043 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38900_e44041) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38900_e44043 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign38900_e44047;
        locals.var_q_d1_qcoth__blk830_dn4 = assign38900_e44047_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign38900_e44047_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign38900_e44047_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign38900_e44047_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign38900_e44047_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign38910_e44076, assign38910_e44076_d_n4, assign38910_e44076_d_n6, assign38910_e44076_d_n7, assign38910_e44076_d_n8, assign38910_e44076_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38910_e44061: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign38910_e44064: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign38910_e44065: f64 = (assign38910_e44061 * assign38910_e44064);
        let assign38910_e44066: f64 = (locals.var_q_d1_qsq__blk826 - assign38910_e44065);
        let assign38910_e44068: f64 = (assign38910_e44066 * locals.var_q_temp1__blk814);
        let assign38910_e44071: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign38910_e44073: f64 = (assign38910_e44071 / locals.var_q_d1_qsq__blk826);
        let assign38910_e44074: f64 = (assign38910_e44068 + assign38910_e44073);
        (assign38910_e44074, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38910_e44064) + (assign38910_e44061 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38910_e44066 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44071 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38910_e44064) + (assign38910_e44061 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38910_e44066 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44071 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38910_e44064) + (assign38910_e44061 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38910_e44066 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44071 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38910_e44064) + (assign38910_e44061 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38910_e44066 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44071 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38910_e44064) + (assign38910_e44061 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38910_e44066 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44071 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign38910_e44076;
        locals.var_q_d2_qcoth__blk832_dn4 = assign38910_e44076_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign38910_e44076_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign38910_e44076_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign38910_e44076_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign38910_e44076_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign38920_e44093, assign38920_e44093_d_n4, assign38920_e44093_d_n6, assign38920_e44093_d_n7, assign38920_e44093_d_n8, assign38920_e44093_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38920_e44090: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign38920_e44091: f64 = (1.0 - assign38920_e44090);
        (assign38920_e44091, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38920_e44093;
        locals.var_q_temp2__blk815_dn4 = assign38920_e44093_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38920_e44093_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38920_e44093_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38920_e44093_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38920_e44093_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign38930_e44110, assign38930_e44110_d_n4, assign38930_e44110_d_n6, assign38930_e44110_d_n7, assign38930_e44110_d_n8, assign38930_e44110_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38930_e44106: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign38930_e44108: f64 = (assign38930_e44106 * locals.var_q_temp2__blk815);
        (assign38930_e44108, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44106 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44106 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44106 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44106 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44106 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign38930_e44110;
        locals.var_q_d1_ln__blk835_dn4 = assign38930_e44110_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign38930_e44110_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign38930_e44110_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign38930_e44110_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign38930_e44110_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign38940_e44135, assign38940_e44135_d_n4, assign38940_e44135_d_n6, assign38940_e44135_d_n7, assign38940_e44135_d_n8, assign38940_e44135_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38940_e44123: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign38940_e44128: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign38940_e44129: f64 = (locals.var_q_d1_ln__blk835 + assign38940_e44128);
        let assign38940_e44130: f64 = (locals.var_q_d1_qsq__blk826 * assign38940_e44129);
        let assign38940_e44131: f64 = (assign38940_e44123 - assign38940_e44130);
        let assign38940_e44133: f64 = (assign38940_e44131 / locals.var_q_qsq__blk825);
        (assign38940_e44133, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38940_e44129) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38940_e44131 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38940_e44129) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38940_e44131 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38940_e44129) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38940_e44131 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38940_e44129) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38940_e44131 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38940_e44129) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38940_e44131 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign38940_e44135;
        locals.var_q_d2_ln__blk836_dn4 = assign38940_e44135_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign38940_e44135_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign38940_e44135_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign38940_e44135_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign38940_e44135_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let (assign38950_e44167, assign38950_e44167_d_n4, assign38950_e44167_d_n6, assign38950_e44167_d_n7, assign38950_e44167_d_n8, assign38950_e44167_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign38950_e44151: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign38950_e44155: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign38950_e44159: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign38950_e44160: f64 = (1.0 - assign38950_e44159);
        let assign38950_e44161: f64 = (assign38950_e44155 * assign38950_e44160);
        let assign38950_e44162: f64 = (1.0 - assign38950_e44161);
        let assign38950_e44163: f64 = (assign38950_e44151 * assign38950_e44162);
        let assign38950_e44164: f64 = (1.0 - assign38950_e44163);
        let assign38950_e44165: f64 = (0.1666666666667 * assign38950_e44164);
        (assign38950_e44165, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign38950_e44162) + (assign38950_e44151 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign38950_e44160) + (assign38950_e44155 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign38950_e44162) + (assign38950_e44151 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign38950_e44160) + (assign38950_e44155 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign38950_e44162) + (assign38950_e44151 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign38950_e44160) + (assign38950_e44155 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign38950_e44162) + (assign38950_e44151 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign38950_e44160) + (assign38950_e44155 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign38950_e44162) + (assign38950_e44151 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign38950_e44160) + (assign38950_e44155 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign38950_e44167;
        locals.var_q_temp3__blk816_dn4 = assign38950_e44167_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign38950_e44167_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign38950_e44167_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign38950_e44167_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign38950_e44167_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign38960_e44185, assign38960_e44185_d_n4, assign38960_e44185_d_n6, assign38960_e44185_d_n7, assign38960_e44185_d_n8, assign38960_e44185_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign38960_e44182: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign38960_e44183: f64 = (2.0 + assign38960_e44182);
        (assign38960_e44183, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38960_e44185;
        locals.var_q_qcoth__blk829_dn4 = assign38960_e44185_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38960_e44185_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38960_e44185_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38960_e44185_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38960_e44185_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let (assign38970_e44217, assign38970_e44217_d_n4, assign38970_e44217_d_n6, assign38970_e44217_d_n7, assign38970_e44217_d_n8, assign38970_e44217_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign38970_e44201: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign38970_e44205: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign38970_e44209: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign38970_e44210: f64 = (1.0 - assign38970_e44209);
        let assign38970_e44211: f64 = (assign38970_e44205 * assign38970_e44210);
        let assign38970_e44212: f64 = (1.0 - assign38970_e44211);
        let assign38970_e44213: f64 = (assign38970_e44201 * assign38970_e44212);
        let assign38970_e44214: f64 = (1.0 - assign38970_e44213);
        let assign38970_e44215: f64 = (0.1666666666667 * assign38970_e44214);
        (assign38970_e44215, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign38970_e44212) + (assign38970_e44201 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign38970_e44210) + (assign38970_e44205 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign38970_e44212) + (assign38970_e44201 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign38970_e44210) + (assign38970_e44205 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign38970_e44212) + (assign38970_e44201 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign38970_e44210) + (assign38970_e44205 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign38970_e44212) + (assign38970_e44201 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign38970_e44210) + (assign38970_e44205 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign38970_e44212) + (assign38970_e44201 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign38970_e44210) + (assign38970_e44205 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38970_e44217;
        locals.var_q_temp1__blk814_dn4 = assign38970_e44217_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38970_e44217_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38970_e44217_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38970_e44217_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38970_e44217_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_113(
        locals: &mut StampLocals,
    ) {
        let (assign38980_e44233, assign38980_e44233_d_n4, assign38980_e44233_d_n6, assign38980_e44233_d_n7, assign38980_e44233_d_n8, assign38980_e44233_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign38980_e44231: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign38980_e44231, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign38980_e44233;
        locals.var_q_d1_qcoth__blk830_dn4 = assign38980_e44233_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign38980_e44233_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign38980_e44233_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign38980_e44233_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign38980_e44233_d_n9;
        locals.var_q_d1_qcoth__blk830_rv = 0.0;

        let (assign38990_e44265, assign38990_e44265_d_n4, assign38990_e44265_d_n6, assign38990_e44265_d_n7, assign38990_e44265_d_n8, assign38990_e44265_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign38990_e44249: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign38990_e44253: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign38990_e44257: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign38990_e44258: f64 = (1.0 - assign38990_e44257);
        let assign38990_e44259: f64 = (assign38990_e44253 * assign38990_e44258);
        let assign38990_e44260: f64 = (1.0 - assign38990_e44259);
        let assign38990_e44261: f64 = (assign38990_e44249 * assign38990_e44260);
        let assign38990_e44262: f64 = (1.0 - assign38990_e44261);
        let assign38990_e44263: f64 = (0.0055555555556 * assign38990_e44262);
        (assign38990_e44263, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign38990_e44260) + (assign38990_e44249 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign38990_e44258) + (assign38990_e44253 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign38990_e44260) + (assign38990_e44249 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign38990_e44258) + (assign38990_e44253 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign38990_e44260) + (assign38990_e44249 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign38990_e44258) + (assign38990_e44253 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign38990_e44260) + (assign38990_e44249 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign38990_e44258) + (assign38990_e44253 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign38990_e44260) + (assign38990_e44249 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign38990_e44258) + (assign38990_e44253 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38990_e44265;
        locals.var_q_temp2__blk815_dn4 = assign38990_e44265_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38990_e44265_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38990_e44265_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38990_e44265_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38990_e44265_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign39000_e44287, assign39000_e44287_d_n4, assign39000_e44287_d_n6, assign39000_e44287_d_n7, assign39000_e44287_d_n8, assign39000_e44287_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39000_e44279: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign39000_e44282: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign39000_e44284: f64 = (assign39000_e44282 * locals.var_q_temp2__blk815);
        let assign39000_e44285: f64 = (assign39000_e44279 - assign39000_e44284);
        (assign39000_e44285, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign39000_e44282 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign39000_e44282 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign39000_e44282 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign39000_e44282 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign39000_e44282 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign39000_e44287;
        locals.var_q_d2_qcoth__blk832_dn4 = assign39000_e44287_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign39000_e44287_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign39000_e44287_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign39000_e44287_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign39000_e44287_d_n9;
        locals.var_q_d2_qcoth__blk832_rv = 0.0;

        let (assign39010_e44306, assign39010_e44306_d_n4, assign39010_e44306_d_n6, assign39010_e44306_d_n7, assign39010_e44306_d_n8, assign39010_e44306_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39010_e44300: f64 = (-0.5);
        let assign39010_e44302: f64 = (assign39010_e44300 * locals.var_q_d1_qsq__blk826);
        let assign39010_e44304: f64 = (assign39010_e44302 * locals.var_q_temp3__blk816);
        (assign39010_e44304, (((assign39010_e44300 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign39010_e44302 * locals.var_q_temp3__blk816_dn4)), (((assign39010_e44300 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign39010_e44302 * locals.var_q_temp3__blk816_dn6)), (((assign39010_e44300 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign39010_e44302 * locals.var_q_temp3__blk816_dn7)), (((assign39010_e44300 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign39010_e44302 * locals.var_q_temp3__blk816_dn8)), (((assign39010_e44300 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign39010_e44302 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign39010_e44306;
        locals.var_q_d1_ln__blk835_dn4 = assign39010_e44306_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign39010_e44306_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign39010_e44306_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign39010_e44306_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign39010_e44306_d_n9;
        locals.var_q_d1_ln__blk835_rv = 0.0;

        let (assign39020_e44345, assign39020_e44345_d_n4, assign39020_e44345_d_n6, assign39020_e44345_d_n7, assign39020_e44345_d_n8, assign39020_e44345_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39020_e44319: f64 = (-0.5);
        let assign39020_e44321: f64 = (assign39020_e44319 * locals.var_q_d2_qsq__blk827);
        let assign39020_e44323: f64 = (assign39020_e44321 * locals.var_q_temp3__blk816);
        let assign39020_e44326: f64 = (0.25 * 0.0055555555556);
        let assign39020_e44328: f64 = (assign39020_e44326 * locals.var_q_d1_qsq__blk826);
        let assign39020_e44330: f64 = (assign39020_e44328 * locals.var_q_d1_qsq__blk826);
        let assign39020_e44334: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign39020_e44338: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign39020_e44339: f64 = (2.0 - assign39020_e44338);
        let assign39020_e44340: f64 = (assign39020_e44334 * assign39020_e44339);
        let assign39020_e44341: f64 = (1.0 - assign39020_e44340);
        let assign39020_e44342: f64 = (assign39020_e44330 * assign39020_e44341);
        let assign39020_e44343: f64 = (assign39020_e44323 + assign39020_e44342);
        (assign39020_e44343, ((((assign39020_e44319 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign39020_e44321 * locals.var_q_temp3__blk816_dn4)) + (((((assign39020_e44326 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign39020_e44328 * locals.var_q_d1_qsq__blk826_dn4)) * assign39020_e44341) + (assign39020_e44330 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign39020_e44339) + (assign39020_e44334 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign39020_e44319 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign39020_e44321 * locals.var_q_temp3__blk816_dn6)) + (((((assign39020_e44326 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign39020_e44328 * locals.var_q_d1_qsq__blk826_dn6)) * assign39020_e44341) + (assign39020_e44330 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign39020_e44339) + (assign39020_e44334 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign39020_e44319 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign39020_e44321 * locals.var_q_temp3__blk816_dn7)) + (((((assign39020_e44326 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign39020_e44328 * locals.var_q_d1_qsq__blk826_dn7)) * assign39020_e44341) + (assign39020_e44330 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign39020_e44339) + (assign39020_e44334 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign39020_e44319 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign39020_e44321 * locals.var_q_temp3__blk816_dn8)) + (((((assign39020_e44326 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign39020_e44328 * locals.var_q_d1_qsq__blk826_dn8)) * assign39020_e44341) + (assign39020_e44330 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign39020_e44339) + (assign39020_e44334 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign39020_e44319 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign39020_e44321 * locals.var_q_temp3__blk816_dn9)) + (((((assign39020_e44326 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign39020_e44328 * locals.var_q_d1_qsq__blk826_dn9)) * assign39020_e44341) + (assign39020_e44330 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign39020_e44339) + (assign39020_e44334 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign39020_e44345;
        locals.var_q_d2_ln__blk836_dn4 = assign39020_e44345_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign39020_e44345_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign39020_e44345_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign39020_e44345_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign39020_e44345_d_n9;
        locals.var_q_d2_ln__blk836_rv = 0.0;

        let assign39030_e44348: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign39030_e44348;
        locals.var_guard1201_rv = 0.0;

        let (assign39040_e44368, assign39040_e44368_d_n4, assign39040_e44368_d_n6, assign39040_e44368_d_n7, assign39040_e44368_d_n8, assign39040_e44368_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign39040_e44358: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign39040_e44363: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign39040_e44364: f64 = (locals.var_q_invexpq__blk831 * assign39040_e44363);
        let assign39040_e44365: f64 = (1.0 - assign39040_e44364);
        let assign39040_e44366: f64 = (assign39040_e44358 / assign39040_e44365);
        (assign39040_e44366, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign39040_e44365) - (assign39040_e44358 * (-((locals.var_q_invexpq__blk831_dn4 * assign39040_e44363) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39040_e44365 * assign39040_e44365)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign39040_e44365) - (assign39040_e44358 * (-((locals.var_q_invexpq__blk831_dn6 * assign39040_e44363) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39040_e44365 * assign39040_e44365)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign39040_e44365) - (assign39040_e44358 * (-((locals.var_q_invexpq__blk831_dn7 * assign39040_e44363) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39040_e44365 * assign39040_e44365)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign39040_e44365) - (assign39040_e44358 * (-((locals.var_q_invexpq__blk831_dn8 * assign39040_e44363) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39040_e44365 * assign39040_e44365)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign39040_e44365) - (assign39040_e44358 * (-((locals.var_q_invexpq__blk831_dn9 * assign39040_e44363) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39040_e44365 * assign39040_e44365)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39040_e44368;
        locals.var_q_temp2__blk815_dn4 = assign39040_e44368_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39040_e44368_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39040_e44368_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39040_e44368_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39040_e44368_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign39050_e44380, assign39050_e44380_d_n4, assign39050_e44380_d_n6, assign39050_e44380_d_n7, assign39050_e44380_d_n8, assign39050_e44380_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign39050_e44378: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign39050_e44378, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign39050_e44380;
        locals.var_q_sh_term__blk833_dn4 = assign39050_e44380_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign39050_e44380_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign39050_e44380_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign39050_e44380_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign39050_e44380_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign39060_e44393, assign39060_e44393_d_n4, assign39060_e44393_d_n6, assign39060_e44393_d_n7, assign39060_e44393_d_n8, assign39060_e44393_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign39060_e44389: f64 = (locals.var_q_temp2__blk815).ln();
        let assign39060_e44391: f64 = (assign39060_e44389 - locals.var_q_rac_qsq__blk828);
        (assign39060_e44391, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign39060_e44393;
        locals.var_q_ln_term__blk834_dn4 = assign39060_e44393_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign39060_e44393_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign39060_e44393_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign39060_e44393_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign39060_e44393_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let assign39070_e44396: f64 = (-0.005);
        let assign39070_e44397: f64 = if locals.var_q_qsq__blk825 < assign39070_e44396 { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign39070_e44397;
        locals.var_guard1202_rv = 0.0;

        let (assign39080_e44413, assign39080_e44413_d_n4, assign39080_e44413_d_n6, assign39080_e44413_d_n7, assign39080_e44413_d_n8, assign39080_e44413_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        let assign39080_e44410: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign39080_e44411: f64 = (assign39080_e44410).sin();
        (assign39080_e44411, ((assign39080_e44410).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39080_e44410).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39080_e44410).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39080_e44410).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39080_e44410).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39080_e44413;
        locals.var_q_temp2__blk815_dn4 = assign39080_e44413_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39080_e44413_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39080_e44413_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39080_e44413_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39080_e44413_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign39090_e44431, assign39090_e44431_d_n4, assign39090_e44431_d_n6, assign39090_e44431_d_n7, assign39090_e44431_d_n8, assign39090_e44431_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        let assign39090_e44425: f64 = (-locals.var_q_qsq__blk825);
        let assign39090_e44428: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign39090_e44429: f64 = (assign39090_e44425 / assign39090_e44428);
        (assign39090_e44429, ((((-locals.var_q_qsq__blk825_dn4) * assign39090_e44428) - (assign39090_e44425 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign39090_e44428 * assign39090_e44428)), ((((-locals.var_q_qsq__blk825_dn6) * assign39090_e44428) - (assign39090_e44425 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign39090_e44428 * assign39090_e44428)), ((((-locals.var_q_qsq__blk825_dn7) * assign39090_e44428) - (assign39090_e44425 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign39090_e44428 * assign39090_e44428)), ((((-locals.var_q_qsq__blk825_dn8) * assign39090_e44428) - (assign39090_e44425 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign39090_e44428 * assign39090_e44428)), ((((-locals.var_q_qsq__blk825_dn9) * assign39090_e44428) - (assign39090_e44425 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign39090_e44428 * assign39090_e44428)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign39090_e44431;
        locals.var_q_sh_term__blk833_dn4 = assign39090_e44431_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign39090_e44431_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign39090_e44431_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign39090_e44431_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign39090_e44431_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign39100_e44445, assign39100_e44445_d_n4, assign39100_e44445_d_n6, assign39100_e44445_d_n7, assign39100_e44445_d_n8, assign39100_e44445_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        let assign39100_e44443: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign39100_e44443, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign39100_e44445;
        locals.var_q_ln_term__blk834_dn4 = assign39100_e44445_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign39100_e44445_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign39100_e44445_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign39100_e44445_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign39100_e44445_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let (assign39110_e44475, assign39110_e44475_d_n4, assign39110_e44475_d_n6, assign39110_e44475_d_n7, assign39110_e44475_d_n8, assign39110_e44475_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
        let assign39110_e44460: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign39110_e44464: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign39110_e44468: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign39110_e44469: f64 = (1.0 - assign39110_e44468);
        let assign39110_e44470: f64 = (assign39110_e44464 * assign39110_e44469);
        let assign39110_e44471: f64 = (1.0 - assign39110_e44470);
        let assign39110_e44472: f64 = (assign39110_e44460 * assign39110_e44471);
        let assign39110_e44473: f64 = (4.0 - assign39110_e44472);
        (assign39110_e44473, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign39110_e44471) + (assign39110_e44460 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign39110_e44469) + (assign39110_e44464 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign39110_e44471) + (assign39110_e44460 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign39110_e44469) + (assign39110_e44464 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign39110_e44471) + (assign39110_e44460 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign39110_e44469) + (assign39110_e44464 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign39110_e44471) + (assign39110_e44460 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign39110_e44469) + (assign39110_e44464 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign39110_e44471) + (assign39110_e44460 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign39110_e44469) + (assign39110_e44464 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign39110_e44475;
        locals.var_q_sh_term__blk833_dn4 = assign39110_e44475_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign39110_e44475_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign39110_e44475_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign39110_e44475_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign39110_e44475_d_n9;
        locals.var_q_sh_term__blk833_rv = 0.0;

        let (assign39120_e44490, assign39120_e44490_d_n4, assign39120_e44490_d_n6, assign39120_e44490_d_n7, assign39120_e44490_d_n8, assign39120_e44490_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
        let assign39120_e44488: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign39120_e44488, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign39120_e44490;
        locals.var_q_ln_term__blk834_dn4 = assign39120_e44490_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign39120_e44490_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign39120_e44490_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign39120_e44490_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign39120_e44490_d_n9;
        locals.var_q_ln_term__blk834_rv = 0.0;

        let assign39130_e44493: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign39130_e44495: f64 = (assign39130_e44493 + locals.var_q_qcoth__blk829);
        let assign39130_e44497: f64 = if assign39130_e44495 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign39130_e44497;
        locals.var_guard1203_rv = 0.0;

        let (assign39140_e44509, assign39140_e44509_d_n4, assign39140_e44509_d_n6, assign39140_e44509_d_n7, assign39140_e44509_d_n8, assign39140_e44509_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign39140_e44507: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign39140_e44507, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign39140_e44509;
        locals.var_q_expnum__blk837_dn4 = assign39140_e44509_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign39140_e44509_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign39140_e44509_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign39140_e44509_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign39140_e44509_d_n9;
        locals.var_q_expnum__blk837_rv = 0.0;

        let (assign39150_e44521, assign39150_e44521_d_n4, assign39150_e44521_d_n6, assign39150_e44521_d_n7, assign39150_e44521_d_n8, assign39150_e44521_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign39150_e44519: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign39150_e44519, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign39150_e44521;
        locals.var_q_d1_expnum__blk838_dn4 = assign39150_e44521_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign39150_e44521_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign39150_e44521_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign39150_e44521_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign39150_e44521_d_n9;
        locals.var_q_d1_expnum__blk838_rv = 0.0;

        let (assign39160_e44531, assign39160_e44531_d_n4, assign39160_e44531_d_n6, assign39160_e44531_d_n7, assign39160_e44531_d_n8, assign39160_e44531_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign39160_e44531;
        locals.var_q_d2_expnum__blk839_dn4 = assign39160_e44531_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign39160_e44531_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign39160_e44531_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign39160_e44531_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign39160_e44531_d_n9;
        locals.var_q_d2_expnum__blk839_rv = 0.0;

        let (assign39170_e44546, assign39170_e44546_d_n4, assign39170_e44546_d_n6, assign39170_e44546_d_n7, assign39170_e44546_d_n8, assign39170_e44546_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
        let assign39170_e44543: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign39170_e44544: f64 = (1.0 / assign39170_e44543);
        (assign39170_e44544, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign39170_e44543 * assign39170_e44543))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign39170_e44543 * assign39170_e44543))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign39170_e44543 * assign39170_e44543))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign39170_e44543 * assign39170_e44543))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign39170_e44543 * assign39170_e44543))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39170_e44546;
        locals.var_q_temp2__blk815_dn4 = assign39170_e44546_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39170_e44546_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39170_e44546_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39170_e44546_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39170_e44546_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign39180_e44559, assign39180_e44559_d_n4, assign39180_e44559_d_n6, assign39180_e44559_d_n7, assign39180_e44559_d_n8, assign39180_e44559_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
        let assign39180_e44557: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign39180_e44557, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign39180_e44559;
        locals.var_q_temp3__blk816_dn4 = assign39180_e44559_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign39180_e44559_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign39180_e44559_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign39180_e44559_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign39180_e44559_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign39190_e44574, assign39190_e44574_d_n4, assign39190_e44574_d_n6, assign39190_e44574_d_n7, assign39190_e44574_d_n8, assign39190_e44574_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
        let assign39190_e44570: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign39190_e44572: f64 = (assign39190_e44570 * locals.var_q_temp2__blk815);
        (assign39190_e44572, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign39190_e44570 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign39190_e44570 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign39190_e44570 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign39190_e44570 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign39190_e44570 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign39190_e44574;
        locals.var_q_expnum__blk837_dn4 = assign39190_e44574_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign39190_e44574_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign39190_e44574_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign39190_e44574_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign39190_e44574_d_n9;
        locals.var_q_expnum__blk837_rv = 0.0;

        let (assign39200_e44595, assign39200_e44595_d_n4, assign39200_e44595_d_n6, assign39200_e44595_d_n7, assign39200_e44595_d_n8, assign39200_e44595_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
        let assign39200_e44585: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign39200_e44587: f64 = (assign39200_e44585 - locals.var_q_aexp__blk824);
        let assign39200_e44590: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign39200_e44591: f64 = (assign39200_e44587 - assign39200_e44590);
        let assign39200_e44593: f64 = (assign39200_e44591 * locals.var_q_temp2__blk815);
        (assign39200_e44593, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign39200_e44591 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign39200_e44591 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign39200_e44591 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign39200_e44591 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign39200_e44591 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign39200_e44595;
        locals.var_q_d1_expnum__blk838_dn4 = assign39200_e44595_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign39200_e44595_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign39200_e44595_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign39200_e44595_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign39200_e44595_d_n9;
        locals.var_q_d1_expnum__blk838_rv = 0.0;

        let (assign39210_e44626, assign39210_e44626_d_n4, assign39210_e44626_d_n6, assign39210_e44626_d_n7, assign39210_e44626_d_n8, assign39210_e44626_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
        let assign39210_e44606: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign39210_e44609: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign39210_e44611: f64 = (assign39210_e44609 * locals.var_q_d1_expnum__blk838);
        let assign39210_e44612: f64 = (assign39210_e44606 + assign39210_e44611);
        let assign39210_e44614: f64 = (assign39210_e44612 + locals.var_q_aexp__blk824);
        let assign39210_e44618: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign39210_e44619: f64 = (locals.var_q_d2_ln__blk836 + assign39210_e44618);
        let assign39210_e44621: f64 = (assign39210_e44619 * locals.var_q_sh_term__blk833);
        let assign39210_e44622: f64 = (assign39210_e44614 - assign39210_e44621);
        let assign39210_e44624: f64 = (assign39210_e44622 * locals.var_q_temp2__blk815);
        (assign39210_e44624, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign39210_e44609 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign39210_e44619 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign39210_e44622 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign39210_e44609 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign39210_e44619 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign39210_e44622 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign39210_e44609 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign39210_e44619 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign39210_e44622 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign39210_e44609 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign39210_e44619 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign39210_e44622 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign39210_e44609 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign39210_e44619 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign39210_e44622 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign39210_e44626;
        locals.var_q_d2_expnum__blk839_dn4 = assign39210_e44626_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign39210_e44626_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign39210_e44626_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign39210_e44626_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign39210_e44626_d_n9;
        locals.var_q_d2_expnum__blk839_rv = 0.0;

        let assign39220_e44629: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign39220_e44629;
        locals.var_guard1204_rv = 0.0;

        let (assign39230_e44640, assign39230_e44640_d_n4, assign39230_e44640_d_n6, assign39230_e44640_d_n7, assign39230_e44640_d_n8, assign39230_e44640_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign39230_e44638: f64 = (locals.var_q_expnum__blk837).ln();
        (assign39230_e44638, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign39230_e44640;
        locals.var_q_lnexpnum__blk840_dn4 = assign39230_e44640_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign39230_e44640_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign39230_e44640_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign39230_e44640_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign39230_e44640_d_n9;
        locals.var_q_lnexpnum__blk840_rv = 0.0;

        let (assign39240_e44652, assign39240_e44652_d_n4, assign39240_e44652_d_n6, assign39240_e44652_d_n7, assign39240_e44652_d_n8, assign39240_e44652_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign39240_e44650: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign39240_e44650, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39240_e44652;
        locals.var_q_temp1__blk814_dn4 = assign39240_e44652_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39240_e44652_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39240_e44652_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39240_e44652_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39240_e44652_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign39250_e44664, assign39250_e44664_d_n4, assign39250_e44664_d_n6, assign39250_e44664_d_n7, assign39250_e44664_d_n8, assign39250_e44664_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign39250_e44662: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign39250_e44662, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign39250_e44664;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign39250_e44664_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign39250_e44664_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign39250_e44664_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign39250_e44664_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign39250_e44664_d_n9;
        locals.var_q_d1_lnexpnum__blk841_rv = 0.0;

        let (assign39260_e44680, assign39260_e44680_d_n4, assign39260_e44680_d_n6, assign39260_e44680_d_n7, assign39260_e44680_d_n8, assign39260_e44680_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign39260_e44674: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign39260_e44677: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign39260_e44678: f64 = (assign39260_e44674 - assign39260_e44677);
        (assign39260_e44678, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign39260_e44680;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign39260_e44680_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign39260_e44680_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign39260_e44680_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign39260_e44680_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign39260_e44680_d_n9;
        locals.var_q_d2_lnexpnum__blk842_rv = 0.0;

        let (assign39270_e44697, assign39270_e44697_d_n4, assign39270_e44697_d_n6, assign39270_e44697_d_n7, assign39270_e44697_d_n8, assign39270_e44697_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign39270_e44691: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign39270_e44693: f64 = (-locals.var_q_k1q1__blk823);
        let assign39270_e44694: f64 = (assign39270_e44693).ln();
        let assign39270_e44695: f64 = (assign39270_e44691 + assign39270_e44694);
        (assign39270_e44695, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign39270_e44693)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign39270_e44693)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign39270_e44693)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign39270_e44693)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign39270_e44693)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign39270_e44697;
        locals.var_q_lnexpnum__blk840_dn4 = assign39270_e44697_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign39270_e44697_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign39270_e44697_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign39270_e44697_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign39270_e44697_d_n9;
        locals.var_q_lnexpnum__blk840_rv = 0.0;

        let (assign39280_e44710, assign39280_e44710_d_n4, assign39280_e44710_d_n6, assign39280_e44710_d_n7, assign39280_e44710_d_n8, assign39280_e44710_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign39280_e44708: f64 = (1.0 / locals.var_q1d__blk1001);
        (assign39280_e44708, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39280_e44710;
        locals.var_q_temp1__blk814_dn4 = assign39280_e44710_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39280_e44710_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39280_e44710_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39280_e44710_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39280_e44710_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_114(
        locals: &mut StampLocals,
    ) {
        let (assign39290_e44723, assign39290_e44723_d_n4, assign39290_e44723_d_n6, assign39290_e44723_d_n7, assign39290_e44723_d_n8, assign39290_e44723_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign39290_e44721: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign39290_e44721, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign39290_e44723;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign39290_e44723_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign39290_e44723_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign39290_e44723_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign39290_e44723_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign39290_e44723_d_n9;
        locals.var_q_d1_lnexpnum__blk841_rv = 0.0;

        let (assign39300_e44737, assign39300_e44737_d_n4, assign39300_e44737_d_n6, assign39300_e44737_d_n7, assign39300_e44737_d_n8, assign39300_e44737_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign39300_e44733: f64 = (-locals.var_q_temp1__blk814);
        let assign39300_e44735: f64 = (assign39300_e44733 * locals.var_q_temp1__blk814);
        (assign39300_e44735, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign39300_e44733 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign39300_e44733 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign39300_e44733 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign39300_e44733 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign39300_e44733 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign39300_e44737;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign39300_e44737_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign39300_e44737_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign39300_e44737_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign39300_e44737_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign39300_e44737_d_n9;
        locals.var_q_d2_lnexpnum__blk842_rv = 0.0;

        let (assign39310_e44755, assign39310_e44755_d_n4, assign39310_e44755_d_n6, assign39310_e44755_d_n7, assign39310_e44755_d_n8, assign39310_e44755_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39310_e44745: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign39310_e44747: f64 = (assign39310_e44745 + locals.var_q1d__blk1001);
        let assign39310_e44750: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign39310_e44751: f64 = (assign39310_e44747 + assign39310_e44750);
        let assign39310_e44753: f64 = (assign39310_e44751 - locals.var_q_ln_term__blk834);
        (assign39310_e44753, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign39310_e44755;
        locals.var_q_q2_int__blk843_dn4 = assign39310_e44755_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign39310_e44755_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign39310_e44755_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign39310_e44755_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign39310_e44755_d_n9;
        locals.var_q_q2_int__blk843_rv = 0.0;

        let (assign39320_e44769, assign39320_e44769_d_n4, assign39320_e44769_d_n6, assign39320_e44769_d_n7, assign39320_e44769_d_n8, assign39320_e44769_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39320_e44764: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign39320_e44765: f64 = (1.0 + assign39320_e44764);
        let assign39320_e44767: f64 = (assign39320_e44765 - locals.var_q_d1_ln__blk835);
        (assign39320_e44767, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign39320_e44769;
        locals.var_q_d1_q2__blk844_dn4 = assign39320_e44769_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign39320_e44769_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign39320_e44769_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign39320_e44769_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign39320_e44769_d_n9;
        locals.var_q_d1_q2__blk844_rv = 0.0;

        let (assign39330_e44781, assign39330_e44781_d_n4, assign39330_e44781_d_n6, assign39330_e44781_d_n7, assign39330_e44781_d_n8, assign39330_e44781_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39330_e44777: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign39330_e44779: f64 = (assign39330_e44777 - locals.var_q_d2_ln__blk836);
        (assign39330_e44779, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign39330_e44781;
        locals.var_q_d2_q2__blk845_dn4 = assign39330_e44781_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign39330_e44781_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign39330_e44781_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign39330_e44781_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign39330_e44781_d_n9;
        locals.var_q_d2_q2__blk845_rv = 0.0;

        let (assign39340_e44793, assign39340_e44793_d_n4, assign39340_e44793_d_n6, assign39340_e44793_d_n7, assign39340_e44793_d_n8, assign39340_e44793_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39340_e44790: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign39340_e44791: f64 = (locals.var_q_k1q1__blk823 + assign39340_e44790);
        (assign39340_e44791, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign39340_e44793;
        locals.var_q_qi_int__blk846_dn4 = assign39340_e44793_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign39340_e44793_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign39340_e44793_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign39340_e44793_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign39340_e44793_d_n9;
        locals.var_q_qi_int__blk846_rv = 0.0;

        let (assign39350_e44805, assign39350_e44805_d_n4, assign39350_e44805_d_n6, assign39350_e44805_d_n7, assign39350_e44805_d_n8, assign39350_e44805_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39350_e44802: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign39350_e44803: f64 = (locals.var_k1__blk932 + assign39350_e44802);
        (assign39350_e44803, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign39350_e44805;
        locals.var_q_d1_qi__blk847_dn4 = assign39350_e44805_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign39350_e44805_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign39350_e44805_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign39350_e44805_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign39350_e44805_d_n9;
        locals.var_q_d1_qi__blk847_rv = 0.0;

        let (assign39360_e44815, assign39360_e44815_d_n4, assign39360_e44815_d_n6, assign39360_e44815_d_n7, assign39360_e44815_d_n8, assign39360_e44815_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39360_e44813: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign39360_e44813, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign39360_e44815;
        locals.var_q_d2_qi__blk848_dn4 = assign39360_e44815_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign39360_e44815_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign39360_e44815_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign39360_e44815_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign39360_e44815_d_n9;
        locals.var_q_d2_qi__blk848_rv = 0.0;

        let (assign39370_e44827, assign39370_e44827_d_n4, assign39370_e44827_d_n6, assign39370_e44827_d_n7, assign39370_e44827_d_n8, assign39370_e44827_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39370_e44823: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign39370_e44825: f64 = (assign39370_e44823 - locals.var_q_aexp__blk824);
        (assign39370_e44825, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign39370_e44827;
        locals.var_q_zero__blk849_dn4 = assign39370_e44827_d_n4;
        locals.var_q_zero__blk849_dn6 = assign39370_e44827_d_n6;
        locals.var_q_zero__blk849_dn7 = assign39370_e44827_d_n7;
        locals.var_q_zero__blk849_dn8 = assign39370_e44827_d_n8;
        locals.var_q_zero__blk849_dn9 = assign39370_e44827_d_n9;
        locals.var_q_zero__blk849_rv = 0.0;

        let (assign39380_e44843, assign39380_e44843_d_n4, assign39380_e44843_d_n6, assign39380_e44843_d_n7, assign39380_e44843_d_n8, assign39380_e44843_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39380_e44835: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign39380_e44838: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign39380_e44839: f64 = (assign39380_e44835 + assign39380_e44838);
        let assign39380_e44841: f64 = (assign39380_e44839 + locals.var_q_aexp__blk824);
        (assign39380_e44841, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign39380_e44843;
        locals.var_q_d1_zero__blk850_dn4 = assign39380_e44843_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign39380_e44843_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign39380_e44843_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign39380_e44843_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign39380_e44843_d_n9;
        locals.var_q_d1_zero__blk850_rv = 0.0;

        let (assign39390_e44865, assign39390_e44865_d_n4, assign39390_e44865_d_n6, assign39390_e44865_d_n7, assign39390_e44865_d_n8, assign39390_e44865_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39390_e44851: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign39390_e44854: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign39390_e44856: f64 = (assign39390_e44854 * locals.var_q_d1_expnum__blk838);
        let assign39390_e44857: f64 = (assign39390_e44851 + assign39390_e44856);
        let assign39390_e44860: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign39390_e44861: f64 = (assign39390_e44857 + assign39390_e44860);
        let assign39390_e44863: f64 = (assign39390_e44861 - locals.var_q_aexp__blk824);
        (assign39390_e44863, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign39390_e44854 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign39390_e44854 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign39390_e44854 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign39390_e44854 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign39390_e44854 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign39390_e44865;
        locals.var_q_d2_zero__blk851_dn4 = assign39390_e44865_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign39390_e44865_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign39390_e44865_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign39390_e44865_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign39390_e44865_d_n9;
        locals.var_q_d2_zero__blk851_rv = 0.0;

        let (assign39400_e44881, assign39400_e44881_d_n4, assign39400_e44881_d_n6, assign39400_e44881_d_n7, assign39400_e44881_d_n8, assign39400_e44881_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39400_e44873: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign39400_e44876: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign39400_e44878: f64 = (assign39400_e44876 * locals.var_q_d2_zero__blk851);
        let assign39400_e44879: f64 = (assign39400_e44873 - assign39400_e44878);
        (assign39400_e44879, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign39400_e44876 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign39400_e44876 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign39400_e44876 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign39400_e44876 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign39400_e44876 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign39400_e44881;
        locals.var_q_temp__blk860_dn4 = assign39400_e44881_d_n4;
        locals.var_q_temp__blk860_dn6 = assign39400_e44881_d_n6;
        locals.var_q_temp__blk860_dn7 = assign39400_e44881_d_n7;
        locals.var_q_temp__blk860_dn8 = assign39400_e44881_d_n8;
        locals.var_q_temp__blk860_dn9 = assign39400_e44881_d_n9;
        locals.var_q_temp__blk860_rv = 0.0;

        let (assign39410_e44900, assign39410_e44900_d_n4, assign39410_e44900_d_n6, assign39410_e44900_d_n7, assign39410_e44900_d_n8, assign39410_e44900_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39410_e44888: f64 = (-locals.var_q_zero__blk849);
        let assign39410_e44890: f64 = (assign39410_e44888 * locals.var_q_d1_zero__blk850);
        let assign39410_e44892: f64 = (assign39410_e44890 * locals.var_q_temp__blk860);
        let assign39410_e44895: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign39410_e44897: f64 = (assign39410_e44895 + 1e-200);
        let assign39410_e44898: f64 = (assign39410_e44892 / assign39410_e44897);
        (assign39410_e44898, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign39410_e44888 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign39410_e44890 * locals.var_q_temp__blk860_dn4)) * assign39410_e44897) - (assign39410_e44892 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign39410_e44897 * assign39410_e44897)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign39410_e44888 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign39410_e44890 * locals.var_q_temp__blk860_dn6)) * assign39410_e44897) - (assign39410_e44892 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign39410_e44897 * assign39410_e44897)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign39410_e44888 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign39410_e44890 * locals.var_q_temp__blk860_dn7)) * assign39410_e44897) - (assign39410_e44892 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign39410_e44897 * assign39410_e44897)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign39410_e44888 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign39410_e44890 * locals.var_q_temp__blk860_dn8)) * assign39410_e44897) - (assign39410_e44892 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign39410_e44897 * assign39410_e44897)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign39410_e44888 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign39410_e44890 * locals.var_q_temp__blk860_dn9)) * assign39410_e44897) - (assign39410_e44892 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign39410_e44897 * assign39410_e44897)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign39410_e44900;
        locals.var_q_eps2__blk852_dn4 = assign39410_e44900_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign39410_e44900_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign39410_e44900_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign39410_e44900_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign39410_e44900_d_n9;
        locals.var_q_eps2__blk852_rv = 0.0;

        let (assign39420_e44910, assign39420_e44910_d_n4, assign39420_e44910_d_n6, assign39420_e44910_d_n7, assign39420_e44910_d_n8, assign39420_e44910_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39420_e44908: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
        (assign39420_e44908, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign39420_e44910;
        locals.var_q1d__blk1001_dn4 = assign39420_e44910_d_n4;
        locals.var_q1d__blk1001_dn6 = assign39420_e44910_d_n6;
        locals.var_q1d__blk1001_dn7 = assign39420_e44910_d_n7;
        locals.var_q1d__blk1001_dn8 = assign39420_e44910_d_n8;
        locals.var_q1d__blk1001_dn9 = assign39420_e44910_d_n9;
        locals.var_q1d__blk1001_rv = 0.0;

        let (assign39430_e44916, assign39430_e44916_d_n4, assign39430_e44916_d_n6, assign39430_e44916_d_n7, assign39430_e44916_d_n8, assign39430_e44916_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign39430_e44914: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign39430_e44914, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_k1q1d__blk1004, locals.var_k1q1d__blk1004_dn4, locals.var_k1q1d__blk1004_dn6, locals.var_k1q1d__blk1004_dn7, locals.var_k1q1d__blk1004_dn8, locals.var_k1q1d__blk1004_dn9,)
    }
};
        locals.var_k1q1d__blk1004 = assign39430_e44916;
        locals.var_k1q1d__blk1004_dn4 = assign39430_e44916_d_n4;
        locals.var_k1q1d__blk1004_dn6 = assign39430_e44916_d_n6;
        locals.var_k1q1d__blk1004_dn7 = assign39430_e44916_d_n7;
        locals.var_k1q1d__blk1004_dn8 = assign39430_e44916_d_n8;
        locals.var_k1q1d__blk1004_dn9 = assign39430_e44916_d_n9;
        locals.var_k1q1d__blk1004_rv = 0.0;

        let assign39440_e44919: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39440_e44921: f64 = (assign39440_e44919 - locals.var_xdeff__blk1000);
        let assign39440_e44923: f64 = if assign39440_e44921 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign39440_e44923;
        locals.var_guard1205_rv = 0.0;

        let (assign39450_e44934, assign39450_e44934_d_n4, assign39450_e44934_d_n6, assign39450_e44934_d_n7, assign39450_e44934_d_n8, assign39450_e44934_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1205 != 0.0)) {
        let assign39450_e44929: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39450_e44931: f64 = (assign39450_e44929 - locals.var_xdeff__blk1000);
        let assign39450_e44932: f64 = (assign39450_e44931).exp();
        (assign39450_e44932, (assign39450_e44932 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign39450_e44932 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign39450_e44932 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign39450_e44932 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign39450_e44932 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39450_e44934;
        locals.var_q_temp1__blk814_dn4 = assign39450_e44934_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39450_e44934_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39450_e44934_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39450_e44934_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39450_e44934_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign39460_e44975, assign39460_e44975_d_n4, assign39460_e44975_d_n6, assign39460_e44975_d_n7, assign39460_e44975_d_n8, assign39460_e44975_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign39460_e44943: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39460_e44945: f64 = (assign39460_e44943 - locals.var_xdeff__blk1000);
        let assign39460_e44947: f64 = (assign39460_e44945 - 80.0);
        let assign39460_e44952: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39460_e44954: f64 = (assign39460_e44952 - locals.var_xdeff__blk1000);
        let assign39460_e44956: f64 = (assign39460_e44954 - 80.0);
        let assign39460_e44957: f64 = (0.5 * assign39460_e44956);
        let assign39460_e44961: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39460_e44963: f64 = (assign39460_e44961 - locals.var_xdeff__blk1000);
        let assign39460_e44965: f64 = (assign39460_e44963 - 80.0);
        let assign39460_e44967: f64 = (assign39460_e44965 * 0.3333333333333);
        let assign39460_e44968: f64 = (1.0 + assign39460_e44967);
        let assign39460_e44969: f64 = (assign39460_e44957 * assign39460_e44968);
        let assign39460_e44970: f64 = (1.0 + assign39460_e44969);
        let assign39460_e44971: f64 = (assign39460_e44947 * assign39460_e44970);
        let assign39460_e44972: f64 = (1.0 + assign39460_e44971);
        let assign39460_e44973: f64 = (5.54062e34 * assign39460_e44972);
        (assign39460_e44973, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign39460_e44970) + (assign39460_e44947 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign39460_e44968) + (assign39460_e44957 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign39460_e44970) + (assign39460_e44947 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign39460_e44968) + (assign39460_e44957 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign39460_e44970) + (assign39460_e44947 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign39460_e44968) + (assign39460_e44957 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign39460_e44970) + (assign39460_e44947 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign39460_e44968) + (assign39460_e44957 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign39460_e44970) + (assign39460_e44947 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign39460_e44968) + (assign39460_e44957 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39460_e44975;
        locals.var_q_temp1__blk814_dn4 = assign39460_e44975_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39460_e44975_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39460_e44975_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39460_e44975_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39460_e44975_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign39470_e44981, assign39470_e44981_d_n4, assign39470_e44981_d_n6, assign39470_e44981_d_n7, assign39470_e44981_d_n8, assign39470_e44981_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign39470_e44979: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign39470_e44979, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_aexp1d__blk1007, locals.var_aexp1d__blk1007_dn4, locals.var_aexp1d__blk1007_dn6, locals.var_aexp1d__blk1007_dn7, locals.var_aexp1d__blk1007_dn8, locals.var_aexp1d__blk1007_dn9,)
    }
};
        locals.var_aexp1d__blk1007 = assign39470_e44981;
        locals.var_aexp1d__blk1007_dn4 = assign39470_e44981_d_n4;
        locals.var_aexp1d__blk1007_dn6 = assign39470_e44981_d_n6;
        locals.var_aexp1d__blk1007_dn7 = assign39470_e44981_d_n7;
        locals.var_aexp1d__blk1007_dn8 = assign39470_e44981_d_n8;
        locals.var_aexp1d__blk1007_dn9 = assign39470_e44981_d_n9;
        locals.var_aexp1d__blk1007_rv = 0.0;

        let (assign39480_e44989, assign39480_e44989_d_n4, assign39480_e44989_d_n6, assign39480_e44989_d_n7, assign39480_e44989_d_n8, assign39480_e44989_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign39480_e44985: f64 = (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004);
        let assign39480_e44987: f64 = (assign39480_e44985 - locals.var_aexp1d__blk1007);
        (assign39480_e44987, (((locals.var_k1q1d__blk1004_dn4 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn4)) - locals.var_aexp1d__blk1007_dn4), (((locals.var_k1q1d__blk1004_dn6 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn6)) - locals.var_aexp1d__blk1007_dn6), (((locals.var_k1q1d__blk1004_dn7 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn7)) - locals.var_aexp1d__blk1007_dn7), (((locals.var_k1q1d__blk1004_dn8 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn8)) - locals.var_aexp1d__blk1007_dn8), (((locals.var_k1q1d__blk1004_dn9 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn9)) - locals.var_aexp1d__blk1007_dn9),)
    } else {
        (locals.var_qsqd__blk1006, locals.var_qsqd__blk1006_dn4, locals.var_qsqd__blk1006_dn6, locals.var_qsqd__blk1006_dn7, locals.var_qsqd__blk1006_dn8, locals.var_qsqd__blk1006_dn9,)
    }
};
        locals.var_qsqd__blk1006 = assign39480_e44989;
        locals.var_qsqd__blk1006_dn4 = assign39480_e44989_d_n4;
        locals.var_qsqd__blk1006_dn6 = assign39480_e44989_d_n6;
        locals.var_qsqd__blk1006_dn7 = assign39480_e44989_d_n7;
        locals.var_qsqd__blk1006_dn8 = assign39480_e44989_d_n8;
        locals.var_qsqd__blk1006_dn9 = assign39480_e44989_d_n9;
        locals.var_qsqd__blk1006_rv = 0.0;

        let assign39490_e44992: f64 = if locals.var_aexp1d__blk1007 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign39490_e44992;
        locals.var_guard1206_rv = 0.0;

        let (assign39500_e44998, assign39500_e44998_d_n4, assign39500_e44998_d_n6, assign39500_e44998_d_n7, assign39500_e44998_d_n8, assign39500_e44998_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9,)
    }
};
        locals.var_qid__blk1003 = assign39500_e44998;
        locals.var_qid__blk1003_dn4 = assign39500_e44998_d_n4;
        locals.var_qid__blk1003_dn6 = assign39500_e44998_d_n6;
        locals.var_qid__blk1003_dn7 = assign39500_e44998_d_n7;
        locals.var_qid__blk1003_dn8 = assign39500_e44998_d_n8;
        locals.var_qid__blk1003_dn9 = assign39500_e44998_d_n9;
        locals.var_qid__blk1003_rv = 0.0;

        let (assign39510_e45006, assign39510_e45006_d_n4, assign39510_e45006_d_n6, assign39510_e45006_d_n7, assign39510_e45006_d_n8, assign39510_e45006_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
        let assign39510_e45004: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
        (assign39510_e45004, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9),)
    } else {
        (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9,)
    }
};
        locals.var_k2q2d__blk1005 = assign39510_e45006;
        locals.var_k2q2d__blk1005_dn4 = assign39510_e45006_d_n4;
        locals.var_k2q2d__blk1005_dn6 = assign39510_e45006_d_n6;
        locals.var_k2q2d__blk1005_dn7 = assign39510_e45006_d_n7;
        locals.var_k2q2d__blk1005_dn8 = assign39510_e45006_d_n8;
        locals.var_k2q2d__blk1005_dn9 = assign39510_e45006_d_n9;
        locals.var_k2q2d__blk1005_rv = 0.0;

        let (assign39520_e45014, assign39520_e45014_d_n4, assign39520_e45014_d_n6, assign39520_e45014_d_n7, assign39520_e45014_d_n8, assign39520_e45014_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
        let assign39520_e45012: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
        (assign39520_e45012, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)),)
    } else {
        (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9,)
    }
};
        locals.var_q2d__blk1002 = assign39520_e45014;
        locals.var_q2d__blk1002_dn4 = assign39520_e45014_d_n4;
        locals.var_q2d__blk1002_dn6 = assign39520_e45014_d_n6;
        locals.var_q2d__blk1002_dn7 = assign39520_e45014_d_n7;
        locals.var_q2d__blk1002_dn8 = assign39520_e45014_d_n8;
        locals.var_q2d__blk1002_dn9 = assign39520_e45014_d_n9;
        locals.var_q2d__blk1002_rv = 0.0;

        let assign39530_e45017: f64 = (-0.005);
        let assign39530_e45018: f64 = if locals.var_qsqd__blk1006 < assign39530_e45017 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign39530_e45018;
        locals.var_guard1207_rv = 0.0;

        let (assign39540_e45029, assign39540_e45029_d_n4, assign39540_e45029_d_n6, assign39540_e45029_d_n7, assign39540_e45029_d_n8, assign39540_e45029_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
        let assign39540_e45026: f64 = (locals.var_qsqd__blk1006).abs();
        let assign39540_e45027: f64 = (assign39540_e45026).sqrt();
        (assign39540_e45027, (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn4 } else { (-locals.var_qsqd__blk1006_dn4) } / (2.0 * assign39540_e45027)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn6 } else { (-locals.var_qsqd__blk1006_dn6) } / (2.0 * assign39540_e45027)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn7 } else { (-locals.var_qsqd__blk1006_dn7) } / (2.0 * assign39540_e45027)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn8 } else { (-locals.var_qsqd__blk1006_dn8) } / (2.0 * assign39540_e45027)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn9 } else { (-locals.var_qsqd__blk1006_dn9) } / (2.0 * assign39540_e45027)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign39540_e45029;
        locals.var_q_rac_qsq__blk828_dn4 = assign39540_e45029_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign39540_e45029_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign39540_e45029_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign39540_e45029_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign39540_e45029_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign39550_e45043, assign39550_e45043_d_n4, assign39550_e45043_d_n6, assign39550_e45043_d_n7, assign39550_e45043_d_n8, assign39550_e45043_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
        let assign39550_e45039: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign39550_e45040: f64 = (assign39550_e45039).tan();
        let assign39550_e45041: f64 = (locals.var_q_rac_qsq__blk828 / assign39550_e45040);
        (assign39550_e45041, (((locals.var_q_rac_qsq__blk828_dn4 * assign39550_e45040) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign39550_e45039).cos() * (assign39550_e45039).cos())))) / (assign39550_e45040 * assign39550_e45040)), (((locals.var_q_rac_qsq__blk828_dn6 * assign39550_e45040) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign39550_e45039).cos() * (assign39550_e45039).cos())))) / (assign39550_e45040 * assign39550_e45040)), (((locals.var_q_rac_qsq__blk828_dn7 * assign39550_e45040) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign39550_e45039).cos() * (assign39550_e45039).cos())))) / (assign39550_e45040 * assign39550_e45040)), (((locals.var_q_rac_qsq__blk828_dn8 * assign39550_e45040) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign39550_e45039).cos() * (assign39550_e45039).cos())))) / (assign39550_e45040 * assign39550_e45040)), (((locals.var_q_rac_qsq__blk828_dn9 * assign39550_e45040) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign39550_e45039).cos() * (assign39550_e45039).cos())))) / (assign39550_e45040 * assign39550_e45040)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign39550_e45043;
        locals.var_q_qcoth__blk829_dn4 = assign39550_e45043_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign39550_e45043_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign39550_e45043_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign39550_e45043_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign39550_e45043_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let assign39560_e45046: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1208 = assign39560_e45046;
        locals.var_guard1208_rv = 0.0;

        let (assign39570_e45060, assign39570_e45060_d_n4, assign39570_e45060_d_n6, assign39570_e45060_d_n7, assign39570_e45060_d_n8, assign39570_e45060_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
        let assign39570_e45057: f64 = (locals.var_qsqd__blk1006).abs();
        let assign39570_e45058: f64 = (assign39570_e45057).sqrt();
        (assign39570_e45058, (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn4 } else { (-locals.var_qsqd__blk1006_dn4) } / (2.0 * assign39570_e45058)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn6 } else { (-locals.var_qsqd__blk1006_dn6) } / (2.0 * assign39570_e45058)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn7 } else { (-locals.var_qsqd__blk1006_dn7) } / (2.0 * assign39570_e45058)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn8 } else { (-locals.var_qsqd__blk1006_dn8) } / (2.0 * assign39570_e45058)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn9 } else { (-locals.var_qsqd__blk1006_dn9) } / (2.0 * assign39570_e45058)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign39570_e45060;
        locals.var_q_rac_qsq__blk828_dn4 = assign39570_e45060_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign39570_e45060_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign39570_e45060_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign39570_e45060_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign39570_e45060_d_n9;
        locals.var_q_rac_qsq__blk828_rv = 0.0;

        let (assign39580_e45074, assign39580_e45074_d_n4, assign39580_e45074_d_n6, assign39580_e45074_d_n7, assign39580_e45074_d_n8, assign39580_e45074_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
        let assign39580_e45071: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign39580_e45072: f64 = (assign39580_e45071).exp();
        (assign39580_e45072, (assign39580_e45072 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign39580_e45072 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign39580_e45072 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign39580_e45072 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign39580_e45072 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign39580_e45074;
        locals.var_q_invexpq__blk831_dn4 = assign39580_e45074_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign39580_e45074_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign39580_e45074_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign39580_e45074_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign39580_e45074_d_n9;
        locals.var_q_invexpq__blk831_rv = 0.0;

        let (assign39590_e45094, assign39590_e45094_d_n4, assign39590_e45094_d_n6, assign39590_e45094_d_n7, assign39590_e45094_d_n8, assign39590_e45094_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
        let assign39590_e45087: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign39590_e45088: f64 = (locals.var_q_rac_qsq__blk828 * assign39590_e45087);
        let assign39590_e45091: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign39590_e45092: f64 = (assign39590_e45088 / assign39590_e45091);
        (assign39590_e45092, (((((locals.var_q_rac_qsq__blk828_dn4 * assign39590_e45087) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign39590_e45091) - (assign39590_e45088 * (-locals.var_q_invexpq__blk831_dn4))) / (assign39590_e45091 * assign39590_e45091)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign39590_e45087) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign39590_e45091) - (assign39590_e45088 * (-locals.var_q_invexpq__blk831_dn6))) / (assign39590_e45091 * assign39590_e45091)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign39590_e45087) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign39590_e45091) - (assign39590_e45088 * (-locals.var_q_invexpq__blk831_dn7))) / (assign39590_e45091 * assign39590_e45091)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign39590_e45087) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign39590_e45091) - (assign39590_e45088 * (-locals.var_q_invexpq__blk831_dn8))) / (assign39590_e45091 * assign39590_e45091)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign39590_e45087) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign39590_e45091) - (assign39590_e45088 * (-locals.var_q_invexpq__blk831_dn9))) / (assign39590_e45091 * assign39590_e45091)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign39590_e45094;
        locals.var_q_qcoth__blk829_dn4 = assign39590_e45094_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign39590_e45094_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign39590_e45094_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign39590_e45094_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign39590_e45094_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_115(
        locals: &mut StampLocals,
    ) {
        let (assign39600_e45123, assign39600_e45123_d_n4, assign39600_e45123_d_n6, assign39600_e45123_d_n7, assign39600_e45123_d_n8, assign39600_e45123_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 == 0.0)) {
        let assign39600_e45108: f64 = (locals.var_qsqd__blk1006 * 0.1666666666667);
        let assign39600_e45112: f64 = (locals.var_qsqd__blk1006 * 0.0166666666667);
        let assign39600_e45116: f64 = (locals.var_qsqd__blk1006 * 0.0238095238095);
        let assign39600_e45117: f64 = (1.0 - assign39600_e45116);
        let assign39600_e45118: f64 = (assign39600_e45112 * assign39600_e45117);
        let assign39600_e45119: f64 = (1.0 - assign39600_e45118);
        let assign39600_e45120: f64 = (assign39600_e45108 * assign39600_e45119);
        let assign39600_e45121: f64 = (2.0 + assign39600_e45120);
        (assign39600_e45121, (((locals.var_qsqd__blk1006_dn4 * 0.1666666666667) * assign39600_e45119) + (assign39600_e45108 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0166666666667) * assign39600_e45117) + (assign39600_e45112 * (-(locals.var_qsqd__blk1006_dn4 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn6 * 0.1666666666667) * assign39600_e45119) + (assign39600_e45108 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0166666666667) * assign39600_e45117) + (assign39600_e45112 * (-(locals.var_qsqd__blk1006_dn6 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn7 * 0.1666666666667) * assign39600_e45119) + (assign39600_e45108 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0166666666667) * assign39600_e45117) + (assign39600_e45112 * (-(locals.var_qsqd__blk1006_dn7 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn8 * 0.1666666666667) * assign39600_e45119) + (assign39600_e45108 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0166666666667) * assign39600_e45117) + (assign39600_e45112 * (-(locals.var_qsqd__blk1006_dn8 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn9 * 0.1666666666667) * assign39600_e45119) + (assign39600_e45108 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0166666666667) * assign39600_e45117) + (assign39600_e45112 * (-(locals.var_qsqd__blk1006_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign39600_e45123;
        locals.var_q_qcoth__blk829_dn4 = assign39600_e45123_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign39600_e45123_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign39600_e45123_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign39600_e45123_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign39600_e45123_d_n9;
        locals.var_q_qcoth__blk829_rv = 0.0;

        let assign39610_e45126: f64 = (1.01 * locals.var_k1q1d__blk1004);
        let assign39610_e45128: f64 = (assign39610_e45126 + locals.var_q_qcoth__blk829);
        let assign39610_e45130: f64 = if assign39610_e45128 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign39610_e45130;
        locals.var_guard1209_rv = 0.0;

        let (assign39620_e45141, assign39620_e45141_d_n4, assign39620_e45141_d_n6, assign39620_e45141_d_n7, assign39620_e45141_d_n8, assign39620_e45141_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) {
        let assign39620_e45139: f64 = (locals.var_k1q1d__blk1004 + locals.var_q_qcoth__blk829);
        (assign39620_e45139, (locals.var_k1q1d__blk1004_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_k1q1d__blk1004_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_k1q1d__blk1004_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_k1q1d__blk1004_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_k1q1d__blk1004_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39620_e45141;
        locals.var_q_temp1__blk814_dn4 = assign39620_e45141_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39620_e45141_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39620_e45141_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39620_e45141_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39620_e45141_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let assign39630_e45144: f64 = (locals.var_aexp1d__blk1007 * locals.var_k1q1d__blk1004);
        let assign39630_e45147: f64 = (0.9 * locals.var_k1q1d__blk1004);
        let assign39630_e45149: f64 = (assign39630_e45147 * locals.var_k1q1d__blk1004);
        let assign39630_e45151: f64 = (assign39630_e45149 * locals.var_q_temp1__blk814);
        let assign39630_e45152: f64 = if assign39630_e45144 < assign39630_e45151 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign39630_e45152;
        locals.var_guard1210_rv = 0.0;

        let (assign39640_e45167, assign39640_e45167_d_n4, assign39640_e45167_d_n6, assign39640_e45167_d_n7, assign39640_e45167_d_n8, assign39640_e45167_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign39640_e45163: f64 = (locals.var_aexp1d__blk1007 / locals.var_q_temp1__blk814);
        let assign39640_e45165: f64 = (assign39640_e45163 + 1e-80);
        (assign39640_e45165, (((locals.var_aexp1d__blk1007_dn4 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn4)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn6 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn6)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn7 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn7)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn8 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn8)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn9 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn9)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)),)
    } else {
        (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9,)
    }
};
        locals.var_qid__blk1003 = assign39640_e45167;
        locals.var_qid__blk1003_dn4 = assign39640_e45167_d_n4;
        locals.var_qid__blk1003_dn6 = assign39640_e45167_d_n6;
        locals.var_qid__blk1003_dn7 = assign39640_e45167_d_n7;
        locals.var_qid__blk1003_dn8 = assign39640_e45167_d_n8;
        locals.var_qid__blk1003_dn9 = assign39640_e45167_d_n9;
        locals.var_qid__blk1003_rv = 0.0;

        let (assign39650_e45180, assign39650_e45180_d_n4, assign39650_e45180_d_n6, assign39650_e45180_d_n7, assign39650_e45180_d_n8, assign39650_e45180_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign39650_e45178: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
        (assign39650_e45178, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9),)
    } else {
        (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9,)
    }
};
        locals.var_k2q2d__blk1005 = assign39650_e45180;
        locals.var_k2q2d__blk1005_dn4 = assign39650_e45180_d_n4;
        locals.var_k2q2d__blk1005_dn6 = assign39650_e45180_d_n6;
        locals.var_k2q2d__blk1005_dn7 = assign39650_e45180_d_n7;
        locals.var_k2q2d__blk1005_dn8 = assign39650_e45180_d_n8;
        locals.var_k2q2d__blk1005_dn9 = assign39650_e45180_d_n9;
        locals.var_k2q2d__blk1005_rv = 0.0;

        let (assign39660_e45193, assign39660_e45193_d_n4, assign39660_e45193_d_n6, assign39660_e45193_d_n7, assign39660_e45193_d_n8, assign39660_e45193_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign39660_e45191: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
        (assign39660_e45191, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)),)
    } else {
        (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9,)
    }
};
        locals.var_q2d__blk1002 = assign39660_e45193;
        locals.var_q2d__blk1002_dn4 = assign39660_e45193_d_n4;
        locals.var_q2d__blk1002_dn6 = assign39660_e45193_d_n6;
        locals.var_q2d__blk1002_dn7 = assign39660_e45193_d_n7;
        locals.var_q2d__blk1002_dn8 = assign39660_e45193_d_n8;
        locals.var_q2d__blk1002_dn9 = assign39660_e45193_d_n9;
        locals.var_q2d__blk1002_rv = 0.0;

        let assign39670_e45196: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign39670_e45196;
        locals.var_guard1211_rv = 0.0;

        let (assign39680_e45223, assign39680_e45223_d_n4, assign39680_e45223_d_n6, assign39680_e45223_d_n7, assign39680_e45223_d_n8, assign39680_e45223_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 != 0.0)) {
        let assign39680_e45210: f64 = (4.0 * locals.var_qsqd__blk1006);
        let assign39680_e45215: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign39680_e45216: f64 = (locals.var_q_invexpq__blk831 * assign39680_e45215);
        let assign39680_e45217: f64 = (1.0 - assign39680_e45216);
        let assign39680_e45218: f64 = (assign39680_e45210 / assign39680_e45217);
        let assign39680_e45219: f64 = (assign39680_e45218).ln();
        let assign39680_e45221: f64 = (assign39680_e45219 - locals.var_q_rac_qsq__blk828);
        (assign39680_e45221, ((((((4.0 * locals.var_qsqd__blk1006_dn4) * assign39680_e45217) - (assign39680_e45210 * (-((locals.var_q_invexpq__blk831_dn4 * assign39680_e45215) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39680_e45217 * assign39680_e45217)) / assign39680_e45218) - locals.var_q_rac_qsq__blk828_dn4), ((((((4.0 * locals.var_qsqd__blk1006_dn6) * assign39680_e45217) - (assign39680_e45210 * (-((locals.var_q_invexpq__blk831_dn6 * assign39680_e45215) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39680_e45217 * assign39680_e45217)) / assign39680_e45218) - locals.var_q_rac_qsq__blk828_dn6), ((((((4.0 * locals.var_qsqd__blk1006_dn7) * assign39680_e45217) - (assign39680_e45210 * (-((locals.var_q_invexpq__blk831_dn7 * assign39680_e45215) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39680_e45217 * assign39680_e45217)) / assign39680_e45218) - locals.var_q_rac_qsq__blk828_dn7), ((((((4.0 * locals.var_qsqd__blk1006_dn8) * assign39680_e45217) - (assign39680_e45210 * (-((locals.var_q_invexpq__blk831_dn8 * assign39680_e45215) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39680_e45217 * assign39680_e45217)) / assign39680_e45218) - locals.var_q_rac_qsq__blk828_dn8), ((((((4.0 * locals.var_qsqd__blk1006_dn9) * assign39680_e45217) - (assign39680_e45210 * (-((locals.var_q_invexpq__blk831_dn9 * assign39680_e45215) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39680_e45217 * assign39680_e45217)) / assign39680_e45218) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39680_e45223;
        locals.var_q_temp2__blk815_dn4 = assign39680_e45223_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39680_e45223_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39680_e45223_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39680_e45223_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39680_e45223_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let assign39690_e45226: f64 = (-0.005);
        let assign39690_e45227: f64 = if locals.var_qsqd__blk1006 < assign39690_e45226 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign39690_e45227;
        locals.var_guard1212_rv = 0.0;

        let (assign39700_e45247, assign39700_e45247_d_n4, assign39700_e45247_d_n6, assign39700_e45247_d_n7, assign39700_e45247_d_n8, assign39700_e45247_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 != 0.0)) {
        let assign39700_e45244: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign39700_e45245: f64 = (assign39700_e45244).sin();
        (assign39700_e45245, ((assign39700_e45244).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39700_e45244).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39700_e45244).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39700_e45244).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39700_e45244).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign39700_e45247;
        locals.var_q_temp3__blk816_dn4 = assign39700_e45247_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign39700_e45247_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign39700_e45247_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign39700_e45247_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign39700_e45247_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign39710_e45270, assign39710_e45270_d_n4, assign39710_e45270_d_n6, assign39710_e45270_d_n7, assign39710_e45270_d_n8, assign39710_e45270_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 != 0.0)) {
        let assign39710_e45263: f64 = (-locals.var_qsqd__blk1006);
        let assign39710_e45266: f64 = (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816);
        let assign39710_e45267: f64 = (assign39710_e45263 / assign39710_e45266);
        let assign39710_e45268: f64 = (assign39710_e45267).ln();
        (assign39710_e45268, (((((-locals.var_qsqd__blk1006_dn4) * assign39710_e45266) - (assign39710_e45263 * ((locals.var_q_temp3__blk816_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn4)))) / (assign39710_e45266 * assign39710_e45266)) / assign39710_e45267), (((((-locals.var_qsqd__blk1006_dn6) * assign39710_e45266) - (assign39710_e45263 * ((locals.var_q_temp3__blk816_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn6)))) / (assign39710_e45266 * assign39710_e45266)) / assign39710_e45267), (((((-locals.var_qsqd__blk1006_dn7) * assign39710_e45266) - (assign39710_e45263 * ((locals.var_q_temp3__blk816_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn7)))) / (assign39710_e45266 * assign39710_e45266)) / assign39710_e45267), (((((-locals.var_qsqd__blk1006_dn8) * assign39710_e45266) - (assign39710_e45263 * ((locals.var_q_temp3__blk816_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn8)))) / (assign39710_e45266 * assign39710_e45266)) / assign39710_e45267), (((((-locals.var_qsqd__blk1006_dn9) * assign39710_e45266) - (assign39710_e45263 * ((locals.var_q_temp3__blk816_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn9)))) / (assign39710_e45266 * assign39710_e45266)) / assign39710_e45267),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39710_e45270;
        locals.var_q_temp2__blk815_dn4 = assign39710_e45270_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39710_e45270_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39710_e45270_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39710_e45270_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39710_e45270_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign39720_e45305, assign39720_e45305_d_n4, assign39720_e45305_d_n6, assign39720_e45305_d_n7, assign39720_e45305_d_n8, assign39720_e45305_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 == 0.0)) {
        let assign39720_e45289: f64 = (locals.var_qsqd__blk1006 * 0.3333333333333);
        let assign39720_e45293: f64 = (0.05 * locals.var_qsqd__blk1006);
        let assign39720_e45297: f64 = (0.0396825396825397 * locals.var_qsqd__blk1006);
        let assign39720_e45298: f64 = (1.0 - assign39720_e45297);
        let assign39720_e45299: f64 = (assign39720_e45293 * assign39720_e45298);
        let assign39720_e45300: f64 = (1.0 - assign39720_e45299);
        let assign39720_e45301: f64 = (assign39720_e45289 * assign39720_e45300);
        let assign39720_e45302: f64 = (4.0 - assign39720_e45301);
        let assign39720_e45303: f64 = (assign39720_e45302).ln();
        (assign39720_e45303, ((-(((locals.var_qsqd__blk1006_dn4 * 0.3333333333333) * assign39720_e45300) + (assign39720_e45289 * (-(((0.05 * locals.var_qsqd__blk1006_dn4) * assign39720_e45298) + (assign39720_e45293 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn4)))))))) / assign39720_e45302), ((-(((locals.var_qsqd__blk1006_dn6 * 0.3333333333333) * assign39720_e45300) + (assign39720_e45289 * (-(((0.05 * locals.var_qsqd__blk1006_dn6) * assign39720_e45298) + (assign39720_e45293 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn6)))))))) / assign39720_e45302), ((-(((locals.var_qsqd__blk1006_dn7 * 0.3333333333333) * assign39720_e45300) + (assign39720_e45289 * (-(((0.05 * locals.var_qsqd__blk1006_dn7) * assign39720_e45298) + (assign39720_e45293 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn7)))))))) / assign39720_e45302), ((-(((locals.var_qsqd__blk1006_dn8 * 0.3333333333333) * assign39720_e45300) + (assign39720_e45289 * (-(((0.05 * locals.var_qsqd__blk1006_dn8) * assign39720_e45298) + (assign39720_e45293 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn8)))))))) / assign39720_e45302), ((-(((locals.var_qsqd__blk1006_dn9 * 0.3333333333333) * assign39720_e45300) + (assign39720_e45289 * (-(((0.05 * locals.var_qsqd__blk1006_dn9) * assign39720_e45298) + (assign39720_e45293 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn9)))))))) / assign39720_e45302),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39720_e45305;
        locals.var_q_temp2__blk815_dn4 = assign39720_e45305_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39720_e45305_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39720_e45305_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39720_e45305_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39720_e45305_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign39730_e45328, assign39730_e45328_d_n4, assign39730_e45328_d_n6, assign39730_e45328_d_n7, assign39730_e45328_d_n8, assign39730_e45328_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign39730_e45317: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign39730_e45319: f64 = (assign39730_e45317 + locals.var_q1d__blk1001);
        let assign39730_e45322: f64 = (locals.var_q_temp1__blk814).ln();
        let assign39730_e45323: f64 = (2.0 * assign39730_e45322);
        let assign39730_e45324: f64 = (assign39730_e45319 + assign39730_e45323);
        let assign39730_e45326: f64 = (assign39730_e45324 - locals.var_q_temp2__blk815);
        (assign39730_e45326, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * (locals.var_q_temp1__blk814_dn4 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * (locals.var_q_temp1__blk814_dn6 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * (locals.var_q_temp1__blk814_dn7 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * (locals.var_q_temp1__blk814_dn8 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * (locals.var_q_temp1__blk814_dn9 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn9),)
    } else {
        (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9,)
    }
};
        locals.var_q2d__blk1002 = assign39730_e45328;
        locals.var_q2d__blk1002_dn4 = assign39730_e45328_d_n4;
        locals.var_q2d__blk1002_dn6 = assign39730_e45328_d_n6;
        locals.var_q2d__blk1002_dn7 = assign39730_e45328_d_n7;
        locals.var_q2d__blk1002_dn8 = assign39730_e45328_d_n8;
        locals.var_q2d__blk1002_dn9 = assign39730_e45328_d_n9;
        locals.var_q2d__blk1002_rv = 0.0;

        let (assign39740_e45342, assign39740_e45342_d_n4, assign39740_e45342_d_n6, assign39740_e45342_d_n7, assign39740_e45342_d_n8, assign39740_e45342_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign39740_e45340: f64 = (locals.var_k2__blk933 * locals.var_q2d__blk1002);
        (assign39740_e45340, ((locals.var_k2__blk933_dn4 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn9)),)
    } else {
        (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9,)
    }
};
        locals.var_k2q2d__blk1005 = assign39740_e45342;
        locals.var_k2q2d__blk1005_dn4 = assign39740_e45342_d_n4;
        locals.var_k2q2d__blk1005_dn6 = assign39740_e45342_d_n6;
        locals.var_k2q2d__blk1005_dn7 = assign39740_e45342_d_n7;
        locals.var_k2q2d__blk1005_dn8 = assign39740_e45342_d_n8;
        locals.var_k2q2d__blk1005_dn9 = assign39740_e45342_d_n9;
        locals.var_k2q2d__blk1005_rv = 0.0;

        let (assign39750_e45356, assign39750_e45356_d_n4, assign39750_e45356_d_n6, assign39750_e45356_d_n7, assign39750_e45356_d_n8, assign39750_e45356_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign39750_e45354: f64 = (locals.var_k1q1d__blk1004 + locals.var_k2q2d__blk1005);
        (assign39750_e45354, (locals.var_k1q1d__blk1004_dn4 + locals.var_k2q2d__blk1005_dn4), (locals.var_k1q1d__blk1004_dn6 + locals.var_k2q2d__blk1005_dn6), (locals.var_k1q1d__blk1004_dn7 + locals.var_k2q2d__blk1005_dn7), (locals.var_k1q1d__blk1004_dn8 + locals.var_k2q2d__blk1005_dn8), (locals.var_k1q1d__blk1004_dn9 + locals.var_k2q2d__blk1005_dn9),)
    } else {
        (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9,)
    }
};
        locals.var_qid__blk1003 = assign39750_e45356;
        locals.var_qid__blk1003_dn4 = assign39750_e45356_d_n4;
        locals.var_qid__blk1003_dn6 = assign39750_e45356_d_n6;
        locals.var_qid__blk1003_dn7 = assign39750_e45356_d_n7;
        locals.var_qid__blk1003_dn8 = assign39750_e45356_d_n8;
        locals.var_qid__blk1003_dn9 = assign39750_e45356_d_n9;
        locals.var_qid__blk1003_rv = 0.0;

        let assign39760_e45359: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign39760_e45359;
        locals.var_guard1213_rv = 0.0;

        let assign39770_e45362: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39770_e45364: f64 = (assign39770_e45362 - locals.var_xg1x__blk930);
        let assign39770_e45366: f64 = (assign39770_e45364 - locals.var_q_rac_qsq__blk828);
        let assign39770_e45368: f64 = if assign39770_e45366 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign39770_e45368;
        locals.var_guard1214_rv = 0.0;

        let (assign39780_e45389, assign39780_e45389_d_n4, assign39780_e45389_d_n6, assign39780_e45389_d_n7, assign39780_e45389_d_n8, assign39780_e45389_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 != 0.0)) {
        let assign39780_e45382: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39780_e45384: f64 = (assign39780_e45382 - locals.var_xg1x__blk930);
        let assign39780_e45386: f64 = (assign39780_e45384 - locals.var_q_rac_qsq__blk828);
        let assign39780_e45387: f64 = (assign39780_e45386).exp();
        (assign39780_e45387, (assign39780_e45387 * (((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4)), (assign39780_e45387 * (((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6)), (assign39780_e45387 * (((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7)), (assign39780_e45387 * (((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8)), (assign39780_e45387 * (((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign39780_e45389;
        locals.var_q_temp3__blk816_dn4 = assign39780_e45389_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign39780_e45389_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign39780_e45389_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign39780_e45389_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign39780_e45389_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign39790_e45444, assign39790_e45444_d_n4, assign39790_e45444_d_n6, assign39790_e45444_d_n7, assign39790_e45444_d_n8, assign39790_e45444_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 == 0.0)) {
        let assign39790_e45406: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39790_e45408: f64 = (assign39790_e45406 - locals.var_xg1x__blk930);
        let assign39790_e45410: f64 = (assign39790_e45408 - locals.var_q_rac_qsq__blk828);
        let assign39790_e45412: f64 = (assign39790_e45410 - 80.0);
        let assign39790_e45417: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39790_e45419: f64 = (assign39790_e45417 - locals.var_xg1x__blk930);
        let assign39790_e45421: f64 = (assign39790_e45419 - locals.var_q_rac_qsq__blk828);
        let assign39790_e45423: f64 = (assign39790_e45421 - 80.0);
        let assign39790_e45424: f64 = (0.5 * assign39790_e45423);
        let assign39790_e45428: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39790_e45430: f64 = (assign39790_e45428 - locals.var_xg1x__blk930);
        let assign39790_e45432: f64 = (assign39790_e45430 - locals.var_q_rac_qsq__blk828);
        let assign39790_e45434: f64 = (assign39790_e45432 - 80.0);
        let assign39790_e45436: f64 = (assign39790_e45434 * 0.3333333333333);
        let assign39790_e45437: f64 = (1.0 + assign39790_e45436);
        let assign39790_e45438: f64 = (assign39790_e45424 * assign39790_e45437);
        let assign39790_e45439: f64 = (1.0 + assign39790_e45438);
        let assign39790_e45440: f64 = (assign39790_e45412 * assign39790_e45439);
        let assign39790_e45441: f64 = (1.0 + assign39790_e45440);
        let assign39790_e45442: f64 = (5.54062e34 * assign39790_e45441);
        (assign39790_e45442, (5.54062e34 * (((((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4) * assign39790_e45439) + (assign39790_e45412 * (((0.5 * (((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4)) * assign39790_e45437) + (assign39790_e45424 * ((((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6) * assign39790_e45439) + (assign39790_e45412 * (((0.5 * (((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6)) * assign39790_e45437) + (assign39790_e45424 * ((((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7) * assign39790_e45439) + (assign39790_e45412 * (((0.5 * (((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7)) * assign39790_e45437) + (assign39790_e45424 * ((((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8) * assign39790_e45439) + (assign39790_e45412 * (((0.5 * (((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8)) * assign39790_e45437) + (assign39790_e45424 * ((((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9) * assign39790_e45439) + (assign39790_e45412 * (((0.5 * (((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9)) * assign39790_e45437) + (assign39790_e45424 * ((((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign39790_e45444;
        locals.var_q_temp3__blk816_dn4 = assign39790_e45444_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign39790_e45444_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign39790_e45444_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign39790_e45444_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign39790_e45444_d_n9;
        locals.var_q_temp3__blk816_rv = 0.0;

        let (assign39800_e45458, assign39800_e45458_d_n4, assign39800_e45458_d_n6, assign39800_e45458_d_n7, assign39800_e45458_d_n8, assign39800_e45458_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) {
        let assign39800_e45456: f64 = (locals.var_q_temp3__blk816 / locals.var_a0__blk905);
        (assign39800_e45456, (((locals.var_q_temp3__blk816_dn4 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn6 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn7 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn8 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn9 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39800_e45458;
        locals.var_q_temp2__blk815_dn4 = assign39800_e45458_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39800_e45458_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39800_e45458_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39800_e45458_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39800_e45458_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign39810_e45482, assign39810_e45482_d_n4, assign39810_e45482_d_n6, assign39810_e45482_d_n7, assign39810_e45482_d_n8, assign39810_e45482_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) {
        let assign39810_e45470: f64 = (4.0 * locals.var_qsqd__blk1006);
        let assign39810_e45472: f64 = (assign39810_e45470 * locals.var_q_temp2__blk815);
        let assign39810_e45477: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign39810_e45478: f64 = (locals.var_q_invexpq__blk831 * assign39810_e45477);
        let assign39810_e45479: f64 = (1.0 - assign39810_e45478);
        let assign39810_e45480: f64 = (assign39810_e45472 / assign39810_e45479);
        (assign39810_e45480, ((((((4.0 * locals.var_qsqd__blk1006_dn4) * locals.var_q_temp2__blk815) + (assign39810_e45470 * locals.var_q_temp2__blk815_dn4)) * assign39810_e45479) - (assign39810_e45472 * (-((locals.var_q_invexpq__blk831_dn4 * assign39810_e45477) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39810_e45479 * assign39810_e45479)), ((((((4.0 * locals.var_qsqd__blk1006_dn6) * locals.var_q_temp2__blk815) + (assign39810_e45470 * locals.var_q_temp2__blk815_dn6)) * assign39810_e45479) - (assign39810_e45472 * (-((locals.var_q_invexpq__blk831_dn6 * assign39810_e45477) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39810_e45479 * assign39810_e45479)), ((((((4.0 * locals.var_qsqd__blk1006_dn7) * locals.var_q_temp2__blk815) + (assign39810_e45470 * locals.var_q_temp2__blk815_dn7)) * assign39810_e45479) - (assign39810_e45472 * (-((locals.var_q_invexpq__blk831_dn7 * assign39810_e45477) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39810_e45479 * assign39810_e45479)), ((((((4.0 * locals.var_qsqd__blk1006_dn8) * locals.var_q_temp2__blk815) + (assign39810_e45470 * locals.var_q_temp2__blk815_dn8)) * assign39810_e45479) - (assign39810_e45472 * (-((locals.var_q_invexpq__blk831_dn8 * assign39810_e45477) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39810_e45479 * assign39810_e45479)), ((((((4.0 * locals.var_qsqd__blk1006_dn9) * locals.var_q_temp2__blk815) + (assign39810_e45470 * locals.var_q_temp2__blk815_dn9)) * assign39810_e45479) - (assign39810_e45472 * (-((locals.var_q_invexpq__blk831_dn9 * assign39810_e45477) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39810_e45479 * assign39810_e45479)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39810_e45482;
        locals.var_q_temp1__blk814_dn4 = assign39810_e45482_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39810_e45482_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39810_e45482_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39810_e45482_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39810_e45482_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let assign39820_e45485: f64 = (-0.005);
        let assign39820_e45486: f64 = if locals.var_qsqd__blk1006 < assign39820_e45485 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign39820_e45486;
        locals.var_guard1215_rv = 0.0;

        let (assign39830_e45504, assign39830_e45504_d_n4, assign39830_e45504_d_n6, assign39830_e45504_d_n7, assign39830_e45504_d_n8, assign39830_e45504_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign39830_e45501: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign39830_e45502: f64 = (assign39830_e45501).sin();
        (assign39830_e45502, ((assign39830_e45501).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39830_e45501).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39830_e45501).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39830_e45501).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39830_e45501).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39830_e45504;
        locals.var_q_temp2__blk815_dn4 = assign39830_e45504_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39830_e45504_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39830_e45504_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39830_e45504_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39830_e45504_d_n9;
        locals.var_q_temp2__blk815_rv = 0.0;

        let (assign39840_e45526, assign39840_e45526_d_n4, assign39840_e45526_d_n6, assign39840_e45526_d_n7, assign39840_e45526_d_n8, assign39840_e45526_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign39840_e45518: f64 = (-locals.var_qsqd__blk1006);
        let assign39840_e45521: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign39840_e45522: f64 = (assign39840_e45518 / assign39840_e45521);
        let assign39840_e45524: f64 = (assign39840_e45522 / locals.var_aexp1d__blk1007);
        (assign39840_e45524, (((((((-locals.var_qsqd__blk1006_dn4) * assign39840_e45521) - (assign39840_e45518 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign39840_e45521 * assign39840_e45521)) * locals.var_aexp1d__blk1007) - (assign39840_e45522 * locals.var_aexp1d__blk1007_dn4)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn6) * assign39840_e45521) - (assign39840_e45518 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign39840_e45521 * assign39840_e45521)) * locals.var_aexp1d__blk1007) - (assign39840_e45522 * locals.var_aexp1d__blk1007_dn6)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn7) * assign39840_e45521) - (assign39840_e45518 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign39840_e45521 * assign39840_e45521)) * locals.var_aexp1d__blk1007) - (assign39840_e45522 * locals.var_aexp1d__blk1007_dn7)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn8) * assign39840_e45521) - (assign39840_e45518 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign39840_e45521 * assign39840_e45521)) * locals.var_aexp1d__blk1007) - (assign39840_e45522 * locals.var_aexp1d__blk1007_dn8)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn9) * assign39840_e45521) - (assign39840_e45518 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign39840_e45521 * assign39840_e45521)) * locals.var_aexp1d__blk1007) - (assign39840_e45522 * locals.var_aexp1d__blk1007_dn9)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39840_e45526;
        locals.var_q_temp1__blk814_dn4 = assign39840_e45526_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39840_e45526_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39840_e45526_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39840_e45526_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39840_e45526_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign39850_e45560, assign39850_e45560_d_n4, assign39850_e45560_d_n6, assign39850_e45560_d_n7, assign39850_e45560_d_n8, assign39850_e45560_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 == 0.0)) {
        let assign39850_e45543: f64 = (locals.var_qsqd__blk1006 * 0.3333333333333);
        let assign39850_e45547: f64 = (0.05 * locals.var_qsqd__blk1006);
        let assign39850_e45551: f64 = (0.0396825396825397 * locals.var_qsqd__blk1006);
        let assign39850_e45552: f64 = (1.0 - assign39850_e45551);
        let assign39850_e45553: f64 = (assign39850_e45547 * assign39850_e45552);
        let assign39850_e45554: f64 = (1.0 - assign39850_e45553);
        let assign39850_e45555: f64 = (assign39850_e45543 * assign39850_e45554);
        let assign39850_e45556: f64 = (4.0 - assign39850_e45555);
        let assign39850_e45558: f64 = (assign39850_e45556 / locals.var_aexp1d__blk1007);
        (assign39850_e45558, ((((-(((locals.var_qsqd__blk1006_dn4 * 0.3333333333333) * assign39850_e45554) + (assign39850_e45543 * (-(((0.05 * locals.var_qsqd__blk1006_dn4) * assign39850_e45552) + (assign39850_e45547 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn4)))))))) * locals.var_aexp1d__blk1007) - (assign39850_e45556 * locals.var_aexp1d__blk1007_dn4)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn6 * 0.3333333333333) * assign39850_e45554) + (assign39850_e45543 * (-(((0.05 * locals.var_qsqd__blk1006_dn6) * assign39850_e45552) + (assign39850_e45547 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn6)))))))) * locals.var_aexp1d__blk1007) - (assign39850_e45556 * locals.var_aexp1d__blk1007_dn6)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn7 * 0.3333333333333) * assign39850_e45554) + (assign39850_e45543 * (-(((0.05 * locals.var_qsqd__blk1006_dn7) * assign39850_e45552) + (assign39850_e45547 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn7)))))))) * locals.var_aexp1d__blk1007) - (assign39850_e45556 * locals.var_aexp1d__blk1007_dn7)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn8 * 0.3333333333333) * assign39850_e45554) + (assign39850_e45543 * (-(((0.05 * locals.var_qsqd__blk1006_dn8) * assign39850_e45552) + (assign39850_e45547 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn8)))))))) * locals.var_aexp1d__blk1007) - (assign39850_e45556 * locals.var_aexp1d__blk1007_dn8)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn9 * 0.3333333333333) * assign39850_e45554) + (assign39850_e45543 * (-(((0.05 * locals.var_qsqd__blk1006_dn9) * assign39850_e45552) + (assign39850_e45547 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn9)))))))) * locals.var_aexp1d__blk1007) - (assign39850_e45556 * locals.var_aexp1d__blk1007_dn9)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39850_e45560;
        locals.var_q_temp1__blk814_dn4 = assign39850_e45560_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39850_e45560_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39850_e45560_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39850_e45560_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39850_e45560_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign39860_e45578, assign39860_e45578_d_n4, assign39860_e45578_d_n6, assign39860_e45578_d_n7, assign39860_e45578_d_n8, assign39860_e45578_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
        let assign39860_e45570: f64 = (locals.var_k1q1d__blk1004 - locals.var_q_qcoth__blk829);
        let assign39860_e45573: f64 = (1.0 - locals.var_q_temp1__blk814);
        let assign39860_e45574: f64 = (assign39860_e45570 / assign39860_e45573);
        let assign39860_e45576: f64 = (assign39860_e45574 + 1e-80);
        (assign39860_e45576, ((((locals.var_k1q1d__blk1004_dn4 - locals.var_q_qcoth__blk829_dn4) * assign39860_e45573) - (assign39860_e45570 * (-locals.var_q_temp1__blk814_dn4))) / (assign39860_e45573 * assign39860_e45573)), ((((locals.var_k1q1d__blk1004_dn6 - locals.var_q_qcoth__blk829_dn6) * assign39860_e45573) - (assign39860_e45570 * (-locals.var_q_temp1__blk814_dn6))) / (assign39860_e45573 * assign39860_e45573)), ((((locals.var_k1q1d__blk1004_dn7 - locals.var_q_qcoth__blk829_dn7) * assign39860_e45573) - (assign39860_e45570 * (-locals.var_q_temp1__blk814_dn7))) / (assign39860_e45573 * assign39860_e45573)), ((((locals.var_k1q1d__blk1004_dn8 - locals.var_q_qcoth__blk829_dn8) * assign39860_e45573) - (assign39860_e45570 * (-locals.var_q_temp1__blk814_dn8))) / (assign39860_e45573 * assign39860_e45573)), ((((locals.var_k1q1d__blk1004_dn9 - locals.var_q_qcoth__blk829_dn9) * assign39860_e45573) - (assign39860_e45570 * (-locals.var_q_temp1__blk814_dn9))) / (assign39860_e45573 * assign39860_e45573)),)
    } else {
        (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9,)
    }
};
        locals.var_qid__blk1003 = assign39860_e45578;
        locals.var_qid__blk1003_dn4 = assign39860_e45578_d_n4;
        locals.var_qid__blk1003_dn6 = assign39860_e45578_d_n6;
        locals.var_qid__blk1003_dn7 = assign39860_e45578_d_n7;
        locals.var_qid__blk1003_dn8 = assign39860_e45578_d_n8;
        locals.var_qid__blk1003_dn9 = assign39860_e45578_d_n9;
        locals.var_qid__blk1003_rv = 0.0;

        let (assign39870_e45590, assign39870_e45590_d_n4, assign39870_e45590_d_n6, assign39870_e45590_d_n7, assign39870_e45590_d_n8, assign39870_e45590_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
        let assign39870_e45588: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
        (assign39870_e45588, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9),)
    } else {
        (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9,)
    }
};
        locals.var_k2q2d__blk1005 = assign39870_e45590;
        locals.var_k2q2d__blk1005_dn4 = assign39870_e45590_d_n4;
        locals.var_k2q2d__blk1005_dn6 = assign39870_e45590_d_n6;
        locals.var_k2q2d__blk1005_dn7 = assign39870_e45590_d_n7;
        locals.var_k2q2d__blk1005_dn8 = assign39870_e45590_d_n8;
        locals.var_k2q2d__blk1005_dn9 = assign39870_e45590_d_n9;
        locals.var_k2q2d__blk1005_rv = 0.0;

        let (assign39880_e45602, assign39880_e45602_d_n4, assign39880_e45602_d_n6, assign39880_e45602_d_n7, assign39880_e45602_d_n8, assign39880_e45602_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
        let assign39880_e45600: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
        (assign39880_e45600, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)),)
    } else {
        (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9,)
    }
};
        locals.var_q2d__blk1002 = assign39880_e45602;
        locals.var_q2d__blk1002_dn4 = assign39880_e45602_d_n4;
        locals.var_q2d__blk1002_dn6 = assign39880_e45602_d_n6;
        locals.var_q2d__blk1002_dn7 = assign39880_e45602_d_n7;
        locals.var_q2d__blk1002_dn8 = assign39880_e45602_d_n8;
        locals.var_q2d__blk1002_dn9 = assign39880_e45602_d_n9;
        locals.var_q2d__blk1002_rv = 0.0;

        let assign39890_e45605: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign39890_e45607: f64 = (assign39890_e45605 - locals.var_xdeff__blk1000);
        let assign39890_e45609: f64 = if assign39890_e45607 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign39890_e45609;
        locals.var_guard1216_rv = 0.0;

        let (assign39900_e45620, assign39900_e45620_d_n4, assign39900_e45620_d_n6, assign39900_e45620_d_n7, assign39900_e45620_d_n8, assign39900_e45620_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1216 != 0.0)) {
        let assign39900_e45615: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign39900_e45617: f64 = (assign39900_e45615 - locals.var_xdeff__blk1000);
        let assign39900_e45618: f64 = (assign39900_e45617).exp();
        (assign39900_e45618, (assign39900_e45618 * ((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4)), (assign39900_e45618 * ((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6)), (assign39900_e45618 * ((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7)), (assign39900_e45618 * ((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8)), (assign39900_e45618 * ((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39900_e45620;
        locals.var_q_temp1__blk814_dn4 = assign39900_e45620_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39900_e45620_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39900_e45620_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39900_e45620_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39900_e45620_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_116(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign39910_e45661, assign39910_e45661_d_n4, assign39910_e45661_d_n6, assign39910_e45661_d_n7, assign39910_e45661_d_n8, assign39910_e45661_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1216 == 0.0)) {
        let assign39910_e45629: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign39910_e45631: f64 = (assign39910_e45629 - locals.var_xdeff__blk1000);
        let assign39910_e45633: f64 = (assign39910_e45631 - 80.0);
        let assign39910_e45638: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign39910_e45640: f64 = (assign39910_e45638 - locals.var_xdeff__blk1000);
        let assign39910_e45642: f64 = (assign39910_e45640 - 80.0);
        let assign39910_e45643: f64 = (0.5 * assign39910_e45642);
        let assign39910_e45647: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign39910_e45649: f64 = (assign39910_e45647 - locals.var_xdeff__blk1000);
        let assign39910_e45651: f64 = (assign39910_e45649 - 80.0);
        let assign39910_e45653: f64 = (assign39910_e45651 * 0.3333333333333);
        let assign39910_e45654: f64 = (1.0 + assign39910_e45653);
        let assign39910_e45655: f64 = (assign39910_e45643 * assign39910_e45654);
        let assign39910_e45656: f64 = (1.0 + assign39910_e45655);
        let assign39910_e45657: f64 = (assign39910_e45633 * assign39910_e45656);
        let assign39910_e45658: f64 = (1.0 + assign39910_e45657);
        let assign39910_e45659: f64 = (5.54062e34 * assign39910_e45658);
        (assign39910_e45659, (5.54062e34 * ((((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4) * assign39910_e45656) + (assign39910_e45633 * (((0.5 * ((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4)) * assign39910_e45654) + (assign39910_e45643 * (((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6) * assign39910_e45656) + (assign39910_e45633 * (((0.5 * ((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6)) * assign39910_e45654) + (assign39910_e45643 * (((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7) * assign39910_e45656) + (assign39910_e45633 * (((0.5 * ((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7)) * assign39910_e45654) + (assign39910_e45643 * (((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8) * assign39910_e45656) + (assign39910_e45633 * (((0.5 * ((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8)) * assign39910_e45654) + (assign39910_e45643 * (((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9) * assign39910_e45656) + (assign39910_e45633 * (((0.5 * ((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9)) * assign39910_e45654) + (assign39910_e45643 * (((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39910_e45661;
        locals.var_q_temp1__blk814_dn4 = assign39910_e45661_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39910_e45661_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39910_e45661_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39910_e45661_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39910_e45661_d_n9;
        locals.var_q_temp1__blk814_rv = 0.0;

        let (assign39920_e45667, assign39920_e45667_d_n4, assign39920_e45667_d_n6, assign39920_e45667_d_n7, assign39920_e45667_d_n8, assign39920_e45667_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign39920_e45665: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign39920_e45665, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_aexp2d__blk1008, locals.var_aexp2d__blk1008_dn4, locals.var_aexp2d__blk1008_dn6, locals.var_aexp2d__blk1008_dn7, locals.var_aexp2d__blk1008_dn8, locals.var_aexp2d__blk1008_dn9,)
    }
};
        locals.var_aexp2d__blk1008 = assign39920_e45667;
        locals.var_aexp2d__blk1008_dn4 = assign39920_e45667_d_n4;
        locals.var_aexp2d__blk1008_dn6 = assign39920_e45667_d_n6;
        locals.var_aexp2d__blk1008_dn7 = assign39920_e45667_d_n7;
        locals.var_aexp2d__blk1008_dn8 = assign39920_e45667_d_n8;
        locals.var_aexp2d__blk1008_dn9 = assign39920_e45667_d_n9;
        locals.var_aexp2d__blk1008_rv = 0.0;

        let (assign39930_e45671, assign39930_e45671_d_n4, assign39930_e45671_d_n6, assign39930_e45671_d_n7, assign39930_e45671_d_n8, assign39930_e45671_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_a1d__blk1011, locals.var_a1d__blk1011_dn4, locals.var_a1d__blk1011_dn6, locals.var_a1d__blk1011_dn7, locals.var_a1d__blk1011_dn8, locals.var_a1d__blk1011_dn9,)
    }
};
        locals.var_a1d__blk1011 = assign39930_e45671;
        locals.var_a1d__blk1011_dn4 = assign39930_e45671_d_n4;
        locals.var_a1d__blk1011_dn6 = assign39930_e45671_d_n6;
        locals.var_a1d__blk1011_dn7 = assign39930_e45671_d_n7;
        locals.var_a1d__blk1011_dn8 = assign39930_e45671_d_n8;
        locals.var_a1d__blk1011_dn9 = assign39930_e45671_d_n9;
        locals.var_a1d__blk1011_rv = 0.0;

        let (assign39940_e45675, assign39940_e45675_d_n4, assign39940_e45675_d_n6, assign39940_e45675_d_n7, assign39940_e45675_d_n8, assign39940_e45675_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_a2d__blk1012, locals.var_a2d__blk1012_dn4, locals.var_a2d__blk1012_dn6, locals.var_a2d__blk1012_dn7, locals.var_a2d__blk1012_dn8, locals.var_a2d__blk1012_dn9,)
    }
};
        locals.var_a2d__blk1012 = assign39940_e45675;
        locals.var_a2d__blk1012_dn4 = assign39940_e45675_d_n4;
        locals.var_a2d__blk1012_dn6 = assign39940_e45675_d_n6;
        locals.var_a2d__blk1012_dn7 = assign39940_e45675_d_n7;
        locals.var_a2d__blk1012_dn8 = assign39940_e45675_d_n8;
        locals.var_a2d__blk1012_dn9 = assign39940_e45675_d_n9;
        locals.var_a2d__blk1012_rv = 0.0;

        let (assign39950_e45679, assign39950_e45679_d_n4, assign39950_e45679_d_n6, assign39950_e45679_d_n7, assign39950_e45679_d_n8, assign39950_e45679_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b1d__blk1009, locals.var_b1d__blk1009_dn4, locals.var_b1d__blk1009_dn6, locals.var_b1d__blk1009_dn7, locals.var_b1d__blk1009_dn8, locals.var_b1d__blk1009_dn9,)
    }
};
        locals.var_b1d__blk1009 = assign39950_e45679;
        locals.var_b1d__blk1009_dn4 = assign39950_e45679_d_n4;
        locals.var_b1d__blk1009_dn6 = assign39950_e45679_d_n6;
        locals.var_b1d__blk1009_dn7 = assign39950_e45679_d_n7;
        locals.var_b1d__blk1009_dn8 = assign39950_e45679_d_n8;
        locals.var_b1d__blk1009_dn9 = assign39950_e45679_d_n9;
        locals.var_b1d__blk1009_rv = 0.0;

        let (assign39960_e45683, assign39960_e45683_d_n4, assign39960_e45683_d_n6, assign39960_e45683_d_n7, assign39960_e45683_d_n8, assign39960_e45683_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b2d__blk1010, locals.var_b2d__blk1010_dn4, locals.var_b2d__blk1010_dn6, locals.var_b2d__blk1010_dn7, locals.var_b2d__blk1010_dn8, locals.var_b2d__blk1010_dn9,)
    }
};
        locals.var_b2d__blk1010 = assign39960_e45683;
        locals.var_b2d__blk1010_dn4 = assign39960_e45683_d_n4;
        locals.var_b2d__blk1010_dn6 = assign39960_e45683_d_n6;
        locals.var_b2d__blk1010_dn7 = assign39960_e45683_d_n7;
        locals.var_b2d__blk1010_dn8 = assign39960_e45683_d_n8;
        locals.var_b2d__blk1010_dn9 = assign39960_e45683_d_n9;
        locals.var_b2d__blk1010_rv = 0.0;

        let (assign39970_e45687, assign39970_e45687_d_n4, assign39970_e45687_d_n6, assign39970_e45687_d_n7, assign39970_e45687_d_n8, assign39970_e45687_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sumd__blk1013, locals.var_sumd__blk1013_dn4, locals.var_sumd__blk1013_dn6, locals.var_sumd__blk1013_dn7, locals.var_sumd__blk1013_dn8, locals.var_sumd__blk1013_dn9,)
    }
};
        locals.var_sumd__blk1013 = assign39970_e45687;
        locals.var_sumd__blk1013_dn4 = assign39970_e45687_d_n4;
        locals.var_sumd__blk1013_dn6 = assign39970_e45687_d_n6;
        locals.var_sumd__blk1013_dn7 = assign39970_e45687_d_n7;
        locals.var_sumd__blk1013_dn8 = assign39970_e45687_d_n8;
        locals.var_sumd__blk1013_dn9 = assign39970_e45687_d_n9;
        locals.var_sumd__blk1013_rv = 0.0;

        let (assign39980_e45691, assign39980_e45691_d_n4, assign39980_e45691_d_n6, assign39980_e45691_d_n7, assign39980_e45691_d_n8, assign39980_e45691_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9,)
    }
};
        locals.var_dqsqd_dxn_qi__blk1014 = assign39980_e45691;
        locals.var_dqsqd_dxn_qi__blk1014_dn4 = assign39980_e45691_d_n4;
        locals.var_dqsqd_dxn_qi__blk1014_dn6 = assign39980_e45691_d_n6;
        locals.var_dqsqd_dxn_qi__blk1014_dn7 = assign39980_e45691_d_n7;
        locals.var_dqsqd_dxn_qi__blk1014_dn8 = assign39980_e45691_d_n8;
        locals.var_dqsqd_dxn_qi__blk1014_dn9 = assign39980_e45691_d_n9;
        locals.var_dqsqd_dxn_qi__blk1014_rv = 0.0;

        let assign39990_e45694: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1217 = assign39990_e45694;
        locals.var_guard1217_rv = 0.0;

        let (assign40000_e45702, assign40000_e45702_d_n4, assign40000_e45702_d_n6, assign40000_e45702_d_n7, assign40000_e45702_d_n8, assign40000_e45702_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign40000_e45700: f64 = (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906);
        (assign40000_e45700, ((locals.var_aexp1d__blk1007_dn4 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn4)), ((locals.var_aexp1d__blk1007_dn6 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn6)), ((locals.var_aexp1d__blk1007_dn7 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn7)), ((locals.var_aexp1d__blk1007_dn8 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn8)), ((locals.var_aexp1d__blk1007_dn9 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn9)),)
    } else {
        (locals.var_b1d__blk1009, locals.var_b1d__blk1009_dn4, locals.var_b1d__blk1009_dn6, locals.var_b1d__blk1009_dn7, locals.var_b1d__blk1009_dn8, locals.var_b1d__blk1009_dn9,)
    }
};
        locals.var_b1d__blk1009 = assign40000_e45702;
        locals.var_b1d__blk1009_dn4 = assign40000_e45702_d_n4;
        locals.var_b1d__blk1009_dn6 = assign40000_e45702_d_n6;
        locals.var_b1d__blk1009_dn7 = assign40000_e45702_d_n7;
        locals.var_b1d__blk1009_dn8 = assign40000_e45702_d_n8;
        locals.var_b1d__blk1009_dn9 = assign40000_e45702_d_n9;
        locals.var_b1d__blk1009_rv = 0.0;

        let (assign40010_e45710, assign40010_e45710_d_n4, assign40010_e45710_d_n6, assign40010_e45710_d_n7, assign40010_e45710_d_n8, assign40010_e45710_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign40010_e45708: f64 = (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907);
        (assign40010_e45708, ((locals.var_aexp2d__blk1008_dn4 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn4)), ((locals.var_aexp2d__blk1008_dn6 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn6)), ((locals.var_aexp2d__blk1008_dn7 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn7)), ((locals.var_aexp2d__blk1008_dn8 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn8)), ((locals.var_aexp2d__blk1008_dn9 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn9)),)
    } else {
        (locals.var_b2d__blk1010, locals.var_b2d__blk1010_dn4, locals.var_b2d__blk1010_dn6, locals.var_b2d__blk1010_dn7, locals.var_b2d__blk1010_dn8, locals.var_b2d__blk1010_dn9,)
    }
};
        locals.var_b2d__blk1010 = assign40010_e45710;
        locals.var_b2d__blk1010_dn4 = assign40010_e45710_d_n4;
        locals.var_b2d__blk1010_dn6 = assign40010_e45710_d_n6;
        locals.var_b2d__blk1010_dn7 = assign40010_e45710_d_n7;
        locals.var_b2d__blk1010_dn8 = assign40010_e45710_d_n8;
        locals.var_b2d__blk1010_dn9 = assign40010_e45710_d_n9;
        locals.var_b2d__blk1010_rv = 0.0;

        let (assign40020_e45720, assign40020_e45720_d_n4, assign40020_e45720_d_n6, assign40020_e45720_d_n7, assign40020_e45720_d_n8, assign40020_e45720_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign40020_e45717: f64 = (2.0 * locals.var_k1q1d__blk1004);
        let assign40020_e45718: f64 = (locals.var_b1d__blk1009 + assign40020_e45717);
        (assign40020_e45718, (locals.var_b1d__blk1009_dn4 + (2.0 * locals.var_k1q1d__blk1004_dn4)), (locals.var_b1d__blk1009_dn6 + (2.0 * locals.var_k1q1d__blk1004_dn6)), (locals.var_b1d__blk1009_dn7 + (2.0 * locals.var_k1q1d__blk1004_dn7)), (locals.var_b1d__blk1009_dn8 + (2.0 * locals.var_k1q1d__blk1004_dn8)), (locals.var_b1d__blk1009_dn9 + (2.0 * locals.var_k1q1d__blk1004_dn9)),)
    } else {
        (locals.var_a1d__blk1011, locals.var_a1d__blk1011_dn4, locals.var_a1d__blk1011_dn6, locals.var_a1d__blk1011_dn7, locals.var_a1d__blk1011_dn8, locals.var_a1d__blk1011_dn9,)
    }
};
        locals.var_a1d__blk1011 = assign40020_e45720;
        locals.var_a1d__blk1011_dn4 = assign40020_e45720_d_n4;
        locals.var_a1d__blk1011_dn6 = assign40020_e45720_d_n6;
        locals.var_a1d__blk1011_dn7 = assign40020_e45720_d_n7;
        locals.var_a1d__blk1011_dn8 = assign40020_e45720_d_n8;
        locals.var_a1d__blk1011_dn9 = assign40020_e45720_d_n9;
        locals.var_a1d__blk1011_rv = 0.0;

        let (assign40030_e45730, assign40030_e45730_d_n4, assign40030_e45730_d_n6, assign40030_e45730_d_n7, assign40030_e45730_d_n8, assign40030_e45730_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign40030_e45727: f64 = (2.0 * locals.var_k2q2d__blk1005);
        let assign40030_e45728: f64 = (locals.var_b2d__blk1010 + assign40030_e45727);
        (assign40030_e45728, (locals.var_b2d__blk1010_dn4 + (2.0 * locals.var_k2q2d__blk1005_dn4)), (locals.var_b2d__blk1010_dn6 + (2.0 * locals.var_k2q2d__blk1005_dn6)), (locals.var_b2d__blk1010_dn7 + (2.0 * locals.var_k2q2d__blk1005_dn7)), (locals.var_b2d__blk1010_dn8 + (2.0 * locals.var_k2q2d__blk1005_dn8)), (locals.var_b2d__blk1010_dn9 + (2.0 * locals.var_k2q2d__blk1005_dn9)),)
    } else {
        (locals.var_a2d__blk1012, locals.var_a2d__blk1012_dn4, locals.var_a2d__blk1012_dn6, locals.var_a2d__blk1012_dn7, locals.var_a2d__blk1012_dn8, locals.var_a2d__blk1012_dn9,)
    }
};
        locals.var_a2d__blk1012 = assign40030_e45730;
        locals.var_a2d__blk1012_dn4 = assign40030_e45730_d_n4;
        locals.var_a2d__blk1012_dn6 = assign40030_e45730_d_n6;
        locals.var_a2d__blk1012_dn7 = assign40030_e45730_d_n7;
        locals.var_a2d__blk1012_dn8 = assign40030_e45730_d_n8;
        locals.var_a2d__blk1012_dn9 = assign40030_e45730_d_n9;
        locals.var_a2d__blk1012_rv = 0.0;

        let (assign40040_e45742, assign40040_e45742_d_n4, assign40040_e45742_d_n6, assign40040_e45742_d_n7, assign40040_e45742_d_n8, assign40040_e45742_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign40040_e45736: f64 = (2.0 * locals.var_qid__blk1003);
        let assign40040_e45738: f64 = (assign40040_e45736 + locals.var_b1d__blk1009);
        let assign40040_e45740: f64 = (assign40040_e45738 + locals.var_b2d__blk1010);
        (assign40040_e45740, (((2.0 * locals.var_qid__blk1003_dn4) + locals.var_b1d__blk1009_dn4) + locals.var_b2d__blk1010_dn4), (((2.0 * locals.var_qid__blk1003_dn6) + locals.var_b1d__blk1009_dn6) + locals.var_b2d__blk1010_dn6), (((2.0 * locals.var_qid__blk1003_dn7) + locals.var_b1d__blk1009_dn7) + locals.var_b2d__blk1010_dn7), (((2.0 * locals.var_qid__blk1003_dn8) + locals.var_b1d__blk1009_dn8) + locals.var_b2d__blk1010_dn8), (((2.0 * locals.var_qid__blk1003_dn9) + locals.var_b1d__blk1009_dn9) + locals.var_b2d__blk1010_dn9),)
    } else {
        (locals.var_sumd__blk1013, locals.var_sumd__blk1013_dn4, locals.var_sumd__blk1013_dn6, locals.var_sumd__blk1013_dn7, locals.var_sumd__blk1013_dn8, locals.var_sumd__blk1013_dn9,)
    }
};
        locals.var_sumd__blk1013 = assign40040_e45742;
        locals.var_sumd__blk1013_dn4 = assign40040_e45742_d_n4;
        locals.var_sumd__blk1013_dn6 = assign40040_e45742_d_n6;
        locals.var_sumd__blk1013_dn7 = assign40040_e45742_d_n7;
        locals.var_sumd__blk1013_dn8 = assign40040_e45742_d_n8;
        locals.var_sumd__blk1013_dn9 = assign40040_e45742_d_n9;
        locals.var_sumd__blk1013_rv = 0.0;

        let assign40050_e45744: f64 = (locals.var_qsqd__blk1006).abs();
        let assign40050_e45746: f64 = if assign40050_e45744 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign40050_e45746;
        locals.var_guard1218_rv = 0.0;

        let (assign40060_e45772, assign40060_e45772_d_n4, assign40060_e45772_d_n6, assign40060_e45772_d_n7, assign40060_e45772_d_n8, assign40060_e45772_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign40060_e45754: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
        let assign40060_e45758: f64 = (locals.var_q1d__blk1001 + 2.0);
        let assign40060_e45759: f64 = (2.0 * assign40060_e45758);
        let assign40060_e45761: f64 = (assign40060_e45759 * locals.var_a2d__blk1012);
        let assign40060_e45762: f64 = (assign40060_e45754 + assign40060_e45761);
        let assign40060_e45766: f64 = (locals.var_q2d__blk1002 + 2.0);
        let assign40060_e45767: f64 = (2.0 * assign40060_e45766);
        let assign40060_e45769: f64 = (assign40060_e45767 * locals.var_a1d__blk1011);
        let assign40060_e45770: f64 = (assign40060_e45762 + assign40060_e45769);
        (assign40060_e45770, ((((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)) + (((2.0 * locals.var_q1d__blk1001_dn4) * locals.var_a2d__blk1012) + (assign40060_e45759 * locals.var_a2d__blk1012_dn4))) + (((2.0 * locals.var_q2d__blk1002_dn4) * locals.var_a1d__blk1011) + (assign40060_e45767 * locals.var_a1d__blk1011_dn4))), ((((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)) + (((2.0 * locals.var_q1d__blk1001_dn6) * locals.var_a2d__blk1012) + (assign40060_e45759 * locals.var_a2d__blk1012_dn6))) + (((2.0 * locals.var_q2d__blk1002_dn6) * locals.var_a1d__blk1011) + (assign40060_e45767 * locals.var_a1d__blk1011_dn6))), ((((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)) + (((2.0 * locals.var_q1d__blk1001_dn7) * locals.var_a2d__blk1012) + (assign40060_e45759 * locals.var_a2d__blk1012_dn7))) + (((2.0 * locals.var_q2d__blk1002_dn7) * locals.var_a1d__blk1011) + (assign40060_e45767 * locals.var_a1d__blk1011_dn7))), ((((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)) + (((2.0 * locals.var_q1d__blk1001_dn8) * locals.var_a2d__blk1012) + (assign40060_e45759 * locals.var_a2d__blk1012_dn8))) + (((2.0 * locals.var_q2d__blk1002_dn8) * locals.var_a1d__blk1011) + (assign40060_e45767 * locals.var_a1d__blk1011_dn8))), ((((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)) + (((2.0 * locals.var_q1d__blk1001_dn9) * locals.var_a2d__blk1012) + (assign40060_e45759 * locals.var_a2d__blk1012_dn9))) + (((2.0 * locals.var_q2d__blk1002_dn9) * locals.var_a1d__blk1011) + (assign40060_e45767 * locals.var_a1d__blk1011_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40060_e45772;
        locals.var_temp1_dn4 = assign40060_e45772_d_n4;
        locals.var_temp1_dn6 = assign40060_e45772_d_n6;
        locals.var_temp1_dn7 = assign40060_e45772_d_n7;
        locals.var_temp1_dn8 = assign40060_e45772_d_n8;
        locals.var_temp1_dn9 = assign40060_e45772_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign40070_e45789, assign40070_e45789_d_n4, assign40070_e45789_d_n6, assign40070_e45789_d_n7, assign40070_e45789_d_n8, assign40070_e45789_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign40070_e45779: f64 = (-4.0);
        let assign40070_e45781: f64 = (assign40070_e45779 * locals.var_qsqd__blk1006);
        let assign40070_e45783: f64 = (assign40070_e45781 * locals.var_sumd__blk1013);
        let assign40070_e45786: f64 = (locals.var_qid__blk1003 * locals.var_temp1);
        let assign40070_e45787: f64 = (assign40070_e45783 / assign40070_e45786);
        (assign40070_e45787, ((((((assign40070_e45779 * locals.var_qsqd__blk1006_dn4) * locals.var_sumd__blk1013) + (assign40070_e45781 * locals.var_sumd__blk1013_dn4)) * assign40070_e45786) - (assign40070_e45783 * ((locals.var_qid__blk1003_dn4 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn4)))) / (assign40070_e45786 * assign40070_e45786)), ((((((assign40070_e45779 * locals.var_qsqd__blk1006_dn6) * locals.var_sumd__blk1013) + (assign40070_e45781 * locals.var_sumd__blk1013_dn6)) * assign40070_e45786) - (assign40070_e45783 * ((locals.var_qid__blk1003_dn6 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn6)))) / (assign40070_e45786 * assign40070_e45786)), ((((((assign40070_e45779 * locals.var_qsqd__blk1006_dn7) * locals.var_sumd__blk1013) + (assign40070_e45781 * locals.var_sumd__blk1013_dn7)) * assign40070_e45786) - (assign40070_e45783 * ((locals.var_qid__blk1003_dn7 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn7)))) / (assign40070_e45786 * assign40070_e45786)), ((((((assign40070_e45779 * locals.var_qsqd__blk1006_dn8) * locals.var_sumd__blk1013) + (assign40070_e45781 * locals.var_sumd__blk1013_dn8)) * assign40070_e45786) - (assign40070_e45783 * ((locals.var_qid__blk1003_dn8 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn8)))) / (assign40070_e45786 * assign40070_e45786)), ((((((assign40070_e45779 * locals.var_qsqd__blk1006_dn9) * locals.var_sumd__blk1013) + (assign40070_e45781 * locals.var_sumd__blk1013_dn9)) * assign40070_e45786) - (assign40070_e45783 * ((locals.var_qid__blk1003_dn9 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn9)))) / (assign40070_e45786 * assign40070_e45786)),)
    } else {
        (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9,)
    }
};
        locals.var_dqsqd_dxn_qi__blk1014 = assign40070_e45789;
        locals.var_dqsqd_dxn_qi__blk1014_dn4 = assign40070_e45789_d_n4;
        locals.var_dqsqd_dxn_qi__blk1014_dn6 = assign40070_e45789_d_n6;
        locals.var_dqsqd_dxn_qi__blk1014_dn7 = assign40070_e45789_d_n7;
        locals.var_dqsqd_dxn_qi__blk1014_dn8 = assign40070_e45789_d_n8;
        locals.var_dqsqd_dxn_qi__blk1014_dn9 = assign40070_e45789_d_n9;
        locals.var_dqsqd_dxn_qi__blk1014_rv = 0.0;

        let (assign40080_e45816, assign40080_e45816_d_n4, assign40080_e45816_d_n6, assign40080_e45816_d_n7, assign40080_e45816_d_n8, assign40080_e45816_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
        let assign40080_e45800: f64 = (locals.var_qsqd__blk1006 * 0.0333333333333);
        let assign40080_e45804: f64 = (locals.var_qsqd__blk1006 * 0.0357142857143);
        let assign40080_e45808: f64 = (locals.var_qsqd__blk1006 * 0.0333333333333);
        let assign40080_e45809: f64 = (1.0 - assign40080_e45808);
        let assign40080_e45810: f64 = (assign40080_e45804 * assign40080_e45809);
        let assign40080_e45811: f64 = (1.0 - assign40080_e45810);
        let assign40080_e45812: f64 = (assign40080_e45800 * assign40080_e45811);
        let assign40080_e45813: f64 = (1.0 - assign40080_e45812);
        let assign40080_e45814: f64 = (0.1666666666667 * assign40080_e45813);
        (assign40080_e45814, (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0333333333333) * assign40080_e45811) + (assign40080_e45800 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0357142857143) * assign40080_e45809) + (assign40080_e45804 * (-(locals.var_qsqd__blk1006_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0333333333333) * assign40080_e45811) + (assign40080_e45800 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0357142857143) * assign40080_e45809) + (assign40080_e45804 * (-(locals.var_qsqd__blk1006_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0333333333333) * assign40080_e45811) + (assign40080_e45800 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0357142857143) * assign40080_e45809) + (assign40080_e45804 * (-(locals.var_qsqd__blk1006_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0333333333333) * assign40080_e45811) + (assign40080_e45800 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0357142857143) * assign40080_e45809) + (assign40080_e45804 * (-(locals.var_qsqd__blk1006_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0333333333333) * assign40080_e45811) + (assign40080_e45800 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0357142857143) * assign40080_e45809) + (assign40080_e45804 * (-(locals.var_qsqd__blk1006_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40080_e45816;
        locals.var_temp1_dn4 = assign40080_e45816_d_n4;
        locals.var_temp1_dn6 = assign40080_e45816_d_n6;
        locals.var_temp1_dn7 = assign40080_e45816_d_n7;
        locals.var_temp1_dn8 = assign40080_e45816_d_n8;
        locals.var_temp1_dn9 = assign40080_e45816_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign40090_e45843, assign40090_e45843_d_n4, assign40090_e45843_d_n6, assign40090_e45843_d_n7, assign40090_e45843_d_n8, assign40090_e45843_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
        let assign40090_e45825: f64 = (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007);
        let assign40090_e45828: f64 = (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008);
        let assign40090_e45829: f64 = (assign40090_e45825 + assign40090_e45828);
        let assign40090_e45832: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
        let assign40090_e45834: f64 = (assign40090_e45832 * locals.var_qid__blk1003);
        let assign40090_e45838: f64 = (locals.var_qid__blk1003 * locals.var_temp1);
        let assign40090_e45839: f64 = (1.0 + assign40090_e45838);
        let assign40090_e45840: f64 = (assign40090_e45834 * assign40090_e45839);
        let assign40090_e45841: f64 = (assign40090_e45829 + assign40090_e45840);
        (assign40090_e45841, ((((locals.var_a1d__blk1011_dn4 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn4)) + ((locals.var_a2d__blk1012_dn4 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn4))) + ((((((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)) * locals.var_qid__blk1003) + (assign40090_e45832 * locals.var_qid__blk1003_dn4)) * assign40090_e45839) + (assign40090_e45834 * ((locals.var_qid__blk1003_dn4 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn4))))), ((((locals.var_a1d__blk1011_dn6 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn6)) + ((locals.var_a2d__blk1012_dn6 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn6))) + ((((((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)) * locals.var_qid__blk1003) + (assign40090_e45832 * locals.var_qid__blk1003_dn6)) * assign40090_e45839) + (assign40090_e45834 * ((locals.var_qid__blk1003_dn6 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn6))))), ((((locals.var_a1d__blk1011_dn7 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn7)) + ((locals.var_a2d__blk1012_dn7 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn7))) + ((((((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)) * locals.var_qid__blk1003) + (assign40090_e45832 * locals.var_qid__blk1003_dn7)) * assign40090_e45839) + (assign40090_e45834 * ((locals.var_qid__blk1003_dn7 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn7))))), ((((locals.var_a1d__blk1011_dn8 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn8)) + ((locals.var_a2d__blk1012_dn8 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn8))) + ((((((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)) * locals.var_qid__blk1003) + (assign40090_e45832 * locals.var_qid__blk1003_dn8)) * assign40090_e45839) + (assign40090_e45834 * ((locals.var_qid__blk1003_dn8 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn8))))), ((((locals.var_a1d__blk1011_dn9 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn9)) + ((locals.var_a2d__blk1012_dn9 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn9))) + ((((((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)) * locals.var_qid__blk1003) + (assign40090_e45832 * locals.var_qid__blk1003_dn9)) * assign40090_e45839) + (assign40090_e45834 * ((locals.var_qid__blk1003_dn9 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn9))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40090_e45843;
        locals.var_temp2_dn4 = assign40090_e45843_d_n4;
        locals.var_temp2_dn6 = assign40090_e45843_d_n6;
        locals.var_temp2_dn7 = assign40090_e45843_d_n7;
        locals.var_temp2_dn8 = assign40090_e45843_d_n8;
        locals.var_temp2_dn9 = assign40090_e45843_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign40100_e45860, assign40100_e45860_d_n4, assign40100_e45860_d_n6, assign40100_e45860_d_n7, assign40100_e45860_d_n8, assign40100_e45860_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
        let assign40100_e45852: f64 = (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008);
        let assign40100_e45854: f64 = (assign40100_e45852 * locals.var_sumd__blk1013);
        let assign40100_e45857: f64 = (locals.var_qid__blk1003 * locals.var_temp2);
        let assign40100_e45858: f64 = (assign40100_e45854 / assign40100_e45857);
        (assign40100_e45858, (((((((locals.var_aexp1d__blk1007_dn4 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn4)) * locals.var_sumd__blk1013) + (assign40100_e45852 * locals.var_sumd__blk1013_dn4)) * assign40100_e45857) - (assign40100_e45854 * ((locals.var_qid__blk1003_dn4 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn4)))) / (assign40100_e45857 * assign40100_e45857)), (((((((locals.var_aexp1d__blk1007_dn6 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn6)) * locals.var_sumd__blk1013) + (assign40100_e45852 * locals.var_sumd__blk1013_dn6)) * assign40100_e45857) - (assign40100_e45854 * ((locals.var_qid__blk1003_dn6 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn6)))) / (assign40100_e45857 * assign40100_e45857)), (((((((locals.var_aexp1d__blk1007_dn7 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn7)) * locals.var_sumd__blk1013) + (assign40100_e45852 * locals.var_sumd__blk1013_dn7)) * assign40100_e45857) - (assign40100_e45854 * ((locals.var_qid__blk1003_dn7 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn7)))) / (assign40100_e45857 * assign40100_e45857)), (((((((locals.var_aexp1d__blk1007_dn8 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn8)) * locals.var_sumd__blk1013) + (assign40100_e45852 * locals.var_sumd__blk1013_dn8)) * assign40100_e45857) - (assign40100_e45854 * ((locals.var_qid__blk1003_dn8 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn8)))) / (assign40100_e45857 * assign40100_e45857)), (((((((locals.var_aexp1d__blk1007_dn9 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn9)) * locals.var_sumd__blk1013) + (assign40100_e45852 * locals.var_sumd__blk1013_dn9)) * assign40100_e45857) - (assign40100_e45854 * ((locals.var_qid__blk1003_dn9 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn9)))) / (assign40100_e45857 * assign40100_e45857)),)
    } else {
        (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9,)
    }
};
        locals.var_dqsqd_dxn_qi__blk1014 = assign40100_e45860;
        locals.var_dqsqd_dxn_qi__blk1014_dn4 = assign40100_e45860_d_n4;
        locals.var_dqsqd_dxn_qi__blk1014_dn6 = assign40100_e45860_d_n6;
        locals.var_dqsqd_dxn_qi__blk1014_dn7 = assign40100_e45860_d_n7;
        locals.var_dqsqd_dxn_qi__blk1014_dn8 = assign40100_e45860_d_n8;
        locals.var_dqsqd_dxn_qi__blk1014_dn9 = assign40100_e45860_d_n9;
        locals.var_dqsqd_dxn_qi__blk1014_rv = 0.0;

        let (assign40110_e45867, assign40110_e45867_d_n4, assign40110_e45867_d_n6, assign40110_e45867_d_n7, assign40110_e45867_d_n8, assign40110_e45867_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40110_e45864: f64 = (locals.var_qid__blk1003).ln();
        let assign40110_e45865: f64 = (locals.var_xdeff__blk1000 + assign40110_e45864);
        (assign40110_e45865, (locals.var_xdeff__blk1000_dn4 + (locals.var_qid__blk1003_dn4 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn6 + (locals.var_qid__blk1003_dn6 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn7 + (locals.var_qid__blk1003_dn7 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn8 + (locals.var_qid__blk1003_dn8 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn9 + (locals.var_qid__blk1003_dn9 / locals.var_qid__blk1003)),)
    } else {
        (locals.var_xdriftd__blk1015, locals.var_xdriftd__blk1015_dn4, locals.var_xdriftd__blk1015_dn6, locals.var_xdriftd__blk1015_dn7, locals.var_xdriftd__blk1015_dn8, locals.var_xdriftd__blk1015_dn9,)
    }
};
        locals.var_xdriftd__blk1015 = assign40110_e45867;
        locals.var_xdriftd__blk1015_dn4 = assign40110_e45867_d_n4;
        locals.var_xdriftd__blk1015_dn6 = assign40110_e45867_d_n6;
        locals.var_xdriftd__blk1015_dn7 = assign40110_e45867_d_n7;
        locals.var_xdriftd__blk1015_dn8 = assign40110_e45867_d_n8;
        locals.var_xdriftd__blk1015_dn9 = assign40110_e45867_d_n9;
        locals.var_xdriftd__blk1015_rv = 0.0;

        let (assign40120_e45875, assign40120_e45875_d_n4, assign40120_e45875_d_n6, assign40120_e45875_d_n7, assign40120_e45875_d_n8, assign40120_e45875_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40120_e45872: f64 = (locals.var_qis__blk938 + locals.var_qid__blk1003);
        let assign40120_e45873: f64 = (0.5 * assign40120_e45872);
        (assign40120_e45873, (0.5 * (locals.var_qis__blk938_dn4 + locals.var_qid__blk1003_dn4)), (0.5 * (locals.var_qis__blk938_dn6 + locals.var_qid__blk1003_dn6)), (0.5 * (locals.var_qis__blk938_dn7 + locals.var_qid__blk1003_dn7)), (0.5 * (locals.var_qis__blk938_dn8 + locals.var_qid__blk1003_dn8)), (0.5 * (locals.var_qis__blk938_dn9 + locals.var_qid__blk1003_dn9)),)
    } else {
        (locals.var_qim__blk1016, locals.var_qim__blk1016_dn4, locals.var_qim__blk1016_dn6, locals.var_qim__blk1016_dn7, locals.var_qim__blk1016_dn8, locals.var_qim__blk1016_dn9,)
    }
};
        locals.var_qim__blk1016 = assign40120_e45875;
        locals.var_qim__blk1016_dn4 = assign40120_e45875_d_n4;
        locals.var_qim__blk1016_dn6 = assign40120_e45875_d_n6;
        locals.var_qim__blk1016_dn7 = assign40120_e45875_d_n7;
        locals.var_qim__blk1016_dn8 = assign40120_e45875_d_n8;
        locals.var_qim__blk1016_dn9 = assign40120_e45875_d_n9;
        locals.var_qim__blk1016_rv = 0.0;

        let (assign40130_e45881, assign40130_e45881_d_n4, assign40130_e45881_d_n6, assign40130_e45881_d_n7, assign40130_e45881_d_n8, assign40130_e45881_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40130_e45879: f64 = (locals.var_xdriftd__blk1015 - locals.var_xdrifts__blk951);
        (assign40130_e45879, (locals.var_xdriftd__blk1015_dn4 - locals.var_xdrifts__blk951_dn4), (locals.var_xdriftd__blk1015_dn6 - locals.var_xdrifts__blk951_dn6), (locals.var_xdriftd__blk1015_dn7 - locals.var_xdrifts__blk951_dn7), (locals.var_xdriftd__blk1015_dn8 - locals.var_xdrifts__blk951_dn8), (locals.var_xdriftd__blk1015_dn9 - locals.var_xdrifts__blk951_dn9),)
    } else {
        (locals.var_dxdrift__blk1017, locals.var_dxdrift__blk1017_dn4, locals.var_dxdrift__blk1017_dn6, locals.var_dxdrift__blk1017_dn7, locals.var_dxdrift__blk1017_dn8, locals.var_dxdrift__blk1017_dn9,)
    }
};
        locals.var_dxdrift__blk1017 = assign40130_e45881;
        locals.var_dxdrift__blk1017_dn4 = assign40130_e45881_d_n4;
        locals.var_dxdrift__blk1017_dn6 = assign40130_e45881_d_n6;
        locals.var_dxdrift__blk1017_dn7 = assign40130_e45881_d_n7;
        locals.var_dxdrift__blk1017_dn8 = assign40130_e45881_d_n8;
        locals.var_dxdrift__blk1017_dn9 = assign40130_e45881_d_n9;
        locals.var_dxdrift__blk1017_rv = 0.0;

        let (assign40140_e45885, assign40140_e45885_d_n4, assign40140_e45885_d_n6, assign40140_e45885_d_n7, assign40140_e45885_d_n8, assign40140_e45885_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9,)
    }
};
        locals.var_ratio_pd__blk1020 = assign40140_e45885;
        locals.var_ratio_pd__blk1020_dn4 = assign40140_e45885_d_n4;
        locals.var_ratio_pd__blk1020_dn6 = assign40140_e45885_d_n6;
        locals.var_ratio_pd__blk1020_dn7 = assign40140_e45885_d_n7;
        locals.var_ratio_pd__blk1020_dn8 = assign40140_e45885_d_n8;
        locals.var_ratio_pd__blk1020_dn9 = assign40140_e45885_d_n9;
        locals.var_ratio_pd__blk1020_rv = 0.0;

        let assign40150_e45888: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1219 = assign40150_e45888;
        locals.var_guard1219_rv = 0.0;

        let (assign40160_e45900, assign40160_e45900_d_n4, assign40160_e45900_d_n6, assign40160_e45900_d_n7, assign40160_e45900_d_n8, assign40160_e45900_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign40160_e45895: f64 = (locals.var_k1q1s__blk939 + locals.var_k1q1d__blk1004);
        let assign40160_e45896: f64 = (0.5 * assign40160_e45895);
        let assign40160_e45898: f64 = (assign40160_e45896 / locals.var_k1__blk932);
        (assign40160_e45898, ((((0.5 * (locals.var_k1q1s__blk939_dn4 + locals.var_k1q1d__blk1004_dn4)) * locals.var_k1__blk932) - (assign40160_e45896 * locals.var_k1__blk932_dn4)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn6 + locals.var_k1q1d__blk1004_dn6)) * locals.var_k1__blk932) - (assign40160_e45896 * locals.var_k1__blk932_dn6)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn7 + locals.var_k1q1d__blk1004_dn7)) * locals.var_k1__blk932) - (assign40160_e45896 * locals.var_k1__blk932_dn7)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn8 + locals.var_k1q1d__blk1004_dn8)) * locals.var_k1__blk932) - (assign40160_e45896 * locals.var_k1__blk932_dn8)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn9 + locals.var_k1q1d__blk1004_dn9)) * locals.var_k1__blk932) - (assign40160_e45896 * locals.var_k1__blk932_dn9)) / (locals.var_k1__blk932 * locals.var_k1__blk932)),)
    } else {
        (locals.var_qim_pd__blk1018, locals.var_qim_pd__blk1018_dn4, locals.var_qim_pd__blk1018_dn6, locals.var_qim_pd__blk1018_dn7, locals.var_qim_pd__blk1018_dn8, locals.var_qim_pd__blk1018_dn9,)
    }
};
        locals.var_qim_pd__blk1018 = assign40160_e45900;
        locals.var_qim_pd__blk1018_dn4 = assign40160_e45900_d_n4;
        locals.var_qim_pd__blk1018_dn6 = assign40160_e45900_d_n6;
        locals.var_qim_pd__blk1018_dn7 = assign40160_e45900_d_n7;
        locals.var_qim_pd__blk1018_dn8 = assign40160_e45900_d_n8;
        locals.var_qim_pd__blk1018_dn9 = assign40160_e45900_d_n9;
        locals.var_qim_pd__blk1018_rv = 0.0;

        let (assign40170_e45921, assign40170_e45921_d_n4, assign40170_e45921_d_n6, assign40170_e45921_d_n7, assign40170_e45921_d_n8, assign40170_e45921_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign40170_e45907: f64 = (locals.var_qim_pd__blk1018 + 1e-5);
        let assign40170_e45910: f64 = (locals.var_qim_pd__blk1018 - 1e-5);
        let assign40170_e45913: f64 = (locals.var_qim_pd__blk1018 - 1e-5);
        let assign40170_e45914: f64 = (assign40170_e45910 * assign40170_e45913);
        let assign40170_e45916: f64 = (assign40170_e45914 + 1.0);
        let assign40170_e45917: f64 = (assign40170_e45916).sqrt();
        let assign40170_e45918: f64 = (assign40170_e45907 + assign40170_e45917);
        let assign40170_e45919: f64 = (0.5 * assign40170_e45918);
        (assign40170_e45919, (0.5 * (locals.var_qim_pd__blk1018_dn4 + (((locals.var_qim_pd__blk1018_dn4 * assign40170_e45913) + (assign40170_e45910 * locals.var_qim_pd__blk1018_dn4)) / (2.0 * assign40170_e45917)))), (0.5 * (locals.var_qim_pd__blk1018_dn6 + (((locals.var_qim_pd__blk1018_dn6 * assign40170_e45913) + (assign40170_e45910 * locals.var_qim_pd__blk1018_dn6)) / (2.0 * assign40170_e45917)))), (0.5 * (locals.var_qim_pd__blk1018_dn7 + (((locals.var_qim_pd__blk1018_dn7 * assign40170_e45913) + (assign40170_e45910 * locals.var_qim_pd__blk1018_dn7)) / (2.0 * assign40170_e45917)))), (0.5 * (locals.var_qim_pd__blk1018_dn8 + (((locals.var_qim_pd__blk1018_dn8 * assign40170_e45913) + (assign40170_e45910 * locals.var_qim_pd__blk1018_dn8)) / (2.0 * assign40170_e45917)))), (0.5 * (locals.var_qim_pd__blk1018_dn9 + (((locals.var_qim_pd__blk1018_dn9 * assign40170_e45913) + (assign40170_e45910 * locals.var_qim_pd__blk1018_dn9)) / (2.0 * assign40170_e45917)))),)
    } else {
        (locals.var_qim_pd__blk1018, locals.var_qim_pd__blk1018_dn4, locals.var_qim_pd__blk1018_dn6, locals.var_qim_pd__blk1018_dn7, locals.var_qim_pd__blk1018_dn8, locals.var_qim_pd__blk1018_dn9,)
    }
};
        locals.var_qim_pd__blk1018 = assign40170_e45921;
        locals.var_qim_pd__blk1018_dn4 = assign40170_e45921_d_n4;
        locals.var_qim_pd__blk1018_dn6 = assign40170_e45921_d_n6;
        locals.var_qim_pd__blk1018_dn7 = assign40170_e45921_d_n7;
        locals.var_qim_pd__blk1018_dn8 = assign40170_e45921_d_n8;
        locals.var_qim_pd__blk1018_dn9 = assign40170_e45921_d_n9;
        locals.var_qim_pd__blk1018_rv = 0.0;

        let (assign40180_e45940, assign40180_e45940_d_n4, assign40180_e45940_d_n6, assign40180_e45940_d_n7, assign40180_e45940_d_n8, assign40180_e45940_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign40180_e45927: f64 = (locals.var_qim_pd__blk1018 / locals.var_inv_phit);
        let assign40180_e45930: f64 = (0.25 * locals.var_kp);
        let assign40180_e45932: f64 = (assign40180_e45930 * locals.var_kp);
        let assign40180_e45933: f64 = (assign40180_e45927 + assign40180_e45932);
        let assign40180_e45934: f64 = (assign40180_e45933).sqrt();
        let assign40180_e45937: f64 = (0.5 * locals.var_kp);
        let assign40180_e45938: f64 = (assign40180_e45934 - assign40180_e45937);
        (assign40180_e45938, ((((((locals.var_qim_pd__blk1018_dn4 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn4)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn4) * locals.var_kp) + (assign40180_e45930 * locals.var_kp_dn4))) / (2.0 * assign40180_e45934)) - (0.5 * locals.var_kp_dn4)), ((((((locals.var_qim_pd__blk1018_dn6 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn6)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn6) * locals.var_kp) + (assign40180_e45930 * locals.var_kp_dn6))) / (2.0 * assign40180_e45934)) - (0.5 * locals.var_kp_dn6)), ((((((locals.var_qim_pd__blk1018_dn7 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn7)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn7) * locals.var_kp) + (assign40180_e45930 * locals.var_kp_dn7))) / (2.0 * assign40180_e45934)) - (0.5 * locals.var_kp_dn7)), ((((((locals.var_qim_pd__blk1018_dn8 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn8)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn8) * locals.var_kp) + (assign40180_e45930 * locals.var_kp_dn8))) / (2.0 * assign40180_e45934)) - (0.5 * locals.var_kp_dn8)), ((((((locals.var_qim_pd__blk1018_dn9 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn9)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn9) * locals.var_kp) + (assign40180_e45930 * locals.var_kp_dn9))) / (2.0 * assign40180_e45934)) - (0.5 * locals.var_kp_dn9)),)
    } else {
        (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9,)
    }
};
        locals.var_temp0 = assign40180_e45940;
        locals.var_temp0_dn4 = assign40180_e45940_d_n4;
        locals.var_temp0_dn6 = assign40180_e45940_d_n6;
        locals.var_temp0_dn7 = assign40180_e45940_d_n7;
        locals.var_temp0_dn8 = assign40180_e45940_d_n8;
        locals.var_temp0_dn9 = assign40180_e45940_d_n9;
        locals.var_temp0_rv = 0.0;

        let (assign40190_e45950, assign40190_e45950_d_n4, assign40190_e45950_d_n6, assign40190_e45950_d_n7, assign40190_e45950_d_n8, assign40190_e45950_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign40190_e45946: f64 = (locals.var_temp0).powf(2.0);
        let assign40190_e45948: f64 = (assign40190_e45946 * locals.var_inv_phit);
        (assign40190_e45948, ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn4)) } } else { (assign40190_e45946 * (2.0 * (locals.var_temp0_dn4 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40190_e45946 * locals.var_inv_phit_dn4)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn6)) } } else { (assign40190_e45946 * (2.0 * (locals.var_temp0_dn6 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40190_e45946 * locals.var_inv_phit_dn6)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn7)) } } else { (assign40190_e45946 * (2.0 * (locals.var_temp0_dn7 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40190_e45946 * locals.var_inv_phit_dn7)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn8)) } } else { (assign40190_e45946 * (2.0 * (locals.var_temp0_dn8 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40190_e45946 * locals.var_inv_phit_dn8)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn9)) } } else { (assign40190_e45946 * (2.0 * (locals.var_temp0_dn9 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40190_e45946 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_xp_pd__blk1019, locals.var_xp_pd__blk1019_dn4, locals.var_xp_pd__blk1019_dn6, locals.var_xp_pd__blk1019_dn7, locals.var_xp_pd__blk1019_dn8, locals.var_xp_pd__blk1019_dn9,)
    }
};
        locals.var_xp_pd__blk1019 = assign40190_e45950;
        locals.var_xp_pd__blk1019_dn4 = assign40190_e45950_d_n4;
        locals.var_xp_pd__blk1019_dn6 = assign40190_e45950_d_n6;
        locals.var_xp_pd__blk1019_dn7 = assign40190_e45950_d_n7;
        locals.var_xp_pd__blk1019_dn8 = assign40190_e45950_d_n8;
        locals.var_xp_pd__blk1019_dn9 = assign40190_e45950_d_n9;
        locals.var_xp_pd__blk1019_rv = 0.0;

        let (assign40200_e45960, assign40200_e45960_d_n4, assign40200_e45960_d_n6, assign40200_e45960_d_n7, assign40200_e45960_d_n8, assign40200_e45960_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign40200_e45957: f64 = (locals.var_xp_pd__blk1019 / locals.var_qim_pd__blk1018);
        let assign40200_e45958: f64 = (1.0 - assign40200_e45957);
        (assign40200_e45958, (-(((locals.var_xp_pd__blk1019_dn4 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn4)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn6 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn6)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn7 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn7)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn8 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn8)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn9 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn9)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))),)
    } else {
        (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9,)
    }
};
        locals.var_ratio_pd__blk1020 = assign40200_e45960;
        locals.var_ratio_pd__blk1020_dn4 = assign40200_e45960_d_n4;
        locals.var_ratio_pd__blk1020_dn6 = assign40200_e45960_d_n6;
        locals.var_ratio_pd__blk1020_dn7 = assign40200_e45960_d_n7;
        locals.var_ratio_pd__blk1020_dn8 = assign40200_e45960_d_n8;
        locals.var_ratio_pd__blk1020_dn9 = assign40200_e45960_d_n9;
        locals.var_ratio_pd__blk1020_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_117(
        locals: &mut StampLocals,
    ) {
        let assign40210_e45963: f64 = (locals.var_k1q1d__blk1004 / 2.0);
        let assign40210_e45965: f64 = if assign40210_e45963 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign40210_e45965;
        locals.var_guard1220_rv = 0.0;

        let (assign40220_e45977, assign40220_e45977_d_n4, assign40220_e45977_d_n6, assign40220_e45977_d_n7, assign40220_e45977_d_n8, assign40220_e45977_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1220 != 0.0)) {
        let assign40220_e45972: f64 = (locals.var_k1q1d__blk1004 / 2.0);
        let assign40220_e45973: f64 = (assign40220_e45972).exp();
        let assign40220_e45974: f64 = (1.0 + assign40220_e45973);
        let assign40220_e45975: f64 = (assign40220_e45974).ln();
        (assign40220_e45975, ((assign40220_e45973 * (locals.var_k1q1d__blk1004_dn4 / 2.0)) / assign40220_e45974), ((assign40220_e45973 * (locals.var_k1q1d__blk1004_dn6 / 2.0)) / assign40220_e45974), ((assign40220_e45973 * (locals.var_k1q1d__blk1004_dn7 / 2.0)) / assign40220_e45974), ((assign40220_e45973 * (locals.var_k1q1d__blk1004_dn8 / 2.0)) / assign40220_e45974), ((assign40220_e45973 * (locals.var_k1q1d__blk1004_dn9 / 2.0)) / assign40220_e45974),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40220_e45977;
        locals.var_temp1_dn4 = assign40220_e45977_d_n4;
        locals.var_temp1_dn6 = assign40220_e45977_d_n6;
        locals.var_temp1_dn7 = assign40220_e45977_d_n7;
        locals.var_temp1_dn8 = assign40220_e45977_d_n8;
        locals.var_temp1_dn9 = assign40220_e45977_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign40230_e45986, assign40230_e45986_d_n4, assign40230_e45986_d_n6, assign40230_e45986_d_n7, assign40230_e45986_d_n8, assign40230_e45986_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1220 == 0.0)) {
        let assign40230_e45984: f64 = (locals.var_k1q1d__blk1004 / 2.0);
        (assign40230_e45984, (locals.var_k1q1d__blk1004_dn4 / 2.0), (locals.var_k1q1d__blk1004_dn6 / 2.0), (locals.var_k1q1d__blk1004_dn7 / 2.0), (locals.var_k1q1d__blk1004_dn8 / 2.0), (locals.var_k1q1d__blk1004_dn9 / 2.0),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40230_e45986;
        locals.var_temp1_dn4 = assign40230_e45986_d_n4;
        locals.var_temp1_dn6 = assign40230_e45986_d_n6;
        locals.var_temp1_dn7 = assign40230_e45986_d_n7;
        locals.var_temp1_dn8 = assign40230_e45986_d_n8;
        locals.var_temp1_dn9 = assign40230_e45986_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign40240_e45992, assign40240_e45992_d_n4, assign40240_e45992_d_n6, assign40240_e45992_d_n7, assign40240_e45992_d_n8, assign40240_e45992_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40240_e45990: f64 = (2.0 * locals.var_temp1);
        (assign40240_e45990, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_esurf1d__blk1021, locals.var_esurf1d__blk1021_dn4, locals.var_esurf1d__blk1021_dn6, locals.var_esurf1d__blk1021_dn7, locals.var_esurf1d__blk1021_dn8, locals.var_esurf1d__blk1021_dn9,)
    }
};
        locals.var_esurf1d__blk1021 = assign40240_e45992;
        locals.var_esurf1d__blk1021_dn4 = assign40240_e45992_d_n4;
        locals.var_esurf1d__blk1021_dn6 = assign40240_e45992_d_n6;
        locals.var_esurf1d__blk1021_dn7 = assign40240_e45992_d_n7;
        locals.var_esurf1d__blk1021_dn8 = assign40240_e45992_d_n8;
        locals.var_esurf1d__blk1021_dn9 = assign40240_e45992_d_n9;
        locals.var_esurf1d__blk1021_rv = 0.0;

        let assign40250_e45995: f64 = (locals.var_k2q2d__blk1005 / 2.0);
        let assign40250_e45997: f64 = if assign40250_e45995 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign40250_e45997;
        locals.var_guard1221_rv = 0.0;

        let (assign40260_e46009, assign40260_e46009_d_n4, assign40260_e46009_d_n6, assign40260_e46009_d_n7, assign40260_e46009_d_n8, assign40260_e46009_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1221 != 0.0)) {
        let assign40260_e46004: f64 = (locals.var_k2q2d__blk1005 / 2.0);
        let assign40260_e46005: f64 = (assign40260_e46004).exp();
        let assign40260_e46006: f64 = (1.0 + assign40260_e46005);
        let assign40260_e46007: f64 = (assign40260_e46006).ln();
        (assign40260_e46007, ((assign40260_e46005 * (locals.var_k2q2d__blk1005_dn4 / 2.0)) / assign40260_e46006), ((assign40260_e46005 * (locals.var_k2q2d__blk1005_dn6 / 2.0)) / assign40260_e46006), ((assign40260_e46005 * (locals.var_k2q2d__blk1005_dn7 / 2.0)) / assign40260_e46006), ((assign40260_e46005 * (locals.var_k2q2d__blk1005_dn8 / 2.0)) / assign40260_e46006), ((assign40260_e46005 * (locals.var_k2q2d__blk1005_dn9 / 2.0)) / assign40260_e46006),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40260_e46009;
        locals.var_temp2_dn4 = assign40260_e46009_d_n4;
        locals.var_temp2_dn6 = assign40260_e46009_d_n6;
        locals.var_temp2_dn7 = assign40260_e46009_d_n7;
        locals.var_temp2_dn8 = assign40260_e46009_d_n8;
        locals.var_temp2_dn9 = assign40260_e46009_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign40270_e46018, assign40270_e46018_d_n4, assign40270_e46018_d_n6, assign40270_e46018_d_n7, assign40270_e46018_d_n8, assign40270_e46018_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1221 == 0.0)) {
        let assign40270_e46016: f64 = (locals.var_k2q2d__blk1005 / 2.0);
        (assign40270_e46016, (locals.var_k2q2d__blk1005_dn4 / 2.0), (locals.var_k2q2d__blk1005_dn6 / 2.0), (locals.var_k2q2d__blk1005_dn7 / 2.0), (locals.var_k2q2d__blk1005_dn8 / 2.0), (locals.var_k2q2d__blk1005_dn9 / 2.0),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40270_e46018;
        locals.var_temp2_dn4 = assign40270_e46018_d_n4;
        locals.var_temp2_dn6 = assign40270_e46018_d_n6;
        locals.var_temp2_dn7 = assign40270_e46018_d_n7;
        locals.var_temp2_dn8 = assign40270_e46018_d_n8;
        locals.var_temp2_dn9 = assign40270_e46018_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign40280_e46024, assign40280_e46024_d_n4, assign40280_e46024_d_n6, assign40280_e46024_d_n7, assign40280_e46024_d_n8, assign40280_e46024_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40280_e46022: f64 = (2.0 * locals.var_temp2);
        (assign40280_e46022, (2.0 * locals.var_temp2_dn4), (2.0 * locals.var_temp2_dn6), (2.0 * locals.var_temp2_dn7), (2.0 * locals.var_temp2_dn8), (2.0 * locals.var_temp2_dn9),)
    } else {
        (locals.var_esurf2d__blk1022, locals.var_esurf2d__blk1022_dn4, locals.var_esurf2d__blk1022_dn6, locals.var_esurf2d__blk1022_dn7, locals.var_esurf2d__blk1022_dn8, locals.var_esurf2d__blk1022_dn9,)
    }
};
        locals.var_esurf2d__blk1022 = assign40280_e46024;
        locals.var_esurf2d__blk1022_dn4 = assign40280_e46024_d_n4;
        locals.var_esurf2d__blk1022_dn6 = assign40280_e46024_d_n6;
        locals.var_esurf2d__blk1022_dn7 = assign40280_e46024_d_n7;
        locals.var_esurf2d__blk1022_dn8 = assign40280_e46024_d_n8;
        locals.var_esurf2d__blk1022_dn9 = assign40280_e46024_d_n9;
        locals.var_esurf2d__blk1022_rv = 0.0;

        let (assign40290_e46030, assign40290_e46030_d_n4, assign40290_e46030_d_n6, assign40290_e46030_d_n7, assign40290_e46030_d_n8, assign40290_e46030_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40290_e46028: f64 = (locals.var_esurf2d__blk1022 - locals.var_k2q2d__blk1005);
        (assign40290_e46028, (locals.var_esurf2d__blk1022_dn4 - locals.var_k2q2d__blk1005_dn4), (locals.var_esurf2d__blk1022_dn6 - locals.var_k2q2d__blk1005_dn6), (locals.var_esurf2d__blk1022_dn7 - locals.var_k2q2d__blk1005_dn7), (locals.var_esurf2d__blk1022_dn8 - locals.var_k2q2d__blk1005_dn8), (locals.var_esurf2d__blk1022_dn9 - locals.var_k2q2d__blk1005_dn9),)
    } else {
        (locals.var_ecpl1d__blk1023, locals.var_ecpl1d__blk1023_dn4, locals.var_ecpl1d__blk1023_dn6, locals.var_ecpl1d__blk1023_dn7, locals.var_ecpl1d__blk1023_dn8, locals.var_ecpl1d__blk1023_dn9,)
    }
};
        locals.var_ecpl1d__blk1023 = assign40290_e46030;
        locals.var_ecpl1d__blk1023_dn4 = assign40290_e46030_d_n4;
        locals.var_ecpl1d__blk1023_dn6 = assign40290_e46030_d_n6;
        locals.var_ecpl1d__blk1023_dn7 = assign40290_e46030_d_n7;
        locals.var_ecpl1d__blk1023_dn8 = assign40290_e46030_d_n8;
        locals.var_ecpl1d__blk1023_dn9 = assign40290_e46030_d_n9;
        locals.var_ecpl1d__blk1023_rv = 0.0;

        let (assign40300_e46036, assign40300_e46036_d_n4, assign40300_e46036_d_n6, assign40300_e46036_d_n7, assign40300_e46036_d_n8, assign40300_e46036_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40300_e46034: f64 = (locals.var_esurf1d__blk1021 - locals.var_k1q1d__blk1004);
        (assign40300_e46034, (locals.var_esurf1d__blk1021_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_esurf1d__blk1021_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_esurf1d__blk1021_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_esurf1d__blk1021_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_esurf1d__blk1021_dn9 - locals.var_k1q1d__blk1004_dn9),)
    } else {
        (locals.var_ecpl2d__blk1024, locals.var_ecpl2d__blk1024_dn4, locals.var_ecpl2d__blk1024_dn6, locals.var_ecpl2d__blk1024_dn7, locals.var_ecpl2d__blk1024_dn8, locals.var_ecpl2d__blk1024_dn9,)
    }
};
        locals.var_ecpl2d__blk1024 = assign40300_e46036;
        locals.var_ecpl2d__blk1024_dn4 = assign40300_e46036_d_n4;
        locals.var_ecpl2d__blk1024_dn6 = assign40300_e46036_d_n6;
        locals.var_ecpl2d__blk1024_dn7 = assign40300_e46036_d_n7;
        locals.var_ecpl2d__blk1024_dn8 = assign40300_e46036_d_n8;
        locals.var_ecpl2d__blk1024_dn9 = assign40300_e46036_d_n9;
        locals.var_ecpl2d__blk1024_rv = 0.0;

        let (assign40310_e46046, assign40310_e46046_d_n4, assign40310_e46046_d_n6, assign40310_e46046_d_n7, assign40310_e46046_d_n8, assign40310_e46046_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40310_e46040: f64 = (locals.var_eta_mu * locals.var_esurf1d__blk1021);
        let assign40310_e46043: f64 = (locals.var_one_m_eta * locals.var_ecpl1d__blk1023);
        let assign40310_e46044: f64 = (assign40310_e46040 + assign40310_e46043);
        (assign40310_e46044, ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn4) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn4)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn6) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn6)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn7) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn7)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn8) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn8)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn9) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn9)),)
    } else {
        (locals.var_eeff1d__blk1025, locals.var_eeff1d__blk1025_dn4, locals.var_eeff1d__blk1025_dn6, locals.var_eeff1d__blk1025_dn7, locals.var_eeff1d__blk1025_dn8, locals.var_eeff1d__blk1025_dn9,)
    }
};
        locals.var_eeff1d__blk1025 = assign40310_e46046;
        locals.var_eeff1d__blk1025_dn4 = assign40310_e46046_d_n4;
        locals.var_eeff1d__blk1025_dn6 = assign40310_e46046_d_n6;
        locals.var_eeff1d__blk1025_dn7 = assign40310_e46046_d_n7;
        locals.var_eeff1d__blk1025_dn8 = assign40310_e46046_d_n8;
        locals.var_eeff1d__blk1025_dn9 = assign40310_e46046_d_n9;
        locals.var_eeff1d__blk1025_rv = 0.0;

        let (assign40320_e46056, assign40320_e46056_d_n4, assign40320_e46056_d_n6, assign40320_e46056_d_n7, assign40320_e46056_d_n8, assign40320_e46056_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40320_e46050: f64 = (locals.var_eta_mu * locals.var_esurf2d__blk1022);
        let assign40320_e46053: f64 = (locals.var_one_m_eta * locals.var_ecpl2d__blk1024);
        let assign40320_e46054: f64 = (assign40320_e46050 + assign40320_e46053);
        (assign40320_e46054, ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn4) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn4)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn6) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn6)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn7) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn7)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn8) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn8)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn9) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn9)),)
    } else {
        (locals.var_eeff2d__blk1026, locals.var_eeff2d__blk1026_dn4, locals.var_eeff2d__blk1026_dn6, locals.var_eeff2d__blk1026_dn7, locals.var_eeff2d__blk1026_dn8, locals.var_eeff2d__blk1026_dn9,)
    }
};
        locals.var_eeff2d__blk1026 = assign40320_e46056;
        locals.var_eeff2d__blk1026_dn4 = assign40320_e46056_d_n4;
        locals.var_eeff2d__blk1026_dn6 = assign40320_e46056_d_n6;
        locals.var_eeff2d__blk1026_dn7 = assign40320_e46056_d_n7;
        locals.var_eeff2d__blk1026_dn8 = assign40320_e46056_d_n8;
        locals.var_eeff2d__blk1026_dn9 = assign40320_e46056_d_n9;
        locals.var_eeff2d__blk1026_rv = 0.0;

        let (assign40330_e46064, assign40330_e46064_d_n4, assign40330_e46064_d_n6, assign40330_e46064_d_n7, assign40330_e46064_d_n8, assign40330_e46064_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40330_e46061: f64 = (locals.var_esurf1s__blk952 + locals.var_esurf1d__blk1021);
        let assign40330_e46062: f64 = (0.5 * assign40330_e46061);
        (assign40330_e46062, (0.5 * (locals.var_esurf1s__blk952_dn4 + locals.var_esurf1d__blk1021_dn4)), (0.5 * (locals.var_esurf1s__blk952_dn6 + locals.var_esurf1d__blk1021_dn6)), (0.5 * (locals.var_esurf1s__blk952_dn7 + locals.var_esurf1d__blk1021_dn7)), (0.5 * (locals.var_esurf1s__blk952_dn8 + locals.var_esurf1d__blk1021_dn8)), (0.5 * (locals.var_esurf1s__blk952_dn9 + locals.var_esurf1d__blk1021_dn9)),)
    } else {
        (locals.var_esurf1__blk1027, locals.var_esurf1__blk1027_dn4, locals.var_esurf1__blk1027_dn6, locals.var_esurf1__blk1027_dn7, locals.var_esurf1__blk1027_dn8, locals.var_esurf1__blk1027_dn9,)
    }
};
        locals.var_esurf1__blk1027 = assign40330_e46064;
        locals.var_esurf1__blk1027_dn4 = assign40330_e46064_d_n4;
        locals.var_esurf1__blk1027_dn6 = assign40330_e46064_d_n6;
        locals.var_esurf1__blk1027_dn7 = assign40330_e46064_d_n7;
        locals.var_esurf1__blk1027_dn8 = assign40330_e46064_d_n8;
        locals.var_esurf1__blk1027_dn9 = assign40330_e46064_d_n9;
        locals.var_esurf1__blk1027_rv = 0.0;

        let (assign40340_e46072, assign40340_e46072_d_n4, assign40340_e46072_d_n6, assign40340_e46072_d_n7, assign40340_e46072_d_n8, assign40340_e46072_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40340_e46069: f64 = (locals.var_esurf2s__blk953 + locals.var_esurf2d__blk1022);
        let assign40340_e46070: f64 = (0.5 * assign40340_e46069);
        (assign40340_e46070, (0.5 * (locals.var_esurf2s__blk953_dn4 + locals.var_esurf2d__blk1022_dn4)), (0.5 * (locals.var_esurf2s__blk953_dn6 + locals.var_esurf2d__blk1022_dn6)), (0.5 * (locals.var_esurf2s__blk953_dn7 + locals.var_esurf2d__blk1022_dn7)), (0.5 * (locals.var_esurf2s__blk953_dn8 + locals.var_esurf2d__blk1022_dn8)), (0.5 * (locals.var_esurf2s__blk953_dn9 + locals.var_esurf2d__blk1022_dn9)),)
    } else {
        (locals.var_esurf2__blk1028, locals.var_esurf2__blk1028_dn4, locals.var_esurf2__blk1028_dn6, locals.var_esurf2__blk1028_dn7, locals.var_esurf2__blk1028_dn8, locals.var_esurf2__blk1028_dn9,)
    }
};
        locals.var_esurf2__blk1028 = assign40340_e46072;
        locals.var_esurf2__blk1028_dn4 = assign40340_e46072_d_n4;
        locals.var_esurf2__blk1028_dn6 = assign40340_e46072_d_n6;
        locals.var_esurf2__blk1028_dn7 = assign40340_e46072_d_n7;
        locals.var_esurf2__blk1028_dn8 = assign40340_e46072_d_n8;
        locals.var_esurf2__blk1028_dn9 = assign40340_e46072_d_n9;
        locals.var_esurf2__blk1028_rv = 0.0;

        let (assign40350_e46080, assign40350_e46080_d_n4, assign40350_e46080_d_n6, assign40350_e46080_d_n7, assign40350_e46080_d_n8, assign40350_e46080_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40350_e46077: f64 = (locals.var_esurf1__blk1027 + locals.var_esurf2__blk1028);
        let assign40350_e46078: f64 = (1.0 / assign40350_e46077);
        (assign40350_e46078, (-((locals.var_esurf1__blk1027_dn4 + locals.var_esurf2__blk1028_dn4) / (assign40350_e46077 * assign40350_e46077))), (-((locals.var_esurf1__blk1027_dn6 + locals.var_esurf2__blk1028_dn6) / (assign40350_e46077 * assign40350_e46077))), (-((locals.var_esurf1__blk1027_dn7 + locals.var_esurf2__blk1028_dn7) / (assign40350_e46077 * assign40350_e46077))), (-((locals.var_esurf1__blk1027_dn8 + locals.var_esurf2__blk1028_dn8) / (assign40350_e46077 * assign40350_e46077))), (-((locals.var_esurf1__blk1027_dn9 + locals.var_esurf2__blk1028_dn9) / (assign40350_e46077 * assign40350_e46077))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign40350_e46080;
        locals.var_temp_dn4 = assign40350_e46080_d_n4;
        locals.var_temp_dn6 = assign40350_e46080_d_n6;
        locals.var_temp_dn7 = assign40350_e46080_d_n7;
        locals.var_temp_dn8 = assign40350_e46080_d_n8;
        locals.var_temp_dn9 = assign40350_e46080_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign40360_e46088, assign40360_e46088_d_n4, assign40360_e46088_d_n6, assign40360_e46088_d_n7, assign40360_e46088_d_n8, assign40360_e46088_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40360_e46084: f64 = (locals.var_qim__blk1016 * locals.var_esurf1__blk1027);
        let assign40360_e46086: f64 = (assign40360_e46084 * locals.var_temp);
        (assign40360_e46086, ((((locals.var_qim__blk1016_dn4 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn4)) * locals.var_temp) + (assign40360_e46084 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn6)) * locals.var_temp) + (assign40360_e46084 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn7)) * locals.var_temp) + (assign40360_e46084 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn8)) * locals.var_temp) + (assign40360_e46084 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn9)) * locals.var_temp) + (assign40360_e46084 * locals.var_temp_dn9)),)
    } else {
        (locals.var_qi1m__blk1029, locals.var_qi1m__blk1029_dn4, locals.var_qi1m__blk1029_dn6, locals.var_qi1m__blk1029_dn7, locals.var_qi1m__blk1029_dn8, locals.var_qi1m__blk1029_dn9,)
    }
};
        locals.var_qi1m__blk1029 = assign40360_e46088;
        locals.var_qi1m__blk1029_dn4 = assign40360_e46088_d_n4;
        locals.var_qi1m__blk1029_dn6 = assign40360_e46088_d_n6;
        locals.var_qi1m__blk1029_dn7 = assign40360_e46088_d_n7;
        locals.var_qi1m__blk1029_dn8 = assign40360_e46088_d_n8;
        locals.var_qi1m__blk1029_dn9 = assign40360_e46088_d_n9;
        locals.var_qi1m__blk1029_rv = 0.0;

        let (assign40370_e46096, assign40370_e46096_d_n4, assign40370_e46096_d_n6, assign40370_e46096_d_n7, assign40370_e46096_d_n8, assign40370_e46096_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40370_e46092: f64 = (locals.var_qim__blk1016 * locals.var_esurf2__blk1028);
        let assign40370_e46094: f64 = (assign40370_e46092 * locals.var_temp);
        (assign40370_e46094, ((((locals.var_qim__blk1016_dn4 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn4)) * locals.var_temp) + (assign40370_e46092 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn6)) * locals.var_temp) + (assign40370_e46092 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn7)) * locals.var_temp) + (assign40370_e46092 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn8)) * locals.var_temp) + (assign40370_e46092 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn9)) * locals.var_temp) + (assign40370_e46092 * locals.var_temp_dn9)),)
    } else {
        (locals.var_qi2m__blk1030, locals.var_qi2m__blk1030_dn4, locals.var_qi2m__blk1030_dn6, locals.var_qi2m__blk1030_dn7, locals.var_qi2m__blk1030_dn8, locals.var_qi2m__blk1030_dn9,)
    }
};
        locals.var_qi2m__blk1030 = assign40370_e46096;
        locals.var_qi2m__blk1030_dn4 = assign40370_e46096_d_n4;
        locals.var_qi2m__blk1030_dn6 = assign40370_e46096_d_n6;
        locals.var_qi2m__blk1030_dn7 = assign40370_e46096_d_n7;
        locals.var_qi2m__blk1030_dn8 = assign40370_e46096_d_n8;
        locals.var_qi2m__blk1030_dn9 = assign40370_e46096_d_n9;
        locals.var_qi2m__blk1030_rv = 0.0;

        let (assign40380_e46104, assign40380_e46104_d_n4, assign40380_e46104_d_n6, assign40380_e46104_d_n7, assign40380_e46104_d_n8, assign40380_e46104_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40380_e46101: f64 = (locals.var_ecpl1s__blk954 + locals.var_ecpl1d__blk1023);
        let assign40380_e46102: f64 = (0.5 * assign40380_e46101);
        (assign40380_e46102, (0.5 * (locals.var_ecpl1s__blk954_dn4 + locals.var_ecpl1d__blk1023_dn4)), (0.5 * (locals.var_ecpl1s__blk954_dn6 + locals.var_ecpl1d__blk1023_dn6)), (0.5 * (locals.var_ecpl1s__blk954_dn7 + locals.var_ecpl1d__blk1023_dn7)), (0.5 * (locals.var_ecpl1s__blk954_dn8 + locals.var_ecpl1d__blk1023_dn8)), (0.5 * (locals.var_ecpl1s__blk954_dn9 + locals.var_ecpl1d__blk1023_dn9)),)
    } else {
        (locals.var_ecpl1__blk1031, locals.var_ecpl1__blk1031_dn4, locals.var_ecpl1__blk1031_dn6, locals.var_ecpl1__blk1031_dn7, locals.var_ecpl1__blk1031_dn8, locals.var_ecpl1__blk1031_dn9,)
    }
};
        locals.var_ecpl1__blk1031 = assign40380_e46104;
        locals.var_ecpl1__blk1031_dn4 = assign40380_e46104_d_n4;
        locals.var_ecpl1__blk1031_dn6 = assign40380_e46104_d_n6;
        locals.var_ecpl1__blk1031_dn7 = assign40380_e46104_d_n7;
        locals.var_ecpl1__blk1031_dn8 = assign40380_e46104_d_n8;
        locals.var_ecpl1__blk1031_dn9 = assign40380_e46104_d_n9;
        locals.var_ecpl1__blk1031_rv = 0.0;

        let (assign40390_e46112, assign40390_e46112_d_n4, assign40390_e46112_d_n6, assign40390_e46112_d_n7, assign40390_e46112_d_n8, assign40390_e46112_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40390_e46109: f64 = (locals.var_ecpl2s__blk955 + locals.var_ecpl2d__blk1024);
        let assign40390_e46110: f64 = (0.5 * assign40390_e46109);
        (assign40390_e46110, (0.5 * (locals.var_ecpl2s__blk955_dn4 + locals.var_ecpl2d__blk1024_dn4)), (0.5 * (locals.var_ecpl2s__blk955_dn6 + locals.var_ecpl2d__blk1024_dn6)), (0.5 * (locals.var_ecpl2s__blk955_dn7 + locals.var_ecpl2d__blk1024_dn7)), (0.5 * (locals.var_ecpl2s__blk955_dn8 + locals.var_ecpl2d__blk1024_dn8)), (0.5 * (locals.var_ecpl2s__blk955_dn9 + locals.var_ecpl2d__blk1024_dn9)),)
    } else {
        (locals.var_ecpl2__blk1032, locals.var_ecpl2__blk1032_dn4, locals.var_ecpl2__blk1032_dn6, locals.var_ecpl2__blk1032_dn7, locals.var_ecpl2__blk1032_dn8, locals.var_ecpl2__blk1032_dn9,)
    }
};
        locals.var_ecpl2__blk1032 = assign40390_e46112;
        locals.var_ecpl2__blk1032_dn4 = assign40390_e46112_d_n4;
        locals.var_ecpl2__blk1032_dn6 = assign40390_e46112_d_n6;
        locals.var_ecpl2__blk1032_dn7 = assign40390_e46112_d_n7;
        locals.var_ecpl2__blk1032_dn8 = assign40390_e46112_d_n8;
        locals.var_ecpl2__blk1032_dn9 = assign40390_e46112_d_n9;
        locals.var_ecpl2__blk1032_rv = 0.0;

        let (assign40400_e46120, assign40400_e46120_d_n4, assign40400_e46120_d_n6, assign40400_e46120_d_n7, assign40400_e46120_d_n8, assign40400_e46120_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40400_e46117: f64 = (locals.var_eeff1s__blk956 + locals.var_eeff1d__blk1025);
        let assign40400_e46118: f64 = (0.5 * assign40400_e46117);
        (assign40400_e46118, (0.5 * (locals.var_eeff1s__blk956_dn4 + locals.var_eeff1d__blk1025_dn4)), (0.5 * (locals.var_eeff1s__blk956_dn6 + locals.var_eeff1d__blk1025_dn6)), (0.5 * (locals.var_eeff1s__blk956_dn7 + locals.var_eeff1d__blk1025_dn7)), (0.5 * (locals.var_eeff1s__blk956_dn8 + locals.var_eeff1d__blk1025_dn8)), (0.5 * (locals.var_eeff1s__blk956_dn9 + locals.var_eeff1d__blk1025_dn9)),)
    } else {
        (locals.var_eeff1__blk1033, locals.var_eeff1__blk1033_dn4, locals.var_eeff1__blk1033_dn6, locals.var_eeff1__blk1033_dn7, locals.var_eeff1__blk1033_dn8, locals.var_eeff1__blk1033_dn9,)
    }
};
        locals.var_eeff1__blk1033 = assign40400_e46120;
        locals.var_eeff1__blk1033_dn4 = assign40400_e46120_d_n4;
        locals.var_eeff1__blk1033_dn6 = assign40400_e46120_d_n6;
        locals.var_eeff1__blk1033_dn7 = assign40400_e46120_d_n7;
        locals.var_eeff1__blk1033_dn8 = assign40400_e46120_d_n8;
        locals.var_eeff1__blk1033_dn9 = assign40400_e46120_d_n9;
        locals.var_eeff1__blk1033_rv = 0.0;

        let (assign40410_e46128, assign40410_e46128_d_n4, assign40410_e46128_d_n6, assign40410_e46128_d_n7, assign40410_e46128_d_n8, assign40410_e46128_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40410_e46125: f64 = (locals.var_eeff2s__blk957 + locals.var_eeff2d__blk1026);
        let assign40410_e46126: f64 = (0.5 * assign40410_e46125);
        (assign40410_e46126, (0.5 * (locals.var_eeff2s__blk957_dn4 + locals.var_eeff2d__blk1026_dn4)), (0.5 * (locals.var_eeff2s__blk957_dn6 + locals.var_eeff2d__blk1026_dn6)), (0.5 * (locals.var_eeff2s__blk957_dn7 + locals.var_eeff2d__blk1026_dn7)), (0.5 * (locals.var_eeff2s__blk957_dn8 + locals.var_eeff2d__blk1026_dn8)), (0.5 * (locals.var_eeff2s__blk957_dn9 + locals.var_eeff2d__blk1026_dn9)),)
    } else {
        (locals.var_eeff2__blk1034, locals.var_eeff2__blk1034_dn4, locals.var_eeff2__blk1034_dn6, locals.var_eeff2__blk1034_dn7, locals.var_eeff2__blk1034_dn8, locals.var_eeff2__blk1034_dn9,)
    }
};
        locals.var_eeff2__blk1034 = assign40410_e46128;
        locals.var_eeff2__blk1034_dn4 = assign40410_e46128_d_n4;
        locals.var_eeff2__blk1034_dn6 = assign40410_e46128_d_n6;
        locals.var_eeff2__blk1034_dn7 = assign40410_e46128_d_n7;
        locals.var_eeff2__blk1034_dn8 = assign40410_e46128_d_n8;
        locals.var_eeff2__blk1034_dn9 = assign40410_e46128_d_n9;
        locals.var_eeff2__blk1034_rv = 0.0;

        let (assign40420_e46141, assign40420_e46141_d_n4, assign40420_e46141_d_n6, assign40420_e46141_d_n7, assign40420_e46141_d_n8, assign40420_e46141_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40420_e46132: f64 = (locals.var_esurf1__blk1027 * locals.var_betn1_t);
        let assign40420_e46135: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign40420_e46136: f64 = (assign40420_e46135).exp();
        let assign40420_e46137: f64 = (assign40420_e46132 * assign40420_e46136);
        let assign40420_e46139: f64 = (assign40420_e46137 * locals.var_ratio_pd__blk1020);
        (assign40420_e46139, ((((((locals.var_esurf1__blk1027_dn4 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn4)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))) * locals.var_ratio_pd__blk1020) + (assign40420_e46137 * locals.var_ratio_pd__blk1020_dn4)), ((((((locals.var_esurf1__blk1027_dn6 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn6)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))) * locals.var_ratio_pd__blk1020) + (assign40420_e46137 * locals.var_ratio_pd__blk1020_dn6)), ((((((locals.var_esurf1__blk1027_dn7 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn7)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))) * locals.var_ratio_pd__blk1020) + (assign40420_e46137 * locals.var_ratio_pd__blk1020_dn7)), ((((((locals.var_esurf1__blk1027_dn8 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn8)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))) * locals.var_ratio_pd__blk1020) + (assign40420_e46137 * locals.var_ratio_pd__blk1020_dn8)), ((((((locals.var_esurf1__blk1027_dn9 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn9)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))) * locals.var_ratio_pd__blk1020) + (assign40420_e46137 * locals.var_ratio_pd__blk1020_dn9)),)
    } else {
        (locals.var_c1__blk1035, locals.var_c1__blk1035_dn4, locals.var_c1__blk1035_dn6, locals.var_c1__blk1035_dn7, locals.var_c1__blk1035_dn8, locals.var_c1__blk1035_dn9,)
    }
};
        locals.var_c1__blk1035 = assign40420_e46141;
        locals.var_c1__blk1035_dn4 = assign40420_e46141_d_n4;
        locals.var_c1__blk1035_dn6 = assign40420_e46141_d_n6;
        locals.var_c1__blk1035_dn7 = assign40420_e46141_d_n7;
        locals.var_c1__blk1035_dn8 = assign40420_e46141_d_n8;
        locals.var_c1__blk1035_dn9 = assign40420_e46141_d_n9;
        locals.var_c1__blk1035_rv = 0.0;

        let (assign40430_e46152, assign40430_e46152_d_n4, assign40430_e46152_d_n6, assign40430_e46152_d_n7, assign40430_e46152_d_n8, assign40430_e46152_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40430_e46145: f64 = (locals.var_esurf2__blk1028 * locals.var_betn2_t);
        let assign40430_e46148: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign40430_e46149: f64 = (assign40430_e46148).exp();
        let assign40430_e46150: f64 = (assign40430_e46145 * assign40430_e46149);
        (assign40430_e46150, ((((locals.var_esurf2__blk1028_dn4 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn4)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))), ((((locals.var_esurf2__blk1028_dn6 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn6)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))), ((((locals.var_esurf2__blk1028_dn7 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn7)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))), ((((locals.var_esurf2__blk1028_dn8 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn8)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))), ((((locals.var_esurf2__blk1028_dn9 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn9)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))),)
    } else {
        (locals.var_c2__blk1036, locals.var_c2__blk1036_dn4, locals.var_c2__blk1036_dn6, locals.var_c2__blk1036_dn7, locals.var_c2__blk1036_dn8, locals.var_c2__blk1036_dn9,)
    }
};
        locals.var_c2__blk1036 = assign40430_e46152;
        locals.var_c2__blk1036_dn4 = assign40430_e46152_d_n4;
        locals.var_c2__blk1036_dn6 = assign40430_e46152_d_n6;
        locals.var_c2__blk1036_dn7 = assign40430_e46152_d_n7;
        locals.var_c2__blk1036_dn8 = assign40430_e46152_d_n8;
        locals.var_c2__blk1036_dn9 = assign40430_e46152_d_n9;
        locals.var_c2__blk1036_rv = 0.0;

        let (assign40440_e46158, assign40440_e46158_d_n4, assign40440_e46158_d_n6, assign40440_e46158_d_n7, assign40440_e46158_d_n8, assign40440_e46158_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40440_e46156: f64 = (locals.var_c1__blk1035 + locals.var_c2__blk1036);
        (assign40440_e46156, (locals.var_c1__blk1035_dn4 + locals.var_c2__blk1036_dn4), (locals.var_c1__blk1035_dn6 + locals.var_c2__blk1036_dn6), (locals.var_c1__blk1035_dn7 + locals.var_c2__blk1036_dn7), (locals.var_c1__blk1035_dn8 + locals.var_c2__blk1036_dn8), (locals.var_c1__blk1035_dn9 + locals.var_c2__blk1036_dn9),)
    } else {
        (locals.var_csum__blk1037, locals.var_csum__blk1037_dn4, locals.var_csum__blk1037_dn6, locals.var_csum__blk1037_dn7, locals.var_csum__blk1037_dn8, locals.var_csum__blk1037_dn9,)
    }
};
        locals.var_csum__blk1037 = assign40440_e46158;
        locals.var_csum__blk1037_dn4 = assign40440_e46158_d_n4;
        locals.var_csum__blk1037_dn6 = assign40440_e46158_d_n6;
        locals.var_csum__blk1037_dn7 = assign40440_e46158_d_n7;
        locals.var_csum__blk1037_dn8 = assign40440_e46158_d_n8;
        locals.var_csum__blk1037_dn9 = assign40440_e46158_d_n9;
        locals.var_csum__blk1037_rv = 0.0;

        let (assign40450_e46168, assign40450_e46168_d_n4, assign40450_e46168_d_n6, assign40450_e46168_d_n7, assign40450_e46168_d_n8, assign40450_e46168_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40450_e46164: f64 = (locals.var_xcorb_i * locals.var_ecpl2__blk1032);
        let assign40450_e46165: f64 = (locals.var_ecpl1__blk1031 + assign40450_e46164);
        let assign40450_e46166: f64 = (locals.var_xcor_i * assign40450_e46165);
        (assign40450_e46166, ((locals.var_xcor_i_dn4 * assign40450_e46165) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn4 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn4)))), ((locals.var_xcor_i_dn6 * assign40450_e46165) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn6 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn6)))), ((locals.var_xcor_i_dn7 * assign40450_e46165) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn7 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn7)))), ((locals.var_xcor_i_dn8 * assign40450_e46165) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn8 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn8)))), ((locals.var_xcor_i_dn9 * assign40450_e46165) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn9 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn9)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40450_e46168;
        locals.var_temp1_dn4 = assign40450_e46168_d_n4;
        locals.var_temp1_dn6 = assign40450_e46168_d_n6;
        locals.var_temp1_dn7 = assign40450_e46168_d_n7;
        locals.var_temp1_dn8 = assign40450_e46168_d_n8;
        locals.var_temp1_dn9 = assign40450_e46168_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign40460_e46193, assign40460_e46193_d_n4, assign40460_e46193_d_n6, assign40460_e46193_d_n7, assign40460_e46193_d_n8, assign40460_e46193_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40460_e46173: f64 = (1.0 + locals.var_temp1);
        let assign40460_e46175: f64 = assign40460_e46173;
        let assign40460_e46178: f64 = (1.0 + locals.var_temp1);
        let assign40460_e46180: f64 = assign40460_e46178;
        let assign40460_e46183: f64 = (1.0 + locals.var_temp1);
        let assign40460_e46185: f64 = assign40460_e46183;
        let assign40460_e46186: f64 = (assign40460_e46180 * assign40460_e46185);
        let assign40460_e46188: f64 = (assign40460_e46186 + 0.01);
        let assign40460_e46189: f64 = (assign40460_e46188).sqrt();
        let assign40460_e46190: f64 = (assign40460_e46175 + assign40460_e46189);
        let assign40460_e46191: f64 = (0.5 * assign40460_e46190);
        (assign40460_e46191, (0.5 * (locals.var_temp1_dn4 + (((locals.var_temp1_dn4 * assign40460_e46185) + (assign40460_e46180 * locals.var_temp1_dn4)) / (2.0 * assign40460_e46189)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign40460_e46185) + (assign40460_e46180 * locals.var_temp1_dn6)) / (2.0 * assign40460_e46189)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign40460_e46185) + (assign40460_e46180 * locals.var_temp1_dn7)) / (2.0 * assign40460_e46189)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign40460_e46185) + (assign40460_e46180 * locals.var_temp1_dn8)) / (2.0 * assign40460_e46189)))), (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign40460_e46185) + (assign40460_e46180 * locals.var_temp1_dn9)) / (2.0 * assign40460_e46189)))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40460_e46193;
        locals.var_temp2_dn4 = assign40460_e46193_d_n4;
        locals.var_temp2_dn6 = assign40460_e46193_d_n6;
        locals.var_temp2_dn7 = assign40460_e46193_d_n7;
        locals.var_temp2_dn8 = assign40460_e46193_d_n8;
        locals.var_temp2_dn9 = assign40460_e46193_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign40470_e46224, assign40470_e46224_d_n4, assign40470_e46224_d_n6, assign40470_e46224_d_n7, assign40470_e46224_d_n8, assign40470_e46224_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40470_e46199: f64 = (0.2 * locals.var_temp1);
        let assign40470_e46200: f64 = (1.0 + assign40470_e46199);
        let assign40470_e46202: f64 = assign40470_e46200;
        let assign40470_e46206: f64 = (0.2 * locals.var_temp1);
        let assign40470_e46207: f64 = (1.0 + assign40470_e46206);
        let assign40470_e46209: f64 = assign40470_e46207;
        let assign40470_e46213: f64 = (0.2 * locals.var_temp1);
        let assign40470_e46214: f64 = (1.0 + assign40470_e46213);
        let assign40470_e46216: f64 = assign40470_e46214;
        let assign40470_e46217: f64 = (assign40470_e46209 * assign40470_e46216);
        let assign40470_e46219: f64 = (assign40470_e46217 + 0.01);
        let assign40470_e46220: f64 = (assign40470_e46219).sqrt();
        let assign40470_e46221: f64 = (assign40470_e46202 + assign40470_e46220);
        let assign40470_e46222: f64 = (0.5 * assign40470_e46221);
        (assign40470_e46222, (0.5 * ((0.2 * locals.var_temp1_dn4) + ((((0.2 * locals.var_temp1_dn4) * assign40470_e46216) + (assign40470_e46209 * (0.2 * locals.var_temp1_dn4))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * locals.var_temp1_dn6) + ((((0.2 * locals.var_temp1_dn6) * assign40470_e46216) + (assign40470_e46209 * (0.2 * locals.var_temp1_dn6))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * locals.var_temp1_dn7) + ((((0.2 * locals.var_temp1_dn7) * assign40470_e46216) + (assign40470_e46209 * (0.2 * locals.var_temp1_dn7))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * locals.var_temp1_dn8) + ((((0.2 * locals.var_temp1_dn8) * assign40470_e46216) + (assign40470_e46209 * (0.2 * locals.var_temp1_dn8))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * locals.var_temp1_dn9) + ((((0.2 * locals.var_temp1_dn9) * assign40470_e46216) + (assign40470_e46209 * (0.2 * locals.var_temp1_dn9))) / (2.0 * assign40470_e46220)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign40470_e46224;
        locals.var_temp3_dn4 = assign40470_e46224_d_n4;
        locals.var_temp3_dn6 = assign40470_e46224_d_n6;
        locals.var_temp3_dn7 = assign40470_e46224_d_n7;
        locals.var_temp3_dn8 = assign40470_e46224_d_n8;
        locals.var_temp3_dn9 = assign40470_e46224_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign40480_e46230, assign40480_e46230_d_n4, assign40480_e46230_d_n6, assign40480_e46230_d_n7, assign40480_e46230_d_n8, assign40480_e46230_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40480_e46228: f64 = (locals.var_temp2 / locals.var_temp3);
        (assign40480_e46228, (((locals.var_temp2_dn4 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn6 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn7 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn8 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn9 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3)),)
    } else {
        (locals.var_fcor__blk1038, locals.var_fcor__blk1038_dn4, locals.var_fcor__blk1038_dn6, locals.var_fcor__blk1038_dn7, locals.var_fcor__blk1038_dn8, locals.var_fcor__blk1038_dn9,)
    }
};
        locals.var_fcor__blk1038 = assign40480_e46230;
        locals.var_fcor__blk1038_dn4 = assign40480_e46230_d_n4;
        locals.var_fcor__blk1038_dn6 = assign40480_e46230_d_n6;
        locals.var_fcor__blk1038_dn7 = assign40480_e46230_d_n7;
        locals.var_fcor__blk1038_dn8 = assign40480_e46230_d_n8;
        locals.var_fcor__blk1038_dn9 = assign40480_e46230_d_n9;
        locals.var_fcor__blk1038_rv = 0.0;

        let (assign40490_e46259, assign40490_e46259_d_n4, assign40490_e46259_d_n6, assign40490_e46259_d_n7, assign40490_e46259_d_n8, assign40490_e46259_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40490_e46236: f64 = (locals.var_csfi_i * locals.var_ecpl1__blk1031);
        let assign40490_e46237: f64 = (1.0 + assign40490_e46236);
        let assign40490_e46240: f64 = (locals.var_csbi_i * locals.var_ecpl2__blk1032);
        let assign40490_e46241: f64 = (assign40490_e46237 + assign40490_e46240);
        let assign40490_e46242: f64 = (locals.var_cs_i * assign40490_e46241);
        let assign40490_e46244: f64 = (-locals.var_thecs_i);
        let assign40490_e46248: f64 = (locals.var_qi1m__blk1029 * locals.var_inv_qi1cs);
        let assign40490_e46249: f64 = (1.0 + assign40490_e46248);
        let assign40490_e46252: f64 = (locals.var_qi2m__blk1030 * locals.var_inv_qi2cs);
        let assign40490_e46253: f64 = (assign40490_e46249 + assign40490_e46252);
        let assign40490_e46254: f64 = (assign40490_e46253).ln();
        let assign40490_e46255: f64 = (assign40490_e46244 * assign40490_e46254);
        let assign40490_e46256: f64 = (assign40490_e46255).exp();
        let assign40490_e46257: f64 = (assign40490_e46242 * assign40490_e46256);
        (assign40490_e46257, ((((locals.var_cs_i_dn4 * assign40490_e46241) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn4) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn4)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-locals.var_thecs_i_dn4) * assign40490_e46254) + (assign40490_e46244 * (((locals.var_qi1m__blk1029_dn4 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn4 * locals.var_inv_qi2cs)) / assign40490_e46253)))))), ((((locals.var_cs_i_dn6 * assign40490_e46241) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn6) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn6)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-locals.var_thecs_i_dn6) * assign40490_e46254) + (assign40490_e46244 * (((locals.var_qi1m__blk1029_dn6 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn6 * locals.var_inv_qi2cs)) / assign40490_e46253)))))), ((((locals.var_cs_i_dn7 * assign40490_e46241) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn7) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn7)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-locals.var_thecs_i_dn7) * assign40490_e46254) + (assign40490_e46244 * (((locals.var_qi1m__blk1029_dn7 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn7 * locals.var_inv_qi2cs)) / assign40490_e46253)))))), ((((locals.var_cs_i_dn8 * assign40490_e46241) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn8) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn8)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-locals.var_thecs_i_dn8) * assign40490_e46254) + (assign40490_e46244 * (((locals.var_qi1m__blk1029_dn8 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn8 * locals.var_inv_qi2cs)) / assign40490_e46253)))))), ((((locals.var_cs_i_dn9 * assign40490_e46241) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn9) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn9)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-locals.var_thecs_i_dn9) * assign40490_e46254) + (assign40490_e46244 * (((locals.var_qi1m__blk1029_dn9 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn9 * locals.var_inv_qi2cs)) / assign40490_e46253)))))),)
    } else {
        (locals.var_gcs__blk1039, locals.var_gcs__blk1039_dn4, locals.var_gcs__blk1039_dn6, locals.var_gcs__blk1039_dn7, locals.var_gcs__blk1039_dn8, locals.var_gcs__blk1039_dn9,)
    }
};
        locals.var_gcs__blk1039 = assign40490_e46259;
        locals.var_gcs__blk1039_dn4 = assign40490_e46259_d_n4;
        locals.var_gcs__blk1039_dn6 = assign40490_e46259_d_n6;
        locals.var_gcs__blk1039_dn7 = assign40490_e46259_d_n7;
        locals.var_gcs__blk1039_dn8 = assign40490_e46259_d_n8;
        locals.var_gcs__blk1039_dn9 = assign40490_e46259_d_n9;
        locals.var_gcs__blk1039_rv = 0.0;

        let assign40500_e46262: f64 = if locals.var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign40500_e46262;
        locals.var_guard1222_rv = 0.0;

    }
}
