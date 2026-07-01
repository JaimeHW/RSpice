#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_41(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14370_e21229, assign14370_e21229_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14370_e21227: f64 = (1.0 - locals.var_wh);
        (assign14370_e21227, (-locals.var_wh_dn4),)
    } else {
        (locals.var_wl, locals.var_wl_dn4,)
    }
};
        locals.var_wl = assign14370_e21229;
        locals.var_wl_dn4 = assign14370_e21229_d_n4;
        locals.var_wl_rv = 0.0;

        let assign14380_e21232: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign14380_e21232;
        locals.var_guard241_rv = 0.0;

        let (assign14390_e21260, assign14390_e21260_d_n0, assign14390_e21260_d_n2, assign14390_e21260_d_n3, assign14390_e21260_d_n4, assign14390_e21260_d_n5, assign14390_e21260_d_n6, assign14390_e21260_d_n7, assign14390_e21260_d_n8, assign14390_e21260_d_n9, assign14390_e21260_d_n10, assign14390_e21260_d_n11, assign14390_e21260_d_n13, assign14390_e21260_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14390_e21242: f64 = (210.0 + locals.var_tlow);
        let assign14390_e21245: f64 = (210.0 - locals.var_tlow);
        let assign14390_e21248: f64 = (210.0 - locals.var_tlow);
        let assign14390_e21249: f64 = (assign14390_e21245 * assign14390_e21248);
        let assign14390_e21252: f64 = (0.25 * p.p1788);
        let assign14390_e21254: f64 = (assign14390_e21252 * p.p1788);
        let assign14390_e21255: f64 = (assign14390_e21249 + assign14390_e21254);
        let assign14390_e21256: f64 = (assign14390_e21255).sqrt();
        let assign14390_e21257: f64 = (assign14390_e21242 + assign14390_e21256);
        let assign14390_e21258: f64 = (0.5 * assign14390_e21257);
        (assign14390_e21258, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14390_e21260;
        locals.var_t1_dn0 = assign14390_e21260_d_n0;
        locals.var_t1_dn2 = assign14390_e21260_d_n2;
        locals.var_t1_dn3 = assign14390_e21260_d_n3;
        locals.var_t1_dn4 = assign14390_e21260_d_n4;
        locals.var_t1_dn5 = assign14390_e21260_d_n5;
        locals.var_t1_dn6 = assign14390_e21260_d_n6;
        locals.var_t1_dn7 = assign14390_e21260_d_n7;
        locals.var_t1_dn8 = assign14390_e21260_d_n8;
        locals.var_t1_dn9 = assign14390_e21260_d_n9;
        locals.var_t1_dn10 = assign14390_e21260_d_n10;
        locals.var_t1_dn11 = assign14390_e21260_d_n11;
        locals.var_t1_dn13 = assign14390_e21260_d_n13;
        locals.var_t1_dn14 = assign14390_e21260_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign14400_e21303, assign14400_e21303_d_n0, assign14400_e21303_d_n2, assign14400_e21303_d_n3, assign14400_e21303_d_n4, assign14400_e21303_d_n5, assign14400_e21303_d_n6, assign14400_e21303_d_n7, assign14400_e21303_d_n8, assign14400_e21303_d_n9, assign14400_e21303_d_n10, assign14400_e21303_d_n11, assign14400_e21303_d_n13, assign14400_e21303_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14400_e21269: f64 = (-p.p1790);
        let assign14400_e21272: f64 = (210.0 - p.p1787);
        let assign14400_e21273: f64 = (assign14400_e21269 * assign14400_e21272);
        let assign14400_e21275: f64 = assign14400_e21273;
        let assign14400_e21277: f64 = (-p.p1790);
        let assign14400_e21280: f64 = (210.0 - p.p1787);
        let assign14400_e21281: f64 = (assign14400_e21277 * assign14400_e21280);
        let assign14400_e21283: f64 = assign14400_e21281;
        let assign14400_e21285: f64 = (-p.p1790);
        let assign14400_e21288: f64 = (210.0 - p.p1787);
        let assign14400_e21289: f64 = (assign14400_e21285 * assign14400_e21288);
        let assign14400_e21291: f64 = assign14400_e21289;
        let assign14400_e21292: f64 = (assign14400_e21283 * assign14400_e21291);
        let assign14400_e21295: f64 = (0.25 * p.p1789);
        let assign14400_e21297: f64 = (assign14400_e21295 * p.p1789);
        let assign14400_e21298: f64 = (assign14400_e21292 + assign14400_e21297);
        let assign14400_e21299: f64 = (assign14400_e21298).sqrt();
        let assign14400_e21300: f64 = (assign14400_e21275 + assign14400_e21299);
        let assign14400_e21301: f64 = (0.5 * assign14400_e21300);
        (assign14400_e21301, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14400_e21303;
        locals.var_t2_dn0 = assign14400_e21303_d_n0;
        locals.var_t2_dn2 = assign14400_e21303_d_n2;
        locals.var_t2_dn3 = assign14400_e21303_d_n3;
        locals.var_t2_dn4 = assign14400_e21303_d_n4;
        locals.var_t2_dn5 = assign14400_e21303_d_n5;
        locals.var_t2_dn6 = assign14400_e21303_d_n6;
        locals.var_t2_dn7 = assign14400_e21303_d_n7;
        locals.var_t2_dn8 = assign14400_e21303_d_n8;
        locals.var_t2_dn9 = assign14400_e21303_d_n9;
        locals.var_t2_dn10 = assign14400_e21303_d_n10;
        locals.var_t2_dn11 = assign14400_e21303_d_n11;
        locals.var_t2_dn13 = assign14400_e21303_d_n13;
        locals.var_t2_dn14 = assign14400_e21303_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign14410_e21320, assign14410_e21320_d_n0, assign14410_e21320_d_n2, assign14410_e21320_d_n3, assign14410_e21320_d_n4, assign14410_e21320_d_n5, assign14410_e21320_d_n6, assign14410_e21320_d_n7, assign14410_e21320_d_n8, assign14410_e21320_d_n9, assign14410_e21320_d_n10, assign14410_e21320_d_n11, assign14410_e21320_d_n13, assign14410_e21320_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14410_e21312: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14410_e21314: f64 = (assign14410_e21312 - locals.var_t1);
        let assign14410_e21316: f64 = (assign14410_e21314 - locals.var_t2);
        let assign14410_e21318: f64 = (assign14410_e21316 + 210.0);
        (assign14410_e21318, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14410_e21320;
        locals.var_t3_dn0 = assign14410_e21320_d_n0;
        locals.var_t3_dn2 = assign14410_e21320_d_n2;
        locals.var_t3_dn3 = assign14410_e21320_d_n3;
        locals.var_t3_dn4 = assign14410_e21320_d_n4;
        locals.var_t3_dn5 = assign14410_e21320_d_n5;
        locals.var_t3_dn6 = assign14410_e21320_d_n6;
        locals.var_t3_dn7 = assign14410_e21320_d_n7;
        locals.var_t3_dn8 = assign14410_e21320_d_n8;
        locals.var_t3_dn9 = assign14410_e21320_d_n9;
        locals.var_t3_dn10 = assign14410_e21320_d_n10;
        locals.var_t3_dn11 = assign14410_e21320_d_n11;
        locals.var_t3_dn13 = assign14410_e21320_d_n13;
        locals.var_t3_dn14 = assign14410_e21320_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign14420_e21348, assign14420_e21348_d_n0, assign14420_e21348_d_n2, assign14420_e21348_d_n3, assign14420_e21348_d_n4, assign14420_e21348_d_n5, assign14420_e21348_d_n6, assign14420_e21348_d_n7, assign14420_e21348_d_n8, assign14420_e21348_d_n9, assign14420_e21348_d_n10, assign14420_e21348_d_n11, assign14420_e21348_d_n13, assign14420_e21348_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14420_e21330: f64 = (locals.var_devtemp + locals.var_t3);
        let assign14420_e21333: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14420_e21336: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14420_e21337: f64 = (assign14420_e21333 * assign14420_e21336);
        let assign14420_e21340: f64 = (0.25 * 0.2);
        let assign14420_e21342: f64 = (assign14420_e21340 * 0.2);
        let assign14420_e21343: f64 = (assign14420_e21337 + assign14420_e21342);
        let assign14420_e21344: f64 = (assign14420_e21343).sqrt();
        let assign14420_e21345: f64 = (assign14420_e21330 + assign14420_e21344);
        let assign14420_e21346: f64 = (0.5 * assign14420_e21345);
        (assign14420_e21346, (0.5 * (locals.var_t3_dn0 + ((((-locals.var_t3_dn0) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn0))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn2 + ((((-locals.var_t3_dn2) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn2))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn3 + ((((-locals.var_t3_dn3) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn3))) / (2.0 * assign14420_e21344)))), (0.5 * ((locals.var_devtemp_dn4 + locals.var_t3_dn4) + ((((locals.var_devtemp_dn4 - locals.var_t3_dn4) * assign14420_e21336) + (assign14420_e21333 * (locals.var_devtemp_dn4 - locals.var_t3_dn4))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn5 + ((((-locals.var_t3_dn5) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn5))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn6 + ((((-locals.var_t3_dn6) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn6))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn7 + ((((-locals.var_t3_dn7) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn7))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn8 + ((((-locals.var_t3_dn8) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn8))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn9 + ((((-locals.var_t3_dn9) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn9))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn10 + ((((-locals.var_t3_dn10) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn10))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn11 + ((((-locals.var_t3_dn11) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn11))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn13 + ((((-locals.var_t3_dn13) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn13))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn14 + ((((-locals.var_t3_dn14) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn14))) / (2.0 * assign14420_e21344)))),)
    } else {
        (locals.var_devtempeff, locals.var_devtempeff_dn0, locals.var_devtempeff_dn2, locals.var_devtempeff_dn3, locals.var_devtempeff_dn4, locals.var_devtempeff_dn5, locals.var_devtempeff_dn6, locals.var_devtempeff_dn7, locals.var_devtempeff_dn8, locals.var_devtempeff_dn9, locals.var_devtempeff_dn10, locals.var_devtempeff_dn11, locals.var_devtempeff_dn13, locals.var_devtempeff_dn14,)
    }
};
        locals.var_devtempeff = assign14420_e21348;
        locals.var_devtempeff_dn0 = assign14420_e21348_d_n0;
        locals.var_devtempeff_dn2 = assign14420_e21348_d_n2;
        locals.var_devtempeff_dn3 = assign14420_e21348_d_n3;
        locals.var_devtempeff_dn4 = assign14420_e21348_d_n4;
        locals.var_devtempeff_dn5 = assign14420_e21348_d_n5;
        locals.var_devtempeff_dn6 = assign14420_e21348_d_n6;
        locals.var_devtempeff_dn7 = assign14420_e21348_d_n7;
        locals.var_devtempeff_dn8 = assign14420_e21348_d_n8;
        locals.var_devtempeff_dn9 = assign14420_e21348_d_n9;
        locals.var_devtempeff_dn10 = assign14420_e21348_d_n10;
        locals.var_devtempeff_dn11 = assign14420_e21348_d_n11;
        locals.var_devtempeff_dn13 = assign14420_e21348_d_n13;
        locals.var_devtempeff_dn14 = assign14420_e21348_d_n14;
        locals.var_devtempeff_rv = 0.0;

        let (assign14430_e21377, assign14430_e21377_d_n0, assign14430_e21377_d_n2, assign14430_e21377_d_n3, assign14430_e21377_d_n4, assign14430_e21377_d_n5, assign14430_e21377_d_n6, assign14430_e21377_d_n7, assign14430_e21377_d_n8, assign14430_e21377_d_n9, assign14430_e21377_d_n10, assign14430_e21377_d_n11, assign14430_e21377_d_n13, assign14430_e21377_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14430_e21359: f64 = (locals.var_tnom + locals.var_tlow);
        let assign14430_e21362: f64 = (locals.var_tnom - locals.var_tlow);
        let assign14430_e21365: f64 = (locals.var_tnom - locals.var_tlow);
        let assign14430_e21366: f64 = (assign14430_e21362 * assign14430_e21365);
        let assign14430_e21369: f64 = (0.25 * p.p1788);
        let assign14430_e21371: f64 = (assign14430_e21369 * p.p1788);
        let assign14430_e21372: f64 = (assign14430_e21366 + assign14430_e21371);
        let assign14430_e21373: f64 = (assign14430_e21372).sqrt();
        let assign14430_e21374: f64 = (assign14430_e21359 + assign14430_e21373);
        let assign14430_e21375: f64 = (0.5 * assign14430_e21374);
        (assign14430_e21375, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14430_e21377;
        locals.var_t1_dn0 = assign14430_e21377_d_n0;
        locals.var_t1_dn2 = assign14430_e21377_d_n2;
        locals.var_t1_dn3 = assign14430_e21377_d_n3;
        locals.var_t1_dn4 = assign14430_e21377_d_n4;
        locals.var_t1_dn5 = assign14430_e21377_d_n5;
        locals.var_t1_dn6 = assign14430_e21377_d_n6;
        locals.var_t1_dn7 = assign14430_e21377_d_n7;
        locals.var_t1_dn8 = assign14430_e21377_d_n8;
        locals.var_t1_dn9 = assign14430_e21377_d_n9;
        locals.var_t1_dn10 = assign14430_e21377_d_n10;
        locals.var_t1_dn11 = assign14430_e21377_d_n11;
        locals.var_t1_dn13 = assign14430_e21377_d_n13;
        locals.var_t1_dn14 = assign14430_e21377_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign14440_e21421, assign14440_e21421_d_n0, assign14440_e21421_d_n2, assign14440_e21421_d_n3, assign14440_e21421_d_n4, assign14440_e21421_d_n5, assign14440_e21421_d_n6, assign14440_e21421_d_n7, assign14440_e21421_d_n8, assign14440_e21421_d_n9, assign14440_e21421_d_n10, assign14440_e21421_d_n11, assign14440_e21421_d_n13, assign14440_e21421_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14440_e21387: f64 = (-p.p1790);
        let assign14440_e21390: f64 = (locals.var_tnom - p.p1787);
        let assign14440_e21391: f64 = (assign14440_e21387 * assign14440_e21390);
        let assign14440_e21393: f64 = assign14440_e21391;
        let assign14440_e21395: f64 = (-p.p1790);
        let assign14440_e21398: f64 = (locals.var_tnom - p.p1787);
        let assign14440_e21399: f64 = (assign14440_e21395 * assign14440_e21398);
        let assign14440_e21401: f64 = assign14440_e21399;
        let assign14440_e21403: f64 = (-p.p1790);
        let assign14440_e21406: f64 = (locals.var_tnom - p.p1787);
        let assign14440_e21407: f64 = (assign14440_e21403 * assign14440_e21406);
        let assign14440_e21409: f64 = assign14440_e21407;
        let assign14440_e21410: f64 = (assign14440_e21401 * assign14440_e21409);
        let assign14440_e21413: f64 = (0.25 * p.p1789);
        let assign14440_e21415: f64 = (assign14440_e21413 * p.p1789);
        let assign14440_e21416: f64 = (assign14440_e21410 + assign14440_e21415);
        let assign14440_e21417: f64 = (assign14440_e21416).sqrt();
        let assign14440_e21418: f64 = (assign14440_e21393 + assign14440_e21417);
        let assign14440_e21419: f64 = (0.5 * assign14440_e21418);
        (assign14440_e21419, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14440_e21421;
        locals.var_t2_dn0 = assign14440_e21421_d_n0;
        locals.var_t2_dn2 = assign14440_e21421_d_n2;
        locals.var_t2_dn3 = assign14440_e21421_d_n3;
        locals.var_t2_dn4 = assign14440_e21421_d_n4;
        locals.var_t2_dn5 = assign14440_e21421_d_n5;
        locals.var_t2_dn6 = assign14440_e21421_d_n6;
        locals.var_t2_dn7 = assign14440_e21421_d_n7;
        locals.var_t2_dn8 = assign14440_e21421_d_n8;
        locals.var_t2_dn9 = assign14440_e21421_d_n9;
        locals.var_t2_dn10 = assign14440_e21421_d_n10;
        locals.var_t2_dn11 = assign14440_e21421_d_n11;
        locals.var_t2_dn13 = assign14440_e21421_d_n13;
        locals.var_t2_dn14 = assign14440_e21421_d_n14;
        locals.var_t2_rv = 0.0;

        let assign14450_e21424: f64 = if locals.var_tnom > locals.var_tlow { 1.0 } else { 0.0 };
        locals.var_guard242 = assign14450_e21424;
        locals.var_guard242_rv = 0.0;

        let (assign14460_e21444, assign14460_e21444_d_n0, assign14460_e21444_d_n2, assign14460_e21444_d_n3, assign14460_e21444_d_n4, assign14460_e21444_d_n5, assign14460_e21444_d_n6, assign14460_e21444_d_n7, assign14460_e21444_d_n8, assign14460_e21444_d_n9, assign14460_e21444_d_n10, assign14460_e21444_d_n11, assign14460_e21444_d_n13, assign14460_e21444_d_n14,) = {
    if ((((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 != 0.0)) {
        let assign14460_e21436: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14460_e21438: f64 = (assign14460_e21436 - locals.var_t1);
        let assign14460_e21440: f64 = (assign14460_e21438 - locals.var_t2);
        let assign14460_e21442: f64 = (assign14460_e21440 + locals.var_tnom);
        (assign14460_e21442, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14460_e21444;
        locals.var_t3_dn0 = assign14460_e21444_d_n0;
        locals.var_t3_dn2 = assign14460_e21444_d_n2;
        locals.var_t3_dn3 = assign14460_e21444_d_n3;
        locals.var_t3_dn4 = assign14460_e21444_d_n4;
        locals.var_t3_dn5 = assign14460_e21444_d_n5;
        locals.var_t3_dn6 = assign14460_e21444_d_n6;
        locals.var_t3_dn7 = assign14460_e21444_d_n7;
        locals.var_t3_dn8 = assign14460_e21444_d_n8;
        locals.var_t3_dn9 = assign14460_e21444_d_n9;
        locals.var_t3_dn10 = assign14460_e21444_d_n10;
        locals.var_t3_dn11 = assign14460_e21444_d_n11;
        locals.var_t3_dn13 = assign14460_e21444_d_n13;
        locals.var_t3_dn14 = assign14460_e21444_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign14470_e21465, assign14470_e21465_d_n0, assign14470_e21465_d_n2, assign14470_e21465_d_n3, assign14470_e21465_d_n4, assign14470_e21465_d_n5, assign14470_e21465_d_n6, assign14470_e21465_d_n7, assign14470_e21465_d_n8, assign14470_e21465_d_n9, assign14470_e21465_d_n10, assign14470_e21465_d_n11, assign14470_e21465_d_n13, assign14470_e21465_d_n14,) = {
    if ((((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 == 0.0)) {
        let assign14470_e21457: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14470_e21459: f64 = (assign14470_e21457 - locals.var_t1);
        let assign14470_e21461: f64 = (assign14470_e21459 - locals.var_t2);
        let assign14470_e21463: f64 = (assign14470_e21461 + locals.var_tlow);
        (assign14470_e21463, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14470_e21465;
        locals.var_t3_dn0 = assign14470_e21465_d_n0;
        locals.var_t3_dn2 = assign14470_e21465_d_n2;
        locals.var_t3_dn3 = assign14470_e21465_d_n3;
        locals.var_t3_dn4 = assign14470_e21465_d_n4;
        locals.var_t3_dn5 = assign14470_e21465_d_n5;
        locals.var_t3_dn6 = assign14470_e21465_d_n6;
        locals.var_t3_dn7 = assign14470_e21465_d_n7;
        locals.var_t3_dn8 = assign14470_e21465_d_n8;
        locals.var_t3_dn9 = assign14470_e21465_d_n9;
        locals.var_t3_dn10 = assign14470_e21465_d_n10;
        locals.var_t3_dn11 = assign14470_e21465_d_n11;
        locals.var_t3_dn13 = assign14470_e21465_d_n13;
        locals.var_t3_dn14 = assign14470_e21465_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign14480_e21494, assign14480_e21494_d_n0, assign14480_e21494_d_n2, assign14480_e21494_d_n3, assign14480_e21494_d_n4, assign14480_e21494_d_n5, assign14480_e21494_d_n6, assign14480_e21494_d_n7, assign14480_e21494_d_n8, assign14480_e21494_d_n9, assign14480_e21494_d_n10, assign14480_e21494_d_n11, assign14480_e21494_d_n13, assign14480_e21494_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14480_e21476: f64 = (locals.var_devtemp + locals.var_t3);
        let assign14480_e21479: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14480_e21482: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14480_e21483: f64 = (assign14480_e21479 * assign14480_e21482);
        let assign14480_e21486: f64 = (0.25 * 0.2);
        let assign14480_e21488: f64 = (assign14480_e21486 * 0.2);
        let assign14480_e21489: f64 = (assign14480_e21483 + assign14480_e21488);
        let assign14480_e21490: f64 = (assign14480_e21489).sqrt();
        let assign14480_e21491: f64 = (assign14480_e21476 + assign14480_e21490);
        let assign14480_e21492: f64 = (0.5 * assign14480_e21491);
        (assign14480_e21492, (0.5 * (locals.var_t3_dn0 + ((((-locals.var_t3_dn0) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn0))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn2 + ((((-locals.var_t3_dn2) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn2))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn3 + ((((-locals.var_t3_dn3) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn3))) / (2.0 * assign14480_e21490)))), (0.5 * ((locals.var_devtemp_dn4 + locals.var_t3_dn4) + ((((locals.var_devtemp_dn4 - locals.var_t3_dn4) * assign14480_e21482) + (assign14480_e21479 * (locals.var_devtemp_dn4 - locals.var_t3_dn4))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn5 + ((((-locals.var_t3_dn5) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn5))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn6 + ((((-locals.var_t3_dn6) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn6))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn7 + ((((-locals.var_t3_dn7) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn7))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn8 + ((((-locals.var_t3_dn8) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn8))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn9 + ((((-locals.var_t3_dn9) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn9))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn10 + ((((-locals.var_t3_dn10) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn10))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn11 + ((((-locals.var_t3_dn11) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn11))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn13 + ((((-locals.var_t3_dn13) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn13))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn14 + ((((-locals.var_t3_dn14) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn14))) / (2.0 * assign14480_e21490)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign14480_e21494;
        locals.var_t4_dn0 = assign14480_e21494_d_n0;
        locals.var_t4_dn2 = assign14480_e21494_d_n2;
        locals.var_t4_dn3 = assign14480_e21494_d_n3;
        locals.var_t4_dn4 = assign14480_e21494_d_n4;
        locals.var_t4_dn5 = assign14480_e21494_d_n5;
        locals.var_t4_dn6 = assign14480_e21494_d_n6;
        locals.var_t4_dn7 = assign14480_e21494_d_n7;
        locals.var_t4_dn8 = assign14480_e21494_d_n8;
        locals.var_t4_dn9 = assign14480_e21494_d_n9;
        locals.var_t4_dn10 = assign14480_e21494_d_n10;
        locals.var_t4_dn11 = assign14480_e21494_d_n11;
        locals.var_t4_dn13 = assign14480_e21494_d_n13;
        locals.var_t4_dn14 = assign14480_e21494_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign14490_e21510, assign14490_e21510_d_n0, assign14490_e21510_d_n2, assign14490_e21510_d_n3, assign14490_e21510_d_n4, assign14490_e21510_d_n5, assign14490_e21510_d_n6, assign14490_e21510_d_n7, assign14490_e21510_d_n8, assign14490_e21510_d_n9, assign14490_e21510_d_n10, assign14490_e21510_d_n11, assign14490_e21510_d_n13, assign14490_e21510_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14490_e21504: f64 = (locals.var_wl * locals.var_t4);
        let assign14490_e21507: f64 = (locals.var_wh * locals.var_devtemp);
        let assign14490_e21508: f64 = (assign14490_e21504 + assign14490_e21507);
        (assign14490_e21508, (locals.var_wl * locals.var_t4_dn0), (locals.var_wl * locals.var_t4_dn2), (locals.var_wl * locals.var_t4_dn3), (((locals.var_wl_dn4 * locals.var_t4) + (locals.var_wl * locals.var_t4_dn4)) + ((locals.var_wh_dn4 * locals.var_devtemp) + (locals.var_wh * locals.var_devtemp_dn4))), (locals.var_wl * locals.var_t4_dn5), (locals.var_wl * locals.var_t4_dn6), (locals.var_wl * locals.var_t4_dn7), (locals.var_wl * locals.var_t4_dn8), (locals.var_wl * locals.var_t4_dn9), (locals.var_wl * locals.var_t4_dn10), (locals.var_wl * locals.var_t4_dn11), (locals.var_wl * locals.var_t4_dn13), (locals.var_wl * locals.var_t4_dn14),)
    } else {
        (locals.var_devtempeff, locals.var_devtempeff_dn0, locals.var_devtempeff_dn2, locals.var_devtempeff_dn3, locals.var_devtempeff_dn4, locals.var_devtempeff_dn5, locals.var_devtempeff_dn6, locals.var_devtempeff_dn7, locals.var_devtempeff_dn8, locals.var_devtempeff_dn9, locals.var_devtempeff_dn10, locals.var_devtempeff_dn11, locals.var_devtempeff_dn13, locals.var_devtempeff_dn14,)
    }
};
        locals.var_devtempeff = assign14490_e21510;
        locals.var_devtempeff_dn0 = assign14490_e21510_d_n0;
        locals.var_devtempeff_dn2 = assign14490_e21510_d_n2;
        locals.var_devtempeff_dn3 = assign14490_e21510_d_n3;
        locals.var_devtempeff_dn4 = assign14490_e21510_d_n4;
        locals.var_devtempeff_dn5 = assign14490_e21510_d_n5;
        locals.var_devtempeff_dn6 = assign14490_e21510_d_n6;
        locals.var_devtempeff_dn7 = assign14490_e21510_d_n7;
        locals.var_devtempeff_dn8 = assign14490_e21510_d_n8;
        locals.var_devtempeff_dn9 = assign14490_e21510_d_n9;
        locals.var_devtempeff_dn10 = assign14490_e21510_d_n10;
        locals.var_devtempeff_dn11 = assign14490_e21510_d_n11;
        locals.var_devtempeff_dn13 = assign14490_e21510_d_n13;
        locals.var_devtempeff_dn14 = assign14490_e21510_d_n14;
        locals.var_devtempeff_rv = 0.0;

        let (assign14500_e21536, assign14500_e21536_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14500_e21518: f64 = (locals.var_devtemp + 210.0);
        let assign14500_e21521: f64 = (locals.var_devtemp - 210.0);
        let assign14500_e21524: f64 = (locals.var_devtemp - 210.0);
        let assign14500_e21525: f64 = (assign14500_e21521 * assign14500_e21524);
        let assign14500_e21528: f64 = (0.25 * 0.2);
        let assign14500_e21530: f64 = (assign14500_e21528 * 0.2);
        let assign14500_e21531: f64 = (assign14500_e21525 + assign14500_e21530);
        let assign14500_e21532: f64 = (assign14500_e21531).sqrt();
        let assign14500_e21533: f64 = (assign14500_e21518 - assign14500_e21532);
        let assign14500_e21534: f64 = (0.5 * assign14500_e21533);
        (assign14500_e21534, (0.5 * (locals.var_devtemp_dn4 - (((locals.var_devtemp_dn4 * assign14500_e21524) + (assign14500_e21521 * locals.var_devtemp_dn4)) / (2.0 * assign14500_e21532)))),)
    } else {
        (locals.var_devtemp1, locals.var_devtemp1_dn4,)
    }
};
        locals.var_devtemp1 = assign14500_e21536;
        locals.var_devtemp1_dn4 = assign14500_e21536_d_n4;
        locals.var_devtemp1_rv = 0.0;

        let (assign14510_e21564, assign14510_e21564_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14510_e21545: f64 = (locals.var_tnom + 210.0);
        let assign14510_e21548: f64 = (locals.var_tnom - 210.0);
        let assign14510_e21551: f64 = (locals.var_tnom - 210.0);
        let assign14510_e21552: f64 = (assign14510_e21548 * assign14510_e21551);
        let assign14510_e21555: f64 = (0.25 * 0.2);
        let assign14510_e21557: f64 = (assign14510_e21555 * 0.2);
        let assign14510_e21558: f64 = (assign14510_e21552 + assign14510_e21557);
        let assign14510_e21559: f64 = (assign14510_e21558).sqrt();
        let assign14510_e21560: f64 = (assign14510_e21545 - assign14510_e21559);
        let assign14510_e21561: f64 = (0.5 * assign14510_e21560);
        let assign14510_e21562: f64 = (locals.var_devtemp1 - assign14510_e21561);
        (assign14510_e21562, locals.var_devtemp1_dn4,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4,)
    }
};
        locals.var_deltemp1 = assign14510_e21564;
        locals.var_deltemp1_dn4 = assign14510_e21564_d_n4;
        locals.var_deltemp1_rv = 0.0;

        let (assign14520_e21575, assign14520_e21575_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14520_e21571: f64 = (locals.var_devtemp1 - 210.0);
        let assign14520_e21573: f64 = (assign14520_e21571 / locals.var_tnom);
        (assign14520_e21573, (locals.var_devtemp1_dn4 / locals.var_tnom),)
    } else {
        (locals.var_deltratio1, locals.var_deltratio1_dn4,)
    }
};
        locals.var_deltratio1 = assign14520_e21575;
        locals.var_deltratio1_dn4 = assign14520_e21575_d_n4;
        locals.var_deltratio1_rv = 0.0;

        let (assign14530_e21581, assign14530_e21581_d_n0, assign14530_e21581_d_n2, assign14530_e21581_d_n3, assign14530_e21581_d_n4, assign14530_e21581_d_n5, assign14530_e21581_d_n6, assign14530_e21581_d_n7, assign14530_e21581_d_n8, assign14530_e21581_d_n9, assign14530_e21581_d_n10, assign14530_e21581_d_n11, assign14530_e21581_d_n13, assign14530_e21581_d_n14,) = {
    if (locals.var_guard237 != 0.0) {
        let assign14530_e21579: f64 = (8.617087e-5 * locals.var_devtempeff);
        (assign14530_e21579, (8.617087e-5 * locals.var_devtempeff_dn0), (8.617087e-5 * locals.var_devtempeff_dn2), (8.617087e-5 * locals.var_devtempeff_dn3), (8.617087e-5 * locals.var_devtempeff_dn4), (8.617087e-5 * locals.var_devtempeff_dn5), (8.617087e-5 * locals.var_devtempeff_dn6), (8.617087e-5 * locals.var_devtempeff_dn7), (8.617087e-5 * locals.var_devtempeff_dn8), (8.617087e-5 * locals.var_devtempeff_dn9), (8.617087e-5 * locals.var_devtempeff_dn10), (8.617087e-5 * locals.var_devtempeff_dn11), (8.617087e-5 * locals.var_devtempeff_dn13), (8.617087e-5 * locals.var_devtempeff_dn14),)
    } else {
        (locals.var_vtmeff, locals.var_vtmeff_dn0, locals.var_vtmeff_dn2, locals.var_vtmeff_dn3, locals.var_vtmeff_dn4, locals.var_vtmeff_dn5, locals.var_vtmeff_dn6, locals.var_vtmeff_dn7, locals.var_vtmeff_dn8, locals.var_vtmeff_dn9, locals.var_vtmeff_dn10, locals.var_vtmeff_dn11, locals.var_vtmeff_dn13, locals.var_vtmeff_dn14,)
    }
};
        locals.var_vtmeff = assign14530_e21581;
        locals.var_vtmeff_dn0 = assign14530_e21581_d_n0;
        locals.var_vtmeff_dn2 = assign14530_e21581_d_n2;
        locals.var_vtmeff_dn3 = assign14530_e21581_d_n3;
        locals.var_vtmeff_dn4 = assign14530_e21581_d_n4;
        locals.var_vtmeff_dn5 = assign14530_e21581_d_n5;
        locals.var_vtmeff_dn6 = assign14530_e21581_d_n6;
        locals.var_vtmeff_dn7 = assign14530_e21581_d_n7;
        locals.var_vtmeff_dn8 = assign14530_e21581_d_n8;
        locals.var_vtmeff_dn9 = assign14530_e21581_d_n9;
        locals.var_vtmeff_dn10 = assign14530_e21581_d_n10;
        locals.var_vtmeff_dn11 = assign14530_e21581_d_n11;
        locals.var_vtmeff_dn13 = assign14530_e21581_d_n13;
        locals.var_vtmeff_dn14 = assign14530_e21581_d_n14;
        locals.var_vtmeff_rv = 0.0;

        let assign14540_e21585: f64 = (p.p1718 * locals.var_devtemp);
        let assign14540_e21587: f64 = (assign14540_e21585 * locals.var_devtemp);
        let assign14540_e21590: f64 = (locals.var_devtemp + p.p1719);
        let assign14540_e21591: f64 = (assign14540_e21587 / assign14540_e21590);
        let assign14540_e21592: f64 = (p.p106 - assign14540_e21591);
        locals.var_eg = assign14540_e21592;
        locals.var_eg_dn4 = (-((((((p.p1718 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign14540_e21585 * locals.var_devtemp_dn4)) * assign14540_e21590) - (assign14540_e21587 * locals.var_devtemp_dn4)) / (assign14540_e21590 * assign14540_e21590)));
        locals.var_eg_rv = 0.0;

        let assign14550_e21596: f64 = (p.p1718 * locals.var_tnom);
        let assign14550_e21598: f64 = (assign14550_e21596 * locals.var_tnom);
        let assign14550_e21601: f64 = (locals.var_tnom + p.p1719);
        let assign14550_e21602: f64 = (assign14550_e21598 / assign14550_e21601);
        let assign14550_e21603: f64 = (p.p106 - assign14550_e21602);
        locals.var_eg0 = assign14550_e21603;
        locals.var_eg0_rv = 0.0;

        let __rspice_inv_cse_0: f64 = 1.0 / 300.15;
        let assign14560_e21606: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign14560_e21609: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign14560_e21610: f64 = (assign14560_e21609).sqrt();
        let assign14560_e21611: f64 = (assign14560_e21606 * assign14560_e21610);
        locals.var_t1 = assign14560_e21611;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (((locals.var_devtemp_dn4 / 300.15) * assign14560_e21610) + (assign14560_e21606 * ((locals.var_devtemp_dn4 / 300.15) / (2.0 * assign14560_e21610))));
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign14570_e21614: f64 = (p.p105 * locals.var_t1);
        let assign14570_e21618: f64 = (2.0 * 8.617087e-5);
        let assign14570_e21620: f64 = (assign14570_e21618 * 300.15);
        let assign14570_e21621: f64 = (p.p106 / assign14570_e21620);
        let assign14570_e21625: f64 = (2.0 * locals.var_vtm);
        let assign14570_e21626: f64 = (locals.var_eg / assign14570_e21625);
        let assign14570_e21627: f64 = (assign14570_e21621 - assign14570_e21626);
        let assign14570_e21628: f64 = { let limited_exp_arg = assign14570_e21627; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14570_e21629: f64 = (assign14570_e21614 * assign14570_e21628);
        locals.var_ni = assign14570_e21629;
        locals.var_ni_dn0 = ((p.p105 * locals.var_t1_dn0) * assign14570_e21628);
        locals.var_ni_dn2 = ((p.p105 * locals.var_t1_dn2) * assign14570_e21628);
        locals.var_ni_dn3 = ((p.p105 * locals.var_t1_dn3) * assign14570_e21628);
        locals.var_ni_dn4 = (((p.p105 * locals.var_t1_dn4) * assign14570_e21628) + (assign14570_e21614 * ({ let limited_exp_arg = assign14570_e21627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-(((locals.var_eg_dn4 * assign14570_e21625) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign14570_e21625 * assign14570_e21625))))));
        locals.var_ni_dn5 = ((p.p105 * locals.var_t1_dn5) * assign14570_e21628);
        locals.var_ni_dn6 = ((p.p105 * locals.var_t1_dn6) * assign14570_e21628);
        locals.var_ni_dn7 = ((p.p105 * locals.var_t1_dn7) * assign14570_e21628);
        locals.var_ni_dn8 = ((p.p105 * locals.var_t1_dn8) * assign14570_e21628);
        locals.var_ni_dn9 = ((p.p105 * locals.var_t1_dn9) * assign14570_e21628);
        locals.var_ni_dn10 = ((p.p105 * locals.var_t1_dn10) * assign14570_e21628);
        locals.var_ni_dn11 = ((p.p105 * locals.var_t1_dn11) * assign14570_e21628);
        locals.var_ni_dn13 = ((p.p105 * locals.var_t1_dn13) * assign14570_e21628);
        locals.var_ni_dn14 = ((p.p105 * locals.var_t1_dn14) * assign14570_e21628);
        locals.var_ni_rv = 0.0;

        let assign14580_e21632: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign14580_e21632;
        locals.var_guard243_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_42(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14590_e21638, assign14590_e21638_d_n0, assign14590_e21638_d_n2, assign14590_e21638_d_n3, assign14590_e21638_d_n4, assign14590_e21638_d_n5, assign14590_e21638_d_n6, assign14590_e21638_d_n7, assign14590_e21638_d_n8, assign14590_e21638_d_n9, assign14590_e21638_d_n10, assign14590_e21638_d_n11, assign14590_e21638_d_n13, assign14590_e21638_d_n14,) = {
    if (locals.var_guard243 != 0.0) {
        let assign14590_e21636: f64 = (p.p107 * locals.var_t1);
        (assign14590_e21636, (p.p107 * locals.var_t1_dn0), (p.p107 * locals.var_t1_dn2), (p.p107 * locals.var_t1_dn3), (p.p107 * locals.var_t1_dn4), (p.p107 * locals.var_t1_dn5), (p.p107 * locals.var_t1_dn6), (p.p107 * locals.var_t1_dn7), (p.p107 * locals.var_t1_dn8), (p.p107 * locals.var_t1_dn9), (p.p107 * locals.var_t1_dn10), (p.p107 * locals.var_t1_dn11), (p.p107 * locals.var_t1_dn13), (p.p107 * locals.var_t1_dn14),)
    } else {
        (locals.var_nc, locals.var_nc_dn0, locals.var_nc_dn2, locals.var_nc_dn3, locals.var_nc_dn4, locals.var_nc_dn5, locals.var_nc_dn6, locals.var_nc_dn7, locals.var_nc_dn8, locals.var_nc_dn9, locals.var_nc_dn10, locals.var_nc_dn11, locals.var_nc_dn13, locals.var_nc_dn14,)
    }
};
        locals.var_nc = assign14590_e21638;
        locals.var_nc_dn0 = assign14590_e21638_d_n0;
        locals.var_nc_dn2 = assign14590_e21638_d_n2;
        locals.var_nc_dn3 = assign14590_e21638_d_n3;
        locals.var_nc_dn4 = assign14590_e21638_d_n4;
        locals.var_nc_dn5 = assign14590_e21638_d_n5;
        locals.var_nc_dn6 = assign14590_e21638_d_n6;
        locals.var_nc_dn7 = assign14590_e21638_d_n7;
        locals.var_nc_dn8 = assign14590_e21638_d_n8;
        locals.var_nc_dn9 = assign14590_e21638_d_n9;
        locals.var_nc_dn10 = assign14590_e21638_d_n10;
        locals.var_nc_dn11 = assign14590_e21638_d_n11;
        locals.var_nc_dn13 = assign14590_e21638_d_n13;
        locals.var_nc_dn14 = assign14590_e21638_d_n14;
        locals.var_nc_rv = 0.0;

        let (assign14600_e21652, assign14600_e21652_d_n0, assign14600_e21652_d_n2, assign14600_e21652_d_n3, assign14600_e21652_d_n4, assign14600_e21652_d_n5, assign14600_e21652_d_n6, assign14600_e21652_d_n7, assign14600_e21652_d_n8, assign14600_e21652_d_n9, assign14600_e21652_d_n10, assign14600_e21652_d_n11, assign14600_e21652_d_n13, assign14600_e21652_d_n14,) = {
    if (locals.var_guard243 == 0.0) {
        let assign14600_e21644: f64 = (locals.var_devtempeff / 300.15);
        let assign14600_e21645: f64 = (p.p107 * assign14600_e21644);
        let assign14600_e21648: f64 = (locals.var_devtempeff / 300.15);
        let assign14600_e21649: f64 = (assign14600_e21648).sqrt();
        let assign14600_e21650: f64 = (assign14600_e21645 * assign14600_e21649);
        (assign14600_e21650, (((p.p107 * (locals.var_devtempeff_dn0 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn0 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn2 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn2 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn3 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn3 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn4 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn4 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn5 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn5 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn6 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn6 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn7 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn7 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn8 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn8 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn9 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn9 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn10 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn10 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn11 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn11 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn13 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn13 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn14 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn14 / 300.15) / (2.0 * assign14600_e21649)))),)
    } else {
        (locals.var_nc, locals.var_nc_dn0, locals.var_nc_dn2, locals.var_nc_dn3, locals.var_nc_dn4, locals.var_nc_dn5, locals.var_nc_dn6, locals.var_nc_dn7, locals.var_nc_dn8, locals.var_nc_dn9, locals.var_nc_dn10, locals.var_nc_dn11, locals.var_nc_dn13, locals.var_nc_dn14,)
    }
};
        locals.var_nc = assign14600_e21652;
        locals.var_nc_dn0 = assign14600_e21652_d_n0;
        locals.var_nc_dn2 = assign14600_e21652_d_n2;
        locals.var_nc_dn3 = assign14600_e21652_d_n3;
        locals.var_nc_dn4 = assign14600_e21652_d_n4;
        locals.var_nc_dn5 = assign14600_e21652_d_n5;
        locals.var_nc_dn6 = assign14600_e21652_d_n6;
        locals.var_nc_dn7 = assign14600_e21652_d_n7;
        locals.var_nc_dn8 = assign14600_e21652_d_n8;
        locals.var_nc_dn9 = assign14600_e21652_d_n9;
        locals.var_nc_dn10 = assign14600_e21652_d_n10;
        locals.var_nc_dn11 = assign14600_e21652_d_n11;
        locals.var_nc_dn13 = assign14600_e21652_d_n13;
        locals.var_nc_dn14 = assign14600_e21652_d_n14;
        locals.var_nc_rv = 0.0;

        let (assign14610_e21690, assign14610_e21690_d_n0, assign14610_e21690_d_n2, assign14610_e21690_d_n3, assign14610_e21690_d_n4, assign14610_e21690_d_n5, assign14610_e21690_d_n6, assign14610_e21690_d_n7, assign14610_e21690_d_n8, assign14610_e21690_d_n9, assign14610_e21690_d_n10, assign14610_e21690_d_n11, assign14610_e21690_d_n13, assign14610_e21690_d_n14,) = {
    if (locals.var_guard243 == 0.0) {
        let assign14610_e21657: f64 = (p.p105 * locals.var_t1);
        let (assign14610_e21674, assign14610_e21674_d_n0, assign14610_e21674_d_n2, assign14610_e21674_d_n3, assign14610_e21674_d_n4, assign14610_e21674_d_n5, assign14610_e21674_d_n6, assign14610_e21674_d_n7, assign14610_e21674_d_n8, assign14610_e21674_d_n9, assign14610_e21674_d_n10, assign14610_e21674_d_n11, assign14610_e21674_d_n13, assign14610_e21674_d_n14,) = {
            if (!(assign14610_e21657 > 1e-38)) {
                let assign14610_e21662: f64 = (-87.498233534);
                (assign14610_e21662, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign14610_e21665: f64 = (p.p105 * locals.var_t1);
                let (assign14610_e21673, assign14610_e21673_d_n0, assign14610_e21673_d_n2, assign14610_e21673_d_n3, assign14610_e21673_d_n4, assign14610_e21673_d_n5, assign14610_e21673_d_n6, assign14610_e21673_d_n7, assign14610_e21673_d_n8, assign14610_e21673_d_n9, assign14610_e21673_d_n10, assign14610_e21673_d_n11, assign14610_e21673_d_n13, assign14610_e21673_d_n14,) = {
                    if (assign14610_e21665 > 1e-38) {
                        let assign14610_e21670: f64 = (p.p105 * locals.var_t1);
                        let assign14610_e21671: f64 = (assign14610_e21670).ln();
                        (assign14610_e21671, ((p.p105 * locals.var_t1_dn0) / assign14610_e21670), ((p.p105 * locals.var_t1_dn2) / assign14610_e21670), ((p.p105 * locals.var_t1_dn3) / assign14610_e21670), ((p.p105 * locals.var_t1_dn4) / assign14610_e21670), ((p.p105 * locals.var_t1_dn5) / assign14610_e21670), ((p.p105 * locals.var_t1_dn6) / assign14610_e21670), ((p.p105 * locals.var_t1_dn7) / assign14610_e21670), ((p.p105 * locals.var_t1_dn8) / assign14610_e21670), ((p.p105 * locals.var_t1_dn9) / assign14610_e21670), ((p.p105 * locals.var_t1_dn10) / assign14610_e21670), ((p.p105 * locals.var_t1_dn11) / assign14610_e21670), ((p.p105 * locals.var_t1_dn13) / assign14610_e21670), ((p.p105 * locals.var_t1_dn14) / assign14610_e21670),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14610_e21673, assign14610_e21673_d_n0, assign14610_e21673_d_n2, assign14610_e21673_d_n3, assign14610_e21673_d_n4, assign14610_e21673_d_n5, assign14610_e21673_d_n6, assign14610_e21673_d_n7, assign14610_e21673_d_n8, assign14610_e21673_d_n9, assign14610_e21673_d_n10, assign14610_e21673_d_n11, assign14610_e21673_d_n13, assign14610_e21673_d_n14,)
            }
        };
        let assign14610_e21678: f64 = (2.0 * 8.617087e-5);
        let assign14610_e21680: f64 = (assign14610_e21678 * 300.15);
        let assign14610_e21681: f64 = (p.p106 / assign14610_e21680);
        let assign14610_e21682: f64 = (assign14610_e21674 + assign14610_e21681);
        let assign14610_e21686: f64 = (2.0 * locals.var_vtm);
        let assign14610_e21687: f64 = (locals.var_eg / assign14610_e21686);
        let assign14610_e21688: f64 = (assign14610_e21682 - assign14610_e21687);
        (assign14610_e21688, assign14610_e21674_d_n0, assign14610_e21674_d_n2, assign14610_e21674_d_n3, (assign14610_e21674_d_n4 - (((locals.var_eg_dn4 * assign14610_e21686) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign14610_e21686 * assign14610_e21686))), assign14610_e21674_d_n5, assign14610_e21674_d_n6, assign14610_e21674_d_n7, assign14610_e21674_d_n8, assign14610_e21674_d_n9, assign14610_e21674_d_n10, assign14610_e21674_d_n11, assign14610_e21674_d_n13, assign14610_e21674_d_n14,)
    } else {
        (locals.var_niln, locals.var_niln_dn0, locals.var_niln_dn2, locals.var_niln_dn3, locals.var_niln_dn4, locals.var_niln_dn5, locals.var_niln_dn6, locals.var_niln_dn7, locals.var_niln_dn8, locals.var_niln_dn9, locals.var_niln_dn10, locals.var_niln_dn11, locals.var_niln_dn13, locals.var_niln_dn14,)
    }
};
        locals.var_niln = assign14610_e21690;
        locals.var_niln_dn0 = assign14610_e21690_d_n0;
        locals.var_niln_dn2 = assign14610_e21690_d_n2;
        locals.var_niln_dn3 = assign14610_e21690_d_n3;
        locals.var_niln_dn4 = assign14610_e21690_d_n4;
        locals.var_niln_dn5 = assign14610_e21690_d_n5;
        locals.var_niln_dn6 = assign14610_e21690_d_n6;
        locals.var_niln_dn7 = assign14610_e21690_d_n7;
        locals.var_niln_dn8 = assign14610_e21690_d_n8;
        locals.var_niln_dn9 = assign14610_e21690_d_n9;
        locals.var_niln_dn10 = assign14610_e21690_d_n10;
        locals.var_niln_dn11 = assign14610_e21690_d_n11;
        locals.var_niln_dn13 = assign14610_e21690_d_n13;
        locals.var_niln_dn14 = assign14610_e21690_d_n14;
        locals.var_niln_rv = 0.0;

        let assign14620_e21694: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21695: f64 = (1.0 + assign14620_e21694);
        let assign14620_e21697: f64 = (assign14620_e21695 - 1e-6);
        let assign14620_e21699: f64 = (-10000.0);
        let assign14620_e21701: f64 = (assign14620_e21699 * 0.001);
        let (assign14620_e21762, assign14620_e21762_d_n4,) = {
    if (!(assign14620_e21697 < assign14620_e21701)) {
        let assign14620_e21708: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21709: f64 = (1.0 + assign14620_e21708);
        let assign14620_e21711: f64 = (assign14620_e21709 - 1e-6);
        let assign14620_e21715: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21716: f64 = (1.0 + assign14620_e21715);
        let assign14620_e21718: f64 = (assign14620_e21716 - 1e-6);
        let assign14620_e21722: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21723: f64 = (1.0 + assign14620_e21722);
        let assign14620_e21725: f64 = (assign14620_e21723 - 1e-6);
        let assign14620_e21726: f64 = (assign14620_e21718 * assign14620_e21725);
        let assign14620_e21729: f64 = (4.0 * 0.001);
        let assign14620_e21731: f64 = (assign14620_e21729 * 0.001);
        let assign14620_e21732: f64 = (assign14620_e21726 + assign14620_e21731);
        let assign14620_e21733: f64 = (assign14620_e21732).sqrt();
        let assign14620_e21734: f64 = (assign14620_e21711 + assign14620_e21733);
        let assign14620_e21735: f64 = (0.5 * assign14620_e21734);
        (assign14620_e21735, (0.5 * ((locals.var_tss_i * locals.var_deltemp_dn4) + ((((locals.var_tss_i * locals.var_deltemp_dn4) * assign14620_e21725) + (assign14620_e21718 * (locals.var_tss_i * locals.var_deltemp_dn4))) / (2.0 * assign14620_e21733)))),)
    } else {
        let assign14620_e21739: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21740: f64 = (1.0 + assign14620_e21739);
        let assign14620_e21742: f64 = (assign14620_e21740 - 1e-6);
        let assign14620_e21744: f64 = (-10000.0);
        let assign14620_e21746: f64 = (assign14620_e21744 * 0.001);
        let (assign14620_e21761, assign14620_e21761_d_n4,) = {
            if (assign14620_e21742 < assign14620_e21746) {
                let assign14620_e21749: f64 = (-0.001);
                let assign14620_e21751: f64 = (assign14620_e21749 * 0.001);
                let assign14620_e21755: f64 = (locals.var_tss_i * locals.var_deltemp);
                let assign14620_e21756: f64 = (1.0 + assign14620_e21755);
                let assign14620_e21758: f64 = (assign14620_e21756 - 1e-6);
                let assign14620_e21759: f64 = (assign14620_e21751 / assign14620_e21758);
                (assign14620_e21759, (-((assign14620_e21751 * (locals.var_tss_i * locals.var_deltemp_dn4)) / (assign14620_e21758 * assign14620_e21758))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14620_e21761, assign14620_e21761_d_n4,)
    }
};
        locals.var_thetass = assign14620_e21762;
        locals.var_thetass_dn4 = assign14620_e21762_d_n4;
        locals.var_thetass_rv = 0.0;

        let assign14630_e21765: f64 = (locals.var_vtm * 1.60219e-19);
        locals.var_kt = assign14630_e21765;
        locals.var_kt_dn4 = (locals.var_vtm_dn4 * 1.60219e-19);
        locals.var_kt_rv = 0.0;

        let assign14640_e21768: f64 = (1.05457e-34 * 3.141592653589793);
        let assign14640_e21771: f64 = (2.0 * locals.var_ach);
        let assign14640_e21773: f64 = (assign14640_e21771 / locals.var_weff_ufcm);
        let assign14640_e21774: f64 = (assign14640_e21768 / assign14640_e21773);
        locals.var_t0 = assign14640_e21774;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign14650_e21777: f64 = (locals.var_t0 * locals.var_t0);
        let assign14650_e21780: f64 = (2.0 * locals.var_mx);
        let assign14650_e21781: f64 = (assign14650_e21777 / assign14650_e21780);
        locals.var_e0_1 = assign14650_e21781;
        locals.var_e0_1_dn0 = (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / assign14650_e21780);
        locals.var_e0_1_dn2 = (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / assign14650_e21780);
        locals.var_e0_1_dn3 = (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / assign14650_e21780);
        locals.var_e0_1_dn4 = (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / assign14650_e21780);
        locals.var_e0_1_dn5 = (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / assign14650_e21780);
        locals.var_e0_1_dn6 = (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / assign14650_e21780);
        locals.var_e0_1_dn7 = (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / assign14650_e21780);
        locals.var_e0_1_dn8 = (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / assign14650_e21780);
        locals.var_e0_1_dn9 = (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / assign14650_e21780);
        locals.var_e0_1_dn10 = (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / assign14650_e21780);
        locals.var_e0_1_dn11 = (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / assign14650_e21780);
        locals.var_e0_1_dn13 = (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / assign14650_e21780);
        locals.var_e0_1_dn14 = (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / assign14650_e21780);
        locals.var_e0_1_rv = 0.0;

        let assign14660_e21784: f64 = (locals.var_t0 * locals.var_t0);
        let assign14660_e21787: f64 = (2.0 * locals.var_mxprime);
        let assign14660_e21788: f64 = (assign14660_e21784 / assign14660_e21787);
        locals.var_e0prime = assign14660_e21788;
        locals.var_e0prime_dn0 = (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / assign14660_e21787);
        locals.var_e0prime_dn2 = (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / assign14660_e21787);
        locals.var_e0prime_dn3 = (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / assign14660_e21787);
        locals.var_e0prime_dn4 = (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / assign14660_e21787);
        locals.var_e0prime_dn5 = (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / assign14660_e21787);
        locals.var_e0prime_dn6 = (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / assign14660_e21787);
        locals.var_e0prime_dn7 = (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / assign14660_e21787);
        locals.var_e0prime_dn8 = (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / assign14660_e21787);
        locals.var_e0prime_dn9 = (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / assign14660_e21787);
        locals.var_e0prime_dn10 = (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / assign14660_e21787);
        locals.var_e0prime_dn11 = (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / assign14660_e21787);
        locals.var_e0prime_dn13 = (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / assign14660_e21787);
        locals.var_e0prime_dn14 = (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / assign14660_e21787);
        locals.var_e0prime_rv = 0.0;

        let assign14670_e21791: f64 = (4.0 * locals.var_e0_1);
        locals.var_e1_1 = assign14670_e21791;
        locals.var_e1_1_dn0 = (4.0 * locals.var_e0_1_dn0);
        locals.var_e1_1_dn2 = (4.0 * locals.var_e0_1_dn2);
        locals.var_e1_1_dn3 = (4.0 * locals.var_e0_1_dn3);
        locals.var_e1_1_dn4 = (4.0 * locals.var_e0_1_dn4);
        locals.var_e1_1_dn5 = (4.0 * locals.var_e0_1_dn5);
        locals.var_e1_1_dn6 = (4.0 * locals.var_e0_1_dn6);
        locals.var_e1_1_dn7 = (4.0 * locals.var_e0_1_dn7);
        locals.var_e1_1_dn8 = (4.0 * locals.var_e0_1_dn8);
        locals.var_e1_1_dn9 = (4.0 * locals.var_e0_1_dn9);
        locals.var_e1_1_dn10 = (4.0 * locals.var_e0_1_dn10);
        locals.var_e1_1_dn11 = (4.0 * locals.var_e0_1_dn11);
        locals.var_e1_1_dn13 = (4.0 * locals.var_e0_1_dn13);
        locals.var_e1_1_dn14 = (4.0 * locals.var_e0_1_dn14);
        locals.var_e1_1_rv = 0.0;

        let assign14680_e21794: f64 = (4.0 * locals.var_e0prime);
        locals.var_e1prime = assign14680_e21794;
        locals.var_e1prime_dn0 = (4.0 * locals.var_e0prime_dn0);
        locals.var_e1prime_dn2 = (4.0 * locals.var_e0prime_dn2);
        locals.var_e1prime_dn3 = (4.0 * locals.var_e0prime_dn3);
        locals.var_e1prime_dn4 = (4.0 * locals.var_e0prime_dn4);
        locals.var_e1prime_dn5 = (4.0 * locals.var_e0prime_dn5);
        locals.var_e1prime_dn6 = (4.0 * locals.var_e0prime_dn6);
        locals.var_e1prime_dn7 = (4.0 * locals.var_e0prime_dn7);
        locals.var_e1prime_dn8 = (4.0 * locals.var_e0prime_dn8);
        locals.var_e1prime_dn9 = (4.0 * locals.var_e0prime_dn9);
        locals.var_e1prime_dn10 = (4.0 * locals.var_e0prime_dn10);
        locals.var_e1prime_dn11 = (4.0 * locals.var_e0prime_dn11);
        locals.var_e1prime_dn13 = (4.0 * locals.var_e0prime_dn13);
        locals.var_e1prime_dn14 = (4.0 * locals.var_e0prime_dn14);
        locals.var_e1prime_rv = 0.0;

        let assign14690_e21797: f64 = (locals.var_gprime * locals.var_mdprime);
        let assign14690_e21800: f64 = (locals.var_gfactor * locals.var_md);
        let assign14690_e21801: f64 = (assign14690_e21797 / assign14690_e21800);
        locals.var_t1 = assign14690_e21801;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign14700_e21806: f64 = (locals.var_e0_1 - locals.var_e0prime);
        let assign14700_e21808: f64 = (assign14700_e21806 / locals.var_kt);
        let assign14700_e21809: f64 = { let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14700_e21810: f64 = (locals.var_t1 * assign14700_e21809);
        let assign14700_e21811: f64 = (1.0 + assign14700_e21810);
        locals.var_gam0 = assign14700_e21811;
        locals.var_gam0_dn0 = ((locals.var_t1_dn0 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn0 - locals.var_e0prime_dn0) / locals.var_kt))));
        locals.var_gam0_dn2 = ((locals.var_t1_dn2 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn2 - locals.var_e0prime_dn2) / locals.var_kt))));
        locals.var_gam0_dn3 = ((locals.var_t1_dn3 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn3 - locals.var_e0prime_dn3) / locals.var_kt))));
        locals.var_gam0_dn4 = ((locals.var_t1_dn4 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_e0_1_dn4 - locals.var_e0prime_dn4) * locals.var_kt) - (assign14700_e21806 * locals.var_kt_dn4)) / (locals.var_kt * locals.var_kt)))));
        locals.var_gam0_dn5 = ((locals.var_t1_dn5 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn5 - locals.var_e0prime_dn5) / locals.var_kt))));
        locals.var_gam0_dn6 = ((locals.var_t1_dn6 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn6 - locals.var_e0prime_dn6) / locals.var_kt))));
        locals.var_gam0_dn7 = ((locals.var_t1_dn7 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn7 - locals.var_e0prime_dn7) / locals.var_kt))));
        locals.var_gam0_dn8 = ((locals.var_t1_dn8 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn8 - locals.var_e0prime_dn8) / locals.var_kt))));
        locals.var_gam0_dn9 = ((locals.var_t1_dn9 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn9 - locals.var_e0prime_dn9) / locals.var_kt))));
        locals.var_gam0_dn10 = ((locals.var_t1_dn10 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn10 - locals.var_e0prime_dn10) / locals.var_kt))));
        locals.var_gam0_dn11 = ((locals.var_t1_dn11 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn11 - locals.var_e0prime_dn11) / locals.var_kt))));
        locals.var_gam0_dn13 = ((locals.var_t1_dn13 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn13 - locals.var_e0prime_dn13) / locals.var_kt))));
        locals.var_gam0_dn14 = ((locals.var_t1_dn14 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn14 - locals.var_e0prime_dn14) / locals.var_kt))));
        locals.var_gam0_rv = 0.0;

        let assign14710_e21815: f64 = (locals.var_e0_1 - locals.var_e1_1);
        let assign14710_e21817: f64 = (assign14710_e21815 / locals.var_kt);
        let assign14710_e21818: f64 = { let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14710_e21819: f64 = (locals.var_gam0 + assign14710_e21818);
        let assign14710_e21823: f64 = (locals.var_e0_1 - locals.var_e1prime);
        let assign14710_e21825: f64 = (assign14710_e21823 / locals.var_kt);
        let assign14710_e21826: f64 = { let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14710_e21827: f64 = (locals.var_t1 * assign14710_e21826);
        let assign14710_e21828: f64 = (assign14710_e21819 + assign14710_e21827);
        locals.var_gam1 = assign14710_e21828;
        locals.var_gam1_dn0 = ((locals.var_gam0_dn0 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn0 - locals.var_e1_1_dn0) / locals.var_kt))) + ((locals.var_t1_dn0 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn0 - locals.var_e1prime_dn0) / locals.var_kt)))));
        locals.var_gam1_dn2 = ((locals.var_gam0_dn2 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn2 - locals.var_e1_1_dn2) / locals.var_kt))) + ((locals.var_t1_dn2 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn2 - locals.var_e1prime_dn2) / locals.var_kt)))));
        locals.var_gam1_dn3 = ((locals.var_gam0_dn3 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn3 - locals.var_e1_1_dn3) / locals.var_kt))) + ((locals.var_t1_dn3 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn3 - locals.var_e1prime_dn3) / locals.var_kt)))));
        locals.var_gam1_dn4 = ((locals.var_gam0_dn4 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_e0_1_dn4 - locals.var_e1_1_dn4) * locals.var_kt) - (assign14710_e21815 * locals.var_kt_dn4)) / (locals.var_kt * locals.var_kt)))) + ((locals.var_t1_dn4 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_e0_1_dn4 - locals.var_e1prime_dn4) * locals.var_kt) - (assign14710_e21823 * locals.var_kt_dn4)) / (locals.var_kt * locals.var_kt))))));
        locals.var_gam1_dn5 = ((locals.var_gam0_dn5 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn5 - locals.var_e1_1_dn5) / locals.var_kt))) + ((locals.var_t1_dn5 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn5 - locals.var_e1prime_dn5) / locals.var_kt)))));
        locals.var_gam1_dn6 = ((locals.var_gam0_dn6 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn6 - locals.var_e1_1_dn6) / locals.var_kt))) + ((locals.var_t1_dn6 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn6 - locals.var_e1prime_dn6) / locals.var_kt)))));
        locals.var_gam1_dn7 = ((locals.var_gam0_dn7 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn7 - locals.var_e1_1_dn7) / locals.var_kt))) + ((locals.var_t1_dn7 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn7 - locals.var_e1prime_dn7) / locals.var_kt)))));
        locals.var_gam1_dn8 = ((locals.var_gam0_dn8 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn8 - locals.var_e1_1_dn8) / locals.var_kt))) + ((locals.var_t1_dn8 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn8 - locals.var_e1prime_dn8) / locals.var_kt)))));
        locals.var_gam1_dn9 = ((locals.var_gam0_dn9 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn9 - locals.var_e1_1_dn9) / locals.var_kt))) + ((locals.var_t1_dn9 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn9 - locals.var_e1prime_dn9) / locals.var_kt)))));
        locals.var_gam1_dn10 = ((locals.var_gam0_dn10 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn10 - locals.var_e1_1_dn10) / locals.var_kt))) + ((locals.var_t1_dn10 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn10 - locals.var_e1prime_dn10) / locals.var_kt)))));
        locals.var_gam1_dn11 = ((locals.var_gam0_dn11 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn11 - locals.var_e1_1_dn11) / locals.var_kt))) + ((locals.var_t1_dn11 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn11 - locals.var_e1prime_dn11) / locals.var_kt)))));
        locals.var_gam1_dn13 = ((locals.var_gam0_dn13 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn13 - locals.var_e1_1_dn13) / locals.var_kt))) + ((locals.var_t1_dn13 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn13 - locals.var_e1prime_dn13) / locals.var_kt)))));
        locals.var_gam1_dn14 = ((locals.var_gam0_dn14 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn14 - locals.var_e1_1_dn14) / locals.var_kt))) + ((locals.var_t1_dn14 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn14 - locals.var_e1prime_dn14) / locals.var_kt)))));
        locals.var_gam1_rv = 0.0;

        let assign14720_e21830: f64 = (-locals.var_vtm);
        let assign14720_e21833: f64 = (locals.var_gfactor * locals.var_md);
        let assign14720_e21836: f64 = (3.141592653589793 * 1.05457e-34);
        let assign14720_e21838: f64 = (assign14720_e21836 * 1.05457e-34);
        let assign14720_e21840: f64 = (assign14720_e21838 * locals.var_nc);
        let assign14720_e21841: f64 = (assign14720_e21833 / assign14720_e21840);
        let assign14720_e21843: f64 = (assign14720_e21841 * locals.var_kt);
        let assign14720_e21846: f64 = (2.0 * locals.var_ach);
        let assign14720_e21848: f64 = (assign14720_e21846 / locals.var_weff_ufcm);
        let assign14720_e21849: f64 = (assign14720_e21843 / assign14720_e21848);
        let assign14720_e21851: f64 = (assign14720_e21849 * locals.var_gam1);
        let (assign14720_e21904, assign14720_e21904_d_n0, assign14720_e21904_d_n2, assign14720_e21904_d_n3, assign14720_e21904_d_n4, assign14720_e21904_d_n5, assign14720_e21904_d_n6, assign14720_e21904_d_n7, assign14720_e21904_d_n8, assign14720_e21904_d_n9, assign14720_e21904_d_n10, assign14720_e21904_d_n11, assign14720_e21904_d_n13, assign14720_e21904_d_n14,) = {
    if (!(assign14720_e21851 > 1e-38)) {
        let assign14720_e21856: f64 = (-87.498233534);
        (assign14720_e21856, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign14720_e21859: f64 = (locals.var_gfactor * locals.var_md);
        let assign14720_e21862: f64 = (3.141592653589793 * 1.05457e-34);
        let assign14720_e21864: f64 = (assign14720_e21862 * 1.05457e-34);
        let assign14720_e21866: f64 = (assign14720_e21864 * locals.var_nc);
        let assign14720_e21867: f64 = (assign14720_e21859 / assign14720_e21866);
        let assign14720_e21869: f64 = (assign14720_e21867 * locals.var_kt);
        let assign14720_e21872: f64 = (2.0 * locals.var_ach);
        let assign14720_e21874: f64 = (assign14720_e21872 / locals.var_weff_ufcm);
        let assign14720_e21875: f64 = (assign14720_e21869 / assign14720_e21874);
        let assign14720_e21877: f64 = (assign14720_e21875 * locals.var_gam1);
        let (assign14720_e21903, assign14720_e21903_d_n0, assign14720_e21903_d_n2, assign14720_e21903_d_n3, assign14720_e21903_d_n4, assign14720_e21903_d_n5, assign14720_e21903_d_n6, assign14720_e21903_d_n7, assign14720_e21903_d_n8, assign14720_e21903_d_n9, assign14720_e21903_d_n10, assign14720_e21903_d_n11, assign14720_e21903_d_n13, assign14720_e21903_d_n14,) = {
            if (assign14720_e21877 > 1e-38) {
                let assign14720_e21882: f64 = (locals.var_gfactor * locals.var_md);
                let assign14720_e21885: f64 = (3.141592653589793 * 1.05457e-34);
                let assign14720_e21887: f64 = (assign14720_e21885 * 1.05457e-34);
                let assign14720_e21889: f64 = (assign14720_e21887 * locals.var_nc);
                let assign14720_e21890: f64 = (assign14720_e21882 / assign14720_e21889);
                let assign14720_e21892: f64 = (assign14720_e21890 * locals.var_kt);
                let assign14720_e21895: f64 = (2.0 * locals.var_ach);
                let assign14720_e21897: f64 = (assign14720_e21895 / locals.var_weff_ufcm);
                let assign14720_e21898: f64 = (assign14720_e21892 / assign14720_e21897);
                let assign14720_e21900: f64 = (assign14720_e21898 * locals.var_gam1);
                let assign14720_e21901: f64 = (assign14720_e21900).ln();
                (assign14720_e21901, ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn0)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn0)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn2)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn2)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn3)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn3)) / assign14720_e21900), (((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn4)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) + (assign14720_e21890 * locals.var_kt_dn4)) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn4)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn5)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn5)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn6)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn6)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn7)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn7)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn8)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn8)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn9)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn9)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn10)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn10)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn11)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn11)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn13)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn13)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn14)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn14)) / assign14720_e21900),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14720_e21903, assign14720_e21903_d_n0, assign14720_e21903_d_n2, assign14720_e21903_d_n3, assign14720_e21903_d_n4, assign14720_e21903_d_n5, assign14720_e21903_d_n6, assign14720_e21903_d_n7, assign14720_e21903_d_n8, assign14720_e21903_d_n9, assign14720_e21903_d_n10, assign14720_e21903_d_n11, assign14720_e21903_d_n13, assign14720_e21903_d_n14,)
    }
};
        let assign14720_e21905: f64 = (assign14720_e21830 * assign14720_e21904);
        locals.var_t2 = assign14720_e21905;
        locals.var_t2_dn0 = (assign14720_e21830 * assign14720_e21904_d_n0);
        locals.var_t2_dn2 = (assign14720_e21830 * assign14720_e21904_d_n2);
        locals.var_t2_dn3 = (assign14720_e21830 * assign14720_e21904_d_n3);
        locals.var_t2_dn4 = (((-locals.var_vtm_dn4) * assign14720_e21904) + (assign14720_e21830 * assign14720_e21904_d_n4));
        locals.var_t2_dn5 = (assign14720_e21830 * assign14720_e21904_d_n5);
        locals.var_t2_dn6 = (assign14720_e21830 * assign14720_e21904_d_n6);
        locals.var_t2_dn7 = (assign14720_e21830 * assign14720_e21904_d_n7);
        locals.var_t2_dn8 = (assign14720_e21830 * assign14720_e21904_d_n8);
        locals.var_t2_dn9 = (assign14720_e21830 * assign14720_e21904_d_n9);
        locals.var_t2_dn10 = (assign14720_e21830 * assign14720_e21904_d_n10);
        locals.var_t2_dn11 = (assign14720_e21830 * assign14720_e21904_d_n11);
        locals.var_t2_dn13 = (assign14720_e21830 * assign14720_e21904_d_n13);
        locals.var_t2_dn14 = (assign14720_e21830 * assign14720_e21904_d_n14);
        locals.var_t2_rv = 0.0;

        let assign14730_e21909: f64 = (locals.var_e0_1 / 1.60219e-19);
        let assign14730_e21911: f64 = (assign14730_e21909 + locals.var_t2);
        let assign14730_e21912: f64 = (locals.var_qmfactor_i * assign14730_e21911);
        locals.var_dvch_qm = assign14730_e21912;
        locals.var_dvch_qm_dn0 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn0 / 1.60219e-19) + locals.var_t2_dn0));
        locals.var_dvch_qm_dn2 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn2 / 1.60219e-19) + locals.var_t2_dn2));
        locals.var_dvch_qm_dn3 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn3 / 1.60219e-19) + locals.var_t2_dn3));
        locals.var_dvch_qm_dn4 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn4 / 1.60219e-19) + locals.var_t2_dn4));
        locals.var_dvch_qm_dn5 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn5 / 1.60219e-19) + locals.var_t2_dn5));
        locals.var_dvch_qm_dn6 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn6 / 1.60219e-19) + locals.var_t2_dn6));
        locals.var_dvch_qm_dn7 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn7 / 1.60219e-19) + locals.var_t2_dn7));
        locals.var_dvch_qm_dn8 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn8 / 1.60219e-19) + locals.var_t2_dn8));
        locals.var_dvch_qm_dn9 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn9 / 1.60219e-19) + locals.var_t2_dn9));
        locals.var_dvch_qm_dn10 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn10 / 1.60219e-19) + locals.var_t2_dn10));
        locals.var_dvch_qm_dn11 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn11 / 1.60219e-19) + locals.var_t2_dn11));
        locals.var_dvch_qm_dn13 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn13 / 1.60219e-19) + locals.var_t2_dn13));
        locals.var_dvch_qm_dn14 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn14 / 1.60219e-19) + locals.var_t2_dn14));
        locals.var_dvch_qm_rv = 0.0;

        let assign14740_e21914: f64 = (locals.var_tratio).ln();
        locals.var_trat_ln = assign14740_e21914;
        locals.var_trat_ln_dn4 = (locals.var_tratio_dn4 / locals.var_tratio);
        locals.var_trat_ln_rv = 0.0;

        let assign14750_e21917: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard244 = assign14750_e21917;
        locals.var_guard244_rv = 0.0;

        let (assign14760_e21926, assign14760_e21926_d_n0, assign14760_e21926_d_n2, assign14760_e21926_d_n3, assign14760_e21926_d_n4, assign14760_e21926_d_n5, assign14760_e21926_d_n6, assign14760_e21926_d_n7, assign14760_e21926_d_n8, assign14760_e21926_d_n9, assign14760_e21926_d_n10, assign14760_e21926_d_n11, assign14760_e21926_d_n13, assign14760_e21926_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14760_e21922: f64 = (locals.var_ute_i * locals.var_trat_ln);
        let assign14760_e21923: f64 = (assign14760_e21922).exp();
        let assign14760_e21924: f64 = (locals.var_u0_i * assign14760_e21923);
        (assign14760_e21924, (locals.var_u0_i_dn0 * assign14760_e21923), (locals.var_u0_i_dn2 * assign14760_e21923), (locals.var_u0_i_dn3 * assign14760_e21923), ((locals.var_u0_i_dn4 * assign14760_e21923) + (locals.var_u0_i * (assign14760_e21923 * (locals.var_ute_i * locals.var_trat_ln_dn4)))), (locals.var_u0_i_dn5 * assign14760_e21923), (locals.var_u0_i_dn6 * assign14760_e21923), (locals.var_u0_i_dn7 * assign14760_e21923), (locals.var_u0_i_dn8 * assign14760_e21923), (locals.var_u0_i_dn9 * assign14760_e21923), (locals.var_u0_i_dn10 * assign14760_e21923), (locals.var_u0_i_dn11 * assign14760_e21923), (locals.var_u0_i_dn13 * assign14760_e21923), (locals.var_u0_i_dn14 * assign14760_e21923),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14760_e21926;
        locals.var_t1_dn0 = assign14760_e21926_d_n0;
        locals.var_t1_dn2 = assign14760_e21926_d_n2;
        locals.var_t1_dn3 = assign14760_e21926_d_n3;
        locals.var_t1_dn4 = assign14760_e21926_d_n4;
        locals.var_t1_dn5 = assign14760_e21926_d_n5;
        locals.var_t1_dn6 = assign14760_e21926_d_n6;
        locals.var_t1_dn7 = assign14760_e21926_d_n7;
        locals.var_t1_dn8 = assign14760_e21926_d_n8;
        locals.var_t1_dn9 = assign14760_e21926_d_n9;
        locals.var_t1_dn10 = assign14760_e21926_d_n10;
        locals.var_t1_dn11 = assign14760_e21926_d_n11;
        locals.var_t1_dn13 = assign14760_e21926_d_n13;
        locals.var_t1_dn14 = assign14760_e21926_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign14770_e21980, assign14770_e21980_d_n0, assign14770_e21980_d_n2, assign14770_e21980_d_n3, assign14770_e21980_d_n4, assign14770_e21980_d_n5, assign14770_e21980_d_n6, assign14770_e21980_d_n7, assign14770_e21980_d_n8, assign14770_e21980_d_n9, assign14770_e21980_d_n10, assign14770_e21980_d_n11, assign14770_e21980_d_n13, assign14770_e21980_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14770_e21930: f64 = (-0.9);
        let assign14770_e21932: f64 = (assign14770_e21930 * locals.var_t1);
        let assign14770_e21936: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign14770_e21938: f64 = (-0.9);
        let assign14770_e21940: f64 = (assign14770_e21938 * locals.var_t1);
        let assign14770_e21941: f64 = (assign14770_e21936 - assign14770_e21940);
        let assign14770_e21943: f64 = (assign14770_e21941 - 0.0001);
        let assign14770_e21946: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign14770_e21948: f64 = (-0.9);
        let assign14770_e21950: f64 = (assign14770_e21948 * locals.var_t1);
        let assign14770_e21951: f64 = (assign14770_e21946 - assign14770_e21950);
        let assign14770_e21953: f64 = (assign14770_e21951 - 0.0001);
        let assign14770_e21956: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign14770_e21958: f64 = (-0.9);
        let assign14770_e21960: f64 = (assign14770_e21958 * locals.var_t1);
        let assign14770_e21961: f64 = (assign14770_e21956 - assign14770_e21960);
        let assign14770_e21963: f64 = (assign14770_e21961 - 0.0001);
        let assign14770_e21964: f64 = (assign14770_e21953 * assign14770_e21963);
        let assign14770_e21967: f64 = (-0.9);
        let assign14770_e21969: f64 = (assign14770_e21967 * locals.var_t1);
        let assign14770_e21970: f64 = (4.0 * assign14770_e21969);
        let assign14770_e21972: f64 = (assign14770_e21970 * 0.0001);
        let assign14770_e21973: f64 = (assign14770_e21964 - assign14770_e21972);
        let assign14770_e21974: f64 = (assign14770_e21973).sqrt();
        let assign14770_e21975: f64 = (assign14770_e21943 + assign14770_e21974);
        let assign14770_e21976: f64 = (0.5 * assign14770_e21975);
        let assign14770_e21977: f64 = (assign14770_e21932 + assign14770_e21976);
        let assign14770_e21978: f64 = (locals.var_t1 + assign14770_e21977);
        (assign14770_e21978, (locals.var_t1_dn0 + ((assign14770_e21930 * locals.var_t1_dn0) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn0)) + (((((-(assign14770_e21948 * locals.var_t1_dn0)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn0)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn2 + ((assign14770_e21930 * locals.var_t1_dn2) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn2)) + (((((-(assign14770_e21948 * locals.var_t1_dn2)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn2)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn3 + ((assign14770_e21930 * locals.var_t1_dn3) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn3)) + (((((-(assign14770_e21948 * locals.var_t1_dn3)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn3)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn4 + ((assign14770_e21930 * locals.var_t1_dn4) + (0.5 * (((locals.var_utl_i * locals.var_deltemp_dn4) - (assign14770_e21938 * locals.var_t1_dn4)) + ((((((locals.var_utl_i * locals.var_deltemp_dn4) - (assign14770_e21948 * locals.var_t1_dn4)) * assign14770_e21963) + (assign14770_e21953 * ((locals.var_utl_i * locals.var_deltemp_dn4) - (assign14770_e21958 * locals.var_t1_dn4)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn5 + ((assign14770_e21930 * locals.var_t1_dn5) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn5)) + (((((-(assign14770_e21948 * locals.var_t1_dn5)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn5)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn6 + ((assign14770_e21930 * locals.var_t1_dn6) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn6)) + (((((-(assign14770_e21948 * locals.var_t1_dn6)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn6)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn7 + ((assign14770_e21930 * locals.var_t1_dn7) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn7)) + (((((-(assign14770_e21948 * locals.var_t1_dn7)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn7)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn8 + ((assign14770_e21930 * locals.var_t1_dn8) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn8)) + (((((-(assign14770_e21948 * locals.var_t1_dn8)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn8)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn9 + ((assign14770_e21930 * locals.var_t1_dn9) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn9)) + (((((-(assign14770_e21948 * locals.var_t1_dn9)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn9)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn10 + ((assign14770_e21930 * locals.var_t1_dn10) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn10)) + (((((-(assign14770_e21948 * locals.var_t1_dn10)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn10)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn11 + ((assign14770_e21930 * locals.var_t1_dn11) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn11)) + (((((-(assign14770_e21948 * locals.var_t1_dn11)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn11)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn13 + ((assign14770_e21930 * locals.var_t1_dn13) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn13)) + (((((-(assign14770_e21948 * locals.var_t1_dn13)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn13)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn14 + ((assign14770_e21930 * locals.var_t1_dn14) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn14)) + (((((-(assign14770_e21948 * locals.var_t1_dn14)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn14)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign14770_e21974)))))),)
    } else {
        (locals.var_u0_v, locals.var_u0_v_dn0, locals.var_u0_v_dn2, locals.var_u0_v_dn3, locals.var_u0_v_dn4, locals.var_u0_v_dn5, locals.var_u0_v_dn6, locals.var_u0_v_dn7, locals.var_u0_v_dn8, locals.var_u0_v_dn9, locals.var_u0_v_dn10, locals.var_u0_v_dn11, locals.var_u0_v_dn13, locals.var_u0_v_dn14,)
    }
};
        locals.var_u0_v = assign14770_e21980;
        locals.var_u0_v_dn0 = assign14770_e21980_d_n0;
        locals.var_u0_v_dn2 = assign14770_e21980_d_n2;
        locals.var_u0_v_dn3 = assign14770_e21980_d_n3;
        locals.var_u0_v_dn4 = assign14770_e21980_d_n4;
        locals.var_u0_v_dn5 = assign14770_e21980_d_n5;
        locals.var_u0_v_dn6 = assign14770_e21980_d_n6;
        locals.var_u0_v_dn7 = assign14770_e21980_d_n7;
        locals.var_u0_v_dn8 = assign14770_e21980_d_n8;
        locals.var_u0_v_dn9 = assign14770_e21980_d_n9;
        locals.var_u0_v_dn10 = assign14770_e21980_d_n10;
        locals.var_u0_v_dn11 = assign14770_e21980_d_n11;
        locals.var_u0_v_dn13 = assign14770_e21980_d_n13;
        locals.var_u0_v_dn14 = assign14770_e21980_d_n14;
        locals.var_u0_v_rv = 0.0;

        let assign14780_e21983: f64 = if p.p66 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard245 = assign14780_e21983;
        locals.var_guard245_rv = 0.0;

        let (assign14790_e21994, assign14790_e21994_d_n0, assign14790_e21994_d_n2, assign14790_e21994_d_n3, assign14790_e21994_d_n4, assign14790_e21994_d_n5, assign14790_e21994_d_n6, assign14790_e21994_d_n7, assign14790_e21994_d_n8, assign14790_e21994_d_n9, assign14790_e21994_d_n10, assign14790_e21994_d_n11, assign14790_e21994_d_n13, assign14790_e21994_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        let assign14790_e21990: f64 = (locals.var_uter_i * locals.var_trat_ln);
        let assign14790_e21991: f64 = (assign14790_e21990).exp();
        let assign14790_e21992: f64 = (locals.var_u0r_i * assign14790_e21991);
        (assign14790_e21992, (locals.var_u0r_i_dn0 * assign14790_e21991), (locals.var_u0r_i_dn2 * assign14790_e21991), (locals.var_u0r_i_dn3 * assign14790_e21991), ((locals.var_u0r_i_dn4 * assign14790_e21991) + (locals.var_u0r_i * (assign14790_e21991 * (locals.var_uter_i * locals.var_trat_ln_dn4)))), (locals.var_u0r_i_dn5 * assign14790_e21991), (locals.var_u0r_i_dn6 * assign14790_e21991), (locals.var_u0r_i_dn7 * assign14790_e21991), (locals.var_u0r_i_dn8 * assign14790_e21991), (locals.var_u0r_i_dn9 * assign14790_e21991), (locals.var_u0r_i_dn10 * assign14790_e21991), (locals.var_u0r_i_dn11 * assign14790_e21991), (locals.var_u0r_i_dn13 * assign14790_e21991), (locals.var_u0r_i_dn14 * assign14790_e21991),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14790_e21994;
        locals.var_t1_dn0 = assign14790_e21994_d_n0;
        locals.var_t1_dn2 = assign14790_e21994_d_n2;
        locals.var_t1_dn3 = assign14790_e21994_d_n3;
        locals.var_t1_dn4 = assign14790_e21994_d_n4;
        locals.var_t1_dn5 = assign14790_e21994_d_n5;
        locals.var_t1_dn6 = assign14790_e21994_d_n6;
        locals.var_t1_dn7 = assign14790_e21994_d_n7;
        locals.var_t1_dn8 = assign14790_e21994_d_n8;
        locals.var_t1_dn9 = assign14790_e21994_d_n9;
        locals.var_t1_dn10 = assign14790_e21994_d_n10;
        locals.var_t1_dn11 = assign14790_e21994_d_n11;
        locals.var_t1_dn13 = assign14790_e21994_d_n13;
        locals.var_t1_dn14 = assign14790_e21994_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14800_e22050, assign14800_e22050_d_n0, assign14800_e22050_d_n2, assign14800_e22050_d_n3, assign14800_e22050_d_n4, assign14800_e22050_d_n5, assign14800_e22050_d_n6, assign14800_e22050_d_n7, assign14800_e22050_d_n8, assign14800_e22050_d_n9, assign14800_e22050_d_n10, assign14800_e22050_d_n11, assign14800_e22050_d_n13, assign14800_e22050_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        let assign14800_e22000: f64 = (-0.9);
        let assign14800_e22002: f64 = (assign14800_e22000 * locals.var_t1);
        let assign14800_e22006: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign14800_e22008: f64 = (-0.9);
        let assign14800_e22010: f64 = (assign14800_e22008 * locals.var_t1);
        let assign14800_e22011: f64 = (assign14800_e22006 - assign14800_e22010);
        let assign14800_e22013: f64 = (assign14800_e22011 - 0.0001);
        let assign14800_e22016: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign14800_e22018: f64 = (-0.9);
        let assign14800_e22020: f64 = (assign14800_e22018 * locals.var_t1);
        let assign14800_e22021: f64 = (assign14800_e22016 - assign14800_e22020);
        let assign14800_e22023: f64 = (assign14800_e22021 - 0.0001);
        let assign14800_e22026: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign14800_e22028: f64 = (-0.9);
        let assign14800_e22030: f64 = (assign14800_e22028 * locals.var_t1);
        let assign14800_e22031: f64 = (assign14800_e22026 - assign14800_e22030);
        let assign14800_e22033: f64 = (assign14800_e22031 - 0.0001);
        let assign14800_e22034: f64 = (assign14800_e22023 * assign14800_e22033);
        let assign14800_e22037: f64 = (-0.9);
        let assign14800_e22039: f64 = (assign14800_e22037 * locals.var_t1);
        let assign14800_e22040: f64 = (4.0 * assign14800_e22039);
        let assign14800_e22042: f64 = (assign14800_e22040 * 0.0001);
        let assign14800_e22043: f64 = (assign14800_e22034 - assign14800_e22042);
        let assign14800_e22044: f64 = (assign14800_e22043).sqrt();
        let assign14800_e22045: f64 = (assign14800_e22013 + assign14800_e22044);
        let assign14800_e22046: f64 = (0.5 * assign14800_e22045);
        let assign14800_e22047: f64 = (assign14800_e22002 + assign14800_e22046);
        let assign14800_e22048: f64 = (locals.var_t1 + assign14800_e22047);
        (assign14800_e22048, (locals.var_t1_dn0 + ((assign14800_e22000 * locals.var_t1_dn0) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn0)) + (((((-(assign14800_e22018 * locals.var_t1_dn0)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn0)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn2 + ((assign14800_e22000 * locals.var_t1_dn2) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn2)) + (((((-(assign14800_e22018 * locals.var_t1_dn2)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn2)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn3 + ((assign14800_e22000 * locals.var_t1_dn3) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn3)) + (((((-(assign14800_e22018 * locals.var_t1_dn3)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn3)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn4 + ((assign14800_e22000 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign14800_e22008 * locals.var_t1_dn4)) + ((((((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign14800_e22018 * locals.var_t1_dn4)) * assign14800_e22033) + (assign14800_e22023 * ((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign14800_e22028 * locals.var_t1_dn4)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn5 + ((assign14800_e22000 * locals.var_t1_dn5) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn5)) + (((((-(assign14800_e22018 * locals.var_t1_dn5)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn5)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn6 + ((assign14800_e22000 * locals.var_t1_dn6) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn6)) + (((((-(assign14800_e22018 * locals.var_t1_dn6)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn6)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn7 + ((assign14800_e22000 * locals.var_t1_dn7) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn7)) + (((((-(assign14800_e22018 * locals.var_t1_dn7)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn7)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn8 + ((assign14800_e22000 * locals.var_t1_dn8) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn8)) + (((((-(assign14800_e22018 * locals.var_t1_dn8)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn8)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn9 + ((assign14800_e22000 * locals.var_t1_dn9) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn9)) + (((((-(assign14800_e22018 * locals.var_t1_dn9)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn9)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn10 + ((assign14800_e22000 * locals.var_t1_dn10) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn10)) + (((((-(assign14800_e22018 * locals.var_t1_dn10)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn10)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn11 + ((assign14800_e22000 * locals.var_t1_dn11) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn11)) + (((((-(assign14800_e22018 * locals.var_t1_dn11)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn11)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn13 + ((assign14800_e22000 * locals.var_t1_dn13) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn13)) + (((((-(assign14800_e22018 * locals.var_t1_dn13)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn13)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn14 + ((assign14800_e22000 * locals.var_t1_dn14) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn14)) + (((((-(assign14800_e22018 * locals.var_t1_dn14)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn14)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign14800_e22044)))))),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    }
};
        locals.var_u0r_t = assign14800_e22050;
        locals.var_u0r_t_dn0 = assign14800_e22050_d_n0;
        locals.var_u0r_t_dn2 = assign14800_e22050_d_n2;
        locals.var_u0r_t_dn3 = assign14800_e22050_d_n3;
        locals.var_u0r_t_dn4 = assign14800_e22050_d_n4;
        locals.var_u0r_t_dn5 = assign14800_e22050_d_n5;
        locals.var_u0r_t_dn6 = assign14800_e22050_d_n6;
        locals.var_u0r_t_dn7 = assign14800_e22050_d_n7;
        locals.var_u0r_t_dn8 = assign14800_e22050_d_n8;
        locals.var_u0r_t_dn9 = assign14800_e22050_d_n9;
        locals.var_u0r_t_dn10 = assign14800_e22050_d_n10;
        locals.var_u0r_t_dn11 = assign14800_e22050_d_n11;
        locals.var_u0r_t_dn13 = assign14800_e22050_d_n13;
        locals.var_u0r_t_dn14 = assign14800_e22050_d_n14;
        locals.var_u0r_t_rv = 0.0;

        let (assign14810_e22056, assign14810_e22056_d_n0, assign14810_e22056_d_n2, assign14810_e22056_d_n3, assign14810_e22056_d_n4, assign14810_e22056_d_n5, assign14810_e22056_d_n6, assign14810_e22056_d_n7, assign14810_e22056_d_n8, assign14810_e22056_d_n9, assign14810_e22056_d_n10, assign14810_e22056_d_n11, assign14810_e22056_d_n13, assign14810_e22056_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    } else {
        (locals.var_u0r_v, locals.var_u0r_v_dn0, locals.var_u0r_v_dn2, locals.var_u0r_v_dn3, locals.var_u0r_v_dn4, locals.var_u0r_v_dn5, locals.var_u0r_v_dn6, locals.var_u0r_v_dn7, locals.var_u0r_v_dn8, locals.var_u0r_v_dn9, locals.var_u0r_v_dn10, locals.var_u0r_v_dn11, locals.var_u0r_v_dn13, locals.var_u0r_v_dn14,)
    }
};
        locals.var_u0r_v = assign14810_e22056;
        locals.var_u0r_v_dn0 = assign14810_e22056_d_n0;
        locals.var_u0r_v_dn2 = assign14810_e22056_d_n2;
        locals.var_u0r_v_dn3 = assign14810_e22056_d_n3;
        locals.var_u0r_v_dn4 = assign14810_e22056_d_n4;
        locals.var_u0r_v_dn5 = assign14810_e22056_d_n5;
        locals.var_u0r_v_dn6 = assign14810_e22056_d_n6;
        locals.var_u0r_v_dn7 = assign14810_e22056_d_n7;
        locals.var_u0r_v_dn8 = assign14810_e22056_d_n8;
        locals.var_u0r_v_dn9 = assign14810_e22056_d_n9;
        locals.var_u0r_v_dn10 = assign14810_e22056_d_n10;
        locals.var_u0r_v_dn11 = assign14810_e22056_d_n11;
        locals.var_u0r_v_dn13 = assign14810_e22056_d_n13;
        locals.var_u0r_v_dn14 = assign14810_e22056_d_n14;
        locals.var_u0r_v_rv = 0.0;

        let (assign14820_e22100, assign14820_e22100_d_n0, assign14820_e22100_d_n2, assign14820_e22100_d_n3, assign14820_e22100_d_n4, assign14820_e22100_d_n5, assign14820_e22100_d_n6, assign14820_e22100_d_n7, assign14820_e22100_d_n8, assign14820_e22100_d_n9, assign14820_e22100_d_n10, assign14820_e22100_d_n11, assign14820_e22100_d_n13, assign14820_e22100_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14820_e22060: f64 = (-locals.var_ua_i);
        let assign14820_e22064: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign14820_e22066: f64 = (-locals.var_ua_i);
        let assign14820_e22067: f64 = (assign14820_e22064 - assign14820_e22066);
        let assign14820_e22069: f64 = (assign14820_e22067 - 1e-6);
        let assign14820_e22072: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign14820_e22074: f64 = (-locals.var_ua_i);
        let assign14820_e22075: f64 = (assign14820_e22072 - assign14820_e22074);
        let assign14820_e22077: f64 = (assign14820_e22075 - 1e-6);
        let assign14820_e22080: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign14820_e22082: f64 = (-locals.var_ua_i);
        let assign14820_e22083: f64 = (assign14820_e22080 - assign14820_e22082);
        let assign14820_e22085: f64 = (assign14820_e22083 - 1e-6);
        let assign14820_e22086: f64 = (assign14820_e22077 * assign14820_e22085);
        let assign14820_e22089: f64 = (-locals.var_ua_i);
        let assign14820_e22090: f64 = (4.0 * assign14820_e22089);
        let assign14820_e22092: f64 = (assign14820_e22090 * 1e-6);
        let assign14820_e22093: f64 = (assign14820_e22086 - assign14820_e22092);
        let assign14820_e22094: f64 = (assign14820_e22093).sqrt();
        let assign14820_e22095: f64 = (assign14820_e22069 + assign14820_e22094);
        let assign14820_e22096: f64 = (0.5 * assign14820_e22095);
        let assign14820_e22097: f64 = (assign14820_e22060 + assign14820_e22096);
        let assign14820_e22098: f64 = (locals.var_ua_i + assign14820_e22097);
        (assign14820_e22098, (locals.var_ua_i_dn0 + ((-locals.var_ua_i_dn0) + (0.5 * ((-(-locals.var_ua_i_dn0)) + (((((-(-locals.var_ua_i_dn0)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn0)))) - ((4.0 * (-locals.var_ua_i_dn0)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn2 + ((-locals.var_ua_i_dn2) + (0.5 * ((-(-locals.var_ua_i_dn2)) + (((((-(-locals.var_ua_i_dn2)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn2)))) - ((4.0 * (-locals.var_ua_i_dn2)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn3 + ((-locals.var_ua_i_dn3) + (0.5 * ((-(-locals.var_ua_i_dn3)) + (((((-(-locals.var_ua_i_dn3)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn3)))) - ((4.0 * (-locals.var_ua_i_dn3)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn4 + ((-locals.var_ua_i_dn4) + (0.5 * (((locals.var_ua1_i * locals.var_deltemp_dn4) - (-locals.var_ua_i_dn4)) + ((((((locals.var_ua1_i * locals.var_deltemp_dn4) - (-locals.var_ua_i_dn4)) * assign14820_e22085) + (assign14820_e22077 * ((locals.var_ua1_i * locals.var_deltemp_dn4) - (-locals.var_ua_i_dn4)))) - ((4.0 * (-locals.var_ua_i_dn4)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn5 + ((-locals.var_ua_i_dn5) + (0.5 * ((-(-locals.var_ua_i_dn5)) + (((((-(-locals.var_ua_i_dn5)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn5)))) - ((4.0 * (-locals.var_ua_i_dn5)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn6 + ((-locals.var_ua_i_dn6) + (0.5 * ((-(-locals.var_ua_i_dn6)) + (((((-(-locals.var_ua_i_dn6)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn6)))) - ((4.0 * (-locals.var_ua_i_dn6)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn7 + ((-locals.var_ua_i_dn7) + (0.5 * ((-(-locals.var_ua_i_dn7)) + (((((-(-locals.var_ua_i_dn7)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn7)))) - ((4.0 * (-locals.var_ua_i_dn7)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn8 + ((-locals.var_ua_i_dn8) + (0.5 * ((-(-locals.var_ua_i_dn8)) + (((((-(-locals.var_ua_i_dn8)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn8)))) - ((4.0 * (-locals.var_ua_i_dn8)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn9 + ((-locals.var_ua_i_dn9) + (0.5 * ((-(-locals.var_ua_i_dn9)) + (((((-(-locals.var_ua_i_dn9)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn9)))) - ((4.0 * (-locals.var_ua_i_dn9)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn10 + ((-locals.var_ua_i_dn10) + (0.5 * ((-(-locals.var_ua_i_dn10)) + (((((-(-locals.var_ua_i_dn10)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn10)))) - ((4.0 * (-locals.var_ua_i_dn10)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn11 + ((-locals.var_ua_i_dn11) + (0.5 * ((-(-locals.var_ua_i_dn11)) + (((((-(-locals.var_ua_i_dn11)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn11)))) - ((4.0 * (-locals.var_ua_i_dn11)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn13 + ((-locals.var_ua_i_dn13) + (0.5 * ((-(-locals.var_ua_i_dn13)) + (((((-(-locals.var_ua_i_dn13)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn13)))) - ((4.0 * (-locals.var_ua_i_dn13)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn14 + ((-locals.var_ua_i_dn14) + (0.5 * ((-(-locals.var_ua_i_dn14)) + (((((-(-locals.var_ua_i_dn14)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn14)))) - ((4.0 * (-locals.var_ua_i_dn14)) * 1e-6)) / (2.0 * assign14820_e22094)))))),)
    } else {
        (locals.var_ua_t, locals.var_ua_t_dn0, locals.var_ua_t_dn2, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11, locals.var_ua_t_dn13, locals.var_ua_t_dn14,)
    }
};
        locals.var_ua_t = assign14820_e22100;
        locals.var_ua_t_dn0 = assign14820_e22100_d_n0;
        locals.var_ua_t_dn2 = assign14820_e22100_d_n2;
        locals.var_ua_t_dn3 = assign14820_e22100_d_n3;
        locals.var_ua_t_dn4 = assign14820_e22100_d_n4;
        locals.var_ua_t_dn5 = assign14820_e22100_d_n5;
        locals.var_ua_t_dn6 = assign14820_e22100_d_n6;
        locals.var_ua_t_dn7 = assign14820_e22100_d_n7;
        locals.var_ua_t_dn8 = assign14820_e22100_d_n8;
        locals.var_ua_t_dn9 = assign14820_e22100_d_n9;
        locals.var_ua_t_dn10 = assign14820_e22100_d_n10;
        locals.var_ua_t_dn11 = assign14820_e22100_d_n11;
        locals.var_ua_t_dn13 = assign14820_e22100_d_n13;
        locals.var_ua_t_dn14 = assign14820_e22100_d_n14;
        locals.var_ua_t_rv = 0.0;

        let (assign14830_e22104, assign14830_e22104_d_n0, assign14830_e22104_d_n2, assign14830_e22104_d_n3, assign14830_e22104_d_n4, assign14830_e22104_d_n5, assign14830_e22104_d_n6, assign14830_e22104_d_n7, assign14830_e22104_d_n8, assign14830_e22104_d_n9, assign14830_e22104_d_n10, assign14830_e22104_d_n11, assign14830_e22104_d_n13, assign14830_e22104_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn13, locals.var_eu_i_dn14,)
    } else {
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn13, locals.var_eu_t_dn14,)
    }
};
        locals.var_eu_t = assign14830_e22104;
        locals.var_eu_t_dn0 = assign14830_e22104_d_n0;
        locals.var_eu_t_dn2 = assign14830_e22104_d_n2;
        locals.var_eu_t_dn3 = assign14830_e22104_d_n3;
        locals.var_eu_t_dn4 = assign14830_e22104_d_n4;
        locals.var_eu_t_dn5 = assign14830_e22104_d_n5;
        locals.var_eu_t_dn6 = assign14830_e22104_d_n6;
        locals.var_eu_t_dn7 = assign14830_e22104_d_n7;
        locals.var_eu_t_dn8 = assign14830_e22104_d_n8;
        locals.var_eu_t_dn9 = assign14830_e22104_d_n9;
        locals.var_eu_t_dn10 = assign14830_e22104_d_n10;
        locals.var_eu_t_dn11 = assign14830_e22104_d_n11;
        locals.var_eu_t_dn13 = assign14830_e22104_d_n13;
        locals.var_eu_t_dn14 = assign14830_e22104_d_n14;
        locals.var_eu_t_rv = 0.0;

        let assign14840_e22107: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard246 = assign14840_e22107;
        locals.var_guard246_rv = 0.0;

        let (assign14850_e22153, assign14850_e22153_d_n0, assign14850_e22153_d_n2, assign14850_e22153_d_n3, assign14850_e22153_d_n4, assign14850_e22153_d_n5, assign14850_e22153_d_n6, assign14850_e22153_d_n7, assign14850_e22153_d_n8, assign14850_e22153_d_n9, assign14850_e22153_d_n10, assign14850_e22153_d_n11, assign14850_e22153_d_n13, assign14850_e22153_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard246 != 0.0)) {
        let assign14850_e22113: f64 = (-locals.var_uar_i);
        let assign14850_e22117: f64 = (locals.var_ua1r_i * locals.var_deltemp);
        let assign14850_e22119: f64 = (-locals.var_uar_i);
        let assign14850_e22120: f64 = (assign14850_e22117 - assign14850_e22119);
        let assign14850_e22122: f64 = (assign14850_e22120 - 1e-6);
        let assign14850_e22125: f64 = (locals.var_ua1r_i * locals.var_deltemp);
        let assign14850_e22127: f64 = (-locals.var_uar_i);
        let assign14850_e22128: f64 = (assign14850_e22125 - assign14850_e22127);
        let assign14850_e22130: f64 = (assign14850_e22128 - 1e-6);
        let assign14850_e22133: f64 = (locals.var_ua1r_i * locals.var_deltemp);
        let assign14850_e22135: f64 = (-locals.var_uar_i);
        let assign14850_e22136: f64 = (assign14850_e22133 - assign14850_e22135);
        let assign14850_e22138: f64 = (assign14850_e22136 - 1e-6);
        let assign14850_e22139: f64 = (assign14850_e22130 * assign14850_e22138);
        let assign14850_e22142: f64 = (-locals.var_uar_i);
        let assign14850_e22143: f64 = (4.0 * assign14850_e22142);
        let assign14850_e22145: f64 = (assign14850_e22143 * 1e-6);
        let assign14850_e22146: f64 = (assign14850_e22139 - assign14850_e22145);
        let assign14850_e22147: f64 = (assign14850_e22146).sqrt();
        let assign14850_e22148: f64 = (assign14850_e22122 + assign14850_e22147);
        let assign14850_e22149: f64 = (0.5 * assign14850_e22148);
        let assign14850_e22150: f64 = (assign14850_e22113 + assign14850_e22149);
        let assign14850_e22151: f64 = (locals.var_uar_i + assign14850_e22150);
        (assign14850_e22151, (locals.var_uar_i_dn0 + ((-locals.var_uar_i_dn0) + (0.5 * ((-(-locals.var_uar_i_dn0)) + (((((-(-locals.var_uar_i_dn0)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn0)))) - ((4.0 * (-locals.var_uar_i_dn0)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn2 + ((-locals.var_uar_i_dn2) + (0.5 * ((-(-locals.var_uar_i_dn2)) + (((((-(-locals.var_uar_i_dn2)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn2)))) - ((4.0 * (-locals.var_uar_i_dn2)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn3 + ((-locals.var_uar_i_dn3) + (0.5 * ((-(-locals.var_uar_i_dn3)) + (((((-(-locals.var_uar_i_dn3)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn3)))) - ((4.0 * (-locals.var_uar_i_dn3)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn4 + ((-locals.var_uar_i_dn4) + (0.5 * (((locals.var_ua1r_i * locals.var_deltemp_dn4) - (-locals.var_uar_i_dn4)) + ((((((locals.var_ua1r_i * locals.var_deltemp_dn4) - (-locals.var_uar_i_dn4)) * assign14850_e22138) + (assign14850_e22130 * ((locals.var_ua1r_i * locals.var_deltemp_dn4) - (-locals.var_uar_i_dn4)))) - ((4.0 * (-locals.var_uar_i_dn4)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn5 + ((-locals.var_uar_i_dn5) + (0.5 * ((-(-locals.var_uar_i_dn5)) + (((((-(-locals.var_uar_i_dn5)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn5)))) - ((4.0 * (-locals.var_uar_i_dn5)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn6 + ((-locals.var_uar_i_dn6) + (0.5 * ((-(-locals.var_uar_i_dn6)) + (((((-(-locals.var_uar_i_dn6)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn6)))) - ((4.0 * (-locals.var_uar_i_dn6)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn7 + ((-locals.var_uar_i_dn7) + (0.5 * ((-(-locals.var_uar_i_dn7)) + (((((-(-locals.var_uar_i_dn7)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn7)))) - ((4.0 * (-locals.var_uar_i_dn7)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn8 + ((-locals.var_uar_i_dn8) + (0.5 * ((-(-locals.var_uar_i_dn8)) + (((((-(-locals.var_uar_i_dn8)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn8)))) - ((4.0 * (-locals.var_uar_i_dn8)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn9 + ((-locals.var_uar_i_dn9) + (0.5 * ((-(-locals.var_uar_i_dn9)) + (((((-(-locals.var_uar_i_dn9)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn9)))) - ((4.0 * (-locals.var_uar_i_dn9)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn10 + ((-locals.var_uar_i_dn10) + (0.5 * ((-(-locals.var_uar_i_dn10)) + (((((-(-locals.var_uar_i_dn10)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn10)))) - ((4.0 * (-locals.var_uar_i_dn10)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn11 + ((-locals.var_uar_i_dn11) + (0.5 * ((-(-locals.var_uar_i_dn11)) + (((((-(-locals.var_uar_i_dn11)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn11)))) - ((4.0 * (-locals.var_uar_i_dn11)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn13 + ((-locals.var_uar_i_dn13) + (0.5 * ((-(-locals.var_uar_i_dn13)) + (((((-(-locals.var_uar_i_dn13)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn13)))) - ((4.0 * (-locals.var_uar_i_dn13)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn14 + ((-locals.var_uar_i_dn14) + (0.5 * ((-(-locals.var_uar_i_dn14)) + (((((-(-locals.var_uar_i_dn14)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn14)))) - ((4.0 * (-locals.var_uar_i_dn14)) * 1e-6)) / (2.0 * assign14850_e22147)))))),)
    } else {
        (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn13, locals.var_uar_t_dn14,)
    }
};
        locals.var_uar_t = assign14850_e22153;
        locals.var_uar_t_dn0 = assign14850_e22153_d_n0;
        locals.var_uar_t_dn2 = assign14850_e22153_d_n2;
        locals.var_uar_t_dn3 = assign14850_e22153_d_n3;
        locals.var_uar_t_dn4 = assign14850_e22153_d_n4;
        locals.var_uar_t_dn5 = assign14850_e22153_d_n5;
        locals.var_uar_t_dn6 = assign14850_e22153_d_n6;
        locals.var_uar_t_dn7 = assign14850_e22153_d_n7;
        locals.var_uar_t_dn8 = assign14850_e22153_d_n8;
        locals.var_uar_t_dn9 = assign14850_e22153_d_n9;
        locals.var_uar_t_dn10 = assign14850_e22153_d_n10;
        locals.var_uar_t_dn11 = assign14850_e22153_d_n11;
        locals.var_uar_t_dn13 = assign14850_e22153_d_n13;
        locals.var_uar_t_dn14 = assign14850_e22153_d_n14;
        locals.var_uar_t_rv = 0.0;

        let (assign14860_e22162, assign14860_e22162_d_n0, assign14860_e22162_d_n2, assign14860_e22162_d_n3, assign14860_e22162_d_n4, assign14860_e22162_d_n5, assign14860_e22162_d_n6, assign14860_e22162_d_n7, assign14860_e22162_d_n8, assign14860_e22162_d_n9, assign14860_e22162_d_n10, assign14860_e22162_d_n11, assign14860_e22162_d_n13, assign14860_e22162_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14860_e22158: f64 = (locals.var_ud1_i * locals.var_trat_ln);
        let assign14860_e22159: f64 = (assign14860_e22158).exp();
        let assign14860_e22160: f64 = (locals.var_ud_i * assign14860_e22159);
        (assign14860_e22160, (locals.var_ud_i_dn0 * assign14860_e22159), (locals.var_ud_i_dn2 * assign14860_e22159), (locals.var_ud_i_dn3 * assign14860_e22159), ((locals.var_ud_i_dn4 * assign14860_e22159) + (locals.var_ud_i * (assign14860_e22159 * (locals.var_ud1_i * locals.var_trat_ln_dn4)))), (locals.var_ud_i_dn5 * assign14860_e22159), (locals.var_ud_i_dn6 * assign14860_e22159), (locals.var_ud_i_dn7 * assign14860_e22159), (locals.var_ud_i_dn8 * assign14860_e22159), (locals.var_ud_i_dn9 * assign14860_e22159), (locals.var_ud_i_dn10 * assign14860_e22159), (locals.var_ud_i_dn11 * assign14860_e22159), (locals.var_ud_i_dn13 * assign14860_e22159), (locals.var_ud_i_dn14 * assign14860_e22159),)
    } else {
        (locals.var_ud_t, locals.var_ud_t_dn0, locals.var_ud_t_dn2, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11, locals.var_ud_t_dn13, locals.var_ud_t_dn14,)
    }
};
        locals.var_ud_t = assign14860_e22162;
        locals.var_ud_t_dn0 = assign14860_e22162_d_n0;
        locals.var_ud_t_dn2 = assign14860_e22162_d_n2;
        locals.var_ud_t_dn3 = assign14860_e22162_d_n3;
        locals.var_ud_t_dn4 = assign14860_e22162_d_n4;
        locals.var_ud_t_dn5 = assign14860_e22162_d_n5;
        locals.var_ud_t_dn6 = assign14860_e22162_d_n6;
        locals.var_ud_t_dn7 = assign14860_e22162_d_n7;
        locals.var_ud_t_dn8 = assign14860_e22162_d_n8;
        locals.var_ud_t_dn9 = assign14860_e22162_d_n9;
        locals.var_ud_t_dn10 = assign14860_e22162_d_n10;
        locals.var_ud_t_dn11 = assign14860_e22162_d_n11;
        locals.var_ud_t_dn13 = assign14860_e22162_d_n13;
        locals.var_ud_t_dn14 = assign14860_e22162_d_n14;
        locals.var_ud_t_rv = 0.0;

        let assign14870_e22165: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard247 = assign14870_e22165;
        locals.var_guard247_rv = 0.0;

        let (assign14880_e22176, assign14880_e22176_d_n0, assign14880_e22176_d_n2, assign14880_e22176_d_n3, assign14880_e22176_d_n4, assign14880_e22176_d_n5, assign14880_e22176_d_n6, assign14880_e22176_d_n7, assign14880_e22176_d_n8, assign14880_e22176_d_n9, assign14880_e22176_d_n10, assign14880_e22176_d_n11, assign14880_e22176_d_n13, assign14880_e22176_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard247 != 0.0)) {
        let assign14880_e22172: f64 = (locals.var_ud1r_i * locals.var_trat_ln);
        let assign14880_e22173: f64 = (assign14880_e22172).exp();
        let assign14880_e22174: f64 = (locals.var_udr_i * assign14880_e22173);
        (assign14880_e22174, (locals.var_udr_i_dn0 * assign14880_e22173), (locals.var_udr_i_dn2 * assign14880_e22173), (locals.var_udr_i_dn3 * assign14880_e22173), ((locals.var_udr_i_dn4 * assign14880_e22173) + (locals.var_udr_i * (assign14880_e22173 * (locals.var_ud1r_i * locals.var_trat_ln_dn4)))), (locals.var_udr_i_dn5 * assign14880_e22173), (locals.var_udr_i_dn6 * assign14880_e22173), (locals.var_udr_i_dn7 * assign14880_e22173), (locals.var_udr_i_dn8 * assign14880_e22173), (locals.var_udr_i_dn9 * assign14880_e22173), (locals.var_udr_i_dn10 * assign14880_e22173), (locals.var_udr_i_dn11 * assign14880_e22173), (locals.var_udr_i_dn13 * assign14880_e22173), (locals.var_udr_i_dn14 * assign14880_e22173),)
    } else {
        (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn13, locals.var_udr_t_dn14,)
    }
};
        locals.var_udr_t = assign14880_e22176;
        locals.var_udr_t_dn0 = assign14880_e22176_d_n0;
        locals.var_udr_t_dn2 = assign14880_e22176_d_n2;
        locals.var_udr_t_dn3 = assign14880_e22176_d_n3;
        locals.var_udr_t_dn4 = assign14880_e22176_d_n4;
        locals.var_udr_t_dn5 = assign14880_e22176_d_n5;
        locals.var_udr_t_dn6 = assign14880_e22176_d_n6;
        locals.var_udr_t_dn7 = assign14880_e22176_d_n7;
        locals.var_udr_t_dn8 = assign14880_e22176_d_n8;
        locals.var_udr_t_dn9 = assign14880_e22176_d_n9;
        locals.var_udr_t_dn10 = assign14880_e22176_d_n10;
        locals.var_udr_t_dn11 = assign14880_e22176_d_n11;
        locals.var_udr_t_dn13 = assign14880_e22176_d_n13;
        locals.var_udr_t_dn14 = assign14880_e22176_d_n14;
        locals.var_udr_t_rv = 0.0;

        let (assign14890_e22185, assign14890_e22185_d_n4,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14890_e22181: f64 = (locals.var_ucste_i * locals.var_trat_ln);
        let assign14890_e22182: f64 = (assign14890_e22181).exp();
        let assign14890_e22183: f64 = (locals.var_ucs_i * assign14890_e22182);
        (assign14890_e22183, (locals.var_ucs_i * (assign14890_e22182 * (locals.var_ucste_i * locals.var_trat_ln_dn4))),)
    } else {
        (locals.var_ucs_t, locals.var_ucs_t_dn4,)
    }
};
        locals.var_ucs_t = assign14890_e22185;
        locals.var_ucs_t_dn4 = assign14890_e22185_d_n4;
        locals.var_ucs_t_rv = 0.0;

        let (assign14900_e22260, assign14900_e22260_d_n0, assign14900_e22260_d_n2, assign14900_e22260_d_n3, assign14900_e22260_d_n4, assign14900_e22260_d_n5, assign14900_e22260_d_n6, assign14900_e22260_d_n7, assign14900_e22260_d_n8, assign14900_e22260_d_n9, assign14900_e22260_d_n10, assign14900_e22260_d_n11, assign14900_e22260_d_n13, assign14900_e22260_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14900_e22190: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign14900_e22191: f64 = (1.0 + assign14900_e22190);
        let assign14900_e22193: f64 = (assign14900_e22191 - 1e-6);
        let assign14900_e22195: f64 = (-10000.0);
        let assign14900_e22197: f64 = (assign14900_e22195 * 0.001);
        let (assign14900_e22258, assign14900_e22258_d_n4,) = {
            if (!(assign14900_e22193 < assign14900_e22197)) {
                let assign14900_e22204: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22205: f64 = (1.0 + assign14900_e22204);
                let assign14900_e22207: f64 = (assign14900_e22205 - 1e-6);
                let assign14900_e22211: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22212: f64 = (1.0 + assign14900_e22211);
                let assign14900_e22214: f64 = (assign14900_e22212 - 1e-6);
                let assign14900_e22218: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22219: f64 = (1.0 + assign14900_e22218);
                let assign14900_e22221: f64 = (assign14900_e22219 - 1e-6);
                let assign14900_e22222: f64 = (assign14900_e22214 * assign14900_e22221);
                let assign14900_e22225: f64 = (4.0 * 0.001);
                let assign14900_e22227: f64 = (assign14900_e22225 * 0.001);
                let assign14900_e22228: f64 = (assign14900_e22222 + assign14900_e22227);
                let assign14900_e22229: f64 = (assign14900_e22228).sqrt();
                let assign14900_e22230: f64 = (assign14900_e22207 + assign14900_e22229);
                let assign14900_e22231: f64 = (0.5 * assign14900_e22230);
                (assign14900_e22231, (0.5 * ((locals.var_prt_i * locals.var_deltemp_dn4) + ((((locals.var_prt_i * locals.var_deltemp_dn4) * assign14900_e22221) + (assign14900_e22214 * (locals.var_prt_i * locals.var_deltemp_dn4))) / (2.0 * assign14900_e22229)))),)
            } else {
                let assign14900_e22235: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22236: f64 = (1.0 + assign14900_e22235);
                let assign14900_e22238: f64 = (assign14900_e22236 - 1e-6);
                let assign14900_e22240: f64 = (-10000.0);
                let assign14900_e22242: f64 = (assign14900_e22240 * 0.001);
                let (assign14900_e22257, assign14900_e22257_d_n4,) = {
                    if (assign14900_e22238 < assign14900_e22242) {
                        let assign14900_e22245: f64 = (-0.001);
                        let assign14900_e22247: f64 = (assign14900_e22245 * 0.001);
                        let assign14900_e22251: f64 = (locals.var_prt_i * locals.var_deltemp);
                        let assign14900_e22252: f64 = (1.0 + assign14900_e22251);
                        let assign14900_e22254: f64 = (assign14900_e22252 - 1e-6);
                        let assign14900_e22255: f64 = (assign14900_e22247 / assign14900_e22254);
                        (assign14900_e22255, (-((assign14900_e22247 * (locals.var_prt_i * locals.var_deltemp_dn4)) / (assign14900_e22254 * assign14900_e22254))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign14900_e22257, assign14900_e22257_d_n4,)
            }
        };
        (assign14900_e22258, 0.0, 0.0, 0.0, assign14900_e22258_d_n4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdstemp, locals.var_rdstemp_dn0, locals.var_rdstemp_dn2, locals.var_rdstemp_dn3, locals.var_rdstemp_dn4, locals.var_rdstemp_dn5, locals.var_rdstemp_dn6, locals.var_rdstemp_dn7, locals.var_rdstemp_dn8, locals.var_rdstemp_dn9, locals.var_rdstemp_dn10, locals.var_rdstemp_dn11, locals.var_rdstemp_dn13, locals.var_rdstemp_dn14,)
    }
};
        locals.var_rdstemp = assign14900_e22260;
        locals.var_rdstemp_dn0 = assign14900_e22260_d_n0;
        locals.var_rdstemp_dn2 = assign14900_e22260_d_n2;
        locals.var_rdstemp_dn3 = assign14900_e22260_d_n3;
        locals.var_rdstemp_dn4 = assign14900_e22260_d_n4;
        locals.var_rdstemp_dn5 = assign14900_e22260_d_n5;
        locals.var_rdstemp_dn6 = assign14900_e22260_d_n6;
        locals.var_rdstemp_dn7 = assign14900_e22260_d_n7;
        locals.var_rdstemp_dn8 = assign14900_e22260_d_n8;
        locals.var_rdstemp_dn9 = assign14900_e22260_d_n9;
        locals.var_rdstemp_dn10 = assign14900_e22260_d_n10;
        locals.var_rdstemp_dn11 = assign14900_e22260_d_n11;
        locals.var_rdstemp_dn13 = assign14900_e22260_d_n13;
        locals.var_rdstemp_dn14 = assign14900_e22260_d_n14;
        locals.var_rdstemp_rv = 0.0;

        let assign14910_e22263: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign14910_e22263;
        locals.var_guard248_rv = 0.0;

        let (assign14920_e22312, assign14920_e22312_d_n0, assign14920_e22312_d_n2, assign14920_e22312_d_n3, assign14920_e22312_d_n4, assign14920_e22312_d_n5, assign14920_e22312_d_n6, assign14920_e22312_d_n7, assign14920_e22312_d_n8, assign14920_e22312_d_n9, assign14920_e22312_d_n10, assign14920_e22312_d_n11, assign14920_e22312_d_n13, assign14920_e22312_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard248 != 0.0)) {
        let assign14920_e22269: f64 = (-locals.var_vsat_i);
        let assign14920_e22272: f64 = (-locals.var_at_i);
        let assign14920_e22274: f64 = (assign14920_e22272 * locals.var_deltemp);
        let assign14920_e22276: f64 = (-locals.var_vsat_i);
        let assign14920_e22277: f64 = (assign14920_e22274 - assign14920_e22276);
        let assign14920_e22279: f64 = (assign14920_e22277 - 1e-6);
        let assign14920_e22281: f64 = (-locals.var_at_i);
        let assign14920_e22283: f64 = (assign14920_e22281 * locals.var_deltemp);
        let assign14920_e22285: f64 = (-locals.var_vsat_i);
        let assign14920_e22286: f64 = (assign14920_e22283 - assign14920_e22285);
        let assign14920_e22288: f64 = (assign14920_e22286 - 1e-6);
        let assign14920_e22290: f64 = (-locals.var_at_i);
        let assign14920_e22292: f64 = (assign14920_e22290 * locals.var_deltemp);
        let assign14920_e22294: f64 = (-locals.var_vsat_i);
        let assign14920_e22295: f64 = (assign14920_e22292 - assign14920_e22294);
        let assign14920_e22297: f64 = (assign14920_e22295 - 1e-6);
        let assign14920_e22298: f64 = (assign14920_e22288 * assign14920_e22297);
        let assign14920_e22301: f64 = (-locals.var_vsat_i);
        let assign14920_e22302: f64 = (4.0 * assign14920_e22301);
        let assign14920_e22304: f64 = (assign14920_e22302 * 1e-6);
        let assign14920_e22305: f64 = (assign14920_e22298 - assign14920_e22304);
        let assign14920_e22306: f64 = (assign14920_e22305).sqrt();
        let assign14920_e22307: f64 = (assign14920_e22279 + assign14920_e22306);
        let assign14920_e22308: f64 = (0.5 * assign14920_e22307);
        let assign14920_e22309: f64 = (assign14920_e22269 + assign14920_e22308);
        let assign14920_e22310: f64 = (locals.var_vsat_i + assign14920_e22309);
        (assign14920_e22310, (locals.var_vsat_i_dn0 + ((-locals.var_vsat_i_dn0) + (0.5 * ((-(-locals.var_vsat_i_dn0)) + (((((-(-locals.var_vsat_i_dn0)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn0)))) - ((4.0 * (-locals.var_vsat_i_dn0)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn2 + ((-locals.var_vsat_i_dn2) + (0.5 * ((-(-locals.var_vsat_i_dn2)) + (((((-(-locals.var_vsat_i_dn2)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn2)))) - ((4.0 * (-locals.var_vsat_i_dn2)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn3 + ((-locals.var_vsat_i_dn3) + (0.5 * ((-(-locals.var_vsat_i_dn3)) + (((((-(-locals.var_vsat_i_dn3)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn3)))) - ((4.0 * (-locals.var_vsat_i_dn3)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn4 + ((-locals.var_vsat_i_dn4) + (0.5 * (((assign14920_e22272 * locals.var_deltemp_dn4) - (-locals.var_vsat_i_dn4)) + ((((((assign14920_e22281 * locals.var_deltemp_dn4) - (-locals.var_vsat_i_dn4)) * assign14920_e22297) + (assign14920_e22288 * ((assign14920_e22290 * locals.var_deltemp_dn4) - (-locals.var_vsat_i_dn4)))) - ((4.0 * (-locals.var_vsat_i_dn4)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn5 + ((-locals.var_vsat_i_dn5) + (0.5 * ((-(-locals.var_vsat_i_dn5)) + (((((-(-locals.var_vsat_i_dn5)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn5)))) - ((4.0 * (-locals.var_vsat_i_dn5)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn6 + ((-locals.var_vsat_i_dn6) + (0.5 * ((-(-locals.var_vsat_i_dn6)) + (((((-(-locals.var_vsat_i_dn6)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn6)))) - ((4.0 * (-locals.var_vsat_i_dn6)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn7 + ((-locals.var_vsat_i_dn7) + (0.5 * ((-(-locals.var_vsat_i_dn7)) + (((((-(-locals.var_vsat_i_dn7)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn7)))) - ((4.0 * (-locals.var_vsat_i_dn7)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn8 + ((-locals.var_vsat_i_dn8) + (0.5 * ((-(-locals.var_vsat_i_dn8)) + (((((-(-locals.var_vsat_i_dn8)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn8)))) - ((4.0 * (-locals.var_vsat_i_dn8)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn9 + ((-locals.var_vsat_i_dn9) + (0.5 * ((-(-locals.var_vsat_i_dn9)) + (((((-(-locals.var_vsat_i_dn9)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn9)))) - ((4.0 * (-locals.var_vsat_i_dn9)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn10 + ((-locals.var_vsat_i_dn10) + (0.5 * ((-(-locals.var_vsat_i_dn10)) + (((((-(-locals.var_vsat_i_dn10)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn10)))) - ((4.0 * (-locals.var_vsat_i_dn10)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn11 + ((-locals.var_vsat_i_dn11) + (0.5 * ((-(-locals.var_vsat_i_dn11)) + (((((-(-locals.var_vsat_i_dn11)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn11)))) - ((4.0 * (-locals.var_vsat_i_dn11)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn13 + ((-locals.var_vsat_i_dn13) + (0.5 * ((-(-locals.var_vsat_i_dn13)) + (((((-(-locals.var_vsat_i_dn13)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn13)))) - ((4.0 * (-locals.var_vsat_i_dn13)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn14 + ((-locals.var_vsat_i_dn14) + (0.5 * ((-(-locals.var_vsat_i_dn14)) + (((((-(-locals.var_vsat_i_dn14)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn14)))) - ((4.0 * (-locals.var_vsat_i_dn14)) * 1e-6)) / (2.0 * assign14920_e22306)))))),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign14920_e22312;
        locals.var_vsat_t_dn0 = assign14920_e22312_d_n0;
        locals.var_vsat_t_dn2 = assign14920_e22312_d_n2;
        locals.var_vsat_t_dn3 = assign14920_e22312_d_n3;
        locals.var_vsat_t_dn4 = assign14920_e22312_d_n4;
        locals.var_vsat_t_dn5 = assign14920_e22312_d_n5;
        locals.var_vsat_t_dn6 = assign14920_e22312_d_n6;
        locals.var_vsat_t_dn7 = assign14920_e22312_d_n7;
        locals.var_vsat_t_dn8 = assign14920_e22312_d_n8;
        locals.var_vsat_t_dn9 = assign14920_e22312_d_n9;
        locals.var_vsat_t_dn10 = assign14920_e22312_d_n10;
        locals.var_vsat_t_dn11 = assign14920_e22312_d_n11;
        locals.var_vsat_t_dn13 = assign14920_e22312_d_n13;
        locals.var_vsat_t_dn14 = assign14920_e22312_d_n14;
        locals.var_vsat_t_rv = 0.0;

        let (assign14930_e22398, assign14930_e22398_d_n0, assign14930_e22398_d_n2, assign14930_e22398_d_n3, assign14930_e22398_d_n4, assign14930_e22398_d_n5, assign14930_e22398_d_n6, assign14930_e22398_d_n7, assign14930_e22398_d_n8, assign14930_e22398_d_n9, assign14930_e22398_d_n10, assign14930_e22398_d_n11, assign14930_e22398_d_n13, assign14930_e22398_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard248 == 0.0)) {
        let assign14930_e22320: f64 = (-locals.var_at_i);
        let assign14930_e22322: f64 = (assign14930_e22320 * locals.var_deltemp);
        let assign14930_e22323: f64 = (1.0 + assign14930_e22322);
        let assign14930_e22325: f64 = (assign14930_e22323 - 1e-6);
        let assign14930_e22327: f64 = (-10000.0);
        let assign14930_e22329: f64 = (assign14930_e22327 * 0.001);
        let (assign14930_e22395, assign14930_e22395_d_n4,) = {
            if (!(assign14930_e22325 < assign14930_e22329)) {
                let assign14930_e22335: f64 = (-locals.var_at_i);
                let assign14930_e22337: f64 = (assign14930_e22335 * locals.var_deltemp);
                let assign14930_e22338: f64 = (1.0 + assign14930_e22337);
                let assign14930_e22340: f64 = (assign14930_e22338 - 1e-6);
                let assign14930_e22343: f64 = (-locals.var_at_i);
                let assign14930_e22345: f64 = (assign14930_e22343 * locals.var_deltemp);
                let assign14930_e22346: f64 = (1.0 + assign14930_e22345);
                let assign14930_e22348: f64 = (assign14930_e22346 - 1e-6);
                let assign14930_e22351: f64 = (-locals.var_at_i);
                let assign14930_e22353: f64 = (assign14930_e22351 * locals.var_deltemp);
                let assign14930_e22354: f64 = (1.0 + assign14930_e22353);
                let assign14930_e22356: f64 = (assign14930_e22354 - 1e-6);
                let assign14930_e22357: f64 = (assign14930_e22348 * assign14930_e22356);
                let assign14930_e22360: f64 = (4.0 * 0.001);
                let assign14930_e22362: f64 = (assign14930_e22360 * 0.001);
                let assign14930_e22363: f64 = (assign14930_e22357 + assign14930_e22362);
                let assign14930_e22364: f64 = (assign14930_e22363).sqrt();
                let assign14930_e22365: f64 = (assign14930_e22340 + assign14930_e22364);
                let assign14930_e22366: f64 = (0.5 * assign14930_e22365);
                (assign14930_e22366, (0.5 * ((assign14930_e22335 * locals.var_deltemp_dn4) + ((((assign14930_e22343 * locals.var_deltemp_dn4) * assign14930_e22356) + (assign14930_e22348 * (assign14930_e22351 * locals.var_deltemp_dn4))) / (2.0 * assign14930_e22364)))),)
            } else {
                let assign14930_e22369: f64 = (-locals.var_at_i);
                let assign14930_e22371: f64 = (assign14930_e22369 * locals.var_deltemp);
                let assign14930_e22372: f64 = (1.0 + assign14930_e22371);
                let assign14930_e22374: f64 = (assign14930_e22372 - 1e-6);
                let assign14930_e22376: f64 = (-10000.0);
                let assign14930_e22378: f64 = (assign14930_e22376 * 0.001);
                let (assign14930_e22394, assign14930_e22394_d_n4,) = {
                    if (assign14930_e22374 < assign14930_e22378) {
                        let assign14930_e22381: f64 = (-0.001);
                        let assign14930_e22383: f64 = (assign14930_e22381 * 0.001);
                        let assign14930_e22386: f64 = (-locals.var_at_i);
                        let assign14930_e22388: f64 = (assign14930_e22386 * locals.var_deltemp);
                        let assign14930_e22389: f64 = (1.0 + assign14930_e22388);
                        let assign14930_e22391: f64 = (assign14930_e22389 - 1e-6);
                        let assign14930_e22392: f64 = (assign14930_e22383 / assign14930_e22391);
                        (assign14930_e22392, (-((assign14930_e22383 * (assign14930_e22386 * locals.var_deltemp_dn4)) / (assign14930_e22391 * assign14930_e22391))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign14930_e22394, assign14930_e22394_d_n4,)
            }
        };
        let assign14930_e22396: f64 = (locals.var_vsat_i * assign14930_e22395);
        (assign14930_e22396, (locals.var_vsat_i_dn0 * assign14930_e22395), (locals.var_vsat_i_dn2 * assign14930_e22395), (locals.var_vsat_i_dn3 * assign14930_e22395), ((locals.var_vsat_i_dn4 * assign14930_e22395) + (locals.var_vsat_i * assign14930_e22395_d_n4)), (locals.var_vsat_i_dn5 * assign14930_e22395), (locals.var_vsat_i_dn6 * assign14930_e22395), (locals.var_vsat_i_dn7 * assign14930_e22395), (locals.var_vsat_i_dn8 * assign14930_e22395), (locals.var_vsat_i_dn9 * assign14930_e22395), (locals.var_vsat_i_dn10 * assign14930_e22395), (locals.var_vsat_i_dn11 * assign14930_e22395), (locals.var_vsat_i_dn13 * assign14930_e22395), (locals.var_vsat_i_dn14 * assign14930_e22395),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign14930_e22398;
        locals.var_vsat_t_dn0 = assign14930_e22398_d_n0;
        locals.var_vsat_t_dn2 = assign14930_e22398_d_n2;
        locals.var_vsat_t_dn3 = assign14930_e22398_d_n3;
        locals.var_vsat_t_dn4 = assign14930_e22398_d_n4;
        locals.var_vsat_t_dn5 = assign14930_e22398_d_n5;
        locals.var_vsat_t_dn6 = assign14930_e22398_d_n6;
        locals.var_vsat_t_dn7 = assign14930_e22398_d_n7;
        locals.var_vsat_t_dn8 = assign14930_e22398_d_n8;
        locals.var_vsat_t_dn9 = assign14930_e22398_d_n9;
        locals.var_vsat_t_dn10 = assign14930_e22398_d_n10;
        locals.var_vsat_t_dn11 = assign14930_e22398_d_n11;
        locals.var_vsat_t_dn13 = assign14930_e22398_d_n13;
        locals.var_vsat_t_dn14 = assign14930_e22398_d_n14;
        locals.var_vsat_t_rv = 0.0;

        let assign14940_e22401: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign14940_e22401;
        locals.var_guard249_rv = 0.0;

        let assign14950_e22404: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign14950_e22404;
        locals.var_guard250_rv = 0.0;

        let (assign14960_e22455, assign14960_e22455_d_n4,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard249 != 0.0)) && (locals.var_guard250 != 0.0)) {
        let assign14960_e22412: f64 = (-locals.var_vsatr_i);
        let assign14960_e22415: f64 = (-locals.var_atr_i);
        let assign14960_e22417: f64 = (assign14960_e22415 * locals.var_deltemp);
        let assign14960_e22419: f64 = (-locals.var_vsatr_i);
        let assign14960_e22420: f64 = (assign14960_e22417 - assign14960_e22419);
        let assign14960_e22422: f64 = (assign14960_e22420 - 1e-6);
        let assign14960_e22424: f64 = (-locals.var_atr_i);
        let assign14960_e22426: f64 = (assign14960_e22424 * locals.var_deltemp);
        let assign14960_e22428: f64 = (-locals.var_vsatr_i);
        let assign14960_e22429: f64 = (assign14960_e22426 - assign14960_e22428);
        let assign14960_e22431: f64 = (assign14960_e22429 - 1e-6);
        let assign14960_e22433: f64 = (-locals.var_atr_i);
        let assign14960_e22435: f64 = (assign14960_e22433 * locals.var_deltemp);
        let assign14960_e22437: f64 = (-locals.var_vsatr_i);
        let assign14960_e22438: f64 = (assign14960_e22435 - assign14960_e22437);
        let assign14960_e22440: f64 = (assign14960_e22438 - 1e-6);
        let assign14960_e22441: f64 = (assign14960_e22431 * assign14960_e22440);
        let assign14960_e22444: f64 = (-locals.var_vsatr_i);
        let assign14960_e22445: f64 = (4.0 * assign14960_e22444);
        let assign14960_e22447: f64 = (assign14960_e22445 * 1e-6);
        let assign14960_e22448: f64 = (assign14960_e22441 - assign14960_e22447);
        let assign14960_e22449: f64 = (assign14960_e22448).sqrt();
        let assign14960_e22450: f64 = (assign14960_e22422 + assign14960_e22449);
        let assign14960_e22451: f64 = (0.5 * assign14960_e22450);
        let assign14960_e22452: f64 = (assign14960_e22412 + assign14960_e22451);
        let assign14960_e22453: f64 = (locals.var_vsatr_i + assign14960_e22452);
        (assign14960_e22453, (0.5 * ((assign14960_e22415 * locals.var_deltemp_dn4) + ((((assign14960_e22424 * locals.var_deltemp_dn4) * assign14960_e22440) + (assign14960_e22431 * (assign14960_e22433 * locals.var_deltemp_dn4))) / (2.0 * assign14960_e22449)))),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign14960_e22455;
        locals.var_vsatr_t_dn4 = assign14960_e22455_d_n4;
        locals.var_vsatr_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_44(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14970_e22543, assign14970_e22543_d_n4,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard249 != 0.0)) && (locals.var_guard250 == 0.0)) {
        let assign14970_e22465: f64 = (-locals.var_atr_i);
        let assign14970_e22467: f64 = (assign14970_e22465 * locals.var_deltemp);
        let assign14970_e22468: f64 = (1.0 + assign14970_e22467);
        let assign14970_e22470: f64 = (assign14970_e22468 - 1e-6);
        let assign14970_e22472: f64 = (-10000.0);
        let assign14970_e22474: f64 = (assign14970_e22472 * 0.001);
        let (assign14970_e22540, assign14970_e22540_d_n4,) = {
            if (!(assign14970_e22470 < assign14970_e22474)) {
                let assign14970_e22480: f64 = (-locals.var_atr_i);
                let assign14970_e22482: f64 = (assign14970_e22480 * locals.var_deltemp);
                let assign14970_e22483: f64 = (1.0 + assign14970_e22482);
                let assign14970_e22485: f64 = (assign14970_e22483 - 1e-6);
                let assign14970_e22488: f64 = (-locals.var_atr_i);
                let assign14970_e22490: f64 = (assign14970_e22488 * locals.var_deltemp);
                let assign14970_e22491: f64 = (1.0 + assign14970_e22490);
                let assign14970_e22493: f64 = (assign14970_e22491 - 1e-6);
                let assign14970_e22496: f64 = (-locals.var_atr_i);
                let assign14970_e22498: f64 = (assign14970_e22496 * locals.var_deltemp);
                let assign14970_e22499: f64 = (1.0 + assign14970_e22498);
                let assign14970_e22501: f64 = (assign14970_e22499 - 1e-6);
                let assign14970_e22502: f64 = (assign14970_e22493 * assign14970_e22501);
                let assign14970_e22505: f64 = (4.0 * 0.001);
                let assign14970_e22507: f64 = (assign14970_e22505 * 0.001);
                let assign14970_e22508: f64 = (assign14970_e22502 + assign14970_e22507);
                let assign14970_e22509: f64 = (assign14970_e22508).sqrt();
                let assign14970_e22510: f64 = (assign14970_e22485 + assign14970_e22509);
                let assign14970_e22511: f64 = (0.5 * assign14970_e22510);
                (assign14970_e22511, (0.5 * ((assign14970_e22480 * locals.var_deltemp_dn4) + ((((assign14970_e22488 * locals.var_deltemp_dn4) * assign14970_e22501) + (assign14970_e22493 * (assign14970_e22496 * locals.var_deltemp_dn4))) / (2.0 * assign14970_e22509)))),)
            } else {
                let assign14970_e22514: f64 = (-locals.var_atr_i);
                let assign14970_e22516: f64 = (assign14970_e22514 * locals.var_deltemp);
                let assign14970_e22517: f64 = (1.0 + assign14970_e22516);
                let assign14970_e22519: f64 = (assign14970_e22517 - 1e-6);
                let assign14970_e22521: f64 = (-10000.0);
                let assign14970_e22523: f64 = (assign14970_e22521 * 0.001);
                let (assign14970_e22539, assign14970_e22539_d_n4,) = {
                    if (assign14970_e22519 < assign14970_e22523) {
                        let assign14970_e22526: f64 = (-0.001);
                        let assign14970_e22528: f64 = (assign14970_e22526 * 0.001);
                        let assign14970_e22531: f64 = (-locals.var_atr_i);
                        let assign14970_e22533: f64 = (assign14970_e22531 * locals.var_deltemp);
                        let assign14970_e22534: f64 = (1.0 + assign14970_e22533);
                        let assign14970_e22536: f64 = (assign14970_e22534 - 1e-6);
                        let assign14970_e22537: f64 = (assign14970_e22528 / assign14970_e22536);
                        (assign14970_e22537, (-((assign14970_e22528 * (assign14970_e22531 * locals.var_deltemp_dn4)) / (assign14970_e22536 * assign14970_e22536))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign14970_e22539, assign14970_e22539_d_n4,)
            }
        };
        let assign14970_e22541: f64 = (locals.var_vsatr_i * assign14970_e22540);
        (assign14970_e22541, (locals.var_vsatr_i * assign14970_e22540_d_n4),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign14970_e22543;
        locals.var_vsatr_t_dn4 = assign14970_e22543_d_n4;
        locals.var_vsatr_t_rv = 0.0;

        let assign14980_e22546: f64 = if locals.var_vsatr_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign14980_e22546;
        locals.var_guard251_rv = 0.0;

        let (assign14990_e22554, assign14990_e22554_d_n4,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard249 != 0.0)) && (locals.var_guard251 != 0.0)) {
        (1000.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign14990_e22554;
        locals.var_vsatr_t_dn4 = assign14990_e22554_d_n4;
        locals.var_vsatr_t_rv = 0.0;

        let assign15000_e22557: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign15000_e22557;
        locals.var_guard252_rv = 0.0;

        let (assign15010_e22568, assign15010_e22568_d_n0, assign15010_e22568_d_n2, assign15010_e22568_d_n3, assign15010_e22568_d_n4, assign15010_e22568_d_n5, assign15010_e22568_d_n6, assign15010_e22568_d_n7, assign15010_e22568_d_n8, assign15010_e22568_d_n9, assign15010_e22568_d_n10, assign15010_e22568_d_n11, assign15010_e22568_d_n13, assign15010_e22568_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15010_e22564: f64 = (locals.var_utecv_i * locals.var_trat_ln);
        let assign15010_e22565: f64 = (assign15010_e22564).exp();
        let assign15010_e22566: f64 = (locals.var_u0cv_i * assign15010_e22565);
        (assign15010_e22566, (locals.var_u0cv_i_dn0 * assign15010_e22565), (locals.var_u0cv_i_dn2 * assign15010_e22565), (locals.var_u0cv_i_dn3 * assign15010_e22565), ((locals.var_u0cv_i_dn4 * assign15010_e22565) + (locals.var_u0cv_i * (assign15010_e22565 * (locals.var_utecv_i * locals.var_trat_ln_dn4)))), (locals.var_u0cv_i_dn5 * assign15010_e22565), (locals.var_u0cv_i_dn6 * assign15010_e22565), (locals.var_u0cv_i_dn7 * assign15010_e22565), (locals.var_u0cv_i_dn8 * assign15010_e22565), (locals.var_u0cv_i_dn9 * assign15010_e22565), (locals.var_u0cv_i_dn10 * assign15010_e22565), (locals.var_u0cv_i_dn11 * assign15010_e22565), (locals.var_u0cv_i_dn13 * assign15010_e22565), (locals.var_u0cv_i_dn14 * assign15010_e22565),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15010_e22568;
        locals.var_t1_dn0 = assign15010_e22568_d_n0;
        locals.var_t1_dn2 = assign15010_e22568_d_n2;
        locals.var_t1_dn3 = assign15010_e22568_d_n3;
        locals.var_t1_dn4 = assign15010_e22568_d_n4;
        locals.var_t1_dn5 = assign15010_e22568_d_n5;
        locals.var_t1_dn6 = assign15010_e22568_d_n6;
        locals.var_t1_dn7 = assign15010_e22568_d_n7;
        locals.var_t1_dn8 = assign15010_e22568_d_n8;
        locals.var_t1_dn9 = assign15010_e22568_d_n9;
        locals.var_t1_dn10 = assign15010_e22568_d_n10;
        locals.var_t1_dn11 = assign15010_e22568_d_n11;
        locals.var_t1_dn13 = assign15010_e22568_d_n13;
        locals.var_t1_dn14 = assign15010_e22568_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15020_e22624, assign15020_e22624_d_n0, assign15020_e22624_d_n2, assign15020_e22624_d_n3, assign15020_e22624_d_n4, assign15020_e22624_d_n5, assign15020_e22624_d_n6, assign15020_e22624_d_n7, assign15020_e22624_d_n8, assign15020_e22624_d_n9, assign15020_e22624_d_n10, assign15020_e22624_d_n11, assign15020_e22624_d_n13, assign15020_e22624_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15020_e22574: f64 = (-0.9);
        let assign15020_e22576: f64 = (assign15020_e22574 * locals.var_t1);
        let assign15020_e22580: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15020_e22582: f64 = (-0.9);
        let assign15020_e22584: f64 = (assign15020_e22582 * locals.var_t1);
        let assign15020_e22585: f64 = (assign15020_e22580 - assign15020_e22584);
        let assign15020_e22587: f64 = (assign15020_e22585 - 0.0001);
        let assign15020_e22590: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15020_e22592: f64 = (-0.9);
        let assign15020_e22594: f64 = (assign15020_e22592 * locals.var_t1);
        let assign15020_e22595: f64 = (assign15020_e22590 - assign15020_e22594);
        let assign15020_e22597: f64 = (assign15020_e22595 - 0.0001);
        let assign15020_e22600: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15020_e22602: f64 = (-0.9);
        let assign15020_e22604: f64 = (assign15020_e22602 * locals.var_t1);
        let assign15020_e22605: f64 = (assign15020_e22600 - assign15020_e22604);
        let assign15020_e22607: f64 = (assign15020_e22605 - 0.0001);
        let assign15020_e22608: f64 = (assign15020_e22597 * assign15020_e22607);
        let assign15020_e22611: f64 = (-0.9);
        let assign15020_e22613: f64 = (assign15020_e22611 * locals.var_t1);
        let assign15020_e22614: f64 = (4.0 * assign15020_e22613);
        let assign15020_e22616: f64 = (assign15020_e22614 * 0.0001);
        let assign15020_e22617: f64 = (assign15020_e22608 - assign15020_e22616);
        let assign15020_e22618: f64 = (assign15020_e22617).sqrt();
        let assign15020_e22619: f64 = (assign15020_e22587 + assign15020_e22618);
        let assign15020_e22620: f64 = (0.5 * assign15020_e22619);
        let assign15020_e22621: f64 = (assign15020_e22576 + assign15020_e22620);
        let assign15020_e22622: f64 = (locals.var_t1 + assign15020_e22621);
        (assign15020_e22622, (locals.var_t1_dn0 + ((assign15020_e22574 * locals.var_t1_dn0) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn0)) + (((((-(assign15020_e22592 * locals.var_t1_dn0)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn0)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn2 + ((assign15020_e22574 * locals.var_t1_dn2) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn2)) + (((((-(assign15020_e22592 * locals.var_t1_dn2)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn2)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn3 + ((assign15020_e22574 * locals.var_t1_dn3) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn3)) + (((((-(assign15020_e22592 * locals.var_t1_dn3)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn3)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn4 + ((assign15020_e22574 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15020_e22582 * locals.var_t1_dn4)) + ((((((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15020_e22592 * locals.var_t1_dn4)) * assign15020_e22607) + (assign15020_e22597 * ((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15020_e22602 * locals.var_t1_dn4)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn5 + ((assign15020_e22574 * locals.var_t1_dn5) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn5)) + (((((-(assign15020_e22592 * locals.var_t1_dn5)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn5)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn6 + ((assign15020_e22574 * locals.var_t1_dn6) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn6)) + (((((-(assign15020_e22592 * locals.var_t1_dn6)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn6)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn7 + ((assign15020_e22574 * locals.var_t1_dn7) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn7)) + (((((-(assign15020_e22592 * locals.var_t1_dn7)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn7)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn8 + ((assign15020_e22574 * locals.var_t1_dn8) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn8)) + (((((-(assign15020_e22592 * locals.var_t1_dn8)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn8)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn9 + ((assign15020_e22574 * locals.var_t1_dn9) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn9)) + (((((-(assign15020_e22592 * locals.var_t1_dn9)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn9)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn10 + ((assign15020_e22574 * locals.var_t1_dn10) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn10)) + (((((-(assign15020_e22592 * locals.var_t1_dn10)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn10)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn11 + ((assign15020_e22574 * locals.var_t1_dn11) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn11)) + (((((-(assign15020_e22592 * locals.var_t1_dn11)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn11)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn13 + ((assign15020_e22574 * locals.var_t1_dn13) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn13)) + (((((-(assign15020_e22592 * locals.var_t1_dn13)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn13)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn14 + ((assign15020_e22574 * locals.var_t1_dn14) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn14)) + (((((-(assign15020_e22592 * locals.var_t1_dn14)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn14)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15020_e22618)))))),)
    } else {
        (locals.var_u0_cv, locals.var_u0_cv_dn0, locals.var_u0_cv_dn2, locals.var_u0_cv_dn3, locals.var_u0_cv_dn4, locals.var_u0_cv_dn5, locals.var_u0_cv_dn6, locals.var_u0_cv_dn7, locals.var_u0_cv_dn8, locals.var_u0_cv_dn9, locals.var_u0_cv_dn10, locals.var_u0_cv_dn11, locals.var_u0_cv_dn13, locals.var_u0_cv_dn14,)
    }
};
        locals.var_u0_cv = assign15020_e22624;
        locals.var_u0_cv_dn0 = assign15020_e22624_d_n0;
        locals.var_u0_cv_dn2 = assign15020_e22624_d_n2;
        locals.var_u0_cv_dn3 = assign15020_e22624_d_n3;
        locals.var_u0_cv_dn4 = assign15020_e22624_d_n4;
        locals.var_u0_cv_dn5 = assign15020_e22624_d_n5;
        locals.var_u0_cv_dn6 = assign15020_e22624_d_n6;
        locals.var_u0_cv_dn7 = assign15020_e22624_d_n7;
        locals.var_u0_cv_dn8 = assign15020_e22624_d_n8;
        locals.var_u0_cv_dn9 = assign15020_e22624_d_n9;
        locals.var_u0_cv_dn10 = assign15020_e22624_d_n10;
        locals.var_u0_cv_dn11 = assign15020_e22624_d_n11;
        locals.var_u0_cv_dn13 = assign15020_e22624_d_n13;
        locals.var_u0_cv_dn14 = assign15020_e22624_d_n14;
        locals.var_u0_cv_rv = 0.0;

        let (assign15030_e22670, assign15030_e22670_d_n0, assign15030_e22670_d_n2, assign15030_e22670_d_n3, assign15030_e22670_d_n4, assign15030_e22670_d_n5, assign15030_e22670_d_n6, assign15030_e22670_d_n7, assign15030_e22670_d_n8, assign15030_e22670_d_n9, assign15030_e22670_d_n10, assign15030_e22670_d_n11, assign15030_e22670_d_n13, assign15030_e22670_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15030_e22630: f64 = (-locals.var_uacv_i);
        let assign15030_e22634: f64 = (locals.var_ua1cv_i * locals.var_deltemp);
        let assign15030_e22636: f64 = (-locals.var_uacv_i);
        let assign15030_e22637: f64 = (assign15030_e22634 - assign15030_e22636);
        let assign15030_e22639: f64 = (assign15030_e22637 - 1e-6);
        let assign15030_e22642: f64 = (locals.var_ua1cv_i * locals.var_deltemp);
        let assign15030_e22644: f64 = (-locals.var_uacv_i);
        let assign15030_e22645: f64 = (assign15030_e22642 - assign15030_e22644);
        let assign15030_e22647: f64 = (assign15030_e22645 - 1e-6);
        let assign15030_e22650: f64 = (locals.var_ua1cv_i * locals.var_deltemp);
        let assign15030_e22652: f64 = (-locals.var_uacv_i);
        let assign15030_e22653: f64 = (assign15030_e22650 - assign15030_e22652);
        let assign15030_e22655: f64 = (assign15030_e22653 - 1e-6);
        let assign15030_e22656: f64 = (assign15030_e22647 * assign15030_e22655);
        let assign15030_e22659: f64 = (-locals.var_uacv_i);
        let assign15030_e22660: f64 = (4.0 * assign15030_e22659);
        let assign15030_e22662: f64 = (assign15030_e22660 * 1e-6);
        let assign15030_e22663: f64 = (assign15030_e22656 - assign15030_e22662);
        let assign15030_e22664: f64 = (assign15030_e22663).sqrt();
        let assign15030_e22665: f64 = (assign15030_e22639 + assign15030_e22664);
        let assign15030_e22666: f64 = (0.5 * assign15030_e22665);
        let assign15030_e22667: f64 = (assign15030_e22630 + assign15030_e22666);
        let assign15030_e22668: f64 = (locals.var_uacv_i + assign15030_e22667);
        (assign15030_e22668, 0.0, 0.0, 0.0, (0.5 * ((locals.var_ua1cv_i * locals.var_deltemp_dn4) + ((((locals.var_ua1cv_i * locals.var_deltemp_dn4) * assign15030_e22655) + (assign15030_e22647 * (locals.var_ua1cv_i * locals.var_deltemp_dn4))) / (2.0 * assign15030_e22664)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uacv_t, locals.var_uacv_t_dn0, locals.var_uacv_t_dn2, locals.var_uacv_t_dn3, locals.var_uacv_t_dn4, locals.var_uacv_t_dn5, locals.var_uacv_t_dn6, locals.var_uacv_t_dn7, locals.var_uacv_t_dn8, locals.var_uacv_t_dn9, locals.var_uacv_t_dn10, locals.var_uacv_t_dn11, locals.var_uacv_t_dn13, locals.var_uacv_t_dn14,)
    }
};
        locals.var_uacv_t = assign15030_e22670;
        locals.var_uacv_t_dn0 = assign15030_e22670_d_n0;
        locals.var_uacv_t_dn2 = assign15030_e22670_d_n2;
        locals.var_uacv_t_dn3 = assign15030_e22670_d_n3;
        locals.var_uacv_t_dn4 = assign15030_e22670_d_n4;
        locals.var_uacv_t_dn5 = assign15030_e22670_d_n5;
        locals.var_uacv_t_dn6 = assign15030_e22670_d_n6;
        locals.var_uacv_t_dn7 = assign15030_e22670_d_n7;
        locals.var_uacv_t_dn8 = assign15030_e22670_d_n8;
        locals.var_uacv_t_dn9 = assign15030_e22670_d_n9;
        locals.var_uacv_t_dn10 = assign15030_e22670_d_n10;
        locals.var_uacv_t_dn11 = assign15030_e22670_d_n11;
        locals.var_uacv_t_dn13 = assign15030_e22670_d_n13;
        locals.var_uacv_t_dn14 = assign15030_e22670_d_n14;
        locals.var_uacv_t_rv = 0.0;

        let (assign15040_e22681, assign15040_e22681_d_n4,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15040_e22677: f64 = (locals.var_ud1cv_i * locals.var_trat_ln);
        let assign15040_e22678: f64 = (assign15040_e22677).exp();
        let assign15040_e22679: f64 = (locals.var_udcv_i * assign15040_e22678);
        (assign15040_e22679, (locals.var_udcv_i * (assign15040_e22678 * (locals.var_ud1cv_i * locals.var_trat_ln_dn4))),)
    } else {
        (locals.var_udcv_t, locals.var_udcv_t_dn4,)
    }
};
        locals.var_udcv_t = assign15040_e22681;
        locals.var_udcv_t_dn4 = assign15040_e22681_d_n4;
        locals.var_udcv_t_rv = 0.0;

        let assign15050_e22684: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign15050_e22684;
        locals.var_guard253_rv = 0.0;

        let (assign15060_e22733, assign15060_e22733_d_n0, assign15060_e22733_d_n2, assign15060_e22733_d_n3, assign15060_e22733_d_n4, assign15060_e22733_d_n5, assign15060_e22733_d_n6, assign15060_e22733_d_n7, assign15060_e22733_d_n8, assign15060_e22733_d_n9, assign15060_e22733_d_n10, assign15060_e22733_d_n11, assign15060_e22733_d_n13, assign15060_e22733_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard253 != 0.0)) {
        let assign15060_e22690: f64 = (-locals.var_vsat1_i);
        let assign15060_e22693: f64 = (-locals.var_at_i);
        let assign15060_e22695: f64 = (assign15060_e22693 * locals.var_deltemp);
        let assign15060_e22697: f64 = (-locals.var_vsat1_i);
        let assign15060_e22698: f64 = (assign15060_e22695 - assign15060_e22697);
        let assign15060_e22700: f64 = (assign15060_e22698 - 1e-6);
        let assign15060_e22702: f64 = (-locals.var_at_i);
        let assign15060_e22704: f64 = (assign15060_e22702 * locals.var_deltemp);
        let assign15060_e22706: f64 = (-locals.var_vsat1_i);
        let assign15060_e22707: f64 = (assign15060_e22704 - assign15060_e22706);
        let assign15060_e22709: f64 = (assign15060_e22707 - 1e-6);
        let assign15060_e22711: f64 = (-locals.var_at_i);
        let assign15060_e22713: f64 = (assign15060_e22711 * locals.var_deltemp);
        let assign15060_e22715: f64 = (-locals.var_vsat1_i);
        let assign15060_e22716: f64 = (assign15060_e22713 - assign15060_e22715);
        let assign15060_e22718: f64 = (assign15060_e22716 - 1e-6);
        let assign15060_e22719: f64 = (assign15060_e22709 * assign15060_e22718);
        let assign15060_e22722: f64 = (-locals.var_vsat1_i);
        let assign15060_e22723: f64 = (4.0 * assign15060_e22722);
        let assign15060_e22725: f64 = (assign15060_e22723 * 1e-6);
        let assign15060_e22726: f64 = (assign15060_e22719 - assign15060_e22725);
        let assign15060_e22727: f64 = (assign15060_e22726).sqrt();
        let assign15060_e22728: f64 = (assign15060_e22700 + assign15060_e22727);
        let assign15060_e22729: f64 = (0.5 * assign15060_e22728);
        let assign15060_e22730: f64 = (assign15060_e22690 + assign15060_e22729);
        let assign15060_e22731: f64 = (locals.var_vsat1_i + assign15060_e22730);
        (assign15060_e22731, (locals.var_vsat1_i_dn0 + ((-locals.var_vsat1_i_dn0) + (0.5 * ((-(-locals.var_vsat1_i_dn0)) + (((((-(-locals.var_vsat1_i_dn0)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn0)))) - ((4.0 * (-locals.var_vsat1_i_dn0)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn2 + ((-locals.var_vsat1_i_dn2) + (0.5 * ((-(-locals.var_vsat1_i_dn2)) + (((((-(-locals.var_vsat1_i_dn2)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn2)))) - ((4.0 * (-locals.var_vsat1_i_dn2)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn3 + ((-locals.var_vsat1_i_dn3) + (0.5 * ((-(-locals.var_vsat1_i_dn3)) + (((((-(-locals.var_vsat1_i_dn3)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn3)))) - ((4.0 * (-locals.var_vsat1_i_dn3)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn4 + ((-locals.var_vsat1_i_dn4) + (0.5 * (((assign15060_e22693 * locals.var_deltemp_dn4) - (-locals.var_vsat1_i_dn4)) + ((((((assign15060_e22702 * locals.var_deltemp_dn4) - (-locals.var_vsat1_i_dn4)) * assign15060_e22718) + (assign15060_e22709 * ((assign15060_e22711 * locals.var_deltemp_dn4) - (-locals.var_vsat1_i_dn4)))) - ((4.0 * (-locals.var_vsat1_i_dn4)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn5 + ((-locals.var_vsat1_i_dn5) + (0.5 * ((-(-locals.var_vsat1_i_dn5)) + (((((-(-locals.var_vsat1_i_dn5)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn5)))) - ((4.0 * (-locals.var_vsat1_i_dn5)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn6 + ((-locals.var_vsat1_i_dn6) + (0.5 * ((-(-locals.var_vsat1_i_dn6)) + (((((-(-locals.var_vsat1_i_dn6)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn6)))) - ((4.0 * (-locals.var_vsat1_i_dn6)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn7 + ((-locals.var_vsat1_i_dn7) + (0.5 * ((-(-locals.var_vsat1_i_dn7)) + (((((-(-locals.var_vsat1_i_dn7)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn7)))) - ((4.0 * (-locals.var_vsat1_i_dn7)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn8 + ((-locals.var_vsat1_i_dn8) + (0.5 * ((-(-locals.var_vsat1_i_dn8)) + (((((-(-locals.var_vsat1_i_dn8)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn8)))) - ((4.0 * (-locals.var_vsat1_i_dn8)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn9 + ((-locals.var_vsat1_i_dn9) + (0.5 * ((-(-locals.var_vsat1_i_dn9)) + (((((-(-locals.var_vsat1_i_dn9)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn9)))) - ((4.0 * (-locals.var_vsat1_i_dn9)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn10 + ((-locals.var_vsat1_i_dn10) + (0.5 * ((-(-locals.var_vsat1_i_dn10)) + (((((-(-locals.var_vsat1_i_dn10)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn10)))) - ((4.0 * (-locals.var_vsat1_i_dn10)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn11 + ((-locals.var_vsat1_i_dn11) + (0.5 * ((-(-locals.var_vsat1_i_dn11)) + (((((-(-locals.var_vsat1_i_dn11)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn11)))) - ((4.0 * (-locals.var_vsat1_i_dn11)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn13 + ((-locals.var_vsat1_i_dn13) + (0.5 * ((-(-locals.var_vsat1_i_dn13)) + (((((-(-locals.var_vsat1_i_dn13)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn13)))) - ((4.0 * (-locals.var_vsat1_i_dn13)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn14 + ((-locals.var_vsat1_i_dn14) + (0.5 * ((-(-locals.var_vsat1_i_dn14)) + (((((-(-locals.var_vsat1_i_dn14)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn14)))) - ((4.0 * (-locals.var_vsat1_i_dn14)) * 1e-6)) / (2.0 * assign15060_e22727)))))),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign15060_e22733;
        locals.var_vsat1_t_dn0 = assign15060_e22733_d_n0;
        locals.var_vsat1_t_dn2 = assign15060_e22733_d_n2;
        locals.var_vsat1_t_dn3 = assign15060_e22733_d_n3;
        locals.var_vsat1_t_dn4 = assign15060_e22733_d_n4;
        locals.var_vsat1_t_dn5 = assign15060_e22733_d_n5;
        locals.var_vsat1_t_dn6 = assign15060_e22733_d_n6;
        locals.var_vsat1_t_dn7 = assign15060_e22733_d_n7;
        locals.var_vsat1_t_dn8 = assign15060_e22733_d_n8;
        locals.var_vsat1_t_dn9 = assign15060_e22733_d_n9;
        locals.var_vsat1_t_dn10 = assign15060_e22733_d_n10;
        locals.var_vsat1_t_dn11 = assign15060_e22733_d_n11;
        locals.var_vsat1_t_dn13 = assign15060_e22733_d_n13;
        locals.var_vsat1_t_dn14 = assign15060_e22733_d_n14;
        locals.var_vsat1_t_rv = 0.0;

        let (assign15070_e22819, assign15070_e22819_d_n0, assign15070_e22819_d_n2, assign15070_e22819_d_n3, assign15070_e22819_d_n4, assign15070_e22819_d_n5, assign15070_e22819_d_n6, assign15070_e22819_d_n7, assign15070_e22819_d_n8, assign15070_e22819_d_n9, assign15070_e22819_d_n10, assign15070_e22819_d_n11, assign15070_e22819_d_n13, assign15070_e22819_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard253 == 0.0)) {
        let assign15070_e22741: f64 = (-locals.var_at_i);
        let assign15070_e22743: f64 = (assign15070_e22741 * locals.var_deltemp);
        let assign15070_e22744: f64 = (1.0 + assign15070_e22743);
        let assign15070_e22746: f64 = (assign15070_e22744 - 1e-6);
        let assign15070_e22748: f64 = (-10000.0);
        let assign15070_e22750: f64 = (assign15070_e22748 * 0.001);
        let (assign15070_e22816, assign15070_e22816_d_n4,) = {
            if (!(assign15070_e22746 < assign15070_e22750)) {
                let assign15070_e22756: f64 = (-locals.var_at_i);
                let assign15070_e22758: f64 = (assign15070_e22756 * locals.var_deltemp);
                let assign15070_e22759: f64 = (1.0 + assign15070_e22758);
                let assign15070_e22761: f64 = (assign15070_e22759 - 1e-6);
                let assign15070_e22764: f64 = (-locals.var_at_i);
                let assign15070_e22766: f64 = (assign15070_e22764 * locals.var_deltemp);
                let assign15070_e22767: f64 = (1.0 + assign15070_e22766);
                let assign15070_e22769: f64 = (assign15070_e22767 - 1e-6);
                let assign15070_e22772: f64 = (-locals.var_at_i);
                let assign15070_e22774: f64 = (assign15070_e22772 * locals.var_deltemp);
                let assign15070_e22775: f64 = (1.0 + assign15070_e22774);
                let assign15070_e22777: f64 = (assign15070_e22775 - 1e-6);
                let assign15070_e22778: f64 = (assign15070_e22769 * assign15070_e22777);
                let assign15070_e22781: f64 = (4.0 * 0.001);
                let assign15070_e22783: f64 = (assign15070_e22781 * 0.001);
                let assign15070_e22784: f64 = (assign15070_e22778 + assign15070_e22783);
                let assign15070_e22785: f64 = (assign15070_e22784).sqrt();
                let assign15070_e22786: f64 = (assign15070_e22761 + assign15070_e22785);
                let assign15070_e22787: f64 = (0.5 * assign15070_e22786);
                (assign15070_e22787, (0.5 * ((assign15070_e22756 * locals.var_deltemp_dn4) + ((((assign15070_e22764 * locals.var_deltemp_dn4) * assign15070_e22777) + (assign15070_e22769 * (assign15070_e22772 * locals.var_deltemp_dn4))) / (2.0 * assign15070_e22785)))),)
            } else {
                let assign15070_e22790: f64 = (-locals.var_at_i);
                let assign15070_e22792: f64 = (assign15070_e22790 * locals.var_deltemp);
                let assign15070_e22793: f64 = (1.0 + assign15070_e22792);
                let assign15070_e22795: f64 = (assign15070_e22793 - 1e-6);
                let assign15070_e22797: f64 = (-10000.0);
                let assign15070_e22799: f64 = (assign15070_e22797 * 0.001);
                let (assign15070_e22815, assign15070_e22815_d_n4,) = {
                    if (assign15070_e22795 < assign15070_e22799) {
                        let assign15070_e22802: f64 = (-0.001);
                        let assign15070_e22804: f64 = (assign15070_e22802 * 0.001);
                        let assign15070_e22807: f64 = (-locals.var_at_i);
                        let assign15070_e22809: f64 = (assign15070_e22807 * locals.var_deltemp);
                        let assign15070_e22810: f64 = (1.0 + assign15070_e22809);
                        let assign15070_e22812: f64 = (assign15070_e22810 - 1e-6);
                        let assign15070_e22813: f64 = (assign15070_e22804 / assign15070_e22812);
                        (assign15070_e22813, (-((assign15070_e22804 * (assign15070_e22807 * locals.var_deltemp_dn4)) / (assign15070_e22812 * assign15070_e22812))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15070_e22815, assign15070_e22815_d_n4,)
            }
        };
        let assign15070_e22817: f64 = (locals.var_vsat1_i * assign15070_e22816);
        (assign15070_e22817, (locals.var_vsat1_i_dn0 * assign15070_e22816), (locals.var_vsat1_i_dn2 * assign15070_e22816), (locals.var_vsat1_i_dn3 * assign15070_e22816), ((locals.var_vsat1_i_dn4 * assign15070_e22816) + (locals.var_vsat1_i * assign15070_e22816_d_n4)), (locals.var_vsat1_i_dn5 * assign15070_e22816), (locals.var_vsat1_i_dn6 * assign15070_e22816), (locals.var_vsat1_i_dn7 * assign15070_e22816), (locals.var_vsat1_i_dn8 * assign15070_e22816), (locals.var_vsat1_i_dn9 * assign15070_e22816), (locals.var_vsat1_i_dn10 * assign15070_e22816), (locals.var_vsat1_i_dn11 * assign15070_e22816), (locals.var_vsat1_i_dn13 * assign15070_e22816), (locals.var_vsat1_i_dn14 * assign15070_e22816),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign15070_e22819;
        locals.var_vsat1_t_dn0 = assign15070_e22819_d_n0;
        locals.var_vsat1_t_dn2 = assign15070_e22819_d_n2;
        locals.var_vsat1_t_dn3 = assign15070_e22819_d_n3;
        locals.var_vsat1_t_dn4 = assign15070_e22819_d_n4;
        locals.var_vsat1_t_dn5 = assign15070_e22819_d_n5;
        locals.var_vsat1_t_dn6 = assign15070_e22819_d_n6;
        locals.var_vsat1_t_dn7 = assign15070_e22819_d_n7;
        locals.var_vsat1_t_dn8 = assign15070_e22819_d_n8;
        locals.var_vsat1_t_dn9 = assign15070_e22819_d_n9;
        locals.var_vsat1_t_dn10 = assign15070_e22819_d_n10;
        locals.var_vsat1_t_dn11 = assign15070_e22819_d_n11;
        locals.var_vsat1_t_dn13 = assign15070_e22819_d_n13;
        locals.var_vsat1_t_dn14 = assign15070_e22819_d_n14;
        locals.var_vsat1_t_rv = 0.0;

        let assign15080_e22822: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign15080_e22822;
        locals.var_guard254_rv = 0.0;

        let assign15090_e22825: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign15090_e22825;
        locals.var_guard255_rv = 0.0;

        let (assign15100_e22876, assign15100_e22876_d_n0, assign15100_e22876_d_n2, assign15100_e22876_d_n3, assign15100_e22876_d_n4, assign15100_e22876_d_n5, assign15100_e22876_d_n6, assign15100_e22876_d_n7, assign15100_e22876_d_n8, assign15100_e22876_d_n9, assign15100_e22876_d_n10, assign15100_e22876_d_n11, assign15100_e22876_d_n13, assign15100_e22876_d_n14,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard255 != 0.0)) {
        let assign15100_e22833: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22836: f64 = (-locals.var_at_i);
        let assign15100_e22838: f64 = (assign15100_e22836 * locals.var_deltemp);
        let assign15100_e22840: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22841: f64 = (assign15100_e22838 - assign15100_e22840);
        let assign15100_e22843: f64 = (assign15100_e22841 - 1e-6);
        let assign15100_e22845: f64 = (-locals.var_at_i);
        let assign15100_e22847: f64 = (assign15100_e22845 * locals.var_deltemp);
        let assign15100_e22849: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22850: f64 = (assign15100_e22847 - assign15100_e22849);
        let assign15100_e22852: f64 = (assign15100_e22850 - 1e-6);
        let assign15100_e22854: f64 = (-locals.var_at_i);
        let assign15100_e22856: f64 = (assign15100_e22854 * locals.var_deltemp);
        let assign15100_e22858: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22859: f64 = (assign15100_e22856 - assign15100_e22858);
        let assign15100_e22861: f64 = (assign15100_e22859 - 1e-6);
        let assign15100_e22862: f64 = (assign15100_e22852 * assign15100_e22861);
        let assign15100_e22865: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22866: f64 = (4.0 * assign15100_e22865);
        let assign15100_e22868: f64 = (assign15100_e22866 * 1e-6);
        let assign15100_e22869: f64 = (assign15100_e22862 - assign15100_e22868);
        let assign15100_e22870: f64 = (assign15100_e22869).sqrt();
        let assign15100_e22871: f64 = (assign15100_e22843 + assign15100_e22870);
        let assign15100_e22872: f64 = (0.5 * assign15100_e22871);
        let assign15100_e22873: f64 = (assign15100_e22833 + assign15100_e22872);
        let assign15100_e22874: f64 = (locals.var_vsat1r_i + assign15100_e22873);
        (assign15100_e22874, (locals.var_vsat1r_i_dn0 + ((-locals.var_vsat1r_i_dn0) + (0.5 * ((-(-locals.var_vsat1r_i_dn0)) + (((((-(-locals.var_vsat1r_i_dn0)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn0)))) - ((4.0 * (-locals.var_vsat1r_i_dn0)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn2 + ((-locals.var_vsat1r_i_dn2) + (0.5 * ((-(-locals.var_vsat1r_i_dn2)) + (((((-(-locals.var_vsat1r_i_dn2)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn2)))) - ((4.0 * (-locals.var_vsat1r_i_dn2)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn3 + ((-locals.var_vsat1r_i_dn3) + (0.5 * ((-(-locals.var_vsat1r_i_dn3)) + (((((-(-locals.var_vsat1r_i_dn3)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn3)))) - ((4.0 * (-locals.var_vsat1r_i_dn3)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn4 + ((-locals.var_vsat1r_i_dn4) + (0.5 * (((assign15100_e22836 * locals.var_deltemp_dn4) - (-locals.var_vsat1r_i_dn4)) + ((((((assign15100_e22845 * locals.var_deltemp_dn4) - (-locals.var_vsat1r_i_dn4)) * assign15100_e22861) + (assign15100_e22852 * ((assign15100_e22854 * locals.var_deltemp_dn4) - (-locals.var_vsat1r_i_dn4)))) - ((4.0 * (-locals.var_vsat1r_i_dn4)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn5 + ((-locals.var_vsat1r_i_dn5) + (0.5 * ((-(-locals.var_vsat1r_i_dn5)) + (((((-(-locals.var_vsat1r_i_dn5)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn5)))) - ((4.0 * (-locals.var_vsat1r_i_dn5)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn6 + ((-locals.var_vsat1r_i_dn6) + (0.5 * ((-(-locals.var_vsat1r_i_dn6)) + (((((-(-locals.var_vsat1r_i_dn6)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn6)))) - ((4.0 * (-locals.var_vsat1r_i_dn6)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn7 + ((-locals.var_vsat1r_i_dn7) + (0.5 * ((-(-locals.var_vsat1r_i_dn7)) + (((((-(-locals.var_vsat1r_i_dn7)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn7)))) - ((4.0 * (-locals.var_vsat1r_i_dn7)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn8 + ((-locals.var_vsat1r_i_dn8) + (0.5 * ((-(-locals.var_vsat1r_i_dn8)) + (((((-(-locals.var_vsat1r_i_dn8)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn8)))) - ((4.0 * (-locals.var_vsat1r_i_dn8)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn9 + ((-locals.var_vsat1r_i_dn9) + (0.5 * ((-(-locals.var_vsat1r_i_dn9)) + (((((-(-locals.var_vsat1r_i_dn9)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn9)))) - ((4.0 * (-locals.var_vsat1r_i_dn9)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn10 + ((-locals.var_vsat1r_i_dn10) + (0.5 * ((-(-locals.var_vsat1r_i_dn10)) + (((((-(-locals.var_vsat1r_i_dn10)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn10)))) - ((4.0 * (-locals.var_vsat1r_i_dn10)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn11 + ((-locals.var_vsat1r_i_dn11) + (0.5 * ((-(-locals.var_vsat1r_i_dn11)) + (((((-(-locals.var_vsat1r_i_dn11)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn11)))) - ((4.0 * (-locals.var_vsat1r_i_dn11)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn13 + ((-locals.var_vsat1r_i_dn13) + (0.5 * ((-(-locals.var_vsat1r_i_dn13)) + (((((-(-locals.var_vsat1r_i_dn13)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn13)))) - ((4.0 * (-locals.var_vsat1r_i_dn13)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn14 + ((-locals.var_vsat1r_i_dn14) + (0.5 * ((-(-locals.var_vsat1r_i_dn14)) + (((((-(-locals.var_vsat1r_i_dn14)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn14)))) - ((4.0 * (-locals.var_vsat1r_i_dn14)) * 1e-6)) / (2.0 * assign15100_e22870)))))),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15100_e22876;
        locals.var_vsat1r_t_dn0 = assign15100_e22876_d_n0;
        locals.var_vsat1r_t_dn2 = assign15100_e22876_d_n2;
        locals.var_vsat1r_t_dn3 = assign15100_e22876_d_n3;
        locals.var_vsat1r_t_dn4 = assign15100_e22876_d_n4;
        locals.var_vsat1r_t_dn5 = assign15100_e22876_d_n5;
        locals.var_vsat1r_t_dn6 = assign15100_e22876_d_n6;
        locals.var_vsat1r_t_dn7 = assign15100_e22876_d_n7;
        locals.var_vsat1r_t_dn8 = assign15100_e22876_d_n8;
        locals.var_vsat1r_t_dn9 = assign15100_e22876_d_n9;
        locals.var_vsat1r_t_dn10 = assign15100_e22876_d_n10;
        locals.var_vsat1r_t_dn11 = assign15100_e22876_d_n11;
        locals.var_vsat1r_t_dn13 = assign15100_e22876_d_n13;
        locals.var_vsat1r_t_dn14 = assign15100_e22876_d_n14;
        locals.var_vsat1r_t_rv = 0.0;

        let (assign15110_e22964, assign15110_e22964_d_n0, assign15110_e22964_d_n2, assign15110_e22964_d_n3, assign15110_e22964_d_n4, assign15110_e22964_d_n5, assign15110_e22964_d_n6, assign15110_e22964_d_n7, assign15110_e22964_d_n8, assign15110_e22964_d_n9, assign15110_e22964_d_n10, assign15110_e22964_d_n11, assign15110_e22964_d_n13, assign15110_e22964_d_n14,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard255 == 0.0)) {
        let assign15110_e22886: f64 = (-locals.var_at_i);
        let assign15110_e22888: f64 = (assign15110_e22886 * locals.var_deltemp);
        let assign15110_e22889: f64 = (1.0 + assign15110_e22888);
        let assign15110_e22891: f64 = (assign15110_e22889 - 1e-6);
        let assign15110_e22893: f64 = (-10000.0);
        let assign15110_e22895: f64 = (assign15110_e22893 * 0.001);
        let (assign15110_e22961, assign15110_e22961_d_n4,) = {
            if (!(assign15110_e22891 < assign15110_e22895)) {
                let assign15110_e22901: f64 = (-locals.var_at_i);
                let assign15110_e22903: f64 = (assign15110_e22901 * locals.var_deltemp);
                let assign15110_e22904: f64 = (1.0 + assign15110_e22903);
                let assign15110_e22906: f64 = (assign15110_e22904 - 1e-6);
                let assign15110_e22909: f64 = (-locals.var_at_i);
                let assign15110_e22911: f64 = (assign15110_e22909 * locals.var_deltemp);
                let assign15110_e22912: f64 = (1.0 + assign15110_e22911);
                let assign15110_e22914: f64 = (assign15110_e22912 - 1e-6);
                let assign15110_e22917: f64 = (-locals.var_at_i);
                let assign15110_e22919: f64 = (assign15110_e22917 * locals.var_deltemp);
                let assign15110_e22920: f64 = (1.0 + assign15110_e22919);
                let assign15110_e22922: f64 = (assign15110_e22920 - 1e-6);
                let assign15110_e22923: f64 = (assign15110_e22914 * assign15110_e22922);
                let assign15110_e22926: f64 = (4.0 * 0.001);
                let assign15110_e22928: f64 = (assign15110_e22926 * 0.001);
                let assign15110_e22929: f64 = (assign15110_e22923 + assign15110_e22928);
                let assign15110_e22930: f64 = (assign15110_e22929).sqrt();
                let assign15110_e22931: f64 = (assign15110_e22906 + assign15110_e22930);
                let assign15110_e22932: f64 = (0.5 * assign15110_e22931);
                (assign15110_e22932, (0.5 * ((assign15110_e22901 * locals.var_deltemp_dn4) + ((((assign15110_e22909 * locals.var_deltemp_dn4) * assign15110_e22922) + (assign15110_e22914 * (assign15110_e22917 * locals.var_deltemp_dn4))) / (2.0 * assign15110_e22930)))),)
            } else {
                let assign15110_e22935: f64 = (-locals.var_at_i);
                let assign15110_e22937: f64 = (assign15110_e22935 * locals.var_deltemp);
                let assign15110_e22938: f64 = (1.0 + assign15110_e22937);
                let assign15110_e22940: f64 = (assign15110_e22938 - 1e-6);
                let assign15110_e22942: f64 = (-10000.0);
                let assign15110_e22944: f64 = (assign15110_e22942 * 0.001);
                let (assign15110_e22960, assign15110_e22960_d_n4,) = {
                    if (assign15110_e22940 < assign15110_e22944) {
                        let assign15110_e22947: f64 = (-0.001);
                        let assign15110_e22949: f64 = (assign15110_e22947 * 0.001);
                        let assign15110_e22952: f64 = (-locals.var_at_i);
                        let assign15110_e22954: f64 = (assign15110_e22952 * locals.var_deltemp);
                        let assign15110_e22955: f64 = (1.0 + assign15110_e22954);
                        let assign15110_e22957: f64 = (assign15110_e22955 - 1e-6);
                        let assign15110_e22958: f64 = (assign15110_e22949 / assign15110_e22957);
                        (assign15110_e22958, (-((assign15110_e22949 * (assign15110_e22952 * locals.var_deltemp_dn4)) / (assign15110_e22957 * assign15110_e22957))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15110_e22960, assign15110_e22960_d_n4,)
            }
        };
        let assign15110_e22962: f64 = (locals.var_vsat1r_i * assign15110_e22961);
        (assign15110_e22962, (locals.var_vsat1r_i_dn0 * assign15110_e22961), (locals.var_vsat1r_i_dn2 * assign15110_e22961), (locals.var_vsat1r_i_dn3 * assign15110_e22961), ((locals.var_vsat1r_i_dn4 * assign15110_e22961) + (locals.var_vsat1r_i * assign15110_e22961_d_n4)), (locals.var_vsat1r_i_dn5 * assign15110_e22961), (locals.var_vsat1r_i_dn6 * assign15110_e22961), (locals.var_vsat1r_i_dn7 * assign15110_e22961), (locals.var_vsat1r_i_dn8 * assign15110_e22961), (locals.var_vsat1r_i_dn9 * assign15110_e22961), (locals.var_vsat1r_i_dn10 * assign15110_e22961), (locals.var_vsat1r_i_dn11 * assign15110_e22961), (locals.var_vsat1r_i_dn13 * assign15110_e22961), (locals.var_vsat1r_i_dn14 * assign15110_e22961),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15110_e22964;
        locals.var_vsat1r_t_dn0 = assign15110_e22964_d_n0;
        locals.var_vsat1r_t_dn2 = assign15110_e22964_d_n2;
        locals.var_vsat1r_t_dn3 = assign15110_e22964_d_n3;
        locals.var_vsat1r_t_dn4 = assign15110_e22964_d_n4;
        locals.var_vsat1r_t_dn5 = assign15110_e22964_d_n5;
        locals.var_vsat1r_t_dn6 = assign15110_e22964_d_n6;
        locals.var_vsat1r_t_dn7 = assign15110_e22964_d_n7;
        locals.var_vsat1r_t_dn8 = assign15110_e22964_d_n8;
        locals.var_vsat1r_t_dn9 = assign15110_e22964_d_n9;
        locals.var_vsat1r_t_dn10 = assign15110_e22964_d_n10;
        locals.var_vsat1r_t_dn11 = assign15110_e22964_d_n11;
        locals.var_vsat1r_t_dn13 = assign15110_e22964_d_n13;
        locals.var_vsat1r_t_dn14 = assign15110_e22964_d_n14;
        locals.var_vsat1r_t_rv = 0.0;

        let assign15120_e22967: f64 = if locals.var_vsat1r_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign15120_e22967;
        locals.var_guard256_rv = 0.0;

        let (assign15130_e22975, assign15130_e22975_d_n0, assign15130_e22975_d_n2, assign15130_e22975_d_n3, assign15130_e22975_d_n4, assign15130_e22975_d_n5, assign15130_e22975_d_n6, assign15130_e22975_d_n7, assign15130_e22975_d_n8, assign15130_e22975_d_n9, assign15130_e22975_d_n10, assign15130_e22975_d_n11, assign15130_e22975_d_n13, assign15130_e22975_d_n14,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard256 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15130_e22975;
        locals.var_vsat1r_t_dn0 = assign15130_e22975_d_n0;
        locals.var_vsat1r_t_dn2 = assign15130_e22975_d_n2;
        locals.var_vsat1r_t_dn3 = assign15130_e22975_d_n3;
        locals.var_vsat1r_t_dn4 = assign15130_e22975_d_n4;
        locals.var_vsat1r_t_dn5 = assign15130_e22975_d_n5;
        locals.var_vsat1r_t_dn6 = assign15130_e22975_d_n6;
        locals.var_vsat1r_t_dn7 = assign15130_e22975_d_n7;
        locals.var_vsat1r_t_dn8 = assign15130_e22975_d_n8;
        locals.var_vsat1r_t_dn9 = assign15130_e22975_d_n9;
        locals.var_vsat1r_t_dn10 = assign15130_e22975_d_n10;
        locals.var_vsat1r_t_dn11 = assign15130_e22975_d_n11;
        locals.var_vsat1r_t_dn13 = assign15130_e22975_d_n13;
        locals.var_vsat1r_t_dn14 = assign15130_e22975_d_n14;
        locals.var_vsat1r_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign15140_e22978: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign15140_e22978;
        locals.var_guard257_rv = 0.0;

        let (assign15150_e23027, assign15150_e23027_d_n0, assign15150_e23027_d_n2, assign15150_e23027_d_n3, assign15150_e23027_d_n4, assign15150_e23027_d_n5, assign15150_e23027_d_n6, assign15150_e23027_d_n7, assign15150_e23027_d_n8, assign15150_e23027_d_n9, assign15150_e23027_d_n10, assign15150_e23027_d_n11, assign15150_e23027_d_n13, assign15150_e23027_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard257 != 0.0)) {
        let assign15150_e22984: f64 = (-locals.var_vsatcv_i);
        let assign15150_e22987: f64 = (-locals.var_atcv_i);
        let assign15150_e22989: f64 = (assign15150_e22987 * locals.var_deltemp);
        let assign15150_e22991: f64 = (-locals.var_vsatcv_i);
        let assign15150_e22992: f64 = (assign15150_e22989 - assign15150_e22991);
        let assign15150_e22994: f64 = (assign15150_e22992 - 1e-6);
        let assign15150_e22996: f64 = (-locals.var_atcv_i);
        let assign15150_e22998: f64 = (assign15150_e22996 * locals.var_deltemp);
        let assign15150_e23000: f64 = (-locals.var_vsatcv_i);
        let assign15150_e23001: f64 = (assign15150_e22998 - assign15150_e23000);
        let assign15150_e23003: f64 = (assign15150_e23001 - 1e-6);
        let assign15150_e23005: f64 = (-locals.var_atcv_i);
        let assign15150_e23007: f64 = (assign15150_e23005 * locals.var_deltemp);
        let assign15150_e23009: f64 = (-locals.var_vsatcv_i);
        let assign15150_e23010: f64 = (assign15150_e23007 - assign15150_e23009);
        let assign15150_e23012: f64 = (assign15150_e23010 - 1e-6);
        let assign15150_e23013: f64 = (assign15150_e23003 * assign15150_e23012);
        let assign15150_e23016: f64 = (-locals.var_vsatcv_i);
        let assign15150_e23017: f64 = (4.0 * assign15150_e23016);
        let assign15150_e23019: f64 = (assign15150_e23017 * 1e-6);
        let assign15150_e23020: f64 = (assign15150_e23013 - assign15150_e23019);
        let assign15150_e23021: f64 = (assign15150_e23020).sqrt();
        let assign15150_e23022: f64 = (assign15150_e22994 + assign15150_e23021);
        let assign15150_e23023: f64 = (0.5 * assign15150_e23022);
        let assign15150_e23024: f64 = (assign15150_e22984 + assign15150_e23023);
        let assign15150_e23025: f64 = (locals.var_vsatcv_i + assign15150_e23024);
        (assign15150_e23025, (locals.var_vsatcv_i_dn0 + ((-locals.var_vsatcv_i_dn0) + (0.5 * ((-(-locals.var_vsatcv_i_dn0)) + (((((-(-locals.var_vsatcv_i_dn0)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn0)))) - ((4.0 * (-locals.var_vsatcv_i_dn0)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn2 + ((-locals.var_vsatcv_i_dn2) + (0.5 * ((-(-locals.var_vsatcv_i_dn2)) + (((((-(-locals.var_vsatcv_i_dn2)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn2)))) - ((4.0 * (-locals.var_vsatcv_i_dn2)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn3 + ((-locals.var_vsatcv_i_dn3) + (0.5 * ((-(-locals.var_vsatcv_i_dn3)) + (((((-(-locals.var_vsatcv_i_dn3)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn3)))) - ((4.0 * (-locals.var_vsatcv_i_dn3)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn4 + ((-locals.var_vsatcv_i_dn4) + (0.5 * (((assign15150_e22987 * locals.var_deltemp_dn4) - (-locals.var_vsatcv_i_dn4)) + ((((((assign15150_e22996 * locals.var_deltemp_dn4) - (-locals.var_vsatcv_i_dn4)) * assign15150_e23012) + (assign15150_e23003 * ((assign15150_e23005 * locals.var_deltemp_dn4) - (-locals.var_vsatcv_i_dn4)))) - ((4.0 * (-locals.var_vsatcv_i_dn4)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn5 + ((-locals.var_vsatcv_i_dn5) + (0.5 * ((-(-locals.var_vsatcv_i_dn5)) + (((((-(-locals.var_vsatcv_i_dn5)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn5)))) - ((4.0 * (-locals.var_vsatcv_i_dn5)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn6 + ((-locals.var_vsatcv_i_dn6) + (0.5 * ((-(-locals.var_vsatcv_i_dn6)) + (((((-(-locals.var_vsatcv_i_dn6)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn6)))) - ((4.0 * (-locals.var_vsatcv_i_dn6)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn7 + ((-locals.var_vsatcv_i_dn7) + (0.5 * ((-(-locals.var_vsatcv_i_dn7)) + (((((-(-locals.var_vsatcv_i_dn7)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn7)))) - ((4.0 * (-locals.var_vsatcv_i_dn7)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn8 + ((-locals.var_vsatcv_i_dn8) + (0.5 * ((-(-locals.var_vsatcv_i_dn8)) + (((((-(-locals.var_vsatcv_i_dn8)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn8)))) - ((4.0 * (-locals.var_vsatcv_i_dn8)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn9 + ((-locals.var_vsatcv_i_dn9) + (0.5 * ((-(-locals.var_vsatcv_i_dn9)) + (((((-(-locals.var_vsatcv_i_dn9)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn9)))) - ((4.0 * (-locals.var_vsatcv_i_dn9)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn10 + ((-locals.var_vsatcv_i_dn10) + (0.5 * ((-(-locals.var_vsatcv_i_dn10)) + (((((-(-locals.var_vsatcv_i_dn10)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn10)))) - ((4.0 * (-locals.var_vsatcv_i_dn10)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn11 + ((-locals.var_vsatcv_i_dn11) + (0.5 * ((-(-locals.var_vsatcv_i_dn11)) + (((((-(-locals.var_vsatcv_i_dn11)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn11)))) - ((4.0 * (-locals.var_vsatcv_i_dn11)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn13 + ((-locals.var_vsatcv_i_dn13) + (0.5 * ((-(-locals.var_vsatcv_i_dn13)) + (((((-(-locals.var_vsatcv_i_dn13)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn13)))) - ((4.0 * (-locals.var_vsatcv_i_dn13)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn14 + ((-locals.var_vsatcv_i_dn14) + (0.5 * ((-(-locals.var_vsatcv_i_dn14)) + (((((-(-locals.var_vsatcv_i_dn14)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn14)))) - ((4.0 * (-locals.var_vsatcv_i_dn14)) * 1e-6)) / (2.0 * assign15150_e23021)))))),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign15150_e23027;
        locals.var_vsatcv_t_dn0 = assign15150_e23027_d_n0;
        locals.var_vsatcv_t_dn2 = assign15150_e23027_d_n2;
        locals.var_vsatcv_t_dn3 = assign15150_e23027_d_n3;
        locals.var_vsatcv_t_dn4 = assign15150_e23027_d_n4;
        locals.var_vsatcv_t_dn5 = assign15150_e23027_d_n5;
        locals.var_vsatcv_t_dn6 = assign15150_e23027_d_n6;
        locals.var_vsatcv_t_dn7 = assign15150_e23027_d_n7;
        locals.var_vsatcv_t_dn8 = assign15150_e23027_d_n8;
        locals.var_vsatcv_t_dn9 = assign15150_e23027_d_n9;
        locals.var_vsatcv_t_dn10 = assign15150_e23027_d_n10;
        locals.var_vsatcv_t_dn11 = assign15150_e23027_d_n11;
        locals.var_vsatcv_t_dn13 = assign15150_e23027_d_n13;
        locals.var_vsatcv_t_dn14 = assign15150_e23027_d_n14;
        locals.var_vsatcv_t_rv = 0.0;

        let (assign15160_e23113, assign15160_e23113_d_n0, assign15160_e23113_d_n2, assign15160_e23113_d_n3, assign15160_e23113_d_n4, assign15160_e23113_d_n5, assign15160_e23113_d_n6, assign15160_e23113_d_n7, assign15160_e23113_d_n8, assign15160_e23113_d_n9, assign15160_e23113_d_n10, assign15160_e23113_d_n11, assign15160_e23113_d_n13, assign15160_e23113_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard257 == 0.0)) {
        let assign15160_e23035: f64 = (-locals.var_atcv_i);
        let assign15160_e23037: f64 = (assign15160_e23035 * locals.var_deltemp);
        let assign15160_e23038: f64 = (1.0 + assign15160_e23037);
        let assign15160_e23040: f64 = (assign15160_e23038 - 1e-6);
        let assign15160_e23042: f64 = (-10000.0);
        let assign15160_e23044: f64 = (assign15160_e23042 * 0.001);
        let (assign15160_e23110, assign15160_e23110_d_n4,) = {
            if (!(assign15160_e23040 < assign15160_e23044)) {
                let assign15160_e23050: f64 = (-locals.var_atcv_i);
                let assign15160_e23052: f64 = (assign15160_e23050 * locals.var_deltemp);
                let assign15160_e23053: f64 = (1.0 + assign15160_e23052);
                let assign15160_e23055: f64 = (assign15160_e23053 - 1e-6);
                let assign15160_e23058: f64 = (-locals.var_atcv_i);
                let assign15160_e23060: f64 = (assign15160_e23058 * locals.var_deltemp);
                let assign15160_e23061: f64 = (1.0 + assign15160_e23060);
                let assign15160_e23063: f64 = (assign15160_e23061 - 1e-6);
                let assign15160_e23066: f64 = (-locals.var_atcv_i);
                let assign15160_e23068: f64 = (assign15160_e23066 * locals.var_deltemp);
                let assign15160_e23069: f64 = (1.0 + assign15160_e23068);
                let assign15160_e23071: f64 = (assign15160_e23069 - 1e-6);
                let assign15160_e23072: f64 = (assign15160_e23063 * assign15160_e23071);
                let assign15160_e23075: f64 = (4.0 * 0.001);
                let assign15160_e23077: f64 = (assign15160_e23075 * 0.001);
                let assign15160_e23078: f64 = (assign15160_e23072 + assign15160_e23077);
                let assign15160_e23079: f64 = (assign15160_e23078).sqrt();
                let assign15160_e23080: f64 = (assign15160_e23055 + assign15160_e23079);
                let assign15160_e23081: f64 = (0.5 * assign15160_e23080);
                (assign15160_e23081, (0.5 * ((assign15160_e23050 * locals.var_deltemp_dn4) + ((((assign15160_e23058 * locals.var_deltemp_dn4) * assign15160_e23071) + (assign15160_e23063 * (assign15160_e23066 * locals.var_deltemp_dn4))) / (2.0 * assign15160_e23079)))),)
            } else {
                let assign15160_e23084: f64 = (-locals.var_atcv_i);
                let assign15160_e23086: f64 = (assign15160_e23084 * locals.var_deltemp);
                let assign15160_e23087: f64 = (1.0 + assign15160_e23086);
                let assign15160_e23089: f64 = (assign15160_e23087 - 1e-6);
                let assign15160_e23091: f64 = (-10000.0);
                let assign15160_e23093: f64 = (assign15160_e23091 * 0.001);
                let (assign15160_e23109, assign15160_e23109_d_n4,) = {
                    if (assign15160_e23089 < assign15160_e23093) {
                        let assign15160_e23096: f64 = (-0.001);
                        let assign15160_e23098: f64 = (assign15160_e23096 * 0.001);
                        let assign15160_e23101: f64 = (-locals.var_atcv_i);
                        let assign15160_e23103: f64 = (assign15160_e23101 * locals.var_deltemp);
                        let assign15160_e23104: f64 = (1.0 + assign15160_e23103);
                        let assign15160_e23106: f64 = (assign15160_e23104 - 1e-6);
                        let assign15160_e23107: f64 = (assign15160_e23098 / assign15160_e23106);
                        (assign15160_e23107, (-((assign15160_e23098 * (assign15160_e23101 * locals.var_deltemp_dn4)) / (assign15160_e23106 * assign15160_e23106))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15160_e23109, assign15160_e23109_d_n4,)
            }
        };
        let assign15160_e23111: f64 = (locals.var_vsatcv_i * assign15160_e23110);
        (assign15160_e23111, (locals.var_vsatcv_i_dn0 * assign15160_e23110), (locals.var_vsatcv_i_dn2 * assign15160_e23110), (locals.var_vsatcv_i_dn3 * assign15160_e23110), ((locals.var_vsatcv_i_dn4 * assign15160_e23110) + (locals.var_vsatcv_i * assign15160_e23110_d_n4)), (locals.var_vsatcv_i_dn5 * assign15160_e23110), (locals.var_vsatcv_i_dn6 * assign15160_e23110), (locals.var_vsatcv_i_dn7 * assign15160_e23110), (locals.var_vsatcv_i_dn8 * assign15160_e23110), (locals.var_vsatcv_i_dn9 * assign15160_e23110), (locals.var_vsatcv_i_dn10 * assign15160_e23110), (locals.var_vsatcv_i_dn11 * assign15160_e23110), (locals.var_vsatcv_i_dn13 * assign15160_e23110), (locals.var_vsatcv_i_dn14 * assign15160_e23110),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign15160_e23113;
        locals.var_vsatcv_t_dn0 = assign15160_e23113_d_n0;
        locals.var_vsatcv_t_dn2 = assign15160_e23113_d_n2;
        locals.var_vsatcv_t_dn3 = assign15160_e23113_d_n3;
        locals.var_vsatcv_t_dn4 = assign15160_e23113_d_n4;
        locals.var_vsatcv_t_dn5 = assign15160_e23113_d_n5;
        locals.var_vsatcv_t_dn6 = assign15160_e23113_d_n6;
        locals.var_vsatcv_t_dn7 = assign15160_e23113_d_n7;
        locals.var_vsatcv_t_dn8 = assign15160_e23113_d_n8;
        locals.var_vsatcv_t_dn9 = assign15160_e23113_d_n9;
        locals.var_vsatcv_t_dn10 = assign15160_e23113_d_n10;
        locals.var_vsatcv_t_dn11 = assign15160_e23113_d_n11;
        locals.var_vsatcv_t_dn13 = assign15160_e23113_d_n13;
        locals.var_vsatcv_t_dn14 = assign15160_e23113_d_n14;
        locals.var_vsatcv_t_rv = 0.0;

        let (assign15170_e23202, assign15170_e23202_d_n0, assign15170_e23202_d_n2, assign15170_e23202_d_n3, assign15170_e23202_d_n4, assign15170_e23202_d_n5, assign15170_e23202_d_n6, assign15170_e23202_d_n7, assign15170_e23202_d_n8, assign15170_e23202_d_n9, assign15170_e23202_d_n10, assign15170_e23202_d_n11, assign15170_e23202_d_n13, assign15170_e23202_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign15170_e23119: f64 = (p.p450 * locals.var_deltemp);
        let assign15170_e23120: f64 = (1.0 + assign15170_e23119);
        let assign15170_e23121: f64 = (locals.var_mexp_i * assign15170_e23120);
        let assign15170_e23123: f64 = (assign15170_e23121 - 2.0);
        let assign15170_e23125: f64 = (-10000.0);
        let assign15170_e23127: f64 = (assign15170_e23125 * 0.001);
        let (assign15170_e23198, assign15170_e23198_d_n0, assign15170_e23198_d_n2, assign15170_e23198_d_n3, assign15170_e23198_d_n4, assign15170_e23198_d_n5, assign15170_e23198_d_n6, assign15170_e23198_d_n7, assign15170_e23198_d_n8, assign15170_e23198_d_n9, assign15170_e23198_d_n10, assign15170_e23198_d_n11, assign15170_e23198_d_n13, assign15170_e23198_d_n14,) = {
            if (!(assign15170_e23123 < assign15170_e23127)) {
                let assign15170_e23135: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23136: f64 = (1.0 + assign15170_e23135);
                let assign15170_e23137: f64 = (locals.var_mexp_i * assign15170_e23136);
                let assign15170_e23139: f64 = (assign15170_e23137 - 2.0);
                let assign15170_e23144: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23145: f64 = (1.0 + assign15170_e23144);
                let assign15170_e23146: f64 = (locals.var_mexp_i * assign15170_e23145);
                let assign15170_e23148: f64 = (assign15170_e23146 - 2.0);
                let assign15170_e23153: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23154: f64 = (1.0 + assign15170_e23153);
                let assign15170_e23155: f64 = (locals.var_mexp_i * assign15170_e23154);
                let assign15170_e23157: f64 = (assign15170_e23155 - 2.0);
                let assign15170_e23158: f64 = (assign15170_e23148 * assign15170_e23157);
                let assign15170_e23161: f64 = (4.0 * 0.001);
                let assign15170_e23163: f64 = (assign15170_e23161 * 0.001);
                let assign15170_e23164: f64 = (assign15170_e23158 + assign15170_e23163);
                let assign15170_e23165: f64 = (assign15170_e23164).sqrt();
                let assign15170_e23166: f64 = (assign15170_e23139 + assign15170_e23165);
                let assign15170_e23167: f64 = (0.5 * assign15170_e23166);
                (assign15170_e23167, (0.5 * ((locals.var_mexp_i_dn0 * assign15170_e23136) + ((((locals.var_mexp_i_dn0 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn0 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn2 * assign15170_e23136) + ((((locals.var_mexp_i_dn2 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn2 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn3 * assign15170_e23136) + ((((locals.var_mexp_i_dn3 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn3 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * (((locals.var_mexp_i_dn4 * assign15170_e23136) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4))) + (((((locals.var_mexp_i_dn4 * assign15170_e23145) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4))) * assign15170_e23157) + (assign15170_e23148 * ((locals.var_mexp_i_dn4 * assign15170_e23154) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4))))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn5 * assign15170_e23136) + ((((locals.var_mexp_i_dn5 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn5 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn6 * assign15170_e23136) + ((((locals.var_mexp_i_dn6 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn6 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn7 * assign15170_e23136) + ((((locals.var_mexp_i_dn7 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn7 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn8 * assign15170_e23136) + ((((locals.var_mexp_i_dn8 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn8 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn9 * assign15170_e23136) + ((((locals.var_mexp_i_dn9 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn9 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn10 * assign15170_e23136) + ((((locals.var_mexp_i_dn10 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn10 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn11 * assign15170_e23136) + ((((locals.var_mexp_i_dn11 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn11 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn13 * assign15170_e23136) + ((((locals.var_mexp_i_dn13 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn13 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn14 * assign15170_e23136) + ((((locals.var_mexp_i_dn14 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn14 * assign15170_e23154))) / (2.0 * assign15170_e23165)))),)
            } else {
                let assign15170_e23172: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23173: f64 = (1.0 + assign15170_e23172);
                let assign15170_e23174: f64 = (locals.var_mexp_i * assign15170_e23173);
                let assign15170_e23176: f64 = (assign15170_e23174 - 2.0);
                let assign15170_e23178: f64 = (-10000.0);
                let assign15170_e23180: f64 = (assign15170_e23178 * 0.001);
                let (assign15170_e23197, assign15170_e23197_d_n0, assign15170_e23197_d_n2, assign15170_e23197_d_n3, assign15170_e23197_d_n4, assign15170_e23197_d_n5, assign15170_e23197_d_n6, assign15170_e23197_d_n7, assign15170_e23197_d_n8, assign15170_e23197_d_n9, assign15170_e23197_d_n10, assign15170_e23197_d_n11, assign15170_e23197_d_n13, assign15170_e23197_d_n14,) = {
                    if (assign15170_e23176 < assign15170_e23180) {
                        let assign15170_e23183: f64 = (-0.001);
                        let assign15170_e23185: f64 = (assign15170_e23183 * 0.001);
                        let assign15170_e23190: f64 = (p.p450 * locals.var_deltemp);
                        let assign15170_e23191: f64 = (1.0 + assign15170_e23190);
                        let assign15170_e23192: f64 = (locals.var_mexp_i * assign15170_e23191);
                        let assign15170_e23194: f64 = (assign15170_e23192 - 2.0);
                        let assign15170_e23195: f64 = (assign15170_e23185 / assign15170_e23194);
                        (assign15170_e23195, (-((assign15170_e23185 * (locals.var_mexp_i_dn0 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn2 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn3 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * ((locals.var_mexp_i_dn4 * assign15170_e23191) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4)))) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn5 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn6 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn7 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn8 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn9 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn10 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn11 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn13 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn14 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15170_e23197, assign15170_e23197_d_n0, assign15170_e23197_d_n2, assign15170_e23197_d_n3, assign15170_e23197_d_n4, assign15170_e23197_d_n5, assign15170_e23197_d_n6, assign15170_e23197_d_n7, assign15170_e23197_d_n8, assign15170_e23197_d_n9, assign15170_e23197_d_n10, assign15170_e23197_d_n11, assign15170_e23197_d_n13, assign15170_e23197_d_n14,)
            }
        };
        let assign15170_e23200: f64 = (assign15170_e23198 + 2.0);
        (assign15170_e23200, assign15170_e23198_d_n0, assign15170_e23198_d_n2, assign15170_e23198_d_n3, assign15170_e23198_d_n4, assign15170_e23198_d_n5, assign15170_e23198_d_n6, assign15170_e23198_d_n7, assign15170_e23198_d_n8, assign15170_e23198_d_n9, assign15170_e23198_d_n10, assign15170_e23198_d_n11, assign15170_e23198_d_n13, assign15170_e23198_d_n14,)
    } else {
        (locals.var_mexp_t, locals.var_mexp_t_dn0, locals.var_mexp_t_dn2, locals.var_mexp_t_dn3, locals.var_mexp_t_dn4, locals.var_mexp_t_dn5, locals.var_mexp_t_dn6, locals.var_mexp_t_dn7, locals.var_mexp_t_dn8, locals.var_mexp_t_dn9, locals.var_mexp_t_dn10, locals.var_mexp_t_dn11, locals.var_mexp_t_dn13, locals.var_mexp_t_dn14,)
    }
};
        locals.var_mexp_t = assign15170_e23202;
        locals.var_mexp_t_dn0 = assign15170_e23202_d_n0;
        locals.var_mexp_t_dn2 = assign15170_e23202_d_n2;
        locals.var_mexp_t_dn3 = assign15170_e23202_d_n3;
        locals.var_mexp_t_dn4 = assign15170_e23202_d_n4;
        locals.var_mexp_t_dn5 = assign15170_e23202_d_n5;
        locals.var_mexp_t_dn6 = assign15170_e23202_d_n6;
        locals.var_mexp_t_dn7 = assign15170_e23202_d_n7;
        locals.var_mexp_t_dn8 = assign15170_e23202_d_n8;
        locals.var_mexp_t_dn9 = assign15170_e23202_d_n9;
        locals.var_mexp_t_dn10 = assign15170_e23202_d_n10;
        locals.var_mexp_t_dn11 = assign15170_e23202_d_n11;
        locals.var_mexp_t_dn13 = assign15170_e23202_d_n13;
        locals.var_mexp_t_dn14 = assign15170_e23202_d_n14;
        locals.var_mexp_t_rv = 0.0;

        let assign15180_e23205: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign15180_e23205;
        locals.var_guard258_rv = 0.0;

        let (assign15190_e23296, assign15190_e23296_d_n0, assign15190_e23296_d_n2, assign15190_e23296_d_n3, assign15190_e23296_d_n4, assign15190_e23296_d_n5, assign15190_e23296_d_n6, assign15190_e23296_d_n7, assign15190_e23296_d_n8, assign15190_e23296_d_n9, assign15190_e23296_d_n10, assign15190_e23296_d_n11, assign15190_e23296_d_n13, assign15190_e23296_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign15190_e23213: f64 = (p.p452 * locals.var_deltemp);
        let assign15190_e23214: f64 = (1.0 + assign15190_e23213);
        let assign15190_e23215: f64 = (locals.var_mexpr_i * assign15190_e23214);
        let assign15190_e23217: f64 = (assign15190_e23215 - 2.0);
        let assign15190_e23219: f64 = (-10000.0);
        let assign15190_e23221: f64 = (assign15190_e23219 * 0.001);
        let (assign15190_e23292, assign15190_e23292_d_n0, assign15190_e23292_d_n2, assign15190_e23292_d_n3, assign15190_e23292_d_n4, assign15190_e23292_d_n5, assign15190_e23292_d_n6, assign15190_e23292_d_n7, assign15190_e23292_d_n8, assign15190_e23292_d_n9, assign15190_e23292_d_n10, assign15190_e23292_d_n11, assign15190_e23292_d_n13, assign15190_e23292_d_n14,) = {
            if (!(assign15190_e23217 < assign15190_e23221)) {
                let assign15190_e23229: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23230: f64 = (1.0 + assign15190_e23229);
                let assign15190_e23231: f64 = (locals.var_mexpr_i * assign15190_e23230);
                let assign15190_e23233: f64 = (assign15190_e23231 - 2.0);
                let assign15190_e23238: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23239: f64 = (1.0 + assign15190_e23238);
                let assign15190_e23240: f64 = (locals.var_mexpr_i * assign15190_e23239);
                let assign15190_e23242: f64 = (assign15190_e23240 - 2.0);
                let assign15190_e23247: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23248: f64 = (1.0 + assign15190_e23247);
                let assign15190_e23249: f64 = (locals.var_mexpr_i * assign15190_e23248);
                let assign15190_e23251: f64 = (assign15190_e23249 - 2.0);
                let assign15190_e23252: f64 = (assign15190_e23242 * assign15190_e23251);
                let assign15190_e23255: f64 = (4.0 * 0.001);
                let assign15190_e23257: f64 = (assign15190_e23255 * 0.001);
                let assign15190_e23258: f64 = (assign15190_e23252 + assign15190_e23257);
                let assign15190_e23259: f64 = (assign15190_e23258).sqrt();
                let assign15190_e23260: f64 = (assign15190_e23233 + assign15190_e23259);
                let assign15190_e23261: f64 = (0.5 * assign15190_e23260);
                (assign15190_e23261, (0.5 * ((locals.var_mexpr_i_dn0 * assign15190_e23230) + ((((locals.var_mexpr_i_dn0 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn0 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn2 * assign15190_e23230) + ((((locals.var_mexpr_i_dn2 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn2 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn3 * assign15190_e23230) + ((((locals.var_mexpr_i_dn3 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn3 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * (((locals.var_mexpr_i_dn4 * assign15190_e23230) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4))) + (((((locals.var_mexpr_i_dn4 * assign15190_e23239) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4))) * assign15190_e23251) + (assign15190_e23242 * ((locals.var_mexpr_i_dn4 * assign15190_e23248) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4))))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn5 * assign15190_e23230) + ((((locals.var_mexpr_i_dn5 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn5 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn6 * assign15190_e23230) + ((((locals.var_mexpr_i_dn6 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn6 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn7 * assign15190_e23230) + ((((locals.var_mexpr_i_dn7 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn7 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn8 * assign15190_e23230) + ((((locals.var_mexpr_i_dn8 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn8 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn9 * assign15190_e23230) + ((((locals.var_mexpr_i_dn9 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn9 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn10 * assign15190_e23230) + ((((locals.var_mexpr_i_dn10 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn10 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn11 * assign15190_e23230) + ((((locals.var_mexpr_i_dn11 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn11 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn13 * assign15190_e23230) + ((((locals.var_mexpr_i_dn13 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn13 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn14 * assign15190_e23230) + ((((locals.var_mexpr_i_dn14 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn14 * assign15190_e23248))) / (2.0 * assign15190_e23259)))),)
            } else {
                let assign15190_e23266: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23267: f64 = (1.0 + assign15190_e23266);
                let assign15190_e23268: f64 = (locals.var_mexpr_i * assign15190_e23267);
                let assign15190_e23270: f64 = (assign15190_e23268 - 2.0);
                let assign15190_e23272: f64 = (-10000.0);
                let assign15190_e23274: f64 = (assign15190_e23272 * 0.001);
                let (assign15190_e23291, assign15190_e23291_d_n0, assign15190_e23291_d_n2, assign15190_e23291_d_n3, assign15190_e23291_d_n4, assign15190_e23291_d_n5, assign15190_e23291_d_n6, assign15190_e23291_d_n7, assign15190_e23291_d_n8, assign15190_e23291_d_n9, assign15190_e23291_d_n10, assign15190_e23291_d_n11, assign15190_e23291_d_n13, assign15190_e23291_d_n14,) = {
                    if (assign15190_e23270 < assign15190_e23274) {
                        let assign15190_e23277: f64 = (-0.001);
                        let assign15190_e23279: f64 = (assign15190_e23277 * 0.001);
                        let assign15190_e23284: f64 = (p.p452 * locals.var_deltemp);
                        let assign15190_e23285: f64 = (1.0 + assign15190_e23284);
                        let assign15190_e23286: f64 = (locals.var_mexpr_i * assign15190_e23285);
                        let assign15190_e23288: f64 = (assign15190_e23286 - 2.0);
                        let assign15190_e23289: f64 = (assign15190_e23279 / assign15190_e23288);
                        (assign15190_e23289, (-((assign15190_e23279 * (locals.var_mexpr_i_dn0 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn2 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn3 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * ((locals.var_mexpr_i_dn4 * assign15190_e23285) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4)))) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn5 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn6 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn7 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn8 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn9 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn10 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn11 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn13 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn14 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15190_e23291, assign15190_e23291_d_n0, assign15190_e23291_d_n2, assign15190_e23291_d_n3, assign15190_e23291_d_n4, assign15190_e23291_d_n5, assign15190_e23291_d_n6, assign15190_e23291_d_n7, assign15190_e23291_d_n8, assign15190_e23291_d_n9, assign15190_e23291_d_n10, assign15190_e23291_d_n11, assign15190_e23291_d_n13, assign15190_e23291_d_n14,)
            }
        };
        let assign15190_e23294: f64 = (assign15190_e23292 + 2.0);
        (assign15190_e23294, assign15190_e23292_d_n0, assign15190_e23292_d_n2, assign15190_e23292_d_n3, assign15190_e23292_d_n4, assign15190_e23292_d_n5, assign15190_e23292_d_n6, assign15190_e23292_d_n7, assign15190_e23292_d_n8, assign15190_e23292_d_n9, assign15190_e23292_d_n10, assign15190_e23292_d_n11, assign15190_e23292_d_n13, assign15190_e23292_d_n14,)
    } else {
        (locals.var_mexpr_t, locals.var_mexpr_t_dn0, locals.var_mexpr_t_dn2, locals.var_mexpr_t_dn3, locals.var_mexpr_t_dn4, locals.var_mexpr_t_dn5, locals.var_mexpr_t_dn6, locals.var_mexpr_t_dn7, locals.var_mexpr_t_dn8, locals.var_mexpr_t_dn9, locals.var_mexpr_t_dn10, locals.var_mexpr_t_dn11, locals.var_mexpr_t_dn13, locals.var_mexpr_t_dn14,)
    }
};
        locals.var_mexpr_t = assign15190_e23296;
        locals.var_mexpr_t_dn0 = assign15190_e23296_d_n0;
        locals.var_mexpr_t_dn2 = assign15190_e23296_d_n2;
        locals.var_mexpr_t_dn3 = assign15190_e23296_d_n3;
        locals.var_mexpr_t_dn4 = assign15190_e23296_d_n4;
        locals.var_mexpr_t_dn5 = assign15190_e23296_d_n5;
        locals.var_mexpr_t_dn6 = assign15190_e23296_d_n6;
        locals.var_mexpr_t_dn7 = assign15190_e23296_d_n7;
        locals.var_mexpr_t_dn8 = assign15190_e23296_d_n8;
        locals.var_mexpr_t_dn9 = assign15190_e23296_d_n9;
        locals.var_mexpr_t_dn10 = assign15190_e23296_d_n10;
        locals.var_mexpr_t_dn11 = assign15190_e23296_d_n11;
        locals.var_mexpr_t_dn13 = assign15190_e23296_d_n13;
        locals.var_mexpr_t_dn14 = assign15190_e23296_d_n14;
        locals.var_mexpr_t_rv = 0.0;

        let (assign15200_e23300, assign15200_e23300_d_n4,) = {
    if (locals.var_guard244 != 0.0) {
        (locals.var_ksativ_i, 0.0,)
    } else {
        (locals.var_ksativ_t, locals.var_ksativ_t_dn4,)
    }
};
        locals.var_ksativ_t = assign15200_e23300;
        locals.var_ksativ_t_dn4 = assign15200_e23300_d_n4;
        locals.var_ksativ_t_rv = 0.0;

        let (assign15210_e23304, assign15210_e23304_d_n0, assign15210_e23304_d_n2, assign15210_e23304_d_n3, assign15210_e23304_d_n4, assign15210_e23304_d_n5, assign15210_e23304_d_n6, assign15210_e23304_d_n7, assign15210_e23304_d_n8, assign15210_e23304_d_n9, assign15210_e23304_d_n10, assign15210_e23304_d_n11, assign15210_e23304_d_n13, assign15210_e23304_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        (locals.var_pclm_i, locals.var_pclm_i_dn0, locals.var_pclm_i_dn2, locals.var_pclm_i_dn3, locals.var_pclm_i_dn4, locals.var_pclm_i_dn5, locals.var_pclm_i_dn6, locals.var_pclm_i_dn7, locals.var_pclm_i_dn8, locals.var_pclm_i_dn9, locals.var_pclm_i_dn10, locals.var_pclm_i_dn11, locals.var_pclm_i_dn13, locals.var_pclm_i_dn14,)
    } else {
        (locals.var_pclm_t, locals.var_pclm_t_dn0, locals.var_pclm_t_dn2, locals.var_pclm_t_dn3, locals.var_pclm_t_dn4, locals.var_pclm_t_dn5, locals.var_pclm_t_dn6, locals.var_pclm_t_dn7, locals.var_pclm_t_dn8, locals.var_pclm_t_dn9, locals.var_pclm_t_dn10, locals.var_pclm_t_dn11, locals.var_pclm_t_dn13, locals.var_pclm_t_dn14,)
    }
};
        locals.var_pclm_t = assign15210_e23304;
        locals.var_pclm_t_dn0 = assign15210_e23304_d_n0;
        locals.var_pclm_t_dn2 = assign15210_e23304_d_n2;
        locals.var_pclm_t_dn3 = assign15210_e23304_d_n3;
        locals.var_pclm_t_dn4 = assign15210_e23304_d_n4;
        locals.var_pclm_t_dn5 = assign15210_e23304_d_n5;
        locals.var_pclm_t_dn6 = assign15210_e23304_d_n6;
        locals.var_pclm_t_dn7 = assign15210_e23304_d_n7;
        locals.var_pclm_t_dn8 = assign15210_e23304_d_n8;
        locals.var_pclm_t_dn9 = assign15210_e23304_d_n9;
        locals.var_pclm_t_dn10 = assign15210_e23304_d_n10;
        locals.var_pclm_t_dn11 = assign15210_e23304_d_n11;
        locals.var_pclm_t_dn13 = assign15210_e23304_d_n13;
        locals.var_pclm_t_dn14 = assign15210_e23304_d_n14;
        locals.var_pclm_t_rv = 0.0;

        let (assign15220_e23314, assign15220_e23314_d_n0, assign15220_e23314_d_n2, assign15220_e23314_d_n3, assign15220_e23314_d_n4, assign15220_e23314_d_n5, assign15220_e23314_d_n6, assign15220_e23314_d_n7, assign15220_e23314_d_n8, assign15220_e23314_d_n9, assign15220_e23314_d_n10, assign15220_e23314_d_n11, assign15220_e23314_d_n13, assign15220_e23314_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign15220_e23309: f64 = (p.p1720 / locals.var_leff_1);
        let assign15220_e23310: f64 = (locals.var_kt1_i + assign15220_e23309);
        let assign15220_e23312: f64 = (assign15220_e23310 * locals.var_tratio_m1);
        (assign15220_e23312, ((-((p.p1720 * locals.var_leff_1_dn0) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn2) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn3) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), (((-((p.p1720 * locals.var_leff_1_dn4) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) + (assign15220_e23310 * locals.var_tratio_m1_dn4)), ((-((p.p1720 * locals.var_leff_1_dn5) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn6) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn7) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn8) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn9) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn10) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn11) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn13) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn14) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1),)
    } else {
        (locals.var_dvth_temp, locals.var_dvth_temp_dn0, locals.var_dvth_temp_dn2, locals.var_dvth_temp_dn3, locals.var_dvth_temp_dn4, locals.var_dvth_temp_dn5, locals.var_dvth_temp_dn6, locals.var_dvth_temp_dn7, locals.var_dvth_temp_dn8, locals.var_dvth_temp_dn9, locals.var_dvth_temp_dn10, locals.var_dvth_temp_dn11, locals.var_dvth_temp_dn13, locals.var_dvth_temp_dn14,)
    }
};
        locals.var_dvth_temp = assign15220_e23314;
        locals.var_dvth_temp_dn0 = assign15220_e23314_d_n0;
        locals.var_dvth_temp_dn2 = assign15220_e23314_d_n2;
        locals.var_dvth_temp_dn3 = assign15220_e23314_d_n3;
        locals.var_dvth_temp_dn4 = assign15220_e23314_d_n4;
        locals.var_dvth_temp_dn5 = assign15220_e23314_d_n5;
        locals.var_dvth_temp_dn6 = assign15220_e23314_d_n6;
        locals.var_dvth_temp_dn7 = assign15220_e23314_d_n7;
        locals.var_dvth_temp_dn8 = assign15220_e23314_d_n8;
        locals.var_dvth_temp_dn9 = assign15220_e23314_d_n9;
        locals.var_dvth_temp_dn10 = assign15220_e23314_d_n10;
        locals.var_dvth_temp_dn11 = assign15220_e23314_d_n11;
        locals.var_dvth_temp_dn13 = assign15220_e23314_d_n13;
        locals.var_dvth_temp_dn14 = assign15220_e23314_d_n14;
        locals.var_dvth_temp_rv = 0.0;

        let assign15230_e23317: f64 = if p.p80 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign15230_e23317;
        locals.var_guard259_rv = 0.0;

        let (assign15240_e23333, assign15240_e23333_d_n0, assign15240_e23333_d_n2, assign15240_e23333_d_n3, assign15240_e23333_d_n4, assign15240_e23333_d_n5, assign15240_e23333_d_n6, assign15240_e23333_d_n7, assign15240_e23333_d_n8, assign15240_e23333_d_n9, assign15240_e23333_d_n10, assign15240_e23333_d_n11, assign15240_e23333_d_n13, assign15240_e23333_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15240_e23326: f64 = (locals.var_ute1_i * locals.var_tratio);
        let assign15240_e23327: f64 = (locals.var_ute_i + assign15240_e23326);
        let assign15240_e23329: f64 = (assign15240_e23327 * locals.var_trat_ln);
        let assign15240_e23330: f64 = (assign15240_e23329).exp();
        let assign15240_e23331: f64 = (locals.var_u0_i * assign15240_e23330);
        (assign15240_e23331, (locals.var_u0_i_dn0 * assign15240_e23330), (locals.var_u0_i_dn2 * assign15240_e23330), (locals.var_u0_i_dn3 * assign15240_e23330), ((locals.var_u0_i_dn4 * assign15240_e23330) + (locals.var_u0_i * (assign15240_e23330 * (((locals.var_ute1_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15240_e23327 * locals.var_trat_ln_dn4))))), (locals.var_u0_i_dn5 * assign15240_e23330), (locals.var_u0_i_dn6 * assign15240_e23330), (locals.var_u0_i_dn7 * assign15240_e23330), (locals.var_u0_i_dn8 * assign15240_e23330), (locals.var_u0_i_dn9 * assign15240_e23330), (locals.var_u0_i_dn10 * assign15240_e23330), (locals.var_u0_i_dn11 * assign15240_e23330), (locals.var_u0_i_dn13 * assign15240_e23330), (locals.var_u0_i_dn14 * assign15240_e23330),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15240_e23333;
        locals.var_t1_dn0 = assign15240_e23333_d_n0;
        locals.var_t1_dn2 = assign15240_e23333_d_n2;
        locals.var_t1_dn3 = assign15240_e23333_d_n3;
        locals.var_t1_dn4 = assign15240_e23333_d_n4;
        locals.var_t1_dn5 = assign15240_e23333_d_n5;
        locals.var_t1_dn6 = assign15240_e23333_d_n6;
        locals.var_t1_dn7 = assign15240_e23333_d_n7;
        locals.var_t1_dn8 = assign15240_e23333_d_n8;
        locals.var_t1_dn9 = assign15240_e23333_d_n9;
        locals.var_t1_dn10 = assign15240_e23333_d_n10;
        locals.var_t1_dn11 = assign15240_e23333_d_n11;
        locals.var_t1_dn13 = assign15240_e23333_d_n13;
        locals.var_t1_dn14 = assign15240_e23333_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15250_e23390, assign15250_e23390_d_n0, assign15250_e23390_d_n2, assign15250_e23390_d_n3, assign15250_e23390_d_n4, assign15250_e23390_d_n5, assign15250_e23390_d_n6, assign15250_e23390_d_n7, assign15250_e23390_d_n8, assign15250_e23390_d_n9, assign15250_e23390_d_n10, assign15250_e23390_d_n11, assign15250_e23390_d_n13, assign15250_e23390_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15250_e23340: f64 = (-0.9);
        let assign15250_e23342: f64 = (assign15250_e23340 * locals.var_t1);
        let assign15250_e23346: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15250_e23348: f64 = (-0.9);
        let assign15250_e23350: f64 = (assign15250_e23348 * locals.var_t1);
        let assign15250_e23351: f64 = (assign15250_e23346 - assign15250_e23350);
        let assign15250_e23353: f64 = (assign15250_e23351 - 0.0001);
        let assign15250_e23356: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15250_e23358: f64 = (-0.9);
        let assign15250_e23360: f64 = (assign15250_e23358 * locals.var_t1);
        let assign15250_e23361: f64 = (assign15250_e23356 - assign15250_e23360);
        let assign15250_e23363: f64 = (assign15250_e23361 - 0.0001);
        let assign15250_e23366: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15250_e23368: f64 = (-0.9);
        let assign15250_e23370: f64 = (assign15250_e23368 * locals.var_t1);
        let assign15250_e23371: f64 = (assign15250_e23366 - assign15250_e23370);
        let assign15250_e23373: f64 = (assign15250_e23371 - 0.0001);
        let assign15250_e23374: f64 = (assign15250_e23363 * assign15250_e23373);
        let assign15250_e23377: f64 = (-0.9);
        let assign15250_e23379: f64 = (assign15250_e23377 * locals.var_t1);
        let assign15250_e23380: f64 = (4.0 * assign15250_e23379);
        let assign15250_e23382: f64 = (assign15250_e23380 * 0.0001);
        let assign15250_e23383: f64 = (assign15250_e23374 - assign15250_e23382);
        let assign15250_e23384: f64 = (assign15250_e23383).sqrt();
        let assign15250_e23385: f64 = (assign15250_e23353 + assign15250_e23384);
        let assign15250_e23386: f64 = (0.5 * assign15250_e23385);
        let assign15250_e23387: f64 = (assign15250_e23342 + assign15250_e23386);
        let assign15250_e23388: f64 = (locals.var_t1 + assign15250_e23387);
        (assign15250_e23388, (locals.var_t1_dn0 + ((assign15250_e23340 * locals.var_t1_dn0) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn0)) + (((((-(assign15250_e23358 * locals.var_t1_dn0)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn0)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn2 + ((assign15250_e23340 * locals.var_t1_dn2) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn2)) + (((((-(assign15250_e23358 * locals.var_t1_dn2)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn2)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn3 + ((assign15250_e23340 * locals.var_t1_dn3) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn3)) + (((((-(assign15250_e23358 * locals.var_t1_dn3)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn3)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn4 + ((assign15250_e23340 * locals.var_t1_dn4) + (0.5 * (((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15250_e23348 * locals.var_t1_dn4)) + ((((((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15250_e23358 * locals.var_t1_dn4)) * assign15250_e23373) + (assign15250_e23363 * ((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15250_e23368 * locals.var_t1_dn4)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn5 + ((assign15250_e23340 * locals.var_t1_dn5) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn5)) + (((((-(assign15250_e23358 * locals.var_t1_dn5)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn5)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn6 + ((assign15250_e23340 * locals.var_t1_dn6) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn6)) + (((((-(assign15250_e23358 * locals.var_t1_dn6)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn6)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn7 + ((assign15250_e23340 * locals.var_t1_dn7) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn7)) + (((((-(assign15250_e23358 * locals.var_t1_dn7)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn7)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn8 + ((assign15250_e23340 * locals.var_t1_dn8) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn8)) + (((((-(assign15250_e23358 * locals.var_t1_dn8)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn8)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn9 + ((assign15250_e23340 * locals.var_t1_dn9) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn9)) + (((((-(assign15250_e23358 * locals.var_t1_dn9)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn9)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn10 + ((assign15250_e23340 * locals.var_t1_dn10) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn10)) + (((((-(assign15250_e23358 * locals.var_t1_dn10)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn10)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn11 + ((assign15250_e23340 * locals.var_t1_dn11) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn11)) + (((((-(assign15250_e23358 * locals.var_t1_dn11)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn11)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn13 + ((assign15250_e23340 * locals.var_t1_dn13) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn13)) + (((((-(assign15250_e23358 * locals.var_t1_dn13)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn13)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn14 + ((assign15250_e23340 * locals.var_t1_dn14) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn14)) + (((((-(assign15250_e23358 * locals.var_t1_dn14)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn14)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15250_e23384)))))),)
    } else {
        (locals.var_u0_v, locals.var_u0_v_dn0, locals.var_u0_v_dn2, locals.var_u0_v_dn3, locals.var_u0_v_dn4, locals.var_u0_v_dn5, locals.var_u0_v_dn6, locals.var_u0_v_dn7, locals.var_u0_v_dn8, locals.var_u0_v_dn9, locals.var_u0_v_dn10, locals.var_u0_v_dn11, locals.var_u0_v_dn13, locals.var_u0_v_dn14,)
    }
};
        locals.var_u0_v = assign15250_e23390;
        locals.var_u0_v_dn0 = assign15250_e23390_d_n0;
        locals.var_u0_v_dn2 = assign15250_e23390_d_n2;
        locals.var_u0_v_dn3 = assign15250_e23390_d_n3;
        locals.var_u0_v_dn4 = assign15250_e23390_d_n4;
        locals.var_u0_v_dn5 = assign15250_e23390_d_n5;
        locals.var_u0_v_dn6 = assign15250_e23390_d_n6;
        locals.var_u0_v_dn7 = assign15250_e23390_d_n7;
        locals.var_u0_v_dn8 = assign15250_e23390_d_n8;
        locals.var_u0_v_dn9 = assign15250_e23390_d_n9;
        locals.var_u0_v_dn10 = assign15250_e23390_d_n10;
        locals.var_u0_v_dn11 = assign15250_e23390_d_n11;
        locals.var_u0_v_dn13 = assign15250_e23390_d_n13;
        locals.var_u0_v_dn14 = assign15250_e23390_d_n14;
        locals.var_u0_v_rv = 0.0;

        let assign15260_e23393: f64 = if p.p66 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign15260_e23393;
        locals.var_guard260_rv = 0.0;

        let (assign15270_e23411, assign15270_e23411_d_n0, assign15270_e23411_d_n2, assign15270_e23411_d_n3, assign15270_e23411_d_n4, assign15270_e23411_d_n5, assign15270_e23411_d_n6, assign15270_e23411_d_n7, assign15270_e23411_d_n8, assign15270_e23411_d_n9, assign15270_e23411_d_n10, assign15270_e23411_d_n11, assign15270_e23411_d_n13, assign15270_e23411_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 != 0.0)) {
        let assign15270_e23404: f64 = (locals.var_ute1_i * locals.var_tratio);
        let assign15270_e23405: f64 = (locals.var_uter_i + assign15270_e23404);
        let assign15270_e23407: f64 = (assign15270_e23405 * locals.var_trat_ln);
        let assign15270_e23408: f64 = (assign15270_e23407).exp();
        let assign15270_e23409: f64 = (locals.var_u0r_i * assign15270_e23408);
        (assign15270_e23409, (locals.var_u0r_i_dn0 * assign15270_e23408), (locals.var_u0r_i_dn2 * assign15270_e23408), (locals.var_u0r_i_dn3 * assign15270_e23408), ((locals.var_u0r_i_dn4 * assign15270_e23408) + (locals.var_u0r_i * (assign15270_e23408 * (((locals.var_ute1_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15270_e23405 * locals.var_trat_ln_dn4))))), (locals.var_u0r_i_dn5 * assign15270_e23408), (locals.var_u0r_i_dn6 * assign15270_e23408), (locals.var_u0r_i_dn7 * assign15270_e23408), (locals.var_u0r_i_dn8 * assign15270_e23408), (locals.var_u0r_i_dn9 * assign15270_e23408), (locals.var_u0r_i_dn10 * assign15270_e23408), (locals.var_u0r_i_dn11 * assign15270_e23408), (locals.var_u0r_i_dn13 * assign15270_e23408), (locals.var_u0r_i_dn14 * assign15270_e23408),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15270_e23411;
        locals.var_t1_dn0 = assign15270_e23411_d_n0;
        locals.var_t1_dn2 = assign15270_e23411_d_n2;
        locals.var_t1_dn3 = assign15270_e23411_d_n3;
        locals.var_t1_dn4 = assign15270_e23411_d_n4;
        locals.var_t1_dn5 = assign15270_e23411_d_n5;
        locals.var_t1_dn6 = assign15270_e23411_d_n6;
        locals.var_t1_dn7 = assign15270_e23411_d_n7;
        locals.var_t1_dn8 = assign15270_e23411_d_n8;
        locals.var_t1_dn9 = assign15270_e23411_d_n9;
        locals.var_t1_dn10 = assign15270_e23411_d_n10;
        locals.var_t1_dn11 = assign15270_e23411_d_n11;
        locals.var_t1_dn13 = assign15270_e23411_d_n13;
        locals.var_t1_dn14 = assign15270_e23411_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15280_e23470, assign15280_e23470_d_n0, assign15280_e23470_d_n2, assign15280_e23470_d_n3, assign15280_e23470_d_n4, assign15280_e23470_d_n5, assign15280_e23470_d_n6, assign15280_e23470_d_n7, assign15280_e23470_d_n8, assign15280_e23470_d_n9, assign15280_e23470_d_n10, assign15280_e23470_d_n11, assign15280_e23470_d_n13, assign15280_e23470_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 != 0.0)) {
        let assign15280_e23420: f64 = (-0.9);
        let assign15280_e23422: f64 = (assign15280_e23420 * locals.var_t1);
        let assign15280_e23426: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign15280_e23428: f64 = (-0.9);
        let assign15280_e23430: f64 = (assign15280_e23428 * locals.var_t1);
        let assign15280_e23431: f64 = (assign15280_e23426 - assign15280_e23430);
        let assign15280_e23433: f64 = (assign15280_e23431 - 0.0001);
        let assign15280_e23436: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign15280_e23438: f64 = (-0.9);
        let assign15280_e23440: f64 = (assign15280_e23438 * locals.var_t1);
        let assign15280_e23441: f64 = (assign15280_e23436 - assign15280_e23440);
        let assign15280_e23443: f64 = (assign15280_e23441 - 0.0001);
        let assign15280_e23446: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign15280_e23448: f64 = (-0.9);
        let assign15280_e23450: f64 = (assign15280_e23448 * locals.var_t1);
        let assign15280_e23451: f64 = (assign15280_e23446 - assign15280_e23450);
        let assign15280_e23453: f64 = (assign15280_e23451 - 0.0001);
        let assign15280_e23454: f64 = (assign15280_e23443 * assign15280_e23453);
        let assign15280_e23457: f64 = (-0.9);
        let assign15280_e23459: f64 = (assign15280_e23457 * locals.var_t1);
        let assign15280_e23460: f64 = (4.0 * assign15280_e23459);
        let assign15280_e23462: f64 = (assign15280_e23460 * 0.0001);
        let assign15280_e23463: f64 = (assign15280_e23454 - assign15280_e23462);
        let assign15280_e23464: f64 = (assign15280_e23463).sqrt();
        let assign15280_e23465: f64 = (assign15280_e23433 + assign15280_e23464);
        let assign15280_e23466: f64 = (0.5 * assign15280_e23465);
        let assign15280_e23467: f64 = (assign15280_e23422 + assign15280_e23466);
        let assign15280_e23468: f64 = (locals.var_t1 + assign15280_e23467);
        (assign15280_e23468, (locals.var_t1_dn0 + ((assign15280_e23420 * locals.var_t1_dn0) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn0)) + (((((-(assign15280_e23438 * locals.var_t1_dn0)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn0)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn2 + ((assign15280_e23420 * locals.var_t1_dn2) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn2)) + (((((-(assign15280_e23438 * locals.var_t1_dn2)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn2)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn3 + ((assign15280_e23420 * locals.var_t1_dn3) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn3)) + (((((-(assign15280_e23438 * locals.var_t1_dn3)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn3)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn4 + ((assign15280_e23420 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign15280_e23428 * locals.var_t1_dn4)) + ((((((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign15280_e23438 * locals.var_t1_dn4)) * assign15280_e23453) + (assign15280_e23443 * ((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign15280_e23448 * locals.var_t1_dn4)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn5 + ((assign15280_e23420 * locals.var_t1_dn5) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn5)) + (((((-(assign15280_e23438 * locals.var_t1_dn5)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn5)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn6 + ((assign15280_e23420 * locals.var_t1_dn6) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn6)) + (((((-(assign15280_e23438 * locals.var_t1_dn6)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn6)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn7 + ((assign15280_e23420 * locals.var_t1_dn7) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn7)) + (((((-(assign15280_e23438 * locals.var_t1_dn7)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn7)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn8 + ((assign15280_e23420 * locals.var_t1_dn8) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn8)) + (((((-(assign15280_e23438 * locals.var_t1_dn8)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn8)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn9 + ((assign15280_e23420 * locals.var_t1_dn9) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn9)) + (((((-(assign15280_e23438 * locals.var_t1_dn9)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn9)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn10 + ((assign15280_e23420 * locals.var_t1_dn10) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn10)) + (((((-(assign15280_e23438 * locals.var_t1_dn10)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn10)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn11 + ((assign15280_e23420 * locals.var_t1_dn11) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn11)) + (((((-(assign15280_e23438 * locals.var_t1_dn11)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn11)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn13 + ((assign15280_e23420 * locals.var_t1_dn13) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn13)) + (((((-(assign15280_e23438 * locals.var_t1_dn13)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn13)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn14 + ((assign15280_e23420 * locals.var_t1_dn14) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn14)) + (((((-(assign15280_e23438 * locals.var_t1_dn14)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn14)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15280_e23464)))))),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    }
};
        locals.var_u0r_t = assign15280_e23470;
        locals.var_u0r_t_dn0 = assign15280_e23470_d_n0;
        locals.var_u0r_t_dn2 = assign15280_e23470_d_n2;
        locals.var_u0r_t_dn3 = assign15280_e23470_d_n3;
        locals.var_u0r_t_dn4 = assign15280_e23470_d_n4;
        locals.var_u0r_t_dn5 = assign15280_e23470_d_n5;
        locals.var_u0r_t_dn6 = assign15280_e23470_d_n6;
        locals.var_u0r_t_dn7 = assign15280_e23470_d_n7;
        locals.var_u0r_t_dn8 = assign15280_e23470_d_n8;
        locals.var_u0r_t_dn9 = assign15280_e23470_d_n9;
        locals.var_u0r_t_dn10 = assign15280_e23470_d_n10;
        locals.var_u0r_t_dn11 = assign15280_e23470_d_n11;
        locals.var_u0r_t_dn13 = assign15280_e23470_d_n13;
        locals.var_u0r_t_dn14 = assign15280_e23470_d_n14;
        locals.var_u0r_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_46(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15290_e23479, assign15290_e23479_d_n0, assign15290_e23479_d_n2, assign15290_e23479_d_n3, assign15290_e23479_d_n4, assign15290_e23479_d_n5, assign15290_e23479_d_n6, assign15290_e23479_d_n7, assign15290_e23479_d_n8, assign15290_e23479_d_n9, assign15290_e23479_d_n10, assign15290_e23479_d_n11, assign15290_e23479_d_n13, assign15290_e23479_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 != 0.0)) {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    } else {
        (locals.var_u0r_v, locals.var_u0r_v_dn0, locals.var_u0r_v_dn2, locals.var_u0r_v_dn3, locals.var_u0r_v_dn4, locals.var_u0r_v_dn5, locals.var_u0r_v_dn6, locals.var_u0r_v_dn7, locals.var_u0r_v_dn8, locals.var_u0r_v_dn9, locals.var_u0r_v_dn10, locals.var_u0r_v_dn11, locals.var_u0r_v_dn13, locals.var_u0r_v_dn14,)
    }
};
        locals.var_u0r_v = assign15290_e23479;
        locals.var_u0r_v_dn0 = assign15290_e23479_d_n0;
        locals.var_u0r_v_dn2 = assign15290_e23479_d_n2;
        locals.var_u0r_v_dn3 = assign15290_e23479_d_n3;
        locals.var_u0r_v_dn4 = assign15290_e23479_d_n4;
        locals.var_u0r_v_dn5 = assign15290_e23479_d_n5;
        locals.var_u0r_v_dn6 = assign15290_e23479_d_n6;
        locals.var_u0r_v_dn7 = assign15290_e23479_d_n7;
        locals.var_u0r_v_dn8 = assign15290_e23479_d_n8;
        locals.var_u0r_v_dn9 = assign15290_e23479_d_n9;
        locals.var_u0r_v_dn10 = assign15290_e23479_d_n10;
        locals.var_u0r_v_dn11 = assign15290_e23479_d_n11;
        locals.var_u0r_v_dn13 = assign15290_e23479_d_n13;
        locals.var_u0r_v_dn14 = assign15290_e23479_d_n14;
        locals.var_u0r_v_rv = 0.0;

        let (assign15300_e23495, assign15300_e23495_d_n0, assign15300_e23495_d_n2, assign15300_e23495_d_n3, assign15300_e23495_d_n4, assign15300_e23495_d_n5, assign15300_e23495_d_n6, assign15300_e23495_d_n7, assign15300_e23495_d_n8, assign15300_e23495_d_n9, assign15300_e23495_d_n10, assign15300_e23495_d_n11, assign15300_e23495_d_n13, assign15300_e23495_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15300_e23488: f64 = (locals.var_ua2_i * locals.var_tratio);
        let assign15300_e23489: f64 = (locals.var_ua1_i + assign15300_e23488);
        let assign15300_e23491: f64 = (assign15300_e23489 * locals.var_trat_ln);
        let assign15300_e23492: f64 = (assign15300_e23491).exp();
        let assign15300_e23493: f64 = (locals.var_ua_i * assign15300_e23492);
        (assign15300_e23493, (locals.var_ua_i_dn0 * assign15300_e23492), (locals.var_ua_i_dn2 * assign15300_e23492), (locals.var_ua_i_dn3 * assign15300_e23492), ((locals.var_ua_i_dn4 * assign15300_e23492) + (locals.var_ua_i * (assign15300_e23492 * (((locals.var_ua2_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15300_e23489 * locals.var_trat_ln_dn4))))), (locals.var_ua_i_dn5 * assign15300_e23492), (locals.var_ua_i_dn6 * assign15300_e23492), (locals.var_ua_i_dn7 * assign15300_e23492), (locals.var_ua_i_dn8 * assign15300_e23492), (locals.var_ua_i_dn9 * assign15300_e23492), (locals.var_ua_i_dn10 * assign15300_e23492), (locals.var_ua_i_dn11 * assign15300_e23492), (locals.var_ua_i_dn13 * assign15300_e23492), (locals.var_ua_i_dn14 * assign15300_e23492),)
    } else {
        (locals.var_ua_t, locals.var_ua_t_dn0, locals.var_ua_t_dn2, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11, locals.var_ua_t_dn13, locals.var_ua_t_dn14,)
    }
};
        locals.var_ua_t = assign15300_e23495;
        locals.var_ua_t_dn0 = assign15300_e23495_d_n0;
        locals.var_ua_t_dn2 = assign15300_e23495_d_n2;
        locals.var_ua_t_dn3 = assign15300_e23495_d_n3;
        locals.var_ua_t_dn4 = assign15300_e23495_d_n4;
        locals.var_ua_t_dn5 = assign15300_e23495_d_n5;
        locals.var_ua_t_dn6 = assign15300_e23495_d_n6;
        locals.var_ua_t_dn7 = assign15300_e23495_d_n7;
        locals.var_ua_t_dn8 = assign15300_e23495_d_n8;
        locals.var_ua_t_dn9 = assign15300_e23495_d_n9;
        locals.var_ua_t_dn10 = assign15300_e23495_d_n10;
        locals.var_ua_t_dn11 = assign15300_e23495_d_n11;
        locals.var_ua_t_dn13 = assign15300_e23495_d_n13;
        locals.var_ua_t_dn14 = assign15300_e23495_d_n14;
        locals.var_ua_t_rv = 0.0;

        let assign15310_e23498: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign15310_e23498;
        locals.var_guard261_rv = 0.0;

        let (assign15320_e23516, assign15320_e23516_d_n0, assign15320_e23516_d_n2, assign15320_e23516_d_n3, assign15320_e23516_d_n4, assign15320_e23516_d_n5, assign15320_e23516_d_n6, assign15320_e23516_d_n7, assign15320_e23516_d_n8, assign15320_e23516_d_n9, assign15320_e23516_d_n10, assign15320_e23516_d_n11, assign15320_e23516_d_n13, assign15320_e23516_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard261 != 0.0)) {
        let assign15320_e23509: f64 = (locals.var_ua2_i * locals.var_tratio);
        let assign15320_e23510: f64 = (locals.var_ua1r_i + assign15320_e23509);
        let assign15320_e23512: f64 = (assign15320_e23510 * locals.var_trat_ln);
        let assign15320_e23513: f64 = (assign15320_e23512).exp();
        let assign15320_e23514: f64 = (locals.var_uar_i * assign15320_e23513);
        (assign15320_e23514, (locals.var_uar_i_dn0 * assign15320_e23513), (locals.var_uar_i_dn2 * assign15320_e23513), (locals.var_uar_i_dn3 * assign15320_e23513), ((locals.var_uar_i_dn4 * assign15320_e23513) + (locals.var_uar_i * (assign15320_e23513 * (((locals.var_ua2_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15320_e23510 * locals.var_trat_ln_dn4))))), (locals.var_uar_i_dn5 * assign15320_e23513), (locals.var_uar_i_dn6 * assign15320_e23513), (locals.var_uar_i_dn7 * assign15320_e23513), (locals.var_uar_i_dn8 * assign15320_e23513), (locals.var_uar_i_dn9 * assign15320_e23513), (locals.var_uar_i_dn10 * assign15320_e23513), (locals.var_uar_i_dn11 * assign15320_e23513), (locals.var_uar_i_dn13 * assign15320_e23513), (locals.var_uar_i_dn14 * assign15320_e23513),)
    } else {
        (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn13, locals.var_uar_t_dn14,)
    }
};
        locals.var_uar_t = assign15320_e23516;
        locals.var_uar_t_dn0 = assign15320_e23516_d_n0;
        locals.var_uar_t_dn2 = assign15320_e23516_d_n2;
        locals.var_uar_t_dn3 = assign15320_e23516_d_n3;
        locals.var_uar_t_dn4 = assign15320_e23516_d_n4;
        locals.var_uar_t_dn5 = assign15320_e23516_d_n5;
        locals.var_uar_t_dn6 = assign15320_e23516_d_n6;
        locals.var_uar_t_dn7 = assign15320_e23516_d_n7;
        locals.var_uar_t_dn8 = assign15320_e23516_d_n8;
        locals.var_uar_t_dn9 = assign15320_e23516_d_n9;
        locals.var_uar_t_dn10 = assign15320_e23516_d_n10;
        locals.var_uar_t_dn11 = assign15320_e23516_d_n11;
        locals.var_uar_t_dn13 = assign15320_e23516_d_n13;
        locals.var_uar_t_dn14 = assign15320_e23516_d_n14;
        locals.var_uar_t_rv = 0.0;

        let (assign15330_e23532, assign15330_e23532_d_n0, assign15330_e23532_d_n2, assign15330_e23532_d_n3, assign15330_e23532_d_n4, assign15330_e23532_d_n5, assign15330_e23532_d_n6, assign15330_e23532_d_n7, assign15330_e23532_d_n8, assign15330_e23532_d_n9, assign15330_e23532_d_n10, assign15330_e23532_d_n11, assign15330_e23532_d_n13, assign15330_e23532_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15330_e23525: f64 = (locals.var_ud2_i * locals.var_tratio);
        let assign15330_e23526: f64 = (locals.var_ud1_i + assign15330_e23525);
        let assign15330_e23528: f64 = (assign15330_e23526 * locals.var_trat_ln);
        let assign15330_e23529: f64 = (assign15330_e23528).exp();
        let assign15330_e23530: f64 = (locals.var_ud_i * assign15330_e23529);
        (assign15330_e23530, (locals.var_ud_i_dn0 * assign15330_e23529), (locals.var_ud_i_dn2 * assign15330_e23529), (locals.var_ud_i_dn3 * assign15330_e23529), ((locals.var_ud_i_dn4 * assign15330_e23529) + (locals.var_ud_i * (assign15330_e23529 * (((locals.var_ud2_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15330_e23526 * locals.var_trat_ln_dn4))))), (locals.var_ud_i_dn5 * assign15330_e23529), (locals.var_ud_i_dn6 * assign15330_e23529), (locals.var_ud_i_dn7 * assign15330_e23529), (locals.var_ud_i_dn8 * assign15330_e23529), (locals.var_ud_i_dn9 * assign15330_e23529), (locals.var_ud_i_dn10 * assign15330_e23529), (locals.var_ud_i_dn11 * assign15330_e23529), (locals.var_ud_i_dn13 * assign15330_e23529), (locals.var_ud_i_dn14 * assign15330_e23529),)
    } else {
        (locals.var_ud_t, locals.var_ud_t_dn0, locals.var_ud_t_dn2, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11, locals.var_ud_t_dn13, locals.var_ud_t_dn14,)
    }
};
        locals.var_ud_t = assign15330_e23532;
        locals.var_ud_t_dn0 = assign15330_e23532_d_n0;
        locals.var_ud_t_dn2 = assign15330_e23532_d_n2;
        locals.var_ud_t_dn3 = assign15330_e23532_d_n3;
        locals.var_ud_t_dn4 = assign15330_e23532_d_n4;
        locals.var_ud_t_dn5 = assign15330_e23532_d_n5;
        locals.var_ud_t_dn6 = assign15330_e23532_d_n6;
        locals.var_ud_t_dn7 = assign15330_e23532_d_n7;
        locals.var_ud_t_dn8 = assign15330_e23532_d_n8;
        locals.var_ud_t_dn9 = assign15330_e23532_d_n9;
        locals.var_ud_t_dn10 = assign15330_e23532_d_n10;
        locals.var_ud_t_dn11 = assign15330_e23532_d_n11;
        locals.var_ud_t_dn13 = assign15330_e23532_d_n13;
        locals.var_ud_t_dn14 = assign15330_e23532_d_n14;
        locals.var_ud_t_rv = 0.0;

        let assign15340_e23535: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign15340_e23535;
        locals.var_guard262_rv = 0.0;

        let (assign15350_e23553, assign15350_e23553_d_n0, assign15350_e23553_d_n2, assign15350_e23553_d_n3, assign15350_e23553_d_n4, assign15350_e23553_d_n5, assign15350_e23553_d_n6, assign15350_e23553_d_n7, assign15350_e23553_d_n8, assign15350_e23553_d_n9, assign15350_e23553_d_n10, assign15350_e23553_d_n11, assign15350_e23553_d_n13, assign15350_e23553_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard262 != 0.0)) {
        let assign15350_e23546: f64 = (locals.var_ud2_i * locals.var_tratio);
        let assign15350_e23547: f64 = (locals.var_ud1r_i + assign15350_e23546);
        let assign15350_e23549: f64 = (assign15350_e23547 * locals.var_trat_ln);
        let assign15350_e23550: f64 = (assign15350_e23549).exp();
        let assign15350_e23551: f64 = (locals.var_udr_i * assign15350_e23550);
        (assign15350_e23551, (locals.var_udr_i_dn0 * assign15350_e23550), (locals.var_udr_i_dn2 * assign15350_e23550), (locals.var_udr_i_dn3 * assign15350_e23550), ((locals.var_udr_i_dn4 * assign15350_e23550) + (locals.var_udr_i * (assign15350_e23550 * (((locals.var_ud2_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15350_e23547 * locals.var_trat_ln_dn4))))), (locals.var_udr_i_dn5 * assign15350_e23550), (locals.var_udr_i_dn6 * assign15350_e23550), (locals.var_udr_i_dn7 * assign15350_e23550), (locals.var_udr_i_dn8 * assign15350_e23550), (locals.var_udr_i_dn9 * assign15350_e23550), (locals.var_udr_i_dn10 * assign15350_e23550), (locals.var_udr_i_dn11 * assign15350_e23550), (locals.var_udr_i_dn13 * assign15350_e23550), (locals.var_udr_i_dn14 * assign15350_e23550),)
    } else {
        (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn13, locals.var_udr_t_dn14,)
    }
};
        locals.var_udr_t = assign15350_e23553;
        locals.var_udr_t_dn0 = assign15350_e23553_d_n0;
        locals.var_udr_t_dn2 = assign15350_e23553_d_n2;
        locals.var_udr_t_dn3 = assign15350_e23553_d_n3;
        locals.var_udr_t_dn4 = assign15350_e23553_d_n4;
        locals.var_udr_t_dn5 = assign15350_e23553_d_n5;
        locals.var_udr_t_dn6 = assign15350_e23553_d_n6;
        locals.var_udr_t_dn7 = assign15350_e23553_d_n7;
        locals.var_udr_t_dn8 = assign15350_e23553_d_n8;
        locals.var_udr_t_dn9 = assign15350_e23553_d_n9;
        locals.var_udr_t_dn10 = assign15350_e23553_d_n10;
        locals.var_udr_t_dn11 = assign15350_e23553_d_n11;
        locals.var_udr_t_dn13 = assign15350_e23553_d_n13;
        locals.var_udr_t_dn14 = assign15350_e23553_d_n14;
        locals.var_udr_t_rv = 0.0;

        let (assign15360_e23569, assign15360_e23569_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15360_e23562: f64 = (p.p881 * locals.var_tratio);
        let assign15360_e23563: f64 = (locals.var_ucste_i + assign15360_e23562);
        let assign15360_e23565: f64 = (assign15360_e23563 * locals.var_trat_ln);
        let assign15360_e23566: f64 = (assign15360_e23565).exp();
        let assign15360_e23567: f64 = (locals.var_ucs_i * assign15360_e23566);
        (assign15360_e23567, (locals.var_ucs_i * (assign15360_e23566 * (((p.p881 * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15360_e23563 * locals.var_trat_ln_dn4)))),)
    } else {
        (locals.var_ucs_t, locals.var_ucs_t_dn4,)
    }
};
        locals.var_ucs_t = assign15360_e23569;
        locals.var_ucs_t_dn4 = assign15360_e23569_d_n4;
        locals.var_ucs_t_rv = 0.0;

        let (assign15370_e23583, assign15370_e23583_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15370_e23577: f64 = (locals.var_uds1_i * locals.var_tratio_m1);
        let assign15370_e23578: f64 = { let limited_exp_arg = assign15370_e23577; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15370_e23580: f64 = (assign15370_e23578 - 1.0);
        let assign15370_e23581: f64 = (locals.var_uds_i * assign15370_e23580);
        (assign15370_e23581, (locals.var_uds_i * ({ let limited_exp_arg = assign15370_e23577; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_uds1_i * locals.var_tratio_m1_dn4))),)
    } else {
        (locals.var_uds_t, locals.var_uds_t_dn4,)
    }
};
        locals.var_uds_t = assign15370_e23583;
        locals.var_uds_t_dn4 = assign15370_e23583_d_n4;
        locals.var_uds_t_rv = 0.0;

        let (assign15380_e23597, assign15380_e23597_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15380_e23591: f64 = (locals.var_udd1_i * locals.var_tratio_m1);
        let assign15380_e23592: f64 = { let limited_exp_arg = assign15380_e23591; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15380_e23594: f64 = (assign15380_e23592 - 1.0);
        let assign15380_e23595: f64 = (locals.var_udd_i * assign15380_e23594);
        (assign15380_e23595, (locals.var_udd_i * ({ let limited_exp_arg = assign15380_e23591; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_udd1_i * locals.var_tratio_m1_dn4))),)
    } else {
        (locals.var_udd_t, locals.var_udd_t_dn4,)
    }
};
        locals.var_udd_t = assign15380_e23597;
        locals.var_udd_t_dn4 = assign15380_e23597_d_n4;
        locals.var_udd_t_rv = 0.0;

        let (assign15390_e23606, assign15390_e23606_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15390_e23604: f64 = (0.5 + locals.var_uds_t);
        (assign15390_e23604, locals.var_uds_t_dn4,)
    } else {
        (locals.var_udseff_t, locals.var_udseff_t_dn4,)
    }
};
        locals.var_udseff_t = assign15390_e23606;
        locals.var_udseff_t_dn4 = assign15390_e23606_d_n4;
        locals.var_udseff_t_rv = 0.0;

        let (assign15400_e23615, assign15400_e23615_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15400_e23613: f64 = (0.5 + locals.var_udd_t);
        (assign15400_e23613, locals.var_udd_t_dn4,)
    } else {
        (locals.var_uddeff_t, locals.var_uddeff_t_dn4,)
    }
};
        locals.var_uddeff_t = assign15400_e23615;
        locals.var_uddeff_t_dn4 = assign15400_e23615_d_n4;
        locals.var_uddeff_t_rv = 0.0;

        let assign15410_e23618: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign15410_e23618;
        locals.var_guard263_rv = 0.0;

        let (assign15420_e23667, assign15420_e23667_d_n0, assign15420_e23667_d_n2, assign15420_e23667_d_n3, assign15420_e23667_d_n4, assign15420_e23667_d_n5, assign15420_e23667_d_n6, assign15420_e23667_d_n7, assign15420_e23667_d_n8, assign15420_e23667_d_n9, assign15420_e23667_d_n10, assign15420_e23667_d_n11, assign15420_e23667_d_n13, assign15420_e23667_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard263 != 0.0)) {
        let assign15420_e23627: f64 = (-locals.var_eu_i);
        let assign15420_e23631: f64 = (locals.var_eu1_i * locals.var_deltemp);
        let assign15420_e23633: f64 = (-locals.var_eu_i);
        let assign15420_e23634: f64 = (assign15420_e23631 - assign15420_e23633);
        let assign15420_e23636: f64 = (assign15420_e23634 - 1e-6);
        let assign15420_e23639: f64 = (locals.var_eu1_i * locals.var_deltemp);
        let assign15420_e23641: f64 = (-locals.var_eu_i);
        let assign15420_e23642: f64 = (assign15420_e23639 - assign15420_e23641);
        let assign15420_e23644: f64 = (assign15420_e23642 - 1e-6);
        let assign15420_e23647: f64 = (locals.var_eu1_i * locals.var_deltemp);
        let assign15420_e23649: f64 = (-locals.var_eu_i);
        let assign15420_e23650: f64 = (assign15420_e23647 - assign15420_e23649);
        let assign15420_e23652: f64 = (assign15420_e23650 - 1e-6);
        let assign15420_e23653: f64 = (assign15420_e23644 * assign15420_e23652);
        let assign15420_e23656: f64 = (-locals.var_eu_i);
        let assign15420_e23657: f64 = (4.0 * assign15420_e23656);
        let assign15420_e23659: f64 = (assign15420_e23657 * 1e-6);
        let assign15420_e23660: f64 = (assign15420_e23653 - assign15420_e23659);
        let assign15420_e23661: f64 = (assign15420_e23660).sqrt();
        let assign15420_e23662: f64 = (assign15420_e23636 + assign15420_e23661);
        let assign15420_e23663: f64 = (0.5 * assign15420_e23662);
        let assign15420_e23664: f64 = (assign15420_e23627 + assign15420_e23663);
        let assign15420_e23665: f64 = (locals.var_eu_i + assign15420_e23664);
        (assign15420_e23665, (locals.var_eu_i_dn0 + ((-locals.var_eu_i_dn0) + (0.5 * ((-(-locals.var_eu_i_dn0)) + (((((-(-locals.var_eu_i_dn0)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn0)))) - ((4.0 * (-locals.var_eu_i_dn0)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn2 + ((-locals.var_eu_i_dn2) + (0.5 * ((-(-locals.var_eu_i_dn2)) + (((((-(-locals.var_eu_i_dn2)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn2)))) - ((4.0 * (-locals.var_eu_i_dn2)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn3 + ((-locals.var_eu_i_dn3) + (0.5 * ((-(-locals.var_eu_i_dn3)) + (((((-(-locals.var_eu_i_dn3)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn3)))) - ((4.0 * (-locals.var_eu_i_dn3)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn4 + ((-locals.var_eu_i_dn4) + (0.5 * (((locals.var_eu1_i * locals.var_deltemp_dn4) - (-locals.var_eu_i_dn4)) + ((((((locals.var_eu1_i * locals.var_deltemp_dn4) - (-locals.var_eu_i_dn4)) * assign15420_e23652) + (assign15420_e23644 * ((locals.var_eu1_i * locals.var_deltemp_dn4) - (-locals.var_eu_i_dn4)))) - ((4.0 * (-locals.var_eu_i_dn4)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn5 + ((-locals.var_eu_i_dn5) + (0.5 * ((-(-locals.var_eu_i_dn5)) + (((((-(-locals.var_eu_i_dn5)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn5)))) - ((4.0 * (-locals.var_eu_i_dn5)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn6 + ((-locals.var_eu_i_dn6) + (0.5 * ((-(-locals.var_eu_i_dn6)) + (((((-(-locals.var_eu_i_dn6)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn6)))) - ((4.0 * (-locals.var_eu_i_dn6)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn7 + ((-locals.var_eu_i_dn7) + (0.5 * ((-(-locals.var_eu_i_dn7)) + (((((-(-locals.var_eu_i_dn7)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn7)))) - ((4.0 * (-locals.var_eu_i_dn7)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn8 + ((-locals.var_eu_i_dn8) + (0.5 * ((-(-locals.var_eu_i_dn8)) + (((((-(-locals.var_eu_i_dn8)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn8)))) - ((4.0 * (-locals.var_eu_i_dn8)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn9 + ((-locals.var_eu_i_dn9) + (0.5 * ((-(-locals.var_eu_i_dn9)) + (((((-(-locals.var_eu_i_dn9)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn9)))) - ((4.0 * (-locals.var_eu_i_dn9)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn10 + ((-locals.var_eu_i_dn10) + (0.5 * ((-(-locals.var_eu_i_dn10)) + (((((-(-locals.var_eu_i_dn10)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn10)))) - ((4.0 * (-locals.var_eu_i_dn10)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn11 + ((-locals.var_eu_i_dn11) + (0.5 * ((-(-locals.var_eu_i_dn11)) + (((((-(-locals.var_eu_i_dn11)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn11)))) - ((4.0 * (-locals.var_eu_i_dn11)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn13 + ((-locals.var_eu_i_dn13) + (0.5 * ((-(-locals.var_eu_i_dn13)) + (((((-(-locals.var_eu_i_dn13)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn13)))) - ((4.0 * (-locals.var_eu_i_dn13)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn14 + ((-locals.var_eu_i_dn14) + (0.5 * ((-(-locals.var_eu_i_dn14)) + (((((-(-locals.var_eu_i_dn14)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn14)))) - ((4.0 * (-locals.var_eu_i_dn14)) * 1e-6)) / (2.0 * assign15420_e23661)))))),)
    } else {
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn13, locals.var_eu_t_dn14,)
    }
};
        locals.var_eu_t = assign15420_e23667;
        locals.var_eu_t_dn0 = assign15420_e23667_d_n0;
        locals.var_eu_t_dn2 = assign15420_e23667_d_n2;
        locals.var_eu_t_dn3 = assign15420_e23667_d_n3;
        locals.var_eu_t_dn4 = assign15420_e23667_d_n4;
        locals.var_eu_t_dn5 = assign15420_e23667_d_n5;
        locals.var_eu_t_dn6 = assign15420_e23667_d_n6;
        locals.var_eu_t_dn7 = assign15420_e23667_d_n7;
        locals.var_eu_t_dn8 = assign15420_e23667_d_n8;
        locals.var_eu_t_dn9 = assign15420_e23667_d_n9;
        locals.var_eu_t_dn10 = assign15420_e23667_d_n10;
        locals.var_eu_t_dn11 = assign15420_e23667_d_n11;
        locals.var_eu_t_dn13 = assign15420_e23667_d_n13;
        locals.var_eu_t_dn14 = assign15420_e23667_d_n14;
        locals.var_eu_t_rv = 0.0;

        let (assign15430_e23750, assign15430_e23750_d_n0, assign15430_e23750_d_n2, assign15430_e23750_d_n3, assign15430_e23750_d_n4, assign15430_e23750_d_n5, assign15430_e23750_d_n6, assign15430_e23750_d_n7, assign15430_e23750_d_n8, assign15430_e23750_d_n9, assign15430_e23750_d_n10, assign15430_e23750_d_n11, assign15430_e23750_d_n13, assign15430_e23750_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard263 == 0.0)) {
        let assign15430_e23679: f64 = (locals.var_eu1_i * locals.var_deltemp);
        let assign15430_e23680: f64 = (1.0 + assign15430_e23679);
        let assign15430_e23682: f64 = (assign15430_e23680 - 1e-6);
        let assign15430_e23684: f64 = (-10000.0);
        let assign15430_e23686: f64 = (assign15430_e23684 * 0.001);
        let (assign15430_e23747, assign15430_e23747_d_n4,) = {
            if (!(assign15430_e23682 < assign15430_e23686)) {
                let assign15430_e23693: f64 = (locals.var_eu1_i * locals.var_deltemp);
                let assign15430_e23694: f64 = (1.0 + assign15430_e23693);
                let assign15430_e23696: f64 = (assign15430_e23694 - 1e-6);
                let assign15430_e23700: f64 = (locals.var_eu1_i * locals.var_deltemp);
                let assign15430_e23701: f64 = (1.0 + assign15430_e23700);
                let assign15430_e23703: f64 = (assign15430_e23701 - 1e-6);
                let assign15430_e23707: f64 = (locals.var_eu1_i * locals.var_deltemp);
                let assign15430_e23708: f64 = (1.0 + assign15430_e23707);
                let assign15430_e23710: f64 = (assign15430_e23708 - 1e-6);
                let assign15430_e23711: f64 = (assign15430_e23703 * assign15430_e23710);
                let assign15430_e23714: f64 = (4.0 * 0.001);
                let assign15430_e23716: f64 = (assign15430_e23714 * 0.001);
                let assign15430_e23717: f64 = (assign15430_e23711 + assign15430_e23716);
                let assign15430_e23718: f64 = (assign15430_e23717).sqrt();
                let assign15430_e23719: f64 = (assign15430_e23696 + assign15430_e23718);
                let assign15430_e23720: f64 = (0.5 * assign15430_e23719);
                (assign15430_e23720, (0.5 * ((locals.var_eu1_i * locals.var_deltemp_dn4) + ((((locals.var_eu1_i * locals.var_deltemp_dn4) * assign15430_e23710) + (assign15430_e23703 * (locals.var_eu1_i * locals.var_deltemp_dn4))) / (2.0 * assign15430_e23718)))),)
            } else {
                let assign15430_e23724: f64 = (locals.var_eu1_i * locals.var_deltemp);
                let assign15430_e23725: f64 = (1.0 + assign15430_e23724);
                let assign15430_e23727: f64 = (assign15430_e23725 - 1e-6);
                let assign15430_e23729: f64 = (-10000.0);
                let assign15430_e23731: f64 = (assign15430_e23729 * 0.001);
                let (assign15430_e23746, assign15430_e23746_d_n4,) = {
                    if (assign15430_e23727 < assign15430_e23731) {
                        let assign15430_e23734: f64 = (-0.001);
                        let assign15430_e23736: f64 = (assign15430_e23734 * 0.001);
                        let assign15430_e23740: f64 = (locals.var_eu1_i * locals.var_deltemp);
                        let assign15430_e23741: f64 = (1.0 + assign15430_e23740);
                        let assign15430_e23743: f64 = (assign15430_e23741 - 1e-6);
                        let assign15430_e23744: f64 = (assign15430_e23736 / assign15430_e23743);
                        (assign15430_e23744, (-((assign15430_e23736 * (locals.var_eu1_i * locals.var_deltemp_dn4)) / (assign15430_e23743 * assign15430_e23743))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15430_e23746, assign15430_e23746_d_n4,)
            }
        };
        let assign15430_e23748: f64 = (locals.var_eu_i * assign15430_e23747);
        (assign15430_e23748, (locals.var_eu_i_dn0 * assign15430_e23747), (locals.var_eu_i_dn2 * assign15430_e23747), (locals.var_eu_i_dn3 * assign15430_e23747), ((locals.var_eu_i_dn4 * assign15430_e23747) + (locals.var_eu_i * assign15430_e23747_d_n4)), (locals.var_eu_i_dn5 * assign15430_e23747), (locals.var_eu_i_dn6 * assign15430_e23747), (locals.var_eu_i_dn7 * assign15430_e23747), (locals.var_eu_i_dn8 * assign15430_e23747), (locals.var_eu_i_dn9 * assign15430_e23747), (locals.var_eu_i_dn10 * assign15430_e23747), (locals.var_eu_i_dn11 * assign15430_e23747), (locals.var_eu_i_dn13 * assign15430_e23747), (locals.var_eu_i_dn14 * assign15430_e23747),)
    } else {
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn13, locals.var_eu_t_dn14,)
    }
};
        locals.var_eu_t = assign15430_e23750;
        locals.var_eu_t_dn0 = assign15430_e23750_d_n0;
        locals.var_eu_t_dn2 = assign15430_e23750_d_n2;
        locals.var_eu_t_dn3 = assign15430_e23750_d_n3;
        locals.var_eu_t_dn4 = assign15430_e23750_d_n4;
        locals.var_eu_t_dn5 = assign15430_e23750_d_n5;
        locals.var_eu_t_dn6 = assign15430_e23750_d_n6;
        locals.var_eu_t_dn7 = assign15430_e23750_d_n7;
        locals.var_eu_t_dn8 = assign15430_e23750_d_n8;
        locals.var_eu_t_dn9 = assign15430_e23750_d_n9;
        locals.var_eu_t_dn10 = assign15430_e23750_d_n10;
        locals.var_eu_t_dn11 = assign15430_e23750_d_n11;
        locals.var_eu_t_dn13 = assign15430_e23750_d_n13;
        locals.var_eu_t_dn14 = assign15430_e23750_d_n14;
        locals.var_eu_t_rv = 0.0;

        let assign15440_e23753: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign15440_e23753;
        locals.var_guard264_rv = 0.0;

        let (assign15450_e23771, assign15450_e23771_d_n0, assign15450_e23771_d_n2, assign15450_e23771_d_n3, assign15450_e23771_d_n4, assign15450_e23771_d_n5, assign15450_e23771_d_n6, assign15450_e23771_d_n7, assign15450_e23771_d_n8, assign15450_e23771_d_n9, assign15450_e23771_d_n10, assign15450_e23771_d_n11, assign15450_e23771_d_n13, assign15450_e23771_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard264 != 0.0)) {
        let assign15450_e23764: f64 = (locals.var_ute1cv_i * locals.var_tratio);
        let assign15450_e23765: f64 = (locals.var_utecv_i + assign15450_e23764);
        let assign15450_e23767: f64 = (assign15450_e23765 * locals.var_trat_ln);
        let assign15450_e23768: f64 = (assign15450_e23767).exp();
        let assign15450_e23769: f64 = (locals.var_u0cv_i * assign15450_e23768);
        (assign15450_e23769, (locals.var_u0cv_i_dn0 * assign15450_e23768), (locals.var_u0cv_i_dn2 * assign15450_e23768), (locals.var_u0cv_i_dn3 * assign15450_e23768), ((locals.var_u0cv_i_dn4 * assign15450_e23768) + (locals.var_u0cv_i * (assign15450_e23768 * (((locals.var_ute1cv_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15450_e23765 * locals.var_trat_ln_dn4))))), (locals.var_u0cv_i_dn5 * assign15450_e23768), (locals.var_u0cv_i_dn6 * assign15450_e23768), (locals.var_u0cv_i_dn7 * assign15450_e23768), (locals.var_u0cv_i_dn8 * assign15450_e23768), (locals.var_u0cv_i_dn9 * assign15450_e23768), (locals.var_u0cv_i_dn10 * assign15450_e23768), (locals.var_u0cv_i_dn11 * assign15450_e23768), (locals.var_u0cv_i_dn13 * assign15450_e23768), (locals.var_u0cv_i_dn14 * assign15450_e23768),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15450_e23771;
        locals.var_t1_dn0 = assign15450_e23771_d_n0;
        locals.var_t1_dn2 = assign15450_e23771_d_n2;
        locals.var_t1_dn3 = assign15450_e23771_d_n3;
        locals.var_t1_dn4 = assign15450_e23771_d_n4;
        locals.var_t1_dn5 = assign15450_e23771_d_n5;
        locals.var_t1_dn6 = assign15450_e23771_d_n6;
        locals.var_t1_dn7 = assign15450_e23771_d_n7;
        locals.var_t1_dn8 = assign15450_e23771_d_n8;
        locals.var_t1_dn9 = assign15450_e23771_d_n9;
        locals.var_t1_dn10 = assign15450_e23771_d_n10;
        locals.var_t1_dn11 = assign15450_e23771_d_n11;
        locals.var_t1_dn13 = assign15450_e23771_d_n13;
        locals.var_t1_dn14 = assign15450_e23771_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15460_e23830, assign15460_e23830_d_n0, assign15460_e23830_d_n2, assign15460_e23830_d_n3, assign15460_e23830_d_n4, assign15460_e23830_d_n5, assign15460_e23830_d_n6, assign15460_e23830_d_n7, assign15460_e23830_d_n8, assign15460_e23830_d_n9, assign15460_e23830_d_n10, assign15460_e23830_d_n11, assign15460_e23830_d_n13, assign15460_e23830_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard264 != 0.0)) {
        let assign15460_e23780: f64 = (-0.9);
        let assign15460_e23782: f64 = (assign15460_e23780 * locals.var_t1);
        let assign15460_e23786: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15460_e23788: f64 = (-0.9);
        let assign15460_e23790: f64 = (assign15460_e23788 * locals.var_t1);
        let assign15460_e23791: f64 = (assign15460_e23786 - assign15460_e23790);
        let assign15460_e23793: f64 = (assign15460_e23791 - 0.0001);
        let assign15460_e23796: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15460_e23798: f64 = (-0.9);
        let assign15460_e23800: f64 = (assign15460_e23798 * locals.var_t1);
        let assign15460_e23801: f64 = (assign15460_e23796 - assign15460_e23800);
        let assign15460_e23803: f64 = (assign15460_e23801 - 0.0001);
        let assign15460_e23806: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15460_e23808: f64 = (-0.9);
        let assign15460_e23810: f64 = (assign15460_e23808 * locals.var_t1);
        let assign15460_e23811: f64 = (assign15460_e23806 - assign15460_e23810);
        let assign15460_e23813: f64 = (assign15460_e23811 - 0.0001);
        let assign15460_e23814: f64 = (assign15460_e23803 * assign15460_e23813);
        let assign15460_e23817: f64 = (-0.9);
        let assign15460_e23819: f64 = (assign15460_e23817 * locals.var_t1);
        let assign15460_e23820: f64 = (4.0 * assign15460_e23819);
        let assign15460_e23822: f64 = (assign15460_e23820 * 0.0001);
        let assign15460_e23823: f64 = (assign15460_e23814 - assign15460_e23822);
        let assign15460_e23824: f64 = (assign15460_e23823).sqrt();
        let assign15460_e23825: f64 = (assign15460_e23793 + assign15460_e23824);
        let assign15460_e23826: f64 = (0.5 * assign15460_e23825);
        let assign15460_e23827: f64 = (assign15460_e23782 + assign15460_e23826);
        let assign15460_e23828: f64 = (locals.var_t1 + assign15460_e23827);
        (assign15460_e23828, (locals.var_t1_dn0 + ((assign15460_e23780 * locals.var_t1_dn0) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn0)) + (((((-(assign15460_e23798 * locals.var_t1_dn0)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn0)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn2 + ((assign15460_e23780 * locals.var_t1_dn2) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn2)) + (((((-(assign15460_e23798 * locals.var_t1_dn2)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn2)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn3 + ((assign15460_e23780 * locals.var_t1_dn3) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn3)) + (((((-(assign15460_e23798 * locals.var_t1_dn3)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn3)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn4 + ((assign15460_e23780 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15460_e23788 * locals.var_t1_dn4)) + ((((((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15460_e23798 * locals.var_t1_dn4)) * assign15460_e23813) + (assign15460_e23803 * ((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15460_e23808 * locals.var_t1_dn4)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn5 + ((assign15460_e23780 * locals.var_t1_dn5) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn5)) + (((((-(assign15460_e23798 * locals.var_t1_dn5)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn5)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn6 + ((assign15460_e23780 * locals.var_t1_dn6) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn6)) + (((((-(assign15460_e23798 * locals.var_t1_dn6)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn6)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn7 + ((assign15460_e23780 * locals.var_t1_dn7) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn7)) + (((((-(assign15460_e23798 * locals.var_t1_dn7)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn7)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn8 + ((assign15460_e23780 * locals.var_t1_dn8) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn8)) + (((((-(assign15460_e23798 * locals.var_t1_dn8)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn8)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn9 + ((assign15460_e23780 * locals.var_t1_dn9) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn9)) + (((((-(assign15460_e23798 * locals.var_t1_dn9)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn9)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn10 + ((assign15460_e23780 * locals.var_t1_dn10) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn10)) + (((((-(assign15460_e23798 * locals.var_t1_dn10)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn10)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn11 + ((assign15460_e23780 * locals.var_t1_dn11) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn11)) + (((((-(assign15460_e23798 * locals.var_t1_dn11)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn11)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn13 + ((assign15460_e23780 * locals.var_t1_dn13) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn13)) + (((((-(assign15460_e23798 * locals.var_t1_dn13)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn13)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn14 + ((assign15460_e23780 * locals.var_t1_dn14) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn14)) + (((((-(assign15460_e23798 * locals.var_t1_dn14)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn14)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15460_e23824)))))),)
    } else {
        (locals.var_u0_cv, locals.var_u0_cv_dn0, locals.var_u0_cv_dn2, locals.var_u0_cv_dn3, locals.var_u0_cv_dn4, locals.var_u0_cv_dn5, locals.var_u0_cv_dn6, locals.var_u0_cv_dn7, locals.var_u0_cv_dn8, locals.var_u0_cv_dn9, locals.var_u0_cv_dn10, locals.var_u0_cv_dn11, locals.var_u0_cv_dn13, locals.var_u0_cv_dn14,)
    }
};
        locals.var_u0_cv = assign15460_e23830;
        locals.var_u0_cv_dn0 = assign15460_e23830_d_n0;
        locals.var_u0_cv_dn2 = assign15460_e23830_d_n2;
        locals.var_u0_cv_dn3 = assign15460_e23830_d_n3;
        locals.var_u0_cv_dn4 = assign15460_e23830_d_n4;
        locals.var_u0_cv_dn5 = assign15460_e23830_d_n5;
        locals.var_u0_cv_dn6 = assign15460_e23830_d_n6;
        locals.var_u0_cv_dn7 = assign15460_e23830_d_n7;
        locals.var_u0_cv_dn8 = assign15460_e23830_d_n8;
        locals.var_u0_cv_dn9 = assign15460_e23830_d_n9;
        locals.var_u0_cv_dn10 = assign15460_e23830_d_n10;
        locals.var_u0_cv_dn11 = assign15460_e23830_d_n11;
        locals.var_u0_cv_dn13 = assign15460_e23830_d_n13;
        locals.var_u0_cv_dn14 = assign15460_e23830_d_n14;
        locals.var_u0_cv_rv = 0.0;

        let (assign15470_e23848, assign15470_e23848_d_n0, assign15470_e23848_d_n2, assign15470_e23848_d_n3, assign15470_e23848_d_n4, assign15470_e23848_d_n5, assign15470_e23848_d_n6, assign15470_e23848_d_n7, assign15470_e23848_d_n8, assign15470_e23848_d_n9, assign15470_e23848_d_n10, assign15470_e23848_d_n11, assign15470_e23848_d_n13, assign15470_e23848_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard264 != 0.0)) {
        let assign15470_e23841: f64 = (locals.var_ua2cv_i * locals.var_tratio);
        let assign15470_e23842: f64 = (locals.var_ua1cv_i + assign15470_e23841);
        let assign15470_e23844: f64 = (assign15470_e23842 * locals.var_trat_ln);
        let assign15470_e23845: f64 = (assign15470_e23844).exp();
        let assign15470_e23846: f64 = (locals.var_uacv_i * assign15470_e23845);
        (assign15470_e23846, 0.0, 0.0, 0.0, (locals.var_uacv_i * (assign15470_e23845 * (((locals.var_ua2cv_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15470_e23842 * locals.var_trat_ln_dn4)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uacv_t, locals.var_uacv_t_dn0, locals.var_uacv_t_dn2, locals.var_uacv_t_dn3, locals.var_uacv_t_dn4, locals.var_uacv_t_dn5, locals.var_uacv_t_dn6, locals.var_uacv_t_dn7, locals.var_uacv_t_dn8, locals.var_uacv_t_dn9, locals.var_uacv_t_dn10, locals.var_uacv_t_dn11, locals.var_uacv_t_dn13, locals.var_uacv_t_dn14,)
    }
};
        locals.var_uacv_t = assign15470_e23848;
        locals.var_uacv_t_dn0 = assign15470_e23848_d_n0;
        locals.var_uacv_t_dn2 = assign15470_e23848_d_n2;
        locals.var_uacv_t_dn3 = assign15470_e23848_d_n3;
        locals.var_uacv_t_dn4 = assign15470_e23848_d_n4;
        locals.var_uacv_t_dn5 = assign15470_e23848_d_n5;
        locals.var_uacv_t_dn6 = assign15470_e23848_d_n6;
        locals.var_uacv_t_dn7 = assign15470_e23848_d_n7;
        locals.var_uacv_t_dn8 = assign15470_e23848_d_n8;
        locals.var_uacv_t_dn9 = assign15470_e23848_d_n9;
        locals.var_uacv_t_dn10 = assign15470_e23848_d_n10;
        locals.var_uacv_t_dn11 = assign15470_e23848_d_n11;
        locals.var_uacv_t_dn13 = assign15470_e23848_d_n13;
        locals.var_uacv_t_dn14 = assign15470_e23848_d_n14;
        locals.var_uacv_t_rv = 0.0;

        let (assign15480_e23866, assign15480_e23866_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard264 != 0.0)) {
        let assign15480_e23859: f64 = (locals.var_ud2cv_i * locals.var_tratio);
        let assign15480_e23860: f64 = (locals.var_ud1cv_i + assign15480_e23859);
        let assign15480_e23862: f64 = (assign15480_e23860 * locals.var_trat_ln);
        let assign15480_e23863: f64 = (assign15480_e23862).exp();
        let assign15480_e23864: f64 = (locals.var_udcv_i * assign15480_e23863);
        (assign15480_e23864, (locals.var_udcv_i * (assign15480_e23863 * (((locals.var_ud2cv_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15480_e23860 * locals.var_trat_ln_dn4)))),)
    } else {
        (locals.var_udcv_t, locals.var_udcv_t_dn4,)
    }
};
        locals.var_udcv_t = assign15480_e23866;
        locals.var_udcv_t_dn4 = assign15480_e23866_d_n4;
        locals.var_udcv_t_rv = 0.0;

        let assign15490_e23869: f64 = if locals.var_prt_i == locals.var_prt1_i { 1.0 } else { 0.0 };
        locals.var_guard265 = assign15490_e23869;
        locals.var_guard265_rv = 0.0;

        let (assign15500_e23882, assign15500_e23882_d_n0, assign15500_e23882_d_n2, assign15500_e23882_d_n3, assign15500_e23882_d_n4, assign15500_e23882_d_n5, assign15500_e23882_d_n6, assign15500_e23882_d_n7, assign15500_e23882_d_n8, assign15500_e23882_d_n9, assign15500_e23882_d_n10, assign15500_e23882_d_n11, assign15500_e23882_d_n13, assign15500_e23882_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 != 0.0)) {
        let assign15500_e23879: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign15500_e23880: f64 = (1.0 + assign15500_e23879);
        (assign15500_e23880, 0.0, 0.0, 0.0, (locals.var_prt_i * locals.var_deltemp_dn4), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15500_e23882;
        locals.var_t2_dn0 = assign15500_e23882_d_n0;
        locals.var_t2_dn2 = assign15500_e23882_d_n2;
        locals.var_t2_dn3 = assign15500_e23882_d_n3;
        locals.var_t2_dn4 = assign15500_e23882_d_n4;
        locals.var_t2_dn5 = assign15500_e23882_d_n5;
        locals.var_t2_dn6 = assign15500_e23882_d_n6;
        locals.var_t2_dn7 = assign15500_e23882_d_n7;
        locals.var_t2_dn8 = assign15500_e23882_d_n8;
        locals.var_t2_dn9 = assign15500_e23882_d_n9;
        locals.var_t2_dn10 = assign15500_e23882_d_n10;
        locals.var_t2_dn11 = assign15500_e23882_d_n11;
        locals.var_t2_dn13 = assign15500_e23882_d_n13;
        locals.var_t2_dn14 = assign15500_e23882_d_n14;
        locals.var_t2_rv = 0.0;

        let assign15510_e23885: f64 = if locals.var_tr0_i < locals.var_tnom { 1.0 } else { 0.0 };
        locals.var_guard266 = assign15510_e23885;
        locals.var_guard266_rv = 0.0;

        let (assign15520_e23901, assign15520_e23901_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign15520_e23898: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign15520_e23899: f64 = (1.0 + assign15520_e23898);
        (assign15520_e23899, (locals.var_prt_i * locals.var_deltemp_dn4),)
    } else {
        (locals.var_rdstemp0, locals.var_rdstemp0_dn4,)
    }
};
        locals.var_rdstemp0 = assign15520_e23901;
        locals.var_rdstemp0_dn4 = assign15520_e23901_d_n4;
        locals.var_rdstemp0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15530_e23925, assign15530_e23925_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign15530_e23915: f64 = (locals.var_devtemp - locals.var_tr0_i);
        let assign15530_e23916: f64 = (locals.var_prt1_i * assign15530_e23915);
        let assign15530_e23917: f64 = (1.0 + assign15530_e23916);
        let assign15530_e23921: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign15530_e23922: f64 = (locals.var_prt_i * assign15530_e23921);
        let assign15530_e23923: f64 = (assign15530_e23917 + assign15530_e23922);
        (assign15530_e23923, (locals.var_prt1_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp1, locals.var_rdstemp1_dn4,)
    }
};
        locals.var_rdstemp1 = assign15530_e23925;
        locals.var_rdstemp1_dn4 = assign15530_e23925_d_n4;
        locals.var_rdstemp1_rv = 0.0;

        let (assign15540_e23943, assign15540_e23943_d_n0, assign15540_e23943_d_n2, assign15540_e23943_d_n3, assign15540_e23943_d_n4, assign15540_e23943_d_n5, assign15540_e23943_d_n6, assign15540_e23943_d_n7, assign15540_e23943_d_n8, assign15540_e23943_d_n9, assign15540_e23943_d_n10, assign15540_e23943_d_n11, assign15540_e23943_d_n13, assign15540_e23943_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign15540_e23937: f64 = (locals.var_prt_i - locals.var_prt1_i);
        let assign15540_e23940: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign15540_e23941: f64 = (assign15540_e23937 * assign15540_e23940);
        (assign15540_e23941, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign15540_e23943;
        locals.var_t3_dn0 = assign15540_e23943_d_n0;
        locals.var_t3_dn2 = assign15540_e23943_d_n2;
        locals.var_t3_dn3 = assign15540_e23943_d_n3;
        locals.var_t3_dn4 = assign15540_e23943_d_n4;
        locals.var_t3_dn5 = assign15540_e23943_d_n5;
        locals.var_t3_dn6 = assign15540_e23943_d_n6;
        locals.var_t3_dn7 = assign15540_e23943_d_n7;
        locals.var_t3_dn8 = assign15540_e23943_d_n8;
        locals.var_t3_dn9 = assign15540_e23943_d_n9;
        locals.var_t3_dn10 = assign15540_e23943_d_n10;
        locals.var_t3_dn11 = assign15540_e23943_d_n11;
        locals.var_t3_dn13 = assign15540_e23943_d_n13;
        locals.var_t3_dn14 = assign15540_e23943_d_n14;
        locals.var_t3_rv = 0.0;

        let assign15550_e23946: f64 = if locals.var_prt1_i < locals.var_prt_i { 1.0 } else { 0.0 };
        locals.var_guard267 = assign15550_e23946;
        locals.var_guard267_rv = 0.0;

        let (assign15560_e24000, assign15560_e24000_d_n0, assign15560_e24000_d_n2, assign15560_e24000_d_n3, assign15560_e24000_d_n4, assign15560_e24000_d_n5, assign15560_e24000_d_n6, assign15560_e24000_d_n7, assign15560_e24000_d_n8, assign15560_e24000_d_n9, assign15560_e24000_d_n10, assign15560_e24000_d_n11, assign15560_e24000_d_n13, assign15560_e24000_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard267 != 0.0)) {
        let assign15560_e23961: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign15560_e23964: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign15560_e23967: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign15560_e23968: f64 = (assign15560_e23964 * assign15560_e23967);
        let assign15560_e23971: f64 = (0.25 * locals.var_sprt_i);
        let assign15560_e23973: f64 = (assign15560_e23971 * locals.var_sprt_i);
        let assign15560_e23974: f64 = (assign15560_e23968 + assign15560_e23973);
        let assign15560_e23975: f64 = (assign15560_e23974).sqrt();
        let assign15560_e23976: f64 = (assign15560_e23961 + assign15560_e23975);
        let assign15560_e23977: f64 = (0.5 * assign15560_e23976);
        let assign15560_e23981: f64 = locals.var_t3;
        let assign15560_e23984: f64 = locals.var_t3;
        let assign15560_e23987: f64 = locals.var_t3;
        let assign15560_e23988: f64 = (assign15560_e23984 * assign15560_e23987);
        let assign15560_e23991: f64 = (0.25 * locals.var_sprt_i);
        let assign15560_e23993: f64 = (assign15560_e23991 * locals.var_sprt_i);
        let assign15560_e23994: f64 = (assign15560_e23988 + assign15560_e23993);
        let assign15560_e23995: f64 = (assign15560_e23994).sqrt();
        let assign15560_e23996: f64 = (assign15560_e23981 + assign15560_e23995);
        let assign15560_e23997: f64 = (0.5 * assign15560_e23996);
        let assign15560_e23998: f64 = (assign15560_e23977 - assign15560_e23997);
        (assign15560_e23998, (-(0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn0)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn2)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn3 + (((locals.var_t3_dn3 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn3)) / (2.0 * assign15560_e23995))))), ((0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) + ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign15560_e23967) + (assign15560_e23964 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign15560_e23975)))) - (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn4)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn5)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn6)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn7)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn8)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn9)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn10)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn11)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn13 + (((locals.var_t3_dn13 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn13)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn14)) / (2.0 * assign15560_e23995))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15560_e24000;
        locals.var_t2_dn0 = assign15560_e24000_d_n0;
        locals.var_t2_dn2 = assign15560_e24000_d_n2;
        locals.var_t2_dn3 = assign15560_e24000_d_n3;
        locals.var_t2_dn4 = assign15560_e24000_d_n4;
        locals.var_t2_dn5 = assign15560_e24000_d_n5;
        locals.var_t2_dn6 = assign15560_e24000_d_n6;
        locals.var_t2_dn7 = assign15560_e24000_d_n7;
        locals.var_t2_dn8 = assign15560_e24000_d_n8;
        locals.var_t2_dn9 = assign15560_e24000_d_n9;
        locals.var_t2_dn10 = assign15560_e24000_d_n10;
        locals.var_t2_dn11 = assign15560_e24000_d_n11;
        locals.var_t2_dn13 = assign15560_e24000_d_n13;
        locals.var_t2_dn14 = assign15560_e24000_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15570_e24055, assign15570_e24055_d_n0, assign15570_e24055_d_n2, assign15570_e24055_d_n3, assign15570_e24055_d_n4, assign15570_e24055_d_n5, assign15570_e24055_d_n6, assign15570_e24055_d_n7, assign15570_e24055_d_n8, assign15570_e24055_d_n9, assign15570_e24055_d_n10, assign15570_e24055_d_n11, assign15570_e24055_d_n13, assign15570_e24055_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard267 == 0.0)) {
        let assign15570_e24016: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign15570_e24019: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign15570_e24022: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign15570_e24023: f64 = (assign15570_e24019 * assign15570_e24022);
        let assign15570_e24026: f64 = (0.25 * locals.var_sprt_i);
        let assign15570_e24028: f64 = (assign15570_e24026 * locals.var_sprt_i);
        let assign15570_e24029: f64 = (assign15570_e24023 + assign15570_e24028);
        let assign15570_e24030: f64 = (assign15570_e24029).sqrt();
        let assign15570_e24031: f64 = (assign15570_e24016 - assign15570_e24030);
        let assign15570_e24032: f64 = (0.5 * assign15570_e24031);
        let assign15570_e24036: f64 = locals.var_t3;
        let assign15570_e24039: f64 = locals.var_t3;
        let assign15570_e24042: f64 = locals.var_t3;
        let assign15570_e24043: f64 = (assign15570_e24039 * assign15570_e24042);
        let assign15570_e24046: f64 = (0.25 * locals.var_sprt_i);
        let assign15570_e24048: f64 = (assign15570_e24046 * locals.var_sprt_i);
        let assign15570_e24049: f64 = (assign15570_e24043 + assign15570_e24048);
        let assign15570_e24050: f64 = (assign15570_e24049).sqrt();
        let assign15570_e24051: f64 = (assign15570_e24036 - assign15570_e24050);
        let assign15570_e24052: f64 = (0.5 * assign15570_e24051);
        let assign15570_e24053: f64 = (assign15570_e24032 - assign15570_e24052);
        (assign15570_e24053, (-(0.5 * (locals.var_t3_dn0 - (((locals.var_t3_dn0 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn0)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn2 - (((locals.var_t3_dn2 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn2)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn3 - (((locals.var_t3_dn3 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn3)) / (2.0 * assign15570_e24050))))), ((0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) - ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign15570_e24022) + (assign15570_e24019 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign15570_e24030)))) - (0.5 * (locals.var_t3_dn4 - (((locals.var_t3_dn4 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn4)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn5 - (((locals.var_t3_dn5 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn5)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn6 - (((locals.var_t3_dn6 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn6)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn7 - (((locals.var_t3_dn7 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn7)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn8 - (((locals.var_t3_dn8 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn8)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn9 - (((locals.var_t3_dn9 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn9)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn10 - (((locals.var_t3_dn10 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn10)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn11 - (((locals.var_t3_dn11 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn11)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn13 - (((locals.var_t3_dn13 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn13)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn14 - (((locals.var_t3_dn14 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn14)) / (2.0 * assign15570_e24050))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15570_e24055;
        locals.var_t2_dn0 = assign15570_e24055_d_n0;
        locals.var_t2_dn2 = assign15570_e24055_d_n2;
        locals.var_t2_dn3 = assign15570_e24055_d_n3;
        locals.var_t2_dn4 = assign15570_e24055_d_n4;
        locals.var_t2_dn5 = assign15570_e24055_d_n5;
        locals.var_t2_dn6 = assign15570_e24055_d_n6;
        locals.var_t2_dn7 = assign15570_e24055_d_n7;
        locals.var_t2_dn8 = assign15570_e24055_d_n8;
        locals.var_t2_dn9 = assign15570_e24055_d_n9;
        locals.var_t2_dn10 = assign15570_e24055_d_n10;
        locals.var_t2_dn11 = assign15570_e24055_d_n11;
        locals.var_t2_dn13 = assign15570_e24055_d_n13;
        locals.var_t2_dn14 = assign15570_e24055_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15580_e24074, assign15580_e24074_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) {
        let assign15580_e24070: f64 = (locals.var_devtemp - locals.var_tnom);
        let assign15580_e24071: f64 = (locals.var_prt1_i * assign15580_e24070);
        let assign15580_e24072: f64 = (1.0 + assign15580_e24071);
        (assign15580_e24072, (locals.var_prt1_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp1, locals.var_rdstemp1_dn4,)
    }
};
        locals.var_rdstemp1 = assign15580_e24074;
        locals.var_rdstemp1_dn4 = assign15580_e24074_d_n4;
        locals.var_rdstemp1_rv = 0.0;

        let (assign15590_e24099, assign15590_e24099_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) {
        let assign15590_e24089: f64 = (locals.var_devtemp - locals.var_tr0_i);
        let assign15590_e24090: f64 = (locals.var_prt_i * assign15590_e24089);
        let assign15590_e24091: f64 = (1.0 + assign15590_e24090);
        let assign15590_e24095: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign15590_e24096: f64 = (locals.var_prt1_i * assign15590_e24095);
        let assign15590_e24097: f64 = (assign15590_e24091 + assign15590_e24096);
        (assign15590_e24097, (locals.var_prt_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp0, locals.var_rdstemp0_dn4,)
    }
};
        locals.var_rdstemp0 = assign15590_e24099;
        locals.var_rdstemp0_dn4 = assign15590_e24099_d_n4;
        locals.var_rdstemp0_rv = 0.0;

        let (assign15600_e24118, assign15600_e24118_d_n0, assign15600_e24118_d_n2, assign15600_e24118_d_n3, assign15600_e24118_d_n4, assign15600_e24118_d_n5, assign15600_e24118_d_n6, assign15600_e24118_d_n7, assign15600_e24118_d_n8, assign15600_e24118_d_n9, assign15600_e24118_d_n10, assign15600_e24118_d_n11, assign15600_e24118_d_n13, assign15600_e24118_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) {
        let assign15600_e24112: f64 = (locals.var_prt1_i - locals.var_prt_i);
        let assign15600_e24115: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign15600_e24116: f64 = (assign15600_e24112 * assign15600_e24115);
        (assign15600_e24116, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign15600_e24118;
        locals.var_t3_dn0 = assign15600_e24118_d_n0;
        locals.var_t3_dn2 = assign15600_e24118_d_n2;
        locals.var_t3_dn3 = assign15600_e24118_d_n3;
        locals.var_t3_dn4 = assign15600_e24118_d_n4;
        locals.var_t3_dn5 = assign15600_e24118_d_n5;
        locals.var_t3_dn6 = assign15600_e24118_d_n6;
        locals.var_t3_dn7 = assign15600_e24118_d_n7;
        locals.var_t3_dn8 = assign15600_e24118_d_n8;
        locals.var_t3_dn9 = assign15600_e24118_d_n9;
        locals.var_t3_dn10 = assign15600_e24118_d_n10;
        locals.var_t3_dn11 = assign15600_e24118_d_n11;
        locals.var_t3_dn13 = assign15600_e24118_d_n13;
        locals.var_t3_dn14 = assign15600_e24118_d_n14;
        locals.var_t3_rv = 0.0;

        let assign15610_e24121: f64 = if locals.var_prt1_i < locals.var_prt_i { 1.0 } else { 0.0 };
        locals.var_guard268 = assign15610_e24121;
        locals.var_guard268_rv = 0.0;

        let (assign15620_e24176, assign15620_e24176_d_n0, assign15620_e24176_d_n2, assign15620_e24176_d_n3, assign15620_e24176_d_n4, assign15620_e24176_d_n5, assign15620_e24176_d_n6, assign15620_e24176_d_n7, assign15620_e24176_d_n8, assign15620_e24176_d_n9, assign15620_e24176_d_n10, assign15620_e24176_d_n11, assign15620_e24176_d_n13, assign15620_e24176_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) && (locals.var_guard268 != 0.0)) {
        let assign15620_e24137: f64 = (locals.var_rdstemp1 + locals.var_rdstemp0);
        let assign15620_e24140: f64 = (locals.var_rdstemp1 - locals.var_rdstemp0);
        let assign15620_e24143: f64 = (locals.var_rdstemp1 - locals.var_rdstemp0);
        let assign15620_e24144: f64 = (assign15620_e24140 * assign15620_e24143);
        let assign15620_e24147: f64 = (0.25 * locals.var_sprt_i);
        let assign15620_e24149: f64 = (assign15620_e24147 * locals.var_sprt_i);
        let assign15620_e24150: f64 = (assign15620_e24144 + assign15620_e24149);
        let assign15620_e24151: f64 = (assign15620_e24150).sqrt();
        let assign15620_e24152: f64 = (assign15620_e24137 + assign15620_e24151);
        let assign15620_e24153: f64 = (0.5 * assign15620_e24152);
        let assign15620_e24157: f64 = locals.var_t3;
        let assign15620_e24160: f64 = locals.var_t3;
        let assign15620_e24163: f64 = locals.var_t3;
        let assign15620_e24164: f64 = (assign15620_e24160 * assign15620_e24163);
        let assign15620_e24167: f64 = (0.25 * locals.var_sprt_i);
        let assign15620_e24169: f64 = (assign15620_e24167 * locals.var_sprt_i);
        let assign15620_e24170: f64 = (assign15620_e24164 + assign15620_e24169);
        let assign15620_e24171: f64 = (assign15620_e24170).sqrt();
        let assign15620_e24172: f64 = (assign15620_e24157 + assign15620_e24171);
        let assign15620_e24173: f64 = (0.5 * assign15620_e24172);
        let assign15620_e24174: f64 = (assign15620_e24153 - assign15620_e24173);
        (assign15620_e24174, (-(0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn0)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn2)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn3 + (((locals.var_t3_dn3 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn3)) / (2.0 * assign15620_e24171))))), ((0.5 * ((locals.var_rdstemp1_dn4 + locals.var_rdstemp0_dn4) + ((((locals.var_rdstemp1_dn4 - locals.var_rdstemp0_dn4) * assign15620_e24143) + (assign15620_e24140 * (locals.var_rdstemp1_dn4 - locals.var_rdstemp0_dn4))) / (2.0 * assign15620_e24151)))) - (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn4)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn5)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn6)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn7)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn8)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn9)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn10)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn11)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn13 + (((locals.var_t3_dn13 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn13)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn14)) / (2.0 * assign15620_e24171))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15620_e24176;
        locals.var_t2_dn0 = assign15620_e24176_d_n0;
        locals.var_t2_dn2 = assign15620_e24176_d_n2;
        locals.var_t2_dn3 = assign15620_e24176_d_n3;
        locals.var_t2_dn4 = assign15620_e24176_d_n4;
        locals.var_t2_dn5 = assign15620_e24176_d_n5;
        locals.var_t2_dn6 = assign15620_e24176_d_n6;
        locals.var_t2_dn7 = assign15620_e24176_d_n7;
        locals.var_t2_dn8 = assign15620_e24176_d_n8;
        locals.var_t2_dn9 = assign15620_e24176_d_n9;
        locals.var_t2_dn10 = assign15620_e24176_d_n10;
        locals.var_t2_dn11 = assign15620_e24176_d_n11;
        locals.var_t2_dn13 = assign15620_e24176_d_n13;
        locals.var_t2_dn14 = assign15620_e24176_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15630_e24232, assign15630_e24232_d_n0, assign15630_e24232_d_n2, assign15630_e24232_d_n3, assign15630_e24232_d_n4, assign15630_e24232_d_n5, assign15630_e24232_d_n6, assign15630_e24232_d_n7, assign15630_e24232_d_n8, assign15630_e24232_d_n9, assign15630_e24232_d_n10, assign15630_e24232_d_n11, assign15630_e24232_d_n13, assign15630_e24232_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) && (locals.var_guard268 == 0.0)) {
        let assign15630_e24193: f64 = (locals.var_rdstemp1 + locals.var_rdstemp0);
        let assign15630_e24196: f64 = (locals.var_rdstemp1 - locals.var_rdstemp0);
        let assign15630_e24199: f64 = (locals.var_rdstemp1 - locals.var_rdstemp0);
        let assign15630_e24200: f64 = (assign15630_e24196 * assign15630_e24199);
        let assign15630_e24203: f64 = (0.25 * locals.var_sprt_i);
        let assign15630_e24205: f64 = (assign15630_e24203 * locals.var_sprt_i);
        let assign15630_e24206: f64 = (assign15630_e24200 + assign15630_e24205);
        let assign15630_e24207: f64 = (assign15630_e24206).sqrt();
        let assign15630_e24208: f64 = (assign15630_e24193 - assign15630_e24207);
        let assign15630_e24209: f64 = (0.5 * assign15630_e24208);
        let assign15630_e24213: f64 = locals.var_t3;
        let assign15630_e24216: f64 = locals.var_t3;
        let assign15630_e24219: f64 = locals.var_t3;
        let assign15630_e24220: f64 = (assign15630_e24216 * assign15630_e24219);
        let assign15630_e24223: f64 = (0.25 * locals.var_sprt_i);
        let assign15630_e24225: f64 = (assign15630_e24223 * locals.var_sprt_i);
        let assign15630_e24226: f64 = (assign15630_e24220 + assign15630_e24225);
        let assign15630_e24227: f64 = (assign15630_e24226).sqrt();
        let assign15630_e24228: f64 = (assign15630_e24213 - assign15630_e24227);
        let assign15630_e24229: f64 = (0.5 * assign15630_e24228);
        let assign15630_e24230: f64 = (assign15630_e24209 - assign15630_e24229);
        (assign15630_e24230, (-(0.5 * (locals.var_t3_dn0 - (((locals.var_t3_dn0 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn0)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn2 - (((locals.var_t3_dn2 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn2)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn3 - (((locals.var_t3_dn3 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn3)) / (2.0 * assign15630_e24227))))), ((0.5 * ((locals.var_rdstemp1_dn4 + locals.var_rdstemp0_dn4) - ((((locals.var_rdstemp1_dn4 - locals.var_rdstemp0_dn4) * assign15630_e24199) + (assign15630_e24196 * (locals.var_rdstemp1_dn4 - locals.var_rdstemp0_dn4))) / (2.0 * assign15630_e24207)))) - (0.5 * (locals.var_t3_dn4 - (((locals.var_t3_dn4 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn4)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn5 - (((locals.var_t3_dn5 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn5)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn6 - (((locals.var_t3_dn6 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn6)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn7 - (((locals.var_t3_dn7 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn7)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn8 - (((locals.var_t3_dn8 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn8)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn9 - (((locals.var_t3_dn9 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn9)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn10 - (((locals.var_t3_dn10 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn10)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn11 - (((locals.var_t3_dn11 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn11)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn13 - (((locals.var_t3_dn13 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn13)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn14 - (((locals.var_t3_dn14 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn14)) / (2.0 * assign15630_e24227))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15630_e24232;
        locals.var_t2_dn0 = assign15630_e24232_d_n0;
        locals.var_t2_dn2 = assign15630_e24232_d_n2;
        locals.var_t2_dn3 = assign15630_e24232_d_n3;
        locals.var_t2_dn4 = assign15630_e24232_d_n4;
        locals.var_t2_dn5 = assign15630_e24232_d_n5;
        locals.var_t2_dn6 = assign15630_e24232_d_n6;
        locals.var_t2_dn7 = assign15630_e24232_d_n7;
        locals.var_t2_dn8 = assign15630_e24232_d_n8;
        locals.var_t2_dn9 = assign15630_e24232_d_n9;
        locals.var_t2_dn10 = assign15630_e24232_d_n10;
        locals.var_t2_dn11 = assign15630_e24232_d_n11;
        locals.var_t2_dn13 = assign15630_e24232_d_n13;
        locals.var_t2_dn14 = assign15630_e24232_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15640_e24286, assign15640_e24286_d_n0, assign15640_e24286_d_n2, assign15640_e24286_d_n3, assign15640_e24286_d_n4, assign15640_e24286_d_n5, assign15640_e24286_d_n6, assign15640_e24286_d_n7, assign15640_e24286_d_n8, assign15640_e24286_d_n9, assign15640_e24286_d_n10, assign15640_e24286_d_n11, assign15640_e24286_d_n13, assign15640_e24286_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15640_e24239: f64 = (locals.var_t2 - 1e-6);
        let assign15640_e24241: f64 = (-10000.0);
        let assign15640_e24243: f64 = (assign15640_e24241 * 0.001);
        let (assign15640_e24284, assign15640_e24284_d_n0, assign15640_e24284_d_n2, assign15640_e24284_d_n3, assign15640_e24284_d_n4, assign15640_e24284_d_n5, assign15640_e24284_d_n6, assign15640_e24284_d_n7, assign15640_e24284_d_n8, assign15640_e24284_d_n9, assign15640_e24284_d_n10, assign15640_e24284_d_n11, assign15640_e24284_d_n13, assign15640_e24284_d_n14,) = {
            if (!(assign15640_e24239 < assign15640_e24243)) {
                let assign15640_e24249: f64 = (locals.var_t2 - 1e-6);
                let assign15640_e24252: f64 = (locals.var_t2 - 1e-6);
                let assign15640_e24255: f64 = (locals.var_t2 - 1e-6);
                let assign15640_e24256: f64 = (assign15640_e24252 * assign15640_e24255);
                let assign15640_e24259: f64 = (4.0 * 0.001);
                let assign15640_e24261: f64 = (assign15640_e24259 * 0.001);
                let assign15640_e24262: f64 = (assign15640_e24256 + assign15640_e24261);
                let assign15640_e24263: f64 = (assign15640_e24262).sqrt();
                let assign15640_e24264: f64 = (assign15640_e24249 + assign15640_e24263);
                let assign15640_e24265: f64 = (0.5 * assign15640_e24264);
                (assign15640_e24265, (0.5 * (locals.var_t2_dn0 + (((locals.var_t2_dn0 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn0)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn2 + (((locals.var_t2_dn2 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn2)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn3)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn4)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn5)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn6)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn7)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn8)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn9)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn10)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn11)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn13 + (((locals.var_t2_dn13 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn13)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn14 + (((locals.var_t2_dn14 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn14)) / (2.0 * assign15640_e24263)))),)
            } else {
                let assign15640_e24268: f64 = (locals.var_t2 - 1e-6);
                let assign15640_e24270: f64 = (-10000.0);
                let assign15640_e24272: f64 = (assign15640_e24270 * 0.001);
                let (assign15640_e24283, assign15640_e24283_d_n0, assign15640_e24283_d_n2, assign15640_e24283_d_n3, assign15640_e24283_d_n4, assign15640_e24283_d_n5, assign15640_e24283_d_n6, assign15640_e24283_d_n7, assign15640_e24283_d_n8, assign15640_e24283_d_n9, assign15640_e24283_d_n10, assign15640_e24283_d_n11, assign15640_e24283_d_n13, assign15640_e24283_d_n14,) = {
                    if (assign15640_e24268 < assign15640_e24272) {
                        let assign15640_e24275: f64 = (-0.001);
                        let assign15640_e24277: f64 = (assign15640_e24275 * 0.001);
                        let assign15640_e24280: f64 = (locals.var_t2 - 1e-6);
                        let assign15640_e24281: f64 = (assign15640_e24277 / assign15640_e24280);
                        (assign15640_e24281, (-((assign15640_e24277 * locals.var_t2_dn0) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn2) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn3) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn4) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn5) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn6) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn7) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn8) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn9) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn10) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn11) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn13) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn14) / (assign15640_e24280 * assign15640_e24280))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15640_e24283, assign15640_e24283_d_n0, assign15640_e24283_d_n2, assign15640_e24283_d_n3, assign15640_e24283_d_n4, assign15640_e24283_d_n5, assign15640_e24283_d_n6, assign15640_e24283_d_n7, assign15640_e24283_d_n8, assign15640_e24283_d_n9, assign15640_e24283_d_n10, assign15640_e24283_d_n11, assign15640_e24283_d_n13, assign15640_e24283_d_n14,)
            }
        };
        (assign15640_e24284, assign15640_e24284_d_n0, assign15640_e24284_d_n2, assign15640_e24284_d_n3, assign15640_e24284_d_n4, assign15640_e24284_d_n5, assign15640_e24284_d_n6, assign15640_e24284_d_n7, assign15640_e24284_d_n8, assign15640_e24284_d_n9, assign15640_e24284_d_n10, assign15640_e24284_d_n11, assign15640_e24284_d_n13, assign15640_e24284_d_n14,)
    } else {
        (locals.var_rdstemp, locals.var_rdstemp_dn0, locals.var_rdstemp_dn2, locals.var_rdstemp_dn3, locals.var_rdstemp_dn4, locals.var_rdstemp_dn5, locals.var_rdstemp_dn6, locals.var_rdstemp_dn7, locals.var_rdstemp_dn8, locals.var_rdstemp_dn9, locals.var_rdstemp_dn10, locals.var_rdstemp_dn11, locals.var_rdstemp_dn13, locals.var_rdstemp_dn14,)
    }
};
        locals.var_rdstemp = assign15640_e24286;
        locals.var_rdstemp_dn0 = assign15640_e24286_d_n0;
        locals.var_rdstemp_dn2 = assign15640_e24286_d_n2;
        locals.var_rdstemp_dn3 = assign15640_e24286_d_n3;
        locals.var_rdstemp_dn4 = assign15640_e24286_d_n4;
        locals.var_rdstemp_dn5 = assign15640_e24286_d_n5;
        locals.var_rdstemp_dn6 = assign15640_e24286_d_n6;
        locals.var_rdstemp_dn7 = assign15640_e24286_d_n7;
        locals.var_rdstemp_dn8 = assign15640_e24286_d_n8;
        locals.var_rdstemp_dn9 = assign15640_e24286_d_n9;
        locals.var_rdstemp_dn10 = assign15640_e24286_d_n10;
        locals.var_rdstemp_dn11 = assign15640_e24286_d_n11;
        locals.var_rdstemp_dn13 = assign15640_e24286_d_n13;
        locals.var_rdstemp_dn14 = assign15640_e24286_d_n14;
        locals.var_rdstemp_rv = 0.0;

        let assign15650_e24289: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign15650_e24289;
        locals.var_guard269_rv = 0.0;

        let (assign15660_e24359, assign15660_e24359_d_n0, assign15660_e24359_d_n2, assign15660_e24359_d_n3, assign15660_e24359_d_n4, assign15660_e24359_d_n5, assign15660_e24359_d_n6, assign15660_e24359_d_n7, assign15660_e24359_d_n8, assign15660_e24359_d_n9, assign15660_e24359_d_n10, assign15660_e24359_d_n11, assign15660_e24359_d_n13, assign15660_e24359_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard269 != 0.0)) {
        let assign15660_e24298: f64 = (-locals.var_vsat_i);
        let assign15660_e24301: f64 = (-locals.var_at_i);
        let assign15660_e24303: f64 = (assign15660_e24301 * locals.var_deltemp);
        let assign15660_e24306: f64 = (p.p561 * locals.var_deltemp);
        let assign15660_e24308: f64 = (assign15660_e24306 * locals.var_deltemp);
        let assign15660_e24309: f64 = (assign15660_e24303 + assign15660_e24308);
        let assign15660_e24311: f64 = (-locals.var_vsat_i);
        let assign15660_e24312: f64 = (assign15660_e24309 - assign15660_e24311);
        let assign15660_e24314: f64 = (assign15660_e24312 - 1e-6);
        let assign15660_e24316: f64 = (-locals.var_at_i);
        let assign15660_e24318: f64 = (assign15660_e24316 * locals.var_deltemp);
        let assign15660_e24321: f64 = (p.p561 * locals.var_deltemp);
        let assign15660_e24323: f64 = (assign15660_e24321 * locals.var_deltemp);
        let assign15660_e24324: f64 = (assign15660_e24318 + assign15660_e24323);
        let assign15660_e24326: f64 = (-locals.var_vsat_i);
        let assign15660_e24327: f64 = (assign15660_e24324 - assign15660_e24326);
        let assign15660_e24329: f64 = (assign15660_e24327 - 1e-6);
        let assign15660_e24331: f64 = (-locals.var_at_i);
        let assign15660_e24333: f64 = (assign15660_e24331 * locals.var_deltemp);
        let assign15660_e24336: f64 = (p.p561 * locals.var_deltemp);
        let assign15660_e24338: f64 = (assign15660_e24336 * locals.var_deltemp);
        let assign15660_e24339: f64 = (assign15660_e24333 + assign15660_e24338);
        let assign15660_e24341: f64 = (-locals.var_vsat_i);
        let assign15660_e24342: f64 = (assign15660_e24339 - assign15660_e24341);
        let assign15660_e24344: f64 = (assign15660_e24342 - 1e-6);
        let assign15660_e24345: f64 = (assign15660_e24329 * assign15660_e24344);
        let assign15660_e24348: f64 = (-locals.var_vsat_i);
        let assign15660_e24349: f64 = (4.0 * assign15660_e24348);
        let assign15660_e24351: f64 = (assign15660_e24349 * 1e-6);
        let assign15660_e24352: f64 = (assign15660_e24345 - assign15660_e24351);
        let assign15660_e24353: f64 = (assign15660_e24352).sqrt();
        let assign15660_e24354: f64 = (assign15660_e24314 + assign15660_e24353);
        let assign15660_e24355: f64 = (0.5 * assign15660_e24354);
        let assign15660_e24356: f64 = (assign15660_e24298 + assign15660_e24355);
        let assign15660_e24357: f64 = (locals.var_vsat_i + assign15660_e24356);
        (assign15660_e24357, (locals.var_vsat_i_dn0 + ((-locals.var_vsat_i_dn0) + (0.5 * ((-(-locals.var_vsat_i_dn0)) + (((((-(-locals.var_vsat_i_dn0)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn0)))) - ((4.0 * (-locals.var_vsat_i_dn0)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn2 + ((-locals.var_vsat_i_dn2) + (0.5 * ((-(-locals.var_vsat_i_dn2)) + (((((-(-locals.var_vsat_i_dn2)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn2)))) - ((4.0 * (-locals.var_vsat_i_dn2)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn3 + ((-locals.var_vsat_i_dn3) + (0.5 * ((-(-locals.var_vsat_i_dn3)) + (((((-(-locals.var_vsat_i_dn3)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn3)))) - ((4.0 * (-locals.var_vsat_i_dn3)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn4 + ((-locals.var_vsat_i_dn4) + (0.5 * ((((assign15660_e24301 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15660_e24306 * locals.var_deltemp_dn4))) - (-locals.var_vsat_i_dn4)) + (((((((assign15660_e24316 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15660_e24321 * locals.var_deltemp_dn4))) - (-locals.var_vsat_i_dn4)) * assign15660_e24344) + (assign15660_e24329 * (((assign15660_e24331 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15660_e24336 * locals.var_deltemp_dn4))) - (-locals.var_vsat_i_dn4)))) - ((4.0 * (-locals.var_vsat_i_dn4)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn5 + ((-locals.var_vsat_i_dn5) + (0.5 * ((-(-locals.var_vsat_i_dn5)) + (((((-(-locals.var_vsat_i_dn5)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn5)))) - ((4.0 * (-locals.var_vsat_i_dn5)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn6 + ((-locals.var_vsat_i_dn6) + (0.5 * ((-(-locals.var_vsat_i_dn6)) + (((((-(-locals.var_vsat_i_dn6)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn6)))) - ((4.0 * (-locals.var_vsat_i_dn6)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn7 + ((-locals.var_vsat_i_dn7) + (0.5 * ((-(-locals.var_vsat_i_dn7)) + (((((-(-locals.var_vsat_i_dn7)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn7)))) - ((4.0 * (-locals.var_vsat_i_dn7)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn8 + ((-locals.var_vsat_i_dn8) + (0.5 * ((-(-locals.var_vsat_i_dn8)) + (((((-(-locals.var_vsat_i_dn8)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn8)))) - ((4.0 * (-locals.var_vsat_i_dn8)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn9 + ((-locals.var_vsat_i_dn9) + (0.5 * ((-(-locals.var_vsat_i_dn9)) + (((((-(-locals.var_vsat_i_dn9)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn9)))) - ((4.0 * (-locals.var_vsat_i_dn9)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn10 + ((-locals.var_vsat_i_dn10) + (0.5 * ((-(-locals.var_vsat_i_dn10)) + (((((-(-locals.var_vsat_i_dn10)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn10)))) - ((4.0 * (-locals.var_vsat_i_dn10)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn11 + ((-locals.var_vsat_i_dn11) + (0.5 * ((-(-locals.var_vsat_i_dn11)) + (((((-(-locals.var_vsat_i_dn11)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn11)))) - ((4.0 * (-locals.var_vsat_i_dn11)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn13 + ((-locals.var_vsat_i_dn13) + (0.5 * ((-(-locals.var_vsat_i_dn13)) + (((((-(-locals.var_vsat_i_dn13)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn13)))) - ((4.0 * (-locals.var_vsat_i_dn13)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn14 + ((-locals.var_vsat_i_dn14) + (0.5 * ((-(-locals.var_vsat_i_dn14)) + (((((-(-locals.var_vsat_i_dn14)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn14)))) - ((4.0 * (-locals.var_vsat_i_dn14)) * 1e-6)) / (2.0 * assign15660_e24353)))))),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign15660_e24359;
        locals.var_vsat_t_dn0 = assign15660_e24359_d_n0;
        locals.var_vsat_t_dn2 = assign15660_e24359_d_n2;
        locals.var_vsat_t_dn3 = assign15660_e24359_d_n3;
        locals.var_vsat_t_dn4 = assign15660_e24359_d_n4;
        locals.var_vsat_t_dn5 = assign15660_e24359_d_n5;
        locals.var_vsat_t_dn6 = assign15660_e24359_d_n6;
        locals.var_vsat_t_dn7 = assign15660_e24359_d_n7;
        locals.var_vsat_t_dn8 = assign15660_e24359_d_n8;
        locals.var_vsat_t_dn9 = assign15660_e24359_d_n9;
        locals.var_vsat_t_dn10 = assign15660_e24359_d_n10;
        locals.var_vsat_t_dn11 = assign15660_e24359_d_n11;
        locals.var_vsat_t_dn13 = assign15660_e24359_d_n13;
        locals.var_vsat_t_dn14 = assign15660_e24359_d_n14;
        locals.var_vsat_t_rv = 0.0;

        let (assign15670_e24484, assign15670_e24484_d_n0, assign15670_e24484_d_n2, assign15670_e24484_d_n3, assign15670_e24484_d_n4, assign15670_e24484_d_n5, assign15670_e24484_d_n6, assign15670_e24484_d_n7, assign15670_e24484_d_n8, assign15670_e24484_d_n9, assign15670_e24484_d_n10, assign15670_e24484_d_n11, assign15670_e24484_d_n13, assign15670_e24484_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard269 == 0.0)) {
        let assign15670_e24370: f64 = (-locals.var_at_i);
        let assign15670_e24372: f64 = (assign15670_e24370 * locals.var_deltemp);
        let assign15670_e24373: f64 = (1.0 + assign15670_e24372);
        let assign15670_e24376: f64 = (p.p561 * locals.var_deltemp);
        let assign15670_e24378: f64 = (assign15670_e24376 * locals.var_deltemp);
        let assign15670_e24379: f64 = (assign15670_e24373 + assign15670_e24378);
        let assign15670_e24381: f64 = (assign15670_e24379 - 1e-6);
        let assign15670_e24383: f64 = (-10000.0);
        let assign15670_e24385: f64 = (assign15670_e24383 * 0.001);
        let (assign15670_e24481, assign15670_e24481_d_n4,) = {
            if (!(assign15670_e24381 < assign15670_e24385)) {
                let assign15670_e24391: f64 = (-locals.var_at_i);
                let assign15670_e24393: f64 = (assign15670_e24391 * locals.var_deltemp);
                let assign15670_e24394: f64 = (1.0 + assign15670_e24393);
                let assign15670_e24397: f64 = (p.p561 * locals.var_deltemp);
                let assign15670_e24399: f64 = (assign15670_e24397 * locals.var_deltemp);
                let assign15670_e24400: f64 = (assign15670_e24394 + assign15670_e24399);
                let assign15670_e24402: f64 = (assign15670_e24400 - 1e-6);
                let assign15670_e24405: f64 = (-locals.var_at_i);
                let assign15670_e24407: f64 = (assign15670_e24405 * locals.var_deltemp);
                let assign15670_e24408: f64 = (1.0 + assign15670_e24407);
                let assign15670_e24411: f64 = (p.p561 * locals.var_deltemp);
                let assign15670_e24413: f64 = (assign15670_e24411 * locals.var_deltemp);
                let assign15670_e24414: f64 = (assign15670_e24408 + assign15670_e24413);
                let assign15670_e24416: f64 = (assign15670_e24414 - 1e-6);
                let assign15670_e24419: f64 = (-locals.var_at_i);
                let assign15670_e24421: f64 = (assign15670_e24419 * locals.var_deltemp);
                let assign15670_e24422: f64 = (1.0 + assign15670_e24421);
                let assign15670_e24425: f64 = (p.p561 * locals.var_deltemp);
                let assign15670_e24427: f64 = (assign15670_e24425 * locals.var_deltemp);
                let assign15670_e24428: f64 = (assign15670_e24422 + assign15670_e24427);
                let assign15670_e24430: f64 = (assign15670_e24428 - 1e-6);
                let assign15670_e24431: f64 = (assign15670_e24416 * assign15670_e24430);
                let assign15670_e24434: f64 = (4.0 * 0.001);
                let assign15670_e24436: f64 = (assign15670_e24434 * 0.001);
                let assign15670_e24437: f64 = (assign15670_e24431 + assign15670_e24436);
                let assign15670_e24438: f64 = (assign15670_e24437).sqrt();
                let assign15670_e24439: f64 = (assign15670_e24402 + assign15670_e24438);
                let assign15670_e24440: f64 = (0.5 * assign15670_e24439);
                (assign15670_e24440, (0.5 * (((assign15670_e24391 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15670_e24397 * locals.var_deltemp_dn4))) + (((((assign15670_e24405 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15670_e24411 * locals.var_deltemp_dn4))) * assign15670_e24430) + (assign15670_e24416 * ((assign15670_e24419 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15670_e24425 * locals.var_deltemp_dn4))))) / (2.0 * assign15670_e24438)))),)
            } else {
                let assign15670_e24443: f64 = (-locals.var_at_i);
                let assign15670_e24445: f64 = (assign15670_e24443 * locals.var_deltemp);
                let assign15670_e24446: f64 = (1.0 + assign15670_e24445);
                let assign15670_e24449: f64 = (p.p561 * locals.var_deltemp);
                let assign15670_e24451: f64 = (assign15670_e24449 * locals.var_deltemp);
                let assign15670_e24452: f64 = (assign15670_e24446 + assign15670_e24451);
                let assign15670_e24454: f64 = (assign15670_e24452 - 1e-6);
                let assign15670_e24456: f64 = (-10000.0);
                let assign15670_e24458: f64 = (assign15670_e24456 * 0.001);
                let (assign15670_e24480, assign15670_e24480_d_n4,) = {
                    if (assign15670_e24454 < assign15670_e24458) {
                        let assign15670_e24461: f64 = (-0.001);
                        let assign15670_e24463: f64 = (assign15670_e24461 * 0.001);
                        let assign15670_e24466: f64 = (-locals.var_at_i);
                        let assign15670_e24468: f64 = (assign15670_e24466 * locals.var_deltemp);
                        let assign15670_e24469: f64 = (1.0 + assign15670_e24468);
                        let assign15670_e24472: f64 = (p.p561 * locals.var_deltemp);
                        let assign15670_e24474: f64 = (assign15670_e24472 * locals.var_deltemp);
                        let assign15670_e24475: f64 = (assign15670_e24469 + assign15670_e24474);
                        let assign15670_e24477: f64 = (assign15670_e24475 - 1e-6);
                        let assign15670_e24478: f64 = (assign15670_e24463 / assign15670_e24477);
                        (assign15670_e24478, (-((assign15670_e24463 * ((assign15670_e24466 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15670_e24472 * locals.var_deltemp_dn4)))) / (assign15670_e24477 * assign15670_e24477))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15670_e24480, assign15670_e24480_d_n4,)
            }
        };
        let assign15670_e24482: f64 = (locals.var_vsat_i * assign15670_e24481);
        (assign15670_e24482, (locals.var_vsat_i_dn0 * assign15670_e24481), (locals.var_vsat_i_dn2 * assign15670_e24481), (locals.var_vsat_i_dn3 * assign15670_e24481), ((locals.var_vsat_i_dn4 * assign15670_e24481) + (locals.var_vsat_i * assign15670_e24481_d_n4)), (locals.var_vsat_i_dn5 * assign15670_e24481), (locals.var_vsat_i_dn6 * assign15670_e24481), (locals.var_vsat_i_dn7 * assign15670_e24481), (locals.var_vsat_i_dn8 * assign15670_e24481), (locals.var_vsat_i_dn9 * assign15670_e24481), (locals.var_vsat_i_dn10 * assign15670_e24481), (locals.var_vsat_i_dn11 * assign15670_e24481), (locals.var_vsat_i_dn13 * assign15670_e24481), (locals.var_vsat_i_dn14 * assign15670_e24481),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign15670_e24484;
        locals.var_vsat_t_dn0 = assign15670_e24484_d_n0;
        locals.var_vsat_t_dn2 = assign15670_e24484_d_n2;
        locals.var_vsat_t_dn3 = assign15670_e24484_d_n3;
        locals.var_vsat_t_dn4 = assign15670_e24484_d_n4;
        locals.var_vsat_t_dn5 = assign15670_e24484_d_n5;
        locals.var_vsat_t_dn6 = assign15670_e24484_d_n6;
        locals.var_vsat_t_dn7 = assign15670_e24484_d_n7;
        locals.var_vsat_t_dn8 = assign15670_e24484_d_n8;
        locals.var_vsat_t_dn9 = assign15670_e24484_d_n9;
        locals.var_vsat_t_dn10 = assign15670_e24484_d_n10;
        locals.var_vsat_t_dn11 = assign15670_e24484_d_n11;
        locals.var_vsat_t_dn13 = assign15670_e24484_d_n13;
        locals.var_vsat_t_dn14 = assign15670_e24484_d_n14;
        locals.var_vsat_t_rv = 0.0;

        let assign15680_e24487: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard270 = assign15680_e24487;
        locals.var_guard270_rv = 0.0;

        let assign15690_e24490: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard271 = assign15690_e24490;
        locals.var_guard271_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15700_e24562, assign15700_e24562_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard270 != 0.0)) && (locals.var_guard271 != 0.0)) {
        let assign15700_e24501: f64 = (-locals.var_vsatr_i);
        let assign15700_e24504: f64 = (-locals.var_atr_i);
        let assign15700_e24506: f64 = (assign15700_e24504 * locals.var_deltemp);
        let assign15700_e24509: f64 = (p.p561 * locals.var_deltemp);
        let assign15700_e24511: f64 = (assign15700_e24509 * locals.var_deltemp);
        let assign15700_e24512: f64 = (assign15700_e24506 + assign15700_e24511);
        let assign15700_e24514: f64 = (-locals.var_vsatr_i);
        let assign15700_e24515: f64 = (assign15700_e24512 - assign15700_e24514);
        let assign15700_e24517: f64 = (assign15700_e24515 - 1e-6);
        let assign15700_e24519: f64 = (-locals.var_atr_i);
        let assign15700_e24521: f64 = (assign15700_e24519 * locals.var_deltemp);
        let assign15700_e24524: f64 = (p.p561 * locals.var_deltemp);
        let assign15700_e24526: f64 = (assign15700_e24524 * locals.var_deltemp);
        let assign15700_e24527: f64 = (assign15700_e24521 + assign15700_e24526);
        let assign15700_e24529: f64 = (-locals.var_vsatr_i);
        let assign15700_e24530: f64 = (assign15700_e24527 - assign15700_e24529);
        let assign15700_e24532: f64 = (assign15700_e24530 - 1e-6);
        let assign15700_e24534: f64 = (-locals.var_atr_i);
        let assign15700_e24536: f64 = (assign15700_e24534 * locals.var_deltemp);
        let assign15700_e24539: f64 = (p.p561 * locals.var_deltemp);
        let assign15700_e24541: f64 = (assign15700_e24539 * locals.var_deltemp);
        let assign15700_e24542: f64 = (assign15700_e24536 + assign15700_e24541);
        let assign15700_e24544: f64 = (-locals.var_vsatr_i);
        let assign15700_e24545: f64 = (assign15700_e24542 - assign15700_e24544);
        let assign15700_e24547: f64 = (assign15700_e24545 - 1e-6);
        let assign15700_e24548: f64 = (assign15700_e24532 * assign15700_e24547);
        let assign15700_e24551: f64 = (-locals.var_vsatr_i);
        let assign15700_e24552: f64 = (4.0 * assign15700_e24551);
        let assign15700_e24554: f64 = (assign15700_e24552 * 1e-6);
        let assign15700_e24555: f64 = (assign15700_e24548 - assign15700_e24554);
        let assign15700_e24556: f64 = (assign15700_e24555).sqrt();
        let assign15700_e24557: f64 = (assign15700_e24517 + assign15700_e24556);
        let assign15700_e24558: f64 = (0.5 * assign15700_e24557);
        let assign15700_e24559: f64 = (assign15700_e24501 + assign15700_e24558);
        let assign15700_e24560: f64 = (locals.var_vsatr_i + assign15700_e24559);
        (assign15700_e24560, (0.5 * (((assign15700_e24504 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15700_e24509 * locals.var_deltemp_dn4))) + (((((assign15700_e24519 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15700_e24524 * locals.var_deltemp_dn4))) * assign15700_e24547) + (assign15700_e24532 * ((assign15700_e24534 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15700_e24539 * locals.var_deltemp_dn4))))) / (2.0 * assign15700_e24556)))),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign15700_e24562;
        locals.var_vsatr_t_dn4 = assign15700_e24562_d_n4;
        locals.var_vsatr_t_rv = 0.0;

        let (assign15710_e24689, assign15710_e24689_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard270 != 0.0)) && (locals.var_guard271 == 0.0)) {
        let assign15710_e24575: f64 = (-locals.var_atr_i);
        let assign15710_e24577: f64 = (assign15710_e24575 * locals.var_deltemp);
        let assign15710_e24578: f64 = (1.0 + assign15710_e24577);
        let assign15710_e24581: f64 = (p.p561 * locals.var_deltemp);
        let assign15710_e24583: f64 = (assign15710_e24581 * locals.var_deltemp);
        let assign15710_e24584: f64 = (assign15710_e24578 + assign15710_e24583);
        let assign15710_e24586: f64 = (assign15710_e24584 - 1e-6);
        let assign15710_e24588: f64 = (-10000.0);
        let assign15710_e24590: f64 = (assign15710_e24588 * 0.001);
        let (assign15710_e24686, assign15710_e24686_d_n4,) = {
            if (!(assign15710_e24586 < assign15710_e24590)) {
                let assign15710_e24596: f64 = (-locals.var_atr_i);
                let assign15710_e24598: f64 = (assign15710_e24596 * locals.var_deltemp);
                let assign15710_e24599: f64 = (1.0 + assign15710_e24598);
                let assign15710_e24602: f64 = (p.p561 * locals.var_deltemp);
                let assign15710_e24604: f64 = (assign15710_e24602 * locals.var_deltemp);
                let assign15710_e24605: f64 = (assign15710_e24599 + assign15710_e24604);
                let assign15710_e24607: f64 = (assign15710_e24605 - 1e-6);
                let assign15710_e24610: f64 = (-locals.var_atr_i);
                let assign15710_e24612: f64 = (assign15710_e24610 * locals.var_deltemp);
                let assign15710_e24613: f64 = (1.0 + assign15710_e24612);
                let assign15710_e24616: f64 = (p.p561 * locals.var_deltemp);
                let assign15710_e24618: f64 = (assign15710_e24616 * locals.var_deltemp);
                let assign15710_e24619: f64 = (assign15710_e24613 + assign15710_e24618);
                let assign15710_e24621: f64 = (assign15710_e24619 - 1e-6);
                let assign15710_e24624: f64 = (-locals.var_atr_i);
                let assign15710_e24626: f64 = (assign15710_e24624 * locals.var_deltemp);
                let assign15710_e24627: f64 = (1.0 + assign15710_e24626);
                let assign15710_e24630: f64 = (p.p561 * locals.var_deltemp);
                let assign15710_e24632: f64 = (assign15710_e24630 * locals.var_deltemp);
                let assign15710_e24633: f64 = (assign15710_e24627 + assign15710_e24632);
                let assign15710_e24635: f64 = (assign15710_e24633 - 1e-6);
                let assign15710_e24636: f64 = (assign15710_e24621 * assign15710_e24635);
                let assign15710_e24639: f64 = (4.0 * 0.001);
                let assign15710_e24641: f64 = (assign15710_e24639 * 0.001);
                let assign15710_e24642: f64 = (assign15710_e24636 + assign15710_e24641);
                let assign15710_e24643: f64 = (assign15710_e24642).sqrt();
                let assign15710_e24644: f64 = (assign15710_e24607 + assign15710_e24643);
                let assign15710_e24645: f64 = (0.5 * assign15710_e24644);
                (assign15710_e24645, (0.5 * (((assign15710_e24596 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15710_e24602 * locals.var_deltemp_dn4))) + (((((assign15710_e24610 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15710_e24616 * locals.var_deltemp_dn4))) * assign15710_e24635) + (assign15710_e24621 * ((assign15710_e24624 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15710_e24630 * locals.var_deltemp_dn4))))) / (2.0 * assign15710_e24643)))),)
            } else {
                let assign15710_e24648: f64 = (-locals.var_atr_i);
                let assign15710_e24650: f64 = (assign15710_e24648 * locals.var_deltemp);
                let assign15710_e24651: f64 = (1.0 + assign15710_e24650);
                let assign15710_e24654: f64 = (p.p561 * locals.var_deltemp);
                let assign15710_e24656: f64 = (assign15710_e24654 * locals.var_deltemp);
                let assign15710_e24657: f64 = (assign15710_e24651 + assign15710_e24656);
                let assign15710_e24659: f64 = (assign15710_e24657 - 1e-6);
                let assign15710_e24661: f64 = (-10000.0);
                let assign15710_e24663: f64 = (assign15710_e24661 * 0.001);
                let (assign15710_e24685, assign15710_e24685_d_n4,) = {
                    if (assign15710_e24659 < assign15710_e24663) {
                        let assign15710_e24666: f64 = (-0.001);
                        let assign15710_e24668: f64 = (assign15710_e24666 * 0.001);
                        let assign15710_e24671: f64 = (-locals.var_atr_i);
                        let assign15710_e24673: f64 = (assign15710_e24671 * locals.var_deltemp);
                        let assign15710_e24674: f64 = (1.0 + assign15710_e24673);
                        let assign15710_e24677: f64 = (p.p561 * locals.var_deltemp);
                        let assign15710_e24679: f64 = (assign15710_e24677 * locals.var_deltemp);
                        let assign15710_e24680: f64 = (assign15710_e24674 + assign15710_e24679);
                        let assign15710_e24682: f64 = (assign15710_e24680 - 1e-6);
                        let assign15710_e24683: f64 = (assign15710_e24668 / assign15710_e24682);
                        (assign15710_e24683, (-((assign15710_e24668 * ((assign15710_e24671 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15710_e24677 * locals.var_deltemp_dn4)))) / (assign15710_e24682 * assign15710_e24682))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15710_e24685, assign15710_e24685_d_n4,)
            }
        };
        let assign15710_e24687: f64 = (locals.var_vsatr_i * assign15710_e24686);
        (assign15710_e24687, (locals.var_vsatr_i * assign15710_e24686_d_n4),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign15710_e24689;
        locals.var_vsatr_t_dn4 = assign15710_e24689_d_n4;
        locals.var_vsatr_t_rv = 0.0;

        let assign15720_e24692: f64 = if locals.var_vsatr_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard272 = assign15720_e24692;
        locals.var_guard272_rv = 0.0;

        let (assign15730_e24703, assign15730_e24703_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard270 != 0.0)) && (locals.var_guard272 != 0.0)) {
        (1000.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign15730_e24703;
        locals.var_vsatr_t_dn4 = assign15730_e24703_d_n4;
        locals.var_vsatr_t_rv = 0.0;

        let assign15740_e24706: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign15740_e24706;
        locals.var_guard273_rv = 0.0;

        let (assign15750_e24776, assign15750_e24776_d_n0, assign15750_e24776_d_n2, assign15750_e24776_d_n3, assign15750_e24776_d_n4, assign15750_e24776_d_n5, assign15750_e24776_d_n6, assign15750_e24776_d_n7, assign15750_e24776_d_n8, assign15750_e24776_d_n9, assign15750_e24776_d_n10, assign15750_e24776_d_n11, assign15750_e24776_d_n13, assign15750_e24776_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard273 != 0.0)) {
        let assign15750_e24715: f64 = (-locals.var_vsat1_i);
        let assign15750_e24718: f64 = (-locals.var_at_i);
        let assign15750_e24720: f64 = (assign15750_e24718 * locals.var_deltemp);
        let assign15750_e24723: f64 = (p.p561 * locals.var_deltemp);
        let assign15750_e24725: f64 = (assign15750_e24723 * locals.var_deltemp);
        let assign15750_e24726: f64 = (assign15750_e24720 + assign15750_e24725);
        let assign15750_e24728: f64 = (-locals.var_vsat1_i);
        let assign15750_e24729: f64 = (assign15750_e24726 - assign15750_e24728);
        let assign15750_e24731: f64 = (assign15750_e24729 - 1e-6);
        let assign15750_e24733: f64 = (-locals.var_at_i);
        let assign15750_e24735: f64 = (assign15750_e24733 * locals.var_deltemp);
        let assign15750_e24738: f64 = (p.p561 * locals.var_deltemp);
        let assign15750_e24740: f64 = (assign15750_e24738 * locals.var_deltemp);
        let assign15750_e24741: f64 = (assign15750_e24735 + assign15750_e24740);
        let assign15750_e24743: f64 = (-locals.var_vsat1_i);
        let assign15750_e24744: f64 = (assign15750_e24741 - assign15750_e24743);
        let assign15750_e24746: f64 = (assign15750_e24744 - 1e-6);
        let assign15750_e24748: f64 = (-locals.var_at_i);
        let assign15750_e24750: f64 = (assign15750_e24748 * locals.var_deltemp);
        let assign15750_e24753: f64 = (p.p561 * locals.var_deltemp);
        let assign15750_e24755: f64 = (assign15750_e24753 * locals.var_deltemp);
        let assign15750_e24756: f64 = (assign15750_e24750 + assign15750_e24755);
        let assign15750_e24758: f64 = (-locals.var_vsat1_i);
        let assign15750_e24759: f64 = (assign15750_e24756 - assign15750_e24758);
        let assign15750_e24761: f64 = (assign15750_e24759 - 1e-6);
        let assign15750_e24762: f64 = (assign15750_e24746 * assign15750_e24761);
        let assign15750_e24765: f64 = (-locals.var_vsat1_i);
        let assign15750_e24766: f64 = (4.0 * assign15750_e24765);
        let assign15750_e24768: f64 = (assign15750_e24766 * 1e-6);
        let assign15750_e24769: f64 = (assign15750_e24762 - assign15750_e24768);
        let assign15750_e24770: f64 = (assign15750_e24769).sqrt();
        let assign15750_e24771: f64 = (assign15750_e24731 + assign15750_e24770);
        let assign15750_e24772: f64 = (0.5 * assign15750_e24771);
        let assign15750_e24773: f64 = (assign15750_e24715 + assign15750_e24772);
        let assign15750_e24774: f64 = (locals.var_vsat1_i + assign15750_e24773);
        (assign15750_e24774, (locals.var_vsat1_i_dn0 + ((-locals.var_vsat1_i_dn0) + (0.5 * ((-(-locals.var_vsat1_i_dn0)) + (((((-(-locals.var_vsat1_i_dn0)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn0)))) - ((4.0 * (-locals.var_vsat1_i_dn0)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn2 + ((-locals.var_vsat1_i_dn2) + (0.5 * ((-(-locals.var_vsat1_i_dn2)) + (((((-(-locals.var_vsat1_i_dn2)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn2)))) - ((4.0 * (-locals.var_vsat1_i_dn2)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn3 + ((-locals.var_vsat1_i_dn3) + (0.5 * ((-(-locals.var_vsat1_i_dn3)) + (((((-(-locals.var_vsat1_i_dn3)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn3)))) - ((4.0 * (-locals.var_vsat1_i_dn3)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn4 + ((-locals.var_vsat1_i_dn4) + (0.5 * ((((assign15750_e24718 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15750_e24723 * locals.var_deltemp_dn4))) - (-locals.var_vsat1_i_dn4)) + (((((((assign15750_e24733 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15750_e24738 * locals.var_deltemp_dn4))) - (-locals.var_vsat1_i_dn4)) * assign15750_e24761) + (assign15750_e24746 * (((assign15750_e24748 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15750_e24753 * locals.var_deltemp_dn4))) - (-locals.var_vsat1_i_dn4)))) - ((4.0 * (-locals.var_vsat1_i_dn4)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn5 + ((-locals.var_vsat1_i_dn5) + (0.5 * ((-(-locals.var_vsat1_i_dn5)) + (((((-(-locals.var_vsat1_i_dn5)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn5)))) - ((4.0 * (-locals.var_vsat1_i_dn5)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn6 + ((-locals.var_vsat1_i_dn6) + (0.5 * ((-(-locals.var_vsat1_i_dn6)) + (((((-(-locals.var_vsat1_i_dn6)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn6)))) - ((4.0 * (-locals.var_vsat1_i_dn6)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn7 + ((-locals.var_vsat1_i_dn7) + (0.5 * ((-(-locals.var_vsat1_i_dn7)) + (((((-(-locals.var_vsat1_i_dn7)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn7)))) - ((4.0 * (-locals.var_vsat1_i_dn7)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn8 + ((-locals.var_vsat1_i_dn8) + (0.5 * ((-(-locals.var_vsat1_i_dn8)) + (((((-(-locals.var_vsat1_i_dn8)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn8)))) - ((4.0 * (-locals.var_vsat1_i_dn8)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn9 + ((-locals.var_vsat1_i_dn9) + (0.5 * ((-(-locals.var_vsat1_i_dn9)) + (((((-(-locals.var_vsat1_i_dn9)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn9)))) - ((4.0 * (-locals.var_vsat1_i_dn9)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn10 + ((-locals.var_vsat1_i_dn10) + (0.5 * ((-(-locals.var_vsat1_i_dn10)) + (((((-(-locals.var_vsat1_i_dn10)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn10)))) - ((4.0 * (-locals.var_vsat1_i_dn10)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn11 + ((-locals.var_vsat1_i_dn11) + (0.5 * ((-(-locals.var_vsat1_i_dn11)) + (((((-(-locals.var_vsat1_i_dn11)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn11)))) - ((4.0 * (-locals.var_vsat1_i_dn11)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn13 + ((-locals.var_vsat1_i_dn13) + (0.5 * ((-(-locals.var_vsat1_i_dn13)) + (((((-(-locals.var_vsat1_i_dn13)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn13)))) - ((4.0 * (-locals.var_vsat1_i_dn13)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn14 + ((-locals.var_vsat1_i_dn14) + (0.5 * ((-(-locals.var_vsat1_i_dn14)) + (((((-(-locals.var_vsat1_i_dn14)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn14)))) - ((4.0 * (-locals.var_vsat1_i_dn14)) * 1e-6)) / (2.0 * assign15750_e24770)))))),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign15750_e24776;
        locals.var_vsat1_t_dn0 = assign15750_e24776_d_n0;
        locals.var_vsat1_t_dn2 = assign15750_e24776_d_n2;
        locals.var_vsat1_t_dn3 = assign15750_e24776_d_n3;
        locals.var_vsat1_t_dn4 = assign15750_e24776_d_n4;
        locals.var_vsat1_t_dn5 = assign15750_e24776_d_n5;
        locals.var_vsat1_t_dn6 = assign15750_e24776_d_n6;
        locals.var_vsat1_t_dn7 = assign15750_e24776_d_n7;
        locals.var_vsat1_t_dn8 = assign15750_e24776_d_n8;
        locals.var_vsat1_t_dn9 = assign15750_e24776_d_n9;
        locals.var_vsat1_t_dn10 = assign15750_e24776_d_n10;
        locals.var_vsat1_t_dn11 = assign15750_e24776_d_n11;
        locals.var_vsat1_t_dn13 = assign15750_e24776_d_n13;
        locals.var_vsat1_t_dn14 = assign15750_e24776_d_n14;
        locals.var_vsat1_t_rv = 0.0;

        let (assign15760_e24901, assign15760_e24901_d_n0, assign15760_e24901_d_n2, assign15760_e24901_d_n3, assign15760_e24901_d_n4, assign15760_e24901_d_n5, assign15760_e24901_d_n6, assign15760_e24901_d_n7, assign15760_e24901_d_n8, assign15760_e24901_d_n9, assign15760_e24901_d_n10, assign15760_e24901_d_n11, assign15760_e24901_d_n13, assign15760_e24901_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard273 == 0.0)) {
        let assign15760_e24787: f64 = (-locals.var_at_i);
        let assign15760_e24789: f64 = (assign15760_e24787 * locals.var_deltemp);
        let assign15760_e24790: f64 = (1.0 + assign15760_e24789);
        let assign15760_e24793: f64 = (p.p561 * locals.var_deltemp);
        let assign15760_e24795: f64 = (assign15760_e24793 * locals.var_deltemp);
        let assign15760_e24796: f64 = (assign15760_e24790 + assign15760_e24795);
        let assign15760_e24798: f64 = (assign15760_e24796 - 1e-6);
        let assign15760_e24800: f64 = (-10000.0);
        let assign15760_e24802: f64 = (assign15760_e24800 * 0.001);
        let (assign15760_e24898, assign15760_e24898_d_n4,) = {
            if (!(assign15760_e24798 < assign15760_e24802)) {
                let assign15760_e24808: f64 = (-locals.var_at_i);
                let assign15760_e24810: f64 = (assign15760_e24808 * locals.var_deltemp);
                let assign15760_e24811: f64 = (1.0 + assign15760_e24810);
                let assign15760_e24814: f64 = (p.p561 * locals.var_deltemp);
                let assign15760_e24816: f64 = (assign15760_e24814 * locals.var_deltemp);
                let assign15760_e24817: f64 = (assign15760_e24811 + assign15760_e24816);
                let assign15760_e24819: f64 = (assign15760_e24817 - 1e-6);
                let assign15760_e24822: f64 = (-locals.var_at_i);
                let assign15760_e24824: f64 = (assign15760_e24822 * locals.var_deltemp);
                let assign15760_e24825: f64 = (1.0 + assign15760_e24824);
                let assign15760_e24828: f64 = (p.p561 * locals.var_deltemp);
                let assign15760_e24830: f64 = (assign15760_e24828 * locals.var_deltemp);
                let assign15760_e24831: f64 = (assign15760_e24825 + assign15760_e24830);
                let assign15760_e24833: f64 = (assign15760_e24831 - 1e-6);
                let assign15760_e24836: f64 = (-locals.var_at_i);
                let assign15760_e24838: f64 = (assign15760_e24836 * locals.var_deltemp);
                let assign15760_e24839: f64 = (1.0 + assign15760_e24838);
                let assign15760_e24842: f64 = (p.p561 * locals.var_deltemp);
                let assign15760_e24844: f64 = (assign15760_e24842 * locals.var_deltemp);
                let assign15760_e24845: f64 = (assign15760_e24839 + assign15760_e24844);
                let assign15760_e24847: f64 = (assign15760_e24845 - 1e-6);
                let assign15760_e24848: f64 = (assign15760_e24833 * assign15760_e24847);
                let assign15760_e24851: f64 = (4.0 * 0.001);
                let assign15760_e24853: f64 = (assign15760_e24851 * 0.001);
                let assign15760_e24854: f64 = (assign15760_e24848 + assign15760_e24853);
                let assign15760_e24855: f64 = (assign15760_e24854).sqrt();
                let assign15760_e24856: f64 = (assign15760_e24819 + assign15760_e24855);
                let assign15760_e24857: f64 = (0.5 * assign15760_e24856);
                (assign15760_e24857, (0.5 * (((assign15760_e24808 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15760_e24814 * locals.var_deltemp_dn4))) + (((((assign15760_e24822 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15760_e24828 * locals.var_deltemp_dn4))) * assign15760_e24847) + (assign15760_e24833 * ((assign15760_e24836 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15760_e24842 * locals.var_deltemp_dn4))))) / (2.0 * assign15760_e24855)))),)
            } else {
                let assign15760_e24860: f64 = (-locals.var_at_i);
                let assign15760_e24862: f64 = (assign15760_e24860 * locals.var_deltemp);
                let assign15760_e24863: f64 = (1.0 + assign15760_e24862);
                let assign15760_e24866: f64 = (p.p561 * locals.var_deltemp);
                let assign15760_e24868: f64 = (assign15760_e24866 * locals.var_deltemp);
                let assign15760_e24869: f64 = (assign15760_e24863 + assign15760_e24868);
                let assign15760_e24871: f64 = (assign15760_e24869 - 1e-6);
                let assign15760_e24873: f64 = (-10000.0);
                let assign15760_e24875: f64 = (assign15760_e24873 * 0.001);
                let (assign15760_e24897, assign15760_e24897_d_n4,) = {
                    if (assign15760_e24871 < assign15760_e24875) {
                        let assign15760_e24878: f64 = (-0.001);
                        let assign15760_e24880: f64 = (assign15760_e24878 * 0.001);
                        let assign15760_e24883: f64 = (-locals.var_at_i);
                        let assign15760_e24885: f64 = (assign15760_e24883 * locals.var_deltemp);
                        let assign15760_e24886: f64 = (1.0 + assign15760_e24885);
                        let assign15760_e24889: f64 = (p.p561 * locals.var_deltemp);
                        let assign15760_e24891: f64 = (assign15760_e24889 * locals.var_deltemp);
                        let assign15760_e24892: f64 = (assign15760_e24886 + assign15760_e24891);
                        let assign15760_e24894: f64 = (assign15760_e24892 - 1e-6);
                        let assign15760_e24895: f64 = (assign15760_e24880 / assign15760_e24894);
                        (assign15760_e24895, (-((assign15760_e24880 * ((assign15760_e24883 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15760_e24889 * locals.var_deltemp_dn4)))) / (assign15760_e24894 * assign15760_e24894))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15760_e24897, assign15760_e24897_d_n4,)
            }
        };
        let assign15760_e24899: f64 = (locals.var_vsat1_i * assign15760_e24898);
        (assign15760_e24899, (locals.var_vsat1_i_dn0 * assign15760_e24898), (locals.var_vsat1_i_dn2 * assign15760_e24898), (locals.var_vsat1_i_dn3 * assign15760_e24898), ((locals.var_vsat1_i_dn4 * assign15760_e24898) + (locals.var_vsat1_i * assign15760_e24898_d_n4)), (locals.var_vsat1_i_dn5 * assign15760_e24898), (locals.var_vsat1_i_dn6 * assign15760_e24898), (locals.var_vsat1_i_dn7 * assign15760_e24898), (locals.var_vsat1_i_dn8 * assign15760_e24898), (locals.var_vsat1_i_dn9 * assign15760_e24898), (locals.var_vsat1_i_dn10 * assign15760_e24898), (locals.var_vsat1_i_dn11 * assign15760_e24898), (locals.var_vsat1_i_dn13 * assign15760_e24898), (locals.var_vsat1_i_dn14 * assign15760_e24898),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign15760_e24901;
        locals.var_vsat1_t_dn0 = assign15760_e24901_d_n0;
        locals.var_vsat1_t_dn2 = assign15760_e24901_d_n2;
        locals.var_vsat1_t_dn3 = assign15760_e24901_d_n3;
        locals.var_vsat1_t_dn4 = assign15760_e24901_d_n4;
        locals.var_vsat1_t_dn5 = assign15760_e24901_d_n5;
        locals.var_vsat1_t_dn6 = assign15760_e24901_d_n6;
        locals.var_vsat1_t_dn7 = assign15760_e24901_d_n7;
        locals.var_vsat1_t_dn8 = assign15760_e24901_d_n8;
        locals.var_vsat1_t_dn9 = assign15760_e24901_d_n9;
        locals.var_vsat1_t_dn10 = assign15760_e24901_d_n10;
        locals.var_vsat1_t_dn11 = assign15760_e24901_d_n11;
        locals.var_vsat1_t_dn13 = assign15760_e24901_d_n13;
        locals.var_vsat1_t_dn14 = assign15760_e24901_d_n14;
        locals.var_vsat1_t_rv = 0.0;

        let assign15770_e24904: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign15770_e24904;
        locals.var_guard274_rv = 0.0;

        let assign15780_e24907: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign15780_e24907;
        locals.var_guard275_rv = 0.0;

        let (assign15790_e24979, assign15790_e24979_d_n0, assign15790_e24979_d_n2, assign15790_e24979_d_n3, assign15790_e24979_d_n4, assign15790_e24979_d_n5, assign15790_e24979_d_n6, assign15790_e24979_d_n7, assign15790_e24979_d_n8, assign15790_e24979_d_n9, assign15790_e24979_d_n10, assign15790_e24979_d_n11, assign15790_e24979_d_n13, assign15790_e24979_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) {
        let assign15790_e24918: f64 = (-locals.var_vsat1r_i);
        let assign15790_e24921: f64 = (-locals.var_at_i);
        let assign15790_e24923: f64 = (assign15790_e24921 * locals.var_deltemp);
        let assign15790_e24926: f64 = (p.p561 * locals.var_deltemp);
        let assign15790_e24928: f64 = (assign15790_e24926 * locals.var_deltemp);
        let assign15790_e24929: f64 = (assign15790_e24923 + assign15790_e24928);
        let assign15790_e24931: f64 = (-locals.var_vsat1r_i);
        let assign15790_e24932: f64 = (assign15790_e24929 - assign15790_e24931);
        let assign15790_e24934: f64 = (assign15790_e24932 - 1e-6);
        let assign15790_e24936: f64 = (-locals.var_at_i);
        let assign15790_e24938: f64 = (assign15790_e24936 * locals.var_deltemp);
        let assign15790_e24941: f64 = (p.p561 * locals.var_deltemp);
        let assign15790_e24943: f64 = (assign15790_e24941 * locals.var_deltemp);
        let assign15790_e24944: f64 = (assign15790_e24938 + assign15790_e24943);
        let assign15790_e24946: f64 = (-locals.var_vsat1r_i);
        let assign15790_e24947: f64 = (assign15790_e24944 - assign15790_e24946);
        let assign15790_e24949: f64 = (assign15790_e24947 - 1e-6);
        let assign15790_e24951: f64 = (-locals.var_at_i);
        let assign15790_e24953: f64 = (assign15790_e24951 * locals.var_deltemp);
        let assign15790_e24956: f64 = (p.p561 * locals.var_deltemp);
        let assign15790_e24958: f64 = (assign15790_e24956 * locals.var_deltemp);
        let assign15790_e24959: f64 = (assign15790_e24953 + assign15790_e24958);
        let assign15790_e24961: f64 = (-locals.var_vsat1r_i);
        let assign15790_e24962: f64 = (assign15790_e24959 - assign15790_e24961);
        let assign15790_e24964: f64 = (assign15790_e24962 - 1e-6);
        let assign15790_e24965: f64 = (assign15790_e24949 * assign15790_e24964);
        let assign15790_e24968: f64 = (-locals.var_vsat1r_i);
        let assign15790_e24969: f64 = (4.0 * assign15790_e24968);
        let assign15790_e24971: f64 = (assign15790_e24969 * 1e-6);
        let assign15790_e24972: f64 = (assign15790_e24965 - assign15790_e24971);
        let assign15790_e24973: f64 = (assign15790_e24972).sqrt();
        let assign15790_e24974: f64 = (assign15790_e24934 + assign15790_e24973);
        let assign15790_e24975: f64 = (0.5 * assign15790_e24974);
        let assign15790_e24976: f64 = (assign15790_e24918 + assign15790_e24975);
        let assign15790_e24977: f64 = (locals.var_vsat1r_i + assign15790_e24976);
        (assign15790_e24977, (locals.var_vsat1r_i_dn0 + ((-locals.var_vsat1r_i_dn0) + (0.5 * ((-(-locals.var_vsat1r_i_dn0)) + (((((-(-locals.var_vsat1r_i_dn0)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn0)))) - ((4.0 * (-locals.var_vsat1r_i_dn0)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn2 + ((-locals.var_vsat1r_i_dn2) + (0.5 * ((-(-locals.var_vsat1r_i_dn2)) + (((((-(-locals.var_vsat1r_i_dn2)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn2)))) - ((4.0 * (-locals.var_vsat1r_i_dn2)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn3 + ((-locals.var_vsat1r_i_dn3) + (0.5 * ((-(-locals.var_vsat1r_i_dn3)) + (((((-(-locals.var_vsat1r_i_dn3)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn3)))) - ((4.0 * (-locals.var_vsat1r_i_dn3)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn4 + ((-locals.var_vsat1r_i_dn4) + (0.5 * ((((assign15790_e24921 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15790_e24926 * locals.var_deltemp_dn4))) - (-locals.var_vsat1r_i_dn4)) + (((((((assign15790_e24936 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15790_e24941 * locals.var_deltemp_dn4))) - (-locals.var_vsat1r_i_dn4)) * assign15790_e24964) + (assign15790_e24949 * (((assign15790_e24951 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15790_e24956 * locals.var_deltemp_dn4))) - (-locals.var_vsat1r_i_dn4)))) - ((4.0 * (-locals.var_vsat1r_i_dn4)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn5 + ((-locals.var_vsat1r_i_dn5) + (0.5 * ((-(-locals.var_vsat1r_i_dn5)) + (((((-(-locals.var_vsat1r_i_dn5)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn5)))) - ((4.0 * (-locals.var_vsat1r_i_dn5)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn6 + ((-locals.var_vsat1r_i_dn6) + (0.5 * ((-(-locals.var_vsat1r_i_dn6)) + (((((-(-locals.var_vsat1r_i_dn6)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn6)))) - ((4.0 * (-locals.var_vsat1r_i_dn6)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn7 + ((-locals.var_vsat1r_i_dn7) + (0.5 * ((-(-locals.var_vsat1r_i_dn7)) + (((((-(-locals.var_vsat1r_i_dn7)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn7)))) - ((4.0 * (-locals.var_vsat1r_i_dn7)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn8 + ((-locals.var_vsat1r_i_dn8) + (0.5 * ((-(-locals.var_vsat1r_i_dn8)) + (((((-(-locals.var_vsat1r_i_dn8)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn8)))) - ((4.0 * (-locals.var_vsat1r_i_dn8)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn9 + ((-locals.var_vsat1r_i_dn9) + (0.5 * ((-(-locals.var_vsat1r_i_dn9)) + (((((-(-locals.var_vsat1r_i_dn9)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn9)))) - ((4.0 * (-locals.var_vsat1r_i_dn9)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn10 + ((-locals.var_vsat1r_i_dn10) + (0.5 * ((-(-locals.var_vsat1r_i_dn10)) + (((((-(-locals.var_vsat1r_i_dn10)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn10)))) - ((4.0 * (-locals.var_vsat1r_i_dn10)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn11 + ((-locals.var_vsat1r_i_dn11) + (0.5 * ((-(-locals.var_vsat1r_i_dn11)) + (((((-(-locals.var_vsat1r_i_dn11)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn11)))) - ((4.0 * (-locals.var_vsat1r_i_dn11)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn13 + ((-locals.var_vsat1r_i_dn13) + (0.5 * ((-(-locals.var_vsat1r_i_dn13)) + (((((-(-locals.var_vsat1r_i_dn13)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn13)))) - ((4.0 * (-locals.var_vsat1r_i_dn13)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn14 + ((-locals.var_vsat1r_i_dn14) + (0.5 * ((-(-locals.var_vsat1r_i_dn14)) + (((((-(-locals.var_vsat1r_i_dn14)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn14)))) - ((4.0 * (-locals.var_vsat1r_i_dn14)) * 1e-6)) / (2.0 * assign15790_e24973)))))),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15790_e24979;
        locals.var_vsat1r_t_dn0 = assign15790_e24979_d_n0;
        locals.var_vsat1r_t_dn2 = assign15790_e24979_d_n2;
        locals.var_vsat1r_t_dn3 = assign15790_e24979_d_n3;
        locals.var_vsat1r_t_dn4 = assign15790_e24979_d_n4;
        locals.var_vsat1r_t_dn5 = assign15790_e24979_d_n5;
        locals.var_vsat1r_t_dn6 = assign15790_e24979_d_n6;
        locals.var_vsat1r_t_dn7 = assign15790_e24979_d_n7;
        locals.var_vsat1r_t_dn8 = assign15790_e24979_d_n8;
        locals.var_vsat1r_t_dn9 = assign15790_e24979_d_n9;
        locals.var_vsat1r_t_dn10 = assign15790_e24979_d_n10;
        locals.var_vsat1r_t_dn11 = assign15790_e24979_d_n11;
        locals.var_vsat1r_t_dn13 = assign15790_e24979_d_n13;
        locals.var_vsat1r_t_dn14 = assign15790_e24979_d_n14;
        locals.var_vsat1r_t_rv = 0.0;

        let (assign15800_e25106, assign15800_e25106_d_n0, assign15800_e25106_d_n2, assign15800_e25106_d_n3, assign15800_e25106_d_n4, assign15800_e25106_d_n5, assign15800_e25106_d_n6, assign15800_e25106_d_n7, assign15800_e25106_d_n8, assign15800_e25106_d_n9, assign15800_e25106_d_n10, assign15800_e25106_d_n11, assign15800_e25106_d_n13, assign15800_e25106_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 == 0.0)) {
        let assign15800_e24992: f64 = (-locals.var_at_i);
        let assign15800_e24994: f64 = (assign15800_e24992 * locals.var_deltemp);
        let assign15800_e24995: f64 = (1.0 + assign15800_e24994);
        let assign15800_e24998: f64 = (p.p561 * locals.var_deltemp);
        let assign15800_e25000: f64 = (assign15800_e24998 * locals.var_deltemp);
        let assign15800_e25001: f64 = (assign15800_e24995 + assign15800_e25000);
        let assign15800_e25003: f64 = (assign15800_e25001 - 1e-6);
        let assign15800_e25005: f64 = (-10000.0);
        let assign15800_e25007: f64 = (assign15800_e25005 * 0.001);
        let (assign15800_e25103, assign15800_e25103_d_n4,) = {
            if (!(assign15800_e25003 < assign15800_e25007)) {
                let assign15800_e25013: f64 = (-locals.var_at_i);
                let assign15800_e25015: f64 = (assign15800_e25013 * locals.var_deltemp);
                let assign15800_e25016: f64 = (1.0 + assign15800_e25015);
                let assign15800_e25019: f64 = (p.p561 * locals.var_deltemp);
                let assign15800_e25021: f64 = (assign15800_e25019 * locals.var_deltemp);
                let assign15800_e25022: f64 = (assign15800_e25016 + assign15800_e25021);
                let assign15800_e25024: f64 = (assign15800_e25022 - 1e-6);
                let assign15800_e25027: f64 = (-locals.var_at_i);
                let assign15800_e25029: f64 = (assign15800_e25027 * locals.var_deltemp);
                let assign15800_e25030: f64 = (1.0 + assign15800_e25029);
                let assign15800_e25033: f64 = (p.p561 * locals.var_deltemp);
                let assign15800_e25035: f64 = (assign15800_e25033 * locals.var_deltemp);
                let assign15800_e25036: f64 = (assign15800_e25030 + assign15800_e25035);
                let assign15800_e25038: f64 = (assign15800_e25036 - 1e-6);
                let assign15800_e25041: f64 = (-locals.var_at_i);
                let assign15800_e25043: f64 = (assign15800_e25041 * locals.var_deltemp);
                let assign15800_e25044: f64 = (1.0 + assign15800_e25043);
                let assign15800_e25047: f64 = (p.p561 * locals.var_deltemp);
                let assign15800_e25049: f64 = (assign15800_e25047 * locals.var_deltemp);
                let assign15800_e25050: f64 = (assign15800_e25044 + assign15800_e25049);
                let assign15800_e25052: f64 = (assign15800_e25050 - 1e-6);
                let assign15800_e25053: f64 = (assign15800_e25038 * assign15800_e25052);
                let assign15800_e25056: f64 = (4.0 * 0.001);
                let assign15800_e25058: f64 = (assign15800_e25056 * 0.001);
                let assign15800_e25059: f64 = (assign15800_e25053 + assign15800_e25058);
                let assign15800_e25060: f64 = (assign15800_e25059).sqrt();
                let assign15800_e25061: f64 = (assign15800_e25024 + assign15800_e25060);
                let assign15800_e25062: f64 = (0.5 * assign15800_e25061);
                (assign15800_e25062, (0.5 * (((assign15800_e25013 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15800_e25019 * locals.var_deltemp_dn4))) + (((((assign15800_e25027 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15800_e25033 * locals.var_deltemp_dn4))) * assign15800_e25052) + (assign15800_e25038 * ((assign15800_e25041 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15800_e25047 * locals.var_deltemp_dn4))))) / (2.0 * assign15800_e25060)))),)
            } else {
                let assign15800_e25065: f64 = (-locals.var_at_i);
                let assign15800_e25067: f64 = (assign15800_e25065 * locals.var_deltemp);
                let assign15800_e25068: f64 = (1.0 + assign15800_e25067);
                let assign15800_e25071: f64 = (p.p561 * locals.var_deltemp);
                let assign15800_e25073: f64 = (assign15800_e25071 * locals.var_deltemp);
                let assign15800_e25074: f64 = (assign15800_e25068 + assign15800_e25073);
                let assign15800_e25076: f64 = (assign15800_e25074 - 1e-6);
                let assign15800_e25078: f64 = (-10000.0);
                let assign15800_e25080: f64 = (assign15800_e25078 * 0.001);
                let (assign15800_e25102, assign15800_e25102_d_n4,) = {
                    if (assign15800_e25076 < assign15800_e25080) {
                        let assign15800_e25083: f64 = (-0.001);
                        let assign15800_e25085: f64 = (assign15800_e25083 * 0.001);
                        let assign15800_e25088: f64 = (-locals.var_at_i);
                        let assign15800_e25090: f64 = (assign15800_e25088 * locals.var_deltemp);
                        let assign15800_e25091: f64 = (1.0 + assign15800_e25090);
                        let assign15800_e25094: f64 = (p.p561 * locals.var_deltemp);
                        let assign15800_e25096: f64 = (assign15800_e25094 * locals.var_deltemp);
                        let assign15800_e25097: f64 = (assign15800_e25091 + assign15800_e25096);
                        let assign15800_e25099: f64 = (assign15800_e25097 - 1e-6);
                        let assign15800_e25100: f64 = (assign15800_e25085 / assign15800_e25099);
                        (assign15800_e25100, (-((assign15800_e25085 * ((assign15800_e25088 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15800_e25094 * locals.var_deltemp_dn4)))) / (assign15800_e25099 * assign15800_e25099))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15800_e25102, assign15800_e25102_d_n4,)
            }
        };
        let assign15800_e25104: f64 = (locals.var_vsat1r_i * assign15800_e25103);
        (assign15800_e25104, (locals.var_vsat1r_i_dn0 * assign15800_e25103), (locals.var_vsat1r_i_dn2 * assign15800_e25103), (locals.var_vsat1r_i_dn3 * assign15800_e25103), ((locals.var_vsat1r_i_dn4 * assign15800_e25103) + (locals.var_vsat1r_i * assign15800_e25103_d_n4)), (locals.var_vsat1r_i_dn5 * assign15800_e25103), (locals.var_vsat1r_i_dn6 * assign15800_e25103), (locals.var_vsat1r_i_dn7 * assign15800_e25103), (locals.var_vsat1r_i_dn8 * assign15800_e25103), (locals.var_vsat1r_i_dn9 * assign15800_e25103), (locals.var_vsat1r_i_dn10 * assign15800_e25103), (locals.var_vsat1r_i_dn11 * assign15800_e25103), (locals.var_vsat1r_i_dn13 * assign15800_e25103), (locals.var_vsat1r_i_dn14 * assign15800_e25103),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15800_e25106;
        locals.var_vsat1r_t_dn0 = assign15800_e25106_d_n0;
        locals.var_vsat1r_t_dn2 = assign15800_e25106_d_n2;
        locals.var_vsat1r_t_dn3 = assign15800_e25106_d_n3;
        locals.var_vsat1r_t_dn4 = assign15800_e25106_d_n4;
        locals.var_vsat1r_t_dn5 = assign15800_e25106_d_n5;
        locals.var_vsat1r_t_dn6 = assign15800_e25106_d_n6;
        locals.var_vsat1r_t_dn7 = assign15800_e25106_d_n7;
        locals.var_vsat1r_t_dn8 = assign15800_e25106_d_n8;
        locals.var_vsat1r_t_dn9 = assign15800_e25106_d_n9;
        locals.var_vsat1r_t_dn10 = assign15800_e25106_d_n10;
        locals.var_vsat1r_t_dn11 = assign15800_e25106_d_n11;
        locals.var_vsat1r_t_dn13 = assign15800_e25106_d_n13;
        locals.var_vsat1r_t_dn14 = assign15800_e25106_d_n14;
        locals.var_vsat1r_t_rv = 0.0;

        let assign15810_e25109: f64 = if locals.var_vsat1r_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign15810_e25109;
        locals.var_guard276_rv = 0.0;

        let (assign15820_e25120, assign15820_e25120_d_n0, assign15820_e25120_d_n2, assign15820_e25120_d_n3, assign15820_e25120_d_n4, assign15820_e25120_d_n5, assign15820_e25120_d_n6, assign15820_e25120_d_n7, assign15820_e25120_d_n8, assign15820_e25120_d_n9, assign15820_e25120_d_n10, assign15820_e25120_d_n11, assign15820_e25120_d_n13, assign15820_e25120_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard274 != 0.0)) && (locals.var_guard276 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15820_e25120;
        locals.var_vsat1r_t_dn0 = assign15820_e25120_d_n0;
        locals.var_vsat1r_t_dn2 = assign15820_e25120_d_n2;
        locals.var_vsat1r_t_dn3 = assign15820_e25120_d_n3;
        locals.var_vsat1r_t_dn4 = assign15820_e25120_d_n4;
        locals.var_vsat1r_t_dn5 = assign15820_e25120_d_n5;
        locals.var_vsat1r_t_dn6 = assign15820_e25120_d_n6;
        locals.var_vsat1r_t_dn7 = assign15820_e25120_d_n7;
        locals.var_vsat1r_t_dn8 = assign15820_e25120_d_n8;
        locals.var_vsat1r_t_dn9 = assign15820_e25120_d_n9;
        locals.var_vsat1r_t_dn10 = assign15820_e25120_d_n10;
        locals.var_vsat1r_t_dn11 = assign15820_e25120_d_n11;
        locals.var_vsat1r_t_dn13 = assign15820_e25120_d_n13;
        locals.var_vsat1r_t_dn14 = assign15820_e25120_d_n14;
        locals.var_vsat1r_t_rv = 0.0;

        let assign15830_e25123: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign15830_e25123;
        locals.var_guard277_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15840_e25193, assign15840_e25193_d_n0, assign15840_e25193_d_n2, assign15840_e25193_d_n3, assign15840_e25193_d_n4, assign15840_e25193_d_n5, assign15840_e25193_d_n6, assign15840_e25193_d_n7, assign15840_e25193_d_n8, assign15840_e25193_d_n9, assign15840_e25193_d_n10, assign15840_e25193_d_n11, assign15840_e25193_d_n13, assign15840_e25193_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard277 != 0.0)) {
        let assign15840_e25132: f64 = (-locals.var_vsatcv_i);
        let assign15840_e25135: f64 = (-locals.var_atcv_i);
        let assign15840_e25137: f64 = (assign15840_e25135 * locals.var_deltemp);
        let assign15840_e25140: f64 = (p.p574 * locals.var_deltemp);
        let assign15840_e25142: f64 = (assign15840_e25140 * locals.var_deltemp);
        let assign15840_e25143: f64 = (assign15840_e25137 + assign15840_e25142);
        let assign15840_e25145: f64 = (-locals.var_vsatcv_i);
        let assign15840_e25146: f64 = (assign15840_e25143 - assign15840_e25145);
        let assign15840_e25148: f64 = (assign15840_e25146 - 1e-6);
        let assign15840_e25150: f64 = (-locals.var_atcv_i);
        let assign15840_e25152: f64 = (assign15840_e25150 * locals.var_deltemp);
        let assign15840_e25155: f64 = (p.p574 * locals.var_deltemp);
        let assign15840_e25157: f64 = (assign15840_e25155 * locals.var_deltemp);
        let assign15840_e25158: f64 = (assign15840_e25152 + assign15840_e25157);
        let assign15840_e25160: f64 = (-locals.var_vsatcv_i);
        let assign15840_e25161: f64 = (assign15840_e25158 - assign15840_e25160);
        let assign15840_e25163: f64 = (assign15840_e25161 - 1e-6);
        let assign15840_e25165: f64 = (-locals.var_atcv_i);
        let assign15840_e25167: f64 = (assign15840_e25165 * locals.var_deltemp);
        let assign15840_e25170: f64 = (p.p574 * locals.var_deltemp);
        let assign15840_e25172: f64 = (assign15840_e25170 * locals.var_deltemp);
        let assign15840_e25173: f64 = (assign15840_e25167 + assign15840_e25172);
        let assign15840_e25175: f64 = (-locals.var_vsatcv_i);
        let assign15840_e25176: f64 = (assign15840_e25173 - assign15840_e25175);
        let assign15840_e25178: f64 = (assign15840_e25176 - 1e-6);
        let assign15840_e25179: f64 = (assign15840_e25163 * assign15840_e25178);
        let assign15840_e25182: f64 = (-locals.var_vsatcv_i);
        let assign15840_e25183: f64 = (4.0 * assign15840_e25182);
        let assign15840_e25185: f64 = (assign15840_e25183 * 1e-6);
        let assign15840_e25186: f64 = (assign15840_e25179 - assign15840_e25185);
        let assign15840_e25187: f64 = (assign15840_e25186).sqrt();
        let assign15840_e25188: f64 = (assign15840_e25148 + assign15840_e25187);
        let assign15840_e25189: f64 = (0.5 * assign15840_e25188);
        let assign15840_e25190: f64 = (assign15840_e25132 + assign15840_e25189);
        let assign15840_e25191: f64 = (locals.var_vsatcv_i + assign15840_e25190);
        (assign15840_e25191, (locals.var_vsatcv_i_dn0 + ((-locals.var_vsatcv_i_dn0) + (0.5 * ((-(-locals.var_vsatcv_i_dn0)) + (((((-(-locals.var_vsatcv_i_dn0)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn0)))) - ((4.0 * (-locals.var_vsatcv_i_dn0)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn2 + ((-locals.var_vsatcv_i_dn2) + (0.5 * ((-(-locals.var_vsatcv_i_dn2)) + (((((-(-locals.var_vsatcv_i_dn2)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn2)))) - ((4.0 * (-locals.var_vsatcv_i_dn2)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn3 + ((-locals.var_vsatcv_i_dn3) + (0.5 * ((-(-locals.var_vsatcv_i_dn3)) + (((((-(-locals.var_vsatcv_i_dn3)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn3)))) - ((4.0 * (-locals.var_vsatcv_i_dn3)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn4 + ((-locals.var_vsatcv_i_dn4) + (0.5 * ((((assign15840_e25135 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15840_e25140 * locals.var_deltemp_dn4))) - (-locals.var_vsatcv_i_dn4)) + (((((((assign15840_e25150 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15840_e25155 * locals.var_deltemp_dn4))) - (-locals.var_vsatcv_i_dn4)) * assign15840_e25178) + (assign15840_e25163 * (((assign15840_e25165 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15840_e25170 * locals.var_deltemp_dn4))) - (-locals.var_vsatcv_i_dn4)))) - ((4.0 * (-locals.var_vsatcv_i_dn4)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn5 + ((-locals.var_vsatcv_i_dn5) + (0.5 * ((-(-locals.var_vsatcv_i_dn5)) + (((((-(-locals.var_vsatcv_i_dn5)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn5)))) - ((4.0 * (-locals.var_vsatcv_i_dn5)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn6 + ((-locals.var_vsatcv_i_dn6) + (0.5 * ((-(-locals.var_vsatcv_i_dn6)) + (((((-(-locals.var_vsatcv_i_dn6)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn6)))) - ((4.0 * (-locals.var_vsatcv_i_dn6)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn7 + ((-locals.var_vsatcv_i_dn7) + (0.5 * ((-(-locals.var_vsatcv_i_dn7)) + (((((-(-locals.var_vsatcv_i_dn7)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn7)))) - ((4.0 * (-locals.var_vsatcv_i_dn7)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn8 + ((-locals.var_vsatcv_i_dn8) + (0.5 * ((-(-locals.var_vsatcv_i_dn8)) + (((((-(-locals.var_vsatcv_i_dn8)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn8)))) - ((4.0 * (-locals.var_vsatcv_i_dn8)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn9 + ((-locals.var_vsatcv_i_dn9) + (0.5 * ((-(-locals.var_vsatcv_i_dn9)) + (((((-(-locals.var_vsatcv_i_dn9)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn9)))) - ((4.0 * (-locals.var_vsatcv_i_dn9)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn10 + ((-locals.var_vsatcv_i_dn10) + (0.5 * ((-(-locals.var_vsatcv_i_dn10)) + (((((-(-locals.var_vsatcv_i_dn10)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn10)))) - ((4.0 * (-locals.var_vsatcv_i_dn10)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn11 + ((-locals.var_vsatcv_i_dn11) + (0.5 * ((-(-locals.var_vsatcv_i_dn11)) + (((((-(-locals.var_vsatcv_i_dn11)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn11)))) - ((4.0 * (-locals.var_vsatcv_i_dn11)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn13 + ((-locals.var_vsatcv_i_dn13) + (0.5 * ((-(-locals.var_vsatcv_i_dn13)) + (((((-(-locals.var_vsatcv_i_dn13)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn13)))) - ((4.0 * (-locals.var_vsatcv_i_dn13)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn14 + ((-locals.var_vsatcv_i_dn14) + (0.5 * ((-(-locals.var_vsatcv_i_dn14)) + (((((-(-locals.var_vsatcv_i_dn14)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn14)))) - ((4.0 * (-locals.var_vsatcv_i_dn14)) * 1e-6)) / (2.0 * assign15840_e25187)))))),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign15840_e25193;
        locals.var_vsatcv_t_dn0 = assign15840_e25193_d_n0;
        locals.var_vsatcv_t_dn2 = assign15840_e25193_d_n2;
        locals.var_vsatcv_t_dn3 = assign15840_e25193_d_n3;
        locals.var_vsatcv_t_dn4 = assign15840_e25193_d_n4;
        locals.var_vsatcv_t_dn5 = assign15840_e25193_d_n5;
        locals.var_vsatcv_t_dn6 = assign15840_e25193_d_n6;
        locals.var_vsatcv_t_dn7 = assign15840_e25193_d_n7;
        locals.var_vsatcv_t_dn8 = assign15840_e25193_d_n8;
        locals.var_vsatcv_t_dn9 = assign15840_e25193_d_n9;
        locals.var_vsatcv_t_dn10 = assign15840_e25193_d_n10;
        locals.var_vsatcv_t_dn11 = assign15840_e25193_d_n11;
        locals.var_vsatcv_t_dn13 = assign15840_e25193_d_n13;
        locals.var_vsatcv_t_dn14 = assign15840_e25193_d_n14;
        locals.var_vsatcv_t_rv = 0.0;

        let (assign15850_e25318, assign15850_e25318_d_n0, assign15850_e25318_d_n2, assign15850_e25318_d_n3, assign15850_e25318_d_n4, assign15850_e25318_d_n5, assign15850_e25318_d_n6, assign15850_e25318_d_n7, assign15850_e25318_d_n8, assign15850_e25318_d_n9, assign15850_e25318_d_n10, assign15850_e25318_d_n11, assign15850_e25318_d_n13, assign15850_e25318_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard277 == 0.0)) {
        let assign15850_e25204: f64 = (-locals.var_atcv_i);
        let assign15850_e25206: f64 = (assign15850_e25204 * locals.var_deltemp);
        let assign15850_e25207: f64 = (1.0 + assign15850_e25206);
        let assign15850_e25210: f64 = (p.p574 * locals.var_deltemp);
        let assign15850_e25212: f64 = (assign15850_e25210 * locals.var_deltemp);
        let assign15850_e25213: f64 = (assign15850_e25207 + assign15850_e25212);
        let assign15850_e25215: f64 = (assign15850_e25213 - 1e-6);
        let assign15850_e25217: f64 = (-10000.0);
        let assign15850_e25219: f64 = (assign15850_e25217 * 0.001);
        let (assign15850_e25315, assign15850_e25315_d_n4,) = {
            if (!(assign15850_e25215 < assign15850_e25219)) {
                let assign15850_e25225: f64 = (-locals.var_atcv_i);
                let assign15850_e25227: f64 = (assign15850_e25225 * locals.var_deltemp);
                let assign15850_e25228: f64 = (1.0 + assign15850_e25227);
                let assign15850_e25231: f64 = (p.p574 * locals.var_deltemp);
                let assign15850_e25233: f64 = (assign15850_e25231 * locals.var_deltemp);
                let assign15850_e25234: f64 = (assign15850_e25228 + assign15850_e25233);
                let assign15850_e25236: f64 = (assign15850_e25234 - 1e-6);
                let assign15850_e25239: f64 = (-locals.var_atcv_i);
                let assign15850_e25241: f64 = (assign15850_e25239 * locals.var_deltemp);
                let assign15850_e25242: f64 = (1.0 + assign15850_e25241);
                let assign15850_e25245: f64 = (p.p574 * locals.var_deltemp);
                let assign15850_e25247: f64 = (assign15850_e25245 * locals.var_deltemp);
                let assign15850_e25248: f64 = (assign15850_e25242 + assign15850_e25247);
                let assign15850_e25250: f64 = (assign15850_e25248 - 1e-6);
                let assign15850_e25253: f64 = (-locals.var_atcv_i);
                let assign15850_e25255: f64 = (assign15850_e25253 * locals.var_deltemp);
                let assign15850_e25256: f64 = (1.0 + assign15850_e25255);
                let assign15850_e25259: f64 = (p.p574 * locals.var_deltemp);
                let assign15850_e25261: f64 = (assign15850_e25259 * locals.var_deltemp);
                let assign15850_e25262: f64 = (assign15850_e25256 + assign15850_e25261);
                let assign15850_e25264: f64 = (assign15850_e25262 - 1e-6);
                let assign15850_e25265: f64 = (assign15850_e25250 * assign15850_e25264);
                let assign15850_e25268: f64 = (4.0 * 0.001);
                let assign15850_e25270: f64 = (assign15850_e25268 * 0.001);
                let assign15850_e25271: f64 = (assign15850_e25265 + assign15850_e25270);
                let assign15850_e25272: f64 = (assign15850_e25271).sqrt();
                let assign15850_e25273: f64 = (assign15850_e25236 + assign15850_e25272);
                let assign15850_e25274: f64 = (0.5 * assign15850_e25273);
                (assign15850_e25274, (0.5 * (((assign15850_e25225 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15850_e25231 * locals.var_deltemp_dn4))) + (((((assign15850_e25239 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15850_e25245 * locals.var_deltemp_dn4))) * assign15850_e25264) + (assign15850_e25250 * ((assign15850_e25253 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15850_e25259 * locals.var_deltemp_dn4))))) / (2.0 * assign15850_e25272)))),)
            } else {
                let assign15850_e25277: f64 = (-locals.var_atcv_i);
                let assign15850_e25279: f64 = (assign15850_e25277 * locals.var_deltemp);
                let assign15850_e25280: f64 = (1.0 + assign15850_e25279);
                let assign15850_e25283: f64 = (p.p574 * locals.var_deltemp);
                let assign15850_e25285: f64 = (assign15850_e25283 * locals.var_deltemp);
                let assign15850_e25286: f64 = (assign15850_e25280 + assign15850_e25285);
                let assign15850_e25288: f64 = (assign15850_e25286 - 1e-6);
                let assign15850_e25290: f64 = (-10000.0);
                let assign15850_e25292: f64 = (assign15850_e25290 * 0.001);
                let (assign15850_e25314, assign15850_e25314_d_n4,) = {
                    if (assign15850_e25288 < assign15850_e25292) {
                        let assign15850_e25295: f64 = (-0.001);
                        let assign15850_e25297: f64 = (assign15850_e25295 * 0.001);
                        let assign15850_e25300: f64 = (-locals.var_atcv_i);
                        let assign15850_e25302: f64 = (assign15850_e25300 * locals.var_deltemp);
                        let assign15850_e25303: f64 = (1.0 + assign15850_e25302);
                        let assign15850_e25306: f64 = (p.p574 * locals.var_deltemp);
                        let assign15850_e25308: f64 = (assign15850_e25306 * locals.var_deltemp);
                        let assign15850_e25309: f64 = (assign15850_e25303 + assign15850_e25308);
                        let assign15850_e25311: f64 = (assign15850_e25309 - 1e-6);
                        let assign15850_e25312: f64 = (assign15850_e25297 / assign15850_e25311);
                        (assign15850_e25312, (-((assign15850_e25297 * ((assign15850_e25300 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15850_e25306 * locals.var_deltemp_dn4)))) / (assign15850_e25311 * assign15850_e25311))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15850_e25314, assign15850_e25314_d_n4,)
            }
        };
        let assign15850_e25316: f64 = (locals.var_vsatcv_i * assign15850_e25315);
        (assign15850_e25316, (locals.var_vsatcv_i_dn0 * assign15850_e25315), (locals.var_vsatcv_i_dn2 * assign15850_e25315), (locals.var_vsatcv_i_dn3 * assign15850_e25315), ((locals.var_vsatcv_i_dn4 * assign15850_e25315) + (locals.var_vsatcv_i * assign15850_e25315_d_n4)), (locals.var_vsatcv_i_dn5 * assign15850_e25315), (locals.var_vsatcv_i_dn6 * assign15850_e25315), (locals.var_vsatcv_i_dn7 * assign15850_e25315), (locals.var_vsatcv_i_dn8 * assign15850_e25315), (locals.var_vsatcv_i_dn9 * assign15850_e25315), (locals.var_vsatcv_i_dn10 * assign15850_e25315), (locals.var_vsatcv_i_dn11 * assign15850_e25315), (locals.var_vsatcv_i_dn13 * assign15850_e25315), (locals.var_vsatcv_i_dn14 * assign15850_e25315),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign15850_e25318;
        locals.var_vsatcv_t_dn0 = assign15850_e25318_d_n0;
        locals.var_vsatcv_t_dn2 = assign15850_e25318_d_n2;
        locals.var_vsatcv_t_dn3 = assign15850_e25318_d_n3;
        locals.var_vsatcv_t_dn4 = assign15850_e25318_d_n4;
        locals.var_vsatcv_t_dn5 = assign15850_e25318_d_n5;
        locals.var_vsatcv_t_dn6 = assign15850_e25318_d_n6;
        locals.var_vsatcv_t_dn7 = assign15850_e25318_d_n7;
        locals.var_vsatcv_t_dn8 = assign15850_e25318_d_n8;
        locals.var_vsatcv_t_dn9 = assign15850_e25318_d_n9;
        locals.var_vsatcv_t_dn10 = assign15850_e25318_d_n10;
        locals.var_vsatcv_t_dn11 = assign15850_e25318_d_n11;
        locals.var_vsatcv_t_dn13 = assign15850_e25318_d_n13;
        locals.var_vsatcv_t_dn14 = assign15850_e25318_d_n14;
        locals.var_vsatcv_t_rv = 0.0;

        let (assign15860_e25446, assign15860_e25446_d_n0, assign15860_e25446_d_n2, assign15860_e25446_d_n3, assign15860_e25446_d_n4, assign15860_e25446_d_n5, assign15860_e25446_d_n6, assign15860_e25446_d_n7, assign15860_e25446_d_n8, assign15860_e25446_d_n9, assign15860_e25446_d_n10, assign15860_e25446_d_n11, assign15860_e25446_d_n13, assign15860_e25446_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15860_e25327: f64 = (p.p450 * locals.var_deltemp);
        let assign15860_e25328: f64 = (1.0 + assign15860_e25327);
        let assign15860_e25331: f64 = (p.p451 * locals.var_deltemp);
        let assign15860_e25333: f64 = (assign15860_e25331 * locals.var_deltemp);
        let assign15860_e25334: f64 = (assign15860_e25328 + assign15860_e25333);
        let assign15860_e25335: f64 = (locals.var_mexp_i * assign15860_e25334);
        let assign15860_e25337: f64 = (assign15860_e25335 - 2.0);
        let assign15860_e25339: f64 = (-10000.0);
        let assign15860_e25341: f64 = (assign15860_e25339 * 0.001);
        let (assign15860_e25442, assign15860_e25442_d_n0, assign15860_e25442_d_n2, assign15860_e25442_d_n3, assign15860_e25442_d_n4, assign15860_e25442_d_n5, assign15860_e25442_d_n6, assign15860_e25442_d_n7, assign15860_e25442_d_n8, assign15860_e25442_d_n9, assign15860_e25442_d_n10, assign15860_e25442_d_n11, assign15860_e25442_d_n13, assign15860_e25442_d_n14,) = {
            if (!(assign15860_e25337 < assign15860_e25341)) {
                let assign15860_e25349: f64 = (p.p450 * locals.var_deltemp);
                let assign15860_e25350: f64 = (1.0 + assign15860_e25349);
                let assign15860_e25353: f64 = (p.p451 * locals.var_deltemp);
                let assign15860_e25355: f64 = (assign15860_e25353 * locals.var_deltemp);
                let assign15860_e25356: f64 = (assign15860_e25350 + assign15860_e25355);
                let assign15860_e25357: f64 = (locals.var_mexp_i * assign15860_e25356);
                let assign15860_e25359: f64 = (assign15860_e25357 - 2.0);
                let assign15860_e25364: f64 = (p.p450 * locals.var_deltemp);
                let assign15860_e25365: f64 = (1.0 + assign15860_e25364);
                let assign15860_e25368: f64 = (p.p451 * locals.var_deltemp);
                let assign15860_e25370: f64 = (assign15860_e25368 * locals.var_deltemp);
                let assign15860_e25371: f64 = (assign15860_e25365 + assign15860_e25370);
                let assign15860_e25372: f64 = (locals.var_mexp_i * assign15860_e25371);
                let assign15860_e25374: f64 = (assign15860_e25372 - 2.0);
                let assign15860_e25379: f64 = (p.p450 * locals.var_deltemp);
                let assign15860_e25380: f64 = (1.0 + assign15860_e25379);
                let assign15860_e25383: f64 = (p.p451 * locals.var_deltemp);
                let assign15860_e25385: f64 = (assign15860_e25383 * locals.var_deltemp);
                let assign15860_e25386: f64 = (assign15860_e25380 + assign15860_e25385);
                let assign15860_e25387: f64 = (locals.var_mexp_i * assign15860_e25386);
                let assign15860_e25389: f64 = (assign15860_e25387 - 2.0);
                let assign15860_e25390: f64 = (assign15860_e25374 * assign15860_e25389);
                let assign15860_e25393: f64 = (4.0 * 0.001);
                let assign15860_e25395: f64 = (assign15860_e25393 * 0.001);
                let assign15860_e25396: f64 = (assign15860_e25390 + assign15860_e25395);
                let assign15860_e25397: f64 = (assign15860_e25396).sqrt();
                let assign15860_e25398: f64 = (assign15860_e25359 + assign15860_e25397);
                let assign15860_e25399: f64 = (0.5 * assign15860_e25398);
                (assign15860_e25399, (0.5 * ((locals.var_mexp_i_dn0 * assign15860_e25356) + ((((locals.var_mexp_i_dn0 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn0 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn2 * assign15860_e25356) + ((((locals.var_mexp_i_dn2 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn2 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn3 * assign15860_e25356) + ((((locals.var_mexp_i_dn3 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn3 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * (((locals.var_mexp_i_dn4 * assign15860_e25356) + (locals.var_mexp_i * ((p.p450 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15860_e25353 * locals.var_deltemp_dn4))))) + (((((locals.var_mexp_i_dn4 * assign15860_e25371) + (locals.var_mexp_i * ((p.p450 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15860_e25368 * locals.var_deltemp_dn4))))) * assign15860_e25389) + (assign15860_e25374 * ((locals.var_mexp_i_dn4 * assign15860_e25386) + (locals.var_mexp_i * ((p.p450 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15860_e25383 * locals.var_deltemp_dn4))))))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn5 * assign15860_e25356) + ((((locals.var_mexp_i_dn5 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn5 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn6 * assign15860_e25356) + ((((locals.var_mexp_i_dn6 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn6 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn7 * assign15860_e25356) + ((((locals.var_mexp_i_dn7 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn7 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn8 * assign15860_e25356) + ((((locals.var_mexp_i_dn8 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn8 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn9 * assign15860_e25356) + ((((locals.var_mexp_i_dn9 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn9 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn10 * assign15860_e25356) + ((((locals.var_mexp_i_dn10 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn10 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn11 * assign15860_e25356) + ((((locals.var_mexp_i_dn11 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn11 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn13 * assign15860_e25356) + ((((locals.var_mexp_i_dn13 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn13 * assign15860_e25386))) / (2.0 * assign15860_e25397)))), (0.5 * ((locals.var_mexp_i_dn14 * assign15860_e25356) + ((((locals.var_mexp_i_dn14 * assign15860_e25371) * assign15860_e25389) + (assign15860_e25374 * (locals.var_mexp_i_dn14 * assign15860_e25386))) / (2.0 * assign15860_e25397)))),)
            } else {
                let assign15860_e25404: f64 = (p.p450 * locals.var_deltemp);
                let assign15860_e25405: f64 = (1.0 + assign15860_e25404);
                let assign15860_e25408: f64 = (p.p451 * locals.var_deltemp);
                let assign15860_e25410: f64 = (assign15860_e25408 * locals.var_deltemp);
                let assign15860_e25411: f64 = (assign15860_e25405 + assign15860_e25410);
                let assign15860_e25412: f64 = (locals.var_mexp_i * assign15860_e25411);
                let assign15860_e25414: f64 = (assign15860_e25412 - 2.0);
                let assign15860_e25416: f64 = (-10000.0);
                let assign15860_e25418: f64 = (assign15860_e25416 * 0.001);
                let (assign15860_e25441, assign15860_e25441_d_n0, assign15860_e25441_d_n2, assign15860_e25441_d_n3, assign15860_e25441_d_n4, assign15860_e25441_d_n5, assign15860_e25441_d_n6, assign15860_e25441_d_n7, assign15860_e25441_d_n8, assign15860_e25441_d_n9, assign15860_e25441_d_n10, assign15860_e25441_d_n11, assign15860_e25441_d_n13, assign15860_e25441_d_n14,) = {
                    if (assign15860_e25414 < assign15860_e25418) {
                        let assign15860_e25421: f64 = (-0.001);
                        let assign15860_e25423: f64 = (assign15860_e25421 * 0.001);
                        let assign15860_e25428: f64 = (p.p450 * locals.var_deltemp);
                        let assign15860_e25429: f64 = (1.0 + assign15860_e25428);
                        let assign15860_e25432: f64 = (p.p451 * locals.var_deltemp);
                        let assign15860_e25434: f64 = (assign15860_e25432 * locals.var_deltemp);
                        let assign15860_e25435: f64 = (assign15860_e25429 + assign15860_e25434);
                        let assign15860_e25436: f64 = (locals.var_mexp_i * assign15860_e25435);
                        let assign15860_e25438: f64 = (assign15860_e25436 - 2.0);
                        let assign15860_e25439: f64 = (assign15860_e25423 / assign15860_e25438);
                        (assign15860_e25439, (-((assign15860_e25423 * (locals.var_mexp_i_dn0 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn2 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn3 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * ((locals.var_mexp_i_dn4 * assign15860_e25435) + (locals.var_mexp_i * ((p.p450 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15860_e25432 * locals.var_deltemp_dn4)))))) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn5 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn6 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn7 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn8 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn9 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn10 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn11 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn13 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))), (-((assign15860_e25423 * (locals.var_mexp_i_dn14 * assign15860_e25435)) / (assign15860_e25438 * assign15860_e25438))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15860_e25441, assign15860_e25441_d_n0, assign15860_e25441_d_n2, assign15860_e25441_d_n3, assign15860_e25441_d_n4, assign15860_e25441_d_n5, assign15860_e25441_d_n6, assign15860_e25441_d_n7, assign15860_e25441_d_n8, assign15860_e25441_d_n9, assign15860_e25441_d_n10, assign15860_e25441_d_n11, assign15860_e25441_d_n13, assign15860_e25441_d_n14,)
            }
        };
        let assign15860_e25444: f64 = (assign15860_e25442 + 2.0);
        (assign15860_e25444, assign15860_e25442_d_n0, assign15860_e25442_d_n2, assign15860_e25442_d_n3, assign15860_e25442_d_n4, assign15860_e25442_d_n5, assign15860_e25442_d_n6, assign15860_e25442_d_n7, assign15860_e25442_d_n8, assign15860_e25442_d_n9, assign15860_e25442_d_n10, assign15860_e25442_d_n11, assign15860_e25442_d_n13, assign15860_e25442_d_n14,)
    } else {
        (locals.var_mexp_t, locals.var_mexp_t_dn0, locals.var_mexp_t_dn2, locals.var_mexp_t_dn3, locals.var_mexp_t_dn4, locals.var_mexp_t_dn5, locals.var_mexp_t_dn6, locals.var_mexp_t_dn7, locals.var_mexp_t_dn8, locals.var_mexp_t_dn9, locals.var_mexp_t_dn10, locals.var_mexp_t_dn11, locals.var_mexp_t_dn13, locals.var_mexp_t_dn14,)
    }
};
        locals.var_mexp_t = assign15860_e25446;
        locals.var_mexp_t_dn0 = assign15860_e25446_d_n0;
        locals.var_mexp_t_dn2 = assign15860_e25446_d_n2;
        locals.var_mexp_t_dn3 = assign15860_e25446_d_n3;
        locals.var_mexp_t_dn4 = assign15860_e25446_d_n4;
        locals.var_mexp_t_dn5 = assign15860_e25446_d_n5;
        locals.var_mexp_t_dn6 = assign15860_e25446_d_n6;
        locals.var_mexp_t_dn7 = assign15860_e25446_d_n7;
        locals.var_mexp_t_dn8 = assign15860_e25446_d_n8;
        locals.var_mexp_t_dn9 = assign15860_e25446_d_n9;
        locals.var_mexp_t_dn10 = assign15860_e25446_d_n10;
        locals.var_mexp_t_dn11 = assign15860_e25446_d_n11;
        locals.var_mexp_t_dn13 = assign15860_e25446_d_n13;
        locals.var_mexp_t_dn14 = assign15860_e25446_d_n14;
        locals.var_mexp_t_rv = 0.0;

        let assign15870_e25449: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign15870_e25449;
        locals.var_guard278_rv = 0.0;

        let (assign15880_e25579, assign15880_e25579_d_n0, assign15880_e25579_d_n2, assign15880_e25579_d_n3, assign15880_e25579_d_n4, assign15880_e25579_d_n5, assign15880_e25579_d_n6, assign15880_e25579_d_n7, assign15880_e25579_d_n8, assign15880_e25579_d_n9, assign15880_e25579_d_n10, assign15880_e25579_d_n11, assign15880_e25579_d_n13, assign15880_e25579_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard278 != 0.0)) {
        let assign15880_e25460: f64 = (p.p452 * locals.var_deltemp);
        let assign15880_e25461: f64 = (1.0 + assign15880_e25460);
        let assign15880_e25464: f64 = (p.p451 * locals.var_deltemp);
        let assign15880_e25466: f64 = (assign15880_e25464 * locals.var_deltemp);
        let assign15880_e25467: f64 = (assign15880_e25461 + assign15880_e25466);
        let assign15880_e25468: f64 = (locals.var_mexpr_i * assign15880_e25467);
        let assign15880_e25470: f64 = (assign15880_e25468 - 2.0);
        let assign15880_e25472: f64 = (-10000.0);
        let assign15880_e25474: f64 = (assign15880_e25472 * 0.001);
        let (assign15880_e25575, assign15880_e25575_d_n0, assign15880_e25575_d_n2, assign15880_e25575_d_n3, assign15880_e25575_d_n4, assign15880_e25575_d_n5, assign15880_e25575_d_n6, assign15880_e25575_d_n7, assign15880_e25575_d_n8, assign15880_e25575_d_n9, assign15880_e25575_d_n10, assign15880_e25575_d_n11, assign15880_e25575_d_n13, assign15880_e25575_d_n14,) = {
            if (!(assign15880_e25470 < assign15880_e25474)) {
                let assign15880_e25482: f64 = (p.p452 * locals.var_deltemp);
                let assign15880_e25483: f64 = (1.0 + assign15880_e25482);
                let assign15880_e25486: f64 = (p.p451 * locals.var_deltemp);
                let assign15880_e25488: f64 = (assign15880_e25486 * locals.var_deltemp);
                let assign15880_e25489: f64 = (assign15880_e25483 + assign15880_e25488);
                let assign15880_e25490: f64 = (locals.var_mexpr_i * assign15880_e25489);
                let assign15880_e25492: f64 = (assign15880_e25490 - 2.0);
                let assign15880_e25497: f64 = (p.p452 * locals.var_deltemp);
                let assign15880_e25498: f64 = (1.0 + assign15880_e25497);
                let assign15880_e25501: f64 = (p.p451 * locals.var_deltemp);
                let assign15880_e25503: f64 = (assign15880_e25501 * locals.var_deltemp);
                let assign15880_e25504: f64 = (assign15880_e25498 + assign15880_e25503);
                let assign15880_e25505: f64 = (locals.var_mexpr_i * assign15880_e25504);
                let assign15880_e25507: f64 = (assign15880_e25505 - 2.0);
                let assign15880_e25512: f64 = (p.p452 * locals.var_deltemp);
                let assign15880_e25513: f64 = (1.0 + assign15880_e25512);
                let assign15880_e25516: f64 = (p.p451 * locals.var_deltemp);
                let assign15880_e25518: f64 = (assign15880_e25516 * locals.var_deltemp);
                let assign15880_e25519: f64 = (assign15880_e25513 + assign15880_e25518);
                let assign15880_e25520: f64 = (locals.var_mexpr_i * assign15880_e25519);
                let assign15880_e25522: f64 = (assign15880_e25520 - 2.0);
                let assign15880_e25523: f64 = (assign15880_e25507 * assign15880_e25522);
                let assign15880_e25526: f64 = (4.0 * 0.001);
                let assign15880_e25528: f64 = (assign15880_e25526 * 0.001);
                let assign15880_e25529: f64 = (assign15880_e25523 + assign15880_e25528);
                let assign15880_e25530: f64 = (assign15880_e25529).sqrt();
                let assign15880_e25531: f64 = (assign15880_e25492 + assign15880_e25530);
                let assign15880_e25532: f64 = (0.5 * assign15880_e25531);
                (assign15880_e25532, (0.5 * ((locals.var_mexpr_i_dn0 * assign15880_e25489) + ((((locals.var_mexpr_i_dn0 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn0 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn2 * assign15880_e25489) + ((((locals.var_mexpr_i_dn2 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn2 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn3 * assign15880_e25489) + ((((locals.var_mexpr_i_dn3 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn3 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * (((locals.var_mexpr_i_dn4 * assign15880_e25489) + (locals.var_mexpr_i * ((p.p452 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15880_e25486 * locals.var_deltemp_dn4))))) + (((((locals.var_mexpr_i_dn4 * assign15880_e25504) + (locals.var_mexpr_i * ((p.p452 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15880_e25501 * locals.var_deltemp_dn4))))) * assign15880_e25522) + (assign15880_e25507 * ((locals.var_mexpr_i_dn4 * assign15880_e25519) + (locals.var_mexpr_i * ((p.p452 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15880_e25516 * locals.var_deltemp_dn4))))))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn5 * assign15880_e25489) + ((((locals.var_mexpr_i_dn5 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn5 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn6 * assign15880_e25489) + ((((locals.var_mexpr_i_dn6 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn6 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn7 * assign15880_e25489) + ((((locals.var_mexpr_i_dn7 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn7 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn8 * assign15880_e25489) + ((((locals.var_mexpr_i_dn8 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn8 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn9 * assign15880_e25489) + ((((locals.var_mexpr_i_dn9 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn9 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn10 * assign15880_e25489) + ((((locals.var_mexpr_i_dn10 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn10 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn11 * assign15880_e25489) + ((((locals.var_mexpr_i_dn11 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn11 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn13 * assign15880_e25489) + ((((locals.var_mexpr_i_dn13 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn13 * assign15880_e25519))) / (2.0 * assign15880_e25530)))), (0.5 * ((locals.var_mexpr_i_dn14 * assign15880_e25489) + ((((locals.var_mexpr_i_dn14 * assign15880_e25504) * assign15880_e25522) + (assign15880_e25507 * (locals.var_mexpr_i_dn14 * assign15880_e25519))) / (2.0 * assign15880_e25530)))),)
            } else {
                let assign15880_e25537: f64 = (p.p452 * locals.var_deltemp);
                let assign15880_e25538: f64 = (1.0 + assign15880_e25537);
                let assign15880_e25541: f64 = (p.p451 * locals.var_deltemp);
                let assign15880_e25543: f64 = (assign15880_e25541 * locals.var_deltemp);
                let assign15880_e25544: f64 = (assign15880_e25538 + assign15880_e25543);
                let assign15880_e25545: f64 = (locals.var_mexpr_i * assign15880_e25544);
                let assign15880_e25547: f64 = (assign15880_e25545 - 2.0);
                let assign15880_e25549: f64 = (-10000.0);
                let assign15880_e25551: f64 = (assign15880_e25549 * 0.001);
                let (assign15880_e25574, assign15880_e25574_d_n0, assign15880_e25574_d_n2, assign15880_e25574_d_n3, assign15880_e25574_d_n4, assign15880_e25574_d_n5, assign15880_e25574_d_n6, assign15880_e25574_d_n7, assign15880_e25574_d_n8, assign15880_e25574_d_n9, assign15880_e25574_d_n10, assign15880_e25574_d_n11, assign15880_e25574_d_n13, assign15880_e25574_d_n14,) = {
                    if (assign15880_e25547 < assign15880_e25551) {
                        let assign15880_e25554: f64 = (-0.001);
                        let assign15880_e25556: f64 = (assign15880_e25554 * 0.001);
                        let assign15880_e25561: f64 = (p.p452 * locals.var_deltemp);
                        let assign15880_e25562: f64 = (1.0 + assign15880_e25561);
                        let assign15880_e25565: f64 = (p.p451 * locals.var_deltemp);
                        let assign15880_e25567: f64 = (assign15880_e25565 * locals.var_deltemp);
                        let assign15880_e25568: f64 = (assign15880_e25562 + assign15880_e25567);
                        let assign15880_e25569: f64 = (locals.var_mexpr_i * assign15880_e25568);
                        let assign15880_e25571: f64 = (assign15880_e25569 - 2.0);
                        let assign15880_e25572: f64 = (assign15880_e25556 / assign15880_e25571);
                        (assign15880_e25572, (-((assign15880_e25556 * (locals.var_mexpr_i_dn0 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn2 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn3 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * ((locals.var_mexpr_i_dn4 * assign15880_e25568) + (locals.var_mexpr_i * ((p.p452 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15880_e25565 * locals.var_deltemp_dn4)))))) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn5 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn6 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn7 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn8 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn9 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn10 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn11 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn13 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))), (-((assign15880_e25556 * (locals.var_mexpr_i_dn14 * assign15880_e25568)) / (assign15880_e25571 * assign15880_e25571))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15880_e25574, assign15880_e25574_d_n0, assign15880_e25574_d_n2, assign15880_e25574_d_n3, assign15880_e25574_d_n4, assign15880_e25574_d_n5, assign15880_e25574_d_n6, assign15880_e25574_d_n7, assign15880_e25574_d_n8, assign15880_e25574_d_n9, assign15880_e25574_d_n10, assign15880_e25574_d_n11, assign15880_e25574_d_n13, assign15880_e25574_d_n14,)
            }
        };
        let assign15880_e25577: f64 = (assign15880_e25575 + 2.0);
        (assign15880_e25577, assign15880_e25575_d_n0, assign15880_e25575_d_n2, assign15880_e25575_d_n3, assign15880_e25575_d_n4, assign15880_e25575_d_n5, assign15880_e25575_d_n6, assign15880_e25575_d_n7, assign15880_e25575_d_n8, assign15880_e25575_d_n9, assign15880_e25575_d_n10, assign15880_e25575_d_n11, assign15880_e25575_d_n13, assign15880_e25575_d_n14,)
    } else {
        (locals.var_mexpr_t, locals.var_mexpr_t_dn0, locals.var_mexpr_t_dn2, locals.var_mexpr_t_dn3, locals.var_mexpr_t_dn4, locals.var_mexpr_t_dn5, locals.var_mexpr_t_dn6, locals.var_mexpr_t_dn7, locals.var_mexpr_t_dn8, locals.var_mexpr_t_dn9, locals.var_mexpr_t_dn10, locals.var_mexpr_t_dn11, locals.var_mexpr_t_dn13, locals.var_mexpr_t_dn14,)
    }
};
        locals.var_mexpr_t = assign15880_e25579;
        locals.var_mexpr_t_dn0 = assign15880_e25579_d_n0;
        locals.var_mexpr_t_dn2 = assign15880_e25579_d_n2;
        locals.var_mexpr_t_dn3 = assign15880_e25579_d_n3;
        locals.var_mexpr_t_dn4 = assign15880_e25579_d_n4;
        locals.var_mexpr_t_dn5 = assign15880_e25579_d_n5;
        locals.var_mexpr_t_dn6 = assign15880_e25579_d_n6;
        locals.var_mexpr_t_dn7 = assign15880_e25579_d_n7;
        locals.var_mexpr_t_dn8 = assign15880_e25579_d_n8;
        locals.var_mexpr_t_dn9 = assign15880_e25579_d_n9;
        locals.var_mexpr_t_dn10 = assign15880_e25579_d_n10;
        locals.var_mexpr_t_dn11 = assign15880_e25579_d_n11;
        locals.var_mexpr_t_dn13 = assign15880_e25579_d_n13;
        locals.var_mexpr_t_dn14 = assign15880_e25579_d_n14;
        locals.var_mexpr_t_rv = 0.0;

        let assign15890_e25582: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign15890_e25582;
        locals.var_guard279_rv = 0.0;

        let (assign15900_e25649, assign15900_e25649_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign15900_e25591: f64 = (-locals.var_ksativ_i);
        let assign15900_e25595: f64 = (p.p498 * locals.var_deltemp);
        let assign15900_e25598: f64 = (p.p499 * locals.var_deltemp);
        let assign15900_e25600: f64 = (assign15900_e25598 * locals.var_deltemp);
        let assign15900_e25601: f64 = (assign15900_e25595 + assign15900_e25600);
        let assign15900_e25603: f64 = (-locals.var_ksativ_i);
        let assign15900_e25604: f64 = (assign15900_e25601 - assign15900_e25603);
        let assign15900_e25606: f64 = (assign15900_e25604 - 1e-6);
        let assign15900_e25609: f64 = (p.p498 * locals.var_deltemp);
        let assign15900_e25612: f64 = (p.p499 * locals.var_deltemp);
        let assign15900_e25614: f64 = (assign15900_e25612 * locals.var_deltemp);
        let assign15900_e25615: f64 = (assign15900_e25609 + assign15900_e25614);
        let assign15900_e25617: f64 = (-locals.var_ksativ_i);
        let assign15900_e25618: f64 = (assign15900_e25615 - assign15900_e25617);
        let assign15900_e25620: f64 = (assign15900_e25618 - 1e-6);
        let assign15900_e25623: f64 = (p.p498 * locals.var_deltemp);
        let assign15900_e25626: f64 = (p.p499 * locals.var_deltemp);
        let assign15900_e25628: f64 = (assign15900_e25626 * locals.var_deltemp);
        let assign15900_e25629: f64 = (assign15900_e25623 + assign15900_e25628);
        let assign15900_e25631: f64 = (-locals.var_ksativ_i);
        let assign15900_e25632: f64 = (assign15900_e25629 - assign15900_e25631);
        let assign15900_e25634: f64 = (assign15900_e25632 - 1e-6);
        let assign15900_e25635: f64 = (assign15900_e25620 * assign15900_e25634);
        let assign15900_e25638: f64 = (-locals.var_ksativ_i);
        let assign15900_e25639: f64 = (4.0 * assign15900_e25638);
        let assign15900_e25641: f64 = (assign15900_e25639 * 1e-6);
        let assign15900_e25642: f64 = (assign15900_e25635 - assign15900_e25641);
        let assign15900_e25643: f64 = (assign15900_e25642).sqrt();
        let assign15900_e25644: f64 = (assign15900_e25606 + assign15900_e25643);
        let assign15900_e25645: f64 = (0.5 * assign15900_e25644);
        let assign15900_e25646: f64 = (assign15900_e25591 + assign15900_e25645);
        let assign15900_e25647: f64 = (locals.var_ksativ_i + assign15900_e25646);
        (assign15900_e25647, (0.5 * (((p.p498 * locals.var_deltemp_dn4) + (((p.p499 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15900_e25598 * locals.var_deltemp_dn4))) + (((((p.p498 * locals.var_deltemp_dn4) + (((p.p499 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15900_e25612 * locals.var_deltemp_dn4))) * assign15900_e25634) + (assign15900_e25620 * ((p.p498 * locals.var_deltemp_dn4) + (((p.p499 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15900_e25626 * locals.var_deltemp_dn4))))) / (2.0 * assign15900_e25643)))),)
    } else {
        (locals.var_ksativ_t, locals.var_ksativ_t_dn4,)
    }
};
        locals.var_ksativ_t = assign15900_e25649;
        locals.var_ksativ_t_dn4 = assign15900_e25649_d_n4;
        locals.var_ksativ_t_rv = 0.0;

        let (assign15910_e25768, assign15910_e25768_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard279 == 0.0)) {
        let assign15910_e25661: f64 = (p.p498 * locals.var_deltemp);
        let assign15910_e25662: f64 = (1.0 + assign15910_e25661);
        let assign15910_e25665: f64 = (p.p499 * locals.var_deltemp);
        let assign15910_e25667: f64 = (assign15910_e25665 * locals.var_deltemp);
        let assign15910_e25668: f64 = (assign15910_e25662 + assign15910_e25667);
        let assign15910_e25670: f64 = (assign15910_e25668 - 1e-6);
        let assign15910_e25672: f64 = (-10000.0);
        let assign15910_e25674: f64 = (assign15910_e25672 * 0.001);
        let (assign15910_e25765, assign15910_e25765_d_n4,) = {
            if (!(assign15910_e25670 < assign15910_e25674)) {
                let assign15910_e25681: f64 = (p.p498 * locals.var_deltemp);
                let assign15910_e25682: f64 = (1.0 + assign15910_e25681);
                let assign15910_e25685: f64 = (p.p499 * locals.var_deltemp);
                let assign15910_e25687: f64 = (assign15910_e25685 * locals.var_deltemp);
                let assign15910_e25688: f64 = (assign15910_e25682 + assign15910_e25687);
                let assign15910_e25690: f64 = (assign15910_e25688 - 1e-6);
                let assign15910_e25694: f64 = (p.p498 * locals.var_deltemp);
                let assign15910_e25695: f64 = (1.0 + assign15910_e25694);
                let assign15910_e25698: f64 = (p.p499 * locals.var_deltemp);
                let assign15910_e25700: f64 = (assign15910_e25698 * locals.var_deltemp);
                let assign15910_e25701: f64 = (assign15910_e25695 + assign15910_e25700);
                let assign15910_e25703: f64 = (assign15910_e25701 - 1e-6);
                let assign15910_e25707: f64 = (p.p498 * locals.var_deltemp);
                let assign15910_e25708: f64 = (1.0 + assign15910_e25707);
                let assign15910_e25711: f64 = (p.p499 * locals.var_deltemp);
                let assign15910_e25713: f64 = (assign15910_e25711 * locals.var_deltemp);
                let assign15910_e25714: f64 = (assign15910_e25708 + assign15910_e25713);
                let assign15910_e25716: f64 = (assign15910_e25714 - 1e-6);
                let assign15910_e25717: f64 = (assign15910_e25703 * assign15910_e25716);
                let assign15910_e25720: f64 = (4.0 * 0.001);
                let assign15910_e25722: f64 = (assign15910_e25720 * 0.001);
                let assign15910_e25723: f64 = (assign15910_e25717 + assign15910_e25722);
                let assign15910_e25724: f64 = (assign15910_e25723).sqrt();
                let assign15910_e25725: f64 = (assign15910_e25690 + assign15910_e25724);
                let assign15910_e25726: f64 = (0.5 * assign15910_e25725);
                (assign15910_e25726, (0.5 * (((p.p498 * locals.var_deltemp_dn4) + (((p.p499 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15910_e25685 * locals.var_deltemp_dn4))) + (((((p.p498 * locals.var_deltemp_dn4) + (((p.p499 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15910_e25698 * locals.var_deltemp_dn4))) * assign15910_e25716) + (assign15910_e25703 * ((p.p498 * locals.var_deltemp_dn4) + (((p.p499 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15910_e25711 * locals.var_deltemp_dn4))))) / (2.0 * assign15910_e25724)))),)
            } else {
                let assign15910_e25730: f64 = (p.p498 * locals.var_deltemp);
                let assign15910_e25731: f64 = (1.0 + assign15910_e25730);
                let assign15910_e25734: f64 = (p.p499 * locals.var_deltemp);
                let assign15910_e25736: f64 = (assign15910_e25734 * locals.var_deltemp);
                let assign15910_e25737: f64 = (assign15910_e25731 + assign15910_e25736);
                let assign15910_e25739: f64 = (assign15910_e25737 - 1e-6);
                let assign15910_e25741: f64 = (-10000.0);
                let assign15910_e25743: f64 = (assign15910_e25741 * 0.001);
                let (assign15910_e25764, assign15910_e25764_d_n4,) = {
                    if (assign15910_e25739 < assign15910_e25743) {
                        let assign15910_e25746: f64 = (-0.001);
                        let assign15910_e25748: f64 = (assign15910_e25746 * 0.001);
                        let assign15910_e25752: f64 = (p.p498 * locals.var_deltemp);
                        let assign15910_e25753: f64 = (1.0 + assign15910_e25752);
                        let assign15910_e25756: f64 = (p.p499 * locals.var_deltemp);
                        let assign15910_e25758: f64 = (assign15910_e25756 * locals.var_deltemp);
                        let assign15910_e25759: f64 = (assign15910_e25753 + assign15910_e25758);
                        let assign15910_e25761: f64 = (assign15910_e25759 - 1e-6);
                        let assign15910_e25762: f64 = (assign15910_e25748 / assign15910_e25761);
                        (assign15910_e25762, (-((assign15910_e25748 * ((p.p498 * locals.var_deltemp_dn4) + (((p.p499 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15910_e25756 * locals.var_deltemp_dn4)))) / (assign15910_e25761 * assign15910_e25761))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15910_e25764, assign15910_e25764_d_n4,)
            }
        };
        let assign15910_e25766: f64 = (locals.var_ksativ_i * assign15910_e25765);
        (assign15910_e25766, (locals.var_ksativ_i * assign15910_e25765_d_n4),)
    } else {
        (locals.var_ksativ_t, locals.var_ksativ_t_dn4,)
    }
};
        locals.var_ksativ_t = assign15910_e25768;
        locals.var_ksativ_t_dn4 = assign15910_e25768_d_n4;
        locals.var_ksativ_t_rv = 0.0;

        let assign15920_e25771: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign15920_e25771;
        locals.var_guard280_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15930_e25820, assign15930_e25820_d_n0, assign15930_e25820_d_n2, assign15930_e25820_d_n3, assign15930_e25820_d_n4, assign15930_e25820_d_n5, assign15930_e25820_d_n6, assign15930_e25820_d_n7, assign15930_e25820_d_n8, assign15930_e25820_d_n9, assign15930_e25820_d_n10, assign15930_e25820_d_n11, assign15930_e25820_d_n13, assign15930_e25820_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard280 != 0.0)) {
        let assign15930_e25780: f64 = (-locals.var_pclm_i);
        let assign15930_e25784: f64 = (p.p1026 * locals.var_deltemp);
        let assign15930_e25786: f64 = (-locals.var_pclm_i);
        let assign15930_e25787: f64 = (assign15930_e25784 - assign15930_e25786);
        let assign15930_e25789: f64 = (assign15930_e25787 - 1e-6);
        let assign15930_e25792: f64 = (p.p1026 * locals.var_deltemp);
        let assign15930_e25794: f64 = (-locals.var_pclm_i);
        let assign15930_e25795: f64 = (assign15930_e25792 - assign15930_e25794);
        let assign15930_e25797: f64 = (assign15930_e25795 - 1e-6);
        let assign15930_e25800: f64 = (p.p1026 * locals.var_deltemp);
        let assign15930_e25802: f64 = (-locals.var_pclm_i);
        let assign15930_e25803: f64 = (assign15930_e25800 - assign15930_e25802);
        let assign15930_e25805: f64 = (assign15930_e25803 - 1e-6);
        let assign15930_e25806: f64 = (assign15930_e25797 * assign15930_e25805);
        let assign15930_e25809: f64 = (-locals.var_pclm_i);
        let assign15930_e25810: f64 = (4.0 * assign15930_e25809);
        let assign15930_e25812: f64 = (assign15930_e25810 * 1e-6);
        let assign15930_e25813: f64 = (assign15930_e25806 - assign15930_e25812);
        let assign15930_e25814: f64 = (assign15930_e25813).sqrt();
        let assign15930_e25815: f64 = (assign15930_e25789 + assign15930_e25814);
        let assign15930_e25816: f64 = (0.5 * assign15930_e25815);
        let assign15930_e25817: f64 = (assign15930_e25780 + assign15930_e25816);
        let assign15930_e25818: f64 = (locals.var_pclm_i + assign15930_e25817);
        (assign15930_e25818, (locals.var_pclm_i_dn0 + ((-locals.var_pclm_i_dn0) + (0.5 * ((-(-locals.var_pclm_i_dn0)) + (((((-(-locals.var_pclm_i_dn0)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn0)))) - ((4.0 * (-locals.var_pclm_i_dn0)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn2 + ((-locals.var_pclm_i_dn2) + (0.5 * ((-(-locals.var_pclm_i_dn2)) + (((((-(-locals.var_pclm_i_dn2)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn2)))) - ((4.0 * (-locals.var_pclm_i_dn2)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn3 + ((-locals.var_pclm_i_dn3) + (0.5 * ((-(-locals.var_pclm_i_dn3)) + (((((-(-locals.var_pclm_i_dn3)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn3)))) - ((4.0 * (-locals.var_pclm_i_dn3)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn4 + ((-locals.var_pclm_i_dn4) + (0.5 * (((p.p1026 * locals.var_deltemp_dn4) - (-locals.var_pclm_i_dn4)) + ((((((p.p1026 * locals.var_deltemp_dn4) - (-locals.var_pclm_i_dn4)) * assign15930_e25805) + (assign15930_e25797 * ((p.p1026 * locals.var_deltemp_dn4) - (-locals.var_pclm_i_dn4)))) - ((4.0 * (-locals.var_pclm_i_dn4)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn5 + ((-locals.var_pclm_i_dn5) + (0.5 * ((-(-locals.var_pclm_i_dn5)) + (((((-(-locals.var_pclm_i_dn5)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn5)))) - ((4.0 * (-locals.var_pclm_i_dn5)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn6 + ((-locals.var_pclm_i_dn6) + (0.5 * ((-(-locals.var_pclm_i_dn6)) + (((((-(-locals.var_pclm_i_dn6)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn6)))) - ((4.0 * (-locals.var_pclm_i_dn6)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn7 + ((-locals.var_pclm_i_dn7) + (0.5 * ((-(-locals.var_pclm_i_dn7)) + (((((-(-locals.var_pclm_i_dn7)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn7)))) - ((4.0 * (-locals.var_pclm_i_dn7)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn8 + ((-locals.var_pclm_i_dn8) + (0.5 * ((-(-locals.var_pclm_i_dn8)) + (((((-(-locals.var_pclm_i_dn8)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn8)))) - ((4.0 * (-locals.var_pclm_i_dn8)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn9 + ((-locals.var_pclm_i_dn9) + (0.5 * ((-(-locals.var_pclm_i_dn9)) + (((((-(-locals.var_pclm_i_dn9)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn9)))) - ((4.0 * (-locals.var_pclm_i_dn9)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn10 + ((-locals.var_pclm_i_dn10) + (0.5 * ((-(-locals.var_pclm_i_dn10)) + (((((-(-locals.var_pclm_i_dn10)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn10)))) - ((4.0 * (-locals.var_pclm_i_dn10)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn11 + ((-locals.var_pclm_i_dn11) + (0.5 * ((-(-locals.var_pclm_i_dn11)) + (((((-(-locals.var_pclm_i_dn11)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn11)))) - ((4.0 * (-locals.var_pclm_i_dn11)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn13 + ((-locals.var_pclm_i_dn13) + (0.5 * ((-(-locals.var_pclm_i_dn13)) + (((((-(-locals.var_pclm_i_dn13)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn13)))) - ((4.0 * (-locals.var_pclm_i_dn13)) * 1e-6)) / (2.0 * assign15930_e25814)))))), (locals.var_pclm_i_dn14 + ((-locals.var_pclm_i_dn14) + (0.5 * ((-(-locals.var_pclm_i_dn14)) + (((((-(-locals.var_pclm_i_dn14)) * assign15930_e25805) + (assign15930_e25797 * (-(-locals.var_pclm_i_dn14)))) - ((4.0 * (-locals.var_pclm_i_dn14)) * 1e-6)) / (2.0 * assign15930_e25814)))))),)
    } else {
        (locals.var_pclm_t, locals.var_pclm_t_dn0, locals.var_pclm_t_dn2, locals.var_pclm_t_dn3, locals.var_pclm_t_dn4, locals.var_pclm_t_dn5, locals.var_pclm_t_dn6, locals.var_pclm_t_dn7, locals.var_pclm_t_dn8, locals.var_pclm_t_dn9, locals.var_pclm_t_dn10, locals.var_pclm_t_dn11, locals.var_pclm_t_dn13, locals.var_pclm_t_dn14,)
    }
};
        locals.var_pclm_t = assign15930_e25820;
        locals.var_pclm_t_dn0 = assign15930_e25820_d_n0;
        locals.var_pclm_t_dn2 = assign15930_e25820_d_n2;
        locals.var_pclm_t_dn3 = assign15930_e25820_d_n3;
        locals.var_pclm_t_dn4 = assign15930_e25820_d_n4;
        locals.var_pclm_t_dn5 = assign15930_e25820_d_n5;
        locals.var_pclm_t_dn6 = assign15930_e25820_d_n6;
        locals.var_pclm_t_dn7 = assign15930_e25820_d_n7;
        locals.var_pclm_t_dn8 = assign15930_e25820_d_n8;
        locals.var_pclm_t_dn9 = assign15930_e25820_d_n9;
        locals.var_pclm_t_dn10 = assign15930_e25820_d_n10;
        locals.var_pclm_t_dn11 = assign15930_e25820_d_n11;
        locals.var_pclm_t_dn13 = assign15930_e25820_d_n13;
        locals.var_pclm_t_dn14 = assign15930_e25820_d_n14;
        locals.var_pclm_t_rv = 0.0;

        let (assign15940_e25903, assign15940_e25903_d_n0, assign15940_e25903_d_n2, assign15940_e25903_d_n3, assign15940_e25903_d_n4, assign15940_e25903_d_n5, assign15940_e25903_d_n6, assign15940_e25903_d_n7, assign15940_e25903_d_n8, assign15940_e25903_d_n9, assign15940_e25903_d_n10, assign15940_e25903_d_n11, assign15940_e25903_d_n13, assign15940_e25903_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard280 == 0.0)) {
        let assign15940_e25832: f64 = (p.p1026 * locals.var_deltemp);
        let assign15940_e25833: f64 = (1.0 + assign15940_e25832);
        let assign15940_e25835: f64 = (assign15940_e25833 - 1e-6);
        let assign15940_e25837: f64 = (-10000.0);
        let assign15940_e25839: f64 = (assign15940_e25837 * 0.001);
        let (assign15940_e25900, assign15940_e25900_d_n4,) = {
            if (!(assign15940_e25835 < assign15940_e25839)) {
                let assign15940_e25846: f64 = (p.p1026 * locals.var_deltemp);
                let assign15940_e25847: f64 = (1.0 + assign15940_e25846);
                let assign15940_e25849: f64 = (assign15940_e25847 - 1e-6);
                let assign15940_e25853: f64 = (p.p1026 * locals.var_deltemp);
                let assign15940_e25854: f64 = (1.0 + assign15940_e25853);
                let assign15940_e25856: f64 = (assign15940_e25854 - 1e-6);
                let assign15940_e25860: f64 = (p.p1026 * locals.var_deltemp);
                let assign15940_e25861: f64 = (1.0 + assign15940_e25860);
                let assign15940_e25863: f64 = (assign15940_e25861 - 1e-6);
                let assign15940_e25864: f64 = (assign15940_e25856 * assign15940_e25863);
                let assign15940_e25867: f64 = (4.0 * 0.001);
                let assign15940_e25869: f64 = (assign15940_e25867 * 0.001);
                let assign15940_e25870: f64 = (assign15940_e25864 + assign15940_e25869);
                let assign15940_e25871: f64 = (assign15940_e25870).sqrt();
                let assign15940_e25872: f64 = (assign15940_e25849 + assign15940_e25871);
                let assign15940_e25873: f64 = (0.5 * assign15940_e25872);
                (assign15940_e25873, (0.5 * ((p.p1026 * locals.var_deltemp_dn4) + ((((p.p1026 * locals.var_deltemp_dn4) * assign15940_e25863) + (assign15940_e25856 * (p.p1026 * locals.var_deltemp_dn4))) / (2.0 * assign15940_e25871)))),)
            } else {
                let assign15940_e25877: f64 = (p.p1026 * locals.var_deltemp);
                let assign15940_e25878: f64 = (1.0 + assign15940_e25877);
                let assign15940_e25880: f64 = (assign15940_e25878 - 1e-6);
                let assign15940_e25882: f64 = (-10000.0);
                let assign15940_e25884: f64 = (assign15940_e25882 * 0.001);
                let (assign15940_e25899, assign15940_e25899_d_n4,) = {
                    if (assign15940_e25880 < assign15940_e25884) {
                        let assign15940_e25887: f64 = (-0.001);
                        let assign15940_e25889: f64 = (assign15940_e25887 * 0.001);
                        let assign15940_e25893: f64 = (p.p1026 * locals.var_deltemp);
                        let assign15940_e25894: f64 = (1.0 + assign15940_e25893);
                        let assign15940_e25896: f64 = (assign15940_e25894 - 1e-6);
                        let assign15940_e25897: f64 = (assign15940_e25889 / assign15940_e25896);
                        (assign15940_e25897, (-((assign15940_e25889 * (p.p1026 * locals.var_deltemp_dn4)) / (assign15940_e25896 * assign15940_e25896))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15940_e25899, assign15940_e25899_d_n4,)
            }
        };
        let assign15940_e25901: f64 = (locals.var_pclm_i * assign15940_e25900);
        (assign15940_e25901, (locals.var_pclm_i_dn0 * assign15940_e25900), (locals.var_pclm_i_dn2 * assign15940_e25900), (locals.var_pclm_i_dn3 * assign15940_e25900), ((locals.var_pclm_i_dn4 * assign15940_e25900) + (locals.var_pclm_i * assign15940_e25900_d_n4)), (locals.var_pclm_i_dn5 * assign15940_e25900), (locals.var_pclm_i_dn6 * assign15940_e25900), (locals.var_pclm_i_dn7 * assign15940_e25900), (locals.var_pclm_i_dn8 * assign15940_e25900), (locals.var_pclm_i_dn9 * assign15940_e25900), (locals.var_pclm_i_dn10 * assign15940_e25900), (locals.var_pclm_i_dn11 * assign15940_e25900), (locals.var_pclm_i_dn13 * assign15940_e25900), (locals.var_pclm_i_dn14 * assign15940_e25900),)
    } else {
        (locals.var_pclm_t, locals.var_pclm_t_dn0, locals.var_pclm_t_dn2, locals.var_pclm_t_dn3, locals.var_pclm_t_dn4, locals.var_pclm_t_dn5, locals.var_pclm_t_dn6, locals.var_pclm_t_dn7, locals.var_pclm_t_dn8, locals.var_pclm_t_dn9, locals.var_pclm_t_dn10, locals.var_pclm_t_dn11, locals.var_pclm_t_dn13, locals.var_pclm_t_dn14,)
    }
};
        locals.var_pclm_t = assign15940_e25903;
        locals.var_pclm_t_dn0 = assign15940_e25903_d_n0;
        locals.var_pclm_t_dn2 = assign15940_e25903_d_n2;
        locals.var_pclm_t_dn3 = assign15940_e25903_d_n3;
        locals.var_pclm_t_dn4 = assign15940_e25903_d_n4;
        locals.var_pclm_t_dn5 = assign15940_e25903_d_n5;
        locals.var_pclm_t_dn6 = assign15940_e25903_d_n6;
        locals.var_pclm_t_dn7 = assign15940_e25903_d_n7;
        locals.var_pclm_t_dn8 = assign15940_e25903_d_n8;
        locals.var_pclm_t_dn9 = assign15940_e25903_d_n9;
        locals.var_pclm_t_dn10 = assign15940_e25903_d_n10;
        locals.var_pclm_t_dn11 = assign15940_e25903_d_n11;
        locals.var_pclm_t_dn13 = assign15940_e25903_d_n13;
        locals.var_pclm_t_dn14 = assign15940_e25903_d_n14;
        locals.var_pclm_t_rv = 0.0;

        let (assign15950_e25938, assign15950_e25938_d_n0, assign15950_e25938_d_n2, assign15950_e25938_d_n3, assign15950_e25938_d_n4, assign15950_e25938_d_n5, assign15950_e25938_d_n6, assign15950_e25938_d_n7, assign15950_e25938_d_n8, assign15950_e25938_d_n9, assign15950_e25938_d_n10, assign15950_e25938_d_n11, assign15950_e25938_d_n13, assign15950_e25938_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15950_e25911: f64 = (p.p1720 / locals.var_leff_1);
        let assign15950_e25912: f64 = (locals.var_kt1_i + assign15950_e25911);
        let assign15950_e25914: f64 = (assign15950_e25912 * locals.var_tratio_m1);
        let assign15950_e25920: f64 = (locals.var_devtemp - p.p1749);
        let assign15950_e25921: f64 = (p.p1748 * assign15950_e25920);
        let assign15950_e25922: f64 = { let limited_exp_arg = assign15950_e25921; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15950_e25923: f64 = (1.0 + assign15950_e25922);
        let assign15950_e25924: f64 = (p.p1747 / assign15950_e25923);
        let assign15950_e25925: f64 = (assign15950_e25914 + assign15950_e25924);
        let assign15950_e25931: f64 = (locals.var_tnom - p.p1749);
        let assign15950_e25932: f64 = (p.p1748 * assign15950_e25931);
        let assign15950_e25933: f64 = { let limited_exp_arg = assign15950_e25932; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15950_e25934: f64 = (1.0 + assign15950_e25933);
        let assign15950_e25935: f64 = (p.p1747 / assign15950_e25934);
        let assign15950_e25936: f64 = (assign15950_e25925 - assign15950_e25935);
        (assign15950_e25936, ((-((p.p1720 * locals.var_leff_1_dn0) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn2) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn3) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((((-((p.p1720 * locals.var_leff_1_dn4) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) + (assign15950_e25912 * locals.var_tratio_m1_dn4)) + (-((p.p1747 * ({ let limited_exp_arg = assign15950_e25921; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_devtemp_dn4))) / (assign15950_e25923 * assign15950_e25923)))), ((-((p.p1720 * locals.var_leff_1_dn5) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn6) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn7) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn8) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn9) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn10) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn11) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn13) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn14) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1),)
    } else {
        (locals.var_dvth_temp, locals.var_dvth_temp_dn0, locals.var_dvth_temp_dn2, locals.var_dvth_temp_dn3, locals.var_dvth_temp_dn4, locals.var_dvth_temp_dn5, locals.var_dvth_temp_dn6, locals.var_dvth_temp_dn7, locals.var_dvth_temp_dn8, locals.var_dvth_temp_dn9, locals.var_dvth_temp_dn10, locals.var_dvth_temp_dn11, locals.var_dvth_temp_dn13, locals.var_dvth_temp_dn14,)
    }
};
        locals.var_dvth_temp = assign15950_e25938;
        locals.var_dvth_temp_dn0 = assign15950_e25938_d_n0;
        locals.var_dvth_temp_dn2 = assign15950_e25938_d_n2;
        locals.var_dvth_temp_dn3 = assign15950_e25938_d_n3;
        locals.var_dvth_temp_dn4 = assign15950_e25938_d_n4;
        locals.var_dvth_temp_dn5 = assign15950_e25938_d_n5;
        locals.var_dvth_temp_dn6 = assign15950_e25938_d_n6;
        locals.var_dvth_temp_dn7 = assign15950_e25938_d_n7;
        locals.var_dvth_temp_dn8 = assign15950_e25938_d_n8;
        locals.var_dvth_temp_dn9 = assign15950_e25938_d_n9;
        locals.var_dvth_temp_dn10 = assign15950_e25938_d_n10;
        locals.var_dvth_temp_dn11 = assign15950_e25938_d_n11;
        locals.var_dvth_temp_dn13 = assign15950_e25938_d_n13;
        locals.var_dvth_temp_dn14 = assign15950_e25938_d_n14;
        locals.var_dvth_temp_rv = 0.0;

        let (assign15960_e25955, assign15960_e25955_d_n0, assign15960_e25955_d_n2, assign15960_e25955_d_n3, assign15960_e25955_d_n4, assign15960_e25955_d_n5, assign15960_e25955_d_n6, assign15960_e25955_d_n7, assign15960_e25955_d_n8, assign15960_e25955_d_n9, assign15960_e25955_d_n10, assign15960_e25955_d_n11, assign15960_e25955_d_n13, assign15960_e25955_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign15960_e25948: f64 = (locals.var_ute1_i * locals.var_deltratio1);
        let assign15960_e25949: f64 = (locals.var_ute_i + assign15960_e25948);
        let assign15960_e25951: f64 = (assign15960_e25949 * locals.var_trat_ln);
        let assign15960_e25952: f64 = (assign15960_e25951).exp();
        let assign15960_e25953: f64 = (locals.var_u0_i * assign15960_e25952);
        (assign15960_e25953, (locals.var_u0_i_dn0 * assign15960_e25952), (locals.var_u0_i_dn2 * assign15960_e25952), (locals.var_u0_i_dn3 * assign15960_e25952), ((locals.var_u0_i_dn4 * assign15960_e25952) + (locals.var_u0_i * (assign15960_e25952 * (((locals.var_ute1_i * locals.var_deltratio1_dn4) * locals.var_trat_ln) + (assign15960_e25949 * locals.var_trat_ln_dn4))))), (locals.var_u0_i_dn5 * assign15960_e25952), (locals.var_u0_i_dn6 * assign15960_e25952), (locals.var_u0_i_dn7 * assign15960_e25952), (locals.var_u0_i_dn8 * assign15960_e25952), (locals.var_u0_i_dn9 * assign15960_e25952), (locals.var_u0_i_dn10 * assign15960_e25952), (locals.var_u0_i_dn11 * assign15960_e25952), (locals.var_u0_i_dn13 * assign15960_e25952), (locals.var_u0_i_dn14 * assign15960_e25952),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15960_e25955;
        locals.var_t1_dn0 = assign15960_e25955_d_n0;
        locals.var_t1_dn2 = assign15960_e25955_d_n2;
        locals.var_t1_dn3 = assign15960_e25955_d_n3;
        locals.var_t1_dn4 = assign15960_e25955_d_n4;
        locals.var_t1_dn5 = assign15960_e25955_d_n5;
        locals.var_t1_dn6 = assign15960_e25955_d_n6;
        locals.var_t1_dn7 = assign15960_e25955_d_n7;
        locals.var_t1_dn8 = assign15960_e25955_d_n8;
        locals.var_t1_dn9 = assign15960_e25955_d_n9;
        locals.var_t1_dn10 = assign15960_e25955_d_n10;
        locals.var_t1_dn11 = assign15960_e25955_d_n11;
        locals.var_t1_dn13 = assign15960_e25955_d_n13;
        locals.var_t1_dn14 = assign15960_e25955_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15970_e26013, assign15970_e26013_d_n0, assign15970_e26013_d_n2, assign15970_e26013_d_n3, assign15970_e26013_d_n4, assign15970_e26013_d_n5, assign15970_e26013_d_n6, assign15970_e26013_d_n7, assign15970_e26013_d_n8, assign15970_e26013_d_n9, assign15970_e26013_d_n10, assign15970_e26013_d_n11, assign15970_e26013_d_n13, assign15970_e26013_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign15970_e25963: f64 = (-0.9);
        let assign15970_e25965: f64 = (assign15970_e25963 * locals.var_t1);
        let assign15970_e25969: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15970_e25971: f64 = (-0.9);
        let assign15970_e25973: f64 = (assign15970_e25971 * locals.var_t1);
        let assign15970_e25974: f64 = (assign15970_e25969 - assign15970_e25973);
        let assign15970_e25976: f64 = (assign15970_e25974 - 0.0001);
        let assign15970_e25979: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15970_e25981: f64 = (-0.9);
        let assign15970_e25983: f64 = (assign15970_e25981 * locals.var_t1);
        let assign15970_e25984: f64 = (assign15970_e25979 - assign15970_e25983);
        let assign15970_e25986: f64 = (assign15970_e25984 - 0.0001);
        let assign15970_e25989: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15970_e25991: f64 = (-0.9);
        let assign15970_e25993: f64 = (assign15970_e25991 * locals.var_t1);
        let assign15970_e25994: f64 = (assign15970_e25989 - assign15970_e25993);
        let assign15970_e25996: f64 = (assign15970_e25994 - 0.0001);
        let assign15970_e25997: f64 = (assign15970_e25986 * assign15970_e25996);
        let assign15970_e26000: f64 = (-0.9);
        let assign15970_e26002: f64 = (assign15970_e26000 * locals.var_t1);
        let assign15970_e26003: f64 = (4.0 * assign15970_e26002);
        let assign15970_e26005: f64 = (assign15970_e26003 * 0.0001);
        let assign15970_e26006: f64 = (assign15970_e25997 - assign15970_e26005);
        let assign15970_e26007: f64 = (assign15970_e26006).sqrt();
        let assign15970_e26008: f64 = (assign15970_e25976 + assign15970_e26007);
        let assign15970_e26009: f64 = (0.5 * assign15970_e26008);
        let assign15970_e26010: f64 = (assign15970_e25965 + assign15970_e26009);
        let assign15970_e26011: f64 = (locals.var_t1 + assign15970_e26010);
        (assign15970_e26011, (locals.var_t1_dn0 + ((assign15970_e25963 * locals.var_t1_dn0) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn0)) + (((((-(assign15970_e25981 * locals.var_t1_dn0)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn0)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn2 + ((assign15970_e25963 * locals.var_t1_dn2) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn2)) + (((((-(assign15970_e25981 * locals.var_t1_dn2)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn2)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn3 + ((assign15970_e25963 * locals.var_t1_dn3) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn3)) + (((((-(assign15970_e25981 * locals.var_t1_dn3)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn3)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn4 + ((assign15970_e25963 * locals.var_t1_dn4) + (0.5 * (((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15970_e25971 * locals.var_t1_dn4)) + ((((((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15970_e25981 * locals.var_t1_dn4)) * assign15970_e25996) + (assign15970_e25986 * ((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15970_e25991 * locals.var_t1_dn4)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn5 + ((assign15970_e25963 * locals.var_t1_dn5) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn5)) + (((((-(assign15970_e25981 * locals.var_t1_dn5)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn5)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn6 + ((assign15970_e25963 * locals.var_t1_dn6) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn6)) + (((((-(assign15970_e25981 * locals.var_t1_dn6)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn6)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn7 + ((assign15970_e25963 * locals.var_t1_dn7) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn7)) + (((((-(assign15970_e25981 * locals.var_t1_dn7)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn7)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn8 + ((assign15970_e25963 * locals.var_t1_dn8) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn8)) + (((((-(assign15970_e25981 * locals.var_t1_dn8)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn8)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn9 + ((assign15970_e25963 * locals.var_t1_dn9) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn9)) + (((((-(assign15970_e25981 * locals.var_t1_dn9)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn9)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn10 + ((assign15970_e25963 * locals.var_t1_dn10) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn10)) + (((((-(assign15970_e25981 * locals.var_t1_dn10)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn10)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn11 + ((assign15970_e25963 * locals.var_t1_dn11) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn11)) + (((((-(assign15970_e25981 * locals.var_t1_dn11)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn11)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn13 + ((assign15970_e25963 * locals.var_t1_dn13) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn13)) + (((((-(assign15970_e25981 * locals.var_t1_dn13)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn13)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15970_e26007)))))), (locals.var_t1_dn14 + ((assign15970_e25963 * locals.var_t1_dn14) + (0.5 * ((-(assign15970_e25971 * locals.var_t1_dn14)) + (((((-(assign15970_e25981 * locals.var_t1_dn14)) * assign15970_e25996) + (assign15970_e25986 * (-(assign15970_e25991 * locals.var_t1_dn14)))) - ((4.0 * (assign15970_e26000 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15970_e26007)))))),)
    } else {
        (locals.var_u0_v, locals.var_u0_v_dn0, locals.var_u0_v_dn2, locals.var_u0_v_dn3, locals.var_u0_v_dn4, locals.var_u0_v_dn5, locals.var_u0_v_dn6, locals.var_u0_v_dn7, locals.var_u0_v_dn8, locals.var_u0_v_dn9, locals.var_u0_v_dn10, locals.var_u0_v_dn11, locals.var_u0_v_dn13, locals.var_u0_v_dn14,)
    }
};
        locals.var_u0_v = assign15970_e26013;
        locals.var_u0_v_dn0 = assign15970_e26013_d_n0;
        locals.var_u0_v_dn2 = assign15970_e26013_d_n2;
        locals.var_u0_v_dn3 = assign15970_e26013_d_n3;
        locals.var_u0_v_dn4 = assign15970_e26013_d_n4;
        locals.var_u0_v_dn5 = assign15970_e26013_d_n5;
        locals.var_u0_v_dn6 = assign15970_e26013_d_n6;
        locals.var_u0_v_dn7 = assign15970_e26013_d_n7;
        locals.var_u0_v_dn8 = assign15970_e26013_d_n8;
        locals.var_u0_v_dn9 = assign15970_e26013_d_n9;
        locals.var_u0_v_dn10 = assign15970_e26013_d_n10;
        locals.var_u0_v_dn11 = assign15970_e26013_d_n11;
        locals.var_u0_v_dn13 = assign15970_e26013_d_n13;
        locals.var_u0_v_dn14 = assign15970_e26013_d_n14;
        locals.var_u0_v_rv = 0.0;

        let assign15980_e26016: f64 = if p.p66 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard281 = assign15980_e26016;
        locals.var_guard281_rv = 0.0;

        let (assign15990_e26035, assign15990_e26035_d_n0, assign15990_e26035_d_n2, assign15990_e26035_d_n3, assign15990_e26035_d_n4, assign15990_e26035_d_n5, assign15990_e26035_d_n6, assign15990_e26035_d_n7, assign15990_e26035_d_n8, assign15990_e26035_d_n9, assign15990_e26035_d_n10, assign15990_e26035_d_n11, assign15990_e26035_d_n13, assign15990_e26035_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard281 != 0.0)) {
        let assign15990_e26028: f64 = (locals.var_ute1_i * locals.var_deltratio1);
        let assign15990_e26029: f64 = (locals.var_uter_i + assign15990_e26028);
        let assign15990_e26031: f64 = (assign15990_e26029 * locals.var_trat_ln);
        let assign15990_e26032: f64 = (assign15990_e26031).exp();
        let assign15990_e26033: f64 = (locals.var_u0r_i * assign15990_e26032);
        (assign15990_e26033, (locals.var_u0r_i_dn0 * assign15990_e26032), (locals.var_u0r_i_dn2 * assign15990_e26032), (locals.var_u0r_i_dn3 * assign15990_e26032), ((locals.var_u0r_i_dn4 * assign15990_e26032) + (locals.var_u0r_i * (assign15990_e26032 * (((locals.var_ute1_i * locals.var_deltratio1_dn4) * locals.var_trat_ln) + (assign15990_e26029 * locals.var_trat_ln_dn4))))), (locals.var_u0r_i_dn5 * assign15990_e26032), (locals.var_u0r_i_dn6 * assign15990_e26032), (locals.var_u0r_i_dn7 * assign15990_e26032), (locals.var_u0r_i_dn8 * assign15990_e26032), (locals.var_u0r_i_dn9 * assign15990_e26032), (locals.var_u0r_i_dn10 * assign15990_e26032), (locals.var_u0r_i_dn11 * assign15990_e26032), (locals.var_u0r_i_dn13 * assign15990_e26032), (locals.var_u0r_i_dn14 * assign15990_e26032),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15990_e26035;
        locals.var_t1_dn0 = assign15990_e26035_d_n0;
        locals.var_t1_dn2 = assign15990_e26035_d_n2;
        locals.var_t1_dn3 = assign15990_e26035_d_n3;
        locals.var_t1_dn4 = assign15990_e26035_d_n4;
        locals.var_t1_dn5 = assign15990_e26035_d_n5;
        locals.var_t1_dn6 = assign15990_e26035_d_n6;
        locals.var_t1_dn7 = assign15990_e26035_d_n7;
        locals.var_t1_dn8 = assign15990_e26035_d_n8;
        locals.var_t1_dn9 = assign15990_e26035_d_n9;
        locals.var_t1_dn10 = assign15990_e26035_d_n10;
        locals.var_t1_dn11 = assign15990_e26035_d_n11;
        locals.var_t1_dn13 = assign15990_e26035_d_n13;
        locals.var_t1_dn14 = assign15990_e26035_d_n14;
        locals.var_t1_rv = 0.0;

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
        locals.var_u0r_t_rv = 0.0;

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
        locals.var_u0r_v_rv = 0.0;

        let assign16020_e26108: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard282 = assign16020_e26108;
        locals.var_guard282_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_ua_tl_rv = 0.0;

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
        locals.var_ua_th_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16070_e26269, assign16070_e26269_d_n0, assign16070_e26269_d_n2, assign16070_e26269_d_n3, assign16070_e26269_d_n4, assign16070_e26269_d_n5, assign16070_e26269_d_n6, assign16070_e26269_d_n7, assign16070_e26269_d_n8, assign16070_e26269_d_n9, assign16070_e26269_d_n10, assign16070_e26269_d_n11, assign16070_e26269_d_n13, assign16070_e26269_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard282 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tnom;
        let assign16070_e26227: f64 = (210.0 * __rspice_inv_cse_0);
        let assign16070_e26232: f64 = (210.0 * __rspice_inv_cse_0);
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
        locals.var_t2_rv = 0.0;

        let (assign16080_e26298, assign16080_e26298_d_n0, assign16080_e26298_d_n2, assign16080_e26298_d_n3, assign16080_e26298_d_n4, assign16080_e26298_d_n5, assign16080_e26298_d_n6, assign16080_e26298_d_n7, assign16080_e26298_d_n8, assign16080_e26298_d_n9, assign16080_e26298_d_n10, assign16080_e26298_d_n11, assign16080_e26298_d_n13, assign16080_e26298_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard282 == 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_tnom;
        let assign16080_e26281: f64 = (210.0 * __rspice_inv_cse_1);
        let assign16080_e26286: f64 = (210.0 * __rspice_inv_cse_1);
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
        locals.var_t1_rv = 0.0;

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
        locals.var_ua_tl_rv = 0.0;

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
        locals.var_ua_th_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_ua_t_rv = 0.0;

        let assign16130_e26392: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard283 = assign16130_e26392;
        locals.var_guard283_rv = 0.0;

        let assign16140_e26395: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign16140_e26395;
        locals.var_guard284_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

        let (assign16160_e26474, assign16160_e26474_d_n0, assign16160_e26474_d_n2, assign16160_e26474_d_n3, assign16160_e26474_d_n4, assign16160_e26474_d_n5, assign16160_e26474_d_n6, assign16160_e26474_d_n7, assign16160_e26474_d_n8, assign16160_e26474_d_n9, assign16160_e26474_d_n10, assign16160_e26474_d_n11, assign16160_e26474_d_n13, assign16160_e26474_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 != 0.0)) {
        let assign16160_e26458: f64 = (210.0 - locals.var_tnom);
        let assign16160_e26459: f64 = (locals.var_ua1r_i * assign16160_e26458);
        let assign16160_e26460: f64 = (locals.var_uar_i + assign16160_e26459);
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_tnom;
        let assign16160_e26463: f64 = (210.0 * __rspice_inv_cse_2);
        let assign16160_e26468: f64 = (210.0 * __rspice_inv_cse_2);
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
        locals.var_t1_rv = 0.0;

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
        locals.var_uar_tl_rv = 0.0;

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
        locals.var_uar_th_rv = 0.0;

        let (assign16190_e26566, assign16190_e26566_d_n0, assign16190_e26566_d_n2, assign16190_e26566_d_n3, assign16190_e26566_d_n4, assign16190_e26566_d_n5, assign16190_e26566_d_n6, assign16190_e26566_d_n7, assign16190_e26566_d_n8, assign16190_e26566_d_n9, assign16190_e26566_d_n10, assign16190_e26566_d_n11, assign16190_e26566_d_n13, assign16190_e26566_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 == 0.0)) {
        let __rspice_inv_cse_3: f64 = 1.0 / locals.var_tnom;
        let assign16190_e26524: f64 = (210.0 * __rspice_inv_cse_3);
        let assign16190_e26529: f64 = (210.0 * __rspice_inv_cse_3);
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
        locals.var_t2_rv = 0.0;

        let (assign16200_e26597, assign16200_e26597_d_n0, assign16200_e26597_d_n2, assign16200_e26597_d_n3, assign16200_e26597_d_n4, assign16200_e26597_d_n5, assign16200_e26597_d_n6, assign16200_e26597_d_n7, assign16200_e26597_d_n8, assign16200_e26597_d_n9, assign16200_e26597_d_n10, assign16200_e26597_d_n11, assign16200_e26597_d_n13, assign16200_e26597_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 == 0.0)) {
        let __rspice_inv_cse_4: f64 = 1.0 / locals.var_tnom;
        let assign16200_e26580: f64 = (210.0 * __rspice_inv_cse_4);
        let assign16200_e26585: f64 = (210.0 * __rspice_inv_cse_4);
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
        locals.var_t1_rv = 0.0;

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
        locals.var_uar_tl_rv = 0.0;

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
        locals.var_uar_th_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t0_rv = 0.0;

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
        locals.var_uar_t_rv = 0.0;

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
        locals.var_ud_t_rv = 0.0;

        let assign16260_e26716: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign16260_e26716;
        locals.var_guard285_rv = 0.0;

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
        locals.var_udr_t_rv = 0.0;

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
        locals.var_ucs_t_rv = 0.0;

        let assign16290_e26756: f64 = (locals.var_tnom - 210.0);
        let assign16290_e26757: f64 = (locals.var_uds1_i * assign16290_e26756);
        let assign16290_e26759: f64 = (assign16290_e26757 / locals.var_tnom);
        let assign16290_e26760: f64 = (assign16290_e26759).abs();
        let assign16290_e26762: f64 = if assign16290_e26760 < 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard286 = assign16290_e26762;
        locals.var_guard286_rv = 0.0;

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
        locals.var_uds_t_rv = 0.0;

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
        locals.var_uds_t_rv = 0.0;

        let assign16320_e26813: f64 = (locals.var_tnom - 210.0);
        let assign16320_e26814: f64 = (locals.var_udd1_i * assign16320_e26813);
        let assign16320_e26816: f64 = (assign16320_e26814 / locals.var_tnom);
        let assign16320_e26817: f64 = (assign16320_e26816).abs();
        let assign16320_e26819: f64 = if assign16320_e26817 < 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign16320_e26819;
        locals.var_guard287_rv = 0.0;

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
        locals.var_udd_t_rv = 0.0;

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
        locals.var_udd_t_rv = 0.0;

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
        locals.var_udseff_t_rv = 0.0;

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
        locals.var_uddeff_t_rv = 0.0;

        let assign16370_e26889: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign16370_e26889;
        locals.var_guard288_rv = 0.0;

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
        locals.var_eu_t_rv = 0.0;

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
        locals.var_vsat_t_rv = 0.0;

        let assign16400_e27013: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard289 = assign16400_e27013;
        locals.var_guard289_rv = 0.0;

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
        locals.var_vsatr_t_rv = 0.0;

        let assign16420_e27089: f64 = if locals.var_vsatr_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard290 = assign16420_e27089;
        locals.var_guard290_rv = 0.0;

        let (assign16430_e27103, assign16430_e27103_d_n4,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard290 != 0.0)) {
        (1000.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign16430_e27103;
        locals.var_vsatr_t_dn4 = assign16430_e27103_d_n4;
        locals.var_vsatr_t_rv = 0.0;

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
        locals.var_vsat1_t_rv = 0.0;

        let assign16450_e27177: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard291 = assign16450_e27177;
        locals.var_guard291_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_53(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_vsat1r_t_rv = 0.0;

        let assign16470_e27253: f64 = if locals.var_vsat1r_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard292 = assign16470_e27253;
        locals.var_guard292_rv = 0.0;

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
        locals.var_vsat1r_t_rv = 0.0;

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
        locals.var_vsatcv_t_rv = 0.0;

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
        locals.var_ksativ_t_rv = 0.0;

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
        locals.var_pclm_t_rv = 0.0;

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
        locals.var_eu_t_rv = 0.0;

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
        locals.var_vsat_t_rv = 0.0;

        let assign16540_e27663: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard293 = assign16540_e27663;
        locals.var_guard293_rv = 0.0;

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
        locals.var_vsatr_t_rv = 0.0;

        let assign16560_e27788: f64 = if locals.var_vsatr_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard294 = assign16560_e27788;
        locals.var_guard294_rv = 0.0;

        let (assign16570_e27803, assign16570_e27803_d_n4,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard293 != 0.0)) && (locals.var_guard294 != 0.0)) {
        (1000.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign16570_e27803;
        locals.var_vsatr_t_dn4 = assign16570_e27803_d_n4;
        locals.var_vsatr_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_vsat1_t_rv = 0.0;

        let assign16590_e27926: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard295 = assign16590_e27926;
        locals.var_guard295_rv = 0.0;

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
        locals.var_vsat1r_t_rv = 0.0;

        let assign16610_e28051: f64 = if locals.var_vsat1r_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign16610_e28051;
        locals.var_guard296_rv = 0.0;

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
        locals.var_vsat1r_t_rv = 0.0;

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
        locals.var_vsatcv_t_rv = 0.0;

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
        locals.var_ksativ_t_rv = 0.0;

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
        locals.var_pclm_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_mexp_t_rv = 0.0;

        let assign16670_e28522: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign16670_e28522;
        locals.var_guard297_rv = 0.0;

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
        locals.var_mexpr_t_rv = 0.0;

        let assign16690_e28656: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign16690_e28656;
        locals.var_guard298_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_u0_cv_rv = 0.0;

        let assign16720_e28738: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard299 = assign16720_e28738;
        locals.var_guard299_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_uacv_tl_rv = 0.0;

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
        locals.var_uacv_th_rv = 0.0;

        let (assign16770_e28909, assign16770_e28909_d_n0, assign16770_e28909_d_n2, assign16770_e28909_d_n3, assign16770_e28909_d_n4, assign16770_e28909_d_n5, assign16770_e28909_d_n6, assign16770_e28909_d_n7, assign16770_e28909_d_n8, assign16770_e28909_d_n9, assign16770_e28909_d_n10, assign16770_e28909_d_n11, assign16770_e28909_d_n13, assign16770_e28909_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) && (locals.var_guard299 == 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_tnom;
        let assign16770_e28867: f64 = (210.0 * __rspice_inv_cse_1);
        let assign16770_e28872: f64 = (210.0 * __rspice_inv_cse_1);
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
        locals.var_t2_rv = 0.0;

        let (assign16780_e28940, assign16780_e28940_d_n0, assign16780_e28940_d_n2, assign16780_e28940_d_n3, assign16780_e28940_d_n4, assign16780_e28940_d_n5, assign16780_e28940_d_n6, assign16780_e28940_d_n7, assign16780_e28940_d_n8, assign16780_e28940_d_n9, assign16780_e28940_d_n10, assign16780_e28940_d_n11, assign16780_e28940_d_n13, assign16780_e28940_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) && (locals.var_guard299 == 0.0)) {
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_tnom;
        let assign16780_e28923: f64 = (210.0 * __rspice_inv_cse_2);
        let assign16780_e28928: f64 = (210.0 * __rspice_inv_cse_2);
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
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        locals: &mut StampLocals,
    ) {
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
        locals.var_uacv_tl_rv = 0.0;

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
        locals.var_uacv_th_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_uacv_t_rv = 0.0;

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
        locals.var_udcv_t_rv = 0.0;

        let assign16840_e29061: f64 = if locals.var_prt_i == locals.var_prt1_i { 1.0 } else { 0.0 };
        locals.var_guard300 = assign16840_e29061;
        locals.var_guard300_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

        let assign16860_e29078: f64 = if locals.var_tr0_i < 210.0 { 1.0 } else { 0.0 };
        locals.var_guard301 = assign16860_e29078;
        locals.var_guard301_rv = 0.0;

        let assign16870_e29081: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign16870_e29081;
        locals.var_guard302_rv = 0.0;

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
        locals.var_rdstemp0_rv = 0.0;

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
        locals.var_rdstemp1_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

        let assign16920_e29178: f64 = if locals.var_prt1_i < locals.var_prt_i { 1.0 } else { 0.0 };
        locals.var_guard303 = assign16920_e29178;
        locals.var_guard303_rv = 0.0;

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
        locals.var_t5_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t5_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

        let assign16970_e29373: f64 = if locals.var_tnom > locals.var_tr0_i { 1.0 } else { 0.0 };
        locals.var_guard304 = assign16970_e29373;
        locals.var_guard304_rv = 0.0;

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
        locals.var_rdstemp0_rv = 0.0;

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
        locals.var_rdstemp1_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

    }
}
