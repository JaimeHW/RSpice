#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_112(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign31390_e46183: f64 = if locals.var_chi__blk945 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1018 = assign31390_e46183;

        let (assign31430_e46245, assign31430_e46245_d_n0, assign31430_e46245_d_n2, assign31430_e46245_d_n6, assign31430_e46245_d_n7, assign31430_e46245_d_n10, assign31430_e46245_d_n11, assign31430_e46245_d_n12, assign31430_e46245_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1018 != 0.0)) {
        let assign31430_e46239: f64 = (locals.var_fb__blk969 * locals.var_fb__blk969);
        let assign31430_e46242: f64 = (10.0 * 2.220446049250313e-16);
        let assign31430_e46243: f64 = (assign31430_e46239 + assign31430_e46242);
        (assign31430_e46243, ((locals.var_fb__blk969_dn0 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn0)), ((locals.var_fb__blk969_dn2 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn2)), ((locals.var_fb__blk969_dn6 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn6)), ((locals.var_fb__blk969_dn7 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn7)), ((locals.var_fb__blk969_dn10 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn10)), ((locals.var_fb__blk969_dn11 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn11)), ((locals.var_fb__blk969_dn12 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn12)), ((locals.var_fb__blk969_dn17 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn17)),)
    } else {
        (locals.var_xi0__blk978, locals.var_xi0__blk978_dn0, locals.var_xi0__blk978_dn2, locals.var_xi0__blk978_dn6, locals.var_xi0__blk978_dn7, locals.var_xi0__blk978_dn10, locals.var_xi0__blk978_dn11, locals.var_xi0__blk978_dn12, locals.var_xi0__blk978_dn17,)
    }
};
        locals.var_xi0__blk978 = assign31430_e46245;
        locals.var_xi0__blk978_dn0 = assign31430_e46245_d_n0;
        locals.var_xi0__blk978_dn2 = assign31430_e46245_d_n2;
        locals.var_xi0__blk978_dn6 = assign31430_e46245_d_n6;
        locals.var_xi0__blk978_dn7 = assign31430_e46245_d_n7;
        locals.var_xi0__blk978_dn10 = assign31430_e46245_d_n10;
        locals.var_xi0__blk978_dn11 = assign31430_e46245_d_n11;
        locals.var_xi0__blk978_dn12 = assign31430_e46245_d_n12;
        locals.var_xi0__blk978_dn17 = assign31430_e46245_d_n17;

        let (assign31440_e46265, assign31440_e46265_d_n0, assign31440_e46265_d_n2, assign31440_e46265_d_n6, assign31440_e46265_d_n7, assign31440_e46265_d_n10, assign31440_e46265_d_n11, assign31440_e46265_d_n12, assign31440_e46265_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1018 != 0.0)) {
        let assign31440_e46262: f64 = (10.0 * 2.220446049250313e-16);
        let assign31440_e46263: f64 = (locals.var_fb__blk969 + assign31440_e46262);
        (assign31440_e46263, locals.var_fb__blk969_dn0, locals.var_fb__blk969_dn2, locals.var_fb__blk969_dn6, locals.var_fb__blk969_dn7, locals.var_fb__blk969_dn10, locals.var_fb__blk969_dn11, locals.var_fb__blk969_dn12, locals.var_fb__blk969_dn17,)
    } else {
        (locals.var_xi0p12__blk979, locals.var_xi0p12__blk979_dn0, locals.var_xi0p12__blk979_dn2, locals.var_xi0p12__blk979_dn6, locals.var_xi0p12__blk979_dn7, locals.var_xi0p12__blk979_dn10, locals.var_xi0p12__blk979_dn11, locals.var_xi0p12__blk979_dn12, locals.var_xi0p12__blk979_dn17,)
    }
};
        locals.var_xi0p12__blk979 = assign31440_e46265;
        locals.var_xi0p12__blk979_dn0 = assign31440_e46265_d_n0;
        locals.var_xi0p12__blk979_dn2 = assign31440_e46265_d_n2;
        locals.var_xi0p12__blk979_dn6 = assign31440_e46265_d_n6;
        locals.var_xi0p12__blk979_dn7 = assign31440_e46265_d_n7;
        locals.var_xi0p12__blk979_dn10 = assign31440_e46265_d_n10;
        locals.var_xi0p12__blk979_dn11 = assign31440_e46265_d_n11;
        locals.var_xi0p12__blk979_dn12 = assign31440_e46265_d_n12;
        locals.var_xi0p12__blk979_dn17 = assign31440_e46265_d_n17;

        let (assign31460_e46301, assign31460_e46301_d_n0, assign31460_e46301_d_n2, assign31460_e46301_d_n6, assign31460_e46301_d_n7, assign31460_e46301_d_n10, assign31460_e46301_d_n11, assign31460_e46301_d_n12, assign31460_e46301_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1018 == 0.0)) {
        let assign31460_e46299: f64 = (locals.var_chi__blk945 - 1.0);
        (assign31460_e46299, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17,)
    } else {
        (locals.var_xi0__blk978, locals.var_xi0__blk978_dn0, locals.var_xi0__blk978_dn2, locals.var_xi0__blk978_dn6, locals.var_xi0__blk978_dn7, locals.var_xi0__blk978_dn10, locals.var_xi0__blk978_dn11, locals.var_xi0__blk978_dn12, locals.var_xi0__blk978_dn17,)
    }
};
        locals.var_xi0__blk978 = assign31460_e46301;
        locals.var_xi0__blk978_dn0 = assign31460_e46301_d_n0;
        locals.var_xi0__blk978_dn2 = assign31460_e46301_d_n2;
        locals.var_xi0__blk978_dn6 = assign31460_e46301_d_n6;
        locals.var_xi0__blk978_dn7 = assign31460_e46301_d_n7;
        locals.var_xi0__blk978_dn10 = assign31460_e46301_d_n10;
        locals.var_xi0__blk978_dn11 = assign31460_e46301_d_n11;
        locals.var_xi0__blk978_dn12 = assign31460_e46301_d_n12;
        locals.var_xi0__blk978_dn17 = assign31460_e46301_d_n17;

        let (assign31470_e46319, assign31470_e46319_d_n0, assign31470_e46319_d_n2, assign31470_e46319_d_n6, assign31470_e46319_d_n7, assign31470_e46319_d_n10, assign31470_e46319_d_n11, assign31470_e46319_d_n12, assign31470_e46319_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1018 == 0.0)) {
        let assign31470_e46317: f64 = (locals.var_xi0__blk978).sqrt();
        (assign31470_e46317, (locals.var_xi0__blk978_dn0 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn2 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn6 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn7 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn10 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn11 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn12 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn17 / (2.0 * assign31470_e46317)),)
    } else {
        (locals.var_xi0p12__blk979, locals.var_xi0p12__blk979_dn0, locals.var_xi0p12__blk979_dn2, locals.var_xi0p12__blk979_dn6, locals.var_xi0p12__blk979_dn7, locals.var_xi0p12__blk979_dn10, locals.var_xi0p12__blk979_dn11, locals.var_xi0p12__blk979_dn12, locals.var_xi0p12__blk979_dn17,)
    }
};
        locals.var_xi0p12__blk979 = assign31470_e46319;
        locals.var_xi0p12__blk979_dn0 = assign31470_e46319_d_n0;
        locals.var_xi0p12__blk979_dn2 = assign31470_e46319_d_n2;
        locals.var_xi0p12__blk979_dn6 = assign31470_e46319_d_n6;
        locals.var_xi0p12__blk979_dn7 = assign31470_e46319_d_n7;
        locals.var_xi0p12__blk979_dn10 = assign31470_e46319_d_n10;
        locals.var_xi0p12__blk979_dn11 = assign31470_e46319_d_n11;
        locals.var_xi0p12__blk979_dn12 = assign31470_e46319_d_n12;
        locals.var_xi0p12__blk979_dn17 = assign31470_e46319_d_n17;

        let (assign31480_e46335, assign31480_e46335_d_n0, assign31480_e46335_d_n2, assign31480_e46335_d_n6, assign31480_e46335_d_n7, assign31480_e46335_d_n10, assign31480_e46335_d_n11, assign31480_e46335_d_n12, assign31480_e46335_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31480_e46333: f64 = (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979);
        (assign31480_e46333, ((locals.var_cnst0over__blk930_dn0 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn0)), ((locals.var_cnst0over__blk930_dn2 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn2)), ((locals.var_cnst0over__blk930_dn6 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn6)), ((locals.var_cnst0over__blk930_dn7 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn7)), ((locals.var_cnst0over__blk930_dn10 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn10)), ((locals.var_cnst0over__blk930_dn11 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn11)), ((locals.var_cnst0over__blk930_dn12 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn12)), ((locals.var_cnst0over__blk930_dn17 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign31480_e46335;
        locals.var_qbuld_dn0 = assign31480_e46335_d_n0;
        locals.var_qbuld_dn2 = assign31480_e46335_d_n2;
        locals.var_qbuld_dn6 = assign31480_e46335_d_n6;
        locals.var_qbuld_dn7 = assign31480_e46335_d_n7;
        locals.var_qbuld_dn10 = assign31480_e46335_d_n10;
        locals.var_qbuld_dn11 = assign31480_e46335_d_n11;
        locals.var_qbuld_dn12 = assign31480_e46335_d_n12;
        locals.var_qbuld_dn17 = assign31480_e46335_d_n17;

        let (assign31490_e46353, assign31490_e46353_d_n0, assign31490_e46353_d_n2, assign31490_e46353_d_n6, assign31490_e46353_d_n7, assign31490_e46353_d_n10, assign31490_e46353_d_n11, assign31490_e46353_d_n12, assign31490_e46353_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31490_e46350: f64 = (locals.var_fs02__blk971 + locals.var_xi0p12__blk979);
        let assign31490_e46351: f64 = (1.0 / assign31490_e46350);
        (assign31490_e46351, (-((locals.var_fs02__blk971_dn0 + locals.var_xi0p12__blk979_dn0) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn2 + locals.var_xi0p12__blk979_dn2) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn6 + locals.var_xi0p12__blk979_dn6) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn7 + locals.var_xi0p12__blk979_dn7) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn10 + locals.var_xi0p12__blk979_dn10) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn11 + locals.var_xi0p12__blk979_dn11) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn12 + locals.var_xi0p12__blk979_dn12) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn17 + locals.var_xi0p12__blk979_dn17) / (assign31490_e46350 * assign31490_e46350))),)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign31490_e46353;
        locals.var_t1__blk898_dn0 = assign31490_e46353_d_n0;
        locals.var_t1__blk898_dn2 = assign31490_e46353_d_n2;
        locals.var_t1__blk898_dn6 = assign31490_e46353_d_n6;
        locals.var_t1__blk898_dn7 = assign31490_e46353_d_n7;
        locals.var_t1__blk898_dn10 = assign31490_e46353_d_n10;
        locals.var_t1__blk898_dn11 = assign31490_e46353_d_n11;
        locals.var_t1__blk898_dn12 = assign31490_e46353_d_n12;
        locals.var_t1__blk898_dn17 = assign31490_e46353_d_n17;

        let (assign31500_e46371, assign31500_e46371_d_n0, assign31500_e46371_d_n2, assign31500_e46371_d_n6, assign31500_e46371_d_n7, assign31500_e46371_d_n10, assign31500_e46371_d_n11, assign31500_e46371_d_n12, assign31500_e46371_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31500_e46367: f64 = (locals.var_cnst0over__blk930 * locals.var_fs01__blk967);
        let assign31500_e46369: f64 = (assign31500_e46367 * locals.var_t1__blk898);
        (assign31500_e46369, ((((locals.var_cnst0over__blk930_dn0 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn0)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn0)), ((((locals.var_cnst0over__blk930_dn2 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn2)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn2)), ((((locals.var_cnst0over__blk930_dn6 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn6)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn6)), ((((locals.var_cnst0over__blk930_dn7 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn7)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn7)), ((((locals.var_cnst0over__blk930_dn10 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn10)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn10)), ((((locals.var_cnst0over__blk930_dn11 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn11)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn11)), ((((locals.var_cnst0over__blk930_dn12 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn12)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn12)), ((((locals.var_cnst0over__blk930_dn17 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn17)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign31500_e46371;
        locals.var_qiuld_dn0 = assign31500_e46371_d_n0;
        locals.var_qiuld_dn2 = assign31500_e46371_d_n2;
        locals.var_qiuld_dn6 = assign31500_e46371_d_n6;
        locals.var_qiuld_dn7 = assign31500_e46371_d_n7;
        locals.var_qiuld_dn10 = assign31500_e46371_d_n10;
        locals.var_qiuld_dn11 = assign31500_e46371_d_n11;
        locals.var_qiuld_dn12 = assign31500_e46371_d_n12;
        locals.var_qiuld_dn17 = assign31500_e46371_d_n17;

        let (assign31510_e46387, assign31510_e46387_d_n0, assign31510_e46387_d_n2, assign31510_e46387_d_n6, assign31510_e46387_d_n7, assign31510_e46387_d_n10, assign31510_e46387_d_n11, assign31510_e46387_d_n12, assign31510_e46387_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31510_e46385: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign31510_e46385, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign31510_e46387;
        locals.var_qsuld_dn0 = assign31510_e46387_d_n0;
        locals.var_qsuld_dn2 = assign31510_e46387_d_n2;
        locals.var_qsuld_dn6 = assign31510_e46387_d_n6;
        locals.var_qsuld_dn7 = assign31510_e46387_d_n7;
        locals.var_qsuld_dn10 = assign31510_e46387_d_n10;
        locals.var_qsuld_dn11 = assign31510_e46387_d_n11;
        locals.var_qsuld_dn12 = assign31510_e46387_d_n12;
        locals.var_qsuld_dn17 = assign31510_e46387_d_n17;

        let (assign31520_e46398, assign31520_e46398_d_n0, assign31520_e46398_d_n2, assign31520_e46398_d_n6, assign31520_e46398_d_n7, assign31520_e46398_d_n10, assign31520_e46398_d_n11, assign31520_e46398_d_n12, assign31520_e46398_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign31520_e46396: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign31520_e46396, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign31520_e46398;
        locals.var_qiuld_dn0 = assign31520_e46398_d_n0;
        locals.var_qiuld_dn2 = assign31520_e46398_d_n2;
        locals.var_qiuld_dn6 = assign31520_e46398_d_n6;
        locals.var_qiuld_dn7 = assign31520_e46398_d_n7;
        locals.var_qiuld_dn10 = assign31520_e46398_d_n10;
        locals.var_qiuld_dn11 = assign31520_e46398_d_n11;
        locals.var_qiuld_dn12 = assign31520_e46398_d_n12;
        locals.var_qiuld_dn17 = assign31520_e46398_d_n17;

        let (assign31530_e46416, assign31530_e46416_d_n0, assign31530_e46416_d_n2, assign31530_e46416_d_n6, assign31530_e46416_d_n7, assign31530_e46416_d_n10, assign31530_e46416_d_n11, assign31530_e46416_d_n12, assign31530_e46416_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let (assign31530_e46414,) = {
            if (p.p43 == 1.0) {
                let assign31530_e46410: f64 = (locals.var_w_dioscv * locals.var_lov);
                (assign31530_e46410,)
            } else {
                let assign31530_e46413: f64 = (locals.var_weffcv_nf * locals.var_lov);
                (assign31530_e46413,)
            }
        };
        (assign31530_e46414, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk901, locals.var_t4__blk901_dn0, locals.var_t4__blk901_dn2, locals.var_t4__blk901_dn6, locals.var_t4__blk901_dn7, locals.var_t4__blk901_dn10, locals.var_t4__blk901_dn11, locals.var_t4__blk901_dn12, locals.var_t4__blk901_dn17,)
    }
};
        locals.var_t4__blk901 = assign31530_e46416;
        locals.var_t4__blk901_dn0 = assign31530_e46416_d_n0;
        locals.var_t4__blk901_dn2 = assign31530_e46416_d_n2;
        locals.var_t4__blk901_dn6 = assign31530_e46416_d_n6;
        locals.var_t4__blk901_dn7 = assign31530_e46416_d_n7;
        locals.var_t4__blk901_dn10 = assign31530_e46416_d_n10;
        locals.var_t4__blk901_dn11 = assign31530_e46416_d_n11;
        locals.var_t4__blk901_dn12 = assign31530_e46416_d_n12;
        locals.var_t4__blk901_dn17 = assign31530_e46416_d_n17;

        let assign31540_e46427: f64 = if (((locals.var_flg_overs__blk916 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloops__blk914 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1020 = assign31540_e46427;

        let (assign31550_e46440, assign31550_e46440_d_n0, assign31550_e46440_d_n2, assign31550_e46440_d_n6, assign31550_e46440_d_n7, assign31550_e46440_d_n10, assign31550_e46440_d_n11, assign31550_e46440_d_n12, assign31550_e46440_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1020 != 0.0)) {
        let assign31550_e46438: f64 = (locals.var_t4__blk901 * locals.var_qsuld);
        (assign31550_e46438, ((locals.var_t4__blk901_dn0 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn17,)
    }
};
        locals.var_qovs = assign31550_e46440;
        locals.var_qovs_dn0 = assign31550_e46440_d_n0;
        locals.var_qovs_dn2 = assign31550_e46440_d_n2;
        locals.var_qovs_dn6 = assign31550_e46440_d_n6;
        locals.var_qovs_dn7 = assign31550_e46440_d_n7;
        locals.var_qovs_dn10 = assign31550_e46440_d_n10;
        locals.var_qovs_dn11 = assign31550_e46440_d_n11;
        locals.var_qovs_dn12 = assign31550_e46440_d_n12;
        locals.var_qovs_dn17 = assign31550_e46440_d_n17;

        let (assign31560_e46453, assign31560_e46453_d_n0, assign31560_e46453_d_n2, assign31560_e46453_d_n6, assign31560_e46453_d_n7, assign31560_e46453_d_n10, assign31560_e46453_d_n11, assign31560_e46453_d_n12, assign31560_e46453_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1020 != 0.0)) {
        let assign31560_e46451: f64 = (locals.var_t4__blk901 * locals.var_qbuld);
        (assign31560_e46451, ((locals.var_t4__blk901_dn0 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, locals.var_qbsld_dn17,)
    }
};
        locals.var_qbsld = assign31560_e46453;
        locals.var_qbsld_dn0 = assign31560_e46453_d_n0;
        locals.var_qbsld_dn2 = assign31560_e46453_d_n2;
        locals.var_qbsld_dn6 = assign31560_e46453_d_n6;
        locals.var_qbsld_dn7 = assign31560_e46453_d_n7;
        locals.var_qbsld_dn10 = assign31560_e46453_d_n10;
        locals.var_qbsld_dn11 = assign31560_e46453_d_n11;
        locals.var_qbsld_dn12 = assign31560_e46453_d_n12;
        locals.var_qbsld_dn17 = assign31560_e46453_d_n17;

        let assign31570_e46464: f64 = if (((locals.var_flg_overd__blk917 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloopd__blk915 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1021 = assign31570_e46464;

        let (assign31580_e46477, assign31580_e46477_d_n0, assign31580_e46477_d_n2, assign31580_e46477_d_n6, assign31580_e46477_d_n7, assign31580_e46477_d_n10, assign31580_e46477_d_n11, assign31580_e46477_d_n12, assign31580_e46477_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1021 != 0.0)) {
        let assign31580_e46475: f64 = (locals.var_t4__blk901 * locals.var_qsuld);
        (assign31580_e46475, ((locals.var_t4__blk901_dn0 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn17,)
    }
};
        locals.var_qovd = assign31580_e46477;
        locals.var_qovd_dn0 = assign31580_e46477_d_n0;
        locals.var_qovd_dn2 = assign31580_e46477_d_n2;
        locals.var_qovd_dn6 = assign31580_e46477_d_n6;
        locals.var_qovd_dn7 = assign31580_e46477_d_n7;
        locals.var_qovd_dn10 = assign31580_e46477_d_n10;
        locals.var_qovd_dn11 = assign31580_e46477_d_n11;
        locals.var_qovd_dn12 = assign31580_e46477_d_n12;
        locals.var_qovd_dn17 = assign31580_e46477_d_n17;

        let (assign31590_e46490, assign31590_e46490_d_n0, assign31590_e46490_d_n2, assign31590_e46490_d_n6, assign31590_e46490_d_n7, assign31590_e46490_d_n10, assign31590_e46490_d_n11, assign31590_e46490_d_n12, assign31590_e46490_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1021 != 0.0)) {
        let assign31590_e46488: f64 = (locals.var_t4__blk901 * locals.var_qbuld);
        (assign31590_e46488, ((locals.var_t4__blk901_dn0 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, locals.var_qbdld_dn17,)
    }
};
        locals.var_qbdld = assign31590_e46490;
        locals.var_qbdld_dn0 = assign31590_e46490_d_n0;
        locals.var_qbdld_dn2 = assign31590_e46490_d_n2;
        locals.var_qbdld_dn6 = assign31590_e46490_d_n6;
        locals.var_qbdld_dn7 = assign31590_e46490_d_n7;
        locals.var_qbdld_dn10 = assign31590_e46490_d_n10;
        locals.var_qbdld_dn11 = assign31590_e46490_d_n11;
        locals.var_qbdld_dn12 = assign31590_e46490_d_n12;
        locals.var_qbdld_dn17 = assign31590_e46490_d_n17;

        let (assign31600_e46502,) = {
    if ((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) {
        let assign31600_e46496: f64 = (locals.var_modervs * locals.var_cgso_given);
        let assign31600_e46499: f64 = (locals.var_modenml * locals.var_cgdo_given);
        let assign31600_e46500: f64 = (assign31600_e46496 + assign31600_e46499);
        (assign31600_e46500,)
    } else {
        (locals.var_flg_overgiven,)
    }
};
        locals.var_flg_overgiven = assign31600_e46502;

        let (assign31610_e46516, assign31610_e46516_d_n0, assign31610_e46516_d_n2, assign31610_e46516_d_n6, assign31610_e46516_d_n7, assign31610_e46516_d_n10, assign31610_e46516_d_n11, assign31610_e46516_d_n12, assign31610_e46516_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31610_e46510: f64 = (locals.var_modervs * p.p170);
        let assign31610_e46513: f64 = (locals.var_modenml * p.p169);
        let assign31610_e46514: f64 = (assign31610_e46510 + assign31610_e46513);
        (assign31610_e46514, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31610_e46516;
        locals.var_cgdoe_dn0 = assign31610_e46516_d_n0;
        locals.var_cgdoe_dn2 = assign31610_e46516_d_n2;
        locals.var_cgdoe_dn6 = assign31610_e46516_d_n6;
        locals.var_cgdoe_dn7 = assign31610_e46516_d_n7;
        locals.var_cgdoe_dn10 = assign31610_e46516_d_n10;
        locals.var_cgdoe_dn11 = assign31610_e46516_d_n11;
        locals.var_cgdoe_dn12 = assign31610_e46516_d_n12;
        locals.var_cgdoe_dn17 = assign31610_e46516_d_n17;

        let assign31620_e46519: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1022 = assign31620_e46519;

        let (assign31630_e46535, assign31630_e46535_d_n0, assign31630_e46535_d_n2, assign31630_e46535_d_n6, assign31630_e46535_d_n7, assign31630_e46535_d_n10, assign31630_e46535_d_n11, assign31630_e46535_d_n12, assign31630_e46535_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1022 != 0.0)) {
        let assign31630_e46529: f64 = (locals.var_modervs * locals.var_w_dioscv);
        let assign31630_e46532: f64 = (locals.var_modenml * locals.var_w_diodcv);
        let assign31630_e46533: f64 = (assign31630_e46529 + assign31630_e46532);
        (assign31630_e46533, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign31630_e46535;
        locals.var_t1__blk898_dn0 = assign31630_e46535_d_n0;
        locals.var_t1__blk898_dn2 = assign31630_e46535_d_n2;
        locals.var_t1__blk898_dn6 = assign31630_e46535_d_n6;
        locals.var_t1__blk898_dn7 = assign31630_e46535_d_n7;
        locals.var_t1__blk898_dn10 = assign31630_e46535_d_n10;
        locals.var_t1__blk898_dn11 = assign31630_e46535_d_n11;
        locals.var_t1__blk898_dn12 = assign31630_e46535_d_n12;
        locals.var_t1__blk898_dn17 = assign31630_e46535_d_n17;

        let (assign31640_e46548, assign31640_e46548_d_n0, assign31640_e46548_d_n2, assign31640_e46548_d_n6, assign31640_e46548_d_n7, assign31640_e46548_d_n10, assign31640_e46548_d_n11, assign31640_e46548_d_n12, assign31640_e46548_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1022 != 0.0)) {
        let assign31640_e46545: f64 = (-locals.var_t1__blk898);
        let assign31640_e46546: f64 = (locals.var_cgdoe * assign31640_e46545);
        (assign31640_e46546, ((locals.var_cgdoe_dn0 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn0))), ((locals.var_cgdoe_dn2 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn2))), ((locals.var_cgdoe_dn6 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn6))), ((locals.var_cgdoe_dn7 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn7))), ((locals.var_cgdoe_dn10 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn10))), ((locals.var_cgdoe_dn11 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn11))), ((locals.var_cgdoe_dn12 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn12))), ((locals.var_cgdoe_dn17 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn17))),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31640_e46548;
        locals.var_cgdoe_dn0 = assign31640_e46548_d_n0;
        locals.var_cgdoe_dn2 = assign31640_e46548_d_n2;
        locals.var_cgdoe_dn6 = assign31640_e46548_d_n6;
        locals.var_cgdoe_dn7 = assign31640_e46548_d_n7;
        locals.var_cgdoe_dn10 = assign31640_e46548_d_n10;
        locals.var_cgdoe_dn11 = assign31640_e46548_d_n11;
        locals.var_cgdoe_dn12 = assign31640_e46548_d_n12;
        locals.var_cgdoe_dn17 = assign31640_e46548_d_n17;

        let (assign31650_e46562, assign31650_e46562_d_n0, assign31650_e46562_d_n2, assign31650_e46562_d_n6, assign31650_e46562_d_n7, assign31650_e46562_d_n10, assign31650_e46562_d_n11, assign31650_e46562_d_n12, assign31650_e46562_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1022 == 0.0)) {
        let assign31650_e46559: f64 = (-locals.var_weffcv_nf);
        let assign31650_e46560: f64 = (locals.var_cgdoe * assign31650_e46559);
        (assign31650_e46560, (locals.var_cgdoe_dn0 * assign31650_e46559), (locals.var_cgdoe_dn2 * assign31650_e46559), (locals.var_cgdoe_dn6 * assign31650_e46559), (locals.var_cgdoe_dn7 * assign31650_e46559), (locals.var_cgdoe_dn10 * assign31650_e46559), (locals.var_cgdoe_dn11 * assign31650_e46559), (locals.var_cgdoe_dn12 * assign31650_e46559), (locals.var_cgdoe_dn17 * assign31650_e46559),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31650_e46562;
        locals.var_cgdoe_dn0 = assign31650_e46562_d_n0;
        locals.var_cgdoe_dn2 = assign31650_e46562_d_n2;
        locals.var_cgdoe_dn6 = assign31650_e46562_d_n6;
        locals.var_cgdoe_dn7 = assign31650_e46562_d_n7;
        locals.var_cgdoe_dn10 = assign31650_e46562_d_n10;
        locals.var_cgdoe_dn11 = assign31650_e46562_d_n11;
        locals.var_cgdoe_dn12 = assign31650_e46562_d_n12;
        locals.var_cgdoe_dn17 = assign31650_e46562_d_n17;

        let (assign31660_e46577, assign31660_e46577_d_n0, assign31660_e46577_d_n2, assign31660_e46577_d_n6, assign31660_e46577_d_n7, assign31660_e46577_d_n10, assign31660_e46577_d_n11, assign31660_e46577_d_n12, assign31660_e46577_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31660_e46570: f64 = (-locals.var_cgdoe);
        let assign31660_e46573: f64 = (locals.var_vgs - locals.var_vds);
        let assign31660_e46574: f64 = (assign31660_e46570 * assign31660_e46573);
        let assign31660_e46575: f64 = (locals.var_qgod + assign31660_e46574);
        (assign31660_e46575, (locals.var_qgod_dn0 + (((-locals.var_cgdoe_dn0) * assign31660_e46573) + (assign31660_e46570 * (-locals.var_vds_dn0)))), (locals.var_qgod_dn2 + (((-locals.var_cgdoe_dn2) * assign31660_e46573) + (assign31660_e46570 * (-locals.var_vds_dn2)))), (locals.var_qgod_dn6 + (((-locals.var_cgdoe_dn6) * assign31660_e46573) + (assign31660_e46570 * (locals.var_vgs_dn6 - locals.var_vds_dn6)))), (locals.var_qgod_dn7 + (((-locals.var_cgdoe_dn7) * assign31660_e46573) + (assign31660_e46570 * (locals.var_vgs_dn7 - locals.var_vds_dn7)))), (locals.var_qgod_dn10 + (((-locals.var_cgdoe_dn10) * assign31660_e46573) + (assign31660_e46570 * (-locals.var_vds_dn10)))), (locals.var_qgod_dn11 + (((-locals.var_cgdoe_dn11) * assign31660_e46573) + (assign31660_e46570 * (locals.var_vgs_dn11 - locals.var_vds_dn11)))), (locals.var_qgod_dn12 + (((-locals.var_cgdoe_dn12) * assign31660_e46573) + (assign31660_e46570 * (-locals.var_vds_dn12)))), (locals.var_qgod_dn17 + (((-locals.var_cgdoe_dn17) * assign31660_e46573) + (assign31660_e46570 * (-locals.var_vds_dn17)))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign31660_e46577;
        locals.var_qgod_dn0 = assign31660_e46577_d_n0;
        locals.var_qgod_dn2 = assign31660_e46577_d_n2;
        locals.var_qgod_dn6 = assign31660_e46577_d_n6;
        locals.var_qgod_dn7 = assign31660_e46577_d_n7;
        locals.var_qgod_dn10 = assign31660_e46577_d_n10;
        locals.var_qgod_dn11 = assign31660_e46577_d_n11;
        locals.var_qgod_dn12 = assign31660_e46577_d_n12;
        locals.var_qgod_dn17 = assign31660_e46577_d_n17;

        let (assign31670_e46589,) = {
    if ((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) {
        let assign31670_e46583: f64 = (locals.var_modenml * locals.var_cgso_given);
        let assign31670_e46586: f64 = (locals.var_modervs * locals.var_cgdo_given);
        let assign31670_e46587: f64 = (assign31670_e46583 + assign31670_e46586);
        (assign31670_e46587,)
    } else {
        (locals.var_flg_overgiven,)
    }
};
        locals.var_flg_overgiven = assign31670_e46589;

        let (assign31680_e46603, assign31680_e46603_d_n0, assign31680_e46603_d_n2, assign31680_e46603_d_n6, assign31680_e46603_d_n7, assign31680_e46603_d_n10, assign31680_e46603_d_n11, assign31680_e46603_d_n12, assign31680_e46603_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31680_e46597: f64 = (locals.var_modenml * p.p170);
        let assign31680_e46600: f64 = (locals.var_modervs * p.p169);
        let assign31680_e46601: f64 = (assign31680_e46597 + assign31680_e46600);
        (assign31680_e46601, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31680_e46603;
        locals.var_cgsoe_dn0 = assign31680_e46603_d_n0;
        locals.var_cgsoe_dn2 = assign31680_e46603_d_n2;
        locals.var_cgsoe_dn6 = assign31680_e46603_d_n6;
        locals.var_cgsoe_dn7 = assign31680_e46603_d_n7;
        locals.var_cgsoe_dn10 = assign31680_e46603_d_n10;
        locals.var_cgsoe_dn11 = assign31680_e46603_d_n11;
        locals.var_cgsoe_dn12 = assign31680_e46603_d_n12;
        locals.var_cgsoe_dn17 = assign31680_e46603_d_n17;

        let assign31690_e46606: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1023 = assign31690_e46606;

        let (assign31700_e46622, assign31700_e46622_d_n0, assign31700_e46622_d_n2, assign31700_e46622_d_n6, assign31700_e46622_d_n7, assign31700_e46622_d_n10, assign31700_e46622_d_n11, assign31700_e46622_d_n12, assign31700_e46622_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1023 != 0.0)) {
        let assign31700_e46616: f64 = (locals.var_modenml * locals.var_w_dioscv);
        let assign31700_e46619: f64 = (locals.var_modervs * locals.var_w_diodcv);
        let assign31700_e46620: f64 = (assign31700_e46616 + assign31700_e46619);
        (assign31700_e46620, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign31700_e46622;
        locals.var_t1__blk898_dn0 = assign31700_e46622_d_n0;
        locals.var_t1__blk898_dn2 = assign31700_e46622_d_n2;
        locals.var_t1__blk898_dn6 = assign31700_e46622_d_n6;
        locals.var_t1__blk898_dn7 = assign31700_e46622_d_n7;
        locals.var_t1__blk898_dn10 = assign31700_e46622_d_n10;
        locals.var_t1__blk898_dn11 = assign31700_e46622_d_n11;
        locals.var_t1__blk898_dn12 = assign31700_e46622_d_n12;
        locals.var_t1__blk898_dn17 = assign31700_e46622_d_n17;

        let (assign31710_e46635, assign31710_e46635_d_n0, assign31710_e46635_d_n2, assign31710_e46635_d_n6, assign31710_e46635_d_n7, assign31710_e46635_d_n10, assign31710_e46635_d_n11, assign31710_e46635_d_n12, assign31710_e46635_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1023 != 0.0)) {
        let assign31710_e46632: f64 = (-locals.var_t1__blk898);
        let assign31710_e46633: f64 = (locals.var_cgsoe * assign31710_e46632);
        (assign31710_e46633, ((locals.var_cgsoe_dn0 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn0))), ((locals.var_cgsoe_dn2 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn2))), ((locals.var_cgsoe_dn6 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn6))), ((locals.var_cgsoe_dn7 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn7))), ((locals.var_cgsoe_dn10 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn10))), ((locals.var_cgsoe_dn11 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn11))), ((locals.var_cgsoe_dn12 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn12))), ((locals.var_cgsoe_dn17 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn17))),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31710_e46635;
        locals.var_cgsoe_dn0 = assign31710_e46635_d_n0;
        locals.var_cgsoe_dn2 = assign31710_e46635_d_n2;
        locals.var_cgsoe_dn6 = assign31710_e46635_d_n6;
        locals.var_cgsoe_dn7 = assign31710_e46635_d_n7;
        locals.var_cgsoe_dn10 = assign31710_e46635_d_n10;
        locals.var_cgsoe_dn11 = assign31710_e46635_d_n11;
        locals.var_cgsoe_dn12 = assign31710_e46635_d_n12;
        locals.var_cgsoe_dn17 = assign31710_e46635_d_n17;

        let (assign31720_e46649, assign31720_e46649_d_n0, assign31720_e46649_d_n2, assign31720_e46649_d_n6, assign31720_e46649_d_n7, assign31720_e46649_d_n10, assign31720_e46649_d_n11, assign31720_e46649_d_n12, assign31720_e46649_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1023 == 0.0)) {
        let assign31720_e46646: f64 = (-locals.var_weffcv_nf);
        let assign31720_e46647: f64 = (locals.var_cgsoe * assign31720_e46646);
        (assign31720_e46647, (locals.var_cgsoe_dn0 * assign31720_e46646), (locals.var_cgsoe_dn2 * assign31720_e46646), (locals.var_cgsoe_dn6 * assign31720_e46646), (locals.var_cgsoe_dn7 * assign31720_e46646), (locals.var_cgsoe_dn10 * assign31720_e46646), (locals.var_cgsoe_dn11 * assign31720_e46646), (locals.var_cgsoe_dn12 * assign31720_e46646), (locals.var_cgsoe_dn17 * assign31720_e46646),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31720_e46649;
        locals.var_cgsoe_dn0 = assign31720_e46649_d_n0;
        locals.var_cgsoe_dn2 = assign31720_e46649_d_n2;
        locals.var_cgsoe_dn6 = assign31720_e46649_d_n6;
        locals.var_cgsoe_dn7 = assign31720_e46649_d_n7;
        locals.var_cgsoe_dn10 = assign31720_e46649_d_n10;
        locals.var_cgsoe_dn11 = assign31720_e46649_d_n11;
        locals.var_cgsoe_dn12 = assign31720_e46649_d_n12;
        locals.var_cgsoe_dn17 = assign31720_e46649_d_n17;

        let (assign31730_e46662, assign31730_e46662_d_n0, assign31730_e46662_d_n2, assign31730_e46662_d_n6, assign31730_e46662_d_n7, assign31730_e46662_d_n10, assign31730_e46662_d_n11, assign31730_e46662_d_n12, assign31730_e46662_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31730_e46657: f64 = (-locals.var_cgsoe);
        let assign31730_e46659: f64 = (assign31730_e46657 * locals.var_vgs);
        let assign31730_e46660: f64 = (locals.var_qgos + assign31730_e46659);
        (assign31730_e46660, (locals.var_qgos_dn0 + ((-locals.var_cgsoe_dn0) * locals.var_vgs)), (locals.var_qgos_dn2 + ((-locals.var_cgsoe_dn2) * locals.var_vgs)), (locals.var_qgos_dn6 + (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31730_e46657 * locals.var_vgs_dn6))), (locals.var_qgos_dn7 + (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31730_e46657 * locals.var_vgs_dn7))), (locals.var_qgos_dn10 + ((-locals.var_cgsoe_dn10) * locals.var_vgs)), (locals.var_qgos_dn11 + (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31730_e46657 * locals.var_vgs_dn11))), (locals.var_qgos_dn12 + ((-locals.var_cgsoe_dn12) * locals.var_vgs)), (locals.var_qgos_dn17 + ((-locals.var_cgsoe_dn17) * locals.var_vgs)),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign31730_e46662;
        locals.var_qgos_dn0 = assign31730_e46662_d_n0;
        locals.var_qgos_dn2 = assign31730_e46662_d_n2;
        locals.var_qgos_dn6 = assign31730_e46662_d_n6;
        locals.var_qgos_dn7 = assign31730_e46662_d_n7;
        locals.var_qgos_dn10 = assign31730_e46662_d_n10;
        locals.var_qgos_dn11 = assign31730_e46662_d_n11;
        locals.var_qgos_dn12 = assign31730_e46662_d_n12;
        locals.var_qgos_dn17 = assign31730_e46662_d_n17;

        let assign31740_e46675: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgdo_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgso_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1024 = assign31740_e46675;

        let assign31750_e46678: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign31750_e46678;

    }

    pub(super) fn stamp_transient_block_113(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31760_e46694, assign31760_e46694_d_n0, assign31760_e46694_d_n2, assign31760_e46694_d_n6, assign31760_e46694_d_n7, assign31760_e46694_d_n10, assign31760_e46694_d_n11, assign31760_e46694_d_n12, assign31760_e46694_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 != 0.0)) && (locals.var_guard1025 != 0.0)) {
        let assign31760_e46688: f64 = (-locals.var_cox0__blk908);
        let assign31760_e46690: f64 = (assign31760_e46688 * p.p188);
        let assign31760_e46692: f64 = (assign31760_e46690 * locals.var_w_diodcv);
        (assign31760_e46692, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31760_e46694;
        locals.var_cgdoe_dn0 = assign31760_e46694_d_n0;
        locals.var_cgdoe_dn2 = assign31760_e46694_d_n2;
        locals.var_cgdoe_dn6 = assign31760_e46694_d_n6;
        locals.var_cgdoe_dn7 = assign31760_e46694_d_n7;
        locals.var_cgdoe_dn10 = assign31760_e46694_d_n10;
        locals.var_cgdoe_dn11 = assign31760_e46694_d_n11;
        locals.var_cgdoe_dn12 = assign31760_e46694_d_n12;
        locals.var_cgdoe_dn17 = assign31760_e46694_d_n17;

        let (assign31770_e46711, assign31770_e46711_d_n0, assign31770_e46711_d_n2, assign31770_e46711_d_n6, assign31770_e46711_d_n7, assign31770_e46711_d_n10, assign31770_e46711_d_n11, assign31770_e46711_d_n12, assign31770_e46711_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 != 0.0)) && (locals.var_guard1025 == 0.0)) {
        let assign31770_e46705: f64 = (-locals.var_cox0__blk908);
        let assign31770_e46707: f64 = (assign31770_e46705 * p.p188);
        let assign31770_e46709: f64 = (assign31770_e46707 * locals.var_weffcv_nf);
        (assign31770_e46709, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31770_e46711;
        locals.var_cgdoe_dn0 = assign31770_e46711_d_n0;
        locals.var_cgdoe_dn2 = assign31770_e46711_d_n2;
        locals.var_cgdoe_dn6 = assign31770_e46711_d_n6;
        locals.var_cgdoe_dn7 = assign31770_e46711_d_n7;
        locals.var_cgdoe_dn10 = assign31770_e46711_d_n10;
        locals.var_cgdoe_dn11 = assign31770_e46711_d_n11;
        locals.var_cgdoe_dn12 = assign31770_e46711_d_n12;
        locals.var_cgdoe_dn17 = assign31770_e46711_d_n17;

        let (assign31780_e46727, assign31780_e46727_d_n0, assign31780_e46727_d_n2, assign31780_e46727_d_n6, assign31780_e46727_d_n7, assign31780_e46727_d_n10, assign31780_e46727_d_n11, assign31780_e46727_d_n12, assign31780_e46727_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 == 0.0)) {
        let assign31780_e46721: f64 = (locals.var_modervs * p.p170);
        let assign31780_e46724: f64 = (locals.var_modenml * p.p169);
        let assign31780_e46725: f64 = (assign31780_e46721 + assign31780_e46724);
        (assign31780_e46725, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31780_e46727;
        locals.var_cgdoe_dn0 = assign31780_e46727_d_n0;
        locals.var_cgdoe_dn2 = assign31780_e46727_d_n2;
        locals.var_cgdoe_dn6 = assign31780_e46727_d_n6;
        locals.var_cgdoe_dn7 = assign31780_e46727_d_n7;
        locals.var_cgdoe_dn10 = assign31780_e46727_d_n10;
        locals.var_cgdoe_dn11 = assign31780_e46727_d_n11;
        locals.var_cgdoe_dn12 = assign31780_e46727_d_n12;
        locals.var_cgdoe_dn17 = assign31780_e46727_d_n17;

        let assign31790_e46730: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign31790_e46730;

        let (assign31800_e46748, assign31800_e46748_d_n0, assign31800_e46748_d_n2, assign31800_e46748_d_n6, assign31800_e46748_d_n7, assign31800_e46748_d_n10, assign31800_e46748_d_n11, assign31800_e46748_d_n12, assign31800_e46748_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 == 0.0)) && (locals.var_guard1026 != 0.0)) {
        let assign31800_e46742: f64 = (locals.var_modervs * locals.var_w_dioscv);
        let assign31800_e46745: f64 = (locals.var_modenml * locals.var_w_diodcv);
        let assign31800_e46746: f64 = (assign31800_e46742 + assign31800_e46745);
        (assign31800_e46746, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign31800_e46748;
        locals.var_t1__blk898_dn0 = assign31800_e46748_d_n0;
        locals.var_t1__blk898_dn2 = assign31800_e46748_d_n2;
        locals.var_t1__blk898_dn6 = assign31800_e46748_d_n6;
        locals.var_t1__blk898_dn7 = assign31800_e46748_d_n7;
        locals.var_t1__blk898_dn10 = assign31800_e46748_d_n10;
        locals.var_t1__blk898_dn11 = assign31800_e46748_d_n11;
        locals.var_t1__blk898_dn12 = assign31800_e46748_d_n12;
        locals.var_t1__blk898_dn17 = assign31800_e46748_d_n17;

        let (assign31810_e46763, assign31810_e46763_d_n0, assign31810_e46763_d_n2, assign31810_e46763_d_n6, assign31810_e46763_d_n7, assign31810_e46763_d_n10, assign31810_e46763_d_n11, assign31810_e46763_d_n12, assign31810_e46763_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 == 0.0)) && (locals.var_guard1026 != 0.0)) {
        let assign31810_e46760: f64 = (-locals.var_t1__blk898);
        let assign31810_e46761: f64 = (locals.var_cgdoe * assign31810_e46760);
        (assign31810_e46761, ((locals.var_cgdoe_dn0 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn0))), ((locals.var_cgdoe_dn2 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn2))), ((locals.var_cgdoe_dn6 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn6))), ((locals.var_cgdoe_dn7 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn7))), ((locals.var_cgdoe_dn10 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn10))), ((locals.var_cgdoe_dn11 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn11))), ((locals.var_cgdoe_dn12 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn12))), ((locals.var_cgdoe_dn17 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn17))),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31810_e46763;
        locals.var_cgdoe_dn0 = assign31810_e46763_d_n0;
        locals.var_cgdoe_dn2 = assign31810_e46763_d_n2;
        locals.var_cgdoe_dn6 = assign31810_e46763_d_n6;
        locals.var_cgdoe_dn7 = assign31810_e46763_d_n7;
        locals.var_cgdoe_dn10 = assign31810_e46763_d_n10;
        locals.var_cgdoe_dn11 = assign31810_e46763_d_n11;
        locals.var_cgdoe_dn12 = assign31810_e46763_d_n12;
        locals.var_cgdoe_dn17 = assign31810_e46763_d_n17;

        let (assign31820_e46779, assign31820_e46779_d_n0, assign31820_e46779_d_n2, assign31820_e46779_d_n6, assign31820_e46779_d_n7, assign31820_e46779_d_n10, assign31820_e46779_d_n11, assign31820_e46779_d_n12, assign31820_e46779_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 == 0.0)) && (locals.var_guard1026 == 0.0)) {
        let assign31820_e46776: f64 = (-locals.var_weffcv_nf);
        let assign31820_e46777: f64 = (locals.var_cgdoe * assign31820_e46776);
        (assign31820_e46777, (locals.var_cgdoe_dn0 * assign31820_e46776), (locals.var_cgdoe_dn2 * assign31820_e46776), (locals.var_cgdoe_dn6 * assign31820_e46776), (locals.var_cgdoe_dn7 * assign31820_e46776), (locals.var_cgdoe_dn10 * assign31820_e46776), (locals.var_cgdoe_dn11 * assign31820_e46776), (locals.var_cgdoe_dn12 * assign31820_e46776), (locals.var_cgdoe_dn17 * assign31820_e46776),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31820_e46779;
        locals.var_cgdoe_dn0 = assign31820_e46779_d_n0;
        locals.var_cgdoe_dn2 = assign31820_e46779_d_n2;
        locals.var_cgdoe_dn6 = assign31820_e46779_d_n6;
        locals.var_cgdoe_dn7 = assign31820_e46779_d_n7;
        locals.var_cgdoe_dn10 = assign31820_e46779_d_n10;
        locals.var_cgdoe_dn11 = assign31820_e46779_d_n11;
        locals.var_cgdoe_dn12 = assign31820_e46779_d_n12;
        locals.var_cgdoe_dn17 = assign31820_e46779_d_n17;

        let (assign31830_e46791, assign31830_e46791_d_n0, assign31830_e46791_d_n2, assign31830_e46791_d_n6, assign31830_e46791_d_n7, assign31830_e46791_d_n10, assign31830_e46791_d_n11, assign31830_e46791_d_n12, assign31830_e46791_d_n17,) = {
    if ((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) {
        let assign31830_e46785: f64 = (-locals.var_cgdoe);
        let assign31830_e46788: f64 = (locals.var_vgs - locals.var_vds);
        let assign31830_e46789: f64 = (assign31830_e46785 * assign31830_e46788);
        (assign31830_e46789, (((-locals.var_cgdoe_dn0) * assign31830_e46788) + (assign31830_e46785 * (-locals.var_vds_dn0))), (((-locals.var_cgdoe_dn2) * assign31830_e46788) + (assign31830_e46785 * (-locals.var_vds_dn2))), (((-locals.var_cgdoe_dn6) * assign31830_e46788) + (assign31830_e46785 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((-locals.var_cgdoe_dn7) * assign31830_e46788) + (assign31830_e46785 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((-locals.var_cgdoe_dn10) * assign31830_e46788) + (assign31830_e46785 * (-locals.var_vds_dn10))), (((-locals.var_cgdoe_dn11) * assign31830_e46788) + (assign31830_e46785 * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (((-locals.var_cgdoe_dn12) * assign31830_e46788) + (assign31830_e46785 * (-locals.var_vds_dn12))), (((-locals.var_cgdoe_dn17) * assign31830_e46788) + (assign31830_e46785 * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign31830_e46791;
        locals.var_qgod_dn0 = assign31830_e46791_d_n0;
        locals.var_qgod_dn2 = assign31830_e46791_d_n2;
        locals.var_qgod_dn6 = assign31830_e46791_d_n6;
        locals.var_qgod_dn7 = assign31830_e46791_d_n7;
        locals.var_qgod_dn10 = assign31830_e46791_d_n10;
        locals.var_qgod_dn11 = assign31830_e46791_d_n11;
        locals.var_qgod_dn12 = assign31830_e46791_d_n12;
        locals.var_qgod_dn17 = assign31830_e46791_d_n17;

        let assign31840_e46804: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgso_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1027 = assign31840_e46804;

        let assign31850_e46807: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1028 = assign31850_e46807;

        let (assign31860_e46823, assign31860_e46823_d_n0, assign31860_e46823_d_n2, assign31860_e46823_d_n6, assign31860_e46823_d_n7, assign31860_e46823_d_n10, assign31860_e46823_d_n11, assign31860_e46823_d_n12, assign31860_e46823_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 != 0.0)) && (locals.var_guard1028 != 0.0)) {
        let assign31860_e46817: f64 = (-locals.var_cox0__blk908);
        let assign31860_e46819: f64 = (assign31860_e46817 * p.p188);
        let assign31860_e46821: f64 = (assign31860_e46819 * locals.var_w_dioscv);
        (assign31860_e46821, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31860_e46823;
        locals.var_cgsoe_dn0 = assign31860_e46823_d_n0;
        locals.var_cgsoe_dn2 = assign31860_e46823_d_n2;
        locals.var_cgsoe_dn6 = assign31860_e46823_d_n6;
        locals.var_cgsoe_dn7 = assign31860_e46823_d_n7;
        locals.var_cgsoe_dn10 = assign31860_e46823_d_n10;
        locals.var_cgsoe_dn11 = assign31860_e46823_d_n11;
        locals.var_cgsoe_dn12 = assign31860_e46823_d_n12;
        locals.var_cgsoe_dn17 = assign31860_e46823_d_n17;

        let (assign31870_e46840, assign31870_e46840_d_n0, assign31870_e46840_d_n2, assign31870_e46840_d_n6, assign31870_e46840_d_n7, assign31870_e46840_d_n10, assign31870_e46840_d_n11, assign31870_e46840_d_n12, assign31870_e46840_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 != 0.0)) && (locals.var_guard1028 == 0.0)) {
        let assign31870_e46834: f64 = (-locals.var_cox0__blk908);
        let assign31870_e46836: f64 = (assign31870_e46834 * p.p188);
        let assign31870_e46838: f64 = (assign31870_e46836 * locals.var_weffcv_nf);
        (assign31870_e46838, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31870_e46840;
        locals.var_cgsoe_dn0 = assign31870_e46840_d_n0;
        locals.var_cgsoe_dn2 = assign31870_e46840_d_n2;
        locals.var_cgsoe_dn6 = assign31870_e46840_d_n6;
        locals.var_cgsoe_dn7 = assign31870_e46840_d_n7;
        locals.var_cgsoe_dn10 = assign31870_e46840_d_n10;
        locals.var_cgsoe_dn11 = assign31870_e46840_d_n11;
        locals.var_cgsoe_dn12 = assign31870_e46840_d_n12;
        locals.var_cgsoe_dn17 = assign31870_e46840_d_n17;

        let (assign31880_e46856, assign31880_e46856_d_n0, assign31880_e46856_d_n2, assign31880_e46856_d_n6, assign31880_e46856_d_n7, assign31880_e46856_d_n10, assign31880_e46856_d_n11, assign31880_e46856_d_n12, assign31880_e46856_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 == 0.0)) {
        let assign31880_e46850: f64 = (locals.var_modenml * p.p170);
        let assign31880_e46853: f64 = (locals.var_modervs * p.p169);
        let assign31880_e46854: f64 = (assign31880_e46850 + assign31880_e46853);
        (assign31880_e46854, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31880_e46856;
        locals.var_cgsoe_dn0 = assign31880_e46856_d_n0;
        locals.var_cgsoe_dn2 = assign31880_e46856_d_n2;
        locals.var_cgsoe_dn6 = assign31880_e46856_d_n6;
        locals.var_cgsoe_dn7 = assign31880_e46856_d_n7;
        locals.var_cgsoe_dn10 = assign31880_e46856_d_n10;
        locals.var_cgsoe_dn11 = assign31880_e46856_d_n11;
        locals.var_cgsoe_dn12 = assign31880_e46856_d_n12;
        locals.var_cgsoe_dn17 = assign31880_e46856_d_n17;

        let assign31890_e46859: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1029 = assign31890_e46859;

        let (assign31900_e46877, assign31900_e46877_d_n0, assign31900_e46877_d_n2, assign31900_e46877_d_n6, assign31900_e46877_d_n7, assign31900_e46877_d_n10, assign31900_e46877_d_n11, assign31900_e46877_d_n12, assign31900_e46877_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 == 0.0)) && (locals.var_guard1029 != 0.0)) {
        let assign31900_e46871: f64 = (locals.var_modenml * locals.var_w_dioscv);
        let assign31900_e46874: f64 = (locals.var_modervs * locals.var_w_diodcv);
        let assign31900_e46875: f64 = (assign31900_e46871 + assign31900_e46874);
        (assign31900_e46875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign31900_e46877;
        locals.var_t1__blk898_dn0 = assign31900_e46877_d_n0;
        locals.var_t1__blk898_dn2 = assign31900_e46877_d_n2;
        locals.var_t1__blk898_dn6 = assign31900_e46877_d_n6;
        locals.var_t1__blk898_dn7 = assign31900_e46877_d_n7;
        locals.var_t1__blk898_dn10 = assign31900_e46877_d_n10;
        locals.var_t1__blk898_dn11 = assign31900_e46877_d_n11;
        locals.var_t1__blk898_dn12 = assign31900_e46877_d_n12;
        locals.var_t1__blk898_dn17 = assign31900_e46877_d_n17;

        let (assign31910_e46892, assign31910_e46892_d_n0, assign31910_e46892_d_n2, assign31910_e46892_d_n6, assign31910_e46892_d_n7, assign31910_e46892_d_n10, assign31910_e46892_d_n11, assign31910_e46892_d_n12, assign31910_e46892_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 == 0.0)) && (locals.var_guard1029 != 0.0)) {
        let assign31910_e46889: f64 = (-locals.var_t1__blk898);
        let assign31910_e46890: f64 = (locals.var_cgsoe * assign31910_e46889);
        (assign31910_e46890, ((locals.var_cgsoe_dn0 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn0))), ((locals.var_cgsoe_dn2 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn2))), ((locals.var_cgsoe_dn6 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn6))), ((locals.var_cgsoe_dn7 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn7))), ((locals.var_cgsoe_dn10 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn10))), ((locals.var_cgsoe_dn11 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn11))), ((locals.var_cgsoe_dn12 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn12))), ((locals.var_cgsoe_dn17 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn17))),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31910_e46892;
        locals.var_cgsoe_dn0 = assign31910_e46892_d_n0;
        locals.var_cgsoe_dn2 = assign31910_e46892_d_n2;
        locals.var_cgsoe_dn6 = assign31910_e46892_d_n6;
        locals.var_cgsoe_dn7 = assign31910_e46892_d_n7;
        locals.var_cgsoe_dn10 = assign31910_e46892_d_n10;
        locals.var_cgsoe_dn11 = assign31910_e46892_d_n11;
        locals.var_cgsoe_dn12 = assign31910_e46892_d_n12;
        locals.var_cgsoe_dn17 = assign31910_e46892_d_n17;

        let (assign31920_e46908, assign31920_e46908_d_n0, assign31920_e46908_d_n2, assign31920_e46908_d_n6, assign31920_e46908_d_n7, assign31920_e46908_d_n10, assign31920_e46908_d_n11, assign31920_e46908_d_n12, assign31920_e46908_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 == 0.0)) && (locals.var_guard1029 == 0.0)) {
        let assign31920_e46905: f64 = (-locals.var_weffcv_nf);
        let assign31920_e46906: f64 = (locals.var_cgsoe * assign31920_e46905);
        (assign31920_e46906, (locals.var_cgsoe_dn0 * assign31920_e46905), (locals.var_cgsoe_dn2 * assign31920_e46905), (locals.var_cgsoe_dn6 * assign31920_e46905), (locals.var_cgsoe_dn7 * assign31920_e46905), (locals.var_cgsoe_dn10 * assign31920_e46905), (locals.var_cgsoe_dn11 * assign31920_e46905), (locals.var_cgsoe_dn12 * assign31920_e46905), (locals.var_cgsoe_dn17 * assign31920_e46905),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31920_e46908;
        locals.var_cgsoe_dn0 = assign31920_e46908_d_n0;
        locals.var_cgsoe_dn2 = assign31920_e46908_d_n2;
        locals.var_cgsoe_dn6 = assign31920_e46908_d_n6;
        locals.var_cgsoe_dn7 = assign31920_e46908_d_n7;
        locals.var_cgsoe_dn10 = assign31920_e46908_d_n10;
        locals.var_cgsoe_dn11 = assign31920_e46908_d_n11;
        locals.var_cgsoe_dn12 = assign31920_e46908_d_n12;
        locals.var_cgsoe_dn17 = assign31920_e46908_d_n17;

        let (assign31930_e46918, assign31930_e46918_d_n0, assign31930_e46918_d_n2, assign31930_e46918_d_n6, assign31930_e46918_d_n7, assign31930_e46918_d_n10, assign31930_e46918_d_n11, assign31930_e46918_d_n12, assign31930_e46918_d_n17,) = {
    if ((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) {
        let assign31930_e46914: f64 = (-locals.var_cgsoe);
        let assign31930_e46916: f64 = (assign31930_e46914 * locals.var_vgs);
        (assign31930_e46916, ((-locals.var_cgsoe_dn0) * locals.var_vgs), ((-locals.var_cgsoe_dn2) * locals.var_vgs), (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31930_e46914 * locals.var_vgs_dn6)), (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31930_e46914 * locals.var_vgs_dn7)), ((-locals.var_cgsoe_dn10) * locals.var_vgs), (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31930_e46914 * locals.var_vgs_dn11)), ((-locals.var_cgsoe_dn12) * locals.var_vgs), ((-locals.var_cgsoe_dn17) * locals.var_vgs),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign31930_e46918;
        locals.var_qgos_dn0 = assign31930_e46918_d_n0;
        locals.var_qgos_dn2 = assign31930_e46918_d_n2;
        locals.var_qgos_dn6 = assign31930_e46918_d_n6;
        locals.var_qgos_dn7 = assign31930_e46918_d_n7;
        locals.var_qgos_dn10 = assign31930_e46918_d_n10;
        locals.var_qgos_dn11 = assign31930_e46918_d_n11;
        locals.var_qgos_dn12 = assign31930_e46918_d_n12;
        locals.var_qgos_dn17 = assign31930_e46918_d_n17;

        let assign31940_e46921: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1030 = assign31940_e46921;

        let (assign31950_e46925, assign31950_e46925_d_n6, assign31950_e46925_d_n12,) = {
    if (locals.var_guard1030 != 0.0) {
        (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12,)
    } else {
        (locals.var_vbdj, locals.var_vbdj_dn6, locals.var_vbdj_dn12,)
    }
};
        locals.var_vbdj = assign31950_e46925;
        locals.var_vbdj_dn6 = assign31950_e46925_d_n6;
        locals.var_vbdj_dn12 = assign31950_e46925_d_n12;

        let (assign31960_e46929, assign31960_e46929_d_n7, assign31960_e46929_d_n12,) = {
    if (locals.var_guard1030 != 0.0) {
        (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12,)
    } else {
        (locals.var_vbsj, locals.var_vbsj_dn7, locals.var_vbsj_dn12,)
    }
};
        locals.var_vbsj = assign31960_e46929;
        locals.var_vbsj_dn7 = assign31960_e46929_d_n7;
        locals.var_vbsj_dn12 = assign31960_e46929_d_n12;

        let (assign31970_e46951, assign31970_e46951_d_n0, assign31970_e46951_d_n2, assign31970_e46951_d_n6, assign31970_e46951_d_n7, assign31970_e46951_d_n10, assign31970_e46951_d_n11, assign31970_e46951_d_n12, assign31970_e46951_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign31970_e46934: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign31970_e46937: f64 = (locals.var_eg * locals.var_beta);
        let assign31970_e46938: f64 = (assign31970_e46934 - assign31970_e46937);
        let assign31970_e46942: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign31970_e46943: f64 = (assign31970_e46942).ln();
        let assign31970_e46944: f64 = (p.p175 * assign31970_e46943);
        let assign31970_e46945: f64 = (assign31970_e46938 + assign31970_e46944);
        let assign31970_e46947: f64 = (assign31970_e46945 / p.p174);
        let assign31970_e46948: f64 = (assign31970_e46947).exp();
        let assign31970_e46949: f64 = (p.p173 * assign31970_e46948);
        (assign31970_e46949, (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p175 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31970_e46942))) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn12, locals.var_js_dn17,)
    }
};
        locals.var_js = assign31970_e46951;
        locals.var_js_dn0 = assign31970_e46951_d_n0;
        locals.var_js_dn2 = assign31970_e46951_d_n2;
        locals.var_js_dn6 = assign31970_e46951_d_n6;
        locals.var_js_dn7 = assign31970_e46951_d_n7;
        locals.var_js_dn10 = assign31970_e46951_d_n10;
        locals.var_js_dn11 = assign31970_e46951_d_n11;
        locals.var_js_dn12 = assign31970_e46951_d_n12;
        locals.var_js_dn17 = assign31970_e46951_d_n17;

        let (assign31980_e46973, assign31980_e46973_d_n0, assign31980_e46973_d_n2, assign31980_e46973_d_n6, assign31980_e46973_d_n7, assign31980_e46973_d_n10, assign31980_e46973_d_n11, assign31980_e46973_d_n12, assign31980_e46973_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign31980_e46956: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign31980_e46959: f64 = (locals.var_eg * locals.var_beta);
        let assign31980_e46960: f64 = (assign31980_e46956 - assign31980_e46959);
        let assign31980_e46964: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign31980_e46965: f64 = (assign31980_e46964).ln();
        let assign31980_e46966: f64 = (p.p176 * assign31980_e46965);
        let assign31980_e46967: f64 = (assign31980_e46960 + assign31980_e46966);
        let assign31980_e46969: f64 = (assign31980_e46967 / p.p174);
        let assign31980_e46970: f64 = (assign31980_e46969).exp();
        let assign31980_e46971: f64 = (p.p173 * assign31980_e46970);
        (assign31980_e46971, (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p176 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31980_e46964))) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn12, locals.var_js2_dn17,)
    }
};
        locals.var_js2 = assign31980_e46973;
        locals.var_js2_dn0 = assign31980_e46973_d_n0;
        locals.var_js2_dn2 = assign31980_e46973_d_n2;
        locals.var_js2_dn6 = assign31980_e46973_d_n6;
        locals.var_js2_dn7 = assign31980_e46973_d_n7;
        locals.var_js2_dn10 = assign31980_e46973_d_n10;
        locals.var_js2_dn11 = assign31980_e46973_d_n11;
        locals.var_js2_dn12 = assign31980_e46973_d_n12;
        locals.var_js2_dn17 = assign31980_e46973_d_n17;

        let (assign31990_e46981, assign31990_e46981_d_n0, assign31990_e46981_d_n2, assign31990_e46981_d_n6, assign31990_e46981_d_n7, assign31990_e46981_d_n10, assign31990_e46981_d_n11, assign31990_e46981_d_n12, assign31990_e46981_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign31990_e46977: f64 = (locals.var_w_diod * p.p237);
        let assign31990_e46979: f64 = (assign31990_e46977 * locals.var_js);
        (assign31990_e46979, (assign31990_e46977 * locals.var_js_dn0), (assign31990_e46977 * locals.var_js_dn2), (assign31990_e46977 * locals.var_js_dn6), (assign31990_e46977 * locals.var_js_dn7), (assign31990_e46977 * locals.var_js_dn10), (assign31990_e46977 * locals.var_js_dn11), (assign31990_e46977 * locals.var_js_dn12), (assign31990_e46977 * locals.var_js_dn17),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17,)
    }
};
        locals.var_isbd = assign31990_e46981;
        locals.var_isbd_dn0 = assign31990_e46981_d_n0;
        locals.var_isbd_dn2 = assign31990_e46981_d_n2;
        locals.var_isbd_dn6 = assign31990_e46981_d_n6;
        locals.var_isbd_dn7 = assign31990_e46981_d_n7;
        locals.var_isbd_dn10 = assign31990_e46981_d_n10;
        locals.var_isbd_dn11 = assign31990_e46981_d_n11;
        locals.var_isbd_dn12 = assign31990_e46981_d_n12;
        locals.var_isbd_dn17 = assign31990_e46981_d_n17;

        let (assign32000_e46989, assign32000_e46989_d_n0, assign32000_e46989_d_n2, assign32000_e46989_d_n6, assign32000_e46989_d_n7, assign32000_e46989_d_n10, assign32000_e46989_d_n11, assign32000_e46989_d_n12, assign32000_e46989_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32000_e46985: f64 = (locals.var_w_diod * p.p237);
        let assign32000_e46987: f64 = (assign32000_e46985 * locals.var_js2);
        (assign32000_e46987, (assign32000_e46985 * locals.var_js2_dn0), (assign32000_e46985 * locals.var_js2_dn2), (assign32000_e46985 * locals.var_js2_dn6), (assign32000_e46985 * locals.var_js2_dn7), (assign32000_e46985 * locals.var_js2_dn10), (assign32000_e46985 * locals.var_js2_dn11), (assign32000_e46985 * locals.var_js2_dn12), (assign32000_e46985 * locals.var_js2_dn17),)
    } else {
        (locals.var_isbd2, locals.var_isbd2_dn0, locals.var_isbd2_dn2, locals.var_isbd2_dn6, locals.var_isbd2_dn7, locals.var_isbd2_dn10, locals.var_isbd2_dn11, locals.var_isbd2_dn12, locals.var_isbd2_dn17,)
    }
};
        locals.var_isbd2 = assign32000_e46989;
        locals.var_isbd2_dn0 = assign32000_e46989_d_n0;
        locals.var_isbd2_dn2 = assign32000_e46989_d_n2;
        locals.var_isbd2_dn6 = assign32000_e46989_d_n6;
        locals.var_isbd2_dn7 = assign32000_e46989_d_n7;
        locals.var_isbd2_dn10 = assign32000_e46989_d_n10;
        locals.var_isbd2_dn11 = assign32000_e46989_d_n11;
        locals.var_isbd2_dn12 = assign32000_e46989_d_n12;
        locals.var_isbd2_dn17 = assign32000_e46989_d_n17;

        let (assign32010_e46997, assign32010_e46997_d_n0, assign32010_e46997_d_n2, assign32010_e46997_d_n6, assign32010_e46997_d_n7, assign32010_e46997_d_n10, assign32010_e46997_d_n11, assign32010_e46997_d_n12, assign32010_e46997_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32010_e46993: f64 = (locals.var_w_dios * p.p237);
        let assign32010_e46995: f64 = (assign32010_e46993 * locals.var_js);
        (assign32010_e46995, (assign32010_e46993 * locals.var_js_dn0), (assign32010_e46993 * locals.var_js_dn2), (assign32010_e46993 * locals.var_js_dn6), (assign32010_e46993 * locals.var_js_dn7), (assign32010_e46993 * locals.var_js_dn10), (assign32010_e46993 * locals.var_js_dn11), (assign32010_e46993 * locals.var_js_dn12), (assign32010_e46993 * locals.var_js_dn17),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn12, locals.var_isbs_dn17,)
    }
};
        locals.var_isbs = assign32010_e46997;
        locals.var_isbs_dn0 = assign32010_e46997_d_n0;
        locals.var_isbs_dn2 = assign32010_e46997_d_n2;
        locals.var_isbs_dn6 = assign32010_e46997_d_n6;
        locals.var_isbs_dn7 = assign32010_e46997_d_n7;
        locals.var_isbs_dn10 = assign32010_e46997_d_n10;
        locals.var_isbs_dn11 = assign32010_e46997_d_n11;
        locals.var_isbs_dn12 = assign32010_e46997_d_n12;
        locals.var_isbs_dn17 = assign32010_e46997_d_n17;

        let (assign32020_e47005, assign32020_e47005_d_n0, assign32020_e47005_d_n2, assign32020_e47005_d_n6, assign32020_e47005_d_n7, assign32020_e47005_d_n10, assign32020_e47005_d_n11, assign32020_e47005_d_n12, assign32020_e47005_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32020_e47001: f64 = (locals.var_w_dios * p.p237);
        let assign32020_e47003: f64 = (assign32020_e47001 * locals.var_js2);
        (assign32020_e47003, (assign32020_e47001 * locals.var_js2_dn0), (assign32020_e47001 * locals.var_js2_dn2), (assign32020_e47001 * locals.var_js2_dn6), (assign32020_e47001 * locals.var_js2_dn7), (assign32020_e47001 * locals.var_js2_dn10), (assign32020_e47001 * locals.var_js2_dn11), (assign32020_e47001 * locals.var_js2_dn12), (assign32020_e47001 * locals.var_js2_dn17),)
    } else {
        (locals.var_isbs2, locals.var_isbs2_dn0, locals.var_isbs2_dn2, locals.var_isbs2_dn6, locals.var_isbs2_dn7, locals.var_isbs2_dn10, locals.var_isbs2_dn11, locals.var_isbs2_dn12, locals.var_isbs2_dn17,)
    }
};
        locals.var_isbs2 = assign32020_e47005;
        locals.var_isbs2_dn0 = assign32020_e47005_d_n0;
        locals.var_isbs2_dn2 = assign32020_e47005_d_n2;
        locals.var_isbs2_dn6 = assign32020_e47005_d_n6;
        locals.var_isbs2_dn7 = assign32020_e47005_d_n7;
        locals.var_isbs2_dn10 = assign32020_e47005_d_n10;
        locals.var_isbs2_dn11 = assign32020_e47005_d_n11;
        locals.var_isbs2_dn12 = assign32020_e47005_d_n12;
        locals.var_isbs2_dn17 = assign32020_e47005_d_n17;

        let (assign32030_e47011, assign32030_e47011_d_n6, assign32030_e47011_d_n7, assign32030_e47011_d_n10, assign32030_e47011_d_n12,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32030_e47009: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign32030_e47009, 0.0, 0.0, (locals.var_ttemp_dn10 / locals.var_uc_tnom), 0.0,)
    } else {
        (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12,)
    }
};
        locals.var_t1__blk1032 = assign32030_e47011;
        locals.var_t1__blk1032_dn6 = assign32030_e47011_d_n6;
        locals.var_t1__blk1032_dn7 = assign32030_e47011_d_n7;
        locals.var_t1__blk1032_dn10 = assign32030_e47011_d_n10;
        locals.var_t1__blk1032_dn12 = assign32030_e47011_d_n12;

        let (assign32050_e47023, assign32050_e47023_d_n0, assign32050_e47023_d_n2, assign32050_e47023_d_n6, assign32050_e47023_d_n7, assign32050_e47023_d_n10, assign32050_e47023_d_n11, assign32050_e47023_d_n12, assign32050_e47023_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32050_e47021: f64 = (locals.var_isbd + 1e-50);
        (assign32050_e47021, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17,)
    } else {
        (locals.var_t2__blk1033, locals.var_t2__blk1033_dn0, locals.var_t2__blk1033_dn2, locals.var_t2__blk1033_dn6, locals.var_t2__blk1033_dn7, locals.var_t2__blk1033_dn10, locals.var_t2__blk1033_dn11, locals.var_t2__blk1033_dn12, locals.var_t2__blk1033_dn17,)
    }
};
        locals.var_t2__blk1033 = assign32050_e47023;
        locals.var_t2__blk1033_dn0 = assign32050_e47023_d_n0;
        locals.var_t2__blk1033_dn2 = assign32050_e47023_d_n2;
        locals.var_t2__blk1033_dn6 = assign32050_e47023_d_n6;
        locals.var_t2__blk1033_dn7 = assign32050_e47023_d_n7;
        locals.var_t2__blk1033_dn10 = assign32050_e47023_d_n10;
        locals.var_t2__blk1033_dn11 = assign32050_e47023_d_n11;
        locals.var_t2__blk1033_dn12 = assign32050_e47023_d_n12;
        locals.var_t2__blk1033_dn17 = assign32050_e47023_d_n17;

        let (assign32070_e47037, assign32070_e47037_d_n10,) = {
    if (locals.var_guard1030 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn10,)
    }
};
        locals.var_vbdt = assign32070_e47037;
        locals.var_vbdt_dn10 = assign32070_e47037_d_n10;

        let (assign32080_e47045, assign32080_e47045_d_n10,) = {
    if (locals.var_guard1030 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vbst, locals.var_vbst_dn10,)
    }
};
        locals.var_vbst = assign32080_e47045;
        locals.var_vbst_dn10 = assign32080_e47045_d_n10;

        let (assign32090_e47051, assign32090_e47051_d_n10,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32090_e47049: f64 = (p.p174 * locals.var_beta_inv);
        (assign32090_e47049, (p.p174 * locals.var_beta_inv_dn10),)
    } else {
        (locals.var_nvtm, locals.var_nvtm_dn10,)
    }
};
        locals.var_nvtm = assign32090_e47051;
        locals.var_nvtm_dn10 = assign32090_e47051_d_n10;

        let assign32100_e47054: f64 = if locals.var_vbdj < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard1059 = assign32100_e47054;

    }

    pub(super) fn stamp_transient_block_114(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32110_e47063, assign32110_e47063_d_n6, assign32110_e47063_d_n7, assign32110_e47063_d_n10, assign32110_e47063_d_n12,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1059 != 0.0)) {
        let assign32110_e47060: f64 = (locals.var_vbdj / locals.var_nvtm);
        let assign32110_e47061: f64 = (assign32110_e47060).exp();
        (assign32110_e47061, (assign32110_e47061 * (locals.var_vbdj_dn6 / locals.var_nvtm)), 0.0, (assign32110_e47061 * (-((locals.var_vbdj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32110_e47061 * (locals.var_vbdj_dn12 / locals.var_nvtm)),)
    } else {
        (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12,)
    }
};
        locals.var_t1__blk1032 = assign32110_e47063;
        locals.var_t1__blk1032_dn6 = assign32110_e47063_d_n6;
        locals.var_t1__blk1032_dn7 = assign32110_e47063_d_n7;
        locals.var_t1__blk1032_dn10 = assign32110_e47063_d_n10;
        locals.var_t1__blk1032_dn12 = assign32110_e47063_d_n12;

        let (assign32120_e47073, assign32120_e47073_d_n0, assign32120_e47073_d_n2, assign32120_e47073_d_n6, assign32120_e47073_d_n7, assign32120_e47073_d_n10, assign32120_e47073_d_n11, assign32120_e47073_d_n12, assign32120_e47073_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1059 != 0.0)) {
        let assign32120_e47070: f64 = (locals.var_t1__blk1032 - 1.0);
        let assign32120_e47071: f64 = (locals.var_isbd * assign32120_e47070);
        (assign32120_e47071, (locals.var_isbd_dn0 * assign32120_e47070), (locals.var_isbd_dn2 * assign32120_e47070), ((locals.var_isbd_dn6 * assign32120_e47070) + (locals.var_isbd * locals.var_t1__blk1032_dn6)), ((locals.var_isbd_dn7 * assign32120_e47070) + (locals.var_isbd * locals.var_t1__blk1032_dn7)), ((locals.var_isbd_dn10 * assign32120_e47070) + (locals.var_isbd * locals.var_t1__blk1032_dn10)), (locals.var_isbd_dn11 * assign32120_e47070), ((locals.var_isbd_dn12 * assign32120_e47070) + (locals.var_isbd * locals.var_t1__blk1032_dn12)), (locals.var_isbd_dn17 * assign32120_e47070),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32120_e47073;
        locals.var_ibd_dn0 = assign32120_e47073_d_n0;
        locals.var_ibd_dn2 = assign32120_e47073_d_n2;
        locals.var_ibd_dn6 = assign32120_e47073_d_n6;
        locals.var_ibd_dn7 = assign32120_e47073_d_n7;
        locals.var_ibd_dn10 = assign32120_e47073_d_n10;
        locals.var_ibd_dn11 = assign32120_e47073_d_n11;
        locals.var_ibd_dn12 = assign32120_e47073_d_n12;
        locals.var_ibd_dn17 = assign32120_e47073_d_n17;

        let (assign32130_e47083, assign32130_e47083_d_n6, assign32130_e47083_d_n7, assign32130_e47083_d_n10, assign32130_e47083_d_n12,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1059 == 0.0)) {
        let assign32130_e47080: f64 = (locals.var_vbdt / locals.var_nvtm);
        let assign32130_e47081: f64 = (assign32130_e47080).exp();
        (assign32130_e47081, 0.0, 0.0, (assign32130_e47081 * (((locals.var_vbdt_dn10 * locals.var_nvtm) - (locals.var_vbdt * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0,)
    } else {
        (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12,)
    }
};
        locals.var_t1__blk1032 = assign32130_e47083;
        locals.var_t1__blk1032_dn6 = assign32130_e47083_d_n6;
        locals.var_t1__blk1032_dn7 = assign32130_e47083_d_n7;
        locals.var_t1__blk1032_dn10 = assign32130_e47083_d_n10;
        locals.var_t1__blk1032_dn12 = assign32130_e47083_d_n12;

        let (assign32140_e47104, assign32140_e47104_d_n0, assign32140_e47104_d_n2, assign32140_e47104_d_n6, assign32140_e47104_d_n7, assign32140_e47104_d_n10, assign32140_e47104_d_n11, assign32140_e47104_d_n12, assign32140_e47104_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1059 == 0.0)) {
        let assign32140_e47091: f64 = (locals.var_t1__blk1032 - 1.0);
        let assign32140_e47092: f64 = (locals.var_isbd * assign32140_e47091);
        let assign32140_e47095: f64 = (locals.var_isbd / locals.var_nvtm);
        let assign32140_e47097: f64 = (assign32140_e47095 * locals.var_t1__blk1032);
        let assign32140_e47100: f64 = (locals.var_vbdj - locals.var_vbdt);
        let assign32140_e47101: f64 = (assign32140_e47097 * assign32140_e47100);
        let assign32140_e47102: f64 = (assign32140_e47092 + assign32140_e47101);
        (assign32140_e47102, ((locals.var_isbd_dn0 * assign32140_e47091) + (((locals.var_isbd_dn0 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32140_e47100)), ((locals.var_isbd_dn2 * assign32140_e47091) + (((locals.var_isbd_dn2 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32140_e47100)), (((locals.var_isbd_dn6 * assign32140_e47091) + (locals.var_isbd * locals.var_t1__blk1032_dn6)) + (((((locals.var_isbd_dn6 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32140_e47095 * locals.var_t1__blk1032_dn6)) * assign32140_e47100) + (assign32140_e47097 * locals.var_vbdj_dn6))), (((locals.var_isbd_dn7 * assign32140_e47091) + (locals.var_isbd * locals.var_t1__blk1032_dn7)) + ((((locals.var_isbd_dn7 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32140_e47095 * locals.var_t1__blk1032_dn7)) * assign32140_e47100)), (((locals.var_isbd_dn10 * assign32140_e47091) + (locals.var_isbd * locals.var_t1__blk1032_dn10)) + (((((((locals.var_isbd_dn10 * locals.var_nvtm) - (locals.var_isbd * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1032) + (assign32140_e47095 * locals.var_t1__blk1032_dn10)) * assign32140_e47100) + (assign32140_e47097 * (-locals.var_vbdt_dn10)))), ((locals.var_isbd_dn11 * assign32140_e47091) + (((locals.var_isbd_dn11 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32140_e47100)), (((locals.var_isbd_dn12 * assign32140_e47091) + (locals.var_isbd * locals.var_t1__blk1032_dn12)) + (((((locals.var_isbd_dn12 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32140_e47095 * locals.var_t1__blk1032_dn12)) * assign32140_e47100) + (assign32140_e47097 * locals.var_vbdj_dn12))), ((locals.var_isbd_dn17 * assign32140_e47091) + (((locals.var_isbd_dn17 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32140_e47100)),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32140_e47104;
        locals.var_ibd_dn0 = assign32140_e47104_d_n0;
        locals.var_ibd_dn2 = assign32140_e47104_d_n2;
        locals.var_ibd_dn6 = assign32140_e47104_d_n6;
        locals.var_ibd_dn7 = assign32140_e47104_d_n7;
        locals.var_ibd_dn10 = assign32140_e47104_d_n10;
        locals.var_ibd_dn11 = assign32140_e47104_d_n11;
        locals.var_ibd_dn12 = assign32140_e47104_d_n12;
        locals.var_ibd_dn17 = assign32140_e47104_d_n17;

        let (assign32150_e47114, assign32150_e47114_d_n0, assign32150_e47114_d_n2, assign32150_e47114_d_n6, assign32150_e47114_d_n7, assign32150_e47114_d_n10, assign32150_e47114_d_n11, assign32150_e47114_d_n12, assign32150_e47114_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32150_e47109: f64 = (p.p178 * locals.var_vbdj);
        let assign32150_e47111: f64 = (assign32150_e47109 * locals.var_isbd2);
        let assign32150_e47112: f64 = (locals.var_ibd + assign32150_e47111);
        (assign32150_e47112, (locals.var_ibd_dn0 + (assign32150_e47109 * locals.var_isbd2_dn0)), (locals.var_ibd_dn2 + (assign32150_e47109 * locals.var_isbd2_dn2)), (locals.var_ibd_dn6 + (((p.p178 * locals.var_vbdj_dn6) * locals.var_isbd2) + (assign32150_e47109 * locals.var_isbd2_dn6))), (locals.var_ibd_dn7 + (assign32150_e47109 * locals.var_isbd2_dn7)), (locals.var_ibd_dn10 + (assign32150_e47109 * locals.var_isbd2_dn10)), (locals.var_ibd_dn11 + (assign32150_e47109 * locals.var_isbd2_dn11)), (locals.var_ibd_dn12 + (((p.p178 * locals.var_vbdj_dn12) * locals.var_isbd2) + (assign32150_e47109 * locals.var_isbd2_dn12))), (locals.var_ibd_dn17 + (assign32150_e47109 * locals.var_isbd2_dn17)),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32150_e47114;
        locals.var_ibd_dn0 = assign32150_e47114_d_n0;
        locals.var_ibd_dn2 = assign32150_e47114_d_n2;
        locals.var_ibd_dn6 = assign32150_e47114_d_n6;
        locals.var_ibd_dn7 = assign32150_e47114_d_n7;
        locals.var_ibd_dn10 = assign32150_e47114_d_n10;
        locals.var_ibd_dn11 = assign32150_e47114_d_n11;
        locals.var_ibd_dn12 = assign32150_e47114_d_n12;
        locals.var_ibd_dn17 = assign32150_e47114_d_n17;

        let assign32160_e47117: f64 = if locals.var_vbsj < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard1060 = assign32160_e47117;

        let (assign32170_e47126, assign32170_e47126_d_n6, assign32170_e47126_d_n7, assign32170_e47126_d_n10, assign32170_e47126_d_n12,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1060 != 0.0)) {
        let assign32170_e47123: f64 = (locals.var_vbsj / locals.var_nvtm);
        let assign32170_e47124: f64 = (assign32170_e47123).exp();
        (assign32170_e47124, 0.0, (assign32170_e47124 * (locals.var_vbsj_dn7 / locals.var_nvtm)), (assign32170_e47124 * (-((locals.var_vbsj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32170_e47124 * (locals.var_vbsj_dn12 / locals.var_nvtm)),)
    } else {
        (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12,)
    }
};
        locals.var_t1__blk1032 = assign32170_e47126;
        locals.var_t1__blk1032_dn6 = assign32170_e47126_d_n6;
        locals.var_t1__blk1032_dn7 = assign32170_e47126_d_n7;
        locals.var_t1__blk1032_dn10 = assign32170_e47126_d_n10;
        locals.var_t1__blk1032_dn12 = assign32170_e47126_d_n12;

        let (assign32180_e47136, assign32180_e47136_d_n0, assign32180_e47136_d_n2, assign32180_e47136_d_n6, assign32180_e47136_d_n7, assign32180_e47136_d_n10, assign32180_e47136_d_n11, assign32180_e47136_d_n12, assign32180_e47136_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1060 != 0.0)) {
        let assign32180_e47133: f64 = (locals.var_t1__blk1032 - 1.0);
        let assign32180_e47134: f64 = (locals.var_isbs * assign32180_e47133);
        (assign32180_e47134, (locals.var_isbs_dn0 * assign32180_e47133), (locals.var_isbs_dn2 * assign32180_e47133), ((locals.var_isbs_dn6 * assign32180_e47133) + (locals.var_isbs * locals.var_t1__blk1032_dn6)), ((locals.var_isbs_dn7 * assign32180_e47133) + (locals.var_isbs * locals.var_t1__blk1032_dn7)), ((locals.var_isbs_dn10 * assign32180_e47133) + (locals.var_isbs * locals.var_t1__blk1032_dn10)), (locals.var_isbs_dn11 * assign32180_e47133), ((locals.var_isbs_dn12 * assign32180_e47133) + (locals.var_isbs * locals.var_t1__blk1032_dn12)), (locals.var_isbs_dn17 * assign32180_e47133),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32180_e47136;
        locals.var_ibs_dn0 = assign32180_e47136_d_n0;
        locals.var_ibs_dn2 = assign32180_e47136_d_n2;
        locals.var_ibs_dn6 = assign32180_e47136_d_n6;
        locals.var_ibs_dn7 = assign32180_e47136_d_n7;
        locals.var_ibs_dn10 = assign32180_e47136_d_n10;
        locals.var_ibs_dn11 = assign32180_e47136_d_n11;
        locals.var_ibs_dn12 = assign32180_e47136_d_n12;
        locals.var_ibs_dn17 = assign32180_e47136_d_n17;

        let (assign32190_e47146, assign32190_e47146_d_n6, assign32190_e47146_d_n7, assign32190_e47146_d_n10, assign32190_e47146_d_n12,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1060 == 0.0)) {
        let assign32190_e47143: f64 = (locals.var_vbst / locals.var_nvtm);
        let assign32190_e47144: f64 = (assign32190_e47143).exp();
        (assign32190_e47144, 0.0, 0.0, (assign32190_e47144 * (((locals.var_vbst_dn10 * locals.var_nvtm) - (locals.var_vbst * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0,)
    } else {
        (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12,)
    }
};
        locals.var_t1__blk1032 = assign32190_e47146;
        locals.var_t1__blk1032_dn6 = assign32190_e47146_d_n6;
        locals.var_t1__blk1032_dn7 = assign32190_e47146_d_n7;
        locals.var_t1__blk1032_dn10 = assign32190_e47146_d_n10;
        locals.var_t1__blk1032_dn12 = assign32190_e47146_d_n12;

        let (assign32200_e47167, assign32200_e47167_d_n0, assign32200_e47167_d_n2, assign32200_e47167_d_n6, assign32200_e47167_d_n7, assign32200_e47167_d_n10, assign32200_e47167_d_n11, assign32200_e47167_d_n12, assign32200_e47167_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1060 == 0.0)) {
        let assign32200_e47154: f64 = (locals.var_t1__blk1032 - 1.0);
        let assign32200_e47155: f64 = (locals.var_isbs * assign32200_e47154);
        let assign32200_e47158: f64 = (locals.var_isbs / locals.var_nvtm);
        let assign32200_e47160: f64 = (assign32200_e47158 * locals.var_t1__blk1032);
        let assign32200_e47163: f64 = (locals.var_vbsj - locals.var_vbst);
        let assign32200_e47164: f64 = (assign32200_e47160 * assign32200_e47163);
        let assign32200_e47165: f64 = (assign32200_e47155 + assign32200_e47164);
        (assign32200_e47165, ((locals.var_isbs_dn0 * assign32200_e47154) + (((locals.var_isbs_dn0 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32200_e47163)), ((locals.var_isbs_dn2 * assign32200_e47154) + (((locals.var_isbs_dn2 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32200_e47163)), (((locals.var_isbs_dn6 * assign32200_e47154) + (locals.var_isbs * locals.var_t1__blk1032_dn6)) + ((((locals.var_isbs_dn6 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32200_e47158 * locals.var_t1__blk1032_dn6)) * assign32200_e47163)), (((locals.var_isbs_dn7 * assign32200_e47154) + (locals.var_isbs * locals.var_t1__blk1032_dn7)) + (((((locals.var_isbs_dn7 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32200_e47158 * locals.var_t1__blk1032_dn7)) * assign32200_e47163) + (assign32200_e47160 * locals.var_vbsj_dn7))), (((locals.var_isbs_dn10 * assign32200_e47154) + (locals.var_isbs * locals.var_t1__blk1032_dn10)) + (((((((locals.var_isbs_dn10 * locals.var_nvtm) - (locals.var_isbs * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1032) + (assign32200_e47158 * locals.var_t1__blk1032_dn10)) * assign32200_e47163) + (assign32200_e47160 * (-locals.var_vbst_dn10)))), ((locals.var_isbs_dn11 * assign32200_e47154) + (((locals.var_isbs_dn11 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32200_e47163)), (((locals.var_isbs_dn12 * assign32200_e47154) + (locals.var_isbs * locals.var_t1__blk1032_dn12)) + (((((locals.var_isbs_dn12 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32200_e47158 * locals.var_t1__blk1032_dn12)) * assign32200_e47163) + (assign32200_e47160 * locals.var_vbsj_dn12))), ((locals.var_isbs_dn17 * assign32200_e47154) + (((locals.var_isbs_dn17 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32200_e47163)),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32200_e47167;
        locals.var_ibs_dn0 = assign32200_e47167_d_n0;
        locals.var_ibs_dn2 = assign32200_e47167_d_n2;
        locals.var_ibs_dn6 = assign32200_e47167_d_n6;
        locals.var_ibs_dn7 = assign32200_e47167_d_n7;
        locals.var_ibs_dn10 = assign32200_e47167_d_n10;
        locals.var_ibs_dn11 = assign32200_e47167_d_n11;
        locals.var_ibs_dn12 = assign32200_e47167_d_n12;
        locals.var_ibs_dn17 = assign32200_e47167_d_n17;

        let (assign32210_e47177, assign32210_e47177_d_n0, assign32210_e47177_d_n2, assign32210_e47177_d_n6, assign32210_e47177_d_n7, assign32210_e47177_d_n10, assign32210_e47177_d_n11, assign32210_e47177_d_n12, assign32210_e47177_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32210_e47172: f64 = (p.p178 * locals.var_vbsj);
        let assign32210_e47174: f64 = (assign32210_e47172 * locals.var_isbs2);
        let assign32210_e47175: f64 = (locals.var_ibs + assign32210_e47174);
        (assign32210_e47175, (locals.var_ibs_dn0 + (assign32210_e47172 * locals.var_isbs2_dn0)), (locals.var_ibs_dn2 + (assign32210_e47172 * locals.var_isbs2_dn2)), (locals.var_ibs_dn6 + (assign32210_e47172 * locals.var_isbs2_dn6)), (locals.var_ibs_dn7 + (((p.p178 * locals.var_vbsj_dn7) * locals.var_isbs2) + (assign32210_e47172 * locals.var_isbs2_dn7))), (locals.var_ibs_dn10 + (assign32210_e47172 * locals.var_isbs2_dn10)), (locals.var_ibs_dn11 + (assign32210_e47172 * locals.var_isbs2_dn11)), (locals.var_ibs_dn12 + (((p.p178 * locals.var_vbsj_dn12) * locals.var_isbs2) + (assign32210_e47172 * locals.var_isbs2_dn12))), (locals.var_ibs_dn17 + (assign32210_e47172 * locals.var_isbs2_dn17)),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32210_e47177;
        locals.var_ibs_dn0 = assign32210_e47177_d_n0;
        locals.var_ibs_dn2 = assign32210_e47177_d_n2;
        locals.var_ibs_dn6 = assign32210_e47177_d_n6;
        locals.var_ibs_dn7 = assign32210_e47177_d_n7;
        locals.var_ibs_dn10 = assign32210_e47177_d_n10;
        locals.var_ibs_dn11 = assign32210_e47177_d_n11;
        locals.var_ibs_dn12 = assign32210_e47177_d_n12;
        locals.var_ibs_dn17 = assign32210_e47177_d_n17;

        let (assign32220_e47185, assign32220_e47185_d_n0, assign32220_e47185_d_n2, assign32220_e47185_d_n6, assign32220_e47185_d_n7, assign32220_e47185_d_n10, assign32220_e47185_d_n11, assign32220_e47185_d_n12, assign32220_e47185_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32220_e47182: f64 = (locals.var_gjmin * locals.var_vbdj);
        let assign32220_e47183: f64 = (locals.var_ibd + assign32220_e47182);
        (assign32220_e47183, locals.var_ibd_dn0, locals.var_ibd_dn2, (locals.var_ibd_dn6 + (locals.var_gjmin * locals.var_vbdj_dn6)), locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, (locals.var_ibd_dn12 + (locals.var_gjmin * locals.var_vbdj_dn12)), locals.var_ibd_dn17,)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32220_e47185;
        locals.var_ibd_dn0 = assign32220_e47185_d_n0;
        locals.var_ibd_dn2 = assign32220_e47185_d_n2;
        locals.var_ibd_dn6 = assign32220_e47185_d_n6;
        locals.var_ibd_dn7 = assign32220_e47185_d_n7;
        locals.var_ibd_dn10 = assign32220_e47185_d_n10;
        locals.var_ibd_dn11 = assign32220_e47185_d_n11;
        locals.var_ibd_dn12 = assign32220_e47185_d_n12;
        locals.var_ibd_dn17 = assign32220_e47185_d_n17;

        let (assign32230_e47193, assign32230_e47193_d_n0, assign32230_e47193_d_n2, assign32230_e47193_d_n6, assign32230_e47193_d_n7, assign32230_e47193_d_n10, assign32230_e47193_d_n11, assign32230_e47193_d_n12, assign32230_e47193_d_n17,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32230_e47190: f64 = (locals.var_gjmin * locals.var_vbsj);
        let assign32230_e47191: f64 = (locals.var_ibs + assign32230_e47190);
        (assign32230_e47191, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, (locals.var_ibs_dn7 + (locals.var_gjmin * locals.var_vbsj_dn7)), locals.var_ibs_dn10, locals.var_ibs_dn11, (locals.var_ibs_dn12 + (locals.var_gjmin * locals.var_vbsj_dn12)), locals.var_ibs_dn17,)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32230_e47193;
        locals.var_ibs_dn0 = assign32230_e47193_d_n0;
        locals.var_ibs_dn2 = assign32230_e47193_d_n2;
        locals.var_ibs_dn6 = assign32230_e47193_d_n6;
        locals.var_ibs_dn7 = assign32230_e47193_d_n7;
        locals.var_ibs_dn10 = assign32230_e47193_d_n10;
        locals.var_ibs_dn11 = assign32230_e47193_d_n11;
        locals.var_ibs_dn12 = assign32230_e47193_d_n12;
        locals.var_ibs_dn17 = assign32230_e47193_d_n17;

        let (assign32240_e47199,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32240_e47197: f64 = (p.p179 * p.p2);
        (assign32240_e47197,)
    } else {
        (locals.var_czbd,)
    }
};
        locals.var_czbd = assign32240_e47199;

        let (assign32250_e47205,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32250_e47203: f64 = (p.p179 * p.p3);
        (assign32250_e47203,)
    } else {
        (locals.var_czbs,)
    }
};
        locals.var_czbs = assign32250_e47205;

        let (assign32260_e47211,) = {
    if (locals.var_guard1030 != 0.0) {
        let assign32260_e47209: f64 = (p.p237 - p.p238);
        (assign32260_e47209,)
    } else {
        (locals.var_xp_max,)
    }
};
        locals.var_xp_max = assign32260_e47211;

        let assign32270_e47214: f64 = if locals.var_xp_max <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign32270_e47214;

        let (assign32280_e47220,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1061 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_czbd,)
    }
};
        locals.var_czbd = assign32280_e47220;

        let (assign32290_e47226,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1061 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_czbs,)
    }
};
        locals.var_czbs = assign32290_e47226;

        let assign32300_e47229: f64 = if p.p5 > locals.var_w_dioscv { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign32300_e47229;

        let (assign32310_e47239,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) {
        let assign32310_e47236: f64 = (p.p5 - locals.var_w_dioscv);
        let assign32310_e47237: f64 = (p.p180 * assign32310_e47236);
        (assign32310_e47237,)
    } else {
        (locals.var_czbssw,)
    }
};
        locals.var_czbssw = assign32310_e47239;

        let (assign32320_e47247,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) {
        let assign32320_e47245: f64 = (p.p181 * locals.var_w_dioscv);
        (assign32320_e47245,)
    } else {
        (locals.var_czbsswg,)
    }
};
        locals.var_czbsswg = assign32320_e47247;

        let assign32330_e47250: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign32330_e47250;

        let assign32340_e47253: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign32340_e47253;

        let (assign32350_e47267, assign32350_e47267_d_n6, assign32350_e47267_d_n7, assign32350_e47267_d_n12,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign32350_e47264: f64 = (locals.var_vbsj / p.p185);
        let assign32350_e47265: f64 = (1.0 - assign32350_e47264);
        (assign32350_e47265, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12,)
    }
};
        locals.var_arg__blk1057 = assign32350_e47267;
        locals.var_arg__blk1057_dn6 = assign32350_e47267_d_n6;
        locals.var_arg__blk1057_dn7 = assign32350_e47267_d_n7;
        locals.var_arg__blk1057_dn12 = assign32350_e47267_d_n12;

        let assign32360_e47270: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign32360_e47270;

        let (assign32370_e47285, assign32370_e47285_d_n6, assign32370_e47285_d_n7, assign32370_e47285_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) {
        let assign32370_e47282: f64 = (locals.var_arg__blk1057).sqrt();
        let assign32370_e47283: f64 = (1.0 / assign32370_e47282);
        (assign32370_e47283, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32370_e47282)) / (assign32370_e47282 * assign32370_e47282))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32370_e47282)) / (assign32370_e47282 * assign32370_e47282))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32370_e47282)) / (assign32370_e47282 * assign32370_e47282))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32370_e47285;
        locals.var_sarg_dn6 = assign32370_e47285_d_n6;
        locals.var_sarg_dn7 = assign32370_e47285_d_n7;
        locals.var_sarg_dn12 = assign32370_e47285_d_n12;

        let (assign32380_e47301, assign32380_e47301_d_n6, assign32380_e47301_d_n7, assign32380_e47301_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
        let assign32380_e47298: f64 = (-p.p182);
        let assign32380_e47299: f64 = (locals.var_arg__blk1057).powf(assign32380_e47298);
        (assign32380_e47299, if 0.0 == 0.0 && ((assign32380_e47298) as f64).is_finite() && ((assign32380_e47298) as f64).fract() == 0.0 { if assign32380_e47298 == 0.0 { 0.0 } else { (assign32380_e47298 * ((locals.var_arg__blk1057).powf(assign32380_e47298 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32380_e47299 * (assign32380_e47298 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32380_e47298) as f64).is_finite() && ((assign32380_e47298) as f64).fract() == 0.0 { if assign32380_e47298 == 0.0 { 0.0 } else { (assign32380_e47298 * ((locals.var_arg__blk1057).powf(assign32380_e47298 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32380_e47299 * (assign32380_e47298 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32380_e47298) as f64).is_finite() && ((assign32380_e47298) as f64).fract() == 0.0 { if assign32380_e47298 == 0.0 { 0.0 } else { (assign32380_e47298 * ((locals.var_arg__blk1057).powf(assign32380_e47298 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32380_e47299 * (assign32380_e47298 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32380_e47301;
        locals.var_sarg_dn6 = assign32380_e47301_d_n6;
        locals.var_sarg_dn7 = assign32380_e47301_d_n7;
        locals.var_sarg_dn12 = assign32380_e47301_d_n12;

        let (assign32390_e47323, assign32390_e47323_d_n0, assign32390_e47323_d_n2, assign32390_e47323_d_n6, assign32390_e47323_d_n7, assign32390_e47323_d_n10, assign32390_e47323_d_n11, assign32390_e47323_d_n12, assign32390_e47323_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign32390_e47311: f64 = (p.p185 * locals.var_czbs);
        let assign32390_e47315: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
        let assign32390_e47316: f64 = (1.0 - assign32390_e47315);
        let assign32390_e47317: f64 = (assign32390_e47311 * assign32390_e47316);
        let assign32390_e47320: f64 = (1.0 - p.p182);
        let assign32390_e47321: f64 = (assign32390_e47317 / assign32390_e47320);
        (assign32390_e47321, 0.0, 0.0, ((assign32390_e47311 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32390_e47320), ((assign32390_e47311 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32390_e47320), 0.0, 0.0, ((assign32390_e47311 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32390_e47320), 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32390_e47323;
        locals.var_qbs_dn0 = assign32390_e47323_d_n0;
        locals.var_qbs_dn2 = assign32390_e47323_d_n2;
        locals.var_qbs_dn6 = assign32390_e47323_d_n6;
        locals.var_qbs_dn7 = assign32390_e47323_d_n7;
        locals.var_qbs_dn10 = assign32390_e47323_d_n10;
        locals.var_qbs_dn11 = assign32390_e47323_d_n11;
        locals.var_qbs_dn12 = assign32390_e47323_d_n12;
        locals.var_qbs_dn17 = assign32390_e47323_d_n17;

        let (assign32400_e47334, assign32400_e47334_d_n0, assign32400_e47334_d_n2, assign32400_e47334_d_n6, assign32400_e47334_d_n7, assign32400_e47334_d_n10, assign32400_e47334_d_n11, assign32400_e47334_d_n12, assign32400_e47334_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32400_e47334;
        locals.var_qbs_dn0 = assign32400_e47334_d_n0;
        locals.var_qbs_dn2 = assign32400_e47334_d_n2;
        locals.var_qbs_dn6 = assign32400_e47334_d_n6;
        locals.var_qbs_dn7 = assign32400_e47334_d_n7;
        locals.var_qbs_dn10 = assign32400_e47334_d_n10;
        locals.var_qbs_dn11 = assign32400_e47334_d_n11;
        locals.var_qbs_dn12 = assign32400_e47334_d_n12;
        locals.var_qbs_dn17 = assign32400_e47334_d_n17;

        let assign32410_e47337: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign32410_e47337;

        let (assign32420_e47351, assign32420_e47351_d_n6, assign32420_e47351_d_n7, assign32420_e47351_d_n12,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign32420_e47348: f64 = (locals.var_vbsj / p.p186);
        let assign32420_e47349: f64 = (1.0 - assign32420_e47348);
        (assign32420_e47349, 0.0, (-(locals.var_vbsj_dn7 / p.p186)), (-(locals.var_vbsj_dn12 / p.p186)),)
    } else {
        (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12,)
    }
};
        locals.var_arg__blk1057 = assign32420_e47351;
        locals.var_arg__blk1057_dn6 = assign32420_e47351_d_n6;
        locals.var_arg__blk1057_dn7 = assign32420_e47351_d_n7;
        locals.var_arg__blk1057_dn12 = assign32420_e47351_d_n12;

        let assign32430_e47354: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign32430_e47354;

        let (assign32440_e47369, assign32440_e47369_d_n6, assign32440_e47369_d_n7, assign32440_e47369_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
        let assign32440_e47366: f64 = (locals.var_arg__blk1057).sqrt();
        let assign32440_e47367: f64 = (1.0 / assign32440_e47366);
        (assign32440_e47367, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32440_e47366)) / (assign32440_e47366 * assign32440_e47366))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32440_e47366)) / (assign32440_e47366 * assign32440_e47366))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32440_e47366)) / (assign32440_e47366 * assign32440_e47366))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32440_e47369;
        locals.var_sarg_dn6 = assign32440_e47369_d_n6;
        locals.var_sarg_dn7 = assign32440_e47369_d_n7;
        locals.var_sarg_dn12 = assign32440_e47369_d_n12;

        let (assign32450_e47385, assign32450_e47385_d_n6, assign32450_e47385_d_n7, assign32450_e47385_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 == 0.0)) {
        let assign32450_e47382: f64 = (-p.p183);
        let assign32450_e47383: f64 = (locals.var_arg__blk1057).powf(assign32450_e47382);
        (assign32450_e47383, if 0.0 == 0.0 && ((assign32450_e47382) as f64).is_finite() && ((assign32450_e47382) as f64).fract() == 0.0 { if assign32450_e47382 == 0.0 { 0.0 } else { (assign32450_e47382 * ((locals.var_arg__blk1057).powf(assign32450_e47382 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32450_e47383 * (assign32450_e47382 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32450_e47382) as f64).is_finite() && ((assign32450_e47382) as f64).fract() == 0.0 { if assign32450_e47382 == 0.0 { 0.0 } else { (assign32450_e47382 * ((locals.var_arg__blk1057).powf(assign32450_e47382 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32450_e47383 * (assign32450_e47382 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32450_e47382) as f64).is_finite() && ((assign32450_e47382) as f64).fract() == 0.0 { if assign32450_e47382 == 0.0 { 0.0 } else { (assign32450_e47382 * ((locals.var_arg__blk1057).powf(assign32450_e47382 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32450_e47383 * (assign32450_e47382 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32450_e47385;
        locals.var_sarg_dn6 = assign32450_e47385_d_n6;
        locals.var_sarg_dn7 = assign32450_e47385_d_n7;
        locals.var_sarg_dn12 = assign32450_e47385_d_n12;

        let (assign32460_e47409, assign32460_e47409_d_n0, assign32460_e47409_d_n2, assign32460_e47409_d_n6, assign32460_e47409_d_n7, assign32460_e47409_d_n10, assign32460_e47409_d_n11, assign32460_e47409_d_n12, assign32460_e47409_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign32460_e47396: f64 = (p.p186 * locals.var_czbssw);
        let assign32460_e47400: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
        let assign32460_e47401: f64 = (1.0 - assign32460_e47400);
        let assign32460_e47402: f64 = (assign32460_e47396 * assign32460_e47401);
        let assign32460_e47405: f64 = (1.0 - p.p183);
        let assign32460_e47406: f64 = (assign32460_e47402 / assign32460_e47405);
        let assign32460_e47407: f64 = (locals.var_qbs + assign32460_e47406);
        (assign32460_e47407, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32460_e47396 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32460_e47405)), (locals.var_qbs_dn7 + ((assign32460_e47396 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32460_e47405)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32460_e47396 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32460_e47405)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32460_e47409;
        locals.var_qbs_dn0 = assign32460_e47409_d_n0;
        locals.var_qbs_dn2 = assign32460_e47409_d_n2;
        locals.var_qbs_dn6 = assign32460_e47409_d_n6;
        locals.var_qbs_dn7 = assign32460_e47409_d_n7;
        locals.var_qbs_dn10 = assign32460_e47409_d_n10;
        locals.var_qbs_dn11 = assign32460_e47409_d_n11;
        locals.var_qbs_dn12 = assign32460_e47409_d_n12;
        locals.var_qbs_dn17 = assign32460_e47409_d_n17;

        let assign32470_e47412: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign32470_e47412;

        let (assign32480_e47426, assign32480_e47426_d_n6, assign32480_e47426_d_n7, assign32480_e47426_d_n12,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign32480_e47423: f64 = (locals.var_vbsj / p.p187);
        let assign32480_e47424: f64 = (1.0 - assign32480_e47423);
        (assign32480_e47424, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12,)
    }
};
        locals.var_arg__blk1057 = assign32480_e47426;
        locals.var_arg__blk1057_dn6 = assign32480_e47426_d_n6;
        locals.var_arg__blk1057_dn7 = assign32480_e47426_d_n7;
        locals.var_arg__blk1057_dn12 = assign32480_e47426_d_n12;

        let assign32490_e47429: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign32490_e47429;

        let (assign32500_e47444, assign32500_e47444_d_n6, assign32500_e47444_d_n7, assign32500_e47444_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
        let assign32500_e47441: f64 = (locals.var_arg__blk1057).sqrt();
        let assign32500_e47442: f64 = (1.0 / assign32500_e47441);
        (assign32500_e47442, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32500_e47441)) / (assign32500_e47441 * assign32500_e47441))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32500_e47441)) / (assign32500_e47441 * assign32500_e47441))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32500_e47441)) / (assign32500_e47441 * assign32500_e47441))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32500_e47444;
        locals.var_sarg_dn6 = assign32500_e47444_d_n6;
        locals.var_sarg_dn7 = assign32500_e47444_d_n7;
        locals.var_sarg_dn12 = assign32500_e47444_d_n12;

    }

    pub(super) fn stamp_transient_block_115(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32510_e47460, assign32510_e47460_d_n6, assign32510_e47460_d_n7, assign32510_e47460_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 == 0.0)) {
        let assign32510_e47457: f64 = (-p.p184);
        let assign32510_e47458: f64 = (locals.var_arg__blk1057).powf(assign32510_e47457);
        (assign32510_e47458, if 0.0 == 0.0 && ((assign32510_e47457) as f64).is_finite() && ((assign32510_e47457) as f64).fract() == 0.0 { if assign32510_e47457 == 0.0 { 0.0 } else { (assign32510_e47457 * ((locals.var_arg__blk1057).powf(assign32510_e47457 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32510_e47458 * (assign32510_e47457 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32510_e47457) as f64).is_finite() && ((assign32510_e47457) as f64).fract() == 0.0 { if assign32510_e47457 == 0.0 { 0.0 } else { (assign32510_e47457 * ((locals.var_arg__blk1057).powf(assign32510_e47457 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32510_e47458 * (assign32510_e47457 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32510_e47457) as f64).is_finite() && ((assign32510_e47457) as f64).fract() == 0.0 { if assign32510_e47457 == 0.0 { 0.0 } else { (assign32510_e47457 * ((locals.var_arg__blk1057).powf(assign32510_e47457 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32510_e47458 * (assign32510_e47457 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32510_e47460;
        locals.var_sarg_dn6 = assign32510_e47460_d_n6;
        locals.var_sarg_dn7 = assign32510_e47460_d_n7;
        locals.var_sarg_dn12 = assign32510_e47460_d_n12;

        let (assign32520_e47484, assign32520_e47484_d_n0, assign32520_e47484_d_n2, assign32520_e47484_d_n6, assign32520_e47484_d_n7, assign32520_e47484_d_n10, assign32520_e47484_d_n11, assign32520_e47484_d_n12, assign32520_e47484_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign32520_e47471: f64 = (p.p187 * locals.var_czbsswg);
        let assign32520_e47475: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
        let assign32520_e47476: f64 = (1.0 - assign32520_e47475);
        let assign32520_e47477: f64 = (assign32520_e47471 * assign32520_e47476);
        let assign32520_e47480: f64 = (1.0 - p.p184);
        let assign32520_e47481: f64 = (assign32520_e47477 / assign32520_e47480);
        let assign32520_e47482: f64 = (locals.var_qbs + assign32520_e47481);
        (assign32520_e47482, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32520_e47471 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32520_e47480)), (locals.var_qbs_dn7 + ((assign32520_e47471 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32520_e47480)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32520_e47471 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32520_e47480)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32520_e47484;
        locals.var_qbs_dn0 = assign32520_e47484_d_n0;
        locals.var_qbs_dn2 = assign32520_e47484_d_n2;
        locals.var_qbs_dn6 = assign32520_e47484_d_n6;
        locals.var_qbs_dn7 = assign32520_e47484_d_n7;
        locals.var_qbs_dn10 = assign32520_e47484_d_n10;
        locals.var_qbs_dn11 = assign32520_e47484_d_n11;
        locals.var_qbs_dn12 = assign32520_e47484_d_n12;
        locals.var_qbs_dn17 = assign32520_e47484_d_n17;

        let (assign32530_e47497, assign32530_e47497_d_n6, assign32530_e47497_d_n7, assign32530_e47497_d_n10, assign32530_e47497_d_n12,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
        let assign32530_e47493: f64 = (locals.var_czbs + locals.var_czbssw);
        let assign32530_e47495: f64 = (assign32530_e47493 + locals.var_czbsswg);
        (assign32530_e47495, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12,)
    }
};
        locals.var_t1__blk1032 = assign32530_e47497;
        locals.var_t1__blk1032_dn6 = assign32530_e47497_d_n6;
        locals.var_t1__blk1032_dn7 = assign32530_e47497_d_n7;
        locals.var_t1__blk1032_dn10 = assign32530_e47497_d_n10;
        locals.var_t1__blk1032_dn12 = assign32530_e47497_d_n12;

        let (assign32540_e47522, assign32540_e47522_d_n0, assign32540_e47522_d_n2, assign32540_e47522_d_n6, assign32540_e47522_d_n7, assign32540_e47522_d_n10, assign32540_e47522_d_n11, assign32540_e47522_d_n12, assign32540_e47522_d_n17,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
        let assign32540_e47506: f64 = (locals.var_czbs * p.p182);
        let assign32540_e47508: f64 = (assign32540_e47506 / p.p185);
        let assign32540_e47511: f64 = (locals.var_czbssw * p.p183);
        let assign32540_e47513: f64 = (assign32540_e47511 / p.p186);
        let assign32540_e47514: f64 = (assign32540_e47508 + assign32540_e47513);
        let assign32540_e47517: f64 = (locals.var_czbsswg * p.p184);
        let assign32540_e47519: f64 = (assign32540_e47517 / p.p187);
        let assign32540_e47520: f64 = (assign32540_e47514 + assign32540_e47519);
        (assign32540_e47520, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1033, locals.var_t2__blk1033_dn0, locals.var_t2__blk1033_dn2, locals.var_t2__blk1033_dn6, locals.var_t2__blk1033_dn7, locals.var_t2__blk1033_dn10, locals.var_t2__blk1033_dn11, locals.var_t2__blk1033_dn12, locals.var_t2__blk1033_dn17,)
    }
};
        locals.var_t2__blk1033 = assign32540_e47522;
        locals.var_t2__blk1033_dn0 = assign32540_e47522_d_n0;
        locals.var_t2__blk1033_dn2 = assign32540_e47522_d_n2;
        locals.var_t2__blk1033_dn6 = assign32540_e47522_d_n6;
        locals.var_t2__blk1033_dn7 = assign32540_e47522_d_n7;
        locals.var_t2__blk1033_dn10 = assign32540_e47522_d_n10;
        locals.var_t2__blk1033_dn11 = assign32540_e47522_d_n11;
        locals.var_t2__blk1033_dn12 = assign32540_e47522_d_n12;
        locals.var_t2__blk1033_dn17 = assign32540_e47522_d_n17;

        let (assign32550_e47539, assign32550_e47539_d_n0, assign32550_e47539_d_n2, assign32550_e47539_d_n6, assign32550_e47539_d_n7, assign32550_e47539_d_n10, assign32550_e47539_d_n11, assign32550_e47539_d_n12, assign32550_e47539_d_n17,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
        let assign32550_e47533: f64 = (locals.var_vbsj * 0.5);
        let assign32550_e47535: f64 = (assign32550_e47533 * locals.var_t2__blk1033);
        let assign32550_e47536: f64 = (locals.var_t1__blk1032 + assign32550_e47535);
        let assign32550_e47537: f64 = (locals.var_vbsj * assign32550_e47536);
        (assign32550_e47537, (locals.var_vbsj * (assign32550_e47533 * locals.var_t2__blk1033_dn0)), (locals.var_vbsj * (assign32550_e47533 * locals.var_t2__blk1033_dn2)), (locals.var_vbsj * (locals.var_t1__blk1032_dn6 + (assign32550_e47533 * locals.var_t2__blk1033_dn6))), ((locals.var_vbsj_dn7 * assign32550_e47536) + (locals.var_vbsj * (locals.var_t1__blk1032_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1033) + (assign32550_e47533 * locals.var_t2__blk1033_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1032_dn10 + (assign32550_e47533 * locals.var_t2__blk1033_dn10))), (locals.var_vbsj * (assign32550_e47533 * locals.var_t2__blk1033_dn11)), ((locals.var_vbsj_dn12 * assign32550_e47536) + (locals.var_vbsj * (locals.var_t1__blk1032_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1033) + (assign32550_e47533 * locals.var_t2__blk1033_dn12))))), (locals.var_vbsj * (assign32550_e47533 * locals.var_t2__blk1033_dn17)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32550_e47539;
        locals.var_qbs_dn0 = assign32550_e47539_d_n0;
        locals.var_qbs_dn2 = assign32550_e47539_d_n2;
        locals.var_qbs_dn6 = assign32550_e47539_d_n6;
        locals.var_qbs_dn7 = assign32550_e47539_d_n7;
        locals.var_qbs_dn10 = assign32550_e47539_d_n10;
        locals.var_qbs_dn11 = assign32550_e47539_d_n11;
        locals.var_qbs_dn12 = assign32550_e47539_d_n12;
        locals.var_qbs_dn17 = assign32550_e47539_d_n17;

        let (assign32560_e47548,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) {
        let assign32560_e47546: f64 = (p.p181 * p.p5);
        (assign32560_e47546,)
    } else {
        (locals.var_czbsswg,)
    }
};
        locals.var_czbsswg = assign32560_e47548;

        let assign32570_e47551: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign32570_e47551;

        let assign32580_e47554: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign32580_e47554;

        let (assign32590_e47569, assign32590_e47569_d_n6, assign32590_e47569_d_n7, assign32590_e47569_d_n12,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) {
        let assign32590_e47566: f64 = (locals.var_vbsj / p.p185);
        let assign32590_e47567: f64 = (1.0 - assign32590_e47566);
        (assign32590_e47567, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12,)
    }
};
        locals.var_arg__blk1057 = assign32590_e47569;
        locals.var_arg__blk1057_dn6 = assign32590_e47569_d_n6;
        locals.var_arg__blk1057_dn7 = assign32590_e47569_d_n7;
        locals.var_arg__blk1057_dn12 = assign32590_e47569_d_n12;

        let assign32600_e47572: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign32600_e47572;

        let (assign32610_e47588, assign32610_e47588_d_n6, assign32610_e47588_d_n7, assign32610_e47588_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 != 0.0)) {
        let assign32610_e47585: f64 = (locals.var_arg__blk1057).sqrt();
        let assign32610_e47586: f64 = (1.0 / assign32610_e47585);
        (assign32610_e47586, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32610_e47585)) / (assign32610_e47585 * assign32610_e47585))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32610_e47585)) / (assign32610_e47585 * assign32610_e47585))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32610_e47585)) / (assign32610_e47585 * assign32610_e47585))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32610_e47588;
        locals.var_sarg_dn6 = assign32610_e47588_d_n6;
        locals.var_sarg_dn7 = assign32610_e47588_d_n7;
        locals.var_sarg_dn12 = assign32610_e47588_d_n12;

        let (assign32620_e47605, assign32620_e47605_d_n6, assign32620_e47605_d_n7, assign32620_e47605_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 == 0.0)) {
        let assign32620_e47602: f64 = (-p.p182);
        let assign32620_e47603: f64 = (locals.var_arg__blk1057).powf(assign32620_e47602);
        (assign32620_e47603, if 0.0 == 0.0 && ((assign32620_e47602) as f64).is_finite() && ((assign32620_e47602) as f64).fract() == 0.0 { if assign32620_e47602 == 0.0 { 0.0 } else { (assign32620_e47602 * ((locals.var_arg__blk1057).powf(assign32620_e47602 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32620_e47603 * (assign32620_e47602 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32620_e47602) as f64).is_finite() && ((assign32620_e47602) as f64).fract() == 0.0 { if assign32620_e47602 == 0.0 { 0.0 } else { (assign32620_e47602 * ((locals.var_arg__blk1057).powf(assign32620_e47602 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32620_e47603 * (assign32620_e47602 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32620_e47602) as f64).is_finite() && ((assign32620_e47602) as f64).fract() == 0.0 { if assign32620_e47602 == 0.0 { 0.0 } else { (assign32620_e47602 * ((locals.var_arg__blk1057).powf(assign32620_e47602 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32620_e47603 * (assign32620_e47602 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32620_e47605;
        locals.var_sarg_dn6 = assign32620_e47605_d_n6;
        locals.var_sarg_dn7 = assign32620_e47605_d_n7;
        locals.var_sarg_dn12 = assign32620_e47605_d_n12;

        let (assign32630_e47628, assign32630_e47628_d_n0, assign32630_e47628_d_n2, assign32630_e47628_d_n6, assign32630_e47628_d_n7, assign32630_e47628_d_n10, assign32630_e47628_d_n11, assign32630_e47628_d_n12, assign32630_e47628_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) {
        let assign32630_e47616: f64 = (p.p185 * locals.var_czbs);
        let assign32630_e47620: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
        let assign32630_e47621: f64 = (1.0 - assign32630_e47620);
        let assign32630_e47622: f64 = (assign32630_e47616 * assign32630_e47621);
        let assign32630_e47625: f64 = (1.0 - p.p182);
        let assign32630_e47626: f64 = (assign32630_e47622 / assign32630_e47625);
        (assign32630_e47626, 0.0, 0.0, ((assign32630_e47616 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32630_e47625), ((assign32630_e47616 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32630_e47625), 0.0, 0.0, ((assign32630_e47616 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32630_e47625), 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32630_e47628;
        locals.var_qbs_dn0 = assign32630_e47628_d_n0;
        locals.var_qbs_dn2 = assign32630_e47628_d_n2;
        locals.var_qbs_dn6 = assign32630_e47628_d_n6;
        locals.var_qbs_dn7 = assign32630_e47628_d_n7;
        locals.var_qbs_dn10 = assign32630_e47628_d_n10;
        locals.var_qbs_dn11 = assign32630_e47628_d_n11;
        locals.var_qbs_dn12 = assign32630_e47628_d_n12;
        locals.var_qbs_dn17 = assign32630_e47628_d_n17;

        let (assign32640_e47640, assign32640_e47640_d_n0, assign32640_e47640_d_n2, assign32640_e47640_d_n6, assign32640_e47640_d_n7, assign32640_e47640_d_n10, assign32640_e47640_d_n11, assign32640_e47640_d_n12, assign32640_e47640_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32640_e47640;
        locals.var_qbs_dn0 = assign32640_e47640_d_n0;
        locals.var_qbs_dn2 = assign32640_e47640_d_n2;
        locals.var_qbs_dn6 = assign32640_e47640_d_n6;
        locals.var_qbs_dn7 = assign32640_e47640_d_n7;
        locals.var_qbs_dn10 = assign32640_e47640_d_n10;
        locals.var_qbs_dn11 = assign32640_e47640_d_n11;
        locals.var_qbs_dn12 = assign32640_e47640_d_n12;
        locals.var_qbs_dn17 = assign32640_e47640_d_n17;

        let assign32650_e47643: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign32650_e47643;

        let (assign32660_e47658, assign32660_e47658_d_n6, assign32660_e47658_d_n7, assign32660_e47658_d_n12,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1073 != 0.0)) {
        let assign32660_e47655: f64 = (locals.var_vbsj / p.p187);
        let assign32660_e47656: f64 = (1.0 - assign32660_e47655);
        (assign32660_e47656, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12,)
    }
};
        locals.var_arg__blk1057 = assign32660_e47658;
        locals.var_arg__blk1057_dn6 = assign32660_e47658_d_n6;
        locals.var_arg__blk1057_dn7 = assign32660_e47658_d_n7;
        locals.var_arg__blk1057_dn12 = assign32660_e47658_d_n12;

        let assign32670_e47661: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign32670_e47661;

        let (assign32680_e47677, assign32680_e47677_d_n6, assign32680_e47677_d_n7, assign32680_e47677_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) {
        let assign32680_e47674: f64 = (locals.var_arg__blk1057).sqrt();
        let assign32680_e47675: f64 = (1.0 / assign32680_e47674);
        (assign32680_e47675, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32680_e47674)) / (assign32680_e47674 * assign32680_e47674))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32680_e47674)) / (assign32680_e47674 * assign32680_e47674))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32680_e47674)) / (assign32680_e47674 * assign32680_e47674))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32680_e47677;
        locals.var_sarg_dn6 = assign32680_e47677_d_n6;
        locals.var_sarg_dn7 = assign32680_e47677_d_n7;
        locals.var_sarg_dn12 = assign32680_e47677_d_n12;

        let (assign32690_e47694, assign32690_e47694_d_n6, assign32690_e47694_d_n7, assign32690_e47694_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
        let assign32690_e47691: f64 = (-p.p184);
        let assign32690_e47692: f64 = (locals.var_arg__blk1057).powf(assign32690_e47691);
        (assign32690_e47692, if 0.0 == 0.0 && ((assign32690_e47691) as f64).is_finite() && ((assign32690_e47691) as f64).fract() == 0.0 { if assign32690_e47691 == 0.0 { 0.0 } else { (assign32690_e47691 * ((locals.var_arg__blk1057).powf(assign32690_e47691 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32690_e47692 * (assign32690_e47691 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32690_e47691) as f64).is_finite() && ((assign32690_e47691) as f64).fract() == 0.0 { if assign32690_e47691 == 0.0 { 0.0 } else { (assign32690_e47691 * ((locals.var_arg__blk1057).powf(assign32690_e47691 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32690_e47692 * (assign32690_e47691 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32690_e47691) as f64).is_finite() && ((assign32690_e47691) as f64).fract() == 0.0 { if assign32690_e47691 == 0.0 { 0.0 } else { (assign32690_e47691 * ((locals.var_arg__blk1057).powf(assign32690_e47691 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32690_e47692 * (assign32690_e47691 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32690_e47694;
        locals.var_sarg_dn6 = assign32690_e47694_d_n6;
        locals.var_sarg_dn7 = assign32690_e47694_d_n7;
        locals.var_sarg_dn12 = assign32690_e47694_d_n12;

        let (assign32700_e47719, assign32700_e47719_d_n0, assign32700_e47719_d_n2, assign32700_e47719_d_n6, assign32700_e47719_d_n7, assign32700_e47719_d_n10, assign32700_e47719_d_n11, assign32700_e47719_d_n12, assign32700_e47719_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1073 != 0.0)) {
        let assign32700_e47706: f64 = (p.p187 * locals.var_czbsswg);
        let assign32700_e47710: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
        let assign32700_e47711: f64 = (1.0 - assign32700_e47710);
        let assign32700_e47712: f64 = (assign32700_e47706 * assign32700_e47711);
        let assign32700_e47715: f64 = (1.0 - p.p184);
        let assign32700_e47716: f64 = (assign32700_e47712 / assign32700_e47715);
        let assign32700_e47717: f64 = (locals.var_qbs + assign32700_e47716);
        (assign32700_e47717, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32700_e47706 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32700_e47715)), (locals.var_qbs_dn7 + ((assign32700_e47706 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32700_e47715)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32700_e47706 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32700_e47715)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32700_e47719;
        locals.var_qbs_dn0 = assign32700_e47719_d_n0;
        locals.var_qbs_dn2 = assign32700_e47719_d_n2;
        locals.var_qbs_dn6 = assign32700_e47719_d_n6;
        locals.var_qbs_dn7 = assign32700_e47719_d_n7;
        locals.var_qbs_dn10 = assign32700_e47719_d_n10;
        locals.var_qbs_dn11 = assign32700_e47719_d_n11;
        locals.var_qbs_dn12 = assign32700_e47719_d_n12;
        locals.var_qbs_dn17 = assign32700_e47719_d_n17;

        let (assign32710_e47731, assign32710_e47731_d_n6, assign32710_e47731_d_n7, assign32710_e47731_d_n10, assign32710_e47731_d_n12,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 == 0.0)) {
        let assign32710_e47729: f64 = (locals.var_czbs + locals.var_czbsswg);
        (assign32710_e47729, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12,)
    }
};
        locals.var_t1__blk1032 = assign32710_e47731;
        locals.var_t1__blk1032_dn6 = assign32710_e47731_d_n6;
        locals.var_t1__blk1032_dn7 = assign32710_e47731_d_n7;
        locals.var_t1__blk1032_dn10 = assign32710_e47731_d_n10;
        locals.var_t1__blk1032_dn12 = assign32710_e47731_d_n12;

        let (assign32720_e47751, assign32720_e47751_d_n0, assign32720_e47751_d_n2, assign32720_e47751_d_n6, assign32720_e47751_d_n7, assign32720_e47751_d_n10, assign32720_e47751_d_n11, assign32720_e47751_d_n12, assign32720_e47751_d_n17,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 == 0.0)) {
        let assign32720_e47741: f64 = (locals.var_czbs * p.p182);
        let assign32720_e47743: f64 = (assign32720_e47741 / p.p185);
        let assign32720_e47746: f64 = (locals.var_czbsswg * p.p184);
        let assign32720_e47748: f64 = (assign32720_e47746 / p.p187);
        let assign32720_e47749: f64 = (assign32720_e47743 + assign32720_e47748);
        (assign32720_e47749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1033, locals.var_t2__blk1033_dn0, locals.var_t2__blk1033_dn2, locals.var_t2__blk1033_dn6, locals.var_t2__blk1033_dn7, locals.var_t2__blk1033_dn10, locals.var_t2__blk1033_dn11, locals.var_t2__blk1033_dn12, locals.var_t2__blk1033_dn17,)
    }
};
        locals.var_t2__blk1033 = assign32720_e47751;
        locals.var_t2__blk1033_dn0 = assign32720_e47751_d_n0;
        locals.var_t2__blk1033_dn2 = assign32720_e47751_d_n2;
        locals.var_t2__blk1033_dn6 = assign32720_e47751_d_n6;
        locals.var_t2__blk1033_dn7 = assign32720_e47751_d_n7;
        locals.var_t2__blk1033_dn10 = assign32720_e47751_d_n10;
        locals.var_t2__blk1033_dn11 = assign32720_e47751_d_n11;
        locals.var_t2__blk1033_dn12 = assign32720_e47751_d_n12;
        locals.var_t2__blk1033_dn17 = assign32720_e47751_d_n17;

        let (assign32730_e47769, assign32730_e47769_d_n0, assign32730_e47769_d_n2, assign32730_e47769_d_n6, assign32730_e47769_d_n7, assign32730_e47769_d_n10, assign32730_e47769_d_n11, assign32730_e47769_d_n12, assign32730_e47769_d_n17,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 == 0.0)) {
        let assign32730_e47763: f64 = (locals.var_vbsj * 0.5);
        let assign32730_e47765: f64 = (assign32730_e47763 * locals.var_t2__blk1033);
        let assign32730_e47766: f64 = (locals.var_t1__blk1032 + assign32730_e47765);
        let assign32730_e47767: f64 = (locals.var_vbsj * assign32730_e47766);
        (assign32730_e47767, (locals.var_vbsj * (assign32730_e47763 * locals.var_t2__blk1033_dn0)), (locals.var_vbsj * (assign32730_e47763 * locals.var_t2__blk1033_dn2)), (locals.var_vbsj * (locals.var_t1__blk1032_dn6 + (assign32730_e47763 * locals.var_t2__blk1033_dn6))), ((locals.var_vbsj_dn7 * assign32730_e47766) + (locals.var_vbsj * (locals.var_t1__blk1032_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1033) + (assign32730_e47763 * locals.var_t2__blk1033_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1032_dn10 + (assign32730_e47763 * locals.var_t2__blk1033_dn10))), (locals.var_vbsj * (assign32730_e47763 * locals.var_t2__blk1033_dn11)), ((locals.var_vbsj_dn12 * assign32730_e47766) + (locals.var_vbsj * (locals.var_t1__blk1032_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1033) + (assign32730_e47763 * locals.var_t2__blk1033_dn12))))), (locals.var_vbsj * (assign32730_e47763 * locals.var_t2__blk1033_dn17)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32730_e47769;
        locals.var_qbs_dn0 = assign32730_e47769_d_n0;
        locals.var_qbs_dn2 = assign32730_e47769_d_n2;
        locals.var_qbs_dn6 = assign32730_e47769_d_n6;
        locals.var_qbs_dn7 = assign32730_e47769_d_n7;
        locals.var_qbs_dn10 = assign32730_e47769_d_n10;
        locals.var_qbs_dn11 = assign32730_e47769_d_n11;
        locals.var_qbs_dn12 = assign32730_e47769_d_n12;
        locals.var_qbs_dn17 = assign32730_e47769_d_n17;

        let assign32740_e47772: f64 = if p.p4 > locals.var_w_diodcv { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign32740_e47772;

        let (assign32750_e47782,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) {
        let assign32750_e47779: f64 = (p.p4 - locals.var_w_diodcv);
        let assign32750_e47780: f64 = (p.p180 * assign32750_e47779);
        (assign32750_e47780,)
    } else {
        (locals.var_czbdsw,)
    }
};
        locals.var_czbdsw = assign32750_e47782;

        let (assign32760_e47790,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) {
        let assign32760_e47788: f64 = (p.p181 * locals.var_w_diodcv);
        (assign32760_e47788,)
    } else {
        (locals.var_czbdswg,)
    }
};
        locals.var_czbdswg = assign32760_e47790;

        let assign32770_e47793: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign32770_e47793;

        let assign32780_e47796: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign32780_e47796;

        let (assign32790_e47810, assign32790_e47810_d_n6, assign32790_e47810_d_n7, assign32790_e47810_d_n12,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 != 0.0)) {
        let assign32790_e47807: f64 = (locals.var_vbdj / p.p185);
        let assign32790_e47808: f64 = (1.0 - assign32790_e47807);
        (assign32790_e47808, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12,)
    }
};
        locals.var_arg__blk1057 = assign32790_e47810;
        locals.var_arg__blk1057_dn6 = assign32790_e47810_d_n6;
        locals.var_arg__blk1057_dn7 = assign32790_e47810_d_n7;
        locals.var_arg__blk1057_dn12 = assign32790_e47810_d_n12;

        let assign32800_e47813: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign32800_e47813;

        let (assign32810_e47828, assign32810_e47828_d_n6, assign32810_e47828_d_n7, assign32810_e47828_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) {
        let assign32810_e47825: f64 = (locals.var_arg__blk1057).sqrt();
        let assign32810_e47826: f64 = (1.0 / assign32810_e47825);
        (assign32810_e47826, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32810_e47825)) / (assign32810_e47825 * assign32810_e47825))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32810_e47825)) / (assign32810_e47825 * assign32810_e47825))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32810_e47825)) / (assign32810_e47825 * assign32810_e47825))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32810_e47828;
        locals.var_sarg_dn6 = assign32810_e47828_d_n6;
        locals.var_sarg_dn7 = assign32810_e47828_d_n7;
        locals.var_sarg_dn12 = assign32810_e47828_d_n12;

        let (assign32820_e47844, assign32820_e47844_d_n6, assign32820_e47844_d_n7, assign32820_e47844_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
        let assign32820_e47841: f64 = (-p.p182);
        let assign32820_e47842: f64 = (locals.var_arg__blk1057).powf(assign32820_e47841);
        (assign32820_e47842, if 0.0 == 0.0 && ((assign32820_e47841) as f64).is_finite() && ((assign32820_e47841) as f64).fract() == 0.0 { if assign32820_e47841 == 0.0 { 0.0 } else { (assign32820_e47841 * ((locals.var_arg__blk1057).powf(assign32820_e47841 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32820_e47842 * (assign32820_e47841 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32820_e47841) as f64).is_finite() && ((assign32820_e47841) as f64).fract() == 0.0 { if assign32820_e47841 == 0.0 { 0.0 } else { (assign32820_e47841 * ((locals.var_arg__blk1057).powf(assign32820_e47841 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32820_e47842 * (assign32820_e47841 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32820_e47841) as f64).is_finite() && ((assign32820_e47841) as f64).fract() == 0.0 { if assign32820_e47841 == 0.0 { 0.0 } else { (assign32820_e47841 * ((locals.var_arg__blk1057).powf(assign32820_e47841 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32820_e47842 * (assign32820_e47841 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32820_e47844;
        locals.var_sarg_dn6 = assign32820_e47844_d_n6;
        locals.var_sarg_dn7 = assign32820_e47844_d_n7;
        locals.var_sarg_dn12 = assign32820_e47844_d_n12;

        let (assign32830_e47866, assign32830_e47866_d_n0, assign32830_e47866_d_n2, assign32830_e47866_d_n6, assign32830_e47866_d_n7, assign32830_e47866_d_n10, assign32830_e47866_d_n11, assign32830_e47866_d_n12, assign32830_e47866_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 != 0.0)) {
        let assign32830_e47854: f64 = (p.p185 * locals.var_czbd);
        let assign32830_e47858: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
        let assign32830_e47859: f64 = (1.0 - assign32830_e47858);
        let assign32830_e47860: f64 = (assign32830_e47854 * assign32830_e47859);
        let assign32830_e47863: f64 = (1.0 - p.p182);
        let assign32830_e47864: f64 = (assign32830_e47860 / assign32830_e47863);
        (assign32830_e47864, 0.0, 0.0, ((assign32830_e47854 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32830_e47863), ((assign32830_e47854 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32830_e47863), 0.0, 0.0, ((assign32830_e47854 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32830_e47863), 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32830_e47866;
        locals.var_qbd_dn0 = assign32830_e47866_d_n0;
        locals.var_qbd_dn2 = assign32830_e47866_d_n2;
        locals.var_qbd_dn6 = assign32830_e47866_d_n6;
        locals.var_qbd_dn7 = assign32830_e47866_d_n7;
        locals.var_qbd_dn10 = assign32830_e47866_d_n10;
        locals.var_qbd_dn11 = assign32830_e47866_d_n11;
        locals.var_qbd_dn12 = assign32830_e47866_d_n12;
        locals.var_qbd_dn17 = assign32830_e47866_d_n17;

        let (assign32840_e47877, assign32840_e47877_d_n0, assign32840_e47877_d_n2, assign32840_e47877_d_n6, assign32840_e47877_d_n7, assign32840_e47877_d_n10, assign32840_e47877_d_n11, assign32840_e47877_d_n12, assign32840_e47877_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32840_e47877;
        locals.var_qbd_dn0 = assign32840_e47877_d_n0;
        locals.var_qbd_dn2 = assign32840_e47877_d_n2;
        locals.var_qbd_dn6 = assign32840_e47877_d_n6;
        locals.var_qbd_dn7 = assign32840_e47877_d_n7;
        locals.var_qbd_dn10 = assign32840_e47877_d_n10;
        locals.var_qbd_dn11 = assign32840_e47877_d_n11;
        locals.var_qbd_dn12 = assign32840_e47877_d_n12;
        locals.var_qbd_dn17 = assign32840_e47877_d_n17;

        let assign32850_e47880: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign32850_e47880;

        let (assign32860_e47894, assign32860_e47894_d_n6, assign32860_e47894_d_n7, assign32860_e47894_d_n12,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1079 != 0.0)) {
        let assign32860_e47891: f64 = (locals.var_vbdj / p.p186);
        let assign32860_e47892: f64 = (1.0 - assign32860_e47891);
        (assign32860_e47892, (-(locals.var_vbdj_dn6 / p.p186)), 0.0, (-(locals.var_vbdj_dn12 / p.p186)),)
    } else {
        (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12,)
    }
};
        locals.var_arg__blk1057 = assign32860_e47894;
        locals.var_arg__blk1057_dn6 = assign32860_e47894_d_n6;
        locals.var_arg__blk1057_dn7 = assign32860_e47894_d_n7;
        locals.var_arg__blk1057_dn12 = assign32860_e47894_d_n12;

        let assign32870_e47897: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign32870_e47897;

        let (assign32880_e47912, assign32880_e47912_d_n6, assign32880_e47912_d_n7, assign32880_e47912_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 != 0.0)) {
        let assign32880_e47909: f64 = (locals.var_arg__blk1057).sqrt();
        let assign32880_e47910: f64 = (1.0 / assign32880_e47909);
        (assign32880_e47910, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32880_e47909)) / (assign32880_e47909 * assign32880_e47909))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32880_e47909)) / (assign32880_e47909 * assign32880_e47909))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32880_e47909)) / (assign32880_e47909 * assign32880_e47909))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32880_e47912;
        locals.var_sarg_dn6 = assign32880_e47912_d_n6;
        locals.var_sarg_dn7 = assign32880_e47912_d_n7;
        locals.var_sarg_dn12 = assign32880_e47912_d_n12;

        let (assign32890_e47928, assign32890_e47928_d_n6, assign32890_e47928_d_n7, assign32890_e47928_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 == 0.0)) {
        let assign32890_e47925: f64 = (-p.p183);
        let assign32890_e47926: f64 = (locals.var_arg__blk1057).powf(assign32890_e47925);
        (assign32890_e47926, if 0.0 == 0.0 && ((assign32890_e47925) as f64).is_finite() && ((assign32890_e47925) as f64).fract() == 0.0 { if assign32890_e47925 == 0.0 { 0.0 } else { (assign32890_e47925 * ((locals.var_arg__blk1057).powf(assign32890_e47925 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32890_e47926 * (assign32890_e47925 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32890_e47925) as f64).is_finite() && ((assign32890_e47925) as f64).fract() == 0.0 { if assign32890_e47925 == 0.0 { 0.0 } else { (assign32890_e47925 * ((locals.var_arg__blk1057).powf(assign32890_e47925 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32890_e47926 * (assign32890_e47925 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32890_e47925) as f64).is_finite() && ((assign32890_e47925) as f64).fract() == 0.0 { if assign32890_e47925 == 0.0 { 0.0 } else { (assign32890_e47925 * ((locals.var_arg__blk1057).powf(assign32890_e47925 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32890_e47926 * (assign32890_e47925 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32890_e47928;
        locals.var_sarg_dn6 = assign32890_e47928_d_n6;
        locals.var_sarg_dn7 = assign32890_e47928_d_n7;
        locals.var_sarg_dn12 = assign32890_e47928_d_n12;

    }

    pub(super) fn stamp_transient_block_116(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32900_e47952, assign32900_e47952_d_n0, assign32900_e47952_d_n2, assign32900_e47952_d_n6, assign32900_e47952_d_n7, assign32900_e47952_d_n10, assign32900_e47952_d_n11, assign32900_e47952_d_n12, assign32900_e47952_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1079 != 0.0)) {
        let assign32900_e47939: f64 = (p.p186 * locals.var_czbdsw);
        let assign32900_e47943: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
        let assign32900_e47944: f64 = (1.0 - assign32900_e47943);
        let assign32900_e47945: f64 = (assign32900_e47939 * assign32900_e47944);
        let assign32900_e47948: f64 = (1.0 - p.p183);
        let assign32900_e47949: f64 = (assign32900_e47945 / assign32900_e47948);
        let assign32900_e47950: f64 = (locals.var_qbd + assign32900_e47949);
        (assign32900_e47950, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32900_e47939 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32900_e47948)), (locals.var_qbd_dn7 + ((assign32900_e47939 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32900_e47948)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32900_e47939 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32900_e47948)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32900_e47952;
        locals.var_qbd_dn0 = assign32900_e47952_d_n0;
        locals.var_qbd_dn2 = assign32900_e47952_d_n2;
        locals.var_qbd_dn6 = assign32900_e47952_d_n6;
        locals.var_qbd_dn7 = assign32900_e47952_d_n7;
        locals.var_qbd_dn10 = assign32900_e47952_d_n10;
        locals.var_qbd_dn11 = assign32900_e47952_d_n11;
        locals.var_qbd_dn12 = assign32900_e47952_d_n12;
        locals.var_qbd_dn17 = assign32900_e47952_d_n17;

        let assign32910_e47955: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign32910_e47955;

        let (assign32920_e47969, assign32920_e47969_d_n6, assign32920_e47969_d_n7, assign32920_e47969_d_n12,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign32920_e47966: f64 = (locals.var_vbdj / p.p187);
        let assign32920_e47967: f64 = (1.0 - assign32920_e47966);
        (assign32920_e47967, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12,)
    }
};
        locals.var_arg__blk1057 = assign32920_e47969;
        locals.var_arg__blk1057_dn6 = assign32920_e47969_d_n6;
        locals.var_arg__blk1057_dn7 = assign32920_e47969_d_n7;
        locals.var_arg__blk1057_dn12 = assign32920_e47969_d_n12;

        let assign32930_e47972: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign32930_e47972;

        let (assign32940_e47987, assign32940_e47987_d_n6, assign32940_e47987_d_n7, assign32940_e47987_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
        let assign32940_e47984: f64 = (locals.var_arg__blk1057).sqrt();
        let assign32940_e47985: f64 = (1.0 / assign32940_e47984);
        (assign32940_e47985, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32940_e47984)) / (assign32940_e47984 * assign32940_e47984))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32940_e47984)) / (assign32940_e47984 * assign32940_e47984))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32940_e47984)) / (assign32940_e47984 * assign32940_e47984))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32940_e47987;
        locals.var_sarg_dn6 = assign32940_e47987_d_n6;
        locals.var_sarg_dn7 = assign32940_e47987_d_n7;
        locals.var_sarg_dn12 = assign32940_e47987_d_n12;

        let (assign32950_e48003, assign32950_e48003_d_n6, assign32950_e48003_d_n7, assign32950_e48003_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 == 0.0)) {
        let assign32950_e48000: f64 = (-p.p184);
        let assign32950_e48001: f64 = (locals.var_arg__blk1057).powf(assign32950_e48000);
        (assign32950_e48001, if 0.0 == 0.0 && ((assign32950_e48000) as f64).is_finite() && ((assign32950_e48000) as f64).fract() == 0.0 { if assign32950_e48000 == 0.0 { 0.0 } else { (assign32950_e48000 * ((locals.var_arg__blk1057).powf(assign32950_e48000 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32950_e48001 * (assign32950_e48000 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32950_e48000) as f64).is_finite() && ((assign32950_e48000) as f64).fract() == 0.0 { if assign32950_e48000 == 0.0 { 0.0 } else { (assign32950_e48000 * ((locals.var_arg__blk1057).powf(assign32950_e48000 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32950_e48001 * (assign32950_e48000 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32950_e48000) as f64).is_finite() && ((assign32950_e48000) as f64).fract() == 0.0 { if assign32950_e48000 == 0.0 { 0.0 } else { (assign32950_e48000 * ((locals.var_arg__blk1057).powf(assign32950_e48000 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32950_e48001 * (assign32950_e48000 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32950_e48003;
        locals.var_sarg_dn6 = assign32950_e48003_d_n6;
        locals.var_sarg_dn7 = assign32950_e48003_d_n7;
        locals.var_sarg_dn12 = assign32950_e48003_d_n12;

        let (assign32960_e48027, assign32960_e48027_d_n0, assign32960_e48027_d_n2, assign32960_e48027_d_n6, assign32960_e48027_d_n7, assign32960_e48027_d_n10, assign32960_e48027_d_n11, assign32960_e48027_d_n12, assign32960_e48027_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign32960_e48014: f64 = (p.p187 * locals.var_czbdswg);
        let assign32960_e48018: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
        let assign32960_e48019: f64 = (1.0 - assign32960_e48018);
        let assign32960_e48020: f64 = (assign32960_e48014 * assign32960_e48019);
        let assign32960_e48023: f64 = (1.0 - p.p184);
        let assign32960_e48024: f64 = (assign32960_e48020 / assign32960_e48023);
        let assign32960_e48025: f64 = (locals.var_qbd + assign32960_e48024);
        (assign32960_e48025, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32960_e48014 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32960_e48023)), (locals.var_qbd_dn7 + ((assign32960_e48014 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32960_e48023)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32960_e48014 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32960_e48023)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32960_e48027;
        locals.var_qbd_dn0 = assign32960_e48027_d_n0;
        locals.var_qbd_dn2 = assign32960_e48027_d_n2;
        locals.var_qbd_dn6 = assign32960_e48027_d_n6;
        locals.var_qbd_dn7 = assign32960_e48027_d_n7;
        locals.var_qbd_dn10 = assign32960_e48027_d_n10;
        locals.var_qbd_dn11 = assign32960_e48027_d_n11;
        locals.var_qbd_dn12 = assign32960_e48027_d_n12;
        locals.var_qbd_dn17 = assign32960_e48027_d_n17;

        let (assign32970_e48040, assign32970_e48040_d_n6, assign32970_e48040_d_n7, assign32970_e48040_d_n10, assign32970_e48040_d_n12,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
        let assign32970_e48036: f64 = (locals.var_czbd + locals.var_czbdsw);
        let assign32970_e48038: f64 = (assign32970_e48036 + locals.var_czbdswg);
        (assign32970_e48038, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12,)
    }
};
        locals.var_t1__blk1032 = assign32970_e48040;
        locals.var_t1__blk1032_dn6 = assign32970_e48040_d_n6;
        locals.var_t1__blk1032_dn7 = assign32970_e48040_d_n7;
        locals.var_t1__blk1032_dn10 = assign32970_e48040_d_n10;
        locals.var_t1__blk1032_dn12 = assign32970_e48040_d_n12;

        let (assign32980_e48065, assign32980_e48065_d_n0, assign32980_e48065_d_n2, assign32980_e48065_d_n6, assign32980_e48065_d_n7, assign32980_e48065_d_n10, assign32980_e48065_d_n11, assign32980_e48065_d_n12, assign32980_e48065_d_n17,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
        let assign32980_e48049: f64 = (locals.var_czbd * p.p182);
        let assign32980_e48051: f64 = (assign32980_e48049 / p.p185);
        let assign32980_e48054: f64 = (locals.var_czbdsw * p.p183);
        let assign32980_e48056: f64 = (assign32980_e48054 / p.p186);
        let assign32980_e48057: f64 = (assign32980_e48051 + assign32980_e48056);
        let assign32980_e48060: f64 = (locals.var_czbdswg * p.p184);
        let assign32980_e48062: f64 = (assign32980_e48060 / p.p187);
        let assign32980_e48063: f64 = (assign32980_e48057 + assign32980_e48062);
        (assign32980_e48063, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1033, locals.var_t2__blk1033_dn0, locals.var_t2__blk1033_dn2, locals.var_t2__blk1033_dn6, locals.var_t2__blk1033_dn7, locals.var_t2__blk1033_dn10, locals.var_t2__blk1033_dn11, locals.var_t2__blk1033_dn12, locals.var_t2__blk1033_dn17,)
    }
};
        locals.var_t2__blk1033 = assign32980_e48065;
        locals.var_t2__blk1033_dn0 = assign32980_e48065_d_n0;
        locals.var_t2__blk1033_dn2 = assign32980_e48065_d_n2;
        locals.var_t2__blk1033_dn6 = assign32980_e48065_d_n6;
        locals.var_t2__blk1033_dn7 = assign32980_e48065_d_n7;
        locals.var_t2__blk1033_dn10 = assign32980_e48065_d_n10;
        locals.var_t2__blk1033_dn11 = assign32980_e48065_d_n11;
        locals.var_t2__blk1033_dn12 = assign32980_e48065_d_n12;
        locals.var_t2__blk1033_dn17 = assign32980_e48065_d_n17;

        let (assign32990_e48082, assign32990_e48082_d_n0, assign32990_e48082_d_n2, assign32990_e48082_d_n6, assign32990_e48082_d_n7, assign32990_e48082_d_n10, assign32990_e48082_d_n11, assign32990_e48082_d_n12, assign32990_e48082_d_n17,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
        let assign32990_e48076: f64 = (locals.var_vbdj * 0.5);
        let assign32990_e48078: f64 = (assign32990_e48076 * locals.var_t2__blk1033);
        let assign32990_e48079: f64 = (locals.var_t1__blk1032 + assign32990_e48078);
        let assign32990_e48080: f64 = (locals.var_vbdj * assign32990_e48079);
        (assign32990_e48080, (locals.var_vbdj * (assign32990_e48076 * locals.var_t2__blk1033_dn0)), (locals.var_vbdj * (assign32990_e48076 * locals.var_t2__blk1033_dn2)), ((locals.var_vbdj_dn6 * assign32990_e48079) + (locals.var_vbdj * (locals.var_t1__blk1032_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1033) + (assign32990_e48076 * locals.var_t2__blk1033_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1032_dn7 + (assign32990_e48076 * locals.var_t2__blk1033_dn7))), (locals.var_vbdj * (locals.var_t1__blk1032_dn10 + (assign32990_e48076 * locals.var_t2__blk1033_dn10))), (locals.var_vbdj * (assign32990_e48076 * locals.var_t2__blk1033_dn11)), ((locals.var_vbdj_dn12 * assign32990_e48079) + (locals.var_vbdj * (locals.var_t1__blk1032_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1033) + (assign32990_e48076 * locals.var_t2__blk1033_dn12))))), (locals.var_vbdj * (assign32990_e48076 * locals.var_t2__blk1033_dn17)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32990_e48082;
        locals.var_qbd_dn0 = assign32990_e48082_d_n0;
        locals.var_qbd_dn2 = assign32990_e48082_d_n2;
        locals.var_qbd_dn6 = assign32990_e48082_d_n6;
        locals.var_qbd_dn7 = assign32990_e48082_d_n7;
        locals.var_qbd_dn10 = assign32990_e48082_d_n10;
        locals.var_qbd_dn11 = assign32990_e48082_d_n11;
        locals.var_qbd_dn12 = assign32990_e48082_d_n12;
        locals.var_qbd_dn17 = assign32990_e48082_d_n17;

        let (assign33000_e48091,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) {
        let assign33000_e48089: f64 = (p.p181 * p.p4);
        (assign33000_e48089,)
    } else {
        (locals.var_czbdswg,)
    }
};
        locals.var_czbdswg = assign33000_e48091;

        let assign33010_e48094: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign33010_e48094;

        let assign33020_e48097: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign33020_e48097;

        let (assign33030_e48112, assign33030_e48112_d_n6, assign33030_e48112_d_n7, assign33030_e48112_d_n12,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 != 0.0)) {
        let assign33030_e48109: f64 = (locals.var_vbdj / p.p185);
        let assign33030_e48110: f64 = (1.0 - assign33030_e48109);
        (assign33030_e48110, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12,)
    }
};
        locals.var_arg__blk1057 = assign33030_e48112;
        locals.var_arg__blk1057_dn6 = assign33030_e48112_d_n6;
        locals.var_arg__blk1057_dn7 = assign33030_e48112_d_n7;
        locals.var_arg__blk1057_dn12 = assign33030_e48112_d_n12;

        let assign33040_e48115: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1085 = assign33040_e48115;

        let (assign33050_e48131, assign33050_e48131_d_n6, assign33050_e48131_d_n7, assign33050_e48131_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 != 0.0)) {
        let assign33050_e48128: f64 = (locals.var_arg__blk1057).sqrt();
        let assign33050_e48129: f64 = (1.0 / assign33050_e48128);
        (assign33050_e48129, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign33050_e48128)) / (assign33050_e48128 * assign33050_e48128))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign33050_e48128)) / (assign33050_e48128 * assign33050_e48128))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign33050_e48128)) / (assign33050_e48128 * assign33050_e48128))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33050_e48131;
        locals.var_sarg_dn6 = assign33050_e48131_d_n6;
        locals.var_sarg_dn7 = assign33050_e48131_d_n7;
        locals.var_sarg_dn12 = assign33050_e48131_d_n12;

        let (assign33060_e48148, assign33060_e48148_d_n6, assign33060_e48148_d_n7, assign33060_e48148_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 == 0.0)) {
        let assign33060_e48145: f64 = (-p.p182);
        let assign33060_e48146: f64 = (locals.var_arg__blk1057).powf(assign33060_e48145);
        (assign33060_e48146, if 0.0 == 0.0 && ((assign33060_e48145) as f64).is_finite() && ((assign33060_e48145) as f64).fract() == 0.0 { if assign33060_e48145 == 0.0 { 0.0 } else { (assign33060_e48145 * ((locals.var_arg__blk1057).powf(assign33060_e48145 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign33060_e48146 * (assign33060_e48145 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33060_e48145) as f64).is_finite() && ((assign33060_e48145) as f64).fract() == 0.0 { if assign33060_e48145 == 0.0 { 0.0 } else { (assign33060_e48145 * ((locals.var_arg__blk1057).powf(assign33060_e48145 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign33060_e48146 * (assign33060_e48145 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33060_e48145) as f64).is_finite() && ((assign33060_e48145) as f64).fract() == 0.0 { if assign33060_e48145 == 0.0 { 0.0 } else { (assign33060_e48145 * ((locals.var_arg__blk1057).powf(assign33060_e48145 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign33060_e48146 * (assign33060_e48145 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33060_e48148;
        locals.var_sarg_dn6 = assign33060_e48148_d_n6;
        locals.var_sarg_dn7 = assign33060_e48148_d_n7;
        locals.var_sarg_dn12 = assign33060_e48148_d_n12;

        let (assign33070_e48171, assign33070_e48171_d_n0, assign33070_e48171_d_n2, assign33070_e48171_d_n6, assign33070_e48171_d_n7, assign33070_e48171_d_n10, assign33070_e48171_d_n11, assign33070_e48171_d_n12, assign33070_e48171_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 != 0.0)) {
        let assign33070_e48159: f64 = (p.p185 * locals.var_czbd);
        let assign33070_e48163: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
        let assign33070_e48164: f64 = (1.0 - assign33070_e48163);
        let assign33070_e48165: f64 = (assign33070_e48159 * assign33070_e48164);
        let assign33070_e48168: f64 = (1.0 - p.p182);
        let assign33070_e48169: f64 = (assign33070_e48165 / assign33070_e48168);
        (assign33070_e48169, 0.0, 0.0, ((assign33070_e48159 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign33070_e48168), ((assign33070_e48159 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign33070_e48168), 0.0, 0.0, ((assign33070_e48159 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign33070_e48168), 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33070_e48171;
        locals.var_qbd_dn0 = assign33070_e48171_d_n0;
        locals.var_qbd_dn2 = assign33070_e48171_d_n2;
        locals.var_qbd_dn6 = assign33070_e48171_d_n6;
        locals.var_qbd_dn7 = assign33070_e48171_d_n7;
        locals.var_qbd_dn10 = assign33070_e48171_d_n10;
        locals.var_qbd_dn11 = assign33070_e48171_d_n11;
        locals.var_qbd_dn12 = assign33070_e48171_d_n12;
        locals.var_qbd_dn17 = assign33070_e48171_d_n17;

        let (assign33080_e48183, assign33080_e48183_d_n0, assign33080_e48183_d_n2, assign33080_e48183_d_n6, assign33080_e48183_d_n7, assign33080_e48183_d_n10, assign33080_e48183_d_n11, assign33080_e48183_d_n12, assign33080_e48183_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33080_e48183;
        locals.var_qbd_dn0 = assign33080_e48183_d_n0;
        locals.var_qbd_dn2 = assign33080_e48183_d_n2;
        locals.var_qbd_dn6 = assign33080_e48183_d_n6;
        locals.var_qbd_dn7 = assign33080_e48183_d_n7;
        locals.var_qbd_dn10 = assign33080_e48183_d_n10;
        locals.var_qbd_dn11 = assign33080_e48183_d_n11;
        locals.var_qbd_dn12 = assign33080_e48183_d_n12;
        locals.var_qbd_dn17 = assign33080_e48183_d_n17;

        let assign33090_e48186: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1086 = assign33090_e48186;

        let (assign33100_e48201, assign33100_e48201_d_n6, assign33100_e48201_d_n7, assign33100_e48201_d_n12,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1086 != 0.0)) {
        let assign33100_e48198: f64 = (locals.var_vbdj / p.p187);
        let assign33100_e48199: f64 = (1.0 - assign33100_e48198);
        (assign33100_e48199, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12,)
    }
};
        locals.var_arg__blk1057 = assign33100_e48201;
        locals.var_arg__blk1057_dn6 = assign33100_e48201_d_n6;
        locals.var_arg__blk1057_dn7 = assign33100_e48201_d_n7;
        locals.var_arg__blk1057_dn12 = assign33100_e48201_d_n12;

        let assign33110_e48204: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1087 = assign33110_e48204;

        let (assign33120_e48220, assign33120_e48220_d_n6, assign33120_e48220_d_n7, assign33120_e48220_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1086 != 0.0)) && (locals.var_guard1087 != 0.0)) {
        let assign33120_e48217: f64 = (locals.var_arg__blk1057).sqrt();
        let assign33120_e48218: f64 = (1.0 / assign33120_e48217);
        (assign33120_e48218, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign33120_e48217)) / (assign33120_e48217 * assign33120_e48217))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign33120_e48217)) / (assign33120_e48217 * assign33120_e48217))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign33120_e48217)) / (assign33120_e48217 * assign33120_e48217))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33120_e48220;
        locals.var_sarg_dn6 = assign33120_e48220_d_n6;
        locals.var_sarg_dn7 = assign33120_e48220_d_n7;
        locals.var_sarg_dn12 = assign33120_e48220_d_n12;

        let (assign33130_e48237, assign33130_e48237_d_n6, assign33130_e48237_d_n7, assign33130_e48237_d_n12,) = {
    if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1086 != 0.0)) && (locals.var_guard1087 == 0.0)) {
        let assign33130_e48234: f64 = (-p.p184);
        let assign33130_e48235: f64 = (locals.var_arg__blk1057).powf(assign33130_e48234);
        (assign33130_e48235, if 0.0 == 0.0 && ((assign33130_e48234) as f64).is_finite() && ((assign33130_e48234) as f64).fract() == 0.0 { if assign33130_e48234 == 0.0 { 0.0 } else { (assign33130_e48234 * ((locals.var_arg__blk1057).powf(assign33130_e48234 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign33130_e48235 * (assign33130_e48234 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33130_e48234) as f64).is_finite() && ((assign33130_e48234) as f64).fract() == 0.0 { if assign33130_e48234 == 0.0 { 0.0 } else { (assign33130_e48234 * ((locals.var_arg__blk1057).powf(assign33130_e48234 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign33130_e48235 * (assign33130_e48234 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33130_e48234) as f64).is_finite() && ((assign33130_e48234) as f64).fract() == 0.0 { if assign33130_e48234 == 0.0 { 0.0 } else { (assign33130_e48234 * ((locals.var_arg__blk1057).powf(assign33130_e48234 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign33130_e48235 * (assign33130_e48234 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33130_e48237;
        locals.var_sarg_dn6 = assign33130_e48237_d_n6;
        locals.var_sarg_dn7 = assign33130_e48237_d_n7;
        locals.var_sarg_dn12 = assign33130_e48237_d_n12;

        let (assign33140_e48262, assign33140_e48262_d_n0, assign33140_e48262_d_n2, assign33140_e48262_d_n6, assign33140_e48262_d_n7, assign33140_e48262_d_n10, assign33140_e48262_d_n11, assign33140_e48262_d_n12, assign33140_e48262_d_n17,) = {
    if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1086 != 0.0)) {
        let assign33140_e48249: f64 = (p.p187 * locals.var_czbdswg);
        let assign33140_e48253: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
        let assign33140_e48254: f64 = (1.0 - assign33140_e48253);
        let assign33140_e48255: f64 = (assign33140_e48249 * assign33140_e48254);
        let assign33140_e48258: f64 = (1.0 - p.p184);
        let assign33140_e48259: f64 = (assign33140_e48255 / assign33140_e48258);
        let assign33140_e48260: f64 = (locals.var_qbd + assign33140_e48259);
        (assign33140_e48260, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign33140_e48249 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign33140_e48258)), (locals.var_qbd_dn7 + ((assign33140_e48249 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign33140_e48258)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign33140_e48249 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign33140_e48258)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33140_e48262;
        locals.var_qbd_dn0 = assign33140_e48262_d_n0;
        locals.var_qbd_dn2 = assign33140_e48262_d_n2;
        locals.var_qbd_dn6 = assign33140_e48262_d_n6;
        locals.var_qbd_dn7 = assign33140_e48262_d_n7;
        locals.var_qbd_dn10 = assign33140_e48262_d_n10;
        locals.var_qbd_dn11 = assign33140_e48262_d_n11;
        locals.var_qbd_dn12 = assign33140_e48262_d_n12;
        locals.var_qbd_dn17 = assign33140_e48262_d_n17;

        let (assign33150_e48274, assign33150_e48274_d_n6, assign33150_e48274_d_n7, assign33150_e48274_d_n10, assign33150_e48274_d_n12,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 == 0.0)) {
        let assign33150_e48272: f64 = (locals.var_czbd + locals.var_czbdswg);
        (assign33150_e48272, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12,)
    }
};
        locals.var_t1__blk1032 = assign33150_e48274;
        locals.var_t1__blk1032_dn6 = assign33150_e48274_d_n6;
        locals.var_t1__blk1032_dn7 = assign33150_e48274_d_n7;
        locals.var_t1__blk1032_dn10 = assign33150_e48274_d_n10;
        locals.var_t1__blk1032_dn12 = assign33150_e48274_d_n12;

        let (assign33160_e48294, assign33160_e48294_d_n0, assign33160_e48294_d_n2, assign33160_e48294_d_n6, assign33160_e48294_d_n7, assign33160_e48294_d_n10, assign33160_e48294_d_n11, assign33160_e48294_d_n12, assign33160_e48294_d_n17,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 == 0.0)) {
        let assign33160_e48284: f64 = (locals.var_czbd * p.p182);
        let assign33160_e48286: f64 = (assign33160_e48284 / p.p185);
        let assign33160_e48289: f64 = (locals.var_czbdswg * p.p184);
        let assign33160_e48291: f64 = (assign33160_e48289 / p.p187);
        let assign33160_e48292: f64 = (assign33160_e48286 + assign33160_e48291);
        (assign33160_e48292, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1033, locals.var_t2__blk1033_dn0, locals.var_t2__blk1033_dn2, locals.var_t2__blk1033_dn6, locals.var_t2__blk1033_dn7, locals.var_t2__blk1033_dn10, locals.var_t2__blk1033_dn11, locals.var_t2__blk1033_dn12, locals.var_t2__blk1033_dn17,)
    }
};
        locals.var_t2__blk1033 = assign33160_e48294;
        locals.var_t2__blk1033_dn0 = assign33160_e48294_d_n0;
        locals.var_t2__blk1033_dn2 = assign33160_e48294_d_n2;
        locals.var_t2__blk1033_dn6 = assign33160_e48294_d_n6;
        locals.var_t2__blk1033_dn7 = assign33160_e48294_d_n7;
        locals.var_t2__blk1033_dn10 = assign33160_e48294_d_n10;
        locals.var_t2__blk1033_dn11 = assign33160_e48294_d_n11;
        locals.var_t2__blk1033_dn12 = assign33160_e48294_d_n12;
        locals.var_t2__blk1033_dn17 = assign33160_e48294_d_n17;

        let (assign33170_e48312, assign33170_e48312_d_n0, assign33170_e48312_d_n2, assign33170_e48312_d_n6, assign33170_e48312_d_n7, assign33170_e48312_d_n10, assign33170_e48312_d_n11, assign33170_e48312_d_n12, assign33170_e48312_d_n17,) = {
    if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 == 0.0)) {
        let assign33170_e48306: f64 = (locals.var_vbdj * 0.5);
        let assign33170_e48308: f64 = (assign33170_e48306 * locals.var_t2__blk1033);
        let assign33170_e48309: f64 = (locals.var_t1__blk1032 + assign33170_e48308);
        let assign33170_e48310: f64 = (locals.var_vbdj * assign33170_e48309);
        (assign33170_e48310, (locals.var_vbdj * (assign33170_e48306 * locals.var_t2__blk1033_dn0)), (locals.var_vbdj * (assign33170_e48306 * locals.var_t2__blk1033_dn2)), ((locals.var_vbdj_dn6 * assign33170_e48309) + (locals.var_vbdj * (locals.var_t1__blk1032_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1033) + (assign33170_e48306 * locals.var_t2__blk1033_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1032_dn7 + (assign33170_e48306 * locals.var_t2__blk1033_dn7))), (locals.var_vbdj * (locals.var_t1__blk1032_dn10 + (assign33170_e48306 * locals.var_t2__blk1033_dn10))), (locals.var_vbdj * (assign33170_e48306 * locals.var_t2__blk1033_dn11)), ((locals.var_vbdj_dn12 * assign33170_e48309) + (locals.var_vbdj * (locals.var_t1__blk1032_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1033) + (assign33170_e48306 * locals.var_t2__blk1033_dn12))))), (locals.var_vbdj * (assign33170_e48306 * locals.var_t2__blk1033_dn17)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33170_e48312;
        locals.var_qbd_dn0 = assign33170_e48312_d_n0;
        locals.var_qbd_dn2 = assign33170_e48312_d_n2;
        locals.var_qbd_dn6 = assign33170_e48312_d_n6;
        locals.var_qbd_dn7 = assign33170_e48312_d_n7;
        locals.var_qbd_dn10 = assign33170_e48312_d_n10;
        locals.var_qbd_dn11 = assign33170_e48312_d_n11;
        locals.var_qbd_dn12 = assign33170_e48312_d_n12;
        locals.var_qbd_dn17 = assign33170_e48312_d_n17;

        let assign33180_e48315: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1088 = assign33180_e48315;

        let (assign33190_e48328, assign33190_e48328_d_n0, assign33190_e48328_d_n2, assign33190_e48328_d_n6, assign33190_e48328_d_n7, assign33190_e48328_d_n10, assign33190_e48328_d_n11, assign33190_e48328_d_n12, assign33190_e48328_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
        let assign33190_e48320: f64 = (-1.6021918e-19);
        let assign33190_e48322: f64 = (assign33190_e48320 * locals.var_uc_nsubs);
        let assign33190_e48324: f64 = (assign33190_e48322 * locals.var_xp_max);
        let assign33190_e48326: f64 = (assign33190_e48324 * p.p3);
        (assign33190_e48326, (((assign33190_e48320 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p3),)
    } else {
        (locals.var_qbs_max, locals.var_qbs_max_dn0, locals.var_qbs_max_dn2, locals.var_qbs_max_dn6, locals.var_qbs_max_dn7, locals.var_qbs_max_dn10, locals.var_qbs_max_dn11, locals.var_qbs_max_dn12, locals.var_qbs_max_dn17,)
    }
};
        locals.var_qbs_max = assign33190_e48328;
        locals.var_qbs_max_dn0 = assign33190_e48328_d_n0;
        locals.var_qbs_max_dn2 = assign33190_e48328_d_n2;
        locals.var_qbs_max_dn6 = assign33190_e48328_d_n6;
        locals.var_qbs_max_dn7 = assign33190_e48328_d_n7;
        locals.var_qbs_max_dn10 = assign33190_e48328_d_n10;
        locals.var_qbs_max_dn11 = assign33190_e48328_d_n11;
        locals.var_qbs_max_dn12 = assign33190_e48328_d_n12;
        locals.var_qbs_max_dn17 = assign33190_e48328_d_n17;

        let (assign33200_e48337, assign33200_e48337_d_n0, assign33200_e48337_d_n2, assign33200_e48337_d_n6, assign33200_e48337_d_n7, assign33200_e48337_d_n10, assign33200_e48337_d_n11, assign33200_e48337_d_n12, assign33200_e48337_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
        let assign33200_e48334: f64 = (-locals.var_qbs_max);
        let assign33200_e48335: f64 = (0.001 * assign33200_e48334);
        (assign33200_e48335, (0.001 * (-locals.var_qbs_max_dn0)), (0.001 * (-locals.var_qbs_max_dn2)), (0.001 * (-locals.var_qbs_max_dn6)), (0.001 * (-locals.var_qbs_max_dn7)), (0.001 * (-locals.var_qbs_max_dn10)), (0.001 * (-locals.var_qbs_max_dn11)), (0.001 * (-locals.var_qbs_max_dn12)), (0.001 * (-locals.var_qbs_max_dn17)),)
    } else {
        (locals.var_dlt_qbs, locals.var_dlt_qbs_dn0, locals.var_dlt_qbs_dn2, locals.var_dlt_qbs_dn6, locals.var_dlt_qbs_dn7, locals.var_dlt_qbs_dn10, locals.var_dlt_qbs_dn11, locals.var_dlt_qbs_dn12, locals.var_dlt_qbs_dn17,)
    }
};
        locals.var_dlt_qbs = assign33200_e48337;
        locals.var_dlt_qbs_dn0 = assign33200_e48337_d_n0;
        locals.var_dlt_qbs_dn2 = assign33200_e48337_d_n2;
        locals.var_dlt_qbs_dn6 = assign33200_e48337_d_n6;
        locals.var_dlt_qbs_dn7 = assign33200_e48337_d_n7;
        locals.var_dlt_qbs_dn10 = assign33200_e48337_d_n10;
        locals.var_dlt_qbs_dn11 = assign33200_e48337_d_n11;
        locals.var_dlt_qbs_dn12 = assign33200_e48337_d_n12;
        locals.var_dlt_qbs_dn17 = assign33200_e48337_d_n17;

        let (assign33210_e48349, assign33210_e48349_d_n0, assign33210_e48349_d_n2, assign33210_e48349_d_n6, assign33210_e48349_d_n7, assign33210_e48349_d_n10, assign33210_e48349_d_n11, assign33210_e48349_d_n12, assign33210_e48349_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
        let assign33210_e48342: f64 = (-locals.var_qbs_max);
        let assign33210_e48344: f64 = (-locals.var_qbs);
        let assign33210_e48345: f64 = (assign33210_e48342 - assign33210_e48344);
        let assign33210_e48347: f64 = (assign33210_e48345 - locals.var_dlt_qbs);
        (assign33210_e48347, (((-locals.var_qbs_max_dn0) - (-locals.var_qbs_dn0)) - locals.var_dlt_qbs_dn0), (((-locals.var_qbs_max_dn2) - (-locals.var_qbs_dn2)) - locals.var_dlt_qbs_dn2), (((-locals.var_qbs_max_dn6) - (-locals.var_qbs_dn6)) - locals.var_dlt_qbs_dn6), (((-locals.var_qbs_max_dn7) - (-locals.var_qbs_dn7)) - locals.var_dlt_qbs_dn7), (((-locals.var_qbs_max_dn10) - (-locals.var_qbs_dn10)) - locals.var_dlt_qbs_dn10), (((-locals.var_qbs_max_dn11) - (-locals.var_qbs_dn11)) - locals.var_dlt_qbs_dn11), (((-locals.var_qbs_max_dn12) - (-locals.var_qbs_dn12)) - locals.var_dlt_qbs_dn12), (((-locals.var_qbs_max_dn17) - (-locals.var_qbs_dn17)) - locals.var_dlt_qbs_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign33210_e48349;
        locals.var_tmf1_dn0 = assign33210_e48349_d_n0;
        locals.var_tmf1_dn2 = assign33210_e48349_d_n2;
        locals.var_tmf1_dn6 = assign33210_e48349_d_n6;
        locals.var_tmf1_dn7 = assign33210_e48349_d_n7;
        locals.var_tmf1_dn10 = assign33210_e48349_d_n10;
        locals.var_tmf1_dn11 = assign33210_e48349_d_n11;
        locals.var_tmf1_dn12 = assign33210_e48349_d_n12;
        locals.var_tmf1_dn17 = assign33210_e48349_d_n17;

        let (assign33220_e48360, assign33220_e48360_d_n0, assign33220_e48360_d_n2, assign33220_e48360_d_n6, assign33220_e48360_d_n7, assign33220_e48360_d_n10, assign33220_e48360_d_n11, assign33220_e48360_d_n12, assign33220_e48360_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
        let assign33220_e48355: f64 = (-locals.var_qbs_max);
        let assign33220_e48356: f64 = (4.0 * assign33220_e48355);
        let assign33220_e48358: f64 = (assign33220_e48356 * locals.var_dlt_qbs);
        (assign33220_e48358, (((4.0 * (-locals.var_qbs_max_dn0)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn0)), (((4.0 * (-locals.var_qbs_max_dn2)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn2)), (((4.0 * (-locals.var_qbs_max_dn6)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn6)), (((4.0 * (-locals.var_qbs_max_dn7)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn7)), (((4.0 * (-locals.var_qbs_max_dn10)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn10)), (((4.0 * (-locals.var_qbs_max_dn11)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn11)), (((4.0 * (-locals.var_qbs_max_dn12)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn12)), (((4.0 * (-locals.var_qbs_max_dn17)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33220_e48360;
        locals.var_tmf2_dn0 = assign33220_e48360_d_n0;
        locals.var_tmf2_dn2 = assign33220_e48360_d_n2;
        locals.var_tmf2_dn6 = assign33220_e48360_d_n6;
        locals.var_tmf2_dn7 = assign33220_e48360_d_n7;
        locals.var_tmf2_dn10 = assign33220_e48360_d_n10;
        locals.var_tmf2_dn11 = assign33220_e48360_d_n11;
        locals.var_tmf2_dn12 = assign33220_e48360_d_n12;
        locals.var_tmf2_dn17 = assign33220_e48360_d_n17;

        let (assign33230_e48372, assign33230_e48372_d_n0, assign33230_e48372_d_n2, assign33230_e48372_d_n6, assign33230_e48372_d_n7, assign33230_e48372_d_n10, assign33230_e48372_d_n11, assign33230_e48372_d_n12, assign33230_e48372_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
        let (assign33230_e48370, assign33230_e48370_d_n0, assign33230_e48370_d_n2, assign33230_e48370_d_n6, assign33230_e48370_d_n7, assign33230_e48370_d_n10, assign33230_e48370_d_n11, assign33230_e48370_d_n12, assign33230_e48370_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign33230_e48369: f64 = (-locals.var_tmf2);
                (assign33230_e48369, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign33230_e48370, assign33230_e48370_d_n0, assign33230_e48370_d_n2, assign33230_e48370_d_n6, assign33230_e48370_d_n7, assign33230_e48370_d_n10, assign33230_e48370_d_n11, assign33230_e48370_d_n12, assign33230_e48370_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33230_e48372;
        locals.var_tmf2_dn0 = assign33230_e48372_d_n0;
        locals.var_tmf2_dn2 = assign33230_e48372_d_n2;
        locals.var_tmf2_dn6 = assign33230_e48372_d_n6;
        locals.var_tmf2_dn7 = assign33230_e48372_d_n7;
        locals.var_tmf2_dn10 = assign33230_e48372_d_n10;
        locals.var_tmf2_dn11 = assign33230_e48372_d_n11;
        locals.var_tmf2_dn12 = assign33230_e48372_d_n12;
        locals.var_tmf2_dn17 = assign33230_e48372_d_n17;

    }

    pub(super) fn stamp_transient_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33240_e48383, assign33240_e48383_d_n0, assign33240_e48383_d_n2, assign33240_e48383_d_n6, assign33240_e48383_d_n7, assign33240_e48383_d_n10, assign33240_e48383_d_n11, assign33240_e48383_d_n12, assign33240_e48383_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
        let assign33240_e48378: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign33240_e48380: f64 = (assign33240_e48378 + locals.var_tmf2);
        let assign33240_e48381: f64 = (assign33240_e48380).sqrt();
        (assign33240_e48381, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33240_e48381)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33240_e48383;
        locals.var_tmf2_dn0 = assign33240_e48383_d_n0;
        locals.var_tmf2_dn2 = assign33240_e48383_d_n2;
        locals.var_tmf2_dn6 = assign33240_e48383_d_n6;
        locals.var_tmf2_dn7 = assign33240_e48383_d_n7;
        locals.var_tmf2_dn10 = assign33240_e48383_d_n10;
        locals.var_tmf2_dn11 = assign33240_e48383_d_n11;
        locals.var_tmf2_dn12 = assign33240_e48383_d_n12;
        locals.var_tmf2_dn17 = assign33240_e48383_d_n17;

        let (assign33250_e48396, assign33250_e48396_d_n0, assign33250_e48396_d_n2, assign33250_e48396_d_n6, assign33250_e48396_d_n7, assign33250_e48396_d_n10, assign33250_e48396_d_n11, assign33250_e48396_d_n12, assign33250_e48396_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
        let assign33250_e48388: f64 = (-locals.var_qbs_max);
        let assign33250_e48392: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign33250_e48393: f64 = (0.5 * assign33250_e48392);
        let assign33250_e48394: f64 = (assign33250_e48388 - assign33250_e48393);
        (assign33250_e48394, ((-locals.var_qbs_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbs_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbs_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbs_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbs_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbs_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbs_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbs_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign33250_e48396;
        locals.var_qbs_dn0 = assign33250_e48396_d_n0;
        locals.var_qbs_dn2 = assign33250_e48396_d_n2;
        locals.var_qbs_dn6 = assign33250_e48396_d_n6;
        locals.var_qbs_dn7 = assign33250_e48396_d_n7;
        locals.var_qbs_dn10 = assign33250_e48396_d_n10;
        locals.var_qbs_dn11 = assign33250_e48396_d_n11;
        locals.var_qbs_dn12 = assign33250_e48396_d_n12;
        locals.var_qbs_dn17 = assign33250_e48396_d_n17;

        let (assign33260_e48405, assign33260_e48405_d_n0, assign33260_e48405_d_n2, assign33260_e48405_d_n6, assign33260_e48405_d_n7, assign33260_e48405_d_n10, assign33260_e48405_d_n11, assign33260_e48405_d_n12, assign33260_e48405_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
        let assign33260_e48402: f64 = (-1.0);
        let assign33260_e48403: f64 = (locals.var_qbs * assign33260_e48402);
        (assign33260_e48403, (locals.var_qbs_dn0 * assign33260_e48402), (locals.var_qbs_dn2 * assign33260_e48402), (locals.var_qbs_dn6 * assign33260_e48402), (locals.var_qbs_dn7 * assign33260_e48402), (locals.var_qbs_dn10 * assign33260_e48402), (locals.var_qbs_dn11 * assign33260_e48402), (locals.var_qbs_dn12 * assign33260_e48402), (locals.var_qbs_dn17 * assign33260_e48402),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign33260_e48405;
        locals.var_qbs_dn0 = assign33260_e48405_d_n0;
        locals.var_qbs_dn2 = assign33260_e48405_d_n2;
        locals.var_qbs_dn6 = assign33260_e48405_d_n6;
        locals.var_qbs_dn7 = assign33260_e48405_d_n7;
        locals.var_qbs_dn10 = assign33260_e48405_d_n10;
        locals.var_qbs_dn11 = assign33260_e48405_d_n11;
        locals.var_qbs_dn12 = assign33260_e48405_d_n12;
        locals.var_qbs_dn17 = assign33260_e48405_d_n17;

        let assign33270_e48408: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1089 = assign33270_e48408;

        let (assign33280_e48421, assign33280_e48421_d_n0, assign33280_e48421_d_n2, assign33280_e48421_d_n6, assign33280_e48421_d_n7, assign33280_e48421_d_n10, assign33280_e48421_d_n11, assign33280_e48421_d_n12, assign33280_e48421_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
        let assign33280_e48413: f64 = (-1.6021918e-19);
        let assign33280_e48415: f64 = (assign33280_e48413 * locals.var_uc_nsubs);
        let assign33280_e48417: f64 = (assign33280_e48415 * locals.var_xp_max);
        let assign33280_e48419: f64 = (assign33280_e48417 * p.p2);
        (assign33280_e48419, (((assign33280_e48413 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p2),)
    } else {
        (locals.var_qbd_max, locals.var_qbd_max_dn0, locals.var_qbd_max_dn2, locals.var_qbd_max_dn6, locals.var_qbd_max_dn7, locals.var_qbd_max_dn10, locals.var_qbd_max_dn11, locals.var_qbd_max_dn12, locals.var_qbd_max_dn17,)
    }
};
        locals.var_qbd_max = assign33280_e48421;
        locals.var_qbd_max_dn0 = assign33280_e48421_d_n0;
        locals.var_qbd_max_dn2 = assign33280_e48421_d_n2;
        locals.var_qbd_max_dn6 = assign33280_e48421_d_n6;
        locals.var_qbd_max_dn7 = assign33280_e48421_d_n7;
        locals.var_qbd_max_dn10 = assign33280_e48421_d_n10;
        locals.var_qbd_max_dn11 = assign33280_e48421_d_n11;
        locals.var_qbd_max_dn12 = assign33280_e48421_d_n12;
        locals.var_qbd_max_dn17 = assign33280_e48421_d_n17;

        let (assign33290_e48430, assign33290_e48430_d_n0, assign33290_e48430_d_n2, assign33290_e48430_d_n6, assign33290_e48430_d_n7, assign33290_e48430_d_n10, assign33290_e48430_d_n11, assign33290_e48430_d_n12, assign33290_e48430_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
        let assign33290_e48427: f64 = (-locals.var_qbd_max);
        let assign33290_e48428: f64 = (0.001 * assign33290_e48427);
        (assign33290_e48428, (0.001 * (-locals.var_qbd_max_dn0)), (0.001 * (-locals.var_qbd_max_dn2)), (0.001 * (-locals.var_qbd_max_dn6)), (0.001 * (-locals.var_qbd_max_dn7)), (0.001 * (-locals.var_qbd_max_dn10)), (0.001 * (-locals.var_qbd_max_dn11)), (0.001 * (-locals.var_qbd_max_dn12)), (0.001 * (-locals.var_qbd_max_dn17)),)
    } else {
        (locals.var_dlt_qbd, locals.var_dlt_qbd_dn0, locals.var_dlt_qbd_dn2, locals.var_dlt_qbd_dn6, locals.var_dlt_qbd_dn7, locals.var_dlt_qbd_dn10, locals.var_dlt_qbd_dn11, locals.var_dlt_qbd_dn12, locals.var_dlt_qbd_dn17,)
    }
};
        locals.var_dlt_qbd = assign33290_e48430;
        locals.var_dlt_qbd_dn0 = assign33290_e48430_d_n0;
        locals.var_dlt_qbd_dn2 = assign33290_e48430_d_n2;
        locals.var_dlt_qbd_dn6 = assign33290_e48430_d_n6;
        locals.var_dlt_qbd_dn7 = assign33290_e48430_d_n7;
        locals.var_dlt_qbd_dn10 = assign33290_e48430_d_n10;
        locals.var_dlt_qbd_dn11 = assign33290_e48430_d_n11;
        locals.var_dlt_qbd_dn12 = assign33290_e48430_d_n12;
        locals.var_dlt_qbd_dn17 = assign33290_e48430_d_n17;

        let (assign33300_e48442, assign33300_e48442_d_n0, assign33300_e48442_d_n2, assign33300_e48442_d_n6, assign33300_e48442_d_n7, assign33300_e48442_d_n10, assign33300_e48442_d_n11, assign33300_e48442_d_n12, assign33300_e48442_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
        let assign33300_e48435: f64 = (-locals.var_qbd_max);
        let assign33300_e48437: f64 = (-locals.var_qbd);
        let assign33300_e48438: f64 = (assign33300_e48435 - assign33300_e48437);
        let assign33300_e48440: f64 = (assign33300_e48438 - locals.var_dlt_qbd);
        (assign33300_e48440, (((-locals.var_qbd_max_dn0) - (-locals.var_qbd_dn0)) - locals.var_dlt_qbd_dn0), (((-locals.var_qbd_max_dn2) - (-locals.var_qbd_dn2)) - locals.var_dlt_qbd_dn2), (((-locals.var_qbd_max_dn6) - (-locals.var_qbd_dn6)) - locals.var_dlt_qbd_dn6), (((-locals.var_qbd_max_dn7) - (-locals.var_qbd_dn7)) - locals.var_dlt_qbd_dn7), (((-locals.var_qbd_max_dn10) - (-locals.var_qbd_dn10)) - locals.var_dlt_qbd_dn10), (((-locals.var_qbd_max_dn11) - (-locals.var_qbd_dn11)) - locals.var_dlt_qbd_dn11), (((-locals.var_qbd_max_dn12) - (-locals.var_qbd_dn12)) - locals.var_dlt_qbd_dn12), (((-locals.var_qbd_max_dn17) - (-locals.var_qbd_dn17)) - locals.var_dlt_qbd_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign33300_e48442;
        locals.var_tmf1_dn0 = assign33300_e48442_d_n0;
        locals.var_tmf1_dn2 = assign33300_e48442_d_n2;
        locals.var_tmf1_dn6 = assign33300_e48442_d_n6;
        locals.var_tmf1_dn7 = assign33300_e48442_d_n7;
        locals.var_tmf1_dn10 = assign33300_e48442_d_n10;
        locals.var_tmf1_dn11 = assign33300_e48442_d_n11;
        locals.var_tmf1_dn12 = assign33300_e48442_d_n12;
        locals.var_tmf1_dn17 = assign33300_e48442_d_n17;

        let (assign33310_e48453, assign33310_e48453_d_n0, assign33310_e48453_d_n2, assign33310_e48453_d_n6, assign33310_e48453_d_n7, assign33310_e48453_d_n10, assign33310_e48453_d_n11, assign33310_e48453_d_n12, assign33310_e48453_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
        let assign33310_e48448: f64 = (-locals.var_qbd_max);
        let assign33310_e48449: f64 = (4.0 * assign33310_e48448);
        let assign33310_e48451: f64 = (assign33310_e48449 * locals.var_dlt_qbd);
        (assign33310_e48451, (((4.0 * (-locals.var_qbd_max_dn0)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn0)), (((4.0 * (-locals.var_qbd_max_dn2)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn2)), (((4.0 * (-locals.var_qbd_max_dn6)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn6)), (((4.0 * (-locals.var_qbd_max_dn7)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn7)), (((4.0 * (-locals.var_qbd_max_dn10)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn10)), (((4.0 * (-locals.var_qbd_max_dn11)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn11)), (((4.0 * (-locals.var_qbd_max_dn12)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn12)), (((4.0 * (-locals.var_qbd_max_dn17)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33310_e48453;
        locals.var_tmf2_dn0 = assign33310_e48453_d_n0;
        locals.var_tmf2_dn2 = assign33310_e48453_d_n2;
        locals.var_tmf2_dn6 = assign33310_e48453_d_n6;
        locals.var_tmf2_dn7 = assign33310_e48453_d_n7;
        locals.var_tmf2_dn10 = assign33310_e48453_d_n10;
        locals.var_tmf2_dn11 = assign33310_e48453_d_n11;
        locals.var_tmf2_dn12 = assign33310_e48453_d_n12;
        locals.var_tmf2_dn17 = assign33310_e48453_d_n17;

        let (assign33320_e48465, assign33320_e48465_d_n0, assign33320_e48465_d_n2, assign33320_e48465_d_n6, assign33320_e48465_d_n7, assign33320_e48465_d_n10, assign33320_e48465_d_n11, assign33320_e48465_d_n12, assign33320_e48465_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
        let (assign33320_e48463, assign33320_e48463_d_n0, assign33320_e48463_d_n2, assign33320_e48463_d_n6, assign33320_e48463_d_n7, assign33320_e48463_d_n10, assign33320_e48463_d_n11, assign33320_e48463_d_n12, assign33320_e48463_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign33320_e48462: f64 = (-locals.var_tmf2);
                (assign33320_e48462, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign33320_e48463, assign33320_e48463_d_n0, assign33320_e48463_d_n2, assign33320_e48463_d_n6, assign33320_e48463_d_n7, assign33320_e48463_d_n10, assign33320_e48463_d_n11, assign33320_e48463_d_n12, assign33320_e48463_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33320_e48465;
        locals.var_tmf2_dn0 = assign33320_e48465_d_n0;
        locals.var_tmf2_dn2 = assign33320_e48465_d_n2;
        locals.var_tmf2_dn6 = assign33320_e48465_d_n6;
        locals.var_tmf2_dn7 = assign33320_e48465_d_n7;
        locals.var_tmf2_dn10 = assign33320_e48465_d_n10;
        locals.var_tmf2_dn11 = assign33320_e48465_d_n11;
        locals.var_tmf2_dn12 = assign33320_e48465_d_n12;
        locals.var_tmf2_dn17 = assign33320_e48465_d_n17;

        let (assign33330_e48476, assign33330_e48476_d_n0, assign33330_e48476_d_n2, assign33330_e48476_d_n6, assign33330_e48476_d_n7, assign33330_e48476_d_n10, assign33330_e48476_d_n11, assign33330_e48476_d_n12, assign33330_e48476_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
        let assign33330_e48471: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign33330_e48473: f64 = (assign33330_e48471 + locals.var_tmf2);
        let assign33330_e48474: f64 = (assign33330_e48473).sqrt();
        (assign33330_e48474, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33330_e48474)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33330_e48476;
        locals.var_tmf2_dn0 = assign33330_e48476_d_n0;
        locals.var_tmf2_dn2 = assign33330_e48476_d_n2;
        locals.var_tmf2_dn6 = assign33330_e48476_d_n6;
        locals.var_tmf2_dn7 = assign33330_e48476_d_n7;
        locals.var_tmf2_dn10 = assign33330_e48476_d_n10;
        locals.var_tmf2_dn11 = assign33330_e48476_d_n11;
        locals.var_tmf2_dn12 = assign33330_e48476_d_n12;
        locals.var_tmf2_dn17 = assign33330_e48476_d_n17;

        let (assign33340_e48489, assign33340_e48489_d_n0, assign33340_e48489_d_n2, assign33340_e48489_d_n6, assign33340_e48489_d_n7, assign33340_e48489_d_n10, assign33340_e48489_d_n11, assign33340_e48489_d_n12, assign33340_e48489_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
        let assign33340_e48481: f64 = (-locals.var_qbd_max);
        let assign33340_e48485: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign33340_e48486: f64 = (0.5 * assign33340_e48485);
        let assign33340_e48487: f64 = (assign33340_e48481 - assign33340_e48486);
        (assign33340_e48487, ((-locals.var_qbd_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbd_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbd_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbd_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbd_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbd_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbd_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbd_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33340_e48489;
        locals.var_qbd_dn0 = assign33340_e48489_d_n0;
        locals.var_qbd_dn2 = assign33340_e48489_d_n2;
        locals.var_qbd_dn6 = assign33340_e48489_d_n6;
        locals.var_qbd_dn7 = assign33340_e48489_d_n7;
        locals.var_qbd_dn10 = assign33340_e48489_d_n10;
        locals.var_qbd_dn11 = assign33340_e48489_d_n11;
        locals.var_qbd_dn12 = assign33340_e48489_d_n12;
        locals.var_qbd_dn17 = assign33340_e48489_d_n17;

        let (assign33350_e48498, assign33350_e48498_d_n0, assign33350_e48498_d_n2, assign33350_e48498_d_n6, assign33350_e48498_d_n7, assign33350_e48498_d_n10, assign33350_e48498_d_n11, assign33350_e48498_d_n12, assign33350_e48498_d_n17,) = {
    if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
        let assign33350_e48495: f64 = (-1.0);
        let assign33350_e48496: f64 = (locals.var_qbd * assign33350_e48495);
        (assign33350_e48496, (locals.var_qbd_dn0 * assign33350_e48495), (locals.var_qbd_dn2 * assign33350_e48495), (locals.var_qbd_dn6 * assign33350_e48495), (locals.var_qbd_dn7 * assign33350_e48495), (locals.var_qbd_dn10 * assign33350_e48495), (locals.var_qbd_dn11 * assign33350_e48495), (locals.var_qbd_dn12 * assign33350_e48495), (locals.var_qbd_dn17 * assign33350_e48495),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33350_e48498;
        locals.var_qbd_dn0 = assign33350_e48498_d_n0;
        locals.var_qbd_dn2 = assign33350_e48498_d_n2;
        locals.var_qbd_dn6 = assign33350_e48498_d_n6;
        locals.var_qbd_dn7 = assign33350_e48498_d_n7;
        locals.var_qbd_dn10 = assign33350_e48498_d_n10;
        locals.var_qbd_dn11 = assign33350_e48498_d_n11;
        locals.var_qbd_dn12 = assign33350_e48498_d_n12;
        locals.var_qbd_dn17 = assign33350_e48498_d_n17;

        let assign33360_e48501: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1095 = assign33360_e48501;

        let (assign33370_e48507,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1095 != 0.0)) {
        (p.p233,)
    } else {
        (locals.var_t10__blk1090,)
    }
};
        locals.var_t10__blk1090 = assign33370_e48507;

        let (assign33380_e48513,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1095 != 0.0)) {
        (p.p234,)
    } else {
        (locals.var_t11__blk1091,)
    }
};
        locals.var_t11__blk1091 = assign33380_e48513;

        let (assign33390_e48519, assign33390_e48519_d_n0, assign33390_e48519_d_n2, assign33390_e48519_d_n6, assign33390_e48519_d_n7, assign33390_e48519_d_n10, assign33390_e48519_d_n11, assign33390_e48519_d_n12, assign33390_e48519_d_n17,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1095 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12, locals.var_lch_dn17,)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12, locals.var_t12_dn17,)
    }
};
        locals.var_t12 = assign33390_e48519;
        locals.var_t12_dn0 = assign33390_e48519_d_n0;
        locals.var_t12_dn2 = assign33390_e48519_d_n2;
        locals.var_t12_dn6 = assign33390_e48519_d_n6;
        locals.var_t12_dn7 = assign33390_e48519_d_n7;
        locals.var_t12_dn10 = assign33390_e48519_d_n10;
        locals.var_t12_dn11 = assign33390_e48519_d_n11;
        locals.var_t12_dn12 = assign33390_e48519_d_n12;
        locals.var_t12_dn17 = assign33390_e48519_d_n17;

        let (assign33400_e48531, assign33400_e48531_d_n0, assign33400_e48531_d_n2, assign33400_e48531_d_n6, assign33400_e48531_d_n7, assign33400_e48531_d_n10, assign33400_e48531_d_n11, assign33400_e48531_d_n12, assign33400_e48531_d_n17,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1095 != 0.0)) {
        let assign33400_e48525: f64 = (locals.var_t10__blk1090 * locals.var_t11__blk1091);
        let assign33400_e48527: f64 = (assign33400_e48525 * locals.var_t12);
        let assign33400_e48529: f64 = (assign33400_e48527 * locals.var_t12);
        (assign33400_e48529, (((assign33400_e48525 * locals.var_t12_dn0) * locals.var_t12) + (assign33400_e48527 * locals.var_t12_dn0)), (((assign33400_e48525 * locals.var_t12_dn2) * locals.var_t12) + (assign33400_e48527 * locals.var_t12_dn2)), (((assign33400_e48525 * locals.var_t12_dn6) * locals.var_t12) + (assign33400_e48527 * locals.var_t12_dn6)), (((assign33400_e48525 * locals.var_t12_dn7) * locals.var_t12) + (assign33400_e48527 * locals.var_t12_dn7)), (((assign33400_e48525 * locals.var_t12_dn10) * locals.var_t12) + (assign33400_e48527 * locals.var_t12_dn10)), (((assign33400_e48525 * locals.var_t12_dn11) * locals.var_t12) + (assign33400_e48527 * locals.var_t12_dn11)), (((assign33400_e48525 * locals.var_t12_dn12) * locals.var_t12) + (assign33400_e48527 * locals.var_t12_dn12)), (((assign33400_e48525 * locals.var_t12_dn17) * locals.var_t12) + (assign33400_e48527 * locals.var_t12_dn17)),)
    } else {
        (locals.var_t1__blk1093, locals.var_t1__blk1093_dn0, locals.var_t1__blk1093_dn2, locals.var_t1__blk1093_dn6, locals.var_t1__blk1093_dn7, locals.var_t1__blk1093_dn10, locals.var_t1__blk1093_dn11, locals.var_t1__blk1093_dn12, locals.var_t1__blk1093_dn17,)
    }
};
        locals.var_t1__blk1093 = assign33400_e48531;
        locals.var_t1__blk1093_dn0 = assign33400_e48531_d_n0;
        locals.var_t1__blk1093_dn2 = assign33400_e48531_d_n2;
        locals.var_t1__blk1093_dn6 = assign33400_e48531_d_n6;
        locals.var_t1__blk1093_dn7 = assign33400_e48531_d_n7;
        locals.var_t1__blk1093_dn10 = assign33400_e48531_d_n10;
        locals.var_t1__blk1093_dn11 = assign33400_e48531_d_n11;
        locals.var_t1__blk1093_dn12 = assign33400_e48531_d_n12;
        locals.var_t1__blk1093_dn17 = assign33400_e48531_d_n17;

        let (assign33440_e48570, assign33440_e48570_d_n0, assign33440_e48570_d_n2, assign33440_e48570_d_n6, assign33440_e48570_d_n7, assign33440_e48570_d_n10, assign33440_e48570_d_n11, assign33440_e48570_d_n12, assign33440_e48570_d_n17,) = {
    if (locals.var_flg_nqs != 0.0) {
        (p.p235, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1093, locals.var_t1__blk1093_dn0, locals.var_t1__blk1093_dn2, locals.var_t1__blk1093_dn6, locals.var_t1__blk1093_dn7, locals.var_t1__blk1093_dn10, locals.var_t1__blk1093_dn11, locals.var_t1__blk1093_dn12, locals.var_t1__blk1093_dn17,)
    }
};
        locals.var_t1__blk1093 = assign33440_e48570;
        locals.var_t1__blk1093_dn0 = assign33440_e48570_d_n0;
        locals.var_t1__blk1093_dn2 = assign33440_e48570_d_n2;
        locals.var_t1__blk1093_dn6 = assign33440_e48570_d_n6;
        locals.var_t1__blk1093_dn7 = assign33440_e48570_d_n7;
        locals.var_t1__blk1093_dn10 = assign33440_e48570_d_n10;
        locals.var_t1__blk1093_dn11 = assign33440_e48570_d_n11;
        locals.var_t1__blk1093_dn12 = assign33440_e48570_d_n12;
        locals.var_t1__blk1093_dn17 = assign33440_e48570_d_n17;

        let (assign33450_e48576, assign33450_e48576_d_n0, assign33450_e48576_d_n2, assign33450_e48576_d_n6, assign33450_e48576_d_n7, assign33450_e48576_d_n10, assign33450_e48576_d_n11, assign33450_e48576_d_n12, assign33450_e48576_d_n17,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign33450_e48574: f64 = (locals.var_t1__blk1093 * locals.var_c_fox);
        (assign33450_e48574, ((locals.var_t1__blk1093_dn0 * locals.var_c_fox) + (locals.var_t1__blk1093 * locals.var_c_fox_dn0)), ((locals.var_t1__blk1093_dn2 * locals.var_c_fox) + (locals.var_t1__blk1093 * locals.var_c_fox_dn2)), ((locals.var_t1__blk1093_dn6 * locals.var_c_fox) + (locals.var_t1__blk1093 * locals.var_c_fox_dn6)), ((locals.var_t1__blk1093_dn7 * locals.var_c_fox) + (locals.var_t1__blk1093 * locals.var_c_fox_dn7)), ((locals.var_t1__blk1093_dn10 * locals.var_c_fox) + (locals.var_t1__blk1093 * locals.var_c_fox_dn10)), ((locals.var_t1__blk1093_dn11 * locals.var_c_fox) + (locals.var_t1__blk1093 * locals.var_c_fox_dn11)), ((locals.var_t1__blk1093_dn12 * locals.var_c_fox) + (locals.var_t1__blk1093 * locals.var_c_fox_dn12)), ((locals.var_t1__blk1093_dn17 * locals.var_c_fox) + (locals.var_t1__blk1093 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_taub, locals.var_taub_dn0, locals.var_taub_dn2, locals.var_taub_dn6, locals.var_taub_dn7, locals.var_taub_dn10, locals.var_taub_dn11, locals.var_taub_dn12, locals.var_taub_dn17,)
    }
};
        locals.var_taub = assign33450_e48576;
        locals.var_taub_dn0 = assign33450_e48576_d_n0;
        locals.var_taub_dn2 = assign33450_e48576_d_n2;
        locals.var_taub_dn6 = assign33450_e48576_d_n6;
        locals.var_taub_dn7 = assign33450_e48576_d_n7;
        locals.var_taub_dn10 = assign33450_e48576_d_n10;
        locals.var_taub_dn11 = assign33450_e48576_d_n11;
        locals.var_taub_dn12 = assign33450_e48576_d_n12;
        locals.var_taub_dn17 = assign33450_e48576_d_n17;

        let assign33580_e48752: f64 = if ((p.p32 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1122 = assign33580_e48752;

        let (assign33590_e48760, assign33590_e48760_d_n0, assign33590_e48760_d_n2, assign33590_e48760_d_n6, assign33590_e48760_d_n7, assign33590_e48760_d_n10, assign33590_e48760_d_n11, assign33590_e48760_d_n12, assign33590_e48760_d_n17,) = {
    if (locals.var_guard1122 != 0.0) {
        let assign33590_e48756: f64 = (locals.var_psdl - locals.var_ps0);
        let assign33590_e48758: f64 = (assign33590_e48756 / locals.var_lch);
        (assign33590_e48758, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn12 - locals.var_ps0_dn12) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn17 - locals.var_ps0_dn17) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn17)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn12, locals.var_eyd_dn17,)
    }
};
        locals.var_eyd = assign33590_e48760;
        locals.var_eyd_dn0 = assign33590_e48760_d_n0;
        locals.var_eyd_dn2 = assign33590_e48760_d_n2;
        locals.var_eyd_dn6 = assign33590_e48760_d_n6;
        locals.var_eyd_dn7 = assign33590_e48760_d_n7;
        locals.var_eyd_dn10 = assign33590_e48760_d_n10;
        locals.var_eyd_dn11 = assign33590_e48760_d_n11;
        locals.var_eyd_dn12 = assign33590_e48760_d_n12;
        locals.var_eyd_dn17 = assign33590_e48760_d_n17;

        let (assign33600_e48768, assign33600_e48768_d_n0, assign33600_e48768_d_n2, assign33600_e48768_d_n6, assign33600_e48768_d_n7, assign33600_e48768_d_n10, assign33600_e48768_d_n11, assign33600_e48768_d_n12, assign33600_e48768_d_n17,) = {
    if (locals.var_guard1122 != 0.0) {
        let assign33600_e48764: f64 = (locals.var_muun * locals.var_eyd);
        let assign33600_e48766: f64 = (assign33600_e48764 / 100000.0);
        (assign33600_e48766, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 100000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 100000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 100000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 100000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 100000.0), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / 100000.0), (((locals.var_muun_dn12 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn12)) / 100000.0), (((locals.var_muun_dn17 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn17)) / 100000.0),)
    } else {
        (locals.var_t12__blk1106, locals.var_t12__blk1106_dn0, locals.var_t12__blk1106_dn2, locals.var_t12__blk1106_dn6, locals.var_t12__blk1106_dn7, locals.var_t12__blk1106_dn10, locals.var_t12__blk1106_dn11, locals.var_t12__blk1106_dn12, locals.var_t12__blk1106_dn17,)
    }
};
        locals.var_t12__blk1106 = assign33600_e48768;
        locals.var_t12__blk1106_dn0 = assign33600_e48768_d_n0;
        locals.var_t12__blk1106_dn2 = assign33600_e48768_d_n2;
        locals.var_t12__blk1106_dn6 = assign33600_e48768_d_n6;
        locals.var_t12__blk1106_dn7 = assign33600_e48768_d_n7;
        locals.var_t12__blk1106_dn10 = assign33600_e48768_d_n10;
        locals.var_t12__blk1106_dn11 = assign33600_e48768_d_n11;
        locals.var_t12__blk1106_dn12 = assign33600_e48768_d_n12;
        locals.var_t12__blk1106_dn17 = assign33600_e48768_d_n17;

        let assign33610_e48772: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48773: f64 = (1.0 - assign33610_e48772);
        let assign33610_e48780: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48781: f64 = (1.0 + assign33610_e48780);
        let assign33610_e48783: f64 = if ((assign33610_e48773 <= p.p113) && (p.p113 <= assign33610_e48781)) { 1.0 } else { 0.0 };
        locals.var_guard1123 = assign33610_e48783;

        let (assign33620_e48789, assign33620_e48789_d_n0, assign33620_e48789_d_n2, assign33620_e48789_d_n6, assign33620_e48789_d_n7, assign33620_e48789_d_n10, assign33620_e48789_d_n11, assign33620_e48789_d_n12, assign33620_e48789_d_n17,) = {
    if ((locals.var_guard1122 != 0.0) && (locals.var_guard1123 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk1107, locals.var_t7__blk1107_dn0, locals.var_t7__blk1107_dn2, locals.var_t7__blk1107_dn6, locals.var_t7__blk1107_dn7, locals.var_t7__blk1107_dn10, locals.var_t7__blk1107_dn11, locals.var_t7__blk1107_dn12, locals.var_t7__blk1107_dn17,)
    }
};
        locals.var_t7__blk1107 = assign33620_e48789;
        locals.var_t7__blk1107_dn0 = assign33620_e48789_d_n0;
        locals.var_t7__blk1107_dn2 = assign33620_e48789_d_n2;
        locals.var_t7__blk1107_dn6 = assign33620_e48789_d_n6;
        locals.var_t7__blk1107_dn7 = assign33620_e48789_d_n7;
        locals.var_t7__blk1107_dn10 = assign33620_e48789_d_n10;
        locals.var_t7__blk1107_dn11 = assign33620_e48789_d_n11;
        locals.var_t7__blk1107_dn12 = assign33620_e48789_d_n12;
        locals.var_t7__blk1107_dn17 = assign33620_e48789_d_n17;

        let assign33630_e48793: f64 = (10.0 * 2.220446049250313e-16);
        let assign33630_e48794: f64 = (2.0 - assign33630_e48793);
        let assign33630_e48801: f64 = (10.0 * 2.220446049250313e-16);
        let assign33630_e48802: f64 = (2.0 + assign33630_e48801);
        let assign33630_e48804: f64 = if ((assign33630_e48794 <= p.p113) && (p.p113 <= assign33630_e48802)) { 1.0 } else { 0.0 };
        locals.var_guard1124 = assign33630_e48804;

        let (assign33640_e48813, assign33640_e48813_d_n0, assign33640_e48813_d_n2, assign33640_e48813_d_n6, assign33640_e48813_d_n7, assign33640_e48813_d_n10, assign33640_e48813_d_n11, assign33640_e48813_d_n12, assign33640_e48813_d_n17,) = {
    if (((locals.var_guard1122 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
        (locals.var_t12__blk1106, locals.var_t12__blk1106_dn0, locals.var_t12__blk1106_dn2, locals.var_t12__blk1106_dn6, locals.var_t12__blk1106_dn7, locals.var_t12__blk1106_dn10, locals.var_t12__blk1106_dn11, locals.var_t12__blk1106_dn12, locals.var_t12__blk1106_dn17,)
    } else {
        (locals.var_t7__blk1107, locals.var_t7__blk1107_dn0, locals.var_t7__blk1107_dn2, locals.var_t7__blk1107_dn6, locals.var_t7__blk1107_dn7, locals.var_t7__blk1107_dn10, locals.var_t7__blk1107_dn11, locals.var_t7__blk1107_dn12, locals.var_t7__blk1107_dn17,)
    }
};
        locals.var_t7__blk1107 = assign33640_e48813;
        locals.var_t7__blk1107_dn0 = assign33640_e48813_d_n0;
        locals.var_t7__blk1107_dn2 = assign33640_e48813_d_n2;
        locals.var_t7__blk1107_dn6 = assign33640_e48813_d_n6;
        locals.var_t7__blk1107_dn7 = assign33640_e48813_d_n7;
        locals.var_t7__blk1107_dn10 = assign33640_e48813_d_n10;
        locals.var_t7__blk1107_dn11 = assign33640_e48813_d_n11;
        locals.var_t7__blk1107_dn12 = assign33640_e48813_d_n12;
        locals.var_t7__blk1107_dn17 = assign33640_e48813_d_n17;

        let (assign33650_e48827, assign33650_e48827_d_n0, assign33650_e48827_d_n2, assign33650_e48827_d_n6, assign33650_e48827_d_n7, assign33650_e48827_d_n10, assign33650_e48827_d_n11, assign33650_e48827_d_n12, assign33650_e48827_d_n17,) = {
    if (((locals.var_guard1122 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 == 0.0)) {
        let assign33650_e48824: f64 = (p.p113 - 1.0);
        let assign33650_e48825: f64 = (locals.var_t12__blk1106).powf(assign33650_e48824);
        (assign33650_e48825, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn0)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn0 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn2)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn2 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn6)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn6 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn7)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn7 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn10)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn10 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn11)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn11 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn12)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn12 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn17)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn17 / locals.var_t12__blk1106))) },)
    } else {
        (locals.var_t7__blk1107, locals.var_t7__blk1107_dn0, locals.var_t7__blk1107_dn2, locals.var_t7__blk1107_dn6, locals.var_t7__blk1107_dn7, locals.var_t7__blk1107_dn10, locals.var_t7__blk1107_dn11, locals.var_t7__blk1107_dn12, locals.var_t7__blk1107_dn17,)
    }
};
        locals.var_t7__blk1107 = assign33650_e48827;
        locals.var_t7__blk1107_dn0 = assign33650_e48827_d_n0;
        locals.var_t7__blk1107_dn2 = assign33650_e48827_d_n2;
        locals.var_t7__blk1107_dn6 = assign33650_e48827_d_n6;
        locals.var_t7__blk1107_dn7 = assign33650_e48827_d_n7;
        locals.var_t7__blk1107_dn10 = assign33650_e48827_d_n10;
        locals.var_t7__blk1107_dn11 = assign33650_e48827_d_n11;
        locals.var_t7__blk1107_dn12 = assign33650_e48827_d_n12;
        locals.var_t7__blk1107_dn17 = assign33650_e48827_d_n17;

        let (assign33660_e48833, assign33660_e48833_d_n0, assign33660_e48833_d_n2, assign33660_e48833_d_n6, assign33660_e48833_d_n7, assign33660_e48833_d_n10, assign33660_e48833_d_n11, assign33660_e48833_d_n12, assign33660_e48833_d_n17,) = {
    if (locals.var_guard1122 != 0.0) {
        let assign33660_e48831: f64 = (locals.var_t12__blk1106 * locals.var_t7__blk1107);
        (assign33660_e48831, ((locals.var_t12__blk1106_dn0 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn0)), ((locals.var_t12__blk1106_dn2 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn2)), ((locals.var_t12__blk1106_dn6 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn6)), ((locals.var_t12__blk1106_dn7 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn7)), ((locals.var_t12__blk1106_dn10 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn10)), ((locals.var_t12__blk1106_dn11 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn11)), ((locals.var_t12__blk1106_dn12 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn12)), ((locals.var_t12__blk1106_dn17 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn17)),)
    } else {
        (locals.var_t8__blk1108, locals.var_t8__blk1108_dn0, locals.var_t8__blk1108_dn2, locals.var_t8__blk1108_dn6, locals.var_t8__blk1108_dn7, locals.var_t8__blk1108_dn10, locals.var_t8__blk1108_dn11, locals.var_t8__blk1108_dn12, locals.var_t8__blk1108_dn17,)
    }
};
        locals.var_t8__blk1108 = assign33660_e48833;
        locals.var_t8__blk1108_dn0 = assign33660_e48833_d_n0;
        locals.var_t8__blk1108_dn2 = assign33660_e48833_d_n2;
        locals.var_t8__blk1108_dn6 = assign33660_e48833_d_n6;
        locals.var_t8__blk1108_dn7 = assign33660_e48833_d_n7;
        locals.var_t8__blk1108_dn10 = assign33660_e48833_d_n10;
        locals.var_t8__blk1108_dn11 = assign33660_e48833_d_n11;
        locals.var_t8__blk1108_dn12 = assign33660_e48833_d_n12;
        locals.var_t8__blk1108_dn17 = assign33660_e48833_d_n17;

        let (assign33670_e48839, assign33670_e48839_d_n0, assign33670_e48839_d_n2, assign33670_e48839_d_n6, assign33670_e48839_d_n7, assign33670_e48839_d_n10, assign33670_e48839_d_n11, assign33670_e48839_d_n12, assign33670_e48839_d_n17,) = {
    if (locals.var_guard1122 != 0.0) {
        let assign33670_e48837: f64 = (1.0 + locals.var_t8__blk1108);
        (assign33670_e48837, locals.var_t8__blk1108_dn0, locals.var_t8__blk1108_dn2, locals.var_t8__blk1108_dn6, locals.var_t8__blk1108_dn7, locals.var_t8__blk1108_dn10, locals.var_t8__blk1108_dn11, locals.var_t8__blk1108_dn12, locals.var_t8__blk1108_dn17,)
    } else {
        (locals.var_t9__blk1109, locals.var_t9__blk1109_dn0, locals.var_t9__blk1109_dn2, locals.var_t9__blk1109_dn6, locals.var_t9__blk1109_dn7, locals.var_t9__blk1109_dn10, locals.var_t9__blk1109_dn11, locals.var_t9__blk1109_dn12, locals.var_t9__blk1109_dn17,)
    }
};
        locals.var_t9__blk1109 = assign33670_e48839;
        locals.var_t9__blk1109_dn0 = assign33670_e48839_d_n0;
        locals.var_t9__blk1109_dn2 = assign33670_e48839_d_n2;
        locals.var_t9__blk1109_dn6 = assign33670_e48839_d_n6;
        locals.var_t9__blk1109_dn7 = assign33670_e48839_d_n7;
        locals.var_t9__blk1109_dn10 = assign33670_e48839_d_n10;
        locals.var_t9__blk1109_dn11 = assign33670_e48839_d_n11;
        locals.var_t9__blk1109_dn12 = assign33670_e48839_d_n12;
        locals.var_t9__blk1109_dn17 = assign33670_e48839_d_n17;

        let (assign33680_e48850, assign33680_e48850_d_n0, assign33680_e48850_d_n2, assign33680_e48850_d_n6, assign33680_e48850_d_n7, assign33680_e48850_d_n10, assign33680_e48850_d_n11, assign33680_e48850_d_n12, assign33680_e48850_d_n17,) = {
    if (locals.var_guard1122 != 0.0) {
        let assign33680_e48843: f64 = (-1.0);
        let assign33680_e48845: f64 = (assign33680_e48843 / p.p113);
        let assign33680_e48847: f64 = (assign33680_e48845 - 1.0);
        let assign33680_e48848: f64 = (locals.var_t9__blk1109).powf(assign33680_e48847);
        (assign33680_e48848, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn0)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn0 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn2)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn2 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn6)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn6 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn7)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn7 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn10)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn10 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn11)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn11 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn12)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn12 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn17)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn17 / locals.var_t9__blk1109))) },)
    } else {
        (locals.var_t10__blk1110, locals.var_t10__blk1110_dn0, locals.var_t10__blk1110_dn2, locals.var_t10__blk1110_dn6, locals.var_t10__blk1110_dn7, locals.var_t10__blk1110_dn10, locals.var_t10__blk1110_dn11, locals.var_t10__blk1110_dn12, locals.var_t10__blk1110_dn17,)
    }
};
        locals.var_t10__blk1110 = assign33680_e48850;
        locals.var_t10__blk1110_dn0 = assign33680_e48850_d_n0;
        locals.var_t10__blk1110_dn2 = assign33680_e48850_d_n2;
        locals.var_t10__blk1110_dn6 = assign33680_e48850_d_n6;
        locals.var_t10__blk1110_dn7 = assign33680_e48850_d_n7;
        locals.var_t10__blk1110_dn10 = assign33680_e48850_d_n10;
        locals.var_t10__blk1110_dn11 = assign33680_e48850_d_n11;
        locals.var_t10__blk1110_dn12 = assign33680_e48850_d_n12;
        locals.var_t10__blk1110_dn17 = assign33680_e48850_d_n17;

        let (assign33690_e48856, assign33690_e48856_d_n0, assign33690_e48856_d_n2, assign33690_e48856_d_n6, assign33690_e48856_d_n7, assign33690_e48856_d_n10, assign33690_e48856_d_n11, assign33690_e48856_d_n12, assign33690_e48856_d_n17,) = {
    if (locals.var_guard1122 != 0.0) {
        let assign33690_e48854: f64 = (locals.var_t9__blk1109 * locals.var_t10__blk1110);
        (assign33690_e48854, ((locals.var_t9__blk1109_dn0 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn0)), ((locals.var_t9__blk1109_dn2 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn2)), ((locals.var_t9__blk1109_dn6 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn6)), ((locals.var_t9__blk1109_dn7 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn7)), ((locals.var_t9__blk1109_dn10 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn10)), ((locals.var_t9__blk1109_dn11 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn11)), ((locals.var_t9__blk1109_dn12 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn12)), ((locals.var_t9__blk1109_dn17 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn17)),)
    } else {
        (locals.var_t11__blk1111, locals.var_t11__blk1111_dn0, locals.var_t11__blk1111_dn2, locals.var_t11__blk1111_dn6, locals.var_t11__blk1111_dn7, locals.var_t11__blk1111_dn10, locals.var_t11__blk1111_dn11, locals.var_t11__blk1111_dn12, locals.var_t11__blk1111_dn17,)
    }
};
        locals.var_t11__blk1111 = assign33690_e48856;
        locals.var_t11__blk1111_dn0 = assign33690_e48856_d_n0;
        locals.var_t11__blk1111_dn2 = assign33690_e48856_d_n2;
        locals.var_t11__blk1111_dn6 = assign33690_e48856_d_n6;
        locals.var_t11__blk1111_dn7 = assign33690_e48856_d_n7;
        locals.var_t11__blk1111_dn10 = assign33690_e48856_d_n10;
        locals.var_t11__blk1111_dn11 = assign33690_e48856_d_n11;
        locals.var_t11__blk1111_dn12 = assign33690_e48856_d_n12;
        locals.var_t11__blk1111_dn17 = assign33690_e48856_d_n17;

    }

    pub(super) fn stamp_transient_block_118(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33700_e48862, assign33700_e48862_d_n0, assign33700_e48862_d_n2, assign33700_e48862_d_n6, assign33700_e48862_d_n7, assign33700_e48862_d_n10, assign33700_e48862_d_n11, assign33700_e48862_d_n12, assign33700_e48862_d_n17,) = {
    if (locals.var_guard1122 != 0.0) {
        let assign33700_e48860: f64 = (locals.var_muun * locals.var_t11__blk1111);
        (assign33700_e48860, ((locals.var_muun_dn0 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn0)), ((locals.var_muun_dn2 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn2)), ((locals.var_muun_dn6 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn6)), ((locals.var_muun_dn7 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn7)), ((locals.var_muun_dn10 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn10)), ((locals.var_muun_dn11 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn11)), ((locals.var_muun_dn12 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn12)), ((locals.var_muun_dn17 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn17)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn12, locals.var_mud_hoso_dn17,)
    }
};
        locals.var_mud_hoso = assign33700_e48862;
        locals.var_mud_hoso_dn0 = assign33700_e48862_d_n0;
        locals.var_mud_hoso_dn2 = assign33700_e48862_d_n2;
        locals.var_mud_hoso_dn6 = assign33700_e48862_d_n6;
        locals.var_mud_hoso_dn7 = assign33700_e48862_d_n7;
        locals.var_mud_hoso_dn10 = assign33700_e48862_d_n10;
        locals.var_mud_hoso_dn11 = assign33700_e48862_d_n11;
        locals.var_mud_hoso_dn12 = assign33700_e48862_d_n12;
        locals.var_mud_hoso_dn17 = assign33700_e48862_d_n17;

        let (assign33710_e48870, assign33710_e48870_d_n0, assign33710_e48870_d_n2, assign33710_e48870_d_n6, assign33710_e48870_d_n7, assign33710_e48870_d_n10, assign33710_e48870_d_n11, assign33710_e48870_d_n12, assign33710_e48870_d_n17,) = {
    if (locals.var_guard1122 != 0.0) {
        let assign33710_e48866: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign33710_e48868: f64 = (assign33710_e48866 / 2.0);
        (assign33710_e48868, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn12 + locals.var_mud_hoso_dn12) / 2.0), ((locals.var_mu_dn17 + locals.var_mud_hoso_dn17) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn12, locals.var_mu_ave_dn17,)
    }
};
        locals.var_mu_ave = assign33710_e48870;
        locals.var_mu_ave_dn0 = assign33710_e48870_d_n0;
        locals.var_mu_ave_dn2 = assign33710_e48870_d_n2;
        locals.var_mu_ave_dn6 = assign33710_e48870_d_n6;
        locals.var_mu_ave_dn7 = assign33710_e48870_d_n7;
        locals.var_mu_ave_dn10 = assign33710_e48870_d_n10;
        locals.var_mu_ave_dn11 = assign33710_e48870_d_n11;
        locals.var_mu_ave_dn12 = assign33710_e48870_d_n12;
        locals.var_mu_ave_dn17 = assign33710_e48870_d_n17;

        let (assign33720_e48876, assign33720_e48876_d_n0, assign33720_e48876_d_n2, assign33720_e48876_d_n6, assign33720_e48876_d_n7, assign33720_e48876_d_n10, assign33720_e48876_d_n11, assign33720_e48876_d_n12, assign33720_e48876_d_n17,) = {
    if (locals.var_guard1122 != 0.0) {
        let assign33720_e48874: f64 = (locals.var_alpha * locals.var_alpha);
        (assign33720_e48874, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_t0__blk1112, locals.var_t0__blk1112_dn0, locals.var_t0__blk1112_dn2, locals.var_t0__blk1112_dn6, locals.var_t0__blk1112_dn7, locals.var_t0__blk1112_dn10, locals.var_t0__blk1112_dn11, locals.var_t0__blk1112_dn12, locals.var_t0__blk1112_dn17,)
    }
};
        locals.var_t0__blk1112 = assign33720_e48876;
        locals.var_t0__blk1112_dn0 = assign33720_e48876_d_n0;
        locals.var_t0__blk1112_dn2 = assign33720_e48876_d_n2;
        locals.var_t0__blk1112_dn6 = assign33720_e48876_d_n6;
        locals.var_t0__blk1112_dn7 = assign33720_e48876_d_n7;
        locals.var_t0__blk1112_dn10 = assign33720_e48876_d_n10;
        locals.var_t0__blk1112_dn11 = assign33720_e48876_d_n11;
        locals.var_t0__blk1112_dn12 = assign33720_e48876_d_n12;
        locals.var_t0__blk1112_dn17 = assign33720_e48876_d_n17;

        let (assign33730_e48938, assign33730_e48938_d_n0, assign33730_e48938_d_n2, assign33730_e48938_d_n6, assign33730_e48938_d_n7, assign33730_e48938_d_n10, assign33730_e48938_d_n11, assign33730_e48938_d_n12, assign33730_e48938_d_n17,) = {
    if (locals.var_guard1122 != 0.0) {
        let assign33730_e48880: f64 = (locals.var_weff_nf * locals.var_c_fox);
        let assign33730_e48882: f64 = (assign33730_e48880 * locals.var_vgvt);
        let assign33730_e48884: f64 = (assign33730_e48882 * locals.var_mu);
        let assign33730_e48888: f64 = (3.0 * locals.var_alpha);
        let assign33730_e48889: f64 = (1.0 + assign33730_e48888);
        let assign33730_e48892: f64 = (6.0 * locals.var_t0__blk1112);
        let assign33730_e48893: f64 = (assign33730_e48889 + assign33730_e48892);
        let assign33730_e48895: f64 = (assign33730_e48893 * locals.var_mud_hoso);
        let assign33730_e48897: f64 = (assign33730_e48895 * locals.var_mud_hoso);
        let assign33730_e48901: f64 = (4.0 * locals.var_alpha);
        let assign33730_e48902: f64 = (3.0 + assign33730_e48901);
        let assign33730_e48905: f64 = (3.0 * locals.var_t0__blk1112);
        let assign33730_e48906: f64 = (assign33730_e48902 + assign33730_e48905);
        let assign33730_e48908: f64 = (assign33730_e48906 * locals.var_mud_hoso);
        let assign33730_e48910: f64 = (assign33730_e48908 * locals.var_mu);
        let assign33730_e48911: f64 = (assign33730_e48897 + assign33730_e48910);
        let assign33730_e48915: f64 = (3.0 * locals.var_alpha);
        let assign33730_e48916: f64 = (6.0 + assign33730_e48915);
        let assign33730_e48918: f64 = (assign33730_e48916 + locals.var_t0__blk1112);
        let assign33730_e48920: f64 = (assign33730_e48918 * locals.var_mu);
        let assign33730_e48922: f64 = (assign33730_e48920 * locals.var_mu);
        let assign33730_e48923: f64 = (assign33730_e48911 + assign33730_e48922);
        let assign33730_e48924: f64 = (assign33730_e48884 * assign33730_e48923);
        let assign33730_e48927: f64 = (15.0 * locals.var_lch);
        let assign33730_e48930: f64 = (1.0 + locals.var_alpha);
        let assign33730_e48931: f64 = (assign33730_e48927 * assign33730_e48930);
        let assign33730_e48933: f64 = (assign33730_e48931 * locals.var_mu_ave);
        let assign33730_e48935: f64 = (assign33730_e48933 * locals.var_mu_ave);
        let assign33730_e48936: f64 = (assign33730_e48924 / assign33730_e48935);
        (assign33730_e48936, ((((((((((locals.var_weff_nf * locals.var_c_fox_dn0) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn0)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0__blk1112_dn0)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0__blk1112_dn0)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0__blk1112_dn0) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn0)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn0))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn0) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn0)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn2) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn2)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0__blk1112_dn2)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0__blk1112_dn2)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0__blk1112_dn2) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn2)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn2))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn2) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn2)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn6) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn6)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0__blk1112_dn6)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0__blk1112_dn6)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0__blk1112_dn6) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn6)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn6))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn6) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn6)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn7) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn7)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0__blk1112_dn7)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0__blk1112_dn7)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0__blk1112_dn7) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn7)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn7))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn7) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn7)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn10) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn10)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0__blk1112_dn10)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0__blk1112_dn10)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0__blk1112_dn10) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn10)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn10))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn10) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn10)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn11) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn11)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0__blk1112_dn11)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0__blk1112_dn11)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0__blk1112_dn11) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn11)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn11))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn11) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn11)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn12) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn12)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn12)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn12) + (6.0 * locals.var_t0__blk1112_dn12)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn12)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn12)) + ((((((4.0 * locals.var_alpha_dn12) + (3.0 * locals.var_t0__blk1112_dn12)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn12)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn12))) + ((((((3.0 * locals.var_alpha_dn12) + locals.var_t0__blk1112_dn12) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn12)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn12))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn12) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn12)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn12)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn12)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn17) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn17)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn17)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn17) + (6.0 * locals.var_t0__blk1112_dn17)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn17)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn17)) + ((((((4.0 * locals.var_alpha_dn17) + (3.0 * locals.var_t0__blk1112_dn17)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn17)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn17))) + ((((((3.0 * locals.var_alpha_dn17) + locals.var_t0__blk1112_dn17) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn17)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn17))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn17) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn17)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn17)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn17)))) / (assign33730_e48935 * assign33730_e48935)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17,)
    }
};
        locals.var_nthrml = assign33730_e48938;
        locals.var_nthrml_dn0 = assign33730_e48938_d_n0;
        locals.var_nthrml_dn2 = assign33730_e48938_d_n2;
        locals.var_nthrml_dn6 = assign33730_e48938_d_n6;
        locals.var_nthrml_dn7 = assign33730_e48938_d_n7;
        locals.var_nthrml_dn10 = assign33730_e48938_d_n10;
        locals.var_nthrml_dn11 = assign33730_e48938_d_n11;
        locals.var_nthrml_dn12 = assign33730_e48938_d_n12;
        locals.var_nthrml_dn17 = assign33730_e48938_d_n17;

        let (assign33740_e48943, assign33740_e48943_d_n0, assign33740_e48943_d_n2, assign33740_e48943_d_n6, assign33740_e48943_d_n7, assign33740_e48943_d_n10, assign33740_e48943_d_n11, assign33740_e48943_d_n12, assign33740_e48943_d_n17,) = {
    if (locals.var_guard1122 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17,)
    }
};
        locals.var_nthrml = assign33740_e48943;
        locals.var_nthrml_dn0 = assign33740_e48943_d_n0;
        locals.var_nthrml_dn2 = assign33740_e48943_d_n2;
        locals.var_nthrml_dn6 = assign33740_e48943_d_n6;
        locals.var_nthrml_dn7 = assign33740_e48943_d_n7;
        locals.var_nthrml_dn10 = assign33740_e48943_d_n10;
        locals.var_nthrml_dn11 = assign33740_e48943_d_n11;
        locals.var_nthrml_dn12 = assign33740_e48943_d_n12;
        locals.var_nthrml_dn17 = assign33740_e48943_d_n17;

        let assign33750_e48957: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1125 = assign33750_e48957;

        let (assign33760_e48962, assign33760_e48962_d_n0, assign33760_e48962_d_n2, assign33760_e48962_d_n6, assign33760_e48962_d_n7, assign33760_e48962_d_n10, assign33760_e48962_d_n11, assign33760_e48962_d_n12, assign33760_e48962_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33760_e48960: f64 = (locals.var_kusail).sqrt();
        (assign33760_e48960, (locals.var_kusail_dn0 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn2 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn6 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn7 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn10 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn11 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn12 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn17 / (2.0 * assign33760_e48960)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn12, locals.var_sqrtkusail_dn17,)
    }
};
        locals.var_sqrtkusail = assign33760_e48962;
        locals.var_sqrtkusail_dn0 = assign33760_e48962_d_n0;
        locals.var_sqrtkusail_dn2 = assign33760_e48962_d_n2;
        locals.var_sqrtkusail_dn6 = assign33760_e48962_d_n6;
        locals.var_sqrtkusail_dn7 = assign33760_e48962_d_n7;
        locals.var_sqrtkusail_dn10 = assign33760_e48962_d_n10;
        locals.var_sqrtkusail_dn11 = assign33760_e48962_d_n11;
        locals.var_sqrtkusail_dn12 = assign33760_e48962_d_n12;
        locals.var_sqrtkusail_dn17 = assign33760_e48962_d_n17;

        let (assign33770_e48968, assign33770_e48968_d_n0, assign33770_e48968_d_n2, assign33770_e48968_d_n6, assign33770_e48968_d_n7, assign33770_e48968_d_n10, assign33770_e48968_d_n11, assign33770_e48968_d_n12, assign33770_e48968_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33770_e48966: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign33770_e48966, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12), (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17),)
    } else {
        (locals.var_t2__blk1114, locals.var_t2__blk1114_dn0, locals.var_t2__blk1114_dn2, locals.var_t2__blk1114_dn6, locals.var_t2__blk1114_dn7, locals.var_t2__blk1114_dn10, locals.var_t2__blk1114_dn11, locals.var_t2__blk1114_dn12, locals.var_t2__blk1114_dn17,)
    }
};
        locals.var_t2__blk1114 = assign33770_e48968;
        locals.var_t2__blk1114_dn0 = assign33770_e48968_d_n0;
        locals.var_t2__blk1114_dn2 = assign33770_e48968_d_n2;
        locals.var_t2__blk1114_dn6 = assign33770_e48968_d_n6;
        locals.var_t2__blk1114_dn7 = assign33770_e48968_d_n7;
        locals.var_t2__blk1114_dn10 = assign33770_e48968_d_n10;
        locals.var_t2__blk1114_dn11 = assign33770_e48968_d_n11;
        locals.var_t2__blk1114_dn12 = assign33770_e48968_d_n12;
        locals.var_t2__blk1114_dn17 = assign33770_e48968_d_n17;

        let (assign33780_e48974, assign33780_e48974_d_n0, assign33780_e48974_d_n2, assign33780_e48974_d_n6, assign33780_e48974_d_n7, assign33780_e48974_d_n10, assign33780_e48974_d_n11, assign33780_e48974_d_n12, assign33780_e48974_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33780_e48972: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign33780_e48972, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)), ((locals.var_kusai00_dn17 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn17)),)
    } else {
        (locals.var_t3__blk1115, locals.var_t3__blk1115_dn0, locals.var_t3__blk1115_dn2, locals.var_t3__blk1115_dn6, locals.var_t3__blk1115_dn7, locals.var_t3__blk1115_dn10, locals.var_t3__blk1115_dn11, locals.var_t3__blk1115_dn12, locals.var_t3__blk1115_dn17,)
    }
};
        locals.var_t3__blk1115 = assign33780_e48974;
        locals.var_t3__blk1115_dn0 = assign33780_e48974_d_n0;
        locals.var_t3__blk1115_dn2 = assign33780_e48974_d_n2;
        locals.var_t3__blk1115_dn6 = assign33780_e48974_d_n6;
        locals.var_t3__blk1115_dn7 = assign33780_e48974_d_n7;
        locals.var_t3__blk1115_dn10 = assign33780_e48974_d_n10;
        locals.var_t3__blk1115_dn11 = assign33780_e48974_d_n11;
        locals.var_t3__blk1115_dn12 = assign33780_e48974_d_n12;
        locals.var_t3__blk1115_dn17 = assign33780_e48974_d_n17;

        let (assign33790_e48980, assign33790_e48980_d_n0, assign33790_e48980_d_n2, assign33790_e48980_d_n6, assign33790_e48980_d_n7, assign33790_e48980_d_n10, assign33790_e48980_d_n11, assign33790_e48980_d_n12, assign33790_e48980_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33790_e48978: f64 = (locals.var_kusail * locals.var_kusail);
        (assign33790_e48978, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)), ((locals.var_kusail_dn17 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn17)),)
    } else {
        (locals.var_t4__blk1116, locals.var_t4__blk1116_dn0, locals.var_t4__blk1116_dn2, locals.var_t4__blk1116_dn6, locals.var_t4__blk1116_dn7, locals.var_t4__blk1116_dn10, locals.var_t4__blk1116_dn11, locals.var_t4__blk1116_dn12, locals.var_t4__blk1116_dn17,)
    }
};
        locals.var_t4__blk1116 = assign33790_e48980;
        locals.var_t4__blk1116_dn0 = assign33790_e48980_d_n0;
        locals.var_t4__blk1116_dn2 = assign33790_e48980_d_n2;
        locals.var_t4__blk1116_dn6 = assign33790_e48980_d_n6;
        locals.var_t4__blk1116_dn7 = assign33790_e48980_d_n7;
        locals.var_t4__blk1116_dn10 = assign33790_e48980_d_n10;
        locals.var_t4__blk1116_dn11 = assign33790_e48980_d_n11;
        locals.var_t4__blk1116_dn12 = assign33790_e48980_d_n12;
        locals.var_t4__blk1116_dn17 = assign33790_e48980_d_n17;

        let (assign33800_e48988, assign33800_e48988_d_n0, assign33800_e48988_d_n2, assign33800_e48988_d_n6, assign33800_e48988_d_n7, assign33800_e48988_d_n10, assign33800_e48988_d_n11, assign33800_e48988_d_n12, assign33800_e48988_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33800_e48984: f64 = (42.0 * locals.var_kusai00);
        let assign33800_e48986: f64 = (assign33800_e48984 * locals.var_kusail);
        (assign33800_e48986, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn12) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn12)), (((42.0 * locals.var_kusai00_dn17) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn17)),)
    } else {
        (locals.var_t5__blk1117, locals.var_t5__blk1117_dn0, locals.var_t5__blk1117_dn2, locals.var_t5__blk1117_dn6, locals.var_t5__blk1117_dn7, locals.var_t5__blk1117_dn10, locals.var_t5__blk1117_dn11, locals.var_t5__blk1117_dn12, locals.var_t5__blk1117_dn17,)
    }
};
        locals.var_t5__blk1117 = assign33800_e48988;
        locals.var_t5__blk1117_dn0 = assign33800_e48988_d_n0;
        locals.var_t5__blk1117_dn2 = assign33800_e48988_d_n2;
        locals.var_t5__blk1117_dn6 = assign33800_e48988_d_n6;
        locals.var_t5__blk1117_dn7 = assign33800_e48988_d_n7;
        locals.var_t5__blk1117_dn10 = assign33800_e48988_d_n10;
        locals.var_t5__blk1117_dn11 = assign33800_e48988_d_n11;
        locals.var_t5__blk1117_dn12 = assign33800_e48988_d_n12;
        locals.var_t5__blk1117_dn17 = assign33800_e48988_d_n17;

        let (assign33810_e48998, assign33810_e48998_d_n0, assign33810_e48998_d_n2, assign33810_e48998_d_n6, assign33810_e48998_d_n7, assign33810_e48998_d_n10, assign33810_e48998_d_n11, assign33810_e48998_d_n12, assign33810_e48998_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33810_e48994: f64 = (locals.var_t3__blk1115 + locals.var_t4__blk1116);
        let assign33810_e48995: f64 = (4.0 * assign33810_e48994);
        let assign33810_e48996: f64 = (locals.var_t5__blk1117 + assign33810_e48995);
        (assign33810_e48996, (locals.var_t5__blk1117_dn0 + (4.0 * (locals.var_t3__blk1115_dn0 + locals.var_t4__blk1116_dn0))), (locals.var_t5__blk1117_dn2 + (4.0 * (locals.var_t3__blk1115_dn2 + locals.var_t4__blk1116_dn2))), (locals.var_t5__blk1117_dn6 + (4.0 * (locals.var_t3__blk1115_dn6 + locals.var_t4__blk1116_dn6))), (locals.var_t5__blk1117_dn7 + (4.0 * (locals.var_t3__blk1115_dn7 + locals.var_t4__blk1116_dn7))), (locals.var_t5__blk1117_dn10 + (4.0 * (locals.var_t3__blk1115_dn10 + locals.var_t4__blk1116_dn10))), (locals.var_t5__blk1117_dn11 + (4.0 * (locals.var_t3__blk1115_dn11 + locals.var_t4__blk1116_dn11))), (locals.var_t5__blk1117_dn12 + (4.0 * (locals.var_t3__blk1115_dn12 + locals.var_t4__blk1116_dn12))), (locals.var_t5__blk1117_dn17 + (4.0 * (locals.var_t3__blk1115_dn17 + locals.var_t4__blk1116_dn17))),)
    } else {
        (locals.var_t5__blk1117, locals.var_t5__blk1117_dn0, locals.var_t5__blk1117_dn2, locals.var_t5__blk1117_dn6, locals.var_t5__blk1117_dn7, locals.var_t5__blk1117_dn10, locals.var_t5__blk1117_dn11, locals.var_t5__blk1117_dn12, locals.var_t5__blk1117_dn17,)
    }
};
        locals.var_t5__blk1117 = assign33810_e48998;
        locals.var_t5__blk1117_dn0 = assign33810_e48998_d_n0;
        locals.var_t5__blk1117_dn2 = assign33810_e48998_d_n2;
        locals.var_t5__blk1117_dn6 = assign33810_e48998_d_n6;
        locals.var_t5__blk1117_dn7 = assign33810_e48998_d_n7;
        locals.var_t5__blk1117_dn10 = assign33810_e48998_d_n10;
        locals.var_t5__blk1117_dn11 = assign33810_e48998_d_n11;
        locals.var_t5__blk1117_dn12 = assign33810_e48998_d_n12;
        locals.var_t5__blk1117_dn17 = assign33810_e48998_d_n17;

        let (assign33820_e49012, assign33820_e49012_d_n0, assign33820_e49012_d_n2, assign33820_e49012_d_n6, assign33820_e49012_d_n7, assign33820_e49012_d_n10, assign33820_e49012_d_n11, assign33820_e49012_d_n12, assign33820_e49012_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33820_e49003: f64 = (20.0 * locals.var_sqrtkusail);
        let assign33820_e49005: f64 = (assign33820_e49003 * locals.var_vgvt);
        let assign33820_e49008: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign33820_e49009: f64 = (assign33820_e49005 * assign33820_e49008);
        let assign33820_e49010: f64 = (locals.var_t5__blk1117 + assign33820_e49009);
        (assign33820_e49010, (locals.var_t5__blk1117_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn0)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5__blk1117_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn2)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5__blk1117_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn6)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5__blk1117_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn7)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5__blk1117_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn10)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5__blk1117_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn11)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5__blk1117_dn12 + (((((20.0 * locals.var_sqrtkusail_dn12) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn12)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn12 + locals.var_kusail_dn12)))), (locals.var_t5__blk1117_dn17 + (((((20.0 * locals.var_sqrtkusail_dn17) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn17)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn17 + locals.var_kusail_dn17)))),)
    } else {
        (locals.var_t5__blk1117, locals.var_t5__blk1117_dn0, locals.var_t5__blk1117_dn2, locals.var_t5__blk1117_dn6, locals.var_t5__blk1117_dn7, locals.var_t5__blk1117_dn10, locals.var_t5__blk1117_dn11, locals.var_t5__blk1117_dn12, locals.var_t5__blk1117_dn17,)
    }
};
        locals.var_t5__blk1117 = assign33820_e49012;
        locals.var_t5__blk1117_dn0 = assign33820_e49012_d_n0;
        locals.var_t5__blk1117_dn2 = assign33820_e49012_d_n2;
        locals.var_t5__blk1117_dn6 = assign33820_e49012_d_n6;
        locals.var_t5__blk1117_dn7 = assign33820_e49012_d_n7;
        locals.var_t5__blk1117_dn10 = assign33820_e49012_d_n10;
        locals.var_t5__blk1117_dn11 = assign33820_e49012_d_n11;
        locals.var_t5__blk1117_dn12 = assign33820_e49012_d_n12;
        locals.var_t5__blk1117_dn17 = assign33820_e49012_d_n17;

        let (assign33830_e49018, assign33830_e49018_d_n0, assign33830_e49018_d_n2, assign33830_e49018_d_n6, assign33830_e49018_d_n7, assign33830_e49018_d_n10, assign33830_e49018_d_n11, assign33830_e49018_d_n12, assign33830_e49018_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33830_e49016: f64 = (locals.var_t2__blk1114 * locals.var_t2__blk1114);
        (assign33830_e49016, ((locals.var_t2__blk1114_dn0 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn0)), ((locals.var_t2__blk1114_dn2 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn2)), ((locals.var_t2__blk1114_dn6 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn6)), ((locals.var_t2__blk1114_dn7 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn7)), ((locals.var_t2__blk1114_dn10 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn10)), ((locals.var_t2__blk1114_dn11 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn11)), ((locals.var_t2__blk1114_dn12 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn12)), ((locals.var_t2__blk1114_dn17 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn17)),)
    } else {
        (locals.var_t10w, locals.var_t10w_dn0, locals.var_t10w_dn2, locals.var_t10w_dn6, locals.var_t10w_dn7, locals.var_t10w_dn10, locals.var_t10w_dn11, locals.var_t10w_dn12, locals.var_t10w_dn17,)
    }
};
        locals.var_t10w = assign33830_e49018;
        locals.var_t10w_dn0 = assign33830_e49018_d_n0;
        locals.var_t10w_dn2 = assign33830_e49018_d_n2;
        locals.var_t10w_dn6 = assign33830_e49018_d_n6;
        locals.var_t10w_dn7 = assign33830_e49018_d_n7;
        locals.var_t10w_dn10 = assign33830_e49018_d_n10;
        locals.var_t10w_dn11 = assign33830_e49018_d_n11;
        locals.var_t10w_dn12 = assign33830_e49018_d_n12;
        locals.var_t10w_dn17 = assign33830_e49018_d_n17;

        let (assign33840_e49024, assign33840_e49024_d_n0, assign33840_e49024_d_n2, assign33840_e49024_d_n6, assign33840_e49024_d_n7, assign33840_e49024_d_n10, assign33840_e49024_d_n11, assign33840_e49024_d_n12, assign33840_e49024_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33840_e49022: f64 = (locals.var_t10w * locals.var_t10w);
        (assign33840_e49022, ((locals.var_t10w_dn0 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn0)), ((locals.var_t10w_dn2 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn2)), ((locals.var_t10w_dn6 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn6)), ((locals.var_t10w_dn7 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn7)), ((locals.var_t10w_dn10 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn10)), ((locals.var_t10w_dn11 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn11)), ((locals.var_t10w_dn12 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn12)), ((locals.var_t10w_dn17 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn17)),)
    } else {
        (locals.var_t10__blk1110, locals.var_t10__blk1110_dn0, locals.var_t10__blk1110_dn2, locals.var_t10__blk1110_dn6, locals.var_t10__blk1110_dn7, locals.var_t10__blk1110_dn10, locals.var_t10__blk1110_dn11, locals.var_t10__blk1110_dn12, locals.var_t10__blk1110_dn17,)
    }
};
        locals.var_t10__blk1110 = assign33840_e49024;
        locals.var_t10__blk1110_dn0 = assign33840_e49024_d_n0;
        locals.var_t10__blk1110_dn2 = assign33840_e49024_d_n2;
        locals.var_t10__blk1110_dn6 = assign33840_e49024_d_n6;
        locals.var_t10__blk1110_dn7 = assign33840_e49024_d_n7;
        locals.var_t10__blk1110_dn10 = assign33840_e49024_d_n10;
        locals.var_t10__blk1110_dn11 = assign33840_e49024_d_n11;
        locals.var_t10__blk1110_dn12 = assign33840_e49024_d_n12;
        locals.var_t10__blk1110_dn17 = assign33840_e49024_d_n17;

        let (assign33850_e49032, assign33850_e49032_d_n0, assign33850_e49032_d_n2, assign33850_e49032_d_n6, assign33850_e49032_d_n7, assign33850_e49032_d_n10, assign33850_e49032_d_n11, assign33850_e49032_d_n12, assign33850_e49032_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33850_e49029: f64 = (locals.var_t10__blk1110 * locals.var_t2__blk1114);
        let assign33850_e49030: f64 = (locals.var_t5__blk1117 / assign33850_e49029);
        (assign33850_e49030, (((locals.var_t5__blk1117_dn0 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn0 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn0)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn2 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn2 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn2)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn6 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn6 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn6)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn7 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn7 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn7)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn10 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn10 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn10)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn11 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn11 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn11)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn12 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn12 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn12)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn17 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn17 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn17)))) / (assign33850_e49029 * assign33850_e49029)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn12, locals.var_kusai_ig_dn17,)
    }
};
        locals.var_kusai_ig = assign33850_e49032;
        locals.var_kusai_ig_dn0 = assign33850_e49032_d_n0;
        locals.var_kusai_ig_dn2 = assign33850_e49032_d_n2;
        locals.var_kusai_ig_dn6 = assign33850_e49032_d_n6;
        locals.var_kusai_ig_dn7 = assign33850_e49032_d_n7;
        locals.var_kusai_ig_dn10 = assign33850_e49032_d_n10;
        locals.var_kusai_ig_dn11 = assign33850_e49032_d_n11;
        locals.var_kusai_ig_dn12 = assign33850_e49032_d_n12;
        locals.var_kusai_ig_dn17 = assign33850_e49032_d_n17;

        let (assign33860_e49042, assign33860_e49042_d_n0, assign33860_e49042_d_n2, assign33860_e49042_d_n6, assign33860_e49042_d_n7, assign33860_e49042_d_n10, assign33860_e49042_d_n11, assign33860_e49042_d_n12, assign33860_e49042_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33860_e49036: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign33860_e49038: f64 = (assign33860_e49036 * locals.var_mu);
        let assign33860_e49040: f64 = (assign33860_e49038 * locals.var_c_fox);
        (assign33860_e49040, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn0)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn2)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn6)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn7)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn10)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn11) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn11)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn11)), (((((-((locals.var_weff_nf * locals.var_lch_dn12) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn12)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn12)), (((((-((locals.var_weff_nf * locals.var_lch_dn17) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn17)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn12, locals.var_gds0_ign_dn17,)
    }
};
        locals.var_gds0_ign = assign33860_e49042;
        locals.var_gds0_ign_dn0 = assign33860_e49042_d_n0;
        locals.var_gds0_ign_dn2 = assign33860_e49042_d_n2;
        locals.var_gds0_ign_dn6 = assign33860_e49042_d_n6;
        locals.var_gds0_ign_dn7 = assign33860_e49042_d_n7;
        locals.var_gds0_ign_dn10 = assign33860_e49042_d_n10;
        locals.var_gds0_ign_dn11 = assign33860_e49042_d_n11;
        locals.var_gds0_ign_dn12 = assign33860_e49042_d_n12;
        locals.var_gds0_ign_dn17 = assign33860_e49042_d_n17;

        let (assign33870_e49048, assign33870_e49048_d_n0, assign33870_e49048_d_n2, assign33870_e49048_d_n6, assign33870_e49048_d_n7, assign33870_e49048_d_n10, assign33870_e49048_d_n11, assign33870_e49048_d_n12, assign33870_e49048_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33870_e49046: f64 = (locals.var_gds0_ign * locals.var_vgvt);
        (assign33870_e49046, ((locals.var_gds0_ign_dn0 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn0)), ((locals.var_gds0_ign_dn2 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn2)), ((locals.var_gds0_ign_dn6 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn6)), ((locals.var_gds0_ign_dn7 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn7)), ((locals.var_gds0_ign_dn10 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn10)), ((locals.var_gds0_ign_dn11 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn11)), ((locals.var_gds0_ign_dn12 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn12)), ((locals.var_gds0_ign_dn17 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn17)),)
    } else {
        (locals.var_gds0_h2, locals.var_gds0_h2_dn0, locals.var_gds0_h2_dn2, locals.var_gds0_h2_dn6, locals.var_gds0_h2_dn7, locals.var_gds0_h2_dn10, locals.var_gds0_h2_dn11, locals.var_gds0_h2_dn12, locals.var_gds0_h2_dn17,)
    }
};
        locals.var_gds0_h2 = assign33870_e49048;
        locals.var_gds0_h2_dn0 = assign33870_e49048_d_n0;
        locals.var_gds0_h2_dn2 = assign33870_e49048_d_n2;
        locals.var_gds0_h2_dn6 = assign33870_e49048_d_n6;
        locals.var_gds0_h2_dn7 = assign33870_e49048_d_n7;
        locals.var_gds0_h2_dn10 = assign33870_e49048_d_n10;
        locals.var_gds0_h2_dn11 = assign33870_e49048_d_n11;
        locals.var_gds0_h2_dn12 = assign33870_e49048_d_n12;
        locals.var_gds0_h2_dn17 = assign33870_e49048_d_n17;

        let (assign33880_e49054, assign33880_e49054_d_n0, assign33880_e49054_d_n2, assign33880_e49054_d_n6, assign33880_e49054_d_n7, assign33880_e49054_d_n10, assign33880_e49054_d_n11, assign33880_e49054_d_n12, assign33880_e49054_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33880_e49052: f64 = (locals.var_nthrml / locals.var_gds0_h2);
        (assign33880_e49052, (((locals.var_nthrml_dn0 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn0)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn2 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn2)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn6 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn6)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn7 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn7)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn10 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn10)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn11 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn11)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn12 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn12)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn17 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn17)) / (locals.var_gds0_h2 * locals.var_gds0_h2)),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn10, locals.var_gamma_dn11, locals.var_gamma_dn12, locals.var_gamma_dn17,)
    }
};
        locals.var_gamma = assign33880_e49054;
        locals.var_gamma_dn0 = assign33880_e49054_d_n0;
        locals.var_gamma_dn2 = assign33880_e49054_d_n2;
        locals.var_gamma_dn6 = assign33880_e49054_d_n6;
        locals.var_gamma_dn7 = assign33880_e49054_d_n7;
        locals.var_gamma_dn10 = assign33880_e49054_d_n10;
        locals.var_gamma_dn11 = assign33880_e49054_d_n11;
        locals.var_gamma_dn12 = assign33880_e49054_d_n12;
        locals.var_gamma_dn17 = assign33880_e49054_d_n17;

        let (assign33890_e49066, assign33890_e49066_d_n0, assign33890_e49066_d_n2, assign33890_e49066_d_n6, assign33890_e49066_d_n7, assign33890_e49066_d_n10, assign33890_e49066_d_n11, assign33890_e49066_d_n12, assign33890_e49066_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33890_e49059: f64 = (4.0 * locals.var_vgvt);
        let assign33890_e49061: f64 = (assign33890_e49059 * locals.var_sqrtkusail);
        let assign33890_e49062: f64 = (locals.var_kusai00 + assign33890_e49061);
        let assign33890_e49064: f64 = (assign33890_e49062 + locals.var_kusail);
        (assign33890_e49064, ((locals.var_kusai00_dn0 + (((4.0 * locals.var_vgvt_dn0) * locals.var_sqrtkusail) + (assign33890_e49059 * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0), ((locals.var_kusai00_dn2 + (((4.0 * locals.var_vgvt_dn2) * locals.var_sqrtkusail) + (assign33890_e49059 * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2), ((locals.var_kusai00_dn6 + (((4.0 * locals.var_vgvt_dn6) * locals.var_sqrtkusail) + (assign33890_e49059 * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6), ((locals.var_kusai00_dn7 + (((4.0 * locals.var_vgvt_dn7) * locals.var_sqrtkusail) + (assign33890_e49059 * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7), ((locals.var_kusai00_dn10 + (((4.0 * locals.var_vgvt_dn10) * locals.var_sqrtkusail) + (assign33890_e49059 * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10), ((locals.var_kusai00_dn11 + (((4.0 * locals.var_vgvt_dn11) * locals.var_sqrtkusail) + (assign33890_e49059 * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11), ((locals.var_kusai00_dn12 + (((4.0 * locals.var_vgvt_dn12) * locals.var_sqrtkusail) + (assign33890_e49059 * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12), ((locals.var_kusai00_dn17 + (((4.0 * locals.var_vgvt_dn17) * locals.var_sqrtkusail) + (assign33890_e49059 * locals.var_sqrtkusail_dn17))) + locals.var_kusail_dn17),)
    } else {
        (locals.var_t7w, locals.var_t7w_dn0, locals.var_t7w_dn2, locals.var_t7w_dn6, locals.var_t7w_dn7, locals.var_t7w_dn10, locals.var_t7w_dn11, locals.var_t7w_dn12, locals.var_t7w_dn17,)
    }
};
        locals.var_t7w = assign33890_e49066;
        locals.var_t7w_dn0 = assign33890_e49066_d_n0;
        locals.var_t7w_dn2 = assign33890_e49066_d_n2;
        locals.var_t7w_dn6 = assign33890_e49066_d_n6;
        locals.var_t7w_dn7 = assign33890_e49066_d_n7;
        locals.var_t7w_dn10 = assign33890_e49066_d_n10;
        locals.var_t7w_dn11 = assign33890_e49066_d_n11;
        locals.var_t7w_dn12 = assign33890_e49066_d_n12;
        locals.var_t7w_dn17 = assign33890_e49066_d_n17;

        let (assign33900_e49087, assign33900_e49087_d_n0, assign33900_e49087_d_n2, assign33900_e49087_d_n6, assign33900_e49087_d_n7, assign33900_e49087_d_n10, assign33900_e49087_d_n11, assign33900_e49087_d_n12, assign33900_e49087_d_n17,) = {
    if (locals.var_guard1125 != 0.0) {
        let assign33900_e49070: f64 = (3.872983346207417 * locals.var_kusai00l);
        let assign33900_e49072: f64 = (assign33900_e49070 * locals.var_t7w);
        let assign33900_e49075: f64 = (6.0 * locals.var_t2__blk1114);
        let assign33900_e49078: f64 = (locals.var_gamma * locals.var_t2__blk1114);
        let assign33900_e49080: f64 = (assign33900_e49078 * locals.var_vgvt);
        let assign33900_e49082: f64 = (assign33900_e49080 * locals.var_t5__blk1117);
        let assign33900_e49083: f64 = (assign33900_e49082).sqrt();
        let assign33900_e49084: f64 = (assign33900_e49075 * assign33900_e49083);
        let assign33900_e49085: f64 = (assign33900_e49072 / assign33900_e49084);
        (assign33900_e49085, ((((((3.872983346207417 * locals.var_kusai00l_dn0) * locals.var_t7w) + (assign33900_e49070 * locals.var_t7w_dn0)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * locals.var_t2__blk1114_dn0) * assign33900_e49083) + (assign33900_e49075 * (((((((locals.var_gamma_dn0 * locals.var_t2__blk1114) + (locals.var_gamma * locals.var_t2__blk1114_dn0)) * locals.var_vgvt) + (assign33900_e49078 * locals.var_vgvt_dn0)) * locals.var_t5__blk1117) + (assign33900_e49080 * locals.var_t5__blk1117_dn0)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * locals.var_kusai00l_dn2) * locals.var_t7w) + (assign33900_e49070 * locals.var_t7w_dn2)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * locals.var_t2__blk1114_dn2) * assign33900_e49083) + (assign33900_e49075 * (((((((locals.var_gamma_dn2 * locals.var_t2__blk1114) + (locals.var_gamma * locals.var_t2__blk1114_dn2)) * locals.var_vgvt) + (assign33900_e49078 * locals.var_vgvt_dn2)) * locals.var_t5__blk1117) + (assign33900_e49080 * locals.var_t5__blk1117_dn2)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * locals.var_kusai00l_dn6) * locals.var_t7w) + (assign33900_e49070 * locals.var_t7w_dn6)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * locals.var_t2__blk1114_dn6) * assign33900_e49083) + (assign33900_e49075 * (((((((locals.var_gamma_dn6 * locals.var_t2__blk1114) + (locals.var_gamma * locals.var_t2__blk1114_dn6)) * locals.var_vgvt) + (assign33900_e49078 * locals.var_vgvt_dn6)) * locals.var_t5__blk1117) + (assign33900_e49080 * locals.var_t5__blk1117_dn6)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * locals.var_kusai00l_dn7) * locals.var_t7w) + (assign33900_e49070 * locals.var_t7w_dn7)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * locals.var_t2__blk1114_dn7) * assign33900_e49083) + (assign33900_e49075 * (((((((locals.var_gamma_dn7 * locals.var_t2__blk1114) + (locals.var_gamma * locals.var_t2__blk1114_dn7)) * locals.var_vgvt) + (assign33900_e49078 * locals.var_vgvt_dn7)) * locals.var_t5__blk1117) + (assign33900_e49080 * locals.var_t5__blk1117_dn7)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * locals.var_kusai00l_dn10) * locals.var_t7w) + (assign33900_e49070 * locals.var_t7w_dn10)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * locals.var_t2__blk1114_dn10) * assign33900_e49083) + (assign33900_e49075 * (((((((locals.var_gamma_dn10 * locals.var_t2__blk1114) + (locals.var_gamma * locals.var_t2__blk1114_dn10)) * locals.var_vgvt) + (assign33900_e49078 * locals.var_vgvt_dn10)) * locals.var_t5__blk1117) + (assign33900_e49080 * locals.var_t5__blk1117_dn10)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * locals.var_kusai00l_dn11) * locals.var_t7w) + (assign33900_e49070 * locals.var_t7w_dn11)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * locals.var_t2__blk1114_dn11) * assign33900_e49083) + (assign33900_e49075 * (((((((locals.var_gamma_dn11 * locals.var_t2__blk1114) + (locals.var_gamma * locals.var_t2__blk1114_dn11)) * locals.var_vgvt) + (assign33900_e49078 * locals.var_vgvt_dn11)) * locals.var_t5__blk1117) + (assign33900_e49080 * locals.var_t5__blk1117_dn11)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * locals.var_kusai00l_dn12) * locals.var_t7w) + (assign33900_e49070 * locals.var_t7w_dn12)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * locals.var_t2__blk1114_dn12) * assign33900_e49083) + (assign33900_e49075 * (((((((locals.var_gamma_dn12 * locals.var_t2__blk1114) + (locals.var_gamma * locals.var_t2__blk1114_dn12)) * locals.var_vgvt) + (assign33900_e49078 * locals.var_vgvt_dn12)) * locals.var_t5__blk1117) + (assign33900_e49080 * locals.var_t5__blk1117_dn12)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * locals.var_kusai00l_dn17) * locals.var_t7w) + (assign33900_e49070 * locals.var_t7w_dn17)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * locals.var_t2__blk1114_dn17) * assign33900_e49083) + (assign33900_e49075 * (((((((locals.var_gamma_dn17 * locals.var_t2__blk1114) + (locals.var_gamma * locals.var_t2__blk1114_dn17)) * locals.var_vgvt) + (assign33900_e49078 * locals.var_vgvt_dn17)) * locals.var_t5__blk1117) + (assign33900_e49080 * locals.var_t5__blk1117_dn17)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)),)
    } else {
        (locals.var_crl_f, locals.var_crl_f_dn0, locals.var_crl_f_dn2, locals.var_crl_f_dn6, locals.var_crl_f_dn7, locals.var_crl_f_dn10, locals.var_crl_f_dn11, locals.var_crl_f_dn12, locals.var_crl_f_dn17,)
    }
};
        locals.var_crl_f = assign33900_e49087;
        locals.var_crl_f_dn0 = assign33900_e49087_d_n0;
        locals.var_crl_f_dn2 = assign33900_e49087_d_n2;
        locals.var_crl_f_dn6 = assign33900_e49087_d_n6;
        locals.var_crl_f_dn7 = assign33900_e49087_d_n7;
        locals.var_crl_f_dn10 = assign33900_e49087_d_n10;
        locals.var_crl_f_dn11 = assign33900_e49087_d_n11;
        locals.var_crl_f_dn12 = assign33900_e49087_d_n12;
        locals.var_crl_f_dn17 = assign33900_e49087_d_n17;

        let assign33910_e49090: f64 = (locals.var_ids + locals.var_idsibpc);
        locals.var_ids = assign33910_e49090;
        locals.var_ids_dn0 = (locals.var_ids_dn0 + locals.var_idsibpc_dn0);
        locals.var_ids_dn2 = (locals.var_ids_dn2 + locals.var_idsibpc_dn2);
        locals.var_ids_dn6 = (locals.var_ids_dn6 + locals.var_idsibpc_dn6);
        locals.var_ids_dn7 = (locals.var_ids_dn7 + locals.var_idsibpc_dn7);
        locals.var_ids_dn10 = (locals.var_ids_dn10 + locals.var_idsibpc_dn10);
        locals.var_ids_dn11 = (locals.var_ids_dn11 + locals.var_idsibpc_dn11);
        locals.var_ids_dn12 = (locals.var_ids_dn12 + locals.var_idsibpc_dn12);
        locals.var_ids_dn17 = (locals.var_ids_dn17 + locals.var_idsibpc_dn17);

        let assign33920_e49093: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1126 = assign33920_e49093;

        let (assign33930_e49099,) = {
    if (locals.var_guard1126 != 0.0) {
        let assign33930_e49097: f64 = (locals.var_cbtp + locals.var_cbtn);
        (assign33930_e49097,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign33930_e49099;

        let (assign33940_e49109,) = {
    if ((locals.var_guard1126 != 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign33940_e49106: f64 = (p.p168 * locals.var_lgleff);
        let assign33940_e49107: f64 = (locals.var_cgbe - assign33940_e49106);
        (assign33940_e49107,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign33940_e49109;

        let (assign33950_e49118, assign33950_e49118_d_n0, assign33950_e49118_d_n2, assign33950_e49118_d_n6, assign33950_e49118_d_n7, assign33950_e49118_d_n10, assign33950_e49118_d_n11, assign33950_e49118_d_n12, assign33950_e49118_d_n17,) = {
    if (locals.var_guard1126 != 0.0) {
        let assign33950_e49112: f64 = (-locals.var_cgbe);
        let assign33950_e49115: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign33950_e49116: f64 = (assign33950_e49112 * assign33950_e49115);
        (assign33950_e49116, (assign33950_e49112 * (-locals.var_vbsp_dn0)), (assign33950_e49112 * (-locals.var_vbsp_dn2)), (assign33950_e49112 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33950_e49112 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33950_e49112 * (-locals.var_vbsp_dn10)), (assign33950_e49112 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33950_e49112 * (-locals.var_vbsp_dn12)), (assign33950_e49112 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign33950_e49118;
        locals.var_qgob_dn0 = assign33950_e49118_d_n0;
        locals.var_qgob_dn2 = assign33950_e49118_d_n2;
        locals.var_qgob_dn6 = assign33950_e49118_d_n6;
        locals.var_qgob_dn7 = assign33950_e49118_d_n7;
        locals.var_qgob_dn10 = assign33950_e49118_d_n10;
        locals.var_qgob_dn11 = assign33950_e49118_d_n11;
        locals.var_qgob_dn12 = assign33950_e49118_d_n12;
        locals.var_qgob_dn17 = assign33950_e49118_d_n17;

        let (assign33960_e49128,) = {
    if (locals.var_guard1126 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cfu,)
    }
};
        locals.var_cfu = assign33960_e49128;

        let (assign33970_e49138,) = {
    if (locals.var_guard1126 != 0.0) {
        let assign33970_e49132: f64 = (locals.var_cfu * p.p9);
        let assign33970_e49135: f64 = (locals.var_wgate + locals.var_uc_pdbcp);
        let assign33970_e49136: f64 = (assign33970_e49132 * assign33970_e49135);
        (assign33970_e49136,)
    } else {
        (locals.var_cfd,)
    }
};
        locals.var_cfd = assign33970_e49138;

        let (assign33980_e49148,) = {
    if (locals.var_guard1126 != 0.0) {
        let assign33980_e49142: f64 = (locals.var_cfu * p.p9);
        let assign33980_e49145: f64 = (locals.var_wgate + locals.var_uc_psbcp);
        let assign33980_e49146: f64 = (assign33980_e49142 * assign33980_e49145);
        (assign33980_e49146,)
    } else {
        (locals.var_cfs,)
    }
};
        locals.var_cfs = assign33980_e49148;

    }

    pub(super) fn stamp_transient_block_119(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33990_e49156, assign33990_e49156_d_n0, assign33990_e49156_d_n2, assign33990_e49156_d_n6, assign33990_e49156_d_n7, assign33990_e49156_d_n10, assign33990_e49156_d_n11, assign33990_e49156_d_n12, assign33990_e49156_d_n17,) = {
    if (locals.var_guard1126 != 0.0) {
        let assign33990_e49153: f64 = (locals.var_vgs - locals.var_vds);
        let assign33990_e49154: f64 = (locals.var_cfd * assign33990_e49153);
        (assign33990_e49154, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17,)
    }
};
        locals.var_qfd = assign33990_e49156;
        locals.var_qfd_dn0 = assign33990_e49156_d_n0;
        locals.var_qfd_dn2 = assign33990_e49156_d_n2;
        locals.var_qfd_dn6 = assign33990_e49156_d_n6;
        locals.var_qfd_dn7 = assign33990_e49156_d_n7;
        locals.var_qfd_dn10 = assign33990_e49156_d_n10;
        locals.var_qfd_dn11 = assign33990_e49156_d_n11;
        locals.var_qfd_dn12 = assign33990_e49156_d_n12;
        locals.var_qfd_dn17 = assign33990_e49156_d_n17;

        let (assign34000_e49162, assign34000_e49162_d_n6, assign34000_e49162_d_n7, assign34000_e49162_d_n11,) = {
    if (locals.var_guard1126 != 0.0) {
        let assign34000_e49160: f64 = (locals.var_cfs * locals.var_vgs);
        (assign34000_e49160, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11),)
    } else {
        (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11,)
    }
};
        locals.var_qfs = assign34000_e49162;
        locals.var_qfs_dn6 = assign34000_e49162_d_n6;
        locals.var_qfs_dn7 = assign34000_e49162_d_n7;
        locals.var_qfs_dn11 = assign34000_e49162_d_n11;

        let (assign34010_e49174, assign34010_e49174_d_n0, assign34010_e49174_d_n2, assign34010_e49174_d_n6, assign34010_e49174_d_n7, assign34010_e49174_d_n10, assign34010_e49174_d_n11, assign34010_e49174_d_n12, assign34010_e49174_d_n17,) = {
    if (locals.var_guard1126 != 0.0) {
        let assign34010_e49166: f64 = (locals.var_cfu * p.p19);
        let assign34010_e49168: f64 = (assign34010_e49166 * p.p9);
        let assign34010_e49171: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign34010_e49172: f64 = (assign34010_e49168 * assign34010_e49171);
        (assign34010_e49172, (assign34010_e49168 * (-locals.var_vbsp_dn0)), (assign34010_e49168 * (-locals.var_vbsp_dn2)), (assign34010_e49168 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34010_e49168 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34010_e49168 * (-locals.var_vbsp_dn10)), (assign34010_e49168 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34010_e49168 * (-locals.var_vbsp_dn12)), (assign34010_e49168 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qfbc, locals.var_qfbc_dn0, locals.var_qfbc_dn2, locals.var_qfbc_dn6, locals.var_qfbc_dn7, locals.var_qfbc_dn10, locals.var_qfbc_dn11, locals.var_qfbc_dn12, locals.var_qfbc_dn17,)
    }
};
        locals.var_qfbc = assign34010_e49174;
        locals.var_qfbc_dn0 = assign34010_e49174_d_n0;
        locals.var_qfbc_dn2 = assign34010_e49174_d_n2;
        locals.var_qfbc_dn6 = assign34010_e49174_d_n6;
        locals.var_qfbc_dn7 = assign34010_e49174_d_n7;
        locals.var_qfbc_dn10 = assign34010_e49174_d_n10;
        locals.var_qfbc_dn11 = assign34010_e49174_d_n11;
        locals.var_qfbc_dn12 = assign34010_e49174_d_n12;
        locals.var_qfbc_dn17 = assign34010_e49174_d_n17;

        let (assign34020_e49180, assign34020_e49180_d_n0, assign34020_e49180_d_n2, assign34020_e49180_d_n6, assign34020_e49180_d_n7, assign34020_e49180_d_n10, assign34020_e49180_d_n11, assign34020_e49180_d_n12, assign34020_e49180_d_n17,) = {
    if (locals.var_guard1126 != 0.0) {
        let assign34020_e49178: f64 = (locals.var_qgod + locals.var_qfd);
        (assign34020_e49178, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign34020_e49180;
        locals.var_qgod_dn0 = assign34020_e49180_d_n0;
        locals.var_qgod_dn2 = assign34020_e49180_d_n2;
        locals.var_qgod_dn6 = assign34020_e49180_d_n6;
        locals.var_qgod_dn7 = assign34020_e49180_d_n7;
        locals.var_qgod_dn10 = assign34020_e49180_d_n10;
        locals.var_qgod_dn11 = assign34020_e49180_d_n11;
        locals.var_qgod_dn12 = assign34020_e49180_d_n12;
        locals.var_qgod_dn17 = assign34020_e49180_d_n17;

        let (assign34030_e49186, assign34030_e49186_d_n0, assign34030_e49186_d_n2, assign34030_e49186_d_n6, assign34030_e49186_d_n7, assign34030_e49186_d_n10, assign34030_e49186_d_n11, assign34030_e49186_d_n12, assign34030_e49186_d_n17,) = {
    if (locals.var_guard1126 != 0.0) {
        let assign34030_e49184: f64 = (locals.var_qgos + locals.var_qfs);
        (assign34030_e49184, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17,)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign34030_e49186;
        locals.var_qgos_dn0 = assign34030_e49186_d_n0;
        locals.var_qgos_dn2 = assign34030_e49186_d_n2;
        locals.var_qgos_dn6 = assign34030_e49186_d_n6;
        locals.var_qgos_dn7 = assign34030_e49186_d_n7;
        locals.var_qgos_dn10 = assign34030_e49186_d_n10;
        locals.var_qgos_dn11 = assign34030_e49186_d_n11;
        locals.var_qgos_dn12 = assign34030_e49186_d_n12;
        locals.var_qgos_dn17 = assign34030_e49186_d_n17;

        let (assign34040_e49192, assign34040_e49192_d_n0, assign34040_e49192_d_n2, assign34040_e49192_d_n6, assign34040_e49192_d_n7, assign34040_e49192_d_n10, assign34040_e49192_d_n11, assign34040_e49192_d_n12, assign34040_e49192_d_n17,) = {
    if (locals.var_guard1126 != 0.0) {
        let assign34040_e49190: f64 = (locals.var_qgob + locals.var_qfbc);
        (assign34040_e49190, (locals.var_qgob_dn0 + locals.var_qfbc_dn0), (locals.var_qgob_dn2 + locals.var_qfbc_dn2), (locals.var_qgob_dn6 + locals.var_qfbc_dn6), (locals.var_qgob_dn7 + locals.var_qfbc_dn7), (locals.var_qgob_dn10 + locals.var_qfbc_dn10), (locals.var_qgob_dn11 + locals.var_qfbc_dn11), (locals.var_qgob_dn12 + locals.var_qfbc_dn12), (locals.var_qgob_dn17 + locals.var_qfbc_dn17),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34040_e49192;
        locals.var_qgob_dn0 = assign34040_e49192_d_n0;
        locals.var_qgob_dn2 = assign34040_e49192_d_n2;
        locals.var_qgob_dn6 = assign34040_e49192_d_n6;
        locals.var_qgob_dn7 = assign34040_e49192_d_n7;
        locals.var_qgob_dn10 = assign34040_e49192_d_n10;
        locals.var_qgob_dn11 = assign34040_e49192_d_n11;
        locals.var_qgob_dn12 = assign34040_e49192_d_n12;
        locals.var_qgob_dn17 = assign34040_e49192_d_n17;

        let (assign34050_e49202,) = {
    if ((locals.var_guard1126 == 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign34050_e49198: f64 = (-p.p168);
        let assign34050_e49200: f64 = (assign34050_e49198 * locals.var_lgleff);
        (assign34050_e49200,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign34050_e49202;

        let (assign34060_e49214, assign34060_e49214_d_n0, assign34060_e49214_d_n2, assign34060_e49214_d_n6, assign34060_e49214_d_n7, assign34060_e49214_d_n10, assign34060_e49214_d_n11, assign34060_e49214_d_n12, assign34060_e49214_d_n17,) = {
    if ((locals.var_guard1126 == 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign34060_e49208: f64 = (-locals.var_cgbe);
        let assign34060_e49211: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign34060_e49212: f64 = (assign34060_e49208 * assign34060_e49211);
        (assign34060_e49212, (assign34060_e49208 * (-locals.var_vbsp_dn0)), (assign34060_e49208 * (-locals.var_vbsp_dn2)), (assign34060_e49208 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34060_e49208 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34060_e49208 * (-locals.var_vbsp_dn10)), (assign34060_e49208 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34060_e49208 * (-locals.var_vbsp_dn12)), (assign34060_e49208 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34060_e49214;
        locals.var_qgob_dn0 = assign34060_e49214_d_n0;
        locals.var_qgob_dn2 = assign34060_e49214_d_n2;
        locals.var_qgob_dn6 = assign34060_e49214_d_n6;
        locals.var_qgob_dn7 = assign34060_e49214_d_n7;
        locals.var_qgob_dn10 = assign34060_e49214_d_n10;
        locals.var_qgob_dn11 = assign34060_e49214_d_n11;
        locals.var_qgob_dn12 = assign34060_e49214_d_n12;
        locals.var_qgob_dn17 = assign34060_e49214_d_n17;

        let (assign34070_e49222,) = {
    if ((locals.var_guard1126 == 0.0) && (locals.var_cgbo_given == 0.0)) {
        (0.0,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign34070_e49222;

        let (assign34080_e49230, assign34080_e49230_d_n0, assign34080_e49230_d_n2, assign34080_e49230_d_n6, assign34080_e49230_d_n7, assign34080_e49230_d_n10, assign34080_e49230_d_n11, assign34080_e49230_d_n12, assign34080_e49230_d_n17,) = {
    if ((locals.var_guard1126 == 0.0) && (locals.var_cgbo_given == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34080_e49230;
        locals.var_qgob_dn0 = assign34080_e49230_d_n0;
        locals.var_qgob_dn2 = assign34080_e49230_d_n2;
        locals.var_qgob_dn6 = assign34080_e49230_d_n6;
        locals.var_qgob_dn7 = assign34080_e49230_d_n7;
        locals.var_qgob_dn10 = assign34080_e49230_d_n10;
        locals.var_qgob_dn11 = assign34080_e49230_d_n11;
        locals.var_qgob_dn12 = assign34080_e49230_d_n12;
        locals.var_qgob_dn17 = assign34080_e49230_d_n17;

        let (assign34090_e49245,) = {
    if (locals.var_guard1126 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cf,)
    }
};
        locals.var_cf = assign34090_e49245;

        let (assign34100_e49250,) = {
    if (locals.var_guard1126 == 0.0) {
        (locals.var_cf,)
    } else {
        (locals.var_cfd,)
    }
};
        locals.var_cfd = assign34100_e49250;

        let (assign34110_e49255,) = {
    if (locals.var_guard1126 == 0.0) {
        (locals.var_cf,)
    } else {
        (locals.var_cfs,)
    }
};
        locals.var_cfs = assign34110_e49255;

        let (assign34120_e49264, assign34120_e49264_d_n0, assign34120_e49264_d_n2, assign34120_e49264_d_n6, assign34120_e49264_d_n7, assign34120_e49264_d_n10, assign34120_e49264_d_n11, assign34120_e49264_d_n12, assign34120_e49264_d_n17,) = {
    if (locals.var_guard1126 == 0.0) {
        let assign34120_e49261: f64 = (locals.var_vgs - locals.var_vds);
        let assign34120_e49262: f64 = (locals.var_cfd * assign34120_e49261);
        (assign34120_e49262, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17,)
    }
};
        locals.var_qfd = assign34120_e49264;
        locals.var_qfd_dn0 = assign34120_e49264_d_n0;
        locals.var_qfd_dn2 = assign34120_e49264_d_n2;
        locals.var_qfd_dn6 = assign34120_e49264_d_n6;
        locals.var_qfd_dn7 = assign34120_e49264_d_n7;
        locals.var_qfd_dn10 = assign34120_e49264_d_n10;
        locals.var_qfd_dn11 = assign34120_e49264_d_n11;
        locals.var_qfd_dn12 = assign34120_e49264_d_n12;
        locals.var_qfd_dn17 = assign34120_e49264_d_n17;

        let (assign34130_e49271, assign34130_e49271_d_n6, assign34130_e49271_d_n7, assign34130_e49271_d_n11,) = {
    if (locals.var_guard1126 == 0.0) {
        let assign34130_e49269: f64 = (locals.var_cfs * locals.var_vgs);
        (assign34130_e49269, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11),)
    } else {
        (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11,)
    }
};
        locals.var_qfs = assign34130_e49271;
        locals.var_qfs_dn6 = assign34130_e49271_d_n6;
        locals.var_qfs_dn7 = assign34130_e49271_d_n7;
        locals.var_qfs_dn11 = assign34130_e49271_d_n11;

        let (assign34140_e49278, assign34140_e49278_d_n0, assign34140_e49278_d_n2, assign34140_e49278_d_n6, assign34140_e49278_d_n7, assign34140_e49278_d_n10, assign34140_e49278_d_n11, assign34140_e49278_d_n12, assign34140_e49278_d_n17,) = {
    if (locals.var_guard1126 == 0.0) {
        let assign34140_e49276: f64 = (locals.var_qgod + locals.var_qfd);
        (assign34140_e49276, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign34140_e49278;
        locals.var_qgod_dn0 = assign34140_e49278_d_n0;
        locals.var_qgod_dn2 = assign34140_e49278_d_n2;
        locals.var_qgod_dn6 = assign34140_e49278_d_n6;
        locals.var_qgod_dn7 = assign34140_e49278_d_n7;
        locals.var_qgod_dn10 = assign34140_e49278_d_n10;
        locals.var_qgod_dn11 = assign34140_e49278_d_n11;
        locals.var_qgod_dn12 = assign34140_e49278_d_n12;
        locals.var_qgod_dn17 = assign34140_e49278_d_n17;

        let (assign34150_e49285, assign34150_e49285_d_n0, assign34150_e49285_d_n2, assign34150_e49285_d_n6, assign34150_e49285_d_n7, assign34150_e49285_d_n10, assign34150_e49285_d_n11, assign34150_e49285_d_n12, assign34150_e49285_d_n17,) = {
    if (locals.var_guard1126 == 0.0) {
        let assign34150_e49283: f64 = (locals.var_qgos + locals.var_qfs);
        (assign34150_e49283, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17,)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign34150_e49285;
        locals.var_qgos_dn0 = assign34150_e49285_d_n0;
        locals.var_qgos_dn2 = assign34150_e49285_d_n2;
        locals.var_qgos_dn6 = assign34150_e49285_d_n6;
        locals.var_qgos_dn7 = assign34150_e49285_d_n7;
        locals.var_qgos_dn10 = assign34150_e49285_d_n10;
        locals.var_qgos_dn11 = assign34150_e49285_d_n11;
        locals.var_qgos_dn12 = assign34150_e49285_d_n12;
        locals.var_qgos_dn17 = assign34150_e49285_d_n17;

        let assign34160_e49288: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign34160_e49288;
        locals.var_idse_dn0 = (locals.var_mfactor * locals.var_ids_dn0);
        locals.var_idse_dn2 = (locals.var_mfactor * locals.var_ids_dn2);
        locals.var_idse_dn6 = (locals.var_mfactor * locals.var_ids_dn6);
        locals.var_idse_dn7 = (locals.var_mfactor * locals.var_ids_dn7);
        locals.var_idse_dn10 = (locals.var_mfactor * locals.var_ids_dn10);
        locals.var_idse_dn11 = (locals.var_mfactor * locals.var_ids_dn11);
        locals.var_idse_dn12 = (locals.var_mfactor * locals.var_ids_dn12);
        locals.var_idse_dn17 = (locals.var_mfactor * locals.var_ids_dn17);

        let (assign34170_e49292, assign34170_e49292_d_n0, assign34170_e49292_d_n2, assign34170_e49292_d_n6, assign34170_e49292_d_n7, assign34170_e49292_d_n10, assign34170_e49292_d_n11, assign34170_e49292_d_n12, assign34170_e49292_d_n13, assign34170_e49292_d_n15, assign34170_e49292_d_n16, assign34170_e49292_d_n17, assign34170_e49292_d_n18,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34170_e49292;
        locals.var_qde_dn0 = assign34170_e49292_d_n0;
        locals.var_qde_dn2 = assign34170_e49292_d_n2;
        locals.var_qde_dn6 = assign34170_e49292_d_n6;
        locals.var_qde_dn7 = assign34170_e49292_d_n7;
        locals.var_qde_dn10 = assign34170_e49292_d_n10;
        locals.var_qde_dn11 = assign34170_e49292_d_n11;
        locals.var_qde_dn12 = assign34170_e49292_d_n12;
        locals.var_qde_dn13 = assign34170_e49292_d_n13;
        locals.var_qde_dn15 = assign34170_e49292_d_n15;
        locals.var_qde_dn16 = assign34170_e49292_d_n16;
        locals.var_qde_dn17 = assign34170_e49292_d_n17;
        locals.var_qde_dn18 = assign34170_e49292_d_n18;

        let (assign34180_e49296, assign34180_e49296_d_n0, assign34180_e49296_d_n2, assign34180_e49296_d_n6, assign34180_e49296_d_n7, assign34180_e49296_d_n10, assign34180_e49296_d_n11, assign34180_e49296_d_n12, assign34180_e49296_d_n13, assign34180_e49296_d_n15, assign34180_e49296_d_n16, assign34180_e49296_d_n17, assign34180_e49296_d_n18,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34180_e49296;
        locals.var_qge_dn0 = assign34180_e49296_d_n0;
        locals.var_qge_dn2 = assign34180_e49296_d_n2;
        locals.var_qge_dn6 = assign34180_e49296_d_n6;
        locals.var_qge_dn7 = assign34180_e49296_d_n7;
        locals.var_qge_dn10 = assign34180_e49296_d_n10;
        locals.var_qge_dn11 = assign34180_e49296_d_n11;
        locals.var_qge_dn12 = assign34180_e49296_d_n12;
        locals.var_qge_dn13 = assign34180_e49296_d_n13;
        locals.var_qge_dn15 = assign34180_e49296_d_n15;
        locals.var_qge_dn16 = assign34180_e49296_d_n16;
        locals.var_qge_dn17 = assign34180_e49296_d_n17;
        locals.var_qge_dn18 = assign34180_e49296_d_n18;

        let assign34190_e49299: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1127 = assign34190_e49299;

        let (assign34200_e49305, assign34200_e49305_d_n0, assign34200_e49305_d_n2, assign34200_e49305_d_n6, assign34200_e49305_d_n7, assign34200_e49305_d_n10, assign34200_e49305_d_n11, assign34200_e49305_d_n12, assign34200_e49305_d_n13, assign34200_e49305_d_n15, assign34200_e49305_d_n16, assign34200_e49305_d_n17, assign34200_e49305_d_n18,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1127 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34200_e49305;
        locals.var_qse_dn0 = assign34200_e49305_d_n0;
        locals.var_qse_dn2 = assign34200_e49305_d_n2;
        locals.var_qse_dn6 = assign34200_e49305_d_n6;
        locals.var_qse_dn7 = assign34200_e49305_d_n7;
        locals.var_qse_dn10 = assign34200_e49305_d_n10;
        locals.var_qse_dn11 = assign34200_e49305_d_n11;
        locals.var_qse_dn12 = assign34200_e49305_d_n12;
        locals.var_qse_dn13 = assign34200_e49305_d_n13;
        locals.var_qse_dn15 = assign34200_e49305_d_n15;
        locals.var_qse_dn16 = assign34200_e49305_d_n16;
        locals.var_qse_dn17 = assign34200_e49305_d_n17;
        locals.var_qse_dn18 = assign34200_e49305_d_n18;

        let (assign34210_e49311, assign34210_e49311_d_n0, assign34210_e49311_d_n2, assign34210_e49311_d_n6, assign34210_e49311_d_n7, assign34210_e49311_d_n10, assign34210_e49311_d_n11, assign34210_e49311_d_n12, assign34210_e49311_d_n17,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1127 != 0.0)) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
    }
};
        locals.var_xd = assign34210_e49311;
        locals.var_xd_dn0 = assign34210_e49311_d_n0;
        locals.var_xd_dn2 = assign34210_e49311_d_n2;
        locals.var_xd_dn6 = assign34210_e49311_d_n6;
        locals.var_xd_dn7 = assign34210_e49311_d_n7;
        locals.var_xd_dn10 = assign34210_e49311_d_n10;
        locals.var_xd_dn11 = assign34210_e49311_d_n11;
        locals.var_xd_dn12 = assign34210_e49311_d_n12;
        locals.var_xd_dn17 = assign34210_e49311_d_n17;

        let (assign34220_e49319, assign34220_e49319_d_n0, assign34220_e49319_d_n2, assign34220_e49319_d_n6, assign34220_e49319_d_n7, assign34220_e49319_d_n10, assign34220_e49319_d_n11, assign34220_e49319_d_n12, assign34220_e49319_d_n13, assign34220_e49319_d_n15, assign34220_e49319_d_n16, assign34220_e49319_d_n17, assign34220_e49319_d_n18,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1127 != 0.0)) {
        let assign34220_e49317: f64 = (locals.var_mfactor * locals.var_qb);
        (assign34220_e49317, (locals.var_mfactor * locals.var_qb_dn0), (locals.var_mfactor * locals.var_qb_dn2), (locals.var_mfactor * locals.var_qb_dn6), (locals.var_mfactor * locals.var_qb_dn7), (locals.var_mfactor * locals.var_qb_dn10), (locals.var_mfactor * locals.var_qb_dn11), (locals.var_mfactor * locals.var_qb_dn12), (locals.var_mfactor * locals.var_qb_dn13), (locals.var_mfactor * locals.var_qb_dn15), (locals.var_mfactor * locals.var_qb_dn16), (locals.var_mfactor * locals.var_qb_dn17), (locals.var_mfactor * locals.var_qb_dn18),)
    } else {
        (locals.var_qb_qs, locals.var_qb_qs_dn0, locals.var_qb_qs_dn2, locals.var_qb_qs_dn6, locals.var_qb_qs_dn7, locals.var_qb_qs_dn10, locals.var_qb_qs_dn11, locals.var_qb_qs_dn12, locals.var_qb_qs_dn13, locals.var_qb_qs_dn15, locals.var_qb_qs_dn16, locals.var_qb_qs_dn17, locals.var_qb_qs_dn18,)
    }
};
        locals.var_qb_qs = assign34220_e49319;
        locals.var_qb_qs_dn0 = assign34220_e49319_d_n0;
        locals.var_qb_qs_dn2 = assign34220_e49319_d_n2;
        locals.var_qb_qs_dn6 = assign34220_e49319_d_n6;
        locals.var_qb_qs_dn7 = assign34220_e49319_d_n7;
        locals.var_qb_qs_dn10 = assign34220_e49319_d_n10;
        locals.var_qb_qs_dn11 = assign34220_e49319_d_n11;
        locals.var_qb_qs_dn12 = assign34220_e49319_d_n12;
        locals.var_qb_qs_dn13 = assign34220_e49319_d_n13;
        locals.var_qb_qs_dn15 = assign34220_e49319_d_n15;
        locals.var_qb_qs_dn16 = assign34220_e49319_d_n16;
        locals.var_qb_qs_dn17 = assign34220_e49319_d_n17;
        locals.var_qb_qs_dn18 = assign34220_e49319_d_n18;

        let (assign34240_e49334, assign34240_e49334_d_n0, assign34240_e49334_d_n2, assign34240_e49334_d_n6, assign34240_e49334_d_n7, assign34240_e49334_d_n10, assign34240_e49334_d_n11, assign34240_e49334_d_n12, assign34240_e49334_d_n13, assign34240_e49334_d_n15, assign34240_e49334_d_n16, assign34240_e49334_d_n17, assign34240_e49334_d_n18,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1127 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign34240_e49334;
        locals.var_qbe_dn0 = assign34240_e49334_d_n0;
        locals.var_qbe_dn2 = assign34240_e49334_d_n2;
        locals.var_qbe_dn6 = assign34240_e49334_d_n6;
        locals.var_qbe_dn7 = assign34240_e49334_d_n7;
        locals.var_qbe_dn10 = assign34240_e49334_d_n10;
        locals.var_qbe_dn11 = assign34240_e49334_d_n11;
        locals.var_qbe_dn12 = assign34240_e49334_d_n12;
        locals.var_qbe_dn13 = assign34240_e49334_d_n13;
        locals.var_qbe_dn15 = assign34240_e49334_d_n15;
        locals.var_qbe_dn16 = assign34240_e49334_d_n16;
        locals.var_qbe_dn17 = assign34240_e49334_d_n17;
        locals.var_qbe_dn18 = assign34240_e49334_d_n18;

        let (assign34250_e49343, assign34250_e49343_d_n0, assign34250_e49343_d_n2, assign34250_e49343_d_n6, assign34250_e49343_d_n7, assign34250_e49343_d_n10, assign34250_e49343_d_n11, assign34250_e49343_d_n12, assign34250_e49343_d_n13, assign34250_e49343_d_n15, assign34250_e49343_d_n16, assign34250_e49343_d_n17, assign34250_e49343_d_n18,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1127 == 0.0)) {
        let assign34250_e49341: f64 = (locals.var_mfactor * locals.var_qsub);
        (assign34250_e49341, (locals.var_mfactor * locals.var_qsub_dn0), (locals.var_mfactor * locals.var_qsub_dn2), (locals.var_mfactor * locals.var_qsub_dn6), (locals.var_mfactor * locals.var_qsub_dn7), (locals.var_mfactor * locals.var_qsub_dn10), (locals.var_mfactor * locals.var_qsub_dn11), (locals.var_mfactor * locals.var_qsub_dn12), 0.0, 0.0, 0.0, (locals.var_mfactor * locals.var_qsub_dn17), 0.0,)
    } else {
        (locals.var_qb_qs, locals.var_qb_qs_dn0, locals.var_qb_qs_dn2, locals.var_qb_qs_dn6, locals.var_qb_qs_dn7, locals.var_qb_qs_dn10, locals.var_qb_qs_dn11, locals.var_qb_qs_dn12, locals.var_qb_qs_dn13, locals.var_qb_qs_dn15, locals.var_qb_qs_dn16, locals.var_qb_qs_dn17, locals.var_qb_qs_dn18,)
    }
};
        locals.var_qb_qs = assign34250_e49343;
        locals.var_qb_qs_dn0 = assign34250_e49343_d_n0;
        locals.var_qb_qs_dn2 = assign34250_e49343_d_n2;
        locals.var_qb_qs_dn6 = assign34250_e49343_d_n6;
        locals.var_qb_qs_dn7 = assign34250_e49343_d_n7;
        locals.var_qb_qs_dn10 = assign34250_e49343_d_n10;
        locals.var_qb_qs_dn11 = assign34250_e49343_d_n11;
        locals.var_qb_qs_dn12 = assign34250_e49343_d_n12;
        locals.var_qb_qs_dn13 = assign34250_e49343_d_n13;
        locals.var_qb_qs_dn15 = assign34250_e49343_d_n15;
        locals.var_qb_qs_dn16 = assign34250_e49343_d_n16;
        locals.var_qb_qs_dn17 = assign34250_e49343_d_n17;
        locals.var_qb_qs_dn18 = assign34250_e49343_d_n18;

        let assign34280_e49370: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1128 = assign34280_e49370;

        let (assign34290_e49382, assign34290_e49382_d_n0, assign34290_e49382_d_n2, assign34290_e49382_d_n6, assign34290_e49382_d_n7, assign34290_e49382_d_n10, assign34290_e49382_d_n11, assign34290_e49382_d_n12, assign34290_e49382_d_n13, assign34290_e49382_d_n15, assign34290_e49382_d_n16, assign34290_e49382_d_n17, assign34290_e49382_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 != 0.0)) {
        let assign34290_e49377: f64 = (-locals.var_qb);
        let assign34290_e49379: f64 = (assign34290_e49377 - locals.var_qi);
        let assign34290_e49380: f64 = (locals.var_mfactor * assign34290_e49379);
        (assign34290_e49380, (locals.var_mfactor * ((-locals.var_qb_dn0) - locals.var_qi_dn0)), (locals.var_mfactor * ((-locals.var_qb_dn2) - locals.var_qi_dn2)), (locals.var_mfactor * ((-locals.var_qb_dn6) - locals.var_qi_dn6)), (locals.var_mfactor * ((-locals.var_qb_dn7) - locals.var_qi_dn7)), (locals.var_mfactor * ((-locals.var_qb_dn10) - locals.var_qi_dn10)), (locals.var_mfactor * ((-locals.var_qb_dn11) - locals.var_qi_dn11)), (locals.var_mfactor * ((-locals.var_qb_dn12) - locals.var_qi_dn12)), (locals.var_mfactor * (-locals.var_qb_dn13)), (locals.var_mfactor * (-locals.var_qb_dn15)), (locals.var_mfactor * (-locals.var_qb_dn16)), (locals.var_mfactor * ((-locals.var_qb_dn17) - locals.var_qi_dn17)), (locals.var_mfactor * (-locals.var_qb_dn18)),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34290_e49382;
        locals.var_qge_dn0 = assign34290_e49382_d_n0;
        locals.var_qge_dn2 = assign34290_e49382_d_n2;
        locals.var_qge_dn6 = assign34290_e49382_d_n6;
        locals.var_qge_dn7 = assign34290_e49382_d_n7;
        locals.var_qge_dn10 = assign34290_e49382_d_n10;
        locals.var_qge_dn11 = assign34290_e49382_d_n11;
        locals.var_qge_dn12 = assign34290_e49382_d_n12;
        locals.var_qge_dn13 = assign34290_e49382_d_n13;
        locals.var_qge_dn15 = assign34290_e49382_d_n15;
        locals.var_qge_dn16 = assign34290_e49382_d_n16;
        locals.var_qge_dn17 = assign34290_e49382_d_n17;
        locals.var_qge_dn18 = assign34290_e49382_d_n18;

        let (assign34300_e49391, assign34300_e49391_d_n0, assign34300_e49391_d_n2, assign34300_e49391_d_n6, assign34300_e49391_d_n7, assign34300_e49391_d_n10, assign34300_e49391_d_n11, assign34300_e49391_d_n12, assign34300_e49391_d_n13, assign34300_e49391_d_n15, assign34300_e49391_d_n16, assign34300_e49391_d_n17, assign34300_e49391_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 != 0.0)) {
        let assign34300_e49389: f64 = (locals.var_mfactor * locals.var_qd);
        (assign34300_e49389, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn12), (locals.var_mfactor * locals.var_qd_dn13), (locals.var_mfactor * locals.var_qd_dn15), (locals.var_mfactor * locals.var_qd_dn16), (locals.var_mfactor * locals.var_qd_dn17), (locals.var_mfactor * locals.var_qd_dn18),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34300_e49391;
        locals.var_qde_dn0 = assign34300_e49391_d_n0;
        locals.var_qde_dn2 = assign34300_e49391_d_n2;
        locals.var_qde_dn6 = assign34300_e49391_d_n6;
        locals.var_qde_dn7 = assign34300_e49391_d_n7;
        locals.var_qde_dn10 = assign34300_e49391_d_n10;
        locals.var_qde_dn11 = assign34300_e49391_d_n11;
        locals.var_qde_dn12 = assign34300_e49391_d_n12;
        locals.var_qde_dn13 = assign34300_e49391_d_n13;
        locals.var_qde_dn15 = assign34300_e49391_d_n15;
        locals.var_qde_dn16 = assign34300_e49391_d_n16;
        locals.var_qde_dn17 = assign34300_e49391_d_n17;
        locals.var_qde_dn18 = assign34300_e49391_d_n18;

        let (assign34310_e49402, assign34310_e49402_d_n0, assign34310_e49402_d_n2, assign34310_e49402_d_n6, assign34310_e49402_d_n7, assign34310_e49402_d_n10, assign34310_e49402_d_n11, assign34310_e49402_d_n12, assign34310_e49402_d_n13, assign34310_e49402_d_n15, assign34310_e49402_d_n16, assign34310_e49402_d_n17, assign34310_e49402_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 != 0.0)) {
        let assign34310_e49399: f64 = (locals.var_qi - locals.var_qd);
        let assign34310_e49400: f64 = (locals.var_mfactor * assign34310_e49399);
        (assign34310_e49400, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn12 - locals.var_qd_dn12)), (locals.var_mfactor * (-locals.var_qd_dn13)), (locals.var_mfactor * (-locals.var_qd_dn15)), (locals.var_mfactor * (-locals.var_qd_dn16)), (locals.var_mfactor * (locals.var_qi_dn17 - locals.var_qd_dn17)), (locals.var_mfactor * (-locals.var_qd_dn18)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34310_e49402;
        locals.var_qse_dn0 = assign34310_e49402_d_n0;
        locals.var_qse_dn2 = assign34310_e49402_d_n2;
        locals.var_qse_dn6 = assign34310_e49402_d_n6;
        locals.var_qse_dn7 = assign34310_e49402_d_n7;
        locals.var_qse_dn10 = assign34310_e49402_d_n10;
        locals.var_qse_dn11 = assign34310_e49402_d_n11;
        locals.var_qse_dn12 = assign34310_e49402_d_n12;
        locals.var_qse_dn13 = assign34310_e49402_d_n13;
        locals.var_qse_dn15 = assign34310_e49402_d_n15;
        locals.var_qse_dn16 = assign34310_e49402_d_n16;
        locals.var_qse_dn17 = assign34310_e49402_d_n17;
        locals.var_qse_dn18 = assign34310_e49402_d_n18;

    }

    pub(super) fn stamp_transient_block_120(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34320_e49419, assign34320_e49419_d_n0, assign34320_e49419_d_n2, assign34320_e49419_d_n6, assign34320_e49419_d_n7, assign34320_e49419_d_n10, assign34320_e49419_d_n11, assign34320_e49419_d_n12, assign34320_e49419_d_n13, assign34320_e49419_d_n15, assign34320_e49419_d_n16, assign34320_e49419_d_n17, assign34320_e49419_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 == 0.0)) {
        let assign34320_e49410: f64 = (-locals.var_qsub);
        let assign34320_e49412: f64 = (assign34320_e49410 - locals.var_qi);
        let assign34320_e49414: f64 = (assign34320_e49412 - locals.var_qs_fb);
        let assign34320_e49416: f64 = (assign34320_e49414 - locals.var_qd_fb);
        let assign34320_e49417: f64 = (locals.var_mfactor * assign34320_e49416);
        (assign34320_e49417, (locals.var_mfactor * ((((-locals.var_qsub_dn0) - locals.var_qi_dn0) - locals.var_qs_fb_dn0) - locals.var_qd_fb_dn0)), (locals.var_mfactor * ((((-locals.var_qsub_dn2) - locals.var_qi_dn2) - locals.var_qs_fb_dn2) - locals.var_qd_fb_dn2)), (locals.var_mfactor * ((((-locals.var_qsub_dn6) - locals.var_qi_dn6) - locals.var_qs_fb_dn6) - locals.var_qd_fb_dn6)), (locals.var_mfactor * ((((-locals.var_qsub_dn7) - locals.var_qi_dn7) - locals.var_qs_fb_dn7) - locals.var_qd_fb_dn7)), (locals.var_mfactor * ((((-locals.var_qsub_dn10) - locals.var_qi_dn10) - locals.var_qs_fb_dn10) - locals.var_qd_fb_dn10)), (locals.var_mfactor * ((((-locals.var_qsub_dn11) - locals.var_qi_dn11) - locals.var_qs_fb_dn11) - locals.var_qd_fb_dn11)), (locals.var_mfactor * ((((-locals.var_qsub_dn12) - locals.var_qi_dn12) - locals.var_qs_fb_dn12) - locals.var_qd_fb_dn12)), (locals.var_mfactor * ((-locals.var_qs_fb_dn13) - locals.var_qd_fb_dn13)), (locals.var_mfactor * ((-locals.var_qs_fb_dn15) - locals.var_qd_fb_dn15)), (locals.var_mfactor * ((-locals.var_qs_fb_dn16) - locals.var_qd_fb_dn16)), (locals.var_mfactor * ((((-locals.var_qsub_dn17) - locals.var_qi_dn17) - locals.var_qs_fb_dn17) - locals.var_qd_fb_dn17)), (locals.var_mfactor * ((-locals.var_qs_fb_dn18) - locals.var_qd_fb_dn18)),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34320_e49419;
        locals.var_qge_dn0 = assign34320_e49419_d_n0;
        locals.var_qge_dn2 = assign34320_e49419_d_n2;
        locals.var_qge_dn6 = assign34320_e49419_d_n6;
        locals.var_qge_dn7 = assign34320_e49419_d_n7;
        locals.var_qge_dn10 = assign34320_e49419_d_n10;
        locals.var_qge_dn11 = assign34320_e49419_d_n11;
        locals.var_qge_dn12 = assign34320_e49419_d_n12;
        locals.var_qge_dn13 = assign34320_e49419_d_n13;
        locals.var_qge_dn15 = assign34320_e49419_d_n15;
        locals.var_qge_dn16 = assign34320_e49419_d_n16;
        locals.var_qge_dn17 = assign34320_e49419_d_n17;
        locals.var_qge_dn18 = assign34320_e49419_d_n18;

        let (assign34330_e49431, assign34330_e49431_d_n0, assign34330_e49431_d_n2, assign34330_e49431_d_n6, assign34330_e49431_d_n7, assign34330_e49431_d_n10, assign34330_e49431_d_n11, assign34330_e49431_d_n12, assign34330_e49431_d_n13, assign34330_e49431_d_n15, assign34330_e49431_d_n16, assign34330_e49431_d_n17, assign34330_e49431_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 == 0.0)) {
        let assign34330_e49428: f64 = (locals.var_qd + locals.var_qd_fb);
        let assign34330_e49429: f64 = (locals.var_mfactor * assign34330_e49428);
        (assign34330_e49429, (locals.var_mfactor * (locals.var_qd_dn0 + locals.var_qd_fb_dn0)), (locals.var_mfactor * (locals.var_qd_dn2 + locals.var_qd_fb_dn2)), (locals.var_mfactor * (locals.var_qd_dn6 + locals.var_qd_fb_dn6)), (locals.var_mfactor * (locals.var_qd_dn7 + locals.var_qd_fb_dn7)), (locals.var_mfactor * (locals.var_qd_dn10 + locals.var_qd_fb_dn10)), (locals.var_mfactor * (locals.var_qd_dn11 + locals.var_qd_fb_dn11)), (locals.var_mfactor * (locals.var_qd_dn12 + locals.var_qd_fb_dn12)), (locals.var_mfactor * (locals.var_qd_dn13 + locals.var_qd_fb_dn13)), (locals.var_mfactor * (locals.var_qd_dn15 + locals.var_qd_fb_dn15)), (locals.var_mfactor * (locals.var_qd_dn16 + locals.var_qd_fb_dn16)), (locals.var_mfactor * (locals.var_qd_dn17 + locals.var_qd_fb_dn17)), (locals.var_mfactor * (locals.var_qd_dn18 + locals.var_qd_fb_dn18)),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34330_e49431;
        locals.var_qde_dn0 = assign34330_e49431_d_n0;
        locals.var_qde_dn2 = assign34330_e49431_d_n2;
        locals.var_qde_dn6 = assign34330_e49431_d_n6;
        locals.var_qde_dn7 = assign34330_e49431_d_n7;
        locals.var_qde_dn10 = assign34330_e49431_d_n10;
        locals.var_qde_dn11 = assign34330_e49431_d_n11;
        locals.var_qde_dn12 = assign34330_e49431_d_n12;
        locals.var_qde_dn13 = assign34330_e49431_d_n13;
        locals.var_qde_dn15 = assign34330_e49431_d_n15;
        locals.var_qde_dn16 = assign34330_e49431_d_n16;
        locals.var_qde_dn17 = assign34330_e49431_d_n17;
        locals.var_qde_dn18 = assign34330_e49431_d_n18;

        let (assign34340_e49445, assign34340_e49445_d_n0, assign34340_e49445_d_n2, assign34340_e49445_d_n6, assign34340_e49445_d_n7, assign34340_e49445_d_n10, assign34340_e49445_d_n11, assign34340_e49445_d_n12, assign34340_e49445_d_n13, assign34340_e49445_d_n15, assign34340_e49445_d_n16, assign34340_e49445_d_n17, assign34340_e49445_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 == 0.0)) {
        let assign34340_e49440: f64 = (locals.var_qi - locals.var_qd);
        let assign34340_e49442: f64 = (assign34340_e49440 + locals.var_qs_fb);
        let assign34340_e49443: f64 = (locals.var_mfactor * assign34340_e49442);
        (assign34340_e49443, (locals.var_mfactor * ((locals.var_qi_dn0 - locals.var_qd_dn0) + locals.var_qs_fb_dn0)), (locals.var_mfactor * ((locals.var_qi_dn2 - locals.var_qd_dn2) + locals.var_qs_fb_dn2)), (locals.var_mfactor * ((locals.var_qi_dn6 - locals.var_qd_dn6) + locals.var_qs_fb_dn6)), (locals.var_mfactor * ((locals.var_qi_dn7 - locals.var_qd_dn7) + locals.var_qs_fb_dn7)), (locals.var_mfactor * ((locals.var_qi_dn10 - locals.var_qd_dn10) + locals.var_qs_fb_dn10)), (locals.var_mfactor * ((locals.var_qi_dn11 - locals.var_qd_dn11) + locals.var_qs_fb_dn11)), (locals.var_mfactor * ((locals.var_qi_dn12 - locals.var_qd_dn12) + locals.var_qs_fb_dn12)), (locals.var_mfactor * ((-locals.var_qd_dn13) + locals.var_qs_fb_dn13)), (locals.var_mfactor * ((-locals.var_qd_dn15) + locals.var_qs_fb_dn15)), (locals.var_mfactor * ((-locals.var_qd_dn16) + locals.var_qs_fb_dn16)), (locals.var_mfactor * ((locals.var_qi_dn17 - locals.var_qd_dn17) + locals.var_qs_fb_dn17)), (locals.var_mfactor * ((-locals.var_qd_dn18) + locals.var_qs_fb_dn18)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34340_e49445;
        locals.var_qse_dn0 = assign34340_e49445_d_n0;
        locals.var_qse_dn2 = assign34340_e49445_d_n2;
        locals.var_qse_dn6 = assign34340_e49445_d_n6;
        locals.var_qse_dn7 = assign34340_e49445_d_n7;
        locals.var_qse_dn10 = assign34340_e49445_d_n10;
        locals.var_qse_dn11 = assign34340_e49445_d_n11;
        locals.var_qse_dn12 = assign34340_e49445_d_n12;
        locals.var_qse_dn13 = assign34340_e49445_d_n13;
        locals.var_qse_dn15 = assign34340_e49445_d_n15;
        locals.var_qse_dn16 = assign34340_e49445_d_n16;
        locals.var_qse_dn17 = assign34340_e49445_d_n17;
        locals.var_qse_dn18 = assign34340_e49445_d_n18;

        let assign34350_e49448: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1134 = assign34350_e49448;

        let (assign34360_e49452, assign34360_e49452_d_n0, assign34360_e49452_d_n2, assign34360_e49452_d_n6, assign34360_e49452_d_n7, assign34360_e49452_d_n10, assign34360_e49452_d_n11, assign34360_e49452_d_n12, assign34360_e49452_d_n17,) = {
    if (locals.var_guard1134 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34360_e49452;
        locals.var_qy_dn0 = assign34360_e49452_d_n0;
        locals.var_qy_dn2 = assign34360_e49452_d_n2;
        locals.var_qy_dn6 = assign34360_e49452_d_n6;
        locals.var_qy_dn7 = assign34360_e49452_d_n7;
        locals.var_qy_dn10 = assign34360_e49452_d_n10;
        locals.var_qy_dn11 = assign34360_e49452_d_n11;
        locals.var_qy_dn12 = assign34360_e49452_d_n12;
        locals.var_qy_dn17 = assign34360_e49452_d_n17;

        let (assign34370_e49461, assign34370_e49461_d_n0, assign34370_e49461_d_n2, assign34370_e49461_d_n6, assign34370_e49461_d_n7, assign34370_e49461_d_n10, assign34370_e49461_d_n11, assign34370_e49461_d_n12, assign34370_e49461_d_n17,) = {
    if (locals.var_guard1134 == 0.0) {
        let assign34370_e49457: f64 = (locals.var_ec * locals.var_leff);
        let assign34370_e49459: f64 = (assign34370_e49457 + locals.var_ps0);
        (assign34370_e49459, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn12 * locals.var_leff) + locals.var_ps0_dn12), ((locals.var_ec_dn17 * locals.var_leff) + locals.var_ps0_dn17),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17,)
    }
};
        locals.var_pslk = assign34370_e49461;
        locals.var_pslk_dn0 = assign34370_e49461_d_n0;
        locals.var_pslk_dn2 = assign34370_e49461_d_n2;
        locals.var_pslk_dn6 = assign34370_e49461_d_n6;
        locals.var_pslk_dn7 = assign34370_e49461_d_n7;
        locals.var_pslk_dn10 = assign34370_e49461_d_n10;
        locals.var_pslk_dn11 = assign34370_e49461_d_n11;
        locals.var_pslk_dn12 = assign34370_e49461_d_n12;
        locals.var_pslk_dn17 = assign34370_e49461_d_n17;

        let assign34380_e49464: f64 = if locals.var_pslk > locals.var_psdl { 1.0 } else { 0.0 };
        locals.var_guard1135 = assign34380_e49464;

        let (assign34390_e49471, assign34390_e49471_d_n0, assign34390_e49471_d_n2, assign34390_e49471_d_n6, assign34390_e49471_d_n7, assign34390_e49471_d_n10, assign34390_e49471_d_n11, assign34390_e49471_d_n12, assign34390_e49471_d_n17,) = {
    if ((locals.var_guard1134 == 0.0) && (locals.var_guard1135 != 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17,)
    }
};
        locals.var_pslk = assign34390_e49471;
        locals.var_pslk_dn0 = assign34390_e49471_d_n0;
        locals.var_pslk_dn2 = assign34390_e49471_d_n2;
        locals.var_pslk_dn6 = assign34390_e49471_d_n6;
        locals.var_pslk_dn7 = assign34390_e49471_d_n7;
        locals.var_pslk_dn10 = assign34390_e49471_d_n10;
        locals.var_pslk_dn11 = assign34390_e49471_d_n11;
        locals.var_pslk_dn12 = assign34390_e49471_d_n12;
        locals.var_pslk_dn17 = assign34390_e49471_d_n17;

        let (assign34400_e49486, assign34400_e49486_d_n0, assign34400_e49486_d_n2, assign34400_e49486_d_n6, assign34400_e49486_d_n7, assign34400_e49486_d_n10, assign34400_e49486_d_n11, assign34400_e49486_d_n12, assign34400_e49486_d_n17,) = {
    if (locals.var_guard1134 == 0.0) {
        let assign34400_e49477: f64 = (locals.var_vds + locals.var_ps0);
        let assign34400_e49478: f64 = (locals.var_aclm * assign34400_e49477);
        let assign34400_e49481: f64 = (1.0 - locals.var_aclm);
        let assign34400_e49483: f64 = (assign34400_e49481 * locals.var_pslk);
        let assign34400_e49484: f64 = (assign34400_e49478 + assign34400_e49483);
        (assign34400_e49484, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign34400_e49481 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign34400_e49481 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign34400_e49481 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign34400_e49481 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign34400_e49481 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign34400_e49481 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + (assign34400_e49481 * locals.var_pslk_dn12)), ((locals.var_aclm * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + (assign34400_e49481 * locals.var_pslk_dn17)),)
    } else {
        (locals.var_t1__blk1130, locals.var_t1__blk1130_dn0, locals.var_t1__blk1130_dn2, locals.var_t1__blk1130_dn6, locals.var_t1__blk1130_dn7, locals.var_t1__blk1130_dn10, locals.var_t1__blk1130_dn11, locals.var_t1__blk1130_dn12, locals.var_t1__blk1130_dn17,)
    }
};
        locals.var_t1__blk1130 = assign34400_e49486;
        locals.var_t1__blk1130_dn0 = assign34400_e49486_d_n0;
        locals.var_t1__blk1130_dn2 = assign34400_e49486_d_n2;
        locals.var_t1__blk1130_dn6 = assign34400_e49486_d_n6;
        locals.var_t1__blk1130_dn7 = assign34400_e49486_d_n7;
        locals.var_t1__blk1130_dn10 = assign34400_e49486_d_n10;
        locals.var_t1__blk1130_dn11 = assign34400_e49486_d_n11;
        locals.var_t1__blk1130_dn12 = assign34400_e49486_d_n12;
        locals.var_t1__blk1130_dn17 = assign34400_e49486_d_n17;

        let (assign34410_e49496, assign34410_e49496_d_n0, assign34410_e49496_d_n2, assign34410_e49496_d_n6, assign34410_e49496_d_n7, assign34410_e49496_d_n10, assign34410_e49496_d_n11, assign34410_e49496_d_n12, assign34410_e49496_d_n17,) = {
    if (locals.var_guard1134 == 0.0) {
        let assign34410_e49491: f64 = (2.0 * 1.034943e-10);
        let assign34410_e49493: f64 = (assign34410_e49491 / locals.var_q_nsub);
        let assign34410_e49494: f64 = (assign34410_e49493).sqrt();
        (assign34410_e49494, ((-((assign34410_e49491 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)),)
    } else {
        (locals.var_t10__blk1131, locals.var_t10__blk1131_dn0, locals.var_t10__blk1131_dn2, locals.var_t10__blk1131_dn6, locals.var_t10__blk1131_dn7, locals.var_t10__blk1131_dn10, locals.var_t10__blk1131_dn11, locals.var_t10__blk1131_dn12, locals.var_t10__blk1131_dn17,)
    }
};
        locals.var_t10__blk1131 = assign34410_e49496;
        locals.var_t10__blk1131_dn0 = assign34410_e49496_d_n0;
        locals.var_t10__blk1131_dn2 = assign34410_e49496_d_n2;
        locals.var_t10__blk1131_dn6 = assign34410_e49496_d_n6;
        locals.var_t10__blk1131_dn7 = assign34410_e49496_d_n7;
        locals.var_t10__blk1131_dn10 = assign34410_e49496_d_n10;
        locals.var_t10__blk1131_dn11 = assign34410_e49496_d_n11;
        locals.var_t10__blk1131_dn12 = assign34410_e49496_d_n12;
        locals.var_t10__blk1131_dn17 = assign34410_e49496_d_n17;

        let (assign34420_e49503, assign34420_e49503_d_n0, assign34420_e49503_d_n2, assign34420_e49503_d_n6, assign34420_e49503_d_n7, assign34420_e49503_d_n10, assign34420_e49503_d_n11, assign34420_e49503_d_n12, assign34420_e49503_d_n17,) = {
    if (locals.var_guard1134 == 0.0) {
        let assign34420_e49501: f64 = (locals.var_t10__blk1131 * 1.3);
        (assign34420_e49501, (locals.var_t10__blk1131_dn0 * 1.3), (locals.var_t10__blk1131_dn2 * 1.3), (locals.var_t10__blk1131_dn6 * 1.3), (locals.var_t10__blk1131_dn7 * 1.3), (locals.var_t10__blk1131_dn10 * 1.3), (locals.var_t10__blk1131_dn11 * 1.3), (locals.var_t10__blk1131_dn12 * 1.3), (locals.var_t10__blk1131_dn17 * 1.3),)
    } else {
        (locals.var_t3__blk1132, locals.var_t3__blk1132_dn0, locals.var_t3__blk1132_dn2, locals.var_t3__blk1132_dn6, locals.var_t3__blk1132_dn7, locals.var_t3__blk1132_dn10, locals.var_t3__blk1132_dn11, locals.var_t3__blk1132_dn12, locals.var_t3__blk1132_dn17,)
    }
};
        locals.var_t3__blk1132 = assign34420_e49503;
        locals.var_t3__blk1132_dn0 = assign34420_e49503_d_n0;
        locals.var_t3__blk1132_dn2 = assign34420_e49503_d_n2;
        locals.var_t3__blk1132_dn6 = assign34420_e49503_d_n6;
        locals.var_t3__blk1132_dn7 = assign34420_e49503_d_n7;
        locals.var_t3__blk1132_dn10 = assign34420_e49503_d_n10;
        locals.var_t3__blk1132_dn11 = assign34420_e49503_d_n11;
        locals.var_t3__blk1132_dn12 = assign34420_e49503_d_n12;
        locals.var_t3__blk1132_dn17 = assign34420_e49503_d_n17;

        let (assign34430_e49512, assign34430_e49512_d_n0, assign34430_e49512_d_n2, assign34430_e49512_d_n6, assign34430_e49512_d_n7, assign34430_e49512_d_n10, assign34430_e49512_d_n11, assign34430_e49512_d_n12, assign34430_e49512_d_n17,) = {
    if (locals.var_guard1134 == 0.0) {
        let assign34430_e49508: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign34430_e49510: f64 = (assign34430_e49508 * locals.var_t3__blk1132);
        (assign34430_e49510, (assign34430_e49508 * locals.var_t3__blk1132_dn0), (assign34430_e49508 * locals.var_t3__blk1132_dn2), (assign34430_e49508 * locals.var_t3__blk1132_dn6), (assign34430_e49508 * locals.var_t3__blk1132_dn7), (assign34430_e49508 * locals.var_t3__blk1132_dn10), (assign34430_e49508 * locals.var_t3__blk1132_dn11), (assign34430_e49508 * locals.var_t3__blk1132_dn12), (assign34430_e49508 * locals.var_t3__blk1132_dn17),)
    } else {
        (locals.var_t2__blk1133, locals.var_t2__blk1133_dn0, locals.var_t2__blk1133_dn2, locals.var_t2__blk1133_dn6, locals.var_t2__blk1133_dn7, locals.var_t2__blk1133_dn10, locals.var_t2__blk1133_dn11, locals.var_t2__blk1133_dn12, locals.var_t2__blk1133_dn17,)
    }
};
        locals.var_t2__blk1133 = assign34430_e49512;
        locals.var_t2__blk1133_dn0 = assign34430_e49512_d_n0;
        locals.var_t2__blk1133_dn2 = assign34430_e49512_d_n2;
        locals.var_t2__blk1133_dn6 = assign34430_e49512_d_n6;
        locals.var_t2__blk1133_dn7 = assign34430_e49512_d_n7;
        locals.var_t2__blk1133_dn10 = assign34430_e49512_d_n10;
        locals.var_t2__blk1133_dn11 = assign34430_e49512_d_n11;
        locals.var_t2__blk1133_dn12 = assign34430_e49512_d_n12;
        locals.var_t2__blk1133_dn17 = assign34430_e49512_d_n17;

        let (assign34440_e49527, assign34440_e49527_d_n0, assign34440_e49527_d_n2, assign34440_e49527_d_n6, assign34440_e49527_d_n7, assign34440_e49527_d_n10, assign34440_e49527_d_n11, assign34440_e49527_d_n12, assign34440_e49527_d_n17,) = {
    if (locals.var_guard1134 == 0.0) {
        let assign34440_e49517: f64 = (locals.var_ps0 + locals.var_vds);
        let assign34440_e49519: f64 = (assign34440_e49517 - locals.var_t1__blk1130);
        let assign34440_e49521: f64 = (assign34440_e49519 / p.p64);
        let assign34440_e49523: f64 = (assign34440_e49521 - locals.var_ec);
        let assign34440_e49525: f64 = (assign34440_e49523 * locals.var_t2__blk1133);
        (assign34440_e49525, ((((((locals.var_ps0_dn0 + locals.var_vds_dn0) - locals.var_t1__blk1130_dn0) / p.p64) - locals.var_ec_dn0) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn0)), ((((((locals.var_ps0_dn2 + locals.var_vds_dn2) - locals.var_t1__blk1130_dn2) / p.p64) - locals.var_ec_dn2) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn2)), ((((((locals.var_ps0_dn6 + locals.var_vds_dn6) - locals.var_t1__blk1130_dn6) / p.p64) - locals.var_ec_dn6) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn6)), ((((((locals.var_ps0_dn7 + locals.var_vds_dn7) - locals.var_t1__blk1130_dn7) / p.p64) - locals.var_ec_dn7) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn7)), ((((((locals.var_ps0_dn10 + locals.var_vds_dn10) - locals.var_t1__blk1130_dn10) / p.p64) - locals.var_ec_dn10) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn10)), ((((((locals.var_ps0_dn11 + locals.var_vds_dn11) - locals.var_t1__blk1130_dn11) / p.p64) - locals.var_ec_dn11) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn11)), ((((((locals.var_ps0_dn12 + locals.var_vds_dn12) - locals.var_t1__blk1130_dn12) / p.p64) - locals.var_ec_dn12) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn12)), ((((((locals.var_ps0_dn17 + locals.var_vds_dn17) - locals.var_t1__blk1130_dn17) / p.p64) - locals.var_ec_dn17) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn17)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34440_e49527;
        locals.var_qy_dn0 = assign34440_e49527_d_n0;
        locals.var_qy_dn2 = assign34440_e49527_d_n2;
        locals.var_qy_dn6 = assign34440_e49527_d_n6;
        locals.var_qy_dn7 = assign34440_e49527_d_n7;
        locals.var_qy_dn10 = assign34440_e49527_d_n10;
        locals.var_qy_dn11 = assign34440_e49527_d_n11;
        locals.var_qy_dn12 = assign34440_e49527_d_n12;
        locals.var_qy_dn17 = assign34440_e49527_d_n17;

        let assign34450_e49530: f64 = if p.p65 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1136 = assign34450_e49530;

        let (assign34460_e49538, assign34460_e49538_d_n0, assign34460_e49538_d_n2, assign34460_e49538_d_n6, assign34460_e49538_d_n7, assign34460_e49538_d_n10, assign34460_e49538_d_n11, assign34460_e49538_d_n12, assign34460_e49538_d_n17,) = {
    if (locals.var_guard1136 != 0.0) {
        let assign34460_e49535: f64 = (locals.var_cqyb0 * locals.var_vbsp);
        let assign34460_e49536: f64 = (locals.var_qy + assign34460_e49535);
        (assign34460_e49536, (locals.var_qy_dn0 + (locals.var_cqyb0 * locals.var_vbsp_dn0)), (locals.var_qy_dn2 + (locals.var_cqyb0 * locals.var_vbsp_dn2)), (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbsp_dn6)), (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbsp_dn7)), (locals.var_qy_dn10 + (locals.var_cqyb0 * locals.var_vbsp_dn10)), (locals.var_qy_dn11 + (locals.var_cqyb0 * locals.var_vbsp_dn11)), (locals.var_qy_dn12 + (locals.var_cqyb0 * locals.var_vbsp_dn12)), (locals.var_qy_dn17 + (locals.var_cqyb0 * locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34460_e49538;
        locals.var_qy_dn0 = assign34460_e49538_d_n0;
        locals.var_qy_dn2 = assign34460_e49538_d_n2;
        locals.var_qy_dn6 = assign34460_e49538_d_n6;
        locals.var_qy_dn7 = assign34460_e49538_d_n7;
        locals.var_qy_dn10 = assign34460_e49538_d_n10;
        locals.var_qy_dn11 = assign34460_e49538_d_n11;
        locals.var_qy_dn12 = assign34460_e49538_d_n12;
        locals.var_qy_dn17 = assign34460_e49538_d_n17;

        let assign34470_e49541: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1137 = assign34470_e49541;

        let assign34480_e49544: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1138 = assign34480_e49544;

        let (assign34490_e49557, assign34490_e49557_d_n0, assign34490_e49557_d_n2, assign34490_e49557_d_n6, assign34490_e49557_d_n7, assign34490_e49557_d_n10, assign34490_e49557_d_n11, assign34490_e49557_d_n12, assign34490_e49557_d_n17,) = {
    if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
        let assign34490_e49549: f64 = (-locals.var_qbody_bt_p_sus);
        let assign34490_e49551: f64 = (assign34490_e49549 - locals.var_qbody_bt_p_sud);
        let assign34490_e49553: f64 = (assign34490_e49551 - locals.var_qbody_bt_n_sus);
        let assign34490_e49555: f64 = (assign34490_e49553 - locals.var_qbody_bt_n_sud);
        (assign34490_e49555, ((((-locals.var_qbody_bt_p_sus_dn0) - locals.var_qbody_bt_p_sud_dn0) - locals.var_qbody_bt_n_sus_dn0) - locals.var_qbody_bt_n_sud_dn0), ((((-locals.var_qbody_bt_p_sus_dn2) - locals.var_qbody_bt_p_sud_dn2) - locals.var_qbody_bt_n_sus_dn2) - locals.var_qbody_bt_n_sud_dn2), ((((-locals.var_qbody_bt_p_sus_dn6) - locals.var_qbody_bt_p_sud_dn6) - locals.var_qbody_bt_n_sus_dn6) - locals.var_qbody_bt_n_sud_dn6), ((((-locals.var_qbody_bt_p_sus_dn7) - locals.var_qbody_bt_p_sud_dn7) - locals.var_qbody_bt_n_sus_dn7) - locals.var_qbody_bt_n_sud_dn7), ((((-locals.var_qbody_bt_p_sus_dn10) - locals.var_qbody_bt_p_sud_dn10) - locals.var_qbody_bt_n_sus_dn10) - locals.var_qbody_bt_n_sud_dn10), ((((-locals.var_qbody_bt_p_sus_dn11) - locals.var_qbody_bt_p_sud_dn11) - locals.var_qbody_bt_n_sus_dn11) - locals.var_qbody_bt_n_sud_dn11), ((((-locals.var_qbody_bt_p_sus_dn12) - locals.var_qbody_bt_p_sud_dn12) - locals.var_qbody_bt_n_sus_dn12) - locals.var_qbody_bt_n_sud_dn12), ((((-locals.var_qbody_bt_p_sus_dn17) - locals.var_qbody_bt_p_sud_dn17) - locals.var_qbody_bt_n_sus_dn17) - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_q_bt_ge, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, locals.var_q_bt_ge_dn17,)
    }
};
        locals.var_q_bt_ge = assign34490_e49557;
        locals.var_q_bt_ge_dn0 = assign34490_e49557_d_n0;
        locals.var_q_bt_ge_dn2 = assign34490_e49557_d_n2;
        locals.var_q_bt_ge_dn6 = assign34490_e49557_d_n6;
        locals.var_q_bt_ge_dn7 = assign34490_e49557_d_n7;
        locals.var_q_bt_ge_dn10 = assign34490_e49557_d_n10;
        locals.var_q_bt_ge_dn11 = assign34490_e49557_d_n11;
        locals.var_q_bt_ge_dn12 = assign34490_e49557_d_n12;
        locals.var_q_bt_ge_dn17 = assign34490_e49557_d_n17;

        let (assign34500_e49565, assign34500_e49565_d_n0, assign34500_e49565_d_n2, assign34500_e49565_d_n6, assign34500_e49565_d_n7, assign34500_e49565_d_n10, assign34500_e49565_d_n11, assign34500_e49565_d_n12, assign34500_e49565_d_n17,) = {
    if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
        let assign34500_e49563: f64 = (locals.var_qbody_bt_p_iud + locals.var_qbody_bt_n_iud);
        (assign34500_e49563, (locals.var_qbody_bt_p_iud_dn0 + locals.var_qbody_bt_n_iud_dn0), (locals.var_qbody_bt_p_iud_dn2 + locals.var_qbody_bt_n_iud_dn2), (locals.var_qbody_bt_p_iud_dn6 + locals.var_qbody_bt_n_iud_dn6), (locals.var_qbody_bt_p_iud_dn7 + locals.var_qbody_bt_n_iud_dn7), (locals.var_qbody_bt_p_iud_dn10 + locals.var_qbody_bt_n_iud_dn10), (locals.var_qbody_bt_p_iud_dn11 + locals.var_qbody_bt_n_iud_dn11), (locals.var_qbody_bt_p_iud_dn12 + locals.var_qbody_bt_n_iud_dn12), (locals.var_qbody_bt_p_iud_dn17 + locals.var_qbody_bt_n_iud_dn17),)
    } else {
        (locals.var_q_bt_de, locals.var_q_bt_de_dn0, locals.var_q_bt_de_dn2, locals.var_q_bt_de_dn6, locals.var_q_bt_de_dn7, locals.var_q_bt_de_dn10, locals.var_q_bt_de_dn11, locals.var_q_bt_de_dn12, locals.var_q_bt_de_dn17,)
    }
};
        locals.var_q_bt_de = assign34500_e49565;
        locals.var_q_bt_de_dn0 = assign34500_e49565_d_n0;
        locals.var_q_bt_de_dn2 = assign34500_e49565_d_n2;
        locals.var_q_bt_de_dn6 = assign34500_e49565_d_n6;
        locals.var_q_bt_de_dn7 = assign34500_e49565_d_n7;
        locals.var_q_bt_de_dn10 = assign34500_e49565_d_n10;
        locals.var_q_bt_de_dn11 = assign34500_e49565_d_n11;
        locals.var_q_bt_de_dn12 = assign34500_e49565_d_n12;
        locals.var_q_bt_de_dn17 = assign34500_e49565_d_n17;

        let (assign34510_e49573, assign34510_e49573_d_n0, assign34510_e49573_d_n2, assign34510_e49573_d_n6, assign34510_e49573_d_n7, assign34510_e49573_d_n10, assign34510_e49573_d_n11, assign34510_e49573_d_n12, assign34510_e49573_d_n17,) = {
    if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
        let assign34510_e49571: f64 = (locals.var_qbody_bt_p_ius + locals.var_qbody_bt_n_ius);
        (assign34510_e49571, (locals.var_qbody_bt_p_ius_dn0 + locals.var_qbody_bt_n_ius_dn0), (locals.var_qbody_bt_p_ius_dn2 + locals.var_qbody_bt_n_ius_dn2), (locals.var_qbody_bt_p_ius_dn6 + locals.var_qbody_bt_n_ius_dn6), (locals.var_qbody_bt_p_ius_dn7 + locals.var_qbody_bt_n_ius_dn7), (locals.var_qbody_bt_p_ius_dn10 + locals.var_qbody_bt_n_ius_dn10), (locals.var_qbody_bt_p_ius_dn11 + locals.var_qbody_bt_n_ius_dn11), (locals.var_qbody_bt_p_ius_dn12 + locals.var_qbody_bt_n_ius_dn12), (locals.var_qbody_bt_p_ius_dn17 + locals.var_qbody_bt_n_ius_dn17),)
    } else {
        (locals.var_q_bt_se, locals.var_q_bt_se_dn0, locals.var_q_bt_se_dn2, locals.var_q_bt_se_dn6, locals.var_q_bt_se_dn7, locals.var_q_bt_se_dn10, locals.var_q_bt_se_dn11, locals.var_q_bt_se_dn12, locals.var_q_bt_se_dn17,)
    }
};
        locals.var_q_bt_se = assign34510_e49573;
        locals.var_q_bt_se_dn0 = assign34510_e49573_d_n0;
        locals.var_q_bt_se_dn2 = assign34510_e49573_d_n2;
        locals.var_q_bt_se_dn6 = assign34510_e49573_d_n6;
        locals.var_q_bt_se_dn7 = assign34510_e49573_d_n7;
        locals.var_q_bt_se_dn10 = assign34510_e49573_d_n10;
        locals.var_q_bt_se_dn11 = assign34510_e49573_d_n11;
        locals.var_q_bt_se_dn12 = assign34510_e49573_d_n12;
        locals.var_q_bt_se_dn17 = assign34510_e49573_d_n17;

        let (assign34520_e49595, assign34520_e49595_d_n0, assign34520_e49595_d_n2, assign34520_e49595_d_n6, assign34520_e49595_d_n7, assign34520_e49595_d_n10, assign34520_e49595_d_n11, assign34520_e49595_d_n12, assign34520_e49595_d_n13, assign34520_e49595_d_n15, assign34520_e49595_d_n16, assign34520_e49595_d_n17, assign34520_e49595_d_n18,) = {
    if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
        let assign34520_e49581: f64 = (locals.var_qgod + locals.var_qgos);
        let assign34520_e49583: f64 = (assign34520_e49581 + locals.var_qgob);
        let assign34520_e49585: f64 = (assign34520_e49583 - locals.var_qy);
        let assign34520_e49587: f64 = (assign34520_e49585 - locals.var_qovs);
        let assign34520_e49589: f64 = (assign34520_e49587 - locals.var_qovd);
        let assign34520_e49591: f64 = (assign34520_e49589 + locals.var_q_bt_ge);
        let assign34520_e49592: f64 = (locals.var_mfactor * assign34520_e49591);
        let assign34520_e49593: f64 = (locals.var_qge + assign34520_e49592);
        (assign34520_e49593, (locals.var_qge_dn0 + (locals.var_mfactor * ((((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0) + locals.var_q_bt_ge_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2) + locals.var_q_bt_ge_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * ((((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6) + locals.var_q_bt_ge_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7) + locals.var_q_bt_ge_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * ((((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10) + locals.var_q_bt_ge_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11) + locals.var_q_bt_ge_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * ((((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12) + locals.var_q_bt_ge_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * ((((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17) + locals.var_q_bt_ge_dn17))), locals.var_qge_dn18,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34520_e49595;
        locals.var_qge_dn0 = assign34520_e49595_d_n0;
        locals.var_qge_dn2 = assign34520_e49595_d_n2;
        locals.var_qge_dn6 = assign34520_e49595_d_n6;
        locals.var_qge_dn7 = assign34520_e49595_d_n7;
        locals.var_qge_dn10 = assign34520_e49595_d_n10;
        locals.var_qge_dn11 = assign34520_e49595_d_n11;
        locals.var_qge_dn12 = assign34520_e49595_d_n12;
        locals.var_qge_dn13 = assign34520_e49595_d_n13;
        locals.var_qge_dn15 = assign34520_e49595_d_n15;
        locals.var_qge_dn16 = assign34520_e49595_d_n16;
        locals.var_qge_dn17 = assign34520_e49595_d_n17;
        locals.var_qge_dn18 = assign34520_e49595_d_n18;

        let (assign34530_e49612, assign34530_e49612_d_n0, assign34530_e49612_d_n2, assign34530_e49612_d_n6, assign34530_e49612_d_n7, assign34530_e49612_d_n10, assign34530_e49612_d_n11, assign34530_e49612_d_n12, assign34530_e49612_d_n13, assign34530_e49612_d_n15, assign34530_e49612_d_n16, assign34530_e49612_d_n17, assign34530_e49612_d_n18,) = {
    if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
        let assign34530_e49602: f64 = (-locals.var_qgod);
        let assign34530_e49604: f64 = (assign34530_e49602 + locals.var_qy);
        let assign34530_e49606: f64 = (assign34530_e49604 + locals.var_qbdld);
        let assign34530_e49608: f64 = (assign34530_e49606 + locals.var_q_bt_de);
        let assign34530_e49609: f64 = (locals.var_mfactor * assign34530_e49608);
        let assign34530_e49610: f64 = (locals.var_qde + assign34530_e49609);
        (assign34530_e49610, (locals.var_qde_dn0 + (locals.var_mfactor * ((((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0) + locals.var_q_bt_de_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2) + locals.var_q_bt_de_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * ((((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6) + locals.var_q_bt_de_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7) + locals.var_q_bt_de_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * ((((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10) + locals.var_q_bt_de_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11) + locals.var_q_bt_de_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * ((((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12) + locals.var_q_bt_de_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * ((((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17) + locals.var_q_bt_de_dn17))), locals.var_qde_dn18,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34530_e49612;
        locals.var_qde_dn0 = assign34530_e49612_d_n0;
        locals.var_qde_dn2 = assign34530_e49612_d_n2;
        locals.var_qde_dn6 = assign34530_e49612_d_n6;
        locals.var_qde_dn7 = assign34530_e49612_d_n7;
        locals.var_qde_dn10 = assign34530_e49612_d_n10;
        locals.var_qde_dn11 = assign34530_e49612_d_n11;
        locals.var_qde_dn12 = assign34530_e49612_d_n12;
        locals.var_qde_dn13 = assign34530_e49612_d_n13;
        locals.var_qde_dn15 = assign34530_e49612_d_n15;
        locals.var_qde_dn16 = assign34530_e49612_d_n16;
        locals.var_qde_dn17 = assign34530_e49612_d_n17;
        locals.var_qde_dn18 = assign34530_e49612_d_n18;

        let (assign34540_e49627, assign34540_e49627_d_n0, assign34540_e49627_d_n2, assign34540_e49627_d_n6, assign34540_e49627_d_n7, assign34540_e49627_d_n10, assign34540_e49627_d_n11, assign34540_e49627_d_n12, assign34540_e49627_d_n13, assign34540_e49627_d_n15, assign34540_e49627_d_n16, assign34540_e49627_d_n17, assign34540_e49627_d_n18,) = {
    if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
        let assign34540_e49619: f64 = (-locals.var_qgos);
        let assign34540_e49621: f64 = (assign34540_e49619 + locals.var_qbsld);
        let assign34540_e49623: f64 = (assign34540_e49621 + locals.var_q_bt_se);
        let assign34540_e49624: f64 = (locals.var_mfactor * assign34540_e49623);
        let assign34540_e49625: f64 = (locals.var_qse + assign34540_e49624);
        (assign34540_e49625, (locals.var_qse_dn0 + (locals.var_mfactor * (((-locals.var_qgos_dn0) + locals.var_qbsld_dn0) + locals.var_q_bt_se_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * (((-locals.var_qgos_dn2) + locals.var_qbsld_dn2) + locals.var_q_bt_se_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * (((-locals.var_qgos_dn6) + locals.var_qbsld_dn6) + locals.var_q_bt_se_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * (((-locals.var_qgos_dn7) + locals.var_qbsld_dn7) + locals.var_q_bt_se_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * (((-locals.var_qgos_dn10) + locals.var_qbsld_dn10) + locals.var_q_bt_se_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * (((-locals.var_qgos_dn11) + locals.var_qbsld_dn11) + locals.var_q_bt_se_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * (((-locals.var_qgos_dn12) + locals.var_qbsld_dn12) + locals.var_q_bt_se_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * (((-locals.var_qgos_dn17) + locals.var_qbsld_dn17) + locals.var_q_bt_se_dn17))), locals.var_qse_dn18,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34540_e49627;
        locals.var_qse_dn0 = assign34540_e49627_d_n0;
        locals.var_qse_dn2 = assign34540_e49627_d_n2;
        locals.var_qse_dn6 = assign34540_e49627_d_n6;
        locals.var_qse_dn7 = assign34540_e49627_d_n7;
        locals.var_qse_dn10 = assign34540_e49627_d_n10;
        locals.var_qse_dn11 = assign34540_e49627_d_n11;
        locals.var_qse_dn12 = assign34540_e49627_d_n12;
        locals.var_qse_dn13 = assign34540_e49627_d_n13;
        locals.var_qse_dn15 = assign34540_e49627_d_n15;
        locals.var_qse_dn16 = assign34540_e49627_d_n16;
        locals.var_qse_dn17 = assign34540_e49627_d_n17;
        locals.var_qse_dn18 = assign34540_e49627_d_n18;

        let (assign34550_e49648, assign34550_e49648_d_n0, assign34550_e49648_d_n2, assign34550_e49648_d_n6, assign34550_e49648_d_n7, assign34550_e49648_d_n10, assign34550_e49648_d_n11, assign34550_e49648_d_n12, assign34550_e49648_d_n13, assign34550_e49648_d_n15, assign34550_e49648_d_n16, assign34550_e49648_d_n17, assign34550_e49648_d_n18,) = {
    if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 == 0.0)) {
        let assign34550_e49636: f64 = (locals.var_qgod + locals.var_qgos);
        let assign34550_e49638: f64 = (assign34550_e49636 + locals.var_qgob);
        let assign34550_e49640: f64 = (assign34550_e49638 - locals.var_qy);
        let assign34550_e49642: f64 = (assign34550_e49640 - locals.var_qovs);
        let assign34550_e49644: f64 = (assign34550_e49642 - locals.var_qovd);
        let assign34550_e49645: f64 = (locals.var_mfactor * assign34550_e49644);
        let assign34550_e49646: f64 = (locals.var_qge + assign34550_e49645);
        (assign34550_e49646, (locals.var_qge_dn0 + (locals.var_mfactor * (((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * (((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * (((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * (((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * (((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * (((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * (((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * (((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17))), locals.var_qge_dn18,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34550_e49648;
        locals.var_qge_dn0 = assign34550_e49648_d_n0;
        locals.var_qge_dn2 = assign34550_e49648_d_n2;
        locals.var_qge_dn6 = assign34550_e49648_d_n6;
        locals.var_qge_dn7 = assign34550_e49648_d_n7;
        locals.var_qge_dn10 = assign34550_e49648_d_n10;
        locals.var_qge_dn11 = assign34550_e49648_d_n11;
        locals.var_qge_dn12 = assign34550_e49648_d_n12;
        locals.var_qge_dn13 = assign34550_e49648_d_n13;
        locals.var_qge_dn15 = assign34550_e49648_d_n15;
        locals.var_qge_dn16 = assign34550_e49648_d_n16;
        locals.var_qge_dn17 = assign34550_e49648_d_n17;
        locals.var_qge_dn18 = assign34550_e49648_d_n18;

        let (assign34560_e49664, assign34560_e49664_d_n0, assign34560_e49664_d_n2, assign34560_e49664_d_n6, assign34560_e49664_d_n7, assign34560_e49664_d_n10, assign34560_e49664_d_n11, assign34560_e49664_d_n12, assign34560_e49664_d_n13, assign34560_e49664_d_n15, assign34560_e49664_d_n16, assign34560_e49664_d_n17, assign34560_e49664_d_n18,) = {
    if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 == 0.0)) {
        let assign34560_e49656: f64 = (-locals.var_qgod);
        let assign34560_e49658: f64 = (assign34560_e49656 + locals.var_qy);
        let assign34560_e49660: f64 = (assign34560_e49658 + locals.var_qbdld);
        let assign34560_e49661: f64 = (locals.var_mfactor * assign34560_e49660);
        let assign34560_e49662: f64 = (locals.var_qde + assign34560_e49661);
        (assign34560_e49662, (locals.var_qde_dn0 + (locals.var_mfactor * (((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * (((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * (((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * (((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * (((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * (((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * (((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * (((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17))), locals.var_qde_dn18,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34560_e49664;
        locals.var_qde_dn0 = assign34560_e49664_d_n0;
        locals.var_qde_dn2 = assign34560_e49664_d_n2;
        locals.var_qde_dn6 = assign34560_e49664_d_n6;
        locals.var_qde_dn7 = assign34560_e49664_d_n7;
        locals.var_qde_dn10 = assign34560_e49664_d_n10;
        locals.var_qde_dn11 = assign34560_e49664_d_n11;
        locals.var_qde_dn12 = assign34560_e49664_d_n12;
        locals.var_qde_dn13 = assign34560_e49664_d_n13;
        locals.var_qde_dn15 = assign34560_e49664_d_n15;
        locals.var_qde_dn16 = assign34560_e49664_d_n16;
        locals.var_qde_dn17 = assign34560_e49664_d_n17;
        locals.var_qde_dn18 = assign34560_e49664_d_n18;

        let (assign34570_e49678, assign34570_e49678_d_n0, assign34570_e49678_d_n2, assign34570_e49678_d_n6, assign34570_e49678_d_n7, assign34570_e49678_d_n10, assign34570_e49678_d_n11, assign34570_e49678_d_n12, assign34570_e49678_d_n13, assign34570_e49678_d_n15, assign34570_e49678_d_n16, assign34570_e49678_d_n17, assign34570_e49678_d_n18,) = {
    if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 == 0.0)) {
        let assign34570_e49672: f64 = (-locals.var_qgos);
        let assign34570_e49674: f64 = (assign34570_e49672 + locals.var_qbsld);
        let assign34570_e49675: f64 = (locals.var_mfactor * assign34570_e49674);
        let assign34570_e49676: f64 = (locals.var_qse + assign34570_e49675);
        (assign34570_e49676, (locals.var_qse_dn0 + (locals.var_mfactor * ((-locals.var_qgos_dn0) + locals.var_qbsld_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * ((-locals.var_qgos_dn2) + locals.var_qbsld_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * ((-locals.var_qgos_dn6) + locals.var_qbsld_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * ((-locals.var_qgos_dn7) + locals.var_qbsld_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * ((-locals.var_qgos_dn10) + locals.var_qbsld_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * ((-locals.var_qgos_dn11) + locals.var_qbsld_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * ((-locals.var_qgos_dn12) + locals.var_qbsld_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * ((-locals.var_qgos_dn17) + locals.var_qbsld_dn17))), locals.var_qse_dn18,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34570_e49678;
        locals.var_qse_dn0 = assign34570_e49678_d_n0;
        locals.var_qse_dn2 = assign34570_e49678_d_n2;
        locals.var_qse_dn6 = assign34570_e49678_d_n6;
        locals.var_qse_dn7 = assign34570_e49678_d_n7;
        locals.var_qse_dn10 = assign34570_e49678_d_n10;
        locals.var_qse_dn11 = assign34570_e49678_d_n11;
        locals.var_qse_dn12 = assign34570_e49678_d_n12;
        locals.var_qse_dn13 = assign34570_e49678_d_n13;
        locals.var_qse_dn15 = assign34570_e49678_d_n15;
        locals.var_qse_dn16 = assign34570_e49678_d_n16;
        locals.var_qse_dn17 = assign34570_e49678_d_n17;
        locals.var_qse_dn18 = assign34570_e49678_d_n18;

        let assign34600_e49683: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1139 = assign34600_e49683;

        let (assign34610_e49689, assign34610_e49689_d_n0, assign34610_e49689_d_n2, assign34610_e49689_d_n6, assign34610_e49689_d_n7, assign34610_e49689_d_n10, assign34610_e49689_d_n11, assign34610_e49689_d_n12, assign34610_e49689_d_n17,) = {
    if (locals.var_guard1139 != 0.0) {
        let assign34610_e49687: f64 = (locals.var_mfactor * locals.var_ibs);
        (assign34610_e49687, (locals.var_mfactor * locals.var_ibs_dn0), (locals.var_mfactor * locals.var_ibs_dn2), (locals.var_mfactor * locals.var_ibs_dn6), (locals.var_mfactor * locals.var_ibs_dn7), (locals.var_mfactor * locals.var_ibs_dn10), (locals.var_mfactor * locals.var_ibs_dn11), (locals.var_mfactor * locals.var_ibs_dn12), (locals.var_mfactor * locals.var_ibs_dn17),)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign34610_e49689;
        locals.var_ibsb_dn0 = assign34610_e49689_d_n0;
        locals.var_ibsb_dn2 = assign34610_e49689_d_n2;
        locals.var_ibsb_dn6 = assign34610_e49689_d_n6;
        locals.var_ibsb_dn7 = assign34610_e49689_d_n7;
        locals.var_ibsb_dn10 = assign34610_e49689_d_n10;
        locals.var_ibsb_dn11 = assign34610_e49689_d_n11;
        locals.var_ibsb_dn12 = assign34610_e49689_d_n12;
        locals.var_ibsb_dn17 = assign34610_e49689_d_n17;

    }

    pub(super) fn stamp_transient_block_121(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34620_e49695, assign34620_e49695_d_n0, assign34620_e49695_d_n2, assign34620_e49695_d_n6, assign34620_e49695_d_n7, assign34620_e49695_d_n10, assign34620_e49695_d_n11, assign34620_e49695_d_n12, assign34620_e49695_d_n17,) = {
    if (locals.var_guard1139 != 0.0) {
        let assign34620_e49693: f64 = (locals.var_mfactor * locals.var_ibd);
        (assign34620_e49693, (locals.var_mfactor * locals.var_ibd_dn0), (locals.var_mfactor * locals.var_ibd_dn2), (locals.var_mfactor * locals.var_ibd_dn6), (locals.var_mfactor * locals.var_ibd_dn7), (locals.var_mfactor * locals.var_ibd_dn10), (locals.var_mfactor * locals.var_ibd_dn11), (locals.var_mfactor * locals.var_ibd_dn12), (locals.var_mfactor * locals.var_ibd_dn17),)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign34620_e49695;
        locals.var_ibdb_dn0 = assign34620_e49695_d_n0;
        locals.var_ibdb_dn2 = assign34620_e49695_d_n2;
        locals.var_ibdb_dn6 = assign34620_e49695_d_n6;
        locals.var_ibdb_dn7 = assign34620_e49695_d_n7;
        locals.var_ibdb_dn10 = assign34620_e49695_d_n10;
        locals.var_ibdb_dn11 = assign34620_e49695_d_n11;
        locals.var_ibdb_dn12 = assign34620_e49695_d_n12;
        locals.var_ibdb_dn17 = assign34620_e49695_d_n17;

        let (assign34630_e49701, assign34630_e49701_d_n0, assign34630_e49701_d_n2, assign34630_e49701_d_n6, assign34630_e49701_d_n7, assign34630_e49701_d_n10, assign34630_e49701_d_n11, assign34630_e49701_d_n12, assign34630_e49701_d_n17,) = {
    if (locals.var_guard1139 != 0.0) {
        let assign34630_e49699: f64 = (locals.var_mfactor * locals.var_qbd);
        (assign34630_e49699, (locals.var_mfactor * locals.var_qbd_dn0), (locals.var_mfactor * locals.var_qbd_dn2), (locals.var_mfactor * locals.var_qbd_dn6), (locals.var_mfactor * locals.var_qbd_dn7), (locals.var_mfactor * locals.var_qbd_dn10), (locals.var_mfactor * locals.var_qbd_dn11), (locals.var_mfactor * locals.var_qbd_dn12), (locals.var_mfactor * locals.var_qbd_dn17),)
    } else {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    }
};
        locals.var_qbd_s0 = assign34630_e49701;
        locals.var_qbd_s0_dn0 = assign34630_e49701_d_n0;
        locals.var_qbd_s0_dn2 = assign34630_e49701_d_n2;
        locals.var_qbd_s0_dn6 = assign34630_e49701_d_n6;
        locals.var_qbd_s0_dn7 = assign34630_e49701_d_n7;
        locals.var_qbd_s0_dn10 = assign34630_e49701_d_n10;
        locals.var_qbd_s0_dn11 = assign34630_e49701_d_n11;
        locals.var_qbd_s0_dn12 = assign34630_e49701_d_n12;
        locals.var_qbd_s0_dn17 = assign34630_e49701_d_n17;

        let (assign34640_e49707, assign34640_e49707_d_n0, assign34640_e49707_d_n2, assign34640_e49707_d_n6, assign34640_e49707_d_n7, assign34640_e49707_d_n10, assign34640_e49707_d_n11, assign34640_e49707_d_n12, assign34640_e49707_d_n17,) = {
    if (locals.var_guard1139 != 0.0) {
        let assign34640_e49705: f64 = (locals.var_mfactor * locals.var_qbs);
        (assign34640_e49705, (locals.var_mfactor * locals.var_qbs_dn0), (locals.var_mfactor * locals.var_qbs_dn2), (locals.var_mfactor * locals.var_qbs_dn6), (locals.var_mfactor * locals.var_qbs_dn7), (locals.var_mfactor * locals.var_qbs_dn10), (locals.var_mfactor * locals.var_qbs_dn11), (locals.var_mfactor * locals.var_qbs_dn12), (locals.var_mfactor * locals.var_qbs_dn17),)
    } else {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    }
};
        locals.var_qbs_s0 = assign34640_e49707;
        locals.var_qbs_s0_dn0 = assign34640_e49707_d_n0;
        locals.var_qbs_s0_dn2 = assign34640_e49707_d_n2;
        locals.var_qbs_s0_dn6 = assign34640_e49707_d_n6;
        locals.var_qbs_s0_dn7 = assign34640_e49707_d_n7;
        locals.var_qbs_s0_dn10 = assign34640_e49707_d_n10;
        locals.var_qbs_s0_dn11 = assign34640_e49707_d_n11;
        locals.var_qbs_s0_dn12 = assign34640_e49707_d_n12;
        locals.var_qbs_s0_dn17 = assign34640_e49707_d_n17;

        let (assign34650_e49712, assign34650_e49712_d_n0, assign34650_e49712_d_n2, assign34650_e49712_d_n6, assign34650_e49712_d_n7, assign34650_e49712_d_n10, assign34650_e49712_d_n11, assign34650_e49712_d_n12, assign34650_e49712_d_n17,) = {
    if (locals.var_guard1139 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign34650_e49712;
        locals.var_ibsb_dn0 = assign34650_e49712_d_n0;
        locals.var_ibsb_dn2 = assign34650_e49712_d_n2;
        locals.var_ibsb_dn6 = assign34650_e49712_d_n6;
        locals.var_ibsb_dn7 = assign34650_e49712_d_n7;
        locals.var_ibsb_dn10 = assign34650_e49712_d_n10;
        locals.var_ibsb_dn11 = assign34650_e49712_d_n11;
        locals.var_ibsb_dn12 = assign34650_e49712_d_n12;
        locals.var_ibsb_dn17 = assign34650_e49712_d_n17;

        let (assign34660_e49717, assign34660_e49717_d_n0, assign34660_e49717_d_n2, assign34660_e49717_d_n6, assign34660_e49717_d_n7, assign34660_e49717_d_n10, assign34660_e49717_d_n11, assign34660_e49717_d_n12, assign34660_e49717_d_n17,) = {
    if (locals.var_guard1139 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign34660_e49717;
        locals.var_ibdb_dn0 = assign34660_e49717_d_n0;
        locals.var_ibdb_dn2 = assign34660_e49717_d_n2;
        locals.var_ibdb_dn6 = assign34660_e49717_d_n6;
        locals.var_ibdb_dn7 = assign34660_e49717_d_n7;
        locals.var_ibdb_dn10 = assign34660_e49717_d_n10;
        locals.var_ibdb_dn11 = assign34660_e49717_d_n11;
        locals.var_ibdb_dn12 = assign34660_e49717_d_n12;
        locals.var_ibdb_dn17 = assign34660_e49717_d_n17;

        let (assign34670_e49722, assign34670_e49722_d_n0, assign34670_e49722_d_n2, assign34670_e49722_d_n6, assign34670_e49722_d_n7, assign34670_e49722_d_n10, assign34670_e49722_d_n11, assign34670_e49722_d_n12, assign34670_e49722_d_n17,) = {
    if (locals.var_guard1139 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    }
};
        locals.var_qbd_s0 = assign34670_e49722;
        locals.var_qbd_s0_dn0 = assign34670_e49722_d_n0;
        locals.var_qbd_s0_dn2 = assign34670_e49722_d_n2;
        locals.var_qbd_s0_dn6 = assign34670_e49722_d_n6;
        locals.var_qbd_s0_dn7 = assign34670_e49722_d_n7;
        locals.var_qbd_s0_dn10 = assign34670_e49722_d_n10;
        locals.var_qbd_s0_dn11 = assign34670_e49722_d_n11;
        locals.var_qbd_s0_dn12 = assign34670_e49722_d_n12;
        locals.var_qbd_s0_dn17 = assign34670_e49722_d_n17;

        let (assign34680_e49727, assign34680_e49727_d_n0, assign34680_e49727_d_n2, assign34680_e49727_d_n6, assign34680_e49727_d_n7, assign34680_e49727_d_n10, assign34680_e49727_d_n11, assign34680_e49727_d_n12, assign34680_e49727_d_n17,) = {
    if (locals.var_guard1139 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    }
};
        locals.var_qbs_s0 = assign34680_e49727;
        locals.var_qbs_s0_dn0 = assign34680_e49727_d_n0;
        locals.var_qbs_s0_dn2 = assign34680_e49727_d_n2;
        locals.var_qbs_s0_dn6 = assign34680_e49727_d_n6;
        locals.var_qbs_s0_dn7 = assign34680_e49727_d_n7;
        locals.var_qbs_s0_dn10 = assign34680_e49727_d_n10;
        locals.var_qbs_s0_dn11 = assign34680_e49727_d_n11;
        locals.var_qbs_s0_dn12 = assign34680_e49727_d_n12;
        locals.var_qbs_s0_dn17 = assign34680_e49727_d_n17;

        let assign34690_e49730: f64 = if p.p25 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1140 = assign34690_e49730;

        let (assign34700_e49734, assign34700_e49734_d_n0, assign34700_e49734_d_n2, assign34700_e49734_d_n6, assign34700_e49734_d_n7, assign34700_e49734_d_n10, assign34700_e49734_d_n11, assign34700_e49734_d_n12, assign34700_e49734_d_n17,) = {
    if (locals.var_guard1140 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    }
};
        locals.var_isube = assign34700_e49734;
        locals.var_isube_dn0 = assign34700_e49734_d_n0;
        locals.var_isube_dn2 = assign34700_e49734_d_n2;
        locals.var_isube_dn6 = assign34700_e49734_d_n6;
        locals.var_isube_dn7 = assign34700_e49734_d_n7;
        locals.var_isube_dn10 = assign34700_e49734_d_n10;
        locals.var_isube_dn11 = assign34700_e49734_d_n11;
        locals.var_isube_dn12 = assign34700_e49734_d_n12;
        locals.var_isube_dn17 = assign34700_e49734_d_n17;

        let (assign34710_e49741, assign34710_e49741_d_n0, assign34710_e49741_d_n2, assign34710_e49741_d_n6, assign34710_e49741_d_n7, assign34710_e49741_d_n10, assign34710_e49741_d_n11, assign34710_e49741_d_n12, assign34710_e49741_d_n17,) = {
    if (locals.var_guard1140 == 0.0) {
        let assign34710_e49739: f64 = (locals.var_mfactor * locals.var_isub);
        (assign34710_e49739, (locals.var_mfactor * locals.var_isub_dn0), (locals.var_mfactor * locals.var_isub_dn2), (locals.var_mfactor * locals.var_isub_dn6), (locals.var_mfactor * locals.var_isub_dn7), (locals.var_mfactor * locals.var_isub_dn10), (locals.var_mfactor * locals.var_isub_dn11), (locals.var_mfactor * locals.var_isub_dn12), (locals.var_mfactor * locals.var_isub_dn17),)
    } else {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    }
};
        locals.var_isube = assign34710_e49741;
        locals.var_isube_dn0 = assign34710_e49741_d_n0;
        locals.var_isube_dn2 = assign34710_e49741_d_n2;
        locals.var_isube_dn6 = assign34710_e49741_d_n6;
        locals.var_isube_dn7 = assign34710_e49741_d_n7;
        locals.var_isube_dn10 = assign34710_e49741_d_n10;
        locals.var_isube_dn11 = assign34710_e49741_d_n11;
        locals.var_isube_dn12 = assign34710_e49741_d_n12;
        locals.var_isube_dn17 = assign34710_e49741_d_n17;

        let assign34720_e49744: f64 = (-locals.var_igb);
        let assign34720_e49745: f64 = (locals.var_mfactor * assign34720_e49744);
        locals.var_igbe = assign34720_e49745;
        locals.var_igbe_dn0 = (locals.var_mfactor * (-locals.var_igb_dn0));
        locals.var_igbe_dn2 = (locals.var_mfactor * (-locals.var_igb_dn2));
        locals.var_igbe_dn6 = (locals.var_mfactor * (-locals.var_igb_dn6));
        locals.var_igbe_dn7 = (locals.var_mfactor * (-locals.var_igb_dn7));
        locals.var_igbe_dn10 = (locals.var_mfactor * (-locals.var_igb_dn10));
        locals.var_igbe_dn11 = (locals.var_mfactor * (-locals.var_igb_dn11));
        locals.var_igbe_dn12 = (locals.var_mfactor * (-locals.var_igb_dn12));
        locals.var_igbe_dn17 = (locals.var_mfactor * (-locals.var_igb_dn17));

        let assign34730_e49748: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1141 = assign34730_e49748;

        let (assign34740_e49758, assign34740_e49758_d_n0, assign34740_e49758_d_n2, assign34740_e49758_d_n6, assign34740_e49758_d_n7, assign34740_e49758_d_n10, assign34740_e49758_d_n11, assign34740_e49758_d_n12, assign34740_e49758_d_n17,) = {
    if (locals.var_guard1141 != 0.0) {
        let assign34740_e49753: f64 = (locals.var_glpart1 * locals.var_igate);
        let assign34740_e49755: f64 = (assign34740_e49753 - locals.var_igd);
        let assign34740_e49756: f64 = (locals.var_mfactor * assign34740_e49755);
        (assign34740_e49756, (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn0) - locals.var_igd_dn0)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn2) - locals.var_igd_dn2)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn6) - locals.var_igd_dn6)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn7) - locals.var_igd_dn7)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn10) - locals.var_igd_dn10)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn11) - locals.var_igd_dn11)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn12) - locals.var_igd_dn12)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn17) - locals.var_igd_dn17)),)
    } else {
        (locals.var_igde, locals.var_igde_dn0, locals.var_igde_dn2, locals.var_igde_dn6, locals.var_igde_dn7, locals.var_igde_dn10, locals.var_igde_dn11, locals.var_igde_dn12, locals.var_igde_dn17,)
    }
};
        locals.var_igde = assign34740_e49758;
        locals.var_igde_dn0 = assign34740_e49758_d_n0;
        locals.var_igde_dn2 = assign34740_e49758_d_n2;
        locals.var_igde_dn6 = assign34740_e49758_d_n6;
        locals.var_igde_dn7 = assign34740_e49758_d_n7;
        locals.var_igde_dn10 = assign34740_e49758_d_n10;
        locals.var_igde_dn11 = assign34740_e49758_d_n11;
        locals.var_igde_dn12 = assign34740_e49758_d_n12;
        locals.var_igde_dn17 = assign34740_e49758_d_n17;

        let (assign34750_e49771, assign34750_e49771_d_n0, assign34750_e49771_d_n2, assign34750_e49771_d_n6, assign34750_e49771_d_n7, assign34750_e49771_d_n10, assign34750_e49771_d_n11, assign34750_e49771_d_n12, assign34750_e49771_d_n17,) = {
    if (locals.var_guard1141 == 0.0) {
        let assign34750_e49764: f64 = (1.0 - locals.var_glpart1);
        let assign34750_e49766: f64 = (assign34750_e49764 * locals.var_igate);
        let assign34750_e49768: f64 = (assign34750_e49766 - locals.var_igs);
        let assign34750_e49769: f64 = (locals.var_mfactor * assign34750_e49768);
        (assign34750_e49769, (locals.var_mfactor * ((assign34750_e49764 * locals.var_igate_dn0) - locals.var_igs_dn0)), (locals.var_mfactor * ((assign34750_e49764 * locals.var_igate_dn2) - locals.var_igs_dn2)), (locals.var_mfactor * ((assign34750_e49764 * locals.var_igate_dn6) - locals.var_igs_dn6)), (locals.var_mfactor * ((assign34750_e49764 * locals.var_igate_dn7) - locals.var_igs_dn7)), (locals.var_mfactor * ((assign34750_e49764 * locals.var_igate_dn10) - locals.var_igs_dn10)), (locals.var_mfactor * ((assign34750_e49764 * locals.var_igate_dn11) - locals.var_igs_dn11)), (locals.var_mfactor * ((assign34750_e49764 * locals.var_igate_dn12) - locals.var_igs_dn12)), (locals.var_mfactor * ((assign34750_e49764 * locals.var_igate_dn17) - locals.var_igs_dn17)),)
    } else {
        (locals.var_igde, locals.var_igde_dn0, locals.var_igde_dn2, locals.var_igde_dn6, locals.var_igde_dn7, locals.var_igde_dn10, locals.var_igde_dn11, locals.var_igde_dn12, locals.var_igde_dn17,)
    }
};
        locals.var_igde = assign34750_e49771;
        locals.var_igde_dn0 = assign34750_e49771_d_n0;
        locals.var_igde_dn2 = assign34750_e49771_d_n2;
        locals.var_igde_dn6 = assign34750_e49771_d_n6;
        locals.var_igde_dn7 = assign34750_e49771_d_n7;
        locals.var_igde_dn10 = assign34750_e49771_d_n10;
        locals.var_igde_dn11 = assign34750_e49771_d_n11;
        locals.var_igde_dn12 = assign34750_e49771_d_n12;
        locals.var_igde_dn17 = assign34750_e49771_d_n17;

        let assign34760_e49774: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1142 = assign34760_e49774;

        let (assign34770_e49786, assign34770_e49786_d_n0, assign34770_e49786_d_n2, assign34770_e49786_d_n6, assign34770_e49786_d_n7, assign34770_e49786_d_n10, assign34770_e49786_d_n11, assign34770_e49786_d_n12, assign34770_e49786_d_n17,) = {
    if (locals.var_guard1142 != 0.0) {
        let assign34770_e49779: f64 = (1.0 - locals.var_glpart1);
        let assign34770_e49781: f64 = (assign34770_e49779 * locals.var_igate);
        let assign34770_e49783: f64 = (assign34770_e49781 - locals.var_igs);
        let assign34770_e49784: f64 = (locals.var_mfactor * assign34770_e49783);
        (assign34770_e49784, (locals.var_mfactor * ((assign34770_e49779 * locals.var_igate_dn0) - locals.var_igs_dn0)), (locals.var_mfactor * ((assign34770_e49779 * locals.var_igate_dn2) - locals.var_igs_dn2)), (locals.var_mfactor * ((assign34770_e49779 * locals.var_igate_dn6) - locals.var_igs_dn6)), (locals.var_mfactor * ((assign34770_e49779 * locals.var_igate_dn7) - locals.var_igs_dn7)), (locals.var_mfactor * ((assign34770_e49779 * locals.var_igate_dn10) - locals.var_igs_dn10)), (locals.var_mfactor * ((assign34770_e49779 * locals.var_igate_dn11) - locals.var_igs_dn11)), (locals.var_mfactor * ((assign34770_e49779 * locals.var_igate_dn12) - locals.var_igs_dn12)), (locals.var_mfactor * ((assign34770_e49779 * locals.var_igate_dn17) - locals.var_igs_dn17)),)
    } else {
        (locals.var_igse, locals.var_igse_dn0, locals.var_igse_dn2, locals.var_igse_dn6, locals.var_igse_dn7, locals.var_igse_dn10, locals.var_igse_dn11, locals.var_igse_dn12, locals.var_igse_dn17,)
    }
};
        locals.var_igse = assign34770_e49786;
        locals.var_igse_dn0 = assign34770_e49786_d_n0;
        locals.var_igse_dn2 = assign34770_e49786_d_n2;
        locals.var_igse_dn6 = assign34770_e49786_d_n6;
        locals.var_igse_dn7 = assign34770_e49786_d_n7;
        locals.var_igse_dn10 = assign34770_e49786_d_n10;
        locals.var_igse_dn11 = assign34770_e49786_d_n11;
        locals.var_igse_dn12 = assign34770_e49786_d_n12;
        locals.var_igse_dn17 = assign34770_e49786_d_n17;

        let (assign34780_e49797, assign34780_e49797_d_n0, assign34780_e49797_d_n2, assign34780_e49797_d_n6, assign34780_e49797_d_n7, assign34780_e49797_d_n10, assign34780_e49797_d_n11, assign34780_e49797_d_n12, assign34780_e49797_d_n17,) = {
    if (locals.var_guard1142 == 0.0) {
        let assign34780_e49792: f64 = (locals.var_glpart1 * locals.var_igate);
        let assign34780_e49794: f64 = (assign34780_e49792 - locals.var_igd);
        let assign34780_e49795: f64 = (locals.var_mfactor * assign34780_e49794);
        (assign34780_e49795, (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn0) - locals.var_igd_dn0)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn2) - locals.var_igd_dn2)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn6) - locals.var_igd_dn6)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn7) - locals.var_igd_dn7)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn10) - locals.var_igd_dn10)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn11) - locals.var_igd_dn11)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn12) - locals.var_igd_dn12)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn17) - locals.var_igd_dn17)),)
    } else {
        (locals.var_igse, locals.var_igse_dn0, locals.var_igse_dn2, locals.var_igse_dn6, locals.var_igse_dn7, locals.var_igse_dn10, locals.var_igse_dn11, locals.var_igse_dn12, locals.var_igse_dn17,)
    }
};
        locals.var_igse = assign34780_e49797;
        locals.var_igse_dn0 = assign34780_e49797_d_n0;
        locals.var_igse_dn2 = assign34780_e49797_d_n2;
        locals.var_igse_dn6 = assign34780_e49797_d_n6;
        locals.var_igse_dn7 = assign34780_e49797_d_n7;
        locals.var_igse_dn10 = assign34780_e49797_d_n10;
        locals.var_igse_dn11 = assign34780_e49797_d_n11;
        locals.var_igse_dn12 = assign34780_e49797_d_n12;
        locals.var_igse_dn17 = assign34780_e49797_d_n17;

        let (assign34790_e49807, assign34790_e49807_d_n0, assign34790_e49807_d_n2, assign34790_e49807_d_n6, assign34790_e49807_d_n7, assign34790_e49807_d_n10, assign34790_e49807_d_n11, assign34790_e49807_d_n12, assign34790_e49807_d_n17,) = {
    if (locals.var_mode == 1.0) {
        let assign34790_e49803: f64 = (locals.var_mfactor * locals.var_igidl);
        (assign34790_e49803, (locals.var_mfactor * locals.var_igidl_dn0), (locals.var_mfactor * locals.var_igidl_dn2), (locals.var_mfactor * locals.var_igidl_dn6), (locals.var_mfactor * locals.var_igidl_dn7), (locals.var_mfactor * locals.var_igidl_dn10), (locals.var_mfactor * locals.var_igidl_dn11), (locals.var_mfactor * locals.var_igidl_dn12), (locals.var_mfactor * locals.var_igidl_dn17),)
    } else {
        let assign34790_e49806: f64 = (locals.var_mfactor * locals.var_igisl);
        (assign34790_e49806, (locals.var_mfactor * locals.var_igisl_dn0), (locals.var_mfactor * locals.var_igisl_dn2), (locals.var_mfactor * locals.var_igisl_dn6), (locals.var_mfactor * locals.var_igisl_dn7), (locals.var_mfactor * locals.var_igisl_dn10), (locals.var_mfactor * locals.var_igisl_dn11), (locals.var_mfactor * locals.var_igisl_dn12), (locals.var_mfactor * locals.var_igisl_dn17),)
    }
};
        locals.var_igidle = assign34790_e49807;
        locals.var_igidle_dn0 = assign34790_e49807_d_n0;
        locals.var_igidle_dn2 = assign34790_e49807_d_n2;
        locals.var_igidle_dn6 = assign34790_e49807_d_n6;
        locals.var_igidle_dn7 = assign34790_e49807_d_n7;
        locals.var_igidle_dn10 = assign34790_e49807_d_n10;
        locals.var_igidle_dn11 = assign34790_e49807_d_n11;
        locals.var_igidle_dn12 = assign34790_e49807_d_n12;
        locals.var_igidle_dn17 = assign34790_e49807_d_n17;

        let (assign34800_e49817, assign34800_e49817_d_n0, assign34800_e49817_d_n2, assign34800_e49817_d_n6, assign34800_e49817_d_n7, assign34800_e49817_d_n10, assign34800_e49817_d_n11, assign34800_e49817_d_n12, assign34800_e49817_d_n17,) = {
    if (locals.var_mode == 1.0) {
        let assign34800_e49813: f64 = (locals.var_mfactor * locals.var_igisl);
        (assign34800_e49813, (locals.var_mfactor * locals.var_igisl_dn0), (locals.var_mfactor * locals.var_igisl_dn2), (locals.var_mfactor * locals.var_igisl_dn6), (locals.var_mfactor * locals.var_igisl_dn7), (locals.var_mfactor * locals.var_igisl_dn10), (locals.var_mfactor * locals.var_igisl_dn11), (locals.var_mfactor * locals.var_igisl_dn12), (locals.var_mfactor * locals.var_igisl_dn17),)
    } else {
        let assign34800_e49816: f64 = (locals.var_mfactor * locals.var_igidl);
        (assign34800_e49816, (locals.var_mfactor * locals.var_igidl_dn0), (locals.var_mfactor * locals.var_igidl_dn2), (locals.var_mfactor * locals.var_igidl_dn6), (locals.var_mfactor * locals.var_igidl_dn7), (locals.var_mfactor * locals.var_igidl_dn10), (locals.var_mfactor * locals.var_igidl_dn11), (locals.var_mfactor * locals.var_igidl_dn12), (locals.var_mfactor * locals.var_igidl_dn17),)
    }
};
        locals.var_igisle = assign34800_e49817;
        locals.var_igisle_dn0 = assign34800_e49817_d_n0;
        locals.var_igisle_dn2 = assign34800_e49817_d_n2;
        locals.var_igisle_dn6 = assign34800_e49817_d_n6;
        locals.var_igisle_dn7 = assign34800_e49817_d_n7;
        locals.var_igisle_dn10 = assign34800_e49817_d_n10;
        locals.var_igisle_dn11 = assign34800_e49817_d_n11;
        locals.var_igisle_dn12 = assign34800_e49817_d_n12;
        locals.var_igisle_dn17 = assign34800_e49817_d_n17;

        let assign34820_e49823: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign34820_e49823;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn7 = (locals.var_mfactor * locals.var_nthrml_dn7);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn11 = (locals.var_mfactor * locals.var_nthrml_dn11);
        locals.var_noithrml_dn12 = (locals.var_mfactor * locals.var_nthrml_dn12);
        locals.var_noithrml_dn17 = (locals.var_mfactor * locals.var_nthrml_dn17);

        let assign34830_e49826: f64 = locals.var_qge_dn6;
        locals.var_cgdbd = assign34830_e49826;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn12 = 0.0;
        locals.var_cgdbd_dn13 = 0.0;
        locals.var_cgdbd_dn15 = 0.0;
        locals.var_cgdbd_dn16 = 0.0;
        locals.var_cgdbd_dn17 = 0.0;
        locals.var_cgdbd_dn18 = 0.0;

        let assign34840_e49829: f64 = (p.p50 * locals.var_cgdbd);
        locals.var_cgdbd = assign34840_e49829;
        locals.var_cgdbd_dn0 = (p.p50 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p50 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn6 = (p.p50 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p50 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn10 = (p.p50 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p50 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn12 = (p.p50 * locals.var_cgdbd_dn12);
        locals.var_cgdbd_dn13 = (p.p50 * locals.var_cgdbd_dn13);
        locals.var_cgdbd_dn15 = (p.p50 * locals.var_cgdbd_dn15);
        locals.var_cgdbd_dn16 = (p.p50 * locals.var_cgdbd_dn16);
        locals.var_cgdbd_dn17 = (p.p50 * locals.var_cgdbd_dn17);
        locals.var_cgdbd_dn18 = (p.p50 * locals.var_cgdbd_dn18);

        let assign34850_e49832: f64 = locals.var_qge_dn7;
        locals.var_cgsbd = assign34850_e49832;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn12 = 0.0;
        locals.var_cgsbd_dn13 = 0.0;
        locals.var_cgsbd_dn15 = 0.0;
        locals.var_cgsbd_dn16 = 0.0;
        locals.var_cgsbd_dn17 = 0.0;
        locals.var_cgsbd_dn18 = 0.0;

        let assign34860_e49835: f64 = (p.p50 * locals.var_cgsbd);
        locals.var_cgsbd = assign34860_e49835;
        locals.var_cgsbd_dn0 = (p.p50 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p50 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn6 = (p.p50 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p50 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn10 = (p.p50 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p50 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn12 = (p.p50 * locals.var_cgsbd_dn12);
        locals.var_cgsbd_dn13 = (p.p50 * locals.var_cgsbd_dn13);
        locals.var_cgsbd_dn15 = (p.p50 * locals.var_cgsbd_dn15);
        locals.var_cgsbd_dn16 = (p.p50 * locals.var_cgsbd_dn16);
        locals.var_cgsbd_dn17 = (p.p50 * locals.var_cgsbd_dn17);
        locals.var_cgsbd_dn18 = (p.p50 * locals.var_cgsbd_dn18);

        let (assign34870_e49841, assign34870_e49841_d_n0, assign34870_e49841_d_n2, assign34870_e49841_d_n6, assign34870_e49841_d_n7, assign34870_e49841_d_n10, assign34870_e49841_d_n11, assign34870_e49841_d_n12, assign34870_e49841_d_n13, assign34870_e49841_d_n15, assign34870_e49841_d_n16, assign34870_e49841_d_n17, assign34870_e49841_d_n18,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18,)
    }
};
        locals.var_cgsb = assign34870_e49841;
        locals.var_cgsb_dn0 = assign34870_e49841_d_n0;
        locals.var_cgsb_dn2 = assign34870_e49841_d_n2;
        locals.var_cgsb_dn6 = assign34870_e49841_d_n6;
        locals.var_cgsb_dn7 = assign34870_e49841_d_n7;
        locals.var_cgsb_dn10 = assign34870_e49841_d_n10;
        locals.var_cgsb_dn11 = assign34870_e49841_d_n11;
        locals.var_cgsb_dn12 = assign34870_e49841_d_n12;
        locals.var_cgsb_dn13 = assign34870_e49841_d_n13;
        locals.var_cgsb_dn15 = assign34870_e49841_d_n15;
        locals.var_cgsb_dn16 = assign34870_e49841_d_n16;
        locals.var_cgsb_dn17 = assign34870_e49841_d_n17;
        locals.var_cgsb_dn18 = assign34870_e49841_d_n18;

        let assign34880_e49855: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1149 = assign34880_e49855;

        let (assign34890_e49865,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign34890_e49859: f64 = (1e-6 * locals.var_c_fox);
        let assign34890_e49861: f64 = (assign34890_e49859 * locals.var_weffcv_nf);
        let assign34890_e49863: f64 = (assign34890_e49861 * locals.var_leff_cv);
        (assign34890_e49863,)
    } else {
        (locals.var_t0__blk1143,)
    }
};
        locals.var_t0__blk1143 = assign34890_e49865;

        let (assign34900_e49871, assign34900_e49871_d_n0, assign34900_e49871_d_n2, assign34900_e49871_d_n6, assign34900_e49871_d_n7, assign34900_e49871_d_n10, assign34900_e49871_d_n11, assign34900_e49871_d_n12, assign34900_e49871_d_n13, assign34900_e49871_d_n15, assign34900_e49871_d_n16, assign34900_e49871_d_n17, assign34900_e49871_d_n18,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign34900_e49869: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign34900_e49869, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn12 / locals.var_mfactor), (locals.var_cgsb_dn13 / locals.var_mfactor), (locals.var_cgsb_dn15 / locals.var_mfactor), (locals.var_cgsb_dn16 / locals.var_mfactor), (locals.var_cgsb_dn17 / locals.var_mfactor), (locals.var_cgsb_dn18 / locals.var_mfactor),)
    } else {
        (locals.var_t1__blk1144, locals.var_t1__blk1144_dn0, locals.var_t1__blk1144_dn2, locals.var_t1__blk1144_dn6, locals.var_t1__blk1144_dn7, locals.var_t1__blk1144_dn10, locals.var_t1__blk1144_dn11, locals.var_t1__blk1144_dn12, locals.var_t1__blk1144_dn13, locals.var_t1__blk1144_dn15, locals.var_t1__blk1144_dn16, locals.var_t1__blk1144_dn17, locals.var_t1__blk1144_dn18,)
    }
};
        locals.var_t1__blk1144 = assign34900_e49871;
        locals.var_t1__blk1144_dn0 = assign34900_e49871_d_n0;
        locals.var_t1__blk1144_dn2 = assign34900_e49871_d_n2;
        locals.var_t1__blk1144_dn6 = assign34900_e49871_d_n6;
        locals.var_t1__blk1144_dn7 = assign34900_e49871_d_n7;
        locals.var_t1__blk1144_dn10 = assign34900_e49871_d_n10;
        locals.var_t1__blk1144_dn11 = assign34900_e49871_d_n11;
        locals.var_t1__blk1144_dn12 = assign34900_e49871_d_n12;
        locals.var_t1__blk1144_dn13 = assign34900_e49871_d_n13;
        locals.var_t1__blk1144_dn15 = assign34900_e49871_d_n15;
        locals.var_t1__blk1144_dn16 = assign34900_e49871_d_n16;
        locals.var_t1__blk1144_dn17 = assign34900_e49871_d_n17;
        locals.var_t1__blk1144_dn18 = assign34900_e49871_d_n18;

        let (assign34910_e49885, assign34910_e49885_d_n0, assign34910_e49885_d_n2, assign34910_e49885_d_n6, assign34910_e49885_d_n7, assign34910_e49885_d_n10, assign34910_e49885_d_n11, assign34910_e49885_d_n12, assign34910_e49885_d_n13, assign34910_e49885_d_n15, assign34910_e49885_d_n16, assign34910_e49885_d_n17, assign34910_e49885_d_n18,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign34910_e49875: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign34910_e49877: f64 = (assign34910_e49875 * locals.var_beta_inv);
        let assign34910_e49879: f64 = (assign34910_e49877 * locals.var_t1__blk1144);
        let assign34910_e49881: f64 = (assign34910_e49879 * locals.var_t1__blk1144);
        let assign34910_e49883: f64 = (assign34910_e49881 / locals.var_gds0_ign);
        (assign34910_e49883, ((((((assign34910_e49877 * locals.var_t1__blk1144_dn0) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn0)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn2) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn2)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn6) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn6)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn7) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn7)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign34910_e49875 * locals.var_beta_inv_dn10) * locals.var_t1__blk1144) + (assign34910_e49877 * locals.var_t1__blk1144_dn10)) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn10)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn11) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn11)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn12) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn12)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn12)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34910_e49877 * locals.var_t1__blk1144_dn13) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn13)) / locals.var_gds0_ign), ((((assign34910_e49877 * locals.var_t1__blk1144_dn15) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn15)) / locals.var_gds0_ign), ((((assign34910_e49877 * locals.var_t1__blk1144_dn16) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn16)) / locals.var_gds0_ign), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn17) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn17)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn17)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34910_e49877 * locals.var_t1__blk1144_dn18) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn18)) / locals.var_gds0_ign),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn12, locals.var_nign0_dn13, locals.var_nign0_dn15, locals.var_nign0_dn16, locals.var_nign0_dn17, locals.var_nign0_dn18,)
    }
};
        locals.var_nign0 = assign34910_e49885;
        locals.var_nign0_dn0 = assign34910_e49885_d_n0;
        locals.var_nign0_dn2 = assign34910_e49885_d_n2;
        locals.var_nign0_dn6 = assign34910_e49885_d_n6;
        locals.var_nign0_dn7 = assign34910_e49885_d_n7;
        locals.var_nign0_dn10 = assign34910_e49885_d_n10;
        locals.var_nign0_dn11 = assign34910_e49885_d_n11;
        locals.var_nign0_dn12 = assign34910_e49885_d_n12;
        locals.var_nign0_dn13 = assign34910_e49885_d_n13;
        locals.var_nign0_dn15 = assign34910_e49885_d_n15;
        locals.var_nign0_dn16 = assign34910_e49885_d_n16;
        locals.var_nign0_dn17 = assign34910_e49885_d_n17;
        locals.var_nign0_dn18 = assign34910_e49885_d_n18;

        let assign34920_e49889: f64 = (10.0 * 2.220446049250313e-16);
        let assign34920_e49894: f64 = (10.0 * 2.220446049250313e-16);
        let assign34920_e49896: f64 = if ((locals.var_kusai00l > assign34920_e49889) && (locals.var_vds > assign34920_e49894)) { 1.0 } else { 0.0 };
        locals.var_guard1150 = assign34920_e49896;

        let (assign34930_e49904, assign34930_e49904_d_n0, assign34930_e49904_d_n2, assign34930_e49904_d_n6, assign34930_e49904_d_n7, assign34930_e49904_d_n10, assign34930_e49904_d_n11, assign34930_e49904_d_n12, assign34930_e49904_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1150 != 0.0)) {
        let assign34930_e49902: f64 = (locals.var_muun / locals.var_mu);
        (assign34930_e49902, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn12 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn12)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn17 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn17)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn12, locals.var_mumoda_dn17,)
    }
};
        locals.var_mumoda = assign34930_e49904;
        locals.var_mumoda_dn0 = assign34930_e49904_d_n0;
        locals.var_mumoda_dn2 = assign34930_e49904_d_n2;
        locals.var_mumoda_dn6 = assign34930_e49904_d_n6;
        locals.var_mumoda_dn7 = assign34930_e49904_d_n7;
        locals.var_mumoda_dn10 = assign34930_e49904_d_n10;
        locals.var_mumoda_dn11 = assign34930_e49904_d_n11;
        locals.var_mumoda_dn12 = assign34930_e49904_d_n12;
        locals.var_mumoda_dn17 = assign34930_e49904_d_n17;

        let (assign34940_e49916, assign34940_e49916_d_n0, assign34940_e49916_d_n2, assign34940_e49916_d_n6, assign34940_e49916_d_n7, assign34940_e49916_d_n10, assign34940_e49916_d_n11, assign34940_e49916_d_n12, assign34940_e49916_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1150 != 0.0)) {
        let assign34940_e49910: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign34940_e49912: f64 = (assign34940_e49910 - locals.var_mumoda);
        let assign34940_e49914: f64 = (assign34940_e49912 / locals.var_vds);
        (assign34940_e49914, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn12) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn12)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn17) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn17)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn12, locals.var_mumodb_dn17,)
    }
};
        locals.var_mumodb = assign34940_e49916;
        locals.var_mumodb_dn0 = assign34940_e49916_d_n0;
        locals.var_mumodb_dn2 = assign34940_e49916_d_n2;
        locals.var_mumodb_dn6 = assign34940_e49916_d_n6;
        locals.var_mumodb_dn7 = assign34940_e49916_d_n7;
        locals.var_mumodb_dn10 = assign34940_e49916_d_n10;
        locals.var_mumodb_dn11 = assign34940_e49916_d_n11;
        locals.var_mumodb_dn12 = assign34940_e49916_d_n12;
        locals.var_mumodb_dn17 = assign34940_e49916_d_n17;

    }

    pub(super) fn stamp_transient_block_122(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign34950_e49938, assign34950_e49938_d_n0, assign34950_e49938_d_n2, assign34950_e49938_d_n6, assign34950_e49938_d_n7, assign34950_e49938_d_n10, assign34950_e49938_d_n11, assign34950_e49938_d_n12, assign34950_e49938_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1150 != 0.0)) {
        let assign34950_e49923: f64 = (0.6666666666666667 * locals.var_mumodb);
        let assign34950_e49927: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
        let assign34950_e49928: f64 = (locals.var_kusai00 + assign34950_e49927);
        let assign34950_e49930: f64 = (assign34950_e49928 + locals.var_kusail);
        let assign34950_e49931: f64 = (assign34950_e49923 * assign34950_e49930);
        let assign34950_e49934: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        let assign34950_e49935: f64 = (assign34950_e49931 / assign34950_e49934);
        let assign34950_e49936: f64 = (locals.var_mumoda + assign34950_e49935);
        (assign34950_e49936, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn12 + ((((((0.6666666666666667 * locals.var_mumodb_dn12) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn12 + ((locals.var_vgvt_dn12 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn17 + ((((((0.6666666666666667 * locals.var_mumodb_dn17) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn17 + ((locals.var_vgvt_dn17 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn17))) + locals.var_kusail_dn17))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17))) / (assign34950_e49934 * assign34950_e49934))),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17,)
    }
};
        locals.var_correct_w1 = assign34950_e49938;
        locals.var_correct_w1_dn0 = assign34950_e49938_d_n0;
        locals.var_correct_w1_dn2 = assign34950_e49938_d_n2;
        locals.var_correct_w1_dn6 = assign34950_e49938_d_n6;
        locals.var_correct_w1_dn7 = assign34950_e49938_d_n7;
        locals.var_correct_w1_dn10 = assign34950_e49938_d_n10;
        locals.var_correct_w1_dn11 = assign34950_e49938_d_n11;
        locals.var_correct_w1_dn12 = assign34950_e49938_d_n12;
        locals.var_correct_w1_dn17 = assign34950_e49938_d_n17;

        let (assign34960_e49947, assign34960_e49947_d_n0, assign34960_e49947_d_n2, assign34960_e49947_d_n6, assign34960_e49947_d_n7, assign34960_e49947_d_n10, assign34960_e49947_d_n11, assign34960_e49947_d_n12, assign34960_e49947_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1150 == 0.0)) {
        let assign34960_e49945: f64 = (locals.var_muun / locals.var_mud_hoso);
        (assign34960_e49945, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17,)
    }
};
        locals.var_correct_w1 = assign34960_e49947;
        locals.var_correct_w1_dn0 = assign34960_e49947_d_n0;
        locals.var_correct_w1_dn2 = assign34960_e49947_d_n2;
        locals.var_correct_w1_dn6 = assign34960_e49947_d_n6;
        locals.var_correct_w1_dn7 = assign34960_e49947_d_n7;
        locals.var_correct_w1_dn10 = assign34960_e49947_d_n10;
        locals.var_correct_w1_dn11 = assign34960_e49947_d_n11;
        locals.var_correct_w1_dn12 = assign34960_e49947_d_n12;
        locals.var_correct_w1_dn17 = assign34960_e49947_d_n17;

        let (assign34970_e49957, assign34970_e49957_d_n0, assign34970_e49957_d_n2, assign34970_e49957_d_n6, assign34970_e49957_d_n7, assign34970_e49957_d_n10, assign34970_e49957_d_n11, assign34970_e49957_d_n12, assign34970_e49957_d_n13, assign34970_e49957_d_n15, assign34970_e49957_d_n16, assign34970_e49957_d_n17, assign34970_e49957_d_n18,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign34970_e49951: f64 = (locals.var_mfactor * locals.var_nign0);
        let assign34970_e49953: f64 = (assign34970_e49951 * locals.var_kusai_ig);
        let assign34970_e49955: f64 = (assign34970_e49953 * locals.var_correct_w1);
        (assign34970_e49955, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn12) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn12)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn12)), (((locals.var_mfactor * locals.var_nign0_dn13) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn15) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn16) * locals.var_kusai_ig) * locals.var_correct_w1), (((((locals.var_mfactor * locals.var_nign0_dn17) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn17)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn17)), (((locals.var_mfactor * locals.var_nign0_dn18) * locals.var_kusai_ig) * locals.var_correct_w1),)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign34970_e49957;
        locals.var_noiigate_dn0 = assign34970_e49957_d_n0;
        locals.var_noiigate_dn2 = assign34970_e49957_d_n2;
        locals.var_noiigate_dn6 = assign34970_e49957_d_n6;
        locals.var_noiigate_dn7 = assign34970_e49957_d_n7;
        locals.var_noiigate_dn10 = assign34970_e49957_d_n10;
        locals.var_noiigate_dn11 = assign34970_e49957_d_n11;
        locals.var_noiigate_dn12 = assign34970_e49957_d_n12;
        locals.var_noiigate_dn13 = assign34970_e49957_d_n13;
        locals.var_noiigate_dn15 = assign34970_e49957_d_n15;
        locals.var_noiigate_dn16 = assign34970_e49957_d_n16;
        locals.var_noiigate_dn17 = assign34970_e49957_d_n17;
        locals.var_noiigate_dn18 = assign34970_e49957_d_n18;

        let (assign34980_e49961, assign34980_e49961_d_n0, assign34980_e49961_d_n2, assign34980_e49961_d_n6, assign34980_e49961_d_n7, assign34980_e49961_d_n10, assign34980_e49961_d_n11, assign34980_e49961_d_n12, assign34980_e49961_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        (locals.var_crl_f, locals.var_crl_f_dn0, locals.var_crl_f_dn2, locals.var_crl_f_dn6, locals.var_crl_f_dn7, locals.var_crl_f_dn10, locals.var_crl_f_dn11, locals.var_crl_f_dn12, locals.var_crl_f_dn17,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn12, locals.var_noicross_dn17,)
    }
};
        locals.var_noicross = assign34980_e49961;
        locals.var_noicross_dn0 = assign34980_e49961_d_n0;
        locals.var_noicross_dn2 = assign34980_e49961_d_n2;
        locals.var_noicross_dn6 = assign34980_e49961_d_n6;
        locals.var_noicross_dn7 = assign34980_e49961_d_n7;
        locals.var_noicross_dn10 = assign34980_e49961_d_n10;
        locals.var_noicross_dn11 = assign34980_e49961_d_n11;
        locals.var_noicross_dn12 = assign34980_e49961_d_n12;
        locals.var_noicross_dn17 = assign34980_e49961_d_n17;

        let (assign34990_e49975, assign34990_e49975_d_n0, assign34990_e49975_d_n2, assign34990_e49975_d_n6, assign34990_e49975_d_n7, assign34990_e49975_d_n10, assign34990_e49975_d_n11, assign34990_e49975_d_n12, assign34990_e49975_d_n13, assign34990_e49975_d_n15, assign34990_e49975_d_n16, assign34990_e49975_d_n17, assign34990_e49975_d_n18,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign34990_e49964: f64 = (-locals.var_t1__blk1144);
        let (assign34990_e49973, assign34990_e49973_d_n0, assign34990_e49973_d_n2, assign34990_e49973_d_n6, assign34990_e49973_d_n7, assign34990_e49973_d_n10, assign34990_e49973_d_n11, assign34990_e49973_d_n12, assign34990_e49973_d_n13, assign34990_e49973_d_n15, assign34990_e49973_d_n16, assign34990_e49973_d_n17, assign34990_e49973_d_n18,) = {
            if ((assign34990_e49964 > locals.var_t0__blk1143) && (locals.var_noiigate > 0.0)) {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign34990_e49973, assign34990_e49973_d_n0, assign34990_e49973_d_n2, assign34990_e49973_d_n6, assign34990_e49973_d_n7, assign34990_e49973_d_n10, assign34990_e49973_d_n11, assign34990_e49973_d_n12, assign34990_e49973_d_n13, assign34990_e49973_d_n15, assign34990_e49973_d_n16, assign34990_e49973_d_n17, assign34990_e49973_d_n18,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign34990_e49975;
        locals.var_noiigate_dn0 = assign34990_e49975_d_n0;
        locals.var_noiigate_dn2 = assign34990_e49975_d_n2;
        locals.var_noiigate_dn6 = assign34990_e49975_d_n6;
        locals.var_noiigate_dn7 = assign34990_e49975_d_n7;
        locals.var_noiigate_dn10 = assign34990_e49975_d_n10;
        locals.var_noiigate_dn11 = assign34990_e49975_d_n11;
        locals.var_noiigate_dn12 = assign34990_e49975_d_n12;
        locals.var_noiigate_dn13 = assign34990_e49975_d_n13;
        locals.var_noiigate_dn15 = assign34990_e49975_d_n15;
        locals.var_noiigate_dn16 = assign34990_e49975_d_n16;
        locals.var_noiigate_dn17 = assign34990_e49975_d_n17;
        locals.var_noiigate_dn18 = assign34990_e49975_d_n18;

        let (assign35000_e49985, assign35000_e49985_d_n0, assign35000_e49985_d_n2, assign35000_e49985_d_n6, assign35000_e49985_d_n7, assign35000_e49985_d_n10, assign35000_e49985_d_n11, assign35000_e49985_d_n12, assign35000_e49985_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35000_e49978: f64 = (-locals.var_t1__blk1144);
        let (assign35000_e49983, assign35000_e49983_d_n0, assign35000_e49983_d_n2, assign35000_e49983_d_n6, assign35000_e49983_d_n7, assign35000_e49983_d_n10, assign35000_e49983_d_n11, assign35000_e49983_d_n12, assign35000_e49983_d_n17,) = {
            if (assign35000_e49978 > locals.var_t0__blk1143) {
                (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn12, locals.var_noicross_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign35000_e49983, assign35000_e49983_d_n0, assign35000_e49983_d_n2, assign35000_e49983_d_n6, assign35000_e49983_d_n7, assign35000_e49983_d_n10, assign35000_e49983_d_n11, assign35000_e49983_d_n12, assign35000_e49983_d_n17,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn12, locals.var_noicross_dn17,)
    }
};
        locals.var_noicross = assign35000_e49985;
        locals.var_noicross_dn0 = assign35000_e49985_d_n0;
        locals.var_noicross_dn2 = assign35000_e49985_d_n2;
        locals.var_noicross_dn6 = assign35000_e49985_d_n6;
        locals.var_noicross_dn7 = assign35000_e49985_d_n7;
        locals.var_noicross_dn10 = assign35000_e49985_d_n10;
        locals.var_noicross_dn11 = assign35000_e49985_d_n11;
        locals.var_noicross_dn12 = assign35000_e49985_d_n12;
        locals.var_noicross_dn17 = assign35000_e49985_d_n17;

        let (assign35010_e49990, assign35010_e49990_d_n0, assign35010_e49990_d_n2, assign35010_e49990_d_n6, assign35010_e49990_d_n7, assign35010_e49990_d_n10, assign35010_e49990_d_n11, assign35010_e49990_d_n12, assign35010_e49990_d_n13, assign35010_e49990_d_n15, assign35010_e49990_d_n16, assign35010_e49990_d_n17, assign35010_e49990_d_n18,) = {
    if (locals.var_guard1149 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign35010_e49990;
        locals.var_noiigate_dn0 = assign35010_e49990_d_n0;
        locals.var_noiigate_dn2 = assign35010_e49990_d_n2;
        locals.var_noiigate_dn6 = assign35010_e49990_d_n6;
        locals.var_noiigate_dn7 = assign35010_e49990_d_n7;
        locals.var_noiigate_dn10 = assign35010_e49990_d_n10;
        locals.var_noiigate_dn11 = assign35010_e49990_d_n11;
        locals.var_noiigate_dn12 = assign35010_e49990_d_n12;
        locals.var_noiigate_dn13 = assign35010_e49990_d_n13;
        locals.var_noiigate_dn15 = assign35010_e49990_d_n15;
        locals.var_noiigate_dn16 = assign35010_e49990_d_n16;
        locals.var_noiigate_dn17 = assign35010_e49990_d_n17;
        locals.var_noiigate_dn18 = assign35010_e49990_d_n18;

        let (assign35020_e49995, assign35020_e49995_d_n0, assign35020_e49995_d_n2, assign35020_e49995_d_n6, assign35020_e49995_d_n7, assign35020_e49995_d_n10, assign35020_e49995_d_n11, assign35020_e49995_d_n12, assign35020_e49995_d_n17,) = {
    if (locals.var_guard1149 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn12, locals.var_noicross_dn17,)
    }
};
        locals.var_noicross = assign35020_e49995;
        locals.var_noicross_dn0 = assign35020_e49995_d_n0;
        locals.var_noicross_dn2 = assign35020_e49995_d_n2;
        locals.var_noicross_dn6 = assign35020_e49995_d_n6;
        locals.var_noicross_dn7 = assign35020_e49995_d_n7;
        locals.var_noicross_dn10 = assign35020_e49995_d_n10;
        locals.var_noicross_dn11 = assign35020_e49995_d_n11;
        locals.var_noicross_dn12 = assign35020_e49995_d_n12;
        locals.var_noicross_dn17 = assign35020_e49995_d_n17;

        locals.var_rdde = 0.0;
        locals.var_rdde_dn0 = 0.0;
        locals.var_rdde_dn2 = 0.0;
        locals.var_rdde_dn6 = 0.0;
        locals.var_rdde_dn7 = 0.0;
        locals.var_rdde_dn10 = 0.0;
        locals.var_rdde_dn11 = 0.0;
        locals.var_rdde_dn12 = 0.0;
        locals.var_rdde_dn17 = 0.0;

        locals.var_rsde = 0.0;
        locals.var_rsde_dn0 = 0.0;
        locals.var_rsde_dn2 = 0.0;
        locals.var_rsde_dn6 = 0.0;
        locals.var_rsde_dn7 = 0.0;
        locals.var_rsde_dn10 = 0.0;
        locals.var_rsde_dn11 = 0.0;
        locals.var_rsde_dn12 = 0.0;
        locals.var_rsde_dn17 = 0.0;

        let assign35070_e50002: f64 = if p.p259 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1151 = assign35070_e50002;

        let (assign35080_e50006,) = {
    if (locals.var_guard1151 != 0.0) {
        (1.0,)
    } else {
        (locals.var_rdmod,)
    }
};
        locals.var_rdmod = assign35080_e50006;

        let assign35090_e50009: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1171 = assign35090_e50009;

        let (assign35100_e50017,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 != 0.0)) {
        let assign35100_e50015: f64 = (p.p264 / 1e-6);
        (assign35100_e50015,)
    } else {
        (locals.var_nover,)
    }
};
        locals.var_nover = assign35100_e50017;

        let (assign35110_e50023,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 != 0.0)) {
        (p.p266,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35110_e50023;

        let (assign35120_e50029,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 != 0.0)) {
        (p.p268,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35120_e50029;

        let (assign35130_e50035, assign35130_e50035_d_n10,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35130_e50035;
        locals.var_rrdrbb_dn10 = assign35130_e50035_d_n10;

        let (assign35140_e50048,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 != 0.0)) {
        let (assign35140_e50046,) = {
            if (p.p263 > 0.0) {
                let assign35140_e50044: f64 = (p.p263 * p.p255);
                (assign35140_e50044,)
            } else {
                (0.0,)
            }
        };
        (assign35140_e50046,)
    } else {
        (locals.var_rsd0,)
    }
};
        locals.var_rsd0 = assign35140_e50048;

        let (assign35150_e50054,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 != 0.0)) {
        (p.p258,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign35150_e50054;

        let (assign35160_e50062, assign35160_e50062_d_n0, assign35160_e50062_d_n2, assign35160_e50062_d_n6, assign35160_e50062_d_n7,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 != 0.0)) {
        let assign35160_e50060: f64 = (p.p50 * (nv7 - nv2));
        (assign35160_e50060, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7,)
    }
};
        locals.var_vrdr = assign35160_e50062;
        locals.var_vrdr_dn0 = assign35160_e50062_d_n0;
        locals.var_vrdr_dn2 = assign35160_e50062_d_n2;
        locals.var_vrdr_dn6 = assign35160_e50062_d_n6;
        locals.var_vrdr_dn7 = assign35160_e50062_d_n7;

        let (assign35170_e50071,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 == 0.0)) {
        let assign35170_e50069: f64 = (p.p59 / 1e-6);
        (assign35170_e50069,)
    } else {
        (locals.var_nover,)
    }
};
        locals.var_nover = assign35170_e50071;

        let (assign35180_e50078,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 == 0.0)) {
        (p.p265,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35180_e50078;

        let (assign35190_e50085,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 == 0.0)) {
        (p.p267,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35190_e50085;

        let (assign35200_e50092, assign35200_e50092_d_n10,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35200_e50092;
        locals.var_rrdrbb_dn10 = assign35200_e50092_d_n10;

        let (assign35210_e50106,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 == 0.0)) {
        let (assign35210_e50104,) = {
            if (p.p263 > 0.0) {
                let assign35210_e50102: f64 = (p.p263 * p.p256);
                (assign35210_e50102,)
            } else {
                (0.0,)
            }
        };
        (assign35210_e50104,)
    } else {
        (locals.var_rsd0,)
    }
};
        locals.var_rsd0 = assign35210_e50106;

        let (assign35220_e50113,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 == 0.0)) {
        (p.p257,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign35220_e50113;

        let (assign35230_e50122, assign35230_e50122_d_n0, assign35230_e50122_d_n2, assign35230_e50122_d_n6, assign35230_e50122_d_n7,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 == 0.0)) {
        let assign35230_e50120: f64 = (p.p50 * (nv0 - nv6));
        (assign35230_e50120, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7,)
    }
};
        locals.var_vrdr = assign35230_e50122;
        locals.var_vrdr_dn0 = assign35230_e50122_d_n0;
        locals.var_vrdr_dn2 = assign35230_e50122_d_n2;
        locals.var_vrdr_dn6 = assign35230_e50122_d_n6;
        locals.var_vrdr_dn7 = assign35230_e50122_d_n7;

        let (assign35240_e50133,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35240_e50126: f64 = (p.p271 * p.p271);
        let assign35240_e50129: f64 = (p.p56 * p.p56);
        let assign35240_e50130: f64 = (assign35240_e50126 + assign35240_e50129);
        let assign35240_e50131: f64 = (assign35240_e50130).sqrt();
        (assign35240_e50131,)
    } else {
        (locals.var_xov,)
    }
};
        locals.var_xov = assign35240_e50133;

        let (assign35250_e50139,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35250_e50137: f64 = (locals.var_weff * p.p9);
        (assign35250_e50137,)
    } else {
        (locals.var_weff_nf_1,)
    }
};
        locals.var_weff_nf_1 = assign35250_e50139;

        let (assign35260_e50145,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35260_e50143: f64 = (locals.var_mks_rdrmue / 10000.0);
        (assign35260_e50143,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35260_e50145;

        let (assign35270_e50151,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35270_e50149: f64 = (locals.var_mks_rdrvmax / 100.0);
        (assign35270_e50149,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35270_e50151;

        let (assign35280_e50157, assign35280_e50157_d_n10,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35280_e50155: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign35280_e50155, (locals.var_ttemp_dn10 / locals.var_uc_tnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn10,)
    }
};
        locals.var_tratio = assign35280_e50157;
        locals.var_tratio_dn10 = assign35280_e50157_d_n10;

        let (assign35290_e50163, assign35290_e50163_d_n0, assign35290_e50163_d_n2, assign35290_e50163_d_n6, assign35290_e50163_d_n7, assign35290_e50163_d_n10, assign35290_e50163_d_n11, assign35290_e50163_d_n12, assign35290_e50163_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35290_e50161: f64 = (locals.var_tratio).powf(p.p269);
        (assign35290_e50161, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio).powf(p.p269 - 1.0) * locals.var_tratio_dn10)) } } else { (assign35290_e50161 * (p.p269 * (locals.var_tratio_dn10 / locals.var_tratio))) }, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35290_e50163;
        locals.var_t1_dn0 = assign35290_e50163_d_n0;
        locals.var_t1_dn2 = assign35290_e50163_d_n2;
        locals.var_t1_dn6 = assign35290_e50163_d_n6;
        locals.var_t1_dn7 = assign35290_e50163_d_n7;
        locals.var_t1_dn10 = assign35290_e50163_d_n10;
        locals.var_t1_dn11 = assign35290_e50163_d_n11;
        locals.var_t1_dn12 = assign35290_e50163_d_n12;
        locals.var_t1_dn17 = assign35290_e50163_d_n17;

        let (assign35300_e50169, assign35300_e50169_d_n0, assign35300_e50169_d_n2, assign35300_e50169_d_n6, assign35300_e50169_d_n7, assign35300_e50169_d_n10, assign35300_e50169_d_n11, assign35300_e50169_d_n12, assign35300_e50169_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35300_e50167: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign35300_e50167, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17,)
    }
};
        locals.var_mu0 = assign35300_e50169;
        locals.var_mu0_dn0 = assign35300_e50169_d_n0;
        locals.var_mu0_dn2 = assign35300_e50169_d_n2;
        locals.var_mu0_dn6 = assign35300_e50169_d_n6;
        locals.var_mu0_dn7 = assign35300_e50169_d_n7;
        locals.var_mu0_dn10 = assign35300_e50169_d_n10;
        locals.var_mu0_dn11 = assign35300_e50169_d_n11;
        locals.var_mu0_dn12 = assign35300_e50169_d_n12;
        locals.var_mu0_dn17 = assign35300_e50169_d_n17;

        let (assign35310_e50189, assign35310_e50189_d_n0, assign35310_e50189_d_n2, assign35310_e50189_d_n6, assign35310_e50189_d_n7, assign35310_e50189_d_n10, assign35310_e50189_d_n11, assign35310_e50189_d_n12, assign35310_e50189_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35310_e50174: f64 = (0.4 * locals.var_tratio);
        let assign35310_e50175: f64 = (1.8 + assign35310_e50174);
        let assign35310_e50178: f64 = (0.1 * locals.var_tratio);
        let assign35310_e50180: f64 = (assign35310_e50178 * locals.var_tratio);
        let assign35310_e50181: f64 = (assign35310_e50175 + assign35310_e50180);
        let assign35310_e50185: f64 = (1.0 - locals.var_tratio);
        let assign35310_e50186: f64 = (p.p270 * assign35310_e50185);
        let assign35310_e50187: f64 = (assign35310_e50181 - assign35310_e50186);
        (assign35310_e50187, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign35310_e50178 * locals.var_tratio_dn10))) - (p.p270 * (-locals.var_tratio_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign35310_e50189;
        locals.var_t0_dn0 = assign35310_e50189_d_n0;
        locals.var_t0_dn2 = assign35310_e50189_d_n2;
        locals.var_t0_dn6 = assign35310_e50189_d_n6;
        locals.var_t0_dn7 = assign35310_e50189_d_n7;
        locals.var_t0_dn10 = assign35310_e50189_d_n10;
        locals.var_t0_dn11 = assign35310_e50189_d_n11;
        locals.var_t0_dn12 = assign35310_e50189_d_n12;
        locals.var_t0_dn17 = assign35310_e50189_d_n17;

        let (assign35320_e50195, assign35320_e50195_d_n0, assign35320_e50195_d_n2, assign35320_e50195_d_n6, assign35320_e50195_d_n7, assign35320_e50195_d_n10, assign35320_e50195_d_n11, assign35320_e50195_d_n12, assign35320_e50195_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35320_e50193: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign35320_e50193, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_vmaxe__blk1164, locals.var_vmaxe__blk1164_dn0, locals.var_vmaxe__blk1164_dn2, locals.var_vmaxe__blk1164_dn6, locals.var_vmaxe__blk1164_dn7, locals.var_vmaxe__blk1164_dn10, locals.var_vmaxe__blk1164_dn11, locals.var_vmaxe__blk1164_dn12, locals.var_vmaxe__blk1164_dn17,)
    }
};
        locals.var_vmaxe__blk1164 = assign35320_e50195;
        locals.var_vmaxe__blk1164_dn0 = assign35320_e50195_d_n0;
        locals.var_vmaxe__blk1164_dn2 = assign35320_e50195_d_n2;
        locals.var_vmaxe__blk1164_dn6 = assign35320_e50195_d_n6;
        locals.var_vmaxe__blk1164_dn7 = assign35320_e50195_d_n7;
        locals.var_vmaxe__blk1164_dn10 = assign35320_e50195_d_n10;
        locals.var_vmaxe__blk1164_dn11 = assign35320_e50195_d_n11;
        locals.var_vmaxe__blk1164_dn12 = assign35320_e50195_d_n12;
        locals.var_vmaxe__blk1164_dn17 = assign35320_e50195_d_n17;

    }

    pub(super) fn stamp_transient_block_123(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign35330_e50205, assign35330_e50205_d_n10,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35330_e50201: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        let assign35330_e50202: f64 = (p.p274 * assign35330_e50201);
        let assign35330_e50203: f64 = (locals.var_rrdrbb + assign35330_e50202);
        (assign35330_e50203, (locals.var_rrdrbb_dn10 + (p.p274 * locals.var_ttemp_dn10)),)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35330_e50205;
        locals.var_rrdrbb_dn10 = assign35330_e50205_d_n10;

        let (assign35340_e50215,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35340_e50211: f64 = (locals.var_lgle).powf(p.p280);
        let assign35340_e50212: f64 = (p.p279 / assign35340_e50211);
        let assign35340_e50213: f64 = (1.0 + assign35340_e50212);
        (assign35340_e50213,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign35340_e50215;

        let (assign35350_e50225,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35350_e50221: f64 = (locals.var_lgle).powf(p.p278);
        let assign35350_e50222: f64 = (p.p277 / assign35350_e50221);
        let assign35350_e50223: f64 = (1.0 + assign35350_e50222);
        (assign35350_e50223,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign35350_e50225;

        let (assign35360_e50235,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35360_e50231: f64 = (locals.var_wg).powf(p.p276);
        let assign35360_e50232: f64 = (p.p275 / assign35360_e50231);
        let assign35360_e50233: f64 = (1.0 + assign35360_e50232);
        (assign35360_e50233,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign35360_e50235;

        let (assign35370_e50241, assign35370_e50241_d_n0, assign35370_e50241_d_n2, assign35370_e50241_d_n6, assign35370_e50241_d_n7, assign35370_e50241_d_n10, assign35370_e50241_d_n11, assign35370_e50241_d_n12, assign35370_e50241_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35370_e50239: f64 = (locals.var_mu0 * locals.var_rdrmuele);
        (assign35370_e50239, (locals.var_mu0_dn0 * locals.var_rdrmuele), (locals.var_mu0_dn2 * locals.var_rdrmuele), (locals.var_mu0_dn6 * locals.var_rdrmuele), (locals.var_mu0_dn7 * locals.var_rdrmuele), (locals.var_mu0_dn10 * locals.var_rdrmuele), (locals.var_mu0_dn11 * locals.var_rdrmuele), (locals.var_mu0_dn12 * locals.var_rdrmuele), (locals.var_mu0_dn17 * locals.var_rdrmuele),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17,)
    }
};
        locals.var_mu0 = assign35370_e50241;
        locals.var_mu0_dn0 = assign35370_e50241_d_n0;
        locals.var_mu0_dn2 = assign35370_e50241_d_n2;
        locals.var_mu0_dn6 = assign35370_e50241_d_n6;
        locals.var_mu0_dn7 = assign35370_e50241_d_n7;
        locals.var_mu0_dn10 = assign35370_e50241_d_n10;
        locals.var_mu0_dn11 = assign35370_e50241_d_n11;
        locals.var_mu0_dn12 = assign35370_e50241_d_n12;
        locals.var_mu0_dn17 = assign35370_e50241_d_n17;

        let (assign35380_e50251, assign35380_e50251_d_n0, assign35380_e50251_d_n2, assign35380_e50251_d_n6, assign35380_e50251_d_n7, assign35380_e50251_d_n10, assign35380_e50251_d_n11, assign35380_e50251_d_n12, assign35380_e50251_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35380_e50245: f64 = (locals.var_vmaxe__blk1164 * locals.var_rdrvmaxwe);
        let assign35380_e50247: f64 = (assign35380_e50245 * locals.var_rdrvmaxle);
        let assign35380_e50249: f64 = (assign35380_e50247 + 1e-50);
        (assign35380_e50249, ((locals.var_vmaxe__blk1164_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn12 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn17 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe__blk1164, locals.var_vmaxe__blk1164_dn0, locals.var_vmaxe__blk1164_dn2, locals.var_vmaxe__blk1164_dn6, locals.var_vmaxe__blk1164_dn7, locals.var_vmaxe__blk1164_dn10, locals.var_vmaxe__blk1164_dn11, locals.var_vmaxe__blk1164_dn12, locals.var_vmaxe__blk1164_dn17,)
    }
};
        locals.var_vmaxe__blk1164 = assign35380_e50251;
        locals.var_vmaxe__blk1164_dn0 = assign35380_e50251_d_n0;
        locals.var_vmaxe__blk1164_dn2 = assign35380_e50251_d_n2;
        locals.var_vmaxe__blk1164_dn6 = assign35380_e50251_d_n6;
        locals.var_vmaxe__blk1164_dn7 = assign35380_e50251_d_n7;
        locals.var_vmaxe__blk1164_dn10 = assign35380_e50251_d_n10;
        locals.var_vmaxe__blk1164_dn11 = assign35380_e50251_d_n11;
        locals.var_vmaxe__blk1164_dn12 = assign35380_e50251_d_n12;
        locals.var_vmaxe__blk1164_dn17 = assign35380_e50251_d_n17;

        let (assign35390_e50257, assign35390_e50257_d_n0, assign35390_e50257_d_n2, assign35390_e50257_d_n6, assign35390_e50257_d_n7,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35390_e50255: f64 = (locals.var_vrdr / locals.var_ldrifte);
        (assign35390_e50255, (locals.var_vrdr_dn0 / locals.var_ldrifte), (locals.var_vrdr_dn2 / locals.var_ldrifte), (locals.var_vrdr_dn6 / locals.var_ldrifte), (locals.var_vrdr_dn7 / locals.var_ldrifte),)
    } else {
        (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn6, locals.var_edri_dn7,)
    }
};
        locals.var_edri = assign35390_e50257;
        locals.var_edri_dn0 = assign35390_e50257_d_n0;
        locals.var_edri_dn2 = assign35390_e50257_d_n2;
        locals.var_edri_dn6 = assign35390_e50257_d_n6;
        locals.var_edri_dn7 = assign35390_e50257_d_n7;

        let (assign35400_e50263, assign35400_e50263_d_n0, assign35400_e50263_d_n2, assign35400_e50263_d_n6, assign35400_e50263_d_n7, assign35400_e50263_d_n10, assign35400_e50263_d_n11, assign35400_e50263_d_n12, assign35400_e50263_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35400_e50261: f64 = (locals.var_mu0 * locals.var_edri);
        (assign35400_e50261, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), (locals.var_mu0_dn10 * locals.var_edri), (locals.var_mu0_dn11 * locals.var_edri), (locals.var_mu0_dn12 * locals.var_edri), (locals.var_mu0_dn17 * locals.var_edri),)
    } else {
        (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn12, locals.var_vdri_dn17,)
    }
};
        locals.var_vdri = assign35400_e50263;
        locals.var_vdri_dn0 = assign35400_e50263_d_n0;
        locals.var_vdri_dn2 = assign35400_e50263_d_n2;
        locals.var_vdri_dn6 = assign35400_e50263_d_n6;
        locals.var_vdri_dn7 = assign35400_e50263_d_n7;
        locals.var_vdri_dn10 = assign35400_e50263_d_n10;
        locals.var_vdri_dn11 = assign35400_e50263_d_n11;
        locals.var_vdri_dn12 = assign35400_e50263_d_n12;
        locals.var_vdri_dn17 = assign35400_e50263_d_n17;

        let assign35410_e50266: f64 = if locals.var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign35410_e50266;

        let (assign35420_e50274, assign35420_e50274_d_n0, assign35420_e50274_d_n2, assign35420_e50274_d_n6, assign35420_e50274_d_n7, assign35420_e50274_d_n10, assign35420_e50274_d_n11, assign35420_e50274_d_n12, assign35420_e50274_d_n17,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1172 != 0.0)) {
        let assign35420_e50272: f64 = (locals.var_vdri / locals.var_vmaxe__blk1164);
        (assign35420_e50272, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn0)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn2)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn6)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn7)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn10)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn11)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn12 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn12)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn17 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn17)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35420_e50274;
        locals.var_t1_dn0 = assign35420_e50274_d_n0;
        locals.var_t1_dn2 = assign35420_e50274_d_n2;
        locals.var_t1_dn6 = assign35420_e50274_d_n6;
        locals.var_t1_dn7 = assign35420_e50274_d_n7;
        locals.var_t1_dn10 = assign35420_e50274_d_n10;
        locals.var_t1_dn11 = assign35420_e50274_d_n11;
        locals.var_t1_dn12 = assign35420_e50274_d_n12;
        locals.var_t1_dn17 = assign35420_e50274_d_n17;

        let (assign35430_e50284, assign35430_e50284_d_n0, assign35430_e50284_d_n2, assign35430_e50284_d_n6, assign35430_e50284_d_n7, assign35430_e50284_d_n10, assign35430_e50284_d_n11, assign35430_e50284_d_n12, assign35430_e50284_d_n17,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1172 == 0.0)) {
        let assign35430_e50280: f64 = (-locals.var_vdri);
        let assign35430_e50282: f64 = (assign35430_e50280 / locals.var_vmaxe__blk1164);
        (assign35430_e50282, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn0)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn2)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn6)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn7)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn10)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn11)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn12) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn12)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn17) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn17)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35430_e50284;
        locals.var_t1_dn0 = assign35430_e50284_d_n0;
        locals.var_t1_dn2 = assign35430_e50284_d_n2;
        locals.var_t1_dn6 = assign35430_e50284_d_n6;
        locals.var_t1_dn7 = assign35430_e50284_d_n7;
        locals.var_t1_dn10 = assign35430_e50284_d_n10;
        locals.var_t1_dn11 = assign35430_e50284_d_n11;
        locals.var_t1_dn12 = assign35430_e50284_d_n12;
        locals.var_t1_dn17 = assign35430_e50284_d_n17;

        let assign35440_e50288: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50289: f64 = (1.0 - assign35440_e50288);
        let assign35440_e50296: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50297: f64 = (1.0 + assign35440_e50296);
        let assign35440_e50299: f64 = if ((assign35440_e50289 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35440_e50297)) { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign35440_e50299;

        let (assign35450_e50305, assign35450_e50305_d_n0, assign35450_e50305_d_n2, assign35450_e50305_d_n6, assign35450_e50305_d_n7, assign35450_e50305_d_n10, assign35450_e50305_d_n11, assign35450_e50305_d_n12, assign35450_e50305_d_n17,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1173 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35450_e50305;
        locals.var_t3_dn0 = assign35450_e50305_d_n0;
        locals.var_t3_dn2 = assign35450_e50305_d_n2;
        locals.var_t3_dn6 = assign35450_e50305_d_n6;
        locals.var_t3_dn7 = assign35450_e50305_d_n7;
        locals.var_t3_dn10 = assign35450_e50305_d_n10;
        locals.var_t3_dn11 = assign35450_e50305_d_n11;
        locals.var_t3_dn12 = assign35450_e50305_d_n12;
        locals.var_t3_dn17 = assign35450_e50305_d_n17;

        let assign35460_e50309: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50310: f64 = (2.0 - assign35460_e50309);
        let assign35460_e50317: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50318: f64 = (2.0 + assign35460_e50317);
        let assign35460_e50320: f64 = if ((assign35460_e50310 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35460_e50318)) { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign35460_e50320;

        let (assign35470_e50329, assign35470_e50329_d_n0, assign35470_e50329_d_n2, assign35470_e50329_d_n6, assign35470_e50329_d_n7, assign35470_e50329_d_n10, assign35470_e50329_d_n11, assign35470_e50329_d_n12, assign35470_e50329_d_n17,) = {
    if (((locals.var_guard1151 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35470_e50329;
        locals.var_t3_dn0 = assign35470_e50329_d_n0;
        locals.var_t3_dn2 = assign35470_e50329_d_n2;
        locals.var_t3_dn6 = assign35470_e50329_d_n6;
        locals.var_t3_dn7 = assign35470_e50329_d_n7;
        locals.var_t3_dn10 = assign35470_e50329_d_n10;
        locals.var_t3_dn11 = assign35470_e50329_d_n11;
        locals.var_t3_dn12 = assign35470_e50329_d_n12;
        locals.var_t3_dn17 = assign35470_e50329_d_n17;

        let (assign35480_e50343, assign35480_e50343_d_n0, assign35480_e50343_d_n2, assign35480_e50343_d_n6, assign35480_e50343_d_n7, assign35480_e50343_d_n10, assign35480_e50343_d_n11, assign35480_e50343_d_n12, assign35480_e50343_d_n17,) = {
    if (((locals.var_guard1151 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 == 0.0)) {
        let assign35480_e50340: f64 = (locals.var_rrdrbb - 1.0);
        let assign35480_e50341: f64 = (locals.var_t1).powf(assign35480_e50340);
        (assign35480_e50341, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn0)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn2)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn6)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn7)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb_dn10 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn10)) } } else { (assign35480_e50341 * ((locals.var_rrdrbb_dn10 * (locals.var_t1).ln()) + (assign35480_e50340 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn11)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn12)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn17)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn17 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35480_e50343;
        locals.var_t3_dn0 = assign35480_e50343_d_n0;
        locals.var_t3_dn2 = assign35480_e50343_d_n2;
        locals.var_t3_dn6 = assign35480_e50343_d_n6;
        locals.var_t3_dn7 = assign35480_e50343_d_n7;
        locals.var_t3_dn10 = assign35480_e50343_d_n10;
        locals.var_t3_dn11 = assign35480_e50343_d_n11;
        locals.var_t3_dn12 = assign35480_e50343_d_n12;
        locals.var_t3_dn17 = assign35480_e50343_d_n17;

        let (assign35490_e50349, assign35490_e50349_d_n0, assign35490_e50349_d_n2, assign35490_e50349_d_n6, assign35490_e50349_d_n7, assign35490_e50349_d_n10, assign35490_e50349_d_n11, assign35490_e50349_d_n12, assign35490_e50349_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35490_e50347: f64 = (locals.var_t1 * locals.var_t3);
        (assign35490_e50347, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign35490_e50349;
        locals.var_t2_dn0 = assign35490_e50349_d_n0;
        locals.var_t2_dn2 = assign35490_e50349_d_n2;
        locals.var_t2_dn6 = assign35490_e50349_d_n6;
        locals.var_t2_dn7 = assign35490_e50349_d_n7;
        locals.var_t2_dn10 = assign35490_e50349_d_n10;
        locals.var_t2_dn11 = assign35490_e50349_d_n11;
        locals.var_t2_dn12 = assign35490_e50349_d_n12;
        locals.var_t2_dn17 = assign35490_e50349_d_n17;

        let (assign35500_e50355, assign35500_e50355_d_n0, assign35500_e50355_d_n2, assign35500_e50355_d_n6, assign35500_e50355_d_n7, assign35500_e50355_d_n10, assign35500_e50355_d_n11, assign35500_e50355_d_n12, assign35500_e50355_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35500_e50353: f64 = (1.0 + locals.var_t2);
        (assign35500_e50353, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign35500_e50355;
        locals.var_t4_dn0 = assign35500_e50355_d_n0;
        locals.var_t4_dn2 = assign35500_e50355_d_n2;
        locals.var_t4_dn6 = assign35500_e50355_d_n6;
        locals.var_t4_dn7 = assign35500_e50355_d_n7;
        locals.var_t4_dn10 = assign35500_e50355_d_n10;
        locals.var_t4_dn11 = assign35500_e50355_d_n11;
        locals.var_t4_dn12 = assign35500_e50355_d_n12;
        locals.var_t4_dn17 = assign35500_e50355_d_n17;

        let assign35510_e50359: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50360: f64 = (1.0 - assign35510_e50359);
        let assign35510_e50367: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50368: f64 = (1.0 + assign35510_e50367);
        let assign35510_e50370: f64 = if ((assign35510_e50360 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35510_e50368)) { 1.0 } else { 0.0 };
        locals.var_guard1175 = assign35510_e50370;

        let (assign35520_e50378, assign35520_e50378_d_n0, assign35520_e50378_d_n2, assign35520_e50378_d_n6, assign35520_e50378_d_n7, assign35520_e50378_d_n10, assign35520_e50378_d_n11, assign35520_e50378_d_n12, assign35520_e50378_d_n17,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1175 != 0.0)) {
        let assign35520_e50376: f64 = (1.0 / locals.var_t4);
        (assign35520_e50376, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign35520_e50378;
        locals.var_t5_dn0 = assign35520_e50378_d_n0;
        locals.var_t5_dn2 = assign35520_e50378_d_n2;
        locals.var_t5_dn6 = assign35520_e50378_d_n6;
        locals.var_t5_dn7 = assign35520_e50378_d_n7;
        locals.var_t5_dn10 = assign35520_e50378_d_n10;
        locals.var_t5_dn11 = assign35520_e50378_d_n11;
        locals.var_t5_dn12 = assign35520_e50378_d_n12;
        locals.var_t5_dn17 = assign35520_e50378_d_n17;

        let assign35530_e50382: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50383: f64 = (2.0 - assign35530_e50382);
        let assign35530_e50390: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50391: f64 = (2.0 + assign35530_e50390);
        let assign35530_e50393: f64 = if ((assign35530_e50383 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35530_e50391)) { 1.0 } else { 0.0 };
        locals.var_guard1176 = assign35530_e50393;

        let (assign35540_e50405, assign35540_e50405_d_n0, assign35540_e50405_d_n2, assign35540_e50405_d_n6, assign35540_e50405_d_n7, assign35540_e50405_d_n10, assign35540_e50405_d_n11, assign35540_e50405_d_n12, assign35540_e50405_d_n17,) = {
    if (((locals.var_guard1151 != 0.0) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 != 0.0)) {
        let assign35540_e50402: f64 = (locals.var_t4).sqrt();
        let assign35540_e50403: f64 = (1.0 / assign35540_e50402);
        (assign35540_e50403, (-((locals.var_t4_dn0 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn2 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn6 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn7 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn10 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn11 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn12 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn17 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign35540_e50405;
        locals.var_t5_dn0 = assign35540_e50405_d_n0;
        locals.var_t5_dn2 = assign35540_e50405_d_n2;
        locals.var_t5_dn6 = assign35540_e50405_d_n6;
        locals.var_t5_dn7 = assign35540_e50405_d_n7;
        locals.var_t5_dn10 = assign35540_e50405_d_n10;
        locals.var_t5_dn11 = assign35540_e50405_d_n11;
        locals.var_t5_dn12 = assign35540_e50405_d_n12;
        locals.var_t5_dn17 = assign35540_e50405_d_n17;

        let (assign35550_e50422, assign35550_e50422_d_n0, assign35550_e50422_d_n2, assign35550_e50422_d_n6, assign35550_e50422_d_n7, assign35550_e50422_d_n10, assign35550_e50422_d_n11, assign35550_e50422_d_n12, assign35550_e50422_d_n17,) = {
    if (((locals.var_guard1151 != 0.0) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 == 0.0)) {
        let assign35550_e50415: f64 = (-1.0);
        let assign35550_e50417: f64 = (assign35550_e50415 / locals.var_rrdrbb);
        let assign35550_e50419: f64 = (assign35550_e50417 - 1.0);
        let assign35550_e50420: f64 = (locals.var_t4).powf(assign35550_e50419);
        (assign35550_e50420, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn0)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn2)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn6)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn7)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign35550_e50415 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn10)) } } else { (assign35550_e50420 * (((-((assign35550_e50415 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) * (locals.var_t4).ln()) + (assign35550_e50419 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn11)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn12)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn17)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn17 / locals.var_t4))) },)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign35550_e50422;
        locals.var_t6_dn0 = assign35550_e50422_d_n0;
        locals.var_t6_dn2 = assign35550_e50422_d_n2;
        locals.var_t6_dn6 = assign35550_e50422_d_n6;
        locals.var_t6_dn7 = assign35550_e50422_d_n7;
        locals.var_t6_dn10 = assign35550_e50422_d_n10;
        locals.var_t6_dn11 = assign35550_e50422_d_n11;
        locals.var_t6_dn12 = assign35550_e50422_d_n12;
        locals.var_t6_dn17 = assign35550_e50422_d_n17;

        let (assign35560_e50434, assign35560_e50434_d_n0, assign35560_e50434_d_n2, assign35560_e50434_d_n6, assign35560_e50434_d_n7, assign35560_e50434_d_n10, assign35560_e50434_d_n11, assign35560_e50434_d_n12, assign35560_e50434_d_n17,) = {
    if (((locals.var_guard1151 != 0.0) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 == 0.0)) {
        let assign35560_e50432: f64 = (locals.var_t4 * locals.var_t6);
        (assign35560_e50432, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign35560_e50434;
        locals.var_t5_dn0 = assign35560_e50434_d_n0;
        locals.var_t5_dn2 = assign35560_e50434_d_n2;
        locals.var_t5_dn6 = assign35560_e50434_d_n6;
        locals.var_t5_dn7 = assign35560_e50434_d_n7;
        locals.var_t5_dn10 = assign35560_e50434_d_n10;
        locals.var_t5_dn11 = assign35560_e50434_d_n11;
        locals.var_t5_dn12 = assign35560_e50434_d_n12;
        locals.var_t5_dn17 = assign35560_e50434_d_n17;

        let (assign35570_e50440, assign35570_e50440_d_n0, assign35570_e50440_d_n2, assign35570_e50440_d_n6, assign35570_e50440_d_n7, assign35570_e50440_d_n10, assign35570_e50440_d_n11, assign35570_e50440_d_n12, assign35570_e50440_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35570_e50438: f64 = (locals.var_mu0 * locals.var_t5);
        (assign35570_e50438, ((locals.var_mu0_dn0 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn0)), ((locals.var_mu0_dn2 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn2)), ((locals.var_mu0_dn6 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn6)), ((locals.var_mu0_dn7 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn7)), ((locals.var_mu0_dn10 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn10)), ((locals.var_mu0_dn11 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn11)), ((locals.var_mu0_dn12 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn12)), ((locals.var_mu0_dn17 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn17)),)
    } else {
        (locals.var_mu__blk1167, locals.var_mu__blk1167_dn0, locals.var_mu__blk1167_dn2, locals.var_mu__blk1167_dn6, locals.var_mu__blk1167_dn7, locals.var_mu__blk1167_dn10, locals.var_mu__blk1167_dn11, locals.var_mu__blk1167_dn12, locals.var_mu__blk1167_dn17,)
    }
};
        locals.var_mu__blk1167 = assign35570_e50440;
        locals.var_mu__blk1167_dn0 = assign35570_e50440_d_n0;
        locals.var_mu__blk1167_dn2 = assign35570_e50440_d_n2;
        locals.var_mu__blk1167_dn6 = assign35570_e50440_d_n6;
        locals.var_mu__blk1167_dn7 = assign35570_e50440_d_n7;
        locals.var_mu__blk1167_dn10 = assign35570_e50440_d_n10;
        locals.var_mu__blk1167_dn11 = assign35570_e50440_d_n11;
        locals.var_mu__blk1167_dn12 = assign35570_e50440_d_n12;
        locals.var_mu__blk1167_dn17 = assign35570_e50440_d_n17;

        let (assign35580_e50446, assign35580_e50446_d_n0, assign35580_e50446_d_n2, assign35580_e50446_d_n6, assign35580_e50446_d_n7, assign35580_e50446_d_n10, assign35580_e50446_d_n11, assign35580_e50446_d_n12, assign35580_e50446_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35580_e50444: f64 = (1.6021918e-19 / locals.var_ldrifte);
        (assign35580_e50444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35580_e50446;
        locals.var_t1_dn0 = assign35580_e50446_d_n0;
        locals.var_t1_dn2 = assign35580_e50446_d_n2;
        locals.var_t1_dn6 = assign35580_e50446_d_n6;
        locals.var_t1_dn7 = assign35580_e50446_d_n7;
        locals.var_t1_dn10 = assign35580_e50446_d_n10;
        locals.var_t1_dn11 = assign35580_e50446_d_n11;
        locals.var_t1_dn12 = assign35580_e50446_d_n12;
        locals.var_t1_dn17 = assign35580_e50446_d_n17;

        let (assign35590_e50456, assign35590_e50456_d_n0, assign35590_e50456_d_n2, assign35590_e50456_d_n6, assign35590_e50456_d_n7, assign35590_e50456_d_n10, assign35590_e50456_d_n11, assign35590_e50456_d_n12, assign35590_e50456_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35590_e50450: f64 = (locals.var_t1 * locals.var_xov);
        let assign35590_e50452: f64 = (assign35590_e50450 * locals.var_mu__blk1167);
        let assign35590_e50454: f64 = (assign35590_e50452 * locals.var_nover);
        (assign35590_e50454, ((((locals.var_t1_dn0 * locals.var_xov) * locals.var_mu__blk1167) + (assign35590_e50450 * locals.var_mu__blk1167_dn0)) * locals.var_nover), ((((locals.var_t1_dn2 * locals.var_xov) * locals.var_mu__blk1167) + (assign35590_e50450 * locals.var_mu__blk1167_dn2)) * locals.var_nover), ((((locals.var_t1_dn6 * locals.var_xov) * locals.var_mu__blk1167) + (assign35590_e50450 * locals.var_mu__blk1167_dn6)) * locals.var_nover), ((((locals.var_t1_dn7 * locals.var_xov) * locals.var_mu__blk1167) + (assign35590_e50450 * locals.var_mu__blk1167_dn7)) * locals.var_nover), ((((locals.var_t1_dn10 * locals.var_xov) * locals.var_mu__blk1167) + (assign35590_e50450 * locals.var_mu__blk1167_dn10)) * locals.var_nover), ((((locals.var_t1_dn11 * locals.var_xov) * locals.var_mu__blk1167) + (assign35590_e50450 * locals.var_mu__blk1167_dn11)) * locals.var_nover), ((((locals.var_t1_dn12 * locals.var_xov) * locals.var_mu__blk1167) + (assign35590_e50450 * locals.var_mu__blk1167_dn12)) * locals.var_nover), ((((locals.var_t1_dn17 * locals.var_xov) * locals.var_mu__blk1167) + (assign35590_e50450 * locals.var_mu__blk1167_dn17)) * locals.var_nover),)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn12, locals.var_gd_dn17,)
    }
};
        locals.var_gd = assign35590_e50456;
        locals.var_gd_dn0 = assign35590_e50456_d_n0;
        locals.var_gd_dn2 = assign35590_e50456_d_n2;
        locals.var_gd_dn6 = assign35590_e50456_d_n6;
        locals.var_gd_dn7 = assign35590_e50456_d_n7;
        locals.var_gd_dn10 = assign35590_e50456_d_n10;
        locals.var_gd_dn11 = assign35590_e50456_d_n11;
        locals.var_gd_dn12 = assign35590_e50456_d_n12;
        locals.var_gd_dn17 = assign35590_e50456_d_n17;

        let assign35600_e50459: f64 = if locals.var_gd <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign35600_e50459;

        let (assign35610_e50465, assign35610_e50465_d_n0, assign35610_e50465_d_n2, assign35610_e50465_d_n6, assign35610_e50465_d_n7, assign35610_e50465_d_n10, assign35610_e50465_d_n11, assign35610_e50465_d_n12, assign35610_e50465_d_n17,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1177 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn12, locals.var_gd_dn17,)
    }
};
        locals.var_gd = assign35610_e50465;
        locals.var_gd_dn0 = assign35610_e50465_d_n0;
        locals.var_gd_dn2 = assign35610_e50465_d_n2;
        locals.var_gd_dn6 = assign35610_e50465_d_n6;
        locals.var_gd_dn7 = assign35610_e50465_d_n7;
        locals.var_gd_dn10 = assign35610_e50465_d_n10;
        locals.var_gd_dn11 = assign35610_e50465_d_n11;
        locals.var_gd_dn12 = assign35610_e50465_d_n12;
        locals.var_gd_dn17 = assign35610_e50465_d_n17;

        let (assign35620_e50471, assign35620_e50471_d_n0, assign35620_e50471_d_n2, assign35620_e50471_d_n6, assign35620_e50471_d_n7, assign35620_e50471_d_n10, assign35620_e50471_d_n11, assign35620_e50471_d_n12, assign35620_e50471_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35620_e50469: f64 = (1.0 / locals.var_gd);
        (assign35620_e50469, (-(locals.var_gd_dn0 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn2 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn6 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn7 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn10 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn11 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn12 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn17 / (locals.var_gd * locals.var_gd))),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn12, locals.var_rsd_dn17,)
    }
};
        locals.var_rsd = assign35620_e50471;
        locals.var_rsd_dn0 = assign35620_e50471_d_n0;
        locals.var_rsd_dn2 = assign35620_e50471_d_n2;
        locals.var_rsd_dn6 = assign35620_e50471_d_n6;
        locals.var_rsd_dn7 = assign35620_e50471_d_n7;
        locals.var_rsd_dn10 = assign35620_e50471_d_n10;
        locals.var_rsd_dn11 = assign35620_e50471_d_n11;
        locals.var_rsd_dn12 = assign35620_e50471_d_n12;
        locals.var_rsd_dn17 = assign35620_e50471_d_n17;

        let (assign35630_e50477, assign35630_e50477_d_n0, assign35630_e50477_d_n2, assign35630_e50477_d_n6, assign35630_e50477_d_n7, assign35630_e50477_d_n10, assign35630_e50477_d_n11, assign35630_e50477_d_n12, assign35630_e50477_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35630_e50475: f64 = (locals.var_rsd / locals.var_weff_nf_1);
        (assign35630_e50475, (locals.var_rsd_dn0 / locals.var_weff_nf_1), (locals.var_rsd_dn2 / locals.var_weff_nf_1), (locals.var_rsd_dn6 / locals.var_weff_nf_1), (locals.var_rsd_dn7 / locals.var_weff_nf_1), (locals.var_rsd_dn10 / locals.var_weff_nf_1), (locals.var_rsd_dn11 / locals.var_weff_nf_1), (locals.var_rsd_dn12 / locals.var_weff_nf_1), (locals.var_rsd_dn17 / locals.var_weff_nf_1),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn12, locals.var_rsd_dn17,)
    }
};
        locals.var_rsd = assign35630_e50477;
        locals.var_rsd_dn0 = assign35630_e50477_d_n0;
        locals.var_rsd_dn2 = assign35630_e50477_d_n2;
        locals.var_rsd_dn6 = assign35630_e50477_d_n6;
        locals.var_rsd_dn7 = assign35630_e50477_d_n7;
        locals.var_rsd_dn10 = assign35630_e50477_d_n10;
        locals.var_rsd_dn11 = assign35630_e50477_d_n11;
        locals.var_rsd_dn12 = assign35630_e50477_d_n12;
        locals.var_rsd_dn17 = assign35630_e50477_d_n17;

        let (assign35640_e50483, assign35640_e50483_d_n0, assign35640_e50483_d_n2, assign35640_e50483_d_n6, assign35640_e50483_d_n7, assign35640_e50483_d_n10, assign35640_e50483_d_n11, assign35640_e50483_d_n12, assign35640_e50483_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35640_e50481: f64 = (locals.var_rsd + locals.var_rsd0);
        (assign35640_e50481, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn12, locals.var_rsd_dn17,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn12, locals.var_rsd_dn17,)
    }
};
        locals.var_rsd = assign35640_e50483;
        locals.var_rsd_dn0 = assign35640_e50483_d_n0;
        locals.var_rsd_dn2 = assign35640_e50483_d_n2;
        locals.var_rsd_dn6 = assign35640_e50483_d_n6;
        locals.var_rsd_dn7 = assign35640_e50483_d_n7;
        locals.var_rsd_dn10 = assign35640_e50483_d_n10;
        locals.var_rsd_dn11 = assign35640_e50483_d_n11;
        locals.var_rsd_dn12 = assign35640_e50483_d_n12;
        locals.var_rsd_dn17 = assign35640_e50483_d_n17;

        let assign35660_e50501: f64 = if locals.var_rsd < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1178 = assign35660_e50501;

        let (assign35670_e50507, assign35670_e50507_d_n0, assign35670_e50507_d_n2, assign35670_e50507_d_n6, assign35670_e50507_d_n7, assign35670_e50507_d_n10, assign35670_e50507_d_n11, assign35670_e50507_d_n12, assign35670_e50507_d_n17,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1178 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn12, locals.var_rsd_dn17,)
    }
};
        locals.var_rsd = assign35670_e50507;
        locals.var_rsd_dn0 = assign35670_e50507_d_n0;
        locals.var_rsd_dn2 = assign35670_e50507_d_n2;
        locals.var_rsd_dn6 = assign35670_e50507_d_n6;
        locals.var_rsd_dn7 = assign35670_e50507_d_n7;
        locals.var_rsd_dn10 = assign35670_e50507_d_n10;
        locals.var_rsd_dn11 = assign35670_e50507_d_n11;
        locals.var_rsd_dn12 = assign35670_e50507_d_n12;
        locals.var_rsd_dn17 = assign35670_e50507_d_n17;

    }

    pub(super) fn stamp_transient_block_124(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign35680_e50513, assign35680_e50513_d_n0, assign35680_e50513_d_n2, assign35680_e50513_d_n6, assign35680_e50513_d_n7, assign35680_e50513_d_n10, assign35680_e50513_d_n11, assign35680_e50513_d_n12, assign35680_e50513_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35680_e50511: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign35680_e50511, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn12 / locals.var_mfactor), (locals.var_rsd_dn17 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn12, locals.var_rsde_dn17,)
    }
};
        locals.var_rsde = assign35680_e50513;
        locals.var_rsde_dn0 = assign35680_e50513_d_n0;
        locals.var_rsde_dn2 = assign35680_e50513_d_n2;
        locals.var_rsde_dn6 = assign35680_e50513_d_n6;
        locals.var_rsde_dn7 = assign35680_e50513_d_n7;
        locals.var_rsde_dn10 = assign35680_e50513_d_n10;
        locals.var_rsde_dn11 = assign35680_e50513_d_n11;
        locals.var_rsde_dn12 = assign35680_e50513_d_n12;
        locals.var_rsde_dn17 = assign35680_e50513_d_n17;

        let assign35700_e50520: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1179 = assign35700_e50520;

        let (assign35710_e50524,) = {
    if (locals.var_guard1179 != 0.0) {
        (2.0,)
    } else {
        (locals.var_rdmod,)
    }
};
        locals.var_rdmod = assign35710_e50524;

        let assign35720_e50527: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign35720_e50527;

        let (assign35730_e50535,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 != 0.0)) {
        let assign35730_e50533: f64 = (p.p264 / 1e-6);
        (assign35730_e50533,)
    } else {
        (locals.var_nover__blk1190,)
    }
};
        locals.var_nover__blk1190 = assign35730_e50535;

        let (assign35740_e50541,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 != 0.0)) {
        (p.p266,)
    } else {
        (locals.var_mks_rdrmue__blk1183,)
    }
};
        locals.var_mks_rdrmue__blk1183 = assign35740_e50541;

        let (assign35750_e50547,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 != 0.0)) {
        (p.p268,)
    } else {
        (locals.var_mks_rdrvmax__blk1184,)
    }
};
        locals.var_mks_rdrvmax__blk1184 = assign35750_e50547;

        let (assign35760_e50553, assign35760_e50553_d_n10,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (locals.var_rrdrbb__blk1185, locals.var_rrdrbb__blk1185_dn10,)
    }
};
        locals.var_rrdrbb__blk1185 = assign35760_e50553;
        locals.var_rrdrbb__blk1185_dn10 = assign35760_e50553_d_n10;

        let (assign35770_e50566,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 != 0.0)) {
        let (assign35770_e50564,) = {
            if (p.p263 > 0.0) {
                let assign35770_e50562: f64 = (p.p263 * p.p255);
                (assign35770_e50562,)
            } else {
                (0.0,)
            }
        };
        (assign35770_e50564,)
    } else {
        (locals.var_rsd0__blk1186,)
    }
};
        locals.var_rsd0__blk1186 = assign35770_e50566;

        let (assign35780_e50572,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 != 0.0)) {
        (p.p258,)
    } else {
        (locals.var_ldrifte__blk1189,)
    }
};
        locals.var_ldrifte__blk1189 = assign35780_e50572;

        let (assign35790_e50580, assign35790_e50580_d_n0, assign35790_e50580_d_n2, assign35790_e50580_d_n6, assign35790_e50580_d_n7,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 != 0.0)) {
        let assign35790_e50578: f64 = (p.p50 * (nv7 - nv2));
        (assign35790_e50578, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (locals.var_vrdr__blk1187, locals.var_vrdr__blk1187_dn0, locals.var_vrdr__blk1187_dn2, locals.var_vrdr__blk1187_dn6, locals.var_vrdr__blk1187_dn7,)
    }
};
        locals.var_vrdr__blk1187 = assign35790_e50580;
        locals.var_vrdr__blk1187_dn0 = assign35790_e50580_d_n0;
        locals.var_vrdr__blk1187_dn2 = assign35790_e50580_d_n2;
        locals.var_vrdr__blk1187_dn6 = assign35790_e50580_d_n6;
        locals.var_vrdr__blk1187_dn7 = assign35790_e50580_d_n7;

        let (assign35800_e50589,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 == 0.0)) {
        let assign35800_e50587: f64 = (p.p59 / 1e-6);
        (assign35800_e50587,)
    } else {
        (locals.var_nover__blk1190,)
    }
};
        locals.var_nover__blk1190 = assign35800_e50589;

        let (assign35810_e50596,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 == 0.0)) {
        (p.p265,)
    } else {
        (locals.var_mks_rdrmue__blk1183,)
    }
};
        locals.var_mks_rdrmue__blk1183 = assign35810_e50596;

        let (assign35820_e50603,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 == 0.0)) {
        (p.p267,)
    } else {
        (locals.var_mks_rdrvmax__blk1184,)
    }
};
        locals.var_mks_rdrvmax__blk1184 = assign35820_e50603;

        let (assign35830_e50610, assign35830_e50610_d_n10,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (locals.var_rrdrbb__blk1185, locals.var_rrdrbb__blk1185_dn10,)
    }
};
        locals.var_rrdrbb__blk1185 = assign35830_e50610;
        locals.var_rrdrbb__blk1185_dn10 = assign35830_e50610_d_n10;

        let (assign35840_e50624,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 == 0.0)) {
        let (assign35840_e50622,) = {
            if (p.p263 > 0.0) {
                let assign35840_e50620: f64 = (p.p263 * p.p256);
                (assign35840_e50620,)
            } else {
                (0.0,)
            }
        };
        (assign35840_e50622,)
    } else {
        (locals.var_rsd0__blk1186,)
    }
};
        locals.var_rsd0__blk1186 = assign35840_e50624;

        let (assign35850_e50631,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 == 0.0)) {
        (p.p257,)
    } else {
        (locals.var_ldrifte__blk1189,)
    }
};
        locals.var_ldrifte__blk1189 = assign35850_e50631;

        let (assign35860_e50640, assign35860_e50640_d_n0, assign35860_e50640_d_n2, assign35860_e50640_d_n6, assign35860_e50640_d_n7,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 == 0.0)) {
        let assign35860_e50638: f64 = (p.p50 * (nv0 - nv6));
        (assign35860_e50638, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (locals.var_vrdr__blk1187, locals.var_vrdr__blk1187_dn0, locals.var_vrdr__blk1187_dn2, locals.var_vrdr__blk1187_dn6, locals.var_vrdr__blk1187_dn7,)
    }
};
        locals.var_vrdr__blk1187 = assign35860_e50640;
        locals.var_vrdr__blk1187_dn0 = assign35860_e50640_d_n0;
        locals.var_vrdr__blk1187_dn2 = assign35860_e50640_d_n2;
        locals.var_vrdr__blk1187_dn6 = assign35860_e50640_d_n6;
        locals.var_vrdr__blk1187_dn7 = assign35860_e50640_d_n7;

        let (assign35870_e50651,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35870_e50644: f64 = (p.p271 * p.p271);
        let assign35870_e50647: f64 = (p.p56 * p.p56);
        let assign35870_e50648: f64 = (assign35870_e50644 + assign35870_e50647);
        let assign35870_e50649: f64 = (assign35870_e50648).sqrt();
        (assign35870_e50649,)
    } else {
        (locals.var_xov__blk1196,)
    }
};
        locals.var_xov__blk1196 = assign35870_e50651;

        let (assign35880_e50657,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35880_e50655: f64 = (locals.var_weff * p.p9);
        (assign35880_e50655,)
    } else {
        (locals.var_weff_nf__blk1198,)
    }
};
        locals.var_weff_nf__blk1198 = assign35880_e50657;

        let (assign35890_e50663,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35890_e50661: f64 = (locals.var_mks_rdrmue__blk1183 / 10000.0);
        (assign35890_e50661,)
    } else {
        (locals.var_mks_rdrmue__blk1183,)
    }
};
        locals.var_mks_rdrmue__blk1183 = assign35890_e50663;

        let (assign35900_e50669,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35900_e50667: f64 = (locals.var_mks_rdrvmax__blk1184 / 100.0);
        (assign35900_e50667,)
    } else {
        (locals.var_mks_rdrvmax__blk1184,)
    }
};
        locals.var_mks_rdrvmax__blk1184 = assign35900_e50669;

        let (assign35910_e50675, assign35910_e50675_d_n10,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35910_e50673: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign35910_e50673, (locals.var_ttemp_dn10 / locals.var_uc_tnom),)
    } else {
        (locals.var_tratio__blk1188, locals.var_tratio__blk1188_dn10,)
    }
};
        locals.var_tratio__blk1188 = assign35910_e50675;
        locals.var_tratio__blk1188_dn10 = assign35910_e50675_d_n10;

        let (assign35920_e50681, assign35920_e50681_d_n0, assign35920_e50681_d_n2, assign35920_e50681_d_n6, assign35920_e50681_d_n7, assign35920_e50681_d_n10, assign35920_e50681_d_n11, assign35920_e50681_d_n12, assign35920_e50681_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35920_e50679: f64 = (locals.var_tratio__blk1188).powf(p.p269);
        (assign35920_e50679, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio__blk1188).powf(p.p269 - 1.0) * locals.var_tratio__blk1188_dn10)) } } else { (assign35920_e50679 * (p.p269 * (locals.var_tratio__blk1188_dn10 / locals.var_tratio__blk1188))) }, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35920_e50681;
        locals.var_t1_dn0 = assign35920_e50681_d_n0;
        locals.var_t1_dn2 = assign35920_e50681_d_n2;
        locals.var_t1_dn6 = assign35920_e50681_d_n6;
        locals.var_t1_dn7 = assign35920_e50681_d_n7;
        locals.var_t1_dn10 = assign35920_e50681_d_n10;
        locals.var_t1_dn11 = assign35920_e50681_d_n11;
        locals.var_t1_dn12 = assign35920_e50681_d_n12;
        locals.var_t1_dn17 = assign35920_e50681_d_n17;

        let (assign35930_e50687, assign35930_e50687_d_n0, assign35930_e50687_d_n2, assign35930_e50687_d_n6, assign35930_e50687_d_n7, assign35930_e50687_d_n10, assign35930_e50687_d_n11, assign35930_e50687_d_n12, assign35930_e50687_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35930_e50685: f64 = (locals.var_mks_rdrmue__blk1183 / locals.var_t1);
        (assign35930_e50685, (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_mu0__blk1191, locals.var_mu0__blk1191_dn0, locals.var_mu0__blk1191_dn2, locals.var_mu0__blk1191_dn6, locals.var_mu0__blk1191_dn7, locals.var_mu0__blk1191_dn10, locals.var_mu0__blk1191_dn11, locals.var_mu0__blk1191_dn12, locals.var_mu0__blk1191_dn17,)
    }
};
        locals.var_mu0__blk1191 = assign35930_e50687;
        locals.var_mu0__blk1191_dn0 = assign35930_e50687_d_n0;
        locals.var_mu0__blk1191_dn2 = assign35930_e50687_d_n2;
        locals.var_mu0__blk1191_dn6 = assign35930_e50687_d_n6;
        locals.var_mu0__blk1191_dn7 = assign35930_e50687_d_n7;
        locals.var_mu0__blk1191_dn10 = assign35930_e50687_d_n10;
        locals.var_mu0__blk1191_dn11 = assign35930_e50687_d_n11;
        locals.var_mu0__blk1191_dn12 = assign35930_e50687_d_n12;
        locals.var_mu0__blk1191_dn17 = assign35930_e50687_d_n17;

        let (assign35940_e50707, assign35940_e50707_d_n0, assign35940_e50707_d_n2, assign35940_e50707_d_n6, assign35940_e50707_d_n7, assign35940_e50707_d_n10, assign35940_e50707_d_n11, assign35940_e50707_d_n12, assign35940_e50707_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35940_e50692: f64 = (0.4 * locals.var_tratio__blk1188);
        let assign35940_e50693: f64 = (1.8 + assign35940_e50692);
        let assign35940_e50696: f64 = (0.1 * locals.var_tratio__blk1188);
        let assign35940_e50698: f64 = (assign35940_e50696 * locals.var_tratio__blk1188);
        let assign35940_e50699: f64 = (assign35940_e50693 + assign35940_e50698);
        let assign35940_e50703: f64 = (1.0 - locals.var_tratio__blk1188);
        let assign35940_e50704: f64 = (p.p270 * assign35940_e50703);
        let assign35940_e50705: f64 = (assign35940_e50699 - assign35940_e50704);
        (assign35940_e50705, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio__blk1188_dn10) + (((0.1 * locals.var_tratio__blk1188_dn10) * locals.var_tratio__blk1188) + (assign35940_e50696 * locals.var_tratio__blk1188_dn10))) - (p.p270 * (-locals.var_tratio__blk1188_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign35940_e50707;
        locals.var_t0_dn0 = assign35940_e50707_d_n0;
        locals.var_t0_dn2 = assign35940_e50707_d_n2;
        locals.var_t0_dn6 = assign35940_e50707_d_n6;
        locals.var_t0_dn7 = assign35940_e50707_d_n7;
        locals.var_t0_dn10 = assign35940_e50707_d_n10;
        locals.var_t0_dn11 = assign35940_e50707_d_n11;
        locals.var_t0_dn12 = assign35940_e50707_d_n12;
        locals.var_t0_dn17 = assign35940_e50707_d_n17;

        let (assign35950_e50713, assign35950_e50713_d_n0, assign35950_e50713_d_n2, assign35950_e50713_d_n6, assign35950_e50713_d_n7, assign35950_e50713_d_n10, assign35950_e50713_d_n11, assign35950_e50713_d_n12, assign35950_e50713_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35950_e50711: f64 = (locals.var_mks_rdrvmax__blk1184 / locals.var_t0);
        (assign35950_e50711, (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_vmaxe__blk1192, locals.var_vmaxe__blk1192_dn0, locals.var_vmaxe__blk1192_dn2, locals.var_vmaxe__blk1192_dn6, locals.var_vmaxe__blk1192_dn7, locals.var_vmaxe__blk1192_dn10, locals.var_vmaxe__blk1192_dn11, locals.var_vmaxe__blk1192_dn12, locals.var_vmaxe__blk1192_dn17,)
    }
};
        locals.var_vmaxe__blk1192 = assign35950_e50713;
        locals.var_vmaxe__blk1192_dn0 = assign35950_e50713_d_n0;
        locals.var_vmaxe__blk1192_dn2 = assign35950_e50713_d_n2;
        locals.var_vmaxe__blk1192_dn6 = assign35950_e50713_d_n6;
        locals.var_vmaxe__blk1192_dn7 = assign35950_e50713_d_n7;
        locals.var_vmaxe__blk1192_dn10 = assign35950_e50713_d_n10;
        locals.var_vmaxe__blk1192_dn11 = assign35950_e50713_d_n11;
        locals.var_vmaxe__blk1192_dn12 = assign35950_e50713_d_n12;
        locals.var_vmaxe__blk1192_dn17 = assign35950_e50713_d_n17;

        let (assign35960_e50723, assign35960_e50723_d_n10,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35960_e50719: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        let assign35960_e50720: f64 = (p.p274 * assign35960_e50719);
        let assign35960_e50721: f64 = (locals.var_rrdrbb__blk1185 + assign35960_e50720);
        (assign35960_e50721, (locals.var_rrdrbb__blk1185_dn10 + (p.p274 * locals.var_ttemp_dn10)),)
    } else {
        (locals.var_rrdrbb__blk1185, locals.var_rrdrbb__blk1185_dn10,)
    }
};
        locals.var_rrdrbb__blk1185 = assign35960_e50723;
        locals.var_rrdrbb__blk1185_dn10 = assign35960_e50723_d_n10;

        let (assign35970_e50733,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35970_e50729: f64 = (locals.var_lgle).powf(p.p280);
        let assign35970_e50730: f64 = (p.p279 / assign35970_e50729);
        let assign35970_e50731: f64 = (1.0 + assign35970_e50730);
        (assign35970_e50731,)
    } else {
        (locals.var_rdrmuele__blk1180,)
    }
};
        locals.var_rdrmuele__blk1180 = assign35970_e50733;

        let (assign35980_e50743,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35980_e50739: f64 = (locals.var_lgle).powf(p.p278);
        let assign35980_e50740: f64 = (p.p277 / assign35980_e50739);
        let assign35980_e50741: f64 = (1.0 + assign35980_e50740);
        (assign35980_e50741,)
    } else {
        (locals.var_rdrvmaxle__blk1182,)
    }
};
        locals.var_rdrvmaxle__blk1182 = assign35980_e50743;

        let (assign35990_e50753,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign35990_e50749: f64 = (locals.var_wg).powf(p.p276);
        let assign35990_e50750: f64 = (p.p275 / assign35990_e50749);
        let assign35990_e50751: f64 = (1.0 + assign35990_e50750);
        (assign35990_e50751,)
    } else {
        (locals.var_rdrvmaxwe__blk1181,)
    }
};
        locals.var_rdrvmaxwe__blk1181 = assign35990_e50753;

        let (assign36000_e50759, assign36000_e50759_d_n0, assign36000_e50759_d_n2, assign36000_e50759_d_n6, assign36000_e50759_d_n7, assign36000_e50759_d_n10, assign36000_e50759_d_n11, assign36000_e50759_d_n12, assign36000_e50759_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36000_e50757: f64 = (locals.var_mu0__blk1191 * locals.var_rdrmuele__blk1180);
        (assign36000_e50757, (locals.var_mu0__blk1191_dn0 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn2 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn6 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn7 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn10 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn11 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn12 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn17 * locals.var_rdrmuele__blk1180),)
    } else {
        (locals.var_mu0__blk1191, locals.var_mu0__blk1191_dn0, locals.var_mu0__blk1191_dn2, locals.var_mu0__blk1191_dn6, locals.var_mu0__blk1191_dn7, locals.var_mu0__blk1191_dn10, locals.var_mu0__blk1191_dn11, locals.var_mu0__blk1191_dn12, locals.var_mu0__blk1191_dn17,)
    }
};
        locals.var_mu0__blk1191 = assign36000_e50759;
        locals.var_mu0__blk1191_dn0 = assign36000_e50759_d_n0;
        locals.var_mu0__blk1191_dn2 = assign36000_e50759_d_n2;
        locals.var_mu0__blk1191_dn6 = assign36000_e50759_d_n6;
        locals.var_mu0__blk1191_dn7 = assign36000_e50759_d_n7;
        locals.var_mu0__blk1191_dn10 = assign36000_e50759_d_n10;
        locals.var_mu0__blk1191_dn11 = assign36000_e50759_d_n11;
        locals.var_mu0__blk1191_dn12 = assign36000_e50759_d_n12;
        locals.var_mu0__blk1191_dn17 = assign36000_e50759_d_n17;

        let (assign36010_e50769, assign36010_e50769_d_n0, assign36010_e50769_d_n2, assign36010_e50769_d_n6, assign36010_e50769_d_n7, assign36010_e50769_d_n10, assign36010_e50769_d_n11, assign36010_e50769_d_n12, assign36010_e50769_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36010_e50763: f64 = (locals.var_vmaxe__blk1192 * locals.var_rdrvmaxwe__blk1181);
        let assign36010_e50765: f64 = (assign36010_e50763 * locals.var_rdrvmaxle__blk1182);
        let assign36010_e50767: f64 = (assign36010_e50765 + 1e-50);
        (assign36010_e50767, ((locals.var_vmaxe__blk1192_dn0 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn2 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn6 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn7 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn10 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn11 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn12 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn17 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182),)
    } else {
        (locals.var_vmaxe__blk1192, locals.var_vmaxe__blk1192_dn0, locals.var_vmaxe__blk1192_dn2, locals.var_vmaxe__blk1192_dn6, locals.var_vmaxe__blk1192_dn7, locals.var_vmaxe__blk1192_dn10, locals.var_vmaxe__blk1192_dn11, locals.var_vmaxe__blk1192_dn12, locals.var_vmaxe__blk1192_dn17,)
    }
};
        locals.var_vmaxe__blk1192 = assign36010_e50769;
        locals.var_vmaxe__blk1192_dn0 = assign36010_e50769_d_n0;
        locals.var_vmaxe__blk1192_dn2 = assign36010_e50769_d_n2;
        locals.var_vmaxe__blk1192_dn6 = assign36010_e50769_d_n6;
        locals.var_vmaxe__blk1192_dn7 = assign36010_e50769_d_n7;
        locals.var_vmaxe__blk1192_dn10 = assign36010_e50769_d_n10;
        locals.var_vmaxe__blk1192_dn11 = assign36010_e50769_d_n11;
        locals.var_vmaxe__blk1192_dn12 = assign36010_e50769_d_n12;
        locals.var_vmaxe__blk1192_dn17 = assign36010_e50769_d_n17;

        let (assign36020_e50775, assign36020_e50775_d_n0, assign36020_e50775_d_n2, assign36020_e50775_d_n6, assign36020_e50775_d_n7,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36020_e50773: f64 = (locals.var_vrdr__blk1187 / locals.var_ldrifte__blk1189);
        (assign36020_e50773, (locals.var_vrdr__blk1187_dn0 / locals.var_ldrifte__blk1189), (locals.var_vrdr__blk1187_dn2 / locals.var_ldrifte__blk1189), (locals.var_vrdr__blk1187_dn6 / locals.var_ldrifte__blk1189), (locals.var_vrdr__blk1187_dn7 / locals.var_ldrifte__blk1189),)
    } else {
        (locals.var_edri__blk1193, locals.var_edri__blk1193_dn0, locals.var_edri__blk1193_dn2, locals.var_edri__blk1193_dn6, locals.var_edri__blk1193_dn7,)
    }
};
        locals.var_edri__blk1193 = assign36020_e50775;
        locals.var_edri__blk1193_dn0 = assign36020_e50775_d_n0;
        locals.var_edri__blk1193_dn2 = assign36020_e50775_d_n2;
        locals.var_edri__blk1193_dn6 = assign36020_e50775_d_n6;
        locals.var_edri__blk1193_dn7 = assign36020_e50775_d_n7;

        let (assign36030_e50781, assign36030_e50781_d_n0, assign36030_e50781_d_n2, assign36030_e50781_d_n6, assign36030_e50781_d_n7, assign36030_e50781_d_n10, assign36030_e50781_d_n11, assign36030_e50781_d_n12, assign36030_e50781_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36030_e50779: f64 = (locals.var_mu0__blk1191 * locals.var_edri__blk1193);
        (assign36030_e50779, ((locals.var_mu0__blk1191_dn0 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn0)), ((locals.var_mu0__blk1191_dn2 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn2)), ((locals.var_mu0__blk1191_dn6 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn6)), ((locals.var_mu0__blk1191_dn7 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn7)), (locals.var_mu0__blk1191_dn10 * locals.var_edri__blk1193), (locals.var_mu0__blk1191_dn11 * locals.var_edri__blk1193), (locals.var_mu0__blk1191_dn12 * locals.var_edri__blk1193), (locals.var_mu0__blk1191_dn17 * locals.var_edri__blk1193),)
    } else {
        (locals.var_vdri__blk1194, locals.var_vdri__blk1194_dn0, locals.var_vdri__blk1194_dn2, locals.var_vdri__blk1194_dn6, locals.var_vdri__blk1194_dn7, locals.var_vdri__blk1194_dn10, locals.var_vdri__blk1194_dn11, locals.var_vdri__blk1194_dn12, locals.var_vdri__blk1194_dn17,)
    }
};
        locals.var_vdri__blk1194 = assign36030_e50781;
        locals.var_vdri__blk1194_dn0 = assign36030_e50781_d_n0;
        locals.var_vdri__blk1194_dn2 = assign36030_e50781_d_n2;
        locals.var_vdri__blk1194_dn6 = assign36030_e50781_d_n6;
        locals.var_vdri__blk1194_dn7 = assign36030_e50781_d_n7;
        locals.var_vdri__blk1194_dn10 = assign36030_e50781_d_n10;
        locals.var_vdri__blk1194_dn11 = assign36030_e50781_d_n11;
        locals.var_vdri__blk1194_dn12 = assign36030_e50781_d_n12;
        locals.var_vdri__blk1194_dn17 = assign36030_e50781_d_n17;

        let assign36040_e50784: f64 = if locals.var_vrdr__blk1187 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign36040_e50784;

        let (assign36050_e50792, assign36050_e50792_d_n0, assign36050_e50792_d_n2, assign36050_e50792_d_n6, assign36050_e50792_d_n7, assign36050_e50792_d_n10, assign36050_e50792_d_n11, assign36050_e50792_d_n12, assign36050_e50792_d_n17,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign36050_e50790: f64 = (locals.var_vdri__blk1194 / locals.var_vmaxe__blk1192);
        (assign36050_e50790, (((locals.var_vdri__blk1194_dn0 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn0)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn2 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn2)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn6 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn6)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn7 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn7)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn10 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn10)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn11 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn11)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn12 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn12)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn17 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn17)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign36050_e50792;
        locals.var_t1_dn0 = assign36050_e50792_d_n0;
        locals.var_t1_dn2 = assign36050_e50792_d_n2;
        locals.var_t1_dn6 = assign36050_e50792_d_n6;
        locals.var_t1_dn7 = assign36050_e50792_d_n7;
        locals.var_t1_dn10 = assign36050_e50792_d_n10;
        locals.var_t1_dn11 = assign36050_e50792_d_n11;
        locals.var_t1_dn12 = assign36050_e50792_d_n12;
        locals.var_t1_dn17 = assign36050_e50792_d_n17;

        let (assign36060_e50802, assign36060_e50802_d_n0, assign36060_e50802_d_n2, assign36060_e50802_d_n6, assign36060_e50802_d_n7, assign36060_e50802_d_n10, assign36060_e50802_d_n11, assign36060_e50802_d_n12, assign36060_e50802_d_n17,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign36060_e50798: f64 = (-locals.var_vdri__blk1194);
        let assign36060_e50800: f64 = (assign36060_e50798 / locals.var_vmaxe__blk1192);
        (assign36060_e50800, ((((-locals.var_vdri__blk1194_dn0) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn0)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn2) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn2)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn6) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn6)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn7) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn7)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn10) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn10)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn11) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn11)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn12) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn12)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn17) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn17)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign36060_e50802;
        locals.var_t1_dn0 = assign36060_e50802_d_n0;
        locals.var_t1_dn2 = assign36060_e50802_d_n2;
        locals.var_t1_dn6 = assign36060_e50802_d_n6;
        locals.var_t1_dn7 = assign36060_e50802_d_n7;
        locals.var_t1_dn10 = assign36060_e50802_d_n10;
        locals.var_t1_dn11 = assign36060_e50802_d_n11;
        locals.var_t1_dn12 = assign36060_e50802_d_n12;
        locals.var_t1_dn17 = assign36060_e50802_d_n17;

        let assign36070_e50806: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50807: f64 = (1.0 - assign36070_e50806);
        let assign36070_e50814: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50815: f64 = (1.0 + assign36070_e50814);
        let assign36070_e50817: f64 = if ((assign36070_e50807 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36070_e50815)) { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign36070_e50817;

        let (assign36080_e50823, assign36080_e50823_d_n0, assign36080_e50823_d_n2, assign36080_e50823_d_n6, assign36080_e50823_d_n7, assign36080_e50823_d_n10, assign36080_e50823_d_n11, assign36080_e50823_d_n12, assign36080_e50823_d_n17,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1201 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign36080_e50823;
        locals.var_t3_dn0 = assign36080_e50823_d_n0;
        locals.var_t3_dn2 = assign36080_e50823_d_n2;
        locals.var_t3_dn6 = assign36080_e50823_d_n6;
        locals.var_t3_dn7 = assign36080_e50823_d_n7;
        locals.var_t3_dn10 = assign36080_e50823_d_n10;
        locals.var_t3_dn11 = assign36080_e50823_d_n11;
        locals.var_t3_dn12 = assign36080_e50823_d_n12;
        locals.var_t3_dn17 = assign36080_e50823_d_n17;

        let assign36090_e50827: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50828: f64 = (2.0 - assign36090_e50827);
        let assign36090_e50835: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50836: f64 = (2.0 + assign36090_e50835);
        let assign36090_e50838: f64 = if ((assign36090_e50828 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36090_e50836)) { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign36090_e50838;

    }

    pub(super) fn stamp_transient_block_125(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36100_e50847, assign36100_e50847_d_n0, assign36100_e50847_d_n2, assign36100_e50847_d_n6, assign36100_e50847_d_n7, assign36100_e50847_d_n10, assign36100_e50847_d_n11, assign36100_e50847_d_n12, assign36100_e50847_d_n17,) = {
    if (((locals.var_guard1179 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign36100_e50847;
        locals.var_t3_dn0 = assign36100_e50847_d_n0;
        locals.var_t3_dn2 = assign36100_e50847_d_n2;
        locals.var_t3_dn6 = assign36100_e50847_d_n6;
        locals.var_t3_dn7 = assign36100_e50847_d_n7;
        locals.var_t3_dn10 = assign36100_e50847_d_n10;
        locals.var_t3_dn11 = assign36100_e50847_d_n11;
        locals.var_t3_dn12 = assign36100_e50847_d_n12;
        locals.var_t3_dn17 = assign36100_e50847_d_n17;

        let (assign36110_e50861, assign36110_e50861_d_n0, assign36110_e50861_d_n2, assign36110_e50861_d_n6, assign36110_e50861_d_n7, assign36110_e50861_d_n10, assign36110_e50861_d_n11, assign36110_e50861_d_n12, assign36110_e50861_d_n17,) = {
    if (((locals.var_guard1179 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
        let assign36110_e50858: f64 = (locals.var_rrdrbb__blk1185 - 1.0);
        let assign36110_e50859: f64 = (locals.var_t1).powf(assign36110_e50858);
        (assign36110_e50859, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn0)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn2)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn6)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn7)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb__blk1185_dn10 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn10)) } } else { (assign36110_e50859 * ((locals.var_rrdrbb__blk1185_dn10 * (locals.var_t1).ln()) + (assign36110_e50858 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn11)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn12)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn17)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn17 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign36110_e50861;
        locals.var_t3_dn0 = assign36110_e50861_d_n0;
        locals.var_t3_dn2 = assign36110_e50861_d_n2;
        locals.var_t3_dn6 = assign36110_e50861_d_n6;
        locals.var_t3_dn7 = assign36110_e50861_d_n7;
        locals.var_t3_dn10 = assign36110_e50861_d_n10;
        locals.var_t3_dn11 = assign36110_e50861_d_n11;
        locals.var_t3_dn12 = assign36110_e50861_d_n12;
        locals.var_t3_dn17 = assign36110_e50861_d_n17;

        let (assign36120_e50867, assign36120_e50867_d_n0, assign36120_e50867_d_n2, assign36120_e50867_d_n6, assign36120_e50867_d_n7, assign36120_e50867_d_n10, assign36120_e50867_d_n11, assign36120_e50867_d_n12, assign36120_e50867_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36120_e50865: f64 = (locals.var_t1 * locals.var_t3);
        (assign36120_e50865, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign36120_e50867;
        locals.var_t2_dn0 = assign36120_e50867_d_n0;
        locals.var_t2_dn2 = assign36120_e50867_d_n2;
        locals.var_t2_dn6 = assign36120_e50867_d_n6;
        locals.var_t2_dn7 = assign36120_e50867_d_n7;
        locals.var_t2_dn10 = assign36120_e50867_d_n10;
        locals.var_t2_dn11 = assign36120_e50867_d_n11;
        locals.var_t2_dn12 = assign36120_e50867_d_n12;
        locals.var_t2_dn17 = assign36120_e50867_d_n17;

        let (assign36130_e50873, assign36130_e50873_d_n0, assign36130_e50873_d_n2, assign36130_e50873_d_n6, assign36130_e50873_d_n7, assign36130_e50873_d_n10, assign36130_e50873_d_n11, assign36130_e50873_d_n12, assign36130_e50873_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36130_e50871: f64 = (1.0 + locals.var_t2);
        (assign36130_e50871, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign36130_e50873;
        locals.var_t4_dn0 = assign36130_e50873_d_n0;
        locals.var_t4_dn2 = assign36130_e50873_d_n2;
        locals.var_t4_dn6 = assign36130_e50873_d_n6;
        locals.var_t4_dn7 = assign36130_e50873_d_n7;
        locals.var_t4_dn10 = assign36130_e50873_d_n10;
        locals.var_t4_dn11 = assign36130_e50873_d_n11;
        locals.var_t4_dn12 = assign36130_e50873_d_n12;
        locals.var_t4_dn17 = assign36130_e50873_d_n17;

        let assign36140_e50877: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50878: f64 = (1.0 - assign36140_e50877);
        let assign36140_e50885: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50886: f64 = (1.0 + assign36140_e50885);
        let assign36140_e50888: f64 = if ((assign36140_e50878 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36140_e50886)) { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign36140_e50888;

        let (assign36150_e50896, assign36150_e50896_d_n0, assign36150_e50896_d_n2, assign36150_e50896_d_n6, assign36150_e50896_d_n7, assign36150_e50896_d_n10, assign36150_e50896_d_n11, assign36150_e50896_d_n12, assign36150_e50896_d_n17,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1203 != 0.0)) {
        let assign36150_e50894: f64 = (1.0 / locals.var_t4);
        (assign36150_e50894, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign36150_e50896;
        locals.var_t5_dn0 = assign36150_e50896_d_n0;
        locals.var_t5_dn2 = assign36150_e50896_d_n2;
        locals.var_t5_dn6 = assign36150_e50896_d_n6;
        locals.var_t5_dn7 = assign36150_e50896_d_n7;
        locals.var_t5_dn10 = assign36150_e50896_d_n10;
        locals.var_t5_dn11 = assign36150_e50896_d_n11;
        locals.var_t5_dn12 = assign36150_e50896_d_n12;
        locals.var_t5_dn17 = assign36150_e50896_d_n17;

        let assign36160_e50900: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50901: f64 = (2.0 - assign36160_e50900);
        let assign36160_e50908: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50909: f64 = (2.0 + assign36160_e50908);
        let assign36160_e50911: f64 = if ((assign36160_e50901 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36160_e50909)) { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign36160_e50911;

        let (assign36170_e50923, assign36170_e50923_d_n0, assign36170_e50923_d_n2, assign36170_e50923_d_n6, assign36170_e50923_d_n7, assign36170_e50923_d_n10, assign36170_e50923_d_n11, assign36170_e50923_d_n12, assign36170_e50923_d_n17,) = {
    if (((locals.var_guard1179 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign36170_e50920: f64 = (locals.var_t4).sqrt();
        let assign36170_e50921: f64 = (1.0 / assign36170_e50920);
        (assign36170_e50921, (-((locals.var_t4_dn0 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn2 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn6 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn7 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn10 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn11 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn12 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn17 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign36170_e50923;
        locals.var_t5_dn0 = assign36170_e50923_d_n0;
        locals.var_t5_dn2 = assign36170_e50923_d_n2;
        locals.var_t5_dn6 = assign36170_e50923_d_n6;
        locals.var_t5_dn7 = assign36170_e50923_d_n7;
        locals.var_t5_dn10 = assign36170_e50923_d_n10;
        locals.var_t5_dn11 = assign36170_e50923_d_n11;
        locals.var_t5_dn12 = assign36170_e50923_d_n12;
        locals.var_t5_dn17 = assign36170_e50923_d_n17;

        let (assign36180_e50940, assign36180_e50940_d_n0, assign36180_e50940_d_n2, assign36180_e50940_d_n6, assign36180_e50940_d_n7, assign36180_e50940_d_n10, assign36180_e50940_d_n11, assign36180_e50940_d_n12, assign36180_e50940_d_n17,) = {
    if (((locals.var_guard1179 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign36180_e50933: f64 = (-1.0);
        let assign36180_e50935: f64 = (assign36180_e50933 / locals.var_rrdrbb__blk1185);
        let assign36180_e50937: f64 = (assign36180_e50935 - 1.0);
        let assign36180_e50938: f64 = (locals.var_t4).powf(assign36180_e50937);
        (assign36180_e50938, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn0)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn2)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn6)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn7)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign36180_e50933 * locals.var_rrdrbb__blk1185_dn10) / (locals.var_rrdrbb__blk1185 * locals.var_rrdrbb__blk1185))) == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn10)) } } else { (assign36180_e50938 * (((-((assign36180_e50933 * locals.var_rrdrbb__blk1185_dn10) / (locals.var_rrdrbb__blk1185 * locals.var_rrdrbb__blk1185))) * (locals.var_t4).ln()) + (assign36180_e50937 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn11)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn12)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn17)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn17 / locals.var_t4))) },)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign36180_e50940;
        locals.var_t6_dn0 = assign36180_e50940_d_n0;
        locals.var_t6_dn2 = assign36180_e50940_d_n2;
        locals.var_t6_dn6 = assign36180_e50940_d_n6;
        locals.var_t6_dn7 = assign36180_e50940_d_n7;
        locals.var_t6_dn10 = assign36180_e50940_d_n10;
        locals.var_t6_dn11 = assign36180_e50940_d_n11;
        locals.var_t6_dn12 = assign36180_e50940_d_n12;
        locals.var_t6_dn17 = assign36180_e50940_d_n17;

        let (assign36190_e50952, assign36190_e50952_d_n0, assign36190_e50952_d_n2, assign36190_e50952_d_n6, assign36190_e50952_d_n7, assign36190_e50952_d_n10, assign36190_e50952_d_n11, assign36190_e50952_d_n12, assign36190_e50952_d_n17,) = {
    if (((locals.var_guard1179 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign36190_e50950: f64 = (locals.var_t4 * locals.var_t6);
        (assign36190_e50950, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign36190_e50952;
        locals.var_t5_dn0 = assign36190_e50952_d_n0;
        locals.var_t5_dn2 = assign36190_e50952_d_n2;
        locals.var_t5_dn6 = assign36190_e50952_d_n6;
        locals.var_t5_dn7 = assign36190_e50952_d_n7;
        locals.var_t5_dn10 = assign36190_e50952_d_n10;
        locals.var_t5_dn11 = assign36190_e50952_d_n11;
        locals.var_t5_dn12 = assign36190_e50952_d_n12;
        locals.var_t5_dn17 = assign36190_e50952_d_n17;

        let (assign36200_e50958, assign36200_e50958_d_n0, assign36200_e50958_d_n2, assign36200_e50958_d_n6, assign36200_e50958_d_n7, assign36200_e50958_d_n10, assign36200_e50958_d_n11, assign36200_e50958_d_n12, assign36200_e50958_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36200_e50956: f64 = (locals.var_mu0__blk1191 * locals.var_t5);
        (assign36200_e50956, ((locals.var_mu0__blk1191_dn0 * locals.var_t5) + (locals.var_mu0__blk1191 * locals.var_t5_dn0)), ((locals.var_mu0__blk1191_dn2 * locals.var_t5) + (locals.var_mu0__blk1191 * locals.var_t5_dn2)), ((locals.var_mu0__blk1191_dn6 * locals.var_t5) + (locals.var_mu0__blk1191 * locals.var_t5_dn6)), ((locals.var_mu0__blk1191_dn7 * locals.var_t5) + (locals.var_mu0__blk1191 * locals.var_t5_dn7)), ((locals.var_mu0__blk1191_dn10 * locals.var_t5) + (locals.var_mu0__blk1191 * locals.var_t5_dn10)), ((locals.var_mu0__blk1191_dn11 * locals.var_t5) + (locals.var_mu0__blk1191 * locals.var_t5_dn11)), ((locals.var_mu0__blk1191_dn12 * locals.var_t5) + (locals.var_mu0__blk1191 * locals.var_t5_dn12)), ((locals.var_mu0__blk1191_dn17 * locals.var_t5) + (locals.var_mu0__blk1191 * locals.var_t5_dn17)),)
    } else {
        (locals.var_mu__blk1195, locals.var_mu__blk1195_dn0, locals.var_mu__blk1195_dn2, locals.var_mu__blk1195_dn6, locals.var_mu__blk1195_dn7, locals.var_mu__blk1195_dn10, locals.var_mu__blk1195_dn11, locals.var_mu__blk1195_dn12, locals.var_mu__blk1195_dn17,)
    }
};
        locals.var_mu__blk1195 = assign36200_e50958;
        locals.var_mu__blk1195_dn0 = assign36200_e50958_d_n0;
        locals.var_mu__blk1195_dn2 = assign36200_e50958_d_n2;
        locals.var_mu__blk1195_dn6 = assign36200_e50958_d_n6;
        locals.var_mu__blk1195_dn7 = assign36200_e50958_d_n7;
        locals.var_mu__blk1195_dn10 = assign36200_e50958_d_n10;
        locals.var_mu__blk1195_dn11 = assign36200_e50958_d_n11;
        locals.var_mu__blk1195_dn12 = assign36200_e50958_d_n12;
        locals.var_mu__blk1195_dn17 = assign36200_e50958_d_n17;

        let (assign36210_e50964, assign36210_e50964_d_n0, assign36210_e50964_d_n2, assign36210_e50964_d_n6, assign36210_e50964_d_n7, assign36210_e50964_d_n10, assign36210_e50964_d_n11, assign36210_e50964_d_n12, assign36210_e50964_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36210_e50962: f64 = (1.6021918e-19 / locals.var_ldrifte__blk1189);
        (assign36210_e50962, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign36210_e50964;
        locals.var_t1_dn0 = assign36210_e50964_d_n0;
        locals.var_t1_dn2 = assign36210_e50964_d_n2;
        locals.var_t1_dn6 = assign36210_e50964_d_n6;
        locals.var_t1_dn7 = assign36210_e50964_d_n7;
        locals.var_t1_dn10 = assign36210_e50964_d_n10;
        locals.var_t1_dn11 = assign36210_e50964_d_n11;
        locals.var_t1_dn12 = assign36210_e50964_d_n12;
        locals.var_t1_dn17 = assign36210_e50964_d_n17;

        let (assign36220_e50974, assign36220_e50974_d_n0, assign36220_e50974_d_n2, assign36220_e50974_d_n6, assign36220_e50974_d_n7, assign36220_e50974_d_n10, assign36220_e50974_d_n11, assign36220_e50974_d_n12, assign36220_e50974_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36220_e50968: f64 = (locals.var_t1 * locals.var_xov__blk1196);
        let assign36220_e50970: f64 = (assign36220_e50968 * locals.var_mu__blk1195);
        let assign36220_e50972: f64 = (assign36220_e50970 * locals.var_nover__blk1190);
        (assign36220_e50972, ((((locals.var_t1_dn0 * locals.var_xov__blk1196) * locals.var_mu__blk1195) + (assign36220_e50968 * locals.var_mu__blk1195_dn0)) * locals.var_nover__blk1190), ((((locals.var_t1_dn2 * locals.var_xov__blk1196) * locals.var_mu__blk1195) + (assign36220_e50968 * locals.var_mu__blk1195_dn2)) * locals.var_nover__blk1190), ((((locals.var_t1_dn6 * locals.var_xov__blk1196) * locals.var_mu__blk1195) + (assign36220_e50968 * locals.var_mu__blk1195_dn6)) * locals.var_nover__blk1190), ((((locals.var_t1_dn7 * locals.var_xov__blk1196) * locals.var_mu__blk1195) + (assign36220_e50968 * locals.var_mu__blk1195_dn7)) * locals.var_nover__blk1190), ((((locals.var_t1_dn10 * locals.var_xov__blk1196) * locals.var_mu__blk1195) + (assign36220_e50968 * locals.var_mu__blk1195_dn10)) * locals.var_nover__blk1190), ((((locals.var_t1_dn11 * locals.var_xov__blk1196) * locals.var_mu__blk1195) + (assign36220_e50968 * locals.var_mu__blk1195_dn11)) * locals.var_nover__blk1190), ((((locals.var_t1_dn12 * locals.var_xov__blk1196) * locals.var_mu__blk1195) + (assign36220_e50968 * locals.var_mu__blk1195_dn12)) * locals.var_nover__blk1190), ((((locals.var_t1_dn17 * locals.var_xov__blk1196) * locals.var_mu__blk1195) + (assign36220_e50968 * locals.var_mu__blk1195_dn17)) * locals.var_nover__blk1190),)
    } else {
        (locals.var_gd__blk1197, locals.var_gd__blk1197_dn0, locals.var_gd__blk1197_dn2, locals.var_gd__blk1197_dn6, locals.var_gd__blk1197_dn7, locals.var_gd__blk1197_dn10, locals.var_gd__blk1197_dn11, locals.var_gd__blk1197_dn12, locals.var_gd__blk1197_dn17,)
    }
};
        locals.var_gd__blk1197 = assign36220_e50974;
        locals.var_gd__blk1197_dn0 = assign36220_e50974_d_n0;
        locals.var_gd__blk1197_dn2 = assign36220_e50974_d_n2;
        locals.var_gd__blk1197_dn6 = assign36220_e50974_d_n6;
        locals.var_gd__blk1197_dn7 = assign36220_e50974_d_n7;
        locals.var_gd__blk1197_dn10 = assign36220_e50974_d_n10;
        locals.var_gd__blk1197_dn11 = assign36220_e50974_d_n11;
        locals.var_gd__blk1197_dn12 = assign36220_e50974_d_n12;
        locals.var_gd__blk1197_dn17 = assign36220_e50974_d_n17;

        let assign36230_e50977: f64 = if locals.var_gd__blk1197 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign36230_e50977;

        let (assign36240_e50983, assign36240_e50983_d_n0, assign36240_e50983_d_n2, assign36240_e50983_d_n6, assign36240_e50983_d_n7, assign36240_e50983_d_n10, assign36240_e50983_d_n11, assign36240_e50983_d_n12, assign36240_e50983_d_n17,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1205 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gd__blk1197, locals.var_gd__blk1197_dn0, locals.var_gd__blk1197_dn2, locals.var_gd__blk1197_dn6, locals.var_gd__blk1197_dn7, locals.var_gd__blk1197_dn10, locals.var_gd__blk1197_dn11, locals.var_gd__blk1197_dn12, locals.var_gd__blk1197_dn17,)
    }
};
        locals.var_gd__blk1197 = assign36240_e50983;
        locals.var_gd__blk1197_dn0 = assign36240_e50983_d_n0;
        locals.var_gd__blk1197_dn2 = assign36240_e50983_d_n2;
        locals.var_gd__blk1197_dn6 = assign36240_e50983_d_n6;
        locals.var_gd__blk1197_dn7 = assign36240_e50983_d_n7;
        locals.var_gd__blk1197_dn10 = assign36240_e50983_d_n10;
        locals.var_gd__blk1197_dn11 = assign36240_e50983_d_n11;
        locals.var_gd__blk1197_dn12 = assign36240_e50983_d_n12;
        locals.var_gd__blk1197_dn17 = assign36240_e50983_d_n17;

        let (assign36250_e50989, assign36250_e50989_d_n0, assign36250_e50989_d_n2, assign36250_e50989_d_n6, assign36250_e50989_d_n7, assign36250_e50989_d_n10, assign36250_e50989_d_n11, assign36250_e50989_d_n12, assign36250_e50989_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36250_e50987: f64 = (1.0 / locals.var_gd__blk1197);
        (assign36250_e50987, (-(locals.var_gd__blk1197_dn0 / (locals.var_gd__blk1197 * locals.var_gd__blk1197))), (-(locals.var_gd__blk1197_dn2 / (locals.var_gd__blk1197 * locals.var_gd__blk1197))), (-(locals.var_gd__blk1197_dn6 / (locals.var_gd__blk1197 * locals.var_gd__blk1197))), (-(locals.var_gd__blk1197_dn7 / (locals.var_gd__blk1197 * locals.var_gd__blk1197))), (-(locals.var_gd__blk1197_dn10 / (locals.var_gd__blk1197 * locals.var_gd__blk1197))), (-(locals.var_gd__blk1197_dn11 / (locals.var_gd__blk1197 * locals.var_gd__blk1197))), (-(locals.var_gd__blk1197_dn12 / (locals.var_gd__blk1197 * locals.var_gd__blk1197))), (-(locals.var_gd__blk1197_dn17 / (locals.var_gd__blk1197 * locals.var_gd__blk1197))),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn12, locals.var_rsd_dn17,)
    }
};
        locals.var_rsd = assign36250_e50989;
        locals.var_rsd_dn0 = assign36250_e50989_d_n0;
        locals.var_rsd_dn2 = assign36250_e50989_d_n2;
        locals.var_rsd_dn6 = assign36250_e50989_d_n6;
        locals.var_rsd_dn7 = assign36250_e50989_d_n7;
        locals.var_rsd_dn10 = assign36250_e50989_d_n10;
        locals.var_rsd_dn11 = assign36250_e50989_d_n11;
        locals.var_rsd_dn12 = assign36250_e50989_d_n12;
        locals.var_rsd_dn17 = assign36250_e50989_d_n17;

        let (assign36260_e50995, assign36260_e50995_d_n0, assign36260_e50995_d_n2, assign36260_e50995_d_n6, assign36260_e50995_d_n7, assign36260_e50995_d_n10, assign36260_e50995_d_n11, assign36260_e50995_d_n12, assign36260_e50995_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36260_e50993: f64 = (locals.var_rsd / locals.var_weff_nf__blk1198);
        (assign36260_e50993, (locals.var_rsd_dn0 / locals.var_weff_nf__blk1198), (locals.var_rsd_dn2 / locals.var_weff_nf__blk1198), (locals.var_rsd_dn6 / locals.var_weff_nf__blk1198), (locals.var_rsd_dn7 / locals.var_weff_nf__blk1198), (locals.var_rsd_dn10 / locals.var_weff_nf__blk1198), (locals.var_rsd_dn11 / locals.var_weff_nf__blk1198), (locals.var_rsd_dn12 / locals.var_weff_nf__blk1198), (locals.var_rsd_dn17 / locals.var_weff_nf__blk1198),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn12, locals.var_rsd_dn17,)
    }
};
        locals.var_rsd = assign36260_e50995;
        locals.var_rsd_dn0 = assign36260_e50995_d_n0;
        locals.var_rsd_dn2 = assign36260_e50995_d_n2;
        locals.var_rsd_dn6 = assign36260_e50995_d_n6;
        locals.var_rsd_dn7 = assign36260_e50995_d_n7;
        locals.var_rsd_dn10 = assign36260_e50995_d_n10;
        locals.var_rsd_dn11 = assign36260_e50995_d_n11;
        locals.var_rsd_dn12 = assign36260_e50995_d_n12;
        locals.var_rsd_dn17 = assign36260_e50995_d_n17;

        let (assign36270_e51001, assign36270_e51001_d_n0, assign36270_e51001_d_n2, assign36270_e51001_d_n6, assign36270_e51001_d_n7, assign36270_e51001_d_n10, assign36270_e51001_d_n11, assign36270_e51001_d_n12, assign36270_e51001_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36270_e50999: f64 = (locals.var_rsd + locals.var_rsd0__blk1186);
        (assign36270_e50999, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn12, locals.var_rsd_dn17,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn12, locals.var_rsd_dn17,)
    }
};
        locals.var_rsd = assign36270_e51001;
        locals.var_rsd_dn0 = assign36270_e51001_d_n0;
        locals.var_rsd_dn2 = assign36270_e51001_d_n2;
        locals.var_rsd_dn6 = assign36270_e51001_d_n6;
        locals.var_rsd_dn7 = assign36270_e51001_d_n7;
        locals.var_rsd_dn10 = assign36270_e51001_d_n10;
        locals.var_rsd_dn11 = assign36270_e51001_d_n11;
        locals.var_rsd_dn12 = assign36270_e51001_d_n12;
        locals.var_rsd_dn17 = assign36270_e51001_d_n17;

        let assign36290_e51019: f64 = if locals.var_rsd < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign36290_e51019;

        let (assign36300_e51025, assign36300_e51025_d_n0, assign36300_e51025_d_n2, assign36300_e51025_d_n6, assign36300_e51025_d_n7, assign36300_e51025_d_n10, assign36300_e51025_d_n11, assign36300_e51025_d_n12, assign36300_e51025_d_n17,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1206 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn12, locals.var_rsd_dn17,)
    }
};
        locals.var_rsd = assign36300_e51025;
        locals.var_rsd_dn0 = assign36300_e51025_d_n0;
        locals.var_rsd_dn2 = assign36300_e51025_d_n2;
        locals.var_rsd_dn6 = assign36300_e51025_d_n6;
        locals.var_rsd_dn7 = assign36300_e51025_d_n7;
        locals.var_rsd_dn10 = assign36300_e51025_d_n10;
        locals.var_rsd_dn11 = assign36300_e51025_d_n11;
        locals.var_rsd_dn12 = assign36300_e51025_d_n12;
        locals.var_rsd_dn17 = assign36300_e51025_d_n17;

        let (assign36310_e51031, assign36310_e51031_d_n0, assign36310_e51031_d_n2, assign36310_e51031_d_n6, assign36310_e51031_d_n7, assign36310_e51031_d_n10, assign36310_e51031_d_n11, assign36310_e51031_d_n12, assign36310_e51031_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36310_e51029: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign36310_e51029, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn12 / locals.var_mfactor), (locals.var_rsd_dn17 / locals.var_mfactor),)
    } else {
        (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn12, locals.var_rdde_dn17,)
    }
};
        locals.var_rdde = assign36310_e51031;
        locals.var_rdde_dn0 = assign36310_e51031_d_n0;
        locals.var_rdde_dn2 = assign36310_e51031_d_n2;
        locals.var_rdde_dn6 = assign36310_e51031_d_n6;
        locals.var_rdde_dn7 = assign36310_e51031_d_n7;
        locals.var_rdde_dn10 = assign36310_e51031_d_n10;
        locals.var_rdde_dn11 = assign36310_e51031_d_n11;
        locals.var_rdde_dn12 = assign36310_e51031_d_n12;
        locals.var_rdde_dn17 = assign36310_e51031_d_n17;

        let assign36330_e51038: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign36330_e51038;

        let assign36360_e51057: f64 = (1e-15 / 0.0001);
        let assign36360_e51058: f64 = if locals.var_taub < assign36360_e51057 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign36360_e51058;

        let (assign36370_e51068, assign36370_e51068_d_n0, assign36370_e51068_d_n2, assign36370_e51068_d_n6, assign36370_e51068_d_n7, assign36370_e51068_d_n10, assign36370_e51068_d_n11, assign36370_e51068_d_n12, assign36370_e51068_d_n17,) = {
    if (((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) && (locals.var_guard1209 != 0.0)) {
        let assign36370_e51066: f64 = (1e-15 / 0.0001);
        (assign36370_e51066, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_taub, locals.var_taub_dn0, locals.var_taub_dn2, locals.var_taub_dn6, locals.var_taub_dn7, locals.var_taub_dn10, locals.var_taub_dn11, locals.var_taub_dn12, locals.var_taub_dn17,)
    }
};
        locals.var_taub = assign36370_e51068;
        locals.var_taub_dn0 = assign36370_e51068_d_n0;
        locals.var_taub_dn2 = assign36370_e51068_d_n2;
        locals.var_taub_dn6 = assign36370_e51068_d_n6;
        locals.var_taub_dn7 = assign36370_e51068_d_n7;
        locals.var_taub_dn10 = assign36370_e51068_d_n10;
        locals.var_taub_dn11 = assign36370_e51068_d_n11;
        locals.var_taub_dn12 = assign36370_e51068_d_n12;
        locals.var_taub_dn17 = assign36370_e51068_d_n17;

        let (assign36380_e51081, assign36380_e51081_d_n0, assign36380_e51081_d_n2, assign36380_e51081_d_n6, assign36380_e51081_d_n7, assign36380_e51081_d_n10, assign36380_e51081_d_n11, assign36380_e51081_d_n12, assign36380_e51081_d_n17,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let (assign36380_e51079, assign36380_e51079_d_n0, assign36380_e51079_d_n2, assign36380_e51079_d_n6, assign36380_e51079_d_n7, assign36380_e51079_d_n10, assign36380_e51079_d_n11, assign36380_e51079_d_n12, assign36380_e51079_d_n17,) = {
            if (locals.var_mode == 1.0) {
                (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
            } else {
                let assign36380_e51078: f64 = (1.0 - locals.var_xd);
                (assign36380_e51078, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn12), (-locals.var_xd_dn17),)
            }
        };
        (assign36380_e51079, assign36380_e51079_d_n0, assign36380_e51079_d_n2, assign36380_e51079_d_n6, assign36380_e51079_d_n7, assign36380_e51079_d_n10, assign36380_e51079_d_n11, assign36380_e51079_d_n12, assign36380_e51079_d_n17,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign36380_e51081;
        locals.var_qdrat_dn0 = assign36380_e51081_d_n0;
        locals.var_qdrat_dn2 = assign36380_e51081_d_n2;
        locals.var_qdrat_dn6 = assign36380_e51081_d_n6;
        locals.var_qdrat_dn7 = assign36380_e51081_d_n7;
        locals.var_qdrat_dn10 = assign36380_e51081_d_n10;
        locals.var_qdrat_dn11 = assign36380_e51081_d_n11;
        locals.var_qdrat_dn12 = assign36380_e51081_d_n12;
        locals.var_qdrat_dn17 = assign36380_e51081_d_n17;

        let (assign36400_e51101, assign36400_e51101_d_n0, assign36400_e51101_d_n2, assign36400_e51101_d_n6, assign36400_e51101_d_n7, assign36400_e51101_d_n10, assign36400_e51101_d_n11, assign36400_e51101_d_n12, assign36400_e51101_d_n13, assign36400_e51101_d_n15, assign36400_e51101_d_n16, assign36400_e51101_d_n17, assign36400_e51101_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36400_e51097: f64 = (locals.var_qb_nqs - locals.var_qb_qs);
        let assign36400_e51099: f64 = (assign36400_e51097 / locals.var_taub);
        (assign36400_e51099, ((((-locals.var_qb_qs_dn0) * locals.var_taub) - (assign36400_e51097 * locals.var_taub_dn0)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn2) * locals.var_taub) - (assign36400_e51097 * locals.var_taub_dn2)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn6) * locals.var_taub) - (assign36400_e51097 * locals.var_taub_dn6)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn7) * locals.var_taub) - (assign36400_e51097 * locals.var_taub_dn7)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn10) * locals.var_taub) - (assign36400_e51097 * locals.var_taub_dn10)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn11) * locals.var_taub) - (assign36400_e51097 * locals.var_taub_dn11)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn12) * locals.var_taub) - (assign36400_e51097 * locals.var_taub_dn12)) / (locals.var_taub * locals.var_taub)), ((locals.var_qb_nqs_dn13 - locals.var_qb_qs_dn13) / locals.var_taub), ((-locals.var_qb_qs_dn15) / locals.var_taub), ((-locals.var_qb_qs_dn16) / locals.var_taub), ((((-locals.var_qb_qs_dn17) * locals.var_taub) - (assign36400_e51097 * locals.var_taub_dn17)) / (locals.var_taub * locals.var_taub)), ((-locals.var_qb_qs_dn18) / locals.var_taub),)
    } else {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn15, locals.var_iqb_nqs_dn16, locals.var_iqb_nqs_dn17, locals.var_iqb_nqs_dn18,)
    }
};
        locals.var_iqb_nqs = assign36400_e51101;
        locals.var_iqb_nqs_dn0 = assign36400_e51101_d_n0;
        locals.var_iqb_nqs_dn2 = assign36400_e51101_d_n2;
        locals.var_iqb_nqs_dn6 = assign36400_e51101_d_n6;
        locals.var_iqb_nqs_dn7 = assign36400_e51101_d_n7;
        locals.var_iqb_nqs_dn10 = assign36400_e51101_d_n10;
        locals.var_iqb_nqs_dn11 = assign36400_e51101_d_n11;
        locals.var_iqb_nqs_dn12 = assign36400_e51101_d_n12;
        locals.var_iqb_nqs_dn13 = assign36400_e51101_d_n13;
        locals.var_iqb_nqs_dn15 = assign36400_e51101_d_n15;
        locals.var_iqb_nqs_dn16 = assign36400_e51101_d_n16;
        locals.var_iqb_nqs_dn17 = assign36400_e51101_d_n17;
        locals.var_iqb_nqs_dn18 = assign36400_e51101_d_n18;

        let (assign36410_e51111, assign36410_e51111_d_n0, assign36410_e51111_d_n2, assign36410_e51111_d_n6, assign36410_e51111_d_n7, assign36410_e51111_d_n10, assign36410_e51111_d_n11, assign36410_e51111_d_n12, assign36410_e51111_d_n15, assign36410_e51111_d_n17, assign36410_e51111_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36410_e51107: f64 = (locals.var_qi_nqs * locals.var_qdrat);
        let assign36410_e51109: f64 = (assign36410_e51107 + locals.var_q_bt_se);
        (assign36410_e51109, ((locals.var_qi_nqs * locals.var_qdrat_dn0) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * locals.var_qdrat_dn2) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * locals.var_qdrat_dn6) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * locals.var_qdrat_dn7) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * locals.var_qdrat_dn10) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * locals.var_qdrat_dn11) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * locals.var_qdrat_dn12) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * locals.var_qdrat_dn17) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * locals.var_qdrat),)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign36410_e51111;
        locals.var_qd_nqs_dn0 = assign36410_e51111_d_n0;
        locals.var_qd_nqs_dn2 = assign36410_e51111_d_n2;
        locals.var_qd_nqs_dn6 = assign36410_e51111_d_n6;
        locals.var_qd_nqs_dn7 = assign36410_e51111_d_n7;
        locals.var_qd_nqs_dn10 = assign36410_e51111_d_n10;
        locals.var_qd_nqs_dn11 = assign36410_e51111_d_n11;
        locals.var_qd_nqs_dn12 = assign36410_e51111_d_n12;
        locals.var_qd_nqs_dn15 = assign36410_e51111_d_n15;
        locals.var_qd_nqs_dn17 = assign36410_e51111_d_n17;
        locals.var_qd_nqs_dn18 = assign36410_e51111_d_n18;

        let (assign36420_e51123, assign36420_e51123_d_n0, assign36420_e51123_d_n2, assign36420_e51123_d_n6, assign36420_e51123_d_n7, assign36420_e51123_d_n10, assign36420_e51123_d_n11, assign36420_e51123_d_n12, assign36420_e51123_d_n16, assign36420_e51123_d_n17, assign36420_e51123_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36420_e51118: f64 = (1.0 - locals.var_qdrat);
        let assign36420_e51119: f64 = (locals.var_qi_nqs * assign36420_e51118);
        let assign36420_e51121: f64 = (assign36420_e51119 + locals.var_q_bt_se);
        (assign36420_e51121, ((locals.var_qi_nqs * (-locals.var_qdrat_dn0)) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * (-locals.var_qdrat_dn2)) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * (-locals.var_qdrat_dn6)) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * (-locals.var_qdrat_dn7)) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * (-locals.var_qdrat_dn10)) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * (-locals.var_qdrat_dn11)) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * (-locals.var_qdrat_dn12)) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * (-locals.var_qdrat_dn17)) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * assign36420_e51118),)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36420_e51123;
        locals.var_qs_nqs_dn0 = assign36420_e51123_d_n0;
        locals.var_qs_nqs_dn2 = assign36420_e51123_d_n2;
        locals.var_qs_nqs_dn6 = assign36420_e51123_d_n6;
        locals.var_qs_nqs_dn7 = assign36420_e51123_d_n7;
        locals.var_qs_nqs_dn10 = assign36420_e51123_d_n10;
        locals.var_qs_nqs_dn11 = assign36420_e51123_d_n11;
        locals.var_qs_nqs_dn12 = assign36420_e51123_d_n12;
        locals.var_qs_nqs_dn16 = assign36420_e51123_d_n16;
        locals.var_qs_nqs_dn17 = assign36420_e51123_d_n17;
        locals.var_qs_nqs_dn18 = assign36420_e51123_d_n18;

        let (assign36430_e51134, assign36430_e51134_d_n0, assign36430_e51134_d_n2, assign36430_e51134_d_n6, assign36430_e51134_d_n7, assign36430_e51134_d_n10, assign36430_e51134_d_n11, assign36430_e51134_d_n12, assign36430_e51134_d_n13, assign36430_e51134_d_n15, assign36430_e51134_d_n16, assign36430_e51134_d_n17, assign36430_e51134_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36430_e51128: f64 = (-locals.var_qi_nqs);
        let assign36430_e51130: f64 = (assign36430_e51128 - locals.var_qb_nqs);
        let assign36430_e51132: f64 = (assign36430_e51130 + locals.var_q_bt_ge);
        (assign36430_e51132, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, (-locals.var_qb_nqs_dn13), 0.0, 0.0, locals.var_q_bt_ge_dn17, (-locals.var_qi_nqs_dn18),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36430_e51134;
        locals.var_qg_nqs_dn0 = assign36430_e51134_d_n0;
        locals.var_qg_nqs_dn2 = assign36430_e51134_d_n2;
        locals.var_qg_nqs_dn6 = assign36430_e51134_d_n6;
        locals.var_qg_nqs_dn7 = assign36430_e51134_d_n7;
        locals.var_qg_nqs_dn10 = assign36430_e51134_d_n10;
        locals.var_qg_nqs_dn11 = assign36430_e51134_d_n11;
        locals.var_qg_nqs_dn12 = assign36430_e51134_d_n12;
        locals.var_qg_nqs_dn13 = assign36430_e51134_d_n13;
        locals.var_qg_nqs_dn15 = assign36430_e51134_d_n15;
        locals.var_qg_nqs_dn16 = assign36430_e51134_d_n16;
        locals.var_qg_nqs_dn17 = assign36430_e51134_d_n17;
        locals.var_qg_nqs_dn18 = assign36430_e51134_d_n18;

        let (assign36450_e51148, assign36450_e51148_d_n0, assign36450_e51148_d_n2, assign36450_e51148_d_n6, assign36450_e51148_d_n7, assign36450_e51148_d_n10, assign36450_e51148_d_n11, assign36450_e51148_d_n12, assign36450_e51148_d_n13, assign36450_e51148_d_n15, assign36450_e51148_d_n16, assign36450_e51148_d_n17, assign36450_e51148_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn15, locals.var_iqb_nqs_dn16, locals.var_iqb_nqs_dn17, locals.var_iqb_nqs_dn18,)
    }
};
        locals.var_iqb_nqs = assign36450_e51148;
        locals.var_iqb_nqs_dn0 = assign36450_e51148_d_n0;
        locals.var_iqb_nqs_dn2 = assign36450_e51148_d_n2;
        locals.var_iqb_nqs_dn6 = assign36450_e51148_d_n6;
        locals.var_iqb_nqs_dn7 = assign36450_e51148_d_n7;
        locals.var_iqb_nqs_dn10 = assign36450_e51148_d_n10;
        locals.var_iqb_nqs_dn11 = assign36450_e51148_d_n11;
        locals.var_iqb_nqs_dn12 = assign36450_e51148_d_n12;
        locals.var_iqb_nqs_dn13 = assign36450_e51148_d_n13;
        locals.var_iqb_nqs_dn15 = assign36450_e51148_d_n15;
        locals.var_iqb_nqs_dn16 = assign36450_e51148_d_n16;
        locals.var_iqb_nqs_dn17 = assign36450_e51148_d_n17;
        locals.var_iqb_nqs_dn18 = assign36450_e51148_d_n18;

        let (assign36460_e51155, assign36460_e51155_d_n0, assign36460_e51155_d_n2, assign36460_e51155_d_n6, assign36460_e51155_d_n7, assign36460_e51155_d_n10, assign36460_e51155_d_n11, assign36460_e51155_d_n12, assign36460_e51155_d_n15, assign36460_e51155_d_n17, assign36460_e51155_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign36460_e51155;
        locals.var_qd_nqs_dn0 = assign36460_e51155_d_n0;
        locals.var_qd_nqs_dn2 = assign36460_e51155_d_n2;
        locals.var_qd_nqs_dn6 = assign36460_e51155_d_n6;
        locals.var_qd_nqs_dn7 = assign36460_e51155_d_n7;
        locals.var_qd_nqs_dn10 = assign36460_e51155_d_n10;
        locals.var_qd_nqs_dn11 = assign36460_e51155_d_n11;
        locals.var_qd_nqs_dn12 = assign36460_e51155_d_n12;
        locals.var_qd_nqs_dn15 = assign36460_e51155_d_n15;
        locals.var_qd_nqs_dn17 = assign36460_e51155_d_n17;
        locals.var_qd_nqs_dn18 = assign36460_e51155_d_n18;

    }

    pub(super) fn stamp_transient_block_126(
        locals: &mut StampLocals,
    ) {
        let (assign36470_e51162, assign36470_e51162_d_n0, assign36470_e51162_d_n2, assign36470_e51162_d_n6, assign36470_e51162_d_n7, assign36470_e51162_d_n10, assign36470_e51162_d_n11, assign36470_e51162_d_n12, assign36470_e51162_d_n16, assign36470_e51162_d_n17, assign36470_e51162_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36470_e51162;
        locals.var_qs_nqs_dn0 = assign36470_e51162_d_n0;
        locals.var_qs_nqs_dn2 = assign36470_e51162_d_n2;
        locals.var_qs_nqs_dn6 = assign36470_e51162_d_n6;
        locals.var_qs_nqs_dn7 = assign36470_e51162_d_n7;
        locals.var_qs_nqs_dn10 = assign36470_e51162_d_n10;
        locals.var_qs_nqs_dn11 = assign36470_e51162_d_n11;
        locals.var_qs_nqs_dn12 = assign36470_e51162_d_n12;
        locals.var_qs_nqs_dn16 = assign36470_e51162_d_n16;
        locals.var_qs_nqs_dn17 = assign36470_e51162_d_n17;
        locals.var_qs_nqs_dn18 = assign36470_e51162_d_n18;

        let (assign36480_e51169, assign36480_e51169_d_n0, assign36480_e51169_d_n2, assign36480_e51169_d_n6, assign36480_e51169_d_n7, assign36480_e51169_d_n10, assign36480_e51169_d_n11, assign36480_e51169_d_n12, assign36480_e51169_d_n13, assign36480_e51169_d_n15, assign36480_e51169_d_n16, assign36480_e51169_d_n17, assign36480_e51169_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36480_e51169;
        locals.var_qg_nqs_dn0 = assign36480_e51169_d_n0;
        locals.var_qg_nqs_dn2 = assign36480_e51169_d_n2;
        locals.var_qg_nqs_dn6 = assign36480_e51169_d_n6;
        locals.var_qg_nqs_dn7 = assign36480_e51169_d_n7;
        locals.var_qg_nqs_dn10 = assign36480_e51169_d_n10;
        locals.var_qg_nqs_dn11 = assign36480_e51169_d_n11;
        locals.var_qg_nqs_dn12 = assign36480_e51169_d_n12;
        locals.var_qg_nqs_dn13 = assign36480_e51169_d_n13;
        locals.var_qg_nqs_dn15 = assign36480_e51169_d_n15;
        locals.var_qg_nqs_dn16 = assign36480_e51169_d_n16;
        locals.var_qg_nqs_dn17 = assign36480_e51169_d_n17;
        locals.var_qg_nqs_dn18 = assign36480_e51169_d_n18;

        let (assign36490_e51176, assign36490_e51176_d_n13,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign36490_e51176;
        locals.var_qb_nqs_dn13 = assign36490_e51176_d_n13;

        let assign36520_e51196: f64 = (1e-15 / 0.0001);
        let assign36520_e51197: f64 = if locals.var_taub < assign36520_e51196 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign36520_e51197;

        let (assign36530_e51208, assign36530_e51208_d_n0, assign36530_e51208_d_n2, assign36530_e51208_d_n6, assign36530_e51208_d_n7, assign36530_e51208_d_n10, assign36530_e51208_d_n11, assign36530_e51208_d_n12, assign36530_e51208_d_n17,) = {
    if (((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs != 0.0)) && (locals.var_guard1211 != 0.0)) {
        let assign36530_e51206: f64 = (1e-15 / 0.0001);
        (assign36530_e51206, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_taub, locals.var_taub_dn0, locals.var_taub_dn2, locals.var_taub_dn6, locals.var_taub_dn7, locals.var_taub_dn10, locals.var_taub_dn11, locals.var_taub_dn12, locals.var_taub_dn17,)
    }
};
        locals.var_taub = assign36530_e51208;
        locals.var_taub_dn0 = assign36530_e51208_d_n0;
        locals.var_taub_dn2 = assign36530_e51208_d_n2;
        locals.var_taub_dn6 = assign36530_e51208_d_n6;
        locals.var_taub_dn7 = assign36530_e51208_d_n7;
        locals.var_taub_dn10 = assign36530_e51208_d_n10;
        locals.var_taub_dn11 = assign36530_e51208_d_n11;
        locals.var_taub_dn12 = assign36530_e51208_d_n12;
        locals.var_taub_dn17 = assign36530_e51208_d_n17;

        let (assign36560_e51241, assign36560_e51241_d_n0, assign36560_e51241_d_n2, assign36560_e51241_d_n6, assign36560_e51241_d_n7, assign36560_e51241_d_n10, assign36560_e51241_d_n11, assign36560_e51241_d_n12, assign36560_e51241_d_n13, assign36560_e51241_d_n15, assign36560_e51241_d_n16, assign36560_e51241_d_n17, assign36560_e51241_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36560_e51237: f64 = (locals.var_qb_nqs - locals.var_qb_qs);
        let assign36560_e51239: f64 = (assign36560_e51237 / locals.var_taub);
        (assign36560_e51239, ((((-locals.var_qb_qs_dn0) * locals.var_taub) - (assign36560_e51237 * locals.var_taub_dn0)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn2) * locals.var_taub) - (assign36560_e51237 * locals.var_taub_dn2)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn6) * locals.var_taub) - (assign36560_e51237 * locals.var_taub_dn6)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn7) * locals.var_taub) - (assign36560_e51237 * locals.var_taub_dn7)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn10) * locals.var_taub) - (assign36560_e51237 * locals.var_taub_dn10)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn11) * locals.var_taub) - (assign36560_e51237 * locals.var_taub_dn11)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn12) * locals.var_taub) - (assign36560_e51237 * locals.var_taub_dn12)) / (locals.var_taub * locals.var_taub)), ((locals.var_qb_nqs_dn13 - locals.var_qb_qs_dn13) / locals.var_taub), ((-locals.var_qb_qs_dn15) / locals.var_taub), ((-locals.var_qb_qs_dn16) / locals.var_taub), ((((-locals.var_qb_qs_dn17) * locals.var_taub) - (assign36560_e51237 * locals.var_taub_dn17)) / (locals.var_taub * locals.var_taub)), ((-locals.var_qb_qs_dn18) / locals.var_taub),)
    } else {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn15, locals.var_iqb_nqs_dn16, locals.var_iqb_nqs_dn17, locals.var_iqb_nqs_dn18,)
    }
};
        locals.var_iqb_nqs = assign36560_e51241;
        locals.var_iqb_nqs_dn0 = assign36560_e51241_d_n0;
        locals.var_iqb_nqs_dn2 = assign36560_e51241_d_n2;
        locals.var_iqb_nqs_dn6 = assign36560_e51241_d_n6;
        locals.var_iqb_nqs_dn7 = assign36560_e51241_d_n7;
        locals.var_iqb_nqs_dn10 = assign36560_e51241_d_n10;
        locals.var_iqb_nqs_dn11 = assign36560_e51241_d_n11;
        locals.var_iqb_nqs_dn12 = assign36560_e51241_d_n12;
        locals.var_iqb_nqs_dn13 = assign36560_e51241_d_n13;
        locals.var_iqb_nqs_dn15 = assign36560_e51241_d_n15;
        locals.var_iqb_nqs_dn16 = assign36560_e51241_d_n16;
        locals.var_iqb_nqs_dn17 = assign36560_e51241_d_n17;
        locals.var_iqb_nqs_dn18 = assign36560_e51241_d_n18;

        let (assign36570_e51248, assign36570_e51248_d_n0, assign36570_e51248_d_n2, assign36570_e51248_d_n6, assign36570_e51248_d_n7, assign36570_e51248_d_n10, assign36570_e51248_d_n11, assign36570_e51248_d_n12, assign36570_e51248_d_n13, assign36570_e51248_d_n15, assign36570_e51248_d_n16, assign36570_e51248_d_n17, assign36570_e51248_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn15, locals.var_iqb_nqs_dn16, locals.var_iqb_nqs_dn17, locals.var_iqb_nqs_dn18,)
    }
};
        locals.var_iqb_nqs = assign36570_e51248;
        locals.var_iqb_nqs_dn0 = assign36570_e51248_d_n0;
        locals.var_iqb_nqs_dn2 = assign36570_e51248_d_n2;
        locals.var_iqb_nqs_dn6 = assign36570_e51248_d_n6;
        locals.var_iqb_nqs_dn7 = assign36570_e51248_d_n7;
        locals.var_iqb_nqs_dn10 = assign36570_e51248_d_n10;
        locals.var_iqb_nqs_dn11 = assign36570_e51248_d_n11;
        locals.var_iqb_nqs_dn12 = assign36570_e51248_d_n12;
        locals.var_iqb_nqs_dn13 = assign36570_e51248_d_n13;
        locals.var_iqb_nqs_dn15 = assign36570_e51248_d_n15;
        locals.var_iqb_nqs_dn16 = assign36570_e51248_d_n16;
        locals.var_iqb_nqs_dn17 = assign36570_e51248_d_n17;
        locals.var_iqb_nqs_dn18 = assign36570_e51248_d_n18;

        let (assign36580_e51260, assign36580_e51260_d_n0, assign36580_e51260_d_n2, assign36580_e51260_d_n6, assign36580_e51260_d_n7, assign36580_e51260_d_n10, assign36580_e51260_d_n11, assign36580_e51260_d_n12, assign36580_e51260_d_n13, assign36580_e51260_d_n15, assign36580_e51260_d_n16, assign36580_e51260_d_n17, assign36580_e51260_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36580_e51254: f64 = (-locals.var_qd_nqs);
        let assign36580_e51256: f64 = (assign36580_e51254 - locals.var_qs_nqs);
        let assign36580_e51258: f64 = (assign36580_e51256 - locals.var_qb_nqs);
        (assign36580_e51258, ((-locals.var_qd_nqs_dn0) - locals.var_qs_nqs_dn0), ((-locals.var_qd_nqs_dn2) - locals.var_qs_nqs_dn2), ((-locals.var_qd_nqs_dn6) - locals.var_qs_nqs_dn6), ((-locals.var_qd_nqs_dn7) - locals.var_qs_nqs_dn7), ((-locals.var_qd_nqs_dn10) - locals.var_qs_nqs_dn10), ((-locals.var_qd_nqs_dn11) - locals.var_qs_nqs_dn11), ((-locals.var_qd_nqs_dn12) - locals.var_qs_nqs_dn12), (-locals.var_qb_nqs_dn13), (-locals.var_qd_nqs_dn15), (-locals.var_qs_nqs_dn16), ((-locals.var_qd_nqs_dn17) - locals.var_qs_nqs_dn17), ((-locals.var_qd_nqs_dn18) - locals.var_qs_nqs_dn18),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36580_e51260;
        locals.var_qg_nqs_dn0 = assign36580_e51260_d_n0;
        locals.var_qg_nqs_dn2 = assign36580_e51260_d_n2;
        locals.var_qg_nqs_dn6 = assign36580_e51260_d_n6;
        locals.var_qg_nqs_dn7 = assign36580_e51260_d_n7;
        locals.var_qg_nqs_dn10 = assign36580_e51260_d_n10;
        locals.var_qg_nqs_dn11 = assign36580_e51260_d_n11;
        locals.var_qg_nqs_dn12 = assign36580_e51260_d_n12;
        locals.var_qg_nqs_dn13 = assign36580_e51260_d_n13;
        locals.var_qg_nqs_dn15 = assign36580_e51260_d_n15;
        locals.var_qg_nqs_dn16 = assign36580_e51260_d_n16;
        locals.var_qg_nqs_dn17 = assign36580_e51260_d_n17;
        locals.var_qg_nqs_dn18 = assign36580_e51260_d_n18;

        let (assign36610_e51284, assign36610_e51284_d_n0, assign36610_e51284_d_n2, assign36610_e51284_d_n6, assign36610_e51284_d_n7, assign36610_e51284_d_n10, assign36610_e51284_d_n11, assign36610_e51284_d_n12, assign36610_e51284_d_n13, assign36610_e51284_d_n15, assign36610_e51284_d_n16, assign36610_e51284_d_n17, assign36610_e51284_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn15, locals.var_iqb_nqs_dn16, locals.var_iqb_nqs_dn17, locals.var_iqb_nqs_dn18,)
    }
};
        locals.var_iqb_nqs = assign36610_e51284;
        locals.var_iqb_nqs_dn0 = assign36610_e51284_d_n0;
        locals.var_iqb_nqs_dn2 = assign36610_e51284_d_n2;
        locals.var_iqb_nqs_dn6 = assign36610_e51284_d_n6;
        locals.var_iqb_nqs_dn7 = assign36610_e51284_d_n7;
        locals.var_iqb_nqs_dn10 = assign36610_e51284_d_n10;
        locals.var_iqb_nqs_dn11 = assign36610_e51284_d_n11;
        locals.var_iqb_nqs_dn12 = assign36610_e51284_d_n12;
        locals.var_iqb_nqs_dn13 = assign36610_e51284_d_n13;
        locals.var_iqb_nqs_dn15 = assign36610_e51284_d_n15;
        locals.var_iqb_nqs_dn16 = assign36610_e51284_d_n16;
        locals.var_iqb_nqs_dn17 = assign36610_e51284_d_n17;
        locals.var_iqb_nqs_dn18 = assign36610_e51284_d_n18;

        let (assign36620_e51292, assign36620_e51292_d_n0, assign36620_e51292_d_n2, assign36620_e51292_d_n6, assign36620_e51292_d_n7, assign36620_e51292_d_n10, assign36620_e51292_d_n11, assign36620_e51292_d_n12, assign36620_e51292_d_n15, assign36620_e51292_d_n17, assign36620_e51292_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign36620_e51292;
        locals.var_qd_nqs_dn0 = assign36620_e51292_d_n0;
        locals.var_qd_nqs_dn2 = assign36620_e51292_d_n2;
        locals.var_qd_nqs_dn6 = assign36620_e51292_d_n6;
        locals.var_qd_nqs_dn7 = assign36620_e51292_d_n7;
        locals.var_qd_nqs_dn10 = assign36620_e51292_d_n10;
        locals.var_qd_nqs_dn11 = assign36620_e51292_d_n11;
        locals.var_qd_nqs_dn12 = assign36620_e51292_d_n12;
        locals.var_qd_nqs_dn15 = assign36620_e51292_d_n15;
        locals.var_qd_nqs_dn17 = assign36620_e51292_d_n17;
        locals.var_qd_nqs_dn18 = assign36620_e51292_d_n18;

        let (assign36630_e51300, assign36630_e51300_d_n0, assign36630_e51300_d_n2, assign36630_e51300_d_n6, assign36630_e51300_d_n7, assign36630_e51300_d_n10, assign36630_e51300_d_n11, assign36630_e51300_d_n12, assign36630_e51300_d_n16, assign36630_e51300_d_n17, assign36630_e51300_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36630_e51300;
        locals.var_qs_nqs_dn0 = assign36630_e51300_d_n0;
        locals.var_qs_nqs_dn2 = assign36630_e51300_d_n2;
        locals.var_qs_nqs_dn6 = assign36630_e51300_d_n6;
        locals.var_qs_nqs_dn7 = assign36630_e51300_d_n7;
        locals.var_qs_nqs_dn10 = assign36630_e51300_d_n10;
        locals.var_qs_nqs_dn11 = assign36630_e51300_d_n11;
        locals.var_qs_nqs_dn12 = assign36630_e51300_d_n12;
        locals.var_qs_nqs_dn16 = assign36630_e51300_d_n16;
        locals.var_qs_nqs_dn17 = assign36630_e51300_d_n17;
        locals.var_qs_nqs_dn18 = assign36630_e51300_d_n18;

        let (assign36640_e51308, assign36640_e51308_d_n0, assign36640_e51308_d_n2, assign36640_e51308_d_n6, assign36640_e51308_d_n7, assign36640_e51308_d_n10, assign36640_e51308_d_n11, assign36640_e51308_d_n12, assign36640_e51308_d_n13, assign36640_e51308_d_n15, assign36640_e51308_d_n16, assign36640_e51308_d_n17, assign36640_e51308_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36640_e51308;
        locals.var_qg_nqs_dn0 = assign36640_e51308_d_n0;
        locals.var_qg_nqs_dn2 = assign36640_e51308_d_n2;
        locals.var_qg_nqs_dn6 = assign36640_e51308_d_n6;
        locals.var_qg_nqs_dn7 = assign36640_e51308_d_n7;
        locals.var_qg_nqs_dn10 = assign36640_e51308_d_n10;
        locals.var_qg_nqs_dn11 = assign36640_e51308_d_n11;
        locals.var_qg_nqs_dn12 = assign36640_e51308_d_n12;
        locals.var_qg_nqs_dn13 = assign36640_e51308_d_n13;
        locals.var_qg_nqs_dn15 = assign36640_e51308_d_n15;
        locals.var_qg_nqs_dn16 = assign36640_e51308_d_n16;
        locals.var_qg_nqs_dn17 = assign36640_e51308_d_n17;
        locals.var_qg_nqs_dn18 = assign36640_e51308_d_n18;

        let (assign36650_e51316, assign36650_e51316_d_n13,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign36650_e51316;
        locals.var_qb_nqs_dn13 = assign36650_e51316_d_n13;

        locals.var_rdd = locals.var_rdde;
        locals.var_rdd_dn0 = locals.var_rdde_dn0;
        locals.var_rdd_dn2 = locals.var_rdde_dn2;
        locals.var_rdd_dn6 = locals.var_rdde_dn6;
        locals.var_rdd_dn7 = locals.var_rdde_dn7;
        locals.var_rdd_dn10 = locals.var_rdde_dn10;
        locals.var_rdd_dn11 = locals.var_rdde_dn11;
        locals.var_rdd_dn12 = locals.var_rdde_dn12;
        locals.var_rdd_dn17 = locals.var_rdde_dn17;

        locals.var_rsd = locals.var_rsde;
        locals.var_rsd_dn0 = locals.var_rsde_dn0;
        locals.var_rsd_dn2 = locals.var_rsde_dn2;
        locals.var_rsd_dn6 = locals.var_rsde_dn6;
        locals.var_rsd_dn7 = locals.var_rsde_dn7;
        locals.var_rsd_dn10 = locals.var_rsde_dn10;
        locals.var_rsd_dn11 = locals.var_rsde_dn11;
        locals.var_rsd_dn12 = locals.var_rsde_dn12;
        locals.var_rsd_dn17 = locals.var_rsde_dn17;

        let assign36680_e51321: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign36680_e51321;

        let (assign36690_e51325, assign36690_e51325_d_n0, assign36690_e51325_d_n2, assign36690_e51325_d_n6, assign36690_e51325_d_n7, assign36690_e51325_d_n10, assign36690_e51325_d_n11, assign36690_e51325_d_n12, assign36690_e51325_d_n17,) = {
    if (locals.var_guard1212 != 0.0) {
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign36690_e51325;
        locals.var_ids_dn0 = assign36690_e51325_d_n0;
        locals.var_ids_dn2 = assign36690_e51325_d_n2;
        locals.var_ids_dn6 = assign36690_e51325_d_n6;
        locals.var_ids_dn7 = assign36690_e51325_d_n7;
        locals.var_ids_dn10 = assign36690_e51325_d_n10;
        locals.var_ids_dn11 = assign36690_e51325_d_n11;
        locals.var_ids_dn12 = assign36690_e51325_d_n12;
        locals.var_ids_dn17 = assign36690_e51325_d_n17;

        let (assign36700_e51329, assign36700_e51329_d_n0, assign36700_e51329_d_n2, assign36700_e51329_d_n6, assign36700_e51329_d_n7, assign36700_e51329_d_n10, assign36700_e51329_d_n11, assign36700_e51329_d_n12, assign36700_e51329_d_n17,) = {
    if (locals.var_guard1212 != 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign36700_e51329;
        locals.var_isub_dn0 = assign36700_e51329_d_n0;
        locals.var_isub_dn2 = assign36700_e51329_d_n2;
        locals.var_isub_dn6 = assign36700_e51329_d_n6;
        locals.var_isub_dn7 = assign36700_e51329_d_n7;
        locals.var_isub_dn10 = assign36700_e51329_d_n10;
        locals.var_isub_dn11 = assign36700_e51329_d_n11;
        locals.var_isub_dn12 = assign36700_e51329_d_n12;
        locals.var_isub_dn17 = assign36700_e51329_d_n17;

        let (assign36710_e51333, assign36710_e51333_d_n0, assign36710_e51333_d_n2, assign36710_e51333_d_n6, assign36710_e51333_d_n7, assign36710_e51333_d_n10, assign36710_e51333_d_n11, assign36710_e51333_d_n12, assign36710_e51333_d_n17,) = {
    if (locals.var_guard1212 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isubs, locals.var_isubs_dn0, locals.var_isubs_dn2, locals.var_isubs_dn6, locals.var_isubs_dn7, locals.var_isubs_dn10, locals.var_isubs_dn11, locals.var_isubs_dn12, locals.var_isubs_dn17,)
    }
};
        locals.var_isubs = assign36710_e51333;
        locals.var_isubs_dn0 = assign36710_e51333_d_n0;
        locals.var_isubs_dn2 = assign36710_e51333_d_n2;
        locals.var_isubs_dn6 = assign36710_e51333_d_n6;
        locals.var_isubs_dn7 = assign36710_e51333_d_n7;
        locals.var_isubs_dn10 = assign36710_e51333_d_n10;
        locals.var_isubs_dn11 = assign36710_e51333_d_n11;
        locals.var_isubs_dn12 = assign36710_e51333_d_n12;
        locals.var_isubs_dn17 = assign36710_e51333_d_n17;

        let (assign36720_e51339, assign36720_e51339_d_n0, assign36720_e51339_d_n2, assign36720_e51339_d_n6, assign36720_e51339_d_n7, assign36720_e51339_d_n10, assign36720_e51339_d_n11, assign36720_e51339_d_n12, assign36720_e51339_d_n13, assign36720_e51339_d_n15, assign36720_e51339_d_n16, assign36720_e51339_d_n17, assign36720_e51339_d_n18,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign36720_e51337: f64 = (locals.var_qge + locals.var_qg_nqs);
        (assign36720_e51337, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18),)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18,)
    }
};
        locals.var_qg = assign36720_e51339;
        locals.var_qg_dn0 = assign36720_e51339_d_n0;
        locals.var_qg_dn2 = assign36720_e51339_d_n2;
        locals.var_qg_dn6 = assign36720_e51339_d_n6;
        locals.var_qg_dn7 = assign36720_e51339_d_n7;
        locals.var_qg_dn10 = assign36720_e51339_d_n10;
        locals.var_qg_dn11 = assign36720_e51339_d_n11;
        locals.var_qg_dn12 = assign36720_e51339_d_n12;
        locals.var_qg_dn13 = assign36720_e51339_d_n13;
        locals.var_qg_dn15 = assign36720_e51339_d_n15;
        locals.var_qg_dn16 = assign36720_e51339_d_n16;
        locals.var_qg_dn17 = assign36720_e51339_d_n17;
        locals.var_qg_dn18 = assign36720_e51339_d_n18;

        let (assign36730_e51345, assign36730_e51345_d_n0, assign36730_e51345_d_n2, assign36730_e51345_d_n6, assign36730_e51345_d_n7, assign36730_e51345_d_n10, assign36730_e51345_d_n11, assign36730_e51345_d_n12, assign36730_e51345_d_n13, assign36730_e51345_d_n15, assign36730_e51345_d_n16, assign36730_e51345_d_n17, assign36730_e51345_d_n18,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign36730_e51343: f64 = (locals.var_qde + locals.var_qd_nqs);
        (assign36730_e51343, (locals.var_qde_dn0 + locals.var_qd_nqs_dn0), (locals.var_qde_dn2 + locals.var_qd_nqs_dn2), (locals.var_qde_dn6 + locals.var_qd_nqs_dn6), (locals.var_qde_dn7 + locals.var_qd_nqs_dn7), (locals.var_qde_dn10 + locals.var_qd_nqs_dn10), (locals.var_qde_dn11 + locals.var_qd_nqs_dn11), (locals.var_qde_dn12 + locals.var_qd_nqs_dn12), locals.var_qde_dn13, (locals.var_qde_dn15 + locals.var_qd_nqs_dn15), locals.var_qde_dn16, (locals.var_qde_dn17 + locals.var_qd_nqs_dn17), (locals.var_qde_dn18 + locals.var_qd_nqs_dn18),)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign36730_e51345;
        locals.var_qd_dn0 = assign36730_e51345_d_n0;
        locals.var_qd_dn2 = assign36730_e51345_d_n2;
        locals.var_qd_dn6 = assign36730_e51345_d_n6;
        locals.var_qd_dn7 = assign36730_e51345_d_n7;
        locals.var_qd_dn10 = assign36730_e51345_d_n10;
        locals.var_qd_dn11 = assign36730_e51345_d_n11;
        locals.var_qd_dn12 = assign36730_e51345_d_n12;
        locals.var_qd_dn13 = assign36730_e51345_d_n13;
        locals.var_qd_dn15 = assign36730_e51345_d_n15;
        locals.var_qd_dn16 = assign36730_e51345_d_n16;
        locals.var_qd_dn17 = assign36730_e51345_d_n17;
        locals.var_qd_dn18 = assign36730_e51345_d_n18;

        let (assign36750_e51360, assign36750_e51360_d_n0, assign36750_e51360_d_n2, assign36750_e51360_d_n6, assign36750_e51360_d_n7, assign36750_e51360_d_n10, assign36750_e51360_d_n11, assign36750_e51360_d_n12, assign36750_e51360_d_n13, assign36750_e51360_d_n15, assign36750_e51360_d_n16, assign36750_e51360_d_n17, assign36750_e51360_d_n18,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign36750_e51355: f64 = (locals.var_qge + locals.var_qde);
        let assign36750_e51357: f64 = (assign36750_e51355 + locals.var_qse);
        let assign36750_e51358: f64 = (-assign36750_e51357);
        (assign36750_e51358, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign36750_e51360;
        locals.var_qbe_dn0 = assign36750_e51360_d_n0;
        locals.var_qbe_dn2 = assign36750_e51360_d_n2;
        locals.var_qbe_dn6 = assign36750_e51360_d_n6;
        locals.var_qbe_dn7 = assign36750_e51360_d_n7;
        locals.var_qbe_dn10 = assign36750_e51360_d_n10;
        locals.var_qbe_dn11 = assign36750_e51360_d_n11;
        locals.var_qbe_dn12 = assign36750_e51360_d_n12;
        locals.var_qbe_dn13 = assign36750_e51360_d_n13;
        locals.var_qbe_dn15 = assign36750_e51360_d_n15;
        locals.var_qbe_dn16 = assign36750_e51360_d_n16;
        locals.var_qbe_dn17 = assign36750_e51360_d_n17;
        locals.var_qbe_dn18 = assign36750_e51360_d_n18;

        let (assign36760_e51366, assign36760_e51366_d_n0, assign36760_e51366_d_n2, assign36760_e51366_d_n6, assign36760_e51366_d_n7, assign36760_e51366_d_n10, assign36760_e51366_d_n11, assign36760_e51366_d_n12, assign36760_e51366_d_n13, assign36760_e51366_d_n15, assign36760_e51366_d_n16, assign36760_e51366_d_n17, assign36760_e51366_d_n18,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign36760_e51364: f64 = (locals.var_qbe + locals.var_qb_nqs);
        (assign36760_e51364, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign36760_e51366;
        locals.var_qb_dn0 = assign36760_e51366_d_n0;
        locals.var_qb_dn2 = assign36760_e51366_d_n2;
        locals.var_qb_dn6 = assign36760_e51366_d_n6;
        locals.var_qb_dn7 = assign36760_e51366_d_n7;
        locals.var_qb_dn10 = assign36760_e51366_d_n10;
        locals.var_qb_dn11 = assign36760_e51366_d_n11;
        locals.var_qb_dn12 = assign36760_e51366_d_n12;
        locals.var_qb_dn13 = assign36760_e51366_d_n13;
        locals.var_qb_dn15 = assign36760_e51366_d_n15;
        locals.var_qb_dn16 = assign36760_e51366_d_n16;
        locals.var_qb_dn17 = assign36760_e51366_d_n17;
        locals.var_qb_dn18 = assign36760_e51366_d_n18;

        let (assign36770_e51372, assign36770_e51372_d_n0, assign36770_e51372_d_n2, assign36770_e51372_d_n6, assign36770_e51372_d_n7, assign36770_e51372_d_n10, assign36770_e51372_d_n11, assign36770_e51372_d_n12, assign36770_e51372_d_n17,) = {
    if (locals.var_guard1212 == 0.0) {
        let assign36770_e51370: f64 = (-locals.var_idse);
        (assign36770_e51370, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12), (-locals.var_idse_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign36770_e51372;
        locals.var_ids_dn0 = assign36770_e51372_d_n0;
        locals.var_ids_dn2 = assign36770_e51372_d_n2;
        locals.var_ids_dn6 = assign36770_e51372_d_n6;
        locals.var_ids_dn7 = assign36770_e51372_d_n7;
        locals.var_ids_dn10 = assign36770_e51372_d_n10;
        locals.var_ids_dn11 = assign36770_e51372_d_n11;
        locals.var_ids_dn12 = assign36770_e51372_d_n12;
        locals.var_ids_dn17 = assign36770_e51372_d_n17;

        let (assign36780_e51377, assign36780_e51377_d_n0, assign36780_e51377_d_n2, assign36780_e51377_d_n6, assign36780_e51377_d_n7, assign36780_e51377_d_n10, assign36780_e51377_d_n11, assign36780_e51377_d_n12, assign36780_e51377_d_n17,) = {
    if (locals.var_guard1212 == 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    } else {
        (locals.var_isubs, locals.var_isubs_dn0, locals.var_isubs_dn2, locals.var_isubs_dn6, locals.var_isubs_dn7, locals.var_isubs_dn10, locals.var_isubs_dn11, locals.var_isubs_dn12, locals.var_isubs_dn17,)
    }
};
        locals.var_isubs = assign36780_e51377;
        locals.var_isubs_dn0 = assign36780_e51377_d_n0;
        locals.var_isubs_dn2 = assign36780_e51377_d_n2;
        locals.var_isubs_dn6 = assign36780_e51377_d_n6;
        locals.var_isubs_dn7 = assign36780_e51377_d_n7;
        locals.var_isubs_dn10 = assign36780_e51377_d_n10;
        locals.var_isubs_dn11 = assign36780_e51377_d_n11;
        locals.var_isubs_dn12 = assign36780_e51377_d_n12;
        locals.var_isubs_dn17 = assign36780_e51377_d_n17;

        let (assign36790_e51382, assign36790_e51382_d_n0, assign36790_e51382_d_n2, assign36790_e51382_d_n6, assign36790_e51382_d_n7, assign36790_e51382_d_n10, assign36790_e51382_d_n11, assign36790_e51382_d_n12, assign36790_e51382_d_n17,) = {
    if (locals.var_guard1212 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign36790_e51382;
        locals.var_isub_dn0 = assign36790_e51382_d_n0;
        locals.var_isub_dn2 = assign36790_e51382_d_n2;
        locals.var_isub_dn6 = assign36790_e51382_d_n6;
        locals.var_isub_dn7 = assign36790_e51382_d_n7;
        locals.var_isub_dn10 = assign36790_e51382_d_n10;
        locals.var_isub_dn11 = assign36790_e51382_d_n11;
        locals.var_isub_dn12 = assign36790_e51382_d_n12;
        locals.var_isub_dn17 = assign36790_e51382_d_n17;

        let (assign36800_e51389, assign36800_e51389_d_n0, assign36800_e51389_d_n2, assign36800_e51389_d_n6, assign36800_e51389_d_n7, assign36800_e51389_d_n10, assign36800_e51389_d_n11, assign36800_e51389_d_n12, assign36800_e51389_d_n13, assign36800_e51389_d_n15, assign36800_e51389_d_n16, assign36800_e51389_d_n17, assign36800_e51389_d_n18,) = {
    if (locals.var_guard1212 == 0.0) {
        let assign36800_e51387: f64 = (locals.var_qge + locals.var_qg_nqs);
        (assign36800_e51387, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18),)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18,)
    }
};
        locals.var_qg = assign36800_e51389;
        locals.var_qg_dn0 = assign36800_e51389_d_n0;
        locals.var_qg_dn2 = assign36800_e51389_d_n2;
        locals.var_qg_dn6 = assign36800_e51389_d_n6;
        locals.var_qg_dn7 = assign36800_e51389_d_n7;
        locals.var_qg_dn10 = assign36800_e51389_d_n10;
        locals.var_qg_dn11 = assign36800_e51389_d_n11;
        locals.var_qg_dn12 = assign36800_e51389_d_n12;
        locals.var_qg_dn13 = assign36800_e51389_d_n13;
        locals.var_qg_dn15 = assign36800_e51389_d_n15;
        locals.var_qg_dn16 = assign36800_e51389_d_n16;
        locals.var_qg_dn17 = assign36800_e51389_d_n17;
        locals.var_qg_dn18 = assign36800_e51389_d_n18;

        let (assign36810_e51396, assign36810_e51396_d_n0, assign36810_e51396_d_n2, assign36810_e51396_d_n6, assign36810_e51396_d_n7, assign36810_e51396_d_n10, assign36810_e51396_d_n11, assign36810_e51396_d_n12, assign36810_e51396_d_n13, assign36810_e51396_d_n15, assign36810_e51396_d_n16, assign36810_e51396_d_n17, assign36810_e51396_d_n18,) = {
    if (locals.var_guard1212 == 0.0) {
        let assign36810_e51394: f64 = (locals.var_qse + locals.var_qs_nqs);
        (assign36810_e51394, (locals.var_qse_dn0 + locals.var_qs_nqs_dn0), (locals.var_qse_dn2 + locals.var_qs_nqs_dn2), (locals.var_qse_dn6 + locals.var_qs_nqs_dn6), (locals.var_qse_dn7 + locals.var_qs_nqs_dn7), (locals.var_qse_dn10 + locals.var_qs_nqs_dn10), (locals.var_qse_dn11 + locals.var_qs_nqs_dn11), (locals.var_qse_dn12 + locals.var_qs_nqs_dn12), locals.var_qse_dn13, locals.var_qse_dn15, (locals.var_qse_dn16 + locals.var_qs_nqs_dn16), (locals.var_qse_dn17 + locals.var_qs_nqs_dn17), (locals.var_qse_dn18 + locals.var_qs_nqs_dn18),)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign36810_e51396;
        locals.var_qd_dn0 = assign36810_e51396_d_n0;
        locals.var_qd_dn2 = assign36810_e51396_d_n2;
        locals.var_qd_dn6 = assign36810_e51396_d_n6;
        locals.var_qd_dn7 = assign36810_e51396_d_n7;
        locals.var_qd_dn10 = assign36810_e51396_d_n10;
        locals.var_qd_dn11 = assign36810_e51396_d_n11;
        locals.var_qd_dn12 = assign36810_e51396_d_n12;
        locals.var_qd_dn13 = assign36810_e51396_d_n13;
        locals.var_qd_dn15 = assign36810_e51396_d_n15;
        locals.var_qd_dn16 = assign36810_e51396_d_n16;
        locals.var_qd_dn17 = assign36810_e51396_d_n17;
        locals.var_qd_dn18 = assign36810_e51396_d_n18;

    }

    pub(super) fn stamp_transient_block_127(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36830_e51413, assign36830_e51413_d_n0, assign36830_e51413_d_n2, assign36830_e51413_d_n6, assign36830_e51413_d_n7, assign36830_e51413_d_n10, assign36830_e51413_d_n11, assign36830_e51413_d_n12, assign36830_e51413_d_n13, assign36830_e51413_d_n15, assign36830_e51413_d_n16, assign36830_e51413_d_n17, assign36830_e51413_d_n18,) = {
    if (locals.var_guard1212 == 0.0) {
        let assign36830_e51408: f64 = (locals.var_qge + locals.var_qde);
        let assign36830_e51410: f64 = (assign36830_e51408 + locals.var_qse);
        let assign36830_e51411: f64 = (-assign36830_e51410);
        (assign36830_e51411, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign36830_e51413;
        locals.var_qbe_dn0 = assign36830_e51413_d_n0;
        locals.var_qbe_dn2 = assign36830_e51413_d_n2;
        locals.var_qbe_dn6 = assign36830_e51413_d_n6;
        locals.var_qbe_dn7 = assign36830_e51413_d_n7;
        locals.var_qbe_dn10 = assign36830_e51413_d_n10;
        locals.var_qbe_dn11 = assign36830_e51413_d_n11;
        locals.var_qbe_dn12 = assign36830_e51413_d_n12;
        locals.var_qbe_dn13 = assign36830_e51413_d_n13;
        locals.var_qbe_dn15 = assign36830_e51413_d_n15;
        locals.var_qbe_dn16 = assign36830_e51413_d_n16;
        locals.var_qbe_dn17 = assign36830_e51413_d_n17;
        locals.var_qbe_dn18 = assign36830_e51413_d_n18;

        let (assign36840_e51420, assign36840_e51420_d_n0, assign36840_e51420_d_n2, assign36840_e51420_d_n6, assign36840_e51420_d_n7, assign36840_e51420_d_n10, assign36840_e51420_d_n11, assign36840_e51420_d_n12, assign36840_e51420_d_n13, assign36840_e51420_d_n15, assign36840_e51420_d_n16, assign36840_e51420_d_n17, assign36840_e51420_d_n18,) = {
    if (locals.var_guard1212 == 0.0) {
        let assign36840_e51418: f64 = (locals.var_qbe + locals.var_qb_nqs);
        (assign36840_e51418, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign36840_e51420;
        locals.var_qb_dn0 = assign36840_e51420_d_n0;
        locals.var_qb_dn2 = assign36840_e51420_d_n2;
        locals.var_qb_dn6 = assign36840_e51420_d_n6;
        locals.var_qb_dn7 = assign36840_e51420_d_n7;
        locals.var_qb_dn10 = assign36840_e51420_d_n10;
        locals.var_qb_dn11 = assign36840_e51420_d_n11;
        locals.var_qb_dn12 = assign36840_e51420_d_n12;
        locals.var_qb_dn13 = assign36840_e51420_d_n13;
        locals.var_qb_dn15 = assign36840_e51420_d_n15;
        locals.var_qb_dn16 = assign36840_e51420_d_n16;
        locals.var_qb_dn17 = assign36840_e51420_d_n17;
        locals.var_qb_dn18 = assign36840_e51420_d_n18;

        locals.var_igd = locals.var_igde;
        locals.var_igd_dn0 = locals.var_igde_dn0;
        locals.var_igd_dn2 = locals.var_igde_dn2;
        locals.var_igd_dn6 = locals.var_igde_dn6;
        locals.var_igd_dn7 = locals.var_igde_dn7;
        locals.var_igd_dn10 = locals.var_igde_dn10;
        locals.var_igd_dn11 = locals.var_igde_dn11;
        locals.var_igd_dn12 = locals.var_igde_dn12;
        locals.var_igd_dn17 = locals.var_igde_dn17;

        locals.var_igs = locals.var_igse;
        locals.var_igs_dn0 = locals.var_igse_dn0;
        locals.var_igs_dn2 = locals.var_igse_dn2;
        locals.var_igs_dn6 = locals.var_igse_dn6;
        locals.var_igs_dn7 = locals.var_igse_dn7;
        locals.var_igs_dn10 = locals.var_igse_dn10;
        locals.var_igs_dn11 = locals.var_igse_dn11;
        locals.var_igs_dn12 = locals.var_igse_dn12;
        locals.var_igs_dn17 = locals.var_igse_dn17;

        locals.var_igb = locals.var_igbe;
        locals.var_igb_dn0 = locals.var_igbe_dn0;
        locals.var_igb_dn2 = locals.var_igbe_dn2;
        locals.var_igb_dn6 = locals.var_igbe_dn6;
        locals.var_igb_dn7 = locals.var_igbe_dn7;
        locals.var_igb_dn10 = locals.var_igbe_dn10;
        locals.var_igb_dn11 = locals.var_igbe_dn11;
        locals.var_igb_dn12 = locals.var_igbe_dn12;
        locals.var_igb_dn17 = locals.var_igbe_dn17;

        locals.var_igidl = locals.var_igidle;
        locals.var_igidl_dn0 = locals.var_igidle_dn0;
        locals.var_igidl_dn2 = locals.var_igidle_dn2;
        locals.var_igidl_dn6 = locals.var_igidle_dn6;
        locals.var_igidl_dn7 = locals.var_igidle_dn7;
        locals.var_igidl_dn10 = locals.var_igidle_dn10;
        locals.var_igidl_dn11 = locals.var_igidle_dn11;
        locals.var_igidl_dn12 = locals.var_igidle_dn12;
        locals.var_igidl_dn17 = locals.var_igidle_dn17;

        locals.var_igisl = locals.var_igisle;
        locals.var_igisl_dn0 = locals.var_igisle_dn0;
        locals.var_igisl_dn2 = locals.var_igisle_dn2;
        locals.var_igisl_dn6 = locals.var_igisle_dn6;
        locals.var_igisl_dn7 = locals.var_igisle_dn7;
        locals.var_igisl_dn10 = locals.var_igisle_dn10;
        locals.var_igisl_dn11 = locals.var_igisle_dn11;
        locals.var_igisl_dn12 = locals.var_igisle_dn12;
        locals.var_igisl_dn17 = locals.var_igisle_dn17;

        let assign36900_e51428: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign36900_e51428;

        let (assign36910_e51432, assign36910_e51432_d_n0, assign36910_e51432_d_n2, assign36910_e51432_d_n6, assign36910_e51432_d_n7, assign36910_e51432_d_n10, assign36910_e51432_d_n11, assign36910_e51432_d_n12, assign36910_e51432_d_n17,) = {
    if (locals.var_guard1213 != 0.0) {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign36910_e51432;
        locals.var_ibd_dn0 = assign36910_e51432_d_n0;
        locals.var_ibd_dn2 = assign36910_e51432_d_n2;
        locals.var_ibd_dn6 = assign36910_e51432_d_n6;
        locals.var_ibd_dn7 = assign36910_e51432_d_n7;
        locals.var_ibd_dn10 = assign36910_e51432_d_n10;
        locals.var_ibd_dn11 = assign36910_e51432_d_n11;
        locals.var_ibd_dn12 = assign36910_e51432_d_n12;
        locals.var_ibd_dn17 = assign36910_e51432_d_n17;

        let (assign36920_e51436, assign36920_e51436_d_n0, assign36920_e51436_d_n2, assign36920_e51436_d_n6, assign36920_e51436_d_n7, assign36920_e51436_d_n10, assign36920_e51436_d_n11, assign36920_e51436_d_n12, assign36920_e51436_d_n17,) = {
    if (locals.var_guard1213 != 0.0) {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign36920_e51436;
        locals.var_qbd_dn0 = assign36920_e51436_d_n0;
        locals.var_qbd_dn2 = assign36920_e51436_d_n2;
        locals.var_qbd_dn6 = assign36920_e51436_d_n6;
        locals.var_qbd_dn7 = assign36920_e51436_d_n7;
        locals.var_qbd_dn10 = assign36920_e51436_d_n10;
        locals.var_qbd_dn11 = assign36920_e51436_d_n11;
        locals.var_qbd_dn12 = assign36920_e51436_d_n12;
        locals.var_qbd_dn17 = assign36920_e51436_d_n17;

        let (assign36930_e51440, assign36930_e51440_d_n0, assign36930_e51440_d_n2, assign36930_e51440_d_n6, assign36930_e51440_d_n7, assign36930_e51440_d_n10, assign36930_e51440_d_n11, assign36930_e51440_d_n12, assign36930_e51440_d_n17,) = {
    if (locals.var_guard1213 != 0.0) {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign36930_e51440;
        locals.var_ibs_dn0 = assign36930_e51440_d_n0;
        locals.var_ibs_dn2 = assign36930_e51440_d_n2;
        locals.var_ibs_dn6 = assign36930_e51440_d_n6;
        locals.var_ibs_dn7 = assign36930_e51440_d_n7;
        locals.var_ibs_dn10 = assign36930_e51440_d_n10;
        locals.var_ibs_dn11 = assign36930_e51440_d_n11;
        locals.var_ibs_dn12 = assign36930_e51440_d_n12;
        locals.var_ibs_dn17 = assign36930_e51440_d_n17;

        let (assign36940_e51444, assign36940_e51444_d_n0, assign36940_e51444_d_n2, assign36940_e51444_d_n6, assign36940_e51444_d_n7, assign36940_e51444_d_n10, assign36940_e51444_d_n11, assign36940_e51444_d_n12, assign36940_e51444_d_n17,) = {
    if (locals.var_guard1213 != 0.0) {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign36940_e51444;
        locals.var_qbs_dn0 = assign36940_e51444_d_n0;
        locals.var_qbs_dn2 = assign36940_e51444_d_n2;
        locals.var_qbs_dn6 = assign36940_e51444_d_n6;
        locals.var_qbs_dn7 = assign36940_e51444_d_n7;
        locals.var_qbs_dn10 = assign36940_e51444_d_n10;
        locals.var_qbs_dn11 = assign36940_e51444_d_n11;
        locals.var_qbs_dn12 = assign36940_e51444_d_n12;
        locals.var_qbs_dn17 = assign36940_e51444_d_n17;

        let assign36950_e51451: f64 = if ((p.p38 == 1.0) && (locals.var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign36950_e51451;

        let (assign36960_e51457, assign36960_e51457_d_n0, assign36960_e51457_d_n2, assign36960_e51457_d_n6, assign36960_e51457_d_n7, assign36960_e51457_d_n10, assign36960_e51457_d_n11, assign36960_e51457_d_n12, assign36960_e51457_d_n17,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign36960_e51455: f64 = (locals.var_ids * locals.var_vds);
        (assign36960_e51455, ((locals.var_ids_dn0 * locals.var_vds) + (locals.var_ids * locals.var_vds_dn0)), ((locals.var_ids_dn2 * locals.var_vds) + (locals.var_ids * locals.var_vds_dn2)), ((locals.var_ids_dn6 * locals.var_vds) + (locals.var_ids * locals.var_vds_dn6)), ((locals.var_ids_dn7 * locals.var_vds) + (locals.var_ids * locals.var_vds_dn7)), ((locals.var_ids_dn10 * locals.var_vds) + (locals.var_ids * locals.var_vds_dn10)), ((locals.var_ids_dn11 * locals.var_vds) + (locals.var_ids * locals.var_vds_dn11)), ((locals.var_ids_dn12 * locals.var_vds) + (locals.var_ids * locals.var_vds_dn12)), ((locals.var_ids_dn17 * locals.var_vds) + (locals.var_ids * locals.var_vds_dn17)),)
    } else {
        (locals.var_rpower, locals.var_rpower_dn0, locals.var_rpower_dn2, locals.var_rpower_dn6, locals.var_rpower_dn7, locals.var_rpower_dn10, locals.var_rpower_dn11, locals.var_rpower_dn12, locals.var_rpower_dn17,)
    }
};
        locals.var_rpower = assign36960_e51457;
        locals.var_rpower_dn0 = assign36960_e51457_d_n0;
        locals.var_rpower_dn2 = assign36960_e51457_d_n2;
        locals.var_rpower_dn6 = assign36960_e51457_d_n6;
        locals.var_rpower_dn7 = assign36960_e51457_d_n7;
        locals.var_rpower_dn10 = assign36960_e51457_d_n10;
        locals.var_rpower_dn11 = assign36960_e51457_d_n11;
        locals.var_rpower_dn12 = assign36960_e51457_d_n12;
        locals.var_rpower_dn17 = assign36960_e51457_d_n17;

        let (assign36970_e51461,) = {
    if (locals.var_guard1214 != 0.0) {
        (locals.var_cth,)
    } else {
        (locals.var_cthe,)
    }
};
        locals.var_cthe = assign36970_e51461;

        let (assign36980_e51467,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign36980_e51465: f64 = (1.0 / locals.var_rth);
        (assign36980_e51465,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign36980_e51467;

        let (assign36990_e51472, assign36990_e51472_d_n0, assign36990_e51472_d_n2, assign36990_e51472_d_n6, assign36990_e51472_d_n7, assign36990_e51472_d_n10, assign36990_e51472_d_n11, assign36990_e51472_d_n12, assign36990_e51472_d_n17,) = {
    if (locals.var_guard1214 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rpower, locals.var_rpower_dn0, locals.var_rpower_dn2, locals.var_rpower_dn6, locals.var_rpower_dn7, locals.var_rpower_dn10, locals.var_rpower_dn11, locals.var_rpower_dn12, locals.var_rpower_dn17,)
    }
};
        locals.var_rpower = assign36990_e51472;
        locals.var_rpower_dn0 = assign36990_e51472_d_n0;
        locals.var_rpower_dn2 = assign36990_e51472_d_n2;
        locals.var_rpower_dn6 = assign36990_e51472_d_n6;
        locals.var_rpower_dn7 = assign36990_e51472_d_n7;
        locals.var_rpower_dn10 = assign36990_e51472_d_n10;
        locals.var_rpower_dn11 = assign36990_e51472_d_n11;
        locals.var_rpower_dn12 = assign36990_e51472_d_n12;
        locals.var_rpower_dn17 = assign36990_e51472_d_n17;

        let (assign37000_e51477,) = {
    if (locals.var_guard1214 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cthe,)
    }
};
        locals.var_cthe = assign37000_e51477;

        let (assign37010_e51482,) = {
    if (locals.var_guard1214 == 0.0) {
        (0.0,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign37010_e51482;

        locals.var_idse = locals.var_ids;
        locals.var_idse_dn0 = locals.var_ids_dn0;
        locals.var_idse_dn2 = locals.var_ids_dn2;
        locals.var_idse_dn6 = locals.var_ids_dn6;
        locals.var_idse_dn7 = locals.var_ids_dn7;
        locals.var_idse_dn10 = locals.var_ids_dn10;
        locals.var_idse_dn11 = locals.var_ids_dn11;
        locals.var_idse_dn12 = locals.var_ids_dn12;
        locals.var_idse_dn17 = locals.var_ids_dn17;

        let assign37170_e51531: f64 = locals.var_qg_dn6;
        locals.var_cgdbd = assign37170_e51531;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn12 = 0.0;
        locals.var_cgdbd_dn13 = 0.0;
        locals.var_cgdbd_dn15 = 0.0;
        locals.var_cgdbd_dn16 = 0.0;
        locals.var_cgdbd_dn17 = 0.0;
        locals.var_cgdbd_dn18 = 0.0;

        let assign37180_e51534: f64 = (p.p50 * locals.var_cgdbd);
        locals.var_cgdbd = assign37180_e51534;
        locals.var_cgdbd_dn0 = (p.p50 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p50 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn6 = (p.p50 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p50 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn10 = (p.p50 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p50 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn12 = (p.p50 * locals.var_cgdbd_dn12);
        locals.var_cgdbd_dn13 = (p.p50 * locals.var_cgdbd_dn13);
        locals.var_cgdbd_dn15 = (p.p50 * locals.var_cgdbd_dn15);
        locals.var_cgdbd_dn16 = (p.p50 * locals.var_cgdbd_dn16);
        locals.var_cgdbd_dn17 = (p.p50 * locals.var_cgdbd_dn17);
        locals.var_cgdbd_dn18 = (p.p50 * locals.var_cgdbd_dn18);

        let assign37190_e51537: f64 = locals.var_qg_dn7;
        locals.var_cgsbd = assign37190_e51537;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn12 = 0.0;
        locals.var_cgsbd_dn13 = 0.0;
        locals.var_cgsbd_dn15 = 0.0;
        locals.var_cgsbd_dn16 = 0.0;
        locals.var_cgsbd_dn17 = 0.0;
        locals.var_cgsbd_dn18 = 0.0;

        let assign37200_e51540: f64 = (p.p50 * locals.var_cgsbd);
        locals.var_cgsbd = assign37200_e51540;
        locals.var_cgsbd_dn0 = (p.p50 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p50 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn6 = (p.p50 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p50 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn10 = (p.p50 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p50 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn12 = (p.p50 * locals.var_cgsbd_dn12);
        locals.var_cgsbd_dn13 = (p.p50 * locals.var_cgsbd_dn13);
        locals.var_cgsbd_dn15 = (p.p50 * locals.var_cgsbd_dn15);
        locals.var_cgsbd_dn16 = (p.p50 * locals.var_cgsbd_dn16);
        locals.var_cgsbd_dn17 = (p.p50 * locals.var_cgsbd_dn17);
        locals.var_cgsbd_dn18 = (p.p50 * locals.var_cgsbd_dn18);

        let assign37470_e51621: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign37470_e51621;

        let (assign37480_e51627, assign37480_e51627_d_n0, assign37480_e51627_d_n2, assign37480_e51627_d_n6, assign37480_e51627_d_n7, assign37480_e51627_d_n10, assign37480_e51627_d_n11, assign37480_e51627_d_n12, assign37480_e51627_d_n17,) = {
    if (locals.var_guard1216 != 0.0) {
        let assign37480_e51625: f64 = (p.p50 * locals.var_ibd);
        (assign37480_e51625, (p.p50 * locals.var_ibd_dn0), (p.p50 * locals.var_ibd_dn2), (p.p50 * locals.var_ibd_dn6), (p.p50 * locals.var_ibd_dn7), (p.p50 * locals.var_ibd_dn10), (p.p50 * locals.var_ibd_dn11), (p.p50 * locals.var_ibd_dn12), (p.p50 * locals.var_ibd_dn17),)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign37480_e51627;
        locals.var_ibdb_dn0 = assign37480_e51627_d_n0;
        locals.var_ibdb_dn2 = assign37480_e51627_d_n2;
        locals.var_ibdb_dn6 = assign37480_e51627_d_n6;
        locals.var_ibdb_dn7 = assign37480_e51627_d_n7;
        locals.var_ibdb_dn10 = assign37480_e51627_d_n10;
        locals.var_ibdb_dn11 = assign37480_e51627_d_n11;
        locals.var_ibdb_dn12 = assign37480_e51627_d_n12;
        locals.var_ibdb_dn17 = assign37480_e51627_d_n17;

        let (assign37490_e51633, assign37490_e51633_d_n0, assign37490_e51633_d_n2, assign37490_e51633_d_n6, assign37490_e51633_d_n7, assign37490_e51633_d_n10, assign37490_e51633_d_n11, assign37490_e51633_d_n12, assign37490_e51633_d_n17,) = {
    if (locals.var_guard1216 != 0.0) {
        let assign37490_e51631: f64 = (p.p50 * locals.var_ibs);
        (assign37490_e51631, (p.p50 * locals.var_ibs_dn0), (p.p50 * locals.var_ibs_dn2), (p.p50 * locals.var_ibs_dn6), (p.p50 * locals.var_ibs_dn7), (p.p50 * locals.var_ibs_dn10), (p.p50 * locals.var_ibs_dn11), (p.p50 * locals.var_ibs_dn12), (p.p50 * locals.var_ibs_dn17),)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign37490_e51633;
        locals.var_ibsb_dn0 = assign37490_e51633_d_n0;
        locals.var_ibsb_dn2 = assign37490_e51633_d_n2;
        locals.var_ibsb_dn6 = assign37490_e51633_d_n6;
        locals.var_ibsb_dn7 = assign37490_e51633_d_n7;
        locals.var_ibsb_dn10 = assign37490_e51633_d_n10;
        locals.var_ibsb_dn11 = assign37490_e51633_d_n11;
        locals.var_ibsb_dn12 = assign37490_e51633_d_n12;
        locals.var_ibsb_dn17 = assign37490_e51633_d_n17;

        let assign37610_e51685: f64 = (4.0 * 1.3806226e-23);
        let assign37610_e51687: f64 = (assign37610_e51685 * locals.var_ttemp);
        let assign37610_e51689: f64 = assign37610_e51687;
        locals.var_whi_noise = assign37610_e51689;
        locals.var_whi_noise_dn10 = (assign37610_e51685 * locals.var_ttemp_dn10);

        let assign37620_e51692: f64 = if p.p27 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign37620_e51692;

        locals.var_qdrat = locals.var_qdrat_noi;
        locals.var_qdrat_dn0 = locals.var_qdrat_noi_dn0;
        locals.var_qdrat_dn2 = locals.var_qdrat_noi_dn2;
        locals.var_qdrat_dn6 = locals.var_qdrat_noi_dn6;
        locals.var_qdrat_dn7 = locals.var_qdrat_noi_dn7;
        locals.var_qdrat_dn10 = locals.var_qdrat_noi_dn10;
        locals.var_qdrat_dn11 = locals.var_qdrat_noi_dn11;
        locals.var_qdrat_dn12 = locals.var_qdrat_noi_dn12;
        locals.var_qdrat_dn17 = locals.var_qdrat_noi_dn17;

        let assign37640_e51696: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign37640_e51696;
        locals.var_sid_dn0 = (locals.var_whi_noise * locals.var_noithrml_dn0);
        locals.var_sid_dn2 = (locals.var_whi_noise * locals.var_noithrml_dn2);
        locals.var_sid_dn6 = (locals.var_whi_noise * locals.var_noithrml_dn6);
        locals.var_sid_dn7 = (locals.var_whi_noise * locals.var_noithrml_dn7);
        locals.var_sid_dn10 = ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10));
        locals.var_sid_dn11 = (locals.var_whi_noise * locals.var_noithrml_dn11);
        locals.var_sid_dn12 = (locals.var_whi_noise * locals.var_noithrml_dn12);
        locals.var_sid_dn17 = (locals.var_whi_noise * locals.var_noithrml_dn17);

        locals.var_ci = locals.var_noicross;
        locals.var_ci_dn0 = locals.var_noicross_dn0;
        locals.var_ci_dn2 = locals.var_noicross_dn2;
        locals.var_ci_dn6 = locals.var_noicross_dn6;
        locals.var_ci_dn7 = locals.var_noicross_dn7;
        locals.var_ci_dn10 = locals.var_noicross_dn10;
        locals.var_ci_dn11 = locals.var_noicross_dn11;
        locals.var_ci_dn12 = locals.var_noicross_dn12;
        locals.var_ci_dn17 = locals.var_noicross_dn17;

        let (assign37660_e51710, assign37660_e51710_d_n0, assign37660_e51710_d_n2, assign37660_e51710_d_n6, assign37660_e51710_d_n7, assign37660_e51710_d_n10, assign37660_e51710_d_n11, assign37660_e51710_d_n12, assign37660_e51710_d_n13, assign37660_e51710_d_n15, assign37660_e51710_d_n16, assign37660_e51710_d_n17, assign37660_e51710_d_n18,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign37660_e51707: f64 = (locals.var_noiigate / locals.var_sid);
        let assign37660_e51708: f64 = (assign37660_e51707).sqrt();
        (assign37660_e51708, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn12 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn12)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn13 / locals.var_sid) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn15 / locals.var_sid) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn16 / locals.var_sid) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn17 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn17)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn18 / locals.var_sid) / (2.0 * assign37660_e51708)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign37660_e51710;
        locals.var_sigrat_dn0 = assign37660_e51710_d_n0;
        locals.var_sigrat_dn2 = assign37660_e51710_d_n2;
        locals.var_sigrat_dn6 = assign37660_e51710_d_n6;
        locals.var_sigrat_dn7 = assign37660_e51710_d_n7;
        locals.var_sigrat_dn10 = assign37660_e51710_d_n10;
        locals.var_sigrat_dn11 = assign37660_e51710_d_n11;
        locals.var_sigrat_dn12 = assign37660_e51710_d_n12;
        locals.var_sigrat_dn13 = assign37660_e51710_d_n13;
        locals.var_sigrat_dn15 = assign37660_e51710_d_n15;
        locals.var_sigrat_dn16 = assign37660_e51710_d_n16;
        locals.var_sigrat_dn17 = assign37660_e51710_d_n17;
        locals.var_sigrat_dn18 = assign37660_e51710_d_n18;

        let (assign37670_e51722, assign37670_e51722_d_n0, assign37670_e51722_d_n2, assign37670_e51722_d_n6, assign37670_e51722_d_n7, assign37670_e51722_d_n10, assign37670_e51722_d_n11, assign37670_e51722_d_n12, assign37670_e51722_d_n13, assign37670_e51722_d_n15, assign37670_e51722_d_n16, assign37670_e51722_d_n17, assign37670_e51722_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37670_e51717: f64 = (1.0 - locals.var_qdrat);
        let assign37670_e51718: f64 = (locals.var_sigrat * assign37670_e51717);
        (assign37670_e51718, ((locals.var_sigrat_dn0 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37670_e51717), (locals.var_sigrat_dn15 * assign37670_e51717), (locals.var_sigrat_dn16 * assign37670_e51717), ((locals.var_sigrat_dn17 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37670_e51717),)
    } else {
        let assign37670_e51721: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37670_e51721, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    }
};
        locals.var_sigrat_s = assign37670_e51722;
        locals.var_sigrat_s_dn0 = assign37670_e51722_d_n0;
        locals.var_sigrat_s_dn2 = assign37670_e51722_d_n2;
        locals.var_sigrat_s_dn6 = assign37670_e51722_d_n6;
        locals.var_sigrat_s_dn7 = assign37670_e51722_d_n7;
        locals.var_sigrat_s_dn10 = assign37670_e51722_d_n10;
        locals.var_sigrat_s_dn11 = assign37670_e51722_d_n11;
        locals.var_sigrat_s_dn12 = assign37670_e51722_d_n12;
        locals.var_sigrat_s_dn13 = assign37670_e51722_d_n13;
        locals.var_sigrat_s_dn15 = assign37670_e51722_d_n15;
        locals.var_sigrat_s_dn16 = assign37670_e51722_d_n16;
        locals.var_sigrat_s_dn17 = assign37670_e51722_d_n17;
        locals.var_sigrat_s_dn18 = assign37670_e51722_d_n18;

        let (assign37680_e51734, assign37680_e51734_d_n0, assign37680_e51734_d_n2, assign37680_e51734_d_n6, assign37680_e51734_d_n7, assign37680_e51734_d_n10, assign37680_e51734_d_n11, assign37680_e51734_d_n12, assign37680_e51734_d_n13, assign37680_e51734_d_n15, assign37680_e51734_d_n16, assign37680_e51734_d_n17, assign37680_e51734_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37680_e51728: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37680_e51728, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    } else {
        let assign37680_e51732: f64 = (1.0 - locals.var_qdrat);
        let assign37680_e51733: f64 = (locals.var_sigrat * assign37680_e51732);
        (assign37680_e51733, ((locals.var_sigrat_dn0 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37680_e51732), (locals.var_sigrat_dn15 * assign37680_e51732), (locals.var_sigrat_dn16 * assign37680_e51732), ((locals.var_sigrat_dn17 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37680_e51732),)
    }
};
        locals.var_sigrat_d = assign37680_e51734;
        locals.var_sigrat_d_dn0 = assign37680_e51734_d_n0;
        locals.var_sigrat_d_dn2 = assign37680_e51734_d_n2;
        locals.var_sigrat_d_dn6 = assign37680_e51734_d_n6;
        locals.var_sigrat_d_dn7 = assign37680_e51734_d_n7;
        locals.var_sigrat_d_dn10 = assign37680_e51734_d_n10;
        locals.var_sigrat_d_dn11 = assign37680_e51734_d_n11;
        locals.var_sigrat_d_dn12 = assign37680_e51734_d_n12;
        locals.var_sigrat_d_dn13 = assign37680_e51734_d_n13;
        locals.var_sigrat_d_dn15 = assign37680_e51734_d_n15;
        locals.var_sigrat_d_dn16 = assign37680_e51734_d_n16;
        locals.var_sigrat_d_dn17 = assign37680_e51734_d_n17;
        locals.var_sigrat_d_dn18 = assign37680_e51734_d_n18;

        let assign37700_e51744: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign37700_e51744;

        let (assign37710_e51748, assign37710_e51748_d_n0, assign37710_e51748_d_n2, assign37710_e51748_d_n6, assign37710_e51748_d_n7, assign37710_e51748_d_n10, assign37710_e51748_d_n11, assign37710_e51748_d_n12, assign37710_e51748_d_n17,) = {
    if (locals.var_guard1224 != 0.0) {
        (locals.var_rpower, locals.var_rpower_dn0, locals.var_rpower_dn2, locals.var_rpower_dn6, locals.var_rpower_dn7, locals.var_rpower_dn10, locals.var_rpower_dn11, locals.var_rpower_dn12, locals.var_rpower_dn17,)
    } else {
        (locals.var_itemp, locals.var_itemp_dn0, locals.var_itemp_dn2, locals.var_itemp_dn6, locals.var_itemp_dn7, locals.var_itemp_dn10, locals.var_itemp_dn11, locals.var_itemp_dn12, locals.var_itemp_dn17,)
    }
};
        locals.var_itemp = assign37710_e51748;
        locals.var_itemp_dn0 = assign37710_e51748_d_n0;
        locals.var_itemp_dn2 = assign37710_e51748_d_n2;
        locals.var_itemp_dn6 = assign37710_e51748_d_n6;
        locals.var_itemp_dn7 = assign37710_e51748_d_n7;
        locals.var_itemp_dn10 = assign37710_e51748_d_n10;
        locals.var_itemp_dn11 = assign37710_e51748_d_n11;
        locals.var_itemp_dn12 = assign37710_e51748_d_n12;
        locals.var_itemp_dn17 = assign37710_e51748_d_n17;

        let assign37720_e51751: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign37720_e51751;

        let assign37730_e51760: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign37730_e51760;

    }
}
