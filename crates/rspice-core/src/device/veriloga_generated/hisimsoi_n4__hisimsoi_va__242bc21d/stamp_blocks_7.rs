#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_112(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign31370_e46173: f64 = if locals.var_chi__blk943 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1016 = assign31370_e46173;

        let (assign31410_e46235, assign31410_e46235_d_n0, assign31410_e46235_d_n2, assign31410_e46235_d_n6, assign31410_e46235_d_n7, assign31410_e46235_d_n10, assign31410_e46235_d_n11, assign31410_e46235_d_n12, assign31410_e46235_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 != 0.0)) {
        let assign31410_e46229: f64 = (locals.var_fb__blk967 * locals.var_fb__blk967);
        let assign31410_e46232: f64 = (10.0 * 2.220446049250313e-16);
        let assign31410_e46233: f64 = (assign31410_e46229 + assign31410_e46232);
        (assign31410_e46233, ((locals.var_fb__blk967_dn0 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn0)), ((locals.var_fb__blk967_dn2 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn2)), ((locals.var_fb__blk967_dn6 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn6)), ((locals.var_fb__blk967_dn7 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn7)), ((locals.var_fb__blk967_dn10 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn10)), ((locals.var_fb__blk967_dn11 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn11)), ((locals.var_fb__blk967_dn12 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn12)), ((locals.var_fb__blk967_dn17 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn17)),)
    } else {
        (locals.var_xi0__blk976, locals.var_xi0__blk976_dn0, locals.var_xi0__blk976_dn2, locals.var_xi0__blk976_dn6, locals.var_xi0__blk976_dn7, locals.var_xi0__blk976_dn10, locals.var_xi0__blk976_dn11, locals.var_xi0__blk976_dn12, locals.var_xi0__blk976_dn17,)
    }
};
        locals.var_xi0__blk976 = assign31410_e46235;
        locals.var_xi0__blk976_dn0 = assign31410_e46235_d_n0;
        locals.var_xi0__blk976_dn2 = assign31410_e46235_d_n2;
        locals.var_xi0__blk976_dn6 = assign31410_e46235_d_n6;
        locals.var_xi0__blk976_dn7 = assign31410_e46235_d_n7;
        locals.var_xi0__blk976_dn10 = assign31410_e46235_d_n10;
        locals.var_xi0__blk976_dn11 = assign31410_e46235_d_n11;
        locals.var_xi0__blk976_dn12 = assign31410_e46235_d_n12;
        locals.var_xi0__blk976_dn17 = assign31410_e46235_d_n17;

        let (assign31420_e46255, assign31420_e46255_d_n0, assign31420_e46255_d_n2, assign31420_e46255_d_n6, assign31420_e46255_d_n7, assign31420_e46255_d_n10, assign31420_e46255_d_n11, assign31420_e46255_d_n12, assign31420_e46255_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 != 0.0)) {
        let assign31420_e46252: f64 = (10.0 * 2.220446049250313e-16);
        let assign31420_e46253: f64 = (locals.var_fb__blk967 + assign31420_e46252);
        (assign31420_e46253, locals.var_fb__blk967_dn0, locals.var_fb__blk967_dn2, locals.var_fb__blk967_dn6, locals.var_fb__blk967_dn7, locals.var_fb__blk967_dn10, locals.var_fb__blk967_dn11, locals.var_fb__blk967_dn12, locals.var_fb__blk967_dn17,)
    } else {
        (locals.var_xi0p12__blk977, locals.var_xi0p12__blk977_dn0, locals.var_xi0p12__blk977_dn2, locals.var_xi0p12__blk977_dn6, locals.var_xi0p12__blk977_dn7, locals.var_xi0p12__blk977_dn10, locals.var_xi0p12__blk977_dn11, locals.var_xi0p12__blk977_dn12, locals.var_xi0p12__blk977_dn17,)
    }
};
        locals.var_xi0p12__blk977 = assign31420_e46255;
        locals.var_xi0p12__blk977_dn0 = assign31420_e46255_d_n0;
        locals.var_xi0p12__blk977_dn2 = assign31420_e46255_d_n2;
        locals.var_xi0p12__blk977_dn6 = assign31420_e46255_d_n6;
        locals.var_xi0p12__blk977_dn7 = assign31420_e46255_d_n7;
        locals.var_xi0p12__blk977_dn10 = assign31420_e46255_d_n10;
        locals.var_xi0p12__blk977_dn11 = assign31420_e46255_d_n11;
        locals.var_xi0p12__blk977_dn12 = assign31420_e46255_d_n12;
        locals.var_xi0p12__blk977_dn17 = assign31420_e46255_d_n17;

        let (assign31440_e46291, assign31440_e46291_d_n0, assign31440_e46291_d_n2, assign31440_e46291_d_n6, assign31440_e46291_d_n7, assign31440_e46291_d_n10, assign31440_e46291_d_n11, assign31440_e46291_d_n12, assign31440_e46291_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 == 0.0)) {
        let assign31440_e46289: f64 = (locals.var_chi__blk943 - 1.0);
        (assign31440_e46289, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    } else {
        (locals.var_xi0__blk976, locals.var_xi0__blk976_dn0, locals.var_xi0__blk976_dn2, locals.var_xi0__blk976_dn6, locals.var_xi0__blk976_dn7, locals.var_xi0__blk976_dn10, locals.var_xi0__blk976_dn11, locals.var_xi0__blk976_dn12, locals.var_xi0__blk976_dn17,)
    }
};
        locals.var_xi0__blk976 = assign31440_e46291;
        locals.var_xi0__blk976_dn0 = assign31440_e46291_d_n0;
        locals.var_xi0__blk976_dn2 = assign31440_e46291_d_n2;
        locals.var_xi0__blk976_dn6 = assign31440_e46291_d_n6;
        locals.var_xi0__blk976_dn7 = assign31440_e46291_d_n7;
        locals.var_xi0__blk976_dn10 = assign31440_e46291_d_n10;
        locals.var_xi0__blk976_dn11 = assign31440_e46291_d_n11;
        locals.var_xi0__blk976_dn12 = assign31440_e46291_d_n12;
        locals.var_xi0__blk976_dn17 = assign31440_e46291_d_n17;

        let (assign31450_e46309, assign31450_e46309_d_n0, assign31450_e46309_d_n2, assign31450_e46309_d_n6, assign31450_e46309_d_n7, assign31450_e46309_d_n10, assign31450_e46309_d_n11, assign31450_e46309_d_n12, assign31450_e46309_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 == 0.0)) {
        let assign31450_e46307: f64 = (locals.var_xi0__blk976).sqrt();
        (assign31450_e46307, (locals.var_xi0__blk976_dn0 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn2 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn6 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn7 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn10 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn11 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn12 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn17 / (2.0 * assign31450_e46307)),)
    } else {
        (locals.var_xi0p12__blk977, locals.var_xi0p12__blk977_dn0, locals.var_xi0p12__blk977_dn2, locals.var_xi0p12__blk977_dn6, locals.var_xi0p12__blk977_dn7, locals.var_xi0p12__blk977_dn10, locals.var_xi0p12__blk977_dn11, locals.var_xi0p12__blk977_dn12, locals.var_xi0p12__blk977_dn17,)
    }
};
        locals.var_xi0p12__blk977 = assign31450_e46309;
        locals.var_xi0p12__blk977_dn0 = assign31450_e46309_d_n0;
        locals.var_xi0p12__blk977_dn2 = assign31450_e46309_d_n2;
        locals.var_xi0p12__blk977_dn6 = assign31450_e46309_d_n6;
        locals.var_xi0p12__blk977_dn7 = assign31450_e46309_d_n7;
        locals.var_xi0p12__blk977_dn10 = assign31450_e46309_d_n10;
        locals.var_xi0p12__blk977_dn11 = assign31450_e46309_d_n11;
        locals.var_xi0p12__blk977_dn12 = assign31450_e46309_d_n12;
        locals.var_xi0p12__blk977_dn17 = assign31450_e46309_d_n17;

        let (assign31460_e46325, assign31460_e46325_d_n0, assign31460_e46325_d_n2, assign31460_e46325_d_n6, assign31460_e46325_d_n7, assign31460_e46325_d_n10, assign31460_e46325_d_n11, assign31460_e46325_d_n12, assign31460_e46325_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31460_e46323: f64 = (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977);
        (assign31460_e46323, ((locals.var_cnst0over__blk928_dn0 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn0)), ((locals.var_cnst0over__blk928_dn2 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn2)), ((locals.var_cnst0over__blk928_dn6 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn6)), ((locals.var_cnst0over__blk928_dn7 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn7)), ((locals.var_cnst0over__blk928_dn10 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn10)), ((locals.var_cnst0over__blk928_dn11 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn11)), ((locals.var_cnst0over__blk928_dn12 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn12)), ((locals.var_cnst0over__blk928_dn17 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign31460_e46325;
        locals.var_qbuld_dn0 = assign31460_e46325_d_n0;
        locals.var_qbuld_dn2 = assign31460_e46325_d_n2;
        locals.var_qbuld_dn6 = assign31460_e46325_d_n6;
        locals.var_qbuld_dn7 = assign31460_e46325_d_n7;
        locals.var_qbuld_dn10 = assign31460_e46325_d_n10;
        locals.var_qbuld_dn11 = assign31460_e46325_d_n11;
        locals.var_qbuld_dn12 = assign31460_e46325_d_n12;
        locals.var_qbuld_dn17 = assign31460_e46325_d_n17;

        let (assign31470_e46343, assign31470_e46343_d_n0, assign31470_e46343_d_n2, assign31470_e46343_d_n6, assign31470_e46343_d_n7, assign31470_e46343_d_n10, assign31470_e46343_d_n11, assign31470_e46343_d_n12, assign31470_e46343_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31470_e46340: f64 = (locals.var_fs02__blk969 + locals.var_xi0p12__blk977);
        let assign31470_e46341: f64 = (1.0 / assign31470_e46340);
        (assign31470_e46341, (-((locals.var_fs02__blk969_dn0 + locals.var_xi0p12__blk977_dn0) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn2 + locals.var_xi0p12__blk977_dn2) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn6 + locals.var_xi0p12__blk977_dn6) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn7 + locals.var_xi0p12__blk977_dn7) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn10 + locals.var_xi0p12__blk977_dn10) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn11 + locals.var_xi0p12__blk977_dn11) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn12 + locals.var_xi0p12__blk977_dn12) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn17 + locals.var_xi0p12__blk977_dn17) / (assign31470_e46340 * assign31470_e46340))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31470_e46343;
        locals.var_t1__blk896_dn0 = assign31470_e46343_d_n0;
        locals.var_t1__blk896_dn2 = assign31470_e46343_d_n2;
        locals.var_t1__blk896_dn6 = assign31470_e46343_d_n6;
        locals.var_t1__blk896_dn7 = assign31470_e46343_d_n7;
        locals.var_t1__blk896_dn10 = assign31470_e46343_d_n10;
        locals.var_t1__blk896_dn11 = assign31470_e46343_d_n11;
        locals.var_t1__blk896_dn12 = assign31470_e46343_d_n12;
        locals.var_t1__blk896_dn17 = assign31470_e46343_d_n17;

        let (assign31480_e46361, assign31480_e46361_d_n0, assign31480_e46361_d_n2, assign31480_e46361_d_n6, assign31480_e46361_d_n7, assign31480_e46361_d_n10, assign31480_e46361_d_n11, assign31480_e46361_d_n12, assign31480_e46361_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31480_e46357: f64 = (locals.var_cnst0over__blk928 * locals.var_fs01__blk965);
        let assign31480_e46359: f64 = (assign31480_e46357 * locals.var_t1__blk896);
        (assign31480_e46359, ((((locals.var_cnst0over__blk928_dn0 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn0)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn0)), ((((locals.var_cnst0over__blk928_dn2 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn2)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn2)), ((((locals.var_cnst0over__blk928_dn6 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn6)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn6)), ((((locals.var_cnst0over__blk928_dn7 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn7)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn7)), ((((locals.var_cnst0over__blk928_dn10 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn10)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn10)), ((((locals.var_cnst0over__blk928_dn11 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn11)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn11)), ((((locals.var_cnst0over__blk928_dn12 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn12)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn12)), ((((locals.var_cnst0over__blk928_dn17 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn17)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign31480_e46361;
        locals.var_qiuld_dn0 = assign31480_e46361_d_n0;
        locals.var_qiuld_dn2 = assign31480_e46361_d_n2;
        locals.var_qiuld_dn6 = assign31480_e46361_d_n6;
        locals.var_qiuld_dn7 = assign31480_e46361_d_n7;
        locals.var_qiuld_dn10 = assign31480_e46361_d_n10;
        locals.var_qiuld_dn11 = assign31480_e46361_d_n11;
        locals.var_qiuld_dn12 = assign31480_e46361_d_n12;
        locals.var_qiuld_dn17 = assign31480_e46361_d_n17;

        let (assign31490_e46377, assign31490_e46377_d_n0, assign31490_e46377_d_n2, assign31490_e46377_d_n6, assign31490_e46377_d_n7, assign31490_e46377_d_n10, assign31490_e46377_d_n11, assign31490_e46377_d_n12, assign31490_e46377_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31490_e46375: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign31490_e46375, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign31490_e46377;
        locals.var_qsuld_dn0 = assign31490_e46377_d_n0;
        locals.var_qsuld_dn2 = assign31490_e46377_d_n2;
        locals.var_qsuld_dn6 = assign31490_e46377_d_n6;
        locals.var_qsuld_dn7 = assign31490_e46377_d_n7;
        locals.var_qsuld_dn10 = assign31490_e46377_d_n10;
        locals.var_qsuld_dn11 = assign31490_e46377_d_n11;
        locals.var_qsuld_dn12 = assign31490_e46377_d_n12;
        locals.var_qsuld_dn17 = assign31490_e46377_d_n17;

        let (assign31500_e46388, assign31500_e46388_d_n0, assign31500_e46388_d_n2, assign31500_e46388_d_n6, assign31500_e46388_d_n7, assign31500_e46388_d_n10, assign31500_e46388_d_n11, assign31500_e46388_d_n12, assign31500_e46388_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign31500_e46386: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign31500_e46386, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign31500_e46388;
        locals.var_qiuld_dn0 = assign31500_e46388_d_n0;
        locals.var_qiuld_dn2 = assign31500_e46388_d_n2;
        locals.var_qiuld_dn6 = assign31500_e46388_d_n6;
        locals.var_qiuld_dn7 = assign31500_e46388_d_n7;
        locals.var_qiuld_dn10 = assign31500_e46388_d_n10;
        locals.var_qiuld_dn11 = assign31500_e46388_d_n11;
        locals.var_qiuld_dn12 = assign31500_e46388_d_n12;
        locals.var_qiuld_dn17 = assign31500_e46388_d_n17;

        let (assign31510_e46406, assign31510_e46406_d_n0, assign31510_e46406_d_n2, assign31510_e46406_d_n6, assign31510_e46406_d_n7, assign31510_e46406_d_n10, assign31510_e46406_d_n11, assign31510_e46406_d_n12, assign31510_e46406_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let (assign31510_e46404,) = {
            if (p.p43 == 1.0) {
                let assign31510_e46400: f64 = (locals.var_w_dioscv * locals.var_lov);
                (assign31510_e46400,)
            } else {
                let assign31510_e46403: f64 = (locals.var_weffcv_nf * locals.var_lov);
                (assign31510_e46403,)
            }
        };
        (assign31510_e46404, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk899, locals.var_t4__blk899_dn0, locals.var_t4__blk899_dn2, locals.var_t4__blk899_dn6, locals.var_t4__blk899_dn7, locals.var_t4__blk899_dn10, locals.var_t4__blk899_dn11, locals.var_t4__blk899_dn12, locals.var_t4__blk899_dn17,)
    }
};
        locals.var_t4__blk899 = assign31510_e46406;
        locals.var_t4__blk899_dn0 = assign31510_e46406_d_n0;
        locals.var_t4__blk899_dn2 = assign31510_e46406_d_n2;
        locals.var_t4__blk899_dn6 = assign31510_e46406_d_n6;
        locals.var_t4__blk899_dn7 = assign31510_e46406_d_n7;
        locals.var_t4__blk899_dn10 = assign31510_e46406_d_n10;
        locals.var_t4__blk899_dn11 = assign31510_e46406_d_n11;
        locals.var_t4__blk899_dn12 = assign31510_e46406_d_n12;
        locals.var_t4__blk899_dn17 = assign31510_e46406_d_n17;

        let assign31520_e46417: f64 = if (((locals.var_flg_overs__blk914 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloops__blk912 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1018 = assign31520_e46417;

        let (assign31530_e46430, assign31530_e46430_d_n0, assign31530_e46430_d_n2, assign31530_e46430_d_n6, assign31530_e46430_d_n7, assign31530_e46430_d_n10, assign31530_e46430_d_n11, assign31530_e46430_d_n12, assign31530_e46430_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1018 != 0.0)) {
        let assign31530_e46428: f64 = (locals.var_t4__blk899 * locals.var_qsuld);
        (assign31530_e46428, ((locals.var_t4__blk899_dn0 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn17,)
    }
};
        locals.var_qovs = assign31530_e46430;
        locals.var_qovs_dn0 = assign31530_e46430_d_n0;
        locals.var_qovs_dn2 = assign31530_e46430_d_n2;
        locals.var_qovs_dn6 = assign31530_e46430_d_n6;
        locals.var_qovs_dn7 = assign31530_e46430_d_n7;
        locals.var_qovs_dn10 = assign31530_e46430_d_n10;
        locals.var_qovs_dn11 = assign31530_e46430_d_n11;
        locals.var_qovs_dn12 = assign31530_e46430_d_n12;
        locals.var_qovs_dn17 = assign31530_e46430_d_n17;

        let (assign31540_e46443, assign31540_e46443_d_n0, assign31540_e46443_d_n2, assign31540_e46443_d_n6, assign31540_e46443_d_n7, assign31540_e46443_d_n10, assign31540_e46443_d_n11, assign31540_e46443_d_n12, assign31540_e46443_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1018 != 0.0)) {
        let assign31540_e46441: f64 = (locals.var_t4__blk899 * locals.var_qbuld);
        (assign31540_e46441, ((locals.var_t4__blk899_dn0 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, locals.var_qbsld_dn17,)
    }
};
        locals.var_qbsld = assign31540_e46443;
        locals.var_qbsld_dn0 = assign31540_e46443_d_n0;
        locals.var_qbsld_dn2 = assign31540_e46443_d_n2;
        locals.var_qbsld_dn6 = assign31540_e46443_d_n6;
        locals.var_qbsld_dn7 = assign31540_e46443_d_n7;
        locals.var_qbsld_dn10 = assign31540_e46443_d_n10;
        locals.var_qbsld_dn11 = assign31540_e46443_d_n11;
        locals.var_qbsld_dn12 = assign31540_e46443_d_n12;
        locals.var_qbsld_dn17 = assign31540_e46443_d_n17;

        let assign31550_e46454: f64 = if (((locals.var_flg_overd__blk915 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloopd__blk913 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1019 = assign31550_e46454;

        let (assign31560_e46467, assign31560_e46467_d_n0, assign31560_e46467_d_n2, assign31560_e46467_d_n6, assign31560_e46467_d_n7, assign31560_e46467_d_n10, assign31560_e46467_d_n11, assign31560_e46467_d_n12, assign31560_e46467_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1019 != 0.0)) {
        let assign31560_e46465: f64 = (locals.var_t4__blk899 * locals.var_qsuld);
        (assign31560_e46465, ((locals.var_t4__blk899_dn0 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn17,)
    }
};
        locals.var_qovd = assign31560_e46467;
        locals.var_qovd_dn0 = assign31560_e46467_d_n0;
        locals.var_qovd_dn2 = assign31560_e46467_d_n2;
        locals.var_qovd_dn6 = assign31560_e46467_d_n6;
        locals.var_qovd_dn7 = assign31560_e46467_d_n7;
        locals.var_qovd_dn10 = assign31560_e46467_d_n10;
        locals.var_qovd_dn11 = assign31560_e46467_d_n11;
        locals.var_qovd_dn12 = assign31560_e46467_d_n12;
        locals.var_qovd_dn17 = assign31560_e46467_d_n17;

        let (assign31570_e46480, assign31570_e46480_d_n0, assign31570_e46480_d_n2, assign31570_e46480_d_n6, assign31570_e46480_d_n7, assign31570_e46480_d_n10, assign31570_e46480_d_n11, assign31570_e46480_d_n12, assign31570_e46480_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1019 != 0.0)) {
        let assign31570_e46478: f64 = (locals.var_t4__blk899 * locals.var_qbuld);
        (assign31570_e46478, ((locals.var_t4__blk899_dn0 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, locals.var_qbdld_dn17,)
    }
};
        locals.var_qbdld = assign31570_e46480;
        locals.var_qbdld_dn0 = assign31570_e46480_d_n0;
        locals.var_qbdld_dn2 = assign31570_e46480_d_n2;
        locals.var_qbdld_dn6 = assign31570_e46480_d_n6;
        locals.var_qbdld_dn7 = assign31570_e46480_d_n7;
        locals.var_qbdld_dn10 = assign31570_e46480_d_n10;
        locals.var_qbdld_dn11 = assign31570_e46480_d_n11;
        locals.var_qbdld_dn12 = assign31570_e46480_d_n12;
        locals.var_qbdld_dn17 = assign31570_e46480_d_n17;

        let (assign31580_e46492,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) {
        let assign31580_e46486: f64 = (locals.var_modervs * locals.var_cgso_given);
        let assign31580_e46489: f64 = (locals.var_modenml * locals.var_cgdo_given);
        let assign31580_e46490: f64 = (assign31580_e46486 + assign31580_e46489);
        (assign31580_e46490,)
    } else {
        (locals.var_flg_overgiven,)
    }
};
        locals.var_flg_overgiven = assign31580_e46492;

        let (assign31590_e46506, assign31590_e46506_d_n0, assign31590_e46506_d_n2, assign31590_e46506_d_n6, assign31590_e46506_d_n7, assign31590_e46506_d_n10, assign31590_e46506_d_n11, assign31590_e46506_d_n12, assign31590_e46506_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31590_e46500: f64 = (locals.var_modervs * p.p170);
        let assign31590_e46503: f64 = (locals.var_modenml * p.p169);
        let assign31590_e46504: f64 = (assign31590_e46500 + assign31590_e46503);
        (assign31590_e46504, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31590_e46506;
        locals.var_cgdoe_dn0 = assign31590_e46506_d_n0;
        locals.var_cgdoe_dn2 = assign31590_e46506_d_n2;
        locals.var_cgdoe_dn6 = assign31590_e46506_d_n6;
        locals.var_cgdoe_dn7 = assign31590_e46506_d_n7;
        locals.var_cgdoe_dn10 = assign31590_e46506_d_n10;
        locals.var_cgdoe_dn11 = assign31590_e46506_d_n11;
        locals.var_cgdoe_dn12 = assign31590_e46506_d_n12;
        locals.var_cgdoe_dn17 = assign31590_e46506_d_n17;

        let assign31600_e46509: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1020 = assign31600_e46509;

        let (assign31610_e46525, assign31610_e46525_d_n0, assign31610_e46525_d_n2, assign31610_e46525_d_n6, assign31610_e46525_d_n7, assign31610_e46525_d_n10, assign31610_e46525_d_n11, assign31610_e46525_d_n12, assign31610_e46525_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1020 != 0.0)) {
        let assign31610_e46519: f64 = (locals.var_modervs * locals.var_w_dioscv);
        let assign31610_e46522: f64 = (locals.var_modenml * locals.var_w_diodcv);
        let assign31610_e46523: f64 = (assign31610_e46519 + assign31610_e46522);
        (assign31610_e46523, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31610_e46525;
        locals.var_t1__blk896_dn0 = assign31610_e46525_d_n0;
        locals.var_t1__blk896_dn2 = assign31610_e46525_d_n2;
        locals.var_t1__blk896_dn6 = assign31610_e46525_d_n6;
        locals.var_t1__blk896_dn7 = assign31610_e46525_d_n7;
        locals.var_t1__blk896_dn10 = assign31610_e46525_d_n10;
        locals.var_t1__blk896_dn11 = assign31610_e46525_d_n11;
        locals.var_t1__blk896_dn12 = assign31610_e46525_d_n12;
        locals.var_t1__blk896_dn17 = assign31610_e46525_d_n17;

        let (assign31620_e46538, assign31620_e46538_d_n0, assign31620_e46538_d_n2, assign31620_e46538_d_n6, assign31620_e46538_d_n7, assign31620_e46538_d_n10, assign31620_e46538_d_n11, assign31620_e46538_d_n12, assign31620_e46538_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1020 != 0.0)) {
        let assign31620_e46535: f64 = (-locals.var_t1__blk896);
        let assign31620_e46536: f64 = (locals.var_cgdoe * assign31620_e46535);
        (assign31620_e46536, ((locals.var_cgdoe_dn0 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgdoe_dn2 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgdoe_dn6 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgdoe_dn7 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgdoe_dn10 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgdoe_dn11 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgdoe_dn12 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgdoe_dn17 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn17))),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31620_e46538;
        locals.var_cgdoe_dn0 = assign31620_e46538_d_n0;
        locals.var_cgdoe_dn2 = assign31620_e46538_d_n2;
        locals.var_cgdoe_dn6 = assign31620_e46538_d_n6;
        locals.var_cgdoe_dn7 = assign31620_e46538_d_n7;
        locals.var_cgdoe_dn10 = assign31620_e46538_d_n10;
        locals.var_cgdoe_dn11 = assign31620_e46538_d_n11;
        locals.var_cgdoe_dn12 = assign31620_e46538_d_n12;
        locals.var_cgdoe_dn17 = assign31620_e46538_d_n17;

        let (assign31630_e46552, assign31630_e46552_d_n0, assign31630_e46552_d_n2, assign31630_e46552_d_n6, assign31630_e46552_d_n7, assign31630_e46552_d_n10, assign31630_e46552_d_n11, assign31630_e46552_d_n12, assign31630_e46552_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1020 == 0.0)) {
        let assign31630_e46549: f64 = (-locals.var_weffcv_nf);
        let assign31630_e46550: f64 = (locals.var_cgdoe * assign31630_e46549);
        (assign31630_e46550, (locals.var_cgdoe_dn0 * assign31630_e46549), (locals.var_cgdoe_dn2 * assign31630_e46549), (locals.var_cgdoe_dn6 * assign31630_e46549), (locals.var_cgdoe_dn7 * assign31630_e46549), (locals.var_cgdoe_dn10 * assign31630_e46549), (locals.var_cgdoe_dn11 * assign31630_e46549), (locals.var_cgdoe_dn12 * assign31630_e46549), (locals.var_cgdoe_dn17 * assign31630_e46549),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31630_e46552;
        locals.var_cgdoe_dn0 = assign31630_e46552_d_n0;
        locals.var_cgdoe_dn2 = assign31630_e46552_d_n2;
        locals.var_cgdoe_dn6 = assign31630_e46552_d_n6;
        locals.var_cgdoe_dn7 = assign31630_e46552_d_n7;
        locals.var_cgdoe_dn10 = assign31630_e46552_d_n10;
        locals.var_cgdoe_dn11 = assign31630_e46552_d_n11;
        locals.var_cgdoe_dn12 = assign31630_e46552_d_n12;
        locals.var_cgdoe_dn17 = assign31630_e46552_d_n17;

        let (assign31640_e46567, assign31640_e46567_d_n0, assign31640_e46567_d_n2, assign31640_e46567_d_n6, assign31640_e46567_d_n7, assign31640_e46567_d_n10, assign31640_e46567_d_n11, assign31640_e46567_d_n12, assign31640_e46567_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31640_e46560: f64 = (-locals.var_cgdoe);
        let assign31640_e46563: f64 = (locals.var_vgs - locals.var_vds);
        let assign31640_e46564: f64 = (assign31640_e46560 * assign31640_e46563);
        let assign31640_e46565: f64 = (locals.var_qgod + assign31640_e46564);
        (assign31640_e46565, (locals.var_qgod_dn0 + (((-locals.var_cgdoe_dn0) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn0)))), (locals.var_qgod_dn2 + (((-locals.var_cgdoe_dn2) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn2)))), (locals.var_qgod_dn6 + (((-locals.var_cgdoe_dn6) * assign31640_e46563) + (assign31640_e46560 * (locals.var_vgs_dn6 - locals.var_vds_dn6)))), (locals.var_qgod_dn7 + (((-locals.var_cgdoe_dn7) * assign31640_e46563) + (assign31640_e46560 * (locals.var_vgs_dn7 - locals.var_vds_dn7)))), (locals.var_qgod_dn10 + (((-locals.var_cgdoe_dn10) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn10)))), (locals.var_qgod_dn11 + (((-locals.var_cgdoe_dn11) * assign31640_e46563) + (assign31640_e46560 * (locals.var_vgs_dn11 - locals.var_vds_dn11)))), (locals.var_qgod_dn12 + (((-locals.var_cgdoe_dn12) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn12)))), (locals.var_qgod_dn17 + (((-locals.var_cgdoe_dn17) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn17)))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign31640_e46567;
        locals.var_qgod_dn0 = assign31640_e46567_d_n0;
        locals.var_qgod_dn2 = assign31640_e46567_d_n2;
        locals.var_qgod_dn6 = assign31640_e46567_d_n6;
        locals.var_qgod_dn7 = assign31640_e46567_d_n7;
        locals.var_qgod_dn10 = assign31640_e46567_d_n10;
        locals.var_qgod_dn11 = assign31640_e46567_d_n11;
        locals.var_qgod_dn12 = assign31640_e46567_d_n12;
        locals.var_qgod_dn17 = assign31640_e46567_d_n17;

        let (assign31650_e46579,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) {
        let assign31650_e46573: f64 = (locals.var_modenml * locals.var_cgso_given);
        let assign31650_e46576: f64 = (locals.var_modervs * locals.var_cgdo_given);
        let assign31650_e46577: f64 = (assign31650_e46573 + assign31650_e46576);
        (assign31650_e46577,)
    } else {
        (locals.var_flg_overgiven,)
    }
};
        locals.var_flg_overgiven = assign31650_e46579;

        let (assign31660_e46593, assign31660_e46593_d_n0, assign31660_e46593_d_n2, assign31660_e46593_d_n6, assign31660_e46593_d_n7, assign31660_e46593_d_n10, assign31660_e46593_d_n11, assign31660_e46593_d_n12, assign31660_e46593_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31660_e46587: f64 = (locals.var_modenml * p.p170);
        let assign31660_e46590: f64 = (locals.var_modervs * p.p169);
        let assign31660_e46591: f64 = (assign31660_e46587 + assign31660_e46590);
        (assign31660_e46591, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31660_e46593;
        locals.var_cgsoe_dn0 = assign31660_e46593_d_n0;
        locals.var_cgsoe_dn2 = assign31660_e46593_d_n2;
        locals.var_cgsoe_dn6 = assign31660_e46593_d_n6;
        locals.var_cgsoe_dn7 = assign31660_e46593_d_n7;
        locals.var_cgsoe_dn10 = assign31660_e46593_d_n10;
        locals.var_cgsoe_dn11 = assign31660_e46593_d_n11;
        locals.var_cgsoe_dn12 = assign31660_e46593_d_n12;
        locals.var_cgsoe_dn17 = assign31660_e46593_d_n17;

        let assign31670_e46596: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1021 = assign31670_e46596;

        let (assign31680_e46612, assign31680_e46612_d_n0, assign31680_e46612_d_n2, assign31680_e46612_d_n6, assign31680_e46612_d_n7, assign31680_e46612_d_n10, assign31680_e46612_d_n11, assign31680_e46612_d_n12, assign31680_e46612_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1021 != 0.0)) {
        let assign31680_e46606: f64 = (locals.var_modenml * locals.var_w_dioscv);
        let assign31680_e46609: f64 = (locals.var_modervs * locals.var_w_diodcv);
        let assign31680_e46610: f64 = (assign31680_e46606 + assign31680_e46609);
        (assign31680_e46610, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31680_e46612;
        locals.var_t1__blk896_dn0 = assign31680_e46612_d_n0;
        locals.var_t1__blk896_dn2 = assign31680_e46612_d_n2;
        locals.var_t1__blk896_dn6 = assign31680_e46612_d_n6;
        locals.var_t1__blk896_dn7 = assign31680_e46612_d_n7;
        locals.var_t1__blk896_dn10 = assign31680_e46612_d_n10;
        locals.var_t1__blk896_dn11 = assign31680_e46612_d_n11;
        locals.var_t1__blk896_dn12 = assign31680_e46612_d_n12;
        locals.var_t1__blk896_dn17 = assign31680_e46612_d_n17;

        let (assign31690_e46625, assign31690_e46625_d_n0, assign31690_e46625_d_n2, assign31690_e46625_d_n6, assign31690_e46625_d_n7, assign31690_e46625_d_n10, assign31690_e46625_d_n11, assign31690_e46625_d_n12, assign31690_e46625_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1021 != 0.0)) {
        let assign31690_e46622: f64 = (-locals.var_t1__blk896);
        let assign31690_e46623: f64 = (locals.var_cgsoe * assign31690_e46622);
        (assign31690_e46623, ((locals.var_cgsoe_dn0 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgsoe_dn2 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgsoe_dn6 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgsoe_dn7 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgsoe_dn10 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgsoe_dn11 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgsoe_dn12 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgsoe_dn17 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn17))),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31690_e46625;
        locals.var_cgsoe_dn0 = assign31690_e46625_d_n0;
        locals.var_cgsoe_dn2 = assign31690_e46625_d_n2;
        locals.var_cgsoe_dn6 = assign31690_e46625_d_n6;
        locals.var_cgsoe_dn7 = assign31690_e46625_d_n7;
        locals.var_cgsoe_dn10 = assign31690_e46625_d_n10;
        locals.var_cgsoe_dn11 = assign31690_e46625_d_n11;
        locals.var_cgsoe_dn12 = assign31690_e46625_d_n12;
        locals.var_cgsoe_dn17 = assign31690_e46625_d_n17;

        let (assign31700_e46639, assign31700_e46639_d_n0, assign31700_e46639_d_n2, assign31700_e46639_d_n6, assign31700_e46639_d_n7, assign31700_e46639_d_n10, assign31700_e46639_d_n11, assign31700_e46639_d_n12, assign31700_e46639_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1021 == 0.0)) {
        let assign31700_e46636: f64 = (-locals.var_weffcv_nf);
        let assign31700_e46637: f64 = (locals.var_cgsoe * assign31700_e46636);
        (assign31700_e46637, (locals.var_cgsoe_dn0 * assign31700_e46636), (locals.var_cgsoe_dn2 * assign31700_e46636), (locals.var_cgsoe_dn6 * assign31700_e46636), (locals.var_cgsoe_dn7 * assign31700_e46636), (locals.var_cgsoe_dn10 * assign31700_e46636), (locals.var_cgsoe_dn11 * assign31700_e46636), (locals.var_cgsoe_dn12 * assign31700_e46636), (locals.var_cgsoe_dn17 * assign31700_e46636),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31700_e46639;
        locals.var_cgsoe_dn0 = assign31700_e46639_d_n0;
        locals.var_cgsoe_dn2 = assign31700_e46639_d_n2;
        locals.var_cgsoe_dn6 = assign31700_e46639_d_n6;
        locals.var_cgsoe_dn7 = assign31700_e46639_d_n7;
        locals.var_cgsoe_dn10 = assign31700_e46639_d_n10;
        locals.var_cgsoe_dn11 = assign31700_e46639_d_n11;
        locals.var_cgsoe_dn12 = assign31700_e46639_d_n12;
        locals.var_cgsoe_dn17 = assign31700_e46639_d_n17;

        let (assign31710_e46652, assign31710_e46652_d_n0, assign31710_e46652_d_n2, assign31710_e46652_d_n6, assign31710_e46652_d_n7, assign31710_e46652_d_n10, assign31710_e46652_d_n11, assign31710_e46652_d_n12, assign31710_e46652_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31710_e46647: f64 = (-locals.var_cgsoe);
        let assign31710_e46649: f64 = (assign31710_e46647 * locals.var_vgs);
        let assign31710_e46650: f64 = (locals.var_qgos + assign31710_e46649);
        (assign31710_e46650, (locals.var_qgos_dn0 + ((-locals.var_cgsoe_dn0) * locals.var_vgs)), (locals.var_qgos_dn2 + ((-locals.var_cgsoe_dn2) * locals.var_vgs)), (locals.var_qgos_dn6 + (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31710_e46647 * locals.var_vgs_dn6))), (locals.var_qgos_dn7 + (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31710_e46647 * locals.var_vgs_dn7))), (locals.var_qgos_dn10 + ((-locals.var_cgsoe_dn10) * locals.var_vgs)), (locals.var_qgos_dn11 + (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31710_e46647 * locals.var_vgs_dn11))), (locals.var_qgos_dn12 + ((-locals.var_cgsoe_dn12) * locals.var_vgs)), (locals.var_qgos_dn17 + ((-locals.var_cgsoe_dn17) * locals.var_vgs)),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign31710_e46652;
        locals.var_qgos_dn0 = assign31710_e46652_d_n0;
        locals.var_qgos_dn2 = assign31710_e46652_d_n2;
        locals.var_qgos_dn6 = assign31710_e46652_d_n6;
        locals.var_qgos_dn7 = assign31710_e46652_d_n7;
        locals.var_qgos_dn10 = assign31710_e46652_d_n10;
        locals.var_qgos_dn11 = assign31710_e46652_d_n11;
        locals.var_qgos_dn12 = assign31710_e46652_d_n12;
        locals.var_qgos_dn17 = assign31710_e46652_d_n17;

        let assign31720_e46665: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgdo_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgso_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1022 = assign31720_e46665;

        let assign31730_e46668: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1023 = assign31730_e46668;

    }

    pub(super) fn stamp_transient_block_113(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31740_e46684, assign31740_e46684_d_n0, assign31740_e46684_d_n2, assign31740_e46684_d_n6, assign31740_e46684_d_n7, assign31740_e46684_d_n10, assign31740_e46684_d_n11, assign31740_e46684_d_n12, assign31740_e46684_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_guard1023 != 0.0)) {
        let assign31740_e46678: f64 = (-locals.var_cox0__blk906);
        let assign31740_e46680: f64 = (assign31740_e46678 * p.p188);
        let assign31740_e46682: f64 = (assign31740_e46680 * locals.var_w_diodcv);
        (assign31740_e46682, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31740_e46684;
        locals.var_cgdoe_dn0 = assign31740_e46684_d_n0;
        locals.var_cgdoe_dn2 = assign31740_e46684_d_n2;
        locals.var_cgdoe_dn6 = assign31740_e46684_d_n6;
        locals.var_cgdoe_dn7 = assign31740_e46684_d_n7;
        locals.var_cgdoe_dn10 = assign31740_e46684_d_n10;
        locals.var_cgdoe_dn11 = assign31740_e46684_d_n11;
        locals.var_cgdoe_dn12 = assign31740_e46684_d_n12;
        locals.var_cgdoe_dn17 = assign31740_e46684_d_n17;

        let (assign31750_e46701, assign31750_e46701_d_n0, assign31750_e46701_d_n2, assign31750_e46701_d_n6, assign31750_e46701_d_n7, assign31750_e46701_d_n10, assign31750_e46701_d_n11, assign31750_e46701_d_n12, assign31750_e46701_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_guard1023 == 0.0)) {
        let assign31750_e46695: f64 = (-locals.var_cox0__blk906);
        let assign31750_e46697: f64 = (assign31750_e46695 * p.p188);
        let assign31750_e46699: f64 = (assign31750_e46697 * locals.var_weffcv_nf);
        (assign31750_e46699, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31750_e46701;
        locals.var_cgdoe_dn0 = assign31750_e46701_d_n0;
        locals.var_cgdoe_dn2 = assign31750_e46701_d_n2;
        locals.var_cgdoe_dn6 = assign31750_e46701_d_n6;
        locals.var_cgdoe_dn7 = assign31750_e46701_d_n7;
        locals.var_cgdoe_dn10 = assign31750_e46701_d_n10;
        locals.var_cgdoe_dn11 = assign31750_e46701_d_n11;
        locals.var_cgdoe_dn12 = assign31750_e46701_d_n12;
        locals.var_cgdoe_dn17 = assign31750_e46701_d_n17;

        let (assign31760_e46717, assign31760_e46717_d_n0, assign31760_e46717_d_n2, assign31760_e46717_d_n6, assign31760_e46717_d_n7, assign31760_e46717_d_n10, assign31760_e46717_d_n11, assign31760_e46717_d_n12, assign31760_e46717_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) {
        let assign31760_e46711: f64 = (locals.var_modervs * p.p170);
        let assign31760_e46714: f64 = (locals.var_modenml * p.p169);
        let assign31760_e46715: f64 = (assign31760_e46711 + assign31760_e46714);
        (assign31760_e46715, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31760_e46717;
        locals.var_cgdoe_dn0 = assign31760_e46717_d_n0;
        locals.var_cgdoe_dn2 = assign31760_e46717_d_n2;
        locals.var_cgdoe_dn6 = assign31760_e46717_d_n6;
        locals.var_cgdoe_dn7 = assign31760_e46717_d_n7;
        locals.var_cgdoe_dn10 = assign31760_e46717_d_n10;
        locals.var_cgdoe_dn11 = assign31760_e46717_d_n11;
        locals.var_cgdoe_dn12 = assign31760_e46717_d_n12;
        locals.var_cgdoe_dn17 = assign31760_e46717_d_n17;

        let assign31770_e46720: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1024 = assign31770_e46720;

        let (assign31780_e46738, assign31780_e46738_d_n0, assign31780_e46738_d_n2, assign31780_e46738_d_n6, assign31780_e46738_d_n7, assign31780_e46738_d_n10, assign31780_e46738_d_n11, assign31780_e46738_d_n12, assign31780_e46738_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) && (locals.var_guard1024 != 0.0)) {
        let assign31780_e46732: f64 = (locals.var_modervs * locals.var_w_dioscv);
        let assign31780_e46735: f64 = (locals.var_modenml * locals.var_w_diodcv);
        let assign31780_e46736: f64 = (assign31780_e46732 + assign31780_e46735);
        (assign31780_e46736, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31780_e46738;
        locals.var_t1__blk896_dn0 = assign31780_e46738_d_n0;
        locals.var_t1__blk896_dn2 = assign31780_e46738_d_n2;
        locals.var_t1__blk896_dn6 = assign31780_e46738_d_n6;
        locals.var_t1__blk896_dn7 = assign31780_e46738_d_n7;
        locals.var_t1__blk896_dn10 = assign31780_e46738_d_n10;
        locals.var_t1__blk896_dn11 = assign31780_e46738_d_n11;
        locals.var_t1__blk896_dn12 = assign31780_e46738_d_n12;
        locals.var_t1__blk896_dn17 = assign31780_e46738_d_n17;

        let (assign31790_e46753, assign31790_e46753_d_n0, assign31790_e46753_d_n2, assign31790_e46753_d_n6, assign31790_e46753_d_n7, assign31790_e46753_d_n10, assign31790_e46753_d_n11, assign31790_e46753_d_n12, assign31790_e46753_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) && (locals.var_guard1024 != 0.0)) {
        let assign31790_e46750: f64 = (-locals.var_t1__blk896);
        let assign31790_e46751: f64 = (locals.var_cgdoe * assign31790_e46750);
        (assign31790_e46751, ((locals.var_cgdoe_dn0 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgdoe_dn2 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgdoe_dn6 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgdoe_dn7 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgdoe_dn10 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgdoe_dn11 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgdoe_dn12 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgdoe_dn17 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn17))),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31790_e46753;
        locals.var_cgdoe_dn0 = assign31790_e46753_d_n0;
        locals.var_cgdoe_dn2 = assign31790_e46753_d_n2;
        locals.var_cgdoe_dn6 = assign31790_e46753_d_n6;
        locals.var_cgdoe_dn7 = assign31790_e46753_d_n7;
        locals.var_cgdoe_dn10 = assign31790_e46753_d_n10;
        locals.var_cgdoe_dn11 = assign31790_e46753_d_n11;
        locals.var_cgdoe_dn12 = assign31790_e46753_d_n12;
        locals.var_cgdoe_dn17 = assign31790_e46753_d_n17;

        let (assign31800_e46769, assign31800_e46769_d_n0, assign31800_e46769_d_n2, assign31800_e46769_d_n6, assign31800_e46769_d_n7, assign31800_e46769_d_n10, assign31800_e46769_d_n11, assign31800_e46769_d_n12, assign31800_e46769_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) && (locals.var_guard1024 == 0.0)) {
        let assign31800_e46766: f64 = (-locals.var_weffcv_nf);
        let assign31800_e46767: f64 = (locals.var_cgdoe * assign31800_e46766);
        (assign31800_e46767, (locals.var_cgdoe_dn0 * assign31800_e46766), (locals.var_cgdoe_dn2 * assign31800_e46766), (locals.var_cgdoe_dn6 * assign31800_e46766), (locals.var_cgdoe_dn7 * assign31800_e46766), (locals.var_cgdoe_dn10 * assign31800_e46766), (locals.var_cgdoe_dn11 * assign31800_e46766), (locals.var_cgdoe_dn12 * assign31800_e46766), (locals.var_cgdoe_dn17 * assign31800_e46766),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31800_e46769;
        locals.var_cgdoe_dn0 = assign31800_e46769_d_n0;
        locals.var_cgdoe_dn2 = assign31800_e46769_d_n2;
        locals.var_cgdoe_dn6 = assign31800_e46769_d_n6;
        locals.var_cgdoe_dn7 = assign31800_e46769_d_n7;
        locals.var_cgdoe_dn10 = assign31800_e46769_d_n10;
        locals.var_cgdoe_dn11 = assign31800_e46769_d_n11;
        locals.var_cgdoe_dn12 = assign31800_e46769_d_n12;
        locals.var_cgdoe_dn17 = assign31800_e46769_d_n17;

        let (assign31810_e46781, assign31810_e46781_d_n0, assign31810_e46781_d_n2, assign31810_e46781_d_n6, assign31810_e46781_d_n7, assign31810_e46781_d_n10, assign31810_e46781_d_n11, assign31810_e46781_d_n12, assign31810_e46781_d_n17,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) {
        let assign31810_e46775: f64 = (-locals.var_cgdoe);
        let assign31810_e46778: f64 = (locals.var_vgs - locals.var_vds);
        let assign31810_e46779: f64 = (assign31810_e46775 * assign31810_e46778);
        (assign31810_e46779, (((-locals.var_cgdoe_dn0) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn0))), (((-locals.var_cgdoe_dn2) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn2))), (((-locals.var_cgdoe_dn6) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((-locals.var_cgdoe_dn7) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((-locals.var_cgdoe_dn10) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn10))), (((-locals.var_cgdoe_dn11) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (((-locals.var_cgdoe_dn12) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn12))), (((-locals.var_cgdoe_dn17) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign31810_e46781;
        locals.var_qgod_dn0 = assign31810_e46781_d_n0;
        locals.var_qgod_dn2 = assign31810_e46781_d_n2;
        locals.var_qgod_dn6 = assign31810_e46781_d_n6;
        locals.var_qgod_dn7 = assign31810_e46781_d_n7;
        locals.var_qgod_dn10 = assign31810_e46781_d_n10;
        locals.var_qgod_dn11 = assign31810_e46781_d_n11;
        locals.var_qgod_dn12 = assign31810_e46781_d_n12;
        locals.var_qgod_dn17 = assign31810_e46781_d_n17;

        let assign31820_e46794: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgso_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign31820_e46794;

        let assign31830_e46797: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign31830_e46797;

        let (assign31840_e46813, assign31840_e46813_d_n0, assign31840_e46813_d_n2, assign31840_e46813_d_n6, assign31840_e46813_d_n7, assign31840_e46813_d_n10, assign31840_e46813_d_n11, assign31840_e46813_d_n12, assign31840_e46813_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 != 0.0)) && (locals.var_guard1026 != 0.0)) {
        let assign31840_e46807: f64 = (-locals.var_cox0__blk906);
        let assign31840_e46809: f64 = (assign31840_e46807 * p.p188);
        let assign31840_e46811: f64 = (assign31840_e46809 * locals.var_w_dioscv);
        (assign31840_e46811, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31840_e46813;
        locals.var_cgsoe_dn0 = assign31840_e46813_d_n0;
        locals.var_cgsoe_dn2 = assign31840_e46813_d_n2;
        locals.var_cgsoe_dn6 = assign31840_e46813_d_n6;
        locals.var_cgsoe_dn7 = assign31840_e46813_d_n7;
        locals.var_cgsoe_dn10 = assign31840_e46813_d_n10;
        locals.var_cgsoe_dn11 = assign31840_e46813_d_n11;
        locals.var_cgsoe_dn12 = assign31840_e46813_d_n12;
        locals.var_cgsoe_dn17 = assign31840_e46813_d_n17;

        let (assign31850_e46830, assign31850_e46830_d_n0, assign31850_e46830_d_n2, assign31850_e46830_d_n6, assign31850_e46830_d_n7, assign31850_e46830_d_n10, assign31850_e46830_d_n11, assign31850_e46830_d_n12, assign31850_e46830_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 != 0.0)) && (locals.var_guard1026 == 0.0)) {
        let assign31850_e46824: f64 = (-locals.var_cox0__blk906);
        let assign31850_e46826: f64 = (assign31850_e46824 * p.p188);
        let assign31850_e46828: f64 = (assign31850_e46826 * locals.var_weffcv_nf);
        (assign31850_e46828, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31850_e46830;
        locals.var_cgsoe_dn0 = assign31850_e46830_d_n0;
        locals.var_cgsoe_dn2 = assign31850_e46830_d_n2;
        locals.var_cgsoe_dn6 = assign31850_e46830_d_n6;
        locals.var_cgsoe_dn7 = assign31850_e46830_d_n7;
        locals.var_cgsoe_dn10 = assign31850_e46830_d_n10;
        locals.var_cgsoe_dn11 = assign31850_e46830_d_n11;
        locals.var_cgsoe_dn12 = assign31850_e46830_d_n12;
        locals.var_cgsoe_dn17 = assign31850_e46830_d_n17;

        let (assign31860_e46846, assign31860_e46846_d_n0, assign31860_e46846_d_n2, assign31860_e46846_d_n6, assign31860_e46846_d_n7, assign31860_e46846_d_n10, assign31860_e46846_d_n11, assign31860_e46846_d_n12, assign31860_e46846_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) {
        let assign31860_e46840: f64 = (locals.var_modenml * p.p170);
        let assign31860_e46843: f64 = (locals.var_modervs * p.p169);
        let assign31860_e46844: f64 = (assign31860_e46840 + assign31860_e46843);
        (assign31860_e46844, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31860_e46846;
        locals.var_cgsoe_dn0 = assign31860_e46846_d_n0;
        locals.var_cgsoe_dn2 = assign31860_e46846_d_n2;
        locals.var_cgsoe_dn6 = assign31860_e46846_d_n6;
        locals.var_cgsoe_dn7 = assign31860_e46846_d_n7;
        locals.var_cgsoe_dn10 = assign31860_e46846_d_n10;
        locals.var_cgsoe_dn11 = assign31860_e46846_d_n11;
        locals.var_cgsoe_dn12 = assign31860_e46846_d_n12;
        locals.var_cgsoe_dn17 = assign31860_e46846_d_n17;

        let assign31870_e46849: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1027 = assign31870_e46849;

        let (assign31880_e46867, assign31880_e46867_d_n0, assign31880_e46867_d_n2, assign31880_e46867_d_n6, assign31880_e46867_d_n7, assign31880_e46867_d_n10, assign31880_e46867_d_n11, assign31880_e46867_d_n12, assign31880_e46867_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 != 0.0)) {
        let assign31880_e46861: f64 = (locals.var_modenml * locals.var_w_dioscv);
        let assign31880_e46864: f64 = (locals.var_modervs * locals.var_w_diodcv);
        let assign31880_e46865: f64 = (assign31880_e46861 + assign31880_e46864);
        (assign31880_e46865, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31880_e46867;
        locals.var_t1__blk896_dn0 = assign31880_e46867_d_n0;
        locals.var_t1__blk896_dn2 = assign31880_e46867_d_n2;
        locals.var_t1__blk896_dn6 = assign31880_e46867_d_n6;
        locals.var_t1__blk896_dn7 = assign31880_e46867_d_n7;
        locals.var_t1__blk896_dn10 = assign31880_e46867_d_n10;
        locals.var_t1__blk896_dn11 = assign31880_e46867_d_n11;
        locals.var_t1__blk896_dn12 = assign31880_e46867_d_n12;
        locals.var_t1__blk896_dn17 = assign31880_e46867_d_n17;

        let (assign31890_e46882, assign31890_e46882_d_n0, assign31890_e46882_d_n2, assign31890_e46882_d_n6, assign31890_e46882_d_n7, assign31890_e46882_d_n10, assign31890_e46882_d_n11, assign31890_e46882_d_n12, assign31890_e46882_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 != 0.0)) {
        let assign31890_e46879: f64 = (-locals.var_t1__blk896);
        let assign31890_e46880: f64 = (locals.var_cgsoe * assign31890_e46879);
        (assign31890_e46880, ((locals.var_cgsoe_dn0 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgsoe_dn2 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgsoe_dn6 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgsoe_dn7 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgsoe_dn10 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgsoe_dn11 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgsoe_dn12 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgsoe_dn17 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn17))),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31890_e46882;
        locals.var_cgsoe_dn0 = assign31890_e46882_d_n0;
        locals.var_cgsoe_dn2 = assign31890_e46882_d_n2;
        locals.var_cgsoe_dn6 = assign31890_e46882_d_n6;
        locals.var_cgsoe_dn7 = assign31890_e46882_d_n7;
        locals.var_cgsoe_dn10 = assign31890_e46882_d_n10;
        locals.var_cgsoe_dn11 = assign31890_e46882_d_n11;
        locals.var_cgsoe_dn12 = assign31890_e46882_d_n12;
        locals.var_cgsoe_dn17 = assign31890_e46882_d_n17;

        let (assign31900_e46898, assign31900_e46898_d_n0, assign31900_e46898_d_n2, assign31900_e46898_d_n6, assign31900_e46898_d_n7, assign31900_e46898_d_n10, assign31900_e46898_d_n11, assign31900_e46898_d_n12, assign31900_e46898_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 == 0.0)) {
        let assign31900_e46895: f64 = (-locals.var_weffcv_nf);
        let assign31900_e46896: f64 = (locals.var_cgsoe * assign31900_e46895);
        (assign31900_e46896, (locals.var_cgsoe_dn0 * assign31900_e46895), (locals.var_cgsoe_dn2 * assign31900_e46895), (locals.var_cgsoe_dn6 * assign31900_e46895), (locals.var_cgsoe_dn7 * assign31900_e46895), (locals.var_cgsoe_dn10 * assign31900_e46895), (locals.var_cgsoe_dn11 * assign31900_e46895), (locals.var_cgsoe_dn12 * assign31900_e46895), (locals.var_cgsoe_dn17 * assign31900_e46895),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31900_e46898;
        locals.var_cgsoe_dn0 = assign31900_e46898_d_n0;
        locals.var_cgsoe_dn2 = assign31900_e46898_d_n2;
        locals.var_cgsoe_dn6 = assign31900_e46898_d_n6;
        locals.var_cgsoe_dn7 = assign31900_e46898_d_n7;
        locals.var_cgsoe_dn10 = assign31900_e46898_d_n10;
        locals.var_cgsoe_dn11 = assign31900_e46898_d_n11;
        locals.var_cgsoe_dn12 = assign31900_e46898_d_n12;
        locals.var_cgsoe_dn17 = assign31900_e46898_d_n17;

        let (assign31910_e46908, assign31910_e46908_d_n0, assign31910_e46908_d_n2, assign31910_e46908_d_n6, assign31910_e46908_d_n7, assign31910_e46908_d_n10, assign31910_e46908_d_n11, assign31910_e46908_d_n12, assign31910_e46908_d_n17,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) {
        let assign31910_e46904: f64 = (-locals.var_cgsoe);
        let assign31910_e46906: f64 = (assign31910_e46904 * locals.var_vgs);
        (assign31910_e46906, ((-locals.var_cgsoe_dn0) * locals.var_vgs), ((-locals.var_cgsoe_dn2) * locals.var_vgs), (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn6)), (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn7)), ((-locals.var_cgsoe_dn10) * locals.var_vgs), (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn11)), ((-locals.var_cgsoe_dn12) * locals.var_vgs), ((-locals.var_cgsoe_dn17) * locals.var_vgs),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign31910_e46908;
        locals.var_qgos_dn0 = assign31910_e46908_d_n0;
        locals.var_qgos_dn2 = assign31910_e46908_d_n2;
        locals.var_qgos_dn6 = assign31910_e46908_d_n6;
        locals.var_qgos_dn7 = assign31910_e46908_d_n7;
        locals.var_qgos_dn10 = assign31910_e46908_d_n10;
        locals.var_qgos_dn11 = assign31910_e46908_d_n11;
        locals.var_qgos_dn12 = assign31910_e46908_d_n12;
        locals.var_qgos_dn17 = assign31910_e46908_d_n17;

        let assign31920_e46911: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1028 = assign31920_e46911;

        let (assign31930_e46915, assign31930_e46915_d_n6, assign31930_e46915_d_n12,) = {
    if (locals.var_guard1028 != 0.0) {
        (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12,)
    } else {
        (locals.var_vbdj, locals.var_vbdj_dn6, locals.var_vbdj_dn12,)
    }
};
        locals.var_vbdj = assign31930_e46915;
        locals.var_vbdj_dn6 = assign31930_e46915_d_n6;
        locals.var_vbdj_dn12 = assign31930_e46915_d_n12;

        let (assign31940_e46919, assign31940_e46919_d_n7, assign31940_e46919_d_n12,) = {
    if (locals.var_guard1028 != 0.0) {
        (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12,)
    } else {
        (locals.var_vbsj, locals.var_vbsj_dn7, locals.var_vbsj_dn12,)
    }
};
        locals.var_vbsj = assign31940_e46919;
        locals.var_vbsj_dn7 = assign31940_e46919_d_n7;
        locals.var_vbsj_dn12 = assign31940_e46919_d_n12;

        let (assign31950_e46941, assign31950_e46941_d_n0, assign31950_e46941_d_n2, assign31950_e46941_d_n6, assign31950_e46941_d_n7, assign31950_e46941_d_n10, assign31950_e46941_d_n11, assign31950_e46941_d_n12, assign31950_e46941_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign31950_e46924: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign31950_e46927: f64 = (locals.var_eg * locals.var_beta);
        let assign31950_e46928: f64 = (assign31950_e46924 - assign31950_e46927);
        let assign31950_e46932: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign31950_e46933: f64 = (assign31950_e46932).ln();
        let assign31950_e46934: f64 = (p.p175 * assign31950_e46933);
        let assign31950_e46935: f64 = (assign31950_e46928 + assign31950_e46934);
        let assign31950_e46937: f64 = (assign31950_e46935 / p.p174);
        let assign31950_e46938: f64 = (assign31950_e46937).exp();
        let assign31950_e46939: f64 = (p.p173 * assign31950_e46938);
        (assign31950_e46939, (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p175 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31950_e46932))) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn12, locals.var_js_dn17,)
    }
};
        locals.var_js = assign31950_e46941;
        locals.var_js_dn0 = assign31950_e46941_d_n0;
        locals.var_js_dn2 = assign31950_e46941_d_n2;
        locals.var_js_dn6 = assign31950_e46941_d_n6;
        locals.var_js_dn7 = assign31950_e46941_d_n7;
        locals.var_js_dn10 = assign31950_e46941_d_n10;
        locals.var_js_dn11 = assign31950_e46941_d_n11;
        locals.var_js_dn12 = assign31950_e46941_d_n12;
        locals.var_js_dn17 = assign31950_e46941_d_n17;

        let (assign31960_e46963, assign31960_e46963_d_n0, assign31960_e46963_d_n2, assign31960_e46963_d_n6, assign31960_e46963_d_n7, assign31960_e46963_d_n10, assign31960_e46963_d_n11, assign31960_e46963_d_n12, assign31960_e46963_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign31960_e46946: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign31960_e46949: f64 = (locals.var_eg * locals.var_beta);
        let assign31960_e46950: f64 = (assign31960_e46946 - assign31960_e46949);
        let assign31960_e46954: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign31960_e46955: f64 = (assign31960_e46954).ln();
        let assign31960_e46956: f64 = (p.p176 * assign31960_e46955);
        let assign31960_e46957: f64 = (assign31960_e46950 + assign31960_e46956);
        let assign31960_e46959: f64 = (assign31960_e46957 / p.p174);
        let assign31960_e46960: f64 = (assign31960_e46959).exp();
        let assign31960_e46961: f64 = (p.p173 * assign31960_e46960);
        (assign31960_e46961, (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p176 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31960_e46954))) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn12, locals.var_js2_dn17,)
    }
};
        locals.var_js2 = assign31960_e46963;
        locals.var_js2_dn0 = assign31960_e46963_d_n0;
        locals.var_js2_dn2 = assign31960_e46963_d_n2;
        locals.var_js2_dn6 = assign31960_e46963_d_n6;
        locals.var_js2_dn7 = assign31960_e46963_d_n7;
        locals.var_js2_dn10 = assign31960_e46963_d_n10;
        locals.var_js2_dn11 = assign31960_e46963_d_n11;
        locals.var_js2_dn12 = assign31960_e46963_d_n12;
        locals.var_js2_dn17 = assign31960_e46963_d_n17;

        let (assign31970_e46971, assign31970_e46971_d_n0, assign31970_e46971_d_n2, assign31970_e46971_d_n6, assign31970_e46971_d_n7, assign31970_e46971_d_n10, assign31970_e46971_d_n11, assign31970_e46971_d_n12, assign31970_e46971_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign31970_e46967: f64 = (locals.var_w_diod * p.p237);
        let assign31970_e46969: f64 = (assign31970_e46967 * locals.var_js);
        (assign31970_e46969, (assign31970_e46967 * locals.var_js_dn0), (assign31970_e46967 * locals.var_js_dn2), (assign31970_e46967 * locals.var_js_dn6), (assign31970_e46967 * locals.var_js_dn7), (assign31970_e46967 * locals.var_js_dn10), (assign31970_e46967 * locals.var_js_dn11), (assign31970_e46967 * locals.var_js_dn12), (assign31970_e46967 * locals.var_js_dn17),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17,)
    }
};
        locals.var_isbd = assign31970_e46971;
        locals.var_isbd_dn0 = assign31970_e46971_d_n0;
        locals.var_isbd_dn2 = assign31970_e46971_d_n2;
        locals.var_isbd_dn6 = assign31970_e46971_d_n6;
        locals.var_isbd_dn7 = assign31970_e46971_d_n7;
        locals.var_isbd_dn10 = assign31970_e46971_d_n10;
        locals.var_isbd_dn11 = assign31970_e46971_d_n11;
        locals.var_isbd_dn12 = assign31970_e46971_d_n12;
        locals.var_isbd_dn17 = assign31970_e46971_d_n17;

        let (assign31980_e46979, assign31980_e46979_d_n0, assign31980_e46979_d_n2, assign31980_e46979_d_n6, assign31980_e46979_d_n7, assign31980_e46979_d_n10, assign31980_e46979_d_n11, assign31980_e46979_d_n12, assign31980_e46979_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign31980_e46975: f64 = (locals.var_w_diod * p.p237);
        let assign31980_e46977: f64 = (assign31980_e46975 * locals.var_js2);
        (assign31980_e46977, (assign31980_e46975 * locals.var_js2_dn0), (assign31980_e46975 * locals.var_js2_dn2), (assign31980_e46975 * locals.var_js2_dn6), (assign31980_e46975 * locals.var_js2_dn7), (assign31980_e46975 * locals.var_js2_dn10), (assign31980_e46975 * locals.var_js2_dn11), (assign31980_e46975 * locals.var_js2_dn12), (assign31980_e46975 * locals.var_js2_dn17),)
    } else {
        (locals.var_isbd2, locals.var_isbd2_dn0, locals.var_isbd2_dn2, locals.var_isbd2_dn6, locals.var_isbd2_dn7, locals.var_isbd2_dn10, locals.var_isbd2_dn11, locals.var_isbd2_dn12, locals.var_isbd2_dn17,)
    }
};
        locals.var_isbd2 = assign31980_e46979;
        locals.var_isbd2_dn0 = assign31980_e46979_d_n0;
        locals.var_isbd2_dn2 = assign31980_e46979_d_n2;
        locals.var_isbd2_dn6 = assign31980_e46979_d_n6;
        locals.var_isbd2_dn7 = assign31980_e46979_d_n7;
        locals.var_isbd2_dn10 = assign31980_e46979_d_n10;
        locals.var_isbd2_dn11 = assign31980_e46979_d_n11;
        locals.var_isbd2_dn12 = assign31980_e46979_d_n12;
        locals.var_isbd2_dn17 = assign31980_e46979_d_n17;

        let (assign31990_e46987, assign31990_e46987_d_n0, assign31990_e46987_d_n2, assign31990_e46987_d_n6, assign31990_e46987_d_n7, assign31990_e46987_d_n10, assign31990_e46987_d_n11, assign31990_e46987_d_n12, assign31990_e46987_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign31990_e46983: f64 = (locals.var_w_dios * p.p237);
        let assign31990_e46985: f64 = (assign31990_e46983 * locals.var_js);
        (assign31990_e46985, (assign31990_e46983 * locals.var_js_dn0), (assign31990_e46983 * locals.var_js_dn2), (assign31990_e46983 * locals.var_js_dn6), (assign31990_e46983 * locals.var_js_dn7), (assign31990_e46983 * locals.var_js_dn10), (assign31990_e46983 * locals.var_js_dn11), (assign31990_e46983 * locals.var_js_dn12), (assign31990_e46983 * locals.var_js_dn17),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn12, locals.var_isbs_dn17,)
    }
};
        locals.var_isbs = assign31990_e46987;
        locals.var_isbs_dn0 = assign31990_e46987_d_n0;
        locals.var_isbs_dn2 = assign31990_e46987_d_n2;
        locals.var_isbs_dn6 = assign31990_e46987_d_n6;
        locals.var_isbs_dn7 = assign31990_e46987_d_n7;
        locals.var_isbs_dn10 = assign31990_e46987_d_n10;
        locals.var_isbs_dn11 = assign31990_e46987_d_n11;
        locals.var_isbs_dn12 = assign31990_e46987_d_n12;
        locals.var_isbs_dn17 = assign31990_e46987_d_n17;

        let (assign32000_e46995, assign32000_e46995_d_n0, assign32000_e46995_d_n2, assign32000_e46995_d_n6, assign32000_e46995_d_n7, assign32000_e46995_d_n10, assign32000_e46995_d_n11, assign32000_e46995_d_n12, assign32000_e46995_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32000_e46991: f64 = (locals.var_w_dios * p.p237);
        let assign32000_e46993: f64 = (assign32000_e46991 * locals.var_js2);
        (assign32000_e46993, (assign32000_e46991 * locals.var_js2_dn0), (assign32000_e46991 * locals.var_js2_dn2), (assign32000_e46991 * locals.var_js2_dn6), (assign32000_e46991 * locals.var_js2_dn7), (assign32000_e46991 * locals.var_js2_dn10), (assign32000_e46991 * locals.var_js2_dn11), (assign32000_e46991 * locals.var_js2_dn12), (assign32000_e46991 * locals.var_js2_dn17),)
    } else {
        (locals.var_isbs2, locals.var_isbs2_dn0, locals.var_isbs2_dn2, locals.var_isbs2_dn6, locals.var_isbs2_dn7, locals.var_isbs2_dn10, locals.var_isbs2_dn11, locals.var_isbs2_dn12, locals.var_isbs2_dn17,)
    }
};
        locals.var_isbs2 = assign32000_e46995;
        locals.var_isbs2_dn0 = assign32000_e46995_d_n0;
        locals.var_isbs2_dn2 = assign32000_e46995_d_n2;
        locals.var_isbs2_dn6 = assign32000_e46995_d_n6;
        locals.var_isbs2_dn7 = assign32000_e46995_d_n7;
        locals.var_isbs2_dn10 = assign32000_e46995_d_n10;
        locals.var_isbs2_dn11 = assign32000_e46995_d_n11;
        locals.var_isbs2_dn12 = assign32000_e46995_d_n12;
        locals.var_isbs2_dn17 = assign32000_e46995_d_n17;

        let (assign32010_e47001, assign32010_e47001_d_n6, assign32010_e47001_d_n7, assign32010_e47001_d_n10, assign32010_e47001_d_n12,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32010_e46999: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign32010_e46999, 0.0, 0.0, (locals.var_ttemp_dn10 / locals.var_uc_tnom), 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32010_e47001;
        locals.var_t1__blk1030_dn6 = assign32010_e47001_d_n6;
        locals.var_t1__blk1030_dn7 = assign32010_e47001_d_n7;
        locals.var_t1__blk1030_dn10 = assign32010_e47001_d_n10;
        locals.var_t1__blk1030_dn12 = assign32010_e47001_d_n12;

        let (assign32030_e47013, assign32030_e47013_d_n0, assign32030_e47013_d_n2, assign32030_e47013_d_n6, assign32030_e47013_d_n7, assign32030_e47013_d_n10, assign32030_e47013_d_n11, assign32030_e47013_d_n12, assign32030_e47013_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32030_e47011: f64 = (locals.var_isbd + 1e-50);
        (assign32030_e47011, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17,)
    } else {
        (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17,)
    }
};
        locals.var_t2__blk1031 = assign32030_e47013;
        locals.var_t2__blk1031_dn0 = assign32030_e47013_d_n0;
        locals.var_t2__blk1031_dn2 = assign32030_e47013_d_n2;
        locals.var_t2__blk1031_dn6 = assign32030_e47013_d_n6;
        locals.var_t2__blk1031_dn7 = assign32030_e47013_d_n7;
        locals.var_t2__blk1031_dn10 = assign32030_e47013_d_n10;
        locals.var_t2__blk1031_dn11 = assign32030_e47013_d_n11;
        locals.var_t2__blk1031_dn12 = assign32030_e47013_d_n12;
        locals.var_t2__blk1031_dn17 = assign32030_e47013_d_n17;

        let (assign32050_e47027, assign32050_e47027_d_n10,) = {
    if (locals.var_guard1028 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn10,)
    }
};
        locals.var_vbdt = assign32050_e47027;
        locals.var_vbdt_dn10 = assign32050_e47027_d_n10;

        let (assign32060_e47035, assign32060_e47035_d_n10,) = {
    if (locals.var_guard1028 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vbst, locals.var_vbst_dn10,)
    }
};
        locals.var_vbst = assign32060_e47035;
        locals.var_vbst_dn10 = assign32060_e47035_d_n10;

        let (assign32070_e47041, assign32070_e47041_d_n10,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32070_e47039: f64 = (p.p174 * locals.var_beta_inv);
        (assign32070_e47039, (p.p174 * locals.var_beta_inv_dn10),)
    } else {
        (locals.var_nvtm, locals.var_nvtm_dn10,)
    }
};
        locals.var_nvtm = assign32070_e47041;
        locals.var_nvtm_dn10 = assign32070_e47041_d_n10;

        let assign32080_e47044: f64 = if locals.var_vbdj < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard1057 = assign32080_e47044;

    }

    pub(super) fn stamp_transient_block_114(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32090_e47053, assign32090_e47053_d_n6, assign32090_e47053_d_n7, assign32090_e47053_d_n10, assign32090_e47053_d_n12,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 != 0.0)) {
        let assign32090_e47050: f64 = (locals.var_vbdj / locals.var_nvtm);
        let assign32090_e47051: f64 = (assign32090_e47050).exp();
        (assign32090_e47051, (assign32090_e47051 * (locals.var_vbdj_dn6 / locals.var_nvtm)), 0.0, (assign32090_e47051 * (-((locals.var_vbdj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32090_e47051 * (locals.var_vbdj_dn12 / locals.var_nvtm)),)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32090_e47053;
        locals.var_t1__blk1030_dn6 = assign32090_e47053_d_n6;
        locals.var_t1__blk1030_dn7 = assign32090_e47053_d_n7;
        locals.var_t1__blk1030_dn10 = assign32090_e47053_d_n10;
        locals.var_t1__blk1030_dn12 = assign32090_e47053_d_n12;

        let (assign32100_e47063, assign32100_e47063_d_n0, assign32100_e47063_d_n2, assign32100_e47063_d_n6, assign32100_e47063_d_n7, assign32100_e47063_d_n10, assign32100_e47063_d_n11, assign32100_e47063_d_n12, assign32100_e47063_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 != 0.0)) {
        let assign32100_e47060: f64 = (locals.var_t1__blk1030 - 1.0);
        let assign32100_e47061: f64 = (locals.var_isbd * assign32100_e47060);
        (assign32100_e47061, (locals.var_isbd_dn0 * assign32100_e47060), (locals.var_isbd_dn2 * assign32100_e47060), ((locals.var_isbd_dn6 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn6)), ((locals.var_isbd_dn7 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn7)), ((locals.var_isbd_dn10 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn10)), (locals.var_isbd_dn11 * assign32100_e47060), ((locals.var_isbd_dn12 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn12)), (locals.var_isbd_dn17 * assign32100_e47060),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32100_e47063;
        locals.var_ibd_dn0 = assign32100_e47063_d_n0;
        locals.var_ibd_dn2 = assign32100_e47063_d_n2;
        locals.var_ibd_dn6 = assign32100_e47063_d_n6;
        locals.var_ibd_dn7 = assign32100_e47063_d_n7;
        locals.var_ibd_dn10 = assign32100_e47063_d_n10;
        locals.var_ibd_dn11 = assign32100_e47063_d_n11;
        locals.var_ibd_dn12 = assign32100_e47063_d_n12;
        locals.var_ibd_dn17 = assign32100_e47063_d_n17;

        let (assign32110_e47073, assign32110_e47073_d_n6, assign32110_e47073_d_n7, assign32110_e47073_d_n10, assign32110_e47073_d_n12,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 == 0.0)) {
        let assign32110_e47070: f64 = (locals.var_vbdt / locals.var_nvtm);
        let assign32110_e47071: f64 = (assign32110_e47070).exp();
        (assign32110_e47071, 0.0, 0.0, (assign32110_e47071 * (((locals.var_vbdt_dn10 * locals.var_nvtm) - (locals.var_vbdt * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32110_e47073;
        locals.var_t1__blk1030_dn6 = assign32110_e47073_d_n6;
        locals.var_t1__blk1030_dn7 = assign32110_e47073_d_n7;
        locals.var_t1__blk1030_dn10 = assign32110_e47073_d_n10;
        locals.var_t1__blk1030_dn12 = assign32110_e47073_d_n12;

        let (assign32120_e47094, assign32120_e47094_d_n0, assign32120_e47094_d_n2, assign32120_e47094_d_n6, assign32120_e47094_d_n7, assign32120_e47094_d_n10, assign32120_e47094_d_n11, assign32120_e47094_d_n12, assign32120_e47094_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 == 0.0)) {
        let assign32120_e47081: f64 = (locals.var_t1__blk1030 - 1.0);
        let assign32120_e47082: f64 = (locals.var_isbd * assign32120_e47081);
        let assign32120_e47085: f64 = (locals.var_isbd / locals.var_nvtm);
        let assign32120_e47087: f64 = (assign32120_e47085 * locals.var_t1__blk1030);
        let assign32120_e47090: f64 = (locals.var_vbdj - locals.var_vbdt);
        let assign32120_e47091: f64 = (assign32120_e47087 * assign32120_e47090);
        let assign32120_e47092: f64 = (assign32120_e47082 + assign32120_e47091);
        (assign32120_e47092, ((locals.var_isbd_dn0 * assign32120_e47081) + (((locals.var_isbd_dn0 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), ((locals.var_isbd_dn2 * assign32120_e47081) + (((locals.var_isbd_dn2 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), (((locals.var_isbd_dn6 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn6)) + (((((locals.var_isbd_dn6 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn6)) * assign32120_e47090) + (assign32120_e47087 * locals.var_vbdj_dn6))), (((locals.var_isbd_dn7 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn7)) + ((((locals.var_isbd_dn7 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn7)) * assign32120_e47090)), (((locals.var_isbd_dn10 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn10)) + (((((((locals.var_isbd_dn10 * locals.var_nvtm) - (locals.var_isbd * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn10)) * assign32120_e47090) + (assign32120_e47087 * (-locals.var_vbdt_dn10)))), ((locals.var_isbd_dn11 * assign32120_e47081) + (((locals.var_isbd_dn11 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), (((locals.var_isbd_dn12 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn12)) + (((((locals.var_isbd_dn12 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn12)) * assign32120_e47090) + (assign32120_e47087 * locals.var_vbdj_dn12))), ((locals.var_isbd_dn17 * assign32120_e47081) + (((locals.var_isbd_dn17 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32120_e47094;
        locals.var_ibd_dn0 = assign32120_e47094_d_n0;
        locals.var_ibd_dn2 = assign32120_e47094_d_n2;
        locals.var_ibd_dn6 = assign32120_e47094_d_n6;
        locals.var_ibd_dn7 = assign32120_e47094_d_n7;
        locals.var_ibd_dn10 = assign32120_e47094_d_n10;
        locals.var_ibd_dn11 = assign32120_e47094_d_n11;
        locals.var_ibd_dn12 = assign32120_e47094_d_n12;
        locals.var_ibd_dn17 = assign32120_e47094_d_n17;

        let (assign32130_e47104, assign32130_e47104_d_n0, assign32130_e47104_d_n2, assign32130_e47104_d_n6, assign32130_e47104_d_n7, assign32130_e47104_d_n10, assign32130_e47104_d_n11, assign32130_e47104_d_n12, assign32130_e47104_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32130_e47099: f64 = (p.p178 * locals.var_vbdj);
        let assign32130_e47101: f64 = (assign32130_e47099 * locals.var_isbd2);
        let assign32130_e47102: f64 = (locals.var_ibd + assign32130_e47101);
        (assign32130_e47102, (locals.var_ibd_dn0 + (assign32130_e47099 * locals.var_isbd2_dn0)), (locals.var_ibd_dn2 + (assign32130_e47099 * locals.var_isbd2_dn2)), (locals.var_ibd_dn6 + (((p.p178 * locals.var_vbdj_dn6) * locals.var_isbd2) + (assign32130_e47099 * locals.var_isbd2_dn6))), (locals.var_ibd_dn7 + (assign32130_e47099 * locals.var_isbd2_dn7)), (locals.var_ibd_dn10 + (assign32130_e47099 * locals.var_isbd2_dn10)), (locals.var_ibd_dn11 + (assign32130_e47099 * locals.var_isbd2_dn11)), (locals.var_ibd_dn12 + (((p.p178 * locals.var_vbdj_dn12) * locals.var_isbd2) + (assign32130_e47099 * locals.var_isbd2_dn12))), (locals.var_ibd_dn17 + (assign32130_e47099 * locals.var_isbd2_dn17)),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32130_e47104;
        locals.var_ibd_dn0 = assign32130_e47104_d_n0;
        locals.var_ibd_dn2 = assign32130_e47104_d_n2;
        locals.var_ibd_dn6 = assign32130_e47104_d_n6;
        locals.var_ibd_dn7 = assign32130_e47104_d_n7;
        locals.var_ibd_dn10 = assign32130_e47104_d_n10;
        locals.var_ibd_dn11 = assign32130_e47104_d_n11;
        locals.var_ibd_dn12 = assign32130_e47104_d_n12;
        locals.var_ibd_dn17 = assign32130_e47104_d_n17;

        let assign32140_e47107: f64 = if locals.var_vbsj < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard1058 = assign32140_e47107;

        let (assign32150_e47116, assign32150_e47116_d_n6, assign32150_e47116_d_n7, assign32150_e47116_d_n10, assign32150_e47116_d_n12,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 != 0.0)) {
        let assign32150_e47113: f64 = (locals.var_vbsj / locals.var_nvtm);
        let assign32150_e47114: f64 = (assign32150_e47113).exp();
        (assign32150_e47114, 0.0, (assign32150_e47114 * (locals.var_vbsj_dn7 / locals.var_nvtm)), (assign32150_e47114 * (-((locals.var_vbsj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32150_e47114 * (locals.var_vbsj_dn12 / locals.var_nvtm)),)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32150_e47116;
        locals.var_t1__blk1030_dn6 = assign32150_e47116_d_n6;
        locals.var_t1__blk1030_dn7 = assign32150_e47116_d_n7;
        locals.var_t1__blk1030_dn10 = assign32150_e47116_d_n10;
        locals.var_t1__blk1030_dn12 = assign32150_e47116_d_n12;

        let (assign32160_e47126, assign32160_e47126_d_n0, assign32160_e47126_d_n2, assign32160_e47126_d_n6, assign32160_e47126_d_n7, assign32160_e47126_d_n10, assign32160_e47126_d_n11, assign32160_e47126_d_n12, assign32160_e47126_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 != 0.0)) {
        let assign32160_e47123: f64 = (locals.var_t1__blk1030 - 1.0);
        let assign32160_e47124: f64 = (locals.var_isbs * assign32160_e47123);
        (assign32160_e47124, (locals.var_isbs_dn0 * assign32160_e47123), (locals.var_isbs_dn2 * assign32160_e47123), ((locals.var_isbs_dn6 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn6)), ((locals.var_isbs_dn7 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn7)), ((locals.var_isbs_dn10 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn10)), (locals.var_isbs_dn11 * assign32160_e47123), ((locals.var_isbs_dn12 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn12)), (locals.var_isbs_dn17 * assign32160_e47123),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32160_e47126;
        locals.var_ibs_dn0 = assign32160_e47126_d_n0;
        locals.var_ibs_dn2 = assign32160_e47126_d_n2;
        locals.var_ibs_dn6 = assign32160_e47126_d_n6;
        locals.var_ibs_dn7 = assign32160_e47126_d_n7;
        locals.var_ibs_dn10 = assign32160_e47126_d_n10;
        locals.var_ibs_dn11 = assign32160_e47126_d_n11;
        locals.var_ibs_dn12 = assign32160_e47126_d_n12;
        locals.var_ibs_dn17 = assign32160_e47126_d_n17;

        let (assign32170_e47136, assign32170_e47136_d_n6, assign32170_e47136_d_n7, assign32170_e47136_d_n10, assign32170_e47136_d_n12,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 == 0.0)) {
        let assign32170_e47133: f64 = (locals.var_vbst / locals.var_nvtm);
        let assign32170_e47134: f64 = (assign32170_e47133).exp();
        (assign32170_e47134, 0.0, 0.0, (assign32170_e47134 * (((locals.var_vbst_dn10 * locals.var_nvtm) - (locals.var_vbst * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32170_e47136;
        locals.var_t1__blk1030_dn6 = assign32170_e47136_d_n6;
        locals.var_t1__blk1030_dn7 = assign32170_e47136_d_n7;
        locals.var_t1__blk1030_dn10 = assign32170_e47136_d_n10;
        locals.var_t1__blk1030_dn12 = assign32170_e47136_d_n12;

        let (assign32180_e47157, assign32180_e47157_d_n0, assign32180_e47157_d_n2, assign32180_e47157_d_n6, assign32180_e47157_d_n7, assign32180_e47157_d_n10, assign32180_e47157_d_n11, assign32180_e47157_d_n12, assign32180_e47157_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 == 0.0)) {
        let assign32180_e47144: f64 = (locals.var_t1__blk1030 - 1.0);
        let assign32180_e47145: f64 = (locals.var_isbs * assign32180_e47144);
        let assign32180_e47148: f64 = (locals.var_isbs / locals.var_nvtm);
        let assign32180_e47150: f64 = (assign32180_e47148 * locals.var_t1__blk1030);
        let assign32180_e47153: f64 = (locals.var_vbsj - locals.var_vbst);
        let assign32180_e47154: f64 = (assign32180_e47150 * assign32180_e47153);
        let assign32180_e47155: f64 = (assign32180_e47145 + assign32180_e47154);
        (assign32180_e47155, ((locals.var_isbs_dn0 * assign32180_e47144) + (((locals.var_isbs_dn0 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), ((locals.var_isbs_dn2 * assign32180_e47144) + (((locals.var_isbs_dn2 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), (((locals.var_isbs_dn6 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn6)) + ((((locals.var_isbs_dn6 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn6)) * assign32180_e47153)), (((locals.var_isbs_dn7 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn7)) + (((((locals.var_isbs_dn7 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn7)) * assign32180_e47153) + (assign32180_e47150 * locals.var_vbsj_dn7))), (((locals.var_isbs_dn10 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn10)) + (((((((locals.var_isbs_dn10 * locals.var_nvtm) - (locals.var_isbs * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn10)) * assign32180_e47153) + (assign32180_e47150 * (-locals.var_vbst_dn10)))), ((locals.var_isbs_dn11 * assign32180_e47144) + (((locals.var_isbs_dn11 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), (((locals.var_isbs_dn12 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn12)) + (((((locals.var_isbs_dn12 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn12)) * assign32180_e47153) + (assign32180_e47150 * locals.var_vbsj_dn12))), ((locals.var_isbs_dn17 * assign32180_e47144) + (((locals.var_isbs_dn17 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32180_e47157;
        locals.var_ibs_dn0 = assign32180_e47157_d_n0;
        locals.var_ibs_dn2 = assign32180_e47157_d_n2;
        locals.var_ibs_dn6 = assign32180_e47157_d_n6;
        locals.var_ibs_dn7 = assign32180_e47157_d_n7;
        locals.var_ibs_dn10 = assign32180_e47157_d_n10;
        locals.var_ibs_dn11 = assign32180_e47157_d_n11;
        locals.var_ibs_dn12 = assign32180_e47157_d_n12;
        locals.var_ibs_dn17 = assign32180_e47157_d_n17;

        let (assign32190_e47167, assign32190_e47167_d_n0, assign32190_e47167_d_n2, assign32190_e47167_d_n6, assign32190_e47167_d_n7, assign32190_e47167_d_n10, assign32190_e47167_d_n11, assign32190_e47167_d_n12, assign32190_e47167_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32190_e47162: f64 = (p.p178 * locals.var_vbsj);
        let assign32190_e47164: f64 = (assign32190_e47162 * locals.var_isbs2);
        let assign32190_e47165: f64 = (locals.var_ibs + assign32190_e47164);
        (assign32190_e47165, (locals.var_ibs_dn0 + (assign32190_e47162 * locals.var_isbs2_dn0)), (locals.var_ibs_dn2 + (assign32190_e47162 * locals.var_isbs2_dn2)), (locals.var_ibs_dn6 + (assign32190_e47162 * locals.var_isbs2_dn6)), (locals.var_ibs_dn7 + (((p.p178 * locals.var_vbsj_dn7) * locals.var_isbs2) + (assign32190_e47162 * locals.var_isbs2_dn7))), (locals.var_ibs_dn10 + (assign32190_e47162 * locals.var_isbs2_dn10)), (locals.var_ibs_dn11 + (assign32190_e47162 * locals.var_isbs2_dn11)), (locals.var_ibs_dn12 + (((p.p178 * locals.var_vbsj_dn12) * locals.var_isbs2) + (assign32190_e47162 * locals.var_isbs2_dn12))), (locals.var_ibs_dn17 + (assign32190_e47162 * locals.var_isbs2_dn17)),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32190_e47167;
        locals.var_ibs_dn0 = assign32190_e47167_d_n0;
        locals.var_ibs_dn2 = assign32190_e47167_d_n2;
        locals.var_ibs_dn6 = assign32190_e47167_d_n6;
        locals.var_ibs_dn7 = assign32190_e47167_d_n7;
        locals.var_ibs_dn10 = assign32190_e47167_d_n10;
        locals.var_ibs_dn11 = assign32190_e47167_d_n11;
        locals.var_ibs_dn12 = assign32190_e47167_d_n12;
        locals.var_ibs_dn17 = assign32190_e47167_d_n17;

        let (assign32200_e47175, assign32200_e47175_d_n0, assign32200_e47175_d_n2, assign32200_e47175_d_n6, assign32200_e47175_d_n7, assign32200_e47175_d_n10, assign32200_e47175_d_n11, assign32200_e47175_d_n12, assign32200_e47175_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32200_e47172: f64 = (locals.var_gjmin * locals.var_vbdj);
        let assign32200_e47173: f64 = (locals.var_ibd + assign32200_e47172);
        (assign32200_e47173, locals.var_ibd_dn0, locals.var_ibd_dn2, (locals.var_ibd_dn6 + (locals.var_gjmin * locals.var_vbdj_dn6)), locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, (locals.var_ibd_dn12 + (locals.var_gjmin * locals.var_vbdj_dn12)), locals.var_ibd_dn17,)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32200_e47175;
        locals.var_ibd_dn0 = assign32200_e47175_d_n0;
        locals.var_ibd_dn2 = assign32200_e47175_d_n2;
        locals.var_ibd_dn6 = assign32200_e47175_d_n6;
        locals.var_ibd_dn7 = assign32200_e47175_d_n7;
        locals.var_ibd_dn10 = assign32200_e47175_d_n10;
        locals.var_ibd_dn11 = assign32200_e47175_d_n11;
        locals.var_ibd_dn12 = assign32200_e47175_d_n12;
        locals.var_ibd_dn17 = assign32200_e47175_d_n17;

        let (assign32210_e47183, assign32210_e47183_d_n0, assign32210_e47183_d_n2, assign32210_e47183_d_n6, assign32210_e47183_d_n7, assign32210_e47183_d_n10, assign32210_e47183_d_n11, assign32210_e47183_d_n12, assign32210_e47183_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32210_e47180: f64 = (locals.var_gjmin * locals.var_vbsj);
        let assign32210_e47181: f64 = (locals.var_ibs + assign32210_e47180);
        (assign32210_e47181, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, (locals.var_ibs_dn7 + (locals.var_gjmin * locals.var_vbsj_dn7)), locals.var_ibs_dn10, locals.var_ibs_dn11, (locals.var_ibs_dn12 + (locals.var_gjmin * locals.var_vbsj_dn12)), locals.var_ibs_dn17,)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32210_e47183;
        locals.var_ibs_dn0 = assign32210_e47183_d_n0;
        locals.var_ibs_dn2 = assign32210_e47183_d_n2;
        locals.var_ibs_dn6 = assign32210_e47183_d_n6;
        locals.var_ibs_dn7 = assign32210_e47183_d_n7;
        locals.var_ibs_dn10 = assign32210_e47183_d_n10;
        locals.var_ibs_dn11 = assign32210_e47183_d_n11;
        locals.var_ibs_dn12 = assign32210_e47183_d_n12;
        locals.var_ibs_dn17 = assign32210_e47183_d_n17;

        let (assign32220_e47189,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32220_e47187: f64 = (p.p179 * p.p2);
        (assign32220_e47187,)
    } else {
        (locals.var_czbd,)
    }
};
        locals.var_czbd = assign32220_e47189;

        let (assign32230_e47195,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32230_e47193: f64 = (p.p179 * p.p3);
        (assign32230_e47193,)
    } else {
        (locals.var_czbs,)
    }
};
        locals.var_czbs = assign32230_e47195;

        let (assign32240_e47201,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32240_e47199: f64 = (p.p237 - p.p238);
        (assign32240_e47199,)
    } else {
        (locals.var_xp_max,)
    }
};
        locals.var_xp_max = assign32240_e47201;

        let assign32250_e47204: f64 = if locals.var_xp_max <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1059 = assign32250_e47204;

        let (assign32260_e47210,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1059 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_czbd,)
    }
};
        locals.var_czbd = assign32260_e47210;

        let (assign32270_e47216,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1059 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_czbs,)
    }
};
        locals.var_czbs = assign32270_e47216;

        let assign32280_e47219: f64 = if p.p5 > locals.var_w_dioscv { 1.0 } else { 0.0 };
        locals.var_guard1060 = assign32280_e47219;

        let (assign32290_e47229,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) {
        let assign32290_e47226: f64 = (p.p5 - locals.var_w_dioscv);
        let assign32290_e47227: f64 = (p.p180 * assign32290_e47226);
        (assign32290_e47227,)
    } else {
        (locals.var_czbssw,)
    }
};
        locals.var_czbssw = assign32290_e47229;

        let (assign32300_e47237,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) {
        let assign32300_e47235: f64 = (p.p181 * locals.var_w_dioscv);
        (assign32300_e47235,)
    } else {
        (locals.var_czbsswg,)
    }
};
        locals.var_czbsswg = assign32300_e47237;

        let assign32310_e47240: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign32310_e47240;

        let assign32320_e47243: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign32320_e47243;

        let (assign32330_e47257, assign32330_e47257_d_n6, assign32330_e47257_d_n7, assign32330_e47257_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign32330_e47254: f64 = (locals.var_vbsj / p.p185);
        let assign32330_e47255: f64 = (1.0 - assign32330_e47254);
        (assign32330_e47255, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32330_e47257;
        locals.var_arg__blk1055_dn6 = assign32330_e47257_d_n6;
        locals.var_arg__blk1055_dn7 = assign32330_e47257_d_n7;
        locals.var_arg__blk1055_dn12 = assign32330_e47257_d_n12;

        let assign32340_e47260: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign32340_e47260;

        let (assign32350_e47275, assign32350_e47275_d_n6, assign32350_e47275_d_n7, assign32350_e47275_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) {
        let assign32350_e47272: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32350_e47273: f64 = (1.0 / assign32350_e47272);
        (assign32350_e47273, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32350_e47275;
        locals.var_sarg_dn6 = assign32350_e47275_d_n6;
        locals.var_sarg_dn7 = assign32350_e47275_d_n7;
        locals.var_sarg_dn12 = assign32350_e47275_d_n12;

        let (assign32360_e47291, assign32360_e47291_d_n6, assign32360_e47291_d_n7, assign32360_e47291_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
        let assign32360_e47288: f64 = (-p.p182);
        let assign32360_e47289: f64 = (locals.var_arg__blk1055).powf(assign32360_e47288);
        (assign32360_e47289, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32360_e47291;
        locals.var_sarg_dn6 = assign32360_e47291_d_n6;
        locals.var_sarg_dn7 = assign32360_e47291_d_n7;
        locals.var_sarg_dn12 = assign32360_e47291_d_n12;

        let (assign32370_e47313, assign32370_e47313_d_n0, assign32370_e47313_d_n2, assign32370_e47313_d_n6, assign32370_e47313_d_n7, assign32370_e47313_d_n10, assign32370_e47313_d_n11, assign32370_e47313_d_n12, assign32370_e47313_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign32370_e47301: f64 = (p.p185 * locals.var_czbs);
        let assign32370_e47305: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32370_e47306: f64 = (1.0 - assign32370_e47305);
        let assign32370_e47307: f64 = (assign32370_e47301 * assign32370_e47306);
        let assign32370_e47310: f64 = (1.0 - p.p182);
        let assign32370_e47311: f64 = (assign32370_e47307 / assign32370_e47310);
        (assign32370_e47311, 0.0, 0.0, ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32370_e47310), ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32370_e47310), 0.0, 0.0, ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32370_e47310), 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32370_e47313;
        locals.var_qbs_dn0 = assign32370_e47313_d_n0;
        locals.var_qbs_dn2 = assign32370_e47313_d_n2;
        locals.var_qbs_dn6 = assign32370_e47313_d_n6;
        locals.var_qbs_dn7 = assign32370_e47313_d_n7;
        locals.var_qbs_dn10 = assign32370_e47313_d_n10;
        locals.var_qbs_dn11 = assign32370_e47313_d_n11;
        locals.var_qbs_dn12 = assign32370_e47313_d_n12;
        locals.var_qbs_dn17 = assign32370_e47313_d_n17;

        let (assign32380_e47324, assign32380_e47324_d_n0, assign32380_e47324_d_n2, assign32380_e47324_d_n6, assign32380_e47324_d_n7, assign32380_e47324_d_n10, assign32380_e47324_d_n11, assign32380_e47324_d_n12, assign32380_e47324_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32380_e47324;
        locals.var_qbs_dn0 = assign32380_e47324_d_n0;
        locals.var_qbs_dn2 = assign32380_e47324_d_n2;
        locals.var_qbs_dn6 = assign32380_e47324_d_n6;
        locals.var_qbs_dn7 = assign32380_e47324_d_n7;
        locals.var_qbs_dn10 = assign32380_e47324_d_n10;
        locals.var_qbs_dn11 = assign32380_e47324_d_n11;
        locals.var_qbs_dn12 = assign32380_e47324_d_n12;
        locals.var_qbs_dn17 = assign32380_e47324_d_n17;

        let assign32390_e47327: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign32390_e47327;

        let (assign32400_e47341, assign32400_e47341_d_n6, assign32400_e47341_d_n7, assign32400_e47341_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign32400_e47338: f64 = (locals.var_vbsj / p.p186);
        let assign32400_e47339: f64 = (1.0 - assign32400_e47338);
        (assign32400_e47339, 0.0, (-(locals.var_vbsj_dn7 / p.p186)), (-(locals.var_vbsj_dn12 / p.p186)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32400_e47341;
        locals.var_arg__blk1055_dn6 = assign32400_e47341_d_n6;
        locals.var_arg__blk1055_dn7 = assign32400_e47341_d_n7;
        locals.var_arg__blk1055_dn12 = assign32400_e47341_d_n12;

        let assign32410_e47344: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign32410_e47344;

        let (assign32420_e47359, assign32420_e47359_d_n6, assign32420_e47359_d_n7, assign32420_e47359_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) {
        let assign32420_e47356: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32420_e47357: f64 = (1.0 / assign32420_e47356);
        (assign32420_e47357, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32420_e47359;
        locals.var_sarg_dn6 = assign32420_e47359_d_n6;
        locals.var_sarg_dn7 = assign32420_e47359_d_n7;
        locals.var_sarg_dn12 = assign32420_e47359_d_n12;

        let (assign32430_e47375, assign32430_e47375_d_n6, assign32430_e47375_d_n7, assign32430_e47375_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
        let assign32430_e47372: f64 = (-p.p183);
        let assign32430_e47373: f64 = (locals.var_arg__blk1055).powf(assign32430_e47372);
        (assign32430_e47373, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32430_e47375;
        locals.var_sarg_dn6 = assign32430_e47375_d_n6;
        locals.var_sarg_dn7 = assign32430_e47375_d_n7;
        locals.var_sarg_dn12 = assign32430_e47375_d_n12;

        let (assign32440_e47399, assign32440_e47399_d_n0, assign32440_e47399_d_n2, assign32440_e47399_d_n6, assign32440_e47399_d_n7, assign32440_e47399_d_n10, assign32440_e47399_d_n11, assign32440_e47399_d_n12, assign32440_e47399_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign32440_e47386: f64 = (p.p186 * locals.var_czbssw);
        let assign32440_e47390: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32440_e47391: f64 = (1.0 - assign32440_e47390);
        let assign32440_e47392: f64 = (assign32440_e47386 * assign32440_e47391);
        let assign32440_e47395: f64 = (1.0 - p.p183);
        let assign32440_e47396: f64 = (assign32440_e47392 / assign32440_e47395);
        let assign32440_e47397: f64 = (locals.var_qbs + assign32440_e47396);
        (assign32440_e47397, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32440_e47395)), (locals.var_qbs_dn7 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32440_e47395)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32440_e47395)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32440_e47399;
        locals.var_qbs_dn0 = assign32440_e47399_d_n0;
        locals.var_qbs_dn2 = assign32440_e47399_d_n2;
        locals.var_qbs_dn6 = assign32440_e47399_d_n6;
        locals.var_qbs_dn7 = assign32440_e47399_d_n7;
        locals.var_qbs_dn10 = assign32440_e47399_d_n10;
        locals.var_qbs_dn11 = assign32440_e47399_d_n11;
        locals.var_qbs_dn12 = assign32440_e47399_d_n12;
        locals.var_qbs_dn17 = assign32440_e47399_d_n17;

        let assign32450_e47402: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign32450_e47402;

        let (assign32460_e47416, assign32460_e47416_d_n6, assign32460_e47416_d_n7, assign32460_e47416_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign32460_e47413: f64 = (locals.var_vbsj / p.p187);
        let assign32460_e47414: f64 = (1.0 - assign32460_e47413);
        (assign32460_e47414, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32460_e47416;
        locals.var_arg__blk1055_dn6 = assign32460_e47416_d_n6;
        locals.var_arg__blk1055_dn7 = assign32460_e47416_d_n7;
        locals.var_arg__blk1055_dn12 = assign32460_e47416_d_n12;

        let assign32470_e47419: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign32470_e47419;

        let (assign32480_e47434, assign32480_e47434_d_n6, assign32480_e47434_d_n7, assign32480_e47434_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
        let assign32480_e47431: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32480_e47432: f64 = (1.0 / assign32480_e47431);
        (assign32480_e47432, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32480_e47434;
        locals.var_sarg_dn6 = assign32480_e47434_d_n6;
        locals.var_sarg_dn7 = assign32480_e47434_d_n7;
        locals.var_sarg_dn12 = assign32480_e47434_d_n12;

    }

    pub(super) fn stamp_transient_block_115(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32490_e47450, assign32490_e47450_d_n6, assign32490_e47450_d_n7, assign32490_e47450_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 == 0.0)) {
        let assign32490_e47447: f64 = (-p.p184);
        let assign32490_e47448: f64 = (locals.var_arg__blk1055).powf(assign32490_e47447);
        (assign32490_e47448, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32490_e47450;
        locals.var_sarg_dn6 = assign32490_e47450_d_n6;
        locals.var_sarg_dn7 = assign32490_e47450_d_n7;
        locals.var_sarg_dn12 = assign32490_e47450_d_n12;

        let (assign32500_e47474, assign32500_e47474_d_n0, assign32500_e47474_d_n2, assign32500_e47474_d_n6, assign32500_e47474_d_n7, assign32500_e47474_d_n10, assign32500_e47474_d_n11, assign32500_e47474_d_n12, assign32500_e47474_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign32500_e47461: f64 = (p.p187 * locals.var_czbsswg);
        let assign32500_e47465: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32500_e47466: f64 = (1.0 - assign32500_e47465);
        let assign32500_e47467: f64 = (assign32500_e47461 * assign32500_e47466);
        let assign32500_e47470: f64 = (1.0 - p.p184);
        let assign32500_e47471: f64 = (assign32500_e47467 / assign32500_e47470);
        let assign32500_e47472: f64 = (locals.var_qbs + assign32500_e47471);
        (assign32500_e47472, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32500_e47470)), (locals.var_qbs_dn7 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32500_e47470)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32500_e47470)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32500_e47474;
        locals.var_qbs_dn0 = assign32500_e47474_d_n0;
        locals.var_qbs_dn2 = assign32500_e47474_d_n2;
        locals.var_qbs_dn6 = assign32500_e47474_d_n6;
        locals.var_qbs_dn7 = assign32500_e47474_d_n7;
        locals.var_qbs_dn10 = assign32500_e47474_d_n10;
        locals.var_qbs_dn11 = assign32500_e47474_d_n11;
        locals.var_qbs_dn12 = assign32500_e47474_d_n12;
        locals.var_qbs_dn17 = assign32500_e47474_d_n17;

        let (assign32510_e47487, assign32510_e47487_d_n6, assign32510_e47487_d_n7, assign32510_e47487_d_n10, assign32510_e47487_d_n12,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
        let assign32510_e47483: f64 = (locals.var_czbs + locals.var_czbssw);
        let assign32510_e47485: f64 = (assign32510_e47483 + locals.var_czbsswg);
        (assign32510_e47485, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32510_e47487;
        locals.var_t1__blk1030_dn6 = assign32510_e47487_d_n6;
        locals.var_t1__blk1030_dn7 = assign32510_e47487_d_n7;
        locals.var_t1__blk1030_dn10 = assign32510_e47487_d_n10;
        locals.var_t1__blk1030_dn12 = assign32510_e47487_d_n12;

        let (assign32520_e47512, assign32520_e47512_d_n0, assign32520_e47512_d_n2, assign32520_e47512_d_n6, assign32520_e47512_d_n7, assign32520_e47512_d_n10, assign32520_e47512_d_n11, assign32520_e47512_d_n12, assign32520_e47512_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
        let assign32520_e47496: f64 = (locals.var_czbs * p.p182);
        let assign32520_e47498: f64 = (assign32520_e47496 / p.p185);
        let assign32520_e47501: f64 = (locals.var_czbssw * p.p183);
        let assign32520_e47503: f64 = (assign32520_e47501 / p.p186);
        let assign32520_e47504: f64 = (assign32520_e47498 + assign32520_e47503);
        let assign32520_e47507: f64 = (locals.var_czbsswg * p.p184);
        let assign32520_e47509: f64 = (assign32520_e47507 / p.p187);
        let assign32520_e47510: f64 = (assign32520_e47504 + assign32520_e47509);
        (assign32520_e47510, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17,)
    }
};
        locals.var_t2__blk1031 = assign32520_e47512;
        locals.var_t2__blk1031_dn0 = assign32520_e47512_d_n0;
        locals.var_t2__blk1031_dn2 = assign32520_e47512_d_n2;
        locals.var_t2__blk1031_dn6 = assign32520_e47512_d_n6;
        locals.var_t2__blk1031_dn7 = assign32520_e47512_d_n7;
        locals.var_t2__blk1031_dn10 = assign32520_e47512_d_n10;
        locals.var_t2__blk1031_dn11 = assign32520_e47512_d_n11;
        locals.var_t2__blk1031_dn12 = assign32520_e47512_d_n12;
        locals.var_t2__blk1031_dn17 = assign32520_e47512_d_n17;

        let (assign32530_e47529, assign32530_e47529_d_n0, assign32530_e47529_d_n2, assign32530_e47529_d_n6, assign32530_e47529_d_n7, assign32530_e47529_d_n10, assign32530_e47529_d_n11, assign32530_e47529_d_n12, assign32530_e47529_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
        let assign32530_e47523: f64 = (locals.var_vbsj * 0.5);
        let assign32530_e47525: f64 = (assign32530_e47523 * locals.var_t2__blk1031);
        let assign32530_e47526: f64 = (locals.var_t1__blk1030 + assign32530_e47525);
        let assign32530_e47527: f64 = (locals.var_vbsj * assign32530_e47526);
        (assign32530_e47527, (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn0)), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn2)), (locals.var_vbsj * (locals.var_t1__blk1030_dn6 + (assign32530_e47523 * locals.var_t2__blk1031_dn6))), ((locals.var_vbsj_dn7 * assign32530_e47526) + (locals.var_vbsj * (locals.var_t1__blk1030_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1031) + (assign32530_e47523 * locals.var_t2__blk1031_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1030_dn10 + (assign32530_e47523 * locals.var_t2__blk1031_dn10))), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn11)), ((locals.var_vbsj_dn12 * assign32530_e47526) + (locals.var_vbsj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32530_e47523 * locals.var_t2__blk1031_dn12))))), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn17)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32530_e47529;
        locals.var_qbs_dn0 = assign32530_e47529_d_n0;
        locals.var_qbs_dn2 = assign32530_e47529_d_n2;
        locals.var_qbs_dn6 = assign32530_e47529_d_n6;
        locals.var_qbs_dn7 = assign32530_e47529_d_n7;
        locals.var_qbs_dn10 = assign32530_e47529_d_n10;
        locals.var_qbs_dn11 = assign32530_e47529_d_n11;
        locals.var_qbs_dn12 = assign32530_e47529_d_n12;
        locals.var_qbs_dn17 = assign32530_e47529_d_n17;

        let (assign32540_e47538,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) {
        let assign32540_e47536: f64 = (p.p181 * p.p5);
        (assign32540_e47536,)
    } else {
        (locals.var_czbsswg,)
    }
};
        locals.var_czbsswg = assign32540_e47538;

        let assign32550_e47541: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign32550_e47541;

        let assign32560_e47544: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign32560_e47544;

        let (assign32570_e47559, assign32570_e47559_d_n6, assign32570_e47559_d_n7, assign32570_e47559_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
        let assign32570_e47556: f64 = (locals.var_vbsj / p.p185);
        let assign32570_e47557: f64 = (1.0 - assign32570_e47556);
        (assign32570_e47557, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32570_e47559;
        locals.var_arg__blk1055_dn6 = assign32570_e47559_d_n6;
        locals.var_arg__blk1055_dn7 = assign32570_e47559_d_n7;
        locals.var_arg__blk1055_dn12 = assign32570_e47559_d_n12;

        let assign32580_e47562: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign32580_e47562;

        let (assign32590_e47578, assign32590_e47578_d_n6, assign32590_e47578_d_n7, assign32590_e47578_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 != 0.0)) {
        let assign32590_e47575: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32590_e47576: f64 = (1.0 / assign32590_e47575);
        (assign32590_e47576, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32590_e47578;
        locals.var_sarg_dn6 = assign32590_e47578_d_n6;
        locals.var_sarg_dn7 = assign32590_e47578_d_n7;
        locals.var_sarg_dn12 = assign32590_e47578_d_n12;

        let (assign32600_e47595, assign32600_e47595_d_n6, assign32600_e47595_d_n7, assign32600_e47595_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 == 0.0)) {
        let assign32600_e47592: f64 = (-p.p182);
        let assign32600_e47593: f64 = (locals.var_arg__blk1055).powf(assign32600_e47592);
        (assign32600_e47593, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32600_e47595;
        locals.var_sarg_dn6 = assign32600_e47595_d_n6;
        locals.var_sarg_dn7 = assign32600_e47595_d_n7;
        locals.var_sarg_dn12 = assign32600_e47595_d_n12;

        let (assign32610_e47618, assign32610_e47618_d_n0, assign32610_e47618_d_n2, assign32610_e47618_d_n6, assign32610_e47618_d_n7, assign32610_e47618_d_n10, assign32610_e47618_d_n11, assign32610_e47618_d_n12, assign32610_e47618_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
        let assign32610_e47606: f64 = (p.p185 * locals.var_czbs);
        let assign32610_e47610: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32610_e47611: f64 = (1.0 - assign32610_e47610);
        let assign32610_e47612: f64 = (assign32610_e47606 * assign32610_e47611);
        let assign32610_e47615: f64 = (1.0 - p.p182);
        let assign32610_e47616: f64 = (assign32610_e47612 / assign32610_e47615);
        (assign32610_e47616, 0.0, 0.0, ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32610_e47615), ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32610_e47615), 0.0, 0.0, ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32610_e47615), 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32610_e47618;
        locals.var_qbs_dn0 = assign32610_e47618_d_n0;
        locals.var_qbs_dn2 = assign32610_e47618_d_n2;
        locals.var_qbs_dn6 = assign32610_e47618_d_n6;
        locals.var_qbs_dn7 = assign32610_e47618_d_n7;
        locals.var_qbs_dn10 = assign32610_e47618_d_n10;
        locals.var_qbs_dn11 = assign32610_e47618_d_n11;
        locals.var_qbs_dn12 = assign32610_e47618_d_n12;
        locals.var_qbs_dn17 = assign32610_e47618_d_n17;

        let (assign32620_e47630, assign32620_e47630_d_n0, assign32620_e47630_d_n2, assign32620_e47630_d_n6, assign32620_e47630_d_n7, assign32620_e47630_d_n10, assign32620_e47630_d_n11, assign32620_e47630_d_n12, assign32620_e47630_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32620_e47630;
        locals.var_qbs_dn0 = assign32620_e47630_d_n0;
        locals.var_qbs_dn2 = assign32620_e47630_d_n2;
        locals.var_qbs_dn6 = assign32620_e47630_d_n6;
        locals.var_qbs_dn7 = assign32620_e47630_d_n7;
        locals.var_qbs_dn10 = assign32620_e47630_d_n10;
        locals.var_qbs_dn11 = assign32620_e47630_d_n11;
        locals.var_qbs_dn12 = assign32620_e47630_d_n12;
        locals.var_qbs_dn17 = assign32620_e47630_d_n17;

        let assign32630_e47633: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign32630_e47633;

        let (assign32640_e47648, assign32640_e47648_d_n6, assign32640_e47648_d_n7, assign32640_e47648_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) {
        let assign32640_e47645: f64 = (locals.var_vbsj / p.p187);
        let assign32640_e47646: f64 = (1.0 - assign32640_e47645);
        (assign32640_e47646, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32640_e47648;
        locals.var_arg__blk1055_dn6 = assign32640_e47648_d_n6;
        locals.var_arg__blk1055_dn7 = assign32640_e47648_d_n7;
        locals.var_arg__blk1055_dn12 = assign32640_e47648_d_n12;

        let assign32650_e47651: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign32650_e47651;

        let (assign32660_e47667, assign32660_e47667_d_n6, assign32660_e47667_d_n7, assign32660_e47667_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 != 0.0)) {
        let assign32660_e47664: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32660_e47665: f64 = (1.0 / assign32660_e47664);
        (assign32660_e47665, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32660_e47667;
        locals.var_sarg_dn6 = assign32660_e47667_d_n6;
        locals.var_sarg_dn7 = assign32660_e47667_d_n7;
        locals.var_sarg_dn12 = assign32660_e47667_d_n12;

        let (assign32670_e47684, assign32670_e47684_d_n6, assign32670_e47684_d_n7, assign32670_e47684_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 == 0.0)) {
        let assign32670_e47681: f64 = (-p.p184);
        let assign32670_e47682: f64 = (locals.var_arg__blk1055).powf(assign32670_e47681);
        (assign32670_e47682, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32670_e47684;
        locals.var_sarg_dn6 = assign32670_e47684_d_n6;
        locals.var_sarg_dn7 = assign32670_e47684_d_n7;
        locals.var_sarg_dn12 = assign32670_e47684_d_n12;

        let (assign32680_e47709, assign32680_e47709_d_n0, assign32680_e47709_d_n2, assign32680_e47709_d_n6, assign32680_e47709_d_n7, assign32680_e47709_d_n10, assign32680_e47709_d_n11, assign32680_e47709_d_n12, assign32680_e47709_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) {
        let assign32680_e47696: f64 = (p.p187 * locals.var_czbsswg);
        let assign32680_e47700: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32680_e47701: f64 = (1.0 - assign32680_e47700);
        let assign32680_e47702: f64 = (assign32680_e47696 * assign32680_e47701);
        let assign32680_e47705: f64 = (1.0 - p.p184);
        let assign32680_e47706: f64 = (assign32680_e47702 / assign32680_e47705);
        let assign32680_e47707: f64 = (locals.var_qbs + assign32680_e47706);
        (assign32680_e47707, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32680_e47705)), (locals.var_qbs_dn7 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32680_e47705)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32680_e47705)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32680_e47709;
        locals.var_qbs_dn0 = assign32680_e47709_d_n0;
        locals.var_qbs_dn2 = assign32680_e47709_d_n2;
        locals.var_qbs_dn6 = assign32680_e47709_d_n6;
        locals.var_qbs_dn7 = assign32680_e47709_d_n7;
        locals.var_qbs_dn10 = assign32680_e47709_d_n10;
        locals.var_qbs_dn11 = assign32680_e47709_d_n11;
        locals.var_qbs_dn12 = assign32680_e47709_d_n12;
        locals.var_qbs_dn17 = assign32680_e47709_d_n17;

        let (assign32690_e47721, assign32690_e47721_d_n6, assign32690_e47721_d_n7, assign32690_e47721_d_n10, assign32690_e47721_d_n12,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
        let assign32690_e47719: f64 = (locals.var_czbs + locals.var_czbsswg);
        (assign32690_e47719, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32690_e47721;
        locals.var_t1__blk1030_dn6 = assign32690_e47721_d_n6;
        locals.var_t1__blk1030_dn7 = assign32690_e47721_d_n7;
        locals.var_t1__blk1030_dn10 = assign32690_e47721_d_n10;
        locals.var_t1__blk1030_dn12 = assign32690_e47721_d_n12;

        let (assign32700_e47741, assign32700_e47741_d_n0, assign32700_e47741_d_n2, assign32700_e47741_d_n6, assign32700_e47741_d_n7, assign32700_e47741_d_n10, assign32700_e47741_d_n11, assign32700_e47741_d_n12, assign32700_e47741_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
        let assign32700_e47731: f64 = (locals.var_czbs * p.p182);
        let assign32700_e47733: f64 = (assign32700_e47731 / p.p185);
        let assign32700_e47736: f64 = (locals.var_czbsswg * p.p184);
        let assign32700_e47738: f64 = (assign32700_e47736 / p.p187);
        let assign32700_e47739: f64 = (assign32700_e47733 + assign32700_e47738);
        (assign32700_e47739, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17,)
    }
};
        locals.var_t2__blk1031 = assign32700_e47741;
        locals.var_t2__blk1031_dn0 = assign32700_e47741_d_n0;
        locals.var_t2__blk1031_dn2 = assign32700_e47741_d_n2;
        locals.var_t2__blk1031_dn6 = assign32700_e47741_d_n6;
        locals.var_t2__blk1031_dn7 = assign32700_e47741_d_n7;
        locals.var_t2__blk1031_dn10 = assign32700_e47741_d_n10;
        locals.var_t2__blk1031_dn11 = assign32700_e47741_d_n11;
        locals.var_t2__blk1031_dn12 = assign32700_e47741_d_n12;
        locals.var_t2__blk1031_dn17 = assign32700_e47741_d_n17;

        let (assign32710_e47759, assign32710_e47759_d_n0, assign32710_e47759_d_n2, assign32710_e47759_d_n6, assign32710_e47759_d_n7, assign32710_e47759_d_n10, assign32710_e47759_d_n11, assign32710_e47759_d_n12, assign32710_e47759_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
        let assign32710_e47753: f64 = (locals.var_vbsj * 0.5);
        let assign32710_e47755: f64 = (assign32710_e47753 * locals.var_t2__blk1031);
        let assign32710_e47756: f64 = (locals.var_t1__blk1030 + assign32710_e47755);
        let assign32710_e47757: f64 = (locals.var_vbsj * assign32710_e47756);
        (assign32710_e47757, (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn0)), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn2)), (locals.var_vbsj * (locals.var_t1__blk1030_dn6 + (assign32710_e47753 * locals.var_t2__blk1031_dn6))), ((locals.var_vbsj_dn7 * assign32710_e47756) + (locals.var_vbsj * (locals.var_t1__blk1030_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1031) + (assign32710_e47753 * locals.var_t2__blk1031_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1030_dn10 + (assign32710_e47753 * locals.var_t2__blk1031_dn10))), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn11)), ((locals.var_vbsj_dn12 * assign32710_e47756) + (locals.var_vbsj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32710_e47753 * locals.var_t2__blk1031_dn12))))), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn17)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32710_e47759;
        locals.var_qbs_dn0 = assign32710_e47759_d_n0;
        locals.var_qbs_dn2 = assign32710_e47759_d_n2;
        locals.var_qbs_dn6 = assign32710_e47759_d_n6;
        locals.var_qbs_dn7 = assign32710_e47759_d_n7;
        locals.var_qbs_dn10 = assign32710_e47759_d_n10;
        locals.var_qbs_dn11 = assign32710_e47759_d_n11;
        locals.var_qbs_dn12 = assign32710_e47759_d_n12;
        locals.var_qbs_dn17 = assign32710_e47759_d_n17;

        let assign32720_e47762: f64 = if p.p4 > locals.var_w_diodcv { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign32720_e47762;

        let (assign32730_e47772,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) {
        let assign32730_e47769: f64 = (p.p4 - locals.var_w_diodcv);
        let assign32730_e47770: f64 = (p.p180 * assign32730_e47769);
        (assign32730_e47770,)
    } else {
        (locals.var_czbdsw,)
    }
};
        locals.var_czbdsw = assign32730_e47772;

        let (assign32740_e47780,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) {
        let assign32740_e47778: f64 = (p.p181 * locals.var_w_diodcv);
        (assign32740_e47778,)
    } else {
        (locals.var_czbdswg,)
    }
};
        locals.var_czbdswg = assign32740_e47780;

        let assign32750_e47783: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign32750_e47783;

        let assign32760_e47786: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign32760_e47786;

        let (assign32770_e47800, assign32770_e47800_d_n6, assign32770_e47800_d_n7, assign32770_e47800_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
        let assign32770_e47797: f64 = (locals.var_vbdj / p.p185);
        let assign32770_e47798: f64 = (1.0 - assign32770_e47797);
        (assign32770_e47798, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32770_e47800;
        locals.var_arg__blk1055_dn6 = assign32770_e47800_d_n6;
        locals.var_arg__blk1055_dn7 = assign32770_e47800_d_n7;
        locals.var_arg__blk1055_dn12 = assign32770_e47800_d_n12;

        let assign32780_e47803: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign32780_e47803;

        let (assign32790_e47818, assign32790_e47818_d_n6, assign32790_e47818_d_n7, assign32790_e47818_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) {
        let assign32790_e47815: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32790_e47816: f64 = (1.0 / assign32790_e47815);
        (assign32790_e47816, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32790_e47818;
        locals.var_sarg_dn6 = assign32790_e47818_d_n6;
        locals.var_sarg_dn7 = assign32790_e47818_d_n7;
        locals.var_sarg_dn12 = assign32790_e47818_d_n12;

        let (assign32800_e47834, assign32800_e47834_d_n6, assign32800_e47834_d_n7, assign32800_e47834_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
        let assign32800_e47831: f64 = (-p.p182);
        let assign32800_e47832: f64 = (locals.var_arg__blk1055).powf(assign32800_e47831);
        (assign32800_e47832, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32800_e47834;
        locals.var_sarg_dn6 = assign32800_e47834_d_n6;
        locals.var_sarg_dn7 = assign32800_e47834_d_n7;
        locals.var_sarg_dn12 = assign32800_e47834_d_n12;

        let (assign32810_e47856, assign32810_e47856_d_n0, assign32810_e47856_d_n2, assign32810_e47856_d_n6, assign32810_e47856_d_n7, assign32810_e47856_d_n10, assign32810_e47856_d_n11, assign32810_e47856_d_n12, assign32810_e47856_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
        let assign32810_e47844: f64 = (p.p185 * locals.var_czbd);
        let assign32810_e47848: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32810_e47849: f64 = (1.0 - assign32810_e47848);
        let assign32810_e47850: f64 = (assign32810_e47844 * assign32810_e47849);
        let assign32810_e47853: f64 = (1.0 - p.p182);
        let assign32810_e47854: f64 = (assign32810_e47850 / assign32810_e47853);
        (assign32810_e47854, 0.0, 0.0, ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32810_e47853), ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32810_e47853), 0.0, 0.0, ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32810_e47853), 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32810_e47856;
        locals.var_qbd_dn0 = assign32810_e47856_d_n0;
        locals.var_qbd_dn2 = assign32810_e47856_d_n2;
        locals.var_qbd_dn6 = assign32810_e47856_d_n6;
        locals.var_qbd_dn7 = assign32810_e47856_d_n7;
        locals.var_qbd_dn10 = assign32810_e47856_d_n10;
        locals.var_qbd_dn11 = assign32810_e47856_d_n11;
        locals.var_qbd_dn12 = assign32810_e47856_d_n12;
        locals.var_qbd_dn17 = assign32810_e47856_d_n17;

        let (assign32820_e47867, assign32820_e47867_d_n0, assign32820_e47867_d_n2, assign32820_e47867_d_n6, assign32820_e47867_d_n7, assign32820_e47867_d_n10, assign32820_e47867_d_n11, assign32820_e47867_d_n12, assign32820_e47867_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32820_e47867;
        locals.var_qbd_dn0 = assign32820_e47867_d_n0;
        locals.var_qbd_dn2 = assign32820_e47867_d_n2;
        locals.var_qbd_dn6 = assign32820_e47867_d_n6;
        locals.var_qbd_dn7 = assign32820_e47867_d_n7;
        locals.var_qbd_dn10 = assign32820_e47867_d_n10;
        locals.var_qbd_dn11 = assign32820_e47867_d_n11;
        locals.var_qbd_dn12 = assign32820_e47867_d_n12;
        locals.var_qbd_dn17 = assign32820_e47867_d_n17;

        let assign32830_e47870: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign32830_e47870;

        let (assign32840_e47884, assign32840_e47884_d_n6, assign32840_e47884_d_n7, assign32840_e47884_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) {
        let assign32840_e47881: f64 = (locals.var_vbdj / p.p186);
        let assign32840_e47882: f64 = (1.0 - assign32840_e47881);
        (assign32840_e47882, (-(locals.var_vbdj_dn6 / p.p186)), 0.0, (-(locals.var_vbdj_dn12 / p.p186)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32840_e47884;
        locals.var_arg__blk1055_dn6 = assign32840_e47884_d_n6;
        locals.var_arg__blk1055_dn7 = assign32840_e47884_d_n7;
        locals.var_arg__blk1055_dn12 = assign32840_e47884_d_n12;

        let assign32850_e47887: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign32850_e47887;

        let (assign32860_e47902, assign32860_e47902_d_n6, assign32860_e47902_d_n7, assign32860_e47902_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) {
        let assign32860_e47899: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32860_e47900: f64 = (1.0 / assign32860_e47899);
        (assign32860_e47900, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32860_e47902;
        locals.var_sarg_dn6 = assign32860_e47902_d_n6;
        locals.var_sarg_dn7 = assign32860_e47902_d_n7;
        locals.var_sarg_dn12 = assign32860_e47902_d_n12;

        let (assign32870_e47918, assign32870_e47918_d_n6, assign32870_e47918_d_n7, assign32870_e47918_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
        let assign32870_e47915: f64 = (-p.p183);
        let assign32870_e47916: f64 = (locals.var_arg__blk1055).powf(assign32870_e47915);
        (assign32870_e47916, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32870_e47918;
        locals.var_sarg_dn6 = assign32870_e47918_d_n6;
        locals.var_sarg_dn7 = assign32870_e47918_d_n7;
        locals.var_sarg_dn12 = assign32870_e47918_d_n12;

    }

    pub(super) fn stamp_transient_block_116(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32880_e47942, assign32880_e47942_d_n0, assign32880_e47942_d_n2, assign32880_e47942_d_n6, assign32880_e47942_d_n7, assign32880_e47942_d_n10, assign32880_e47942_d_n11, assign32880_e47942_d_n12, assign32880_e47942_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) {
        let assign32880_e47929: f64 = (p.p186 * locals.var_czbdsw);
        let assign32880_e47933: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32880_e47934: f64 = (1.0 - assign32880_e47933);
        let assign32880_e47935: f64 = (assign32880_e47929 * assign32880_e47934);
        let assign32880_e47938: f64 = (1.0 - p.p183);
        let assign32880_e47939: f64 = (assign32880_e47935 / assign32880_e47938);
        let assign32880_e47940: f64 = (locals.var_qbd + assign32880_e47939);
        (assign32880_e47940, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32880_e47938)), (locals.var_qbd_dn7 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32880_e47938)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32880_e47938)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32880_e47942;
        locals.var_qbd_dn0 = assign32880_e47942_d_n0;
        locals.var_qbd_dn2 = assign32880_e47942_d_n2;
        locals.var_qbd_dn6 = assign32880_e47942_d_n6;
        locals.var_qbd_dn7 = assign32880_e47942_d_n7;
        locals.var_qbd_dn10 = assign32880_e47942_d_n10;
        locals.var_qbd_dn11 = assign32880_e47942_d_n11;
        locals.var_qbd_dn12 = assign32880_e47942_d_n12;
        locals.var_qbd_dn17 = assign32880_e47942_d_n17;

        let assign32890_e47945: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign32890_e47945;

        let (assign32900_e47959, assign32900_e47959_d_n6, assign32900_e47959_d_n7, assign32900_e47959_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) {
        let assign32900_e47956: f64 = (locals.var_vbdj / p.p187);
        let assign32900_e47957: f64 = (1.0 - assign32900_e47956);
        (assign32900_e47957, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32900_e47959;
        locals.var_arg__blk1055_dn6 = assign32900_e47959_d_n6;
        locals.var_arg__blk1055_dn7 = assign32900_e47959_d_n7;
        locals.var_arg__blk1055_dn12 = assign32900_e47959_d_n12;

        let assign32910_e47962: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign32910_e47962;

        let (assign32920_e47977, assign32920_e47977_d_n6, assign32920_e47977_d_n7, assign32920_e47977_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 != 0.0)) {
        let assign32920_e47974: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32920_e47975: f64 = (1.0 / assign32920_e47974);
        (assign32920_e47975, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32920_e47977;
        locals.var_sarg_dn6 = assign32920_e47977_d_n6;
        locals.var_sarg_dn7 = assign32920_e47977_d_n7;
        locals.var_sarg_dn12 = assign32920_e47977_d_n12;

        let (assign32930_e47993, assign32930_e47993_d_n6, assign32930_e47993_d_n7, assign32930_e47993_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 == 0.0)) {
        let assign32930_e47990: f64 = (-p.p184);
        let assign32930_e47991: f64 = (locals.var_arg__blk1055).powf(assign32930_e47990);
        (assign32930_e47991, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32930_e47993;
        locals.var_sarg_dn6 = assign32930_e47993_d_n6;
        locals.var_sarg_dn7 = assign32930_e47993_d_n7;
        locals.var_sarg_dn12 = assign32930_e47993_d_n12;

        let (assign32940_e48017, assign32940_e48017_d_n0, assign32940_e48017_d_n2, assign32940_e48017_d_n6, assign32940_e48017_d_n7, assign32940_e48017_d_n10, assign32940_e48017_d_n11, assign32940_e48017_d_n12, assign32940_e48017_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) {
        let assign32940_e48004: f64 = (p.p187 * locals.var_czbdswg);
        let assign32940_e48008: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32940_e48009: f64 = (1.0 - assign32940_e48008);
        let assign32940_e48010: f64 = (assign32940_e48004 * assign32940_e48009);
        let assign32940_e48013: f64 = (1.0 - p.p184);
        let assign32940_e48014: f64 = (assign32940_e48010 / assign32940_e48013);
        let assign32940_e48015: f64 = (locals.var_qbd + assign32940_e48014);
        (assign32940_e48015, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32940_e48013)), (locals.var_qbd_dn7 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32940_e48013)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32940_e48013)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32940_e48017;
        locals.var_qbd_dn0 = assign32940_e48017_d_n0;
        locals.var_qbd_dn2 = assign32940_e48017_d_n2;
        locals.var_qbd_dn6 = assign32940_e48017_d_n6;
        locals.var_qbd_dn7 = assign32940_e48017_d_n7;
        locals.var_qbd_dn10 = assign32940_e48017_d_n10;
        locals.var_qbd_dn11 = assign32940_e48017_d_n11;
        locals.var_qbd_dn12 = assign32940_e48017_d_n12;
        locals.var_qbd_dn17 = assign32940_e48017_d_n17;

        let (assign32950_e48030, assign32950_e48030_d_n6, assign32950_e48030_d_n7, assign32950_e48030_d_n10, assign32950_e48030_d_n12,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
        let assign32950_e48026: f64 = (locals.var_czbd + locals.var_czbdsw);
        let assign32950_e48028: f64 = (assign32950_e48026 + locals.var_czbdswg);
        (assign32950_e48028, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32950_e48030;
        locals.var_t1__blk1030_dn6 = assign32950_e48030_d_n6;
        locals.var_t1__blk1030_dn7 = assign32950_e48030_d_n7;
        locals.var_t1__blk1030_dn10 = assign32950_e48030_d_n10;
        locals.var_t1__blk1030_dn12 = assign32950_e48030_d_n12;

        let (assign32960_e48055, assign32960_e48055_d_n0, assign32960_e48055_d_n2, assign32960_e48055_d_n6, assign32960_e48055_d_n7, assign32960_e48055_d_n10, assign32960_e48055_d_n11, assign32960_e48055_d_n12, assign32960_e48055_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
        let assign32960_e48039: f64 = (locals.var_czbd * p.p182);
        let assign32960_e48041: f64 = (assign32960_e48039 / p.p185);
        let assign32960_e48044: f64 = (locals.var_czbdsw * p.p183);
        let assign32960_e48046: f64 = (assign32960_e48044 / p.p186);
        let assign32960_e48047: f64 = (assign32960_e48041 + assign32960_e48046);
        let assign32960_e48050: f64 = (locals.var_czbdswg * p.p184);
        let assign32960_e48052: f64 = (assign32960_e48050 / p.p187);
        let assign32960_e48053: f64 = (assign32960_e48047 + assign32960_e48052);
        (assign32960_e48053, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17,)
    }
};
        locals.var_t2__blk1031 = assign32960_e48055;
        locals.var_t2__blk1031_dn0 = assign32960_e48055_d_n0;
        locals.var_t2__blk1031_dn2 = assign32960_e48055_d_n2;
        locals.var_t2__blk1031_dn6 = assign32960_e48055_d_n6;
        locals.var_t2__blk1031_dn7 = assign32960_e48055_d_n7;
        locals.var_t2__blk1031_dn10 = assign32960_e48055_d_n10;
        locals.var_t2__blk1031_dn11 = assign32960_e48055_d_n11;
        locals.var_t2__blk1031_dn12 = assign32960_e48055_d_n12;
        locals.var_t2__blk1031_dn17 = assign32960_e48055_d_n17;

        let (assign32970_e48072, assign32970_e48072_d_n0, assign32970_e48072_d_n2, assign32970_e48072_d_n6, assign32970_e48072_d_n7, assign32970_e48072_d_n10, assign32970_e48072_d_n11, assign32970_e48072_d_n12, assign32970_e48072_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
        let assign32970_e48066: f64 = (locals.var_vbdj * 0.5);
        let assign32970_e48068: f64 = (assign32970_e48066 * locals.var_t2__blk1031);
        let assign32970_e48069: f64 = (locals.var_t1__blk1030 + assign32970_e48068);
        let assign32970_e48070: f64 = (locals.var_vbdj * assign32970_e48069);
        (assign32970_e48070, (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn0)), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn2)), ((locals.var_vbdj_dn6 * assign32970_e48069) + (locals.var_vbdj * (locals.var_t1__blk1030_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1031) + (assign32970_e48066 * locals.var_t2__blk1031_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1030_dn7 + (assign32970_e48066 * locals.var_t2__blk1031_dn7))), (locals.var_vbdj * (locals.var_t1__blk1030_dn10 + (assign32970_e48066 * locals.var_t2__blk1031_dn10))), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn11)), ((locals.var_vbdj_dn12 * assign32970_e48069) + (locals.var_vbdj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32970_e48066 * locals.var_t2__blk1031_dn12))))), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn17)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32970_e48072;
        locals.var_qbd_dn0 = assign32970_e48072_d_n0;
        locals.var_qbd_dn2 = assign32970_e48072_d_n2;
        locals.var_qbd_dn6 = assign32970_e48072_d_n6;
        locals.var_qbd_dn7 = assign32970_e48072_d_n7;
        locals.var_qbd_dn10 = assign32970_e48072_d_n10;
        locals.var_qbd_dn11 = assign32970_e48072_d_n11;
        locals.var_qbd_dn12 = assign32970_e48072_d_n12;
        locals.var_qbd_dn17 = assign32970_e48072_d_n17;

        let (assign32980_e48081,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) {
        let assign32980_e48079: f64 = (p.p181 * p.p4);
        (assign32980_e48079,)
    } else {
        (locals.var_czbdswg,)
    }
};
        locals.var_czbdswg = assign32980_e48081;

        let assign32990_e48084: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign32990_e48084;

        let assign33000_e48087: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign33000_e48087;

        let (assign33010_e48102, assign33010_e48102_d_n6, assign33010_e48102_d_n7, assign33010_e48102_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
        let assign33010_e48099: f64 = (locals.var_vbdj / p.p185);
        let assign33010_e48100: f64 = (1.0 - assign33010_e48099);
        (assign33010_e48100, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign33010_e48102;
        locals.var_arg__blk1055_dn6 = assign33010_e48102_d_n6;
        locals.var_arg__blk1055_dn7 = assign33010_e48102_d_n7;
        locals.var_arg__blk1055_dn12 = assign33010_e48102_d_n12;

        let assign33020_e48105: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign33020_e48105;

        let (assign33030_e48121, assign33030_e48121_d_n6, assign33030_e48121_d_n7, assign33030_e48121_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign33030_e48118: f64 = (locals.var_arg__blk1055).sqrt();
        let assign33030_e48119: f64 = (1.0 / assign33030_e48118);
        (assign33030_e48119, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33030_e48121;
        locals.var_sarg_dn6 = assign33030_e48121_d_n6;
        locals.var_sarg_dn7 = assign33030_e48121_d_n7;
        locals.var_sarg_dn12 = assign33030_e48121_d_n12;

        let (assign33040_e48138, assign33040_e48138_d_n6, assign33040_e48138_d_n7, assign33040_e48138_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 == 0.0)) {
        let assign33040_e48135: f64 = (-p.p182);
        let assign33040_e48136: f64 = (locals.var_arg__blk1055).powf(assign33040_e48135);
        (assign33040_e48136, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33040_e48138;
        locals.var_sarg_dn6 = assign33040_e48138_d_n6;
        locals.var_sarg_dn7 = assign33040_e48138_d_n7;
        locals.var_sarg_dn12 = assign33040_e48138_d_n12;

        let (assign33050_e48161, assign33050_e48161_d_n0, assign33050_e48161_d_n2, assign33050_e48161_d_n6, assign33050_e48161_d_n7, assign33050_e48161_d_n10, assign33050_e48161_d_n11, assign33050_e48161_d_n12, assign33050_e48161_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
        let assign33050_e48149: f64 = (p.p185 * locals.var_czbd);
        let assign33050_e48153: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign33050_e48154: f64 = (1.0 - assign33050_e48153);
        let assign33050_e48155: f64 = (assign33050_e48149 * assign33050_e48154);
        let assign33050_e48158: f64 = (1.0 - p.p182);
        let assign33050_e48159: f64 = (assign33050_e48155 / assign33050_e48158);
        (assign33050_e48159, 0.0, 0.0, ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign33050_e48158), ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign33050_e48158), 0.0, 0.0, ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign33050_e48158), 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33050_e48161;
        locals.var_qbd_dn0 = assign33050_e48161_d_n0;
        locals.var_qbd_dn2 = assign33050_e48161_d_n2;
        locals.var_qbd_dn6 = assign33050_e48161_d_n6;
        locals.var_qbd_dn7 = assign33050_e48161_d_n7;
        locals.var_qbd_dn10 = assign33050_e48161_d_n10;
        locals.var_qbd_dn11 = assign33050_e48161_d_n11;
        locals.var_qbd_dn12 = assign33050_e48161_d_n12;
        locals.var_qbd_dn17 = assign33050_e48161_d_n17;

        let (assign33060_e48173, assign33060_e48173_d_n0, assign33060_e48173_d_n2, assign33060_e48173_d_n6, assign33060_e48173_d_n7, assign33060_e48173_d_n10, assign33060_e48173_d_n11, assign33060_e48173_d_n12, assign33060_e48173_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33060_e48173;
        locals.var_qbd_dn0 = assign33060_e48173_d_n0;
        locals.var_qbd_dn2 = assign33060_e48173_d_n2;
        locals.var_qbd_dn6 = assign33060_e48173_d_n6;
        locals.var_qbd_dn7 = assign33060_e48173_d_n7;
        locals.var_qbd_dn10 = assign33060_e48173_d_n10;
        locals.var_qbd_dn11 = assign33060_e48173_d_n11;
        locals.var_qbd_dn12 = assign33060_e48173_d_n12;
        locals.var_qbd_dn17 = assign33060_e48173_d_n17;

        let assign33070_e48176: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign33070_e48176;

        let (assign33080_e48191, assign33080_e48191_d_n6, assign33080_e48191_d_n7, assign33080_e48191_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) {
        let assign33080_e48188: f64 = (locals.var_vbdj / p.p187);
        let assign33080_e48189: f64 = (1.0 - assign33080_e48188);
        (assign33080_e48189, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign33080_e48191;
        locals.var_arg__blk1055_dn6 = assign33080_e48191_d_n6;
        locals.var_arg__blk1055_dn7 = assign33080_e48191_d_n7;
        locals.var_arg__blk1055_dn12 = assign33080_e48191_d_n12;

        let assign33090_e48194: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1085 = assign33090_e48194;

        let (assign33100_e48210, assign33100_e48210_d_n6, assign33100_e48210_d_n7, assign33100_e48210_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 != 0.0)) {
        let assign33100_e48207: f64 = (locals.var_arg__blk1055).sqrt();
        let assign33100_e48208: f64 = (1.0 / assign33100_e48207);
        (assign33100_e48208, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33100_e48210;
        locals.var_sarg_dn6 = assign33100_e48210_d_n6;
        locals.var_sarg_dn7 = assign33100_e48210_d_n7;
        locals.var_sarg_dn12 = assign33100_e48210_d_n12;

        let (assign33110_e48227, assign33110_e48227_d_n6, assign33110_e48227_d_n7, assign33110_e48227_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 == 0.0)) {
        let assign33110_e48224: f64 = (-p.p184);
        let assign33110_e48225: f64 = (locals.var_arg__blk1055).powf(assign33110_e48224);
        (assign33110_e48225, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33110_e48227;
        locals.var_sarg_dn6 = assign33110_e48227_d_n6;
        locals.var_sarg_dn7 = assign33110_e48227_d_n7;
        locals.var_sarg_dn12 = assign33110_e48227_d_n12;

        let (assign33120_e48252, assign33120_e48252_d_n0, assign33120_e48252_d_n2, assign33120_e48252_d_n6, assign33120_e48252_d_n7, assign33120_e48252_d_n10, assign33120_e48252_d_n11, assign33120_e48252_d_n12, assign33120_e48252_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) {
        let assign33120_e48239: f64 = (p.p187 * locals.var_czbdswg);
        let assign33120_e48243: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign33120_e48244: f64 = (1.0 - assign33120_e48243);
        let assign33120_e48245: f64 = (assign33120_e48239 * assign33120_e48244);
        let assign33120_e48248: f64 = (1.0 - p.p184);
        let assign33120_e48249: f64 = (assign33120_e48245 / assign33120_e48248);
        let assign33120_e48250: f64 = (locals.var_qbd + assign33120_e48249);
        (assign33120_e48250, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign33120_e48248)), (locals.var_qbd_dn7 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign33120_e48248)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign33120_e48248)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33120_e48252;
        locals.var_qbd_dn0 = assign33120_e48252_d_n0;
        locals.var_qbd_dn2 = assign33120_e48252_d_n2;
        locals.var_qbd_dn6 = assign33120_e48252_d_n6;
        locals.var_qbd_dn7 = assign33120_e48252_d_n7;
        locals.var_qbd_dn10 = assign33120_e48252_d_n10;
        locals.var_qbd_dn11 = assign33120_e48252_d_n11;
        locals.var_qbd_dn12 = assign33120_e48252_d_n12;
        locals.var_qbd_dn17 = assign33120_e48252_d_n17;

        let (assign33130_e48264, assign33130_e48264_d_n6, assign33130_e48264_d_n7, assign33130_e48264_d_n10, assign33130_e48264_d_n12,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
        let assign33130_e48262: f64 = (locals.var_czbd + locals.var_czbdswg);
        (assign33130_e48262, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign33130_e48264;
        locals.var_t1__blk1030_dn6 = assign33130_e48264_d_n6;
        locals.var_t1__blk1030_dn7 = assign33130_e48264_d_n7;
        locals.var_t1__blk1030_dn10 = assign33130_e48264_d_n10;
        locals.var_t1__blk1030_dn12 = assign33130_e48264_d_n12;

        let (assign33140_e48284, assign33140_e48284_d_n0, assign33140_e48284_d_n2, assign33140_e48284_d_n6, assign33140_e48284_d_n7, assign33140_e48284_d_n10, assign33140_e48284_d_n11, assign33140_e48284_d_n12, assign33140_e48284_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
        let assign33140_e48274: f64 = (locals.var_czbd * p.p182);
        let assign33140_e48276: f64 = (assign33140_e48274 / p.p185);
        let assign33140_e48279: f64 = (locals.var_czbdswg * p.p184);
        let assign33140_e48281: f64 = (assign33140_e48279 / p.p187);
        let assign33140_e48282: f64 = (assign33140_e48276 + assign33140_e48281);
        (assign33140_e48282, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17,)
    }
};
        locals.var_t2__blk1031 = assign33140_e48284;
        locals.var_t2__blk1031_dn0 = assign33140_e48284_d_n0;
        locals.var_t2__blk1031_dn2 = assign33140_e48284_d_n2;
        locals.var_t2__blk1031_dn6 = assign33140_e48284_d_n6;
        locals.var_t2__blk1031_dn7 = assign33140_e48284_d_n7;
        locals.var_t2__blk1031_dn10 = assign33140_e48284_d_n10;
        locals.var_t2__blk1031_dn11 = assign33140_e48284_d_n11;
        locals.var_t2__blk1031_dn12 = assign33140_e48284_d_n12;
        locals.var_t2__blk1031_dn17 = assign33140_e48284_d_n17;

        let (assign33150_e48302, assign33150_e48302_d_n0, assign33150_e48302_d_n2, assign33150_e48302_d_n6, assign33150_e48302_d_n7, assign33150_e48302_d_n10, assign33150_e48302_d_n11, assign33150_e48302_d_n12, assign33150_e48302_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
        let assign33150_e48296: f64 = (locals.var_vbdj * 0.5);
        let assign33150_e48298: f64 = (assign33150_e48296 * locals.var_t2__blk1031);
        let assign33150_e48299: f64 = (locals.var_t1__blk1030 + assign33150_e48298);
        let assign33150_e48300: f64 = (locals.var_vbdj * assign33150_e48299);
        (assign33150_e48300, (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn0)), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn2)), ((locals.var_vbdj_dn6 * assign33150_e48299) + (locals.var_vbdj * (locals.var_t1__blk1030_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1031) + (assign33150_e48296 * locals.var_t2__blk1031_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1030_dn7 + (assign33150_e48296 * locals.var_t2__blk1031_dn7))), (locals.var_vbdj * (locals.var_t1__blk1030_dn10 + (assign33150_e48296 * locals.var_t2__blk1031_dn10))), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn11)), ((locals.var_vbdj_dn12 * assign33150_e48299) + (locals.var_vbdj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign33150_e48296 * locals.var_t2__blk1031_dn12))))), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn17)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33150_e48302;
        locals.var_qbd_dn0 = assign33150_e48302_d_n0;
        locals.var_qbd_dn2 = assign33150_e48302_d_n2;
        locals.var_qbd_dn6 = assign33150_e48302_d_n6;
        locals.var_qbd_dn7 = assign33150_e48302_d_n7;
        locals.var_qbd_dn10 = assign33150_e48302_d_n10;
        locals.var_qbd_dn11 = assign33150_e48302_d_n11;
        locals.var_qbd_dn12 = assign33150_e48302_d_n12;
        locals.var_qbd_dn17 = assign33150_e48302_d_n17;

        let assign33160_e48305: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1086 = assign33160_e48305;

        let (assign33170_e48318, assign33170_e48318_d_n0, assign33170_e48318_d_n2, assign33170_e48318_d_n6, assign33170_e48318_d_n7, assign33170_e48318_d_n10, assign33170_e48318_d_n11, assign33170_e48318_d_n12, assign33170_e48318_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33170_e48310: f64 = (-1.6021918e-19);
        let assign33170_e48312: f64 = (assign33170_e48310 * locals.var_uc_nsubs);
        let assign33170_e48314: f64 = (assign33170_e48312 * locals.var_xp_max);
        let assign33170_e48316: f64 = (assign33170_e48314 * p.p3);
        (assign33170_e48316, (((assign33170_e48310 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p3),)
    } else {
        (locals.var_qbs_max, locals.var_qbs_max_dn0, locals.var_qbs_max_dn2, locals.var_qbs_max_dn6, locals.var_qbs_max_dn7, locals.var_qbs_max_dn10, locals.var_qbs_max_dn11, locals.var_qbs_max_dn12, locals.var_qbs_max_dn17,)
    }
};
        locals.var_qbs_max = assign33170_e48318;
        locals.var_qbs_max_dn0 = assign33170_e48318_d_n0;
        locals.var_qbs_max_dn2 = assign33170_e48318_d_n2;
        locals.var_qbs_max_dn6 = assign33170_e48318_d_n6;
        locals.var_qbs_max_dn7 = assign33170_e48318_d_n7;
        locals.var_qbs_max_dn10 = assign33170_e48318_d_n10;
        locals.var_qbs_max_dn11 = assign33170_e48318_d_n11;
        locals.var_qbs_max_dn12 = assign33170_e48318_d_n12;
        locals.var_qbs_max_dn17 = assign33170_e48318_d_n17;

        let (assign33180_e48327, assign33180_e48327_d_n0, assign33180_e48327_d_n2, assign33180_e48327_d_n6, assign33180_e48327_d_n7, assign33180_e48327_d_n10, assign33180_e48327_d_n11, assign33180_e48327_d_n12, assign33180_e48327_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33180_e48324: f64 = (-locals.var_qbs_max);
        let assign33180_e48325: f64 = (0.001 * assign33180_e48324);
        (assign33180_e48325, (0.001 * (-locals.var_qbs_max_dn0)), (0.001 * (-locals.var_qbs_max_dn2)), (0.001 * (-locals.var_qbs_max_dn6)), (0.001 * (-locals.var_qbs_max_dn7)), (0.001 * (-locals.var_qbs_max_dn10)), (0.001 * (-locals.var_qbs_max_dn11)), (0.001 * (-locals.var_qbs_max_dn12)), (0.001 * (-locals.var_qbs_max_dn17)),)
    } else {
        (locals.var_dlt_qbs, locals.var_dlt_qbs_dn0, locals.var_dlt_qbs_dn2, locals.var_dlt_qbs_dn6, locals.var_dlt_qbs_dn7, locals.var_dlt_qbs_dn10, locals.var_dlt_qbs_dn11, locals.var_dlt_qbs_dn12, locals.var_dlt_qbs_dn17,)
    }
};
        locals.var_dlt_qbs = assign33180_e48327;
        locals.var_dlt_qbs_dn0 = assign33180_e48327_d_n0;
        locals.var_dlt_qbs_dn2 = assign33180_e48327_d_n2;
        locals.var_dlt_qbs_dn6 = assign33180_e48327_d_n6;
        locals.var_dlt_qbs_dn7 = assign33180_e48327_d_n7;
        locals.var_dlt_qbs_dn10 = assign33180_e48327_d_n10;
        locals.var_dlt_qbs_dn11 = assign33180_e48327_d_n11;
        locals.var_dlt_qbs_dn12 = assign33180_e48327_d_n12;
        locals.var_dlt_qbs_dn17 = assign33180_e48327_d_n17;

        let (assign33190_e48339, assign33190_e48339_d_n0, assign33190_e48339_d_n2, assign33190_e48339_d_n6, assign33190_e48339_d_n7, assign33190_e48339_d_n10, assign33190_e48339_d_n11, assign33190_e48339_d_n12, assign33190_e48339_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33190_e48332: f64 = (-locals.var_qbs_max);
        let assign33190_e48334: f64 = (-locals.var_qbs);
        let assign33190_e48335: f64 = (assign33190_e48332 - assign33190_e48334);
        let assign33190_e48337: f64 = (assign33190_e48335 - locals.var_dlt_qbs);
        (assign33190_e48337, (((-locals.var_qbs_max_dn0) - (-locals.var_qbs_dn0)) - locals.var_dlt_qbs_dn0), (((-locals.var_qbs_max_dn2) - (-locals.var_qbs_dn2)) - locals.var_dlt_qbs_dn2), (((-locals.var_qbs_max_dn6) - (-locals.var_qbs_dn6)) - locals.var_dlt_qbs_dn6), (((-locals.var_qbs_max_dn7) - (-locals.var_qbs_dn7)) - locals.var_dlt_qbs_dn7), (((-locals.var_qbs_max_dn10) - (-locals.var_qbs_dn10)) - locals.var_dlt_qbs_dn10), (((-locals.var_qbs_max_dn11) - (-locals.var_qbs_dn11)) - locals.var_dlt_qbs_dn11), (((-locals.var_qbs_max_dn12) - (-locals.var_qbs_dn12)) - locals.var_dlt_qbs_dn12), (((-locals.var_qbs_max_dn17) - (-locals.var_qbs_dn17)) - locals.var_dlt_qbs_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign33190_e48339;
        locals.var_tmf1_dn0 = assign33190_e48339_d_n0;
        locals.var_tmf1_dn2 = assign33190_e48339_d_n2;
        locals.var_tmf1_dn6 = assign33190_e48339_d_n6;
        locals.var_tmf1_dn7 = assign33190_e48339_d_n7;
        locals.var_tmf1_dn10 = assign33190_e48339_d_n10;
        locals.var_tmf1_dn11 = assign33190_e48339_d_n11;
        locals.var_tmf1_dn12 = assign33190_e48339_d_n12;
        locals.var_tmf1_dn17 = assign33190_e48339_d_n17;

        let (assign33200_e48350, assign33200_e48350_d_n0, assign33200_e48350_d_n2, assign33200_e48350_d_n6, assign33200_e48350_d_n7, assign33200_e48350_d_n10, assign33200_e48350_d_n11, assign33200_e48350_d_n12, assign33200_e48350_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33200_e48345: f64 = (-locals.var_qbs_max);
        let assign33200_e48346: f64 = (4.0 * assign33200_e48345);
        let assign33200_e48348: f64 = (assign33200_e48346 * locals.var_dlt_qbs);
        (assign33200_e48348, (((4.0 * (-locals.var_qbs_max_dn0)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn0)), (((4.0 * (-locals.var_qbs_max_dn2)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn2)), (((4.0 * (-locals.var_qbs_max_dn6)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn6)), (((4.0 * (-locals.var_qbs_max_dn7)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn7)), (((4.0 * (-locals.var_qbs_max_dn10)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn10)), (((4.0 * (-locals.var_qbs_max_dn11)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn11)), (((4.0 * (-locals.var_qbs_max_dn12)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn12)), (((4.0 * (-locals.var_qbs_max_dn17)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33200_e48350;
        locals.var_tmf2_dn0 = assign33200_e48350_d_n0;
        locals.var_tmf2_dn2 = assign33200_e48350_d_n2;
        locals.var_tmf2_dn6 = assign33200_e48350_d_n6;
        locals.var_tmf2_dn7 = assign33200_e48350_d_n7;
        locals.var_tmf2_dn10 = assign33200_e48350_d_n10;
        locals.var_tmf2_dn11 = assign33200_e48350_d_n11;
        locals.var_tmf2_dn12 = assign33200_e48350_d_n12;
        locals.var_tmf2_dn17 = assign33200_e48350_d_n17;

        let (assign33210_e48362, assign33210_e48362_d_n0, assign33210_e48362_d_n2, assign33210_e48362_d_n6, assign33210_e48362_d_n7, assign33210_e48362_d_n10, assign33210_e48362_d_n11, assign33210_e48362_d_n12, assign33210_e48362_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let (assign33210_e48360, assign33210_e48360_d_n0, assign33210_e48360_d_n2, assign33210_e48360_d_n6, assign33210_e48360_d_n7, assign33210_e48360_d_n10, assign33210_e48360_d_n11, assign33210_e48360_d_n12, assign33210_e48360_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign33210_e48359: f64 = (-locals.var_tmf2);
                (assign33210_e48359, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign33210_e48360, assign33210_e48360_d_n0, assign33210_e48360_d_n2, assign33210_e48360_d_n6, assign33210_e48360_d_n7, assign33210_e48360_d_n10, assign33210_e48360_d_n11, assign33210_e48360_d_n12, assign33210_e48360_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33210_e48362;
        locals.var_tmf2_dn0 = assign33210_e48362_d_n0;
        locals.var_tmf2_dn2 = assign33210_e48362_d_n2;
        locals.var_tmf2_dn6 = assign33210_e48362_d_n6;
        locals.var_tmf2_dn7 = assign33210_e48362_d_n7;
        locals.var_tmf2_dn10 = assign33210_e48362_d_n10;
        locals.var_tmf2_dn11 = assign33210_e48362_d_n11;
        locals.var_tmf2_dn12 = assign33210_e48362_d_n12;
        locals.var_tmf2_dn17 = assign33210_e48362_d_n17;

    }

    pub(super) fn stamp_transient_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33220_e48373, assign33220_e48373_d_n0, assign33220_e48373_d_n2, assign33220_e48373_d_n6, assign33220_e48373_d_n7, assign33220_e48373_d_n10, assign33220_e48373_d_n11, assign33220_e48373_d_n12, assign33220_e48373_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33220_e48368: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign33220_e48370: f64 = (assign33220_e48368 + locals.var_tmf2);
        let assign33220_e48371: f64 = (assign33220_e48370).sqrt();
        (assign33220_e48371, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33220_e48371)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33220_e48373;
        locals.var_tmf2_dn0 = assign33220_e48373_d_n0;
        locals.var_tmf2_dn2 = assign33220_e48373_d_n2;
        locals.var_tmf2_dn6 = assign33220_e48373_d_n6;
        locals.var_tmf2_dn7 = assign33220_e48373_d_n7;
        locals.var_tmf2_dn10 = assign33220_e48373_d_n10;
        locals.var_tmf2_dn11 = assign33220_e48373_d_n11;
        locals.var_tmf2_dn12 = assign33220_e48373_d_n12;
        locals.var_tmf2_dn17 = assign33220_e48373_d_n17;

        let (assign33230_e48386, assign33230_e48386_d_n0, assign33230_e48386_d_n2, assign33230_e48386_d_n6, assign33230_e48386_d_n7, assign33230_e48386_d_n10, assign33230_e48386_d_n11, assign33230_e48386_d_n12, assign33230_e48386_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33230_e48378: f64 = (-locals.var_qbs_max);
        let assign33230_e48382: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign33230_e48383: f64 = (0.5 * assign33230_e48382);
        let assign33230_e48384: f64 = (assign33230_e48378 - assign33230_e48383);
        (assign33230_e48384, ((-locals.var_qbs_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbs_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbs_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbs_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbs_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbs_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbs_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbs_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign33230_e48386;
        locals.var_qbs_dn0 = assign33230_e48386_d_n0;
        locals.var_qbs_dn2 = assign33230_e48386_d_n2;
        locals.var_qbs_dn6 = assign33230_e48386_d_n6;
        locals.var_qbs_dn7 = assign33230_e48386_d_n7;
        locals.var_qbs_dn10 = assign33230_e48386_d_n10;
        locals.var_qbs_dn11 = assign33230_e48386_d_n11;
        locals.var_qbs_dn12 = assign33230_e48386_d_n12;
        locals.var_qbs_dn17 = assign33230_e48386_d_n17;

        let (assign33240_e48395, assign33240_e48395_d_n0, assign33240_e48395_d_n2, assign33240_e48395_d_n6, assign33240_e48395_d_n7, assign33240_e48395_d_n10, assign33240_e48395_d_n11, assign33240_e48395_d_n12, assign33240_e48395_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33240_e48392: f64 = (-1.0);
        let assign33240_e48393: f64 = (locals.var_qbs * assign33240_e48392);
        (assign33240_e48393, (locals.var_qbs_dn0 * assign33240_e48392), (locals.var_qbs_dn2 * assign33240_e48392), (locals.var_qbs_dn6 * assign33240_e48392), (locals.var_qbs_dn7 * assign33240_e48392), (locals.var_qbs_dn10 * assign33240_e48392), (locals.var_qbs_dn11 * assign33240_e48392), (locals.var_qbs_dn12 * assign33240_e48392), (locals.var_qbs_dn17 * assign33240_e48392),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign33240_e48395;
        locals.var_qbs_dn0 = assign33240_e48395_d_n0;
        locals.var_qbs_dn2 = assign33240_e48395_d_n2;
        locals.var_qbs_dn6 = assign33240_e48395_d_n6;
        locals.var_qbs_dn7 = assign33240_e48395_d_n7;
        locals.var_qbs_dn10 = assign33240_e48395_d_n10;
        locals.var_qbs_dn11 = assign33240_e48395_d_n11;
        locals.var_qbs_dn12 = assign33240_e48395_d_n12;
        locals.var_qbs_dn17 = assign33240_e48395_d_n17;

        let assign33250_e48398: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1087 = assign33250_e48398;

        let (assign33260_e48411, assign33260_e48411_d_n0, assign33260_e48411_d_n2, assign33260_e48411_d_n6, assign33260_e48411_d_n7, assign33260_e48411_d_n10, assign33260_e48411_d_n11, assign33260_e48411_d_n12, assign33260_e48411_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33260_e48403: f64 = (-1.6021918e-19);
        let assign33260_e48405: f64 = (assign33260_e48403 * locals.var_uc_nsubs);
        let assign33260_e48407: f64 = (assign33260_e48405 * locals.var_xp_max);
        let assign33260_e48409: f64 = (assign33260_e48407 * p.p2);
        (assign33260_e48409, (((assign33260_e48403 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p2),)
    } else {
        (locals.var_qbd_max, locals.var_qbd_max_dn0, locals.var_qbd_max_dn2, locals.var_qbd_max_dn6, locals.var_qbd_max_dn7, locals.var_qbd_max_dn10, locals.var_qbd_max_dn11, locals.var_qbd_max_dn12, locals.var_qbd_max_dn17,)
    }
};
        locals.var_qbd_max = assign33260_e48411;
        locals.var_qbd_max_dn0 = assign33260_e48411_d_n0;
        locals.var_qbd_max_dn2 = assign33260_e48411_d_n2;
        locals.var_qbd_max_dn6 = assign33260_e48411_d_n6;
        locals.var_qbd_max_dn7 = assign33260_e48411_d_n7;
        locals.var_qbd_max_dn10 = assign33260_e48411_d_n10;
        locals.var_qbd_max_dn11 = assign33260_e48411_d_n11;
        locals.var_qbd_max_dn12 = assign33260_e48411_d_n12;
        locals.var_qbd_max_dn17 = assign33260_e48411_d_n17;

        let (assign33270_e48420, assign33270_e48420_d_n0, assign33270_e48420_d_n2, assign33270_e48420_d_n6, assign33270_e48420_d_n7, assign33270_e48420_d_n10, assign33270_e48420_d_n11, assign33270_e48420_d_n12, assign33270_e48420_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33270_e48417: f64 = (-locals.var_qbd_max);
        let assign33270_e48418: f64 = (0.001 * assign33270_e48417);
        (assign33270_e48418, (0.001 * (-locals.var_qbd_max_dn0)), (0.001 * (-locals.var_qbd_max_dn2)), (0.001 * (-locals.var_qbd_max_dn6)), (0.001 * (-locals.var_qbd_max_dn7)), (0.001 * (-locals.var_qbd_max_dn10)), (0.001 * (-locals.var_qbd_max_dn11)), (0.001 * (-locals.var_qbd_max_dn12)), (0.001 * (-locals.var_qbd_max_dn17)),)
    } else {
        (locals.var_dlt_qbd, locals.var_dlt_qbd_dn0, locals.var_dlt_qbd_dn2, locals.var_dlt_qbd_dn6, locals.var_dlt_qbd_dn7, locals.var_dlt_qbd_dn10, locals.var_dlt_qbd_dn11, locals.var_dlt_qbd_dn12, locals.var_dlt_qbd_dn17,)
    }
};
        locals.var_dlt_qbd = assign33270_e48420;
        locals.var_dlt_qbd_dn0 = assign33270_e48420_d_n0;
        locals.var_dlt_qbd_dn2 = assign33270_e48420_d_n2;
        locals.var_dlt_qbd_dn6 = assign33270_e48420_d_n6;
        locals.var_dlt_qbd_dn7 = assign33270_e48420_d_n7;
        locals.var_dlt_qbd_dn10 = assign33270_e48420_d_n10;
        locals.var_dlt_qbd_dn11 = assign33270_e48420_d_n11;
        locals.var_dlt_qbd_dn12 = assign33270_e48420_d_n12;
        locals.var_dlt_qbd_dn17 = assign33270_e48420_d_n17;

        let (assign33280_e48432, assign33280_e48432_d_n0, assign33280_e48432_d_n2, assign33280_e48432_d_n6, assign33280_e48432_d_n7, assign33280_e48432_d_n10, assign33280_e48432_d_n11, assign33280_e48432_d_n12, assign33280_e48432_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33280_e48425: f64 = (-locals.var_qbd_max);
        let assign33280_e48427: f64 = (-locals.var_qbd);
        let assign33280_e48428: f64 = (assign33280_e48425 - assign33280_e48427);
        let assign33280_e48430: f64 = (assign33280_e48428 - locals.var_dlt_qbd);
        (assign33280_e48430, (((-locals.var_qbd_max_dn0) - (-locals.var_qbd_dn0)) - locals.var_dlt_qbd_dn0), (((-locals.var_qbd_max_dn2) - (-locals.var_qbd_dn2)) - locals.var_dlt_qbd_dn2), (((-locals.var_qbd_max_dn6) - (-locals.var_qbd_dn6)) - locals.var_dlt_qbd_dn6), (((-locals.var_qbd_max_dn7) - (-locals.var_qbd_dn7)) - locals.var_dlt_qbd_dn7), (((-locals.var_qbd_max_dn10) - (-locals.var_qbd_dn10)) - locals.var_dlt_qbd_dn10), (((-locals.var_qbd_max_dn11) - (-locals.var_qbd_dn11)) - locals.var_dlt_qbd_dn11), (((-locals.var_qbd_max_dn12) - (-locals.var_qbd_dn12)) - locals.var_dlt_qbd_dn12), (((-locals.var_qbd_max_dn17) - (-locals.var_qbd_dn17)) - locals.var_dlt_qbd_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign33280_e48432;
        locals.var_tmf1_dn0 = assign33280_e48432_d_n0;
        locals.var_tmf1_dn2 = assign33280_e48432_d_n2;
        locals.var_tmf1_dn6 = assign33280_e48432_d_n6;
        locals.var_tmf1_dn7 = assign33280_e48432_d_n7;
        locals.var_tmf1_dn10 = assign33280_e48432_d_n10;
        locals.var_tmf1_dn11 = assign33280_e48432_d_n11;
        locals.var_tmf1_dn12 = assign33280_e48432_d_n12;
        locals.var_tmf1_dn17 = assign33280_e48432_d_n17;

        let (assign33290_e48443, assign33290_e48443_d_n0, assign33290_e48443_d_n2, assign33290_e48443_d_n6, assign33290_e48443_d_n7, assign33290_e48443_d_n10, assign33290_e48443_d_n11, assign33290_e48443_d_n12, assign33290_e48443_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33290_e48438: f64 = (-locals.var_qbd_max);
        let assign33290_e48439: f64 = (4.0 * assign33290_e48438);
        let assign33290_e48441: f64 = (assign33290_e48439 * locals.var_dlt_qbd);
        (assign33290_e48441, (((4.0 * (-locals.var_qbd_max_dn0)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn0)), (((4.0 * (-locals.var_qbd_max_dn2)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn2)), (((4.0 * (-locals.var_qbd_max_dn6)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn6)), (((4.0 * (-locals.var_qbd_max_dn7)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn7)), (((4.0 * (-locals.var_qbd_max_dn10)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn10)), (((4.0 * (-locals.var_qbd_max_dn11)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn11)), (((4.0 * (-locals.var_qbd_max_dn12)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn12)), (((4.0 * (-locals.var_qbd_max_dn17)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33290_e48443;
        locals.var_tmf2_dn0 = assign33290_e48443_d_n0;
        locals.var_tmf2_dn2 = assign33290_e48443_d_n2;
        locals.var_tmf2_dn6 = assign33290_e48443_d_n6;
        locals.var_tmf2_dn7 = assign33290_e48443_d_n7;
        locals.var_tmf2_dn10 = assign33290_e48443_d_n10;
        locals.var_tmf2_dn11 = assign33290_e48443_d_n11;
        locals.var_tmf2_dn12 = assign33290_e48443_d_n12;
        locals.var_tmf2_dn17 = assign33290_e48443_d_n17;

        let (assign33300_e48455, assign33300_e48455_d_n0, assign33300_e48455_d_n2, assign33300_e48455_d_n6, assign33300_e48455_d_n7, assign33300_e48455_d_n10, assign33300_e48455_d_n11, assign33300_e48455_d_n12, assign33300_e48455_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let (assign33300_e48453, assign33300_e48453_d_n0, assign33300_e48453_d_n2, assign33300_e48453_d_n6, assign33300_e48453_d_n7, assign33300_e48453_d_n10, assign33300_e48453_d_n11, assign33300_e48453_d_n12, assign33300_e48453_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign33300_e48452: f64 = (-locals.var_tmf2);
                (assign33300_e48452, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign33300_e48453, assign33300_e48453_d_n0, assign33300_e48453_d_n2, assign33300_e48453_d_n6, assign33300_e48453_d_n7, assign33300_e48453_d_n10, assign33300_e48453_d_n11, assign33300_e48453_d_n12, assign33300_e48453_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33300_e48455;
        locals.var_tmf2_dn0 = assign33300_e48455_d_n0;
        locals.var_tmf2_dn2 = assign33300_e48455_d_n2;
        locals.var_tmf2_dn6 = assign33300_e48455_d_n6;
        locals.var_tmf2_dn7 = assign33300_e48455_d_n7;
        locals.var_tmf2_dn10 = assign33300_e48455_d_n10;
        locals.var_tmf2_dn11 = assign33300_e48455_d_n11;
        locals.var_tmf2_dn12 = assign33300_e48455_d_n12;
        locals.var_tmf2_dn17 = assign33300_e48455_d_n17;

        let (assign33310_e48466, assign33310_e48466_d_n0, assign33310_e48466_d_n2, assign33310_e48466_d_n6, assign33310_e48466_d_n7, assign33310_e48466_d_n10, assign33310_e48466_d_n11, assign33310_e48466_d_n12, assign33310_e48466_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33310_e48461: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign33310_e48463: f64 = (assign33310_e48461 + locals.var_tmf2);
        let assign33310_e48464: f64 = (assign33310_e48463).sqrt();
        (assign33310_e48464, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33310_e48464)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33310_e48466;
        locals.var_tmf2_dn0 = assign33310_e48466_d_n0;
        locals.var_tmf2_dn2 = assign33310_e48466_d_n2;
        locals.var_tmf2_dn6 = assign33310_e48466_d_n6;
        locals.var_tmf2_dn7 = assign33310_e48466_d_n7;
        locals.var_tmf2_dn10 = assign33310_e48466_d_n10;
        locals.var_tmf2_dn11 = assign33310_e48466_d_n11;
        locals.var_tmf2_dn12 = assign33310_e48466_d_n12;
        locals.var_tmf2_dn17 = assign33310_e48466_d_n17;

        let (assign33320_e48479, assign33320_e48479_d_n0, assign33320_e48479_d_n2, assign33320_e48479_d_n6, assign33320_e48479_d_n7, assign33320_e48479_d_n10, assign33320_e48479_d_n11, assign33320_e48479_d_n12, assign33320_e48479_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33320_e48471: f64 = (-locals.var_qbd_max);
        let assign33320_e48475: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign33320_e48476: f64 = (0.5 * assign33320_e48475);
        let assign33320_e48477: f64 = (assign33320_e48471 - assign33320_e48476);
        (assign33320_e48477, ((-locals.var_qbd_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbd_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbd_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbd_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbd_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbd_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbd_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbd_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33320_e48479;
        locals.var_qbd_dn0 = assign33320_e48479_d_n0;
        locals.var_qbd_dn2 = assign33320_e48479_d_n2;
        locals.var_qbd_dn6 = assign33320_e48479_d_n6;
        locals.var_qbd_dn7 = assign33320_e48479_d_n7;
        locals.var_qbd_dn10 = assign33320_e48479_d_n10;
        locals.var_qbd_dn11 = assign33320_e48479_d_n11;
        locals.var_qbd_dn12 = assign33320_e48479_d_n12;
        locals.var_qbd_dn17 = assign33320_e48479_d_n17;

        let (assign33330_e48488, assign33330_e48488_d_n0, assign33330_e48488_d_n2, assign33330_e48488_d_n6, assign33330_e48488_d_n7, assign33330_e48488_d_n10, assign33330_e48488_d_n11, assign33330_e48488_d_n12, assign33330_e48488_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33330_e48485: f64 = (-1.0);
        let assign33330_e48486: f64 = (locals.var_qbd * assign33330_e48485);
        (assign33330_e48486, (locals.var_qbd_dn0 * assign33330_e48485), (locals.var_qbd_dn2 * assign33330_e48485), (locals.var_qbd_dn6 * assign33330_e48485), (locals.var_qbd_dn7 * assign33330_e48485), (locals.var_qbd_dn10 * assign33330_e48485), (locals.var_qbd_dn11 * assign33330_e48485), (locals.var_qbd_dn12 * assign33330_e48485), (locals.var_qbd_dn17 * assign33330_e48485),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33330_e48488;
        locals.var_qbd_dn0 = assign33330_e48488_d_n0;
        locals.var_qbd_dn2 = assign33330_e48488_d_n2;
        locals.var_qbd_dn6 = assign33330_e48488_d_n6;
        locals.var_qbd_dn7 = assign33330_e48488_d_n7;
        locals.var_qbd_dn10 = assign33330_e48488_d_n10;
        locals.var_qbd_dn11 = assign33330_e48488_d_n11;
        locals.var_qbd_dn12 = assign33330_e48488_d_n12;
        locals.var_qbd_dn17 = assign33330_e48488_d_n17;

        let assign33560_e48742: f64 = if ((p.p32 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1120 = assign33560_e48742;

        let (assign33570_e48750, assign33570_e48750_d_n0, assign33570_e48750_d_n2, assign33570_e48750_d_n6, assign33570_e48750_d_n7, assign33570_e48750_d_n10, assign33570_e48750_d_n11, assign33570_e48750_d_n12, assign33570_e48750_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33570_e48746: f64 = (locals.var_psdl - locals.var_ps0);
        let assign33570_e48748: f64 = (assign33570_e48746 / locals.var_lch);
        (assign33570_e48748, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn12 - locals.var_ps0_dn12) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn17 - locals.var_ps0_dn17) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn17)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn12, locals.var_eyd_dn17,)
    }
};
        locals.var_eyd = assign33570_e48750;
        locals.var_eyd_dn0 = assign33570_e48750_d_n0;
        locals.var_eyd_dn2 = assign33570_e48750_d_n2;
        locals.var_eyd_dn6 = assign33570_e48750_d_n6;
        locals.var_eyd_dn7 = assign33570_e48750_d_n7;
        locals.var_eyd_dn10 = assign33570_e48750_d_n10;
        locals.var_eyd_dn11 = assign33570_e48750_d_n11;
        locals.var_eyd_dn12 = assign33570_e48750_d_n12;
        locals.var_eyd_dn17 = assign33570_e48750_d_n17;

        let (assign33580_e48758, assign33580_e48758_d_n0, assign33580_e48758_d_n2, assign33580_e48758_d_n6, assign33580_e48758_d_n7, assign33580_e48758_d_n10, assign33580_e48758_d_n11, assign33580_e48758_d_n12, assign33580_e48758_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33580_e48754: f64 = (locals.var_muun * locals.var_eyd);
        let assign33580_e48756: f64 = (assign33580_e48754 / 100000.0);
        (assign33580_e48756, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 100000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 100000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 100000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 100000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 100000.0), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / 100000.0), (((locals.var_muun_dn12 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn12)) / 100000.0), (((locals.var_muun_dn17 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn17)) / 100000.0),)
    } else {
        (locals.var_t12__blk1104, locals.var_t12__blk1104_dn0, locals.var_t12__blk1104_dn2, locals.var_t12__blk1104_dn6, locals.var_t12__blk1104_dn7, locals.var_t12__blk1104_dn10, locals.var_t12__blk1104_dn11, locals.var_t12__blk1104_dn12, locals.var_t12__blk1104_dn17,)
    }
};
        locals.var_t12__blk1104 = assign33580_e48758;
        locals.var_t12__blk1104_dn0 = assign33580_e48758_d_n0;
        locals.var_t12__blk1104_dn2 = assign33580_e48758_d_n2;
        locals.var_t12__blk1104_dn6 = assign33580_e48758_d_n6;
        locals.var_t12__blk1104_dn7 = assign33580_e48758_d_n7;
        locals.var_t12__blk1104_dn10 = assign33580_e48758_d_n10;
        locals.var_t12__blk1104_dn11 = assign33580_e48758_d_n11;
        locals.var_t12__blk1104_dn12 = assign33580_e48758_d_n12;
        locals.var_t12__blk1104_dn17 = assign33580_e48758_d_n17;

        let assign33590_e48762: f64 = (10.0 * 2.220446049250313e-16);
        let assign33590_e48763: f64 = (1.0 - assign33590_e48762);
        let assign33590_e48770: f64 = (10.0 * 2.220446049250313e-16);
        let assign33590_e48771: f64 = (1.0 + assign33590_e48770);
        let assign33590_e48773: f64 = if ((assign33590_e48763 <= p.p113) && (p.p113 <= assign33590_e48771)) { 1.0 } else { 0.0 };
        locals.var_guard1121 = assign33590_e48773;

        let (assign33600_e48779, assign33600_e48779_d_n0, assign33600_e48779_d_n2, assign33600_e48779_d_n6, assign33600_e48779_d_n7, assign33600_e48779_d_n10, assign33600_e48779_d_n11, assign33600_e48779_d_n12, assign33600_e48779_d_n17,) = {
    if ((locals.var_guard1120 != 0.0) && (locals.var_guard1121 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk1105, locals.var_t7__blk1105_dn0, locals.var_t7__blk1105_dn2, locals.var_t7__blk1105_dn6, locals.var_t7__blk1105_dn7, locals.var_t7__blk1105_dn10, locals.var_t7__blk1105_dn11, locals.var_t7__blk1105_dn12, locals.var_t7__blk1105_dn17,)
    }
};
        locals.var_t7__blk1105 = assign33600_e48779;
        locals.var_t7__blk1105_dn0 = assign33600_e48779_d_n0;
        locals.var_t7__blk1105_dn2 = assign33600_e48779_d_n2;
        locals.var_t7__blk1105_dn6 = assign33600_e48779_d_n6;
        locals.var_t7__blk1105_dn7 = assign33600_e48779_d_n7;
        locals.var_t7__blk1105_dn10 = assign33600_e48779_d_n10;
        locals.var_t7__blk1105_dn11 = assign33600_e48779_d_n11;
        locals.var_t7__blk1105_dn12 = assign33600_e48779_d_n12;
        locals.var_t7__blk1105_dn17 = assign33600_e48779_d_n17;

        let assign33610_e48783: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48784: f64 = (2.0 - assign33610_e48783);
        let assign33610_e48791: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48792: f64 = (2.0 + assign33610_e48791);
        let assign33610_e48794: f64 = if ((assign33610_e48784 <= p.p113) && (p.p113 <= assign33610_e48792)) { 1.0 } else { 0.0 };
        locals.var_guard1122 = assign33610_e48794;

        let (assign33620_e48803, assign33620_e48803_d_n0, assign33620_e48803_d_n2, assign33620_e48803_d_n6, assign33620_e48803_d_n7, assign33620_e48803_d_n10, assign33620_e48803_d_n11, assign33620_e48803_d_n12, assign33620_e48803_d_n17,) = {
    if (((locals.var_guard1120 != 0.0) && (locals.var_guard1121 == 0.0)) && (locals.var_guard1122 != 0.0)) {
        (locals.var_t12__blk1104, locals.var_t12__blk1104_dn0, locals.var_t12__blk1104_dn2, locals.var_t12__blk1104_dn6, locals.var_t12__blk1104_dn7, locals.var_t12__blk1104_dn10, locals.var_t12__blk1104_dn11, locals.var_t12__blk1104_dn12, locals.var_t12__blk1104_dn17,)
    } else {
        (locals.var_t7__blk1105, locals.var_t7__blk1105_dn0, locals.var_t7__blk1105_dn2, locals.var_t7__blk1105_dn6, locals.var_t7__blk1105_dn7, locals.var_t7__blk1105_dn10, locals.var_t7__blk1105_dn11, locals.var_t7__blk1105_dn12, locals.var_t7__blk1105_dn17,)
    }
};
        locals.var_t7__blk1105 = assign33620_e48803;
        locals.var_t7__blk1105_dn0 = assign33620_e48803_d_n0;
        locals.var_t7__blk1105_dn2 = assign33620_e48803_d_n2;
        locals.var_t7__blk1105_dn6 = assign33620_e48803_d_n6;
        locals.var_t7__blk1105_dn7 = assign33620_e48803_d_n7;
        locals.var_t7__blk1105_dn10 = assign33620_e48803_d_n10;
        locals.var_t7__blk1105_dn11 = assign33620_e48803_d_n11;
        locals.var_t7__blk1105_dn12 = assign33620_e48803_d_n12;
        locals.var_t7__blk1105_dn17 = assign33620_e48803_d_n17;

        let (assign33630_e48817, assign33630_e48817_d_n0, assign33630_e48817_d_n2, assign33630_e48817_d_n6, assign33630_e48817_d_n7, assign33630_e48817_d_n10, assign33630_e48817_d_n11, assign33630_e48817_d_n12, assign33630_e48817_d_n17,) = {
    if (((locals.var_guard1120 != 0.0) && (locals.var_guard1121 == 0.0)) && (locals.var_guard1122 == 0.0)) {
        let assign33630_e48814: f64 = (p.p113 - 1.0);
        let assign33630_e48815: f64 = (locals.var_t12__blk1104).powf(assign33630_e48814);
        (assign33630_e48815, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn0)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn0 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn2)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn2 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn6)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn6 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn7)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn7 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn10)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn10 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn11)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn11 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn12)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn12 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn17)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn17 / locals.var_t12__blk1104))) },)
    } else {
        (locals.var_t7__blk1105, locals.var_t7__blk1105_dn0, locals.var_t7__blk1105_dn2, locals.var_t7__blk1105_dn6, locals.var_t7__blk1105_dn7, locals.var_t7__blk1105_dn10, locals.var_t7__blk1105_dn11, locals.var_t7__blk1105_dn12, locals.var_t7__blk1105_dn17,)
    }
};
        locals.var_t7__blk1105 = assign33630_e48817;
        locals.var_t7__blk1105_dn0 = assign33630_e48817_d_n0;
        locals.var_t7__blk1105_dn2 = assign33630_e48817_d_n2;
        locals.var_t7__blk1105_dn6 = assign33630_e48817_d_n6;
        locals.var_t7__blk1105_dn7 = assign33630_e48817_d_n7;
        locals.var_t7__blk1105_dn10 = assign33630_e48817_d_n10;
        locals.var_t7__blk1105_dn11 = assign33630_e48817_d_n11;
        locals.var_t7__blk1105_dn12 = assign33630_e48817_d_n12;
        locals.var_t7__blk1105_dn17 = assign33630_e48817_d_n17;

        let (assign33640_e48823, assign33640_e48823_d_n0, assign33640_e48823_d_n2, assign33640_e48823_d_n6, assign33640_e48823_d_n7, assign33640_e48823_d_n10, assign33640_e48823_d_n11, assign33640_e48823_d_n12, assign33640_e48823_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33640_e48821: f64 = (locals.var_t12__blk1104 * locals.var_t7__blk1105);
        (assign33640_e48821, ((locals.var_t12__blk1104_dn0 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn0)), ((locals.var_t12__blk1104_dn2 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn2)), ((locals.var_t12__blk1104_dn6 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn6)), ((locals.var_t12__blk1104_dn7 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn7)), ((locals.var_t12__blk1104_dn10 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn10)), ((locals.var_t12__blk1104_dn11 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn11)), ((locals.var_t12__blk1104_dn12 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn12)), ((locals.var_t12__blk1104_dn17 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn17)),)
    } else {
        (locals.var_t8__blk1106, locals.var_t8__blk1106_dn0, locals.var_t8__blk1106_dn2, locals.var_t8__blk1106_dn6, locals.var_t8__blk1106_dn7, locals.var_t8__blk1106_dn10, locals.var_t8__blk1106_dn11, locals.var_t8__blk1106_dn12, locals.var_t8__blk1106_dn17,)
    }
};
        locals.var_t8__blk1106 = assign33640_e48823;
        locals.var_t8__blk1106_dn0 = assign33640_e48823_d_n0;
        locals.var_t8__blk1106_dn2 = assign33640_e48823_d_n2;
        locals.var_t8__blk1106_dn6 = assign33640_e48823_d_n6;
        locals.var_t8__blk1106_dn7 = assign33640_e48823_d_n7;
        locals.var_t8__blk1106_dn10 = assign33640_e48823_d_n10;
        locals.var_t8__blk1106_dn11 = assign33640_e48823_d_n11;
        locals.var_t8__blk1106_dn12 = assign33640_e48823_d_n12;
        locals.var_t8__blk1106_dn17 = assign33640_e48823_d_n17;

        let (assign33650_e48829, assign33650_e48829_d_n0, assign33650_e48829_d_n2, assign33650_e48829_d_n6, assign33650_e48829_d_n7, assign33650_e48829_d_n10, assign33650_e48829_d_n11, assign33650_e48829_d_n12, assign33650_e48829_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33650_e48827: f64 = (1.0 + locals.var_t8__blk1106);
        (assign33650_e48827, locals.var_t8__blk1106_dn0, locals.var_t8__blk1106_dn2, locals.var_t8__blk1106_dn6, locals.var_t8__blk1106_dn7, locals.var_t8__blk1106_dn10, locals.var_t8__blk1106_dn11, locals.var_t8__blk1106_dn12, locals.var_t8__blk1106_dn17,)
    } else {
        (locals.var_t9__blk1107, locals.var_t9__blk1107_dn0, locals.var_t9__blk1107_dn2, locals.var_t9__blk1107_dn6, locals.var_t9__blk1107_dn7, locals.var_t9__blk1107_dn10, locals.var_t9__blk1107_dn11, locals.var_t9__blk1107_dn12, locals.var_t9__blk1107_dn17,)
    }
};
        locals.var_t9__blk1107 = assign33650_e48829;
        locals.var_t9__blk1107_dn0 = assign33650_e48829_d_n0;
        locals.var_t9__blk1107_dn2 = assign33650_e48829_d_n2;
        locals.var_t9__blk1107_dn6 = assign33650_e48829_d_n6;
        locals.var_t9__blk1107_dn7 = assign33650_e48829_d_n7;
        locals.var_t9__blk1107_dn10 = assign33650_e48829_d_n10;
        locals.var_t9__blk1107_dn11 = assign33650_e48829_d_n11;
        locals.var_t9__blk1107_dn12 = assign33650_e48829_d_n12;
        locals.var_t9__blk1107_dn17 = assign33650_e48829_d_n17;

        let (assign33660_e48840, assign33660_e48840_d_n0, assign33660_e48840_d_n2, assign33660_e48840_d_n6, assign33660_e48840_d_n7, assign33660_e48840_d_n10, assign33660_e48840_d_n11, assign33660_e48840_d_n12, assign33660_e48840_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33660_e48833: f64 = (-1.0);
        let assign33660_e48835: f64 = (assign33660_e48833 / p.p113);
        let assign33660_e48837: f64 = (assign33660_e48835 - 1.0);
        let assign33660_e48838: f64 = (locals.var_t9__blk1107).powf(assign33660_e48837);
        (assign33660_e48838, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn0)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn0 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn2)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn2 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn6)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn6 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn7)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn7 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn10)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn10 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn11)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn11 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn12)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn12 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn17)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn17 / locals.var_t9__blk1107))) },)
    } else {
        (locals.var_t10__blk1108, locals.var_t10__blk1108_dn0, locals.var_t10__blk1108_dn2, locals.var_t10__blk1108_dn6, locals.var_t10__blk1108_dn7, locals.var_t10__blk1108_dn10, locals.var_t10__blk1108_dn11, locals.var_t10__blk1108_dn12, locals.var_t10__blk1108_dn17,)
    }
};
        locals.var_t10__blk1108 = assign33660_e48840;
        locals.var_t10__blk1108_dn0 = assign33660_e48840_d_n0;
        locals.var_t10__blk1108_dn2 = assign33660_e48840_d_n2;
        locals.var_t10__blk1108_dn6 = assign33660_e48840_d_n6;
        locals.var_t10__blk1108_dn7 = assign33660_e48840_d_n7;
        locals.var_t10__blk1108_dn10 = assign33660_e48840_d_n10;
        locals.var_t10__blk1108_dn11 = assign33660_e48840_d_n11;
        locals.var_t10__blk1108_dn12 = assign33660_e48840_d_n12;
        locals.var_t10__blk1108_dn17 = assign33660_e48840_d_n17;

        let (assign33670_e48846, assign33670_e48846_d_n0, assign33670_e48846_d_n2, assign33670_e48846_d_n6, assign33670_e48846_d_n7, assign33670_e48846_d_n10, assign33670_e48846_d_n11, assign33670_e48846_d_n12, assign33670_e48846_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33670_e48844: f64 = (locals.var_t9__blk1107 * locals.var_t10__blk1108);
        (assign33670_e48844, ((locals.var_t9__blk1107_dn0 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn0)), ((locals.var_t9__blk1107_dn2 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn2)), ((locals.var_t9__blk1107_dn6 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn6)), ((locals.var_t9__blk1107_dn7 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn7)), ((locals.var_t9__blk1107_dn10 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn10)), ((locals.var_t9__blk1107_dn11 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn11)), ((locals.var_t9__blk1107_dn12 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn12)), ((locals.var_t9__blk1107_dn17 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn17)),)
    } else {
        (locals.var_t11__blk1109, locals.var_t11__blk1109_dn0, locals.var_t11__blk1109_dn2, locals.var_t11__blk1109_dn6, locals.var_t11__blk1109_dn7, locals.var_t11__blk1109_dn10, locals.var_t11__blk1109_dn11, locals.var_t11__blk1109_dn12, locals.var_t11__blk1109_dn17,)
    }
};
        locals.var_t11__blk1109 = assign33670_e48846;
        locals.var_t11__blk1109_dn0 = assign33670_e48846_d_n0;
        locals.var_t11__blk1109_dn2 = assign33670_e48846_d_n2;
        locals.var_t11__blk1109_dn6 = assign33670_e48846_d_n6;
        locals.var_t11__blk1109_dn7 = assign33670_e48846_d_n7;
        locals.var_t11__blk1109_dn10 = assign33670_e48846_d_n10;
        locals.var_t11__blk1109_dn11 = assign33670_e48846_d_n11;
        locals.var_t11__blk1109_dn12 = assign33670_e48846_d_n12;
        locals.var_t11__blk1109_dn17 = assign33670_e48846_d_n17;

        let (assign33680_e48852, assign33680_e48852_d_n0, assign33680_e48852_d_n2, assign33680_e48852_d_n6, assign33680_e48852_d_n7, assign33680_e48852_d_n10, assign33680_e48852_d_n11, assign33680_e48852_d_n12, assign33680_e48852_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33680_e48850: f64 = (locals.var_muun * locals.var_t11__blk1109);
        (assign33680_e48850, ((locals.var_muun_dn0 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn0)), ((locals.var_muun_dn2 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn2)), ((locals.var_muun_dn6 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn6)), ((locals.var_muun_dn7 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn7)), ((locals.var_muun_dn10 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn10)), ((locals.var_muun_dn11 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn11)), ((locals.var_muun_dn12 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn12)), ((locals.var_muun_dn17 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn17)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn12, locals.var_mud_hoso_dn17,)
    }
};
        locals.var_mud_hoso = assign33680_e48852;
        locals.var_mud_hoso_dn0 = assign33680_e48852_d_n0;
        locals.var_mud_hoso_dn2 = assign33680_e48852_d_n2;
        locals.var_mud_hoso_dn6 = assign33680_e48852_d_n6;
        locals.var_mud_hoso_dn7 = assign33680_e48852_d_n7;
        locals.var_mud_hoso_dn10 = assign33680_e48852_d_n10;
        locals.var_mud_hoso_dn11 = assign33680_e48852_d_n11;
        locals.var_mud_hoso_dn12 = assign33680_e48852_d_n12;
        locals.var_mud_hoso_dn17 = assign33680_e48852_d_n17;

        let (assign33690_e48860, assign33690_e48860_d_n0, assign33690_e48860_d_n2, assign33690_e48860_d_n6, assign33690_e48860_d_n7, assign33690_e48860_d_n10, assign33690_e48860_d_n11, assign33690_e48860_d_n12, assign33690_e48860_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33690_e48856: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign33690_e48858: f64 = (assign33690_e48856 / 2.0);
        (assign33690_e48858, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn12 + locals.var_mud_hoso_dn12) / 2.0), ((locals.var_mu_dn17 + locals.var_mud_hoso_dn17) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn12, locals.var_mu_ave_dn17,)
    }
};
        locals.var_mu_ave = assign33690_e48860;
        locals.var_mu_ave_dn0 = assign33690_e48860_d_n0;
        locals.var_mu_ave_dn2 = assign33690_e48860_d_n2;
        locals.var_mu_ave_dn6 = assign33690_e48860_d_n6;
        locals.var_mu_ave_dn7 = assign33690_e48860_d_n7;
        locals.var_mu_ave_dn10 = assign33690_e48860_d_n10;
        locals.var_mu_ave_dn11 = assign33690_e48860_d_n11;
        locals.var_mu_ave_dn12 = assign33690_e48860_d_n12;
        locals.var_mu_ave_dn17 = assign33690_e48860_d_n17;

        let (assign33700_e48866, assign33700_e48866_d_n0, assign33700_e48866_d_n2, assign33700_e48866_d_n6, assign33700_e48866_d_n7, assign33700_e48866_d_n10, assign33700_e48866_d_n11, assign33700_e48866_d_n12, assign33700_e48866_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33700_e48864: f64 = (locals.var_alpha * locals.var_alpha);
        (assign33700_e48864, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_t0__blk1110, locals.var_t0__blk1110_dn0, locals.var_t0__blk1110_dn2, locals.var_t0__blk1110_dn6, locals.var_t0__blk1110_dn7, locals.var_t0__blk1110_dn10, locals.var_t0__blk1110_dn11, locals.var_t0__blk1110_dn12, locals.var_t0__blk1110_dn17,)
    }
};
        locals.var_t0__blk1110 = assign33700_e48866;
        locals.var_t0__blk1110_dn0 = assign33700_e48866_d_n0;
        locals.var_t0__blk1110_dn2 = assign33700_e48866_d_n2;
        locals.var_t0__blk1110_dn6 = assign33700_e48866_d_n6;
        locals.var_t0__blk1110_dn7 = assign33700_e48866_d_n7;
        locals.var_t0__blk1110_dn10 = assign33700_e48866_d_n10;
        locals.var_t0__blk1110_dn11 = assign33700_e48866_d_n11;
        locals.var_t0__blk1110_dn12 = assign33700_e48866_d_n12;
        locals.var_t0__blk1110_dn17 = assign33700_e48866_d_n17;

    }

    pub(super) fn stamp_transient_block_118(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33710_e48928, assign33710_e48928_d_n0, assign33710_e48928_d_n2, assign33710_e48928_d_n6, assign33710_e48928_d_n7, assign33710_e48928_d_n10, assign33710_e48928_d_n11, assign33710_e48928_d_n12, assign33710_e48928_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33710_e48870: f64 = (locals.var_weff_nf * locals.var_c_fox);
        let assign33710_e48872: f64 = (assign33710_e48870 * locals.var_vgvt);
        let assign33710_e48874: f64 = (assign33710_e48872 * locals.var_mu);
        let assign33710_e48878: f64 = (3.0 * locals.var_alpha);
        let assign33710_e48879: f64 = (1.0 + assign33710_e48878);
        let assign33710_e48882: f64 = (6.0 * locals.var_t0__blk1110);
        let assign33710_e48883: f64 = (assign33710_e48879 + assign33710_e48882);
        let assign33710_e48885: f64 = (assign33710_e48883 * locals.var_mud_hoso);
        let assign33710_e48887: f64 = (assign33710_e48885 * locals.var_mud_hoso);
        let assign33710_e48891: f64 = (4.0 * locals.var_alpha);
        let assign33710_e48892: f64 = (3.0 + assign33710_e48891);
        let assign33710_e48895: f64 = (3.0 * locals.var_t0__blk1110);
        let assign33710_e48896: f64 = (assign33710_e48892 + assign33710_e48895);
        let assign33710_e48898: f64 = (assign33710_e48896 * locals.var_mud_hoso);
        let assign33710_e48900: f64 = (assign33710_e48898 * locals.var_mu);
        let assign33710_e48901: f64 = (assign33710_e48887 + assign33710_e48900);
        let assign33710_e48905: f64 = (3.0 * locals.var_alpha);
        let assign33710_e48906: f64 = (6.0 + assign33710_e48905);
        let assign33710_e48908: f64 = (assign33710_e48906 + locals.var_t0__blk1110);
        let assign33710_e48910: f64 = (assign33710_e48908 * locals.var_mu);
        let assign33710_e48912: f64 = (assign33710_e48910 * locals.var_mu);
        let assign33710_e48913: f64 = (assign33710_e48901 + assign33710_e48912);
        let assign33710_e48914: f64 = (assign33710_e48874 * assign33710_e48913);
        let assign33710_e48917: f64 = (15.0 * locals.var_lch);
        let assign33710_e48920: f64 = (1.0 + locals.var_alpha);
        let assign33710_e48921: f64 = (assign33710_e48917 * assign33710_e48920);
        let assign33710_e48923: f64 = (assign33710_e48921 * locals.var_mu_ave);
        let assign33710_e48925: f64 = (assign33710_e48923 * locals.var_mu_ave);
        let assign33710_e48926: f64 = (assign33710_e48914 / assign33710_e48925);
        (assign33710_e48926, ((((((((((locals.var_weff_nf * locals.var_c_fox_dn0) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn0)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0__blk1110_dn0)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0__blk1110_dn0)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0__blk1110_dn0) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn0)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn0))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn0) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn0)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn2) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn2)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0__blk1110_dn2)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0__blk1110_dn2)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0__blk1110_dn2) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn2)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn2))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn2) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn2)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn6) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn6)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0__blk1110_dn6)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0__blk1110_dn6)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0__blk1110_dn6) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn6)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn6))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn6) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn6)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn7) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn7)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0__blk1110_dn7)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0__blk1110_dn7)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0__blk1110_dn7) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn7)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn7))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn7) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn7)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn10) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn10)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0__blk1110_dn10)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0__blk1110_dn10)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0__blk1110_dn10) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn10)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn10))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn10) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn10)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn11) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn11)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0__blk1110_dn11)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0__blk1110_dn11)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0__blk1110_dn11) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn11)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn11))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn11) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn11)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn12) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn12)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn12)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn12) + (6.0 * locals.var_t0__blk1110_dn12)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn12)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn12)) + ((((((4.0 * locals.var_alpha_dn12) + (3.0 * locals.var_t0__blk1110_dn12)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn12)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn12))) + ((((((3.0 * locals.var_alpha_dn12) + locals.var_t0__blk1110_dn12) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn12)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn12))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn12) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn12)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn12)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn12)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn17) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn17)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn17)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn17) + (6.0 * locals.var_t0__blk1110_dn17)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn17)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn17)) + ((((((4.0 * locals.var_alpha_dn17) + (3.0 * locals.var_t0__blk1110_dn17)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn17)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn17))) + ((((((3.0 * locals.var_alpha_dn17) + locals.var_t0__blk1110_dn17) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn17)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn17))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn17) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn17)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn17)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn17)))) / (assign33710_e48925 * assign33710_e48925)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17,)
    }
};
        locals.var_nthrml = assign33710_e48928;
        locals.var_nthrml_dn0 = assign33710_e48928_d_n0;
        locals.var_nthrml_dn2 = assign33710_e48928_d_n2;
        locals.var_nthrml_dn6 = assign33710_e48928_d_n6;
        locals.var_nthrml_dn7 = assign33710_e48928_d_n7;
        locals.var_nthrml_dn10 = assign33710_e48928_d_n10;
        locals.var_nthrml_dn11 = assign33710_e48928_d_n11;
        locals.var_nthrml_dn12 = assign33710_e48928_d_n12;
        locals.var_nthrml_dn17 = assign33710_e48928_d_n17;

        let (assign33720_e48933, assign33720_e48933_d_n0, assign33720_e48933_d_n2, assign33720_e48933_d_n6, assign33720_e48933_d_n7, assign33720_e48933_d_n10, assign33720_e48933_d_n11, assign33720_e48933_d_n12, assign33720_e48933_d_n17,) = {
    if (locals.var_guard1120 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17,)
    }
};
        locals.var_nthrml = assign33720_e48933;
        locals.var_nthrml_dn0 = assign33720_e48933_d_n0;
        locals.var_nthrml_dn2 = assign33720_e48933_d_n2;
        locals.var_nthrml_dn6 = assign33720_e48933_d_n6;
        locals.var_nthrml_dn7 = assign33720_e48933_d_n7;
        locals.var_nthrml_dn10 = assign33720_e48933_d_n10;
        locals.var_nthrml_dn11 = assign33720_e48933_d_n11;
        locals.var_nthrml_dn12 = assign33720_e48933_d_n12;
        locals.var_nthrml_dn17 = assign33720_e48933_d_n17;

        let assign33730_e48947: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1123 = assign33730_e48947;

        let (assign33740_e48952, assign33740_e48952_d_n0, assign33740_e48952_d_n2, assign33740_e48952_d_n6, assign33740_e48952_d_n7, assign33740_e48952_d_n10, assign33740_e48952_d_n11, assign33740_e48952_d_n12, assign33740_e48952_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33740_e48950: f64 = (locals.var_kusail).sqrt();
        (assign33740_e48950, (locals.var_kusail_dn0 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn2 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn6 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn7 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn10 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn11 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn12 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn17 / (2.0 * assign33740_e48950)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn12, locals.var_sqrtkusail_dn17,)
    }
};
        locals.var_sqrtkusail = assign33740_e48952;
        locals.var_sqrtkusail_dn0 = assign33740_e48952_d_n0;
        locals.var_sqrtkusail_dn2 = assign33740_e48952_d_n2;
        locals.var_sqrtkusail_dn6 = assign33740_e48952_d_n6;
        locals.var_sqrtkusail_dn7 = assign33740_e48952_d_n7;
        locals.var_sqrtkusail_dn10 = assign33740_e48952_d_n10;
        locals.var_sqrtkusail_dn11 = assign33740_e48952_d_n11;
        locals.var_sqrtkusail_dn12 = assign33740_e48952_d_n12;
        locals.var_sqrtkusail_dn17 = assign33740_e48952_d_n17;

        let (assign33750_e48958, assign33750_e48958_d_n0, assign33750_e48958_d_n2, assign33750_e48958_d_n6, assign33750_e48958_d_n7, assign33750_e48958_d_n10, assign33750_e48958_d_n11, assign33750_e48958_d_n12, assign33750_e48958_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33750_e48956: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign33750_e48956, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12), (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17),)
    } else {
        (locals.var_t2__blk1112, locals.var_t2__blk1112_dn0, locals.var_t2__blk1112_dn2, locals.var_t2__blk1112_dn6, locals.var_t2__blk1112_dn7, locals.var_t2__blk1112_dn10, locals.var_t2__blk1112_dn11, locals.var_t2__blk1112_dn12, locals.var_t2__blk1112_dn17,)
    }
};
        locals.var_t2__blk1112 = assign33750_e48958;
        locals.var_t2__blk1112_dn0 = assign33750_e48958_d_n0;
        locals.var_t2__blk1112_dn2 = assign33750_e48958_d_n2;
        locals.var_t2__blk1112_dn6 = assign33750_e48958_d_n6;
        locals.var_t2__blk1112_dn7 = assign33750_e48958_d_n7;
        locals.var_t2__blk1112_dn10 = assign33750_e48958_d_n10;
        locals.var_t2__blk1112_dn11 = assign33750_e48958_d_n11;
        locals.var_t2__blk1112_dn12 = assign33750_e48958_d_n12;
        locals.var_t2__blk1112_dn17 = assign33750_e48958_d_n17;

        let (assign33760_e48964, assign33760_e48964_d_n0, assign33760_e48964_d_n2, assign33760_e48964_d_n6, assign33760_e48964_d_n7, assign33760_e48964_d_n10, assign33760_e48964_d_n11, assign33760_e48964_d_n12, assign33760_e48964_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33760_e48962: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign33760_e48962, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)), ((locals.var_kusai00_dn17 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn17)),)
    } else {
        (locals.var_t3__blk1113, locals.var_t3__blk1113_dn0, locals.var_t3__blk1113_dn2, locals.var_t3__blk1113_dn6, locals.var_t3__blk1113_dn7, locals.var_t3__blk1113_dn10, locals.var_t3__blk1113_dn11, locals.var_t3__blk1113_dn12, locals.var_t3__blk1113_dn17,)
    }
};
        locals.var_t3__blk1113 = assign33760_e48964;
        locals.var_t3__blk1113_dn0 = assign33760_e48964_d_n0;
        locals.var_t3__blk1113_dn2 = assign33760_e48964_d_n2;
        locals.var_t3__blk1113_dn6 = assign33760_e48964_d_n6;
        locals.var_t3__blk1113_dn7 = assign33760_e48964_d_n7;
        locals.var_t3__blk1113_dn10 = assign33760_e48964_d_n10;
        locals.var_t3__blk1113_dn11 = assign33760_e48964_d_n11;
        locals.var_t3__blk1113_dn12 = assign33760_e48964_d_n12;
        locals.var_t3__blk1113_dn17 = assign33760_e48964_d_n17;

        let (assign33770_e48970, assign33770_e48970_d_n0, assign33770_e48970_d_n2, assign33770_e48970_d_n6, assign33770_e48970_d_n7, assign33770_e48970_d_n10, assign33770_e48970_d_n11, assign33770_e48970_d_n12, assign33770_e48970_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33770_e48968: f64 = (locals.var_kusail * locals.var_kusail);
        (assign33770_e48968, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)), ((locals.var_kusail_dn17 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn17)),)
    } else {
        (locals.var_t4__blk1114, locals.var_t4__blk1114_dn0, locals.var_t4__blk1114_dn2, locals.var_t4__blk1114_dn6, locals.var_t4__blk1114_dn7, locals.var_t4__blk1114_dn10, locals.var_t4__blk1114_dn11, locals.var_t4__blk1114_dn12, locals.var_t4__blk1114_dn17,)
    }
};
        locals.var_t4__blk1114 = assign33770_e48970;
        locals.var_t4__blk1114_dn0 = assign33770_e48970_d_n0;
        locals.var_t4__blk1114_dn2 = assign33770_e48970_d_n2;
        locals.var_t4__blk1114_dn6 = assign33770_e48970_d_n6;
        locals.var_t4__blk1114_dn7 = assign33770_e48970_d_n7;
        locals.var_t4__blk1114_dn10 = assign33770_e48970_d_n10;
        locals.var_t4__blk1114_dn11 = assign33770_e48970_d_n11;
        locals.var_t4__blk1114_dn12 = assign33770_e48970_d_n12;
        locals.var_t4__blk1114_dn17 = assign33770_e48970_d_n17;

        let (assign33780_e48978, assign33780_e48978_d_n0, assign33780_e48978_d_n2, assign33780_e48978_d_n6, assign33780_e48978_d_n7, assign33780_e48978_d_n10, assign33780_e48978_d_n11, assign33780_e48978_d_n12, assign33780_e48978_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33780_e48974: f64 = (42.0 * locals.var_kusai00);
        let assign33780_e48976: f64 = (assign33780_e48974 * locals.var_kusail);
        (assign33780_e48976, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn12) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn12)), (((42.0 * locals.var_kusai00_dn17) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn17)),)
    } else {
        (locals.var_t5__blk1115, locals.var_t5__blk1115_dn0, locals.var_t5__blk1115_dn2, locals.var_t5__blk1115_dn6, locals.var_t5__blk1115_dn7, locals.var_t5__blk1115_dn10, locals.var_t5__blk1115_dn11, locals.var_t5__blk1115_dn12, locals.var_t5__blk1115_dn17,)
    }
};
        locals.var_t5__blk1115 = assign33780_e48978;
        locals.var_t5__blk1115_dn0 = assign33780_e48978_d_n0;
        locals.var_t5__blk1115_dn2 = assign33780_e48978_d_n2;
        locals.var_t5__blk1115_dn6 = assign33780_e48978_d_n6;
        locals.var_t5__blk1115_dn7 = assign33780_e48978_d_n7;
        locals.var_t5__blk1115_dn10 = assign33780_e48978_d_n10;
        locals.var_t5__blk1115_dn11 = assign33780_e48978_d_n11;
        locals.var_t5__blk1115_dn12 = assign33780_e48978_d_n12;
        locals.var_t5__blk1115_dn17 = assign33780_e48978_d_n17;

        let (assign33790_e48988, assign33790_e48988_d_n0, assign33790_e48988_d_n2, assign33790_e48988_d_n6, assign33790_e48988_d_n7, assign33790_e48988_d_n10, assign33790_e48988_d_n11, assign33790_e48988_d_n12, assign33790_e48988_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33790_e48984: f64 = (locals.var_t3__blk1113 + locals.var_t4__blk1114);
        let assign33790_e48985: f64 = (4.0 * assign33790_e48984);
        let assign33790_e48986: f64 = (locals.var_t5__blk1115 + assign33790_e48985);
        (assign33790_e48986, (locals.var_t5__blk1115_dn0 + (4.0 * (locals.var_t3__blk1113_dn0 + locals.var_t4__blk1114_dn0))), (locals.var_t5__blk1115_dn2 + (4.0 * (locals.var_t3__blk1113_dn2 + locals.var_t4__blk1114_dn2))), (locals.var_t5__blk1115_dn6 + (4.0 * (locals.var_t3__blk1113_dn6 + locals.var_t4__blk1114_dn6))), (locals.var_t5__blk1115_dn7 + (4.0 * (locals.var_t3__blk1113_dn7 + locals.var_t4__blk1114_dn7))), (locals.var_t5__blk1115_dn10 + (4.0 * (locals.var_t3__blk1113_dn10 + locals.var_t4__blk1114_dn10))), (locals.var_t5__blk1115_dn11 + (4.0 * (locals.var_t3__blk1113_dn11 + locals.var_t4__blk1114_dn11))), (locals.var_t5__blk1115_dn12 + (4.0 * (locals.var_t3__blk1113_dn12 + locals.var_t4__blk1114_dn12))), (locals.var_t5__blk1115_dn17 + (4.0 * (locals.var_t3__blk1113_dn17 + locals.var_t4__blk1114_dn17))),)
    } else {
        (locals.var_t5__blk1115, locals.var_t5__blk1115_dn0, locals.var_t5__blk1115_dn2, locals.var_t5__blk1115_dn6, locals.var_t5__blk1115_dn7, locals.var_t5__blk1115_dn10, locals.var_t5__blk1115_dn11, locals.var_t5__blk1115_dn12, locals.var_t5__blk1115_dn17,)
    }
};
        locals.var_t5__blk1115 = assign33790_e48988;
        locals.var_t5__blk1115_dn0 = assign33790_e48988_d_n0;
        locals.var_t5__blk1115_dn2 = assign33790_e48988_d_n2;
        locals.var_t5__blk1115_dn6 = assign33790_e48988_d_n6;
        locals.var_t5__blk1115_dn7 = assign33790_e48988_d_n7;
        locals.var_t5__blk1115_dn10 = assign33790_e48988_d_n10;
        locals.var_t5__blk1115_dn11 = assign33790_e48988_d_n11;
        locals.var_t5__blk1115_dn12 = assign33790_e48988_d_n12;
        locals.var_t5__blk1115_dn17 = assign33790_e48988_d_n17;

        let (assign33800_e49002, assign33800_e49002_d_n0, assign33800_e49002_d_n2, assign33800_e49002_d_n6, assign33800_e49002_d_n7, assign33800_e49002_d_n10, assign33800_e49002_d_n11, assign33800_e49002_d_n12, assign33800_e49002_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33800_e48993: f64 = (20.0 * locals.var_sqrtkusail);
        let assign33800_e48995: f64 = (assign33800_e48993 * locals.var_vgvt);
        let assign33800_e48998: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign33800_e48999: f64 = (assign33800_e48995 * assign33800_e48998);
        let assign33800_e49000: f64 = (locals.var_t5__blk1115 + assign33800_e48999);
        (assign33800_e49000, (locals.var_t5__blk1115_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn0)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5__blk1115_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn2)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5__blk1115_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn6)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5__blk1115_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn7)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5__blk1115_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn10)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5__blk1115_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn11)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5__blk1115_dn12 + (((((20.0 * locals.var_sqrtkusail_dn12) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn12)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn12 + locals.var_kusail_dn12)))), (locals.var_t5__blk1115_dn17 + (((((20.0 * locals.var_sqrtkusail_dn17) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn17)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn17 + locals.var_kusail_dn17)))),)
    } else {
        (locals.var_t5__blk1115, locals.var_t5__blk1115_dn0, locals.var_t5__blk1115_dn2, locals.var_t5__blk1115_dn6, locals.var_t5__blk1115_dn7, locals.var_t5__blk1115_dn10, locals.var_t5__blk1115_dn11, locals.var_t5__blk1115_dn12, locals.var_t5__blk1115_dn17,)
    }
};
        locals.var_t5__blk1115 = assign33800_e49002;
        locals.var_t5__blk1115_dn0 = assign33800_e49002_d_n0;
        locals.var_t5__blk1115_dn2 = assign33800_e49002_d_n2;
        locals.var_t5__blk1115_dn6 = assign33800_e49002_d_n6;
        locals.var_t5__blk1115_dn7 = assign33800_e49002_d_n7;
        locals.var_t5__blk1115_dn10 = assign33800_e49002_d_n10;
        locals.var_t5__blk1115_dn11 = assign33800_e49002_d_n11;
        locals.var_t5__blk1115_dn12 = assign33800_e49002_d_n12;
        locals.var_t5__blk1115_dn17 = assign33800_e49002_d_n17;

        let (assign33810_e49008, assign33810_e49008_d_n0, assign33810_e49008_d_n2, assign33810_e49008_d_n6, assign33810_e49008_d_n7, assign33810_e49008_d_n10, assign33810_e49008_d_n11, assign33810_e49008_d_n12, assign33810_e49008_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33810_e49006: f64 = (locals.var_t2__blk1112 * locals.var_t2__blk1112);
        (assign33810_e49006, ((locals.var_t2__blk1112_dn0 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn0)), ((locals.var_t2__blk1112_dn2 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn2)), ((locals.var_t2__blk1112_dn6 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn6)), ((locals.var_t2__blk1112_dn7 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn7)), ((locals.var_t2__blk1112_dn10 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn10)), ((locals.var_t2__blk1112_dn11 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn11)), ((locals.var_t2__blk1112_dn12 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn12)), ((locals.var_t2__blk1112_dn17 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn17)),)
    } else {
        (locals.var_t10w, locals.var_t10w_dn0, locals.var_t10w_dn2, locals.var_t10w_dn6, locals.var_t10w_dn7, locals.var_t10w_dn10, locals.var_t10w_dn11, locals.var_t10w_dn12, locals.var_t10w_dn17,)
    }
};
        locals.var_t10w = assign33810_e49008;
        locals.var_t10w_dn0 = assign33810_e49008_d_n0;
        locals.var_t10w_dn2 = assign33810_e49008_d_n2;
        locals.var_t10w_dn6 = assign33810_e49008_d_n6;
        locals.var_t10w_dn7 = assign33810_e49008_d_n7;
        locals.var_t10w_dn10 = assign33810_e49008_d_n10;
        locals.var_t10w_dn11 = assign33810_e49008_d_n11;
        locals.var_t10w_dn12 = assign33810_e49008_d_n12;
        locals.var_t10w_dn17 = assign33810_e49008_d_n17;

        let (assign33820_e49014, assign33820_e49014_d_n0, assign33820_e49014_d_n2, assign33820_e49014_d_n6, assign33820_e49014_d_n7, assign33820_e49014_d_n10, assign33820_e49014_d_n11, assign33820_e49014_d_n12, assign33820_e49014_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33820_e49012: f64 = (locals.var_t10w * locals.var_t10w);
        (assign33820_e49012, ((locals.var_t10w_dn0 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn0)), ((locals.var_t10w_dn2 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn2)), ((locals.var_t10w_dn6 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn6)), ((locals.var_t10w_dn7 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn7)), ((locals.var_t10w_dn10 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn10)), ((locals.var_t10w_dn11 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn11)), ((locals.var_t10w_dn12 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn12)), ((locals.var_t10w_dn17 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn17)),)
    } else {
        (locals.var_t10__blk1108, locals.var_t10__blk1108_dn0, locals.var_t10__blk1108_dn2, locals.var_t10__blk1108_dn6, locals.var_t10__blk1108_dn7, locals.var_t10__blk1108_dn10, locals.var_t10__blk1108_dn11, locals.var_t10__blk1108_dn12, locals.var_t10__blk1108_dn17,)
    }
};
        locals.var_t10__blk1108 = assign33820_e49014;
        locals.var_t10__blk1108_dn0 = assign33820_e49014_d_n0;
        locals.var_t10__blk1108_dn2 = assign33820_e49014_d_n2;
        locals.var_t10__blk1108_dn6 = assign33820_e49014_d_n6;
        locals.var_t10__blk1108_dn7 = assign33820_e49014_d_n7;
        locals.var_t10__blk1108_dn10 = assign33820_e49014_d_n10;
        locals.var_t10__blk1108_dn11 = assign33820_e49014_d_n11;
        locals.var_t10__blk1108_dn12 = assign33820_e49014_d_n12;
        locals.var_t10__blk1108_dn17 = assign33820_e49014_d_n17;

        let (assign33830_e49022, assign33830_e49022_d_n0, assign33830_e49022_d_n2, assign33830_e49022_d_n6, assign33830_e49022_d_n7, assign33830_e49022_d_n10, assign33830_e49022_d_n11, assign33830_e49022_d_n12, assign33830_e49022_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33830_e49019: f64 = (locals.var_t10__blk1108 * locals.var_t2__blk1112);
        let assign33830_e49020: f64 = (locals.var_t5__blk1115 / assign33830_e49019);
        (assign33830_e49020, (((locals.var_t5__blk1115_dn0 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn0 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn0)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn2 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn2 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn2)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn6 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn6 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn6)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn7 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn7 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn7)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn10 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn10 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn10)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn11 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn11 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn11)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn12 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn12 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn12)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn17 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn17 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn17)))) / (assign33830_e49019 * assign33830_e49019)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn12, locals.var_kusai_ig_dn17,)
    }
};
        locals.var_kusai_ig = assign33830_e49022;
        locals.var_kusai_ig_dn0 = assign33830_e49022_d_n0;
        locals.var_kusai_ig_dn2 = assign33830_e49022_d_n2;
        locals.var_kusai_ig_dn6 = assign33830_e49022_d_n6;
        locals.var_kusai_ig_dn7 = assign33830_e49022_d_n7;
        locals.var_kusai_ig_dn10 = assign33830_e49022_d_n10;
        locals.var_kusai_ig_dn11 = assign33830_e49022_d_n11;
        locals.var_kusai_ig_dn12 = assign33830_e49022_d_n12;
        locals.var_kusai_ig_dn17 = assign33830_e49022_d_n17;

        let (assign33840_e49032, assign33840_e49032_d_n0, assign33840_e49032_d_n2, assign33840_e49032_d_n6, assign33840_e49032_d_n7, assign33840_e49032_d_n10, assign33840_e49032_d_n11, assign33840_e49032_d_n12, assign33840_e49032_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33840_e49026: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign33840_e49028: f64 = (assign33840_e49026 * locals.var_mu);
        let assign33840_e49030: f64 = (assign33840_e49028 * locals.var_c_fox);
        (assign33840_e49030, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn0)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn2)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn6)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn7)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn10)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn11) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn11)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn11)), (((((-((locals.var_weff_nf * locals.var_lch_dn12) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn12)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn12)), (((((-((locals.var_weff_nf * locals.var_lch_dn17) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn17)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn12, locals.var_gds0_ign_dn17,)
    }
};
        locals.var_gds0_ign = assign33840_e49032;
        locals.var_gds0_ign_dn0 = assign33840_e49032_d_n0;
        locals.var_gds0_ign_dn2 = assign33840_e49032_d_n2;
        locals.var_gds0_ign_dn6 = assign33840_e49032_d_n6;
        locals.var_gds0_ign_dn7 = assign33840_e49032_d_n7;
        locals.var_gds0_ign_dn10 = assign33840_e49032_d_n10;
        locals.var_gds0_ign_dn11 = assign33840_e49032_d_n11;
        locals.var_gds0_ign_dn12 = assign33840_e49032_d_n12;
        locals.var_gds0_ign_dn17 = assign33840_e49032_d_n17;

        let (assign33850_e49038, assign33850_e49038_d_n0, assign33850_e49038_d_n2, assign33850_e49038_d_n6, assign33850_e49038_d_n7, assign33850_e49038_d_n10, assign33850_e49038_d_n11, assign33850_e49038_d_n12, assign33850_e49038_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33850_e49036: f64 = (locals.var_gds0_ign * locals.var_vgvt);
        (assign33850_e49036, ((locals.var_gds0_ign_dn0 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn0)), ((locals.var_gds0_ign_dn2 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn2)), ((locals.var_gds0_ign_dn6 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn6)), ((locals.var_gds0_ign_dn7 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn7)), ((locals.var_gds0_ign_dn10 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn10)), ((locals.var_gds0_ign_dn11 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn11)), ((locals.var_gds0_ign_dn12 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn12)), ((locals.var_gds0_ign_dn17 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn17)),)
    } else {
        (locals.var_gds0_h2, locals.var_gds0_h2_dn0, locals.var_gds0_h2_dn2, locals.var_gds0_h2_dn6, locals.var_gds0_h2_dn7, locals.var_gds0_h2_dn10, locals.var_gds0_h2_dn11, locals.var_gds0_h2_dn12, locals.var_gds0_h2_dn17,)
    }
};
        locals.var_gds0_h2 = assign33850_e49038;
        locals.var_gds0_h2_dn0 = assign33850_e49038_d_n0;
        locals.var_gds0_h2_dn2 = assign33850_e49038_d_n2;
        locals.var_gds0_h2_dn6 = assign33850_e49038_d_n6;
        locals.var_gds0_h2_dn7 = assign33850_e49038_d_n7;
        locals.var_gds0_h2_dn10 = assign33850_e49038_d_n10;
        locals.var_gds0_h2_dn11 = assign33850_e49038_d_n11;
        locals.var_gds0_h2_dn12 = assign33850_e49038_d_n12;
        locals.var_gds0_h2_dn17 = assign33850_e49038_d_n17;

        let (assign33860_e49044, assign33860_e49044_d_n0, assign33860_e49044_d_n2, assign33860_e49044_d_n6, assign33860_e49044_d_n7, assign33860_e49044_d_n10, assign33860_e49044_d_n11, assign33860_e49044_d_n12, assign33860_e49044_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33860_e49042: f64 = (locals.var_nthrml / locals.var_gds0_h2);
        (assign33860_e49042, (((locals.var_nthrml_dn0 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn0)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn2 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn2)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn6 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn6)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn7 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn7)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn10 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn10)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn11 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn11)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn12 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn12)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn17 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn17)) / (locals.var_gds0_h2 * locals.var_gds0_h2)),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn10, locals.var_gamma_dn11, locals.var_gamma_dn12, locals.var_gamma_dn17,)
    }
};
        locals.var_gamma = assign33860_e49044;
        locals.var_gamma_dn0 = assign33860_e49044_d_n0;
        locals.var_gamma_dn2 = assign33860_e49044_d_n2;
        locals.var_gamma_dn6 = assign33860_e49044_d_n6;
        locals.var_gamma_dn7 = assign33860_e49044_d_n7;
        locals.var_gamma_dn10 = assign33860_e49044_d_n10;
        locals.var_gamma_dn11 = assign33860_e49044_d_n11;
        locals.var_gamma_dn12 = assign33860_e49044_d_n12;
        locals.var_gamma_dn17 = assign33860_e49044_d_n17;

        let (assign33870_e49056, assign33870_e49056_d_n0, assign33870_e49056_d_n2, assign33870_e49056_d_n6, assign33870_e49056_d_n7, assign33870_e49056_d_n10, assign33870_e49056_d_n11, assign33870_e49056_d_n12, assign33870_e49056_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33870_e49049: f64 = (4.0 * locals.var_vgvt);
        let assign33870_e49051: f64 = (assign33870_e49049 * locals.var_sqrtkusail);
        let assign33870_e49052: f64 = (locals.var_kusai00 + assign33870_e49051);
        let assign33870_e49054: f64 = (assign33870_e49052 + locals.var_kusail);
        (assign33870_e49054, ((locals.var_kusai00_dn0 + (((4.0 * locals.var_vgvt_dn0) * locals.var_sqrtkusail) + (assign33870_e49049 * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0), ((locals.var_kusai00_dn2 + (((4.0 * locals.var_vgvt_dn2) * locals.var_sqrtkusail) + (assign33870_e49049 * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2), ((locals.var_kusai00_dn6 + (((4.0 * locals.var_vgvt_dn6) * locals.var_sqrtkusail) + (assign33870_e49049 * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6), ((locals.var_kusai00_dn7 + (((4.0 * locals.var_vgvt_dn7) * locals.var_sqrtkusail) + (assign33870_e49049 * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7), ((locals.var_kusai00_dn10 + (((4.0 * locals.var_vgvt_dn10) * locals.var_sqrtkusail) + (assign33870_e49049 * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10), ((locals.var_kusai00_dn11 + (((4.0 * locals.var_vgvt_dn11) * locals.var_sqrtkusail) + (assign33870_e49049 * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11), ((locals.var_kusai00_dn12 + (((4.0 * locals.var_vgvt_dn12) * locals.var_sqrtkusail) + (assign33870_e49049 * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12), ((locals.var_kusai00_dn17 + (((4.0 * locals.var_vgvt_dn17) * locals.var_sqrtkusail) + (assign33870_e49049 * locals.var_sqrtkusail_dn17))) + locals.var_kusail_dn17),)
    } else {
        (locals.var_t7w, locals.var_t7w_dn0, locals.var_t7w_dn2, locals.var_t7w_dn6, locals.var_t7w_dn7, locals.var_t7w_dn10, locals.var_t7w_dn11, locals.var_t7w_dn12, locals.var_t7w_dn17,)
    }
};
        locals.var_t7w = assign33870_e49056;
        locals.var_t7w_dn0 = assign33870_e49056_d_n0;
        locals.var_t7w_dn2 = assign33870_e49056_d_n2;
        locals.var_t7w_dn6 = assign33870_e49056_d_n6;
        locals.var_t7w_dn7 = assign33870_e49056_d_n7;
        locals.var_t7w_dn10 = assign33870_e49056_d_n10;
        locals.var_t7w_dn11 = assign33870_e49056_d_n11;
        locals.var_t7w_dn12 = assign33870_e49056_d_n12;
        locals.var_t7w_dn17 = assign33870_e49056_d_n17;

        let (assign33880_e49077, assign33880_e49077_d_n0, assign33880_e49077_d_n2, assign33880_e49077_d_n6, assign33880_e49077_d_n7, assign33880_e49077_d_n10, assign33880_e49077_d_n11, assign33880_e49077_d_n12, assign33880_e49077_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33880_e49060: f64 = (3.872983346207417 * locals.var_kusai00l);
        let assign33880_e49062: f64 = (assign33880_e49060 * locals.var_t7w);
        let assign33880_e49065: f64 = (6.0 * locals.var_t2__blk1112);
        let assign33880_e49068: f64 = (locals.var_gamma * locals.var_t2__blk1112);
        let assign33880_e49070: f64 = (assign33880_e49068 * locals.var_vgvt);
        let assign33880_e49072: f64 = (assign33880_e49070 * locals.var_t5__blk1115);
        let assign33880_e49073: f64 = (assign33880_e49072).sqrt();
        let assign33880_e49074: f64 = (assign33880_e49065 * assign33880_e49073);
        let assign33880_e49075: f64 = (assign33880_e49062 / assign33880_e49074);
        (assign33880_e49075, ((((((3.872983346207417 * locals.var_kusai00l_dn0) * locals.var_t7w) + (assign33880_e49060 * locals.var_t7w_dn0)) * assign33880_e49074) - (assign33880_e49062 * (((6.0 * locals.var_t2__blk1112_dn0) * assign33880_e49073) + (assign33880_e49065 * (((((((locals.var_gamma_dn0 * locals.var_t2__blk1112) + (locals.var_gamma * locals.var_t2__blk1112_dn0)) * locals.var_vgvt) + (assign33880_e49068 * locals.var_vgvt_dn0)) * locals.var_t5__blk1115) + (assign33880_e49070 * locals.var_t5__blk1115_dn0)) / (2.0 * assign33880_e49073)))))) / (assign33880_e49074 * assign33880_e49074)), ((((((3.872983346207417 * locals.var_kusai00l_dn2) * locals.var_t7w) + (assign33880_e49060 * locals.var_t7w_dn2)) * assign33880_e49074) - (assign33880_e49062 * (((6.0 * locals.var_t2__blk1112_dn2) * assign33880_e49073) + (assign33880_e49065 * (((((((locals.var_gamma_dn2 * locals.var_t2__blk1112) + (locals.var_gamma * locals.var_t2__blk1112_dn2)) * locals.var_vgvt) + (assign33880_e49068 * locals.var_vgvt_dn2)) * locals.var_t5__blk1115) + (assign33880_e49070 * locals.var_t5__blk1115_dn2)) / (2.0 * assign33880_e49073)))))) / (assign33880_e49074 * assign33880_e49074)), ((((((3.872983346207417 * locals.var_kusai00l_dn6) * locals.var_t7w) + (assign33880_e49060 * locals.var_t7w_dn6)) * assign33880_e49074) - (assign33880_e49062 * (((6.0 * locals.var_t2__blk1112_dn6) * assign33880_e49073) + (assign33880_e49065 * (((((((locals.var_gamma_dn6 * locals.var_t2__blk1112) + (locals.var_gamma * locals.var_t2__blk1112_dn6)) * locals.var_vgvt) + (assign33880_e49068 * locals.var_vgvt_dn6)) * locals.var_t5__blk1115) + (assign33880_e49070 * locals.var_t5__blk1115_dn6)) / (2.0 * assign33880_e49073)))))) / (assign33880_e49074 * assign33880_e49074)), ((((((3.872983346207417 * locals.var_kusai00l_dn7) * locals.var_t7w) + (assign33880_e49060 * locals.var_t7w_dn7)) * assign33880_e49074) - (assign33880_e49062 * (((6.0 * locals.var_t2__blk1112_dn7) * assign33880_e49073) + (assign33880_e49065 * (((((((locals.var_gamma_dn7 * locals.var_t2__blk1112) + (locals.var_gamma * locals.var_t2__blk1112_dn7)) * locals.var_vgvt) + (assign33880_e49068 * locals.var_vgvt_dn7)) * locals.var_t5__blk1115) + (assign33880_e49070 * locals.var_t5__blk1115_dn7)) / (2.0 * assign33880_e49073)))))) / (assign33880_e49074 * assign33880_e49074)), ((((((3.872983346207417 * locals.var_kusai00l_dn10) * locals.var_t7w) + (assign33880_e49060 * locals.var_t7w_dn10)) * assign33880_e49074) - (assign33880_e49062 * (((6.0 * locals.var_t2__blk1112_dn10) * assign33880_e49073) + (assign33880_e49065 * (((((((locals.var_gamma_dn10 * locals.var_t2__blk1112) + (locals.var_gamma * locals.var_t2__blk1112_dn10)) * locals.var_vgvt) + (assign33880_e49068 * locals.var_vgvt_dn10)) * locals.var_t5__blk1115) + (assign33880_e49070 * locals.var_t5__blk1115_dn10)) / (2.0 * assign33880_e49073)))))) / (assign33880_e49074 * assign33880_e49074)), ((((((3.872983346207417 * locals.var_kusai00l_dn11) * locals.var_t7w) + (assign33880_e49060 * locals.var_t7w_dn11)) * assign33880_e49074) - (assign33880_e49062 * (((6.0 * locals.var_t2__blk1112_dn11) * assign33880_e49073) + (assign33880_e49065 * (((((((locals.var_gamma_dn11 * locals.var_t2__blk1112) + (locals.var_gamma * locals.var_t2__blk1112_dn11)) * locals.var_vgvt) + (assign33880_e49068 * locals.var_vgvt_dn11)) * locals.var_t5__blk1115) + (assign33880_e49070 * locals.var_t5__blk1115_dn11)) / (2.0 * assign33880_e49073)))))) / (assign33880_e49074 * assign33880_e49074)), ((((((3.872983346207417 * locals.var_kusai00l_dn12) * locals.var_t7w) + (assign33880_e49060 * locals.var_t7w_dn12)) * assign33880_e49074) - (assign33880_e49062 * (((6.0 * locals.var_t2__blk1112_dn12) * assign33880_e49073) + (assign33880_e49065 * (((((((locals.var_gamma_dn12 * locals.var_t2__blk1112) + (locals.var_gamma * locals.var_t2__blk1112_dn12)) * locals.var_vgvt) + (assign33880_e49068 * locals.var_vgvt_dn12)) * locals.var_t5__blk1115) + (assign33880_e49070 * locals.var_t5__blk1115_dn12)) / (2.0 * assign33880_e49073)))))) / (assign33880_e49074 * assign33880_e49074)), ((((((3.872983346207417 * locals.var_kusai00l_dn17) * locals.var_t7w) + (assign33880_e49060 * locals.var_t7w_dn17)) * assign33880_e49074) - (assign33880_e49062 * (((6.0 * locals.var_t2__blk1112_dn17) * assign33880_e49073) + (assign33880_e49065 * (((((((locals.var_gamma_dn17 * locals.var_t2__blk1112) + (locals.var_gamma * locals.var_t2__blk1112_dn17)) * locals.var_vgvt) + (assign33880_e49068 * locals.var_vgvt_dn17)) * locals.var_t5__blk1115) + (assign33880_e49070 * locals.var_t5__blk1115_dn17)) / (2.0 * assign33880_e49073)))))) / (assign33880_e49074 * assign33880_e49074)),)
    } else {
        (locals.var_crl_f, locals.var_crl_f_dn0, locals.var_crl_f_dn2, locals.var_crl_f_dn6, locals.var_crl_f_dn7, locals.var_crl_f_dn10, locals.var_crl_f_dn11, locals.var_crl_f_dn12, locals.var_crl_f_dn17,)
    }
};
        locals.var_crl_f = assign33880_e49077;
        locals.var_crl_f_dn0 = assign33880_e49077_d_n0;
        locals.var_crl_f_dn2 = assign33880_e49077_d_n2;
        locals.var_crl_f_dn6 = assign33880_e49077_d_n6;
        locals.var_crl_f_dn7 = assign33880_e49077_d_n7;
        locals.var_crl_f_dn10 = assign33880_e49077_d_n10;
        locals.var_crl_f_dn11 = assign33880_e49077_d_n11;
        locals.var_crl_f_dn12 = assign33880_e49077_d_n12;
        locals.var_crl_f_dn17 = assign33880_e49077_d_n17;

        let assign33890_e49080: f64 = (locals.var_ids + locals.var_idsibpc);
        locals.var_ids = assign33890_e49080;
        locals.var_ids_dn0 = (locals.var_ids_dn0 + locals.var_idsibpc_dn0);
        locals.var_ids_dn2 = (locals.var_ids_dn2 + locals.var_idsibpc_dn2);
        locals.var_ids_dn6 = (locals.var_ids_dn6 + locals.var_idsibpc_dn6);
        locals.var_ids_dn7 = (locals.var_ids_dn7 + locals.var_idsibpc_dn7);
        locals.var_ids_dn10 = (locals.var_ids_dn10 + locals.var_idsibpc_dn10);
        locals.var_ids_dn11 = (locals.var_ids_dn11 + locals.var_idsibpc_dn11);
        locals.var_ids_dn12 = (locals.var_ids_dn12 + locals.var_idsibpc_dn12);
        locals.var_ids_dn17 = (locals.var_ids_dn17 + locals.var_idsibpc_dn17);

        let assign33900_e49083: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1124 = assign33900_e49083;

        let (assign33910_e49089,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33910_e49087: f64 = (locals.var_cbtp + locals.var_cbtn);
        (assign33910_e49087,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign33910_e49089;

        let (assign33920_e49099,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign33920_e49096: f64 = (p.p168 * locals.var_lgleff);
        let assign33920_e49097: f64 = (locals.var_cgbe - assign33920_e49096);
        (assign33920_e49097,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign33920_e49099;

        let (assign33930_e49108, assign33930_e49108_d_n0, assign33930_e49108_d_n2, assign33930_e49108_d_n6, assign33930_e49108_d_n7, assign33930_e49108_d_n10, assign33930_e49108_d_n11, assign33930_e49108_d_n12, assign33930_e49108_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33930_e49102: f64 = (-locals.var_cgbe);
        let assign33930_e49105: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign33930_e49106: f64 = (assign33930_e49102 * assign33930_e49105);
        (assign33930_e49106, (assign33930_e49102 * (-locals.var_vbsp_dn0)), (assign33930_e49102 * (-locals.var_vbsp_dn2)), (assign33930_e49102 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33930_e49102 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33930_e49102 * (-locals.var_vbsp_dn10)), (assign33930_e49102 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33930_e49102 * (-locals.var_vbsp_dn12)), (assign33930_e49102 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign33930_e49108;
        locals.var_qgob_dn0 = assign33930_e49108_d_n0;
        locals.var_qgob_dn2 = assign33930_e49108_d_n2;
        locals.var_qgob_dn6 = assign33930_e49108_d_n6;
        locals.var_qgob_dn7 = assign33930_e49108_d_n7;
        locals.var_qgob_dn10 = assign33930_e49108_d_n10;
        locals.var_qgob_dn11 = assign33930_e49108_d_n11;
        locals.var_qgob_dn12 = assign33930_e49108_d_n12;
        locals.var_qgob_dn17 = assign33930_e49108_d_n17;

        let (assign33940_e49118,) = {
    if (locals.var_guard1124 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cfu,)
    }
};
        locals.var_cfu = assign33940_e49118;

        let (assign33950_e49128,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33950_e49122: f64 = (locals.var_cfu * p.p9);
        let assign33950_e49125: f64 = (locals.var_wgate + locals.var_uc_pdbcp);
        let assign33950_e49126: f64 = (assign33950_e49122 * assign33950_e49125);
        (assign33950_e49126,)
    } else {
        (locals.var_cfd,)
    }
};
        locals.var_cfd = assign33950_e49128;

        let (assign33960_e49138,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33960_e49132: f64 = (locals.var_cfu * p.p9);
        let assign33960_e49135: f64 = (locals.var_wgate + locals.var_uc_psbcp);
        let assign33960_e49136: f64 = (assign33960_e49132 * assign33960_e49135);
        (assign33960_e49136,)
    } else {
        (locals.var_cfs,)
    }
};
        locals.var_cfs = assign33960_e49138;

        let (assign33970_e49146, assign33970_e49146_d_n0, assign33970_e49146_d_n2, assign33970_e49146_d_n6, assign33970_e49146_d_n7, assign33970_e49146_d_n10, assign33970_e49146_d_n11, assign33970_e49146_d_n12, assign33970_e49146_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33970_e49143: f64 = (locals.var_vgs - locals.var_vds);
        let assign33970_e49144: f64 = (locals.var_cfd * assign33970_e49143);
        (assign33970_e49144, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17,)
    }
};
        locals.var_qfd = assign33970_e49146;
        locals.var_qfd_dn0 = assign33970_e49146_d_n0;
        locals.var_qfd_dn2 = assign33970_e49146_d_n2;
        locals.var_qfd_dn6 = assign33970_e49146_d_n6;
        locals.var_qfd_dn7 = assign33970_e49146_d_n7;
        locals.var_qfd_dn10 = assign33970_e49146_d_n10;
        locals.var_qfd_dn11 = assign33970_e49146_d_n11;
        locals.var_qfd_dn12 = assign33970_e49146_d_n12;
        locals.var_qfd_dn17 = assign33970_e49146_d_n17;

        let (assign33980_e49152, assign33980_e49152_d_n6, assign33980_e49152_d_n7, assign33980_e49152_d_n11,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33980_e49150: f64 = (locals.var_cfs * locals.var_vgs);
        (assign33980_e49150, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11),)
    } else {
        (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11,)
    }
};
        locals.var_qfs = assign33980_e49152;
        locals.var_qfs_dn6 = assign33980_e49152_d_n6;
        locals.var_qfs_dn7 = assign33980_e49152_d_n7;
        locals.var_qfs_dn11 = assign33980_e49152_d_n11;

        let (assign33990_e49164, assign33990_e49164_d_n0, assign33990_e49164_d_n2, assign33990_e49164_d_n6, assign33990_e49164_d_n7, assign33990_e49164_d_n10, assign33990_e49164_d_n11, assign33990_e49164_d_n12, assign33990_e49164_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33990_e49156: f64 = (locals.var_cfu * p.p19);
        let assign33990_e49158: f64 = (assign33990_e49156 * p.p9);
        let assign33990_e49161: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign33990_e49162: f64 = (assign33990_e49158 * assign33990_e49161);
        (assign33990_e49162, (assign33990_e49158 * (-locals.var_vbsp_dn0)), (assign33990_e49158 * (-locals.var_vbsp_dn2)), (assign33990_e49158 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33990_e49158 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33990_e49158 * (-locals.var_vbsp_dn10)), (assign33990_e49158 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33990_e49158 * (-locals.var_vbsp_dn12)), (assign33990_e49158 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qfbc, locals.var_qfbc_dn0, locals.var_qfbc_dn2, locals.var_qfbc_dn6, locals.var_qfbc_dn7, locals.var_qfbc_dn10, locals.var_qfbc_dn11, locals.var_qfbc_dn12, locals.var_qfbc_dn17,)
    }
};
        locals.var_qfbc = assign33990_e49164;
        locals.var_qfbc_dn0 = assign33990_e49164_d_n0;
        locals.var_qfbc_dn2 = assign33990_e49164_d_n2;
        locals.var_qfbc_dn6 = assign33990_e49164_d_n6;
        locals.var_qfbc_dn7 = assign33990_e49164_d_n7;
        locals.var_qfbc_dn10 = assign33990_e49164_d_n10;
        locals.var_qfbc_dn11 = assign33990_e49164_d_n11;
        locals.var_qfbc_dn12 = assign33990_e49164_d_n12;
        locals.var_qfbc_dn17 = assign33990_e49164_d_n17;

    }

    pub(super) fn stamp_transient_block_119(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34000_e49170, assign34000_e49170_d_n0, assign34000_e49170_d_n2, assign34000_e49170_d_n6, assign34000_e49170_d_n7, assign34000_e49170_d_n10, assign34000_e49170_d_n11, assign34000_e49170_d_n12, assign34000_e49170_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign34000_e49168: f64 = (locals.var_qgod + locals.var_qfd);
        (assign34000_e49168, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign34000_e49170;
        locals.var_qgod_dn0 = assign34000_e49170_d_n0;
        locals.var_qgod_dn2 = assign34000_e49170_d_n2;
        locals.var_qgod_dn6 = assign34000_e49170_d_n6;
        locals.var_qgod_dn7 = assign34000_e49170_d_n7;
        locals.var_qgod_dn10 = assign34000_e49170_d_n10;
        locals.var_qgod_dn11 = assign34000_e49170_d_n11;
        locals.var_qgod_dn12 = assign34000_e49170_d_n12;
        locals.var_qgod_dn17 = assign34000_e49170_d_n17;

        let (assign34010_e49176, assign34010_e49176_d_n0, assign34010_e49176_d_n2, assign34010_e49176_d_n6, assign34010_e49176_d_n7, assign34010_e49176_d_n10, assign34010_e49176_d_n11, assign34010_e49176_d_n12, assign34010_e49176_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign34010_e49174: f64 = (locals.var_qgos + locals.var_qfs);
        (assign34010_e49174, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17,)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign34010_e49176;
        locals.var_qgos_dn0 = assign34010_e49176_d_n0;
        locals.var_qgos_dn2 = assign34010_e49176_d_n2;
        locals.var_qgos_dn6 = assign34010_e49176_d_n6;
        locals.var_qgos_dn7 = assign34010_e49176_d_n7;
        locals.var_qgos_dn10 = assign34010_e49176_d_n10;
        locals.var_qgos_dn11 = assign34010_e49176_d_n11;
        locals.var_qgos_dn12 = assign34010_e49176_d_n12;
        locals.var_qgos_dn17 = assign34010_e49176_d_n17;

        let (assign34020_e49182, assign34020_e49182_d_n0, assign34020_e49182_d_n2, assign34020_e49182_d_n6, assign34020_e49182_d_n7, assign34020_e49182_d_n10, assign34020_e49182_d_n11, assign34020_e49182_d_n12, assign34020_e49182_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign34020_e49180: f64 = (locals.var_qgob + locals.var_qfbc);
        (assign34020_e49180, (locals.var_qgob_dn0 + locals.var_qfbc_dn0), (locals.var_qgob_dn2 + locals.var_qfbc_dn2), (locals.var_qgob_dn6 + locals.var_qfbc_dn6), (locals.var_qgob_dn7 + locals.var_qfbc_dn7), (locals.var_qgob_dn10 + locals.var_qfbc_dn10), (locals.var_qgob_dn11 + locals.var_qfbc_dn11), (locals.var_qgob_dn12 + locals.var_qfbc_dn12), (locals.var_qgob_dn17 + locals.var_qfbc_dn17),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34020_e49182;
        locals.var_qgob_dn0 = assign34020_e49182_d_n0;
        locals.var_qgob_dn2 = assign34020_e49182_d_n2;
        locals.var_qgob_dn6 = assign34020_e49182_d_n6;
        locals.var_qgob_dn7 = assign34020_e49182_d_n7;
        locals.var_qgob_dn10 = assign34020_e49182_d_n10;
        locals.var_qgob_dn11 = assign34020_e49182_d_n11;
        locals.var_qgob_dn12 = assign34020_e49182_d_n12;
        locals.var_qgob_dn17 = assign34020_e49182_d_n17;

        let (assign34030_e49192,) = {
    if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign34030_e49188: f64 = (-p.p168);
        let assign34030_e49190: f64 = (assign34030_e49188 * locals.var_lgleff);
        (assign34030_e49190,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign34030_e49192;

        let (assign34040_e49204, assign34040_e49204_d_n0, assign34040_e49204_d_n2, assign34040_e49204_d_n6, assign34040_e49204_d_n7, assign34040_e49204_d_n10, assign34040_e49204_d_n11, assign34040_e49204_d_n12, assign34040_e49204_d_n17,) = {
    if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign34040_e49198: f64 = (-locals.var_cgbe);
        let assign34040_e49201: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign34040_e49202: f64 = (assign34040_e49198 * assign34040_e49201);
        (assign34040_e49202, (assign34040_e49198 * (-locals.var_vbsp_dn0)), (assign34040_e49198 * (-locals.var_vbsp_dn2)), (assign34040_e49198 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34040_e49198 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34040_e49198 * (-locals.var_vbsp_dn10)), (assign34040_e49198 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34040_e49198 * (-locals.var_vbsp_dn12)), (assign34040_e49198 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34040_e49204;
        locals.var_qgob_dn0 = assign34040_e49204_d_n0;
        locals.var_qgob_dn2 = assign34040_e49204_d_n2;
        locals.var_qgob_dn6 = assign34040_e49204_d_n6;
        locals.var_qgob_dn7 = assign34040_e49204_d_n7;
        locals.var_qgob_dn10 = assign34040_e49204_d_n10;
        locals.var_qgob_dn11 = assign34040_e49204_d_n11;
        locals.var_qgob_dn12 = assign34040_e49204_d_n12;
        locals.var_qgob_dn17 = assign34040_e49204_d_n17;

        let (assign34050_e49212,) = {
    if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given == 0.0)) {
        (0.0,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign34050_e49212;

        let (assign34060_e49220, assign34060_e49220_d_n0, assign34060_e49220_d_n2, assign34060_e49220_d_n6, assign34060_e49220_d_n7, assign34060_e49220_d_n10, assign34060_e49220_d_n11, assign34060_e49220_d_n12, assign34060_e49220_d_n17,) = {
    if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34060_e49220;
        locals.var_qgob_dn0 = assign34060_e49220_d_n0;
        locals.var_qgob_dn2 = assign34060_e49220_d_n2;
        locals.var_qgob_dn6 = assign34060_e49220_d_n6;
        locals.var_qgob_dn7 = assign34060_e49220_d_n7;
        locals.var_qgob_dn10 = assign34060_e49220_d_n10;
        locals.var_qgob_dn11 = assign34060_e49220_d_n11;
        locals.var_qgob_dn12 = assign34060_e49220_d_n12;
        locals.var_qgob_dn17 = assign34060_e49220_d_n17;

        let (assign34070_e49235,) = {
    if (locals.var_guard1124 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cf,)
    }
};
        locals.var_cf = assign34070_e49235;

        let (assign34080_e49240,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_cf,)
    } else {
        (locals.var_cfd,)
    }
};
        locals.var_cfd = assign34080_e49240;

        let (assign34090_e49245,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_cf,)
    } else {
        (locals.var_cfs,)
    }
};
        locals.var_cfs = assign34090_e49245;

        let (assign34100_e49254, assign34100_e49254_d_n0, assign34100_e49254_d_n2, assign34100_e49254_d_n6, assign34100_e49254_d_n7, assign34100_e49254_d_n10, assign34100_e49254_d_n11, assign34100_e49254_d_n12, assign34100_e49254_d_n17,) = {
    if (locals.var_guard1124 == 0.0) {
        let assign34100_e49251: f64 = (locals.var_vgs - locals.var_vds);
        let assign34100_e49252: f64 = (locals.var_cfd * assign34100_e49251);
        (assign34100_e49252, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17,)
    }
};
        locals.var_qfd = assign34100_e49254;
        locals.var_qfd_dn0 = assign34100_e49254_d_n0;
        locals.var_qfd_dn2 = assign34100_e49254_d_n2;
        locals.var_qfd_dn6 = assign34100_e49254_d_n6;
        locals.var_qfd_dn7 = assign34100_e49254_d_n7;
        locals.var_qfd_dn10 = assign34100_e49254_d_n10;
        locals.var_qfd_dn11 = assign34100_e49254_d_n11;
        locals.var_qfd_dn12 = assign34100_e49254_d_n12;
        locals.var_qfd_dn17 = assign34100_e49254_d_n17;

        let (assign34110_e49261, assign34110_e49261_d_n6, assign34110_e49261_d_n7, assign34110_e49261_d_n11,) = {
    if (locals.var_guard1124 == 0.0) {
        let assign34110_e49259: f64 = (locals.var_cfs * locals.var_vgs);
        (assign34110_e49259, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11),)
    } else {
        (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11,)
    }
};
        locals.var_qfs = assign34110_e49261;
        locals.var_qfs_dn6 = assign34110_e49261_d_n6;
        locals.var_qfs_dn7 = assign34110_e49261_d_n7;
        locals.var_qfs_dn11 = assign34110_e49261_d_n11;

        let (assign34120_e49268, assign34120_e49268_d_n0, assign34120_e49268_d_n2, assign34120_e49268_d_n6, assign34120_e49268_d_n7, assign34120_e49268_d_n10, assign34120_e49268_d_n11, assign34120_e49268_d_n12, assign34120_e49268_d_n17,) = {
    if (locals.var_guard1124 == 0.0) {
        let assign34120_e49266: f64 = (locals.var_qgod + locals.var_qfd);
        (assign34120_e49266, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign34120_e49268;
        locals.var_qgod_dn0 = assign34120_e49268_d_n0;
        locals.var_qgod_dn2 = assign34120_e49268_d_n2;
        locals.var_qgod_dn6 = assign34120_e49268_d_n6;
        locals.var_qgod_dn7 = assign34120_e49268_d_n7;
        locals.var_qgod_dn10 = assign34120_e49268_d_n10;
        locals.var_qgod_dn11 = assign34120_e49268_d_n11;
        locals.var_qgod_dn12 = assign34120_e49268_d_n12;
        locals.var_qgod_dn17 = assign34120_e49268_d_n17;

        let (assign34130_e49275, assign34130_e49275_d_n0, assign34130_e49275_d_n2, assign34130_e49275_d_n6, assign34130_e49275_d_n7, assign34130_e49275_d_n10, assign34130_e49275_d_n11, assign34130_e49275_d_n12, assign34130_e49275_d_n17,) = {
    if (locals.var_guard1124 == 0.0) {
        let assign34130_e49273: f64 = (locals.var_qgos + locals.var_qfs);
        (assign34130_e49273, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17,)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign34130_e49275;
        locals.var_qgos_dn0 = assign34130_e49275_d_n0;
        locals.var_qgos_dn2 = assign34130_e49275_d_n2;
        locals.var_qgos_dn6 = assign34130_e49275_d_n6;
        locals.var_qgos_dn7 = assign34130_e49275_d_n7;
        locals.var_qgos_dn10 = assign34130_e49275_d_n10;
        locals.var_qgos_dn11 = assign34130_e49275_d_n11;
        locals.var_qgos_dn12 = assign34130_e49275_d_n12;
        locals.var_qgos_dn17 = assign34130_e49275_d_n17;

        let assign34140_e49278: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign34140_e49278;
        locals.var_idse_dn0 = (locals.var_mfactor * locals.var_ids_dn0);
        locals.var_idse_dn2 = (locals.var_mfactor * locals.var_ids_dn2);
        locals.var_idse_dn6 = (locals.var_mfactor * locals.var_ids_dn6);
        locals.var_idse_dn7 = (locals.var_mfactor * locals.var_ids_dn7);
        locals.var_idse_dn10 = (locals.var_mfactor * locals.var_ids_dn10);
        locals.var_idse_dn11 = (locals.var_mfactor * locals.var_ids_dn11);
        locals.var_idse_dn12 = (locals.var_mfactor * locals.var_ids_dn12);
        locals.var_idse_dn17 = (locals.var_mfactor * locals.var_ids_dn17);

        let (assign34150_e49282, assign34150_e49282_d_n0, assign34150_e49282_d_n2, assign34150_e49282_d_n6, assign34150_e49282_d_n7, assign34150_e49282_d_n10, assign34150_e49282_d_n11, assign34150_e49282_d_n12, assign34150_e49282_d_n13, assign34150_e49282_d_n15, assign34150_e49282_d_n16, assign34150_e49282_d_n17, assign34150_e49282_d_n18,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34150_e49282;
        locals.var_qde_dn0 = assign34150_e49282_d_n0;
        locals.var_qde_dn2 = assign34150_e49282_d_n2;
        locals.var_qde_dn6 = assign34150_e49282_d_n6;
        locals.var_qde_dn7 = assign34150_e49282_d_n7;
        locals.var_qde_dn10 = assign34150_e49282_d_n10;
        locals.var_qde_dn11 = assign34150_e49282_d_n11;
        locals.var_qde_dn12 = assign34150_e49282_d_n12;
        locals.var_qde_dn13 = assign34150_e49282_d_n13;
        locals.var_qde_dn15 = assign34150_e49282_d_n15;
        locals.var_qde_dn16 = assign34150_e49282_d_n16;
        locals.var_qde_dn17 = assign34150_e49282_d_n17;
        locals.var_qde_dn18 = assign34150_e49282_d_n18;

        let (assign34160_e49286, assign34160_e49286_d_n0, assign34160_e49286_d_n2, assign34160_e49286_d_n6, assign34160_e49286_d_n7, assign34160_e49286_d_n10, assign34160_e49286_d_n11, assign34160_e49286_d_n12, assign34160_e49286_d_n13, assign34160_e49286_d_n15, assign34160_e49286_d_n16, assign34160_e49286_d_n17, assign34160_e49286_d_n18,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34160_e49286;
        locals.var_qge_dn0 = assign34160_e49286_d_n0;
        locals.var_qge_dn2 = assign34160_e49286_d_n2;
        locals.var_qge_dn6 = assign34160_e49286_d_n6;
        locals.var_qge_dn7 = assign34160_e49286_d_n7;
        locals.var_qge_dn10 = assign34160_e49286_d_n10;
        locals.var_qge_dn11 = assign34160_e49286_d_n11;
        locals.var_qge_dn12 = assign34160_e49286_d_n12;
        locals.var_qge_dn13 = assign34160_e49286_d_n13;
        locals.var_qge_dn15 = assign34160_e49286_d_n15;
        locals.var_qge_dn16 = assign34160_e49286_d_n16;
        locals.var_qge_dn17 = assign34160_e49286_d_n17;
        locals.var_qge_dn18 = assign34160_e49286_d_n18;

        let assign34170_e49289: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1125 = assign34170_e49289;

        let (assign34180_e49295, assign34180_e49295_d_n0, assign34180_e49295_d_n2, assign34180_e49295_d_n6, assign34180_e49295_d_n7, assign34180_e49295_d_n10, assign34180_e49295_d_n11, assign34180_e49295_d_n12, assign34180_e49295_d_n13, assign34180_e49295_d_n15, assign34180_e49295_d_n16, assign34180_e49295_d_n17, assign34180_e49295_d_n18,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1125 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34180_e49295;
        locals.var_qse_dn0 = assign34180_e49295_d_n0;
        locals.var_qse_dn2 = assign34180_e49295_d_n2;
        locals.var_qse_dn6 = assign34180_e49295_d_n6;
        locals.var_qse_dn7 = assign34180_e49295_d_n7;
        locals.var_qse_dn10 = assign34180_e49295_d_n10;
        locals.var_qse_dn11 = assign34180_e49295_d_n11;
        locals.var_qse_dn12 = assign34180_e49295_d_n12;
        locals.var_qse_dn13 = assign34180_e49295_d_n13;
        locals.var_qse_dn15 = assign34180_e49295_d_n15;
        locals.var_qse_dn16 = assign34180_e49295_d_n16;
        locals.var_qse_dn17 = assign34180_e49295_d_n17;
        locals.var_qse_dn18 = assign34180_e49295_d_n18;

        let (assign34190_e49301, assign34190_e49301_d_n0, assign34190_e49301_d_n2, assign34190_e49301_d_n6, assign34190_e49301_d_n7, assign34190_e49301_d_n10, assign34190_e49301_d_n11, assign34190_e49301_d_n12, assign34190_e49301_d_n17,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1125 != 0.0)) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
    }
};
        locals.var_xd = assign34190_e49301;
        locals.var_xd_dn0 = assign34190_e49301_d_n0;
        locals.var_xd_dn2 = assign34190_e49301_d_n2;
        locals.var_xd_dn6 = assign34190_e49301_d_n6;
        locals.var_xd_dn7 = assign34190_e49301_d_n7;
        locals.var_xd_dn10 = assign34190_e49301_d_n10;
        locals.var_xd_dn11 = assign34190_e49301_d_n11;
        locals.var_xd_dn12 = assign34190_e49301_d_n12;
        locals.var_xd_dn17 = assign34190_e49301_d_n17;

        let (assign34220_e49324, assign34220_e49324_d_n0, assign34220_e49324_d_n2, assign34220_e49324_d_n6, assign34220_e49324_d_n7, assign34220_e49324_d_n10, assign34220_e49324_d_n11, assign34220_e49324_d_n12, assign34220_e49324_d_n13, assign34220_e49324_d_n15, assign34220_e49324_d_n16, assign34220_e49324_d_n17, assign34220_e49324_d_n18,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1125 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign34220_e49324;
        locals.var_qbe_dn0 = assign34220_e49324_d_n0;
        locals.var_qbe_dn2 = assign34220_e49324_d_n2;
        locals.var_qbe_dn6 = assign34220_e49324_d_n6;
        locals.var_qbe_dn7 = assign34220_e49324_d_n7;
        locals.var_qbe_dn10 = assign34220_e49324_d_n10;
        locals.var_qbe_dn11 = assign34220_e49324_d_n11;
        locals.var_qbe_dn12 = assign34220_e49324_d_n12;
        locals.var_qbe_dn13 = assign34220_e49324_d_n13;
        locals.var_qbe_dn15 = assign34220_e49324_d_n15;
        locals.var_qbe_dn16 = assign34220_e49324_d_n16;
        locals.var_qbe_dn17 = assign34220_e49324_d_n17;
        locals.var_qbe_dn18 = assign34220_e49324_d_n18;

        let assign34260_e49360: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1126 = assign34260_e49360;

        let (assign34270_e49372, assign34270_e49372_d_n0, assign34270_e49372_d_n2, assign34270_e49372_d_n6, assign34270_e49372_d_n7, assign34270_e49372_d_n10, assign34270_e49372_d_n11, assign34270_e49372_d_n12, assign34270_e49372_d_n13, assign34270_e49372_d_n15, assign34270_e49372_d_n16, assign34270_e49372_d_n17, assign34270_e49372_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
        let assign34270_e49367: f64 = (-locals.var_qb);
        let assign34270_e49369: f64 = (assign34270_e49367 - locals.var_qi);
        let assign34270_e49370: f64 = (locals.var_mfactor * assign34270_e49369);
        (assign34270_e49370, (locals.var_mfactor * ((-locals.var_qb_dn0) - locals.var_qi_dn0)), (locals.var_mfactor * ((-locals.var_qb_dn2) - locals.var_qi_dn2)), (locals.var_mfactor * ((-locals.var_qb_dn6) - locals.var_qi_dn6)), (locals.var_mfactor * ((-locals.var_qb_dn7) - locals.var_qi_dn7)), (locals.var_mfactor * ((-locals.var_qb_dn10) - locals.var_qi_dn10)), (locals.var_mfactor * ((-locals.var_qb_dn11) - locals.var_qi_dn11)), (locals.var_mfactor * ((-locals.var_qb_dn12) - locals.var_qi_dn12)), (locals.var_mfactor * (-locals.var_qb_dn13)), (locals.var_mfactor * (-locals.var_qb_dn15)), (locals.var_mfactor * (-locals.var_qb_dn16)), (locals.var_mfactor * ((-locals.var_qb_dn17) - locals.var_qi_dn17)), (locals.var_mfactor * (-locals.var_qb_dn18)),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34270_e49372;
        locals.var_qge_dn0 = assign34270_e49372_d_n0;
        locals.var_qge_dn2 = assign34270_e49372_d_n2;
        locals.var_qge_dn6 = assign34270_e49372_d_n6;
        locals.var_qge_dn7 = assign34270_e49372_d_n7;
        locals.var_qge_dn10 = assign34270_e49372_d_n10;
        locals.var_qge_dn11 = assign34270_e49372_d_n11;
        locals.var_qge_dn12 = assign34270_e49372_d_n12;
        locals.var_qge_dn13 = assign34270_e49372_d_n13;
        locals.var_qge_dn15 = assign34270_e49372_d_n15;
        locals.var_qge_dn16 = assign34270_e49372_d_n16;
        locals.var_qge_dn17 = assign34270_e49372_d_n17;
        locals.var_qge_dn18 = assign34270_e49372_d_n18;

        let (assign34280_e49381, assign34280_e49381_d_n0, assign34280_e49381_d_n2, assign34280_e49381_d_n6, assign34280_e49381_d_n7, assign34280_e49381_d_n10, assign34280_e49381_d_n11, assign34280_e49381_d_n12, assign34280_e49381_d_n13, assign34280_e49381_d_n15, assign34280_e49381_d_n16, assign34280_e49381_d_n17, assign34280_e49381_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
        let assign34280_e49379: f64 = (locals.var_mfactor * locals.var_qd);
        (assign34280_e49379, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn12), (locals.var_mfactor * locals.var_qd_dn13), (locals.var_mfactor * locals.var_qd_dn15), (locals.var_mfactor * locals.var_qd_dn16), (locals.var_mfactor * locals.var_qd_dn17), (locals.var_mfactor * locals.var_qd_dn18),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34280_e49381;
        locals.var_qde_dn0 = assign34280_e49381_d_n0;
        locals.var_qde_dn2 = assign34280_e49381_d_n2;
        locals.var_qde_dn6 = assign34280_e49381_d_n6;
        locals.var_qde_dn7 = assign34280_e49381_d_n7;
        locals.var_qde_dn10 = assign34280_e49381_d_n10;
        locals.var_qde_dn11 = assign34280_e49381_d_n11;
        locals.var_qde_dn12 = assign34280_e49381_d_n12;
        locals.var_qde_dn13 = assign34280_e49381_d_n13;
        locals.var_qde_dn15 = assign34280_e49381_d_n15;
        locals.var_qde_dn16 = assign34280_e49381_d_n16;
        locals.var_qde_dn17 = assign34280_e49381_d_n17;
        locals.var_qde_dn18 = assign34280_e49381_d_n18;

        let (assign34290_e49392, assign34290_e49392_d_n0, assign34290_e49392_d_n2, assign34290_e49392_d_n6, assign34290_e49392_d_n7, assign34290_e49392_d_n10, assign34290_e49392_d_n11, assign34290_e49392_d_n12, assign34290_e49392_d_n13, assign34290_e49392_d_n15, assign34290_e49392_d_n16, assign34290_e49392_d_n17, assign34290_e49392_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
        let assign34290_e49389: f64 = (locals.var_qi - locals.var_qd);
        let assign34290_e49390: f64 = (locals.var_mfactor * assign34290_e49389);
        (assign34290_e49390, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn12 - locals.var_qd_dn12)), (locals.var_mfactor * (-locals.var_qd_dn13)), (locals.var_mfactor * (-locals.var_qd_dn15)), (locals.var_mfactor * (-locals.var_qd_dn16)), (locals.var_mfactor * (locals.var_qi_dn17 - locals.var_qd_dn17)), (locals.var_mfactor * (-locals.var_qd_dn18)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34290_e49392;
        locals.var_qse_dn0 = assign34290_e49392_d_n0;
        locals.var_qse_dn2 = assign34290_e49392_d_n2;
        locals.var_qse_dn6 = assign34290_e49392_d_n6;
        locals.var_qse_dn7 = assign34290_e49392_d_n7;
        locals.var_qse_dn10 = assign34290_e49392_d_n10;
        locals.var_qse_dn11 = assign34290_e49392_d_n11;
        locals.var_qse_dn12 = assign34290_e49392_d_n12;
        locals.var_qse_dn13 = assign34290_e49392_d_n13;
        locals.var_qse_dn15 = assign34290_e49392_d_n15;
        locals.var_qse_dn16 = assign34290_e49392_d_n16;
        locals.var_qse_dn17 = assign34290_e49392_d_n17;
        locals.var_qse_dn18 = assign34290_e49392_d_n18;

        let (assign34300_e49409, assign34300_e49409_d_n0, assign34300_e49409_d_n2, assign34300_e49409_d_n6, assign34300_e49409_d_n7, assign34300_e49409_d_n10, assign34300_e49409_d_n11, assign34300_e49409_d_n12, assign34300_e49409_d_n13, assign34300_e49409_d_n15, assign34300_e49409_d_n16, assign34300_e49409_d_n17, assign34300_e49409_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
        let assign34300_e49400: f64 = (-locals.var_qsub);
        let assign34300_e49402: f64 = (assign34300_e49400 - locals.var_qi);
        let assign34300_e49404: f64 = (assign34300_e49402 - locals.var_qs_fb);
        let assign34300_e49406: f64 = (assign34300_e49404 - locals.var_qd_fb);
        let assign34300_e49407: f64 = (locals.var_mfactor * assign34300_e49406);
        (assign34300_e49407, (locals.var_mfactor * ((((-locals.var_qsub_dn0) - locals.var_qi_dn0) - locals.var_qs_fb_dn0) - locals.var_qd_fb_dn0)), (locals.var_mfactor * ((((-locals.var_qsub_dn2) - locals.var_qi_dn2) - locals.var_qs_fb_dn2) - locals.var_qd_fb_dn2)), (locals.var_mfactor * ((((-locals.var_qsub_dn6) - locals.var_qi_dn6) - locals.var_qs_fb_dn6) - locals.var_qd_fb_dn6)), (locals.var_mfactor * ((((-locals.var_qsub_dn7) - locals.var_qi_dn7) - locals.var_qs_fb_dn7) - locals.var_qd_fb_dn7)), (locals.var_mfactor * ((((-locals.var_qsub_dn10) - locals.var_qi_dn10) - locals.var_qs_fb_dn10) - locals.var_qd_fb_dn10)), (locals.var_mfactor * ((((-locals.var_qsub_dn11) - locals.var_qi_dn11) - locals.var_qs_fb_dn11) - locals.var_qd_fb_dn11)), (locals.var_mfactor * ((((-locals.var_qsub_dn12) - locals.var_qi_dn12) - locals.var_qs_fb_dn12) - locals.var_qd_fb_dn12)), (locals.var_mfactor * ((-locals.var_qs_fb_dn13) - locals.var_qd_fb_dn13)), (locals.var_mfactor * ((-locals.var_qs_fb_dn15) - locals.var_qd_fb_dn15)), (locals.var_mfactor * ((-locals.var_qs_fb_dn16) - locals.var_qd_fb_dn16)), (locals.var_mfactor * ((((-locals.var_qsub_dn17) - locals.var_qi_dn17) - locals.var_qs_fb_dn17) - locals.var_qd_fb_dn17)), (locals.var_mfactor * ((-locals.var_qs_fb_dn18) - locals.var_qd_fb_dn18)),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34300_e49409;
        locals.var_qge_dn0 = assign34300_e49409_d_n0;
        locals.var_qge_dn2 = assign34300_e49409_d_n2;
        locals.var_qge_dn6 = assign34300_e49409_d_n6;
        locals.var_qge_dn7 = assign34300_e49409_d_n7;
        locals.var_qge_dn10 = assign34300_e49409_d_n10;
        locals.var_qge_dn11 = assign34300_e49409_d_n11;
        locals.var_qge_dn12 = assign34300_e49409_d_n12;
        locals.var_qge_dn13 = assign34300_e49409_d_n13;
        locals.var_qge_dn15 = assign34300_e49409_d_n15;
        locals.var_qge_dn16 = assign34300_e49409_d_n16;
        locals.var_qge_dn17 = assign34300_e49409_d_n17;
        locals.var_qge_dn18 = assign34300_e49409_d_n18;

        let (assign34310_e49421, assign34310_e49421_d_n0, assign34310_e49421_d_n2, assign34310_e49421_d_n6, assign34310_e49421_d_n7, assign34310_e49421_d_n10, assign34310_e49421_d_n11, assign34310_e49421_d_n12, assign34310_e49421_d_n13, assign34310_e49421_d_n15, assign34310_e49421_d_n16, assign34310_e49421_d_n17, assign34310_e49421_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
        let assign34310_e49418: f64 = (locals.var_qd + locals.var_qd_fb);
        let assign34310_e49419: f64 = (locals.var_mfactor * assign34310_e49418);
        (assign34310_e49419, (locals.var_mfactor * (locals.var_qd_dn0 + locals.var_qd_fb_dn0)), (locals.var_mfactor * (locals.var_qd_dn2 + locals.var_qd_fb_dn2)), (locals.var_mfactor * (locals.var_qd_dn6 + locals.var_qd_fb_dn6)), (locals.var_mfactor * (locals.var_qd_dn7 + locals.var_qd_fb_dn7)), (locals.var_mfactor * (locals.var_qd_dn10 + locals.var_qd_fb_dn10)), (locals.var_mfactor * (locals.var_qd_dn11 + locals.var_qd_fb_dn11)), (locals.var_mfactor * (locals.var_qd_dn12 + locals.var_qd_fb_dn12)), (locals.var_mfactor * (locals.var_qd_dn13 + locals.var_qd_fb_dn13)), (locals.var_mfactor * (locals.var_qd_dn15 + locals.var_qd_fb_dn15)), (locals.var_mfactor * (locals.var_qd_dn16 + locals.var_qd_fb_dn16)), (locals.var_mfactor * (locals.var_qd_dn17 + locals.var_qd_fb_dn17)), (locals.var_mfactor * (locals.var_qd_dn18 + locals.var_qd_fb_dn18)),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34310_e49421;
        locals.var_qde_dn0 = assign34310_e49421_d_n0;
        locals.var_qde_dn2 = assign34310_e49421_d_n2;
        locals.var_qde_dn6 = assign34310_e49421_d_n6;
        locals.var_qde_dn7 = assign34310_e49421_d_n7;
        locals.var_qde_dn10 = assign34310_e49421_d_n10;
        locals.var_qde_dn11 = assign34310_e49421_d_n11;
        locals.var_qde_dn12 = assign34310_e49421_d_n12;
        locals.var_qde_dn13 = assign34310_e49421_d_n13;
        locals.var_qde_dn15 = assign34310_e49421_d_n15;
        locals.var_qde_dn16 = assign34310_e49421_d_n16;
        locals.var_qde_dn17 = assign34310_e49421_d_n17;
        locals.var_qde_dn18 = assign34310_e49421_d_n18;

        let (assign34320_e49435, assign34320_e49435_d_n0, assign34320_e49435_d_n2, assign34320_e49435_d_n6, assign34320_e49435_d_n7, assign34320_e49435_d_n10, assign34320_e49435_d_n11, assign34320_e49435_d_n12, assign34320_e49435_d_n13, assign34320_e49435_d_n15, assign34320_e49435_d_n16, assign34320_e49435_d_n17, assign34320_e49435_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
        let assign34320_e49430: f64 = (locals.var_qi - locals.var_qd);
        let assign34320_e49432: f64 = (assign34320_e49430 + locals.var_qs_fb);
        let assign34320_e49433: f64 = (locals.var_mfactor * assign34320_e49432);
        (assign34320_e49433, (locals.var_mfactor * ((locals.var_qi_dn0 - locals.var_qd_dn0) + locals.var_qs_fb_dn0)), (locals.var_mfactor * ((locals.var_qi_dn2 - locals.var_qd_dn2) + locals.var_qs_fb_dn2)), (locals.var_mfactor * ((locals.var_qi_dn6 - locals.var_qd_dn6) + locals.var_qs_fb_dn6)), (locals.var_mfactor * ((locals.var_qi_dn7 - locals.var_qd_dn7) + locals.var_qs_fb_dn7)), (locals.var_mfactor * ((locals.var_qi_dn10 - locals.var_qd_dn10) + locals.var_qs_fb_dn10)), (locals.var_mfactor * ((locals.var_qi_dn11 - locals.var_qd_dn11) + locals.var_qs_fb_dn11)), (locals.var_mfactor * ((locals.var_qi_dn12 - locals.var_qd_dn12) + locals.var_qs_fb_dn12)), (locals.var_mfactor * ((-locals.var_qd_dn13) + locals.var_qs_fb_dn13)), (locals.var_mfactor * ((-locals.var_qd_dn15) + locals.var_qs_fb_dn15)), (locals.var_mfactor * ((-locals.var_qd_dn16) + locals.var_qs_fb_dn16)), (locals.var_mfactor * ((locals.var_qi_dn17 - locals.var_qd_dn17) + locals.var_qs_fb_dn17)), (locals.var_mfactor * ((-locals.var_qd_dn18) + locals.var_qs_fb_dn18)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34320_e49435;
        locals.var_qse_dn0 = assign34320_e49435_d_n0;
        locals.var_qse_dn2 = assign34320_e49435_d_n2;
        locals.var_qse_dn6 = assign34320_e49435_d_n6;
        locals.var_qse_dn7 = assign34320_e49435_d_n7;
        locals.var_qse_dn10 = assign34320_e49435_d_n10;
        locals.var_qse_dn11 = assign34320_e49435_d_n11;
        locals.var_qse_dn12 = assign34320_e49435_d_n12;
        locals.var_qse_dn13 = assign34320_e49435_d_n13;
        locals.var_qse_dn15 = assign34320_e49435_d_n15;
        locals.var_qse_dn16 = assign34320_e49435_d_n16;
        locals.var_qse_dn17 = assign34320_e49435_d_n17;
        locals.var_qse_dn18 = assign34320_e49435_d_n18;

        let assign34330_e49438: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1132 = assign34330_e49438;

        let (assign34340_e49442, assign34340_e49442_d_n0, assign34340_e49442_d_n2, assign34340_e49442_d_n6, assign34340_e49442_d_n7, assign34340_e49442_d_n10, assign34340_e49442_d_n11, assign34340_e49442_d_n12, assign34340_e49442_d_n17,) = {
    if (locals.var_guard1132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34340_e49442;
        locals.var_qy_dn0 = assign34340_e49442_d_n0;
        locals.var_qy_dn2 = assign34340_e49442_d_n2;
        locals.var_qy_dn6 = assign34340_e49442_d_n6;
        locals.var_qy_dn7 = assign34340_e49442_d_n7;
        locals.var_qy_dn10 = assign34340_e49442_d_n10;
        locals.var_qy_dn11 = assign34340_e49442_d_n11;
        locals.var_qy_dn12 = assign34340_e49442_d_n12;
        locals.var_qy_dn17 = assign34340_e49442_d_n17;

        let (assign34350_e49451, assign34350_e49451_d_n0, assign34350_e49451_d_n2, assign34350_e49451_d_n6, assign34350_e49451_d_n7, assign34350_e49451_d_n10, assign34350_e49451_d_n11, assign34350_e49451_d_n12, assign34350_e49451_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34350_e49447: f64 = (locals.var_ec * locals.var_leff);
        let assign34350_e49449: f64 = (assign34350_e49447 + locals.var_ps0);
        (assign34350_e49449, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn12 * locals.var_leff) + locals.var_ps0_dn12), ((locals.var_ec_dn17 * locals.var_leff) + locals.var_ps0_dn17),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17,)
    }
};
        locals.var_pslk = assign34350_e49451;
        locals.var_pslk_dn0 = assign34350_e49451_d_n0;
        locals.var_pslk_dn2 = assign34350_e49451_d_n2;
        locals.var_pslk_dn6 = assign34350_e49451_d_n6;
        locals.var_pslk_dn7 = assign34350_e49451_d_n7;
        locals.var_pslk_dn10 = assign34350_e49451_d_n10;
        locals.var_pslk_dn11 = assign34350_e49451_d_n11;
        locals.var_pslk_dn12 = assign34350_e49451_d_n12;
        locals.var_pslk_dn17 = assign34350_e49451_d_n17;

        let assign34360_e49454: f64 = if locals.var_pslk > locals.var_psdl { 1.0 } else { 0.0 };
        locals.var_guard1133 = assign34360_e49454;

    }

    pub(super) fn stamp_transient_block_120(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34370_e49461, assign34370_e49461_d_n0, assign34370_e49461_d_n2, assign34370_e49461_d_n6, assign34370_e49461_d_n7, assign34370_e49461_d_n10, assign34370_e49461_d_n11, assign34370_e49461_d_n12, assign34370_e49461_d_n17,) = {
    if ((locals.var_guard1132 == 0.0) && (locals.var_guard1133 != 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
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

        let (assign34380_e49476, assign34380_e49476_d_n0, assign34380_e49476_d_n2, assign34380_e49476_d_n6, assign34380_e49476_d_n7, assign34380_e49476_d_n10, assign34380_e49476_d_n11, assign34380_e49476_d_n12, assign34380_e49476_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34380_e49467: f64 = (locals.var_vds + locals.var_ps0);
        let assign34380_e49468: f64 = (locals.var_aclm * assign34380_e49467);
        let assign34380_e49471: f64 = (1.0 - locals.var_aclm);
        let assign34380_e49473: f64 = (assign34380_e49471 * locals.var_pslk);
        let assign34380_e49474: f64 = (assign34380_e49468 + assign34380_e49473);
        (assign34380_e49474, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign34380_e49471 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign34380_e49471 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign34380_e49471 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign34380_e49471 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign34380_e49471 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign34380_e49471 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + (assign34380_e49471 * locals.var_pslk_dn12)), ((locals.var_aclm * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + (assign34380_e49471 * locals.var_pslk_dn17)),)
    } else {
        (locals.var_t1__blk1128, locals.var_t1__blk1128_dn0, locals.var_t1__blk1128_dn2, locals.var_t1__blk1128_dn6, locals.var_t1__blk1128_dn7, locals.var_t1__blk1128_dn10, locals.var_t1__blk1128_dn11, locals.var_t1__blk1128_dn12, locals.var_t1__blk1128_dn17,)
    }
};
        locals.var_t1__blk1128 = assign34380_e49476;
        locals.var_t1__blk1128_dn0 = assign34380_e49476_d_n0;
        locals.var_t1__blk1128_dn2 = assign34380_e49476_d_n2;
        locals.var_t1__blk1128_dn6 = assign34380_e49476_d_n6;
        locals.var_t1__blk1128_dn7 = assign34380_e49476_d_n7;
        locals.var_t1__blk1128_dn10 = assign34380_e49476_d_n10;
        locals.var_t1__blk1128_dn11 = assign34380_e49476_d_n11;
        locals.var_t1__blk1128_dn12 = assign34380_e49476_d_n12;
        locals.var_t1__blk1128_dn17 = assign34380_e49476_d_n17;

        let (assign34390_e49486, assign34390_e49486_d_n0, assign34390_e49486_d_n2, assign34390_e49486_d_n6, assign34390_e49486_d_n7, assign34390_e49486_d_n10, assign34390_e49486_d_n11, assign34390_e49486_d_n12, assign34390_e49486_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34390_e49481: f64 = (2.0 * 1.034943e-10);
        let assign34390_e49483: f64 = (assign34390_e49481 / locals.var_q_nsub);
        let assign34390_e49484: f64 = (assign34390_e49483).sqrt();
        (assign34390_e49484, ((-((assign34390_e49481 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)),)
    } else {
        (locals.var_t10__blk1129, locals.var_t10__blk1129_dn0, locals.var_t10__blk1129_dn2, locals.var_t10__blk1129_dn6, locals.var_t10__blk1129_dn7, locals.var_t10__blk1129_dn10, locals.var_t10__blk1129_dn11, locals.var_t10__blk1129_dn12, locals.var_t10__blk1129_dn17,)
    }
};
        locals.var_t10__blk1129 = assign34390_e49486;
        locals.var_t10__blk1129_dn0 = assign34390_e49486_d_n0;
        locals.var_t10__blk1129_dn2 = assign34390_e49486_d_n2;
        locals.var_t10__blk1129_dn6 = assign34390_e49486_d_n6;
        locals.var_t10__blk1129_dn7 = assign34390_e49486_d_n7;
        locals.var_t10__blk1129_dn10 = assign34390_e49486_d_n10;
        locals.var_t10__blk1129_dn11 = assign34390_e49486_d_n11;
        locals.var_t10__blk1129_dn12 = assign34390_e49486_d_n12;
        locals.var_t10__blk1129_dn17 = assign34390_e49486_d_n17;

        let (assign34400_e49493, assign34400_e49493_d_n0, assign34400_e49493_d_n2, assign34400_e49493_d_n6, assign34400_e49493_d_n7, assign34400_e49493_d_n10, assign34400_e49493_d_n11, assign34400_e49493_d_n12, assign34400_e49493_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34400_e49491: f64 = (locals.var_t10__blk1129 * 1.3);
        (assign34400_e49491, (locals.var_t10__blk1129_dn0 * 1.3), (locals.var_t10__blk1129_dn2 * 1.3), (locals.var_t10__blk1129_dn6 * 1.3), (locals.var_t10__blk1129_dn7 * 1.3), (locals.var_t10__blk1129_dn10 * 1.3), (locals.var_t10__blk1129_dn11 * 1.3), (locals.var_t10__blk1129_dn12 * 1.3), (locals.var_t10__blk1129_dn17 * 1.3),)
    } else {
        (locals.var_t3__blk1130, locals.var_t3__blk1130_dn0, locals.var_t3__blk1130_dn2, locals.var_t3__blk1130_dn6, locals.var_t3__blk1130_dn7, locals.var_t3__blk1130_dn10, locals.var_t3__blk1130_dn11, locals.var_t3__blk1130_dn12, locals.var_t3__blk1130_dn17,)
    }
};
        locals.var_t3__blk1130 = assign34400_e49493;
        locals.var_t3__blk1130_dn0 = assign34400_e49493_d_n0;
        locals.var_t3__blk1130_dn2 = assign34400_e49493_d_n2;
        locals.var_t3__blk1130_dn6 = assign34400_e49493_d_n6;
        locals.var_t3__blk1130_dn7 = assign34400_e49493_d_n7;
        locals.var_t3__blk1130_dn10 = assign34400_e49493_d_n10;
        locals.var_t3__blk1130_dn11 = assign34400_e49493_d_n11;
        locals.var_t3__blk1130_dn12 = assign34400_e49493_d_n12;
        locals.var_t3__blk1130_dn17 = assign34400_e49493_d_n17;

        let (assign34410_e49502, assign34410_e49502_d_n0, assign34410_e49502_d_n2, assign34410_e49502_d_n6, assign34410_e49502_d_n7, assign34410_e49502_d_n10, assign34410_e49502_d_n11, assign34410_e49502_d_n12, assign34410_e49502_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34410_e49498: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign34410_e49500: f64 = (assign34410_e49498 * locals.var_t3__blk1130);
        (assign34410_e49500, (assign34410_e49498 * locals.var_t3__blk1130_dn0), (assign34410_e49498 * locals.var_t3__blk1130_dn2), (assign34410_e49498 * locals.var_t3__blk1130_dn6), (assign34410_e49498 * locals.var_t3__blk1130_dn7), (assign34410_e49498 * locals.var_t3__blk1130_dn10), (assign34410_e49498 * locals.var_t3__blk1130_dn11), (assign34410_e49498 * locals.var_t3__blk1130_dn12), (assign34410_e49498 * locals.var_t3__blk1130_dn17),)
    } else {
        (locals.var_t2__blk1131, locals.var_t2__blk1131_dn0, locals.var_t2__blk1131_dn2, locals.var_t2__blk1131_dn6, locals.var_t2__blk1131_dn7, locals.var_t2__blk1131_dn10, locals.var_t2__blk1131_dn11, locals.var_t2__blk1131_dn12, locals.var_t2__blk1131_dn17,)
    }
};
        locals.var_t2__blk1131 = assign34410_e49502;
        locals.var_t2__blk1131_dn0 = assign34410_e49502_d_n0;
        locals.var_t2__blk1131_dn2 = assign34410_e49502_d_n2;
        locals.var_t2__blk1131_dn6 = assign34410_e49502_d_n6;
        locals.var_t2__blk1131_dn7 = assign34410_e49502_d_n7;
        locals.var_t2__blk1131_dn10 = assign34410_e49502_d_n10;
        locals.var_t2__blk1131_dn11 = assign34410_e49502_d_n11;
        locals.var_t2__blk1131_dn12 = assign34410_e49502_d_n12;
        locals.var_t2__blk1131_dn17 = assign34410_e49502_d_n17;

        let (assign34420_e49517, assign34420_e49517_d_n0, assign34420_e49517_d_n2, assign34420_e49517_d_n6, assign34420_e49517_d_n7, assign34420_e49517_d_n10, assign34420_e49517_d_n11, assign34420_e49517_d_n12, assign34420_e49517_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34420_e49507: f64 = (locals.var_ps0 + locals.var_vds);
        let assign34420_e49509: f64 = (assign34420_e49507 - locals.var_t1__blk1128);
        let assign34420_e49511: f64 = (assign34420_e49509 / p.p64);
        let assign34420_e49513: f64 = (assign34420_e49511 - locals.var_ec);
        let assign34420_e49515: f64 = (assign34420_e49513 * locals.var_t2__blk1131);
        (assign34420_e49515, ((((((locals.var_ps0_dn0 + locals.var_vds_dn0) - locals.var_t1__blk1128_dn0) / p.p64) - locals.var_ec_dn0) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn0)), ((((((locals.var_ps0_dn2 + locals.var_vds_dn2) - locals.var_t1__blk1128_dn2) / p.p64) - locals.var_ec_dn2) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn2)), ((((((locals.var_ps0_dn6 + locals.var_vds_dn6) - locals.var_t1__blk1128_dn6) / p.p64) - locals.var_ec_dn6) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn6)), ((((((locals.var_ps0_dn7 + locals.var_vds_dn7) - locals.var_t1__blk1128_dn7) / p.p64) - locals.var_ec_dn7) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn7)), ((((((locals.var_ps0_dn10 + locals.var_vds_dn10) - locals.var_t1__blk1128_dn10) / p.p64) - locals.var_ec_dn10) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn10)), ((((((locals.var_ps0_dn11 + locals.var_vds_dn11) - locals.var_t1__blk1128_dn11) / p.p64) - locals.var_ec_dn11) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn11)), ((((((locals.var_ps0_dn12 + locals.var_vds_dn12) - locals.var_t1__blk1128_dn12) / p.p64) - locals.var_ec_dn12) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn12)), ((((((locals.var_ps0_dn17 + locals.var_vds_dn17) - locals.var_t1__blk1128_dn17) / p.p64) - locals.var_ec_dn17) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn17)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34420_e49517;
        locals.var_qy_dn0 = assign34420_e49517_d_n0;
        locals.var_qy_dn2 = assign34420_e49517_d_n2;
        locals.var_qy_dn6 = assign34420_e49517_d_n6;
        locals.var_qy_dn7 = assign34420_e49517_d_n7;
        locals.var_qy_dn10 = assign34420_e49517_d_n10;
        locals.var_qy_dn11 = assign34420_e49517_d_n11;
        locals.var_qy_dn12 = assign34420_e49517_d_n12;
        locals.var_qy_dn17 = assign34420_e49517_d_n17;

        let assign34430_e49520: f64 = if p.p65 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1134 = assign34430_e49520;

        let (assign34440_e49528, assign34440_e49528_d_n0, assign34440_e49528_d_n2, assign34440_e49528_d_n6, assign34440_e49528_d_n7, assign34440_e49528_d_n10, assign34440_e49528_d_n11, assign34440_e49528_d_n12, assign34440_e49528_d_n17,) = {
    if (locals.var_guard1134 != 0.0) {
        let assign34440_e49525: f64 = (locals.var_cqyb0 * locals.var_vbsp);
        let assign34440_e49526: f64 = (locals.var_qy + assign34440_e49525);
        (assign34440_e49526, (locals.var_qy_dn0 + (locals.var_cqyb0 * locals.var_vbsp_dn0)), (locals.var_qy_dn2 + (locals.var_cqyb0 * locals.var_vbsp_dn2)), (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbsp_dn6)), (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbsp_dn7)), (locals.var_qy_dn10 + (locals.var_cqyb0 * locals.var_vbsp_dn10)), (locals.var_qy_dn11 + (locals.var_cqyb0 * locals.var_vbsp_dn11)), (locals.var_qy_dn12 + (locals.var_cqyb0 * locals.var_vbsp_dn12)), (locals.var_qy_dn17 + (locals.var_cqyb0 * locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34440_e49528;
        locals.var_qy_dn0 = assign34440_e49528_d_n0;
        locals.var_qy_dn2 = assign34440_e49528_d_n2;
        locals.var_qy_dn6 = assign34440_e49528_d_n6;
        locals.var_qy_dn7 = assign34440_e49528_d_n7;
        locals.var_qy_dn10 = assign34440_e49528_d_n10;
        locals.var_qy_dn11 = assign34440_e49528_d_n11;
        locals.var_qy_dn12 = assign34440_e49528_d_n12;
        locals.var_qy_dn17 = assign34440_e49528_d_n17;

        let assign34450_e49531: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1135 = assign34450_e49531;

        let assign34460_e49534: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1136 = assign34460_e49534;

        let (assign34470_e49547, assign34470_e49547_d_n0, assign34470_e49547_d_n2, assign34470_e49547_d_n6, assign34470_e49547_d_n7, assign34470_e49547_d_n10, assign34470_e49547_d_n11, assign34470_e49547_d_n12, assign34470_e49547_d_n17,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34470_e49539: f64 = (-locals.var_qbody_bt_p_sus);
        let assign34470_e49541: f64 = (assign34470_e49539 - locals.var_qbody_bt_p_sud);
        let assign34470_e49543: f64 = (assign34470_e49541 - locals.var_qbody_bt_n_sus);
        let assign34470_e49545: f64 = (assign34470_e49543 - locals.var_qbody_bt_n_sud);
        (assign34470_e49545, ((((-locals.var_qbody_bt_p_sus_dn0) - locals.var_qbody_bt_p_sud_dn0) - locals.var_qbody_bt_n_sus_dn0) - locals.var_qbody_bt_n_sud_dn0), ((((-locals.var_qbody_bt_p_sus_dn2) - locals.var_qbody_bt_p_sud_dn2) - locals.var_qbody_bt_n_sus_dn2) - locals.var_qbody_bt_n_sud_dn2), ((((-locals.var_qbody_bt_p_sus_dn6) - locals.var_qbody_bt_p_sud_dn6) - locals.var_qbody_bt_n_sus_dn6) - locals.var_qbody_bt_n_sud_dn6), ((((-locals.var_qbody_bt_p_sus_dn7) - locals.var_qbody_bt_p_sud_dn7) - locals.var_qbody_bt_n_sus_dn7) - locals.var_qbody_bt_n_sud_dn7), ((((-locals.var_qbody_bt_p_sus_dn10) - locals.var_qbody_bt_p_sud_dn10) - locals.var_qbody_bt_n_sus_dn10) - locals.var_qbody_bt_n_sud_dn10), ((((-locals.var_qbody_bt_p_sus_dn11) - locals.var_qbody_bt_p_sud_dn11) - locals.var_qbody_bt_n_sus_dn11) - locals.var_qbody_bt_n_sud_dn11), ((((-locals.var_qbody_bt_p_sus_dn12) - locals.var_qbody_bt_p_sud_dn12) - locals.var_qbody_bt_n_sus_dn12) - locals.var_qbody_bt_n_sud_dn12), ((((-locals.var_qbody_bt_p_sus_dn17) - locals.var_qbody_bt_p_sud_dn17) - locals.var_qbody_bt_n_sus_dn17) - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_q_bt_ge, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, locals.var_q_bt_ge_dn17,)
    }
};
        locals.var_q_bt_ge = assign34470_e49547;
        locals.var_q_bt_ge_dn0 = assign34470_e49547_d_n0;
        locals.var_q_bt_ge_dn2 = assign34470_e49547_d_n2;
        locals.var_q_bt_ge_dn6 = assign34470_e49547_d_n6;
        locals.var_q_bt_ge_dn7 = assign34470_e49547_d_n7;
        locals.var_q_bt_ge_dn10 = assign34470_e49547_d_n10;
        locals.var_q_bt_ge_dn11 = assign34470_e49547_d_n11;
        locals.var_q_bt_ge_dn12 = assign34470_e49547_d_n12;
        locals.var_q_bt_ge_dn17 = assign34470_e49547_d_n17;

        let (assign34480_e49555, assign34480_e49555_d_n0, assign34480_e49555_d_n2, assign34480_e49555_d_n6, assign34480_e49555_d_n7, assign34480_e49555_d_n10, assign34480_e49555_d_n11, assign34480_e49555_d_n12, assign34480_e49555_d_n17,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34480_e49553: f64 = (locals.var_qbody_bt_p_iud + locals.var_qbody_bt_n_iud);
        (assign34480_e49553, (locals.var_qbody_bt_p_iud_dn0 + locals.var_qbody_bt_n_iud_dn0), (locals.var_qbody_bt_p_iud_dn2 + locals.var_qbody_bt_n_iud_dn2), (locals.var_qbody_bt_p_iud_dn6 + locals.var_qbody_bt_n_iud_dn6), (locals.var_qbody_bt_p_iud_dn7 + locals.var_qbody_bt_n_iud_dn7), (locals.var_qbody_bt_p_iud_dn10 + locals.var_qbody_bt_n_iud_dn10), (locals.var_qbody_bt_p_iud_dn11 + locals.var_qbody_bt_n_iud_dn11), (locals.var_qbody_bt_p_iud_dn12 + locals.var_qbody_bt_n_iud_dn12), (locals.var_qbody_bt_p_iud_dn17 + locals.var_qbody_bt_n_iud_dn17),)
    } else {
        (locals.var_q_bt_de, locals.var_q_bt_de_dn0, locals.var_q_bt_de_dn2, locals.var_q_bt_de_dn6, locals.var_q_bt_de_dn7, locals.var_q_bt_de_dn10, locals.var_q_bt_de_dn11, locals.var_q_bt_de_dn12, locals.var_q_bt_de_dn17,)
    }
};
        locals.var_q_bt_de = assign34480_e49555;
        locals.var_q_bt_de_dn0 = assign34480_e49555_d_n0;
        locals.var_q_bt_de_dn2 = assign34480_e49555_d_n2;
        locals.var_q_bt_de_dn6 = assign34480_e49555_d_n6;
        locals.var_q_bt_de_dn7 = assign34480_e49555_d_n7;
        locals.var_q_bt_de_dn10 = assign34480_e49555_d_n10;
        locals.var_q_bt_de_dn11 = assign34480_e49555_d_n11;
        locals.var_q_bt_de_dn12 = assign34480_e49555_d_n12;
        locals.var_q_bt_de_dn17 = assign34480_e49555_d_n17;

        let (assign34490_e49563, assign34490_e49563_d_n0, assign34490_e49563_d_n2, assign34490_e49563_d_n6, assign34490_e49563_d_n7, assign34490_e49563_d_n10, assign34490_e49563_d_n11, assign34490_e49563_d_n12, assign34490_e49563_d_n17,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34490_e49561: f64 = (locals.var_qbody_bt_p_ius + locals.var_qbody_bt_n_ius);
        (assign34490_e49561, (locals.var_qbody_bt_p_ius_dn0 + locals.var_qbody_bt_n_ius_dn0), (locals.var_qbody_bt_p_ius_dn2 + locals.var_qbody_bt_n_ius_dn2), (locals.var_qbody_bt_p_ius_dn6 + locals.var_qbody_bt_n_ius_dn6), (locals.var_qbody_bt_p_ius_dn7 + locals.var_qbody_bt_n_ius_dn7), (locals.var_qbody_bt_p_ius_dn10 + locals.var_qbody_bt_n_ius_dn10), (locals.var_qbody_bt_p_ius_dn11 + locals.var_qbody_bt_n_ius_dn11), (locals.var_qbody_bt_p_ius_dn12 + locals.var_qbody_bt_n_ius_dn12), (locals.var_qbody_bt_p_ius_dn17 + locals.var_qbody_bt_n_ius_dn17),)
    } else {
        (locals.var_q_bt_se, locals.var_q_bt_se_dn0, locals.var_q_bt_se_dn2, locals.var_q_bt_se_dn6, locals.var_q_bt_se_dn7, locals.var_q_bt_se_dn10, locals.var_q_bt_se_dn11, locals.var_q_bt_se_dn12, locals.var_q_bt_se_dn17,)
    }
};
        locals.var_q_bt_se = assign34490_e49563;
        locals.var_q_bt_se_dn0 = assign34490_e49563_d_n0;
        locals.var_q_bt_se_dn2 = assign34490_e49563_d_n2;
        locals.var_q_bt_se_dn6 = assign34490_e49563_d_n6;
        locals.var_q_bt_se_dn7 = assign34490_e49563_d_n7;
        locals.var_q_bt_se_dn10 = assign34490_e49563_d_n10;
        locals.var_q_bt_se_dn11 = assign34490_e49563_d_n11;
        locals.var_q_bt_se_dn12 = assign34490_e49563_d_n12;
        locals.var_q_bt_se_dn17 = assign34490_e49563_d_n17;

        let (assign34500_e49585, assign34500_e49585_d_n0, assign34500_e49585_d_n2, assign34500_e49585_d_n6, assign34500_e49585_d_n7, assign34500_e49585_d_n10, assign34500_e49585_d_n11, assign34500_e49585_d_n12, assign34500_e49585_d_n13, assign34500_e49585_d_n15, assign34500_e49585_d_n16, assign34500_e49585_d_n17, assign34500_e49585_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34500_e49571: f64 = (locals.var_qgod + locals.var_qgos);
        let assign34500_e49573: f64 = (assign34500_e49571 + locals.var_qgob);
        let assign34500_e49575: f64 = (assign34500_e49573 - locals.var_qy);
        let assign34500_e49577: f64 = (assign34500_e49575 - locals.var_qovs);
        let assign34500_e49579: f64 = (assign34500_e49577 - locals.var_qovd);
        let assign34500_e49581: f64 = (assign34500_e49579 + locals.var_q_bt_ge);
        let assign34500_e49582: f64 = (locals.var_mfactor * assign34500_e49581);
        let assign34500_e49583: f64 = (locals.var_qge + assign34500_e49582);
        (assign34500_e49583, (locals.var_qge_dn0 + (locals.var_mfactor * ((((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0) + locals.var_q_bt_ge_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2) + locals.var_q_bt_ge_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * ((((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6) + locals.var_q_bt_ge_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7) + locals.var_q_bt_ge_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * ((((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10) + locals.var_q_bt_ge_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11) + locals.var_q_bt_ge_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * ((((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12) + locals.var_q_bt_ge_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * ((((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17) + locals.var_q_bt_ge_dn17))), locals.var_qge_dn18,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34500_e49585;
        locals.var_qge_dn0 = assign34500_e49585_d_n0;
        locals.var_qge_dn2 = assign34500_e49585_d_n2;
        locals.var_qge_dn6 = assign34500_e49585_d_n6;
        locals.var_qge_dn7 = assign34500_e49585_d_n7;
        locals.var_qge_dn10 = assign34500_e49585_d_n10;
        locals.var_qge_dn11 = assign34500_e49585_d_n11;
        locals.var_qge_dn12 = assign34500_e49585_d_n12;
        locals.var_qge_dn13 = assign34500_e49585_d_n13;
        locals.var_qge_dn15 = assign34500_e49585_d_n15;
        locals.var_qge_dn16 = assign34500_e49585_d_n16;
        locals.var_qge_dn17 = assign34500_e49585_d_n17;
        locals.var_qge_dn18 = assign34500_e49585_d_n18;

        let (assign34510_e49602, assign34510_e49602_d_n0, assign34510_e49602_d_n2, assign34510_e49602_d_n6, assign34510_e49602_d_n7, assign34510_e49602_d_n10, assign34510_e49602_d_n11, assign34510_e49602_d_n12, assign34510_e49602_d_n13, assign34510_e49602_d_n15, assign34510_e49602_d_n16, assign34510_e49602_d_n17, assign34510_e49602_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34510_e49592: f64 = (-locals.var_qgod);
        let assign34510_e49594: f64 = (assign34510_e49592 + locals.var_qy);
        let assign34510_e49596: f64 = (assign34510_e49594 + locals.var_qbdld);
        let assign34510_e49598: f64 = (assign34510_e49596 + locals.var_q_bt_de);
        let assign34510_e49599: f64 = (locals.var_mfactor * assign34510_e49598);
        let assign34510_e49600: f64 = (locals.var_qde + assign34510_e49599);
        (assign34510_e49600, (locals.var_qde_dn0 + (locals.var_mfactor * ((((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0) + locals.var_q_bt_de_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2) + locals.var_q_bt_de_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * ((((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6) + locals.var_q_bt_de_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7) + locals.var_q_bt_de_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * ((((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10) + locals.var_q_bt_de_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11) + locals.var_q_bt_de_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * ((((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12) + locals.var_q_bt_de_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * ((((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17) + locals.var_q_bt_de_dn17))), locals.var_qde_dn18,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34510_e49602;
        locals.var_qde_dn0 = assign34510_e49602_d_n0;
        locals.var_qde_dn2 = assign34510_e49602_d_n2;
        locals.var_qde_dn6 = assign34510_e49602_d_n6;
        locals.var_qde_dn7 = assign34510_e49602_d_n7;
        locals.var_qde_dn10 = assign34510_e49602_d_n10;
        locals.var_qde_dn11 = assign34510_e49602_d_n11;
        locals.var_qde_dn12 = assign34510_e49602_d_n12;
        locals.var_qde_dn13 = assign34510_e49602_d_n13;
        locals.var_qde_dn15 = assign34510_e49602_d_n15;
        locals.var_qde_dn16 = assign34510_e49602_d_n16;
        locals.var_qde_dn17 = assign34510_e49602_d_n17;
        locals.var_qde_dn18 = assign34510_e49602_d_n18;

        let (assign34520_e49617, assign34520_e49617_d_n0, assign34520_e49617_d_n2, assign34520_e49617_d_n6, assign34520_e49617_d_n7, assign34520_e49617_d_n10, assign34520_e49617_d_n11, assign34520_e49617_d_n12, assign34520_e49617_d_n13, assign34520_e49617_d_n15, assign34520_e49617_d_n16, assign34520_e49617_d_n17, assign34520_e49617_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34520_e49609: f64 = (-locals.var_qgos);
        let assign34520_e49611: f64 = (assign34520_e49609 + locals.var_qbsld);
        let assign34520_e49613: f64 = (assign34520_e49611 + locals.var_q_bt_se);
        let assign34520_e49614: f64 = (locals.var_mfactor * assign34520_e49613);
        let assign34520_e49615: f64 = (locals.var_qse + assign34520_e49614);
        (assign34520_e49615, (locals.var_qse_dn0 + (locals.var_mfactor * (((-locals.var_qgos_dn0) + locals.var_qbsld_dn0) + locals.var_q_bt_se_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * (((-locals.var_qgos_dn2) + locals.var_qbsld_dn2) + locals.var_q_bt_se_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * (((-locals.var_qgos_dn6) + locals.var_qbsld_dn6) + locals.var_q_bt_se_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * (((-locals.var_qgos_dn7) + locals.var_qbsld_dn7) + locals.var_q_bt_se_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * (((-locals.var_qgos_dn10) + locals.var_qbsld_dn10) + locals.var_q_bt_se_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * (((-locals.var_qgos_dn11) + locals.var_qbsld_dn11) + locals.var_q_bt_se_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * (((-locals.var_qgos_dn12) + locals.var_qbsld_dn12) + locals.var_q_bt_se_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * (((-locals.var_qgos_dn17) + locals.var_qbsld_dn17) + locals.var_q_bt_se_dn17))), locals.var_qse_dn18,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34520_e49617;
        locals.var_qse_dn0 = assign34520_e49617_d_n0;
        locals.var_qse_dn2 = assign34520_e49617_d_n2;
        locals.var_qse_dn6 = assign34520_e49617_d_n6;
        locals.var_qse_dn7 = assign34520_e49617_d_n7;
        locals.var_qse_dn10 = assign34520_e49617_d_n10;
        locals.var_qse_dn11 = assign34520_e49617_d_n11;
        locals.var_qse_dn12 = assign34520_e49617_d_n12;
        locals.var_qse_dn13 = assign34520_e49617_d_n13;
        locals.var_qse_dn15 = assign34520_e49617_d_n15;
        locals.var_qse_dn16 = assign34520_e49617_d_n16;
        locals.var_qse_dn17 = assign34520_e49617_d_n17;
        locals.var_qse_dn18 = assign34520_e49617_d_n18;

        let (assign34530_e49638, assign34530_e49638_d_n0, assign34530_e49638_d_n2, assign34530_e49638_d_n6, assign34530_e49638_d_n7, assign34530_e49638_d_n10, assign34530_e49638_d_n11, assign34530_e49638_d_n12, assign34530_e49638_d_n13, assign34530_e49638_d_n15, assign34530_e49638_d_n16, assign34530_e49638_d_n17, assign34530_e49638_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
        let assign34530_e49626: f64 = (locals.var_qgod + locals.var_qgos);
        let assign34530_e49628: f64 = (assign34530_e49626 + locals.var_qgob);
        let assign34530_e49630: f64 = (assign34530_e49628 - locals.var_qy);
        let assign34530_e49632: f64 = (assign34530_e49630 - locals.var_qovs);
        let assign34530_e49634: f64 = (assign34530_e49632 - locals.var_qovd);
        let assign34530_e49635: f64 = (locals.var_mfactor * assign34530_e49634);
        let assign34530_e49636: f64 = (locals.var_qge + assign34530_e49635);
        (assign34530_e49636, (locals.var_qge_dn0 + (locals.var_mfactor * (((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * (((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * (((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * (((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * (((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * (((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * (((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * (((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17))), locals.var_qge_dn18,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34530_e49638;
        locals.var_qge_dn0 = assign34530_e49638_d_n0;
        locals.var_qge_dn2 = assign34530_e49638_d_n2;
        locals.var_qge_dn6 = assign34530_e49638_d_n6;
        locals.var_qge_dn7 = assign34530_e49638_d_n7;
        locals.var_qge_dn10 = assign34530_e49638_d_n10;
        locals.var_qge_dn11 = assign34530_e49638_d_n11;
        locals.var_qge_dn12 = assign34530_e49638_d_n12;
        locals.var_qge_dn13 = assign34530_e49638_d_n13;
        locals.var_qge_dn15 = assign34530_e49638_d_n15;
        locals.var_qge_dn16 = assign34530_e49638_d_n16;
        locals.var_qge_dn17 = assign34530_e49638_d_n17;
        locals.var_qge_dn18 = assign34530_e49638_d_n18;

        let (assign34540_e49654, assign34540_e49654_d_n0, assign34540_e49654_d_n2, assign34540_e49654_d_n6, assign34540_e49654_d_n7, assign34540_e49654_d_n10, assign34540_e49654_d_n11, assign34540_e49654_d_n12, assign34540_e49654_d_n13, assign34540_e49654_d_n15, assign34540_e49654_d_n16, assign34540_e49654_d_n17, assign34540_e49654_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
        let assign34540_e49646: f64 = (-locals.var_qgod);
        let assign34540_e49648: f64 = (assign34540_e49646 + locals.var_qy);
        let assign34540_e49650: f64 = (assign34540_e49648 + locals.var_qbdld);
        let assign34540_e49651: f64 = (locals.var_mfactor * assign34540_e49650);
        let assign34540_e49652: f64 = (locals.var_qde + assign34540_e49651);
        (assign34540_e49652, (locals.var_qde_dn0 + (locals.var_mfactor * (((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * (((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * (((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * (((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * (((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * (((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * (((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * (((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17))), locals.var_qde_dn18,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34540_e49654;
        locals.var_qde_dn0 = assign34540_e49654_d_n0;
        locals.var_qde_dn2 = assign34540_e49654_d_n2;
        locals.var_qde_dn6 = assign34540_e49654_d_n6;
        locals.var_qde_dn7 = assign34540_e49654_d_n7;
        locals.var_qde_dn10 = assign34540_e49654_d_n10;
        locals.var_qde_dn11 = assign34540_e49654_d_n11;
        locals.var_qde_dn12 = assign34540_e49654_d_n12;
        locals.var_qde_dn13 = assign34540_e49654_d_n13;
        locals.var_qde_dn15 = assign34540_e49654_d_n15;
        locals.var_qde_dn16 = assign34540_e49654_d_n16;
        locals.var_qde_dn17 = assign34540_e49654_d_n17;
        locals.var_qde_dn18 = assign34540_e49654_d_n18;

        let (assign34550_e49668, assign34550_e49668_d_n0, assign34550_e49668_d_n2, assign34550_e49668_d_n6, assign34550_e49668_d_n7, assign34550_e49668_d_n10, assign34550_e49668_d_n11, assign34550_e49668_d_n12, assign34550_e49668_d_n13, assign34550_e49668_d_n15, assign34550_e49668_d_n16, assign34550_e49668_d_n17, assign34550_e49668_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
        let assign34550_e49662: f64 = (-locals.var_qgos);
        let assign34550_e49664: f64 = (assign34550_e49662 + locals.var_qbsld);
        let assign34550_e49665: f64 = (locals.var_mfactor * assign34550_e49664);
        let assign34550_e49666: f64 = (locals.var_qse + assign34550_e49665);
        (assign34550_e49666, (locals.var_qse_dn0 + (locals.var_mfactor * ((-locals.var_qgos_dn0) + locals.var_qbsld_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * ((-locals.var_qgos_dn2) + locals.var_qbsld_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * ((-locals.var_qgos_dn6) + locals.var_qbsld_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * ((-locals.var_qgos_dn7) + locals.var_qbsld_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * ((-locals.var_qgos_dn10) + locals.var_qbsld_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * ((-locals.var_qgos_dn11) + locals.var_qbsld_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * ((-locals.var_qgos_dn12) + locals.var_qbsld_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * ((-locals.var_qgos_dn17) + locals.var_qbsld_dn17))), locals.var_qse_dn18,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34550_e49668;
        locals.var_qse_dn0 = assign34550_e49668_d_n0;
        locals.var_qse_dn2 = assign34550_e49668_d_n2;
        locals.var_qse_dn6 = assign34550_e49668_d_n6;
        locals.var_qse_dn7 = assign34550_e49668_d_n7;
        locals.var_qse_dn10 = assign34550_e49668_d_n10;
        locals.var_qse_dn11 = assign34550_e49668_d_n11;
        locals.var_qse_dn12 = assign34550_e49668_d_n12;
        locals.var_qse_dn13 = assign34550_e49668_d_n13;
        locals.var_qse_dn15 = assign34550_e49668_d_n15;
        locals.var_qse_dn16 = assign34550_e49668_d_n16;
        locals.var_qse_dn17 = assign34550_e49668_d_n17;
        locals.var_qse_dn18 = assign34550_e49668_d_n18;

        let assign34580_e49673: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1137 = assign34580_e49673;

        let (assign34590_e49679, assign34590_e49679_d_n0, assign34590_e49679_d_n2, assign34590_e49679_d_n6, assign34590_e49679_d_n7, assign34590_e49679_d_n10, assign34590_e49679_d_n11, assign34590_e49679_d_n12, assign34590_e49679_d_n17,) = {
    if (locals.var_guard1137 != 0.0) {
        let assign34590_e49677: f64 = (locals.var_mfactor * locals.var_ibs);
        (assign34590_e49677, (locals.var_mfactor * locals.var_ibs_dn0), (locals.var_mfactor * locals.var_ibs_dn2), (locals.var_mfactor * locals.var_ibs_dn6), (locals.var_mfactor * locals.var_ibs_dn7), (locals.var_mfactor * locals.var_ibs_dn10), (locals.var_mfactor * locals.var_ibs_dn11), (locals.var_mfactor * locals.var_ibs_dn12), (locals.var_mfactor * locals.var_ibs_dn17),)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign34590_e49679;
        locals.var_ibsb_dn0 = assign34590_e49679_d_n0;
        locals.var_ibsb_dn2 = assign34590_e49679_d_n2;
        locals.var_ibsb_dn6 = assign34590_e49679_d_n6;
        locals.var_ibsb_dn7 = assign34590_e49679_d_n7;
        locals.var_ibsb_dn10 = assign34590_e49679_d_n10;
        locals.var_ibsb_dn11 = assign34590_e49679_d_n11;
        locals.var_ibsb_dn12 = assign34590_e49679_d_n12;
        locals.var_ibsb_dn17 = assign34590_e49679_d_n17;

        let (assign34600_e49685, assign34600_e49685_d_n0, assign34600_e49685_d_n2, assign34600_e49685_d_n6, assign34600_e49685_d_n7, assign34600_e49685_d_n10, assign34600_e49685_d_n11, assign34600_e49685_d_n12, assign34600_e49685_d_n17,) = {
    if (locals.var_guard1137 != 0.0) {
        let assign34600_e49683: f64 = (locals.var_mfactor * locals.var_ibd);
        (assign34600_e49683, (locals.var_mfactor * locals.var_ibd_dn0), (locals.var_mfactor * locals.var_ibd_dn2), (locals.var_mfactor * locals.var_ibd_dn6), (locals.var_mfactor * locals.var_ibd_dn7), (locals.var_mfactor * locals.var_ibd_dn10), (locals.var_mfactor * locals.var_ibd_dn11), (locals.var_mfactor * locals.var_ibd_dn12), (locals.var_mfactor * locals.var_ibd_dn17),)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign34600_e49685;
        locals.var_ibdb_dn0 = assign34600_e49685_d_n0;
        locals.var_ibdb_dn2 = assign34600_e49685_d_n2;
        locals.var_ibdb_dn6 = assign34600_e49685_d_n6;
        locals.var_ibdb_dn7 = assign34600_e49685_d_n7;
        locals.var_ibdb_dn10 = assign34600_e49685_d_n10;
        locals.var_ibdb_dn11 = assign34600_e49685_d_n11;
        locals.var_ibdb_dn12 = assign34600_e49685_d_n12;
        locals.var_ibdb_dn17 = assign34600_e49685_d_n17;

        let (assign34610_e49691, assign34610_e49691_d_n0, assign34610_e49691_d_n2, assign34610_e49691_d_n6, assign34610_e49691_d_n7, assign34610_e49691_d_n10, assign34610_e49691_d_n11, assign34610_e49691_d_n12, assign34610_e49691_d_n17,) = {
    if (locals.var_guard1137 != 0.0) {
        let assign34610_e49689: f64 = (locals.var_mfactor * locals.var_qbd);
        (assign34610_e49689, (locals.var_mfactor * locals.var_qbd_dn0), (locals.var_mfactor * locals.var_qbd_dn2), (locals.var_mfactor * locals.var_qbd_dn6), (locals.var_mfactor * locals.var_qbd_dn7), (locals.var_mfactor * locals.var_qbd_dn10), (locals.var_mfactor * locals.var_qbd_dn11), (locals.var_mfactor * locals.var_qbd_dn12), (locals.var_mfactor * locals.var_qbd_dn17),)
    } else {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    }
};
        locals.var_qbd_s0 = assign34610_e49691;
        locals.var_qbd_s0_dn0 = assign34610_e49691_d_n0;
        locals.var_qbd_s0_dn2 = assign34610_e49691_d_n2;
        locals.var_qbd_s0_dn6 = assign34610_e49691_d_n6;
        locals.var_qbd_s0_dn7 = assign34610_e49691_d_n7;
        locals.var_qbd_s0_dn10 = assign34610_e49691_d_n10;
        locals.var_qbd_s0_dn11 = assign34610_e49691_d_n11;
        locals.var_qbd_s0_dn12 = assign34610_e49691_d_n12;
        locals.var_qbd_s0_dn17 = assign34610_e49691_d_n17;

        let (assign34620_e49697, assign34620_e49697_d_n0, assign34620_e49697_d_n2, assign34620_e49697_d_n6, assign34620_e49697_d_n7, assign34620_e49697_d_n10, assign34620_e49697_d_n11, assign34620_e49697_d_n12, assign34620_e49697_d_n17,) = {
    if (locals.var_guard1137 != 0.0) {
        let assign34620_e49695: f64 = (locals.var_mfactor * locals.var_qbs);
        (assign34620_e49695, (locals.var_mfactor * locals.var_qbs_dn0), (locals.var_mfactor * locals.var_qbs_dn2), (locals.var_mfactor * locals.var_qbs_dn6), (locals.var_mfactor * locals.var_qbs_dn7), (locals.var_mfactor * locals.var_qbs_dn10), (locals.var_mfactor * locals.var_qbs_dn11), (locals.var_mfactor * locals.var_qbs_dn12), (locals.var_mfactor * locals.var_qbs_dn17),)
    } else {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    }
};
        locals.var_qbs_s0 = assign34620_e49697;
        locals.var_qbs_s0_dn0 = assign34620_e49697_d_n0;
        locals.var_qbs_s0_dn2 = assign34620_e49697_d_n2;
        locals.var_qbs_s0_dn6 = assign34620_e49697_d_n6;
        locals.var_qbs_s0_dn7 = assign34620_e49697_d_n7;
        locals.var_qbs_s0_dn10 = assign34620_e49697_d_n10;
        locals.var_qbs_s0_dn11 = assign34620_e49697_d_n11;
        locals.var_qbs_s0_dn12 = assign34620_e49697_d_n12;
        locals.var_qbs_s0_dn17 = assign34620_e49697_d_n17;

        let (assign34630_e49702, assign34630_e49702_d_n0, assign34630_e49702_d_n2, assign34630_e49702_d_n6, assign34630_e49702_d_n7, assign34630_e49702_d_n10, assign34630_e49702_d_n11, assign34630_e49702_d_n12, assign34630_e49702_d_n17,) = {
    if (locals.var_guard1137 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign34630_e49702;
        locals.var_ibsb_dn0 = assign34630_e49702_d_n0;
        locals.var_ibsb_dn2 = assign34630_e49702_d_n2;
        locals.var_ibsb_dn6 = assign34630_e49702_d_n6;
        locals.var_ibsb_dn7 = assign34630_e49702_d_n7;
        locals.var_ibsb_dn10 = assign34630_e49702_d_n10;
        locals.var_ibsb_dn11 = assign34630_e49702_d_n11;
        locals.var_ibsb_dn12 = assign34630_e49702_d_n12;
        locals.var_ibsb_dn17 = assign34630_e49702_d_n17;

        let (assign34640_e49707, assign34640_e49707_d_n0, assign34640_e49707_d_n2, assign34640_e49707_d_n6, assign34640_e49707_d_n7, assign34640_e49707_d_n10, assign34640_e49707_d_n11, assign34640_e49707_d_n12, assign34640_e49707_d_n17,) = {
    if (locals.var_guard1137 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign34640_e49707;
        locals.var_ibdb_dn0 = assign34640_e49707_d_n0;
        locals.var_ibdb_dn2 = assign34640_e49707_d_n2;
        locals.var_ibdb_dn6 = assign34640_e49707_d_n6;
        locals.var_ibdb_dn7 = assign34640_e49707_d_n7;
        locals.var_ibdb_dn10 = assign34640_e49707_d_n10;
        locals.var_ibdb_dn11 = assign34640_e49707_d_n11;
        locals.var_ibdb_dn12 = assign34640_e49707_d_n12;
        locals.var_ibdb_dn17 = assign34640_e49707_d_n17;

        let (assign34650_e49712, assign34650_e49712_d_n0, assign34650_e49712_d_n2, assign34650_e49712_d_n6, assign34650_e49712_d_n7, assign34650_e49712_d_n10, assign34650_e49712_d_n11, assign34650_e49712_d_n12, assign34650_e49712_d_n17,) = {
    if (locals.var_guard1137 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    }
};
        locals.var_qbd_s0 = assign34650_e49712;
        locals.var_qbd_s0_dn0 = assign34650_e49712_d_n0;
        locals.var_qbd_s0_dn2 = assign34650_e49712_d_n2;
        locals.var_qbd_s0_dn6 = assign34650_e49712_d_n6;
        locals.var_qbd_s0_dn7 = assign34650_e49712_d_n7;
        locals.var_qbd_s0_dn10 = assign34650_e49712_d_n10;
        locals.var_qbd_s0_dn11 = assign34650_e49712_d_n11;
        locals.var_qbd_s0_dn12 = assign34650_e49712_d_n12;
        locals.var_qbd_s0_dn17 = assign34650_e49712_d_n17;

        let (assign34660_e49717, assign34660_e49717_d_n0, assign34660_e49717_d_n2, assign34660_e49717_d_n6, assign34660_e49717_d_n7, assign34660_e49717_d_n10, assign34660_e49717_d_n11, assign34660_e49717_d_n12, assign34660_e49717_d_n17,) = {
    if (locals.var_guard1137 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    }
};
        locals.var_qbs_s0 = assign34660_e49717;
        locals.var_qbs_s0_dn0 = assign34660_e49717_d_n0;
        locals.var_qbs_s0_dn2 = assign34660_e49717_d_n2;
        locals.var_qbs_s0_dn6 = assign34660_e49717_d_n6;
        locals.var_qbs_s0_dn7 = assign34660_e49717_d_n7;
        locals.var_qbs_s0_dn10 = assign34660_e49717_d_n10;
        locals.var_qbs_s0_dn11 = assign34660_e49717_d_n11;
        locals.var_qbs_s0_dn12 = assign34660_e49717_d_n12;
        locals.var_qbs_s0_dn17 = assign34660_e49717_d_n17;

        let assign34670_e49720: f64 = if p.p25 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1138 = assign34670_e49720;

    }

    pub(super) fn stamp_transient_block_121(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34680_e49724, assign34680_e49724_d_n0, assign34680_e49724_d_n2, assign34680_e49724_d_n6, assign34680_e49724_d_n7, assign34680_e49724_d_n10, assign34680_e49724_d_n11, assign34680_e49724_d_n12, assign34680_e49724_d_n17,) = {
    if (locals.var_guard1138 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    }
};
        locals.var_isube = assign34680_e49724;
        locals.var_isube_dn0 = assign34680_e49724_d_n0;
        locals.var_isube_dn2 = assign34680_e49724_d_n2;
        locals.var_isube_dn6 = assign34680_e49724_d_n6;
        locals.var_isube_dn7 = assign34680_e49724_d_n7;
        locals.var_isube_dn10 = assign34680_e49724_d_n10;
        locals.var_isube_dn11 = assign34680_e49724_d_n11;
        locals.var_isube_dn12 = assign34680_e49724_d_n12;
        locals.var_isube_dn17 = assign34680_e49724_d_n17;

        let (assign34690_e49731, assign34690_e49731_d_n0, assign34690_e49731_d_n2, assign34690_e49731_d_n6, assign34690_e49731_d_n7, assign34690_e49731_d_n10, assign34690_e49731_d_n11, assign34690_e49731_d_n12, assign34690_e49731_d_n17,) = {
    if (locals.var_guard1138 == 0.0) {
        let assign34690_e49729: f64 = (locals.var_mfactor * locals.var_isub);
        (assign34690_e49729, (locals.var_mfactor * locals.var_isub_dn0), (locals.var_mfactor * locals.var_isub_dn2), (locals.var_mfactor * locals.var_isub_dn6), (locals.var_mfactor * locals.var_isub_dn7), (locals.var_mfactor * locals.var_isub_dn10), (locals.var_mfactor * locals.var_isub_dn11), (locals.var_mfactor * locals.var_isub_dn12), (locals.var_mfactor * locals.var_isub_dn17),)
    } else {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    }
};
        locals.var_isube = assign34690_e49731;
        locals.var_isube_dn0 = assign34690_e49731_d_n0;
        locals.var_isube_dn2 = assign34690_e49731_d_n2;
        locals.var_isube_dn6 = assign34690_e49731_d_n6;
        locals.var_isube_dn7 = assign34690_e49731_d_n7;
        locals.var_isube_dn10 = assign34690_e49731_d_n10;
        locals.var_isube_dn11 = assign34690_e49731_d_n11;
        locals.var_isube_dn12 = assign34690_e49731_d_n12;
        locals.var_isube_dn17 = assign34690_e49731_d_n17;

        let assign34700_e49734: f64 = (-locals.var_igb);
        let assign34700_e49735: f64 = (locals.var_mfactor * assign34700_e49734);
        locals.var_igbe = assign34700_e49735;
        locals.var_igbe_dn0 = (locals.var_mfactor * (-locals.var_igb_dn0));
        locals.var_igbe_dn2 = (locals.var_mfactor * (-locals.var_igb_dn2));
        locals.var_igbe_dn6 = (locals.var_mfactor * (-locals.var_igb_dn6));
        locals.var_igbe_dn7 = (locals.var_mfactor * (-locals.var_igb_dn7));
        locals.var_igbe_dn10 = (locals.var_mfactor * (-locals.var_igb_dn10));
        locals.var_igbe_dn11 = (locals.var_mfactor * (-locals.var_igb_dn11));
        locals.var_igbe_dn12 = (locals.var_mfactor * (-locals.var_igb_dn12));
        locals.var_igbe_dn17 = (locals.var_mfactor * (-locals.var_igb_dn17));

        let assign34710_e49738: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1139 = assign34710_e49738;

        let (assign34720_e49748, assign34720_e49748_d_n0, assign34720_e49748_d_n2, assign34720_e49748_d_n6, assign34720_e49748_d_n7, assign34720_e49748_d_n10, assign34720_e49748_d_n11, assign34720_e49748_d_n12, assign34720_e49748_d_n17,) = {
    if (locals.var_guard1139 != 0.0) {
        let assign34720_e49743: f64 = (locals.var_glpart1 * locals.var_igate);
        let assign34720_e49745: f64 = (assign34720_e49743 - locals.var_igd);
        let assign34720_e49746: f64 = (locals.var_mfactor * assign34720_e49745);
        (assign34720_e49746, (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn0) - locals.var_igd_dn0)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn2) - locals.var_igd_dn2)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn6) - locals.var_igd_dn6)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn7) - locals.var_igd_dn7)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn10) - locals.var_igd_dn10)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn11) - locals.var_igd_dn11)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn12) - locals.var_igd_dn12)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn17) - locals.var_igd_dn17)),)
    } else {
        (locals.var_igde, locals.var_igde_dn0, locals.var_igde_dn2, locals.var_igde_dn6, locals.var_igde_dn7, locals.var_igde_dn10, locals.var_igde_dn11, locals.var_igde_dn12, locals.var_igde_dn17,)
    }
};
        locals.var_igde = assign34720_e49748;
        locals.var_igde_dn0 = assign34720_e49748_d_n0;
        locals.var_igde_dn2 = assign34720_e49748_d_n2;
        locals.var_igde_dn6 = assign34720_e49748_d_n6;
        locals.var_igde_dn7 = assign34720_e49748_d_n7;
        locals.var_igde_dn10 = assign34720_e49748_d_n10;
        locals.var_igde_dn11 = assign34720_e49748_d_n11;
        locals.var_igde_dn12 = assign34720_e49748_d_n12;
        locals.var_igde_dn17 = assign34720_e49748_d_n17;

        let (assign34730_e49761, assign34730_e49761_d_n0, assign34730_e49761_d_n2, assign34730_e49761_d_n6, assign34730_e49761_d_n7, assign34730_e49761_d_n10, assign34730_e49761_d_n11, assign34730_e49761_d_n12, assign34730_e49761_d_n17,) = {
    if (locals.var_guard1139 == 0.0) {
        let assign34730_e49754: f64 = (1.0 - locals.var_glpart1);
        let assign34730_e49756: f64 = (assign34730_e49754 * locals.var_igate);
        let assign34730_e49758: f64 = (assign34730_e49756 - locals.var_igs);
        let assign34730_e49759: f64 = (locals.var_mfactor * assign34730_e49758);
        (assign34730_e49759, (locals.var_mfactor * ((assign34730_e49754 * locals.var_igate_dn0) - locals.var_igs_dn0)), (locals.var_mfactor * ((assign34730_e49754 * locals.var_igate_dn2) - locals.var_igs_dn2)), (locals.var_mfactor * ((assign34730_e49754 * locals.var_igate_dn6) - locals.var_igs_dn6)), (locals.var_mfactor * ((assign34730_e49754 * locals.var_igate_dn7) - locals.var_igs_dn7)), (locals.var_mfactor * ((assign34730_e49754 * locals.var_igate_dn10) - locals.var_igs_dn10)), (locals.var_mfactor * ((assign34730_e49754 * locals.var_igate_dn11) - locals.var_igs_dn11)), (locals.var_mfactor * ((assign34730_e49754 * locals.var_igate_dn12) - locals.var_igs_dn12)), (locals.var_mfactor * ((assign34730_e49754 * locals.var_igate_dn17) - locals.var_igs_dn17)),)
    } else {
        (locals.var_igde, locals.var_igde_dn0, locals.var_igde_dn2, locals.var_igde_dn6, locals.var_igde_dn7, locals.var_igde_dn10, locals.var_igde_dn11, locals.var_igde_dn12, locals.var_igde_dn17,)
    }
};
        locals.var_igde = assign34730_e49761;
        locals.var_igde_dn0 = assign34730_e49761_d_n0;
        locals.var_igde_dn2 = assign34730_e49761_d_n2;
        locals.var_igde_dn6 = assign34730_e49761_d_n6;
        locals.var_igde_dn7 = assign34730_e49761_d_n7;
        locals.var_igde_dn10 = assign34730_e49761_d_n10;
        locals.var_igde_dn11 = assign34730_e49761_d_n11;
        locals.var_igde_dn12 = assign34730_e49761_d_n12;
        locals.var_igde_dn17 = assign34730_e49761_d_n17;

        let assign34740_e49764: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1140 = assign34740_e49764;

        let (assign34750_e49776, assign34750_e49776_d_n0, assign34750_e49776_d_n2, assign34750_e49776_d_n6, assign34750_e49776_d_n7, assign34750_e49776_d_n10, assign34750_e49776_d_n11, assign34750_e49776_d_n12, assign34750_e49776_d_n17,) = {
    if (locals.var_guard1140 != 0.0) {
        let assign34750_e49769: f64 = (1.0 - locals.var_glpart1);
        let assign34750_e49771: f64 = (assign34750_e49769 * locals.var_igate);
        let assign34750_e49773: f64 = (assign34750_e49771 - locals.var_igs);
        let assign34750_e49774: f64 = (locals.var_mfactor * assign34750_e49773);
        (assign34750_e49774, (locals.var_mfactor * ((assign34750_e49769 * locals.var_igate_dn0) - locals.var_igs_dn0)), (locals.var_mfactor * ((assign34750_e49769 * locals.var_igate_dn2) - locals.var_igs_dn2)), (locals.var_mfactor * ((assign34750_e49769 * locals.var_igate_dn6) - locals.var_igs_dn6)), (locals.var_mfactor * ((assign34750_e49769 * locals.var_igate_dn7) - locals.var_igs_dn7)), (locals.var_mfactor * ((assign34750_e49769 * locals.var_igate_dn10) - locals.var_igs_dn10)), (locals.var_mfactor * ((assign34750_e49769 * locals.var_igate_dn11) - locals.var_igs_dn11)), (locals.var_mfactor * ((assign34750_e49769 * locals.var_igate_dn12) - locals.var_igs_dn12)), (locals.var_mfactor * ((assign34750_e49769 * locals.var_igate_dn17) - locals.var_igs_dn17)),)
    } else {
        (locals.var_igse, locals.var_igse_dn0, locals.var_igse_dn2, locals.var_igse_dn6, locals.var_igse_dn7, locals.var_igse_dn10, locals.var_igse_dn11, locals.var_igse_dn12, locals.var_igse_dn17,)
    }
};
        locals.var_igse = assign34750_e49776;
        locals.var_igse_dn0 = assign34750_e49776_d_n0;
        locals.var_igse_dn2 = assign34750_e49776_d_n2;
        locals.var_igse_dn6 = assign34750_e49776_d_n6;
        locals.var_igse_dn7 = assign34750_e49776_d_n7;
        locals.var_igse_dn10 = assign34750_e49776_d_n10;
        locals.var_igse_dn11 = assign34750_e49776_d_n11;
        locals.var_igse_dn12 = assign34750_e49776_d_n12;
        locals.var_igse_dn17 = assign34750_e49776_d_n17;

        let (assign34760_e49787, assign34760_e49787_d_n0, assign34760_e49787_d_n2, assign34760_e49787_d_n6, assign34760_e49787_d_n7, assign34760_e49787_d_n10, assign34760_e49787_d_n11, assign34760_e49787_d_n12, assign34760_e49787_d_n17,) = {
    if (locals.var_guard1140 == 0.0) {
        let assign34760_e49782: f64 = (locals.var_glpart1 * locals.var_igate);
        let assign34760_e49784: f64 = (assign34760_e49782 - locals.var_igd);
        let assign34760_e49785: f64 = (locals.var_mfactor * assign34760_e49784);
        (assign34760_e49785, (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn0) - locals.var_igd_dn0)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn2) - locals.var_igd_dn2)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn6) - locals.var_igd_dn6)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn7) - locals.var_igd_dn7)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn10) - locals.var_igd_dn10)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn11) - locals.var_igd_dn11)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn12) - locals.var_igd_dn12)), (locals.var_mfactor * ((locals.var_glpart1 * locals.var_igate_dn17) - locals.var_igd_dn17)),)
    } else {
        (locals.var_igse, locals.var_igse_dn0, locals.var_igse_dn2, locals.var_igse_dn6, locals.var_igse_dn7, locals.var_igse_dn10, locals.var_igse_dn11, locals.var_igse_dn12, locals.var_igse_dn17,)
    }
};
        locals.var_igse = assign34760_e49787;
        locals.var_igse_dn0 = assign34760_e49787_d_n0;
        locals.var_igse_dn2 = assign34760_e49787_d_n2;
        locals.var_igse_dn6 = assign34760_e49787_d_n6;
        locals.var_igse_dn7 = assign34760_e49787_d_n7;
        locals.var_igse_dn10 = assign34760_e49787_d_n10;
        locals.var_igse_dn11 = assign34760_e49787_d_n11;
        locals.var_igse_dn12 = assign34760_e49787_d_n12;
        locals.var_igse_dn17 = assign34760_e49787_d_n17;

        let (assign34770_e49797, assign34770_e49797_d_n0, assign34770_e49797_d_n2, assign34770_e49797_d_n6, assign34770_e49797_d_n7, assign34770_e49797_d_n10, assign34770_e49797_d_n11, assign34770_e49797_d_n12, assign34770_e49797_d_n17,) = {
    if (locals.var_mode == 1.0) {
        let assign34770_e49793: f64 = (locals.var_mfactor * locals.var_igidl);
        (assign34770_e49793, (locals.var_mfactor * locals.var_igidl_dn0), (locals.var_mfactor * locals.var_igidl_dn2), (locals.var_mfactor * locals.var_igidl_dn6), (locals.var_mfactor * locals.var_igidl_dn7), (locals.var_mfactor * locals.var_igidl_dn10), (locals.var_mfactor * locals.var_igidl_dn11), (locals.var_mfactor * locals.var_igidl_dn12), (locals.var_mfactor * locals.var_igidl_dn17),)
    } else {
        let assign34770_e49796: f64 = (locals.var_mfactor * locals.var_igisl);
        (assign34770_e49796, (locals.var_mfactor * locals.var_igisl_dn0), (locals.var_mfactor * locals.var_igisl_dn2), (locals.var_mfactor * locals.var_igisl_dn6), (locals.var_mfactor * locals.var_igisl_dn7), (locals.var_mfactor * locals.var_igisl_dn10), (locals.var_mfactor * locals.var_igisl_dn11), (locals.var_mfactor * locals.var_igisl_dn12), (locals.var_mfactor * locals.var_igisl_dn17),)
    }
};
        locals.var_igidle = assign34770_e49797;
        locals.var_igidle_dn0 = assign34770_e49797_d_n0;
        locals.var_igidle_dn2 = assign34770_e49797_d_n2;
        locals.var_igidle_dn6 = assign34770_e49797_d_n6;
        locals.var_igidle_dn7 = assign34770_e49797_d_n7;
        locals.var_igidle_dn10 = assign34770_e49797_d_n10;
        locals.var_igidle_dn11 = assign34770_e49797_d_n11;
        locals.var_igidle_dn12 = assign34770_e49797_d_n12;
        locals.var_igidle_dn17 = assign34770_e49797_d_n17;

        let (assign34780_e49807, assign34780_e49807_d_n0, assign34780_e49807_d_n2, assign34780_e49807_d_n6, assign34780_e49807_d_n7, assign34780_e49807_d_n10, assign34780_e49807_d_n11, assign34780_e49807_d_n12, assign34780_e49807_d_n17,) = {
    if (locals.var_mode == 1.0) {
        let assign34780_e49803: f64 = (locals.var_mfactor * locals.var_igisl);
        (assign34780_e49803, (locals.var_mfactor * locals.var_igisl_dn0), (locals.var_mfactor * locals.var_igisl_dn2), (locals.var_mfactor * locals.var_igisl_dn6), (locals.var_mfactor * locals.var_igisl_dn7), (locals.var_mfactor * locals.var_igisl_dn10), (locals.var_mfactor * locals.var_igisl_dn11), (locals.var_mfactor * locals.var_igisl_dn12), (locals.var_mfactor * locals.var_igisl_dn17),)
    } else {
        let assign34780_e49806: f64 = (locals.var_mfactor * locals.var_igidl);
        (assign34780_e49806, (locals.var_mfactor * locals.var_igidl_dn0), (locals.var_mfactor * locals.var_igidl_dn2), (locals.var_mfactor * locals.var_igidl_dn6), (locals.var_mfactor * locals.var_igidl_dn7), (locals.var_mfactor * locals.var_igidl_dn10), (locals.var_mfactor * locals.var_igidl_dn11), (locals.var_mfactor * locals.var_igidl_dn12), (locals.var_mfactor * locals.var_igidl_dn17),)
    }
};
        locals.var_igisle = assign34780_e49807;
        locals.var_igisle_dn0 = assign34780_e49807_d_n0;
        locals.var_igisle_dn2 = assign34780_e49807_d_n2;
        locals.var_igisle_dn6 = assign34780_e49807_d_n6;
        locals.var_igisle_dn7 = assign34780_e49807_d_n7;
        locals.var_igisle_dn10 = assign34780_e49807_d_n10;
        locals.var_igisle_dn11 = assign34780_e49807_d_n11;
        locals.var_igisle_dn12 = assign34780_e49807_d_n12;
        locals.var_igisle_dn17 = assign34780_e49807_d_n17;

        let assign34800_e49813: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign34800_e49813;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn7 = (locals.var_mfactor * locals.var_nthrml_dn7);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn11 = (locals.var_mfactor * locals.var_nthrml_dn11);
        locals.var_noithrml_dn12 = (locals.var_mfactor * locals.var_nthrml_dn12);
        locals.var_noithrml_dn17 = (locals.var_mfactor * locals.var_nthrml_dn17);

        let assign34810_e49816: f64 = locals.var_qge_dn6;
        locals.var_cgdbd = assign34810_e49816;
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

        let assign34820_e49819: f64 = (p.p50 * locals.var_cgdbd);
        locals.var_cgdbd = assign34820_e49819;
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

        let assign34830_e49822: f64 = locals.var_qge_dn7;
        locals.var_cgsbd = assign34830_e49822;
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

        let assign34840_e49825: f64 = (p.p50 * locals.var_cgsbd);
        locals.var_cgsbd = assign34840_e49825;
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

        let (assign34850_e49831, assign34850_e49831_d_n0, assign34850_e49831_d_n2, assign34850_e49831_d_n6, assign34850_e49831_d_n7, assign34850_e49831_d_n10, assign34850_e49831_d_n11, assign34850_e49831_d_n12, assign34850_e49831_d_n13, assign34850_e49831_d_n15, assign34850_e49831_d_n16, assign34850_e49831_d_n17, assign34850_e49831_d_n18,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18,)
    }
};
        locals.var_cgsb = assign34850_e49831;
        locals.var_cgsb_dn0 = assign34850_e49831_d_n0;
        locals.var_cgsb_dn2 = assign34850_e49831_d_n2;
        locals.var_cgsb_dn6 = assign34850_e49831_d_n6;
        locals.var_cgsb_dn7 = assign34850_e49831_d_n7;
        locals.var_cgsb_dn10 = assign34850_e49831_d_n10;
        locals.var_cgsb_dn11 = assign34850_e49831_d_n11;
        locals.var_cgsb_dn12 = assign34850_e49831_d_n12;
        locals.var_cgsb_dn13 = assign34850_e49831_d_n13;
        locals.var_cgsb_dn15 = assign34850_e49831_d_n15;
        locals.var_cgsb_dn16 = assign34850_e49831_d_n16;
        locals.var_cgsb_dn17 = assign34850_e49831_d_n17;
        locals.var_cgsb_dn18 = assign34850_e49831_d_n18;

        let assign34860_e49845: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1147 = assign34860_e49845;

        let (assign34870_e49855,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34870_e49849: f64 = (1e-6 * locals.var_c_fox);
        let assign34870_e49851: f64 = (assign34870_e49849 * locals.var_weffcv_nf);
        let assign34870_e49853: f64 = (assign34870_e49851 * locals.var_leff_cv);
        (assign34870_e49853,)
    } else {
        (locals.var_t0__blk1141,)
    }
};
        locals.var_t0__blk1141 = assign34870_e49855;

        let (assign34880_e49861, assign34880_e49861_d_n0, assign34880_e49861_d_n2, assign34880_e49861_d_n6, assign34880_e49861_d_n7, assign34880_e49861_d_n10, assign34880_e49861_d_n11, assign34880_e49861_d_n12, assign34880_e49861_d_n13, assign34880_e49861_d_n15, assign34880_e49861_d_n16, assign34880_e49861_d_n17, assign34880_e49861_d_n18,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34880_e49859: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign34880_e49859, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn12 / locals.var_mfactor), (locals.var_cgsb_dn13 / locals.var_mfactor), (locals.var_cgsb_dn15 / locals.var_mfactor), (locals.var_cgsb_dn16 / locals.var_mfactor), (locals.var_cgsb_dn17 / locals.var_mfactor), (locals.var_cgsb_dn18 / locals.var_mfactor),)
    } else {
        (locals.var_t1__blk1142, locals.var_t1__blk1142_dn0, locals.var_t1__blk1142_dn2, locals.var_t1__blk1142_dn6, locals.var_t1__blk1142_dn7, locals.var_t1__blk1142_dn10, locals.var_t1__blk1142_dn11, locals.var_t1__blk1142_dn12, locals.var_t1__blk1142_dn13, locals.var_t1__blk1142_dn15, locals.var_t1__blk1142_dn16, locals.var_t1__blk1142_dn17, locals.var_t1__blk1142_dn18,)
    }
};
        locals.var_t1__blk1142 = assign34880_e49861;
        locals.var_t1__blk1142_dn0 = assign34880_e49861_d_n0;
        locals.var_t1__blk1142_dn2 = assign34880_e49861_d_n2;
        locals.var_t1__blk1142_dn6 = assign34880_e49861_d_n6;
        locals.var_t1__blk1142_dn7 = assign34880_e49861_d_n7;
        locals.var_t1__blk1142_dn10 = assign34880_e49861_d_n10;
        locals.var_t1__blk1142_dn11 = assign34880_e49861_d_n11;
        locals.var_t1__blk1142_dn12 = assign34880_e49861_d_n12;
        locals.var_t1__blk1142_dn13 = assign34880_e49861_d_n13;
        locals.var_t1__blk1142_dn15 = assign34880_e49861_d_n15;
        locals.var_t1__blk1142_dn16 = assign34880_e49861_d_n16;
        locals.var_t1__blk1142_dn17 = assign34880_e49861_d_n17;
        locals.var_t1__blk1142_dn18 = assign34880_e49861_d_n18;

        let (assign34890_e49875, assign34890_e49875_d_n0, assign34890_e49875_d_n2, assign34890_e49875_d_n6, assign34890_e49875_d_n7, assign34890_e49875_d_n10, assign34890_e49875_d_n11, assign34890_e49875_d_n12, assign34890_e49875_d_n13, assign34890_e49875_d_n15, assign34890_e49875_d_n16, assign34890_e49875_d_n17, assign34890_e49875_d_n18,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34890_e49865: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign34890_e49867: f64 = (assign34890_e49865 * locals.var_beta_inv);
        let assign34890_e49869: f64 = (assign34890_e49867 * locals.var_t1__blk1142);
        let assign34890_e49871: f64 = (assign34890_e49869 * locals.var_t1__blk1142);
        let assign34890_e49873: f64 = (assign34890_e49871 / locals.var_gds0_ign);
        (assign34890_e49873, ((((((assign34890_e49867 * locals.var_t1__blk1142_dn0) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn0)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn2) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn2)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn6) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn6)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn7) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn7)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign34890_e49865 * locals.var_beta_inv_dn10) * locals.var_t1__blk1142) + (assign34890_e49867 * locals.var_t1__blk1142_dn10)) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn10)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn11) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn11)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn12) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn12)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn12)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34890_e49867 * locals.var_t1__blk1142_dn13) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn13)) / locals.var_gds0_ign), ((((assign34890_e49867 * locals.var_t1__blk1142_dn15) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn15)) / locals.var_gds0_ign), ((((assign34890_e49867 * locals.var_t1__blk1142_dn16) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn16)) / locals.var_gds0_ign), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn17) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn17)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn17)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34890_e49867 * locals.var_t1__blk1142_dn18) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn18)) / locals.var_gds0_ign),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn12, locals.var_nign0_dn13, locals.var_nign0_dn15, locals.var_nign0_dn16, locals.var_nign0_dn17, locals.var_nign0_dn18,)
    }
};
        locals.var_nign0 = assign34890_e49875;
        locals.var_nign0_dn0 = assign34890_e49875_d_n0;
        locals.var_nign0_dn2 = assign34890_e49875_d_n2;
        locals.var_nign0_dn6 = assign34890_e49875_d_n6;
        locals.var_nign0_dn7 = assign34890_e49875_d_n7;
        locals.var_nign0_dn10 = assign34890_e49875_d_n10;
        locals.var_nign0_dn11 = assign34890_e49875_d_n11;
        locals.var_nign0_dn12 = assign34890_e49875_d_n12;
        locals.var_nign0_dn13 = assign34890_e49875_d_n13;
        locals.var_nign0_dn15 = assign34890_e49875_d_n15;
        locals.var_nign0_dn16 = assign34890_e49875_d_n16;
        locals.var_nign0_dn17 = assign34890_e49875_d_n17;
        locals.var_nign0_dn18 = assign34890_e49875_d_n18;

        let assign34900_e49879: f64 = (10.0 * 2.220446049250313e-16);
        let assign34900_e49884: f64 = (10.0 * 2.220446049250313e-16);
        let assign34900_e49886: f64 = if ((locals.var_kusai00l > assign34900_e49879) && (locals.var_vds > assign34900_e49884)) { 1.0 } else { 0.0 };
        locals.var_guard1148 = assign34900_e49886;

        let (assign34910_e49894, assign34910_e49894_d_n0, assign34910_e49894_d_n2, assign34910_e49894_d_n6, assign34910_e49894_d_n7, assign34910_e49894_d_n10, assign34910_e49894_d_n11, assign34910_e49894_d_n12, assign34910_e49894_d_n17,) = {
    if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 != 0.0)) {
        let assign34910_e49892: f64 = (locals.var_muun / locals.var_mu);
        (assign34910_e49892, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn12 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn12)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn17 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn17)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn12, locals.var_mumoda_dn17,)
    }
};
        locals.var_mumoda = assign34910_e49894;
        locals.var_mumoda_dn0 = assign34910_e49894_d_n0;
        locals.var_mumoda_dn2 = assign34910_e49894_d_n2;
        locals.var_mumoda_dn6 = assign34910_e49894_d_n6;
        locals.var_mumoda_dn7 = assign34910_e49894_d_n7;
        locals.var_mumoda_dn10 = assign34910_e49894_d_n10;
        locals.var_mumoda_dn11 = assign34910_e49894_d_n11;
        locals.var_mumoda_dn12 = assign34910_e49894_d_n12;
        locals.var_mumoda_dn17 = assign34910_e49894_d_n17;

        let (assign34920_e49906, assign34920_e49906_d_n0, assign34920_e49906_d_n2, assign34920_e49906_d_n6, assign34920_e49906_d_n7, assign34920_e49906_d_n10, assign34920_e49906_d_n11, assign34920_e49906_d_n12, assign34920_e49906_d_n17,) = {
    if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 != 0.0)) {
        let assign34920_e49900: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign34920_e49902: f64 = (assign34920_e49900 - locals.var_mumoda);
        let assign34920_e49904: f64 = (assign34920_e49902 / locals.var_vds);
        (assign34920_e49904, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn12) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn12)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn17) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn17)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn12, locals.var_mumodb_dn17,)
    }
};
        locals.var_mumodb = assign34920_e49906;
        locals.var_mumodb_dn0 = assign34920_e49906_d_n0;
        locals.var_mumodb_dn2 = assign34920_e49906_d_n2;
        locals.var_mumodb_dn6 = assign34920_e49906_d_n6;
        locals.var_mumodb_dn7 = assign34920_e49906_d_n7;
        locals.var_mumodb_dn10 = assign34920_e49906_d_n10;
        locals.var_mumodb_dn11 = assign34920_e49906_d_n11;
        locals.var_mumodb_dn12 = assign34920_e49906_d_n12;
        locals.var_mumodb_dn17 = assign34920_e49906_d_n17;

        let (assign34930_e49928, assign34930_e49928_d_n0, assign34930_e49928_d_n2, assign34930_e49928_d_n6, assign34930_e49928_d_n7, assign34930_e49928_d_n10, assign34930_e49928_d_n11, assign34930_e49928_d_n12, assign34930_e49928_d_n17,) = {
    if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 != 0.0)) {
        let assign34930_e49913: f64 = (0.6666666666666667 * locals.var_mumodb);
        let assign34930_e49917: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
        let assign34930_e49918: f64 = (locals.var_kusai00 + assign34930_e49917);
        let assign34930_e49920: f64 = (assign34930_e49918 + locals.var_kusail);
        let assign34930_e49921: f64 = (assign34930_e49913 * assign34930_e49920);
        let assign34930_e49924: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        let assign34930_e49925: f64 = (assign34930_e49921 / assign34930_e49924);
        let assign34930_e49926: f64 = (locals.var_mumoda + assign34930_e49925);
        (assign34930_e49926, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn12 + ((((((0.6666666666666667 * locals.var_mumodb_dn12) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn12 + ((locals.var_vgvt_dn12 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn17 + ((((((0.6666666666666667 * locals.var_mumodb_dn17) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn17 + ((locals.var_vgvt_dn17 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn17))) + locals.var_kusail_dn17))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17))) / (assign34930_e49924 * assign34930_e49924))),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17,)
    }
};
        locals.var_correct_w1 = assign34930_e49928;
        locals.var_correct_w1_dn0 = assign34930_e49928_d_n0;
        locals.var_correct_w1_dn2 = assign34930_e49928_d_n2;
        locals.var_correct_w1_dn6 = assign34930_e49928_d_n6;
        locals.var_correct_w1_dn7 = assign34930_e49928_d_n7;
        locals.var_correct_w1_dn10 = assign34930_e49928_d_n10;
        locals.var_correct_w1_dn11 = assign34930_e49928_d_n11;
        locals.var_correct_w1_dn12 = assign34930_e49928_d_n12;
        locals.var_correct_w1_dn17 = assign34930_e49928_d_n17;

        let (assign34940_e49937, assign34940_e49937_d_n0, assign34940_e49937_d_n2, assign34940_e49937_d_n6, assign34940_e49937_d_n7, assign34940_e49937_d_n10, assign34940_e49937_d_n11, assign34940_e49937_d_n12, assign34940_e49937_d_n17,) = {
    if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 == 0.0)) {
        let assign34940_e49935: f64 = (locals.var_muun / locals.var_mud_hoso);
        (assign34940_e49935, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17,)
    }
};
        locals.var_correct_w1 = assign34940_e49937;
        locals.var_correct_w1_dn0 = assign34940_e49937_d_n0;
        locals.var_correct_w1_dn2 = assign34940_e49937_d_n2;
        locals.var_correct_w1_dn6 = assign34940_e49937_d_n6;
        locals.var_correct_w1_dn7 = assign34940_e49937_d_n7;
        locals.var_correct_w1_dn10 = assign34940_e49937_d_n10;
        locals.var_correct_w1_dn11 = assign34940_e49937_d_n11;
        locals.var_correct_w1_dn12 = assign34940_e49937_d_n12;
        locals.var_correct_w1_dn17 = assign34940_e49937_d_n17;

        let (assign34950_e49947, assign34950_e49947_d_n0, assign34950_e49947_d_n2, assign34950_e49947_d_n6, assign34950_e49947_d_n7, assign34950_e49947_d_n10, assign34950_e49947_d_n11, assign34950_e49947_d_n12, assign34950_e49947_d_n13, assign34950_e49947_d_n15, assign34950_e49947_d_n16, assign34950_e49947_d_n17, assign34950_e49947_d_n18,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34950_e49941: f64 = (locals.var_mfactor * locals.var_nign0);
        let assign34950_e49943: f64 = (assign34950_e49941 * locals.var_kusai_ig);
        let assign34950_e49945: f64 = (assign34950_e49943 * locals.var_correct_w1);
        (assign34950_e49945, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn12) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn12)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn12)), (((locals.var_mfactor * locals.var_nign0_dn13) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn15) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn16) * locals.var_kusai_ig) * locals.var_correct_w1), (((((locals.var_mfactor * locals.var_nign0_dn17) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn17)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn17)), (((locals.var_mfactor * locals.var_nign0_dn18) * locals.var_kusai_ig) * locals.var_correct_w1),)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign34950_e49947;
        locals.var_noiigate_dn0 = assign34950_e49947_d_n0;
        locals.var_noiigate_dn2 = assign34950_e49947_d_n2;
        locals.var_noiigate_dn6 = assign34950_e49947_d_n6;
        locals.var_noiigate_dn7 = assign34950_e49947_d_n7;
        locals.var_noiigate_dn10 = assign34950_e49947_d_n10;
        locals.var_noiigate_dn11 = assign34950_e49947_d_n11;
        locals.var_noiigate_dn12 = assign34950_e49947_d_n12;
        locals.var_noiigate_dn13 = assign34950_e49947_d_n13;
        locals.var_noiigate_dn15 = assign34950_e49947_d_n15;
        locals.var_noiigate_dn16 = assign34950_e49947_d_n16;
        locals.var_noiigate_dn17 = assign34950_e49947_d_n17;
        locals.var_noiigate_dn18 = assign34950_e49947_d_n18;

        let (assign34960_e49951, assign34960_e49951_d_n0, assign34960_e49951_d_n2, assign34960_e49951_d_n6, assign34960_e49951_d_n7, assign34960_e49951_d_n10, assign34960_e49951_d_n11, assign34960_e49951_d_n12, assign34960_e49951_d_n17,) = {
    if (locals.var_guard1147 != 0.0) {
        (locals.var_crl_f, locals.var_crl_f_dn0, locals.var_crl_f_dn2, locals.var_crl_f_dn6, locals.var_crl_f_dn7, locals.var_crl_f_dn10, locals.var_crl_f_dn11, locals.var_crl_f_dn12, locals.var_crl_f_dn17,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn12, locals.var_noicross_dn17,)
    }
};
        locals.var_noicross = assign34960_e49951;
        locals.var_noicross_dn0 = assign34960_e49951_d_n0;
        locals.var_noicross_dn2 = assign34960_e49951_d_n2;
        locals.var_noicross_dn6 = assign34960_e49951_d_n6;
        locals.var_noicross_dn7 = assign34960_e49951_d_n7;
        locals.var_noicross_dn10 = assign34960_e49951_d_n10;
        locals.var_noicross_dn11 = assign34960_e49951_d_n11;
        locals.var_noicross_dn12 = assign34960_e49951_d_n12;
        locals.var_noicross_dn17 = assign34960_e49951_d_n17;

        let (assign34970_e49965, assign34970_e49965_d_n0, assign34970_e49965_d_n2, assign34970_e49965_d_n6, assign34970_e49965_d_n7, assign34970_e49965_d_n10, assign34970_e49965_d_n11, assign34970_e49965_d_n12, assign34970_e49965_d_n13, assign34970_e49965_d_n15, assign34970_e49965_d_n16, assign34970_e49965_d_n17, assign34970_e49965_d_n18,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34970_e49954: f64 = (-locals.var_t1__blk1142);
        let (assign34970_e49963, assign34970_e49963_d_n0, assign34970_e49963_d_n2, assign34970_e49963_d_n6, assign34970_e49963_d_n7, assign34970_e49963_d_n10, assign34970_e49963_d_n11, assign34970_e49963_d_n12, assign34970_e49963_d_n13, assign34970_e49963_d_n15, assign34970_e49963_d_n16, assign34970_e49963_d_n17, assign34970_e49963_d_n18,) = {
            if ((assign34970_e49954 > locals.var_t0__blk1141) && (locals.var_noiigate > 0.0)) {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign34970_e49963, assign34970_e49963_d_n0, assign34970_e49963_d_n2, assign34970_e49963_d_n6, assign34970_e49963_d_n7, assign34970_e49963_d_n10, assign34970_e49963_d_n11, assign34970_e49963_d_n12, assign34970_e49963_d_n13, assign34970_e49963_d_n15, assign34970_e49963_d_n16, assign34970_e49963_d_n17, assign34970_e49963_d_n18,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign34970_e49965;
        locals.var_noiigate_dn0 = assign34970_e49965_d_n0;
        locals.var_noiigate_dn2 = assign34970_e49965_d_n2;
        locals.var_noiigate_dn6 = assign34970_e49965_d_n6;
        locals.var_noiigate_dn7 = assign34970_e49965_d_n7;
        locals.var_noiigate_dn10 = assign34970_e49965_d_n10;
        locals.var_noiigate_dn11 = assign34970_e49965_d_n11;
        locals.var_noiigate_dn12 = assign34970_e49965_d_n12;
        locals.var_noiigate_dn13 = assign34970_e49965_d_n13;
        locals.var_noiigate_dn15 = assign34970_e49965_d_n15;
        locals.var_noiigate_dn16 = assign34970_e49965_d_n16;
        locals.var_noiigate_dn17 = assign34970_e49965_d_n17;
        locals.var_noiigate_dn18 = assign34970_e49965_d_n18;

        let (assign34980_e49975, assign34980_e49975_d_n0, assign34980_e49975_d_n2, assign34980_e49975_d_n6, assign34980_e49975_d_n7, assign34980_e49975_d_n10, assign34980_e49975_d_n11, assign34980_e49975_d_n12, assign34980_e49975_d_n17,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34980_e49968: f64 = (-locals.var_t1__blk1142);
        let (assign34980_e49973, assign34980_e49973_d_n0, assign34980_e49973_d_n2, assign34980_e49973_d_n6, assign34980_e49973_d_n7, assign34980_e49973_d_n10, assign34980_e49973_d_n11, assign34980_e49973_d_n12, assign34980_e49973_d_n17,) = {
            if (assign34980_e49968 > locals.var_t0__blk1141) {
                (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn12, locals.var_noicross_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign34980_e49973, assign34980_e49973_d_n0, assign34980_e49973_d_n2, assign34980_e49973_d_n6, assign34980_e49973_d_n7, assign34980_e49973_d_n10, assign34980_e49973_d_n11, assign34980_e49973_d_n12, assign34980_e49973_d_n17,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn12, locals.var_noicross_dn17,)
    }
};
        locals.var_noicross = assign34980_e49975;
        locals.var_noicross_dn0 = assign34980_e49975_d_n0;
        locals.var_noicross_dn2 = assign34980_e49975_d_n2;
        locals.var_noicross_dn6 = assign34980_e49975_d_n6;
        locals.var_noicross_dn7 = assign34980_e49975_d_n7;
        locals.var_noicross_dn10 = assign34980_e49975_d_n10;
        locals.var_noicross_dn11 = assign34980_e49975_d_n11;
        locals.var_noicross_dn12 = assign34980_e49975_d_n12;
        locals.var_noicross_dn17 = assign34980_e49975_d_n17;

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
        let (assign34990_e49980, assign34990_e49980_d_n0, assign34990_e49980_d_n2, assign34990_e49980_d_n6, assign34990_e49980_d_n7, assign34990_e49980_d_n10, assign34990_e49980_d_n11, assign34990_e49980_d_n12, assign34990_e49980_d_n13, assign34990_e49980_d_n15, assign34990_e49980_d_n16, assign34990_e49980_d_n17, assign34990_e49980_d_n18,) = {
    if (locals.var_guard1147 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign34990_e49980;
        locals.var_noiigate_dn0 = assign34990_e49980_d_n0;
        locals.var_noiigate_dn2 = assign34990_e49980_d_n2;
        locals.var_noiigate_dn6 = assign34990_e49980_d_n6;
        locals.var_noiigate_dn7 = assign34990_e49980_d_n7;
        locals.var_noiigate_dn10 = assign34990_e49980_d_n10;
        locals.var_noiigate_dn11 = assign34990_e49980_d_n11;
        locals.var_noiigate_dn12 = assign34990_e49980_d_n12;
        locals.var_noiigate_dn13 = assign34990_e49980_d_n13;
        locals.var_noiigate_dn15 = assign34990_e49980_d_n15;
        locals.var_noiigate_dn16 = assign34990_e49980_d_n16;
        locals.var_noiigate_dn17 = assign34990_e49980_d_n17;
        locals.var_noiigate_dn18 = assign34990_e49980_d_n18;

        let (assign35000_e49985, assign35000_e49985_d_n0, assign35000_e49985_d_n2, assign35000_e49985_d_n6, assign35000_e49985_d_n7, assign35000_e49985_d_n10, assign35000_e49985_d_n11, assign35000_e49985_d_n12, assign35000_e49985_d_n17,) = {
    if (locals.var_guard1147 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
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

        let assign35050_e49992: f64 = if p.p259 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1149 = assign35050_e49992;

        let (assign35060_e49996,) = {
    if (locals.var_guard1149 != 0.0) {
        (1.0,)
    } else {
        (locals.var_rdmod,)
    }
};
        locals.var_rdmod = assign35060_e49996;

        let assign35070_e49999: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1169 = assign35070_e49999;

        let (assign35090_e50013,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
        (p.p266,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35090_e50013;

        let (assign35100_e50019,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
        (p.p268,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35100_e50019;

        let (assign35110_e50025, assign35110_e50025_d_n10,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35110_e50025;
        locals.var_rrdrbb_dn10 = assign35110_e50025_d_n10;

        let (assign35130_e50044,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
        (p.p258,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign35130_e50044;

        let (assign35140_e50052, assign35140_e50052_d_n0, assign35140_e50052_d_n2, assign35140_e50052_d_n6, assign35140_e50052_d_n7,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
        let assign35140_e50050: f64 = (p.p50 * (nv7 - nv2));
        (assign35140_e50050, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7,)
    }
};
        locals.var_vrdr = assign35140_e50052;
        locals.var_vrdr_dn0 = assign35140_e50052_d_n0;
        locals.var_vrdr_dn2 = assign35140_e50052_d_n2;
        locals.var_vrdr_dn6 = assign35140_e50052_d_n6;
        locals.var_vrdr_dn7 = assign35140_e50052_d_n7;

        let (assign35160_e50068,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
        (p.p265,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35160_e50068;

        let (assign35170_e50075,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
        (p.p267,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35170_e50075;

        let (assign35180_e50082, assign35180_e50082_d_n10,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35180_e50082;
        locals.var_rrdrbb_dn10 = assign35180_e50082_d_n10;

        let (assign35200_e50103,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
        (p.p257,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign35200_e50103;

        let (assign35210_e50112, assign35210_e50112_d_n0, assign35210_e50112_d_n2, assign35210_e50112_d_n6, assign35210_e50112_d_n7,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
        let assign35210_e50110: f64 = (p.p50 * (nv0 - nv6));
        (assign35210_e50110, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7,)
    }
};
        locals.var_vrdr = assign35210_e50112;
        locals.var_vrdr_dn0 = assign35210_e50112_d_n0;
        locals.var_vrdr_dn2 = assign35210_e50112_d_n2;
        locals.var_vrdr_dn6 = assign35210_e50112_d_n6;
        locals.var_vrdr_dn7 = assign35210_e50112_d_n7;

        let (assign35240_e50135,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35240_e50133: f64 = (locals.var_mks_rdrmue / 10000.0);
        (assign35240_e50133,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35240_e50135;

        let (assign35250_e50141,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35250_e50139: f64 = (locals.var_mks_rdrvmax / 100.0);
        (assign35250_e50139,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35250_e50141;

        let (assign35260_e50147, assign35260_e50147_d_n10,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35260_e50145: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign35260_e50145, (locals.var_ttemp_dn10 / locals.var_uc_tnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn10,)
    }
};
        locals.var_tratio = assign35260_e50147;
        locals.var_tratio_dn10 = assign35260_e50147_d_n10;

        let (assign35270_e50153, assign35270_e50153_d_n0, assign35270_e50153_d_n2, assign35270_e50153_d_n6, assign35270_e50153_d_n7, assign35270_e50153_d_n10, assign35270_e50153_d_n11, assign35270_e50153_d_n12, assign35270_e50153_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35270_e50151: f64 = (locals.var_tratio).powf(p.p269);
        (assign35270_e50151, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio).powf(p.p269 - 1.0) * locals.var_tratio_dn10)) } } else { (assign35270_e50151 * (p.p269 * (locals.var_tratio_dn10 / locals.var_tratio))) }, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35270_e50153;
        locals.var_t1_dn0 = assign35270_e50153_d_n0;
        locals.var_t1_dn2 = assign35270_e50153_d_n2;
        locals.var_t1_dn6 = assign35270_e50153_d_n6;
        locals.var_t1_dn7 = assign35270_e50153_d_n7;
        locals.var_t1_dn10 = assign35270_e50153_d_n10;
        locals.var_t1_dn11 = assign35270_e50153_d_n11;
        locals.var_t1_dn12 = assign35270_e50153_d_n12;
        locals.var_t1_dn17 = assign35270_e50153_d_n17;

        let (assign35280_e50159, assign35280_e50159_d_n0, assign35280_e50159_d_n2, assign35280_e50159_d_n6, assign35280_e50159_d_n7, assign35280_e50159_d_n10, assign35280_e50159_d_n11, assign35280_e50159_d_n12, assign35280_e50159_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35280_e50157: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign35280_e50157, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17,)
    }
};
        locals.var_mu0 = assign35280_e50159;
        locals.var_mu0_dn0 = assign35280_e50159_d_n0;
        locals.var_mu0_dn2 = assign35280_e50159_d_n2;
        locals.var_mu0_dn6 = assign35280_e50159_d_n6;
        locals.var_mu0_dn7 = assign35280_e50159_d_n7;
        locals.var_mu0_dn10 = assign35280_e50159_d_n10;
        locals.var_mu0_dn11 = assign35280_e50159_d_n11;
        locals.var_mu0_dn12 = assign35280_e50159_d_n12;
        locals.var_mu0_dn17 = assign35280_e50159_d_n17;

        let (assign35290_e50179, assign35290_e50179_d_n0, assign35290_e50179_d_n2, assign35290_e50179_d_n6, assign35290_e50179_d_n7, assign35290_e50179_d_n10, assign35290_e50179_d_n11, assign35290_e50179_d_n12, assign35290_e50179_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35290_e50164: f64 = (0.4 * locals.var_tratio);
        let assign35290_e50165: f64 = (1.8 + assign35290_e50164);
        let assign35290_e50168: f64 = (0.1 * locals.var_tratio);
        let assign35290_e50170: f64 = (assign35290_e50168 * locals.var_tratio);
        let assign35290_e50171: f64 = (assign35290_e50165 + assign35290_e50170);
        let assign35290_e50175: f64 = (1.0 - locals.var_tratio);
        let assign35290_e50176: f64 = (p.p270 * assign35290_e50175);
        let assign35290_e50177: f64 = (assign35290_e50171 - assign35290_e50176);
        (assign35290_e50177, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign35290_e50168 * locals.var_tratio_dn10))) - (p.p270 * (-locals.var_tratio_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign35290_e50179;
        locals.var_t0_dn0 = assign35290_e50179_d_n0;
        locals.var_t0_dn2 = assign35290_e50179_d_n2;
        locals.var_t0_dn6 = assign35290_e50179_d_n6;
        locals.var_t0_dn7 = assign35290_e50179_d_n7;
        locals.var_t0_dn10 = assign35290_e50179_d_n10;
        locals.var_t0_dn11 = assign35290_e50179_d_n11;
        locals.var_t0_dn12 = assign35290_e50179_d_n12;
        locals.var_t0_dn17 = assign35290_e50179_d_n17;

        let (assign35300_e50185, assign35300_e50185_d_n0, assign35300_e50185_d_n2, assign35300_e50185_d_n6, assign35300_e50185_d_n7, assign35300_e50185_d_n10, assign35300_e50185_d_n11, assign35300_e50185_d_n12, assign35300_e50185_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35300_e50183: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign35300_e50183, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_vmaxe__blk1162, locals.var_vmaxe__blk1162_dn0, locals.var_vmaxe__blk1162_dn2, locals.var_vmaxe__blk1162_dn6, locals.var_vmaxe__blk1162_dn7, locals.var_vmaxe__blk1162_dn10, locals.var_vmaxe__blk1162_dn11, locals.var_vmaxe__blk1162_dn12, locals.var_vmaxe__blk1162_dn17,)
    }
};
        locals.var_vmaxe__blk1162 = assign35300_e50185;
        locals.var_vmaxe__blk1162_dn0 = assign35300_e50185_d_n0;
        locals.var_vmaxe__blk1162_dn2 = assign35300_e50185_d_n2;
        locals.var_vmaxe__blk1162_dn6 = assign35300_e50185_d_n6;
        locals.var_vmaxe__blk1162_dn7 = assign35300_e50185_d_n7;
        locals.var_vmaxe__blk1162_dn10 = assign35300_e50185_d_n10;
        locals.var_vmaxe__blk1162_dn11 = assign35300_e50185_d_n11;
        locals.var_vmaxe__blk1162_dn12 = assign35300_e50185_d_n12;
        locals.var_vmaxe__blk1162_dn17 = assign35300_e50185_d_n17;

        let (assign35310_e50195, assign35310_e50195_d_n10,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35310_e50191: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        let assign35310_e50192: f64 = (p.p274 * assign35310_e50191);
        let assign35310_e50193: f64 = (locals.var_rrdrbb + assign35310_e50192);
        (assign35310_e50193, (locals.var_rrdrbb_dn10 + (p.p274 * locals.var_ttemp_dn10)),)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35310_e50195;
        locals.var_rrdrbb_dn10 = assign35310_e50195_d_n10;

        let (assign35320_e50205,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35320_e50201: f64 = (locals.var_lgle).powf(p.p280);
        let assign35320_e50202: f64 = (p.p279 / assign35320_e50201);
        let assign35320_e50203: f64 = (1.0 + assign35320_e50202);
        (assign35320_e50203,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign35320_e50205;

        let (assign35330_e50215,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35330_e50211: f64 = (locals.var_lgle).powf(p.p278);
        let assign35330_e50212: f64 = (p.p277 / assign35330_e50211);
        let assign35330_e50213: f64 = (1.0 + assign35330_e50212);
        (assign35330_e50213,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign35330_e50215;

        let (assign35340_e50225,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35340_e50221: f64 = (locals.var_wg).powf(p.p276);
        let assign35340_e50222: f64 = (p.p275 / assign35340_e50221);
        let assign35340_e50223: f64 = (1.0 + assign35340_e50222);
        (assign35340_e50223,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign35340_e50225;

        let (assign35350_e50231, assign35350_e50231_d_n0, assign35350_e50231_d_n2, assign35350_e50231_d_n6, assign35350_e50231_d_n7, assign35350_e50231_d_n10, assign35350_e50231_d_n11, assign35350_e50231_d_n12, assign35350_e50231_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35350_e50229: f64 = (locals.var_mu0 * locals.var_rdrmuele);
        (assign35350_e50229, (locals.var_mu0_dn0 * locals.var_rdrmuele), (locals.var_mu0_dn2 * locals.var_rdrmuele), (locals.var_mu0_dn6 * locals.var_rdrmuele), (locals.var_mu0_dn7 * locals.var_rdrmuele), (locals.var_mu0_dn10 * locals.var_rdrmuele), (locals.var_mu0_dn11 * locals.var_rdrmuele), (locals.var_mu0_dn12 * locals.var_rdrmuele), (locals.var_mu0_dn17 * locals.var_rdrmuele),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17,)
    }
};
        locals.var_mu0 = assign35350_e50231;
        locals.var_mu0_dn0 = assign35350_e50231_d_n0;
        locals.var_mu0_dn2 = assign35350_e50231_d_n2;
        locals.var_mu0_dn6 = assign35350_e50231_d_n6;
        locals.var_mu0_dn7 = assign35350_e50231_d_n7;
        locals.var_mu0_dn10 = assign35350_e50231_d_n10;
        locals.var_mu0_dn11 = assign35350_e50231_d_n11;
        locals.var_mu0_dn12 = assign35350_e50231_d_n12;
        locals.var_mu0_dn17 = assign35350_e50231_d_n17;

        let (assign35360_e50241, assign35360_e50241_d_n0, assign35360_e50241_d_n2, assign35360_e50241_d_n6, assign35360_e50241_d_n7, assign35360_e50241_d_n10, assign35360_e50241_d_n11, assign35360_e50241_d_n12, assign35360_e50241_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35360_e50235: f64 = (locals.var_vmaxe__blk1162 * locals.var_rdrvmaxwe);
        let assign35360_e50237: f64 = (assign35360_e50235 * locals.var_rdrvmaxle);
        let assign35360_e50239: f64 = (assign35360_e50237 + 1e-50);
        (assign35360_e50239, ((locals.var_vmaxe__blk1162_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn12 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn17 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe__blk1162, locals.var_vmaxe__blk1162_dn0, locals.var_vmaxe__blk1162_dn2, locals.var_vmaxe__blk1162_dn6, locals.var_vmaxe__blk1162_dn7, locals.var_vmaxe__blk1162_dn10, locals.var_vmaxe__blk1162_dn11, locals.var_vmaxe__blk1162_dn12, locals.var_vmaxe__blk1162_dn17,)
    }
};
        locals.var_vmaxe__blk1162 = assign35360_e50241;
        locals.var_vmaxe__blk1162_dn0 = assign35360_e50241_d_n0;
        locals.var_vmaxe__blk1162_dn2 = assign35360_e50241_d_n2;
        locals.var_vmaxe__blk1162_dn6 = assign35360_e50241_d_n6;
        locals.var_vmaxe__blk1162_dn7 = assign35360_e50241_d_n7;
        locals.var_vmaxe__blk1162_dn10 = assign35360_e50241_d_n10;
        locals.var_vmaxe__blk1162_dn11 = assign35360_e50241_d_n11;
        locals.var_vmaxe__blk1162_dn12 = assign35360_e50241_d_n12;
        locals.var_vmaxe__blk1162_dn17 = assign35360_e50241_d_n17;

        let (assign35370_e50247, assign35370_e50247_d_n0, assign35370_e50247_d_n2, assign35370_e50247_d_n6, assign35370_e50247_d_n7,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35370_e50245: f64 = (locals.var_vrdr / locals.var_ldrifte);
        (assign35370_e50245, (locals.var_vrdr_dn0 / locals.var_ldrifte), (locals.var_vrdr_dn2 / locals.var_ldrifte), (locals.var_vrdr_dn6 / locals.var_ldrifte), (locals.var_vrdr_dn7 / locals.var_ldrifte),)
    } else {
        (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn6, locals.var_edri_dn7,)
    }
};
        locals.var_edri = assign35370_e50247;
        locals.var_edri_dn0 = assign35370_e50247_d_n0;
        locals.var_edri_dn2 = assign35370_e50247_d_n2;
        locals.var_edri_dn6 = assign35370_e50247_d_n6;
        locals.var_edri_dn7 = assign35370_e50247_d_n7;

        let (assign35380_e50253, assign35380_e50253_d_n0, assign35380_e50253_d_n2, assign35380_e50253_d_n6, assign35380_e50253_d_n7, assign35380_e50253_d_n10, assign35380_e50253_d_n11, assign35380_e50253_d_n12, assign35380_e50253_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35380_e50251: f64 = (locals.var_mu0 * locals.var_edri);
        (assign35380_e50251, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), (locals.var_mu0_dn10 * locals.var_edri), (locals.var_mu0_dn11 * locals.var_edri), (locals.var_mu0_dn12 * locals.var_edri), (locals.var_mu0_dn17 * locals.var_edri),)
    } else {
        (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn12, locals.var_vdri_dn17,)
    }
};
        locals.var_vdri = assign35380_e50253;
        locals.var_vdri_dn0 = assign35380_e50253_d_n0;
        locals.var_vdri_dn2 = assign35380_e50253_d_n2;
        locals.var_vdri_dn6 = assign35380_e50253_d_n6;
        locals.var_vdri_dn7 = assign35380_e50253_d_n7;
        locals.var_vdri_dn10 = assign35380_e50253_d_n10;
        locals.var_vdri_dn11 = assign35380_e50253_d_n11;
        locals.var_vdri_dn12 = assign35380_e50253_d_n12;
        locals.var_vdri_dn17 = assign35380_e50253_d_n17;

        let assign35390_e50256: f64 = if locals.var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1170 = assign35390_e50256;

        let (assign35400_e50264, assign35400_e50264_d_n0, assign35400_e50264_d_n2, assign35400_e50264_d_n6, assign35400_e50264_d_n7, assign35400_e50264_d_n10, assign35400_e50264_d_n11, assign35400_e50264_d_n12, assign35400_e50264_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign35400_e50262: f64 = (locals.var_vdri / locals.var_vmaxe__blk1162);
        (assign35400_e50262, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn0)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn2)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn6)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn7)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn10)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn11)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn12 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn12)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn17 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn17)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35400_e50264;
        locals.var_t1_dn0 = assign35400_e50264_d_n0;
        locals.var_t1_dn2 = assign35400_e50264_d_n2;
        locals.var_t1_dn6 = assign35400_e50264_d_n6;
        locals.var_t1_dn7 = assign35400_e50264_d_n7;
        locals.var_t1_dn10 = assign35400_e50264_d_n10;
        locals.var_t1_dn11 = assign35400_e50264_d_n11;
        locals.var_t1_dn12 = assign35400_e50264_d_n12;
        locals.var_t1_dn17 = assign35400_e50264_d_n17;

        let (assign35410_e50274, assign35410_e50274_d_n0, assign35410_e50274_d_n2, assign35410_e50274_d_n6, assign35410_e50274_d_n7, assign35410_e50274_d_n10, assign35410_e50274_d_n11, assign35410_e50274_d_n12, assign35410_e50274_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1170 == 0.0)) {
        let assign35410_e50270: f64 = (-locals.var_vdri);
        let assign35410_e50272: f64 = (assign35410_e50270 / locals.var_vmaxe__blk1162);
        (assign35410_e50272, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn0)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn2)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn6)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn7)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn10)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn11)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn12) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn12)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn17) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn17)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35410_e50274;
        locals.var_t1_dn0 = assign35410_e50274_d_n0;
        locals.var_t1_dn2 = assign35410_e50274_d_n2;
        locals.var_t1_dn6 = assign35410_e50274_d_n6;
        locals.var_t1_dn7 = assign35410_e50274_d_n7;
        locals.var_t1_dn10 = assign35410_e50274_d_n10;
        locals.var_t1_dn11 = assign35410_e50274_d_n11;
        locals.var_t1_dn12 = assign35410_e50274_d_n12;
        locals.var_t1_dn17 = assign35410_e50274_d_n17;

        let assign35420_e50278: f64 = (10.0 * 2.220446049250313e-16);
        let assign35420_e50279: f64 = (1.0 - assign35420_e50278);
        let assign35420_e50286: f64 = (10.0 * 2.220446049250313e-16);
        let assign35420_e50287: f64 = (1.0 + assign35420_e50286);
        let assign35420_e50289: f64 = if ((assign35420_e50279 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35420_e50287)) { 1.0 } else { 0.0 };
        locals.var_guard1171 = assign35420_e50289;

        let (assign35430_e50295, assign35430_e50295_d_n0, assign35430_e50295_d_n2, assign35430_e50295_d_n6, assign35430_e50295_d_n7, assign35430_e50295_d_n10, assign35430_e50295_d_n11, assign35430_e50295_d_n12, assign35430_e50295_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1171 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35430_e50295;
        locals.var_t3_dn0 = assign35430_e50295_d_n0;
        locals.var_t3_dn2 = assign35430_e50295_d_n2;
        locals.var_t3_dn6 = assign35430_e50295_d_n6;
        locals.var_t3_dn7 = assign35430_e50295_d_n7;
        locals.var_t3_dn10 = assign35430_e50295_d_n10;
        locals.var_t3_dn11 = assign35430_e50295_d_n11;
        locals.var_t3_dn12 = assign35430_e50295_d_n12;
        locals.var_t3_dn17 = assign35430_e50295_d_n17;

        let assign35440_e50299: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50300: f64 = (2.0 - assign35440_e50299);
        let assign35440_e50307: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50308: f64 = (2.0 + assign35440_e50307);
        let assign35440_e50310: f64 = if ((assign35440_e50300 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35440_e50308)) { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign35440_e50310;

        let (assign35450_e50319, assign35450_e50319_d_n0, assign35450_e50319_d_n2, assign35450_e50319_d_n6, assign35450_e50319_d_n7, assign35450_e50319_d_n10, assign35450_e50319_d_n11, assign35450_e50319_d_n12, assign35450_e50319_d_n17,) = {
    if (((locals.var_guard1149 != 0.0) && (locals.var_guard1171 == 0.0)) && (locals.var_guard1172 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35450_e50319;
        locals.var_t3_dn0 = assign35450_e50319_d_n0;
        locals.var_t3_dn2 = assign35450_e50319_d_n2;
        locals.var_t3_dn6 = assign35450_e50319_d_n6;
        locals.var_t3_dn7 = assign35450_e50319_d_n7;
        locals.var_t3_dn10 = assign35450_e50319_d_n10;
        locals.var_t3_dn11 = assign35450_e50319_d_n11;
        locals.var_t3_dn12 = assign35450_e50319_d_n12;
        locals.var_t3_dn17 = assign35450_e50319_d_n17;

        let (assign35460_e50333, assign35460_e50333_d_n0, assign35460_e50333_d_n2, assign35460_e50333_d_n6, assign35460_e50333_d_n7, assign35460_e50333_d_n10, assign35460_e50333_d_n11, assign35460_e50333_d_n12, assign35460_e50333_d_n17,) = {
    if (((locals.var_guard1149 != 0.0) && (locals.var_guard1171 == 0.0)) && (locals.var_guard1172 == 0.0)) {
        let assign35460_e50330: f64 = (locals.var_rrdrbb - 1.0);
        let assign35460_e50331: f64 = (locals.var_t1).powf(assign35460_e50330);
        (assign35460_e50331, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn0)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn2)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn6)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn7)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb_dn10 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn10)) } } else { (assign35460_e50331 * ((locals.var_rrdrbb_dn10 * (locals.var_t1).ln()) + (assign35460_e50330 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn11)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn12)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn17)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn17 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35460_e50333;
        locals.var_t3_dn0 = assign35460_e50333_d_n0;
        locals.var_t3_dn2 = assign35460_e50333_d_n2;
        locals.var_t3_dn6 = assign35460_e50333_d_n6;
        locals.var_t3_dn7 = assign35460_e50333_d_n7;
        locals.var_t3_dn10 = assign35460_e50333_d_n10;
        locals.var_t3_dn11 = assign35460_e50333_d_n11;
        locals.var_t3_dn12 = assign35460_e50333_d_n12;
        locals.var_t3_dn17 = assign35460_e50333_d_n17;

        let (assign35470_e50339, assign35470_e50339_d_n0, assign35470_e50339_d_n2, assign35470_e50339_d_n6, assign35470_e50339_d_n7, assign35470_e50339_d_n10, assign35470_e50339_d_n11, assign35470_e50339_d_n12, assign35470_e50339_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35470_e50337: f64 = (locals.var_t1 * locals.var_t3);
        (assign35470_e50337, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign35470_e50339;
        locals.var_t2_dn0 = assign35470_e50339_d_n0;
        locals.var_t2_dn2 = assign35470_e50339_d_n2;
        locals.var_t2_dn6 = assign35470_e50339_d_n6;
        locals.var_t2_dn7 = assign35470_e50339_d_n7;
        locals.var_t2_dn10 = assign35470_e50339_d_n10;
        locals.var_t2_dn11 = assign35470_e50339_d_n11;
        locals.var_t2_dn12 = assign35470_e50339_d_n12;
        locals.var_t2_dn17 = assign35470_e50339_d_n17;

    }

    pub(super) fn stamp_transient_block_123(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign35480_e50345, assign35480_e50345_d_n0, assign35480_e50345_d_n2, assign35480_e50345_d_n6, assign35480_e50345_d_n7, assign35480_e50345_d_n10, assign35480_e50345_d_n11, assign35480_e50345_d_n12, assign35480_e50345_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35480_e50343: f64 = (1.0 + locals.var_t2);
        (assign35480_e50343, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign35480_e50345;
        locals.var_t4_dn0 = assign35480_e50345_d_n0;
        locals.var_t4_dn2 = assign35480_e50345_d_n2;
        locals.var_t4_dn6 = assign35480_e50345_d_n6;
        locals.var_t4_dn7 = assign35480_e50345_d_n7;
        locals.var_t4_dn10 = assign35480_e50345_d_n10;
        locals.var_t4_dn11 = assign35480_e50345_d_n11;
        locals.var_t4_dn12 = assign35480_e50345_d_n12;
        locals.var_t4_dn17 = assign35480_e50345_d_n17;

        let assign35490_e50349: f64 = (10.0 * 2.220446049250313e-16);
        let assign35490_e50350: f64 = (1.0 - assign35490_e50349);
        let assign35490_e50357: f64 = (10.0 * 2.220446049250313e-16);
        let assign35490_e50358: f64 = (1.0 + assign35490_e50357);
        let assign35490_e50360: f64 = if ((assign35490_e50350 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35490_e50358)) { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign35490_e50360;

        let (assign35500_e50368, assign35500_e50368_d_n0, assign35500_e50368_d_n2, assign35500_e50368_d_n6, assign35500_e50368_d_n7, assign35500_e50368_d_n10, assign35500_e50368_d_n11, assign35500_e50368_d_n12, assign35500_e50368_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1173 != 0.0)) {
        let assign35500_e50366: f64 = (1.0 / locals.var_t4);
        (assign35500_e50366, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign35500_e50368;
        locals.var_t5_dn0 = assign35500_e50368_d_n0;
        locals.var_t5_dn2 = assign35500_e50368_d_n2;
        locals.var_t5_dn6 = assign35500_e50368_d_n6;
        locals.var_t5_dn7 = assign35500_e50368_d_n7;
        locals.var_t5_dn10 = assign35500_e50368_d_n10;
        locals.var_t5_dn11 = assign35500_e50368_d_n11;
        locals.var_t5_dn12 = assign35500_e50368_d_n12;
        locals.var_t5_dn17 = assign35500_e50368_d_n17;

        let assign35510_e50372: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50373: f64 = (2.0 - assign35510_e50372);
        let assign35510_e50380: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50381: f64 = (2.0 + assign35510_e50380);
        let assign35510_e50383: f64 = if ((assign35510_e50373 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35510_e50381)) { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign35510_e50383;

        let (assign35520_e50395, assign35520_e50395_d_n0, assign35520_e50395_d_n2, assign35520_e50395_d_n6, assign35520_e50395_d_n7, assign35520_e50395_d_n10, assign35520_e50395_d_n11, assign35520_e50395_d_n12, assign35520_e50395_d_n17,) = {
    if (((locals.var_guard1149 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 != 0.0)) {
        let assign35520_e50392: f64 = (locals.var_t4).sqrt();
        let assign35520_e50393: f64 = (1.0 / assign35520_e50392);
        (assign35520_e50393, (-((locals.var_t4_dn0 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn2 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn6 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn7 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn10 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn11 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn12 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn17 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign35520_e50395;
        locals.var_t5_dn0 = assign35520_e50395_d_n0;
        locals.var_t5_dn2 = assign35520_e50395_d_n2;
        locals.var_t5_dn6 = assign35520_e50395_d_n6;
        locals.var_t5_dn7 = assign35520_e50395_d_n7;
        locals.var_t5_dn10 = assign35520_e50395_d_n10;
        locals.var_t5_dn11 = assign35520_e50395_d_n11;
        locals.var_t5_dn12 = assign35520_e50395_d_n12;
        locals.var_t5_dn17 = assign35520_e50395_d_n17;

        let (assign35530_e50412, assign35530_e50412_d_n0, assign35530_e50412_d_n2, assign35530_e50412_d_n6, assign35530_e50412_d_n7, assign35530_e50412_d_n10, assign35530_e50412_d_n11, assign35530_e50412_d_n12, assign35530_e50412_d_n17,) = {
    if (((locals.var_guard1149 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 == 0.0)) {
        let assign35530_e50405: f64 = (-1.0);
        let assign35530_e50407: f64 = (assign35530_e50405 / locals.var_rrdrbb);
        let assign35530_e50409: f64 = (assign35530_e50407 - 1.0);
        let assign35530_e50410: f64 = (locals.var_t4).powf(assign35530_e50409);
        (assign35530_e50410, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn0)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn2)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn6)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn7)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign35530_e50405 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn10)) } } else { (assign35530_e50410 * (((-((assign35530_e50405 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) * (locals.var_t4).ln()) + (assign35530_e50409 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn11)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn12)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn17)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn17 / locals.var_t4))) },)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign35530_e50412;
        locals.var_t6_dn0 = assign35530_e50412_d_n0;
        locals.var_t6_dn2 = assign35530_e50412_d_n2;
        locals.var_t6_dn6 = assign35530_e50412_d_n6;
        locals.var_t6_dn7 = assign35530_e50412_d_n7;
        locals.var_t6_dn10 = assign35530_e50412_d_n10;
        locals.var_t6_dn11 = assign35530_e50412_d_n11;
        locals.var_t6_dn12 = assign35530_e50412_d_n12;
        locals.var_t6_dn17 = assign35530_e50412_d_n17;

        let (assign35540_e50424, assign35540_e50424_d_n0, assign35540_e50424_d_n2, assign35540_e50424_d_n6, assign35540_e50424_d_n7, assign35540_e50424_d_n10, assign35540_e50424_d_n11, assign35540_e50424_d_n12, assign35540_e50424_d_n17,) = {
    if (((locals.var_guard1149 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 == 0.0)) {
        let assign35540_e50422: f64 = (locals.var_t4 * locals.var_t6);
        (assign35540_e50422, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign35540_e50424;
        locals.var_t5_dn0 = assign35540_e50424_d_n0;
        locals.var_t5_dn2 = assign35540_e50424_d_n2;
        locals.var_t5_dn6 = assign35540_e50424_d_n6;
        locals.var_t5_dn7 = assign35540_e50424_d_n7;
        locals.var_t5_dn10 = assign35540_e50424_d_n10;
        locals.var_t5_dn11 = assign35540_e50424_d_n11;
        locals.var_t5_dn12 = assign35540_e50424_d_n12;
        locals.var_t5_dn17 = assign35540_e50424_d_n17;

        let (assign35560_e50436, assign35560_e50436_d_n0, assign35560_e50436_d_n2, assign35560_e50436_d_n6, assign35560_e50436_d_n7, assign35560_e50436_d_n10, assign35560_e50436_d_n11, assign35560_e50436_d_n12, assign35560_e50436_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35560_e50434: f64 = (1.6021918e-19 / locals.var_ldrifte);
        (assign35560_e50434, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35560_e50436;
        locals.var_t1_dn0 = assign35560_e50436_d_n0;
        locals.var_t1_dn2 = assign35560_e50436_d_n2;
        locals.var_t1_dn6 = assign35560_e50436_d_n6;
        locals.var_t1_dn7 = assign35560_e50436_d_n7;
        locals.var_t1_dn10 = assign35560_e50436_d_n10;
        locals.var_t1_dn11 = assign35560_e50436_d_n11;
        locals.var_t1_dn12 = assign35560_e50436_d_n12;
        locals.var_t1_dn17 = assign35560_e50436_d_n17;

        let assign35680_e50510: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign35680_e50510;

        let (assign35690_e50514,) = {
    if (locals.var_guard1177 != 0.0) {
        (2.0,)
    } else {
        (locals.var_rdmod,)
    }
};
        locals.var_rdmod = assign35690_e50514;

        let assign35700_e50517: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign35700_e50517;

        let (assign35720_e50531,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 != 0.0)) {
        (p.p266,)
    } else {
        (locals.var_mks_rdrmue__blk1181,)
    }
};
        locals.var_mks_rdrmue__blk1181 = assign35720_e50531;

        let (assign35730_e50537,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 != 0.0)) {
        (p.p268,)
    } else {
        (locals.var_mks_rdrvmax__blk1182,)
    }
};
        locals.var_mks_rdrvmax__blk1182 = assign35730_e50537;

        let (assign35740_e50543, assign35740_e50543_d_n10,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (locals.var_rrdrbb__blk1183, locals.var_rrdrbb__blk1183_dn10,)
    }
};
        locals.var_rrdrbb__blk1183 = assign35740_e50543;
        locals.var_rrdrbb__blk1183_dn10 = assign35740_e50543_d_n10;

        let (assign35760_e50562,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 != 0.0)) {
        (p.p258,)
    } else {
        (locals.var_ldrifte__blk1187,)
    }
};
        locals.var_ldrifte__blk1187 = assign35760_e50562;

        let (assign35770_e50570, assign35770_e50570_d_n0, assign35770_e50570_d_n2, assign35770_e50570_d_n6, assign35770_e50570_d_n7,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 != 0.0)) {
        let assign35770_e50568: f64 = (p.p50 * (nv7 - nv2));
        (assign35770_e50568, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (locals.var_vrdr__blk1185, locals.var_vrdr__blk1185_dn0, locals.var_vrdr__blk1185_dn2, locals.var_vrdr__blk1185_dn6, locals.var_vrdr__blk1185_dn7,)
    }
};
        locals.var_vrdr__blk1185 = assign35770_e50570;
        locals.var_vrdr__blk1185_dn0 = assign35770_e50570_d_n0;
        locals.var_vrdr__blk1185_dn2 = assign35770_e50570_d_n2;
        locals.var_vrdr__blk1185_dn6 = assign35770_e50570_d_n6;
        locals.var_vrdr__blk1185_dn7 = assign35770_e50570_d_n7;

        let (assign35790_e50586,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 == 0.0)) {
        (p.p265,)
    } else {
        (locals.var_mks_rdrmue__blk1181,)
    }
};
        locals.var_mks_rdrmue__blk1181 = assign35790_e50586;

        let (assign35800_e50593,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 == 0.0)) {
        (p.p267,)
    } else {
        (locals.var_mks_rdrvmax__blk1182,)
    }
};
        locals.var_mks_rdrvmax__blk1182 = assign35800_e50593;

        let (assign35810_e50600, assign35810_e50600_d_n10,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (locals.var_rrdrbb__blk1183, locals.var_rrdrbb__blk1183_dn10,)
    }
};
        locals.var_rrdrbb__blk1183 = assign35810_e50600;
        locals.var_rrdrbb__blk1183_dn10 = assign35810_e50600_d_n10;

        let (assign35830_e50621,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 == 0.0)) {
        (p.p257,)
    } else {
        (locals.var_ldrifte__blk1187,)
    }
};
        locals.var_ldrifte__blk1187 = assign35830_e50621;

        let (assign35840_e50630, assign35840_e50630_d_n0, assign35840_e50630_d_n2, assign35840_e50630_d_n6, assign35840_e50630_d_n7,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 == 0.0)) {
        let assign35840_e50628: f64 = (p.p50 * (nv0 - nv6));
        (assign35840_e50628, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (locals.var_vrdr__blk1185, locals.var_vrdr__blk1185_dn0, locals.var_vrdr__blk1185_dn2, locals.var_vrdr__blk1185_dn6, locals.var_vrdr__blk1185_dn7,)
    }
};
        locals.var_vrdr__blk1185 = assign35840_e50630;
        locals.var_vrdr__blk1185_dn0 = assign35840_e50630_d_n0;
        locals.var_vrdr__blk1185_dn2 = assign35840_e50630_d_n2;
        locals.var_vrdr__blk1185_dn6 = assign35840_e50630_d_n6;
        locals.var_vrdr__blk1185_dn7 = assign35840_e50630_d_n7;

        let (assign35870_e50653,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35870_e50651: f64 = (locals.var_mks_rdrmue__blk1181 / 10000.0);
        (assign35870_e50651,)
    } else {
        (locals.var_mks_rdrmue__blk1181,)
    }
};
        locals.var_mks_rdrmue__blk1181 = assign35870_e50653;

        let (assign35880_e50659,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35880_e50657: f64 = (locals.var_mks_rdrvmax__blk1182 / 100.0);
        (assign35880_e50657,)
    } else {
        (locals.var_mks_rdrvmax__blk1182,)
    }
};
        locals.var_mks_rdrvmax__blk1182 = assign35880_e50659;

        let (assign35890_e50665, assign35890_e50665_d_n10,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35890_e50663: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign35890_e50663, (locals.var_ttemp_dn10 / locals.var_uc_tnom),)
    } else {
        (locals.var_tratio__blk1186, locals.var_tratio__blk1186_dn10,)
    }
};
        locals.var_tratio__blk1186 = assign35890_e50665;
        locals.var_tratio__blk1186_dn10 = assign35890_e50665_d_n10;

        let (assign35900_e50671, assign35900_e50671_d_n0, assign35900_e50671_d_n2, assign35900_e50671_d_n6, assign35900_e50671_d_n7, assign35900_e50671_d_n10, assign35900_e50671_d_n11, assign35900_e50671_d_n12, assign35900_e50671_d_n17,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35900_e50669: f64 = (locals.var_tratio__blk1186).powf(p.p269);
        (assign35900_e50669, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio__blk1186).powf(p.p269 - 1.0) * locals.var_tratio__blk1186_dn10)) } } else { (assign35900_e50669 * (p.p269 * (locals.var_tratio__blk1186_dn10 / locals.var_tratio__blk1186))) }, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35900_e50671;
        locals.var_t1_dn0 = assign35900_e50671_d_n0;
        locals.var_t1_dn2 = assign35900_e50671_d_n2;
        locals.var_t1_dn6 = assign35900_e50671_d_n6;
        locals.var_t1_dn7 = assign35900_e50671_d_n7;
        locals.var_t1_dn10 = assign35900_e50671_d_n10;
        locals.var_t1_dn11 = assign35900_e50671_d_n11;
        locals.var_t1_dn12 = assign35900_e50671_d_n12;
        locals.var_t1_dn17 = assign35900_e50671_d_n17;

        let (assign35910_e50677, assign35910_e50677_d_n0, assign35910_e50677_d_n2, assign35910_e50677_d_n6, assign35910_e50677_d_n7, assign35910_e50677_d_n10, assign35910_e50677_d_n11, assign35910_e50677_d_n12, assign35910_e50677_d_n17,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35910_e50675: f64 = (locals.var_mks_rdrmue__blk1181 / locals.var_t1);
        (assign35910_e50675, (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_mu0__blk1189, locals.var_mu0__blk1189_dn0, locals.var_mu0__blk1189_dn2, locals.var_mu0__blk1189_dn6, locals.var_mu0__blk1189_dn7, locals.var_mu0__blk1189_dn10, locals.var_mu0__blk1189_dn11, locals.var_mu0__blk1189_dn12, locals.var_mu0__blk1189_dn17,)
    }
};
        locals.var_mu0__blk1189 = assign35910_e50677;
        locals.var_mu0__blk1189_dn0 = assign35910_e50677_d_n0;
        locals.var_mu0__blk1189_dn2 = assign35910_e50677_d_n2;
        locals.var_mu0__blk1189_dn6 = assign35910_e50677_d_n6;
        locals.var_mu0__blk1189_dn7 = assign35910_e50677_d_n7;
        locals.var_mu0__blk1189_dn10 = assign35910_e50677_d_n10;
        locals.var_mu0__blk1189_dn11 = assign35910_e50677_d_n11;
        locals.var_mu0__blk1189_dn12 = assign35910_e50677_d_n12;
        locals.var_mu0__blk1189_dn17 = assign35910_e50677_d_n17;

        let (assign35920_e50697, assign35920_e50697_d_n0, assign35920_e50697_d_n2, assign35920_e50697_d_n6, assign35920_e50697_d_n7, assign35920_e50697_d_n10, assign35920_e50697_d_n11, assign35920_e50697_d_n12, assign35920_e50697_d_n17,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35920_e50682: f64 = (0.4 * locals.var_tratio__blk1186);
        let assign35920_e50683: f64 = (1.8 + assign35920_e50682);
        let assign35920_e50686: f64 = (0.1 * locals.var_tratio__blk1186);
        let assign35920_e50688: f64 = (assign35920_e50686 * locals.var_tratio__blk1186);
        let assign35920_e50689: f64 = (assign35920_e50683 + assign35920_e50688);
        let assign35920_e50693: f64 = (1.0 - locals.var_tratio__blk1186);
        let assign35920_e50694: f64 = (p.p270 * assign35920_e50693);
        let assign35920_e50695: f64 = (assign35920_e50689 - assign35920_e50694);
        (assign35920_e50695, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio__blk1186_dn10) + (((0.1 * locals.var_tratio__blk1186_dn10) * locals.var_tratio__blk1186) + (assign35920_e50686 * locals.var_tratio__blk1186_dn10))) - (p.p270 * (-locals.var_tratio__blk1186_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign35920_e50697;
        locals.var_t0_dn0 = assign35920_e50697_d_n0;
        locals.var_t0_dn2 = assign35920_e50697_d_n2;
        locals.var_t0_dn6 = assign35920_e50697_d_n6;
        locals.var_t0_dn7 = assign35920_e50697_d_n7;
        locals.var_t0_dn10 = assign35920_e50697_d_n10;
        locals.var_t0_dn11 = assign35920_e50697_d_n11;
        locals.var_t0_dn12 = assign35920_e50697_d_n12;
        locals.var_t0_dn17 = assign35920_e50697_d_n17;

        let (assign35930_e50703, assign35930_e50703_d_n0, assign35930_e50703_d_n2, assign35930_e50703_d_n6, assign35930_e50703_d_n7, assign35930_e50703_d_n10, assign35930_e50703_d_n11, assign35930_e50703_d_n12, assign35930_e50703_d_n17,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35930_e50701: f64 = (locals.var_mks_rdrvmax__blk1182 / locals.var_t0);
        (assign35930_e50701, (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_vmaxe__blk1190, locals.var_vmaxe__blk1190_dn0, locals.var_vmaxe__blk1190_dn2, locals.var_vmaxe__blk1190_dn6, locals.var_vmaxe__blk1190_dn7, locals.var_vmaxe__blk1190_dn10, locals.var_vmaxe__blk1190_dn11, locals.var_vmaxe__blk1190_dn12, locals.var_vmaxe__blk1190_dn17,)
    }
};
        locals.var_vmaxe__blk1190 = assign35930_e50703;
        locals.var_vmaxe__blk1190_dn0 = assign35930_e50703_d_n0;
        locals.var_vmaxe__blk1190_dn2 = assign35930_e50703_d_n2;
        locals.var_vmaxe__blk1190_dn6 = assign35930_e50703_d_n6;
        locals.var_vmaxe__blk1190_dn7 = assign35930_e50703_d_n7;
        locals.var_vmaxe__blk1190_dn10 = assign35930_e50703_d_n10;
        locals.var_vmaxe__blk1190_dn11 = assign35930_e50703_d_n11;
        locals.var_vmaxe__blk1190_dn12 = assign35930_e50703_d_n12;
        locals.var_vmaxe__blk1190_dn17 = assign35930_e50703_d_n17;

        let (assign35940_e50713, assign35940_e50713_d_n10,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35940_e50709: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        let assign35940_e50710: f64 = (p.p274 * assign35940_e50709);
        let assign35940_e50711: f64 = (locals.var_rrdrbb__blk1183 + assign35940_e50710);
        (assign35940_e50711, (locals.var_rrdrbb__blk1183_dn10 + (p.p274 * locals.var_ttemp_dn10)),)
    } else {
        (locals.var_rrdrbb__blk1183, locals.var_rrdrbb__blk1183_dn10,)
    }
};
        locals.var_rrdrbb__blk1183 = assign35940_e50713;
        locals.var_rrdrbb__blk1183_dn10 = assign35940_e50713_d_n10;

        let (assign35950_e50723,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35950_e50719: f64 = (locals.var_lgle).powf(p.p280);
        let assign35950_e50720: f64 = (p.p279 / assign35950_e50719);
        let assign35950_e50721: f64 = (1.0 + assign35950_e50720);
        (assign35950_e50721,)
    } else {
        (locals.var_rdrmuele__blk1178,)
    }
};
        locals.var_rdrmuele__blk1178 = assign35950_e50723;

        let (assign35960_e50733,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35960_e50729: f64 = (locals.var_lgle).powf(p.p278);
        let assign35960_e50730: f64 = (p.p277 / assign35960_e50729);
        let assign35960_e50731: f64 = (1.0 + assign35960_e50730);
        (assign35960_e50731,)
    } else {
        (locals.var_rdrvmaxle__blk1180,)
    }
};
        locals.var_rdrvmaxle__blk1180 = assign35960_e50733;

        let (assign35970_e50743,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35970_e50739: f64 = (locals.var_wg).powf(p.p276);
        let assign35970_e50740: f64 = (p.p275 / assign35970_e50739);
        let assign35970_e50741: f64 = (1.0 + assign35970_e50740);
        (assign35970_e50741,)
    } else {
        (locals.var_rdrvmaxwe__blk1179,)
    }
};
        locals.var_rdrvmaxwe__blk1179 = assign35970_e50743;

        let (assign35980_e50749, assign35980_e50749_d_n0, assign35980_e50749_d_n2, assign35980_e50749_d_n6, assign35980_e50749_d_n7, assign35980_e50749_d_n10, assign35980_e50749_d_n11, assign35980_e50749_d_n12, assign35980_e50749_d_n17,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35980_e50747: f64 = (locals.var_mu0__blk1189 * locals.var_rdrmuele__blk1178);
        (assign35980_e50747, (locals.var_mu0__blk1189_dn0 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn2 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn6 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn7 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn10 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn11 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn12 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn17 * locals.var_rdrmuele__blk1178),)
    } else {
        (locals.var_mu0__blk1189, locals.var_mu0__blk1189_dn0, locals.var_mu0__blk1189_dn2, locals.var_mu0__blk1189_dn6, locals.var_mu0__blk1189_dn7, locals.var_mu0__blk1189_dn10, locals.var_mu0__blk1189_dn11, locals.var_mu0__blk1189_dn12, locals.var_mu0__blk1189_dn17,)
    }
};
        locals.var_mu0__blk1189 = assign35980_e50749;
        locals.var_mu0__blk1189_dn0 = assign35980_e50749_d_n0;
        locals.var_mu0__blk1189_dn2 = assign35980_e50749_d_n2;
        locals.var_mu0__blk1189_dn6 = assign35980_e50749_d_n6;
        locals.var_mu0__blk1189_dn7 = assign35980_e50749_d_n7;
        locals.var_mu0__blk1189_dn10 = assign35980_e50749_d_n10;
        locals.var_mu0__blk1189_dn11 = assign35980_e50749_d_n11;
        locals.var_mu0__blk1189_dn12 = assign35980_e50749_d_n12;
        locals.var_mu0__blk1189_dn17 = assign35980_e50749_d_n17;

        let (assign35990_e50759, assign35990_e50759_d_n0, assign35990_e50759_d_n2, assign35990_e50759_d_n6, assign35990_e50759_d_n7, assign35990_e50759_d_n10, assign35990_e50759_d_n11, assign35990_e50759_d_n12, assign35990_e50759_d_n17,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign35990_e50753: f64 = (locals.var_vmaxe__blk1190 * locals.var_rdrvmaxwe__blk1179);
        let assign35990_e50755: f64 = (assign35990_e50753 * locals.var_rdrvmaxle__blk1180);
        let assign35990_e50757: f64 = (assign35990_e50755 + 1e-50);
        (assign35990_e50757, ((locals.var_vmaxe__blk1190_dn0 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn2 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn6 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn7 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn10 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn11 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn12 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn17 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180),)
    } else {
        (locals.var_vmaxe__blk1190, locals.var_vmaxe__blk1190_dn0, locals.var_vmaxe__blk1190_dn2, locals.var_vmaxe__blk1190_dn6, locals.var_vmaxe__blk1190_dn7, locals.var_vmaxe__blk1190_dn10, locals.var_vmaxe__blk1190_dn11, locals.var_vmaxe__blk1190_dn12, locals.var_vmaxe__blk1190_dn17,)
    }
};
        locals.var_vmaxe__blk1190 = assign35990_e50759;
        locals.var_vmaxe__blk1190_dn0 = assign35990_e50759_d_n0;
        locals.var_vmaxe__blk1190_dn2 = assign35990_e50759_d_n2;
        locals.var_vmaxe__blk1190_dn6 = assign35990_e50759_d_n6;
        locals.var_vmaxe__blk1190_dn7 = assign35990_e50759_d_n7;
        locals.var_vmaxe__blk1190_dn10 = assign35990_e50759_d_n10;
        locals.var_vmaxe__blk1190_dn11 = assign35990_e50759_d_n11;
        locals.var_vmaxe__blk1190_dn12 = assign35990_e50759_d_n12;
        locals.var_vmaxe__blk1190_dn17 = assign35990_e50759_d_n17;

        let (assign36000_e50765, assign36000_e50765_d_n0, assign36000_e50765_d_n2, assign36000_e50765_d_n6, assign36000_e50765_d_n7,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign36000_e50763: f64 = (locals.var_vrdr__blk1185 / locals.var_ldrifte__blk1187);
        (assign36000_e50763, (locals.var_vrdr__blk1185_dn0 / locals.var_ldrifte__blk1187), (locals.var_vrdr__blk1185_dn2 / locals.var_ldrifte__blk1187), (locals.var_vrdr__blk1185_dn6 / locals.var_ldrifte__blk1187), (locals.var_vrdr__blk1185_dn7 / locals.var_ldrifte__blk1187),)
    } else {
        (locals.var_edri__blk1191, locals.var_edri__blk1191_dn0, locals.var_edri__blk1191_dn2, locals.var_edri__blk1191_dn6, locals.var_edri__blk1191_dn7,)
    }
};
        locals.var_edri__blk1191 = assign36000_e50765;
        locals.var_edri__blk1191_dn0 = assign36000_e50765_d_n0;
        locals.var_edri__blk1191_dn2 = assign36000_e50765_d_n2;
        locals.var_edri__blk1191_dn6 = assign36000_e50765_d_n6;
        locals.var_edri__blk1191_dn7 = assign36000_e50765_d_n7;

        let (assign36010_e50771, assign36010_e50771_d_n0, assign36010_e50771_d_n2, assign36010_e50771_d_n6, assign36010_e50771_d_n7, assign36010_e50771_d_n10, assign36010_e50771_d_n11, assign36010_e50771_d_n12, assign36010_e50771_d_n17,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign36010_e50769: f64 = (locals.var_mu0__blk1189 * locals.var_edri__blk1191);
        (assign36010_e50769, ((locals.var_mu0__blk1189_dn0 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn0)), ((locals.var_mu0__blk1189_dn2 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn2)), ((locals.var_mu0__blk1189_dn6 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn6)), ((locals.var_mu0__blk1189_dn7 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn7)), (locals.var_mu0__blk1189_dn10 * locals.var_edri__blk1191), (locals.var_mu0__blk1189_dn11 * locals.var_edri__blk1191), (locals.var_mu0__blk1189_dn12 * locals.var_edri__blk1191), (locals.var_mu0__blk1189_dn17 * locals.var_edri__blk1191),)
    } else {
        (locals.var_vdri__blk1192, locals.var_vdri__blk1192_dn0, locals.var_vdri__blk1192_dn2, locals.var_vdri__blk1192_dn6, locals.var_vdri__blk1192_dn7, locals.var_vdri__blk1192_dn10, locals.var_vdri__blk1192_dn11, locals.var_vdri__blk1192_dn12, locals.var_vdri__blk1192_dn17,)
    }
};
        locals.var_vdri__blk1192 = assign36010_e50771;
        locals.var_vdri__blk1192_dn0 = assign36010_e50771_d_n0;
        locals.var_vdri__blk1192_dn2 = assign36010_e50771_d_n2;
        locals.var_vdri__blk1192_dn6 = assign36010_e50771_d_n6;
        locals.var_vdri__blk1192_dn7 = assign36010_e50771_d_n7;
        locals.var_vdri__blk1192_dn10 = assign36010_e50771_d_n10;
        locals.var_vdri__blk1192_dn11 = assign36010_e50771_d_n11;
        locals.var_vdri__blk1192_dn12 = assign36010_e50771_d_n12;
        locals.var_vdri__blk1192_dn17 = assign36010_e50771_d_n17;

        let assign36020_e50774: f64 = if locals.var_vrdr__blk1185 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign36020_e50774;

        let (assign36030_e50782, assign36030_e50782_d_n0, assign36030_e50782_d_n2, assign36030_e50782_d_n6, assign36030_e50782_d_n7, assign36030_e50782_d_n10, assign36030_e50782_d_n11, assign36030_e50782_d_n12, assign36030_e50782_d_n17,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign36030_e50780: f64 = (locals.var_vdri__blk1192 / locals.var_vmaxe__blk1190);
        (assign36030_e50780, (((locals.var_vdri__blk1192_dn0 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn0)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn2 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn2)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn6 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn6)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn7 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn7)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn10 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn10)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn11 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn11)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn12 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn12)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn17 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn17)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign36030_e50782;
        locals.var_t1_dn0 = assign36030_e50782_d_n0;
        locals.var_t1_dn2 = assign36030_e50782_d_n2;
        locals.var_t1_dn6 = assign36030_e50782_d_n6;
        locals.var_t1_dn7 = assign36030_e50782_d_n7;
        locals.var_t1_dn10 = assign36030_e50782_d_n10;
        locals.var_t1_dn11 = assign36030_e50782_d_n11;
        locals.var_t1_dn12 = assign36030_e50782_d_n12;
        locals.var_t1_dn17 = assign36030_e50782_d_n17;

        let (assign36040_e50792, assign36040_e50792_d_n0, assign36040_e50792_d_n2, assign36040_e50792_d_n6, assign36040_e50792_d_n7, assign36040_e50792_d_n10, assign36040_e50792_d_n11, assign36040_e50792_d_n12, assign36040_e50792_d_n17,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1198 == 0.0)) {
        let assign36040_e50788: f64 = (-locals.var_vdri__blk1192);
        let assign36040_e50790: f64 = (assign36040_e50788 / locals.var_vmaxe__blk1190);
        (assign36040_e50790, ((((-locals.var_vdri__blk1192_dn0) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn0)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn2) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn2)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn6) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn6)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn7) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn7)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn10) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn10)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn11) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn11)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn12) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn12)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn17) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn17)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign36040_e50792;
        locals.var_t1_dn0 = assign36040_e50792_d_n0;
        locals.var_t1_dn2 = assign36040_e50792_d_n2;
        locals.var_t1_dn6 = assign36040_e50792_d_n6;
        locals.var_t1_dn7 = assign36040_e50792_d_n7;
        locals.var_t1_dn10 = assign36040_e50792_d_n10;
        locals.var_t1_dn11 = assign36040_e50792_d_n11;
        locals.var_t1_dn12 = assign36040_e50792_d_n12;
        locals.var_t1_dn17 = assign36040_e50792_d_n17;

    }

    pub(super) fn stamp_transient_block_124(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign36050_e50796: f64 = (10.0 * 2.220446049250313e-16);
        let assign36050_e50797: f64 = (1.0 - assign36050_e50796);
        let assign36050_e50804: f64 = (10.0 * 2.220446049250313e-16);
        let assign36050_e50805: f64 = (1.0 + assign36050_e50804);
        let assign36050_e50807: f64 = if ((assign36050_e50797 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36050_e50805)) { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign36050_e50807;

        let (assign36060_e50813, assign36060_e50813_d_n0, assign36060_e50813_d_n2, assign36060_e50813_d_n6, assign36060_e50813_d_n7, assign36060_e50813_d_n10, assign36060_e50813_d_n11, assign36060_e50813_d_n12, assign36060_e50813_d_n17,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1199 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign36060_e50813;
        locals.var_t3_dn0 = assign36060_e50813_d_n0;
        locals.var_t3_dn2 = assign36060_e50813_d_n2;
        locals.var_t3_dn6 = assign36060_e50813_d_n6;
        locals.var_t3_dn7 = assign36060_e50813_d_n7;
        locals.var_t3_dn10 = assign36060_e50813_d_n10;
        locals.var_t3_dn11 = assign36060_e50813_d_n11;
        locals.var_t3_dn12 = assign36060_e50813_d_n12;
        locals.var_t3_dn17 = assign36060_e50813_d_n17;

        let assign36070_e50817: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50818: f64 = (2.0 - assign36070_e50817);
        let assign36070_e50825: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50826: f64 = (2.0 + assign36070_e50825);
        let assign36070_e50828: f64 = if ((assign36070_e50818 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36070_e50826)) { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign36070_e50828;

        let (assign36080_e50837, assign36080_e50837_d_n0, assign36080_e50837_d_n2, assign36080_e50837_d_n6, assign36080_e50837_d_n7, assign36080_e50837_d_n10, assign36080_e50837_d_n11, assign36080_e50837_d_n12, assign36080_e50837_d_n17,) = {
    if (((locals.var_guard1177 != 0.0) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign36080_e50837;
        locals.var_t3_dn0 = assign36080_e50837_d_n0;
        locals.var_t3_dn2 = assign36080_e50837_d_n2;
        locals.var_t3_dn6 = assign36080_e50837_d_n6;
        locals.var_t3_dn7 = assign36080_e50837_d_n7;
        locals.var_t3_dn10 = assign36080_e50837_d_n10;
        locals.var_t3_dn11 = assign36080_e50837_d_n11;
        locals.var_t3_dn12 = assign36080_e50837_d_n12;
        locals.var_t3_dn17 = assign36080_e50837_d_n17;

        let (assign36090_e50851, assign36090_e50851_d_n0, assign36090_e50851_d_n2, assign36090_e50851_d_n6, assign36090_e50851_d_n7, assign36090_e50851_d_n10, assign36090_e50851_d_n11, assign36090_e50851_d_n12, assign36090_e50851_d_n17,) = {
    if (((locals.var_guard1177 != 0.0) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign36090_e50848: f64 = (locals.var_rrdrbb__blk1183 - 1.0);
        let assign36090_e50849: f64 = (locals.var_t1).powf(assign36090_e50848);
        (assign36090_e50849, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn0)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn2)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn6)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn7)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb__blk1183_dn10 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn10)) } } else { (assign36090_e50849 * ((locals.var_rrdrbb__blk1183_dn10 * (locals.var_t1).ln()) + (assign36090_e50848 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn11)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn12)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn17)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn17 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign36090_e50851;
        locals.var_t3_dn0 = assign36090_e50851_d_n0;
        locals.var_t3_dn2 = assign36090_e50851_d_n2;
        locals.var_t3_dn6 = assign36090_e50851_d_n6;
        locals.var_t3_dn7 = assign36090_e50851_d_n7;
        locals.var_t3_dn10 = assign36090_e50851_d_n10;
        locals.var_t3_dn11 = assign36090_e50851_d_n11;
        locals.var_t3_dn12 = assign36090_e50851_d_n12;
        locals.var_t3_dn17 = assign36090_e50851_d_n17;

        let (assign36100_e50857, assign36100_e50857_d_n0, assign36100_e50857_d_n2, assign36100_e50857_d_n6, assign36100_e50857_d_n7, assign36100_e50857_d_n10, assign36100_e50857_d_n11, assign36100_e50857_d_n12, assign36100_e50857_d_n17,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign36100_e50855: f64 = (locals.var_t1 * locals.var_t3);
        (assign36100_e50855, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign36100_e50857;
        locals.var_t2_dn0 = assign36100_e50857_d_n0;
        locals.var_t2_dn2 = assign36100_e50857_d_n2;
        locals.var_t2_dn6 = assign36100_e50857_d_n6;
        locals.var_t2_dn7 = assign36100_e50857_d_n7;
        locals.var_t2_dn10 = assign36100_e50857_d_n10;
        locals.var_t2_dn11 = assign36100_e50857_d_n11;
        locals.var_t2_dn12 = assign36100_e50857_d_n12;
        locals.var_t2_dn17 = assign36100_e50857_d_n17;

        let (assign36110_e50863, assign36110_e50863_d_n0, assign36110_e50863_d_n2, assign36110_e50863_d_n6, assign36110_e50863_d_n7, assign36110_e50863_d_n10, assign36110_e50863_d_n11, assign36110_e50863_d_n12, assign36110_e50863_d_n17,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign36110_e50861: f64 = (1.0 + locals.var_t2);
        (assign36110_e50861, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign36110_e50863;
        locals.var_t4_dn0 = assign36110_e50863_d_n0;
        locals.var_t4_dn2 = assign36110_e50863_d_n2;
        locals.var_t4_dn6 = assign36110_e50863_d_n6;
        locals.var_t4_dn7 = assign36110_e50863_d_n7;
        locals.var_t4_dn10 = assign36110_e50863_d_n10;
        locals.var_t4_dn11 = assign36110_e50863_d_n11;
        locals.var_t4_dn12 = assign36110_e50863_d_n12;
        locals.var_t4_dn17 = assign36110_e50863_d_n17;

        let assign36120_e50867: f64 = (10.0 * 2.220446049250313e-16);
        let assign36120_e50868: f64 = (1.0 - assign36120_e50867);
        let assign36120_e50875: f64 = (10.0 * 2.220446049250313e-16);
        let assign36120_e50876: f64 = (1.0 + assign36120_e50875);
        let assign36120_e50878: f64 = if ((assign36120_e50868 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36120_e50876)) { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign36120_e50878;

        let (assign36130_e50886, assign36130_e50886_d_n0, assign36130_e50886_d_n2, assign36130_e50886_d_n6, assign36130_e50886_d_n7, assign36130_e50886_d_n10, assign36130_e50886_d_n11, assign36130_e50886_d_n12, assign36130_e50886_d_n17,) = {
    if ((locals.var_guard1177 != 0.0) && (locals.var_guard1201 != 0.0)) {
        let assign36130_e50884: f64 = (1.0 / locals.var_t4);
        (assign36130_e50884, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign36130_e50886;
        locals.var_t5_dn0 = assign36130_e50886_d_n0;
        locals.var_t5_dn2 = assign36130_e50886_d_n2;
        locals.var_t5_dn6 = assign36130_e50886_d_n6;
        locals.var_t5_dn7 = assign36130_e50886_d_n7;
        locals.var_t5_dn10 = assign36130_e50886_d_n10;
        locals.var_t5_dn11 = assign36130_e50886_d_n11;
        locals.var_t5_dn12 = assign36130_e50886_d_n12;
        locals.var_t5_dn17 = assign36130_e50886_d_n17;

        let assign36140_e50890: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50891: f64 = (2.0 - assign36140_e50890);
        let assign36140_e50898: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50899: f64 = (2.0 + assign36140_e50898);
        let assign36140_e50901: f64 = if ((assign36140_e50891 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36140_e50899)) { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign36140_e50901;

        let (assign36150_e50913, assign36150_e50913_d_n0, assign36150_e50913_d_n2, assign36150_e50913_d_n6, assign36150_e50913_d_n7, assign36150_e50913_d_n10, assign36150_e50913_d_n11, assign36150_e50913_d_n12, assign36150_e50913_d_n17,) = {
    if (((locals.var_guard1177 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        let assign36150_e50910: f64 = (locals.var_t4).sqrt();
        let assign36150_e50911: f64 = (1.0 / assign36150_e50910);
        (assign36150_e50911, (-((locals.var_t4_dn0 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn2 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn6 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn7 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn10 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn11 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn12 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn17 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign36150_e50913;
        locals.var_t5_dn0 = assign36150_e50913_d_n0;
        locals.var_t5_dn2 = assign36150_e50913_d_n2;
        locals.var_t5_dn6 = assign36150_e50913_d_n6;
        locals.var_t5_dn7 = assign36150_e50913_d_n7;
        locals.var_t5_dn10 = assign36150_e50913_d_n10;
        locals.var_t5_dn11 = assign36150_e50913_d_n11;
        locals.var_t5_dn12 = assign36150_e50913_d_n12;
        locals.var_t5_dn17 = assign36150_e50913_d_n17;

        let (assign36160_e50930, assign36160_e50930_d_n0, assign36160_e50930_d_n2, assign36160_e50930_d_n6, assign36160_e50930_d_n7, assign36160_e50930_d_n10, assign36160_e50930_d_n11, assign36160_e50930_d_n12, assign36160_e50930_d_n17,) = {
    if (((locals.var_guard1177 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
        let assign36160_e50923: f64 = (-1.0);
        let assign36160_e50925: f64 = (assign36160_e50923 / locals.var_rrdrbb__blk1183);
        let assign36160_e50927: f64 = (assign36160_e50925 - 1.0);
        let assign36160_e50928: f64 = (locals.var_t4).powf(assign36160_e50927);
        (assign36160_e50928, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn0)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn2)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn6)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn7)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign36160_e50923 * locals.var_rrdrbb__blk1183_dn10) / (locals.var_rrdrbb__blk1183 * locals.var_rrdrbb__blk1183))) == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn10)) } } else { (assign36160_e50928 * (((-((assign36160_e50923 * locals.var_rrdrbb__blk1183_dn10) / (locals.var_rrdrbb__blk1183 * locals.var_rrdrbb__blk1183))) * (locals.var_t4).ln()) + (assign36160_e50927 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn11)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn12)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn17)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn17 / locals.var_t4))) },)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign36160_e50930;
        locals.var_t6_dn0 = assign36160_e50930_d_n0;
        locals.var_t6_dn2 = assign36160_e50930_d_n2;
        locals.var_t6_dn6 = assign36160_e50930_d_n6;
        locals.var_t6_dn7 = assign36160_e50930_d_n7;
        locals.var_t6_dn10 = assign36160_e50930_d_n10;
        locals.var_t6_dn11 = assign36160_e50930_d_n11;
        locals.var_t6_dn12 = assign36160_e50930_d_n12;
        locals.var_t6_dn17 = assign36160_e50930_d_n17;

        let (assign36170_e50942, assign36170_e50942_d_n0, assign36170_e50942_d_n2, assign36170_e50942_d_n6, assign36170_e50942_d_n7, assign36170_e50942_d_n10, assign36170_e50942_d_n11, assign36170_e50942_d_n12, assign36170_e50942_d_n17,) = {
    if (((locals.var_guard1177 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
        let assign36170_e50940: f64 = (locals.var_t4 * locals.var_t6);
        (assign36170_e50940, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign36170_e50942;
        locals.var_t5_dn0 = assign36170_e50942_d_n0;
        locals.var_t5_dn2 = assign36170_e50942_d_n2;
        locals.var_t5_dn6 = assign36170_e50942_d_n6;
        locals.var_t5_dn7 = assign36170_e50942_d_n7;
        locals.var_t5_dn10 = assign36170_e50942_d_n10;
        locals.var_t5_dn11 = assign36170_e50942_d_n11;
        locals.var_t5_dn12 = assign36170_e50942_d_n12;
        locals.var_t5_dn17 = assign36170_e50942_d_n17;

        let (assign36190_e50954, assign36190_e50954_d_n0, assign36190_e50954_d_n2, assign36190_e50954_d_n6, assign36190_e50954_d_n7, assign36190_e50954_d_n10, assign36190_e50954_d_n11, assign36190_e50954_d_n12, assign36190_e50954_d_n17,) = {
    if (locals.var_guard1177 != 0.0) {
        let assign36190_e50952: f64 = (1.6021918e-19 / locals.var_ldrifte__blk1187);
        (assign36190_e50952, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign36190_e50954;
        locals.var_t1_dn0 = assign36190_e50954_d_n0;
        locals.var_t1_dn2 = assign36190_e50954_d_n2;
        locals.var_t1_dn6 = assign36190_e50954_d_n6;
        locals.var_t1_dn7 = assign36190_e50954_d_n7;
        locals.var_t1_dn10 = assign36190_e50954_d_n10;
        locals.var_t1_dn11 = assign36190_e50954_d_n11;
        locals.var_t1_dn12 = assign36190_e50954_d_n12;
        locals.var_t1_dn17 = assign36190_e50954_d_n17;

        let assign36310_e51028: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign36310_e51028;

        let (assign36360_e51071, assign36360_e51071_d_n0, assign36360_e51071_d_n2, assign36360_e51071_d_n6, assign36360_e51071_d_n7, assign36360_e51071_d_n10, assign36360_e51071_d_n11, assign36360_e51071_d_n12, assign36360_e51071_d_n17,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let (assign36360_e51069, assign36360_e51069_d_n0, assign36360_e51069_d_n2, assign36360_e51069_d_n6, assign36360_e51069_d_n7, assign36360_e51069_d_n10, assign36360_e51069_d_n11, assign36360_e51069_d_n12, assign36360_e51069_d_n17,) = {
            if (locals.var_mode == 1.0) {
                (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
            } else {
                let assign36360_e51068: f64 = (1.0 - locals.var_xd);
                (assign36360_e51068, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn12), (-locals.var_xd_dn17),)
            }
        };
        (assign36360_e51069, assign36360_e51069_d_n0, assign36360_e51069_d_n2, assign36360_e51069_d_n6, assign36360_e51069_d_n7, assign36360_e51069_d_n10, assign36360_e51069_d_n11, assign36360_e51069_d_n12, assign36360_e51069_d_n17,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign36360_e51071;
        locals.var_qdrat_dn0 = assign36360_e51071_d_n0;
        locals.var_qdrat_dn2 = assign36360_e51071_d_n2;
        locals.var_qdrat_dn6 = assign36360_e51071_d_n6;
        locals.var_qdrat_dn7 = assign36360_e51071_d_n7;
        locals.var_qdrat_dn10 = assign36360_e51071_d_n10;
        locals.var_qdrat_dn11 = assign36360_e51071_d_n11;
        locals.var_qdrat_dn12 = assign36360_e51071_d_n12;
        locals.var_qdrat_dn17 = assign36360_e51071_d_n17;

        let (assign36390_e51101, assign36390_e51101_d_n0, assign36390_e51101_d_n2, assign36390_e51101_d_n6, assign36390_e51101_d_n7, assign36390_e51101_d_n10, assign36390_e51101_d_n11, assign36390_e51101_d_n12, assign36390_e51101_d_n15, assign36390_e51101_d_n17, assign36390_e51101_d_n18,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36390_e51097: f64 = (locals.var_qi_nqs * locals.var_qdrat);
        let assign36390_e51099: f64 = (assign36390_e51097 + locals.var_q_bt_se);
        (assign36390_e51099, ((locals.var_qi_nqs * locals.var_qdrat_dn0) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * locals.var_qdrat_dn2) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * locals.var_qdrat_dn6) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * locals.var_qdrat_dn7) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * locals.var_qdrat_dn10) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * locals.var_qdrat_dn11) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * locals.var_qdrat_dn12) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * locals.var_qdrat_dn17) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * locals.var_qdrat),)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign36390_e51101;
        locals.var_qd_nqs_dn0 = assign36390_e51101_d_n0;
        locals.var_qd_nqs_dn2 = assign36390_e51101_d_n2;
        locals.var_qd_nqs_dn6 = assign36390_e51101_d_n6;
        locals.var_qd_nqs_dn7 = assign36390_e51101_d_n7;
        locals.var_qd_nqs_dn10 = assign36390_e51101_d_n10;
        locals.var_qd_nqs_dn11 = assign36390_e51101_d_n11;
        locals.var_qd_nqs_dn12 = assign36390_e51101_d_n12;
        locals.var_qd_nqs_dn15 = assign36390_e51101_d_n15;
        locals.var_qd_nqs_dn17 = assign36390_e51101_d_n17;
        locals.var_qd_nqs_dn18 = assign36390_e51101_d_n18;

        let (assign36400_e51113, assign36400_e51113_d_n0, assign36400_e51113_d_n2, assign36400_e51113_d_n6, assign36400_e51113_d_n7, assign36400_e51113_d_n10, assign36400_e51113_d_n11, assign36400_e51113_d_n12, assign36400_e51113_d_n16, assign36400_e51113_d_n17, assign36400_e51113_d_n18,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36400_e51108: f64 = (1.0 - locals.var_qdrat);
        let assign36400_e51109: f64 = (locals.var_qi_nqs * assign36400_e51108);
        let assign36400_e51111: f64 = (assign36400_e51109 + locals.var_q_bt_se);
        (assign36400_e51111, ((locals.var_qi_nqs * (-locals.var_qdrat_dn0)) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * (-locals.var_qdrat_dn2)) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * (-locals.var_qdrat_dn6)) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * (-locals.var_qdrat_dn7)) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * (-locals.var_qdrat_dn10)) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * (-locals.var_qdrat_dn11)) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * (-locals.var_qdrat_dn12)) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * (-locals.var_qdrat_dn17)) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * assign36400_e51108),)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36400_e51113;
        locals.var_qs_nqs_dn0 = assign36400_e51113_d_n0;
        locals.var_qs_nqs_dn2 = assign36400_e51113_d_n2;
        locals.var_qs_nqs_dn6 = assign36400_e51113_d_n6;
        locals.var_qs_nqs_dn7 = assign36400_e51113_d_n7;
        locals.var_qs_nqs_dn10 = assign36400_e51113_d_n10;
        locals.var_qs_nqs_dn11 = assign36400_e51113_d_n11;
        locals.var_qs_nqs_dn12 = assign36400_e51113_d_n12;
        locals.var_qs_nqs_dn16 = assign36400_e51113_d_n16;
        locals.var_qs_nqs_dn17 = assign36400_e51113_d_n17;
        locals.var_qs_nqs_dn18 = assign36400_e51113_d_n18;

        let (assign36410_e51124, assign36410_e51124_d_n0, assign36410_e51124_d_n2, assign36410_e51124_d_n6, assign36410_e51124_d_n7, assign36410_e51124_d_n10, assign36410_e51124_d_n11, assign36410_e51124_d_n12, assign36410_e51124_d_n13, assign36410_e51124_d_n15, assign36410_e51124_d_n16, assign36410_e51124_d_n17, assign36410_e51124_d_n18,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36410_e51118: f64 = (-locals.var_qi_nqs);
        let assign36410_e51120: f64 = (assign36410_e51118 - locals.var_qb_nqs);
        let assign36410_e51122: f64 = (assign36410_e51120 + locals.var_q_bt_ge);
        (assign36410_e51122, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, (-locals.var_qb_nqs_dn13), 0.0, 0.0, locals.var_q_bt_ge_dn17, (-locals.var_qi_nqs_dn18),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36410_e51124;
        locals.var_qg_nqs_dn0 = assign36410_e51124_d_n0;
        locals.var_qg_nqs_dn2 = assign36410_e51124_d_n2;
        locals.var_qg_nqs_dn6 = assign36410_e51124_d_n6;
        locals.var_qg_nqs_dn7 = assign36410_e51124_d_n7;
        locals.var_qg_nqs_dn10 = assign36410_e51124_d_n10;
        locals.var_qg_nqs_dn11 = assign36410_e51124_d_n11;
        locals.var_qg_nqs_dn12 = assign36410_e51124_d_n12;
        locals.var_qg_nqs_dn13 = assign36410_e51124_d_n13;
        locals.var_qg_nqs_dn15 = assign36410_e51124_d_n15;
        locals.var_qg_nqs_dn16 = assign36410_e51124_d_n16;
        locals.var_qg_nqs_dn17 = assign36410_e51124_d_n17;
        locals.var_qg_nqs_dn18 = assign36410_e51124_d_n18;

        let (assign36440_e51145, assign36440_e51145_d_n0, assign36440_e51145_d_n2, assign36440_e51145_d_n6, assign36440_e51145_d_n7, assign36440_e51145_d_n10, assign36440_e51145_d_n11, assign36440_e51145_d_n12, assign36440_e51145_d_n15, assign36440_e51145_d_n17, assign36440_e51145_d_n18,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign36440_e51145;
        locals.var_qd_nqs_dn0 = assign36440_e51145_d_n0;
        locals.var_qd_nqs_dn2 = assign36440_e51145_d_n2;
        locals.var_qd_nqs_dn6 = assign36440_e51145_d_n6;
        locals.var_qd_nqs_dn7 = assign36440_e51145_d_n7;
        locals.var_qd_nqs_dn10 = assign36440_e51145_d_n10;
        locals.var_qd_nqs_dn11 = assign36440_e51145_d_n11;
        locals.var_qd_nqs_dn12 = assign36440_e51145_d_n12;
        locals.var_qd_nqs_dn15 = assign36440_e51145_d_n15;
        locals.var_qd_nqs_dn17 = assign36440_e51145_d_n17;
        locals.var_qd_nqs_dn18 = assign36440_e51145_d_n18;

        let (assign36450_e51152, assign36450_e51152_d_n0, assign36450_e51152_d_n2, assign36450_e51152_d_n6, assign36450_e51152_d_n7, assign36450_e51152_d_n10, assign36450_e51152_d_n11, assign36450_e51152_d_n12, assign36450_e51152_d_n16, assign36450_e51152_d_n17, assign36450_e51152_d_n18,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36450_e51152;
        locals.var_qs_nqs_dn0 = assign36450_e51152_d_n0;
        locals.var_qs_nqs_dn2 = assign36450_e51152_d_n2;
        locals.var_qs_nqs_dn6 = assign36450_e51152_d_n6;
        locals.var_qs_nqs_dn7 = assign36450_e51152_d_n7;
        locals.var_qs_nqs_dn10 = assign36450_e51152_d_n10;
        locals.var_qs_nqs_dn11 = assign36450_e51152_d_n11;
        locals.var_qs_nqs_dn12 = assign36450_e51152_d_n12;
        locals.var_qs_nqs_dn16 = assign36450_e51152_d_n16;
        locals.var_qs_nqs_dn17 = assign36450_e51152_d_n17;
        locals.var_qs_nqs_dn18 = assign36450_e51152_d_n18;

        let (assign36460_e51159, assign36460_e51159_d_n0, assign36460_e51159_d_n2, assign36460_e51159_d_n6, assign36460_e51159_d_n7, assign36460_e51159_d_n10, assign36460_e51159_d_n11, assign36460_e51159_d_n12, assign36460_e51159_d_n13, assign36460_e51159_d_n15, assign36460_e51159_d_n16, assign36460_e51159_d_n17, assign36460_e51159_d_n18,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36460_e51159;
        locals.var_qg_nqs_dn0 = assign36460_e51159_d_n0;
        locals.var_qg_nqs_dn2 = assign36460_e51159_d_n2;
        locals.var_qg_nqs_dn6 = assign36460_e51159_d_n6;
        locals.var_qg_nqs_dn7 = assign36460_e51159_d_n7;
        locals.var_qg_nqs_dn10 = assign36460_e51159_d_n10;
        locals.var_qg_nqs_dn11 = assign36460_e51159_d_n11;
        locals.var_qg_nqs_dn12 = assign36460_e51159_d_n12;
        locals.var_qg_nqs_dn13 = assign36460_e51159_d_n13;
        locals.var_qg_nqs_dn15 = assign36460_e51159_d_n15;
        locals.var_qg_nqs_dn16 = assign36460_e51159_d_n16;
        locals.var_qg_nqs_dn17 = assign36460_e51159_d_n17;
        locals.var_qg_nqs_dn18 = assign36460_e51159_d_n18;

        let (assign36470_e51166, assign36470_e51166_d_n13,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign36470_e51166;
        locals.var_qb_nqs_dn13 = assign36470_e51166_d_n13;

        let (assign36560_e51250, assign36560_e51250_d_n0, assign36560_e51250_d_n2, assign36560_e51250_d_n6, assign36560_e51250_d_n7, assign36560_e51250_d_n10, assign36560_e51250_d_n11, assign36560_e51250_d_n12, assign36560_e51250_d_n13, assign36560_e51250_d_n15, assign36560_e51250_d_n16, assign36560_e51250_d_n17, assign36560_e51250_d_n18,) = {
    if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36560_e51244: f64 = (-locals.var_qd_nqs);
        let assign36560_e51246: f64 = (assign36560_e51244 - locals.var_qs_nqs);
        let assign36560_e51248: f64 = (assign36560_e51246 - locals.var_qb_nqs);
        (assign36560_e51248, ((-locals.var_qd_nqs_dn0) - locals.var_qs_nqs_dn0), ((-locals.var_qd_nqs_dn2) - locals.var_qs_nqs_dn2), ((-locals.var_qd_nqs_dn6) - locals.var_qs_nqs_dn6), ((-locals.var_qd_nqs_dn7) - locals.var_qs_nqs_dn7), ((-locals.var_qd_nqs_dn10) - locals.var_qs_nqs_dn10), ((-locals.var_qd_nqs_dn11) - locals.var_qs_nqs_dn11), ((-locals.var_qd_nqs_dn12) - locals.var_qs_nqs_dn12), (-locals.var_qb_nqs_dn13), (-locals.var_qd_nqs_dn15), (-locals.var_qs_nqs_dn16), ((-locals.var_qd_nqs_dn17) - locals.var_qs_nqs_dn17), ((-locals.var_qd_nqs_dn18) - locals.var_qs_nqs_dn18),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36560_e51250;
        locals.var_qg_nqs_dn0 = assign36560_e51250_d_n0;
        locals.var_qg_nqs_dn2 = assign36560_e51250_d_n2;
        locals.var_qg_nqs_dn6 = assign36560_e51250_d_n6;
        locals.var_qg_nqs_dn7 = assign36560_e51250_d_n7;
        locals.var_qg_nqs_dn10 = assign36560_e51250_d_n10;
        locals.var_qg_nqs_dn11 = assign36560_e51250_d_n11;
        locals.var_qg_nqs_dn12 = assign36560_e51250_d_n12;
        locals.var_qg_nqs_dn13 = assign36560_e51250_d_n13;
        locals.var_qg_nqs_dn15 = assign36560_e51250_d_n15;
        locals.var_qg_nqs_dn16 = assign36560_e51250_d_n16;
        locals.var_qg_nqs_dn17 = assign36560_e51250_d_n17;
        locals.var_qg_nqs_dn18 = assign36560_e51250_d_n18;

        let (assign36600_e51282, assign36600_e51282_d_n0, assign36600_e51282_d_n2, assign36600_e51282_d_n6, assign36600_e51282_d_n7, assign36600_e51282_d_n10, assign36600_e51282_d_n11, assign36600_e51282_d_n12, assign36600_e51282_d_n15, assign36600_e51282_d_n17, assign36600_e51282_d_n18,) = {
    if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign36600_e51282;
        locals.var_qd_nqs_dn0 = assign36600_e51282_d_n0;
        locals.var_qd_nqs_dn2 = assign36600_e51282_d_n2;
        locals.var_qd_nqs_dn6 = assign36600_e51282_d_n6;
        locals.var_qd_nqs_dn7 = assign36600_e51282_d_n7;
        locals.var_qd_nqs_dn10 = assign36600_e51282_d_n10;
        locals.var_qd_nqs_dn11 = assign36600_e51282_d_n11;
        locals.var_qd_nqs_dn12 = assign36600_e51282_d_n12;
        locals.var_qd_nqs_dn15 = assign36600_e51282_d_n15;
        locals.var_qd_nqs_dn17 = assign36600_e51282_d_n17;
        locals.var_qd_nqs_dn18 = assign36600_e51282_d_n18;

        let (assign36610_e51290, assign36610_e51290_d_n0, assign36610_e51290_d_n2, assign36610_e51290_d_n6, assign36610_e51290_d_n7, assign36610_e51290_d_n10, assign36610_e51290_d_n11, assign36610_e51290_d_n12, assign36610_e51290_d_n16, assign36610_e51290_d_n17, assign36610_e51290_d_n18,) = {
    if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36610_e51290;
        locals.var_qs_nqs_dn0 = assign36610_e51290_d_n0;
        locals.var_qs_nqs_dn2 = assign36610_e51290_d_n2;
        locals.var_qs_nqs_dn6 = assign36610_e51290_d_n6;
        locals.var_qs_nqs_dn7 = assign36610_e51290_d_n7;
        locals.var_qs_nqs_dn10 = assign36610_e51290_d_n10;
        locals.var_qs_nqs_dn11 = assign36610_e51290_d_n11;
        locals.var_qs_nqs_dn12 = assign36610_e51290_d_n12;
        locals.var_qs_nqs_dn16 = assign36610_e51290_d_n16;
        locals.var_qs_nqs_dn17 = assign36610_e51290_d_n17;
        locals.var_qs_nqs_dn18 = assign36610_e51290_d_n18;

        let (assign36620_e51298, assign36620_e51298_d_n0, assign36620_e51298_d_n2, assign36620_e51298_d_n6, assign36620_e51298_d_n7, assign36620_e51298_d_n10, assign36620_e51298_d_n11, assign36620_e51298_d_n12, assign36620_e51298_d_n13, assign36620_e51298_d_n15, assign36620_e51298_d_n16, assign36620_e51298_d_n17, assign36620_e51298_d_n18,) = {
    if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36620_e51298;
        locals.var_qg_nqs_dn0 = assign36620_e51298_d_n0;
        locals.var_qg_nqs_dn2 = assign36620_e51298_d_n2;
        locals.var_qg_nqs_dn6 = assign36620_e51298_d_n6;
        locals.var_qg_nqs_dn7 = assign36620_e51298_d_n7;
        locals.var_qg_nqs_dn10 = assign36620_e51298_d_n10;
        locals.var_qg_nqs_dn11 = assign36620_e51298_d_n11;
        locals.var_qg_nqs_dn12 = assign36620_e51298_d_n12;
        locals.var_qg_nqs_dn13 = assign36620_e51298_d_n13;
        locals.var_qg_nqs_dn15 = assign36620_e51298_d_n15;
        locals.var_qg_nqs_dn16 = assign36620_e51298_d_n16;
        locals.var_qg_nqs_dn17 = assign36620_e51298_d_n17;
        locals.var_qg_nqs_dn18 = assign36620_e51298_d_n18;

        let (assign36630_e51306, assign36630_e51306_d_n13,) = {
    if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign36630_e51306;
        locals.var_qb_nqs_dn13 = assign36630_e51306_d_n13;

        let assign36660_e51311: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign36660_e51311;

        let (assign36670_e51315, assign36670_e51315_d_n0, assign36670_e51315_d_n2, assign36670_e51315_d_n6, assign36670_e51315_d_n7, assign36670_e51315_d_n10, assign36670_e51315_d_n11, assign36670_e51315_d_n12, assign36670_e51315_d_n17,) = {
    if (locals.var_guard1210 != 0.0) {
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign36670_e51315;
        locals.var_ids_dn0 = assign36670_e51315_d_n0;
        locals.var_ids_dn2 = assign36670_e51315_d_n2;
        locals.var_ids_dn6 = assign36670_e51315_d_n6;
        locals.var_ids_dn7 = assign36670_e51315_d_n7;
        locals.var_ids_dn10 = assign36670_e51315_d_n10;
        locals.var_ids_dn11 = assign36670_e51315_d_n11;
        locals.var_ids_dn12 = assign36670_e51315_d_n12;
        locals.var_ids_dn17 = assign36670_e51315_d_n17;

        let (assign36680_e51319, assign36680_e51319_d_n0, assign36680_e51319_d_n2, assign36680_e51319_d_n6, assign36680_e51319_d_n7, assign36680_e51319_d_n10, assign36680_e51319_d_n11, assign36680_e51319_d_n12, assign36680_e51319_d_n17,) = {
    if (locals.var_guard1210 != 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign36680_e51319;
        locals.var_isub_dn0 = assign36680_e51319_d_n0;
        locals.var_isub_dn2 = assign36680_e51319_d_n2;
        locals.var_isub_dn6 = assign36680_e51319_d_n6;
        locals.var_isub_dn7 = assign36680_e51319_d_n7;
        locals.var_isub_dn10 = assign36680_e51319_d_n10;
        locals.var_isub_dn11 = assign36680_e51319_d_n11;
        locals.var_isub_dn12 = assign36680_e51319_d_n12;
        locals.var_isub_dn17 = assign36680_e51319_d_n17;

    }

    pub(super) fn stamp_transient_block_125(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36690_e51323, assign36690_e51323_d_n0, assign36690_e51323_d_n2, assign36690_e51323_d_n6, assign36690_e51323_d_n7, assign36690_e51323_d_n10, assign36690_e51323_d_n11, assign36690_e51323_d_n12, assign36690_e51323_d_n17,) = {
    if (locals.var_guard1210 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isubs, locals.var_isubs_dn0, locals.var_isubs_dn2, locals.var_isubs_dn6, locals.var_isubs_dn7, locals.var_isubs_dn10, locals.var_isubs_dn11, locals.var_isubs_dn12, locals.var_isubs_dn17,)
    }
};
        locals.var_isubs = assign36690_e51323;
        locals.var_isubs_dn0 = assign36690_e51323_d_n0;
        locals.var_isubs_dn2 = assign36690_e51323_d_n2;
        locals.var_isubs_dn6 = assign36690_e51323_d_n6;
        locals.var_isubs_dn7 = assign36690_e51323_d_n7;
        locals.var_isubs_dn10 = assign36690_e51323_d_n10;
        locals.var_isubs_dn11 = assign36690_e51323_d_n11;
        locals.var_isubs_dn12 = assign36690_e51323_d_n12;
        locals.var_isubs_dn17 = assign36690_e51323_d_n17;

        let (assign36700_e51329, assign36700_e51329_d_n0, assign36700_e51329_d_n2, assign36700_e51329_d_n6, assign36700_e51329_d_n7, assign36700_e51329_d_n10, assign36700_e51329_d_n11, assign36700_e51329_d_n12, assign36700_e51329_d_n13, assign36700_e51329_d_n15, assign36700_e51329_d_n16, assign36700_e51329_d_n17, assign36700_e51329_d_n18,) = {
    if (locals.var_guard1210 != 0.0) {
        let assign36700_e51327: f64 = (locals.var_qge + locals.var_qg_nqs);
        (assign36700_e51327, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18),)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18,)
    }
};
        locals.var_qg = assign36700_e51329;
        locals.var_qg_dn0 = assign36700_e51329_d_n0;
        locals.var_qg_dn2 = assign36700_e51329_d_n2;
        locals.var_qg_dn6 = assign36700_e51329_d_n6;
        locals.var_qg_dn7 = assign36700_e51329_d_n7;
        locals.var_qg_dn10 = assign36700_e51329_d_n10;
        locals.var_qg_dn11 = assign36700_e51329_d_n11;
        locals.var_qg_dn12 = assign36700_e51329_d_n12;
        locals.var_qg_dn13 = assign36700_e51329_d_n13;
        locals.var_qg_dn15 = assign36700_e51329_d_n15;
        locals.var_qg_dn16 = assign36700_e51329_d_n16;
        locals.var_qg_dn17 = assign36700_e51329_d_n17;
        locals.var_qg_dn18 = assign36700_e51329_d_n18;

        let (assign36710_e51335, assign36710_e51335_d_n0, assign36710_e51335_d_n2, assign36710_e51335_d_n6, assign36710_e51335_d_n7, assign36710_e51335_d_n10, assign36710_e51335_d_n11, assign36710_e51335_d_n12, assign36710_e51335_d_n13, assign36710_e51335_d_n15, assign36710_e51335_d_n16, assign36710_e51335_d_n17, assign36710_e51335_d_n18,) = {
    if (locals.var_guard1210 != 0.0) {
        let assign36710_e51333: f64 = (locals.var_qde + locals.var_qd_nqs);
        (assign36710_e51333, (locals.var_qde_dn0 + locals.var_qd_nqs_dn0), (locals.var_qde_dn2 + locals.var_qd_nqs_dn2), (locals.var_qde_dn6 + locals.var_qd_nqs_dn6), (locals.var_qde_dn7 + locals.var_qd_nqs_dn7), (locals.var_qde_dn10 + locals.var_qd_nqs_dn10), (locals.var_qde_dn11 + locals.var_qd_nqs_dn11), (locals.var_qde_dn12 + locals.var_qd_nqs_dn12), locals.var_qde_dn13, (locals.var_qde_dn15 + locals.var_qd_nqs_dn15), locals.var_qde_dn16, (locals.var_qde_dn17 + locals.var_qd_nqs_dn17), (locals.var_qde_dn18 + locals.var_qd_nqs_dn18),)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign36710_e51335;
        locals.var_qd_dn0 = assign36710_e51335_d_n0;
        locals.var_qd_dn2 = assign36710_e51335_d_n2;
        locals.var_qd_dn6 = assign36710_e51335_d_n6;
        locals.var_qd_dn7 = assign36710_e51335_d_n7;
        locals.var_qd_dn10 = assign36710_e51335_d_n10;
        locals.var_qd_dn11 = assign36710_e51335_d_n11;
        locals.var_qd_dn12 = assign36710_e51335_d_n12;
        locals.var_qd_dn13 = assign36710_e51335_d_n13;
        locals.var_qd_dn15 = assign36710_e51335_d_n15;
        locals.var_qd_dn16 = assign36710_e51335_d_n16;
        locals.var_qd_dn17 = assign36710_e51335_d_n17;
        locals.var_qd_dn18 = assign36710_e51335_d_n18;

        let (assign36730_e51350, assign36730_e51350_d_n0, assign36730_e51350_d_n2, assign36730_e51350_d_n6, assign36730_e51350_d_n7, assign36730_e51350_d_n10, assign36730_e51350_d_n11, assign36730_e51350_d_n12, assign36730_e51350_d_n13, assign36730_e51350_d_n15, assign36730_e51350_d_n16, assign36730_e51350_d_n17, assign36730_e51350_d_n18,) = {
    if (locals.var_guard1210 != 0.0) {
        let assign36730_e51345: f64 = (locals.var_qge + locals.var_qde);
        let assign36730_e51347: f64 = (assign36730_e51345 + locals.var_qse);
        let assign36730_e51348: f64 = (-assign36730_e51347);
        (assign36730_e51348, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign36730_e51350;
        locals.var_qbe_dn0 = assign36730_e51350_d_n0;
        locals.var_qbe_dn2 = assign36730_e51350_d_n2;
        locals.var_qbe_dn6 = assign36730_e51350_d_n6;
        locals.var_qbe_dn7 = assign36730_e51350_d_n7;
        locals.var_qbe_dn10 = assign36730_e51350_d_n10;
        locals.var_qbe_dn11 = assign36730_e51350_d_n11;
        locals.var_qbe_dn12 = assign36730_e51350_d_n12;
        locals.var_qbe_dn13 = assign36730_e51350_d_n13;
        locals.var_qbe_dn15 = assign36730_e51350_d_n15;
        locals.var_qbe_dn16 = assign36730_e51350_d_n16;
        locals.var_qbe_dn17 = assign36730_e51350_d_n17;
        locals.var_qbe_dn18 = assign36730_e51350_d_n18;

        let (assign36740_e51356, assign36740_e51356_d_n0, assign36740_e51356_d_n2, assign36740_e51356_d_n6, assign36740_e51356_d_n7, assign36740_e51356_d_n10, assign36740_e51356_d_n11, assign36740_e51356_d_n12, assign36740_e51356_d_n13, assign36740_e51356_d_n15, assign36740_e51356_d_n16, assign36740_e51356_d_n17, assign36740_e51356_d_n18,) = {
    if (locals.var_guard1210 != 0.0) {
        let assign36740_e51354: f64 = (locals.var_qbe + locals.var_qb_nqs);
        (assign36740_e51354, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign36740_e51356;
        locals.var_qb_dn0 = assign36740_e51356_d_n0;
        locals.var_qb_dn2 = assign36740_e51356_d_n2;
        locals.var_qb_dn6 = assign36740_e51356_d_n6;
        locals.var_qb_dn7 = assign36740_e51356_d_n7;
        locals.var_qb_dn10 = assign36740_e51356_d_n10;
        locals.var_qb_dn11 = assign36740_e51356_d_n11;
        locals.var_qb_dn12 = assign36740_e51356_d_n12;
        locals.var_qb_dn13 = assign36740_e51356_d_n13;
        locals.var_qb_dn15 = assign36740_e51356_d_n15;
        locals.var_qb_dn16 = assign36740_e51356_d_n16;
        locals.var_qb_dn17 = assign36740_e51356_d_n17;
        locals.var_qb_dn18 = assign36740_e51356_d_n18;

        let (assign36750_e51362, assign36750_e51362_d_n0, assign36750_e51362_d_n2, assign36750_e51362_d_n6, assign36750_e51362_d_n7, assign36750_e51362_d_n10, assign36750_e51362_d_n11, assign36750_e51362_d_n12, assign36750_e51362_d_n17,) = {
    if (locals.var_guard1210 == 0.0) {
        let assign36750_e51360: f64 = (-locals.var_idse);
        (assign36750_e51360, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12), (-locals.var_idse_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign36750_e51362;
        locals.var_ids_dn0 = assign36750_e51362_d_n0;
        locals.var_ids_dn2 = assign36750_e51362_d_n2;
        locals.var_ids_dn6 = assign36750_e51362_d_n6;
        locals.var_ids_dn7 = assign36750_e51362_d_n7;
        locals.var_ids_dn10 = assign36750_e51362_d_n10;
        locals.var_ids_dn11 = assign36750_e51362_d_n11;
        locals.var_ids_dn12 = assign36750_e51362_d_n12;
        locals.var_ids_dn17 = assign36750_e51362_d_n17;

        let (assign36760_e51367, assign36760_e51367_d_n0, assign36760_e51367_d_n2, assign36760_e51367_d_n6, assign36760_e51367_d_n7, assign36760_e51367_d_n10, assign36760_e51367_d_n11, assign36760_e51367_d_n12, assign36760_e51367_d_n17,) = {
    if (locals.var_guard1210 == 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    } else {
        (locals.var_isubs, locals.var_isubs_dn0, locals.var_isubs_dn2, locals.var_isubs_dn6, locals.var_isubs_dn7, locals.var_isubs_dn10, locals.var_isubs_dn11, locals.var_isubs_dn12, locals.var_isubs_dn17,)
    }
};
        locals.var_isubs = assign36760_e51367;
        locals.var_isubs_dn0 = assign36760_e51367_d_n0;
        locals.var_isubs_dn2 = assign36760_e51367_d_n2;
        locals.var_isubs_dn6 = assign36760_e51367_d_n6;
        locals.var_isubs_dn7 = assign36760_e51367_d_n7;
        locals.var_isubs_dn10 = assign36760_e51367_d_n10;
        locals.var_isubs_dn11 = assign36760_e51367_d_n11;
        locals.var_isubs_dn12 = assign36760_e51367_d_n12;
        locals.var_isubs_dn17 = assign36760_e51367_d_n17;

        let (assign36770_e51372, assign36770_e51372_d_n0, assign36770_e51372_d_n2, assign36770_e51372_d_n6, assign36770_e51372_d_n7, assign36770_e51372_d_n10, assign36770_e51372_d_n11, assign36770_e51372_d_n12, assign36770_e51372_d_n17,) = {
    if (locals.var_guard1210 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign36770_e51372;
        locals.var_isub_dn0 = assign36770_e51372_d_n0;
        locals.var_isub_dn2 = assign36770_e51372_d_n2;
        locals.var_isub_dn6 = assign36770_e51372_d_n6;
        locals.var_isub_dn7 = assign36770_e51372_d_n7;
        locals.var_isub_dn10 = assign36770_e51372_d_n10;
        locals.var_isub_dn11 = assign36770_e51372_d_n11;
        locals.var_isub_dn12 = assign36770_e51372_d_n12;
        locals.var_isub_dn17 = assign36770_e51372_d_n17;

        let (assign36780_e51379, assign36780_e51379_d_n0, assign36780_e51379_d_n2, assign36780_e51379_d_n6, assign36780_e51379_d_n7, assign36780_e51379_d_n10, assign36780_e51379_d_n11, assign36780_e51379_d_n12, assign36780_e51379_d_n13, assign36780_e51379_d_n15, assign36780_e51379_d_n16, assign36780_e51379_d_n17, assign36780_e51379_d_n18,) = {
    if (locals.var_guard1210 == 0.0) {
        let assign36780_e51377: f64 = (locals.var_qge + locals.var_qg_nqs);
        (assign36780_e51377, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18),)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18,)
    }
};
        locals.var_qg = assign36780_e51379;
        locals.var_qg_dn0 = assign36780_e51379_d_n0;
        locals.var_qg_dn2 = assign36780_e51379_d_n2;
        locals.var_qg_dn6 = assign36780_e51379_d_n6;
        locals.var_qg_dn7 = assign36780_e51379_d_n7;
        locals.var_qg_dn10 = assign36780_e51379_d_n10;
        locals.var_qg_dn11 = assign36780_e51379_d_n11;
        locals.var_qg_dn12 = assign36780_e51379_d_n12;
        locals.var_qg_dn13 = assign36780_e51379_d_n13;
        locals.var_qg_dn15 = assign36780_e51379_d_n15;
        locals.var_qg_dn16 = assign36780_e51379_d_n16;
        locals.var_qg_dn17 = assign36780_e51379_d_n17;
        locals.var_qg_dn18 = assign36780_e51379_d_n18;

        let (assign36790_e51386, assign36790_e51386_d_n0, assign36790_e51386_d_n2, assign36790_e51386_d_n6, assign36790_e51386_d_n7, assign36790_e51386_d_n10, assign36790_e51386_d_n11, assign36790_e51386_d_n12, assign36790_e51386_d_n13, assign36790_e51386_d_n15, assign36790_e51386_d_n16, assign36790_e51386_d_n17, assign36790_e51386_d_n18,) = {
    if (locals.var_guard1210 == 0.0) {
        let assign36790_e51384: f64 = (locals.var_qse + locals.var_qs_nqs);
        (assign36790_e51384, (locals.var_qse_dn0 + locals.var_qs_nqs_dn0), (locals.var_qse_dn2 + locals.var_qs_nqs_dn2), (locals.var_qse_dn6 + locals.var_qs_nqs_dn6), (locals.var_qse_dn7 + locals.var_qs_nqs_dn7), (locals.var_qse_dn10 + locals.var_qs_nqs_dn10), (locals.var_qse_dn11 + locals.var_qs_nqs_dn11), (locals.var_qse_dn12 + locals.var_qs_nqs_dn12), locals.var_qse_dn13, locals.var_qse_dn15, (locals.var_qse_dn16 + locals.var_qs_nqs_dn16), (locals.var_qse_dn17 + locals.var_qs_nqs_dn17), (locals.var_qse_dn18 + locals.var_qs_nqs_dn18),)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign36790_e51386;
        locals.var_qd_dn0 = assign36790_e51386_d_n0;
        locals.var_qd_dn2 = assign36790_e51386_d_n2;
        locals.var_qd_dn6 = assign36790_e51386_d_n6;
        locals.var_qd_dn7 = assign36790_e51386_d_n7;
        locals.var_qd_dn10 = assign36790_e51386_d_n10;
        locals.var_qd_dn11 = assign36790_e51386_d_n11;
        locals.var_qd_dn12 = assign36790_e51386_d_n12;
        locals.var_qd_dn13 = assign36790_e51386_d_n13;
        locals.var_qd_dn15 = assign36790_e51386_d_n15;
        locals.var_qd_dn16 = assign36790_e51386_d_n16;
        locals.var_qd_dn17 = assign36790_e51386_d_n17;
        locals.var_qd_dn18 = assign36790_e51386_d_n18;

        let (assign36810_e51403, assign36810_e51403_d_n0, assign36810_e51403_d_n2, assign36810_e51403_d_n6, assign36810_e51403_d_n7, assign36810_e51403_d_n10, assign36810_e51403_d_n11, assign36810_e51403_d_n12, assign36810_e51403_d_n13, assign36810_e51403_d_n15, assign36810_e51403_d_n16, assign36810_e51403_d_n17, assign36810_e51403_d_n18,) = {
    if (locals.var_guard1210 == 0.0) {
        let assign36810_e51398: f64 = (locals.var_qge + locals.var_qde);
        let assign36810_e51400: f64 = (assign36810_e51398 + locals.var_qse);
        let assign36810_e51401: f64 = (-assign36810_e51400);
        (assign36810_e51401, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign36810_e51403;
        locals.var_qbe_dn0 = assign36810_e51403_d_n0;
        locals.var_qbe_dn2 = assign36810_e51403_d_n2;
        locals.var_qbe_dn6 = assign36810_e51403_d_n6;
        locals.var_qbe_dn7 = assign36810_e51403_d_n7;
        locals.var_qbe_dn10 = assign36810_e51403_d_n10;
        locals.var_qbe_dn11 = assign36810_e51403_d_n11;
        locals.var_qbe_dn12 = assign36810_e51403_d_n12;
        locals.var_qbe_dn13 = assign36810_e51403_d_n13;
        locals.var_qbe_dn15 = assign36810_e51403_d_n15;
        locals.var_qbe_dn16 = assign36810_e51403_d_n16;
        locals.var_qbe_dn17 = assign36810_e51403_d_n17;
        locals.var_qbe_dn18 = assign36810_e51403_d_n18;

        let (assign36820_e51410, assign36820_e51410_d_n0, assign36820_e51410_d_n2, assign36820_e51410_d_n6, assign36820_e51410_d_n7, assign36820_e51410_d_n10, assign36820_e51410_d_n11, assign36820_e51410_d_n12, assign36820_e51410_d_n13, assign36820_e51410_d_n15, assign36820_e51410_d_n16, assign36820_e51410_d_n17, assign36820_e51410_d_n18,) = {
    if (locals.var_guard1210 == 0.0) {
        let assign36820_e51408: f64 = (locals.var_qbe + locals.var_qb_nqs);
        (assign36820_e51408, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign36820_e51410;
        locals.var_qb_dn0 = assign36820_e51410_d_n0;
        locals.var_qb_dn2 = assign36820_e51410_d_n2;
        locals.var_qb_dn6 = assign36820_e51410_d_n6;
        locals.var_qb_dn7 = assign36820_e51410_d_n7;
        locals.var_qb_dn10 = assign36820_e51410_d_n10;
        locals.var_qb_dn11 = assign36820_e51410_d_n11;
        locals.var_qb_dn12 = assign36820_e51410_d_n12;
        locals.var_qb_dn13 = assign36820_e51410_d_n13;
        locals.var_qb_dn15 = assign36820_e51410_d_n15;
        locals.var_qb_dn16 = assign36820_e51410_d_n16;
        locals.var_qb_dn17 = assign36820_e51410_d_n17;
        locals.var_qb_dn18 = assign36820_e51410_d_n18;

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

        let assign36880_e51418: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign36880_e51418;

        let (assign36890_e51422, assign36890_e51422_d_n0, assign36890_e51422_d_n2, assign36890_e51422_d_n6, assign36890_e51422_d_n7, assign36890_e51422_d_n10, assign36890_e51422_d_n11, assign36890_e51422_d_n12, assign36890_e51422_d_n17,) = {
    if (locals.var_guard1211 != 0.0) {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign36890_e51422;
        locals.var_ibd_dn0 = assign36890_e51422_d_n0;
        locals.var_ibd_dn2 = assign36890_e51422_d_n2;
        locals.var_ibd_dn6 = assign36890_e51422_d_n6;
        locals.var_ibd_dn7 = assign36890_e51422_d_n7;
        locals.var_ibd_dn10 = assign36890_e51422_d_n10;
        locals.var_ibd_dn11 = assign36890_e51422_d_n11;
        locals.var_ibd_dn12 = assign36890_e51422_d_n12;
        locals.var_ibd_dn17 = assign36890_e51422_d_n17;

        let (assign36900_e51426, assign36900_e51426_d_n0, assign36900_e51426_d_n2, assign36900_e51426_d_n6, assign36900_e51426_d_n7, assign36900_e51426_d_n10, assign36900_e51426_d_n11, assign36900_e51426_d_n12, assign36900_e51426_d_n17,) = {
    if (locals.var_guard1211 != 0.0) {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign36900_e51426;
        locals.var_qbd_dn0 = assign36900_e51426_d_n0;
        locals.var_qbd_dn2 = assign36900_e51426_d_n2;
        locals.var_qbd_dn6 = assign36900_e51426_d_n6;
        locals.var_qbd_dn7 = assign36900_e51426_d_n7;
        locals.var_qbd_dn10 = assign36900_e51426_d_n10;
        locals.var_qbd_dn11 = assign36900_e51426_d_n11;
        locals.var_qbd_dn12 = assign36900_e51426_d_n12;
        locals.var_qbd_dn17 = assign36900_e51426_d_n17;

        let (assign36910_e51430, assign36910_e51430_d_n0, assign36910_e51430_d_n2, assign36910_e51430_d_n6, assign36910_e51430_d_n7, assign36910_e51430_d_n10, assign36910_e51430_d_n11, assign36910_e51430_d_n12, assign36910_e51430_d_n17,) = {
    if (locals.var_guard1211 != 0.0) {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign36910_e51430;
        locals.var_ibs_dn0 = assign36910_e51430_d_n0;
        locals.var_ibs_dn2 = assign36910_e51430_d_n2;
        locals.var_ibs_dn6 = assign36910_e51430_d_n6;
        locals.var_ibs_dn7 = assign36910_e51430_d_n7;
        locals.var_ibs_dn10 = assign36910_e51430_d_n10;
        locals.var_ibs_dn11 = assign36910_e51430_d_n11;
        locals.var_ibs_dn12 = assign36910_e51430_d_n12;
        locals.var_ibs_dn17 = assign36910_e51430_d_n17;

        let (assign36920_e51434, assign36920_e51434_d_n0, assign36920_e51434_d_n2, assign36920_e51434_d_n6, assign36920_e51434_d_n7, assign36920_e51434_d_n10, assign36920_e51434_d_n11, assign36920_e51434_d_n12, assign36920_e51434_d_n17,) = {
    if (locals.var_guard1211 != 0.0) {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign36920_e51434;
        locals.var_qbs_dn0 = assign36920_e51434_d_n0;
        locals.var_qbs_dn2 = assign36920_e51434_d_n2;
        locals.var_qbs_dn6 = assign36920_e51434_d_n6;
        locals.var_qbs_dn7 = assign36920_e51434_d_n7;
        locals.var_qbs_dn10 = assign36920_e51434_d_n10;
        locals.var_qbs_dn11 = assign36920_e51434_d_n11;
        locals.var_qbs_dn12 = assign36920_e51434_d_n12;
        locals.var_qbs_dn17 = assign36920_e51434_d_n17;

        let assign36930_e51441: f64 = if ((p.p38 == 1.0) && (locals.var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign36930_e51441;

        let (assign36950_e51451,) = {
    if (locals.var_guard1212 != 0.0) {
        (locals.var_cth,)
    } else {
        (locals.var_cthe,)
    }
};
        locals.var_cthe = assign36950_e51451;

        let (assign36960_e51457,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign36960_e51455: f64 = (1.0 / locals.var_rth);
        (assign36960_e51455,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign36960_e51457;

        let (assign36980_e51467,) = {
    if (locals.var_guard1212 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cthe,)
    }
};
        locals.var_cthe = assign36980_e51467;

        let (assign36990_e51472,) = {
    if (locals.var_guard1212 == 0.0) {
        (0.0,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign36990_e51472;

        locals.var_idse = locals.var_ids;
        locals.var_idse_dn0 = locals.var_ids_dn0;
        locals.var_idse_dn2 = locals.var_ids_dn2;
        locals.var_idse_dn6 = locals.var_ids_dn6;
        locals.var_idse_dn7 = locals.var_ids_dn7;
        locals.var_idse_dn10 = locals.var_ids_dn10;
        locals.var_idse_dn11 = locals.var_ids_dn11;
        locals.var_idse_dn12 = locals.var_ids_dn12;
        locals.var_idse_dn17 = locals.var_ids_dn17;

        let assign37150_e51521: f64 = locals.var_qg_dn6;
        locals.var_cgdbd = assign37150_e51521;
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

        let assign37160_e51524: f64 = (p.p50 * locals.var_cgdbd);
        locals.var_cgdbd = assign37160_e51524;
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

        let assign37170_e51527: f64 = locals.var_qg_dn7;
        locals.var_cgsbd = assign37170_e51527;
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

        let assign37180_e51530: f64 = (p.p50 * locals.var_cgsbd);
        locals.var_cgsbd = assign37180_e51530;
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

        let assign37450_e51611: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign37450_e51611;

        let (assign37460_e51617, assign37460_e51617_d_n0, assign37460_e51617_d_n2, assign37460_e51617_d_n6, assign37460_e51617_d_n7, assign37460_e51617_d_n10, assign37460_e51617_d_n11, assign37460_e51617_d_n12, assign37460_e51617_d_n17,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign37460_e51615: f64 = (p.p50 * locals.var_ibd);
        (assign37460_e51615, (p.p50 * locals.var_ibd_dn0), (p.p50 * locals.var_ibd_dn2), (p.p50 * locals.var_ibd_dn6), (p.p50 * locals.var_ibd_dn7), (p.p50 * locals.var_ibd_dn10), (p.p50 * locals.var_ibd_dn11), (p.p50 * locals.var_ibd_dn12), (p.p50 * locals.var_ibd_dn17),)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign37460_e51617;
        locals.var_ibdb_dn0 = assign37460_e51617_d_n0;
        locals.var_ibdb_dn2 = assign37460_e51617_d_n2;
        locals.var_ibdb_dn6 = assign37460_e51617_d_n6;
        locals.var_ibdb_dn7 = assign37460_e51617_d_n7;
        locals.var_ibdb_dn10 = assign37460_e51617_d_n10;
        locals.var_ibdb_dn11 = assign37460_e51617_d_n11;
        locals.var_ibdb_dn12 = assign37460_e51617_d_n12;
        locals.var_ibdb_dn17 = assign37460_e51617_d_n17;

    }

    pub(super) fn stamp_transient_block_126(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign37470_e51623, assign37470_e51623_d_n0, assign37470_e51623_d_n2, assign37470_e51623_d_n6, assign37470_e51623_d_n7, assign37470_e51623_d_n10, assign37470_e51623_d_n11, assign37470_e51623_d_n12, assign37470_e51623_d_n17,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign37470_e51621: f64 = (p.p50 * locals.var_ibs);
        (assign37470_e51621, (p.p50 * locals.var_ibs_dn0), (p.p50 * locals.var_ibs_dn2), (p.p50 * locals.var_ibs_dn6), (p.p50 * locals.var_ibs_dn7), (p.p50 * locals.var_ibs_dn10), (p.p50 * locals.var_ibs_dn11), (p.p50 * locals.var_ibs_dn12), (p.p50 * locals.var_ibs_dn17),)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign37470_e51623;
        locals.var_ibsb_dn0 = assign37470_e51623_d_n0;
        locals.var_ibsb_dn2 = assign37470_e51623_d_n2;
        locals.var_ibsb_dn6 = assign37470_e51623_d_n6;
        locals.var_ibsb_dn7 = assign37470_e51623_d_n7;
        locals.var_ibsb_dn10 = assign37470_e51623_d_n10;
        locals.var_ibsb_dn11 = assign37470_e51623_d_n11;
        locals.var_ibsb_dn12 = assign37470_e51623_d_n12;
        locals.var_ibsb_dn17 = assign37470_e51623_d_n17;

        let assign37590_e51675: f64 = (4.0 * 1.3806226e-23);
        let assign37590_e51677: f64 = (assign37590_e51675 * locals.var_ttemp);
        let assign37590_e51679: f64 = assign37590_e51677;
        locals.var_whi_noise = assign37590_e51679;
        locals.var_whi_noise_dn10 = (assign37590_e51675 * locals.var_ttemp_dn10);

        let assign37600_e51682: f64 = if p.p27 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign37600_e51682;

        locals.var_qdrat = locals.var_qdrat_noi;
        locals.var_qdrat_dn0 = locals.var_qdrat_noi_dn0;
        locals.var_qdrat_dn2 = locals.var_qdrat_noi_dn2;
        locals.var_qdrat_dn6 = locals.var_qdrat_noi_dn6;
        locals.var_qdrat_dn7 = locals.var_qdrat_noi_dn7;
        locals.var_qdrat_dn10 = locals.var_qdrat_noi_dn10;
        locals.var_qdrat_dn11 = locals.var_qdrat_noi_dn11;
        locals.var_qdrat_dn12 = locals.var_qdrat_noi_dn12;
        locals.var_qdrat_dn17 = locals.var_qdrat_noi_dn17;

        let assign37620_e51686: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign37620_e51686;
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

        let (assign37640_e51700, assign37640_e51700_d_n0, assign37640_e51700_d_n2, assign37640_e51700_d_n6, assign37640_e51700_d_n7, assign37640_e51700_d_n10, assign37640_e51700_d_n11, assign37640_e51700_d_n12, assign37640_e51700_d_n13, assign37640_e51700_d_n15, assign37640_e51700_d_n16, assign37640_e51700_d_n17, assign37640_e51700_d_n18,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign37640_e51697: f64 = (locals.var_noiigate / locals.var_sid);
        let assign37640_e51698: f64 = (assign37640_e51697).sqrt();
        (assign37640_e51698, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn12 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn12)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn13 / locals.var_sid) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn15 / locals.var_sid) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn16 / locals.var_sid) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn17 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn17)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn18 / locals.var_sid) / (2.0 * assign37640_e51698)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign37640_e51700;
        locals.var_sigrat_dn0 = assign37640_e51700_d_n0;
        locals.var_sigrat_dn2 = assign37640_e51700_d_n2;
        locals.var_sigrat_dn6 = assign37640_e51700_d_n6;
        locals.var_sigrat_dn7 = assign37640_e51700_d_n7;
        locals.var_sigrat_dn10 = assign37640_e51700_d_n10;
        locals.var_sigrat_dn11 = assign37640_e51700_d_n11;
        locals.var_sigrat_dn12 = assign37640_e51700_d_n12;
        locals.var_sigrat_dn13 = assign37640_e51700_d_n13;
        locals.var_sigrat_dn15 = assign37640_e51700_d_n15;
        locals.var_sigrat_dn16 = assign37640_e51700_d_n16;
        locals.var_sigrat_dn17 = assign37640_e51700_d_n17;
        locals.var_sigrat_dn18 = assign37640_e51700_d_n18;

        let (assign37650_e51712, assign37650_e51712_d_n0, assign37650_e51712_d_n2, assign37650_e51712_d_n6, assign37650_e51712_d_n7, assign37650_e51712_d_n10, assign37650_e51712_d_n11, assign37650_e51712_d_n12, assign37650_e51712_d_n13, assign37650_e51712_d_n15, assign37650_e51712_d_n16, assign37650_e51712_d_n17, assign37650_e51712_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37650_e51707: f64 = (1.0 - locals.var_qdrat);
        let assign37650_e51708: f64 = (locals.var_sigrat * assign37650_e51707);
        (assign37650_e51708, ((locals.var_sigrat_dn0 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37650_e51707), (locals.var_sigrat_dn15 * assign37650_e51707), (locals.var_sigrat_dn16 * assign37650_e51707), ((locals.var_sigrat_dn17 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37650_e51707),)
    } else {
        let assign37650_e51711: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37650_e51711, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    }
};
        locals.var_sigrat_s = assign37650_e51712;
        locals.var_sigrat_s_dn0 = assign37650_e51712_d_n0;
        locals.var_sigrat_s_dn2 = assign37650_e51712_d_n2;
        locals.var_sigrat_s_dn6 = assign37650_e51712_d_n6;
        locals.var_sigrat_s_dn7 = assign37650_e51712_d_n7;
        locals.var_sigrat_s_dn10 = assign37650_e51712_d_n10;
        locals.var_sigrat_s_dn11 = assign37650_e51712_d_n11;
        locals.var_sigrat_s_dn12 = assign37650_e51712_d_n12;
        locals.var_sigrat_s_dn13 = assign37650_e51712_d_n13;
        locals.var_sigrat_s_dn15 = assign37650_e51712_d_n15;
        locals.var_sigrat_s_dn16 = assign37650_e51712_d_n16;
        locals.var_sigrat_s_dn17 = assign37650_e51712_d_n17;
        locals.var_sigrat_s_dn18 = assign37650_e51712_d_n18;

        let (assign37660_e51724, assign37660_e51724_d_n0, assign37660_e51724_d_n2, assign37660_e51724_d_n6, assign37660_e51724_d_n7, assign37660_e51724_d_n10, assign37660_e51724_d_n11, assign37660_e51724_d_n12, assign37660_e51724_d_n13, assign37660_e51724_d_n15, assign37660_e51724_d_n16, assign37660_e51724_d_n17, assign37660_e51724_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37660_e51718: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37660_e51718, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    } else {
        let assign37660_e51722: f64 = (1.0 - locals.var_qdrat);
        let assign37660_e51723: f64 = (locals.var_sigrat * assign37660_e51722);
        (assign37660_e51723, ((locals.var_sigrat_dn0 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37660_e51722), (locals.var_sigrat_dn15 * assign37660_e51722), (locals.var_sigrat_dn16 * assign37660_e51722), ((locals.var_sigrat_dn17 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37660_e51722),)
    }
};
        locals.var_sigrat_d = assign37660_e51724;
        locals.var_sigrat_d_dn0 = assign37660_e51724_d_n0;
        locals.var_sigrat_d_dn2 = assign37660_e51724_d_n2;
        locals.var_sigrat_d_dn6 = assign37660_e51724_d_n6;
        locals.var_sigrat_d_dn7 = assign37660_e51724_d_n7;
        locals.var_sigrat_d_dn10 = assign37660_e51724_d_n10;
        locals.var_sigrat_d_dn11 = assign37660_e51724_d_n11;
        locals.var_sigrat_d_dn12 = assign37660_e51724_d_n12;
        locals.var_sigrat_d_dn13 = assign37660_e51724_d_n13;
        locals.var_sigrat_d_dn15 = assign37660_e51724_d_n15;
        locals.var_sigrat_d_dn16 = assign37660_e51724_d_n16;
        locals.var_sigrat_d_dn17 = assign37660_e51724_d_n17;
        locals.var_sigrat_d_dn18 = assign37660_e51724_d_n18;

        let assign37680_e51734: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign37680_e51734;

        let assign37700_e51741: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign37700_e51741;

        let assign37710_e51750: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign37710_e51750;

    }

    pub(super) fn stamp_reactive_block_0(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_idd = 0.0;
        locals.var_idd_dn0 = 0.0;
        locals.var_idd_dn2 = 0.0;
        locals.var_idd_dn6 = 0.0;
        locals.var_idd_dn7 = 0.0;
        locals.var_idd_dn10 = 0.0;
        locals.var_idd_dn11 = 0.0;
        locals.var_idd_dn12 = 0.0;
        locals.var_idd_dn17 = 0.0;
        locals.var_idd_rv = 0.0;

        locals.var_gds0_ign = 1e-12;
        locals.var_gds0_ign_dn0 = 0.0;
        locals.var_gds0_ign_dn2 = 0.0;
        locals.var_gds0_ign_dn6 = 0.0;
        locals.var_gds0_ign_dn7 = 0.0;
        locals.var_gds0_ign_dn10 = 0.0;
        locals.var_gds0_ign_dn11 = 0.0;
        locals.var_gds0_ign_dn12 = 0.0;
        locals.var_gds0_ign_dn17 = 0.0;
        locals.var_gds0_ign_rv = 0.0;

        locals.var_qse = 0.0;
        locals.var_qse_dn0 = 0.0;
        locals.var_qse_dn2 = 0.0;
        locals.var_qse_dn6 = 0.0;
        locals.var_qse_dn7 = 0.0;
        locals.var_qse_dn10 = 0.0;
        locals.var_qse_dn11 = 0.0;
        locals.var_qse_dn12 = 0.0;
        locals.var_qse_dn13 = 0.0;
        locals.var_qse_dn15 = 0.0;
        locals.var_qse_dn16 = 0.0;
        locals.var_qse_dn17 = 0.0;
        locals.var_qse_dn18 = 0.0;
        locals.var_qse_rv = 0.0;

        locals.var_flg_ign = 0.0;
        locals.var_flg_ign_rv = 0.0;

        locals.var_end_of_part_1 = 0.0;
        locals.var_end_of_part_1_rv = 0.0;

        locals.var_xd = 0.0;
        locals.var_xd_dn0 = 0.0;
        locals.var_xd_dn2 = 0.0;
        locals.var_xd_dn6 = 0.0;
        locals.var_xd_dn7 = 0.0;
        locals.var_xd_dn10 = 0.0;
        locals.var_xd_dn11 = 0.0;
        locals.var_xd_dn12 = 0.0;
        locals.var_xd_dn17 = 0.0;
        locals.var_xd_rv = 0.0;

        locals.var_flg_noqi = 0.0;
        locals.var_flg_noqi_rv = 0.0;

        locals.var_flg_zone = 0.0;
        locals.var_flg_zone_rv = 0.0;

        locals.var_psl = 0.0;
        locals.var_psl_dn0 = 0.0;
        locals.var_psl_dn2 = 0.0;
        locals.var_psl_dn6 = 0.0;
        locals.var_psl_dn7 = 0.0;
        locals.var_psl_dn10 = 0.0;
        locals.var_psl_dn11 = 0.0;
        locals.var_psl_dn12 = 0.0;
        locals.var_psl_dn17 = 0.0;
        locals.var_psl_rv = 0.0;

        locals.var_psl_lim = 0.0;
        locals.var_psl_lim_dn0 = 0.0;
        locals.var_psl_lim_dn2 = 0.0;
        locals.var_psl_lim_dn6 = 0.0;
        locals.var_psl_lim_dn7 = 0.0;
        locals.var_psl_lim_dn10 = 0.0;
        locals.var_psl_lim_dn11 = 0.0;
        locals.var_psl_lim_dn12 = 0.0;
        locals.var_psl_lim_dn17 = 0.0;
        locals.var_psl_lim_rv = 0.0;

        locals.var_pds = 0.0;
        locals.var_pds_dn0 = 0.0;
        locals.var_pds_dn2 = 0.0;
        locals.var_pds_dn6 = 0.0;
        locals.var_pds_dn7 = 0.0;
        locals.var_pds_dn10 = 0.0;
        locals.var_pds_dn11 = 0.0;
        locals.var_pds_dn12 = 0.0;
        locals.var_pds_dn17 = 0.0;
        locals.var_pds_rv = 0.0;

        locals.var_pds_ini = 0.0;
        locals.var_pds_ini_dn0 = 0.0;
        locals.var_pds_ini_dn2 = 0.0;
        locals.var_pds_ini_dn6 = 0.0;
        locals.var_pds_ini_dn7 = 0.0;
        locals.var_pds_ini_dn10 = 0.0;
        locals.var_pds_ini_dn11 = 0.0;
        locals.var_pds_ini_dn12 = 0.0;
        locals.var_pds_ini_dn17 = 0.0;
        locals.var_pds_ini_rv = 0.0;

        locals.var_ps0z = 1.0;
        locals.var_ps0z_dn0 = 0.0;
        locals.var_ps0z_dn2 = 0.0;
        locals.var_ps0z_dn6 = 0.0;
        locals.var_ps0z_dn7 = 0.0;
        locals.var_ps0z_dn10 = 0.0;
        locals.var_ps0z_dn11 = 0.0;
        locals.var_ps0z_dn12 = 0.0;
        locals.var_ps0z_dn17 = 0.0;
        locals.var_ps0z_rv = 0.0;

        locals.var_alpha = 0.0;
        locals.var_alpha_dn0 = 0.0;
        locals.var_alpha_dn2 = 0.0;
        locals.var_alpha_dn6 = 0.0;
        locals.var_alpha_dn7 = 0.0;
        locals.var_alpha_dn10 = 0.0;
        locals.var_alpha_dn11 = 0.0;
        locals.var_alpha_dn12 = 0.0;
        locals.var_alpha_dn17 = 0.0;
        locals.var_alpha_rv = 0.0;

        locals.var_vgvt = 0.0;
        locals.var_vgvt_dn0 = 0.0;
        locals.var_vgvt_dn2 = 0.0;
        locals.var_vgvt_dn6 = 0.0;
        locals.var_vgvt_dn7 = 0.0;
        locals.var_vgvt_dn10 = 0.0;
        locals.var_vgvt_dn11 = 0.0;
        locals.var_vgvt_dn12 = 0.0;
        locals.var_vgvt_dn17 = 0.0;
        locals.var_vgvt_rv = 0.0;

        locals.var_qb = 0.0;
        locals.var_qb_dn0 = 0.0;
        locals.var_qb_dn2 = 0.0;
        locals.var_qb_dn6 = 0.0;
        locals.var_qb_dn7 = 0.0;
        locals.var_qb_dn10 = 0.0;
        locals.var_qb_dn11 = 0.0;
        locals.var_qb_dn12 = 0.0;
        locals.var_qb_dn13 = 0.0;
        locals.var_qb_dn15 = 0.0;
        locals.var_qb_dn16 = 0.0;
        locals.var_qb_dn17 = 0.0;
        locals.var_qb_dn18 = 0.0;
        locals.var_qb_rv = 0.0;

        locals.var_qi = 0.0;
        locals.var_qi_dn0 = 0.0;
        locals.var_qi_dn2 = 0.0;
        locals.var_qi_dn6 = 0.0;
        locals.var_qi_dn7 = 0.0;
        locals.var_qi_dn10 = 0.0;
        locals.var_qi_dn11 = 0.0;
        locals.var_qi_dn12 = 0.0;
        locals.var_qi_dn17 = 0.0;
        locals.var_qi_rv = 0.0;

        locals.var_qd = 0.0;
        locals.var_qd_dn0 = 0.0;
        locals.var_qd_dn2 = 0.0;
        locals.var_qd_dn6 = 0.0;
        locals.var_qd_dn7 = 0.0;
        locals.var_qd_dn10 = 0.0;
        locals.var_qd_dn11 = 0.0;
        locals.var_qd_dn12 = 0.0;
        locals.var_qd_dn13 = 0.0;
        locals.var_qd_dn15 = 0.0;
        locals.var_qd_dn16 = 0.0;
        locals.var_qd_dn17 = 0.0;
        locals.var_qd_dn18 = 0.0;
        locals.var_qd_rv = 0.0;

        locals.var_ids = 0.0;
        locals.var_ids_dn0 = 0.0;
        locals.var_ids_dn2 = 0.0;
        locals.var_ids_dn6 = 0.0;
        locals.var_ids_dn7 = 0.0;
        locals.var_ids_dn10 = 0.0;
        locals.var_ids_dn11 = 0.0;
        locals.var_ids_dn12 = 0.0;
        locals.var_ids_dn17 = 0.0;
        locals.var_ids_rv = 0.0;

        locals.var_fb = 0.0;
        locals.var_fb_dn0 = 0.0;
        locals.var_fb_dn2 = 0.0;
        locals.var_fb_dn6 = 0.0;
        locals.var_fb_dn7 = 0.0;
        locals.var_fb_dn10 = 0.0;
        locals.var_fb_dn11 = 0.0;
        locals.var_fb_dn12 = 0.0;
        locals.var_fb_dn17 = 0.0;
        locals.var_fb_rv = 0.0;

        locals.var_qn0 = 0.0;
        locals.var_qn0_dn0 = 0.0;
        locals.var_qn0_dn2 = 0.0;
        locals.var_qn0_dn6 = 0.0;
        locals.var_qn0_dn7 = 0.0;
        locals.var_qn0_dn10 = 0.0;
        locals.var_qn0_dn11 = 0.0;
        locals.var_qn0_dn12 = 0.0;
        locals.var_qn0_dn17 = 0.0;
        locals.var_qn0_rv = 0.0;

        locals.var_mu = 0.0;
        locals.var_mu_dn0 = 0.0;
        locals.var_mu_dn2 = 0.0;
        locals.var_mu_dn6 = 0.0;
        locals.var_mu_dn7 = 0.0;
        locals.var_mu_dn10 = 0.0;
        locals.var_mu_dn11 = 0.0;
        locals.var_mu_dn12 = 0.0;
        locals.var_mu_dn17 = 0.0;
        locals.var_mu_rv = 0.0;

        locals.var_muun = 0.0;
        locals.var_muun_dn0 = 0.0;
        locals.var_muun_dn2 = 0.0;
        locals.var_muun_dn6 = 0.0;
        locals.var_muun_dn7 = 0.0;
        locals.var_muun_dn10 = 0.0;
        locals.var_muun_dn11 = 0.0;
        locals.var_muun_dn12 = 0.0;
        locals.var_muun_dn17 = 0.0;
        locals.var_muun_rv = 0.0;

        locals.var_ey = 0.0;
        locals.var_ey_dn0 = 0.0;
        locals.var_ey_dn2 = 0.0;
        locals.var_ey_dn6 = 0.0;
        locals.var_ey_dn7 = 0.0;
        locals.var_ey_dn10 = 0.0;
        locals.var_ey_dn11 = 0.0;
        locals.var_ey_dn12 = 0.0;
        locals.var_ey_dn17 = 0.0;
        locals.var_ey_rv = 0.0;

        locals.var_isub = 0.0;
        locals.var_isub_dn0 = 0.0;
        locals.var_isub_dn2 = 0.0;
        locals.var_isub_dn6 = 0.0;
        locals.var_isub_dn7 = 0.0;
        locals.var_isub_dn10 = 0.0;
        locals.var_isub_dn11 = 0.0;
        locals.var_isub_dn12 = 0.0;
        locals.var_isub_dn17 = 0.0;
        locals.var_isub_rv = 0.0;

        locals.var_betawl = 1.0;
        locals.var_betawl_dn0 = 0.0;
        locals.var_betawl_dn2 = 0.0;
        locals.var_betawl_dn6 = 0.0;
        locals.var_betawl_dn7 = 0.0;
        locals.var_betawl_dn10 = 0.0;
        locals.var_betawl_dn11 = 0.0;
        locals.var_betawl_dn12 = 0.0;
        locals.var_betawl_dn17 = 0.0;
        locals.var_betawl_rv = 0.0;

        locals.var_idsibpc = 0.0;
        locals.var_idsibpc_dn0 = 0.0;
        locals.var_idsibpc_dn2 = 0.0;
        locals.var_idsibpc_dn6 = 0.0;
        locals.var_idsibpc_dn7 = 0.0;
        locals.var_idsibpc_dn10 = 0.0;
        locals.var_idsibpc_dn11 = 0.0;
        locals.var_idsibpc_dn12 = 0.0;
        locals.var_idsibpc_dn17 = 0.0;
        locals.var_idsibpc_rv = 0.0;

        locals.var_qgos = 0.0;
        locals.var_qgos_dn0 = 0.0;
        locals.var_qgos_dn2 = 0.0;
        locals.var_qgos_dn6 = 0.0;
        locals.var_qgos_dn7 = 0.0;
        locals.var_qgos_dn10 = 0.0;
        locals.var_qgos_dn11 = 0.0;
        locals.var_qgos_dn12 = 0.0;
        locals.var_qgos_dn17 = 0.0;
        locals.var_qgos_rv = 0.0;

        locals.var_qgod = 0.0;
        locals.var_qgod_dn0 = 0.0;
        locals.var_qgod_dn2 = 0.0;
        locals.var_qgod_dn6 = 0.0;
        locals.var_qgod_dn7 = 0.0;
        locals.var_qgod_dn10 = 0.0;
        locals.var_qgod_dn11 = 0.0;
        locals.var_qgod_dn12 = 0.0;
        locals.var_qgod_dn17 = 0.0;
        locals.var_qgod_rv = 0.0;

        locals.var_qgob = 0.0;
        locals.var_qgob_dn0 = 0.0;
        locals.var_qgob_dn2 = 0.0;
        locals.var_qgob_dn6 = 0.0;
        locals.var_qgob_dn7 = 0.0;
        locals.var_qgob_dn10 = 0.0;
        locals.var_qgob_dn11 = 0.0;
        locals.var_qgob_dn12 = 0.0;
        locals.var_qgob_dn17 = 0.0;
        locals.var_qgob_rv = 0.0;

        locals.var_qovd = 0.0;
        locals.var_qovd_dn0 = 0.0;
        locals.var_qovd_dn2 = 0.0;
        locals.var_qovd_dn6 = 0.0;
        locals.var_qovd_dn7 = 0.0;
        locals.var_qovd_dn10 = 0.0;
        locals.var_qovd_dn11 = 0.0;
        locals.var_qovd_dn12 = 0.0;
        locals.var_qovd_dn17 = 0.0;
        locals.var_qovd_rv = 0.0;

        locals.var_qovs = 0.0;
        locals.var_qovs_dn0 = 0.0;
        locals.var_qovs_dn2 = 0.0;
        locals.var_qovs_dn6 = 0.0;
        locals.var_qovs_dn7 = 0.0;
        locals.var_qovs_dn10 = 0.0;
        locals.var_qovs_dn11 = 0.0;
        locals.var_qovs_dn12 = 0.0;
        locals.var_qovs_dn17 = 0.0;
        locals.var_qovs_rv = 0.0;

        locals.var_qbdld = 0.0;
        locals.var_qbdld_dn0 = 0.0;
        locals.var_qbdld_dn2 = 0.0;
        locals.var_qbdld_dn6 = 0.0;
        locals.var_qbdld_dn7 = 0.0;
        locals.var_qbdld_dn10 = 0.0;
        locals.var_qbdld_dn11 = 0.0;
        locals.var_qbdld_dn12 = 0.0;
        locals.var_qbdld_dn17 = 0.0;
        locals.var_qbdld_rv = 0.0;

        locals.var_qbsld = 0.0;
        locals.var_qbsld_dn0 = 0.0;
        locals.var_qbsld_dn2 = 0.0;
        locals.var_qbsld_dn6 = 0.0;
        locals.var_qbsld_dn7 = 0.0;
        locals.var_qbsld_dn10 = 0.0;
        locals.var_qbsld_dn11 = 0.0;
        locals.var_qbsld_dn12 = 0.0;
        locals.var_qbsld_dn17 = 0.0;
        locals.var_qbsld_rv = 0.0;

        locals.var_ibd = 0.0;
        locals.var_ibd_dn0 = 0.0;
        locals.var_ibd_dn2 = 0.0;
        locals.var_ibd_dn6 = 0.0;
        locals.var_ibd_dn7 = 0.0;
        locals.var_ibd_dn10 = 0.0;
        locals.var_ibd_dn11 = 0.0;
        locals.var_ibd_dn12 = 0.0;
        locals.var_ibd_dn17 = 0.0;
        locals.var_ibd_rv = 0.0;

        locals.var_ibs = 0.0;
        locals.var_ibs_dn0 = 0.0;
        locals.var_ibs_dn2 = 0.0;
        locals.var_ibs_dn6 = 0.0;
        locals.var_ibs_dn7 = 0.0;
        locals.var_ibs_dn10 = 0.0;
        locals.var_ibs_dn11 = 0.0;
        locals.var_ibs_dn12 = 0.0;
        locals.var_ibs_dn17 = 0.0;
        locals.var_ibs_rv = 0.0;

        locals.var_qbd = 0.0;
        locals.var_qbd_dn0 = 0.0;
        locals.var_qbd_dn2 = 0.0;
        locals.var_qbd_dn6 = 0.0;
        locals.var_qbd_dn7 = 0.0;
        locals.var_qbd_dn10 = 0.0;
        locals.var_qbd_dn11 = 0.0;
        locals.var_qbd_dn12 = 0.0;
        locals.var_qbd_dn17 = 0.0;
        locals.var_qbd_rv = 0.0;

        locals.var_qbs = 0.0;
        locals.var_qbs_dn0 = 0.0;
        locals.var_qbs_dn2 = 0.0;
        locals.var_qbs_dn6 = 0.0;
        locals.var_qbs_dn7 = 0.0;
        locals.var_qbs_dn10 = 0.0;
        locals.var_qbs_dn11 = 0.0;
        locals.var_qbs_dn12 = 0.0;
        locals.var_qbs_dn17 = 0.0;
        locals.var_qbs_rv = 0.0;

        locals.var_qinm = 0.0;
        locals.var_qinm_dn0 = 0.0;
        locals.var_qinm_dn2 = 0.0;
        locals.var_qinm_dn6 = 0.0;
        locals.var_qinm_dn7 = 0.0;
        locals.var_qinm_dn10 = 0.0;
        locals.var_qinm_dn11 = 0.0;
        locals.var_qinm_dn12 = 0.0;
        locals.var_qinm_dn17 = 0.0;
        locals.var_qinm_rv = 0.0;

        locals.var_qidn = 0.0;
        locals.var_qidn_dn0 = 0.0;
        locals.var_qidn_dn2 = 0.0;
        locals.var_qidn_dn6 = 0.0;
        locals.var_qidn_dn7 = 0.0;
        locals.var_qidn_dn10 = 0.0;
        locals.var_qidn_dn11 = 0.0;
        locals.var_qidn_dn12 = 0.0;
        locals.var_qidn_dn17 = 0.0;
        locals.var_qidn_rv = 0.0;

        locals.var_wdsoi_0 = p.p237;
        locals.var_wdsoi_0_rv = 0.0;

        locals.var_qbody_bt_p_sus = 0.0;
        locals.var_qbody_bt_p_sus_dn0 = 0.0;
        locals.var_qbody_bt_p_sus_dn2 = 0.0;
        locals.var_qbody_bt_p_sus_dn6 = 0.0;
        locals.var_qbody_bt_p_sus_dn7 = 0.0;
        locals.var_qbody_bt_p_sus_dn10 = 0.0;
        locals.var_qbody_bt_p_sus_dn11 = 0.0;
        locals.var_qbody_bt_p_sus_dn12 = 0.0;
        locals.var_qbody_bt_p_sus_dn17 = 0.0;
        locals.var_qbody_bt_p_sus_rv = 0.0;

        locals.var_qbody_bt_p_sud = 0.0;
        locals.var_qbody_bt_p_sud_dn0 = 0.0;
        locals.var_qbody_bt_p_sud_dn2 = 0.0;
        locals.var_qbody_bt_p_sud_dn6 = 0.0;
        locals.var_qbody_bt_p_sud_dn7 = 0.0;
        locals.var_qbody_bt_p_sud_dn10 = 0.0;
        locals.var_qbody_bt_p_sud_dn11 = 0.0;
        locals.var_qbody_bt_p_sud_dn12 = 0.0;
        locals.var_qbody_bt_p_sud_dn17 = 0.0;
        locals.var_qbody_bt_p_sud_rv = 0.0;

        locals.var_qbody_bt_p_iud = 0.0;
        locals.var_qbody_bt_p_iud_dn0 = 0.0;
        locals.var_qbody_bt_p_iud_dn2 = 0.0;
        locals.var_qbody_bt_p_iud_dn6 = 0.0;
        locals.var_qbody_bt_p_iud_dn7 = 0.0;
        locals.var_qbody_bt_p_iud_dn10 = 0.0;
        locals.var_qbody_bt_p_iud_dn11 = 0.0;
        locals.var_qbody_bt_p_iud_dn12 = 0.0;
        locals.var_qbody_bt_p_iud_dn17 = 0.0;
        locals.var_qbody_bt_p_iud_rv = 0.0;

        locals.var_qbody_bt_p_ius = 0.0;
        locals.var_qbody_bt_p_ius_dn0 = 0.0;
        locals.var_qbody_bt_p_ius_dn2 = 0.0;
        locals.var_qbody_bt_p_ius_dn6 = 0.0;
        locals.var_qbody_bt_p_ius_dn7 = 0.0;
        locals.var_qbody_bt_p_ius_dn10 = 0.0;
        locals.var_qbody_bt_p_ius_dn11 = 0.0;
        locals.var_qbody_bt_p_ius_dn12 = 0.0;
        locals.var_qbody_bt_p_ius_dn17 = 0.0;
        locals.var_qbody_bt_p_ius_rv = 0.0;

        locals.var_qbody_bt_n_sus = 0.0;
        locals.var_qbody_bt_n_sus_dn0 = 0.0;
        locals.var_qbody_bt_n_sus_dn2 = 0.0;
        locals.var_qbody_bt_n_sus_dn6 = 0.0;
        locals.var_qbody_bt_n_sus_dn7 = 0.0;
        locals.var_qbody_bt_n_sus_dn10 = 0.0;
        locals.var_qbody_bt_n_sus_dn11 = 0.0;
        locals.var_qbody_bt_n_sus_dn12 = 0.0;
        locals.var_qbody_bt_n_sus_dn17 = 0.0;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        locals.var_qbody_bt_n_sud = 0.0;
        locals.var_qbody_bt_n_sud_dn0 = 0.0;
        locals.var_qbody_bt_n_sud_dn2 = 0.0;
        locals.var_qbody_bt_n_sud_dn6 = 0.0;
        locals.var_qbody_bt_n_sud_dn7 = 0.0;
        locals.var_qbody_bt_n_sud_dn10 = 0.0;
        locals.var_qbody_bt_n_sud_dn11 = 0.0;
        locals.var_qbody_bt_n_sud_dn12 = 0.0;
        locals.var_qbody_bt_n_sud_dn17 = 0.0;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        locals.var_qbody_bt_n_iud = 0.0;
        locals.var_qbody_bt_n_iud_dn0 = 0.0;
        locals.var_qbody_bt_n_iud_dn2 = 0.0;
        locals.var_qbody_bt_n_iud_dn6 = 0.0;
        locals.var_qbody_bt_n_iud_dn7 = 0.0;
        locals.var_qbody_bt_n_iud_dn10 = 0.0;
        locals.var_qbody_bt_n_iud_dn11 = 0.0;
        locals.var_qbody_bt_n_iud_dn12 = 0.0;
        locals.var_qbody_bt_n_iud_dn17 = 0.0;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        locals.var_qbody_bt_n_ius = 0.0;
        locals.var_qbody_bt_n_ius_dn0 = 0.0;
        locals.var_qbody_bt_n_ius_dn2 = 0.0;
        locals.var_qbody_bt_n_ius_dn6 = 0.0;
        locals.var_qbody_bt_n_ius_dn7 = 0.0;
        locals.var_qbody_bt_n_ius_dn10 = 0.0;
        locals.var_qbody_bt_n_ius_dn11 = 0.0;
        locals.var_qbody_bt_n_ius_dn12 = 0.0;
        locals.var_qbody_bt_n_ius_dn17 = 0.0;
        locals.var_qbody_bt_n_ius_rv = 0.0;

    }
}
