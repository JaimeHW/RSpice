#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        locals: &mut StampLocals,
    ) {
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

        let assign52680_e67922: f64 = if locals.var_temp__blk949 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1516 = assign52680_e67922;

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

        let assign52720_e67962: f64 = if locals.var_x_m__blk1421 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1517 = assign52720_e67962;

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

        let assign52750_e68002: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1518 = assign52750_e68002;

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

        let assign52820_e68106: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1519 = assign52820_e68106;

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

    }

    pub(super) fn stamp_transient_block_49(
        locals: &mut StampLocals,
    ) {
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

        let assign53050_e68464: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1520 = assign53050_e68464;

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

        let assign53180_e68617: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1521 = assign53180_e68617;

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

    }

    pub(super) fn stamp_transient_block_50(
        locals: &mut StampLocals,
    ) {
        let (assign53260_e68676,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_xg__blk1343,)
    } else {
        (locals.var_xg_ac,)
    }
};
        locals.var_xg_ac = assign53260_e68676;

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

        let (assign53400_e68733, assign53400_e68733_d_n4,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_phib_dc, locals.var_phib_dc_dn4,)
    } else {
        (locals.var_phib_ac, locals.var_phib_ac_dn4,)
    }
};
        locals.var_phib_ac = assign53400_e68733;
        locals.var_phib_ac_dn4 = assign53400_e68733_d_n4;

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

        let (assign53440_e68753,) = {
    if (locals.var_guard1473 == 0.0) {
        (locals.var_xg_dc,)
    } else {
        (locals.var_xg_ac,)
    }
};
        locals.var_xg_ac = assign53440_e68753;

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

        locals.var_cox_qm = locals.var_cox_i;
        locals.var_cox_qm_dn4 = 0.0;
        locals.var_cox_qm_dn6 = 0.0;
        locals.var_cox_qm_dn7 = 0.0;
        locals.var_cox_qm_dn8 = 0.0;
        locals.var_cox_qm_dn9 = 0.0;

        let assign53600_e68827: f64 = if locals.var_qq > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1522 = assign53600_e68827;

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

        locals.var_gdl_ac = 1.0;
        locals.var_gdl_ac_dn4 = 0.0;
        locals.var_gdl_ac_dn6 = 0.0;
        locals.var_gdl_ac_dn7 = 0.0;
        locals.var_gdl_ac_dn8 = 0.0;
        locals.var_gdl_ac_dn9 = 0.0;

        locals.var_gmob_dl_ac = 1.0;
        locals.var_gmob_dl_ac_dn4 = 0.0;
        locals.var_gmob_dl_ac_dn6 = 0.0;
        locals.var_gmob_dl_ac_dn7 = 0.0;
        locals.var_gmob_dl_ac_dn8 = 0.0;
        locals.var_gmob_dl_ac_dn9 = 0.0;

        locals.var_thesat1_ac = 0.0;
        locals.var_thesat1_ac_dn4 = 0.0;
        locals.var_thesat1_ac_dn6 = 0.0;
        locals.var_thesat1_ac_dn7 = 0.0;
        locals.var_thesat1_ac_dn8 = 0.0;
        locals.var_thesat1_ac_dn9 = 0.0;

        locals.var_gvsat_ac = 1.0;
        locals.var_gvsat_ac_dn4 = 0.0;
        locals.var_gvsat_ac_dn6 = 0.0;
        locals.var_gvsat_ac_dn7 = 0.0;
        locals.var_gvsat_ac_dn8 = 0.0;
        locals.var_gvsat_ac_dn9 = 0.0;

        locals.var_h_ac = 1.0;
        locals.var_h_ac_dn4 = 0.0;
        locals.var_h_ac_dn6 = 0.0;
        locals.var_h_ac_dn7 = 0.0;
        locals.var_h_ac_dn8 = 0.0;
        locals.var_h_ac_dn9 = 0.0;

        locals.var_qg_1 = locals.var_voxm_ac;
        locals.var_qg_1_dn4 = locals.var_voxm_ac_dn4;
        locals.var_qg_1_dn6 = locals.var_voxm_ac_dn6;
        locals.var_qg_1_dn7 = locals.var_voxm_ac_dn7;
        locals.var_qg_1_dn8 = locals.var_voxm_ac_dn8;
        locals.var_qg_1_dn9 = locals.var_voxm_ac_dn9;

    }

    pub(super) fn stamp_transient_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_qi = 0.0;
        locals.var_qi_dn4 = 0.0;
        locals.var_qi_dn6 = 0.0;
        locals.var_qi_dn7 = 0.0;
        locals.var_qi_dn8 = 0.0;
        locals.var_qi_dn9 = 0.0;

        locals.var_qd_1 = 0.0;
        locals.var_qd_1_dn4 = 0.0;
        locals.var_qd_1_dn6 = 0.0;
        locals.var_qd_1_dn7 = 0.0;
        locals.var_qd_1_dn8 = 0.0;
        locals.var_qd_1_dn9 = 0.0;

        locals.var_qb_1 = locals.var_qg_1;
        locals.var_qb_1_dn4 = locals.var_qg_1_dn4;
        locals.var_qb_1_dn6 = locals.var_qg_1_dn6;
        locals.var_qb_1_dn7 = locals.var_qg_1_dn7;
        locals.var_qb_1_dn8 = locals.var_qg_1_dn8;
        locals.var_qb_1_dn9 = locals.var_qg_1_dn9;

        let assign53710_e68858: f64 = if locals.var_xg_ac > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1523 = assign53710_e68858;

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

        let assign53730_e68875: f64 = if locals.var_dl__blk1280 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1524 = assign53730_e68875;

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

        let assign53790_e68923: f64 = (-1.0);
        let assign53790_e68924: f64 = if locals.var_chnl_type == assign53790_e68923 { 1.0 } else { 0.0 };
        locals.var_guard1525 = assign53790_e68924;

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

        let assign53890_e69024: f64 = if p.p49 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1526 = assign53890_e69024;

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

        let assign53960_e69117: f64 = (locals.var_qg_1 * locals.var_cox_qm);
        locals.var_qg = assign53960_e69117;
        locals.var_qg_dn4 = ((locals.var_qg_1_dn4 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn4));
        locals.var_qg_dn6 = ((locals.var_qg_1_dn6 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn6));
        locals.var_qg_dn7 = ((locals.var_qg_1_dn7 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn7));
        locals.var_qg_dn8 = ((locals.var_qg_1_dn8 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn8));
        locals.var_qg_dn9 = ((locals.var_qg_1_dn9 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn9));

        let assign53970_e69119: f64 = (-locals.var_qd_1);
        let assign53970_e69121: f64 = (assign53970_e69119 * locals.var_cox_qm);
        locals.var_qd = assign53970_e69121;
        locals.var_qd_dn4 = (((-locals.var_qd_1_dn4) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn4));
        locals.var_qd_dn6 = (((-locals.var_qd_1_dn6) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn6));
        locals.var_qd_dn7 = (((-locals.var_qd_1_dn7) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn7));
        locals.var_qd_dn8 = (((-locals.var_qd_1_dn8) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn8));
        locals.var_qd_dn9 = (((-locals.var_qd_1_dn9) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn9));

        let assign53980_e69123: f64 = (-locals.var_qb_1);
        let assign53980_e69125: f64 = (assign53980_e69123 * locals.var_cox_qm);
        locals.var_qb = assign53980_e69125;
        locals.var_qb_dn4 = (((-locals.var_qb_1_dn4) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn4));
        locals.var_qb_dn6 = (((-locals.var_qb_1_dn6) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn6));
        locals.var_qb_dn7 = (((-locals.var_qb_1_dn7) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn7));
        locals.var_qb_dn8 = (((-locals.var_qb_1_dn8) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn8));
        locals.var_qb_dn9 = (((-locals.var_qb_1_dn9) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn9));

        locals.var_qsinr = 0.0;
        locals.var_qsinr_dn4 = 0.0;
        locals.var_qsinr_dn6 = 0.0;
        locals.var_qsinr_dn7 = 0.0;
        locals.var_qsinr_dn8 = 0.0;
        locals.var_qsinr_dn9 = 0.0;

        locals.var_qdinr = 0.0;
        locals.var_qdinr_dn4 = 0.0;
        locals.var_qdinr_dn6 = 0.0;
        locals.var_qdinr_dn7 = 0.0;
        locals.var_qdinr_dn8 = 0.0;
        locals.var_qdinr_dn9 = 0.0;

        locals.var_qginr = 0.0;
        locals.var_qginr_dn4 = 0.0;
        locals.var_qginr_dn6 = 0.0;
        locals.var_qginr_dn7 = 0.0;
        locals.var_qginr_dn8 = 0.0;
        locals.var_qginr_dn9 = 0.0;

        let assign54020_e69135: f64 = if ((locals.var_cinr_i > 0.0) || (locals.var_cinrd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1527 = assign54020_e69135;

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

        let assign54050_e69146: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1528 = assign54050_e69146;

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

    }

    pub(super) fn stamp_transient_block_52(
        locals: &mut StampLocals,
    ) {
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

        let assign54160_e69267: f64 = if locals.var_fcinrdep_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1529 = assign54160_e69267;

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

        let assign54190_e69293: f64 = (locals.var_xginrdep).abs();
        let assign54190_e69295: f64 = if assign54190_e69293 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1530 = assign54190_e69295;

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

        let assign54210_e69312: f64 = if locals.var_xginrdep < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1531 = assign54210_e69312;

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

        let assign54230_e69351: f64 = if locals.var_xginrdep < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1532 = assign54230_e69351;

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

        let assign54330_e69441: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1533 = assign54330_e69441;

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

    }

    pub(super) fn stamp_transient_block_53(
        locals: &mut StampLocals,
    ) {
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

        locals.var_qg_ov_s = 0.0;
        locals.var_qg_ov_s_dn4 = 0.0;
        locals.var_qg_ov_s_dn6 = 0.0;
        locals.var_qg_ov_s_dn7 = 0.0;
        locals.var_qg_ov_s_dn8 = 0.0;
        locals.var_qg_ov_s_dn9 = 0.0;

        locals.var_yb_ov_s = 0.0;
        locals.var_yb_ov_s_dn4 = 0.0;
        locals.var_yb_ov_s_dn6 = 0.0;
        locals.var_yb_ov_s_dn7 = 0.0;
        locals.var_yb_ov_s_dn8 = 0.0;
        locals.var_yb_ov_s_dn9 = 0.0;

        let assign54470_e69549: f64 = if ((locals.var_cgov_i > 0.0) && (locals.var_fcgovacc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1534 = assign54470_e69549;

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

        let assign54490_e69562: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1535 = assign54490_e69562;

        let assign54500_e69565: f64 = (-230.25850929940458);
        let assign54500_e69566: f64 = if locals.var_temp__blk949 > assign54500_e69565 { 1.0 } else { 0.0 };
        locals.var_guard1536 = assign54500_e69566;

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

        let assign54530_e69612: f64 = if locals.var_yb_ov_s > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1537 = assign54530_e69612;

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

        let (assign54570_e69666, assign54570_e69666_d_n4, assign54570_e69666_d_n6, assign54570_e69666_d_n7, assign54570_e69666_d_n8, assign54570_e69666_d_n9,) = {
    if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1537 == 0.0)) {
        let assign54570_e69660: f64 = (2.0 * locals.var_xgbeff_ov_s);
        let assign54570_e69663: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54570_e69664: f64 = (assign54570_e69660 / assign54570_e69663);
        (assign54570_e69664, ((((2.0 * locals.var_xgbeff_ov_s_dn4) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn4)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn6) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn6)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn7) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn7)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn8) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn8)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn9) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn9)) / (assign54570_e69663 * assign54570_e69663)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54570_e69666;
        locals.var_temp1_dn4 = assign54570_e69666_d_n4;
        locals.var_temp1_dn6 = assign54570_e69666_d_n6;
        locals.var_temp1_dn7 = assign54570_e69666_d_n7;
        locals.var_temp1_dn8 = assign54570_e69666_d_n8;
        locals.var_temp1_dn9 = assign54570_e69666_d_n9;

        let (assign54580_e69673, assign54580_e69673_d_n4, assign54580_e69673_d_n6, assign54580_e69673_d_n7, assign54580_e69673_d_n8, assign54580_e69673_d_n9,) = {
    if ((locals.var_guard1534 != 0.0) && (locals.var_guard1535 == 0.0)) {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn4, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8, locals.var_xgbeff_ov_s_dn9,)
    }
};
        locals.var_xgbeff_ov_s = assign54580_e69673;
        locals.var_xgbeff_ov_s_dn4 = assign54580_e69673_d_n4;
        locals.var_xgbeff_ov_s_dn6 = assign54580_e69673_d_n6;
        locals.var_xgbeff_ov_s_dn7 = assign54580_e69673_d_n7;
        locals.var_xgbeff_ov_s_dn8 = assign54580_e69673_d_n8;
        locals.var_xgbeff_ov_s_dn9 = assign54580_e69673_d_n9;

        let (assign54590_e69691, assign54590_e69691_d_n4, assign54590_e69691_d_n6, assign54590_e69691_d_n7, assign54590_e69691_d_n8, assign54590_e69691_d_n9,) = {
    if ((locals.var_guard1534 != 0.0) && (locals.var_guard1535 == 0.0)) {
        let assign54590_e69682: f64 = (1.0 + locals.var_xgbeff_ov_s);
        let assign54590_e69683: f64 = (assign54590_e69682).ln();
        let assign54590_e69686: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54590_e69687: f64 = (assign54590_e69683 / assign54590_e69686);
        let assign54590_e69688: f64 = (1.0 - assign54590_e69687);
        let assign54590_e69689: f64 = (locals.var_xgbeff_ov_s * assign54590_e69688);
        (assign54590_e69689, ((locals.var_xgbeff_ov_s_dn4 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn4 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn4)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn6 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn6)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn7 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn7)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn8 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn8)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn9 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn9 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn9)) / (assign54590_e69686 * assign54590_e69686))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54590_e69691;
        locals.var_temp1_dn4 = assign54590_e69691_d_n4;
        locals.var_temp1_dn6 = assign54590_e69691_d_n6;
        locals.var_temp1_dn7 = assign54590_e69691_d_n7;
        locals.var_temp1_dn8 = assign54590_e69691_d_n8;
        locals.var_temp1_dn9 = assign54590_e69691_d_n9;

        let (assign54600_e69706, assign54600_e69706_d_n4, assign54600_e69706_d_n6, assign54600_e69706_d_n7, assign54600_e69706_d_n8, assign54600_e69706_d_n9,) = {
    if (locals.var_guard1534 != 0.0) {
        let assign54600_e69694: f64 = (-2.0);
        let assign54600_e69696: f64 = (assign54600_e69694 * locals.var_fcgovacc_i);
        let assign54600_e69698: f64 = (assign54600_e69696 / locals.var_cgovaccg_i);
        let assign54600_e69700: f64 = (assign54600_e69698 * locals.var_cgov_i);
        let assign54600_e69702: f64 = (assign54600_e69700 * locals.var_phita);
        let assign54600_e69704: f64 = (assign54600_e69702 * locals.var_temp1);
        (assign54600_e69704, (assign54600_e69702 * locals.var_temp1_dn4), (assign54600_e69702 * locals.var_temp1_dn6), (assign54600_e69702 * locals.var_temp1_dn7), (assign54600_e69702 * locals.var_temp1_dn8), (assign54600_e69702 * locals.var_temp1_dn9),)
    } else {
        (locals.var_qg_ov_s, locals.var_qg_ov_s_dn4, locals.var_qg_ov_s_dn6, locals.var_qg_ov_s_dn7, locals.var_qg_ov_s_dn8, locals.var_qg_ov_s_dn9,)
    }
};
        locals.var_qg_ov_s = assign54600_e69706;
        locals.var_qg_ov_s_dn4 = assign54600_e69706_d_n4;
        locals.var_qg_ov_s_dn6 = assign54600_e69706_d_n6;
        locals.var_qg_ov_s_dn7 = assign54600_e69706_d_n7;
        locals.var_qg_ov_s_dn8 = assign54600_e69706_d_n8;
        locals.var_qg_ov_s_dn9 = assign54600_e69706_d_n9;

        locals.var_qg_ov_d = 0.0;
        locals.var_qg_ov_d_dn4 = 0.0;
        locals.var_qg_ov_d_dn6 = 0.0;
        locals.var_qg_ov_d_dn7 = 0.0;
        locals.var_qg_ov_d_dn8 = 0.0;
        locals.var_qg_ov_d_dn9 = 0.0;

        locals.var_yb_ov_d = 0.0;
        locals.var_yb_ov_d_dn4 = 0.0;
        locals.var_yb_ov_d_dn6 = 0.0;
        locals.var_yb_ov_d_dn7 = 0.0;
        locals.var_yb_ov_d_dn8 = 0.0;
        locals.var_yb_ov_d_dn9 = 0.0;

        let assign54630_e69715: f64 = if ((locals.var_cgovd_i > 0.0) && (locals.var_fcgovaccd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1538 = assign54630_e69715;

        let (assign54640_e69725, assign54640_e69725_d_n4, assign54640_e69725_d_n6, assign54640_e69725_d_n7, assign54640_e69725_d_n8, assign54640_e69725_d_n9,) = {
    if (locals.var_guard1538 != 0.0) {
        let assign54640_e69720: f64 = (0.5 * locals.var_xgb_ov);
        let assign54640_e69722: f64 = (assign54640_e69720 + locals.var_dxgb_ov_d);
        let assign54640_e69723: f64 = (locals.var_cgovaccg_i * assign54640_e69722);
        (assign54640_e69723, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn4)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign54640_e69725;
        locals.var_temp__blk949_dn4 = assign54640_e69725_d_n4;
        locals.var_temp__blk949_dn6 = assign54640_e69725_d_n6;
        locals.var_temp__blk949_dn7 = assign54640_e69725_d_n7;
        locals.var_temp__blk949_dn8 = assign54640_e69725_d_n8;
        locals.var_temp__blk949_dn9 = assign54640_e69725_d_n9;

        let assign54650_e69728: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1539 = assign54650_e69728;

        let assign54660_e69731: f64 = (-230.25850929940458);
        let assign54660_e69732: f64 = if locals.var_temp__blk949 > assign54660_e69731 { 1.0 } else { 0.0 };
        locals.var_guard1540 = assign54660_e69732;

        let (assign54670_e69741, assign54670_e69741_d_n4, assign54670_e69741_d_n6, assign54670_e69741_d_n7, assign54670_e69741_d_n8, assign54670_e69741_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1540 != 0.0)) {
        let assign54670_e69739: f64 = (locals.var_temp__blk949).exp();
        (assign54670_e69739, (assign54670_e69739 * locals.var_temp__blk949_dn4), (assign54670_e69739 * locals.var_temp__blk949_dn6), (assign54670_e69739 * locals.var_temp__blk949_dn7), (assign54670_e69739 * locals.var_temp__blk949_dn8), (assign54670_e69739 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn4, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, locals.var_yb_ov_d_dn9,)
    }
};
        locals.var_yb_ov_d = assign54670_e69741;
        locals.var_yb_ov_d_dn4 = assign54670_e69741_d_n4;
        locals.var_yb_ov_d_dn6 = assign54670_e69741_d_n6;
        locals.var_yb_ov_d_dn7 = assign54670_e69741_d_n7;
        locals.var_yb_ov_d_dn8 = assign54670_e69741_d_n8;
        locals.var_yb_ov_d_dn9 = assign54670_e69741_d_n9;

        let (assign54680_e69775, assign54680_e69775_d_n4, assign54680_e69775_d_n6, assign54680_e69775_d_n7, assign54680_e69775_d_n8, assign54680_e69775_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1540 == 0.0)) {
        let assign54680_e69751: f64 = (-230.25850929940458);
        let assign54680_e69753: f64 = (assign54680_e69751 - locals.var_temp__blk949);
        let assign54680_e69757: f64 = (-230.25850929940458);
        let assign54680_e69759: f64 = (assign54680_e69757 - locals.var_temp__blk949);
        let assign54680_e69762: f64 = (-230.25850929940458);
        let assign54680_e69764: f64 = (assign54680_e69762 - locals.var_temp__blk949);
        let assign54680_e69766: f64 = (assign54680_e69764 * 0.3333333333333333);
        let assign54680_e69767: f64 = (1.0 + assign54680_e69766);
        let assign54680_e69768: f64 = (assign54680_e69759 * assign54680_e69767);
        let assign54680_e69769: f64 = (0.5 * assign54680_e69768);
        let assign54680_e69770: f64 = (1.0 + assign54680_e69769);
        let assign54680_e69771: f64 = (assign54680_e69753 * assign54680_e69770);
        let assign54680_e69772: f64 = (1.0 + assign54680_e69771);
        let assign54680_e69773: f64 = (1e-100 / assign54680_e69772);
        (assign54680_e69773, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))),)
    } else {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn4, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, locals.var_yb_ov_d_dn9,)
    }
};
        locals.var_yb_ov_d = assign54680_e69775;
        locals.var_yb_ov_d_dn4 = assign54680_e69775_d_n4;
        locals.var_yb_ov_d_dn6 = assign54680_e69775_d_n6;
        locals.var_yb_ov_d_dn7 = assign54680_e69775_d_n7;
        locals.var_yb_ov_d_dn8 = assign54680_e69775_d_n8;
        locals.var_yb_ov_d_dn9 = assign54680_e69775_d_n9;

        let assign54690_e69778: f64 = if locals.var_yb_ov_d > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1541 = assign54690_e69778;

        let (assign54700_e69789, assign54700_e69789_d_n4, assign54700_e69789_d_n6, assign54700_e69789_d_n7, assign54700_e69789_d_n8, assign54700_e69789_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign54700_e69786: f64 = (1.0 + locals.var_yb_ov_d);
        let assign54700_e69787: f64 = (assign54700_e69786).ln();
        (assign54700_e69787, (locals.var_yb_ov_d_dn4 / assign54700_e69786), (locals.var_yb_ov_d_dn6 / assign54700_e69786), (locals.var_yb_ov_d_dn7 / assign54700_e69786), (locals.var_yb_ov_d_dn8 / assign54700_e69786), (locals.var_yb_ov_d_dn9 / assign54700_e69786),)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn4, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, locals.var_xgbeff_ov_d_dn9,)
    }
};
        locals.var_xgbeff_ov_d = assign54700_e69789;
        locals.var_xgbeff_ov_d_dn4 = assign54700_e69789_d_n4;
        locals.var_xgbeff_ov_d_dn6 = assign54700_e69789_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54700_e69789_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54700_e69789_d_n8;
        locals.var_xgbeff_ov_d_dn9 = assign54700_e69789_d_n9;

        let (assign54710_e69808, assign54710_e69808_d_n4, assign54710_e69808_d_n6, assign54710_e69808_d_n7, assign54710_e69808_d_n8, assign54710_e69808_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign54710_e69799: f64 = (1.0 + locals.var_xgbeff_ov_d);
        let assign54710_e69800: f64 = (assign54710_e69799).ln();
        let assign54710_e69803: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54710_e69804: f64 = (assign54710_e69800 / assign54710_e69803);
        let assign54710_e69805: f64 = (1.0 - assign54710_e69804);
        let assign54710_e69806: f64 = (locals.var_xgbeff_ov_d * assign54710_e69805);
        (assign54710_e69806, ((locals.var_xgbeff_ov_d_dn4 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn4 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn4)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn6 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn6)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn7 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn7)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn8 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn8)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn9 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn9 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn9)) / (assign54710_e69803 * assign54710_e69803))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54710_e69808;
        locals.var_temp1_dn4 = assign54710_e69808_d_n4;
        locals.var_temp1_dn6 = assign54710_e69808_d_n6;
        locals.var_temp1_dn7 = assign54710_e69808_d_n7;
        locals.var_temp1_dn8 = assign54710_e69808_d_n8;
        locals.var_temp1_dn9 = assign54710_e69808_d_n9;

        let (assign54720_e69817, assign54720_e69817_d_n4, assign54720_e69817_d_n6, assign54720_e69817_d_n7, assign54720_e69817_d_n8, assign54720_e69817_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 == 0.0)) {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn4, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, locals.var_yb_ov_d_dn9,)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn4, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, locals.var_xgbeff_ov_d_dn9,)
    }
};
        locals.var_xgbeff_ov_d = assign54720_e69817;
        locals.var_xgbeff_ov_d_dn4 = assign54720_e69817_d_n4;
        locals.var_xgbeff_ov_d_dn6 = assign54720_e69817_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54720_e69817_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54720_e69817_d_n8;
        locals.var_xgbeff_ov_d_dn9 = assign54720_e69817_d_n9;

        let (assign54730_e69832, assign54730_e69832_d_n4, assign54730_e69832_d_n6, assign54730_e69832_d_n7, assign54730_e69832_d_n8, assign54730_e69832_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 == 0.0)) {
        let assign54730_e69826: f64 = (2.0 * locals.var_xgbeff_ov_d);
        let assign54730_e69829: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54730_e69830: f64 = (assign54730_e69826 / assign54730_e69829);
        (assign54730_e69830, ((((2.0 * locals.var_xgbeff_ov_d_dn4) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn4)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn6) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn6)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn7) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn7)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn8) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn8)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn9) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn9)) / (assign54730_e69829 * assign54730_e69829)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54730_e69832;
        locals.var_temp1_dn4 = assign54730_e69832_d_n4;
        locals.var_temp1_dn6 = assign54730_e69832_d_n6;
        locals.var_temp1_dn7 = assign54730_e69832_d_n7;
        locals.var_temp1_dn8 = assign54730_e69832_d_n8;
        locals.var_temp1_dn9 = assign54730_e69832_d_n9;

        let (assign54740_e69839, assign54740_e69839_d_n4, assign54740_e69839_d_n6, assign54740_e69839_d_n7, assign54740_e69839_d_n8, assign54740_e69839_d_n9,) = {
    if ((locals.var_guard1538 != 0.0) && (locals.var_guard1539 == 0.0)) {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn4, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, locals.var_xgbeff_ov_d_dn9,)
    }
};
        locals.var_xgbeff_ov_d = assign54740_e69839;
        locals.var_xgbeff_ov_d_dn4 = assign54740_e69839_d_n4;
        locals.var_xgbeff_ov_d_dn6 = assign54740_e69839_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54740_e69839_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54740_e69839_d_n8;
        locals.var_xgbeff_ov_d_dn9 = assign54740_e69839_d_n9;

        let (assign54750_e69857, assign54750_e69857_d_n4, assign54750_e69857_d_n6, assign54750_e69857_d_n7, assign54750_e69857_d_n8, assign54750_e69857_d_n9,) = {
    if ((locals.var_guard1538 != 0.0) && (locals.var_guard1539 == 0.0)) {
        let assign54750_e69848: f64 = (1.0 + locals.var_xgbeff_ov_d);
        let assign54750_e69849: f64 = (assign54750_e69848).ln();
        let assign54750_e69852: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54750_e69853: f64 = (assign54750_e69849 / assign54750_e69852);
        let assign54750_e69854: f64 = (1.0 - assign54750_e69853);
        let assign54750_e69855: f64 = (locals.var_xgbeff_ov_d * assign54750_e69854);
        (assign54750_e69855, ((locals.var_xgbeff_ov_d_dn4 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn4 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn4)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn6 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn6)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn7 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn7)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn8 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn8)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn9 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn9 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn9)) / (assign54750_e69852 * assign54750_e69852))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54750_e69857;
        locals.var_temp1_dn4 = assign54750_e69857_d_n4;
        locals.var_temp1_dn6 = assign54750_e69857_d_n6;
        locals.var_temp1_dn7 = assign54750_e69857_d_n7;
        locals.var_temp1_dn8 = assign54750_e69857_d_n8;
        locals.var_temp1_dn9 = assign54750_e69857_d_n9;

        let (assign54760_e69872, assign54760_e69872_d_n4, assign54760_e69872_d_n6, assign54760_e69872_d_n7, assign54760_e69872_d_n8, assign54760_e69872_d_n9,) = {
    if (locals.var_guard1538 != 0.0) {
        let assign54760_e69860: f64 = (-2.0);
        let assign54760_e69862: f64 = (assign54760_e69860 * locals.var_fcgovaccd_i);
        let assign54760_e69864: f64 = (assign54760_e69862 / locals.var_cgovaccg_i);
        let assign54760_e69866: f64 = (assign54760_e69864 * locals.var_cgovd_i);
        let assign54760_e69868: f64 = (assign54760_e69866 * locals.var_phita);
        let assign54760_e69870: f64 = (assign54760_e69868 * locals.var_temp1);
        (assign54760_e69870, (assign54760_e69868 * locals.var_temp1_dn4), (assign54760_e69868 * locals.var_temp1_dn6), (assign54760_e69868 * locals.var_temp1_dn7), (assign54760_e69868 * locals.var_temp1_dn8), (assign54760_e69868 * locals.var_temp1_dn9),)
    } else {
        (locals.var_qg_ov_d, locals.var_qg_ov_d_dn4, locals.var_qg_ov_d_dn6, locals.var_qg_ov_d_dn7, locals.var_qg_ov_d_dn8, locals.var_qg_ov_d_dn9,)
    }
};
        locals.var_qg_ov_d = assign54760_e69872;
        locals.var_qg_ov_d_dn4 = assign54760_e69872_d_n4;
        locals.var_qg_ov_d_dn6 = assign54760_e69872_d_n6;
        locals.var_qg_ov_d_dn7 = assign54760_e69872_d_n7;
        locals.var_qg_ov_d_dn8 = assign54760_e69872_d_n8;
        locals.var_qg_ov_d_dn9 = assign54760_e69872_d_n9;

        let assign54770_e69875: f64 = (locals.var_qg_ov_s + locals.var_qg_ov_d);
        locals.var_qg_ov = assign54770_e69875;
        locals.var_qg_ov_dn4 = (locals.var_qg_ov_s_dn4 + locals.var_qg_ov_d_dn4);
        locals.var_qg_ov_dn6 = (locals.var_qg_ov_s_dn6 + locals.var_qg_ov_d_dn6);
        locals.var_qg_ov_dn7 = (locals.var_qg_ov_s_dn7 + locals.var_qg_ov_d_dn7);
        locals.var_qg_ov_dn8 = (locals.var_qg_ov_s_dn8 + locals.var_qg_ov_d_dn8);
        locals.var_qg_ov_dn9 = (locals.var_qg_ov_s_dn9 + locals.var_qg_ov_d_dn9);

        let assign54780_e69878: f64 = (locals.var_cgbov_i * locals.var_vgb);
        let assign54780_e69880: f64 = (assign54780_e69878 + locals.var_qg_ov);
        locals.var_qgb_ov = assign54780_e69880;
        locals.var_qgb_ov_dn4 = locals.var_qg_ov_dn4;
        locals.var_qgb_ov_dn6 = ((locals.var_cgbov_i * locals.var_vgb_dn6) + locals.var_qg_ov_dn6);
        locals.var_qgb_ov_dn7 = ((locals.var_cgbov_i * locals.var_vgb_dn7) + locals.var_qg_ov_dn7);
        locals.var_qgb_ov_dn8 = ((locals.var_cgbov_i * locals.var_vgb_dn8) + locals.var_qg_ov_dn8);
        locals.var_qgb_ov_dn9 = ((locals.var_cgbov_i * locals.var_vgb_dn9) + locals.var_qg_ov_dn9);

        let assign62070_e80735: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1735 = assign62070_e80735;

        locals.var_pdiss_1 = 0.0;
        locals.var_pdiss_1_dn0 = 0.0;
        locals.var_pdiss_1_dn2 = 0.0;
        locals.var_pdiss_1_dn4 = 0.0;
        locals.var_pdiss_1_dn6 = 0.0;
        locals.var_pdiss_1_dn7 = 0.0;
        locals.var_pdiss_1_dn8 = 0.0;
        locals.var_pdiss_1_dn9 = 0.0;

        locals.var_pdiss_s = 0.0;
        locals.var_pdiss_s_dn2 = 0.0;
        locals.var_pdiss_s_dn7 = 0.0;

        locals.var_pdiss_d = 0.0;
        locals.var_pdiss_d_dn0 = 0.0;
        locals.var_pdiss_d_dn8 = 0.0;

        let assign62180_e80762: f64 = if locals.var_rse_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1745 = assign62180_e80762;

    }

    pub(super) fn stamp_transient_block_54(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (assign62190_e80770, assign62190_e80770_d_n2, assign62190_e80770_d_n7,) = {
    if (locals.var_guard1745 != 0.0) {
        let assign62190_e80766: f64 = (locals.var_gsource * (nv2 - nv7));
        let assign62190_e80768: f64 = (assign62190_e80766 * (nv2 - nv7));
        (assign62190_e80768, ((locals.var_gsource * (nv2 - nv7)) + assign62190_e80766), (((-locals.var_gsource) * (nv2 - nv7)) + (-assign62190_e80766)),)
    } else {
        (locals.var_pdiss_s, locals.var_pdiss_s_dn2, locals.var_pdiss_s_dn7,)
    }
};
        locals.var_pdiss_s = assign62190_e80770;
        locals.var_pdiss_s_dn2 = assign62190_e80770_d_n2;
        locals.var_pdiss_s_dn7 = assign62190_e80770_d_n7;

        let assign62200_e80773: f64 = if locals.var_rde_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1746 = assign62200_e80773;

        let (assign62210_e80781, assign62210_e80781_d_n0, assign62210_e80781_d_n8,) = {
    if (locals.var_guard1746 != 0.0) {
        let assign62210_e80777: f64 = (locals.var_gdrain * (nv0 - nv8));
        let assign62210_e80779: f64 = (assign62210_e80777 * (nv0 - nv8));
        (assign62210_e80779, ((locals.var_gdrain * (nv0 - nv8)) + assign62210_e80777), (((-locals.var_gdrain) * (nv0 - nv8)) + (-assign62210_e80777)),)
    } else {
        (locals.var_pdiss_d, locals.var_pdiss_d_dn0, locals.var_pdiss_d_dn8,)
    }
};
        locals.var_pdiss_d = assign62210_e80781;
        locals.var_pdiss_d_dn0 = assign62210_e80781_d_n0;
        locals.var_pdiss_d_dn8 = assign62210_e80781_d_n8;

        let assign62220_e80784: f64 = if locals.var_rth_p > 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1747 = assign62220_e80784;

        let (assign62230_e80802, assign62230_e80802_d_n0, assign62230_e80802_d_n2, assign62230_e80802_d_n4, assign62230_e80802_d_n6, assign62230_e80802_d_n7, assign62230_e80802_d_n8, assign62230_e80802_d_n9,) = {
    if (locals.var_guard1747 != 0.0) {
        let assign62230_e80788: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let assign62230_e80790: f64 = (assign62230_e80788 * locals.var_v_ds);
        let assign62230_e80794: f64 = (locals.var_v_ds + locals.var_v_sb);
        let assign62230_e80795: f64 = (locals.var_iimpact * assign62230_e80794);
        let assign62230_e80796: f64 = (assign62230_e80790 + assign62230_e80795);
        let assign62230_e80798: f64 = (assign62230_e80796 + locals.var_pdiss_s);
        let assign62230_e80800: f64 = (assign62230_e80798 + locals.var_pdiss_d);
        (assign62230_e80800, locals.var_pdiss_d_dn0, locals.var_pdiss_s_dn2, (((locals.var_i_ds_dn4 + locals.var_i_dsedge_dn4) * locals.var_v_ds) + (locals.var_iimpact_dn4 * assign62230_e80794)), (((locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6) * locals.var_v_ds) + (locals.var_iimpact_dn6 * assign62230_e80794)), (((((locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7) * locals.var_v_ds) + (assign62230_e80788 * locals.var_v_ds_dn7)) + ((locals.var_iimpact_dn7 * assign62230_e80794) + (locals.var_iimpact * (locals.var_v_ds_dn7 + locals.var_v_sb_dn7)))) + locals.var_pdiss_s_dn7), (((((locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8) * locals.var_v_ds) + (assign62230_e80788 * locals.var_v_ds_dn8)) + ((locals.var_iimpact_dn8 * assign62230_e80794) + (locals.var_iimpact * (locals.var_v_ds_dn8 + locals.var_v_sb_dn8)))) + locals.var_pdiss_d_dn8), (((locals.var_i_ds_dn9 + locals.var_i_dsedge_dn9) * locals.var_v_ds) + ((locals.var_iimpact_dn9 * assign62230_e80794) + (locals.var_iimpact * locals.var_v_sb_dn9))),)
    } else {
        (locals.var_pdiss_1, locals.var_pdiss_1_dn0, locals.var_pdiss_1_dn2, locals.var_pdiss_1_dn4, locals.var_pdiss_1_dn6, locals.var_pdiss_1_dn7, locals.var_pdiss_1_dn8, locals.var_pdiss_1_dn9,)
    }
};
        locals.var_pdiss_1 = assign62230_e80802;
        locals.var_pdiss_1_dn0 = assign62230_e80802_d_n0;
        locals.var_pdiss_1_dn2 = assign62230_e80802_d_n2;
        locals.var_pdiss_1_dn4 = assign62230_e80802_d_n4;
        locals.var_pdiss_1_dn6 = assign62230_e80802_d_n6;
        locals.var_pdiss_1_dn7 = assign62230_e80802_d_n7;
        locals.var_pdiss_1_dn8 = assign62230_e80802_d_n8;
        locals.var_pdiss_1_dn9 = assign62230_e80802_d_n9;

        let assign62240_e80805: f64 = (locals.var_qg + locals.var_qb);
        let assign62240_e80807: f64 = (assign62240_e80805 + locals.var_qd);
        let assign62240_e80808: f64 = (-assign62240_e80807);
        locals.var_qs = assign62240_e80808;
        locals.var_qs_dn4 = (-((locals.var_qg_dn4 + locals.var_qb_dn4) + locals.var_qd_dn4));
        locals.var_qs_dn6 = (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6));
        locals.var_qs_dn7 = (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7));
        locals.var_qs_dn8 = (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8));
        locals.var_qs_dn9 = (-((locals.var_qg_dn9 + locals.var_qb_dn9) + locals.var_qd_dn9));

        let assign62290_e80839: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1749 = assign62290_e80839;

        let (assign62300_e80843, assign62300_e80843_d_n4, assign62300_e80843_d_n6, assign62300_e80843_d_n7, assign62300_e80843_d_n8, assign62300_e80843_d_n9,) = {
    if (locals.var_guard1749 != 0.0) {
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9,)
    } else {
        (locals.var_temp__blk1748, locals.var_temp__blk1748_dn4, locals.var_temp__blk1748_dn6, locals.var_temp__blk1748_dn7, locals.var_temp__blk1748_dn8, locals.var_temp__blk1748_dn9,)
    }
};
        locals.var_temp__blk1748 = assign62300_e80843;
        locals.var_temp__blk1748_dn4 = assign62300_e80843_d_n4;
        locals.var_temp__blk1748_dn6 = assign62300_e80843_d_n6;
        locals.var_temp__blk1748_dn7 = assign62300_e80843_d_n7;
        locals.var_temp__blk1748_dn8 = assign62300_e80843_d_n8;
        locals.var_temp__blk1748_dn9 = assign62300_e80843_d_n9;

        let (assign62310_e80847, assign62310_e80847_d_n4, assign62310_e80847_d_n6, assign62310_e80847_d_n7, assign62310_e80847_d_n8, assign62310_e80847_d_n9,) = {
    if (locals.var_guard1749 != 0.0) {
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9,)
    } else {
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9,)
    }
};
        locals.var_qd = assign62310_e80847;
        locals.var_qd_dn4 = assign62310_e80847_d_n4;
        locals.var_qd_dn6 = assign62310_e80847_d_n6;
        locals.var_qd_dn7 = assign62310_e80847_d_n7;
        locals.var_qd_dn8 = assign62310_e80847_d_n8;
        locals.var_qd_dn9 = assign62310_e80847_d_n9;

        let (assign62320_e80851, assign62320_e80851_d_n4, assign62320_e80851_d_n6, assign62320_e80851_d_n7, assign62320_e80851_d_n8, assign62320_e80851_d_n9,) = {
    if (locals.var_guard1749 != 0.0) {
        (locals.var_temp__blk1748, locals.var_temp__blk1748_dn4, locals.var_temp__blk1748_dn6, locals.var_temp__blk1748_dn7, locals.var_temp__blk1748_dn8, locals.var_temp__blk1748_dn9,)
    } else {
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9,)
    }
};
        locals.var_qs = assign62320_e80851;
        locals.var_qs_dn4 = assign62320_e80851_d_n4;
        locals.var_qs_dn6 = assign62320_e80851_d_n6;
        locals.var_qs_dn7 = assign62320_e80851_d_n7;
        locals.var_qs_dn8 = assign62320_e80851_d_n8;
        locals.var_qs_dn9 = assign62320_e80851_d_n9;

        locals.var_sidexc = 0.0;
        locals.var_sidexc_dn4 = 0.0;
        locals.var_sidexc_dn6 = 0.0;
        locals.var_sidexc_dn7 = 0.0;
        locals.var_sidexc_dn8 = 0.0;
        locals.var_sidexc_dn9 = 0.0;

        locals.var_mid = 0.0;
        locals.var_mid_dn4 = 0.0;
        locals.var_mid_dn6 = 0.0;
        locals.var_mid_dn7 = 0.0;
        locals.var_mid_dn8 = 0.0;
        locals.var_mid_dn9 = 0.0;

        locals.var_mig = 1e-40;
        locals.var_mig_dn4 = 0.0;
        locals.var_mig_dn6 = 0.0;
        locals.var_mig_dn7 = 0.0;
        locals.var_mig_dn8 = 0.0;
        locals.var_mig_dn9 = 0.0;

        locals.var_migid = 0.0;
        locals.var_migid_dn4 = 0.0;
        locals.var_migid_dn6 = 0.0;
        locals.var_migid_dn7 = 0.0;
        locals.var_migid_dn8 = 0.0;
        locals.var_migid_dn9 = 0.0;

        locals.var_c_igid = 0.0;
        locals.var_c_igid_dn4 = 0.0;
        locals.var_c_igid_dn6 = 0.0;
        locals.var_c_igid_dn7 = 0.0;
        locals.var_c_igid_dn8 = 0.0;
        locals.var_c_igid_dn9 = 0.0;

        let assign62390_e80860: f64 = (locals.var_cox_qm * locals.var_eta_p_ac);
        locals.var_cgeff = assign62390_e80860;
        locals.var_cgeff_dn4 = ((locals.var_cox_qm_dn4 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn4));
        locals.var_cgeff_dn6 = ((locals.var_cox_qm_dn6 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn6));
        locals.var_cgeff_dn7 = ((locals.var_cox_qm_dn7 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn7));
        locals.var_cgeff_dn8 = ((locals.var_cox_qm_dn8 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn8));
        locals.var_cgeff_dn9 = ((locals.var_cox_qm_dn9 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn9));

        locals.var_sqid = 0.0;
        locals.var_sqid_dn4 = 0.0;
        locals.var_sqid_dn6 = 0.0;
        locals.var_sqid_dn7 = 0.0;
        locals.var_sqid_dn8 = 0.0;
        locals.var_sqid_dn9 = 0.0;

        locals.var_sqig = 0.0;
        locals.var_sqig_dn4 = 0.0;
        locals.var_sqig_dn6 = 0.0;
        locals.var_sqig_dn7 = 0.0;
        locals.var_sqig_dn8 = 0.0;
        locals.var_sqig_dn9 = 0.0;

        let assign62450_e80872: f64 = if ((locals.var_xg_dc > 0.0) && (locals.var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1782 = assign62450_e80872;

        let assign62540_e80978: f64 = if p.p32 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1784 = assign62540_e80978;

        let (assign62550_e80986, assign62550_e80986_d_n4, assign62550_e80986_d_n6, assign62550_e80986_d_n7, assign62550_e80986_d_n8, assign62550_e80986_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62550_e80984: f64 = (locals.var_qim1_dc / locals.var_alpha_dc);
        (assign62550_e80984, (((locals.var_qim1_dc_dn4 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn4)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn6 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn6)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn7 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn7)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn8 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn8)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn9 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn9)) / (locals.var_alpha_dc * locals.var_alpha_dc)),)
    } else {
        (locals.var_h0, locals.var_h0_dn4, locals.var_h0_dn6, locals.var_h0_dn7, locals.var_h0_dn8, locals.var_h0_dn9,)
    }
};
        locals.var_h0 = assign62550_e80986;
        locals.var_h0_dn4 = assign62550_e80986_d_n4;
        locals.var_h0_dn6 = assign62550_e80986_d_n6;
        locals.var_h0_dn7 = assign62550_e80986_d_n7;
        locals.var_h0_dn8 = assign62550_e80986_d_n8;
        locals.var_h0_dn9 = assign62550_e80986_d_n9;

        let (assign62560_e80994, assign62560_e80994_d_n4, assign62560_e80994_d_n6, assign62560_e80994_d_n7, assign62560_e80994_d_n8, assign62560_e80994_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62560_e80992: f64 = (locals.var_qim_dc / locals.var_qim1_dc);
        (assign62560_e80992, (((locals.var_qim_dc_dn4 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn4)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn6 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn7 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn8 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn9 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn9)) / (locals.var_qim1_dc * locals.var_qim1_dc)),)
    } else {
        (locals.var_t1, locals.var_t1_dn4, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9,)
    }
};
        locals.var_t1 = assign62560_e80994;
        locals.var_t1_dn4 = assign62560_e80994_d_n4;
        locals.var_t1_dn6 = assign62560_e80994_d_n6;
        locals.var_t1_dn7 = assign62560_e80994_d_n7;
        locals.var_t1_dn8 = assign62560_e80994_d_n8;
        locals.var_t1_dn9 = assign62560_e80994_d_n9;

        let (assign62570_e81006, assign62570_e81006_d_n4, assign62570_e81006_d_n6, assign62570_e81006_d_n7, assign62570_e81006_d_n8, assign62570_e81006_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62570_e81000: f64 = (0.5 * 0.16666666666666666);
        let assign62570_e81003: f64 = (locals.var_dps_dc / locals.var_h0);
        let assign62570_e81004: f64 = (assign62570_e81000 * assign62570_e81003);
        (assign62570_e81004, (assign62570_e81000 * (((locals.var_dps_dc_dn4 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn4)) / (locals.var_h0 * locals.var_h0))), (assign62570_e81000 * (((locals.var_dps_dc_dn6 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn6)) / (locals.var_h0 * locals.var_h0))), (assign62570_e81000 * (((locals.var_dps_dc_dn7 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn7)) / (locals.var_h0 * locals.var_h0))), (assign62570_e81000 * (((locals.var_dps_dc_dn8 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn8)) / (locals.var_h0 * locals.var_h0))), (assign62570_e81000 * (((locals.var_dps_dc_dn9 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn9)) / (locals.var_h0 * locals.var_h0))),)
    } else {
        (locals.var_sqt2, locals.var_sqt2_dn4, locals.var_sqt2_dn6, locals.var_sqt2_dn7, locals.var_sqt2_dn8, locals.var_sqt2_dn9,)
    }
};
        locals.var_sqt2 = assign62570_e81006;
        locals.var_sqt2_dn4 = assign62570_e81006_d_n4;
        locals.var_sqt2_dn6 = assign62570_e81006_d_n6;
        locals.var_sqt2_dn7 = assign62570_e81006_d_n7;
        locals.var_sqt2_dn8 = assign62570_e81006_d_n8;
        locals.var_sqt2_dn9 = assign62570_e81006_d_n9;

        let (assign62580_e81014, assign62580_e81014_d_n4, assign62580_e81014_d_n6, assign62580_e81014_d_n7, assign62580_e81014_d_n8, assign62580_e81014_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62580_e81012: f64 = (locals.var_sqt2 * locals.var_sqt2);
        (assign62580_e81012, ((locals.var_sqt2_dn4 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn4)), ((locals.var_sqt2_dn6 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn6)), ((locals.var_sqt2_dn7 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn7)), ((locals.var_sqt2_dn8 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn8)), ((locals.var_sqt2_dn9 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn9)),)
    } else {
        (locals.var_t2, locals.var_t2_dn4, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9,)
    }
};
        locals.var_t2 = assign62580_e81014;
        locals.var_t2_dn4 = assign62580_e81014_d_n4;
        locals.var_t2_dn6 = assign62580_e81014_d_n6;
        locals.var_t2_dn7 = assign62580_e81014_d_n7;
        locals.var_t2_dn8 = assign62580_e81014_d_n8;
        locals.var_t2_dn9 = assign62580_e81014_d_n9;

        let (assign62590_e81024, assign62590_e81024_d_n4, assign62590_e81024_d_n6, assign62590_e81024_d_n7, assign62590_e81024_d_n8, assign62590_e81024_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62590_e81020: f64 = (locals.var_h0 / locals.var_h_dc);
        let assign62590_e81022: f64 = (assign62590_e81020 - 1.0);
        (assign62590_e81022, (((locals.var_h0_dn4 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn4)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn6 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn6)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn7 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn7)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn8 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn8)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn9 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn9)) / (locals.var_h_dc * locals.var_h_dc)),)
    } else {
        (locals.var_r, locals.var_r_dn4, locals.var_r_dn6, locals.var_r_dn7, locals.var_r_dn8, locals.var_r_dn9,)
    }
};
        locals.var_r = assign62590_e81024;
        locals.var_r_dn4 = assign62590_e81024_d_n4;
        locals.var_r_dn6 = assign62590_e81024_d_n6;
        locals.var_r_dn7 = assign62590_e81024_d_n7;
        locals.var_r_dn8 = assign62590_e81024_d_n8;
        locals.var_r_dn9 = assign62590_e81024_d_n9;

        let (assign62600_e81047, assign62600_e81047_d_n4, assign62600_e81047_d_n6, assign62600_e81047_d_n7, assign62600_e81047_d_n8, assign62600_e81047_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62600_e81032: f64 = (locals.var_r * locals.var_t2);
        let assign62600_e81033: f64 = (12.0 * assign62600_e81032);
        let assign62600_e81034: f64 = (1.0 - assign62600_e81033);
        let (assign62600_e81045, assign62600_e81045_d_n4, assign62600_e81045_d_n6, assign62600_e81045_d_n7, assign62600_e81045_d_n8, assign62600_e81045_d_n9,) = {
            if (assign62600_e81034 > 1e-20) {
                let assign62600_e81041: f64 = (locals.var_r * locals.var_t2);
                let assign62600_e81042: f64 = (12.0 * assign62600_e81041);
                let assign62600_e81043: f64 = (1.0 - assign62600_e81042);
                (assign62600_e81043, (-(12.0 * ((locals.var_r_dn4 * locals.var_t2) + (locals.var_r * locals.var_t2_dn4)))), (-(12.0 * ((locals.var_r_dn6 * locals.var_t2) + (locals.var_r * locals.var_t2_dn6)))), (-(12.0 * ((locals.var_r_dn7 * locals.var_t2) + (locals.var_r * locals.var_t2_dn7)))), (-(12.0 * ((locals.var_r_dn8 * locals.var_t2) + (locals.var_r * locals.var_t2_dn8)))), (-(12.0 * ((locals.var_r_dn9 * locals.var_t2) + (locals.var_r * locals.var_t2_dn9)))),)
            } else {
                (1e-20, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62600_e81045, assign62600_e81045_d_n4, assign62600_e81045_d_n6, assign62600_e81045_d_n7, assign62600_e81045_d_n8, assign62600_e81045_d_n9,)
    } else {
        (locals.var_lc, locals.var_lc_dn4, locals.var_lc_dn6, locals.var_lc_dn7, locals.var_lc_dn8, locals.var_lc_dn9,)
    }
};
        locals.var_lc = assign62600_e81047;
        locals.var_lc_dn4 = assign62600_e81047_d_n4;
        locals.var_lc_dn6 = assign62600_e81047_d_n6;
        locals.var_lc_dn7 = assign62600_e81047_d_n7;
        locals.var_lc_dn8 = assign62600_e81047_d_n8;
        locals.var_lc_dn9 = assign62600_e81047_d_n9;

        let (assign62610_e81057, assign62610_e81057_d_n4, assign62610_e81057_d_n6, assign62610_e81057_d_n7, assign62610_e81057_d_n8, assign62610_e81057_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62610_e81054: f64 = (locals.var_lc * locals.var_lc);
        let assign62610_e81055: f64 = (1.0 / assign62610_e81054);
        (assign62610_e81055, (-(((locals.var_lc_dn4 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn4)) / (assign62610_e81054 * assign62610_e81054))), (-(((locals.var_lc_dn6 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn6)) / (assign62610_e81054 * assign62610_e81054))), (-(((locals.var_lc_dn7 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn7)) / (assign62610_e81054 * assign62610_e81054))), (-(((locals.var_lc_dn8 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn8)) / (assign62610_e81054 * assign62610_e81054))), (-(((locals.var_lc_dn9 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn9)) / (assign62610_e81054 * assign62610_e81054))),)
    } else {
        (locals.var_lcinv2, locals.var_lcinv2_dn4, locals.var_lcinv2_dn6, locals.var_lcinv2_dn7, locals.var_lcinv2_dn8, locals.var_lcinv2_dn9,)
    }
};
        locals.var_lcinv2 = assign62610_e81057;
        locals.var_lcinv2_dn4 = assign62610_e81057_d_n4;
        locals.var_lcinv2_dn6 = assign62610_e81057_d_n6;
        locals.var_lcinv2_dn7 = assign62610_e81057_d_n7;
        locals.var_lcinv2_dn8 = assign62610_e81057_d_n8;
        locals.var_lcinv2_dn9 = assign62610_e81057_d_n9;

        let (assign62620_e81067, assign62620_e81067_d_n4, assign62620_e81067_d_n6, assign62620_e81067_d_n7, assign62620_e81067_d_n8, assign62620_e81067_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62620_e81063: f64 = (locals.var_bet_i * locals.var_qim1_dc);
        let assign62620_e81065: f64 = (assign62620_e81063 * locals.var_gvsatinv_dc);
        (assign62620_e81065, ((((locals.var_bet_i_dn4 * locals.var_qim1_dc) + (locals.var_bet_i * locals.var_qim1_dc_dn4)) * locals.var_gvsatinv_dc) + (assign62620_e81063 * locals.var_gvsatinv_dc_dn4)), (((locals.var_bet_i * locals.var_qim1_dc_dn6) * locals.var_gvsatinv_dc) + (assign62620_e81063 * locals.var_gvsatinv_dc_dn6)), (((locals.var_bet_i * locals.var_qim1_dc_dn7) * locals.var_gvsatinv_dc) + (assign62620_e81063 * locals.var_gvsatinv_dc_dn7)), (((locals.var_bet_i * locals.var_qim1_dc_dn8) * locals.var_gvsatinv_dc) + (assign62620_e81063 * locals.var_gvsatinv_dc_dn8)), (((locals.var_bet_i * locals.var_qim1_dc_dn9) * locals.var_gvsatinv_dc) + (assign62620_e81063 * locals.var_gvsatinv_dc_dn9)),)
    } else {
        (locals.var_g_ideal, locals.var_g_ideal_dn4, locals.var_g_ideal_dn6, locals.var_g_ideal_dn7, locals.var_g_ideal_dn8, locals.var_g_ideal_dn9,)
    }
};
        locals.var_g_ideal = assign62620_e81067;
        locals.var_g_ideal_dn4 = assign62620_e81067_d_n4;
        locals.var_g_ideal_dn6 = assign62620_e81067_d_n6;
        locals.var_g_ideal_dn7 = assign62620_e81067_d_n7;
        locals.var_g_ideal_dn8 = assign62620_e81067_d_n8;
        locals.var_g_ideal_dn9 = assign62620_e81067_d_n9;

        let (assign62630_e81087, assign62630_e81087_d_n4, assign62630_e81087_d_n6, assign62630_e81087_d_n7, assign62630_e81087_d_n8, assign62630_e81087_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62630_e81074: f64 = (12.0 * locals.var_t2);
        let assign62630_e81075: f64 = (locals.var_t1 + assign62630_e81074);
        let assign62630_e81079: f64 = (1.0 + locals.var_t1);
        let assign62630_e81081: f64 = (assign62630_e81079 * locals.var_t2);
        let assign62630_e81083: f64 = (assign62630_e81081 * locals.var_r);
        let assign62630_e81084: f64 = (24.0 * assign62630_e81083);
        let assign62630_e81085: f64 = (assign62630_e81075 - assign62630_e81084);
        (assign62630_e81085, ((locals.var_t1_dn4 + (12.0 * locals.var_t2_dn4)) - (24.0 * ((((locals.var_t1_dn4 * locals.var_t2) + (assign62630_e81079 * locals.var_t2_dn4)) * locals.var_r) + (assign62630_e81081 * locals.var_r_dn4)))), ((locals.var_t1_dn6 + (12.0 * locals.var_t2_dn6)) - (24.0 * ((((locals.var_t1_dn6 * locals.var_t2) + (assign62630_e81079 * locals.var_t2_dn6)) * locals.var_r) + (assign62630_e81081 * locals.var_r_dn6)))), ((locals.var_t1_dn7 + (12.0 * locals.var_t2_dn7)) - (24.0 * ((((locals.var_t1_dn7 * locals.var_t2) + (assign62630_e81079 * locals.var_t2_dn7)) * locals.var_r) + (assign62630_e81081 * locals.var_r_dn7)))), ((locals.var_t1_dn8 + (12.0 * locals.var_t2_dn8)) - (24.0 * ((((locals.var_t1_dn8 * locals.var_t2) + (assign62630_e81079 * locals.var_t2_dn8)) * locals.var_r) + (assign62630_e81081 * locals.var_r_dn8)))), ((locals.var_t1_dn9 + (12.0 * locals.var_t2_dn9)) - (24.0 * ((((locals.var_t1_dn9 * locals.var_t2) + (assign62630_e81079 * locals.var_t2_dn9)) * locals.var_r) + (assign62630_e81081 * locals.var_r_dn9)))),)
    } else {
        (locals.var_mid, locals.var_mid_dn4, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn9,)
    }
};
        locals.var_mid = assign62630_e81087;
        locals.var_mid_dn4 = assign62630_e81087_d_n4;
        locals.var_mid_dn6 = assign62630_e81087_d_n6;
        locals.var_mid_dn7 = assign62630_e81087_d_n7;
        locals.var_mid_dn8 = assign62630_e81087_d_n8;
        locals.var_mid_dn9 = assign62630_e81087_d_n9;

        let (assign62640_e81098, assign62640_e81098_d_n4, assign62640_e81098_d_n6, assign62640_e81098_d_n7, assign62640_e81098_d_n8, assign62640_e81098_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let (assign62640_e81096, assign62640_e81096_d_n4, assign62640_e81096_d_n6, assign62640_e81096_d_n7, assign62640_e81096_d_n8, assign62640_e81096_d_n9,) = {
            if (locals.var_mid > 1e-40) {
                (locals.var_mid, locals.var_mid_dn4, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn9,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62640_e81096, assign62640_e81096_d_n4, assign62640_e81096_d_n6, assign62640_e81096_d_n7, assign62640_e81096_d_n8, assign62640_e81096_d_n9,)
    } else {
        (locals.var_mid, locals.var_mid_dn4, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn9,)
    }
};
        locals.var_mid = assign62640_e81098;
        locals.var_mid_dn4 = assign62640_e81098_d_n4;
        locals.var_mid_dn6 = assign62640_e81098_d_n6;
        locals.var_mid_dn7 = assign62640_e81098_d_n7;
        locals.var_mid_dn8 = assign62640_e81098_d_n8;
        locals.var_mid_dn9 = assign62640_e81098_d_n9;

        let (assign62650_e81108, assign62650_e81108_d_n4, assign62650_e81108_d_n6, assign62650_e81108_d_n7, assign62650_e81108_d_n8, assign62650_e81108_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62650_e81104: f64 = (locals.var_g_ideal * locals.var_lcinv2);
        let assign62650_e81106: f64 = (assign62650_e81104 * locals.var_mid);
        (assign62650_e81106, ((((locals.var_g_ideal_dn4 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn4)) * locals.var_mid) + (assign62650_e81104 * locals.var_mid_dn4)), ((((locals.var_g_ideal_dn6 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn6)) * locals.var_mid) + (assign62650_e81104 * locals.var_mid_dn6)), ((((locals.var_g_ideal_dn7 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn7)) * locals.var_mid) + (assign62650_e81104 * locals.var_mid_dn7)), ((((locals.var_g_ideal_dn8 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn8)) * locals.var_mid) + (assign62650_e81104 * locals.var_mid_dn8)), ((((locals.var_g_ideal_dn9 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn9)) * locals.var_mid) + (assign62650_e81104 * locals.var_mid_dn9)),)
    } else {
        (locals.var_mid, locals.var_mid_dn4, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn9,)
    }
};
        locals.var_mid = assign62650_e81108;
        locals.var_mid_dn4 = assign62650_e81108_d_n4;
        locals.var_mid_dn6 = assign62650_e81108_d_n6;
        locals.var_mid_dn7 = assign62650_e81108_d_n7;
        locals.var_mid_dn8 = assign62650_e81108_d_n8;
        locals.var_mid_dn9 = assign62650_e81108_d_n9;

        let assign62660_e81111: f64 = if locals.var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1785 = assign62660_e81111;

        let (assign62670_e81121, assign62670_e81121_d_n4, assign62670_e81121_d_n6, assign62670_e81121_d_n7, assign62670_e81121_d_n8, assign62670_e81121_d_n9,) = {
    if (((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        let assign62670_e81119: f64 = (locals.var_thesateff_dc / locals.var_gmob_dc);
        (assign62670_e81119, (((locals.var_thesateff_dc_dn4 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn4)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn6 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn7 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn8 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn9 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn9)) / (locals.var_gmob_dc * locals.var_gmob_dc)),)
    } else {
        (locals.var_thesat1_exc, locals.var_thesat1_exc_dn4, locals.var_thesat1_exc_dn6, locals.var_thesat1_exc_dn7, locals.var_thesat1_exc_dn8, locals.var_thesat1_exc_dn9,)
    }
};
        locals.var_thesat1_exc = assign62670_e81121;
        locals.var_thesat1_exc_dn4 = assign62670_e81121_d_n4;
        locals.var_thesat1_exc_dn6 = assign62670_e81121_d_n6;
        locals.var_thesat1_exc_dn7 = assign62670_e81121_d_n7;
        locals.var_thesat1_exc_dn8 = assign62670_e81121_d_n8;
        locals.var_thesat1_exc_dn9 = assign62670_e81121_d_n9;

        let (assign62680_e81135, assign62680_e81135_d_n4, assign62680_e81135_d_n6, assign62680_e81135_d_n7, assign62680_e81135_d_n8, assign62680_e81135_d_n9,) = {
    if (((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        let assign62680_e81129: f64 = (locals.var_thesat1_exc * locals.var_thesat1_exc);
        let assign62680_e81131: f64 = (assign62680_e81129 * locals.var_dps_dc);
        let assign62680_e81133: f64 = (assign62680_e81131 * locals.var_dps_dc);
        (assign62680_e81133, ((((((locals.var_thesat1_exc_dn4 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn4)) * locals.var_dps_dc) + (assign62680_e81129 * locals.var_dps_dc_dn4)) * locals.var_dps_dc) + (assign62680_e81131 * locals.var_dps_dc_dn4)), ((((((locals.var_thesat1_exc_dn6 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn6)) * locals.var_dps_dc) + (assign62680_e81129 * locals.var_dps_dc_dn6)) * locals.var_dps_dc) + (assign62680_e81131 * locals.var_dps_dc_dn6)), ((((((locals.var_thesat1_exc_dn7 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn7)) * locals.var_dps_dc) + (assign62680_e81129 * locals.var_dps_dc_dn7)) * locals.var_dps_dc) + (assign62680_e81131 * locals.var_dps_dc_dn7)), ((((((locals.var_thesat1_exc_dn8 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn8)) * locals.var_dps_dc) + (assign62680_e81129 * locals.var_dps_dc_dn8)) * locals.var_dps_dc) + (assign62680_e81131 * locals.var_dps_dc_dn8)), ((((((locals.var_thesat1_exc_dn9 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn9)) * locals.var_dps_dc) + (assign62680_e81129 * locals.var_dps_dc_dn9)) * locals.var_dps_dc) + (assign62680_e81131 * locals.var_dps_dc_dn9)),)
    } else {
        (locals.var_zsat_exc, locals.var_zsat_exc_dn4, locals.var_zsat_exc_dn6, locals.var_zsat_exc_dn7, locals.var_zsat_exc_dn8, locals.var_zsat_exc_dn9,)
    }
};
        locals.var_zsat_exc = assign62680_e81135;
        locals.var_zsat_exc_dn4 = assign62680_e81135_d_n4;
        locals.var_zsat_exc_dn6 = assign62680_e81135_d_n6;
        locals.var_zsat_exc_dn7 = assign62680_e81135_d_n7;
        locals.var_zsat_exc_dn8 = assign62680_e81135_d_n8;
        locals.var_zsat_exc_dn9 = assign62680_e81135_d_n9;

        let assign62690_e81138: f64 = (-1.0);
        let assign62690_e81139: f64 = if locals.var_chnl_type == assign62690_e81138 { 1.0 } else { 0.0 };
        locals.var_guard1786 = assign62690_e81139;

        let (assign62700_e81155, assign62700_e81155_d_n4, assign62700_e81155_d_n6, assign62700_e81155_d_n7, assign62700_e81155_d_n8, assign62700_e81155_d_n9,) = {
    if ((((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_guard1786 != 0.0)) {
        let assign62700_e81151: f64 = (locals.var_thesat1_exc * locals.var_dps_dc);
        let assign62700_e81152: f64 = (1.0 + assign62700_e81151);
        let assign62700_e81153: f64 = (locals.var_zsat_exc / assign62700_e81152);
        (assign62700_e81153, (((locals.var_zsat_exc_dn4 * assign62700_e81152) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn4 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn4)))) / (assign62700_e81152 * assign62700_e81152)), (((locals.var_zsat_exc_dn6 * assign62700_e81152) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn6 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn6)))) / (assign62700_e81152 * assign62700_e81152)), (((locals.var_zsat_exc_dn7 * assign62700_e81152) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn7 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn7)))) / (assign62700_e81152 * assign62700_e81152)), (((locals.var_zsat_exc_dn8 * assign62700_e81152) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn8 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn8)))) / (assign62700_e81152 * assign62700_e81152)), (((locals.var_zsat_exc_dn9 * assign62700_e81152) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn9 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn9)))) / (assign62700_e81152 * assign62700_e81152)),)
    } else {
        (locals.var_zsat_exc, locals.var_zsat_exc_dn4, locals.var_zsat_exc_dn6, locals.var_zsat_exc_dn7, locals.var_zsat_exc_dn8, locals.var_zsat_exc_dn9,)
    }
};
        locals.var_zsat_exc = assign62700_e81155;
        locals.var_zsat_exc_dn4 = assign62700_e81155_d_n4;
        locals.var_zsat_exc_dn6 = assign62700_e81155_d_n6;
        locals.var_zsat_exc_dn7 = assign62700_e81155_d_n7;
        locals.var_zsat_exc_dn8 = assign62700_e81155_d_n8;
        locals.var_zsat_exc_dn9 = assign62700_e81155_d_n9;

        let (assign62710_e81174, assign62710_e81174_d_n4, assign62710_e81174_d_n6, assign62710_e81174_d_n7, assign62710_e81174_d_n8, assign62710_e81174_d_n9,) = {
    if (((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        let assign62710_e81167: f64 = (2.0 * locals.var_zsat_exc);
        let assign62710_e81168: f64 = (1.0 + assign62710_e81167);
        let assign62710_e81169: f64 = (assign62710_e81168).sqrt();
        let assign62710_e81170: f64 = (1.0 + assign62710_e81169);
        let assign62710_e81171: f64 = (locals.var_gmob_dc * assign62710_e81170);
        let assign62710_e81172: f64 = (0.5 * assign62710_e81171);
        (assign62710_e81172, (0.5 * ((locals.var_gmob_dc_dn4 * assign62710_e81170) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn4) / (2.0 * assign62710_e81169))))), (0.5 * ((locals.var_gmob_dc_dn6 * assign62710_e81170) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn6) / (2.0 * assign62710_e81169))))), (0.5 * ((locals.var_gmob_dc_dn7 * assign62710_e81170) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn7) / (2.0 * assign62710_e81169))))), (0.5 * ((locals.var_gmob_dc_dn8 * assign62710_e81170) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn8) / (2.0 * assign62710_e81169))))), (0.5 * ((locals.var_gmob_dc_dn9 * assign62710_e81170) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn9) / (2.0 * assign62710_e81169))))),)
    } else {
        (locals.var_gvsat_exc, locals.var_gvsat_exc_dn4, locals.var_gvsat_exc_dn6, locals.var_gvsat_exc_dn7, locals.var_gvsat_exc_dn8, locals.var_gvsat_exc_dn9,)
    }
};
        locals.var_gvsat_exc = assign62710_e81174;
        locals.var_gvsat_exc_dn4 = assign62710_e81174_d_n4;
        locals.var_gvsat_exc_dn6 = assign62710_e81174_d_n6;
        locals.var_gvsat_exc_dn7 = assign62710_e81174_d_n7;
        locals.var_gvsat_exc_dn8 = assign62710_e81174_d_n8;
        locals.var_gvsat_exc_dn9 = assign62710_e81174_d_n9;

        let (assign62720_e81186, assign62720_e81186_d_n4, assign62720_e81186_d_n6, assign62720_e81186_d_n7, assign62720_e81186_d_n8, assign62720_e81186_d_n9,) = {
    if (((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        let assign62720_e81183: f64 = (locals.var_gvsat_exc * locals.var_lc);
        let assign62720_e81184: f64 = (locals.var_gmob_dc / assign62720_e81183);
        (assign62720_e81184, (((locals.var_gmob_dc_dn4 * assign62720_e81183) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn4 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn4)))) / (assign62720_e81183 * assign62720_e81183)), (((locals.var_gmob_dc_dn6 * assign62720_e81183) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn6 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn6)))) / (assign62720_e81183 * assign62720_e81183)), (((locals.var_gmob_dc_dn7 * assign62720_e81183) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn7 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn7)))) / (assign62720_e81183 * assign62720_e81183)), (((locals.var_gmob_dc_dn8 * assign62720_e81183) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn8 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn8)))) / (assign62720_e81183 * assign62720_e81183)), (((locals.var_gmob_dc_dn9 * assign62720_e81183) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn9 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn9)))) / (assign62720_e81183 * assign62720_e81183)),)
    } else {
        (locals.var_gfac, locals.var_gfac_dn4, locals.var_gfac_dn6, locals.var_gfac_dn7, locals.var_gfac_dn8, locals.var_gfac_dn9,)
    }
};
        locals.var_gfac = assign62720_e81186;
        locals.var_gfac_dn4 = assign62720_e81186_d_n4;
        locals.var_gfac_dn6 = assign62720_e81186_d_n6;
        locals.var_gfac_dn7 = assign62720_e81186_d_n7;
        locals.var_gfac_dn8 = assign62720_e81186_d_n8;
        locals.var_gfac_dn9 = assign62720_e81186_d_n9;

        let (assign62730_e81202, assign62730_e81202_d_n4, assign62730_e81202_d_n6, assign62730_e81202_d_n7, assign62730_e81202_d_n8, assign62730_e81202_d_n9,) = {
    if (((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        let assign62730_e81194: f64 = (locals.var_fac_exc * locals.var_i_ds);
        let assign62730_e81196: f64 = (assign62730_e81194 * locals.var_vdse_dc);
        let assign62730_e81198: f64 = (assign62730_e81196 * locals.var_gfac);
        let assign62730_e81200: f64 = (assign62730_e81198 * locals.var_gfac);
        (assign62730_e81200, (((((((locals.var_fac_exc * locals.var_i_ds_dn4) * locals.var_vdse_dc) + (assign62730_e81194 * locals.var_vdse_dc_dn4)) * locals.var_gfac) + (assign62730_e81196 * locals.var_gfac_dn4)) * locals.var_gfac) + (assign62730_e81198 * locals.var_gfac_dn4)), (((((((locals.var_fac_exc * locals.var_i_ds_dn6) * locals.var_vdse_dc) + (assign62730_e81194 * locals.var_vdse_dc_dn6)) * locals.var_gfac) + (assign62730_e81196 * locals.var_gfac_dn6)) * locals.var_gfac) + (assign62730_e81198 * locals.var_gfac_dn6)), (((((((locals.var_fac_exc * locals.var_i_ds_dn7) * locals.var_vdse_dc) + (assign62730_e81194 * locals.var_vdse_dc_dn7)) * locals.var_gfac) + (assign62730_e81196 * locals.var_gfac_dn7)) * locals.var_gfac) + (assign62730_e81198 * locals.var_gfac_dn7)), (((((((locals.var_fac_exc * locals.var_i_ds_dn8) * locals.var_vdse_dc) + (assign62730_e81194 * locals.var_vdse_dc_dn8)) * locals.var_gfac) + (assign62730_e81196 * locals.var_gfac_dn8)) * locals.var_gfac) + (assign62730_e81198 * locals.var_gfac_dn8)), (((((((locals.var_fac_exc * locals.var_i_ds_dn9) * locals.var_vdse_dc) + (assign62730_e81194 * locals.var_vdse_dc_dn9)) * locals.var_gfac) + (assign62730_e81196 * locals.var_gfac_dn9)) * locals.var_gfac) + (assign62730_e81198 * locals.var_gfac_dn9)),)
    } else {
        (locals.var_sidexc, locals.var_sidexc_dn4, locals.var_sidexc_dn6, locals.var_sidexc_dn7, locals.var_sidexc_dn8, locals.var_sidexc_dn9,)
    }
};
        locals.var_sidexc = assign62730_e81202;
        locals.var_sidexc_dn4 = assign62730_e81202_d_n4;
        locals.var_sidexc_dn6 = assign62730_e81202_d_n6;
        locals.var_sidexc_dn7 = assign62730_e81202_d_n7;
        locals.var_sidexc_dn8 = assign62730_e81202_d_n8;
        locals.var_sidexc_dn9 = assign62730_e81202_d_n9;

        let (assign62740_e81214, assign62740_e81214_d_n4, assign62740_e81214_d_n6, assign62740_e81214_d_n7, assign62740_e81214_d_n8, assign62740_e81214_d_n9,) = {
    if (((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        let assign62740_e81211: f64 = (locals.var_sidexc / locals.var_nt0);
        let assign62740_e81212: f64 = (locals.var_mid + assign62740_e81211);
        (assign62740_e81212, (locals.var_mid_dn4 + (((locals.var_sidexc_dn4 * locals.var_nt0) - (locals.var_sidexc * locals.var_nt0_dn4)) / (locals.var_nt0 * locals.var_nt0))), (locals.var_mid_dn6 + (locals.var_sidexc_dn6 / locals.var_nt0)), (locals.var_mid_dn7 + (locals.var_sidexc_dn7 / locals.var_nt0)), (locals.var_mid_dn8 + (locals.var_sidexc_dn8 / locals.var_nt0)), (locals.var_mid_dn9 + (locals.var_sidexc_dn9 / locals.var_nt0)),)
    } else {
        (locals.var_mid, locals.var_mid_dn4, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn9,)
    }
};
        locals.var_mid = assign62740_e81214;
        locals.var_mid_dn4 = assign62740_e81214_d_n4;
        locals.var_mid_dn6 = assign62740_e81214_d_n6;
        locals.var_mid_dn7 = assign62740_e81214_d_n7;
        locals.var_mid_dn8 = assign62740_e81214_d_n8;
        locals.var_mid_dn9 = assign62740_e81214_d_n9;

        let (assign62750_e81223, assign62750_e81223_d_n4, assign62750_e81223_d_n6, assign62750_e81223_d_n7, assign62750_e81223_d_n8, assign62750_e81223_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1784 != 0.0)) {
        let assign62750_e81220: f64 = (locals.var_nt * locals.var_mid);
        let assign62750_e81221: f64 = (assign62750_e81220).sqrt();
        (assign62750_e81221, (((locals.var_nt_dn4 * locals.var_mid) + (locals.var_nt * locals.var_mid_dn4)) / (2.0 * assign62750_e81221)), ((locals.var_nt * locals.var_mid_dn6) / (2.0 * assign62750_e81221)), ((locals.var_nt * locals.var_mid_dn7) / (2.0 * assign62750_e81221)), ((locals.var_nt * locals.var_mid_dn8) / (2.0 * assign62750_e81221)), ((locals.var_nt * locals.var_mid_dn9) / (2.0 * assign62750_e81221)),)
    } else {
        (locals.var_sqid, locals.var_sqid_dn4, locals.var_sqid_dn6, locals.var_sqid_dn7, locals.var_sqid_dn8, locals.var_sqid_dn9,)
    }
};
        locals.var_sqid = assign62750_e81223;
        locals.var_sqid_dn4 = assign62750_e81223_d_n4;
        locals.var_sqid_dn6 = assign62750_e81223_d_n6;
        locals.var_sqid_dn7 = assign62750_e81223_d_n7;
        locals.var_sqid_dn8 = assign62750_e81223_d_n8;
        locals.var_sqid_dn9 = assign62750_e81223_d_n9;

    }

    pub(super) fn stamp_transient_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign62760_e81238: f64 = if ((((p.p50 == 1.0) && (locals.var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1787 = assign62760_e81238;

        let (assign62770_e81270, assign62770_e81270_d_n4, assign62770_e81270_d_n6, assign62770_e81270_d_n7, assign62770_e81270_d_n8, assign62770_e81270_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) {
        let assign62770_e81244: f64 = (locals.var_t1 / 12.0);
        let assign62770_e81248: f64 = (locals.var_t1 + 0.2);
        let assign62770_e81251: f64 = (12.0 * locals.var_t2);
        let assign62770_e81252: f64 = (assign62770_e81248 - assign62770_e81251);
        let assign62770_e81253: f64 = (locals.var_t2 * assign62770_e81252);
        let assign62770_e81254: f64 = (assign62770_e81244 - assign62770_e81253);
        let assign62770_e81259: f64 = (locals.var_t1 + 1.0);
        let assign62770_e81262: f64 = (12.0 * locals.var_t2);
        let assign62770_e81263: f64 = (assign62770_e81259 - assign62770_e81262);
        let assign62770_e81264: f64 = (locals.var_t2 * assign62770_e81263);
        let assign62770_e81266: f64 = (assign62770_e81264 * locals.var_r);
        let assign62770_e81267: f64 = (1.6 * assign62770_e81266);
        let assign62770_e81268: f64 = (assign62770_e81254 - assign62770_e81267);
        (assign62770_e81268, (((locals.var_t1_dn4 / 12.0) - ((locals.var_t2_dn4 * assign62770_e81252) + (locals.var_t2 * (locals.var_t1_dn4 - (12.0 * locals.var_t2_dn4))))) - (1.6 * ((((locals.var_t2_dn4 * assign62770_e81263) + (locals.var_t2 * (locals.var_t1_dn4 - (12.0 * locals.var_t2_dn4)))) * locals.var_r) + (assign62770_e81264 * locals.var_r_dn4)))), (((locals.var_t1_dn6 / 12.0) - ((locals.var_t2_dn6 * assign62770_e81252) + (locals.var_t2 * (locals.var_t1_dn6 - (12.0 * locals.var_t2_dn6))))) - (1.6 * ((((locals.var_t2_dn6 * assign62770_e81263) + (locals.var_t2 * (locals.var_t1_dn6 - (12.0 * locals.var_t2_dn6)))) * locals.var_r) + (assign62770_e81264 * locals.var_r_dn6)))), (((locals.var_t1_dn7 / 12.0) - ((locals.var_t2_dn7 * assign62770_e81252) + (locals.var_t2 * (locals.var_t1_dn7 - (12.0 * locals.var_t2_dn7))))) - (1.6 * ((((locals.var_t2_dn7 * assign62770_e81263) + (locals.var_t2 * (locals.var_t1_dn7 - (12.0 * locals.var_t2_dn7)))) * locals.var_r) + (assign62770_e81264 * locals.var_r_dn7)))), (((locals.var_t1_dn8 / 12.0) - ((locals.var_t2_dn8 * assign62770_e81252) + (locals.var_t2 * (locals.var_t1_dn8 - (12.0 * locals.var_t2_dn8))))) - (1.6 * ((((locals.var_t2_dn8 * assign62770_e81263) + (locals.var_t2 * (locals.var_t1_dn8 - (12.0 * locals.var_t2_dn8)))) * locals.var_r) + (assign62770_e81264 * locals.var_r_dn8)))), (((locals.var_t1_dn9 / 12.0) - ((locals.var_t2_dn9 * assign62770_e81252) + (locals.var_t2 * (locals.var_t1_dn9 - (12.0 * locals.var_t2_dn9))))) - (1.6 * ((((locals.var_t2_dn9 * assign62770_e81263) + (locals.var_t2 * (locals.var_t1_dn9 - (12.0 * locals.var_t2_dn9)))) * locals.var_r) + (assign62770_e81264 * locals.var_r_dn9)))),)
    } else {
        (locals.var_mig, locals.var_mig_dn4, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn9,)
    }
};
        locals.var_mig = assign62770_e81270;
        locals.var_mig_dn4 = assign62770_e81270_d_n4;
        locals.var_mig_dn6 = assign62770_e81270_d_n6;
        locals.var_mig_dn7 = assign62770_e81270_d_n7;
        locals.var_mig_dn8 = assign62770_e81270_d_n8;
        locals.var_mig_dn9 = assign62770_e81270_d_n9;

        let (assign62780_e81281, assign62780_e81281_d_n4, assign62780_e81281_d_n6, assign62780_e81281_d_n7, assign62780_e81281_d_n8, assign62780_e81281_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) {
        let (assign62780_e81279, assign62780_e81279_d_n4, assign62780_e81279_d_n6, assign62780_e81279_d_n7, assign62780_e81279_d_n8, assign62780_e81279_d_n9,) = {
            if (locals.var_mig > 1e-40) {
                (locals.var_mig, locals.var_mig_dn4, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn9,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62780_e81279, assign62780_e81279_d_n4, assign62780_e81279_d_n6, assign62780_e81279_d_n7, assign62780_e81279_d_n8, assign62780_e81279_d_n9,)
    } else {
        (locals.var_mig, locals.var_mig_dn4, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn9,)
    }
};
        locals.var_mig = assign62780_e81281;
        locals.var_mig_dn4 = assign62780_e81281_d_n4;
        locals.var_mig_dn6 = assign62780_e81281_d_n6;
        locals.var_mig_dn7 = assign62780_e81281_d_n7;
        locals.var_mig_dn8 = assign62780_e81281_d_n8;
        locals.var_mig_dn9 = assign62780_e81281_d_n9;

        let (assign62790_e81291, assign62790_e81291_d_n4, assign62790_e81291_d_n6, assign62790_e81291_d_n7, assign62790_e81291_d_n8, assign62790_e81291_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) {
        let assign62790_e81287: f64 = (locals.var_lcinv2 / locals.var_g_ideal);
        let assign62790_e81289: f64 = (assign62790_e81287 * locals.var_mig);
        (assign62790_e81289, (((((locals.var_lcinv2_dn4 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn4)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62790_e81287 * locals.var_mig_dn4)), (((((locals.var_lcinv2_dn6 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn6)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62790_e81287 * locals.var_mig_dn6)), (((((locals.var_lcinv2_dn7 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn7)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62790_e81287 * locals.var_mig_dn7)), (((((locals.var_lcinv2_dn8 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn8)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62790_e81287 * locals.var_mig_dn8)), (((((locals.var_lcinv2_dn9 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn9)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62790_e81287 * locals.var_mig_dn9)),)
    } else {
        (locals.var_mig, locals.var_mig_dn4, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn9,)
    }
};
        locals.var_mig = assign62790_e81291;
        locals.var_mig_dn4 = assign62790_e81291_d_n4;
        locals.var_mig_dn6 = assign62790_e81291_d_n6;
        locals.var_mig_dn7 = assign62790_e81291_d_n7;
        locals.var_mig_dn8 = assign62790_e81291_d_n8;
        locals.var_mig_dn9 = assign62790_e81291_d_n9;

        let (assign62800_e81319, assign62800_e81319_d_n4, assign62800_e81319_d_n6, assign62800_e81319_d_n7, assign62800_e81319_d_n8, assign62800_e81319_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) {
        let assign62800_e81297: f64 = (locals.var_lcinv2 * locals.var_sqt2);
        let assign62800_e81301: f64 = (12.0 * locals.var_t2);
        let assign62800_e81302: f64 = (1.0 - assign62800_e81301);
        let assign62800_e81306: f64 = (19.2 * locals.var_t2);
        let assign62800_e81307: f64 = (locals.var_t1 + assign62800_e81306);
        let assign62800_e81311: f64 = (locals.var_t1 * locals.var_t2);
        let assign62800_e81312: f64 = (12.0 * assign62800_e81311);
        let assign62800_e81313: f64 = (assign62800_e81307 - assign62800_e81312);
        let assign62800_e81315: f64 = (assign62800_e81313 * locals.var_r);
        let assign62800_e81316: f64 = (assign62800_e81302 - assign62800_e81315);
        let assign62800_e81317: f64 = (assign62800_e81297 * assign62800_e81316);
        (assign62800_e81317, ((((locals.var_lcinv2_dn4 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn4)) * assign62800_e81316) + (assign62800_e81297 * ((-(12.0 * locals.var_t2_dn4)) - ((((locals.var_t1_dn4 + (19.2 * locals.var_t2_dn4)) - (12.0 * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * locals.var_r) + (assign62800_e81313 * locals.var_r_dn4))))), ((((locals.var_lcinv2_dn6 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn6)) * assign62800_e81316) + (assign62800_e81297 * ((-(12.0 * locals.var_t2_dn6)) - ((((locals.var_t1_dn6 + (19.2 * locals.var_t2_dn6)) - (12.0 * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * locals.var_r) + (assign62800_e81313 * locals.var_r_dn6))))), ((((locals.var_lcinv2_dn7 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn7)) * assign62800_e81316) + (assign62800_e81297 * ((-(12.0 * locals.var_t2_dn7)) - ((((locals.var_t1_dn7 + (19.2 * locals.var_t2_dn7)) - (12.0 * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * locals.var_r) + (assign62800_e81313 * locals.var_r_dn7))))), ((((locals.var_lcinv2_dn8 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn8)) * assign62800_e81316) + (assign62800_e81297 * ((-(12.0 * locals.var_t2_dn8)) - ((((locals.var_t1_dn8 + (19.2 * locals.var_t2_dn8)) - (12.0 * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * locals.var_r) + (assign62800_e81313 * locals.var_r_dn8))))), ((((locals.var_lcinv2_dn9 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn9)) * assign62800_e81316) + (assign62800_e81297 * ((-(12.0 * locals.var_t2_dn9)) - ((((locals.var_t1_dn9 + (19.2 * locals.var_t2_dn9)) - (12.0 * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * locals.var_r) + (assign62800_e81313 * locals.var_r_dn9))))),)
    } else {
        (locals.var_migid0, locals.var_migid0_dn4, locals.var_migid0_dn6, locals.var_migid0_dn7, locals.var_migid0_dn8, locals.var_migid0_dn9,)
    }
};
        locals.var_migid0 = assign62800_e81319;
        locals.var_migid0_dn4 = assign62800_e81319_d_n4;
        locals.var_migid0_dn6 = assign62800_e81319_d_n6;
        locals.var_migid0_dn7 = assign62800_e81319_d_n7;
        locals.var_migid0_dn8 = assign62800_e81319_d_n8;
        locals.var_migid0_dn9 = assign62800_e81319_d_n9;

        let (assign62810_e81335, assign62810_e81335_d_n4, assign62810_e81335_d_n6, assign62810_e81335_d_n7, assign62810_e81335_d_n8, assign62810_e81335_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) {
        let assign62810_e81325: f64 = (locals.var_gvsat_ac * locals.var_gvsat_ac);
        let assign62810_e81327: f64 = (assign62810_e81325 * locals.var_cox_qm);
        let assign62810_e81329: f64 = (assign62810_e81327 * locals.var_eta_p_ac);
        let assign62810_e81332: f64 = (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac);
        let assign62810_e81333: f64 = (assign62810_e81329 / assign62810_e81332);
        (assign62810_e81333, (((((((((locals.var_gvsat_ac_dn4 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn4)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn4)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn4)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn4 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn4)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn6 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn6)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn6)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn6)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn6 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn6)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn7 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn7)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn7)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn7)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn7 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn7)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn8 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn8)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn8)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn8)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn8 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn8)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn9 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn9)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn9)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn9)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn9 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn9)))) / (assign62810_e81332 * assign62810_e81332)),)
    } else {
        (locals.var_cgeff, locals.var_cgeff_dn4, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, locals.var_cgeff_dn9,)
    }
};
        locals.var_cgeff = assign62810_e81335;
        locals.var_cgeff_dn4 = assign62810_e81335_d_n4;
        locals.var_cgeff_dn6 = assign62810_e81335_d_n6;
        locals.var_cgeff_dn7 = assign62810_e81335_d_n7;
        locals.var_cgeff_dn8 = assign62810_e81335_d_n8;
        locals.var_cgeff_dn9 = assign62810_e81335_d_n9;

        let assign62820_e81338: f64 = if locals.var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1788 = assign62820_e81338;

        let (assign62830_e81362, assign62830_e81362_d_n4, assign62830_e81362_d_n6, assign62830_e81362_d_n7, assign62830_e81362_d_n8, assign62830_e81362_d_n9,) = {
    if (((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) && (locals.var_guard1788 != 0.0)) {
        let assign62830_e81349: f64 = (12.0 * locals.var_t2);
        let assign62830_e81350: f64 = (1.0 + assign62830_e81349);
        let assign62830_e81351: f64 = (locals.var_sidexc * assign62830_e81350);
        let assign62830_e81354: f64 = (12.0 * locals.var_g_ideal);
        let assign62830_e81356: f64 = (assign62830_e81354 * locals.var_g_ideal);
        let assign62830_e81358: f64 = (assign62830_e81356 * locals.var_nt0);
        let assign62830_e81359: f64 = (assign62830_e81351 / assign62830_e81358);
        let assign62830_e81360: f64 = (locals.var_mig + assign62830_e81359);
        (assign62830_e81360, (locals.var_mig_dn4 + (((((locals.var_sidexc_dn4 * assign62830_e81350) + (locals.var_sidexc * (12.0 * locals.var_t2_dn4))) * assign62830_e81358) - (assign62830_e81351 * (((((12.0 * locals.var_g_ideal_dn4) * locals.var_g_ideal) + (assign62830_e81354 * locals.var_g_ideal_dn4)) * locals.var_nt0) + (assign62830_e81356 * locals.var_nt0_dn4)))) / (assign62830_e81358 * assign62830_e81358))), (locals.var_mig_dn6 + (((((locals.var_sidexc_dn6 * assign62830_e81350) + (locals.var_sidexc * (12.0 * locals.var_t2_dn6))) * assign62830_e81358) - (assign62830_e81351 * ((((12.0 * locals.var_g_ideal_dn6) * locals.var_g_ideal) + (assign62830_e81354 * locals.var_g_ideal_dn6)) * locals.var_nt0))) / (assign62830_e81358 * assign62830_e81358))), (locals.var_mig_dn7 + (((((locals.var_sidexc_dn7 * assign62830_e81350) + (locals.var_sidexc * (12.0 * locals.var_t2_dn7))) * assign62830_e81358) - (assign62830_e81351 * ((((12.0 * locals.var_g_ideal_dn7) * locals.var_g_ideal) + (assign62830_e81354 * locals.var_g_ideal_dn7)) * locals.var_nt0))) / (assign62830_e81358 * assign62830_e81358))), (locals.var_mig_dn8 + (((((locals.var_sidexc_dn8 * assign62830_e81350) + (locals.var_sidexc * (12.0 * locals.var_t2_dn8))) * assign62830_e81358) - (assign62830_e81351 * ((((12.0 * locals.var_g_ideal_dn8) * locals.var_g_ideal) + (assign62830_e81354 * locals.var_g_ideal_dn8)) * locals.var_nt0))) / (assign62830_e81358 * assign62830_e81358))), (locals.var_mig_dn9 + (((((locals.var_sidexc_dn9 * assign62830_e81350) + (locals.var_sidexc * (12.0 * locals.var_t2_dn9))) * assign62830_e81358) - (assign62830_e81351 * ((((12.0 * locals.var_g_ideal_dn9) * locals.var_g_ideal) + (assign62830_e81354 * locals.var_g_ideal_dn9)) * locals.var_nt0))) / (assign62830_e81358 * assign62830_e81358))),)
    } else {
        (locals.var_mig, locals.var_mig_dn4, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn9,)
    }
};
        locals.var_mig = assign62830_e81362;
        locals.var_mig_dn4 = assign62830_e81362_d_n4;
        locals.var_mig_dn6 = assign62830_e81362_d_n6;
        locals.var_mig_dn7 = assign62830_e81362_d_n7;
        locals.var_mig_dn8 = assign62830_e81362_d_n8;
        locals.var_mig_dn9 = assign62830_e81362_d_n9;

        let (assign62840_e81382, assign62840_e81382_d_n4, assign62840_e81382_d_n6, assign62840_e81382_d_n7, assign62840_e81382_d_n8, assign62840_e81382_d_n9,) = {
    if (((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) && (locals.var_guard1788 != 0.0)) {
        let assign62840_e81371: f64 = (locals.var_sidexc * locals.var_sqt2);
        let assign62840_e81374: f64 = (1.0 + locals.var_r);
        let assign62840_e81375: f64 = (assign62840_e81371 * assign62840_e81374);
        let assign62840_e81378: f64 = (locals.var_g_ideal * locals.var_nt0);
        let assign62840_e81379: f64 = (assign62840_e81375 / assign62840_e81378);
        let assign62840_e81380: f64 = (locals.var_migid0 - assign62840_e81379);
        (assign62840_e81380, (locals.var_migid0_dn4 - (((((((locals.var_sidexc_dn4 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn4)) * assign62840_e81374) + (assign62840_e81371 * locals.var_r_dn4)) * assign62840_e81378) - (assign62840_e81375 * ((locals.var_g_ideal_dn4 * locals.var_nt0) + (locals.var_g_ideal * locals.var_nt0_dn4)))) / (assign62840_e81378 * assign62840_e81378))), (locals.var_migid0_dn6 - (((((((locals.var_sidexc_dn6 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn6)) * assign62840_e81374) + (assign62840_e81371 * locals.var_r_dn6)) * assign62840_e81378) - (assign62840_e81375 * (locals.var_g_ideal_dn6 * locals.var_nt0))) / (assign62840_e81378 * assign62840_e81378))), (locals.var_migid0_dn7 - (((((((locals.var_sidexc_dn7 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn7)) * assign62840_e81374) + (assign62840_e81371 * locals.var_r_dn7)) * assign62840_e81378) - (assign62840_e81375 * (locals.var_g_ideal_dn7 * locals.var_nt0))) / (assign62840_e81378 * assign62840_e81378))), (locals.var_migid0_dn8 - (((((((locals.var_sidexc_dn8 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn8)) * assign62840_e81374) + (assign62840_e81371 * locals.var_r_dn8)) * assign62840_e81378) - (assign62840_e81375 * (locals.var_g_ideal_dn8 * locals.var_nt0))) / (assign62840_e81378 * assign62840_e81378))), (locals.var_migid0_dn9 - (((((((locals.var_sidexc_dn9 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn9)) * assign62840_e81374) + (assign62840_e81371 * locals.var_r_dn9)) * assign62840_e81378) - (assign62840_e81375 * (locals.var_g_ideal_dn9 * locals.var_nt0))) / (assign62840_e81378 * assign62840_e81378))),)
    } else {
        (locals.var_migid0, locals.var_migid0_dn4, locals.var_migid0_dn6, locals.var_migid0_dn7, locals.var_migid0_dn8, locals.var_migid0_dn9,)
    }
};
        locals.var_migid0 = assign62840_e81382;
        locals.var_migid0_dn4 = assign62840_e81382_d_n4;
        locals.var_migid0_dn6 = assign62840_e81382_d_n6;
        locals.var_migid0_dn7 = assign62840_e81382_d_n7;
        locals.var_migid0_dn8 = assign62840_e81382_d_n8;
        locals.var_migid0_dn9 = assign62840_e81382_d_n9;

        let (assign62850_e81391, assign62850_e81391_d_n4, assign62850_e81391_d_n6, assign62850_e81391_d_n7, assign62850_e81391_d_n8, assign62850_e81391_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) {
        let assign62850_e81388: f64 = (locals.var_nt / locals.var_mig);
        let assign62850_e81389: f64 = (assign62850_e81388).sqrt();
        (assign62850_e81389, ((((locals.var_nt_dn4 * locals.var_mig) - (locals.var_nt * locals.var_mig_dn4)) / (locals.var_mig * locals.var_mig)) / (2.0 * assign62850_e81389)), ((-((locals.var_nt * locals.var_mig_dn6) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62850_e81389)), ((-((locals.var_nt * locals.var_mig_dn7) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62850_e81389)), ((-((locals.var_nt * locals.var_mig_dn8) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62850_e81389)), ((-((locals.var_nt * locals.var_mig_dn9) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62850_e81389)),)
    } else {
        (locals.var_sqig, locals.var_sqig_dn4, locals.var_sqig_dn6, locals.var_sqig_dn7, locals.var_sqig_dn8, locals.var_sqig_dn9,)
    }
};
        locals.var_sqig = assign62850_e81391;
        locals.var_sqig_dn4 = assign62850_e81391_d_n4;
        locals.var_sqig_dn6 = assign62850_e81391_d_n6;
        locals.var_sqig_dn7 = assign62850_e81391_d_n7;
        locals.var_sqig_dn8 = assign62850_e81391_d_n8;
        locals.var_sqig_dn9 = assign62850_e81391_d_n9;

        let assign62860_e81394: f64 = if locals.var_sqid <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1789 = assign62860_e81394;

        let (assign62870_e81402, assign62870_e81402_d_n4, assign62870_e81402_d_n6, assign62870_e81402_d_n7, assign62870_e81402_d_n8, assign62870_e81402_d_n9,) = {
    if (((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) && (locals.var_guard1789 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_c_igid, locals.var_c_igid_dn4, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, locals.var_c_igid_dn9,)
    }
};
        locals.var_c_igid = assign62870_e81402;
        locals.var_c_igid_dn4 = assign62870_e81402_d_n4;
        locals.var_c_igid_dn6 = assign62870_e81402_d_n6;
        locals.var_c_igid_dn7 = assign62870_e81402_d_n7;
        locals.var_c_igid_dn8 = assign62870_e81402_d_n8;
        locals.var_c_igid_dn9 = assign62870_e81402_d_n9;

        let (assign62880_e81415, assign62880_e81415_d_n4, assign62880_e81415_d_n6, assign62880_e81415_d_n7, assign62880_e81415_d_n8, assign62880_e81415_d_n9,) = {
    if (((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign62880_e81411: f64 = (locals.var_migid0 * locals.var_sqig);
        let assign62880_e81413: f64 = (assign62880_e81411 / locals.var_sqid);
        (assign62880_e81413, (((((locals.var_migid0_dn4 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn4)) * locals.var_sqid) - (assign62880_e81411 * locals.var_sqid_dn4)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn6 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn6)) * locals.var_sqid) - (assign62880_e81411 * locals.var_sqid_dn6)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn7 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn7)) * locals.var_sqid) - (assign62880_e81411 * locals.var_sqid_dn7)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn8 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn8)) * locals.var_sqid) - (assign62880_e81411 * locals.var_sqid_dn8)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn9 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn9)) * locals.var_sqid) - (assign62880_e81411 * locals.var_sqid_dn9)) / (locals.var_sqid * locals.var_sqid)),)
    } else {
        (locals.var_c_igid, locals.var_c_igid_dn4, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, locals.var_c_igid_dn9,)
    }
};
        locals.var_c_igid = assign62880_e81415;
        locals.var_c_igid_dn4 = assign62880_e81415_d_n4;
        locals.var_c_igid_dn6 = assign62880_e81415_d_n6;
        locals.var_c_igid_dn7 = assign62880_e81415_d_n7;
        locals.var_c_igid_dn8 = assign62880_e81415_d_n8;
        locals.var_c_igid_dn9 = assign62880_e81415_d_n9;

        let (assign62890_e81431, assign62890_e81431_d_n4, assign62890_e81431_d_n6, assign62890_e81431_d_n7, assign62890_e81431_d_n8, assign62890_e81431_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) {
        let (assign62890_e81429, assign62890_e81429_d_n4, assign62890_e81429_d_n6, assign62890_e81429_d_n7, assign62890_e81429_d_n8, assign62890_e81429_d_n9,) = {
            if (locals.var_c_igid > 0.0) {
                let (assign62890_e81427, assign62890_e81427_d_n4, assign62890_e81427_d_n6, assign62890_e81427_d_n7, assign62890_e81427_d_n8, assign62890_e81427_d_n9,) = {
                    if (locals.var_c_igid < 1.0) {
                        (locals.var_c_igid, locals.var_c_igid_dn4, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, locals.var_c_igid_dn9,)
                    } else {
                        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign62890_e81427, assign62890_e81427_d_n4, assign62890_e81427_d_n6, assign62890_e81427_d_n7, assign62890_e81427_d_n8, assign62890_e81427_d_n9,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62890_e81429, assign62890_e81429_d_n4, assign62890_e81429_d_n6, assign62890_e81429_d_n7, assign62890_e81429_d_n8, assign62890_e81429_d_n9,)
    } else {
        (locals.var_c_igid, locals.var_c_igid_dn4, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, locals.var_c_igid_dn9,)
    }
};
        locals.var_c_igid = assign62890_e81431;
        locals.var_c_igid_dn4 = assign62890_e81431_d_n4;
        locals.var_c_igid_dn6 = assign62890_e81431_d_n6;
        locals.var_c_igid_dn7 = assign62890_e81431_d_n7;
        locals.var_c_igid_dn8 = assign62890_e81431_d_n8;
        locals.var_c_igid_dn9 = assign62890_e81431_d_n9;

        let (assign62900_e81441, assign62900_e81441_d_n4, assign62900_e81441_d_n6, assign62900_e81441_d_n7, assign62900_e81441_d_n8, assign62900_e81441_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) {
        let assign62900_e81437: f64 = (locals.var_c_igid * locals.var_sqid);
        let assign62900_e81439: f64 = (assign62900_e81437 / locals.var_sqig);
        (assign62900_e81439, (((((locals.var_c_igid_dn4 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn4)) * locals.var_sqig) - (assign62900_e81437 * locals.var_sqig_dn4)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn6 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn6)) * locals.var_sqig) - (assign62900_e81437 * locals.var_sqig_dn6)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn7 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn7)) * locals.var_sqig) - (assign62900_e81437 * locals.var_sqig_dn7)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn8 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn8)) * locals.var_sqig) - (assign62900_e81437 * locals.var_sqig_dn8)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn9 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn9)) * locals.var_sqig) - (assign62900_e81437 * locals.var_sqig_dn9)) / (locals.var_sqig * locals.var_sqig)),)
    } else {
        (locals.var_migid, locals.var_migid_dn4, locals.var_migid_dn6, locals.var_migid_dn7, locals.var_migid_dn8, locals.var_migid_dn9,)
    }
};
        locals.var_migid = assign62900_e81441;
        locals.var_migid_dn4 = assign62900_e81441_d_n4;
        locals.var_migid_dn6 = assign62900_e81441_d_n6;
        locals.var_migid_dn7 = assign62900_e81441_d_n7;
        locals.var_migid_dn8 = assign62900_e81441_d_n8;
        locals.var_migid_dn9 = assign62900_e81441_d_n9;

        let assign63070_e81549: f64 = if (((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) && (locals.var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1791 = assign63070_e81549;

        let (assign63080_e81557, assign63080_e81557_d_n4, assign63080_e81557_d_n6, assign63080_e81557_d_n7, assign63080_e81557_d_n8, assign63080_e81557_d_n9,) = {
    if (locals.var_guard1791 != 0.0) {
        let assign63080_e81553: f64 = (4.0 * locals.var_dsqredge);
        let assign63080_e81555: f64 = (assign63080_e81553 / locals.var_gfedge2);
        (assign63080_e81555, ((((4.0 * locals.var_dsqredge_dn4) * locals.var_gfedge2) - (assign63080_e81553 * locals.var_gfedge2_dn4)) / (locals.var_gfedge2 * locals.var_gfedge2)), ((4.0 * locals.var_dsqredge_dn6) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn7) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn8) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn9) / locals.var_gfedge2),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign63080_e81557;
        locals.var_temp1_dn4 = assign63080_e81557_d_n4;
        locals.var_temp1_dn6 = assign63080_e81557_d_n6;
        locals.var_temp1_dn7 = assign63080_e81557_d_n7;
        locals.var_temp1_dn8 = assign63080_e81557_d_n8;
        locals.var_temp1_dn9 = assign63080_e81557_d_n9;

        let (assign63100_e81577, assign63100_e81577_d_n4, assign63100_e81577_d_n6, assign63100_e81577_d_n7, assign63100_e81577_d_n8, assign63100_e81577_d_n9,) = {
    if (locals.var_guard1791 != 0.0) {
        let assign63100_e81575: f64 = (locals.var_cox_over_q * locals.var_phit);
        (assign63100_e81575, (locals.var_cox_over_q * locals.var_phit_dn4), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign63100_e81577;
        locals.var_temp1_dn4 = assign63100_e81577_d_n4;
        locals.var_temp1_dn6 = assign63100_e81577_d_n6;
        locals.var_temp1_dn7 = assign63100_e81577_d_n7;
        locals.var_temp1_dn8 = assign63100_e81577_d_n8;
        locals.var_temp1_dn9 = assign63100_e81577_d_n9;

        let (assign63230_e81717, assign63230_e81717_d_n4, assign63230_e81717_d_n6, assign63230_e81717_d_n7, assign63230_e81717_d_n8, assign63230_e81717_d_n9,) = {
    if (locals.var_guard1791 != 0.0) {
        let assign63230_e81715: f64 = (locals.var_alpha_dc * locals.var_h_dc);
        (assign63230_e81715, ((locals.var_alpha_dc_dn4 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn4)), ((locals.var_alpha_dc_dn6 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn6)), ((locals.var_alpha_dc_dn7 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn7)), ((locals.var_alpha_dc_dn8 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn8)), ((locals.var_alpha_dc_dn9 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign63230_e81717;
        locals.var_temp1_dn4 = assign63230_e81717_d_n4;
        locals.var_temp1_dn6 = assign63230_e81717_d_n6;
        locals.var_temp1_dn7 = assign63230_e81717_d_n7;
        locals.var_temp1_dn8 = assign63230_e81717_d_n8;
        locals.var_temp1_dn9 = assign63230_e81717_d_n9;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e1484: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e1484;
        locals.var_guard1_rv = 0.0;

        let (assign10_e1489,) = {
    if (locals.var_guard1 != 0.0) {
        let assign10_e1487: f64 = 1.0;
        (assign10_e1487,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign10_e1489;
        locals.var_chnl_type_rv = 0.0;

        let (assign20_e1495,) = {
    if (locals.var_guard1 == 0.0) {
        let assign20_e1493: f64 = (-1.0);
        (assign20_e1493,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign20_e1495;
        locals.var_chnl_type_rv = 0.0;

        let assign30_e1498: f64 = (8.8541878176e-12 * 11.8);
        locals.var_epssi = assign30_e1498;
        locals.var_epssi_rv = 0.0;

        let assign40_e1501: f64 = (273.15 + p.p38);
        locals.var_tkr = assign40_e1501;
        locals.var_tkr_rv = 0.0;

        let assign2050_e2532: f64 = ctx_temp;
        let assign2050_e2534: f64 = (assign2050_e2532 + p.p55);
        let assign2050_e2536: f64 = (assign2050_e2534 + p.p35);
        locals.var_tka = assign2050_e2536;
        locals.var_tka_rv = 0.0;

        let assign2060_e2539: f64 = (locals.var_tka / locals.var_tkr);
        locals.var_rta = assign2060_e2539;
        locals.var_rta_rv = 0.0;

        let assign2070_e2542: f64 = (locals.var_tka - locals.var_tkr);
        locals.var_delta = assign2070_e2542;
        locals.var_delta_rv = 0.0;

        let assign2080_e2545: f64 = (locals.var_tka * 1.3806505e-23);
        let assign2080_e2547: f64 = (assign2080_e2545 / 1.6021918e-19);
        locals.var_phita = assign2080_e2547;
        locals.var_phita_rv = 0.0;

        let assign2090_e2550: f64 = (1.0 / locals.var_phita);
        locals.var_inv_phita = assign2090_e2550;
        locals.var_inv_phita_rv = 0.0;

        locals.var_nf_i = 1.0;
        locals.var_nf_i_rv = 0.0;

        locals.var_invnf = 1.0;
        locals.var_invnf_rv = 0.0;

        locals.var_le = 0.0;
        locals.var_le_rv = 0.0;

        locals.var_we = 0.0;
        locals.var_we_rv = 0.0;

        locals.var_l_i = p.p0;
        locals.var_l_i_rv = 0.0;

        locals.var_w_i = p.p1;
        locals.var_w_i_rv = 0.0;

        locals.var_sa_i = p.p2;
        locals.var_sa_i_rv = 0.0;

        locals.var_sb_i = p.p3;
        locals.var_sb_i_rv = 0.0;

        locals.var_sd_i = p.p4;
        locals.var_sd_i_rv = 0.0;

        locals.var_sc_i = p.p8;
        locals.var_sc_i_rv = 0.0;

        let assign3390_e3398: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3390_e3398;
        locals.var_guard29_rv = 0.0;

        let (assign3400_e3407,) = {
    if (locals.var_guard29 != 0.0) {
        let (assign3400_e3405,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3400_e3405,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3400_e3407;
        locals.var_nf_i_rv = 0.0;

        let (assign3410_e3414,) = {
    if (locals.var_guard29 != 0.0) {
        let assign3410_e3411: f64 = (locals.var_nf_i + 0.5);
        let assign3410_e3412: f64 = (assign3410_e3411).floor();
        (assign3410_e3412,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3410_e3414;
        locals.var_nf_i_rv = 0.0;

        let (assign3420_e3420,) = {
    if (locals.var_guard29 != 0.0) {
        let assign3420_e3418: f64 = (1.0 / locals.var_nf_i);
        (assign3420_e3418,)
    } else {
        (locals.var_invnf,)
    }
};
        locals.var_invnf = assign3420_e3420;
        locals.var_invnf_rv = 0.0;

        let assign3430_e3423: f64 = (locals.var_w_i * locals.var_invnf);
        let (assign3430_e3430,) = {
    if (assign3430_e3423 > 1e-9) {
        let assign3430_e3428: f64 = (locals.var_w_i * locals.var_invnf);
        (assign3430_e3428,)
    } else {
        (1e-9,)
    }
};
        locals.var_w_i = assign3430_e3430;
        locals.var_w_i_rv = 0.0;

        locals.var_sca_i = p.p5;
        locals.var_sca_i_rv = 0.0;

        locals.var_scb_i = p.p6;
        locals.var_scb_i_rv = 0.0;

        locals.var_scc_i = p.p7;
        locals.var_scc_i_rv = 0.0;

        let assign3480_e3442: f64 = (1e-6 / locals.var_l_i);
        locals.var_il = assign3480_e3442;
        locals.var_il_rv = 0.0;

        let assign3490_e3445: f64 = (1e-6 / locals.var_w_i);
        locals.var_iw = assign3490_e3445;
        locals.var_iw_rv = 0.0;

        let assign3500_e3450: f64 = (p.p190 * locals.var_il);
        let assign3500_e3451: f64 = (1.0 + assign3500_e3450);
        let assign3500_e3452: f64 = (p.p189 * assign3500_e3451);
        let assign3500_e3456: f64 = (p.p191 * locals.var_iw);
        let assign3500_e3457: f64 = (1.0 + assign3500_e3456);
        let assign3500_e3458: f64 = (assign3500_e3452 * assign3500_e3457);
        locals.var_dellps = assign3500_e3458;
        locals.var_dellps_rv = 0.0;

        let assign3510_e3463: f64 = (p.p194 * locals.var_il);
        let assign3510_e3464: f64 = (1.0 + assign3510_e3463);
        let assign3510_e3465: f64 = (p.p193 * assign3510_e3464);
        let assign3510_e3469: f64 = (p.p195 * locals.var_iw);
        let assign3510_e3470: f64 = (1.0 + assign3510_e3469);
        let assign3510_e3471: f64 = (assign3510_e3465 * assign3510_e3470);
        locals.var_delwod = assign3510_e3471;
        locals.var_delwod_rv = 0.0;

        let assign3520_e3474: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3520_e3477: f64 = (2.0 * p.p192);
        let assign3520_e3478: f64 = (assign3520_e3474 - assign3520_e3477);
        let (assign3520_e3489,) = {
    if (assign3520_e3478 > 1e-9) {
        let assign3520_e3483: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3520_e3486: f64 = (2.0 * p.p192);
        let assign3520_e3487: f64 = (assign3520_e3483 - assign3520_e3486);
        (assign3520_e3487,)
    } else {
        (1e-9,)
    }
};
        locals.var_le = assign3520_e3489;
        locals.var_le_rv = 0.0;

        let assign3530_e3492: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3530_e3495: f64 = (2.0 * p.p196);
        let assign3530_e3496: f64 = (assign3530_e3492 - assign3530_e3495);
        let (assign3530_e3507,) = {
    if (assign3530_e3496 > 1e-9) {
        let assign3530_e3501: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3530_e3504: f64 = (2.0 * p.p196);
        let assign3530_e3505: f64 = (assign3530_e3501 - assign3530_e3504);
        (assign3530_e3505,)
    } else {
        (1e-9,)
    }
};
        locals.var_we = assign3530_e3507;
        locals.var_we_rv = 0.0;

        let assign3540_e3510: f64 = (1e-6 / locals.var_le);
        locals.var_ile = assign3540_e3510;
        locals.var_ile_rv = 0.0;

        let assign3550_e3513: f64 = (locals.var_ile * locals.var_ile);
        locals.var_ile2 = assign3550_e3513;
        locals.var_ile2_rv = 0.0;

        let assign3560_e3516: f64 = (1e-6 / locals.var_we);
        locals.var_iwe = assign3560_e3516;
        locals.var_iwe_rv = 0.0;

        let assign3570_e3519: f64 = (1.0 / locals.var_iwe);
        locals.var_iiwe = assign3570_e3519;
        locals.var_iiwe_rv = 0.0;

        let assign3580_e3522: f64 = (locals.var_ile * locals.var_iwe);
        locals.var_iae = assign3580_e3522;
        locals.var_iae_rv = 0.0;

        let assign3590_e3525: f64 = (1.0 / locals.var_iae);
        locals.var_iiae = assign3590_e3525;
        locals.var_iiae_rv = 0.0;

        let assign3600_e3528: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3600_e3531: f64 = (2.0 * p.p192);
        let assign3600_e3532: f64 = (assign3600_e3528 - assign3600_e3531);
        let assign3600_e3534: f64 = (assign3600_e3532 + p.p197);
        let (assign3600_e3547,) = {
    if (assign3600_e3534 > 1e-9) {
        let assign3600_e3539: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3600_e3542: f64 = (2.0 * p.p192);
        let assign3600_e3543: f64 = (assign3600_e3539 - assign3600_e3542);
        let assign3600_e3545: f64 = (assign3600_e3543 + p.p197);
        (assign3600_e3545,)
    } else {
        (1e-9,)
    }
};
        locals.var_lecv = assign3600_e3547;
        locals.var_lecv_rv = 0.0;

        let assign3610_e3550: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3610_e3553: f64 = (2.0 * p.p196);
        let assign3610_e3554: f64 = (assign3610_e3550 - assign3610_e3553);
        let assign3610_e3556: f64 = (assign3610_e3554 + p.p198);
        let (assign3610_e3569,) = {
    if (assign3610_e3556 > 1e-9) {
        let assign3610_e3561: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3610_e3564: f64 = (2.0 * p.p196);
        let assign3610_e3565: f64 = (assign3610_e3561 - assign3610_e3564);
        let assign3610_e3567: f64 = (assign3610_e3565 + p.p198);
        (assign3610_e3567,)
    } else {
        (1e-9,)
    }
};
        locals.var_wecv = assign3610_e3569;
        locals.var_wecv_rv = 0.0;

        let assign3620_e3572: f64 = (locals.var_wecv / 1e-6);
        locals.var_iiwecv = assign3620_e3572;
        locals.var_iiwecv_rv = 0.0;

        let assign3630_e3575: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3630_e3577: f64 = (assign3630_e3575 + p.p197);
        let (assign3630_e3586,) = {
    if (assign3630_e3577 > 1e-9) {
        let assign3630_e3582: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3630_e3584: f64 = (assign3630_e3582 + p.p197);
        (assign3630_e3584,)
    } else {
        (1e-9,)
    }
};
        locals.var_lcv = assign3630_e3586;
        locals.var_lcv_rv = 0.0;

        let assign3650_e3603: f64 = (locals.var_lcv / 1e-6);
        locals.var_iilcv = assign3650_e3603;
        locals.var_iilcv_rv = 0.0;

        locals.var_vfb_p = p.p56;
        locals.var_vfb_p_rv = 0.0;

        locals.var_stvfb_p = p.p57;
        locals.var_stvfb_p_rv = 0.0;

        locals.var_st2vfb_p = p.p58;
        locals.var_st2vfb_p_rv = 0.0;

        locals.var_tox_p = p.p59;
        locals.var_tox_p_rv = 0.0;

        locals.var_epsrox_p = p.p60;
        locals.var_epsrox_p_rv = 0.0;

        locals.var_neff_p = p.p61;
        locals.var_neff_p_rv = 0.0;

        locals.var_gfacnud_p = p.p62;
        locals.var_gfacnud_p_rv = 0.0;

        locals.var_vsbnud_p = p.p63;
        locals.var_vsbnud_p_rv = 0.0;

        locals.var_dvsbnud_p = p.p64;
        locals.var_dvsbnud_p_rv = 0.0;

        locals.var_dphib_p = p.p65;
        locals.var_dphib_p_rv = 0.0;

        locals.var_np_p = p.p66;
        locals.var_np_p_rv = 0.0;

        locals.var_toxov_p = p.p67;
        locals.var_toxov_p_rv = 0.0;

        locals.var_toxovd_p = p.p68;
        locals.var_toxovd_p_rv = 0.0;

        locals.var_nov_p = p.p69;
        locals.var_nov_p_rv = 0.0;

        locals.var_novd_p = p.p70;
        locals.var_novd_p_rv = 0.0;

        locals.var_ct_p = p.p71;
        locals.var_ct_p_rv = 0.0;

        locals.var_ctg_p = p.p73;
        locals.var_ctg_p_rv = 0.0;

        locals.var_ctb_p = p.p72;
        locals.var_ctb_p_rv = 0.0;

        locals.var_stct_p = p.p74;
        locals.var_stct_p_rv = 0.0;

        locals.var_psce_p = p.p78;
        locals.var_psce_p_rv = 0.0;

        locals.var_psced_p = p.p80;
        locals.var_psced_p_rv = 0.0;

        locals.var_psceb_p = p.p79;
        locals.var_psceb_p_rv = 0.0;

        locals.var_cf_p = p.p75;
        locals.var_cf_p_rv = 0.0;

        locals.var_cfd_p = p.p77;
        locals.var_cfd_p_rv = 0.0;

        locals.var_cfb_p = p.p76;
        locals.var_cfb_p_rv = 0.0;

        locals.var_betn_p = p.p81;
        locals.var_betn_p_rv = 0.0;

        locals.var_stbet_p = p.p82;
        locals.var_stbet_p_rv = 0.0;

        locals.var_mue_p = p.p83;
        locals.var_mue_p_rv = 0.0;

        locals.var_stmue_p = p.p84;
        locals.var_stmue_p_rv = 0.0;

        locals.var_themu_p = p.p85;
        locals.var_themu_p_rv = 0.0;

        locals.var_stthemu_p = p.p86;
        locals.var_stthemu_p_rv = 0.0;

        locals.var_cs_p = p.p87;
        locals.var_cs_p_rv = 0.0;

        locals.var_stcs_p = p.p88;
        locals.var_stcs_p_rv = 0.0;

        locals.var_thecs_p = p.p89;
        locals.var_thecs_p_rv = 0.0;

        locals.var_stthecs_p = p.p90;
        locals.var_stthecs_p_rv = 0.0;

        locals.var_xcor_p = p.p91;
        locals.var_xcor_p_rv = 0.0;

        locals.var_stxcor_p = p.p92;
        locals.var_stxcor_p_rv = 0.0;

        locals.var_feta_p = p.p93;
        locals.var_feta_p_rv = 0.0;

        locals.var_rs_p = p.p94;
        locals.var_rs_p_rv = 0.0;

        locals.var_strs_p = p.p95;
        locals.var_strs_p_rv = 0.0;

        locals.var_rsb_p = p.p96;
        locals.var_rsb_p_rv = 0.0;

        locals.var_rsg_p = p.p97;
        locals.var_rsg_p_rv = 0.0;

        locals.var_thesat_p = p.p98;
        locals.var_thesat_p_rv = 0.0;

        locals.var_stthesat_p = p.p99;
        locals.var_stthesat_p_rv = 0.0;

        locals.var_thesatb_p = p.p100;
        locals.var_thesatb_p_rv = 0.0;

        locals.var_thesatg_p = p.p101;
        locals.var_thesatg_p_rv = 0.0;

        locals.var_thesatt_p = p.p102;
        locals.var_thesatt_p_rv = 0.0;

        locals.var_ax_p = p.p103;
        locals.var_ax_p_rv = 0.0;

        locals.var_alp_p = p.p104;
        locals.var_alp_p_rv = 0.0;

        locals.var_alp1_p = p.p105;
        locals.var_alp1_p_rv = 0.0;

        locals.var_alp2_p = p.p106;
        locals.var_alp2_p_rv = 0.0;

        locals.var_vp_p = p.p107;
        locals.var_vp_p_rv = 0.0;

        locals.var_a1_p = p.p108;
        locals.var_a1_p_rv = 0.0;

        locals.var_a2_p = p.p109;
        locals.var_a2_p_rv = 0.0;

        locals.var_sta2_p = p.p110;
        locals.var_sta2_p_rv = 0.0;

        locals.var_a3_p = p.p111;
        locals.var_a3_p_rv = 0.0;

        locals.var_a4_p = p.p112;
        locals.var_a4_p_rv = 0.0;

        locals.var_imaxii_p = p.p113;
        locals.var_imaxii_p_rv = 0.0;

        locals.var_gco_p = p.p114;
        locals.var_gco_p_rv = 0.0;

        locals.var_iginv_p = p.p115;
        locals.var_iginv_p_rv = 0.0;

        locals.var_igov_p = p.p116;
        locals.var_igov_p_rv = 0.0;

        locals.var_igovd_p = p.p117;
        locals.var_igovd_p_rv = 0.0;

        locals.var_stig_p = p.p118;
        locals.var_stig_p_rv = 0.0;

        locals.var_gc2_p = p.p119;
        locals.var_gc2_p_rv = 0.0;

        locals.var_gc3_p = p.p120;
        locals.var_gc3_p_rv = 0.0;

        locals.var_gc2ov_p = p.p119;
        locals.var_gc2ov_p_rv = 0.0;

        let assign4370_e3718: f64 = if param_given[121] { 1.0 } else { 0.0 };
        let assign4370_e3720: f64 = if assign4370_e3718 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign4370_e3720;
        locals.var_guard30_rv = 0.0;

        let (assign4380_e3724,) = {
    if (locals.var_guard30 != 0.0) {
        (p.p121,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign4380_e3724;
        locals.var_gc2ov_p_rv = 0.0;

        locals.var_gc3ov_p = p.p120;
        locals.var_gc3ov_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign4400_e3727: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4400_e3729: f64 = if assign4400_e3727 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign4400_e3729;
        locals.var_guard31_rv = 0.0;

        let (assign4410_e3733,) = {
    if (locals.var_guard31 != 0.0) {
        (p.p122,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign4410_e3733;
        locals.var_gc3ov_p_rv = 0.0;

        locals.var_gc2ovd_p = locals.var_gc2ov_p;
        locals.var_gc2ovd_p_rv = 0.0;

        let assign4430_e3736: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4430_e3738: f64 = if assign4430_e3736 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign4430_e3738;
        locals.var_guard32_rv = 0.0;

        let (assign4440_e3742,) = {
    if (locals.var_guard32 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign4440_e3742;
        locals.var_gc2ovd_p_rv = 0.0;

        locals.var_gc3ovd_p = locals.var_gc3ov_p;
        locals.var_gc3ovd_p_rv = 0.0;

        let assign4460_e3745: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4460_e3747: f64 = if assign4460_e3745 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign4460_e3747;
        locals.var_guard33_rv = 0.0;

        let (assign4470_e3751,) = {
    if (locals.var_guard33 != 0.0) {
        (p.p124,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign4470_e3751;
        locals.var_gc3ovd_p_rv = 0.0;

        locals.var_chib_p = p.p125;
        locals.var_chib_p_rv = 0.0;

        locals.var_agidl_p = p.p126;
        locals.var_agidl_p_rv = 0.0;

        locals.var_agidld_p = p.p127;
        locals.var_agidld_p_rv = 0.0;

        locals.var_bgidl_p = p.p128;
        locals.var_bgidl_p_rv = 0.0;

        locals.var_bgidld_p = p.p129;
        locals.var_bgidld_p_rv = 0.0;

        locals.var_stbgidl_p = p.p130;
        locals.var_stbgidl_p_rv = 0.0;

        locals.var_stbgidld_p = p.p131;
        locals.var_stbgidld_p_rv = 0.0;

        locals.var_cgidl_p = p.p132;
        locals.var_cgidl_p_rv = 0.0;

        locals.var_cgidld_p = p.p133;
        locals.var_cgidld_p_rv = 0.0;

        locals.var_cox_p = p.p134;
        locals.var_cox_p_rv = 0.0;

        locals.var_delvtac_p = p.p135;
        locals.var_delvtac_p_rv = 0.0;

        locals.var_facneffac_p = p.p136;
        locals.var_facneffac_p_rv = 0.0;

        locals.var_thesatac_p = p.p98;
        locals.var_thesatac_p_rv = 0.0;

        let assign4610_e3766: f64 = if param_given[137] { 1.0 } else { 0.0 };
        let assign4610_e3768: f64 = if assign4610_e3766 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign4610_e3768;
        locals.var_guard34_rv = 0.0;

        let (assign4620_e3772,) = {
    if (locals.var_guard34 != 0.0) {
        (p.p137,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign4620_e3772;
        locals.var_thesatac_p_rv = 0.0;

        locals.var_axac_p = p.p103;
        locals.var_axac_p_rv = 0.0;

        let assign4640_e3775: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4640_e3777: f64 = if assign4640_e3775 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign4640_e3777;
        locals.var_guard35_rv = 0.0;

        let (assign4650_e3781,) = {
    if (locals.var_guard35 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign4650_e3781;
        locals.var_axac_p_rv = 0.0;

        locals.var_alpac_p = p.p139;
        locals.var_alpac_p_rv = 0.0;

        locals.var_alp1ac_p = p.p140;
        locals.var_alp1ac_p_rv = 0.0;

        locals.var_cgov_p = p.p141;
        locals.var_cgov_p_rv = 0.0;

        locals.var_cgovd_p = p.p142;
        locals.var_cgovd_p_rv = 0.0;

        locals.var_fcgovacc_p = p.p143;
        locals.var_fcgovacc_p_rv = 0.0;

        locals.var_fcgovaccd_p = p.p144;
        locals.var_fcgovaccd_p_rv = 0.0;

        locals.var_cgovaccg_p = p.p145;
        locals.var_cgovaccg_p_rv = 0.0;

        locals.var_cgbov_p = p.p146;
        locals.var_cgbov_p_rv = 0.0;

        locals.var_cinr_p = p.p147;
        locals.var_cinr_p_rv = 0.0;

        locals.var_cinrd_p = p.p148;
        locals.var_cinrd_p_rv = 0.0;

        locals.var_dvfbinr_p = p.p149;
        locals.var_dvfbinr_p_rv = 0.0;

        locals.var_fcinrdep_p = p.p150;
        locals.var_fcinrdep_p_rv = 0.0;

        locals.var_fcinracc_p = p.p151;
        locals.var_fcinracc_p_rv = 0.0;

        locals.var_axinr_p = p.p152;
        locals.var_axinr_p_rv = 0.0;

        locals.var_fnt_p = p.p155;
        locals.var_fnt_p_rv = 0.0;

        locals.var_vfbedge_p = p.p161;
        locals.var_vfbedge_p_rv = 0.0;

        locals.var_stvfbedge_p = p.p162;
        locals.var_stvfbedge_p_rv = 0.0;

        locals.var_dphibedge_p = p.p163;
        locals.var_dphibedge_p_rv = 0.0;

        locals.var_neffedge_p = p.p164;
        locals.var_neffedge_p_rv = 0.0;

        locals.var_ctedge_p = p.p165;
        locals.var_ctedge_p_rv = 0.0;

        locals.var_betnedge_p = p.p166;
        locals.var_betnedge_p_rv = 0.0;

        locals.var_stbetedge_p = p.p167;
        locals.var_stbetedge_p_rv = 0.0;

        locals.var_psceedge_p = p.p168;
        locals.var_psceedge_p_rv = 0.0;

        locals.var_pscebedge_p = p.p169;
        locals.var_pscebedge_p_rv = 0.0;

        locals.var_pscededge_p = p.p170;
        locals.var_pscededge_p_rv = 0.0;

        locals.var_cfedge_p = p.p171;
        locals.var_cfedge_p_rv = 0.0;

        locals.var_cfdedge_p = p.p173;
        locals.var_cfdedge_p_rv = 0.0;

        locals.var_cfbedge_p = p.p172;
        locals.var_cfbedge_p_rv = 0.0;

        let assign5160_e3834: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign5160_e3834;
        locals.var_guard36_rv = 0.0;

        let (assign5170_e3852,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5170_e3840: f64 = (locals.var_ile).powf(p.p201);
        let assign5170_e3841: f64 = (p.p200 * assign5170_e3840);
        let assign5170_e3842: f64 = (p.p199 + assign5170_e3841);
        let assign5170_e3845: f64 = (p.p202 * locals.var_iwe);
        let assign5170_e3846: f64 = (assign5170_e3842 + assign5170_e3845);
        let assign5170_e3849: f64 = (p.p203 * locals.var_iae);
        let assign5170_e3850: f64 = (assign5170_e3846 + assign5170_e3849);
        (assign5170_e3850,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign5170_e3852;
        locals.var_vfb_p_rv = 0.0;

        let (assign5180_e3868,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5180_e3857: f64 = (p.p205 * locals.var_ile);
        let assign5180_e3858: f64 = (p.p204 + assign5180_e3857);
        let assign5180_e3861: f64 = (p.p206 * locals.var_iwe);
        let assign5180_e3862: f64 = (assign5180_e3858 + assign5180_e3861);
        let assign5180_e3865: f64 = (p.p207 * locals.var_iae);
        let assign5180_e3866: f64 = (assign5180_e3862 + assign5180_e3865);
        (assign5180_e3866,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign5180_e3868;
        locals.var_stvfb_p_rv = 0.0;

        let (assign5190_e3872,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p208,)
    } else {
        (locals.var_st2vfb_p,)
    }
};
        locals.var_st2vfb_p = assign5190_e3872;
        locals.var_st2vfb_p_rv = 0.0;

        let (assign5200_e3876,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p209,)
    } else {
        (locals.var_tox_p,)
    }
};
        locals.var_tox_p = assign5200_e3876;
        locals.var_tox_p_rv = 0.0;

        let (assign5210_e3880,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p210,)
    } else {
        (locals.var_epsrox_p,)
    }
};
        locals.var_epsrox_p = assign5210_e3880;
        locals.var_epsrox_p_rv = 0.0;

        let (assign5220_e3913,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5220_e3886: f64 = (p.p212 * locals.var_iwe);
        let assign5220_e3890: f64 = (locals.var_we / p.p213);
        let assign5220_e3891: f64 = (1.0 + assign5220_e3890);
        let assign5220_e3892: f64 = (assign5220_e3891).ln();
        let assign5220_e3893: f64 = (assign5220_e3886 * assign5220_e3892);
        let assign5220_e3894: f64 = (1.0 + assign5220_e3893);
        let (assign5220_e3910,) = {
            if (assign5220_e3894 > 0.001) {
                let assign5220_e3900: f64 = (p.p212 * locals.var_iwe);
                let assign5220_e3904: f64 = (locals.var_we / p.p213);
                let assign5220_e3905: f64 = (1.0 + assign5220_e3904);
                let assign5220_e3906: f64 = (assign5220_e3905).ln();
                let assign5220_e3907: f64 = (assign5220_e3900 * assign5220_e3906);
                let assign5220_e3908: f64 = (1.0 + assign5220_e3907);
                (assign5220_e3908,)
            } else {
                (0.001,)
            }
        };
        let assign5220_e3911: f64 = (p.p211 * assign5220_e3910);
        (assign5220_e3911,)
    } else {
        (locals.var_nsub0e,)
    }
};
        locals.var_nsub0e = assign5220_e3913;
        locals.var_nsub0e_rv = 0.0;

        let (assign5230_e3946,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5230_e3919: f64 = (p.p215 * locals.var_iwe);
        let assign5230_e3923: f64 = (locals.var_we / p.p216);
        let assign5230_e3924: f64 = (1.0 + assign5230_e3923);
        let assign5230_e3925: f64 = (assign5230_e3924).ln();
        let assign5230_e3926: f64 = (assign5230_e3919 * assign5230_e3925);
        let assign5230_e3927: f64 = (1.0 + assign5230_e3926);
        let (assign5230_e3943,) = {
            if (assign5230_e3927 > 0.001) {
                let assign5230_e3933: f64 = (p.p215 * locals.var_iwe);
                let assign5230_e3937: f64 = (locals.var_we / p.p216);
                let assign5230_e3938: f64 = (1.0 + assign5230_e3937);
                let assign5230_e3939: f64 = (assign5230_e3938).ln();
                let assign5230_e3940: f64 = (assign5230_e3933 * assign5230_e3939);
                let assign5230_e3941: f64 = (1.0 + assign5230_e3940);
                (assign5230_e3941,)
            } else {
                (0.001,)
            }
        };
        let assign5230_e3944: f64 = (p.p214 * assign5230_e3943);
        (assign5230_e3944,)
    } else {
        (locals.var_npcke,)
    }
};
        locals.var_npcke = assign5230_e3946;
        locals.var_npcke_rv = 0.0;

        let (assign5240_e3979,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5240_e3952: f64 = (p.p218 * locals.var_iwe);
        let assign5240_e3956: f64 = (locals.var_we / p.p216);
        let assign5240_e3957: f64 = (1.0 + assign5240_e3956);
        let assign5240_e3958: f64 = (assign5240_e3957).ln();
        let assign5240_e3959: f64 = (assign5240_e3952 * assign5240_e3958);
        let assign5240_e3960: f64 = (1.0 + assign5240_e3959);
        let (assign5240_e3976,) = {
            if (assign5240_e3960 > 0.001) {
                let assign5240_e3966: f64 = (p.p218 * locals.var_iwe);
                let assign5240_e3970: f64 = (locals.var_we / p.p216);
                let assign5240_e3971: f64 = (1.0 + assign5240_e3970);
                let assign5240_e3972: f64 = (assign5240_e3971).ln();
                let assign5240_e3973: f64 = (assign5240_e3966 * assign5240_e3972);
                let assign5240_e3974: f64 = (1.0 + assign5240_e3973);
                (assign5240_e3974,)
            } else {
                (0.001,)
            }
        };
        let assign5240_e3977: f64 = (p.p217 * assign5240_e3976);
        (assign5240_e3977,)
    } else {
        (locals.var_lpcke,)
    }
};
        locals.var_lpcke = assign5240_e3979;
        locals.var_lpcke_rv = 0.0;

        let assign5250_e3983: f64 = (2.0 * locals.var_lpcke);
        let assign5250_e3984: f64 = if locals.var_le > assign5250_e3983 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign5250_e3984;
        locals.var_guard37_rv = 0.0;

        let (assign5260_e3990,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        (75000000000.0,)
    } else {
        (locals.var_aa,)
    }
};
        locals.var_aa = assign5260_e3990;
        locals.var_aa_rv = 0.0;

        let (assign5270_e4004,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5270_e3997: f64 = (0.5 * locals.var_npcke);
        let assign5270_e3998: f64 = (locals.var_nsub0e + assign5270_e3997);
        let assign5270_e3999: f64 = (assign5270_e3998).sqrt();
        let assign5270_e4001: f64 = (locals.var_nsub0e).sqrt();
        let assign5270_e4002: f64 = (assign5270_e3999 - assign5270_e4001);
        (assign5270_e4002,)
    } else {
        (locals.var_bb,)
    }
};
        locals.var_bb = assign5270_e4004;
        locals.var_bb_rv = 0.0;

        let (assign5280_e4029,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5280_e4009: f64 = (locals.var_nsub0e).sqrt();
        let assign5280_e4014: f64 = (2.0 * locals.var_lpcke);
        let assign5280_e4016: f64 = (assign5280_e4014 / locals.var_le);
        let assign5280_e4019: f64 = (locals.var_bb / locals.var_aa);
        let assign5280_e4020: f64 = (assign5280_e4019).exp();
        let assign5280_e4022: f64 = (assign5280_e4020 - 1.0);
        let assign5280_e4023: f64 = (assign5280_e4016 * assign5280_e4022);
        let assign5280_e4024: f64 = (1.0 + assign5280_e4023);
        let assign5280_e4025: f64 = (assign5280_e4024).ln();
        let assign5280_e4026: f64 = (locals.var_aa * assign5280_e4025);
        let assign5280_e4027: f64 = (assign5280_e4009 + assign5280_e4026);
        (assign5280_e4027,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5280_e4029;
        locals.var_nsub_rv = 0.0;

        let (assign5290_e4037,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5290_e4035: f64 = (locals.var_nsub * locals.var_nsub);
        (assign5290_e4035,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5290_e4037;
        locals.var_nsub_rv = 0.0;

        let assign5300_e4040: f64 = if locals.var_le >= locals.var_lpcke { 1.0 } else { 0.0 };
        locals.var_guard38 = assign5300_e4040;
        locals.var_guard38_rv = 0.0;

        let (assign5310_e4055,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 != 0.0)) {
        let assign5310_e4050: f64 = (locals.var_npcke * locals.var_lpcke);
        let assign5310_e4052: f64 = (assign5310_e4050 / locals.var_le);
        let assign5310_e4053: f64 = (locals.var_nsub0e + assign5310_e4052);
        (assign5310_e4053,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5310_e4055;
        locals.var_nsub_rv = 0.0;

        let (assign5320_e4073,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 == 0.0)) {
        let assign5320_e4068: f64 = (locals.var_le / locals.var_lpcke);
        let assign5320_e4069: f64 = (2.0 - assign5320_e4068);
        let assign5320_e4070: f64 = (locals.var_npcke * assign5320_e4069);
        let assign5320_e4071: f64 = (locals.var_nsub0e + assign5320_e4070);
        (assign5320_e4071,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5320_e4073;
        locals.var_nsub_rv = 0.0;

        let (assign5330_e4087,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5330_e4079: f64 = (p.p219 * locals.var_ile);
        let assign5330_e4080: f64 = (1.0 - assign5330_e4079);
        let assign5330_e4083: f64 = (p.p220 * locals.var_ile2);
        let assign5330_e4084: f64 = (assign5330_e4080 - assign5330_e4083);
        let assign5330_e4085: f64 = (locals.var_nsub * assign5330_e4084);
        (assign5330_e4085,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign5330_e4087;
        locals.var_neff_p_rv = 0.0;

        let (assign5340_e4105,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5340_e4093: f64 = (locals.var_ile).powf(p.p223);
        let assign5340_e4094: f64 = (p.p222 * assign5340_e4093);
        let assign5340_e4095: f64 = (p.p221 + assign5340_e4094);
        let assign5340_e4098: f64 = (p.p224 * locals.var_iwe);
        let assign5340_e4099: f64 = (assign5340_e4095 + assign5340_e4098);
        let assign5340_e4102: f64 = (p.p225 * locals.var_iae);
        let assign5340_e4103: f64 = (assign5340_e4099 + assign5340_e4102);
        (assign5340_e4103,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign5340_e4105;
        locals.var_gfacnud_p_rv = 0.0;

        let (assign5350_e4109,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p226,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign5350_e4109;
        locals.var_vsbnud_p_rv = 0.0;

        let (assign5360_e4113,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p227,)
    } else {
        (locals.var_dvsbnud_p,)
    }
};
        locals.var_dvsbnud_p = assign5360_e4113;
        locals.var_dvsbnud_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5370_e4131,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5370_e4119: f64 = (locals.var_ile).powf(p.p230);
        let assign5370_e4120: f64 = (p.p229 * assign5370_e4119);
        let assign5370_e4121: f64 = (p.p228 + assign5370_e4120);
        let assign5370_e4124: f64 = (p.p231 * locals.var_iwe);
        let assign5370_e4125: f64 = (assign5370_e4121 + assign5370_e4124);
        let assign5370_e4128: f64 = (p.p232 * locals.var_iae);
        let assign5370_e4129: f64 = (assign5370_e4125 + assign5370_e4128);
        (assign5370_e4129,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign5370_e4131;
        locals.var_dphib_p_rv = 0.0;

        let (assign5380_e4150,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5380_e4138: f64 = (p.p234 * locals.var_ile);
        let assign5380_e4139: f64 = (1.0 + assign5380_e4138);
        let (assign5380_e4147,) = {
            if (1e-6 > assign5380_e4139) {
                (1e-6,)
            } else {
                let assign5380_e4145: f64 = (p.p234 * locals.var_ile);
                let assign5380_e4146: f64 = (1.0 + assign5380_e4145);
                (assign5380_e4146,)
            }
        };
        let assign5380_e4148: f64 = (p.p233 * assign5380_e4147);
        (assign5380_e4148,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign5380_e4150;
        locals.var_np_p_rv = 0.0;

        let (assign5390_e4154,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p235,)
    } else {
        (locals.var_toxov_p,)
    }
};
        locals.var_toxov_p = assign5390_e4154;
        locals.var_toxov_p_rv = 0.0;

        let (assign5400_e4158,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p236,)
    } else {
        (locals.var_toxovd_p,)
    }
};
        locals.var_toxovd_p = assign5400_e4158;
        locals.var_toxovd_p_rv = 0.0;

        let (assign5410_e4162,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p239,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign5410_e4162;
        locals.var_nov_p_rv = 0.0;

        let (assign5420_e4166,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p240,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign5420_e4166;
        locals.var_novd_p_rv = 0.0;

        let (assign5430_e4188,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5430_e4172: f64 = (locals.var_ile).powf(p.p243);
        let assign5430_e4173: f64 = (p.p242 * assign5430_e4172);
        let assign5430_e4174: f64 = (p.p241 + assign5430_e4173);
        let assign5430_e4178: f64 = (p.p244 * locals.var_iwe);
        let assign5430_e4179: f64 = (1.0 + assign5430_e4178);
        let assign5430_e4180: f64 = (assign5430_e4174 * assign5430_e4179);
        let assign5430_e4184: f64 = (p.p245 * locals.var_iae);
        let assign5430_e4185: f64 = (1.0 + assign5430_e4184);
        let assign5430_e4186: f64 = (assign5430_e4180 * assign5430_e4185);
        (assign5430_e4186,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign5430_e4188;
        locals.var_ct_p_rv = 0.0;

        let (assign5440_e4192,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p247,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign5440_e4192;
        locals.var_ctg_p_rv = 0.0;

        let (assign5450_e4196,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p246,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign5450_e4196;
        locals.var_ctb_p_rv = 0.0;

        let (assign5460_e4200,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p248,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign5460_e4200;
        locals.var_stct_p_rv = 0.0;

        let (assign5470_e4214,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5470_e4205: f64 = (locals.var_ile).powf(p.p250);
        let assign5470_e4206: f64 = (p.p249 * assign5470_e4205);
        let assign5470_e4210: f64 = (p.p251 * locals.var_iwe);
        let assign5470_e4211: f64 = (1.0 + assign5470_e4210);
        let assign5470_e4212: f64 = (assign5470_e4206 * assign5470_e4211);
        (assign5470_e4212,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign5470_e4214;
        locals.var_cf_p_rv = 0.0;

        let (assign5480_e4218,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p253,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign5480_e4218;
        locals.var_cfd_p_rv = 0.0;

        let (assign5490_e4222,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p252,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign5490_e4222;
        locals.var_cfb_p_rv = 0.0;

        let (assign5500_e4236,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5500_e4227: f64 = (locals.var_ile).powf(p.p255);
        let assign5500_e4228: f64 = (p.p254 * assign5500_e4227);
        let assign5500_e4232: f64 = (p.p256 * locals.var_iwe);
        let assign5500_e4233: f64 = (1.0 + assign5500_e4232);
        let assign5500_e4234: f64 = (assign5500_e4228 * assign5500_e4233);
        (assign5500_e4234,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign5500_e4236;
        locals.var_psce_p_rv = 0.0;

        let (assign5510_e4240,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p258,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign5510_e4240;
        locals.var_psced_p_rv = 0.0;

        let (assign5520_e4244,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p257,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign5520_e4244;
        locals.var_psceb_p_rv = 0.0;

        let (assign5530_e4254,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5530_e4250: f64 = (p.p261 * locals.var_iwe);
        let assign5530_e4251: f64 = (1.0 + assign5530_e4250);
        let assign5530_e4252: f64 = (p.p260 * assign5530_e4251);
        (assign5530_e4252,)
    } else {
        (locals.var_fbet1e,)
    }
};
        locals.var_fbet1e = assign5530_e4254;
        locals.var_fbet1e_rv = 0.0;

        let (assign5540_e4273,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5540_e4260: f64 = (p.p263 * locals.var_iwe);
        let assign5540_e4261: f64 = (1.0 + assign5540_e4260);
        let (assign5540_e4270,) = {
            if (assign5540_e4261 > 0.001) {
                let assign5540_e4267: f64 = (p.p263 * locals.var_iwe);
                let assign5540_e4268: f64 = (1.0 + assign5540_e4267);
                (assign5540_e4268,)
            } else {
                (0.001,)
            }
        };
        let assign5540_e4271: f64 = (p.p262 * assign5540_e4270);
        (assign5540_e4271,)
    } else {
        (locals.var_lp1e,)
    }
};
        locals.var_lp1e = assign5540_e4273;
        locals.var_lp1e_rv = 0.0;

        let (assign5550_e4305,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5550_e4278: f64 = (locals.var_fbet1e * locals.var_lp1e);
        let assign5550_e4280: f64 = (assign5550_e4278 / locals.var_le);
        let assign5550_e4283: f64 = (-locals.var_le);
        let assign5550_e4285: f64 = (assign5550_e4283 / locals.var_lp1e);
        let assign5550_e4286: f64 = (assign5550_e4285).exp();
        let assign5550_e4287: f64 = (1.0 - assign5550_e4286);
        let assign5550_e4288: f64 = (assign5550_e4280 * assign5550_e4287);
        let assign5550_e4289: f64 = (1.0 + assign5550_e4288);
        let assign5550_e4292: f64 = (p.p264 * p.p265);
        let assign5550_e4294: f64 = (assign5550_e4292 / locals.var_le);
        let assign5550_e4297: f64 = (-locals.var_le);
        let assign5550_e4299: f64 = (assign5550_e4297 / p.p265);
        let assign5550_e4300: f64 = (assign5550_e4299).exp();
        let assign5550_e4301: f64 = (1.0 - assign5550_e4300);
        let assign5550_e4302: f64 = (assign5550_e4294 * assign5550_e4301);
        let assign5550_e4303: f64 = (assign5550_e4289 + assign5550_e4302);
        (assign5550_e4303,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5550_e4305;
        locals.var_gpe_rv = 0.0;

        let (assign5560_e4314,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign5560_e4312,) = {
            if (locals.var_gpe > 1e-15) {
                (locals.var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5560_e4312,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5560_e4314;
        locals.var_gpe_rv = 0.0;

        let (assign5570_e4333,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5570_e4319: f64 = (p.p266 * locals.var_iwe);
        let assign5570_e4320: f64 = (1.0 + assign5570_e4319);
        let assign5570_e4323: f64 = (p.p267 * locals.var_iwe);
        let assign5570_e4327: f64 = (locals.var_we / p.p268);
        let assign5570_e4328: f64 = (1.0 + assign5570_e4327);
        let assign5570_e4329: f64 = (assign5570_e4328).ln();
        let assign5570_e4330: f64 = (assign5570_e4323 * assign5570_e4329);
        let assign5570_e4331: f64 = (assign5570_e4320 + assign5570_e4330);
        (assign5570_e4331,)
    } else {
        (locals.var_gwe,)
    }
};
        locals.var_gwe = assign5570_e4333;
        locals.var_gwe_rv = 0.0;

        let (assign5580_e4345,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5580_e4337: f64 = (p.p259 * locals.var_we);
        let assign5580_e4340: f64 = (locals.var_gpe * locals.var_le);
        let assign5580_e4341: f64 = (assign5580_e4337 / assign5580_e4340);
        let assign5580_e4343: f64 = (assign5580_e4341 * locals.var_gwe);
        (assign5580_e4343,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign5580_e4345;
        locals.var_betn_p_rv = 0.0;

        let (assign5590_e4361,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5590_e4350: f64 = (p.p270 * locals.var_ile);
        let assign5590_e4351: f64 = (p.p269 + assign5590_e4350);
        let assign5590_e4354: f64 = (p.p271 * locals.var_iwe);
        let assign5590_e4355: f64 = (assign5590_e4351 + assign5590_e4354);
        let assign5590_e4358: f64 = (p.p272 * locals.var_iae);
        let assign5590_e4359: f64 = (assign5590_e4355 + assign5590_e4358);
        (assign5590_e4359,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign5590_e4361;
        locals.var_stbet_p_rv = 0.0;

        let (assign5600_e4371,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5600_e4367: f64 = (p.p274 * locals.var_iwe);
        let assign5600_e4368: f64 = (1.0 + assign5600_e4367);
        let assign5600_e4369: f64 = (p.p273 * assign5600_e4368);
        (assign5600_e4369,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign5600_e4371;
        locals.var_mue_p_rv = 0.0;

        let (assign5610_e4375,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p275,)
    } else {
        (locals.var_stmue_p,)
    }
};
        locals.var_stmue_p = assign5610_e4375;
        locals.var_stmue_p_rv = 0.0;

        let (assign5620_e4379,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign5620_e4379;
        locals.var_themu_p_rv = 0.0;

        let (assign5630_e4383,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p277,)
    } else {
        (locals.var_stthemu_p,)
    }
};
        locals.var_stthemu_p = assign5630_e4383;
        locals.var_stthemu_p_rv = 0.0;

        let (assign5640_e4405,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5640_e4389: f64 = (locals.var_ile).powf(p.p280);
        let assign5640_e4390: f64 = (p.p279 * assign5640_e4389);
        let assign5640_e4391: f64 = (p.p278 + assign5640_e4390);
        let assign5640_e4395: f64 = (p.p281 * locals.var_iwe);
        let assign5640_e4396: f64 = (1.0 + assign5640_e4395);
        let assign5640_e4397: f64 = (assign5640_e4391 * assign5640_e4396);
        let assign5640_e4401: f64 = (p.p282 * locals.var_iae);
        let assign5640_e4402: f64 = (1.0 + assign5640_e4401);
        let assign5640_e4403: f64 = (assign5640_e4397 * assign5640_e4402);
        (assign5640_e4403,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign5640_e4405;
        locals.var_cs_p_rv = 0.0;

        let (assign5650_e4409,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p283,)
    } else {
        (locals.var_stcs_p,)
    }
};
        locals.var_stcs_p = assign5650_e4409;
        locals.var_stcs_p_rv = 0.0;

        let (assign5660_e4413,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p284,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign5660_e4413;
        locals.var_thecs_p_rv = 0.0;

        let (assign5670_e4417,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p285,)
    } else {
        (locals.var_stthecs_p,)
    }
};
        locals.var_stthecs_p = assign5670_e4417;
        locals.var_stthecs_p_rv = 0.0;

        let (assign5680_e4439,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5680_e4423: f64 = (p.p287 * locals.var_ile);
        let assign5680_e4424: f64 = (1.0 + assign5680_e4423);
        let assign5680_e4425: f64 = (p.p286 * assign5680_e4424);
        let assign5680_e4429: f64 = (p.p288 * locals.var_iwe);
        let assign5680_e4430: f64 = (1.0 + assign5680_e4429);
        let assign5680_e4431: f64 = (assign5680_e4425 * assign5680_e4430);
        let assign5680_e4435: f64 = (p.p289 * locals.var_iae);
        let assign5680_e4436: f64 = (1.0 + assign5680_e4435);
        let assign5680_e4437: f64 = (assign5680_e4431 * assign5680_e4436);
        (assign5680_e4437,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign5680_e4439;
        locals.var_xcor_p_rv = 0.0;

        let (assign5690_e4443,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p290,)
    } else {
        (locals.var_stxcor_p,)
    }
};
        locals.var_stxcor_p = assign5690_e4443;
        locals.var_stxcor_p_rv = 0.0;

        let (assign5700_e4447,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p291,)
    } else {
        (locals.var_feta_p,)
    }
};
        locals.var_feta_p = assign5700_e4447;
        locals.var_feta_p_rv = 0.0;

        let (assign5710_e4459,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5710_e4451: f64 = (p.p292 * locals.var_iwe);
        let assign5710_e4455: f64 = (p.p293 * locals.var_iwe);
        let assign5710_e4456: f64 = (1.0 + assign5710_e4455);
        let assign5710_e4457: f64 = (assign5710_e4451 * assign5710_e4456);
        (assign5710_e4457,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign5710_e4459;
        locals.var_rs_p_rv = 0.0;

        let (assign5720_e4463,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p294,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign5720_e4463;
        locals.var_strs_p_rv = 0.0;

        let (assign5730_e4467,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p295,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign5730_e4467;
        locals.var_rsb_p_rv = 0.0;

        let (assign5740_e4471,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p296,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign5740_e4471;
        locals.var_rsg_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign5750_e4497,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5750_e4476: f64 = (p.p298 * locals.var_gwe);
        let assign5750_e4478: f64 = (assign5750_e4476 / locals.var_gpe);
        let assign5750_e4481: f64 = (locals.var_ile).powf(p.p299);
        let assign5750_e4482: f64 = (assign5750_e4478 * assign5750_e4481);
        let assign5750_e4483: f64 = (p.p297 + assign5750_e4482);
        let assign5750_e4487: f64 = (p.p300 * locals.var_iwe);
        let assign5750_e4488: f64 = (1.0 + assign5750_e4487);
        let assign5750_e4489: f64 = (assign5750_e4483 * assign5750_e4488);
        let assign5750_e4493: f64 = (p.p301 * locals.var_iae);
        let assign5750_e4494: f64 = (1.0 + assign5750_e4493);
        let assign5750_e4495: f64 = (assign5750_e4489 * assign5750_e4494);
        (assign5750_e4495,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign5750_e4497;
        locals.var_thesat_p_rv = 0.0;

        let (assign5760_e4513,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5760_e4502: f64 = (p.p303 * locals.var_ile);
        let assign5760_e4503: f64 = (p.p302 + assign5760_e4502);
        let assign5760_e4506: f64 = (p.p304 * locals.var_iwe);
        let assign5760_e4507: f64 = (assign5760_e4503 + assign5760_e4506);
        let assign5760_e4510: f64 = (p.p305 * locals.var_iae);
        let assign5760_e4511: f64 = (assign5760_e4507 + assign5760_e4510);
        (assign5760_e4511,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign5760_e4513;
        locals.var_stthesat_p_rv = 0.0;

        let (assign5770_e4517,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p306,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign5770_e4517;
        locals.var_thesatb_p_rv = 0.0;

        let (assign5780_e4521,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p307,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign5780_e4521;
        locals.var_thesatg_p_rv = 0.0;

        let (assign5790_e4525,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p308,)
    } else {
        (locals.var_thesatt_p,)
    }
};
        locals.var_thesatt_p = assign5790_e4525;
        locals.var_thesatt_p_rv = 0.0;

        let (assign5800_e4535,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5800_e4531: f64 = (p.p310 * locals.var_ile);
        let assign5800_e4532: f64 = (1.0 + assign5800_e4531);
        let assign5800_e4533: f64 = (p.p309 / assign5800_e4532);
        (assign5800_e4533,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign5800_e4535;
        locals.var_ax_p_rv = 0.0;

        let (assign5810_e4549,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5810_e4540: f64 = (locals.var_ile).powf(p.p312);
        let assign5810_e4541: f64 = (p.p311 * assign5810_e4540);
        let assign5810_e4545: f64 = (p.p313 * locals.var_iwe);
        let assign5810_e4546: f64 = (1.0 + assign5810_e4545);
        let assign5810_e4547: f64 = (assign5810_e4541 * assign5810_e4546);
        (assign5810_e4547,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign5810_e4549;
        locals.var_alp_p_rv = 0.0;

        let (assign5820_e4555,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5820_e4553: f64 = (locals.var_ile).powf(p.p315);
        (assign5820_e4553,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign5820_e4555;
        locals.var_tmpx_rv = 0.0;

        let (assign5830_e4575,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5830_e4559: f64 = (p.p314 * locals.var_tmpx);
        let assign5830_e4563: f64 = (p.p317 * locals.var_iwe);
        let assign5830_e4564: f64 = (1.0 + assign5830_e4563);
        let assign5830_e4565: f64 = (assign5830_e4559 * assign5830_e4564);
        let assign5830_e4569: f64 = (p.p316 * locals.var_ile);
        let assign5830_e4571: f64 = (assign5830_e4569 * locals.var_tmpx);
        let assign5830_e4572: f64 = (1.0 + assign5830_e4571);
        let assign5830_e4573: f64 = (assign5830_e4565 / assign5830_e4572);
        (assign5830_e4573,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign5830_e4575;
        locals.var_alp1_p_rv = 0.0;

        let (assign5840_e4581,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5840_e4579: f64 = (locals.var_ile).powf(p.p319);
        (assign5840_e4579,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign5840_e4581;
        locals.var_tmpx_rv = 0.0;

        let (assign5850_e4601,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5850_e4585: f64 = (p.p318 * locals.var_tmpx);
        let assign5850_e4589: f64 = (p.p321 * locals.var_iwe);
        let assign5850_e4590: f64 = (1.0 + assign5850_e4589);
        let assign5850_e4591: f64 = (assign5850_e4585 * assign5850_e4590);
        let assign5850_e4595: f64 = (p.p320 * locals.var_ile);
        let assign5850_e4597: f64 = (assign5850_e4595 * locals.var_tmpx);
        let assign5850_e4598: f64 = (1.0 + assign5850_e4597);
        let assign5850_e4599: f64 = (assign5850_e4591 / assign5850_e4598);
        (assign5850_e4599,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign5850_e4601;
        locals.var_alp2_p_rv = 0.0;

        let (assign5860_e4605,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p322,)
    } else {
        (locals.var_vp_p,)
    }
};
        locals.var_vp_p = assign5860_e4605;
        locals.var_vp_p_rv = 0.0;

        let (assign5870_e4621,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5870_e4611: f64 = (p.p324 * locals.var_ile);
        let assign5870_e4612: f64 = (1.0 + assign5870_e4611);
        let assign5870_e4613: f64 = (p.p323 * assign5870_e4612);
        let assign5870_e4617: f64 = (p.p325 * locals.var_iwe);
        let assign5870_e4618: f64 = (1.0 + assign5870_e4617);
        let assign5870_e4619: f64 = (assign5870_e4613 * assign5870_e4618);
        (assign5870_e4619,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign5870_e4621;
        locals.var_a1_p_rv = 0.0;

        let (assign5880_e4625,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p326,)
    } else {
        (locals.var_a2_p,)
    }
};
        locals.var_a2_p = assign5880_e4625;
        locals.var_a2_p_rv = 0.0;

        let (assign5890_e4629,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p327,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign5890_e4629;
        locals.var_sta2_p_rv = 0.0;

        let (assign5900_e4645,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5900_e4635: f64 = (p.p329 * locals.var_ile);
        let assign5900_e4636: f64 = (1.0 + assign5900_e4635);
        let assign5900_e4637: f64 = (p.p328 * assign5900_e4636);
        let assign5900_e4641: f64 = (p.p330 * locals.var_iwe);
        let assign5900_e4642: f64 = (1.0 + assign5900_e4641);
        let assign5900_e4643: f64 = (assign5900_e4637 * assign5900_e4642);
        (assign5900_e4643,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign5900_e4645;
        locals.var_a3_p_rv = 0.0;

        let (assign5910_e4661,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5910_e4651: f64 = (p.p332 * locals.var_ile);
        let assign5910_e4652: f64 = (1.0 + assign5910_e4651);
        let assign5910_e4653: f64 = (p.p331 * assign5910_e4652);
        let assign5910_e4657: f64 = (p.p333 * locals.var_iwe);
        let assign5910_e4658: f64 = (1.0 + assign5910_e4657);
        let assign5910_e4659: f64 = (assign5910_e4653 * assign5910_e4658);
        (assign5910_e4659,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign5910_e4661;
        locals.var_a4_p_rv = 0.0;

        let (assign5920_e4665,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p334,)
    } else {
        (locals.var_imaxii_p,)
    }
};
        locals.var_imaxii_p = assign5920_e4665;
        locals.var_imaxii_p_rv = 0.0;

        let (assign5930_e4669,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p335,)
    } else {
        (locals.var_gco_p,)
    }
};
        locals.var_gco_p = assign5930_e4669;
        locals.var_gco_p_rv = 0.0;

        let (assign5940_e4675,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5940_e4673: f64 = (p.p336 / locals.var_iae);
        (assign5940_e4673,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign5940_e4675;
        locals.var_iginv_p_rv = 0.0;

        let (assign5950_e4685,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5950_e4679: f64 = (p.p337 * p.p237);
        let assign5950_e4682: f64 = (1e-6 * locals.var_iwe);
        let assign5950_e4683: f64 = (assign5950_e4679 / assign5950_e4682);
        (assign5950_e4683,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign5950_e4685;
        locals.var_igov_p_rv = 0.0;

        let (assign5960_e4695,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5960_e4689: f64 = (p.p338 * p.p238);
        let assign5960_e4692: f64 = (1e-6 * locals.var_iwe);
        let assign5960_e4693: f64 = (assign5960_e4689 / assign5960_e4692);
        (assign5960_e4693,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign5960_e4695;
        locals.var_igovd_p_rv = 0.0;

        let (assign5970_e4699,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p339,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign5970_e4699;
        locals.var_stig_p_rv = 0.0;

        let (assign5980_e4703,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p340,)
    } else {
        (locals.var_gc2_p,)
    }
};
        locals.var_gc2_p = assign5980_e4703;
        locals.var_gc2_p_rv = 0.0;

        let (assign5990_e4707,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p341,)
    } else {
        (locals.var_gc3_p,)
    }
};
        locals.var_gc3_p = assign5990_e4707;
        locals.var_gc3_p_rv = 0.0;

        let (assign6000_e4711,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p340,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6000_e4711;
        locals.var_gc2ov_p_rv = 0.0;

        let assign6010_e4713: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6010_e4715: f64 = if assign6010_e4713 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign6010_e4715;
        locals.var_guard39_rv = 0.0;

        let (assign6020_e4721,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard39 != 0.0)) {
        (p.p342,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6020_e4721;
        locals.var_gc2ov_p_rv = 0.0;

        let (assign6030_e4725,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p341,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6030_e4725;
        locals.var_gc3ov_p_rv = 0.0;

        let assign6040_e4727: f64 = if param_given[343] { 1.0 } else { 0.0 };
        let assign6040_e4729: f64 = if assign6040_e4727 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign6040_e4729;
        locals.var_guard40_rv = 0.0;

        let (assign6050_e4735,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard40 != 0.0)) {
        (p.p343,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6050_e4735;
        locals.var_gc3ov_p_rv = 0.0;

        let (assign6060_e4739,) = {
    if (locals.var_guard36 != 0.0) {
        (locals.var_gc2ov_p,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6060_e4739;
        locals.var_gc2ovd_p_rv = 0.0;

        let assign6070_e4741: f64 = if param_given[344] { 1.0 } else { 0.0 };
        let assign6070_e4743: f64 = if assign6070_e4741 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign6070_e4743;
        locals.var_guard41_rv = 0.0;

        let (assign6080_e4749,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard41 != 0.0)) {
        (p.p344,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6080_e4749;
        locals.var_gc2ovd_p_rv = 0.0;

        let (assign6090_e4753,) = {
    if (locals.var_guard36 != 0.0) {
        (locals.var_gc3ov_p,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6090_e4753;
        locals.var_gc3ovd_p_rv = 0.0;

        let assign6100_e4755: f64 = if param_given[345] { 1.0 } else { 0.0 };
        let assign6100_e4757: f64 = if assign6100_e4755 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign6100_e4757;
        locals.var_guard42_rv = 0.0;

        let (assign6110_e4763,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard42 != 0.0)) {
        (p.p345,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6110_e4763;
        locals.var_gc3ovd_p_rv = 0.0;

        let (assign6120_e4767,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p346,)
    } else {
        (locals.var_chib_p,)
    }
};
        locals.var_chib_p = assign6120_e4767;
        locals.var_chib_p_rv = 0.0;

        let (assign6130_e4777,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6130_e4771: f64 = (p.p347 * p.p237);
        let assign6130_e4774: f64 = (1e-6 * locals.var_iwe);
        let assign6130_e4775: f64 = (assign6130_e4771 / assign6130_e4774);
        (assign6130_e4775,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign6130_e4777;
        locals.var_agidl_p_rv = 0.0;

        let (assign6140_e4787,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6140_e4781: f64 = (p.p348 * p.p238);
        let assign6140_e4784: f64 = (1e-6 * locals.var_iwe);
        let assign6140_e4785: f64 = (assign6140_e4781 / assign6140_e4784);
        (assign6140_e4785,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign6140_e4787;
        locals.var_agidld_p_rv = 0.0;

        let (assign6150_e4791,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p349,)
    } else {
        (locals.var_bgidl_p,)
    }
};
        locals.var_bgidl_p = assign6150_e4791;
        locals.var_bgidl_p_rv = 0.0;

        let (assign6160_e4795,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p350,)
    } else {
        (locals.var_bgidld_p,)
    }
};
        locals.var_bgidld_p = assign6160_e4795;
        locals.var_bgidld_p_rv = 0.0;

        let (assign6170_e4799,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p351,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign6170_e4799;
        locals.var_stbgidl_p_rv = 0.0;

        let (assign6180_e4803,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p352,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign6180_e4803;
        locals.var_stbgidld_p_rv = 0.0;

        let (assign6190_e4807,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p353,)
    } else {
        (locals.var_cgidl_p,)
    }
};
        locals.var_cgidl_p = assign6190_e4807;
        locals.var_cgidl_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign6200_e4811,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p354,)
    } else {
        (locals.var_cgidld_p,)
    }
};
        locals.var_cgidld_p = assign6200_e4811;
        locals.var_cgidld_p_rv = 0.0;

        let (assign6210_e4823,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6210_e4815: f64 = (8.8541878176e-12 * p.p210);
        let assign6210_e4817: f64 = (assign6210_e4815 * locals.var_wecv);
        let assign6210_e4819: f64 = (assign6210_e4817 * locals.var_lecv);
        let assign6210_e4821: f64 = (assign6210_e4819 / p.p209);
        (assign6210_e4821,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign6210_e4823;
        locals.var_cox_p_rv = 0.0;

        let (assign6220_e4835,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6220_e4827: f64 = (8.8541878176e-12 * p.p210);
        let assign6220_e4829: f64 = (assign6220_e4827 * locals.var_wecv);
        let assign6220_e4831: f64 = (assign6220_e4829 * p.p237);
        let assign6220_e4833: f64 = (assign6220_e4831 / p.p235);
        (assign6220_e4833,)
    } else {
        (locals.var_cgov_p,)
    }
};
        locals.var_cgov_p = assign6220_e4835;
        locals.var_cgov_p_rv = 0.0;

        let (assign6230_e4847,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6230_e4839: f64 = (8.8541878176e-12 * p.p210);
        let assign6230_e4841: f64 = (assign6230_e4839 * locals.var_wecv);
        let assign6230_e4843: f64 = (assign6230_e4841 * p.p238);
        let assign6230_e4845: f64 = (assign6230_e4843 / p.p236);
        (assign6230_e4845,)
    } else {
        (locals.var_cgovd_p,)
    }
};
        locals.var_cgovd_p = assign6230_e4847;
        locals.var_cgovd_p_rv = 0.0;

        let (assign6240_e4865,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6240_e4853: f64 = (locals.var_ile).powf(p.p357);
        let assign6240_e4854: f64 = (p.p356 * assign6240_e4853);
        let assign6240_e4855: f64 = (p.p355 + assign6240_e4854);
        let assign6240_e4858: f64 = (p.p358 * locals.var_iwe);
        let assign6240_e4859: f64 = (assign6240_e4855 + assign6240_e4858);
        let assign6240_e4862: f64 = (p.p359 * locals.var_iae);
        let assign6240_e4863: f64 = (assign6240_e4859 + assign6240_e4862);
        (assign6240_e4863,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign6240_e4865;
        locals.var_delvtac_p_rv = 0.0;

        let (assign6250_e4881,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6250_e4870: f64 = (p.p361 * locals.var_ile);
        let assign6250_e4871: f64 = (p.p360 + assign6250_e4870);
        let assign6250_e4874: f64 = (p.p362 * locals.var_iwe);
        let assign6250_e4875: f64 = (assign6250_e4871 + assign6250_e4874);
        let assign6250_e4878: f64 = (p.p363 * locals.var_iae);
        let assign6250_e4879: f64 = (assign6250_e4875 + assign6250_e4878);
        (assign6250_e4879,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign6250_e4881;
        locals.var_facneffac_p_rv = 0.0;

        let (assign6260_e4885,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p297,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6260_e4885;
        locals.var_thesataco_i_rv = 0.0;

        let assign6270_e4887: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6270_e4889: f64 = if assign6270_e4887 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign6270_e4889;
        locals.var_guard43_rv = 0.0;

        let (assign6280_e4895,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard43 != 0.0)) {
        (p.p364,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6280_e4895;
        locals.var_thesataco_i_rv = 0.0;

        let (assign6290_e4899,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p298,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6290_e4899;
        locals.var_thesatacl_i_rv = 0.0;

        let assign6300_e4901: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6300_e4903: f64 = if assign6300_e4901 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign6300_e4903;
        locals.var_guard44_rv = 0.0;

        let (assign6310_e4909,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard44 != 0.0)) {
        (p.p365,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6310_e4909;
        locals.var_thesatacl_i_rv = 0.0;

        let (assign6320_e4913,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p299,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6320_e4913;
        locals.var_thesataclexp_i_rv = 0.0;

        let assign6330_e4915: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6330_e4917: f64 = if assign6330_e4915 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign6330_e4917;
        locals.var_guard45_rv = 0.0;

        let (assign6340_e4923,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard45 != 0.0)) {
        (p.p366,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6340_e4923;
        locals.var_thesataclexp_i_rv = 0.0;

        let (assign6350_e4927,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p300,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6350_e4927;
        locals.var_thesatacw_i_rv = 0.0;

        let assign6360_e4929: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6360_e4931: f64 = if assign6360_e4929 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign6360_e4931;
        locals.var_guard46_rv = 0.0;

        let (assign6370_e4937,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard46 != 0.0)) {
        (p.p367,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6370_e4937;
        locals.var_thesatacw_i_rv = 0.0;

        let (assign6380_e4941,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p301,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6380_e4941;
        locals.var_thesataclw_i_rv = 0.0;

        let assign6390_e4943: f64 = if param_given[368] { 1.0 } else { 0.0 };
        let assign6390_e4945: f64 = if assign6390_e4943 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign6390_e4945;
        locals.var_guard47_rv = 0.0;

        let (assign6400_e4951,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard47 != 0.0)) {
        (p.p368,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6400_e4951;
        locals.var_thesataclw_i_rv = 0.0;

        let (assign6410_e4977,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6410_e4956: f64 = (locals.var_thesatacl_i * locals.var_gwe);
        let assign6410_e4958: f64 = (assign6410_e4956 / locals.var_gpe);
        let assign6410_e4961: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
        let assign6410_e4962: f64 = (assign6410_e4958 * assign6410_e4961);
        let assign6410_e4963: f64 = (locals.var_thesataco_i + assign6410_e4962);
        let assign6410_e4967: f64 = (locals.var_thesatacw_i * locals.var_iwe);
        let assign6410_e4968: f64 = (1.0 + assign6410_e4967);
        let assign6410_e4969: f64 = (assign6410_e4963 * assign6410_e4968);
        let assign6410_e4973: f64 = (locals.var_thesataclw_i * locals.var_iae);
        let assign6410_e4974: f64 = (1.0 + assign6410_e4973);
        let assign6410_e4975: f64 = (assign6410_e4969 * assign6410_e4974);
        (assign6410_e4975,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign6410_e4977;
        locals.var_thesatac_p_rv = 0.0;

        let (assign6420_e4981,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p309,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6420_e4981;
        locals.var_axaco_i_rv = 0.0;

        let assign6430_e4983: f64 = if param_given[369] { 1.0 } else { 0.0 };
        let assign6430_e4985: f64 = if assign6430_e4983 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign6430_e4985;
        locals.var_guard48_rv = 0.0;

        let (assign6440_e4991,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard48 != 0.0)) {
        (p.p369,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6440_e4991;
        locals.var_axaco_i_rv = 0.0;

        let (assign6450_e4995,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p310,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6450_e4995;
        locals.var_axacl_i_rv = 0.0;

        let assign6460_e4997: f64 = if param_given[370] { 1.0 } else { 0.0 };
        let assign6460_e4999: f64 = if assign6460_e4997 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign6460_e4999;
        locals.var_guard49_rv = 0.0;

        let (assign6470_e5005,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard49 != 0.0)) {
        (p.p370,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6470_e5005;
        locals.var_axacl_i_rv = 0.0;

        let (assign6480_e5015,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6480_e5011: f64 = (locals.var_axacl_i * locals.var_ile);
        let assign6480_e5012: f64 = (1.0 + assign6480_e5011);
        let assign6480_e5013: f64 = (locals.var_axaco_i / assign6480_e5012);
        (assign6480_e5013,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign6480_e5015;
        locals.var_axac_p_rv = 0.0;

        let (assign6490_e5029,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6490_e5020: f64 = (locals.var_ile).powf(p.p372);
        let assign6490_e5021: f64 = (p.p371 * assign6490_e5020);
        let assign6490_e5025: f64 = (p.p373 * locals.var_iwe);
        let assign6490_e5026: f64 = (1.0 + assign6490_e5025);
        let assign6490_e5027: f64 = (assign6490_e5021 * assign6490_e5026);
        (assign6490_e5027,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign6490_e5029;
        locals.var_alpac_p_rv = 0.0;

        let (assign6500_e5035,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6500_e5033: f64 = (locals.var_ile).powf(p.p375);
        (assign6500_e5033,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign6500_e5035;
        locals.var_tmpx_rv = 0.0;

        let (assign6510_e5055,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6510_e5039: f64 = (p.p374 * locals.var_tmpx);
        let assign6510_e5043: f64 = (p.p377 * locals.var_iwe);
        let assign6510_e5044: f64 = (1.0 + assign6510_e5043);
        let assign6510_e5045: f64 = (assign6510_e5039 * assign6510_e5044);
        let assign6510_e5049: f64 = (p.p376 * locals.var_ile);
        let assign6510_e5051: f64 = (assign6510_e5049 * locals.var_tmpx);
        let assign6510_e5052: f64 = (1.0 + assign6510_e5051);
        let assign6510_e5053: f64 = (assign6510_e5045 / assign6510_e5052);
        (assign6510_e5053,)
    } else {
        (locals.var_alp1ac_p,)
    }
};
        locals.var_alp1ac_p = assign6510_e5055;
        locals.var_alp1ac_p_rv = 0.0;

        let (assign6520_e5059,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p378,)
    } else {
        (locals.var_fcgovacc_p,)
    }
};
        locals.var_fcgovacc_p = assign6520_e5059;
        locals.var_fcgovacc_p_rv = 0.0;

        let (assign6530_e5063,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p379,)
    } else {
        (locals.var_fcgovaccd_p,)
    }
};
        locals.var_fcgovaccd_p = assign6530_e5063;
        locals.var_fcgovaccd_p_rv = 0.0;

        let (assign6540_e5067,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p380,)
    } else {
        (locals.var_cgovaccg_p,)
    }
};
        locals.var_cgovaccg_p = assign6540_e5067;
        locals.var_cgovaccg_p_rv = 0.0;

        let (assign6550_e5073,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6550_e5071: f64 = (p.p381 * locals.var_iilcv);
        (assign6550_e5071,)
    } else {
        (locals.var_cgbov_p,)
    }
};
        locals.var_cgbov_p = assign6550_e5073;
        locals.var_cgbov_p_rv = 0.0;

        let (assign6560_e5079,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6560_e5077: f64 = (p.p382 * locals.var_iiwecv);
        (assign6560_e5077,)
    } else {
        (locals.var_cinr_p,)
    }
};
        locals.var_cinr_p = assign6560_e5079;
        locals.var_cinr_p_rv = 0.0;

        let (assign6570_e5085,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6570_e5083: f64 = (p.p383 * locals.var_iiwecv);
        (assign6570_e5083,)
    } else {
        (locals.var_cinrd_p,)
    }
};
        locals.var_cinrd_p = assign6570_e5085;
        locals.var_cinrd_p_rv = 0.0;

        let (assign6580_e5089,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p384,)
    } else {
        (locals.var_dvfbinr_p,)
    }
};
        locals.var_dvfbinr_p = assign6580_e5089;
        locals.var_dvfbinr_p_rv = 0.0;

        let (assign6590_e5093,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p385,)
    } else {
        (locals.var_fcinrdep_p,)
    }
};
        locals.var_fcinrdep_p = assign6590_e5093;
        locals.var_fcinrdep_p_rv = 0.0;

        let (assign6600_e5097,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p386,)
    } else {
        (locals.var_fcinracc_p,)
    }
};
        locals.var_fcinracc_p = assign6600_e5097;
        locals.var_fcinracc_p_rv = 0.0;

        let (assign6610_e5101,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p387,)
    } else {
        (locals.var_axinr_p,)
    }
};
        locals.var_axinr_p = assign6610_e5101;
        locals.var_axinr_p_rv = 0.0;

        let (assign6640_e5123,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6640_e5118: f64 = (2.0 * p.p396);
        let assign6640_e5120: f64 = (assign6640_e5118 / locals.var_le);
        let assign6640_e5121: f64 = (1.0 - assign6640_e5120);
        (assign6640_e5121,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign6640_e5123;
        locals.var_temp0_rv = 0.0;

        let (assign6670_e5144,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p390,)
    } else {
        (locals.var_fnt_p,)
    }
};
        locals.var_fnt_p = assign6670_e5144;
        locals.var_fnt_p_rv = 0.0;

        let (assign6730_e5194,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6730_e5188: f64 = (2.0 * p.p398);
        let assign6730_e5191: f64 = (p.p399 * locals.var_we);
        let assign6730_e5192: f64 = (assign6730_e5188 + assign6730_e5191);
        (assign6730_e5192,)
    } else {
        (locals.var_we_edge,)
    }
};
        locals.var_we_edge = assign6730_e5194;
        locals.var_we_edge_rv = 0.0;

        let (assign6760_e5210,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p400,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign6760_e5210;
        locals.var_vfbedge_p_rv = 0.0;

        let (assign6770_e5226,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6770_e5215: f64 = (p.p402 * locals.var_ile);
        let assign6770_e5216: f64 = (p.p401 + assign6770_e5215);
        let assign6770_e5219: f64 = (p.p403 * locals.var_iwe);
        let assign6770_e5220: f64 = (assign6770_e5216 + assign6770_e5219);
        let assign6770_e5223: f64 = (p.p404 * locals.var_iae);
        let assign6770_e5224: f64 = (assign6770_e5220 + assign6770_e5223);
        (assign6770_e5224,)
    } else {
        (locals.var_stvfbedge_p,)
    }
};
        locals.var_stvfbedge_p = assign6770_e5226;
        locals.var_stvfbedge_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign6780_e5244,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6780_e5232: f64 = (locals.var_ile).powf(p.p407);
        let assign6780_e5233: f64 = (p.p406 * assign6780_e5232);
        let assign6780_e5234: f64 = (p.p405 + assign6780_e5233);
        let assign6780_e5237: f64 = (p.p408 * locals.var_iwe);
        let assign6780_e5238: f64 = (assign6780_e5234 + assign6780_e5237);
        let assign6780_e5241: f64 = (p.p409 * locals.var_iae);
        let assign6780_e5242: f64 = (assign6780_e5238 + assign6780_e5241);
        (assign6780_e5242,)
    } else {
        (locals.var_dphibedge_p,)
    }
};
        locals.var_dphibedge_p = assign6780_e5244;
        locals.var_dphibedge_p_rv = 0.0;

        let (assign6790_e5268,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6790_e5251: f64 = (locals.var_ile).powf(p.p412);
        let assign6790_e5252: f64 = (p.p411 * assign6790_e5251);
        let assign6790_e5253: f64 = (1.0 + assign6790_e5252);
        let assign6790_e5254: f64 = (p.p410 * assign6790_e5253);
        let assign6790_e5258: f64 = (p.p413 * locals.var_iwe);
        let assign6790_e5259: f64 = (1.0 + assign6790_e5258);
        let assign6790_e5260: f64 = (assign6790_e5254 * assign6790_e5259);
        let assign6790_e5264: f64 = (p.p414 * locals.var_iae);
        let assign6790_e5265: f64 = (1.0 + assign6790_e5264);
        let assign6790_e5266: f64 = (assign6790_e5260 * assign6790_e5265);
        (assign6790_e5266,)
    } else {
        (locals.var_neffedge_p,)
    }
};
        locals.var_neffedge_p = assign6790_e5268;
        locals.var_neffedge_p_rv = 0.0;

        let (assign6800_e5278,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6800_e5274: f64 = (locals.var_ile).powf(p.p417);
        let assign6800_e5275: f64 = (p.p416 * assign6800_e5274);
        let assign6800_e5276: f64 = (p.p415 + assign6800_e5275);
        (assign6800_e5276,)
    } else {
        (locals.var_ctedge_p,)
    }
};
        locals.var_ctedge_p = assign6800_e5278;
        locals.var_ctedge_p_rv = 0.0;

        let (assign6810_e5296,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6810_e5283: f64 = (p.p418 * p.p419);
        let assign6810_e5285: f64 = (assign6810_e5283 / locals.var_le);
        let assign6810_e5288: f64 = (-locals.var_le);
        let assign6810_e5290: f64 = (assign6810_e5288 / p.p419);
        let assign6810_e5291: f64 = (assign6810_e5290).exp();
        let assign6810_e5292: f64 = (1.0 - assign6810_e5291);
        let assign6810_e5293: f64 = (assign6810_e5285 * assign6810_e5292);
        let assign6810_e5294: f64 = (1.0 + assign6810_e5293);
        (assign6810_e5294,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign6810_e5296;
        locals.var_gpe_edge_rv = 0.0;

        let (assign6820_e5305,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign6820_e5303,) = {
            if (locals.var_gpe_edge > 1e-15) {
                (locals.var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign6820_e5303,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign6820_e5305;
        locals.var_gpe_edge_rv = 0.0;

        let (assign6830_e5321,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6830_e5309: f64 = (p.p259 * locals.var_we_edge);
        let assign6830_e5312: f64 = (locals.var_gpe_edge * locals.var_le);
        let assign6830_e5313: f64 = (assign6830_e5309 / assign6830_e5312);
        let assign6830_e5317: f64 = (p.p420 * locals.var_iwe);
        let assign6830_e5318: f64 = (1.0 + assign6830_e5317);
        let assign6830_e5319: f64 = (assign6830_e5313 * assign6830_e5318);
        (assign6830_e5319,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign6830_e5321;
        locals.var_betnedge_p_rv = 0.0;

        let (assign6840_e5337,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6840_e5326: f64 = (p.p422 * locals.var_ile);
        let assign6840_e5327: f64 = (p.p421 + assign6840_e5326);
        let assign6840_e5330: f64 = (p.p423 * locals.var_iwe);
        let assign6840_e5331: f64 = (assign6840_e5327 + assign6840_e5330);
        let assign6840_e5334: f64 = (p.p424 * locals.var_iae);
        let assign6840_e5335: f64 = (assign6840_e5331 + assign6840_e5334);
        (assign6840_e5335,)
    } else {
        (locals.var_stbetedge_p,)
    }
};
        locals.var_stbetedge_p = assign6840_e5337;
        locals.var_stbetedge_p_rv = 0.0;

        let (assign6850_e5351,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6850_e5342: f64 = (locals.var_ile).powf(p.p426);
        let assign6850_e5343: f64 = (p.p425 * assign6850_e5342);
        let assign6850_e5347: f64 = (p.p427 * locals.var_iwe);
        let assign6850_e5348: f64 = (1.0 + assign6850_e5347);
        let assign6850_e5349: f64 = (assign6850_e5343 * assign6850_e5348);
        (assign6850_e5349,)
    } else {
        (locals.var_psceedge_p,)
    }
};
        locals.var_psceedge_p = assign6850_e5351;
        locals.var_psceedge_p_rv = 0.0;

        let (assign6860_e5355,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p428,)
    } else {
        (locals.var_pscebedge_p,)
    }
};
        locals.var_pscebedge_p = assign6860_e5355;
        locals.var_pscebedge_p_rv = 0.0;

        let (assign6870_e5359,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p429,)
    } else {
        (locals.var_pscededge_p,)
    }
};
        locals.var_pscededge_p = assign6870_e5359;
        locals.var_pscededge_p_rv = 0.0;

        let (assign6880_e5373,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6880_e5364: f64 = (locals.var_ile).powf(p.p431);
        let assign6880_e5365: f64 = (p.p430 * assign6880_e5364);
        let assign6880_e5369: f64 = (p.p432 * locals.var_iwe);
        let assign6880_e5370: f64 = (1.0 + assign6880_e5369);
        let assign6880_e5371: f64 = (assign6880_e5365 * assign6880_e5370);
        (assign6880_e5371,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign6880_e5373;
        locals.var_cfedge_p_rv = 0.0;

        let (assign6890_e5377,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p434,)
    } else {
        (locals.var_cfdedge_p,)
    }
};
        locals.var_cfdedge_p = assign6890_e5377;
        locals.var_cfdedge_p_rv = 0.0;

        let (assign6900_e5381,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p433,)
    } else {
        (locals.var_cfbedge_p,)
    }
};
        locals.var_cfbedge_p = assign6900_e5381;
        locals.var_cfbedge_p_rv = 0.0;

        let (assign6960_e5423,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6960_e5412: f64 = (p.p832 * locals.var_ile);
        let assign6960_e5413: f64 = (p.p831 + assign6960_e5412);
        let assign6960_e5416: f64 = (p.p833 * locals.var_iwe);
        let assign6960_e5417: f64 = (assign6960_e5413 + assign6960_e5416);
        let assign6960_e5420: f64 = (p.p834 * locals.var_iae);
        let assign6960_e5421: f64 = (assign6960_e5417 + assign6960_e5420);
        (assign6960_e5421,)
    } else {
        (locals.var_kvthowe,)
    }
};
        locals.var_kvthowe = assign6960_e5423;
        locals.var_kvthowe_rv = 0.0;

        let (assign6970_e5439,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6970_e5428: f64 = (p.p836 * locals.var_ile);
        let assign6970_e5429: f64 = (p.p835 + assign6970_e5428);
        let assign6970_e5432: f64 = (p.p837 * locals.var_iwe);
        let assign6970_e5433: f64 = (assign6970_e5429 + assign6970_e5432);
        let assign6970_e5436: f64 = (p.p838 * locals.var_iae);
        let assign6970_e5437: f64 = (assign6970_e5433 + assign6970_e5436);
        (assign6970_e5437,)
    } else {
        (locals.var_kuowe,)
    }
};
        locals.var_kuowe = assign6970_e5439;
        locals.var_kuowe_rv = 0.0;

        let assign7140_e5602: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };
        locals.var_guard51 = assign7140_e5602;
        locals.var_guard51_rv = 0.0;

        let (assign7150_e5620,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign7150_e5609: f64 = (p.p461 * locals.var_ile);
        let assign7150_e5610: f64 = (p.p460 + assign7150_e5609);
        let assign7150_e5613: f64 = (p.p462 * locals.var_iwe);
        let assign7150_e5614: f64 = (assign7150_e5610 + assign7150_e5613);
        let assign7150_e5617: f64 = (p.p463 * locals.var_iae);
        let assign7150_e5618: f64 = (assign7150_e5614 + assign7150_e5617);
        (assign7150_e5618,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign7150_e5620;
        locals.var_vfb_p_rv = 0.0;

        let assign7160_e5639: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };
        locals.var_guard52 = assign7160_e5639;
        locals.var_guard52_rv = 0.0;

        let (assign7170_e5657,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard52 != 0.0)) {
        let assign7170_e5646: f64 = (p.p465 * locals.var_ile);
        let assign7170_e5647: f64 = (p.p464 + assign7170_e5646);
        let assign7170_e5650: f64 = (p.p466 * locals.var_iwe);
        let assign7170_e5651: f64 = (assign7170_e5647 + assign7170_e5650);
        let assign7170_e5654: f64 = (p.p467 * locals.var_iae);
        let assign7170_e5655: f64 = (assign7170_e5651 + assign7170_e5654);
        (assign7170_e5655,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign7170_e5657;
        locals.var_stvfb_p_rv = 0.0;

        let assign7180_e5676: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };
        locals.var_guard53 = assign7180_e5676;
        locals.var_guard53_rv = 0.0;

        let (assign7190_e5694,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard53 != 0.0)) {
        let assign7190_e5683: f64 = (p.p469 * locals.var_ile);
        let assign7190_e5684: f64 = (p.p468 + assign7190_e5683);
        let assign7190_e5687: f64 = (p.p470 * locals.var_iwe);
        let assign7190_e5688: f64 = (assign7190_e5684 + assign7190_e5687);
        let assign7190_e5691: f64 = (p.p471 * locals.var_iae);
        let assign7190_e5692: f64 = (assign7190_e5688 + assign7190_e5691);
        (assign7190_e5692,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign7190_e5694;
        locals.var_neff_p_rv = 0.0;

        let assign7200_e5713: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };
        locals.var_guard54 = assign7200_e5713;
        locals.var_guard54_rv = 0.0;

        let (assign7210_e5731,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard54 != 0.0)) {
        let assign7210_e5720: f64 = (p.p473 * locals.var_ile);
        let assign7210_e5721: f64 = (p.p472 + assign7210_e5720);
        let assign7210_e5724: f64 = (p.p474 * locals.var_iwe);
        let assign7210_e5725: f64 = (assign7210_e5721 + assign7210_e5724);
        let assign7210_e5728: f64 = (p.p475 * locals.var_iae);
        let assign7210_e5729: f64 = (assign7210_e5725 + assign7210_e5728);
        (assign7210_e5729,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign7210_e5731;
        locals.var_gfacnud_p_rv = 0.0;

        let assign7220_e5750: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };
        locals.var_guard55 = assign7220_e5750;
        locals.var_guard55_rv = 0.0;

        let (assign7230_e5768,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard55 != 0.0)) {
        let assign7230_e5757: f64 = (p.p477 * locals.var_ile);
        let assign7230_e5758: f64 = (p.p476 + assign7230_e5757);
        let assign7230_e5761: f64 = (p.p478 * locals.var_iwe);
        let assign7230_e5762: f64 = (assign7230_e5758 + assign7230_e5761);
        let assign7230_e5765: f64 = (p.p479 * locals.var_iae);
        let assign7230_e5766: f64 = (assign7230_e5762 + assign7230_e5765);
        (assign7230_e5766,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign7230_e5768;
        locals.var_vsbnud_p_rv = 0.0;

        let assign7240_e5787: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };
        locals.var_guard56 = assign7240_e5787;
        locals.var_guard56_rv = 0.0;

        let (assign7250_e5805,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard56 != 0.0)) {
        let assign7250_e5794: f64 = (p.p481 * locals.var_ile);
        let assign7250_e5795: f64 = (p.p480 + assign7250_e5794);
        let assign7250_e5798: f64 = (p.p482 * locals.var_iwe);
        let assign7250_e5799: f64 = (assign7250_e5795 + assign7250_e5798);
        let assign7250_e5802: f64 = (p.p483 * locals.var_iae);
        let assign7250_e5803: f64 = (assign7250_e5799 + assign7250_e5802);
        (assign7250_e5803,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign7250_e5805;
        locals.var_dphib_p_rv = 0.0;

        let assign7260_e5824: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };
        locals.var_guard57 = assign7260_e5824;
        locals.var_guard57_rv = 0.0;

        let (assign7270_e5842,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard57 != 0.0)) {
        let assign7270_e5831: f64 = (p.p485 * locals.var_ile);
        let assign7270_e5832: f64 = (p.p484 + assign7270_e5831);
        let assign7270_e5835: f64 = (p.p486 * locals.var_iwe);
        let assign7270_e5836: f64 = (assign7270_e5832 + assign7270_e5835);
        let assign7270_e5839: f64 = (p.p487 * locals.var_iae);
        let assign7270_e5840: f64 = (assign7270_e5836 + assign7270_e5839);
        (assign7270_e5840,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign7270_e5842;
        locals.var_np_p_rv = 0.0;

        let assign7280_e5861: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };
        locals.var_guard58 = assign7280_e5861;
        locals.var_guard58_rv = 0.0;

        let (assign7290_e5879,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard58 != 0.0)) {
        let assign7290_e5868: f64 = (p.p489 * locals.var_ile);
        let assign7290_e5869: f64 = (p.p488 + assign7290_e5868);
        let assign7290_e5872: f64 = (p.p490 * locals.var_iwe);
        let assign7290_e5873: f64 = (assign7290_e5869 + assign7290_e5872);
        let assign7290_e5876: f64 = (p.p491 * locals.var_iae);
        let assign7290_e5877: f64 = (assign7290_e5873 + assign7290_e5876);
        (assign7290_e5877,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign7290_e5879;
        locals.var_nov_p_rv = 0.0;

        let assign7300_e5898: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };
        locals.var_guard59 = assign7300_e5898;
        locals.var_guard59_rv = 0.0;

        let (assign7310_e5916,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard59 != 0.0)) {
        let assign7310_e5905: f64 = (p.p493 * locals.var_ile);
        let assign7310_e5906: f64 = (p.p492 + assign7310_e5905);
        let assign7310_e5909: f64 = (p.p494 * locals.var_iwe);
        let assign7310_e5910: f64 = (assign7310_e5906 + assign7310_e5909);
        let assign7310_e5913: f64 = (p.p495 * locals.var_iae);
        let assign7310_e5914: f64 = (assign7310_e5910 + assign7310_e5913);
        (assign7310_e5914,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign7310_e5916;
        locals.var_novd_p_rv = 0.0;

        let assign7320_e5935: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };
        locals.var_guard60 = assign7320_e5935;
        locals.var_guard60_rv = 0.0;

        let (assign7330_e5953,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign7330_e5942: f64 = (p.p497 * locals.var_ile);
        let assign7330_e5943: f64 = (p.p496 + assign7330_e5942);
        let assign7330_e5946: f64 = (p.p498 * locals.var_iwe);
        let assign7330_e5947: f64 = (assign7330_e5943 + assign7330_e5946);
        let assign7330_e5950: f64 = (p.p499 * locals.var_iae);
        let assign7330_e5951: f64 = (assign7330_e5947 + assign7330_e5950);
        (assign7330_e5951,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign7330_e5953;
        locals.var_ct_p_rv = 0.0;

        let assign7340_e5972: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };
        locals.var_guard61 = assign7340_e5972;
        locals.var_guard61_rv = 0.0;

        let (assign7350_e5990,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard61 != 0.0)) {
        let assign7350_e5979: f64 = (p.p505 * locals.var_ile);
        let assign7350_e5980: f64 = (p.p504 + assign7350_e5979);
        let assign7350_e5983: f64 = (p.p506 * locals.var_iwe);
        let assign7350_e5984: f64 = (assign7350_e5980 + assign7350_e5983);
        let assign7350_e5987: f64 = (p.p507 * locals.var_iae);
        let assign7350_e5988: f64 = (assign7350_e5984 + assign7350_e5987);
        (assign7350_e5988,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign7350_e5990;
        locals.var_ctg_p_rv = 0.0;

        let assign7360_e6009: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };
        locals.var_guard62 = assign7360_e6009;
        locals.var_guard62_rv = 0.0;

        let (assign7370_e6027,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard62 != 0.0)) {
        let assign7370_e6016: f64 = (p.p501 * locals.var_ile);
        let assign7370_e6017: f64 = (p.p500 + assign7370_e6016);
        let assign7370_e6020: f64 = (p.p502 * locals.var_iwe);
        let assign7370_e6021: f64 = (assign7370_e6017 + assign7370_e6020);
        let assign7370_e6024: f64 = (p.p503 * locals.var_iae);
        let assign7370_e6025: f64 = (assign7370_e6021 + assign7370_e6024);
        (assign7370_e6025,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign7370_e6027;
        locals.var_ctb_p_rv = 0.0;

        let assign7380_e6046: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };
        locals.var_guard63 = assign7380_e6046;
        locals.var_guard63_rv = 0.0;

        let (assign7390_e6064,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard63 != 0.0)) {
        let assign7390_e6053: f64 = (p.p509 * locals.var_ile);
        let assign7390_e6054: f64 = (p.p508 + assign7390_e6053);
        let assign7390_e6057: f64 = (p.p510 * locals.var_iwe);
        let assign7390_e6058: f64 = (assign7390_e6054 + assign7390_e6057);
        let assign7390_e6061: f64 = (p.p511 * locals.var_iae);
        let assign7390_e6062: f64 = (assign7390_e6058 + assign7390_e6061);
        (assign7390_e6062,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign7390_e6064;
        locals.var_stct_p_rv = 0.0;

        let assign7400_e6083: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };
        locals.var_guard64 = assign7400_e6083;
        locals.var_guard64_rv = 0.0;

        let (assign7410_e6103,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard64 != 0.0)) {
        let assign7410_e6091: f64 = (p.p513 * locals.var_ile);
        let assign7410_e6092: f64 = (p.p512 + assign7410_e6091);
        let assign7410_e6095: f64 = (p.p514 * locals.var_iwe);
        let assign7410_e6096: f64 = (assign7410_e6092 + assign7410_e6095);
        let assign7410_e6099: f64 = (p.p515 * locals.var_iae);
        let assign7410_e6100: f64 = (assign7410_e6096 + assign7410_e6099);
        let assign7410_e6101: f64 = (locals.var_ile2 * assign7410_e6100);
        (assign7410_e6101,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign7410_e6103;
        locals.var_cf_p_rv = 0.0;

        let assign7420_e6122: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };
        locals.var_guard65 = assign7420_e6122;
        locals.var_guard65_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7430_e6140,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard65 != 0.0)) {
        let assign7430_e6129: f64 = (p.p521 * locals.var_ile);
        let assign7430_e6130: f64 = (p.p520 + assign7430_e6129);
        let assign7430_e6133: f64 = (p.p522 * locals.var_iwe);
        let assign7430_e6134: f64 = (assign7430_e6130 + assign7430_e6133);
        let assign7430_e6137: f64 = (p.p523 * locals.var_iae);
        let assign7430_e6138: f64 = (assign7430_e6134 + assign7430_e6137);
        (assign7430_e6138,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign7430_e6140;
        locals.var_cfd_p_rv = 0.0;

        let assign7440_e6159: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };
        locals.var_guard66 = assign7440_e6159;
        locals.var_guard66_rv = 0.0;

        let (assign7450_e6177,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard66 != 0.0)) {
        let assign7450_e6166: f64 = (p.p517 * locals.var_ile);
        let assign7450_e6167: f64 = (p.p516 + assign7450_e6166);
        let assign7450_e6170: f64 = (p.p518 * locals.var_iwe);
        let assign7450_e6171: f64 = (assign7450_e6167 + assign7450_e6170);
        let assign7450_e6174: f64 = (p.p519 * locals.var_iae);
        let assign7450_e6175: f64 = (assign7450_e6171 + assign7450_e6174);
        (assign7450_e6175,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign7450_e6177;
        locals.var_cfb_p_rv = 0.0;

        let assign7460_e6196: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };
        locals.var_guard67 = assign7460_e6196;
        locals.var_guard67_rv = 0.0;

        let (assign7470_e6216,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard67 != 0.0)) {
        let assign7470_e6204: f64 = (p.p525 * locals.var_ile);
        let assign7470_e6205: f64 = (p.p524 + assign7470_e6204);
        let assign7470_e6208: f64 = (p.p526 * locals.var_iwe);
        let assign7470_e6209: f64 = (assign7470_e6205 + assign7470_e6208);
        let assign7470_e6212: f64 = (p.p527 * locals.var_iae);
        let assign7470_e6213: f64 = (assign7470_e6209 + assign7470_e6212);
        let assign7470_e6214: f64 = (locals.var_ile2 * assign7470_e6213);
        (assign7470_e6214,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign7470_e6216;
        locals.var_psce_p_rv = 0.0;

        let assign7480_e6235: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };
        locals.var_guard68 = assign7480_e6235;
        locals.var_guard68_rv = 0.0;

        let (assign7490_e6253,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard68 != 0.0)) {
        let assign7490_e6242: f64 = (p.p533 * locals.var_ile);
        let assign7490_e6243: f64 = (p.p532 + assign7490_e6242);
        let assign7490_e6246: f64 = (p.p534 * locals.var_iwe);
        let assign7490_e6247: f64 = (assign7490_e6243 + assign7490_e6246);
        let assign7490_e6250: f64 = (p.p535 * locals.var_iae);
        let assign7490_e6251: f64 = (assign7490_e6247 + assign7490_e6250);
        (assign7490_e6251,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign7490_e6253;
        locals.var_psced_p_rv = 0.0;

        let assign7500_e6272: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };
        locals.var_guard69 = assign7500_e6272;
        locals.var_guard69_rv = 0.0;

        let (assign7510_e6290,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard69 != 0.0)) {
        let assign7510_e6279: f64 = (p.p529 * locals.var_ile);
        let assign7510_e6280: f64 = (p.p528 + assign7510_e6279);
        let assign7510_e6283: f64 = (p.p530 * locals.var_iwe);
        let assign7510_e6284: f64 = (assign7510_e6280 + assign7510_e6283);
        let assign7510_e6287: f64 = (p.p531 * locals.var_iae);
        let assign7510_e6288: f64 = (assign7510_e6284 + assign7510_e6287);
        (assign7510_e6288,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign7510_e6290;
        locals.var_psceb_p_rv = 0.0;

        let assign7520_e6309: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };
        locals.var_guard70 = assign7520_e6309;
        locals.var_guard70_rv = 0.0;

        let (assign7530_e6331,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard70 != 0.0)) {
        let assign7530_e6315: f64 = (locals.var_we / locals.var_le);
        let assign7530_e6319: f64 = (p.p537 * locals.var_ile);
        let assign7530_e6320: f64 = (p.p536 + assign7530_e6319);
        let assign7530_e6323: f64 = (p.p538 * locals.var_iwe);
        let assign7530_e6324: f64 = (assign7530_e6320 + assign7530_e6323);
        let assign7530_e6327: f64 = (p.p539 * locals.var_iae);
        let assign7530_e6328: f64 = (assign7530_e6324 + assign7530_e6327);
        let assign7530_e6329: f64 = (assign7530_e6315 * assign7530_e6328);
        (assign7530_e6329,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign7530_e6331;
        locals.var_betn_p_rv = 0.0;

        let assign7540_e6350: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };
        locals.var_guard71 = assign7540_e6350;
        locals.var_guard71_rv = 0.0;

        let (assign7550_e6368,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard71 != 0.0)) {
        let assign7550_e6357: f64 = (p.p541 * locals.var_ile);
        let assign7550_e6358: f64 = (p.p540 + assign7550_e6357);
        let assign7550_e6361: f64 = (p.p542 * locals.var_iwe);
        let assign7550_e6362: f64 = (assign7550_e6358 + assign7550_e6361);
        let assign7550_e6365: f64 = (p.p543 * locals.var_iae);
        let assign7550_e6366: f64 = (assign7550_e6362 + assign7550_e6365);
        (assign7550_e6366,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign7550_e6368;
        locals.var_stbet_p_rv = 0.0;

        let assign7560_e6387: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };
        locals.var_guard72 = assign7560_e6387;
        locals.var_guard72_rv = 0.0;

        let (assign7570_e6405,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard72 != 0.0)) {
        let assign7570_e6394: f64 = (p.p545 * locals.var_ile);
        let assign7570_e6395: f64 = (p.p544 + assign7570_e6394);
        let assign7570_e6398: f64 = (p.p546 * locals.var_iwe);
        let assign7570_e6399: f64 = (assign7570_e6395 + assign7570_e6398);
        let assign7570_e6402: f64 = (p.p547 * locals.var_iae);
        let assign7570_e6403: f64 = (assign7570_e6399 + assign7570_e6402);
        (assign7570_e6403,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign7570_e6405;
        locals.var_mue_p_rv = 0.0;

        let assign7580_e6424: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };
        locals.var_guard73 = assign7580_e6424;
        locals.var_guard73_rv = 0.0;

        let (assign7590_e6442,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign7590_e6431: f64 = (p.p549 * locals.var_ile);
        let assign7590_e6432: f64 = (p.p548 + assign7590_e6431);
        let assign7590_e6435: f64 = (p.p550 * locals.var_iwe);
        let assign7590_e6436: f64 = (assign7590_e6432 + assign7590_e6435);
        let assign7590_e6439: f64 = (p.p551 * locals.var_iae);
        let assign7590_e6440: f64 = (assign7590_e6436 + assign7590_e6439);
        (assign7590_e6440,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign7590_e6442;
        locals.var_themu_p_rv = 0.0;

        let assign7600_e6461: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign7600_e6461;
        locals.var_guard74_rv = 0.0;

        let (assign7610_e6479,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard74 != 0.0)) {
        let assign7610_e6468: f64 = (p.p553 * locals.var_ile);
        let assign7610_e6469: f64 = (p.p552 + assign7610_e6468);
        let assign7610_e6472: f64 = (p.p554 * locals.var_iwe);
        let assign7610_e6473: f64 = (assign7610_e6469 + assign7610_e6472);
        let assign7610_e6476: f64 = (p.p555 * locals.var_iae);
        let assign7610_e6477: f64 = (assign7610_e6473 + assign7610_e6476);
        (assign7610_e6477,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign7610_e6479;
        locals.var_cs_p_rv = 0.0;

        let assign7620_e6498: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign7620_e6498;
        locals.var_guard75_rv = 0.0;

        let (assign7630_e6516,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign7630_e6505: f64 = (p.p557 * locals.var_ile);
        let assign7630_e6506: f64 = (p.p556 + assign7630_e6505);
        let assign7630_e6509: f64 = (p.p558 * locals.var_iwe);
        let assign7630_e6510: f64 = (assign7630_e6506 + assign7630_e6509);
        let assign7630_e6513: f64 = (p.p559 * locals.var_iae);
        let assign7630_e6514: f64 = (assign7630_e6510 + assign7630_e6513);
        (assign7630_e6514,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign7630_e6516;
        locals.var_thecs_p_rv = 0.0;

        let assign7640_e6535: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign7640_e6535;
        locals.var_guard76_rv = 0.0;

        let (assign7650_e6553,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign7650_e6542: f64 = (p.p561 * locals.var_ile);
        let assign7650_e6543: f64 = (p.p560 + assign7650_e6542);
        let assign7650_e6546: f64 = (p.p562 * locals.var_iwe);
        let assign7650_e6547: f64 = (assign7650_e6543 + assign7650_e6546);
        let assign7650_e6550: f64 = (p.p563 * locals.var_iae);
        let assign7650_e6551: f64 = (assign7650_e6547 + assign7650_e6550);
        (assign7650_e6551,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign7650_e6553;
        locals.var_xcor_p_rv = 0.0;

        let assign7660_e6572: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign7660_e6572;
        locals.var_guard77_rv = 0.0;

        let (assign7670_e6592,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard77 != 0.0)) {
        let assign7670_e6580: f64 = (p.p565 * locals.var_ile);
        let assign7670_e6581: f64 = (p.p564 + assign7670_e6580);
        let assign7670_e6584: f64 = (p.p566 * locals.var_iwe);
        let assign7670_e6585: f64 = (assign7670_e6581 + assign7670_e6584);
        let assign7670_e6588: f64 = (p.p567 * locals.var_iae);
        let assign7670_e6589: f64 = (assign7670_e6585 + assign7670_e6588);
        let assign7670_e6590: f64 = (locals.var_iwe * assign7670_e6589);
        (assign7670_e6590,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign7670_e6592;
        locals.var_rs_p_rv = 0.0;

        let assign7680_e6611: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign7680_e6611;
        locals.var_guard78_rv = 0.0;

        let (assign7690_e6629,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign7690_e6618: f64 = (p.p569 * locals.var_ile);
        let assign7690_e6619: f64 = (p.p568 + assign7690_e6618);
        let assign7690_e6622: f64 = (p.p570 * locals.var_iwe);
        let assign7690_e6623: f64 = (assign7690_e6619 + assign7690_e6622);
        let assign7690_e6626: f64 = (p.p571 * locals.var_iae);
        let assign7690_e6627: f64 = (assign7690_e6623 + assign7690_e6626);
        (assign7690_e6627,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign7690_e6629;
        locals.var_strs_p_rv = 0.0;

        let assign7700_e6648: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };
        locals.var_guard79 = assign7700_e6648;
        locals.var_guard79_rv = 0.0;

        let (assign7710_e6666,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard79 != 0.0)) {
        let assign7710_e6655: f64 = (p.p573 * locals.var_ile);
        let assign7710_e6656: f64 = (p.p572 + assign7710_e6655);
        let assign7710_e6659: f64 = (p.p574 * locals.var_iwe);
        let assign7710_e6660: f64 = (assign7710_e6656 + assign7710_e6659);
        let assign7710_e6663: f64 = (p.p575 * locals.var_iae);
        let assign7710_e6664: f64 = (assign7710_e6660 + assign7710_e6663);
        (assign7710_e6664,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign7710_e6666;
        locals.var_rsb_p_rv = 0.0;

        let assign7720_e6685: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };
        locals.var_guard80 = assign7720_e6685;
        locals.var_guard80_rv = 0.0;

        let (assign7730_e6703,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard80 != 0.0)) {
        let assign7730_e6692: f64 = (p.p577 * locals.var_ile);
        let assign7730_e6693: f64 = (p.p576 + assign7730_e6692);
        let assign7730_e6696: f64 = (p.p578 * locals.var_iwe);
        let assign7730_e6697: f64 = (assign7730_e6693 + assign7730_e6696);
        let assign7730_e6700: f64 = (p.p579 * locals.var_iae);
        let assign7730_e6701: f64 = (assign7730_e6697 + assign7730_e6700);
        (assign7730_e6701,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign7730_e6703;
        locals.var_rsg_p_rv = 0.0;

        let assign7740_e6722: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        locals.var_guard81 = assign7740_e6722;
        locals.var_guard81_rv = 0.0;

        let (assign7750_e6742,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard81 != 0.0)) {
        let assign7750_e6730: f64 = (p.p581 * locals.var_ile);
        let assign7750_e6731: f64 = (p.p580 + assign7750_e6730);
        let assign7750_e6734: f64 = (p.p582 * locals.var_iwe);
        let assign7750_e6735: f64 = (assign7750_e6731 + assign7750_e6734);
        let assign7750_e6738: f64 = (p.p583 * locals.var_iae);
        let assign7750_e6739: f64 = (assign7750_e6735 + assign7750_e6738);
        let assign7750_e6740: f64 = (locals.var_ile * assign7750_e6739);
        (assign7750_e6740,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign7750_e6742;
        locals.var_thesat_p_rv = 0.0;

        let assign7760_e6761: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        locals.var_guard82 = assign7760_e6761;
        locals.var_guard82_rv = 0.0;

        let (assign7770_e6779,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard82 != 0.0)) {
        let assign7770_e6768: f64 = (p.p585 * locals.var_ile);
        let assign7770_e6769: f64 = (p.p584 + assign7770_e6768);
        let assign7770_e6772: f64 = (p.p586 * locals.var_iwe);
        let assign7770_e6773: f64 = (assign7770_e6769 + assign7770_e6772);
        let assign7770_e6776: f64 = (p.p587 * locals.var_iae);
        let assign7770_e6777: f64 = (assign7770_e6773 + assign7770_e6776);
        (assign7770_e6777,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign7770_e6779;
        locals.var_stthesat_p_rv = 0.0;

        let assign7780_e6798: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };
        locals.var_guard83 = assign7780_e6798;
        locals.var_guard83_rv = 0.0;

        let (assign7790_e6816,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard83 != 0.0)) {
        let assign7790_e6805: f64 = (p.p589 * locals.var_ile);
        let assign7790_e6806: f64 = (p.p588 + assign7790_e6805);
        let assign7790_e6809: f64 = (p.p590 * locals.var_iwe);
        let assign7790_e6810: f64 = (assign7790_e6806 + assign7790_e6809);
        let assign7790_e6813: f64 = (p.p591 * locals.var_iae);
        let assign7790_e6814: f64 = (assign7790_e6810 + assign7790_e6813);
        (assign7790_e6814,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign7790_e6816;
        locals.var_thesatb_p_rv = 0.0;

        let assign7800_e6835: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };
        locals.var_guard84 = assign7800_e6835;
        locals.var_guard84_rv = 0.0;

        let (assign7810_e6853,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign7810_e6842: f64 = (p.p593 * locals.var_ile);
        let assign7810_e6843: f64 = (p.p592 + assign7810_e6842);
        let assign7810_e6846: f64 = (p.p594 * locals.var_iwe);
        let assign7810_e6847: f64 = (assign7810_e6843 + assign7810_e6846);
        let assign7810_e6850: f64 = (p.p595 * locals.var_iae);
        let assign7810_e6851: f64 = (assign7810_e6847 + assign7810_e6850);
        (assign7810_e6851,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign7810_e6853;
        locals.var_thesatg_p_rv = 0.0;

        let assign7820_e6872: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        locals.var_guard85 = assign7820_e6872;
        locals.var_guard85_rv = 0.0;

        let (assign7830_e6890,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign7830_e6879: f64 = (p.p597 * locals.var_ile);
        let assign7830_e6880: f64 = (p.p596 + assign7830_e6879);
        let assign7830_e6883: f64 = (p.p598 * locals.var_iwe);
        let assign7830_e6884: f64 = (assign7830_e6880 + assign7830_e6883);
        let assign7830_e6887: f64 = (p.p599 * locals.var_iae);
        let assign7830_e6888: f64 = (assign7830_e6884 + assign7830_e6887);
        (assign7830_e6888,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign7830_e6890;
        locals.var_ax_p_rv = 0.0;

        let assign7840_e6909: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };
        locals.var_guard86 = assign7840_e6909;
        locals.var_guard86_rv = 0.0;

        let (assign7850_e6929,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign7850_e6917: f64 = (p.p601 * locals.var_ile);
        let assign7850_e6918: f64 = (p.p600 + assign7850_e6917);
        let assign7850_e6921: f64 = (p.p602 * locals.var_iwe);
        let assign7850_e6922: f64 = (assign7850_e6918 + assign7850_e6921);
        let assign7850_e6925: f64 = (p.p603 * locals.var_iae);
        let assign7850_e6926: f64 = (assign7850_e6922 + assign7850_e6925);
        let assign7850_e6927: f64 = (locals.var_ile * assign7850_e6926);
        (assign7850_e6927,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign7850_e6929;
        locals.var_alp_p_rv = 0.0;

        let assign7860_e6948: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };
        locals.var_guard87 = assign7860_e6948;
        locals.var_guard87_rv = 0.0;

        let (assign7870_e6966,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7870_e6955: f64 = (p.p605 * locals.var_ile);
        let assign7870_e6956: f64 = (p.p604 + assign7870_e6955);
        let assign7870_e6959: f64 = (p.p606 * locals.var_iwe);
        let assign7870_e6960: f64 = (assign7870_e6956 + assign7870_e6959);
        let assign7870_e6963: f64 = (p.p607 * locals.var_iae);
        let assign7870_e6964: f64 = (assign7870_e6960 + assign7870_e6963);
        (assign7870_e6964,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign7870_e6966;
        locals.var_alp1_p_rv = 0.0;

        let assign7880_e6985: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7880_e6985;
        locals.var_guard88_rv = 0.0;

        let (assign7890_e7003,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard88 != 0.0)) {
        let assign7890_e6992: f64 = (p.p609 * locals.var_ile);
        let assign7890_e6993: f64 = (p.p608 + assign7890_e6992);
        let assign7890_e6996: f64 = (p.p610 * locals.var_iwe);
        let assign7890_e6997: f64 = (assign7890_e6993 + assign7890_e6996);
        let assign7890_e7000: f64 = (p.p611 * locals.var_iae);
        let assign7890_e7001: f64 = (assign7890_e6997 + assign7890_e7000);
        (assign7890_e7001,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign7890_e7003;
        locals.var_alp2_p_rv = 0.0;

        let assign7900_e7022: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7900_e7022;
        locals.var_guard89_rv = 0.0;

        let (assign7910_e7040,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard89 != 0.0)) {
        let assign7910_e7029: f64 = (p.p613 * locals.var_ile);
        let assign7910_e7030: f64 = (p.p612 + assign7910_e7029);
        let assign7910_e7033: f64 = (p.p614 * locals.var_iwe);
        let assign7910_e7034: f64 = (assign7910_e7030 + assign7910_e7033);
        let assign7910_e7037: f64 = (p.p615 * locals.var_iae);
        let assign7910_e7038: f64 = (assign7910_e7034 + assign7910_e7037);
        (assign7910_e7038,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign7910_e7040;
        locals.var_a1_p_rv = 0.0;

        let assign7920_e7059: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };
        locals.var_guard90 = assign7920_e7059;
        locals.var_guard90_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7930_e7077,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard90 != 0.0)) {
        let assign7930_e7066: f64 = (p.p617 * locals.var_ile);
        let assign7930_e7067: f64 = (p.p616 + assign7930_e7066);
        let assign7930_e7070: f64 = (p.p618 * locals.var_iwe);
        let assign7930_e7071: f64 = (assign7930_e7067 + assign7930_e7070);
        let assign7930_e7074: f64 = (p.p619 * locals.var_iae);
        let assign7930_e7075: f64 = (assign7930_e7071 + assign7930_e7074);
        (assign7930_e7075,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign7930_e7077;
        locals.var_sta2_p_rv = 0.0;

        let assign7940_e7096: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };
        locals.var_guard91 = assign7940_e7096;
        locals.var_guard91_rv = 0.0;

        let (assign7950_e7114,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard91 != 0.0)) {
        let assign7950_e7103: f64 = (p.p621 * locals.var_ile);
        let assign7950_e7104: f64 = (p.p620 + assign7950_e7103);
        let assign7950_e7107: f64 = (p.p622 * locals.var_iwe);
        let assign7950_e7108: f64 = (assign7950_e7104 + assign7950_e7107);
        let assign7950_e7111: f64 = (p.p623 * locals.var_iae);
        let assign7950_e7112: f64 = (assign7950_e7108 + assign7950_e7111);
        (assign7950_e7112,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign7950_e7114;
        locals.var_a3_p_rv = 0.0;

        let assign7960_e7133: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };
        locals.var_guard92 = assign7960_e7133;
        locals.var_guard92_rv = 0.0;

        let (assign7970_e7151,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign7970_e7140: f64 = (p.p625 * locals.var_ile);
        let assign7970_e7141: f64 = (p.p624 + assign7970_e7140);
        let assign7970_e7144: f64 = (p.p626 * locals.var_iwe);
        let assign7970_e7145: f64 = (assign7970_e7141 + assign7970_e7144);
        let assign7970_e7148: f64 = (p.p627 * locals.var_iae);
        let assign7970_e7149: f64 = (assign7970_e7145 + assign7970_e7148);
        (assign7970_e7149,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign7970_e7151;
        locals.var_a4_p_rv = 0.0;

        let assign7980_e7170: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };
        locals.var_guard93 = assign7980_e7170;
        locals.var_guard93_rv = 0.0;

        let (assign7990_e7190,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign7990_e7178: f64 = (p.p629 * locals.var_ile);
        let assign7990_e7179: f64 = (p.p628 + assign7990_e7178);
        let assign7990_e7182: f64 = (p.p630 * locals.var_iwe);
        let assign7990_e7183: f64 = (assign7990_e7179 + assign7990_e7182);
        let assign7990_e7186: f64 = (p.p631 * locals.var_iae);
        let assign7990_e7187: f64 = (assign7990_e7183 + assign7990_e7186);
        let assign7990_e7188: f64 = (locals.var_iiae * assign7990_e7187);
        (assign7990_e7188,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign7990_e7190;
        locals.var_iginv_p_rv = 0.0;

        let assign8000_e7209: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };
        locals.var_guard94 = assign8000_e7209;
        locals.var_guard94_rv = 0.0;

        let (assign8010_e7229,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard94 != 0.0)) {
        let assign8010_e7217: f64 = (p.p633 * locals.var_ile);
        let assign8010_e7218: f64 = (p.p632 + assign8010_e7217);
        let assign8010_e7221: f64 = (p.p634 * locals.var_iwe);
        let assign8010_e7222: f64 = (assign8010_e7218 + assign8010_e7221);
        let assign8010_e7225: f64 = (p.p635 * locals.var_iae);
        let assign8010_e7226: f64 = (assign8010_e7222 + assign8010_e7225);
        let assign8010_e7227: f64 = (locals.var_iiwe * assign8010_e7226);
        (assign8010_e7227,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign8010_e7229;
        locals.var_igov_p_rv = 0.0;

        let assign8020_e7248: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };
        locals.var_guard95 = assign8020_e7248;
        locals.var_guard95_rv = 0.0;

        let (assign8030_e7268,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign8030_e7256: f64 = (p.p637 * locals.var_ile);
        let assign8030_e7257: f64 = (p.p636 + assign8030_e7256);
        let assign8030_e7260: f64 = (p.p638 * locals.var_iwe);
        let assign8030_e7261: f64 = (assign8030_e7257 + assign8030_e7260);
        let assign8030_e7264: f64 = (p.p639 * locals.var_iae);
        let assign8030_e7265: f64 = (assign8030_e7261 + assign8030_e7264);
        let assign8030_e7266: f64 = (locals.var_iiwe * assign8030_e7265);
        (assign8030_e7266,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign8030_e7268;
        locals.var_igovd_p_rv = 0.0;

        let assign8040_e7287: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };
        locals.var_guard96 = assign8040_e7287;
        locals.var_guard96_rv = 0.0;

        let (assign8050_e7305,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard96 != 0.0)) {
        let assign8050_e7294: f64 = (p.p641 * locals.var_ile);
        let assign8050_e7295: f64 = (p.p640 + assign8050_e7294);
        let assign8050_e7298: f64 = (p.p642 * locals.var_iwe);
        let assign8050_e7299: f64 = (assign8050_e7295 + assign8050_e7298);
        let assign8050_e7302: f64 = (p.p643 * locals.var_iae);
        let assign8050_e7303: f64 = (assign8050_e7299 + assign8050_e7302);
        (assign8050_e7303,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign8050_e7305;
        locals.var_stig_p_rv = 0.0;

        let assign8060_e7324: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };
        locals.var_guard97 = assign8060_e7324;
        locals.var_guard97_rv = 0.0;

        let (assign8070_e7344,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard97 != 0.0)) {
        let assign8070_e7332: f64 = (p.p645 * locals.var_ile);
        let assign8070_e7333: f64 = (p.p644 + assign8070_e7332);
        let assign8070_e7336: f64 = (p.p646 * locals.var_iwe);
        let assign8070_e7337: f64 = (assign8070_e7333 + assign8070_e7336);
        let assign8070_e7340: f64 = (p.p647 * locals.var_iae);
        let assign8070_e7341: f64 = (assign8070_e7337 + assign8070_e7340);
        let assign8070_e7342: f64 = (locals.var_iiwe * assign8070_e7341);
        (assign8070_e7342,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign8070_e7344;
        locals.var_agidl_p_rv = 0.0;

        let assign8080_e7363: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };
        locals.var_guard98 = assign8080_e7363;
        locals.var_guard98_rv = 0.0;

        let (assign8090_e7383,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard98 != 0.0)) {
        let assign8090_e7371: f64 = (p.p649 * locals.var_ile);
        let assign8090_e7372: f64 = (p.p648 + assign8090_e7371);
        let assign8090_e7375: f64 = (p.p650 * locals.var_iwe);
        let assign8090_e7376: f64 = (assign8090_e7372 + assign8090_e7375);
        let assign8090_e7379: f64 = (p.p651 * locals.var_iae);
        let assign8090_e7380: f64 = (assign8090_e7376 + assign8090_e7379);
        let assign8090_e7381: f64 = (locals.var_iiwe * assign8090_e7380);
        (assign8090_e7381,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign8090_e7383;
        locals.var_agidld_p_rv = 0.0;

        let assign8100_e7402: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign8100_e7402;
        locals.var_guard99_rv = 0.0;

        let (assign8110_e7420,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard99 != 0.0)) {
        let assign8110_e7409: f64 = (p.p653 * locals.var_ile);
        let assign8110_e7410: f64 = (p.p652 + assign8110_e7409);
        let assign8110_e7413: f64 = (p.p654 * locals.var_iwe);
        let assign8110_e7414: f64 = (assign8110_e7410 + assign8110_e7413);
        let assign8110_e7417: f64 = (p.p655 * locals.var_iae);
        let assign8110_e7418: f64 = (assign8110_e7414 + assign8110_e7417);
        (assign8110_e7418,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign8110_e7420;
        locals.var_stbgidl_p_rv = 0.0;

        let assign8120_e7439: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };
        locals.var_guard100 = assign8120_e7439;
        locals.var_guard100_rv = 0.0;

        let (assign8130_e7457,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard100 != 0.0)) {
        let assign8130_e7446: f64 = (p.p657 * locals.var_ile);
        let assign8130_e7447: f64 = (p.p656 + assign8130_e7446);
        let assign8130_e7450: f64 = (p.p658 * locals.var_iwe);
        let assign8130_e7451: f64 = (assign8130_e7447 + assign8130_e7450);
        let assign8130_e7454: f64 = (p.p659 * locals.var_iae);
        let assign8130_e7455: f64 = (assign8130_e7451 + assign8130_e7454);
        (assign8130_e7455,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign8130_e7457;
        locals.var_stbgidld_p_rv = 0.0;

        let assign8140_e7476: f64 = if (((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) { 1.0 } else { 0.0 };
        locals.var_guard101 = assign8140_e7476;
        locals.var_guard101_rv = 0.0;

        let (assign8150_e7500,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard101 != 0.0)) {
        let assign8150_e7482: f64 = (locals.var_iiwecv * locals.var_lecv);
        let assign8150_e7484: f64 = (assign8150_e7482 / 1e-6);
        let assign8150_e7488: f64 = (p.p661 * locals.var_ile);
        let assign8150_e7489: f64 = (p.p660 + assign8150_e7488);
        let assign8150_e7492: f64 = (p.p662 * locals.var_iwe);
        let assign8150_e7493: f64 = (assign8150_e7489 + assign8150_e7492);
        let assign8150_e7496: f64 = (p.p663 * locals.var_iae);
        let assign8150_e7497: f64 = (assign8150_e7493 + assign8150_e7496);
        let assign8150_e7498: f64 = (assign8150_e7484 * assign8150_e7497);
        (assign8150_e7498,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign8150_e7500;
        locals.var_cox_p_rv = 0.0;

        let assign8160_e7519: f64 = if (((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign8160_e7519;
        locals.var_guard102_rv = 0.0;

        let (assign8170_e7537,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard102 != 0.0)) {
        let assign8170_e7526: f64 = (p.p665 * locals.var_ile);
        let assign8170_e7527: f64 = (p.p664 + assign8170_e7526);
        let assign8170_e7530: f64 = (p.p666 * locals.var_iwe);
        let assign8170_e7531: f64 = (assign8170_e7527 + assign8170_e7530);
        let assign8170_e7534: f64 = (p.p667 * locals.var_iae);
        let assign8170_e7535: f64 = (assign8170_e7531 + assign8170_e7534);
        (assign8170_e7535,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign8170_e7537;
        locals.var_delvtac_p_rv = 0.0;

        let assign8180_e7556: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign8180_e7556;
        locals.var_guard103_rv = 0.0;

        let (assign8190_e7574,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard103 != 0.0)) {
        let assign8190_e7563: f64 = (p.p669 * locals.var_ile);
        let assign8190_e7564: f64 = (p.p668 + assign8190_e7563);
        let assign8190_e7567: f64 = (p.p670 * locals.var_iwe);
        let assign8190_e7568: f64 = (assign8190_e7564 + assign8190_e7567);
        let assign8190_e7571: f64 = (p.p671 * locals.var_iae);
        let assign8190_e7572: f64 = (assign8190_e7568 + assign8190_e7571);
        (assign8190_e7572,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign8190_e7574;
        locals.var_facneffac_p_rv = 0.0;

        let assign8200_e7613: f64 = if (((((((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) || param_given[580]) || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign8200_e7613;
        locals.var_guard104_rv = 0.0;

        let (assign8210_e7619,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p580,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8210_e7619;
        locals.var_poparam_i_rv = 0.0;

        let assign8220_e7621: f64 = if param_given[672] { 1.0 } else { 0.0 };
        let assign8220_e7623: f64 = if assign8220_e7621 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign8220_e7623;
        locals.var_guard105_rv = 0.0;

        let (assign8230_e7631,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard105 != 0.0)) {
        (p.p672,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8230_e7631;
        locals.var_poparam_i_rv = 0.0;

        let (assign8240_e7637,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p581,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8240_e7637;
        locals.var_plparam_i_rv = 0.0;

        let assign8250_e7639: f64 = if param_given[673] { 1.0 } else { 0.0 };
        let assign8250_e7641: f64 = if assign8250_e7639 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign8250_e7641;
        locals.var_guard106_rv = 0.0;

        let (assign8260_e7649,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard106 != 0.0)) {
        (p.p673,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8260_e7649;
        locals.var_plparam_i_rv = 0.0;

        let (assign8270_e7655,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p582,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8270_e7655;
        locals.var_pwparam_i_rv = 0.0;

        let assign8280_e7657: f64 = if param_given[674] { 1.0 } else { 0.0 };
        let assign8280_e7659: f64 = if assign8280_e7657 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign8280_e7659;
        locals.var_guard107_rv = 0.0;

        let (assign8290_e7667,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard107 != 0.0)) {
        (p.p674,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8290_e7667;
        locals.var_pwparam_i_rv = 0.0;

        let (assign8300_e7673,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p583,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8300_e7673;
        locals.var_plwparam_i_rv = 0.0;

        let assign8310_e7675: f64 = if param_given[675] { 1.0 } else { 0.0 };
        let assign8310_e7677: f64 = if assign8310_e7675 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign8310_e7677;
        locals.var_guard108_rv = 0.0;

        let (assign8320_e7685,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard108 != 0.0)) {
        (p.p675,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8320_e7685;
        locals.var_plwparam_i_rv = 0.0;

        let (assign8330_e7705,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        let assign8330_e7693: f64 = (locals.var_plparam_i * locals.var_ile);
        let assign8330_e7694: f64 = (locals.var_poparam_i + assign8330_e7693);
        let assign8330_e7697: f64 = (locals.var_pwparam_i * locals.var_iwe);
        let assign8330_e7698: f64 = (assign8330_e7694 + assign8330_e7697);
        let assign8330_e7701: f64 = (locals.var_plwparam_i * locals.var_iae);
        let assign8330_e7702: f64 = (assign8330_e7698 + assign8330_e7701);
        let assign8330_e7703: f64 = (locals.var_ile * assign8330_e7702);
        (assign8330_e7703,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign8330_e7705;
        locals.var_thesatac_p_rv = 0.0;

        let assign8340_e7744: f64 = if (((((((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) || param_given[596]) || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign8340_e7744;
        locals.var_guard109_rv = 0.0;

        let (assign8350_e7750,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p596,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8350_e7750;
        locals.var_poparam_i_rv = 0.0;

        let assign8360_e7752: f64 = if param_given[676] { 1.0 } else { 0.0 };
        let assign8360_e7754: f64 = if assign8360_e7752 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign8360_e7754;
        locals.var_guard110_rv = 0.0;

        let (assign8370_e7762,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) {
        (p.p676,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8370_e7762;
        locals.var_poparam_i_rv = 0.0;

        let (assign8380_e7768,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p597,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8380_e7768;
        locals.var_plparam_i_rv = 0.0;

        let assign8390_e7770: f64 = if param_given[677] { 1.0 } else { 0.0 };
        let assign8390_e7772: f64 = if assign8390_e7770 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign8390_e7772;
        locals.var_guard111_rv = 0.0;

        let (assign8400_e7780,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
        (p.p677,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8400_e7780;
        locals.var_plparam_i_rv = 0.0;

        let (assign8410_e7786,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p598,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8410_e7786;
        locals.var_pwparam_i_rv = 0.0;

        let assign8420_e7788: f64 = if param_given[678] { 1.0 } else { 0.0 };
        let assign8420_e7790: f64 = if assign8420_e7788 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign8420_e7790;
        locals.var_guard112_rv = 0.0;

        let (assign8430_e7798,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard112 != 0.0)) {
        (p.p678,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8430_e7798;
        locals.var_pwparam_i_rv = 0.0;

        let (assign8440_e7804,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p599,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8440_e7804;
        locals.var_plwparam_i_rv = 0.0;

        let assign8450_e7806: f64 = if param_given[679] { 1.0 } else { 0.0 };
        let assign8450_e7808: f64 = if assign8450_e7806 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign8450_e7808;
        locals.var_guard113_rv = 0.0;

        let (assign8460_e7816,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard113 != 0.0)) {
        (p.p679,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8460_e7816;
        locals.var_plwparam_i_rv = 0.0;

    }
}
